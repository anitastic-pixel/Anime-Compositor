//! D-263: text layers, the Text tool's engine half.
//!
//! Writes `verification/D-263_text_table.md`.
//!
//! The owner asked for the Text tool on 2026-10-02 ("implement the text tool as well"), which
//! lifts D-262 (5). These are its checks:
//!
//! 1. A text layer is made and changed through the commands the page sends, one step of history
//!    each, and what is outside the ranges is refused rather than drawn differently.
//! 2. It saves and reads back exactly, keeps lines no build writes yet, and a file that puts a
//!    text record where it does not belong is refused.
//! 3. The letters are drawn where they were placed, the same every time, from the font that
//!    comes with the program; overlapping outlines are filled whole and holes stay holes.
//! 4. It goes into an exported frame, and a layer switched off leaves the frame as it was.
//! 5. A font that is not on the machine is said, per frame, and nothing is drawn in its place.
//! 6. A project without text is unchanged: every project under `Fixtures/` that opens saves byte
//!    for byte as on e395d8b, the hash D-253 pinned.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::{Command, Document};
use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::export::{export_sequence, ExportReport, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::model::{Id, Layer, Project};
use anime_compositor::persist::Preserved;
use anime_compositor::text::{self, Align, Text};
use anime_compositor::{persist, sha256, OutputAlpha, OutputDepth, WorkingBuffer};

/// Taken on e395d8b (D-253's check), as D-261's check has them.
const FIXTURES_OPENED: usize = 2297;
const FIXTURES_SAVED: &str = "aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d";

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn projects(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            projects(&path, found);
        } else if path.extension().is_some_and(|e| e == "json") {
            found.push(path);
        }
    }
}

fn words(text: &str, size: f64, at: [f64; 2], align: Align) -> Text {
    Text {
        text: text.into(),
        font: Text::BUNDLED_FONT.into(),
        size,
        color: [1.0, 0.9, 0.2],
        at,
        align,
    }
}

/// Where the ink is: the leftmost, rightmost, top and bottom pixel with any cover, and how many.
fn ink(picture: &WorkingBuffer) -> Option<(usize, usize, usize, usize, usize)> {
    let w = picture.width();
    let mut found: Option<(usize, usize, usize, usize, usize)> = None;
    for (i, px) in picture.data().chunks_exact(4).enumerate() {
        if px[3] > 0.0 {
            let (x, y) = (i % w, i / w);
            found = Some(match found {
                None => (x, x, y, y, 1),
                Some((l, r, t, b, n)) => (l.min(x), r.max(x), t.min(y), b.max(y), n + 1),
            });
        }
    }
    found
}

/// Frame 10 of the project's first composition as PNG bytes, and the report.
fn frame_10(project: &Project, name: &str) -> (Vec<u8>, ExportReport) {
    let dir = repo("target/d263_text").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let request = ExportRequest {
        composition: project.compositions[0].id.clone(),
        first_frame: 10,
        last_frame: 10,
        output_dir: dir.clone(),
        naming: "d263_%04d.png".to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: 128,
        missing: MissingSource::Block,
        format: OutputFormat::Png,
        choices: Default::default(),
    };
    let report = export_sequence(project, &repo("Fixtures/reference_shot"), &request, &AtomicBool::new(false));
    let bytes = fs::read(dir.join("d263_0010.png")).unwrap_or_default();
    (bytes, report)
}

