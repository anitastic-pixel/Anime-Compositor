//! G1-core colour, alpha and compositing (B-02, requirement R-10).
//!
//! Document 06 line 45: "An untagged buffer never crosses a module boundary." That is
//! enforced structurally here rather than by convention. [`ImageBuffer`] always carries its
//! [`ColorSpace`] and [`AlphaMode`]; the compositor in [`composite`] does not accept an
//! `ImageBuffer` at all. It accepts only a [`WorkingBuffer`], which cannot be constructed
//! except by converting a tagged buffer into the document 21 working space. Compositing two
//! images in the wrong space is therefore not a checked error, it is unrepresentable.

use rayon::prelude::*;

/// D-878: the motion the finder finds from `a` to `b`, Pixel Motion Blur's settings as added at
/// `vector_detail`, on the graphics card with `card` (the processor's finder when the card
/// cannot), for the checks that hold the card's to the processor's.
#[doc(hidden)]
pub fn found_motion(
    a: &WorkingBuffer,
    b: &WorkingBuffer,
    vector_detail: f64,
    card: bool,
) -> (Vec<f64>, Vec<f64>) {
    let tuning = flow::Tuning {
        card,
        ..flow::Tuning::added(vector_detail)
    };
    let f = flow::estimate(a, b, &tuning);
    (f.x, f.y)
}

/// D-878: whether the motion finder has a graphics card on this machine.
#[doc(hidden)]
pub fn motion_card() -> bool {
    flow::card_available()
}

/// D-878: the card motion finder switched off (`true`) or on again, for B-758's before and after.
#[doc(hidden)]
pub fn motion_card_off(off: bool) {
    flow::CARD_SWITCHED_OFF.store(off, std::sync::atomic::Ordering::Relaxed);
}

/// D-878: how many motions the card has found in this process, for B-758's evidence.
#[doc(hidden)]
pub fn motion_card_uses() -> u64 {
    flow::card_uses()
}

mod along;
pub mod audio;
mod bloom;
mod blurs;
pub mod cache;
mod cel_fx;
pub mod color;
pub mod command;
pub mod compose;
pub mod composite;
pub mod diagnostics;
pub mod effects;
pub mod export;
pub mod expr;
pub mod exr_io;
pub mod film_out;
mod flow;
pub mod frame_stats;
mod glow;
pub mod gpu;
mod grade;
mod hsv_key;
pub mod keykind;
mod layer_fx;
mod layer_map;
mod line_blur;
mod line_smooth;
mod line_width;
pub mod lut;
pub mod mask;
mod matte_refine;
pub mod media;
mod median;
mod mesh;
pub mod model;
pub mod mp4_out;
pub mod package;
pub mod particles;
pub mod perf;
pub mod persist;
pub mod playground;
pub mod png_out;
pub mod preview;
pub mod render;
pub mod selective_blur;
pub mod session_log;
pub mod sha256;
pub mod shape;
mod shape_ops;
pub mod simulate;
mod soft_glow;
pub mod stop;
pub mod text;
pub mod time;
pub mod timesheet;
pub mod trace;
pub mod view;

/// How the RGB channels of a buffer are encoded.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ColorSpace {
    /// Linear light. The working space of document 21.
    LinearLight,
    /// sRGB encoded, IEC 61966-2-1 transfer function. See [`color`].
    Srgb,
    /// D-909: stored in the named space, converted at decode into the working gamut (the second).
    /// Today's two words stay `Srgb` and `LinearLight` with colour management off, so their
    /// pictures and their cache keys are what they were.
    Named(color::Space, color::Gamut),
    /// D-909: a word this build does not have (`COLOR_SPACE_UNKNOWN`). It decodes to transparent.
    /// D-910: the word itself, so the asset saves it back with or without the file it came from.
    Unknown(&'static str),
}

