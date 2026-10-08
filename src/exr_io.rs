//! OpenEXR in and out, as D-62 says (B-16b, requirement R-15).
//!
//! OpenEXR's convention - linear light, premultiplied - is the working buffer's, so neither
//! direction converts colour. What this module does decide is which part of a file is the
//! picture, which channels are the colour, and what is refused; every one of those answers is
//! D-62's, and `Fixtures/exr/expected_exr.json`, written by OpenEXR's own library, is what
//! `tests/b16b_exr.rs` holds them to, pixel for pixel.
//!
//! Refusals are decided from the header, before a pixel is decoded. `refused/htj2k.exr` is the
//! reason that order matters: the crate reads that small file, and it must be refused anyway.

use std::path::Path;

use exr::compression::Compression;
use exr::meta::attribute::{Chromaticities, Text};
use exr::meta::MetaData;
use exr::prelude::{
    f16, AnyChannel, AnyChannels, Blocks, Encoding, FlatSamples, Image, Layer, LayerAttributes,
    LineOrder, ReadChannels, ReadLayers, SmallVec, Vec2, WritableImage,
};

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::time::FrameRate;
use crate::{AlphaMode, ColorSpace, ImageBuffer, WorkingBuffer};

/// Rec. 709 primaries and D65 white, as the file stores them: 32-bit floats.
fn rec709() -> Chromaticities {
    Chromaticities {
        red: Vec2(0.64, 0.33),
        green: Vec2(0.30, 0.60),
        blue: Vec2(0.15, 0.06),
        white: Vec2(0.3127, 0.3290),
    }
}