#[test]
fn d263_text_layers_are_kept_drawn_and_said() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));
    let said = |r: Result<(), String>| if r.is_ok() { "taken".to_string() } else { "refused".to_string() };

    // Row 1, through the commands.
    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let without = doc.project().clone();
    let composition = without.compositions[0].id.clone();
    let apply = |doc: &mut Document, command: Command| doc.apply(command).map(|_| ()).map_err(|d| d.message);
    let title = Id::new("layer-text");
    let first = words("Cut 012", 120.0, [200.0, 600.0], Align::Left);
    let layer = Layer::text(title.clone(), "Text 1", first.clone(), 1920, 1080, 0, 240);
    row(
        "A text layer is added on top of the reference shot",
        "taken",
        said(apply(&mut doc, Command::AddLayer { composition: composition.clone(), layer: Box::new(layer.clone()), index: 4 })),
    );
    let mut bare = layer.clone();
    bare.id = Id::new("layer-text-bare");
    bare.text = None;
    row(
        "A text layer with no words, font or size is refused",
        "refused",
        said(apply(&mut doc, Command::AddLayer { composition: composition.clone(), layer: Box::new(bare), index: 0 })),
    );
    let set = |text: Text| Command::SetText { composition: composition.clone(), layer_id: title.clone(), text };
    let second = Text { text: "Cut 012\nあいう".into(), ..first.clone() };
    row("New words, on two lines and in Japanese, are taken", "taken", said(apply(&mut doc, set(second.clone()))));
    for (what, bad) in [
        ("A size of 0 is refused", Text { size: 0.0, ..first.clone() }),
        ("A size of 5000 pixels is refused", Text { size: 5000.0, ..first.clone() }),
        ("A colour above 1 is refused", Text { color: [1.5, 0.0, 0.0], ..first.clone() }),
        ("A place that is not a number is refused", Text { at: [f64::NAN, 0.0], ..first.clone() }),
        ("A font named by a path rather than a file name is refused", Text { font: "..\\..\\secret.ttf".into(), ..first.clone() }),
        ("A font that is not a .ttf, .otf or .ttc file is refused", Text { font: "notes.txt".into(), ..first.clone() }),
    ] {
        row(what, "refused", said(apply(&mut doc, set(bad))));
    }
    row(
        "Words on a drawn layer, which is not a text layer, are refused",
        "refused",
        said(apply(&mut doc, Command::SetText { composition: composition.clone(), layer_id: Id::new("layer-1"), text: first.clone() })),
    );
    let now = |doc: &Document| doc.project().compositions[0].layer(&title).and_then(|l| l.text.clone()).map(|t| t.text).unwrap_or_default();
    row("The words now", "Cut 012\\nあいう", now(&doc).replace('\n', "\\n"));
    doc.undo();
    row("Undo puts the words back", "Cut 012", now(&doc));
    doc.redo();
    let written = doc.project().clone();

    // Row 2.
    let saved = persist::to_json(&written, &Preserved::none());
    let value: serde_json::Value = serde_json::from_str(&saved).unwrap();
    let on_file = value["compositions"][0]["layers"].as_array().unwrap().iter().find(|l| l["id"] == "layer-text").cloned().unwrap_or_default();
    row(
        "Saved, the layer is kind text with its words, font, size, colour, place and alignment",
        "text, Cut 012\\nあいう, MPLUSRounded1c-Regular.ttf, 120, [1.0,0.9,0.2], [200.0,600.0], left",
        format!(
            "{}, {}, {}, {}, {}, {}, {}",
            on_file["kind"].as_str().unwrap_or("?"),
            on_file["source_text"]["text"].as_str().unwrap_or("?").replace('\n', "\\n"),
            on_file["source_text"]["font"].as_str().unwrap_or("?"),
            on_file["source_text"]["size"],
            on_file["source_text"]["color"],
            on_file["source_text"]["at"],
            on_file["source_text"]["align"].as_str().unwrap_or("?"),
        ),
    );
    row(
        "It has no drawing, footage offset or exposures written",
        "none",
        match ["asset_id", "source_offset_frames", "exposure_spans"].iter().filter(|k| on_file.get(**k).is_some()).copied().collect::<Vec<_>>() {
            v if v.is_empty() => "none".into(),
            v => v.join(", "),
        },
    );
    match persist::load_str(&saved) {
        Ok(back) => {
            row(
                "Opened again, the layer is the same",
                "the same",
                if back.document.project().compositions[0].layer(&title) == written.compositions[0].layer(&title) { "the same".into() } else { "different".into() },
            );
            row(
                "Saved again, the text is the same, byte for byte",
                "the same",
                if persist::to_json(back.document.project(), &back.preserved) == saved { "the same" } else { "different" }.into(),
            );
        }
        Err(d) => {
            row("Opened again, the layer is the same", "the same", d.message.clone());
            row("Saved again, the text is the same, byte for byte", "the same", d.message);
        }
    }
    let mut later = value.clone();
    for l in later["compositions"][0]["layers"].as_array_mut().unwrap() {
        if l["id"] == "layer-text" {
            l["source_text"]["tracking"] = serde_json::json!(25);
            l["text_animators"] = serde_json::json!([{ "range": [0, 3] }]);
        }
    }
    let kept = match persist::load_str(&later.to_string()) {
        Ok(k) => {
            let again: serde_json::Value = serde_json::from_str(&persist::to_json(k.document.project(), &k.preserved)).unwrap();
            let l = again["compositions"][0]["layers"].as_array().unwrap().iter().find(|l| l["id"] == "layer-text").cloned().unwrap_or_default();
            if l["source_text"]["tracking"] == 25 && l["text_animators"][0]["range"][1] == 3 { "kept".to_string() } else { l.to_string() }
        }
        Err(d) => d.message,
    };
    row("A line inside the text record and one on the layer that no build writes yet are kept", "kept", kept);
    let refused = |change: &dyn Fn(&mut serde_json::Value)| {
        let mut v = value.clone();
        change(&mut v);
        if persist::load_str(&v.to_string()).is_err() { "refused".to_string() } else { "opened".to_string() }
    };
    let layers = |v: &mut serde_json::Value| -> Vec<serde_json::Value> { v["compositions"][0]["layers"].as_array().unwrap().clone() };
    let put = |v: &mut serde_json::Value, id: &str, key: &str, what: Option<serde_json::Value>| {
        let mut all = layers(v);
        for l in &mut all {
            if l["id"] == id {
                match &what {
                    Some(x) => { l[key] = x.clone(); }
                    None => { l.as_object_mut().unwrap().remove(key); }
                }
            }
        }
        v["compositions"][0]["layers"] = serde_json::Value::Array(all);
    };
    row(
        "A file with a text record on a drawn layer is refused",
        "refused",
        refused(&|v| put(v, "layer-1", "source_text", Some(on_file["source_text"].clone()))),
    );
    row("A file with a text layer and no text record is refused", "refused", refused(&|v| put(v, "layer-text", "source_text", None)));
    row(
        "A file with a text layer holding exposures is refused",
        "refused",
        refused(&|v| put(v, "layer-text", "exposure_spans", Some(serde_json::json!([])))),
    );
    row(
        "A file with a text size of 0 is refused",
        "refused",
        refused(&|v| {
            let mut t = on_file["source_text"].clone();
            t["size"] = serde_json::json!(0);
            put(v, "layer-text", "source_text", Some(t));
        }),
    );

    // Row 3, the letters themselves.
    let draw = |t: &Text| text::draw(t, 1920, 1080);
    let left = draw(&words("Text あ", 120.0, [200.0, 600.0], Align::Left));
    row("The font that comes with the program is found", "found", if left.is_some() { "found" } else { "not found" }.into());
    let left = left.unwrap_or_else(|| WorkingBuffer::transparent(1920, 1080));
    let box_of = |p: &WorkingBuffer| ink(p).map(|(l, r, t, b, n)| format!("{l}..{r} across, {t}..{b} down, {n} pixels")).unwrap_or("no ink".into());
    let (l, r, t, b, n) = ink(&left).unwrap_or((0, 0, 0, 0, 0));
    row(
        "\"Text あ\" at 120 pixels, placed at 200, 600: ink right of 188, left of 920, below 456 and above 642, and more than 3000 pixels of it",
        "yes",
        if n > 3000 && l >= 188 && r <= 920 && t >= 456 && b <= 642 { "yes".into() } else { box_of(&left) },
    );
    let again = draw(&words("Text あ", 120.0, [200.0, 600.0], Align::Left));
    row(
        "Drawn a second time, every pixel is the same",
        "the same",
        if again.as_ref().is_some_and(|a| a.data() == left.data()) { "the same" } else { "different" }.into(),
    );
    let right = draw(&words("Text あ", 120.0, [1700.0, 600.0], Align::Right)).unwrap_or_else(|| WorkingBuffer::transparent(1920, 1080));
    row(
        "Aligned right at 1700, the ink ends between 1676 and 1702",
        "yes",
        match ink(&right) { Some((_, r, ..)) if (1676..=1702).contains(&r) => "yes".into(), _ => box_of(&right) },
    );
    let centre = draw(&words("Text あ", 120.0, [960.0, 600.0], Align::Center)).unwrap_or_else(|| WorkingBuffer::transparent(1920, 1080));
    row(
        "Centred on 960, the ink's middle is within 24 pixels of it",
        "yes",
        match ink(&centre) { Some((l, r, ..)) if ((l + r) as f64 / 2.0 - 960.0).abs() <= 24.0 => "yes".into(), _ => box_of(&centre) },
    );
    let one = draw(&words("A", 100.0, [100.0, 300.0], Align::Left)).unwrap_or_else(|| WorkingBuffer::transparent(1920, 1080));
    let two = draw(&words("A\nA", 100.0, [100.0, 300.0], Align::Left)).unwrap_or_else(|| WorkingBuffer::transparent(1920, 1080));
    row(
        "A second line is drawn wholly below the first",
        "yes",
        match (ink(&one), ink(&two)) {
            (Some((_, _, _, b1, n1)), Some((_, _, _, b2, n2))) if b2 > b1 + 60 && n2 > n1 * 3 / 2 => "yes".into(),
            _ => format!("one line {}, two lines {}", box_of(&one), box_of(&two)),
        },
    );
    let square = |x0: f64, y0: f64, x1: f64, y1: f64| vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)];
    let mut backwards = square(20.0, 20.0, 30.0, 30.0);
    backwards.reverse();
    let overlap = text::fill(&[square(10.0, 10.0, 30.0, 30.0), square(20.0, 20.0, 40.0, 40.0)], 50, 50);
    let hole = text::fill(&[square(10.0, 10.0, 40.0, 40.0), backwards], 50, 50);
    row(
        "Two outlines turning the same way that overlap are filled where they overlap",
        "1",
        overlap[25 * 50 + 25].to_string(),
    );
    row(
        "An outline turning the other way inside another is a hole, as in an O",
        "hole 0, ring 1",
        format!("hole {}, ring {}", hole[25 * 50 + 25], hole[15 * 50 + 15]),
    );
    row(
        "A 20 by 20 square on whole pixels covers exactly 400 pixels",
        "400",
        text::fill(&[square(10.0, 10.0, 30.0, 30.0)], 50, 50).iter().sum::<f32>().to_string(),
    );

    // Row 4, in an exported frame.
    let (plain, _) = frame_10(&without, "without");
    let (titled, report) = frame_10(&written, "with");
    row("Frame 10 of the reference shot is written", "written", if plain.is_empty() { "missing" } else { "written" }.into());
    row(
        "With the text layer on top, frame 10 changes and nothing is said",
        "changed, nothing said",
        format!(
            "{}, {}",
            if titled != plain && !titled.is_empty() { "changed" } else { "unchanged" },
            if report.diagnostics.is_empty() { "nothing said".to_string() } else { format!("{:?}", report.diagnostics.iter().map(|d| d.id.as_str()).collect::<Vec<_>>()) },
        ),
    );
    let _ = apply(&mut doc, Command::SetLayerEnabled { composition: composition.clone(), layer_id: title.clone(), value: false });
    let (switched_off, _) = frame_10(doc.project(), "off");
    doc.undo();
    row("With the text layer switched off, frame 10 is byte for byte the frame without it", "the same", if switched_off == plain { "the same" } else { "different" }.into());

    // Row 5.
    row(
        "A font this machine does not have is taken: the project may go to a machine that has it",
        "taken",
        said(apply(&mut doc, set(Text { font: "NoSuchFont-Regular.ttf".into(), ..second.clone() }))),
    );
    let (unfound, report) = frame_10(doc.project(), "missing");
    row(
        "With a font this machine does not have, frame 10 says TEXT_FONT_MISSING and is marked incomplete",
        "said, incomplete",
        format!(
            "{}, {}",
            if report.diagnostics.iter().any(|d| d.id == DiagnosticId::TextFontMissing && d.id.as_str() == "TEXT_FONT_MISSING") { "said" } else { "not said" },
            if report.fidelity_incomplete { "incomplete" } else { "complete" },
        ),
    );
    row("Nothing is drawn in its place: the frame is the frame without the layer", "the same", if unfound == plain { "the same" } else { "different" }.into());
    row(
        "The font that comes with the program has its licence beside the others",
        "there",
        if fs::read_to_string(repo("docs/third_party/MPLUSRounded1c-OFL.txt")).is_ok_and(|t| t.contains("SIL OPEN FONT LICENSE")) { "there" } else { "missing" }.into(),
    );

    // Row 6.
    let mut found = Vec::new();
    projects(&repo("Fixtures"), &mut found);
    found.sort();
    let mut all = String::new();
    let mut opened = 0;
    for path in &found {
        let Ok(text) = fs::read_to_string(path) else { continue };
        if let Ok(loaded) = persist::load_str(&text) {
            opened += 1;
            all.push_str(&persist::to_json(loaded.document.project(), &loaded.preserved));
        }
    }
    row("Fixture projects that open", &FIXTURES_OPENED.to_string(), opened.to_string());
    row("Their saved text, all of it, is byte for byte as before (SHA-256)", FIXTURES_SAVED, sha256::hex(all.as_bytes()));

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut md = format!(
        "# D-263: text layers\n\nWritten by `tests/d263_text.rs`. The fixture hash was taken on e395d8b, before D-253.\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, e, a) in &rows {
        let _ = writeln!(md, "| {check} | {e} | {a} | {} |", if e == a { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/D-263_text_table.md"), md).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "{failed:?}");
}
