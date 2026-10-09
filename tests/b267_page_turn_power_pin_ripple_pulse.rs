//! B-267 to B-269: D-388 Page Turn, after CycoreFX's CC Page Turn; D-389 Power Pin, after
//! CycoreFX's CC Power Pin; D-390 Ripple Pulse, after CycoreFX's CC Ripple Pulse ("Distort" in
//! `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-388_page_turn_table.md`, `verification/D-389_power_pin_table.md` and
//! `verification/D-390_ripple_pulse_table.md`.
//!
//! Every expected pixel is `Fixtures/page_turn/expected_page_turn.json`,
//! `Fixtures/power_pin/expected_power_pin.json` or `Fixtures/ripple_pulse/expected_ripple_pulse.json`,
//! written by the matching `tools/*_reference.py` before this code existed and printed in
//! document 25 as FX-PAGETURN-001 to 027, FX-POWERPIN-001 to 025 and FX-RPULSE-001 to 019.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Each also draws the street into `verification/D-388 pictures/` (and D-389, D-390).

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
fn page(controls: &str, at: [f64; 2], direction: f64, radius: f64, light: f64, render: &str, back: &str, opacity: f64, paper: &str) -> Effect {
    Effect::PageTurn {
        controls: controls.into(),
        fold_position: at,
        fold_direction: direction,
        fold_radius: radius,
        light_direction: light,
        render: render.into(),
        back_page: J::from(back),
        back_opacity: opacity,
        paper_color: paper.into(),
        map: None,
    }
}

fn start_page() -> Effect {
    page("bottom_right", [75.0, 75.0], -60.0, 30.0, -45.0, "full", "", 100.0, "#f0f0f0")
}

fn pin(pins: [[f64; 2]; 4], perspective: f64, unstretch: &str, e: [f64; 4]) -> Effect {
    Effect::PowerPin {
        top_left: pins[0],
        top_right: pins[1],
        bottom_left: pins[2],
        bottom_right: pins[3],
        perspective,
        unstretch: unstretch.into(),
        expansion_top: e[0],
        expansion_left: e[1],
        expansion_right: e[2],
        expansion_bottom: e[3],
    }
}

const PINS: [[f64; 2]; 4] = [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]];

