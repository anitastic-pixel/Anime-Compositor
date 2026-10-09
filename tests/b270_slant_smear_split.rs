//! B-270 to B-272: D-391 Slant, after CycoreFX's CC Slant; D-392 Smear, after CycoreFX's CC
//! Smear; D-393 Split, after CycoreFX's CC Split ("Distort" in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-391_slant_table.md`, `verification/D-392_smear_table.md` and
//! `verification/D-393_split_table.md`.
//!
//! Every expected pixel is `Fixtures/slant/expected_slant.json`, `Fixtures/smear/expected_smear.json`
//! or `Fixtures/split/expected_split.json`, written by the matching `tools/*_reference.py` before
//! this code existed and printed in document 25 as FX-SLANT-001 to 024, FX-SMEAR-001 to 020 and
//! FX-SPLIT-001 to 017. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Each also draws the street into `verification/D-391 pictures/` (and D-392, D-393).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn slant(slant: f64, stretching: &str, height: f64, floor: [f64; 2], set_color: &str, color: &str) -> Effect {
    Effect::Slant { slant, stretching: stretching.into(), height, floor, set_color: set_color.into(), color: color.into() }
}

fn smear(from: [f64; 2], to: [f64; 2], reach: f64, radius: f64) -> Effect {
    Effect::Smear { from, to, reach, radius }
}