impl ColorSpace {
    /// D-909: the project file's words for an asset's colour space, with the space each names.
    pub const WORDS: [(&'static str, color::Space); 10] = [
        ("srgb", color::SRGB_REC709),
        ("linear-srgb", color::LINEAR_REC709),
        ("acescg", color::ACESCG),
        ("aces2065-1", color::ACES2065_1),
        ("acescct", color::ACESCCT),
        ("acescc", color::ACESCC),
        ("linear-rec2020", color::LINEAR_REC2020),
        ("linear-p3-d65", color::LINEAR_P3_D65),
        ("bt1886-rec709", color::BT1886_REC709),
        ("gamma22-rec709", color::GAMMA22_REC709),
    ];

    /// The space a word names, read into today's working space (linear Rec.709); `None` for a
    /// word this build does not have.
    pub fn from_word(word: &str) -> Option<ColorSpace> {
        match word {
            "srgb" => Some(ColorSpace::Srgb),
            "linear-srgb" => Some(ColorSpace::LinearLight),
            _ => Self::WORDS
                .iter()
                .find(|(w, _)| *w == word)
                .map(|(_, s)| ColorSpace::Named(*s, color::Gamut::Rec709)),
        }
    }

    /// D-910: an unknown word, kept for saving. Each distinct word is leaked once and reused.
    pub fn unknown(word: &str) -> ColorSpace {
        // ponytail: a global list of every unknown word read this session, never freed; a few
        // words in practice, an interner if a file ever brings thousands.
        static SEEN: std::sync::Mutex<Vec<&'static str>> = std::sync::Mutex::new(Vec::new());
        let mut seen = SEEN.lock().unwrap_or_else(|e| e.into_inner());
        let kept = match seen.iter().find(|w| **w == word) {
            Some(w) => *w,
            None => {
                let w: &'static str = Box::leak(word.to_owned().into_boxed_str());
                seen.push(w);
                w
            }
        };
        ColorSpace::Unknown(kept)
    }

    /// The space the file's numbers are stored in; `None` when unknown.
    pub fn stored(self) -> Option<color::Space> {
        match self {
            ColorSpace::Srgb => Some(color::SRGB_REC709),
            ColorSpace::LinearLight => Some(color::LINEAR_REC709),
            ColorSpace::Named(s, _) => Some(s),
            ColorSpace::Unknown(_) => None,
        }
    }

    /// The project file's word; `None` when unknown (the file's own word is kept instead).
    pub fn word(self) -> Option<&'static str> {
        let s = self.stored()?;
        Self::WORDS.iter().find(|(_, n)| *n == s).map(|(w, _)| *w)
    }
}

/// How the RGB channels relate to alpha.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AlphaMode {
    /// RGB has already been multiplied by alpha.
    Premultiplied,
    /// RGB is independent of alpha. Document 21: G1 PNG input is straight.
    Straight,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BufferError {
    /// `data.len()` was not `width * height * 4`.
    LengthMismatch { expected: usize, actual: usize },
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BufferError::LengthMismatch { expected, actual } => write!(
                f,
                "buffer length {actual} does not match {expected} for the declared extent"
            ),
        }
    }
}

/// Bit depth of an exported PNG. R-09: "a PNG sequence with chosen bit depth".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputDepth {
    Eight,
    Sixteen,
}

/// Alpha policy of an exported PNG. Document 21 line 31 writes straight alpha "unless the
/// chosen encoder/format contract explicitly differs"; R-09 makes that a choice, so it is one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OutputAlpha {
    Straight,
    Premultiplied,
}

/// An RGBA float32 image that knows how it is encoded.
///
/// There is deliberately no way to build one without stating both tags.
#[derive(Clone, Debug)]
pub struct ImageBuffer {
    width: usize,
    height: usize,
    color_space: ColorSpace,
    alpha_mode: AlphaMode,
    /// RGBA, row-major, top-left origin (document 21 coordinate system).
    data: Vec<f32>,
}

