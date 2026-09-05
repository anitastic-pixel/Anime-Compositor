//! B-05b: render trace mode, ADR-012. Every intermediate layer buffer written as a tagged PNG.
//!
//! Document 25 catalogues no trace fixture, because trace mode is a diagnostic facility rather
//! than a rendering rule. The fixture here is therefore built and derived in this file, and
//! every expected value below is a literal worked out by hand from document 21's compositing
//! and encoding rules. None was captured from a run.
//!
//! The two-layer fixture, in the document 21 working space (linear light, premultiplied):
//!
//! - layer 0, ID `bg`: 4x4, every pixel `(0.2, 0.4, 0.6, 1.0)`, identity transform, opacity 1.
//! - layer 1, ID `fg`: 2x2, every pixel `(0.6, 0.0, 0.0, 1.0)`, translated by `(1, 1)`,
//!   opacity 0.5. The translation is a whole number of pixels, so document 21's bilinear taps
//!   land exactly on source pixel centres and the layer is moved without being resampled.
//!
//! Everything the trace writes follows from those two lines:
//!
//! - `fg` covers destination pixels `(1,1)`, `(2,1)`, `(1,2)`, `(2,2)` and nothing else.
//! - `fg` after opacity is `(0.3, 0.0, 0.0, 0.5)`: premultiplied, so RGB and alpha halve
//!   together.
//! - the composite at `(1,1)` is `src + dst * (1 - src_alpha)`
//!   `= (0.3, 0, 0, 0.5) + (0.2, 0.4, 0.6, 1.0) * 0.5 = (0.4, 0.2, 0.3, 1.0)`.
//! - the composite at `(0,0)` is untouched background, `(0.2, 0.4, 0.6, 1.0)`.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::model::Id;
use anime_compositor::render::{render, Affine, FramePlan, LayerDraw};
use anime_compositor::trace::{missing_stages, render_traced, Stage, TraceRequest};
use anime_compositor::{AlphaMode, ColorSpace, ImageBuffer, WorkingBuffer};

// -- reporting ------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
    notes: Vec<String>,
}

impl Report {
    fn check(&mut self, check: impl Into<String>, expected: impl Into<String>, actual: String) {
        self.rows.push(Row {
            check: check.into(),
            expected: expected.into(),
            actual,
        });
    }
    fn note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }
    fn passed(&self) -> usize {
        self.rows.iter().filter(|r| r.pass()).count()
    }
}

fn repo(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

// -- the fixture ----------------------------------------------------------------------------

/// The working space is linear premultiplied. `into_working` is the only constructor, so a
/// buffer tagged as already being in that space passes through it untouched.
fn working(width: usize, height: usize, pixel: [f32; 4]) -> WorkingBuffer {
    let data: Vec<f32> = std::iter::repeat_n(pixel, width * height)
        .flatten()
        .collect();
    ImageBuffer::new(
        width,
        height,
        ColorSpace::LinearLight,
        AlphaMode::Premultiplied,
        data,
    )
    .expect("extent matches data")
    .into_working()
}

const BG: [f32; 4] = [0.2, 0.4, 0.6, 1.0];
const FG: [f32; 4] = [0.6, 0.0, 0.0, 1.0];

fn fixture_plan() -> FramePlan {
    FramePlan {
        width: 4,
        height: 4,
        layers: vec![
            LayerDraw {
                id: Id::new("bg"),
                source: working(4, 4, BG),
                transform: Affine::IDENTITY,
                opacity: 1.0,
            },
            LayerDraw {
                id: Id::new("fg"),
                source: working(2, 2, FG),
                transform: Affine::translation(1.0, 1.0),
                opacity: 0.5,
            },
        ],
    }
}

fn px(p: [f32; 4]) -> String {
    format!("({:.6}, {:.6}, {:.6}, {:.6})", p[0], p[1], p[2], p[3])
}

fn at(buffer: &WorkingBuffer, x: usize, y: usize) -> String {
    px(buffer.pixel(x, y))
}

/// Decode a written trace PNG back to 8-bit RGBA plus its iTXt tags.
fn read_png(path: &Path) -> (usize, usize, Vec<u8>, Vec<(String, String)>) {
    let decoder = png::Decoder::new(std::io::BufReader::new(
        fs::File::open(path).expect("open trace png"),
    ));
    let mut reader = decoder.read_info().expect("png info");
    let mut buf = vec![0u8; reader.output_buffer_size().expect("buffer size")];
    let info = reader.next_frame(&mut buf).expect("png frame");
    buf.truncate(info.buffer_size());
    let tags = reader
        .info()
        .utf8_text
        .iter()
        .map(|c| {
            (
                c.keyword.clone(),
                c.get_text().expect("itxt text is readable"),
            )
        })
        .collect();
    (info.width as usize, info.height as usize, buf, tags)
}

fn tag(tags: &[(String, String)], key: &str) -> String {
    tags.iter()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.clone())
        .unwrap_or_else(|| format!("<no {key} chunk>"))
}