fn split(point_a: [f64; 2], point_b: [f64; 2], split: f64) -> Effect {
    Effect::Split { point_a, point_b, split }
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

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`; for the three here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.slant" => json!({"slant": 0, "stretching": "off", "height": 100, "floor": [50, 100], "set_color": "off", "color": "#000000"}),
        "core.smear" => json!({"from": [40, 50], "to": [60, 50], "reach": 100, "radius": 70}),
        "core.split" => json!({"point_a": [25, 50], "point_b": [75, 50], "split": 50}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(type_id, &format!("b270-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures
/// into `verification/{d} pictures/`; a row each that it draws cleanly and changes what it says
/// it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
}

fn unchanged(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).0 == 0
}

/// Changed, and some of it left clear.
fn changed_with_gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Slant -----------------------------------------------------------------------------------

fn is_slant(e: &Effect) -> bool {
    matches!(e, Effect::Slant { .. })
}

#[test]
fn b270_slant() {
    let mut t = Table::new(
        "slant",
        "# D-391: Slant\n\nB-270, after CycoreFX's CC Slant: the layer leaned over by Slant degrees \
         (the top to the right for plus), its height scaled by Height per cent toward a level \
         floor line through Floor, and without Stretching shortened as well by the cosine of the \
         slant, as if tipped over; Set Color fills it with one colour, keeping its coverage. The \
         formulas are this program's own. Every expected pixel is `Fixtures/slant/expected_slant.json`, \
         written by `tools/slant_reference.py` before this code existed and printed in document 25 \
         as FX-SLANT-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-SLANT-001 to 024 (document 25)");
    t.fixtures("expected_slant.json");

    t.heading("The file");
    let all = files("fx_slant", 24);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_slant_010.json");
    t.row(
        "fx_slant_010.json is saved with its slant, stretching, height, floor, set color and colour",
        &saved.to_string(),
        saved["slant"] == 45
            && saved["stretching"] == "on"
            && saved["height"] == 50
            && saved["floor"] == json!([50, 100])
            && saved["set_color"] == "on"
            && saved["color"] == "#ff0000",
    );
    why(
        &mut t,
        &[
            ("fx_slant_016.json", "Slant's slant runs from -80 to 80, and this is 81."),
            ("fx_slant_017.json", "Slant's slant runs from -80 to 80, and this is -81."),
            ("fx_slant_018.json", "Slant's height runs from 0 to 1000, and this is 1001."),
            ("fx_slant_019.json", "Slant's height runs from 0 to 1000, and this is -1."),
            ("fx_slant_020.json", "Slant's stretching is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_slant_021.json", "Slant's set color is \"on\" or \"off\", and this is \"maybe\"."),
            ("fx_slant_022.json", "Slant's colour is written #rrggbb, and this is \"black\"."),
            ("fx_slant_023.json", "Slant's floor runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused(
        "fx_slant_001.json",
        "a Slant with no `height`",
        r##"{"slant": 0, "stretching": "off", "floor": [50, 100], "set_color": "off", "color": "#000000"}"##,
    );
    t.shape_refused(
        "fx_slant_001.json",
        "a Slant whose floor is one number",
        r##"{"slant": 0, "stretching": "off", "height": 100, "floor": 100, "set_color": "off", "color": "#000000"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_slant_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("slant 80.5", set(slant(80.5, "off", 100.0, [50.0, 100.0], "off", "#000000"))),
            ("height -0.5", set(slant(0.0, "off", -0.5, [50.0, 100.0], "off", "#000000"))),
            ("stretching \"On\", written with a capital", set(slant(0.0, "On", 100.0, [50.0, 100.0], "off", "#000000"))),
            ("colour \"#fff\"", set(slant(0.0, "off", 100.0, [50.0, 100.0], "on", "#fff"))),
            ("slant keyed to 100", keys("slant", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_slant_001.json",
        vec![
            ("slant -25, stretched, height 60, floor 30, 80, set to a dark blue", set(slant(-25.0, "on", 60.0, [30.0, 80.0], "on", "#112233"))),
            ("slant keyed from 0 to 60", keys("slant", &[(0, &[0.0]), (4, &[60.0])])),
            ("floor keyed from 50, 100 to 50, 0", keys("floor", &[(0, &[50.0, 100.0]), (4, &[50.0, 0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_slant_002.json", 0), ("fx_slant_003.json", 0), ("fx_slant_005.json", 0), ("fx_slant_008.json", 0), ("fx_slant_010.json", 0), ("fx_slant_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_slant", 24, &[1, 16, 17, 18, 19, 20, 21, 22, 23, 24], is_slant);
    card_reference(
        &mut t,
        &mut gpu,
        "Slant",
        "core.slant",
        &[
            ("leaning 30, stretched", json!({"slant": 30, "stretching": "on"})),
            ("leaning -20, tipped, height 60 toward a floor across the middle", json!({"slant": -20, "height": 60, "floor": [50, 50]})),
            ("leaning 45, set to orange", json!({"slant": 45, "set_color": "on", "color": "#ff8000"})),
        ],
        is_slant,
    );

    street(
        &mut t,
        "D-391",
        "core.slant",
        &[
            ("as_added", json!({}), "as it starts, upright at full height: nothing changes", unchanged),
            ("lean", json!({"slant": 30}), "leaning 30, tipped: the street leaning right and shorter, its top left clear", changed_with_gaps),
            ("lean_stretched", json!({"slant": 30, "stretching": "on"}), "leaning 30, stretched: leaning right at full height, the corners clear", changed_with_gaps),
            ("half_height", json!({"height": 50, "stretching": "on", "floor": [50, 50]}), "height 50 toward a floor across the middle: squashed to a band, top and bottom clear", changed_with_gaps),
            ("red", json!({"slant": -20, "stretching": "on", "set_color": "on", "color": "#ff0000"}), "leaning -20, set to red: a red shape leaning left", changed_with_gaps),
        ],
    );

    t.finish("D-391_slant_table.md");
}

// --- Smear -----------------------------------------------------------------------------------

fn is_smear(e: &Effect) -> bool {
    matches!(e, Effect::Smear { .. })
}

#[test]
fn b271_smear() {
    let mut t = Table::new(
        "smear",
        "# D-392: Smear\n\nB-271, after CycoreFX's CC Smear: a round patch Radius pixels across \
         dragged from From toward To, Reach per cent of the way (minus drags it back), the drag \
         full along the line between them and easing to nothing at the patch's edge. The formulas \
         are this program's own. Every expected pixel is `Fixtures/smear/expected_smear.json`, \
         written by `tools/smear_reference.py` before this code existed and printed in document 25 \
         as FX-SMEAR-001 to 020. Tolerance 2e-5.\n",
    );

    t.heading("FX-SMEAR-001 to 020 (document 25)");
    t.fixtures("expected_smear.json");

    t.heading("The file");
    let all = files("fx_smear", 20);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_smear_013.json");
    t.row(
        "fx_smear_013.json is saved with its from, to, reach and radius",
        &saved.to_string(),
        saved["from"] == json!([25, 50]) && saved["to"] == json!([30, 50]) && saved["reach"] == 1000 && saved["radius"] == 3,
    );
    why(
        &mut t,
        &[
            ("fx_smear_014.json", "Smear's radius runs from 0 to 1000, and this is 1001."),
            ("fx_smear_015.json", "Smear's radius runs from 0 to 1000, and this is -1."),
            ("fx_smear_016.json", "Smear's reach runs from -1000 to 1000, and this is 1001."),
            ("fx_smear_017.json", "Smear's reach runs from -1000 to 1000, and this is -1001."),
            ("fx_smear_018.json", "Smear's from runs from -1000 to 1000, and this is 1001."),
            ("fx_smear_019.json", "Smear's to runs from -1000 to 1000, and this is -1001."),
        ],
    );
    t.shape_refused("fx_smear_001.json", "a Smear with no `radius`", r#"{"from": [40, 50], "to": [60, 50], "reach": 100}"#);
    t.shape_refused("fx_smear_001.json", "a Smear whose from is three numbers", r#"{"from": [40, 50, 0], "to": [60, 50], "reach": 100, "radius": 70}"#);

    t.heading("Commands");
    let mut document = t.load("fx_smear_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 1000.5", set(smear([40.0, 50.0], [60.0, 50.0], 100.0, 1000.5))),
            ("reach -1000.5", set(smear([40.0, 50.0], [60.0, 50.0], -1000.5, 70.0))),
            ("to at 60, 1000.5", set(smear([40.0, 50.0], [60.0, 1000.5], 100.0, 70.0))),
            ("radius keyed to -10", keys("radius", &[(0, &[70.0]), (4, &[-10.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_smear_001.json",
        vec![
            ("from 30, 40 to 70, 55, reach 150, radius 40", set(smear([30.0, 40.0], [70.0, 55.0], 150.0, 40.0))),
            ("reach keyed from 0 to 300", keys("reach", &[(0, &[0.0]), (4, &[300.0])])),
            ("to keyed from 60, 50 to 60, 90", keys("to", &[(0, &[60.0, 50.0]), (4, &[60.0, 90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_smear_001.json", 0), ("fx_smear_002.json", 0), ("fx_smear_003.json", 0), ("fx_smear_004.json", 0), ("fx_smear_007.json", 0), ("fx_smear_013.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_smear", 20, &[5, 6, 8, 14, 15, 16, 17, 18, 19, 20], is_smear);
    card_reference(
        &mut t,
        &mut gpu,
        "Smear",
        "core.smear",
        &[
            ("as it starts (40, 50 to 60, 50, radius 70)", json!({})),
            ("a long drag, reach 300, radius 200", json!({"from": [30, 40], "to": [60, 60], "reach": 300, "radius": 200})),
            ("dragged back down a diagonal, reach -150, radius 120", json!({"from": [20, 20], "to": [70, 80], "reach": -150, "radius": 120})),
        ],
        is_smear,
    );

    street(
        &mut t,
        "D-392",
        "core.smear",
        &[
            ("as_added", json!({}), "as it starts: a patch in the middle dragged a little to the right", changed),
            ("long_drag", json!({"from": [30, 50], "to": [60, 40], "reach": 200, "radius": 60}), "a long drag up and to the right, reach 200: the street pulled out in a streak", changed),
            (
                "pushed_back",
                json!({"from": [50, 60], "to": [50, 30], "reach": -100, "radius": 50}),
                "from 50, 60 toward 50, 30 at reach -100: the buildings dragged down into the road instead of up",
                changed,
            ),
        ],
    );

    t.finish("D-392_smear_table.md");
}

// --- Split -----------------------------------------------------------------------------------

fn is_split(e: &Effect) -> bool {
    matches!(e, Effect::Split { .. })
}

#[test]
fn b272_split() {
    let mut t = Table::new(
        "split",
        "# D-393: Split\n\nB-272, after CycoreFX's CC Split: the layer torn open along the line \
         from Point A to Point B, the gap Split pixels wide at the middle and closing to nothing at \
         the two points, each side squeezed outward so nothing is lost past the tear's reach. The \
         formulas are this program's own. Every expected pixel is `Fixtures/split/expected_split.json`, \
         written by `tools/split_reference.py` before this code existed and printed in document 25 \
         as FX-SPLIT-001 to 017. Tolerance 2e-5.\n",
    );

    t.heading("FX-SPLIT-001 to 017 (document 25)");
    t.fixtures("expected_split.json");

    t.heading("The file");
    let all = files("fx_split", 17);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_split_006.json");
    t.row(
        "fx_split_006.json is saved with its two points and split",
        &saved.to_string(),
        saved["point_a"] == json!([10, 10]) && saved["point_b"] == json!([90, 90]) && saved["split"] == 3,
    );
    why(
        &mut t,
        &[
            ("fx_split_013.json", "Split's split runs from 0 to 1000, and this is 1001."),
            ("fx_split_014.json", "Split's split runs from 0 to 1000, and this is -1."),
            ("fx_split_015.json", "Split's point a runs from -1000 to 1000, and this is 1001."),
            ("fx_split_016.json", "Split's point b runs from -1000 to 1000, and this is -1001."),
        ],
    );
    t.shape_refused("fx_split_001.json", "a Split with no `split`", r#"{"point_a": [25, 50], "point_b": [75, 50]}"#);
    t.shape_refused("fx_split_001.json", "a Split whose point a is one number", r#"{"point_a": 25, "point_b": [75, 50], "split": 50}"#);

    t.heading("Commands");
    let mut document = t.load("fx_split_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("split 1000.5", set(split([25.0, 50.0], [75.0, 50.0], 1000.5))),
            ("point a at -1000.5, 50", set(split([-1000.5, 50.0], [75.0, 50.0], 50.0))),
            ("split keyed to -10", keys("split", &[(0, &[50.0]), (4, &[-10.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_split_001.json",
        vec![
            ("from 20, 30 to 80, 70, split 35", set(split([20.0, 30.0], [80.0, 70.0], 35.0))),
            ("split keyed from 0 to 200", keys("split", &[(0, &[0.0]), (4, &[200.0])])),
            ("point b keyed from 75, 50 to 75, 90", keys("point_b", &[(0, &[75.0, 50.0]), (4, &[75.0, 90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_split_001.json", 0), ("fx_split_002.json", 0), ("fx_split_005.json", 0), ("fx_split_006.json", 0), ("fx_split_011.json", 0), ("fx_split_012.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_split", 17, &[3, 4, 13, 14, 15, 16, 17], is_split);
    card_reference(
        &mut t,
        &mut gpu,
        "Split",
        "core.split",
        &[
            ("as it starts (25, 50 to 75, 50, split 50)", json!({})),
            ("down a diagonal, split 200", json!({"point_a": [10, 10], "point_b": [90, 90], "split": 200})),
            ("past both edges, split 20", json!({"point_a": [-20, 50], "point_b": [120, 40], "split": 20})),
        ],
        is_split,
    );

    street(
        &mut t,
        "D-393",
        "core.split",
        &[
            ("as_added", json!({}), "as it starts: an eye-shaped gap across the middle, widest in the centre", changed_with_gaps),
            ("wide", json!({"split": 150}), "split 150: the gap opened wide, the two halves squeezed up and down", changed_with_gaps),
            ("diagonal", json!({"point_a": [10, 90], "point_b": [90, 10], "split": 40}), "corner to corner, split 40: a slanted tear", changed_with_gaps),
        ],
    );

    t.finish("D-393_split_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B270_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-270: a measurement, run deliberately with --release --ignored"]
fn b270_distort_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B270_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n- Loops: {passes}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cpu { "the processor" } else { "the card" },
    );
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Slant, leaning 30, stretched", Some(("core.slant", json!({"slant": 30, "stretching": "on"})))),
        ("Noise, then Smear, reach 300, radius 200", Some(("core.smear", json!({"from": [30, 40], "to": [60, 60], "reach": 300, "radius": 200})))),
        ("Noise, then Split, a diagonal, split 200", Some(("core.split", json!({"point_a": [10, 10], "point_b": [90, 90], "split": 200})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame"));
                } else {
                    drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                }
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B270_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-270_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
