//! B-264 to B-266: D-385 Flow Motion, after CycoreFX's CC Flo Motion; D-386 Griddler, after
//! CycoreFX's CC Griddler; D-387 Fisheye, after CycoreFX's CC Lens ("Distort" in
//! `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-385_flow_motion_table.md`, `verification/D-386_griddler_table.md` and
//! `verification/D-387_fisheye_table.md`.
//!
//! Every expected pixel is `Fixtures/flow_motion/expected_flow_motion.json`,
//! `Fixtures/griddler/expected_griddler.json` or `Fixtures/fisheye/expected_fisheye.json`,
//! written by the matching `tools/*_reference.py` before this code existed and printed in
//! document 25 as FX-FLOW-001 to 024, FX-GRIDDLER-001 to 022 and FX-FISHEYE-001 to 020.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Each also draws the street into `verification/D-385 pictures/` (and D-386, D-387).

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

#[allow(clippy::too_many_arguments)]
fn flow(knot_1: [f64; 2], amount_1: f64, knot_2: [f64; 2], amount_2: f64, falloff: f64, tile: &str, finer: &str, aa: &str) -> Effect {
    Effect::FlowMotion {
        knot_1,
        amount_1,
        knot_2,
        amount_2,
        falloff,
        tile_edges: tile.into(),
        finer_controls: finer.into(),
        antialiasing: aa.into(),
    }
}

fn griddler(horizontal_scale: f64, vertical_scale: f64, tile_size: f64, rotation: f64, cut: &str) -> Effect {
    Effect::Griddler { horizontal_scale, vertical_scale, tile_size, rotation, cut_tiles: cut.into() }
}