/// A drawn EXR: the picture, and each reason `MEDIA_EXR_ADJUSTED` must give, in D-62's order.
pub struct ExrPicture {
    pub image: ImageBuffer,
    /// `(reason, detail)`, such as `("non_finite", "4")`. Empty when the file was drawn as stored.
    pub adjusted: Vec<(&'static str, String)>,
}

impl ExrPicture {
    /// The one warning that says the file was drawn but not exactly as stored, or `None`.
    pub fn diagnostic(&self, path: &Path) -> Option<Diagnostic> {
        if self.adjusted.is_empty() {
            return None;
        }
        let lines: Vec<String> = self
            .adjusted
            .iter()
            .map(|(reason, detail)| format!("{reason}: {detail}"))
            .collect();
        Some(
            Diagnostic::new(
                DiagnosticId::MediaExrAdjusted,
                Severity::Warning,
                format!("{} was drawn, but not exactly as stored.", name(path)),
                lines.join("\n"),
            )
            .with_remediation(
                "Check the drawing looks right. The file is left untouched; re-export it from \
                 the program that made it to remove the reasons listed.",
            ),
        )
    }
}

/// Read an EXR file into a buffer tagged linear light, premultiplied.
///
/// A refusal is `MEDIA_UNSUPPORTED_FORMAT` and an unreadable file `MEDIA_DECODE_FAILED`; either
/// way the detail begins `Reason: <reason>.`, with D-62's reason.
pub fn read(path: &Path) -> Result<ExrPicture, Diagnostic> {
    let meta = checked(path)?;
    let header = &meta.headers[0];
    let channels = &header.channels.list;
    let has = |n: &str| channels.iter().any(|c| c.name == *n);
    let colour: &[&str] = if has("R") || has("G") || has("B") {
        &["R", "G", "B"]
    } else if has("RY") || has("BY") {
        return Err(refused(
            path,
            "luminance_chroma",
            "The colour is stored as luminance and colour difference.",
        ));
    } else if has("Y") {
        &["Y"]
    } else {
        return Err(refused(
            path,
            "no_colour_channels",
            "No channel is named R, G, B or Y.",
        ));
    };
    let ignored: Vec<String> = channels
        .iter()
        .map(|c| c.name.to_string())
        .filter(|n| n != "A" && !colour.contains(&n.as_str()))
        .collect();

    let display = header.shared_attributes.display_window;
    let pixel_aspect = header.shared_attributes.pixel_aspect;
    let chromaticities = header.shared_attributes.chromaticities;

    let file = read_all(path)?;
    let layer = &file.layer_data[0];
    let get = |n: &str| values(layer, n);
    let (red, green, blue) = if colour == ["Y"] {
        let y = get("Y");
        (y.clone(), y.clone(), y)
    } else {
        (get("R"), get("G"), get("B"))
    };
    let alpha = get("A");

    let (w, h) = (display.size.0, display.size.1);
    let (dw, dh) = (layer.size.0, layer.size.1);
    let (mut data, overlap) = place(display, layer, [red, green, blue, alpha]);

    let mut adjusted = Vec::new();
    if !ignored.is_empty() {
        adjusted.push(("channels_ignored", ignored.join(", ")));
    }
    if dw * dh > overlap {
        adjusted.push(("outside_display_window", (dw * dh - overlap).to_string()));
    }
    let mut non_finite = 0;
    for v in data.iter_mut().filter(|v| !v.is_finite()) {
        *v = 0.0;
        non_finite += 1;
    }
    if non_finite > 0 {
        adjusted.push(("non_finite", non_finite.to_string()));
    }
    let mut clamped = 0;
    for a in data.iter_mut().skip(3).step_by(4) {
        if !(0.0..=1.0).contains(a) {
            *a = a.clamp(0.0, 1.0);
            clamped += 1;
        }
    }
    if clamped > 0 {
        adjusted.push(("alpha_clamped", clamped.to_string()));
    }
    if pixel_aspect != 1.0 {
        adjusted.push(("pixel_aspect", py_repr(pixel_aspect)));
    }
    if let Some(c) = chromaticities.filter(|c| *c != rec709()) {
        let values = [
            c.red.0, c.red.1, c.green.0, c.green.1, c.blue.0, c.blue.1, c.white.0, c.white.1,
        ];
        let text: Vec<String> = values.iter().map(|v| py_repr(*v)).collect();
        adjusted.push(("primaries", text.join(", ")));
    }

    let image = ImageBuffer::new(
        w,
        h,
        ColorSpace::LinearLight,
        AlphaMode::Premultiplied,
        data,
    )
    .expect("the data was allocated at the display window's size");
    Ok(ExrPicture { image, adjusted })
}

/// The header of a file D-62 reads: one part, not deep, not HTJ2K, every channel at full
/// resolution. Decided before a pixel is decoded.
fn checked(path: &Path) -> Result<MetaData, Diagnostic> {
    let meta = MetaData::read_from_file(path, false).map_err(|e| unreadable(path, &e))?;
    let header = match meta.headers.as_slice() {
        [] => return Err(unreadable(path, &"The file holds no parts.")),
        [one] => one,
        _ => {
            return Err(refused(
                path,
                "multipart",
                "The file holds more than one part.",
            ))
        }
    };
    if header.deep {
        return Err(refused(path, "deep", "The file holds deep data."));
    }
    if matches!(
        header.compression,
        Compression::HTJ2K32 | Compression::HTJ2K256
    ) {
        return Err(refused(path, "htj2k", "The file is HTJ2K compressed."));
    }
    if header.channels.list.iter().any(|c| c.sampling != Vec2(1, 1)) {
        return Err(refused(
            path,
            "subsampled",
            "A channel is stored at less than full resolution.",
        ));
    }
    Ok(meta)
}

/// `MEDIA_DECODE_FAILED`, with D-62's reason `unreadable`.
fn unreadable(path: &Path, e: &dyn std::fmt::Display) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaDecodeFailed,
        Severity::Error,
        format!("{} could not be read.", name(path)),
        format!("Reason: unreadable. {e}"),
    )
    .with_remediation("Check the file opens in another application, then relink the sequence.")
}

type Flat = Layer<AnyChannels<FlatSamples>>;

/// Every channel of the file's largest level.
fn read_all(path: &Path) -> Result<Image<SmallVec<[Flat; 2]>>, Diagnostic> {
    exr::prelude::read()
        .no_deep_data()
        .largest_resolution_level()
        .all_channels()
        .all_layers()
        .all_attributes()
        .non_parallel()
        .from_file(path)
        .map_err(|e| unreadable(path, &e))
}

/// The channel named `n`, as single-precision numbers in the data window's order.
fn values(layer: &Flat, n: &str) -> Option<Vec<f32>> {
    layer
        .channel_data
        .list
        .iter()
        .find(|c| c.name == *n)
        .map(|c| c.sample_data.values_as_f32().collect())
}