fn ripple(center: [f64; 2], pulse_level: f64, time_span: f64, amplitude: f64, bump: &str) -> Effect {
    Effect::RipplePulse { center, pulse_level, time_span, amplitude, render_bump_map: bump.into(), levels: Vec::new() }
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
        "core.page_turn" => json!({
            "controls": "bottom_right", "fold_position": [75, 75], "fold_direction": -60, "fold_radius": 30,
            "light_direction": -45, "render": "full", "back_page": "", "back_opacity": 100, "paper_color": "#f0f0f0"}),
        "core.power_pin" => json!({
            "top_left": [0, 0], "top_right": [100, 0], "bottom_left": [0, 100], "bottom_right": [100, 100],
            "perspective": 100, "unstretch": "off",
            "expansion_top": 0, "expansion_left": 0, "expansion_right": 0, "expansion_bottom": 0}),
        "core.ripple_pulse" => json!({"center": [50, 50], "pulse_level": 0, "time_span": 1, "amplitude": 10, "render_bump_map": "off"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A Pulse Level that has been moving since before frame 0, so every frame has a history: up
/// and down in straight lines, held flat from frame 120 to 200, then dropping and rising again.
fn moving_level() -> J {
    json!({"base": 0, "keyframes": [
        {"frame": -30, "value": 0, "interp": "linear"},
        {"frame": 30, "value": 200, "interp": "linear"},
        {"frame": 60, "value": -100, "interp": "linear"},
        {"frame": 120, "value": 300, "interp": "hold"},
        {"frame": 200, "value": 0, "interp": "linear"},
        {"frame": 260, "value": 250, "interp": "linear"}]})
}

/// Pulse Level dropped from 0 to 10 five frames before frame 0, so frame 0 holds one ring.
fn one_drop() -> J {
    json!({"base": 0, "keyframes": [{"frame": -6, "value": 0, "interp": "hold"}, {"frame": -5, "value": 10, "interp": "linear"}]})
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
        let project = reference(|id| json!([fx(type_id, &format!("b267-{id}"), p)]));
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

// --- Page Turn -------------------------------------------------------------------------------

fn is_page(e: &Effect) -> bool {
    matches!(e, Effect::PageTurn { .. })
}

#[test]
fn b267_page_turn() {
    let mut t = Table::new(
        "page_turn",
        "# D-388: Page Turn\n\nB-267, after CycoreFX's CC Page Turn: a corner (or, in Classic, a \
         straight fold at a chosen angle) turned over towards Fold Position, rolling round a \
         cylinder Fold Radius pixels across, the back drawn as the paper colour or a Back Page \
         layer at Back Opacity, shaded by Light Direction; Render picks the front, the back or \
         both. The formulas are this program's own. Every expected pixel is \
         `Fixtures/page_turn/expected_page_turn.json`, written by `tools/page_turn_reference.py` \
         before this code existed and printed in document 25 as FX-PAGETURN-001 to 027. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-PAGETURN-001 to 027 (document 25)");
    t.fixtures("expected_page_turn.json");

    t.heading("The file");
    let all = files("fx_pageturn", 27);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_pageturn_013.json");
    t.row(
        "fx_pageturn_013.json is saved with its controls, fold, light, render, back page, back opacity and paper colour",
        &saved.to_string(),
        saved["controls"] == "classic"
            && saved["fold_position"] == json!([50, 50])
            && saved["fold_direction"] == 90
            && saved["fold_radius"] == 0
            && saved["light_direction"] == -45
            && saved["render"] == "full"
            && saved["back_page"] == ""
            && saved["back_opacity"] == 100
            && saved["paper_color"] == "#2040a0",
    );
    why(
        &mut t,
        &[
            ("fx_pageturn_019.json", "Page Turn's controls is classic, top_left, top_right, bottom_left or bottom_right, and this is \"middle\"."),
            ("fx_pageturn_020.json", "Page Turn's render is full, front or back, and this is \"sides\"."),
            ("fx_pageturn_021.json", "Page Turn's fold radius runs from 0 to 1000, and this is 1001."),
            ("fx_pageturn_022.json", "Page Turn's fold radius runs from 0 to 1000, and this is -1."),
            ("fx_pageturn_023.json", "Page Turn's back opacity runs from 0 to 100, and this is 101."),
            ("fx_pageturn_024.json", "Page Turn's paper colour is written #rrggbb, and this is \"white\"."),
            ("fx_pageturn_025.json", "Page Turn's back page is the name of a layer of this composition, and this is 3."),
            ("fx_pageturn_026.json", "Page Turn's fold position runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused(
        "fx_pageturn_001.json",
        "a Page Turn with no `fold_radius`",
        r##"{"controls": "bottom_right", "fold_position": [75, 75], "fold_direction": -60, "light_direction": -45, "render": "full", "back_page": "", "back_opacity": 100, "paper_color": "#f0f0f0"}"##,
    );
    t.shape_refused(
        "fx_pageturn_001.json",
        "a Page Turn whose fold position is one number",
        r##"{"controls": "bottom_right", "fold_position": 75, "fold_direction": -60, "fold_radius": 30, "light_direction": -45, "render": "full", "back_page": "", "back_opacity": 100, "paper_color": "#f0f0f0"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_pageturn_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("fold radius 1000.5", set(page("bottom_right", [75.0, 75.0], -60.0, 1000.5, -45.0, "full", "", 100.0, "#f0f0f0"))),
            ("controls \"Classic\", written with a capital", set(page("Classic", [75.0, 75.0], -60.0, 30.0, -45.0, "full", "", 100.0, "#f0f0f0"))),
            ("render \"both\"", set(page("bottom_right", [75.0, 75.0], -60.0, 30.0, -45.0, "both", "", 100.0, "#f0f0f0"))),
            ("paper colour \"#fff\"", set(page("bottom_right", [75.0, 75.0], -60.0, 30.0, -45.0, "full", "", 100.0, "#fff"))),
            ("back opacity keyed to 150", keys("back_opacity", &[(0, &[100.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_pageturn_001.json",
        vec![
            ("classic at 40, 60, turned -30, radius 12, lit from 60, front only, opacity 70, cream paper", set(page("classic", [40.0, 60.0], -30.0, 12.0, 60.0, "front", "", 70.0, "#eeddcc"))),
            ("as it starts again", set(start_page())),
            ("fold position keyed from 100, 100 to 0, 0", keys("fold_position", &[(0, &[100.0, 100.0]), (4, &[0.0, 0.0])])),
            ("fold radius keyed from 0 to 50", keys("fold_radius", &[(0, &[0.0]), (4, &[50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_pageturn_002.json", 0), ("fx_pageturn_005.json", 0), ("fx_pageturn_008.json", 0), ("fx_pageturn_009.json", 0), ("fx_pageturn_012.json", 0), ("fx_pageturn_015.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_pageturn", 27, &[19, 20, 21, 22, 23, 24, 25, 26, 27], is_page);
    card_reference(
        &mut t,
        &mut gpu,
        "Page Turn",
        "core.page_turn",
        &[
            ("as it starts (bottom right to 75, 75, radius 30)", json!({})),
            ("classic down the middle, radius 80, lit from the right", json!({"controls": "classic", "fold_position": [50, 50], "fold_direction": 90, "fold_radius": 80, "light_direction": 90})),
            ("the top left corner to 40, 30, radius 5, blue paper at 60", json!({"controls": "top_left", "fold_position": [40, 30], "fold_radius": 5, "back_opacity": 60, "paper_color": "#2040a0"})),
            ("classic turned -20, radius 200, back only", json!({"controls": "classic", "fold_position": [60, 50], "fold_direction": -20, "fold_radius": 200, "render": "back"})),
            ("the bottom right corner to the middle, its back the fourth layer", json!({"fold_position": [50, 50], "fold_radius": 20, "back_page": "layer-4"})),
        ],
        is_page,
    );

    street(
        &mut t,
        "D-388",
        "core.page_turn",
        &[
            ("as_added", json!({}), "as it starts: the bottom right corner of the street curling up and over, the paper's back showing, the corner left clear", changed_with_gaps),
            (
                "half_flat",
                json!({"controls": "classic", "fold_position": [50, 50], "fold_direction": 90, "fold_radius": 0}),
                "classic down the middle, radius 0: the left half folded flat over the right, its back the pale paper; the left half clear",
                changed_with_gaps,
            ),
            (
                "corner_blue",
                json!({"fold_position": [55, 55], "fold_radius": 15, "paper_color": "#2040a0", "light_direction": 90}),
                "the bottom right corner turned to the middle on blue paper, lit from the right",
                changed_with_gaps,
            ),
            (
                "seen_through",
                json!({"controls": "classic", "fold_position": [60, 50], "fold_direction": 70, "fold_radius": 25, "back_opacity": 0}),
                "classic, back opacity 0: the turned part shows the street through itself, mirrored",
                changed_with_gaps,
            ),
        ],
    );

    t.finish("D-388_page_turn_table.md");
}

// --- Power Pin -------------------------------------------------------------------------------

fn is_pin(e: &Effect) -> bool {
    matches!(e, Effect::PowerPin { .. })
}

#[test]
fn b268_power_pin() {
    let mut t = Table::new(
        "power_pin",
        "# D-389: Power Pin\n\nB-268, after CycoreFX's CC Power Pin: the four corners pinned as \
         Corner Pin pins them, Perspective easing from a flat stretch (0) to full perspective \
         (100), the pinned shape grown or shrunk past its sides by the four expansions, and \
         Unstretch drawing the other way round, the pinned shape stretched back out to fill the \
         layer. The formulas are this program's own. Every expected pixel is \
         `Fixtures/power_pin/expected_power_pin.json`, written by `tools/power_pin_reference.py` \
         before this code existed and printed in document 25 as FX-POWERPIN-001 to 025. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-POWERPIN-001 to 025 (document 25)");
    t.fixtures("expected_power_pin.json");

    t.heading("The file");
    let all = files("fx_powerpin", 25);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_powerpin_007.json");
    t.row(
        "fx_powerpin_007.json is saved with its four pins, perspective, unstretch and expansions",
        &saved.to_string(),
        saved["top_left"] == json!([25, 0])
            && saved["top_right"] == json!([75, 0])
            && saved["bottom_left"] == json!([0, 100])
            && saved["bottom_right"] == json!([100, 100])
            && saved["perspective"] == 100
            && saved["unstretch"] == "on"
            && saved["expansion_top"] == 0
            && saved["expansion_bottom"] == 0,
    );
    why(
        &mut t,
        &[
            ("fx_powerpin_019.json", "Power Pin's perspective runs from 0 to 100, and this is 101."),
            ("fx_powerpin_020.json", "Power Pin's perspective runs from 0 to 100, and this is -1."),
            ("fx_powerpin_021.json", "Power Pin's expansion top runs from -40 to 100, and this is 101."),
            ("fx_powerpin_022.json", "Power Pin's expansion left runs from -40 to 100, and this is -41."),
            ("fx_powerpin_023.json", "Power Pin's unstretch is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_powerpin_024.json", "Power Pin's top left runs from -400 to 500, and this is -401."),
        ],
    );
    t.shape_refused(
        "fx_powerpin_001.json",
        "a Power Pin with no `perspective`",
        r#"{"top_left": [0, 0], "top_right": [100, 0], "bottom_left": [0, 100], "bottom_right": [100, 100], "unstretch": "off", "expansion_top": 0, "expansion_left": 0, "expansion_right": 0, "expansion_bottom": 0}"#,
    );
    t.shape_refused(
        "fx_powerpin_001.json",
        "a Power Pin whose top right is three numbers",
        r#"{"top_left": [0, 0], "top_right": [100, 0, 0], "bottom_left": [0, 100], "bottom_right": [100, 100], "perspective": 100, "unstretch": "off", "expansion_top": 0, "expansion_left": 0, "expansion_right": 0, "expansion_bottom": 0}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_powerpin_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("perspective 100.5", set(pin(PINS, 100.5, "off", [0.0; 4]))),
            ("expansion bottom -40.5", set(pin(PINS, 100.0, "off", [0.0, 0.0, 0.0, -40.5]))),
            ("bottom left at 0, 500.5", set(pin([PINS[0], PINS[1], [0.0, 500.5], PINS[3]], 100.0, "off", [0.0; 4]))),
            ("unstretch \"On\", written with a capital", set(pin(PINS, 100.0, "On", [0.0; 4]))),
            ("expansion right keyed to 150", keys("expansion_right", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_powerpin_001.json",
        vec![
            (
                "pins at 10, 5 / 90, 0 / 0, 100 / 100, 90, perspective 50, unstretched, expansions 10, -5, 20, 0",
                set(pin([[10.0, 5.0], [90.0, 0.0], [0.0, 100.0], [100.0, 90.0]], 50.0, "on", [10.0, -5.0, 20.0, 0.0])),
            ),
            ("perspective keyed from 0 to 100", keys("perspective", &[(0, &[0.0]), (4, &[100.0])])),
            ("top right keyed from 100, 0 to 100, 50", keys("top_right", &[(0, &[100.0, 0.0]), (4, &[100.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_powerpin_002.json", 0), ("fx_powerpin_003.json", 0), ("fx_powerpin_006.json", 0), ("fx_powerpin_010.json", 0), ("fx_powerpin_013.json", 0), ("fx_powerpin_016.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_powerpin", 25, &[1, 19, 20, 21, 22, 23, 24, 25], is_pin);
    card_reference(
        &mut t,
        &mut gpu,
        "Power Pin",
        "core.power_pin",
        &[
            ("a keystone, top pins at 25, 0 and 75, 0", json!({"top_left": [25, 0], "top_right": [75, 0]})),
            ("the same keystone at perspective 30, expanded 20 at the top", json!({"top_left": [25, 0], "top_right": [75, 0], "perspective": 30, "expansion_top": 20})),
            ("pins pulled past the frame, -20, -10 and 110, 120", json!({"top_left": [-20, -10], "bottom_right": [110, 120]})),
            ("unstretched from a tilted square", json!({"top_left": [10, 5], "top_right": [90, 0], "bottom_right": [100, 90], "unstretch": "on"})),
        ],
        is_pin,
    );

    street(
        &mut t,
        "D-389",
        "core.power_pin",
        &[
            ("as_added", json!({}), "as it starts, each pin at its own corner: nothing changes", unchanged),
            (
                "keystone",
                json!({"top_left": [25, 0], "top_right": [75, 0]}),
                "a keystone: the street leaning back, its top narrower, the top corners clear",
                changed_with_gaps,
            ),
            (
                "keystone_flat",
                json!({"top_left": [25, 0], "top_right": [75, 0], "perspective": 0}),
                "the same keystone at perspective 0: squeezed evenly instead",
                changed_with_gaps,
            ),
            (
                "expanded",
                json!({"top_left": [25, 25], "top_right": [75, 25], "bottom_left": [25, 75], "bottom_right": [75, 75], "expansion_left": 50, "expansion_right": 50}),
                "pinned small in the middle, expanded 50 left and right: a wide band across the middle",
                changed_with_gaps,
            ),
            (
                "unstretched",
                json!({"top_left": [25, 0], "top_right": [75, 0], "unstretch": "on"}),
                "unstretch with the keystone: the street stretched the other way, its top widened",
                changed,
            ),
        ],
    );

    t.finish("D-389_power_pin_table.md");
}

// --- Ripple Pulse ----------------------------------------------------------------------------

fn is_ripple(e: &Effect) -> bool {
    matches!(e, Effect::RipplePulse { .. })
}

#[test]
fn b269_ripple_pulse() {
    let mut t = Table::new(
        "ripple_pulse",
        "# D-390: Ripple Pulse\n\nB-269, after CycoreFX's CC Ripple Pulse: every change in Pulse \
         Level sends a ring out from the centre that reaches the corners after Time Span seconds, \
         pushing the picture outward (or drawing it in) Amplitude pixels per unit of change; \
         Render Bump Map draws the heights in grey instead. The formulas are this program's own. \
         Every expected pixel is `Fixtures/ripple_pulse/expected_ripple_pulse.json`, written by \
         `tools/ripple_pulse_reference.py` before this code existed and printed in document 25 \
         as FX-RPULSE-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-RPULSE-001 to 019 (document 25)");
    t.fixtures("expected_ripple_pulse.json");

    t.heading("The file");
    let all = files("fx_rpulse", 19);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_rpulse_005.json");
    t.row(
        "fx_rpulse_005.json is saved with its centre, keyed pulse level, time span, amplitude and bump map",
        &saved.to_string(),
        saved["center"] == json!([25, 50])
            && saved["pulse_level"]["keyframes"][1]["value"] == 10
            && saved["time_span"] == 0.25
            && saved["amplitude"] == 2
            && saved["render_bump_map"] == "off",
    );
    why(
        &mut t,
        &[
            ("fx_rpulse_013.json", "Ripple Pulse's pulse level runs from -1000 to 1000, and this is 1001."),
            ("fx_rpulse_014.json", "Ripple Pulse's time span runs from 0 to 10, and this is 11."),
            ("fx_rpulse_015.json", "Ripple Pulse's time span runs from 0 to 10, and this is -1."),
            ("fx_rpulse_016.json", "Ripple Pulse's amplitude runs from 0 to 1000, and this is 1001."),
            ("fx_rpulse_017.json", "Ripple Pulse's render bump map is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_rpulse_018.json", "Ripple Pulse's center runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused(
        "fx_rpulse_001.json",
        "a Ripple Pulse with no `time_span`",
        r#"{"center": [50, 50], "pulse_level": 0, "amplitude": 10, "render_bump_map": "off"}"#,
    );
    t.shape_refused(
        "fx_rpulse_001.json",
        "a Ripple Pulse whose amplitude is a word",
        r#"{"center": [50, 50], "pulse_level": 0, "time_span": 1, "amplitude": "10", "render_bump_map": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rpulse_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("pulse level -1000.5", set(ripple([50.0, 50.0], -1000.5, 1.0, 10.0, "off"))),
            ("time span 10.5", set(ripple([50.0, 50.0], 0.0, 10.5, 10.0, "off"))),
            ("amplitude -0.5", set(ripple([50.0, 50.0], 0.0, 1.0, -0.5, "off"))),
            ("render bump map \"On\", written with a capital", set(ripple([50.0, 50.0], 0.0, 1.0, 10.0, "On"))),
            ("pulse level keyed to 1500", keys("pulse_level", &[(0, &[0.0]), (4, &[1500.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rpulse_001.json",
        vec![
            ("centre 40, 60, level 30, span 0.5, amplitude 25, bump map", set(ripple([40.0, 60.0], 30.0, 0.5, 25.0, "on"))),
            ("pulse level keyed from 0 to 20", keys("pulse_level", &[(0, &[0.0]), (4, &[20.0])])),
            ("center keyed from 30, 50 to 70, 50", keys("center", &[(0, &[30.0, 50.0]), (4, &[70.0, 50.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rpulse_002.json", 4), ("fx_rpulse_003.json", 4), ("fx_rpulse_004.json", 4), ("fx_rpulse_005.json", 4), ("fx_rpulse_006.json", 4), ("fx_rpulse_008.json", 4)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_rpulse", 19, &[1, 7, 10, 13, 14, 15, 16, 17, 18, 19], is_ripple);
    card_reference(
        &mut t,
        &mut gpu,
        "Ripple Pulse",
        "core.ripple_pulse",
        &[
            ("with a moving pulse level, time span 1, amplitude 10", json!({"pulse_level": moving_level()})),
            ("the same, time span 0.25, amplitude 40, off centre", json!({"pulse_level": moving_level(), "time_span": 0.25, "amplitude": 40, "center": [30, 60]})),
            ("the same as a bump map, amplitude 3", json!({"pulse_level": moving_level(), "amplitude": 3, "render_bump_map": "on"})),
        ],
        is_ripple,
    );

    street(
        &mut t,
        "D-390",
        "core.ripple_pulse",
        &[
            ("as_added", json!({}), "as it starts, the level never moved: nothing changes", unchanged),
            (
                "rings",
                json!({"pulse_level": one_drop(), "time_span": 0.5, "amplitude": 10}),
                "a drop five frames ago, time span 0.5, amplitude 10: one ring, part way out, bending the street outward",
                changed,
            ),
            (
                "bump_map",
                json!({"pulse_level": one_drop(), "time_span": 0.5, "amplitude": 10, "render_bump_map": "on"}),
                "the same as a bump map: the ring drawn in grey instead of the street",
                changed,
            ),
        ],
    );

    t.finish("D-390_ripple_pulse_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B267_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-267: a measurement, run deliberately with --release --ignored"]
fn b267_distort_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B267_CPU").is_ok();
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
        ("Noise, then Page Turn as it starts (bottom right to 75, 75, radius 30)", Some(("core.page_turn", json!({})))),
        ("Noise, then Power Pin, a keystone (25, 0 and 75, 0)", Some(("core.power_pin", json!({"top_left": [25, 0], "top_right": [75, 0]})))),
        ("Noise, then Ripple Pulse, a moving level, time span 1", Some(("core.ripple_pulse", json!({"pulse_level": moving_level()})))),
        ("Noise, then Ripple Pulse as a bump map", Some(("core.ripple_pulse", json!({"pulse_level": moving_level(), "render_bump_map": "on"})))),
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
    let out = std::env::var("B267_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-267_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
