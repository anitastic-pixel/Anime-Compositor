//! B-293: D-414 Circle, after After Effects' Circle ("Generate" in `docs/effects/EFFECTS.md`):
//! a disk or ring about a centre, its edges feathered, inverted if asked, laid in place of the
//! layer or on it by a blending mode.
//!
//! Writes `verification/D-414_circle_table.md`.
//!
//! Every expected pixel is `Fixtures/circle/expected_circle.json`, written by
//! `tools/circle_reference.py` before this code existed and printed in document 25 as
//! FX-CIRCLE-001 to 040. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-414 pictures/`.

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

fn circle(center: [f64; 2], [radius, edge_thickness, feather_outer, feather_inner, opacity]: [f64; 5], words: [&str; 4]) -> Effect {
    let [edge, invert, color, blending_mode] = words.map(str::to_string);
    Effect::Circle { center, radius, edge, edge_thickness, feather_outer, feather_inner, invert, color, opacity, blending_mode }
}

const PLAIN: [&str; 4] = ["none", "off", "#ffffff", "none"];

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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.circle" => json!({"center": [50, 50], "radius": 75, "edge": "none", "edge_thickness": 10, "feather_outer": 0, "feather_inner": 0, "invert": "off", "color": "#ffffff", "opacity": 100, "blending_mode": "none"}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b293-{id}"), p)]));
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

// --- Circle ---------------------------------------------------------------------------------

fn is_circle(e: &Effect) -> bool {
    matches!(e, Effect::Circle { .. })
}

/// Changed, and some of it left clear.
fn gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

/// Changed, and nothing clear that was not clear before.
fn kept(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] > 0 || q[3] == 0)
}

