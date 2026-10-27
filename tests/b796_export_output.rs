//! B-796, D-917: the export's Output with colour management on (ADR-023 step C6). Writes
//! `verification/B-796_export_output_table.md` and the pictures in `verification/B-796 pictures/`.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build): every level
//! and number is OpenColorIO 2.5.1's, written by `tools/output_reference.py` into
//! `Fixtures/colour/output/expected_output.json` before this build's code (FX-COLOUT-001 to 006,
//! document 25). Every check reads the exported file back. The card is judged against this
//! build's processor, as every card table is.

use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::export::{
    export_sequence, ExportChoices, ExportReport, ExportRequest, MissingSource, Output, OutputFormat,
};
use anime_compositor::exr_io::ExrSamples;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::{persist, OutputAlpha, OutputDepth};
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn dir() -> PathBuf {
    repo("Fixtures/colour/output")
}

const W: usize = 264;
const H: usize = 4;

/// One export of `project` into a folder of its own; the report and the folder.
fn export(
    project: &str,
    name: &str,
    format: OutputFormat,
    depth: OutputDepth,
    alpha: OutputAlpha,
    choices: ExportChoices,
) -> (ExportReport, PathBuf) {
    let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("b796").join(name);
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();
    let loaded = persist::load(&dir().join(project))
        .unwrap_or_else(|d| panic!("{project}: {} {}", d.message, d.detail));
    let one = !format.is_one_file();
    let request = ExportRequest {
        composition: Id::new("comp-main"),
        first_frame: 0,
        last_frame: if one { 0 } else { 1 },
        output_dir: out.clone(),
        naming: match format {
            OutputFormat::Png => "f_%04d.png",
            OutputFormat::Exr(_) => "f_%04d.exr",
            OutputFormat::Gif => "film.gif",
            OutputFormat::Apng => "film.png",
            OutputFormat::Mp4 => "film.mp4",
        }
        .to_string(),
        depth,
        alpha,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::Block,
        format,
        choices,
    };
    // The chart is 4 rows; Windows' video encoder takes no picture that small, and the colr label
    // does not hang on the size, so an MP4 is written 64 rows high (the chart, then nothing).
    let mut project = loaded.document.project().clone();
    if format == OutputFormat::Mp4 {
        project.compositions.iter_mut().for_each(|c| c.height = 64);
    }
    let report = export_sequence(&project, &dir(), &request, &AtomicBool::new(false));
    (report, out)
}

fn with(output: Output) -> ExportChoices {
    ExportChoices { output, ..ExportChoices::default() }
}

/// A PNG's chunks, in order: four letters and contents.
fn chunks(bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    let mut at = 8;
    while at + 8 <= bytes.len() {
        let n = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
        let name = String::from_utf8_lossy(&bytes[at + 4..at + 8]).to_string();
        out.push((name, bytes[at + 8..at + 8 + n].to_vec()));
        at += 12 + n;
    }
    out
}

/// The colour chunks a PNG carries, as `labels` spells them; "none" for none.
fn colour_chunks(bytes: &[u8]) -> String {
    let said: Vec<String> = chunks(bytes)
        .into_iter()
        .filter(|(n, _)| ["sRGB", "cICP", "iCCP", "gAMA", "cHRM", "mDCv", "cLLi"].contains(&n.as_str()))
        .map(|(n, d)| format!("{n} {}", d.iter().map(u8::to_string).collect::<Vec<_>>().join(" ")))
        .collect();
    if said.is_empty() {
        "none".into()
    } else {
        said.join(", ")
    }
}

/// The text of a PNG's iTXt `ColorSpace` tag.
fn colour_tag(bytes: &[u8]) -> String {
    for (n, d) in chunks(bytes) {
        if n == "iTXt" && d.starts_with(b"ColorSpace\0") {
            // keyword \0 flag method language \0 translated \0 text
            let rest = &d[b"ColorSpace\0".len() + 2..];
            let mut parts = rest.splitn(3, |b| *b == 0);
            let _ = (parts.next(), parts.next());
            return String::from_utf8_lossy(parts.next().unwrap_or(&[])).to_string();
        }
    }
    "no tag".into()
}