fn fisheye(center: [f64; 2], size: f64, convergence: f64) -> Effect {
    Effect::Fisheye { center, size, convergence }
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
        "core.flow_motion" => json!({
            "knot_1": [25, 50], "amount_1": 10, "knot_2": [75, 50], "amount_2": -10, "falloff": 5,
            "tile_edges": "on", "finer_controls": "off", "antialiasing": "low"}),
        "core.griddler" => json!({"horizontal_scale": 80, "vertical_scale": 80, "tile_size": 10, "rotation": 0, "cut_tiles": "on"}),
        "core.fisheye" => json!({"center": [50, 50], "size": 50, "convergence": 50}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b264-{id}"), p)]));
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

// --- Flow Motion -----------------------------------------------------------------------------

fn is_flow(e: &Effect) -> bool {
    matches!(e, Effect::FlowMotion { .. })
}

#[test]
fn b264_flow_motion() {
    let mut t = Table::new(
        "flow_motion",
        "# D-385: Flow Motion\n\nB-264, after CycoreFX's CC Flo Motion: two knots, each drawing \
         the picture in towards itself for a positive amount and blowing it out of itself for a \
         negative one, over a reach set by Falloff; the edges can repeat mirrored, and each pixel \
         can average 1, 4 or 16 points. The formulas are this program's own. Every expected pixel \
         is `Fixtures/flow_motion/expected_flow_motion.json`, written by \
         `tools/flow_motion_reference.py` before this code existed and printed in document 25 as \
         FX-FLOW-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-FLOW-001 to 024 (document 25)");
    t.fixtures("expected_flow_motion.json");

    t.heading("The file");
    let all = files("fx_flow", 24);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_flow_007.json");
    t.row(
        "fx_flow_007.json is saved with its knots, amounts, falloff, tile edges, finer controls and antialiasing",
        &saved.to_string(),
        saved["knot_1"] == json!([50, 50])
            && saved["amount_1"] == 200
            && saved["amount_2"] == 0
            && saved["falloff"] == 5
            && saved["tile_edges"] == "off"
            && saved["finer_controls"] == "on"
            && saved["antialiasing"] == "low",
    );
    why(
        &mut t,
        &[
            ("fx_flow_016.json", "Flow Motion's amount 1 runs from -1000 to 1000, and this is 1001."),
            ("fx_flow_017.json", "Flow Motion's amount 2 runs from -1000 to 1000, and this is -1001."),
            ("fx_flow_018.json", "Flow Motion's falloff runs from 0 to 10, and this is 11."),
            ("fx_flow_019.json", "Flow Motion's falloff runs from 0 to 10, and this is -1."),
            ("fx_flow_020.json", "Flow Motion's tile edges is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_flow_021.json", "Flow Motion's finer controls is \"on\" or \"off\", and this is \"On\"."),
            ("fx_flow_022.json", "Flow Motion's antialiasing is low, medium or high, and this is \"best\"."),
            ("fx_flow_023.json", "Flow Motion's knot 1 runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused(
        "fx_flow_001.json",
        "a Flow Motion with no `falloff`",
        r#"{"knot_1": [25, 50], "amount_1": 10, "knot_2": [75, 50], "amount_2": -10, "tile_edges": "on", "finer_controls": "off", "antialiasing": "low"}"#,
    );
    t.shape_refused(
        "fx_flow_001.json",
        "a Flow Motion whose knot 2 is one number",
        r#"{"knot_1": [25, 50], "amount_1": 10, "knot_2": 75, "amount_2": -10, "falloff": 5, "tile_edges": "on", "finer_controls": "off", "antialiasing": "low"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_flow_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("amount 1 1000.5", set(flow([25.0, 50.0], 1000.5, [75.0, 50.0], -10.0, 5.0, "on", "off", "low"))),
            ("knot 2 at -1000.5, 50", set(flow([25.0, 50.0], 10.0, [-1000.5, 50.0], -10.0, 5.0, "on", "off", "low"))),
            ("falloff 10.5", set(flow([25.0, 50.0], 10.0, [75.0, 50.0], -10.0, 10.5, "on", "off", "low"))),
            ("antialiasing \"Low\", written with a capital", set(flow([25.0, 50.0], 10.0, [75.0, 50.0], -10.0, 5.0, "on", "off", "Low"))),
            ("tile edges \"mirror\"", set(flow([25.0, 50.0], 10.0, [75.0, 50.0], -10.0, 5.0, "mirror", "off", "low"))),
            ("amount 2 keyed to -2000", keys("amount_2", &[(0, &[-10.0]), (4, &[-2000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_flow_001.json",
        vec![
            ("two knots, finer controls, high, edges off", set(flow([20.0, 30.0], 40.0, [70.0, 60.0], -25.0, 3.0, "off", "on", "high"))),
            ("amount 1 keyed from 0 to 30", keys("amount_1", &[(0, &[0.0]), (4, &[30.0])])),
            ("knot 2 keyed from 75, 50 to 25, 50", keys("knot_2", &[(0, &[75.0, 50.0]), (4, &[25.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_flow_001.json", 0), ("fx_flow_005.json", 0), ("fx_flow_007.json", 0), ("fx_flow_010.json", 0), ("fx_flow_011.json", 0), ("fx_flow_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_flow", 24, &[2, 16, 17, 18, 19, 20, 21, 22, 23, 24], is_flow);
    card_reference(
        &mut t,
        &mut gpu,
        "Flow Motion",
        "core.flow_motion",
        &[
            ("as it starts (10 at the left, -10 at the right)", json!({})),
            ("a strong pull in the middle, edges off, high", json!({"knot_1": [50, 50], "amount_1": 60, "amount_2": 0, "falloff": 4, "tile_edges": "off", "antialiasing": "high"})),
            ("a swell at the top left, finer controls, medium", json!({"knot_1": [10, 10], "amount_1": -300, "knot_2": [80, 70], "amount_2": 150, "finer_controls": "on", "antialiasing": "medium"})),
            ("a wide pinch, falloff 9", json!({"amount_1": -40, "amount_2": -40, "falloff": 9})),
        ],
        is_flow,
    );

    street(
        &mut t,
        "D-385",
        "core.flow_motion",
        &[
            ("as_added", json!({}), "as it starts: the street drawn in towards a point at the left and blown out round a point at the right", changed),
            ("amounts_0", json!({"amount_1": 0, "amount_2": 0}), "both amounts 0: nothing changes", unchanged),
            ("swell_middle", json!({"knot_1": [50, 50], "amount_1": -40, "amount_2": 0, "falloff": 3}), "-40 in the middle, falloff 3: a lens-like swell in the centre", changed),
            (
                "pinch_edges_off",
                json!({"knot_1": [50, 50], "amount_1": 40, "amount_2": 0, "falloff": 8, "tile_edges": "off", "antialiasing": "high"}),
                "40 in the middle, falloff 8, edges off: the street drawn in towards the centre, the edges left clear",
                changed_with_gaps,
            ),
        ],
    );

    t.finish("D-385_flow_motion_table.md");
}

// --- Griddler --------------------------------------------------------------------------------

fn is_griddler(e: &Effect) -> bool {
    matches!(e, Effect::Griddler { .. })
}

#[test]
fn b265_griddler() {
    let mut t = Table::new(
        "griddler",
        "# D-386: Griddler\n\nB-265, after CycoreFX's CC Griddler: the layer cut into square tiles \
         Tile Size per cent of its width, the picture inside each scaled across and down and \
         turned about the tile's own centre; Cut Tiles keeps each tile's picture inside its own \
         square. The formulas are this program's own. Every expected pixel is \
         `Fixtures/griddler/expected_griddler.json`, written by `tools/griddler_reference.py` \
         before this code existed and printed in document 25 as FX-GRIDDLER-001 to 022. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-GRIDDLER-001 to 022 (document 25)");
    t.fixtures("expected_griddler.json");

    t.heading("The file");
    let all = files("fx_griddler", 22);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_griddler_006.json");
    t.row(
        "fx_griddler_006.json is saved with its scales, tile size, rotation and cut tiles",
        &saved.to_string(),
        saved["horizontal_scale"] == 100 && saved["vertical_scale"] == 100 && saved["tile_size"] == 25 && saved["rotation"] == 45 && saved["cut_tiles"] == "off",
    );
    why(
        &mut t,
        &[
            ("fx_griddler_016.json", "Griddler's horizontal scale runs from -1000 to 1000, and this is 1001."),
            ("fx_griddler_017.json", "Griddler's vertical scale runs from -1000 to 1000, and this is -1001."),
            ("fx_griddler_018.json", "Griddler's tile size runs from 0.1 to 100, and this is 0."),
            ("fx_griddler_019.json", "Griddler's tile size runs from 0.1 to 100, and this is 101."),
            ("fx_griddler_020.json", "Griddler's rotation runs from -3600 to 3600, and this is 3601."),
            ("fx_griddler_021.json", "Griddler's cut tiles is \"on\" or \"off\", and this is \"yes\"."),
        ],
    );
    t.shape_refused("fx_griddler_001.json", "a Griddler with no `tile_size`", r#"{"horizontal_scale": 80, "vertical_scale": 80, "rotation": 0, "cut_tiles": "on"}"#);
    t.shape_refused("fx_griddler_001.json", "a Griddler whose rotation is a word", r#"{"horizontal_scale": 80, "vertical_scale": 80, "tile_size": 10, "rotation": "0", "cut_tiles": "on"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_griddler_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("horizontal scale 1000.5", set(griddler(1000.5, 80.0, 10.0, 0.0, "on"))),
            ("tile size 0.05", set(griddler(80.0, 80.0, 0.05, 0.0, "on"))),
            ("rotation -3600.5", set(griddler(80.0, 80.0, 10.0, -3600.5, "on"))),
            ("cut tiles \"On\", written with a capital", set(griddler(80.0, 80.0, 10.0, 0.0, "On"))),
            ("vertical scale keyed to 1500", keys("vertical_scale", &[(0, &[80.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_griddler_001.json",
        vec![
            ("60 across, -120 down, tiles of 20, turned 30, uncut", set(griddler(60.0, -120.0, 20.0, 30.0, "off"))),
            ("rotation keyed from 0 to 90", keys("rotation", &[(0, &[0.0]), (4, &[90.0])])),
            ("tile size keyed from 10 to 40", keys("tile_size", &[(0, &[10.0]), (4, &[40.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_griddler_001.json", 0), ("fx_griddler_004.json", 0), ("fx_griddler_005.json", 0), ("fx_griddler_006.json", 0), ("fx_griddler_009.json", 0), ("fx_griddler_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_griddler", 22, &[2, 16, 17, 18, 19, 20, 21, 22], is_griddler);
    card_reference(
        &mut t,
        &mut gpu,
        "Griddler",
        "core.griddler",
        &[
            ("as it starts (tiles of 10, 80 per cent)", json!({})),
            ("tiles of 5 turned 30, uncut", json!({"tile_size": 5, "rotation": 30, "cut_tiles": "off", "horizontal_scale": 100, "vertical_scale": 100})),
            ("tiles of 20, turned over across, 150 down", json!({"tile_size": 20, "horizontal_scale": -100, "vertical_scale": 150})),
            ("tiles of 12.5 at 40 per cent, turned -200", json!({"tile_size": 12.5, "horizontal_scale": 40, "vertical_scale": 40, "rotation": -200})),
        ],
        is_griddler,
    );

    street(
        &mut t,
        "D-386",
        "core.griddler",
        &[
            ("as_added", json!({}), "as it starts (tiles of 10, 80 per cent): the street in a grid of shrunken tiles with clear seams", changed_with_gaps),
            ("scale_100", json!({"horizontal_scale": 100, "vertical_scale": 100}), "100 per cent, unturned: nothing changes", unchanged),
            ("turned_45_uncut", json!({"tile_size": 20, "horizontal_scale": 100, "vertical_scale": 100, "rotation": 45, "cut_tiles": "off"}), "tiles of 20 turned 45, uncut: every tile holds its piece of the street turned on the slant", changed),
            ("mirrored_across", json!({"tile_size": 25, "horizontal_scale": -100, "vertical_scale": 100}), "tiles of 25 turned over across: each tile's piece of the street mirrored left to right", changed),
        ],
    );

    t.finish("D-386_griddler_table.md");
}

// --- Fisheye ---------------------------------------------------------------------------------

fn is_fisheye(e: &Effect) -> bool {
    matches!(e, Effect::Fisheye { .. })
}

#[test]
fn b266_fisheye() {
    let mut t = Table::new(
        "fisheye",
        "# D-387: Fisheye\n\nB-266, after CycoreFX's CC Lens: the layer seen through a round lens \
         Size per cent of half its diagonal across, bulging like a glass ball for a positive \
         Convergence and pinched like a dish for a negative one; outside the circle is left \
         clear, with a soft one-pixel rim. The formulas are this program's own. Every expected \
         pixel is `Fixtures/fisheye/expected_fisheye.json`, written by \
         `tools/fisheye_reference.py` before this code existed and printed in document 25 as \
         FX-FISHEYE-001 to 020. Tolerance 2e-5.\n",
    );

    t.heading("FX-FISHEYE-001 to 020 (document 25)");
    t.fixtures("expected_fisheye.json");

    t.heading("The file");
    let all = files("fx_fisheye", 20);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_fisheye_007.json");
    t.row(
        "fx_fisheye_007.json is saved with its centre, size and convergence",
        &saved.to_string(),
        saved["center"] == json!([25, 50]) && saved["size"] == 30 && saved["convergence"] == 50,
    );
    why(
        &mut t,
        &[
            ("fx_fisheye_015.json", "Fisheye's size runs from 0 to 1000, and this is 1001."),
            ("fx_fisheye_016.json", "Fisheye's size runs from 0 to 1000, and this is -1."),
            ("fx_fisheye_017.json", "Fisheye's convergence runs from -100 to 100, and this is 101."),
            ("fx_fisheye_018.json", "Fisheye's convergence runs from -100 to 100, and this is -101."),
            ("fx_fisheye_019.json", "Fisheye's center runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused("fx_fisheye_001.json", "a Fisheye with no `size`", r#"{"center": [50, 50], "convergence": 50}"#);
    t.shape_refused("fx_fisheye_001.json", "a Fisheye whose center is a word", r#"{"center": "50, 50", "size": 50, "convergence": 50}"#);

    t.heading("Commands");
    let mut document = t.load("fx_fisheye_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("size 1000.5", set(fisheye([50.0, 50.0], 1000.5, 50.0))),
            ("size -0.5", set(fisheye([50.0, 50.0], -0.5, 50.0))),
            ("convergence 100.5", set(fisheye([50.0, 50.0], 50.0, 100.5))),
            ("center 1000.5, 50", set(fisheye([1000.5, 50.0], 50.0, 50.0))),
            ("convergence keyed to -150", keys("convergence", &[(0, &[50.0]), (4, &[-150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fisheye_001.json",
        vec![
            ("centre 40, 60, size 80, convergence -70", set(fisheye([40.0, 60.0], 80.0, -70.0))),
            ("size keyed from 10 to 120", keys("size", &[(0, &[10.0]), (4, &[120.0])])),
            ("center keyed from 30, 50 to 70, 50", keys("center", &[(0, &[30.0, 50.0]), (4, &[70.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fisheye_001.json", 0), ("fx_fisheye_004.json", 0), ("fx_fisheye_005.json", 0), ("fx_fisheye_007.json", 0), ("fx_fisheye_012.json", 0), ("fx_fisheye_014.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_fisheye", 20, &[15, 16, 17, 18, 19, 20], is_fisheye);
    card_reference(
        &mut t,
        &mut gpu,
        "Fisheye",
        "core.fisheye",
        &[
            ("as it starts (size 50, convergence 50)", json!({})),
            ("a full ball, size 100, convergence 100", json!({"size": 100, "convergence": 100})),
            ("a dish at the left, size 70, convergence -100", json!({"center": [30, 40], "size": 70, "convergence": -100})),
            ("a small flat lens off the frame's edge", json!({"center": [95, 10], "size": 25, "convergence": 0})),
        ],
        is_fisheye,
    );

    street(
        &mut t,
        "D-387",
        "core.fisheye",
        &[
            ("as_added", json!({}), "as it starts (size 50, convergence 50): the middle of the street bulges in a round lens, the corners left clear", changed_with_gaps),
            ("ball_full", json!({"size": 120, "convergence": 100}), "size 120, convergence 100: the whole street wrapped round a glass ball", changed),
            ("dish", json!({"size": 100, "convergence": -100}), "size 100, convergence -100: the middle of the street pinched small, the rim stretched", changed),
        ],
    );

    t.finish("D-387_fisheye_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B264_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-264: a measurement, run deliberately with --release --ignored"]
fn b264_distort_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B264_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 5] = [
        ("Noise alone", None),
        ("Noise, then Flow Motion as it starts (10 and -10, low)", Some(("core.flow_motion", json!({})))),
        ("Noise, then Flow Motion with high antialiasing", Some(("core.flow_motion", json!({"antialiasing": "high"})))),
        ("Noise, then Griddler as it starts (tiles of 10, 80 per cent)", Some(("core.griddler", json!({})))),
        ("Noise, then Fisheye as it starts (size 50, convergence 50)", Some(("core.fisheye", json!({})))),
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
    let out = std::env::var("B264_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-264_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