fn rgba_at(width: usize, bytes: &[u8], x: usize, y: usize) -> String {
    let i = (y * width + x) * 4;
    format!(
        "({}, {}, {}, {})",
        bytes[i],
        bytes[i + 1],
        bytes[i + 2],
        bytes[i + 3]
    )
}

// -- the tests ------------------------------------------------------------------------------

#[test]
fn b05b_trace_fixtures() {
    let mut report = Report::default();
    let plan = fixture_plan();
    let dir = repo("target/b05b_trace");
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear trace dir");
    }

    let request = TraceRequest {
        dir: dir.clone(),
        frame: 7,
    };
    let (frame, written) = render_traced(&plan, 2, &request).expect("trace render");
    let out = dir.join("frame_00007");

    // ---- the anti-drift claim, which is the whole reason trace re-uses `render` -----------
    let untraced = render(&plan, 2);
    report.check(
        "the frame returned by render_traced is byte-identical to the untraced render",
        "identical",
        if frame.data() == untraced.data() {
            "identical"
        } else {
            "different"
        }
        .to_string(),
    );
    let last_composite = written
        .iter()
        .filter(|i| i.stage == Stage::Composite)
        .next_back()
        .expect("a composite stage for the top layer");
    let (fw, _, frame_bytes, _) = read_png(&out.join("frame.png"));
    let (_, _, top_bytes, _) = read_png(&last_composite.path);
    report.check(
        "the top layer's composite image is byte-identical to frame.png",
        "identical",
        if top_bytes == frame_bytes {
            "identical"
        } else {
            "different"
        }
        .to_string(),
    );

    // ---- exactly the expected files, and nothing invented --------------------------------
    let mut names: Vec<String> = fs::read_dir(&out)
        .expect("read trace dir")
        .map(|e| {
            e.expect("dir entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect();
    names.sort();
    report.check(
        "the trace directory holds exactly the eight stage images, the frame and the manifest",
        "frame.png, layer00_bg_composite.png, layer00_bg_decode.png, layer00_bg_opacity.png, \
         layer00_bg_transform.png, layer01_fg_composite.png, layer01_fg_decode.png, \
         layer01_fg_opacity.png, layer01_fg_transform.png, manifest.md",
        names.join(", "),
    );
    report.check(
        "no image claims a stage document 21 lists but this build does not implement",
        "no mask, effects or matte image",
        if names
            .iter()
            .any(|n| n.contains("mask") || n.contains("effect") || n.contains("matte"))
        {
            "found one".to_string()
        } else {
            "no mask, effects or matte image".to_string()
        },
    );

    // ---- extents: decode is layer space, everything after is composition space -----------
    let decode_fg = written
        .iter()
        .find(|i| i.stage == Stage::Decode && i.layer_index == 1)
        .expect("fg decode");
    report.check(
        "the fg decode image is at the layer's own 2x2 extent, not the composition's",
        "2x2",
        format!("{}x{}", decode_fg.width, decode_fg.height),
    );
    let transform_fg = written
        .iter()
        .find(|i| i.stage == Stage::Transform && i.layer_index == 1)
        .expect("fg transform");
    report.check(
        "every stage after the transform is at the 4x4 composition extent",
        "4x4",
        format!("{}x{}", transform_fg.width, transform_fg.height),
    );

    // ---- the stage buffers themselves, in the working space ------------------------------
    // Re-derived here from the plan rather than read back from the 8-bit files, because the
    // point of these rows is the arithmetic, not the encoding. The encoding gets its own rows.
    let fg_alone = render(
        &FramePlan {
            width: 4,
            height: 4,
            layers: vec![LayerDraw {
                opacity: 1.0,
                ..plan.layers[1].clone()
            }],
        },
        2,
    );
    report.check(
        "transform stage: fg lands on destination pixel (1,1) unresampled",
        px(FG),
        at(&fg_alone, 1, 1),
    );
    report.check(
        "transform stage: fg lands on destination pixel (2,2), the far corner of its 2x2",
        px(FG),
        at(&fg_alone, 2, 2),
    );
    report.check(
        "transform stage: an integer translation invents nothing at (0,0)",
        px([0.0, 0.0, 0.0, 0.0]),
        at(&fg_alone, 0, 0),
    );
    report.check(
        "transform stage: an integer translation invents nothing at (3,3)",
        px([0.0, 0.0, 0.0, 0.0]),
        at(&fg_alone, 3, 3),
    );

    let fg_faded = render(
        &FramePlan {
            width: 4,
            height: 4,
            layers: vec![plan.layers[1].clone()],
        },
        2,
    );
    report.check(
        "opacity stage: 0.5 halves RGB and alpha together, because the buffer is premultiplied",
        px([0.3, 0.0, 0.0, 0.5]),
        at(&fg_faded, 1, 1),
    );
    report.check(
        "opacity stage: opacity does not spread a layer beyond where it was",
        px([0.0, 0.0, 0.0, 0.0]),
        at(&fg_faded, 0, 0),
    );

    let bg_only = render(
        &FramePlan {
            width: 4,
            height: 4,
            layers: vec![plan.layers[0].clone()],
        },
        2,
    );
    report.check(
        "composite stage after layer 0 is the background alone",
        px(BG),
        at(&bg_only, 1, 1),
    );
    report.check(
        "composite stage after layer 1: 0.3 + 0.2*0.5 = 0.4 red, 0.4*0.5 = 0.2 green",
        px([0.4, 0.2, 0.3, 1.0]),
        at(&frame, 1, 1),
    );
    report.check(
        "composite stage after layer 1: a pixel fg does not cover keeps the background exactly",
        px(BG),
        at(&frame, 0, 0),
    );

    // ---- the encoding, stated in 8-bit output values -------------------------------------
    // Document 21: output unpremultiplies in linear light, applies the sRGB transfer function
    // and quantises. sRGB(c) = 1.055 * c^(1/2.4) - 0.055 above 0.0031308, and document 27's
    // quantiser rounds to nearest with ties away from zero, so v = floor(255*s + 0.5).
    //   linear 0.2 -> 0.484530 -> 255*0.484530 + 0.5 = 124.055 -> 124
    //   linear 0.4 -> 0.665185 -> 170.122                      -> 170
    //   linear 0.6 -> 0.797739 -> 203.923                      -> 203
    //   linear 0.3 -> 0.583832 -> 149.377                      -> 149
    report.check(
        "frame.png pixel (0,0), background alone: linear (0.2,0.4,0.6,1) encodes to sRGB 8-bit",
        "(124, 170, 203, 255)",
        rgba_at(fw, &frame_bytes, 0, 0),
    );
    report.check(
        "frame.png pixel (1,1), fg over bg: linear (0.4,0.2,0.3,1) encodes to sRGB 8-bit",
        "(170, 124, 149, 255)",
        rgba_at(fw, &frame_bytes, 1, 1),
    );
    // These two rows read the files the trace actually wrote, rather than re-deriving the
    // stage from the plan. Without them a trace that wrote the faded image under both stage
    // names would pass every arithmetic row above: the arithmetic would still be right, and
    // only the labelling would be a lie.
    let (tw, _, transform_bytes, _) = read_png(&out.join("layer01_fg_transform.png"));
    report.check(
        "the transform image is written before opacity, so fg is still fully opaque there",
        "(203, 0, 0, 255)",
        rgba_at(tw, &transform_bytes, 1, 1),
    );
    let (ow, _, opacity_bytes, opacity_tags) = read_png(&out.join("layer01_fg_opacity.png"));
    report.check(
        "the transform and opacity images are different files with different contents",
        "different",
        if transform_bytes == opacity_bytes {
            "identical"
        } else {
            "different"
        }
        .to_string(),
    );
    // Unpremultiply first: (0.3, 0, 0) / 0.5 = (0.6, 0, 0), which is the layer's own colour.
    // Alpha is quantised without a transfer function: floor(255*0.5 + 0.5) = 128.
    report.check(
        "the opacity image unpremultiplies before encoding, so fg keeps its colour at half alpha",
        "(203, 0, 0, 128)",
        rgba_at(ow, &opacity_bytes, 1, 1),
    );
    report.check(
        "a pixel the layer never covered writes fully transparent black",
        "(0, 0, 0, 0)",
        rgba_at(ow, &opacity_bytes, 0, 0),
    );

    // ---- the tags, which are what make a stray file still readable ------------------------
    report.check(
        "each image names its pipeline stage",
        "opacity",
        tag(&opacity_tags, "Stage"),
    );
    report.check(
        "each image names its layer",
        "fg",
        tag(&opacity_tags, "Layer"),
    );
    report.check(
        "each image names the document 21 step it belongs to",
        "6",
        tag(&opacity_tags, "Document21Step"),
    );
    report.check(
        "each image states its own colour space",
        "sRGB IEC 61966-2-1, 8 bits per channel",
        tag(&opacity_tags, "ColorSpace"),
    );
    report.check(
        "each image states its own alpha mode",
        "Straight",
        tag(&opacity_tags, "AlphaMode"),
    );
    report.check(
        "each image states what it was converted from, so the conversion is not silent",
        "converted from linear light, premultiplied, float32",
        tag(&opacity_tags, "WorkingSpace"),
    );
    let (_, _, _, frame_tags) = read_png(&out.join("frame.png"));
    report.check(
        "the finished frame carries the composition frame number it was traced at",
        "7",
        tag(&frame_tags, "Frame"),
    );

    // ---- the manifest --------------------------------------------------------------------
    let manifest = fs::read_to_string(out.join("manifest.md")).expect("manifest");
    report.check(
        "the manifest names the composition frame",
        "names frame 7",
        if manifest.contains("composition frame 7") {
            "names frame 7"
        } else {
            "does not"
        }
        .to_string(),
    );
    let missing_named = missing_stages()
        .iter()
        .filter(|(_, name, _)| manifest.contains(name))
        .count();
    report.check(
        "the manifest names every stage of document 21's order this build does not implement",
        "4 of 4",
        format!("{missing_named} of 4"),
    );
    report.check(
        "the manifest carries the exact layer IDs, not just the file names",
        "both",
        if manifest.contains("`bg`") && manifest.contains("`fg`") {
            "both"
        } else {
            "not both"
        }
        .to_string(),
    );

    report.note(
        "Every expected value above is derived from the two fixture layers stated in the header \
         of `tests/b05b_trace.rs`, by applying document 21's compositing and output-encoding \
         rules by hand. The 8-bit values are the sRGB transfer function evaluated to six \
         decimal places and quantised with document 27's rounding rule.",
    );
    report.note(
        "The tile size for this fixture is 2, so the 4x4 composition is cut into four tiles and \
         every stage image crosses tile seams. B-05a already proved tiling is invisible; this \
         fixture would still catch a trace facility that assembled tiles differently.",
    );

    write_fixture_artifact(&report);
    assert_report(&report, "B-05b trace fixtures");
}

/// ADR-012: trace "is never on by default". The check is that the ordinary entry point cannot
/// write a file at all, so the guarantee does not depend on a flag being set correctly.
#[test]
fn b05b_a_plain_render_writes_nothing() {
    let dir = repo("target/b05b_never_on");
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear");
    }
    let plan = fixture_plan();
    let _ = render(&plan, 2);
    assert!(
        !dir.exists(),
        "an untraced render created a directory it was never given"
    );
}

/// The reference shot contains `layer2_桜_013.png`, a required fixture with a Japanese file
/// name, so a non-ASCII layer ID is a real case here and not a hypothetical one. The file name
/// is reduced to something every filesystem accepts; the ID itself survives in the manifest and
/// in the PNG's own iTXt chunk, which is why the tags are iTXt and not Latin-1 tEXt.
#[test]
fn b05b_a_unicode_layer_id_survives_the_round_trip() {
    let dir = repo("target/b05b_unicode");
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear");
    }
    let id = Id::new("桜 / レイヤー 2");
    let plan = FramePlan {
        width: 2,
        height: 2,
        layers: vec![LayerDraw {
            id: id.clone(),
            source: working(2, 2, BG),
            transform: Affine::IDENTITY,
            opacity: 1.0,
        }],
    };
    let (_, written) = render_traced(
        &plan,
        2,
        &TraceRequest {
            dir: dir.clone(),
            frame: 0,
        },
    )
    .expect("trace render");

    for image in &written {
        let name = image
            .path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        assert!(
            name.is_ascii(),
            "trace file name {name} is not portable across filesystems"
        );
        let (_, _, _, tags) = read_png(&image.path);
        assert_eq!(
            tag(&tags, "Layer"),
            id.as_str(),
            "the PNG lost the exact layer ID"
        );
    }
    let manifest =
        fs::read_to_string(dir.join("frame_00000").join("manifest.md")).expect("manifest");
    assert!(
        manifest.contains(id.as_str()),
        "the manifest lost the exact layer ID"
    );
}