/// A PNG's (first frame's) samples as levels, RGBA per pixel.
fn levels(bytes: &[u8]) -> Vec<[u32; 4]> {
    let mut decoder = png::Decoder::new(std::io::Cursor::new(bytes)).read_info().unwrap();
    let mut buf = vec![0u8; decoder.output_buffer_size().unwrap()];
    let info = decoder.next_frame(&mut buf).unwrap();
    let buf = &buf[..info.buffer_size()];
    match info.bit_depth {
        png::BitDepth::Sixteen => buf
            .chunks_exact(8)
            .map(|p| std::array::from_fn(|c| u16::from_be_bytes([p[2 * c], p[2 * c + 1]]) as u32))
            .collect(),
        _ => buf.chunks_exact(4).map(|p| std::array::from_fn(|c| p[c] as u32)).collect(),
    }
}

/// An MP4's `colr` box: primaries, transfer, matrix, full range.
fn mp4_colr(file: &Path) -> Vec<u32> {
    let bytes = std::fs::read(file).unwrap();
    let Some(at) = bytes.windows(8).rposition(|w| w == b"colrnclx") else {
        return vec![];
    };
    let n = |i: usize| u16::from_be_bytes([bytes[at + 8 + i], bytes[at + 9 + i]]) as u32;
    vec![n(0), n(2), n(4), (bytes[at + 14] >> 7) as u32]
}

/// An EXR's RGBA and its chromaticities.
fn exr(file: &Path) -> (Vec<[f32; 4]>, Vec<[f32; 2]>) {
    let image = exr::prelude::read_first_rgba_layer_from_file(
        file,
        |size, _| (size.width(), vec![[0f32; 4]; size.width() * size.height()]),
        |(w, px): &mut (usize, Vec<[f32; 4]>), at, (r, g, b, a): (f32, f32, f32, f32)| {
            px[at.y() * *w + at.x()] = [r, g, b, a];
        },
    )
    .unwrap();
    let c = image.attributes.chromaticities.expect("chromaticities");
    let xy = |v: exr::math::Vec2<f32>| [v.0, v.1];
    (
        image.layer_data.channel_data.pixels.1,
        vec![xy(c.red), xy(c.green), xy(c.blue), xy(c.white)],
    )
}

fn project_for(working: &str) -> &'static str {
    match working {
        "acescg" => "chart_acescg.json",
        _ => "chart_rec709.json",
    }
}

struct Table {
    checks: Vec<(String, String, String)>,
    cases: Vec<String>,
    card: Vec<String>,
}

impl Table {
    fn check(&mut self, what: &str, expected: impl ToString, actual: impl ToString) {
        self.checks.push((what.to_string(), expected.to_string(), actual.to_string()));
    }
}