/// D-62: the picture is the display window; the data window is copied where it overlaps, every
/// number 0 elsewhere. `channels` are red, green, blue and alpha; where the data window is, a
/// missing colour is 0 and a missing alpha 1. With how many of the data window's pixels overlap.
fn place(
    display: exr::meta::attribute::IntegerBounds,
    layer: &Flat,
    channels: [Option<Vec<f32>>; 4],
) -> (Vec<f32>, usize) {
    let (w, h) = (display.size.0, display.size.1);
    let (dw, dh) = (layer.size.0, layer.size.1);
    let origin = layer.attributes.layer_position;
    let mut data = vec![0f32; w * h * 4];
    let mut overlap = 0usize;
    for sy in 0..dh {
        let y = origin.1 as i64 + sy as i64 - display.position.1 as i64;
        if y < 0 || y >= h as i64 {
            continue;
        }
        for sx in 0..dw {
            let x = origin.0 as i64 + sx as i64 - display.position.0 as i64;
            if x < 0 || x >= w as i64 {
                continue;
            }
            overlap += 1;
            let s = sy * dw + sx;
            let d = (y as usize * w + x as usize) * 4;
            for (c, channel) in channels.iter().enumerate() {
                let default = if c == 3 { 1.0 } else { 0.0 };
                data[d + c] = channel.as_ref().map_or(default, |v| v[s]);
            }
        }
    }
    (data, overlap)
}

/// D-348: a render's pass stored beside the colour, which an effect asks for by what it is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pass {
    Depth,
    Normals,
}

impl Pass {
    pub fn word(self) -> &'static str {
        match self {
            Pass::Depth => "depth",
            Pass::Normals => "normals",
        }
    }
}

/// D-348: the channels among `names` (the file's order) that hold `pass`, as red, green and
/// blue. A name splits at its last dot into a layer and a part, case ignored. The depth is the
/// first part `z` of layer "", `depth`, `zdepth` or `z`, or a name `depth` or `zdepth` with no
/// layer; the normals the first layer `n`, `normal` or `normals` holding all of `x`, `y`, `z`,
/// failing that all of `r`, `g`, `b`.
pub fn pass_channels(names: &[String], pass: Pass) -> Option<[String; 3]> {
    let split = |n: &str| {
        let (layer, part) = n.rsplit_once('.').unwrap_or(("", n));
        (layer.to_ascii_lowercase(), part.to_ascii_lowercase())
    };
    match pass {
        Pass::Depth => names
            .iter()
            .find(|n| {
                let (layer, part) = split(n);
                (part == "z" && ["", "depth", "zdepth", "z"].contains(&layer.as_str()))
                    || (layer.is_empty() && (part == "depth" || part == "zdepth"))
            })
            .map(|n| [n.clone(), n.clone(), n.clone()]),
        Pass::Normals => [["x", "y", "z"], ["r", "g", "b"]].iter().find_map(|parts| {
            names.iter().find_map(|n| {
                let (layer, _) = split(n);
                if !["n", "normal", "normals"].contains(&layer.as_str()) {
                    return None;
                }
                let find = |p: &str| names.iter().find(|m| split(m) == (layer.clone(), p.to_string())).cloned();
                Some([find(parts[0])?, find(parts[1])?, find(parts[2])?])
            })
        }),
    }
}

/// D-348: `EFFECT_CHANNEL_MISSING`, said on each frame an effect finds no pass to read.
pub fn pass_missing(title: String, detail: String) -> Diagnostic {
    Diagnostic::new(DiagnosticId::EffectChannelMissing, Severity::Warning, title, detail).with_remediation(
        "Render the file again with the pass saved in it, or put the effect on a layer whose EXR \
         file holds it. The effect's settings are kept; until then it changes nothing.",
    )
}

/// D-348: the pass in the EXR file at `path`, laid out as the colour is (D-62): each pixel its
/// three numbers and alpha 1 over the whole display window. With how many of its numbers were
/// not finite: not-a-number reads as 0 and an infinity as the largest single-precision number of
/// its sign. `EFFECT_CHANNEL_MISSING` when the file has no such pass; [`read`]'s refusals as it
/// gives them.
pub fn read_pass(path: &Path, pass: Pass) -> Result<(WorkingBuffer, usize), Diagnostic> {
    let meta = checked(path)?;
    let header = &meta.headers[0];
    let names: Vec<String> = header.channels.list.iter().map(|c| c.name.to_string()).collect();
    let Some(chosen) = pass_channels(&names, pass) else {
        return Err(pass_missing(
            format!("{} holds no {} pass.", name(path), pass.word()),
            format!(
                "Its channels are {}. D-348 reads the depth from a channel named Z (or depth.Z, \
                 zdepth.Z, depth) and the normals from N.x, N.y, N.z (or normal.R, normal.G, \
                 normal.B).",
                names.join(", ")
            ),
        ));
    };
    // ponytail: every channel is decoded to use three; read only those if a file with many
    // passes makes the read measurable.
    let file = read_all(path)?;
    let layer = &file.layer_data[0];
    let [r, g, b] = chosen.map(|n| values(layer, &n));
    let display = header.shared_attributes.display_window;
    let (data, _) = place(display, layer, [r, g, b, None]);
    let mut buffer = WorkingBuffer::transparent(display.size.0, display.size.1);
    let mut non_finite = 0;
    for (i, (out, v)) in buffer.data_mut().iter_mut().zip(data).enumerate() {
        *out = if i % 4 == 3 {
            1.0
        } else if v.is_finite() {
            v
        } else {
            non_finite += 1;
            if v.is_nan() { 0.0 } else { f32::MAX.copysign(v) }
        };
    }
    Ok((buffer, non_finite))
}