#[test]
fn b293_circle() {
    let mut t = Table::new(
        "circle",
        "# D-414: Circle\n\nB-293, after After Effects' Circle: a ring from Ri to Ro about Center, a share \
         of the drawing's own size (None: Radius and 0; Edge Radius: the larger and smaller of \
         Radius and Edge Radius; Thickness: Radius and Radius - Thickness, the ring inside the \
         radius; Thickness * Radius: the thickness Thickness times Radius / 100; Thickness & \
         Feather * Radius: the feathers too). The covering is a straight ramp max(feather, 1) \
         pixels wide centred on Ro, times one on Ri when Ri > 0, turned over by Invert Circle. The \
         shape, Color times the covering times Opacity, replaces the layer for None or is laid on \
         it by document 21's layer blend (Normal, Add, Multiply, Screen, Overlay, Soft Light, \
         Stencil Alpha). The layer never grows. Every expected pixel is \
         `Fixtures/circle/expected_circle.json`, written by `tools/circle_reference.py` before \
         this code existed and printed in document 25 as FX-CIRCLE-001 to 040. Tolerance 2e-5.\n",
    );

    t.heading("FX-CIRCLE-001 to 040 (document 25)");
    t.fixtures("expected_circle.json");

    t.heading("The file");
    // FX-CIRCLE-025's colour is written in capitals and saved in small letters, as every colour is.
    let all: Vec<String> = files("fx_circle", 40).into_iter().filter(|f| f != "fx_circle_025.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_circle_025.json");
    t.row(
        "fx_circle_025.json is saved with its colour in small letters, its words and numbers as written",
        &saved.to_string(),
        saved["color"] == "#ff8000" && saved["opacity"] == 50 && saved["radius"] == 4 && saved["center"] == json!([50, 50])
            && saved["edge"] == "none" && saved["invert"] == "off" && saved["blending_mode"] == "none" && saved["edge_thickness"] == 10,
    );
    why(
        &mut t,
        &[
            ("fx_circle_030.json", "Circle's radius runs from 0 to 10000, and this is -1."),
            ("fx_circle_031.json", "Circle's radius runs from 0 to 10000, and this is 10001."),
            (
                "fx_circle_032.json",
                "Circle's edge is \"none\", \"edge_radius\", \"thickness\", \"thickness_radius\" or \"thickness_feather_radius\", and this is \"ring\".",
            ),
            ("fx_circle_033.json", "Circle's edge thickness runs from 0 to 10000, and this is -1."),
            ("fx_circle_034.json", "Circle's feather outer runs from 0 to 10000, and this is 10001."),
            ("fx_circle_035.json", "Circle's opacity runs from 0 to 100, and this is 101."),
            ("fx_circle_036.json", "Circle's center runs from -1000 to 1000, and this is 1001."),
            ("fx_circle_037.json", "Circle's invert is \"off\" or \"on\", and this is \"yes\"."),
            (
                "fx_circle_038.json",
                "Circle's blending mode is \"none\", \"normal\", \"add\", \"multiply\", \"screen\", \"overlay\", \"soft_light\" or \"stencil_alpha\", and this is \"darken\".",
            ),
            ("fx_circle_039.json", "Circle's colour is written #rrggbb, and this is \"#12345\"."),
        ],
    );
    t.shape_refused("fx_circle_001.json", "a Circle with no `edge`", r##"{"center": [50, 50], "radius": 75, "edge_thickness": 10, "feather_outer": 0, "feather_inner": 0, "invert": "off", "color": "#ffffff", "opacity": 100, "blending_mode": "none"}"##);
    t.shape_refused("fx_circle_001.json", "a Circle with a radius in words", r##"{"center": [50, 50], "radius": "wide", "edge": "none", "edge_thickness": 10, "feather_outer": 0, "feather_inner": 0, "invert": "off", "color": "#ffffff", "opacity": 100, "blending_mode": "none"}"##);
    t.shape_refused("fx_circle_001.json", "a Circle whose centre is one number", r##"{"center": [50], "radius": 75, "edge": "none", "edge_thickness": 10, "feather_outer": 0, "feather_inner": 0, "invert": "off", "color": "#ffffff", "opacity": 100, "blending_mode": "none"}"##);

    t.heading("Commands");
    let mut document = t.load("fx_circle_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius -1", set(circle([50.0, 50.0], [-1.0, 10.0, 0.0, 0.0, 100.0], PLAIN))),
            ("edge \"ring\"", set(circle([50.0, 50.0], [75.0, 10.0, 0.0, 0.0, 100.0], ["ring", "off", "#ffffff", "none"]))),
            ("invert \"yes\"", set(circle([50.0, 50.0], [75.0, 10.0, 0.0, 0.0, 100.0], ["none", "yes", "#ffffff", "none"]))),
            ("blending mode \"darken\"", set(circle([50.0, 50.0], [75.0, 10.0, 0.0, 0.0, 100.0], ["none", "off", "#ffffff", "darken"]))),
            ("colour \"white\"", set(circle([50.0, 50.0], [75.0, 10.0, 0.0, 0.0, 100.0], ["none", "off", "white", "none"]))),
            ("radius keyed to 10001", keys("radius", &[(0, &[75.0]), (4, &[10001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_circle_001.json",
        vec![
            ("radius 3, Thickness 1, feathers 2 and 1, inverted, orange at 60, Multiply", set(circle([50.0, 50.0], [3.0, 1.0, 2.0, 1.0, 60.0], ["thickness", "on", "#ff8000", "multiply"]))),
            ("centre keyed from 50, 50 to 25, 50", keys("center", &[(0, &[50.0, 50.0]), (4, &[25.0, 50.0])])),
            ("radius keyed from 2 to 6", keys("radius", &[(0, &[2.0]), (4, &[6.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_circle_001.json", 0), ("fx_circle_002.json", 0), ("fx_circle_008.json", 0), ("fx_circle_011.json", 0), ("fx_circle_016.json", 0), ("fx_circle_021.json", 2), ("fx_circle_024.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_circle", 40, &[20, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40], is_circle);
    card_reference(
        &mut t,
        &mut gpu,
        "Circle",
        "core.circle",
        &[
            ("as it starts (a white disk of radius 75 in the middle, None)", json!({})),
            ("radius 200, feather outer 40, orange at 70 per cent, Normal", json!({"radius": 200, "feather_outer": 40, "color": "#ff8000", "opacity": 70, "blending_mode": "normal"})),
            ("Edge Radius 150 and 90, feathers 10 and 20, violet, Multiply", json!({"radius": 150, "edge": "edge_radius", "edge_thickness": 90, "feather_outer": 10, "feather_inner": 20, "color": "#6450a0", "blending_mode": "multiply"})),
            ("Thickness 30 at radius 120, centre 30, 70, Overlay", json!({"radius": 120, "edge": "thickness", "edge_thickness": 30, "center": [30, 70], "color": "#3070d0", "blending_mode": "overlay"})),
            ("Thickness * Radius 25 at radius 160, Soft Light", json!({"radius": 160, "edge": "thickness_radius", "edge_thickness": 25, "color": "#3070d0", "blending_mode": "soft_light"})),
            ("Thickness & Feather * Radius 20, feathers 10 and 5, Screen", json!({"radius": 180, "edge": "thickness_feather_radius", "edge_thickness": 20, "feather_outer": 10, "feather_inner": 5, "color": "#6450a0", "blending_mode": "screen"})),
            ("radius 220 inverted, feather 60, Add at 50 per cent", json!({"radius": 220, "invert": "on", "feather_outer": 60, "color": "#6450a0", "blending_mode": "add", "opacity": 50})),
            ("radius 250 inverted, feather 80, Stencil Alpha", json!({"radius": 250, "invert": "on", "feather_outer": 80, "blending_mode": "stencil_alpha"})),
        ],
        is_circle,
    );

    street(
        &mut t,
        "D-414",
        "core.circle",
        &[
            ("as_added", json!({}), "as it starts: a white disk of radius 75 in the middle in place of the street, the rest clear", gaps),
            ("normal_feathered", json!({"radius": 140, "feather_outer": 50, "color": "#ff8000", "opacity": 60, "blending_mode": "normal"}), "Normal, an orange disk at 60 per cent, radius 140 with a soft edge 50 wide, over the street", kept),
            ("ring_multiply", json!({"radius": 120, "edge": "thickness", "edge_thickness": 25, "feather_outer": 6, "feather_inner": 6, "color": "#6450a0", "blending_mode": "multiply"}), "Thickness 25 at radius 120, violet, Multiply: a dark ring on the street", kept),
            ("vignette", json!({"radius": 160, "invert": "on", "feather_outer": 120, "color": "#000000", "opacity": 80, "blending_mode": "normal"}), "inverted, black at 80 per cent, feather 120: a vignette darkening the corners", kept),
            ("stencil", json!({"radius": 130, "feather_outer": 40, "blending_mode": "stencil_alpha"}), "Stencil Alpha, radius 130, feather 40: the street seen only through the disk", gaps),
        ],
    );

    t.finish("D-414_circle_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B293_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-293: a measurement, run deliberately with --release --ignored"]
fn b293_circle_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B293_CPU").is_ok();
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
        ("Noise, then Circle as added (radius 75, None)", Some(("core.circle", json!({})))),
        ("Noise, then Circle, radius 200, feather 40, orange at 70, Normal", Some(("core.circle", json!({"radius": 200, "feather_outer": 40, "color": "#ff8000", "opacity": 70, "blending_mode": "normal"})))),
        ("Noise, then Circle, Thickness 30 at radius 120, Overlay", Some(("core.circle", json!({"radius": 120, "edge": "thickness", "edge_thickness": 30, "color": "#3070d0", "blending_mode": "overlay"})))),
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
    let out = std::env::var("B293_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-293_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