#[test]
fn b796_export_output() {
    let expected: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("expected_output.json")).unwrap()).unwrap();
    let mut t = Table { checks: Vec::new(), cases: Vec::new(), card: Vec::new() };
    let png0 = |out: &Path| std::fs::read(out.join("f_0000.png")).unwrap();

    // ---- FX-COLOUT-001: the picture outputs against OpenColorIO -----------------------------
    for case in expected["cases"].as_array().unwrap() {
        let working = case["working"].as_str().unwrap();
        let word = case["output"].as_str().unwrap();
        let output = Output::from_word(word).unwrap();
        let aces = output == Output::AcesView;
        for (key, depth, alpha, top) in [
            ("eight_straight", OutputDepth::Eight, OutputAlpha::Straight, 255u32),
            ("sixteen_straight", OutputDepth::Sixteen, OutputAlpha::Straight, 65535),
            ("eight_premultiplied", OutputDepth::Eight, OutputAlpha::Premultiplied, 255),
        ] {
            let (report, out) = export(
                project_for(working),
                &format!("{working}_{word}_{key}"),
                OutputFormat::Png,
                depth,
                alpha,
                with(output),
            );
            assert!(report.succeeded(), "{working} {word} {key}: {:?}", report.diagnostics);
            let ours = levels(&png0(&out));
            let want: Vec<[u32; 4]> = case[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|p| std::array::from_fn(|c| p[c].as_u64().unwrap() as u32))
                .collect();
            let one = if top == 255 { 1 } else { 257 };
            let (mut largest, mut within) = (0u32, 0usize);
            for (a, b) in ours.iter().zip(&want) {
                let d = (0..4).map(|c| a[c].abs_diff(b[c])).max().unwrap();
                largest = largest.max(d);
                within += (d <= one) as usize;
            }
            // sRGB and Rec.709: 1 level everywhere (1 of 65535 at 16 bits); the ACES view: 1 of
            // 255 on 99.9% of pixels.
            let pass = ours.len() == W * H
                && if aces {
                    within as f64 >= 0.999 * (W * H) as f64
                } else {
                    let exact_one = 1;
                    ours.iter().zip(&want).all(|(a, b)| (0..4).all(|c| a[c].abs_diff(b[c]) <= exact_one))
                };
            t.cases.push(format!(
                "| {working} | {} | {key} | {} | {largest} of {top} | {within} of {} within {one} | {} |",
                output.name(OutputFormat::Png),
                ours.len(),
                W * H,
                if pass { "pass" } else { "FAIL" }
            ));
            t.check(
                &format!("FX-COLOUT-001: {working}, {}, {key}, within the bar", output.name(OutputFormat::Png)),
                "true",
                pass,
            );
        }
    }

    // ---- FX-COLOUT-002: the EXR outputs and their chromaticities -----------------------------
    for case in expected["exr"].as_array().unwrap() {
        let working = case["working"].as_str().unwrap();
        let word = case["output"].as_str().unwrap();
        let output = Output::from_word(word).unwrap();
        let (report, out) = export(
            project_for(working),
            &format!("exr_{working}_{word}"),
            OutputFormat::Exr(ExrSamples::Float),
            OutputDepth::Eight,
            OutputAlpha::Straight,
            with(output),
        );
        assert!(report.succeeded(), "{working} {word}: {:?}", report.diagnostics);
        let (ours, chroma) = exr(&out.join("f_0000.exr"));
        let mut worst = 0f64;
        let mut alpha_exact = true;
        for (a, b) in ours.iter().zip(case["rgba"].as_array().unwrap()) {
            let b: [f64; 4] = std::array::from_fn(|c| b[c].as_f64().unwrap());
            let size = b[..3].iter().fold(1f64, |m, v| m.max(v.abs()));
            for c in 0..3 {
                worst = worst.max((a[c] as f64 - b[c]).abs() / size);
            }
            alpha_exact &= a[3] == b[3] as f32;
        }
        let want_chroma: Vec<[f32; 2]> = expected["labels"]["exr"][working][word]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| [p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32])
            .collect();
        let pass = ours.len() == W * H && worst <= 1e-6 && alpha_exact;
        t.cases.push(format!(
            "| {working} | EXR {} | float | {} | {worst:.2e} of the colour's size | alpha exact: {alpha_exact} | {} |",
            output.name(OutputFormat::Exr(ExrSamples::Float)),
            ours.len(),
            if pass { "pass" } else { "FAIL" }
        ));
        t.check(
            &format!("FX-COLOUT-002: {working}, EXR {word}, numbers within 1e-6, alpha exact"),
            "true",
            pass,
        );
        t.check(
            &format!("FX-COLOUT-002: {working}, EXR {word}, chromaticities"),
            format!("{want_chroma:?}"),
            format!("{chroma:?}"),
        );
    }

    // ---- FX-COLOUT-003: the labels ------------------------------------------------------------
    let labels = &expected["labels"];
    for working in ["acescg", "linear-srgb"] {
        for output in [Output::Default, Output::Rec709, Output::AcesView] {
            let word = output.word();
            let want = &labels["picture"][word];
            let (_, out) = export(
                project_for(working),
                &format!("label_{working}_{word}"),
                OutputFormat::Png,
                OutputDepth::Eight,
                OutputAlpha::Straight,
                with(output),
            );
            let png = png0(&out);
            t.check(
                &format!("FX-COLOUT-003: {working}, {word}: the PNG's colour chunk"),
                want["png_chunk"].as_str().unwrap(),
                colour_chunks(&png),
            );
            t.check(
                &format!("FX-COLOUT-003: {working}, {word}: the PNG's ColorSpace tag"),
                format!("{}, 8 bits per channel", want["colour_tag"].as_str().unwrap()),
                colour_tag(&png),
            );
            let (report, film) = export(
                project_for(working),
                &format!("apng_{working}_{word}"),
                OutputFormat::Apng,
                OutputDepth::Eight,
                OutputAlpha::Straight,
                with(output),
            );
            assert!(report.succeeded(), "{:?}", report.diagnostics);
            let apng = std::fs::read(film.join("film.png")).unwrap();
            t.check(
                &format!("FX-COLOUT-003: {working}, {word}: the animated PNG's chunk, and its first frame is the PNG's"),
                format!("{}, true", want["png_chunk"].as_str().unwrap()),
                format!("{}, {}", colour_chunks(&apng), levels(&apng) == levels(&png)),
            );
            let (report, film) = export(
                project_for(working),
                &format!("mp4_{working}_{word}"),
                OutputFormat::Mp4,
                OutputDepth::Eight,
                OutputAlpha::Straight,
                with(output),
            );
            t.check(
                &format!("FX-COLOUT-003: {working}, {word}: the MP4's colr (primaries, transfer, matrix, full range)"),
                format!("{:?}", want["mp4_colr"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect::<Vec<_>>()),
                if report.succeeded() {
                    format!("{:?}", mp4_colr(&film.join("film.mp4")))
                } else {
                    format!("not written: {:?}", report.diagnostics.iter().map(|d| &d.message).collect::<Vec<_>>())
                },
            );
            let (report, _) = export(
                project_for(working),
                &format!("gif_{working}_{word}"),
                OutputFormat::Gif,
                OutputDepth::Eight,
                OutputAlpha::Straight,
                with(output),
            );
            t.check(&format!("FX-COLOUT-003: {working}, {word}: a GIF is written"), "true", report.succeeded());
        }
    }

    // ---- FX-COLOUT-004: the refusals ------------------------------------------------------------
    let refused = |project: &str, name: &str, format: OutputFormat, output: Output| {
        let (report, out) = export(project, name, format, OutputDepth::Eight, OutputAlpha::Straight, with(output));
        let d = report.diagnostics.first();
        format!(
            "{}, {}, {}, {}",
            report.succeeded(),
            d.map_or("none", |d| d.id.as_str()),
            d.is_some_and(|d| d.message.contains(output.word())),
            std::fs::read_dir(&out).unwrap().count()
        )
    };
    let no = format!("false, {}, true, 0", DiagnosticId::CommandInvalidValue.as_str());
    t.check(
        "FX-COLOUT-004: acescg for a PNG is refused, naming it, nothing written",
        &no,
        refused("chart_acescg.json", "refuse_png", OutputFormat::Png, Output::AcesCg),
    );
    t.check(
        "FX-COLOUT-004: rec709 for an EXR is refused, naming it, nothing written",
        &no,
        refused("chart_acescg.json", "refuse_exr", OutputFormat::Exr(ExrSamples::Half), Output::Rec709),
    );
    for output in [Output::Rec709, Output::AcesView] {
        t.check(
            &format!("FX-COLOUT-004: {} with colour management off is refused, nothing written", output.word()),
            &no,
            refused("chart_off.json", &format!("refuse_off_{}", output.word()), OutputFormat::Png, output),
        );
    }
    t.check(
        "FX-COLOUT-004: acescg for an EXR with colour management off is refused",
        &no,
        refused("chart_off.json", "refuse_off_exr", OutputFormat::Exr(ExrSamples::Float), Output::AcesCg),
    );

    // ---- FX-COLOUT-005: off is today ------------------------------------------------------------
    let off = &labels["off"];
    let (_, out) = export("chart_off.json", "off_png", OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, ExportChoices::default());
    let off_png = png0(&out);
    t.check("FX-COLOUT-005: off, a PNG carries no colour chunk", "none", colour_chunks(&off_png));
    t.check(
        "FX-COLOUT-005: off, the PNG's ColorSpace tag is today's",
        format!("{}, 8 bits per channel", off["colour_tag"].as_str().unwrap()),
        colour_tag(&off_png),
    );
    let (_, out) = export("chart_rec709.json", "on_rec709_default", OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, ExportChoices::default());
    t.check(
        "FX-COLOUT-005: off, the PNG's pixels are those of colour management on in linear Rec.709 at sRGB (today's pixels)",
        "true",
        levels(&off_png) == levels(&png0(&out)),
    );
    let (_, out) = export("chart_off.json", "off_apng", OutputFormat::Apng, OutputDepth::Sixteen, OutputAlpha::Straight, ExportChoices::default());
    t.check("FX-COLOUT-005: off, an animated PNG carries no colour chunk", "none", colour_chunks(&std::fs::read(out.join("film.png")).unwrap()));
    let (report, out) = export("chart_off.json", "off_mp4", OutputFormat::Mp4, OutputDepth::Eight, OutputAlpha::Straight, ExportChoices::default());
    t.check(
        "FX-COLOUT-005: off, the MP4's colr is today's",
        format!("{:?}", off["mp4_colr"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect::<Vec<_>>()),
        if report.succeeded() { format!("{:?}", mp4_colr(&out.join("film.mp4"))) } else { "not written".into() },
    );
    let (_, out) = export("chart_off.json", "off_exr", OutputFormat::Exr(ExrSamples::Float), OutputDepth::Eight, OutputAlpha::Straight, ExportChoices::default());
    let want_chroma: Vec<[f32; 2]> = labels["exr"]["off"]["default"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| [p[0].as_f64().unwrap() as f32, p[1].as_f64().unwrap() as f32])
        .collect();
    t.check("FX-COLOUT-005: off, the EXR's chromaticities are Rec.709's", format!("{want_chroma:?}"), format!("{:?}", exr(&out.join("f_0000.exr")).1));

    // ---- FX-COLOUT-006: the card ------------------------------------------------------------------
    let about = match Gpu::new() {
        Ok(gpu) => {
            let about = gpu.about().to_string();
            drop(gpu);
            for output in [Output::Default, Output::Rec709, Output::AcesView] {
                let word = output.word();
                let (cpu, out_cpu) = export("chart_rec709.json", &format!("cpu_{word}"), OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, with(output));
                let (card, out_card) = export(
                    "chart_rec709.json",
                    &format!("card_{word}"),
                    OutputFormat::Png,
                    OutputDepth::Eight,
                    OutputAlpha::Straight,
                    ExportChoices { on_card: true, output, ..ExportChoices::default() },
                );
                let drew = card
                    .diagnostics
                    .iter()
                    .find(|d| d.message.starts_with("Exported on the graphics card"))
                    .map_or("not said".to_string(), |d| d.message.clone());
                let (a, b) = (levels(&png0(&out_cpu)), levels(&png0(&out_card)));
                let largest = a.iter().zip(&b).map(|(a, b)| (0..4).map(|c| a[c].abs_diff(b[c])).max().unwrap()).max().unwrap_or(99);
                let pass = cpu.succeeded() && card.succeeded() && largest <= 1;
                t.card.push(format!(
                    "| {} | {} | {largest} | {} |",
                    output.name(OutputFormat::Png),
                    drew.split(':').nth(1).unwrap_or(&drew).split('.').next().unwrap_or("").trim(),
                    if pass { "pass" } else { "FAIL" }
                ));
                t.check(
                    &format!("FX-COLOUT-006: the card with {word}: written, nothing refused, within 1 level of the processor"),
                    "true, true, true",
                    format!("{}, {}, {}", card.succeeded(), colour_chunks(&png0(&out_card)) == colour_chunks(&png0(&out_cpu)), largest <= 1),
                );
            }
            about
        }
        Err(e) => {
            t.check("FX-COLOUT-006: a graphics card to check on", "a card", format!("none: {e}"));
            "none".to_string()
        }
    };

    pictures(&mut t);
    write(&t, &about);
    let failed: Vec<&String> = t.checks.iter().filter(|(_, e, a)| e != a).map(|(c, _, _)| c).collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// The chart from the ACEScg project as each Output writes it, and as the build wrote it before
/// D-917 (the ACEScg numbers put through the sRGB curve), each row blown up for the owner.
fn pictures(t: &mut Table) {
    let out = repo("verification/B-796 pictures");
    std::fs::create_dir_all(&out).unwrap();
    let save = |name: &str, px: &[[u32; 4]]| {
        let (kx, ky) = (3, 30);
        let mut big = Vec::with_capacity(W * kx * H * ky * 4);
        for y in 0..H * ky {
            for x in 0..W * kx {
                big.extend(px[(y / ky) * W + x / kx].map(|v| v as u8));
            }
        }
        let file = std::fs::File::create(out.join(name)).unwrap();
        let mut enc = png::Encoder::new(std::io::BufWriter::new(file), (W * kx) as u32, (H * ky) as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().unwrap().write_image_data(&big).unwrap();
    };
    for output in [Output::Default, Output::Rec709, Output::AcesView] {
        let (_, dir) = export("chart_acescg.json", &format!("pic_{}", output.word()), OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, with(output));
        save(&format!("acescg_{}.png", output.word()), &levels(&std::fs::read(dir.join("f_0000.png")).unwrap()));
    }
    // Before D-917: the ACEScg numbers through the sRGB curve, as if they were Rec.709.
    let (_, dir) = export("chart_acescg.json", "pic_before", OutputFormat::Exr(ExrSamples::Float), OutputDepth::Eight, OutputAlpha::Straight, ExportChoices::default());
    let before: Vec<[u32; 4]> = exr(&dir.join("f_0000.exr"))
        .0
        .iter()
        .map(|p| {
            let s = |c: f32| if p[3] > 0.0 { c / p[3] } else { 0.0 };
            [0, 1, 2, 3].map(|c| {
                let v = if c == 3 { p[3] } else { anime_compositor::color::linear_to_srgb(s(p[c])) };
                anime_compositor::color::quantise_u8(v) as u32
            })
        })
        .collect();
    save("acescg_before_D-917.png", &before);
    t.check("the pictures are written", "true", out.join("acescg_before_D-917.png").is_file());
}

fn write(t: &Table, about: &str) {
    let passed = t.checks.iter().filter(|(_, e, a)| e == a).count();
    let mut s = String::new();
    s.push_str("# B-796: the export's Output, checked against OpenColorIO\n\n");
    s.push_str(
        "Written by `tests/b796_export_output.rs` (D-917). The expected values are OpenColorIO \
         2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless), \
         written by `tools/output_reference.py` into `Fixtures/colour/output/expected_output.json` \
         before the code. Every check reads the exported file back. The bar: sRGB and Rec.709 \
         within 1 level on every channel of every pixel, at 8 and 16 bits; the ACES 2.0 SDR view \
         within 1 level of 255 (257 of 65535) on 99.9% of pixels; EXR numbers within 1e-6 of the \
         colour's size, alpha exact. With colour management off every existing export suite is \
         unchanged (run beside this test).\n\n",
    );
    s.push_str(&format!("**{passed} of {} checks pass.**\n\n", t.checks.len()));
    s.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (c, e, a) in &t.checks {
        s.push_str(&format!("| {c} | {e} | {a} | {} |\n", if e == a { "pass" } else { "FAIL" }));
    }
    s.push_str(
        "\n## Each Output against OpenColorIO, case by case\n\n\
         `media/output_chart.exr` (264 by 4: a white ramp to 64, fire to 64, the ColorChecker from \
         4 stops under to 6 over, an orange with alpha 0 to 1) exported from each working space. \
         \"Largest\" is the largest difference on any channel of any pixel.\n\n\
         | Working space | Output | Samples | Pixels | Largest | Within | Result |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for row in &t.cases {
        s.push_str(row);
        s.push('\n');
    }
    s.push_str(&format!(
        "\n## The card against the processor\n\nCard: {about}. `chart_rec709.json`, 8 bits, straight \
         alpha, exported with the tick \"Faster on the graphics card\" and each picture Output, \
         against the same export on the processor.\n\n\
         | Output | What the export said | Largest difference (levels) | Result |\n|---|---|---|---|\n"
    ));
    for row in &t.card {
        s.push_str(row);
        s.push('\n');
    }
    std::fs::write(repo("verification/B-796_export_output_table.md"), s).unwrap();
}