impl ImageBuffer {
    pub fn new(
        width: usize,
        height: usize,
        color_space: ColorSpace,
        alpha_mode: AlphaMode,
        data: Vec<f32>,
    ) -> Result<Self, BufferError> {
        let expected = width * height * 4;
        if data.len() != expected {
            return Err(BufferError::LengthMismatch {
                expected,
                actual: data.len(),
            });
        }
        Ok(ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            data,
        })
    }

    /// Transparent black (document 21: `(0,0,0,0)`), in the given encoding.
    pub fn transparent(
        width: usize,
        height: usize,
        color_space: ColorSpace,
        alpha_mode: AlphaMode,
    ) -> Self {
        ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            data: vec![0.0; width * height * 4],
        }
    }

    /// Decode 8-bit RGBA as document 21 specifies for G1 PNG input: sRGB encoded, straight alpha.
    pub fn from_srgb8_straight(
        width: usize,
        height: usize,
        bytes: &[u8],
    ) -> Result<Self, BufferError> {
        let expected = width * height * 4;
        if bytes.len() != expected {
            return Err(BufferError::LengthMismatch {
                expected,
                actual: bytes.len(),
            });
        }
        // P-18: across the pool. Each sample is its own, so the order of the work cannot move one.
        let data = perf::time(perf::Stage::Dequantise, || {
            bytes.par_iter().map(|&v| color::dequantise_u8(v)).collect()
        });
        ImageBuffer::new(width, height, ColorSpace::Srgb, AlphaMode::Straight, data)
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn color_space(&self) -> ColorSpace {
        self.color_space
    }
    pub fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
    pub fn data(&self) -> &[f32] {
        &self.data
    }

    pub fn pixel(&self, x: usize, y: usize) -> [f32; 4] {
        let i = (y * self.width + x) * 4;
        [
            self.data[i],
            self.data[i + 1],
            self.data[i + 2],
            self.data[i + 3],
        ]
    }

    /// Convert into the document 21 working space: linear light, premultiplied.
    ///
    /// Each conversion runs at most once, and only if the tag says it is needed. A buffer
    /// already tagged `LinearLight` is not transformed again; this is what makes a duplicate
    /// display transform (T-09) impossible rather than merely discouraged.
    pub fn into_working(self) -> WorkingBuffer {
        let ImageBuffer {
            width,
            height,
            color_space,
            alpha_mode,
            mut data,
        } = self;
        // D-909: the curve, and the matrix worked out once for the whole picture (none when the
        // primaries are already the working ones). Today's two tags take today's steps exactly.
        let (curve, matrix) = match color_space {
            ColorSpace::LinearLight | ColorSpace::Unknown(_) => (color::Curve::Linear, None),
            ColorSpace::Srgb => (color::Curve::Srgb, None),
            ColorSpace::Named(s, to) => (
                s.curve,
                (s.gamut != to).then(|| color::gamut_matrix(s.gamut, to)),
            ),
        };
        if matches!(color_space, ColorSpace::Unknown(_)) {
            data.fill(0.0);
        }
        // P-18: across the pool, as the dequantise before it. Each pixel is its own.
        perf::time(perf::Stage::ToLinear, || {
            data.par_chunks_exact_mut(4)
                .with_min_len(4096)
                .for_each(|px| {
                    if curve != color::Curve::Linear {
                        for c in &mut px[..3] {
                            *c = curve.to_linear(*c);
                        }
                    }
                    if let Some(m) = &matrix {
                        let rgb = color::apply_f32(m, [px[0], px[1], px[2]]);
                        px[..3].copy_from_slice(&rgb);
                    }
                    if alpha_mode == AlphaMode::Straight {
                        let a = px[3];
                        for c in &mut px[..3] {
                            *c *= a;
                        }
                    }
                });
        });
        WorkingBuffer(ImageBuffer {
            width,
            height,
            color_space: ColorSpace::LinearLight,
            alpha_mode: AlphaMode::Premultiplied,
            data,
        })
    }
}

/// A buffer proven to be in the document 21 working space: linear light, premultiplied RGB.
///
/// The only constructor is [`ImageBuffer::into_working`].
#[derive(Clone, Debug)]
pub struct WorkingBuffer(ImageBuffer);

impl WorkingBuffer {
    pub fn transparent(width: usize, height: usize) -> Self {
        WorkingBuffer(ImageBuffer::transparent(
            width,
            height,
            ColorSpace::LinearLight,
            AlphaMode::Premultiplied,
        ))
    }

    /// Opaque white everywhere: D-66's adjustment layer shape before its mask.
    pub fn opaque(width: usize, height: usize) -> Self {
        let mut buffer = Self::transparent(width, height);
        buffer.data_mut().fill(1.0);
        buffer
    }

