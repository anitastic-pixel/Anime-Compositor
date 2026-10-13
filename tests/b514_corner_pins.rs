//! B-514: D-634, After Effects' Corner Pins camera for Card Wipe, Card Dance and Shatter: the
//! camera worked out from where the layer's four corners should land, with Auto Focal Length
//! working out the lens from them. Adobe's help says what the pins do but publishes no formula:
//! the rule is ours, stated in `tools/card_dance_reference.py` (step 4) and used by the Card Wipe
//! and Shatter tools.
//!
//! Writes `verification/D-634_corner_pins_table.md` and draws pictures into
//! `verification/D-634 pictures/`.
//!
//! Every expected pixel is in `Fixtures/card_wipe/expected_card_wipe.json` (FX-CW-057 to 069),
//! `Fixtures/card_dance/expected_card_dance.json` (FX-CD-052 and 065 to 076) and
//! `Fixtures/shatter/expected_shatter.json` (FX-SH-067 and 079 to 083), written by the tools
//! before this code existed. FX-CD-052 and FX-SH-067 are checked by B-512's and B-513's tests.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

const PIN_KEYS: [&str; 6] = ["camera_system", "upper_left_corner", "upper_right_corner", "lower_left_corner", "lower_right_corner", "auto_focal_length"];

/// The `holder` layer's effect in `file`.
fn effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
}

