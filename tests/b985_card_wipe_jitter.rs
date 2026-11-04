//! B-985: D-1105, Card Wipe's Position Jitter and Rotation Jitter, after After Effects' Card Wipe
//! groups of those names: each card slid (x, y, z) and turned (x, y, z) at random, by an amount,
//! to a new random place `speed` times a second. Adobe publishes no formula: the rule is ours,
//! stated in `tools/card_wipe_reference.py` (step 2b).
//!
//! Writes `verification/B-985_card_wipe_jitter_table.md` and draws pictures into
//! `verification/B-985 pictures/`.
//!
//! Every expected pixel is in `Fixtures/card_wipe/expected_card_wipe.json` (FX-CW-070 to 081),
//! written by the tool before this code existed. Tolerance 2e-5. Nothing here is a snapshot of a
//! run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, on_card, repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, CARD_JITTER};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// The `holder` layer's effect in `file`.
fn effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("holder"))
        .unwrap()
        .effects[0]
        .effect
        .clone()
}

/// `Fixtures/card_wipe/fx_cw_004.json` at the street's size in `dir`, the holder's settings
/// changed by `p`; `frame` drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/card_wipe/fx_cw_004.json")).unwrap(),
    )
    .unwrap();
    let a = project["assets"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["id"] == "asset-shot")
        .unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == "holder")
        .unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        dir,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b985_card_wipe_jitter() {
    let mut t = Table::new(
        "card_wipe",
        "# B-985: Card Wipe's Position Jitter and Rotation Jitter (D-1105)\n\nAfter Effects' \
         Card Wipe shakes its cards with Position Jitter (X, Y and Z Jitter Amount and Speed) and \
         Rotation Jitter (X, Y and Z Rot Jitter Amount and Speed). Each card is now slid and \
         turned by a smooth random amount that takes a new value Speed times a second; every \
         amount 0, as added, draws as before. Adobe's help publishes no formula, so the rule is \
         ours, stated in `tools/card_wipe_reference.py` (step 2b). Every expected pixel was \
         written by the tool before this code existed. Tolerance 2e-5.\n",
    );
    t.heading("Card Wipe: FX-CW-070 to 081 (document 25)");
    t.fixtures_numbered("expected_card_wipe.json", 70..=81);
    for (file, want) in [
        ("fx_cw_079.json", "x jitter amount"),
        ("fx_cw_080.json", "z jitter speed"),
        ("fx_cw_081.json", "y rot jitter amount"),
    ] {
        let mut why = effect(&t, file).why_invalid();
        if why.is_empty() {
            why = t
                .load(file)
                .warnings
                .iter()
                .map(|d| format!("{} {}", d.message, d.detail))
                .collect::<Vec<_>>()
                .join(" ");
        }
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().replace('_', " ").contains(want),
        );
    }
    let list: Vec<String> = (70..=81).map(|n| format!("fx_cw_{n:03}.json")).collect();
    t.round_trips(&list.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cw_004.json");
    t.row(
        "fx_cw_004.json, from before D-1105, opens with no jitter and is saved without the twelve settings",
        &format!("{:?}", CARD_JITTER.iter().filter(|k| saved.get(**k).is_some()).collect::<Vec<_>>()),
        CARD_JITTER.iter().all(|k| saved.get(*k).is_none()),
    );
    let saved = t.saved_parameters("fx_cw_071.json");
    t.row(
        "fx_cw_071.json is saved with the X jitter it moved and nothing else of the twelve",
        &CARD_JITTER.iter().map(|k| format!("{k} {}", saved.get(*k).map_or("-".to_string(), J::to_string))).collect::<Vec<_>>().join("; "),
        saved["x_jitter_amount"] == json!(0.3)
            && saved["x_jitter_speed"] == json!(6)
            && CARD_JITTER[2..].iter().all(|k| saved.get(*k).is_none()),
    );

    t.heading("Card Wipe on the card: FX-CW-070 to 078, frames 0 to 4 at Full and Draft");
    let mut gpu = Gpu::new().expect("a graphics card");
    let comp = Id::new(MAIN);
    for file in &list[..9] {
        let project = persist::load(&t.root.join(file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::CardWipe { .. })
                });
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 10,
        );
    }

    t.heading("Pictures (verification/B-985 pictures)");
    let dir = std::env::temp_dir().join("b985_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(
        &dir.join("town.png"),
        TOWN.0,
        TOWN.1,
        OutputDepth::Eight,
        &[],
        &effect_table::town(),
    )
    .unwrap();
    for m in ["back", "ramp"] {
        fs::copy(
            repo(&format!("Fixtures/shatter/media/{m}.png")),
            dir.join(format!("media/{m}.png")),
        )
        .unwrap();
    }
    let out = repo("verification/B-985 pictures");
    fs::create_dir_all(&out).unwrap();
    let wipe = json!({"transition_completion": 30, "back_layer": "", "rows": 6, "columns": 10});
    let with = |extra: J| {
        let mut p = wipe.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (name, p) in [
        ("1_position_jitter", with(json!({"x_jitter_amount": 0.2, "y_jitter_amount": 0.2, "x_jitter_speed": 6, "y_jitter_speed": 6}))),
        ("2_depth_jitter", with(json!({"z_jitter_amount": 1, "z_jitter_speed": 6}))),
        ("3_rotation_jitter", with(json!({"x_rot_jitter_amount": 25, "y_rot_jitter_amount": 25, "z_rot_jitter_amount": 15, "x_rot_jitter_speed": 6, "y_rot_jitter_speed": 6, "z_rot_jitter_speed": 6}))),
    ] {
        let (before, _) = picture(&dir, &wipe, 4);
        let (bytes, said) = picture(&dir, &p, 4);
        let (frame0, _) = picture(&dir, &p, 0);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let moved = distance(&bytes, &before).1;
        let shook = distance(&bytes, &frame0).1;
        t.row(
            &format!("{name}.png (frame 4): drawn without a warning, different from no jitter and from its own frame 0"),
            &format!("{moved} pixels differ from no jitter, {shook} from frame 0; warnings {said:?}"),
            said.is_empty() && moved > 0 && shook > 0,
        );
    }
    let (plain, _) = picture(&dir, &wipe, 4);
    png_out::write_rgba(&out.join("0_no_jitter.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &plain).unwrap();
    t.finish("B-985_card_wipe_jitter_table.md");
}