    pub fn as_image(&self) -> &ImageBuffer {
        &self.0
    }
    pub fn width(&self) -> usize {
        self.0.width
    }
    pub fn height(&self) -> usize {
        self.0.height
    }
    pub fn data(&self) -> &[f32] {
        &self.0.data
    }
    pub fn data_mut(&mut self) -> &mut [f32] {
        &mut self.0.data
    }

    /// D-910: `run` given this picture as linear Rec.709 numbers, for a rule written for them,
    /// and the picture taken back to the working space `working` after. Just `run` when
    /// `working` is `None` (colour management off, or working in Rec.709).
    pub fn in_rec709<R>(
        &mut self,
        working: Option<color::Gamut>,
        run: impl FnOnce(&mut Self) -> R,
    ) -> R {
        let Some(g) = working else {
            return run(self);
        };
        color::convert_rgba(self.data_mut(), g, color::Gamut::Rec709);
        let r = run(self);
        color::convert_rgba(self.data_mut(), color::Gamut::Rec709, g);
        r
    }

    /// D-910: this picture through the matrix from `from` to `to`, shared when nothing changes.
    pub fn converted(
        picture: std::sync::Arc<Self>,
        from: color::Gamut,
        to: color::Gamut,
    ) -> std::sync::Arc<Self> {
        if from == to {
            return picture;
        }
        let mut b = std::sync::Arc::unwrap_or_clone(picture);
        color::convert_rgba(b.data_mut(), from, to);
        std::sync::Arc::new(b)
    }
    pub fn pixel(&self, x: usize, y: usize) -> [f32; 4] {
        self.0.pixel(x, y)
    }

    /// Encode for output: unpremultiply in linear light, apply the output transfer function,
    /// then quantise. Document 21 line 31: output "converts the linear working RGB to the
    /// declared output encoding, then writes straight alpha".
    pub fn to_srgb8_straight(&self) -> Vec<u8> {
        perf::time(perf::Stage::Encode, || {
            self.encode(OutputDepth::Eight, OutputAlpha::Straight)
        })
    }

    /// [`to_srgb8_straight`](Self::to_srgb8_straight) at either depth and either alpha policy,
    /// which is what R-09 means by "chosen bit depth ... and alpha policy".
    ///
    /// The samples come out in PNG's order: RGBA per pixel, and at sixteen bits each sample is
    /// two bytes, most significant first, which is what the PNG specification requires.
    /// Straight is the default everywhere and the only thing document 21 line 31 states; the
    /// premultiplied path exists because R-09 offers the choice, and it differs by exactly one
    /// step, the unpremultiply that is not done.
    pub fn encode(&self, depth: OutputDepth, alpha: OutputAlpha) -> Vec<u8> {
        let bytes = match depth {
            OutputDepth::Eight => 1,
            OutputDepth::Sixteen => 2,
        };
        // One pixel in, four samples out, and nothing a pixel does depends on any other pixel:
        // the whole encode is a map, so it is one across the thread pool (P-10). The arithmetic
        // per pixel is exactly what it was when this was a `for` loop - the same unpremultiply,
        // the same `linear_to_srgb` on the same three channels, the same quantiser - because the
        // question this stage answers is what colour a pixel is, and that may not depend on which
        // thread asked. The destination is allocated once and split into one disjoint piece per
        // pixel, so no two threads are ever handed the same byte.
        let mut out = vec![0u8; self.0.data.len() * bytes];
        out.par_chunks_mut(4 * bytes)
            .zip(self.0.data.par_chunks_exact(4))
            .for_each(|(dst, px)| {
                let px = match alpha {
                    OutputAlpha::Straight => composite::unpremultiply([px[0], px[1], px[2], px[3]]),
                    OutputAlpha::Premultiplied => [px[0], px[1], px[2], px[3]],
                };
                let sample = [
                    color::linear_to_srgb(px[0]),
                    color::linear_to_srgb(px[1]),
                    color::linear_to_srgb(px[2]),
                    px[3],
                ];
                for (index, &c) in sample.iter().enumerate() {
                    match depth {
                        OutputDepth::Eight => dst[index] = color::quantise_u8(c),
                        OutputDepth::Sixteen => {
                            dst[index * 2..index * 2 + 2]
                                .copy_from_slice(&color::quantise_u16(c).to_be_bytes());
                        }
                    }
                }
            });
        out
    }
}
