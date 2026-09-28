//! B-117: the starter presets the program comes with, against D-181.
//!
//! Writes `verification/B-117_starter_presets_table.md` and a picture of each preset in
//! `verification/B-117 pictures/`, for the owner to judge the looks.
//!
//! # Where the expected values come from
//!
//! `Fixtures/starter_presets/`, written by `tools/starter_presets_reference.py` before any code:
//! `starter.fxpreset` is the presets themselves, which the build carries inside itself, and
//! `expected_starter_presets.json` the names, effect types and sentences the build must show, and
//! where each is pictured.
//!
//! # What is checked
//!
//! The build's copy is the fixture file, and reads as an import would. Each preset is drawn on
//! frame 100 of the reference shot at Draft, on `layer-3` or on an adjustment layer above all four,
//! with no diagnostic and a picture different from the one without it. The Effects panel's
//! built-in mark, copy and refusals are in `app/ui/index.html` and the playtest.

use std::fs;

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;

mod common;
use common::repo;

/// Frame 100 of the reference shot at Draft, as 8-bit bytes, with what its log said.
fn draw(project: &J) -> (usize, usize, Vec<u8>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string()).expect("the reference shot reads");
    let mut log = FrameLog::new(3);
    let frame = preview::preview_frame_cached(
        loaded.document.project(),
        &Id::new("comp-reference-shot"),
        100,
        &repo("Fixtures/reference_shot"),
        PreviewQuality::Draft,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("frame 100 draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.width(), frame.height(), frame.to_srgb8_straight(), said)
}

#[test]
fn b117_starter_presets() {
    let root = repo("Fixtures/starter_presets");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_starter_presets.json")).unwrap()).unwrap();
    let want = expected["presets"].as_array().unwrap();
    let pictures = repo("verification/B-117 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let mut out = String::from(
        "# B-117: starter presets\n\n\
         Written by `tests/b117_starter_presets.rs` from `Fixtures/starter_presets/`, against D-181. Each line is \
         one check: what `tools/starter_presets_reference.py` says the build must have, and what it has. The \
         pictures are frame 100 of the reference shot at Draft, in `verification/B-117 pictures/`; `before.png` \
         is the shot with no preset.\n\n\
         | Preset | Check | Should | The build | Matches |\n| --- | --- | --- | --- | --- |\n",
    );
    let (mut checks, mut passed) = (0, 0);
    let mut row = |who: &str, what: &str, should: String, built: String| {
        let ok = should == built;
        checks += 1;
        passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        out.push_str(&format!("| {who} | {what} | {should} | {built} | {verdict} |\n"));
    };

    let shipped = persist::STARTER_PRESETS.replace("\r\n", "\n");
    let fixture = fs::read_to_string(root.join("starter.fxpreset")).unwrap().replace("\r\n", "\n");
    row("all", "the build carries the fixture file", "the same text".into(),
        if shipped == fixture { "the same text".into() } else { "different text".into() });
    let read = persist::read_presets(persist::STARTER_PRESETS);
    row("all", "reads as an import would", "read".into(),
        match &read { Ok(_) => "read".into(), Err(d) => format!("refused: {}", d.message) });
    let read = read.unwrap_or_default();
    row("all", "how many", format!("{} (six to ten)", want.len()),
        format!("{}{}", read.len(), if (6..=10).contains(&read.len()) { " (six to ten)" } else { "" }));

    let base: J = serde_json::from_str(&fs::read_to_string(repo("verification/B-08a_project.json")).unwrap()).unwrap();
    let (w, h, before, said) = draw(&base);
    png_out::write_rgba(&pictures.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    row("none", "the shot draws cleanly", "no diagnostic".into(),
        if said.is_empty() { "no diagnostic".into() } else { said.join("; ") });

    for (i, want) in want.iter().enumerate() {
        let name = want["name"].as_str().unwrap();
        let Some(preset) = read.get(i) else {
            row(name, "is there", "yes".into(), "missing".into());
            continue;
        };
        row(name, "name, in this place", name.into(), preset.name.clone());
        let types: Vec<&str> = want["effects"].as_array().unwrap().iter().map(|t| t.as_str().unwrap()).collect();
        row(name, "effects, in order", types.join(", "),
            preset.effects.iter().map(|e| e.type_id()).collect::<Vec<_>>().join(", "));
        row(name, "the sentence the panel shows", want["about"].as_str().unwrap().into(),
            persist::STARTER_ABOUT.get(i).copied().unwrap_or("none").into());

        // Applied as the window applies it: the file's effects, as written, on the layer.
        let effects = serde_json::from_str::<J>(persist::STARTER_PRESETS).unwrap()["presets"][i]["effects"].clone();
        let mut project = base.clone();
        let comp = &mut project["compositions"][0];
        let on = want["shown_on"].as_str().unwrap();
        if on == "layer" {
            let layer = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "layer-3").unwrap();
            layer["effects"] = effects;
        } else {
            let mut adjustment = comp["layers"].as_array().unwrap().iter().find(|l| l["id"] == "layer-4").unwrap().clone();
            let fields = adjustment.as_object_mut().unwrap();
            for k in ["asset_id", "exposure_spans", "source_offset_frames"] {
                fields.remove(k);
            }
            fields.insert("id".into(), json!("layer-adjust"));
            fields.insert("name".into(), json!("adjustment"));
            fields.insert("kind".into(), json!("adjustment"));
            fields.insert("effects".into(), effects);
            comp["layers"].as_array_mut().unwrap().push(adjustment);
            comp["layer_order"].as_array_mut().unwrap().push(json!("layer-adjust"));
        }
        let (w, h, bytes, said) = draw(&project);
        png_out::write_rgba(&pictures.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let where_ = if on == "layer" { "on layer-3" } else { "on an adjustment layer" };
        row(name, &format!("draws {where_}"), "no diagnostic".into(),
            if said.is_empty() { "no diagnostic".into() } else { said.join("; ") });
        let changed = bytes.chunks(4).zip(before.chunks(4)).filter(|(a, b)| a != b).count();
        row(name, &format!("changes the picture: `{name}.png`, {changed} of {} pixels", w * h), "yes".into(),
            if changed > 0 { "yes".into() } else { "no pixel changed".into() });
    }
    out.push_str(&format!("\n**B-117: {passed} of {checks} checks pass.**\n"));
    fs::write(repo("verification/B-117_starter_presets_table.md"), &out).unwrap();
    assert_eq!(passed, checks, "see verification/B-117_starter_presets_table.md");
}