/// The owner-facing artifact: a real frame of the reference shot, traced. Written under
/// `trace/`, which `.gitignore` already excludes, because ADR-012 says trace output is never
/// committed.
#[test]
fn b05b_traces_a_frame_of_the_reference_shot() {
    let Some(plan) = reference_plan() else {
        eprintln!("reference shot not available; skipping the trace artifact");
        return;
    };
    let dir = repo("trace");
    if dir.exists() {
        fs::remove_dir_all(&dir).expect("clear trace dir");
    }
    let (frame, written) = render_traced(
        &plan,
        128,
        &TraceRequest {
            dir: dir.clone(),
            frame: 0,
        },
    )
    .expect("trace render");
    assert_eq!(
        written.len(),
        plan.layers.len() * 4,
        "four stages for every layer"
    );
    assert!(
        frame.data() == render(&plan, 128).data(),
        "the traced frame differs from the untraced one"
    );
    write_reference_artifact(&plan, &written);
}

// -- the reference-shot plan ----------------------------------------------------------------

fn reference_plan() -> Option<FramePlan> {
    let placements = [
        ((0.0, 0.0), (0.0, 0.0), (1.0, 1.0), 0.0),
        ((960.0, 540.0), (480.0, 270.0), (0.4, 0.4), 0.0),
        ((960.0, 540.0), (1440.0, 270.0), (0.4, 0.4), 12.0),
        ((960.0, 540.0), (960.0, 810.0), (0.4, 0.4), 0.0),
    ];
    let mut layers = Vec::new();
    for (i, (anchor, position, scale, rotation)) in placements.iter().enumerate() {
        let name = format!("layer{}", i + 1);
        let source = decode_layer(&name, 0)?;
        layers.push(LayerDraw {
            id: Id::new(&name),
            source,
            transform: Affine::from_transform(*anchor, *position, *scale, *rotation),
            opacity: 1.0,
        });
    }
    Some(FramePlan {
        width: 1920,
        height: 1080,
        layers,
    })
}