fn files(prefix: &str, numbers: std::ops::RangeInclusive<u32>) -> Vec<String> {
    numbers.map(|n| format!("{prefix}_{n:03}.json")).collect()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// The Card Wipes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CardWipe { .. })))
        .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// `Fixtures/{folder}/{file}` at the street's size in `dir` (the street the holder's drawing,
/// the fixture's other layers as they are), the holder's settings changed by `p`; `frame` drawn
/// on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, folder: &str, file: &str, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/{folder}/{file}"))).unwrap()).unwrap();
    let a = project["assets"].as_array_mut().unwrap().iter_mut().find(|a| a["id"] == "asset-shot").unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let holder = comp["layers"].as_array_mut().unwrap().iter_mut().find(|l| l["id"] == "holder").unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

/// One effect's new cases: the fixtures, the refusals, the round trips, and an old file that
/// never used the pins saved without them.
fn section(t: &mut Table, name: &str, folder: &str, prefix: &str, numbers: std::ops::RangeInclusive<u32>, refused: &[(&str, &str)], old: &str) {
    t.root = repo(&format!("Fixtures/{folder}"));
    let (a, b) = (*numbers.start(), *numbers.end());
    t.heading(&format!("{name}: {}-{a:03} to {b:03} (document 25)", prefix.to_uppercase().replace('_', "-")));
    t.fixtures_numbered(&format!("expected_{folder}.json"), numbers.clone());
    for (file, want) in refused {
        // A setting keyed out of range is refused when the file is read, by the warning.
        let mut why = effect(t, file).why_invalid();
        if why.is_empty() {
            why = t.load(file).warnings.iter().map(|d| format!("{} {}", d.message, d.detail)).collect::<Vec<_>>().join(" ");
        }
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    let list = files(prefix, numbers);
    t.round_trips(&list.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters(old);
    let kept = if folder == "card_wipe" { &PIN_KEYS[..] } else { &PIN_KEYS[1..] };
    t.row(
        &format!("{old}, from before D-634, opens with the pins as added and is saved without them"),
        &format!("{:?}", kept.iter().filter(|k| saved.get(**k).is_some()).collect::<Vec<_>>()),
        kept.iter().all(|k| saved.get(*k).is_none()),
    );
}

#[test]
fn b514_corner_pins() {
    let mut t = Table::new(
        "card_wipe",
        "# D-634: the Corner Pins camera\n\nB-514, After Effects' Corner Pins camera system for \
         Card Wipe, Card Dance and Shatter, under the coordinator's approval of the D-634 design \
         for the owner's /loop request. The four pins say where the layer's corners land; the \
         camera that puts them there is worked out from them, with Focal Length as the lens or, \
         with Auto Focal Length on, the lens the pins themselves imply where it lies within 1 to \
         1000 mm. The light stays where it is on the layer. Adobe's help publishes no formula, so \
         the rule is ours, stated in `tools/card_dance_reference.py` (step 4). Every expected \
         pixel was written by the tools before this code existed. Tolerance 2e-5.\n",
    );

    section(
        &mut t,
        "Card Wipe",
        "card_wipe",
        "fx_cw",
        57..=69,
        &[
            ("fx_cw_066.json", "camera system"),
            ("fx_cw_067.json", "camera system"),
            ("fx_cw_068.json", "upper left corner"),
            ("fx_cw_069.json", "auto focal length"),
        ],
        "fx_cw_004.json",
    );
    let saved = t.saved_parameters("fx_cw_058.json");
    t.row(
        "fx_cw_058.json is saved with Corner Pins and the three pins it moved, the lower left pin it left alone not written",
        &PIN_KEYS.iter().map(|k| format!("{k} {}", saved.get(*k).map_or("-".to_string(), J::to_string))).collect::<Vec<_>>().join("; "),
        saved["camera_system"] == "corner_pins"
            && saved["upper_left_corner"] == json!([20, 10])
            && saved["upper_right_corner"] == json!([85, 0])
            && saved["lower_right_corner"] == json!([100, 95])
            && saved.get("lower_left_corner").is_none(),
    );

    t.heading("Card Wipe on the card: FX-CW-057 to 065, frames 0 to 4 at Full and Draft");
    let mut gpu = Gpu::new().expect("a graphics card");
    let comp = Id::new(MAIN);
    for file in files("fx_cw", 57..=65) {
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 10,
        );
    }

    section(
        &mut t,
        "Card Dance",
        "card_dance",
        "fx_cd",
        65..=76,
        &[
            ("fx_cd_074.json", "upper left corner"),
            ("fx_cd_075.json", "auto focal length"),
            ("fx_cd_076.json", "lower right corner"),
        ],
        "fx_cd_040.json",
    );
    section(
        &mut t,
        "Shatter",
        "shatter",
        "fx_sh",
        79..=83,
        &[("fx_sh_082.json", "lower right corner"), ("fx_sh_083.json", "auto focal length")],
        "fx_sh_001.json",
    );

    t.heading("Pictures (verification/D-634 pictures)");
    let dir = std::env::temp_dir().join("b514_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(&dir.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &effect_table::town()).unwrap();
    for m in ["back", "ramp", "colours", "cracks"] {
        fs::copy(repo(&format!("Fixtures/shatter/media/{m}.png")), dir.join(format!("media/{m}.png"))).unwrap();
    }
    let out = repo("verification/D-634 pictures");
    fs::create_dir_all(&out).unwrap();
    let keystone = json!({"camera_system": "corner_pins", "upper_left_corner": [20, 10], "upper_right_corner": [85, 0], "lower_right_corner": [100, 95]});
    let quad = json!({"camera_system": "corner_pins", "upper_left_corner": [5, 10], "upper_right_corner": [90, -5], "lower_right_corner": [100, 95], "lower_left_corner": [-5, 105]});
    let with = |base: &J, extra: J| {
        let mut p = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    let wipe = json!({"transition_completion": 30, "back_layer": ""});
    let dance = json!({"rows": 9, "columns": 16});
    for (name, folder, file, plain, p, frame) in [
        ("1_card_wipe_pulled_in", "card_wipe", "fx_cw_004.json", wipe.clone(), with(&wipe, keystone.clone()), 0),
        ("2_card_wipe_pulled_in_auto_lens", "card_wipe", "fx_cw_004.json", wipe.clone(), with(&with(&wipe, keystone.clone()), json!({"auto_focal_length": "on"})), 0),
        ("3_card_wipe_four_sided_point_light", "card_wipe", "fx_cw_004.json", wipe.clone(), with(&with(&wipe, quad.clone()), json!({"light_type": "point", "light_position": [20, 30], "light_depth": 0.3, "specular_reflection": 1, "highlight_sharpness": 20})), 0),
        ("4_card_dance_pulled_in_auto_lens", "card_dance", "fx_cd_001.json", dance.clone(), with(&with(&dance, keystone.clone()), json!({"auto_focal_length": "on"})), 0),
        ("5_card_dance_four_sided", "card_dance", "fx_cd_001.json", dance.clone(), with(&dance, quad.clone()), 0),
        ("6_shatter_pulled_in_auto_lens", "shatter", "fx_sh_001.json", json!({"force_1_strength": 1}), with(&keystone, json!({"auto_focal_length": "on", "force_1_strength": 1})), 3),
    ] {
        let (before, _) = picture(&dir, folder, file, &plain, frame);
        let (bytes, said) = picture(&dir, folder, file, &p, frame);
        png_out::write_rgba(&out.join(format!("{name}.png")), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &bytes).unwrap();
        let moved = distance(&bytes, &before).1;
        t.row(
            &format!("{name}.png: drawn without a warning, different from the camera as added"),
            &format!("{moved} pixels differ from the same frame seen by the camera as added; warnings {said:?}"),
            said.is_empty() && moved > 0,
        );
    }
    t.finish("D-634_corner_pins_table.md");
}