/// Half or float samples in a written file.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ExrSamples {
    Half,
    Float,
}

/// Write the working buffer as D-62's export file: one part, scanlines top to bottom, `A`,
/// `B`, `G`, `R`, ZIP, both windows the buffer's size at the origin, Rec. 709 and the frame rate.
///
/// The samples are the buffer's, premultiplied. Half clamps to its range and rounds to the
/// nearest half, ties to even; float is exact.
pub fn write(
    path: &Path,
    buffer: &WorkingBuffer,
    samples: ExrSamples,
    rate: FrameRate,
) -> Result<(), String> {
    write_bytes(path, &encode(buffer, samples, rate)?)
}

/// Write what [`encode`] made. A file left half-written is removed, as the crate's own
/// `to_file` does, and a failure is said in the crate's words.
pub fn write_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    std::fs::write(path, bytes).map_err(|e| {
        let _ = std::fs::remove_file(path);
        exr::error::Error::from(e).to_string()
    })
}

/// [`write`]'s file, as bytes, for D-231's export, which encodes frames side by side and writes
/// them in order.
pub fn encode(buffer: &WorkingBuffer, samples: ExrSamples, rate: FrameRate) -> Result<Vec<u8>, String> {
    let (w, h) = (buffer.width(), buffer.height());
    let channel = |name: &str, k: usize| {
        let values = buffer.data().iter().skip(k).step_by(4).copied();
        let data = match samples {
            ExrSamples::Half => FlatSamples::F16(
                values
                    .map(|x| f16::from_f32(x.clamp(-65504.0, 65504.0)))
                    .collect(),
            ),
            ExrSamples::Float => FlatSamples::F32(values.collect()),
        };
        AnyChannel::new(Text::from(name), data)
    };
    let list = AnyChannels::sort(SmallVec::from_vec(vec![
        channel("R", 0),
        channel("G", 1),
        channel("B", 2),
        channel("A", 3),
    ]));
    let mut layer = Layer::new(
        (w, h),
        LayerAttributes::default(),
        Encoding {
            compression: Compression::ZIP16,
            blocks: Blocks::ScanLines,
            line_order: LineOrder::Increasing,
        },
        list,
    );
    // The frame rate is `(i32, u32)` in the crate; a rate past i32 is not a rate anyone set.
    layer.attributes.frames_per_second = Some((rate.numerator() as i32, rate.denominator()));
    let mut image = Image::from_layer(layer);
    image.attributes.chromaticities = Some(rec709());
    let mut file = std::io::Cursor::new(Vec::new());
    image.write().to_buffered(&mut file).map_err(|e| e.to_string())?;
    Ok(file.into_inner())
}

/// True when the file name ends in `.exr`, in any case. The extension decides the reader.
pub fn is_exr(name: &str) -> bool {
    Path::new(name)
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("exr"))
}

fn refused(path: &Path, reason: &str, what: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaUnsupportedFormat,
        Severity::Error,
        format!("{} is an EXR file this build does not draw.", name(path)),
        format!("Reason: {reason}. {what}"),
    )
    .with_remediation(
        "Re-export it as a single-part RGBA EXR. The file is left untouched and the asset record \
         is kept.",
    )
}

/// Python's `repr` of a float, which is how `tools/exr_reference.py` writes the numbers.
///
/// ponytail: Rust never writes an exponent where Python does (1e-05, 1e+16); no D-62 value is
/// that small or large, and the fixture table would catch one that is.
fn py_repr(v: f32) -> String {
    let s = format!("{}", v as f64);
    if s.contains('.') || !v.is_finite() {
        s
    } else {
        s + ".0"
    }
}

fn name(path: &Path) -> String {
    path.file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned()
}