/// The same decode path B-05a's test uses: import the sequence, hold drawing one across the
/// whole composition, resolve frame 0 and decode it into the working space.
fn decode_layer(name: &str, frame: i32) -> Option<WorkingBuffer> {
    use anime_compositor::media::import_sequence;
    use anime_compositor::time::{resolve, ExposureMap, ExposureSpan, LayerTiming, SourceAt};

    let dir = repo("Fixtures/reference_shot").join(name);
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .ok()?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let asset = import_sequence(&files).asset?;
    let first = asset.range()?.0;
    let exposures = ExposureMap::new(vec![ExposureSpan {
        start_frame: 0,
        end_frame_exclusive: 240,
        drawing_number: first,
    }])
    .ok()?;
    let timing = LayerTiming {
        in_frame: 0,
        out_frame: 240,
        source_offset_frames: 0,
    };
    match resolve(&timing, &exposures, &asset, frame).ok()? {
        SourceAt::Drawing { number, .. } => Some(asset.decode(number).ok()?.into_working()),
        SourceAt::Transparent => None,
    }
}

// -- artifacts ------------------------------------------------------------------------------

fn assert_report(report: &Report, title: &str) {
    let failures: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failures.is_empty(),
        "{title}: {} of {} checks failed\n{}",
        failures.len(),
        report.rows.len(),
        failures
            .iter()
            .map(|r| format!(
                "  {}\n    expected {}\n    got      {}",
                r.check, r.expected, r.actual
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn table(report: &Report) -> String {
    use std::fmt::Write as _;
    let mut s = String::from("| Check | Expected | Actual | |\n|---|---|---|---|\n");
    for row in &report.rows {
        let _ = writeln!(
            s,
            "| {} | `{}` | `{}` | {} |",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "FAIL" }
        );
    }
    s
}

fn write_fixture_artifact(report: &Report) {
    use std::fmt::Write as _;
    let mut s = String::from("# B-05b render trace, fixture results\n\n");
    let _ = writeln!(
        s,
        "**{} of {} checks pass.** ADR-012 render trace mode. Produced by \
         `tests/b05b_trace.rs`.\n",
        report.passed(),
        report.rows.len()
    );
    s.push_str(
        "## The fixture\n\nTwo layers, in the document 21 working space, linear light and \
         premultiplied:\n\n\
         | Layer | ID | Source | Every pixel | Transform | Opacity |\n|---|---|---|---|---|---|\n\
         | 0 | `bg` | 4x4 | `(0.2, 0.4, 0.6, 1.0)` | identity | 1.0 |\n\
         | 1 | `fg` | 2x2 | `(0.6, 0.0, 0.0, 1.0)` | translate `(1, 1)` | 0.5 |\n\n\
         The translation is a whole number of pixels, so the bilinear taps land exactly on \
         source pixel centres and `fg` is moved without being blurred. Every expected value \
         below follows from those two rows by hand.\n\n",
    );
    s.push_str(&table(report));
    s.push_str("\n## Notes\n\n");
    for note in &report.notes {
        let _ = writeln!(s, "- {note}");
    }
    s.push_str(
        "\n## What this does not cover\n\nTrace mode shows the four stages of document 21's \
         seven-step layer render order that this build implements. It cannot show the polygon \
         mask, layer effects, the alpha matte or the multiply, screen and add blend modes, \
         because the renderer does not have them. Every trace manifest says so in its own \
         words, so a trace directory can never be mistaken for a complete pipeline.\n",
    );
    fs::write(repo("verification/B-05b_trace_table.md"), s).expect("write artifact");
}

fn write_reference_artifact(plan: &FramePlan, written: &[anime_compositor::trace::TracedImage]) {
    use std::fmt::Write as _;
    let mut s = String::from("# B-05b render trace of the reference shot\n\n");
    let _ = writeln!(
        s,
        "One traced frame of the reference shot, {} layers at {}x{}, written to `trace/` by \
         `tests/b05b_trace.rs`.\n",
        plan.layers.len(),
        plan.width,
        plan.height
    );
    s.push_str(
        "`trace/` is in `.gitignore` and is not committed. ADR-012: trace output is diagnostic, \
         never part of export, never on by default. Re-create it with:\n\n\
         ```text\ncargo test --test b05b_trace b05b_traces_a_frame\n```\n\n\
         ## What was written\n\n| File | Layer | Stage | Document 21 step |\n|---|---|---|---|\n",
    );
    for image in written {
        let _ = writeln!(
            s,
            "| `trace/frame_00000/{}` | `{}` | {} | {} |",
            image.path.file_name().unwrap().to_string_lossy(),
            image.layer_id,
            image.stage.tag(),
            image.stage.document_21_step()
        );
    }
    s.push_str(
        "\n## How to use it\n\nOpen the directory and walk up the stack. Each layer has four \
         images. `decode` is the drawing as imported, at its own size. `transform` is that \
         drawing moved, scaled and turned into the composition. `opacity` is the same again \
         after the layer's opacity. `composite` is everything up to and including that layer.\n\n\
         When a frame looks wrong, the first image in that walk that looks wrong names the \
         stage that broke it, and naming the stage is enough to say what to fix without \
         reading any code.\n",
    );
    fs::write(repo("verification/B-05b_reference_trace.md"), s).expect("write artifact");
}
