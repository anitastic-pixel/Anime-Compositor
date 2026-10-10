//! B-295: D-416 Fractal, after After Effects' Fractal ("Generate" in `docs/effects/EFFECTS.md`):
//! the Mandelbrot or Julia set, coloured by its escape counts, in place of the layer.
//!
//! Writes `verification/D-416_fractal_table.md`.
//!
//! Every expected pixel is `Fixtures/fractal/expected_fractal.json`, written by
//! `tools/fractal_reference.py` before this code existed and printed in document 25 as
//! FX-FRACTALSET-001 to 042. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-416 pictures/`.

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
        "core.fractal" => json!({"set_choice": "mandelbrot", "equation": "z2", "mandelbrot_center": [-0.75, 0], "mandelbrot_magnification": 0, "mandelbrot_escape_limit": 100, "julia_center": [0, 0], "julia_magnification": 0, "julia_escape_limit": 100, "overlay": "off", "transparency": "off", "palette": "lightness_gradient", "hue": 0, "cycle_steps": 10, "cycle_offset": 0, "edge_highlight": "off", "oversample_method": "edge_detect", "oversample_factor": 2}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b295-{id}"), p)]));
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

// --- Fractal --------------------------------------------------------------------------------

fn is_fractal(e: &Effect) -> bool {
    matches!(e, Effect::Fractal { .. })
}

/// A Fractal as it starts but for the set, the magnification, escape limit, hue, cycle steps
/// and oversample factor.
fn fractal(set_choice: &str, [mandelbrot_magnification, mandelbrot_escape_limit, hue, cycle_steps, oversample_factor]: [f64; 5]) -> Effect {
    let w = |s: &str| s.to_string();
    Effect::Fractal {
        set_choice: w(set_choice),
        equation: w("z2"),
        mandelbrot_center: [-0.75, 0.0],
        mandelbrot_magnification,
        mandelbrot_escape_limit,
        julia_center: [0.0, 0.0],
        julia_magnification: 0.0,
        julia_escape_limit: 100.0,
        overlay: w("off"),
        transparency: w("off"),
        palette: w("lightness_gradient"),
        hue,
        cycle_steps,
        cycle_offset: 0.0,
        edge_highlight: w("off"),
        oversample_method: w("edge_detect"),
        oversample_factor,
    }
}

const PLAIN: [f64; 5] = [0.0, 100.0, 0.0, 10.0, 2.0];

/// Changed, and some of it left clear.
fn gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

/// Changed, and every pixel solid.
fn solid(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).all(|p| p[3] == 255)
}

#[test]
fn b295_fractal() {
    let mut t = Table::new(
        "fractal",
        "# D-416: Fractal\n\nB-295, after After Effects' Fractal: the Mandelbrot or Julia set in place of \
         the layer. A pixel's point is the view's centre plus its distance from the drawing's middle \
         times 3 / (height 2^magnification); z^n + c is iterated until a part passes 2 or the Escape \
         Limit runs out (inside the set). The count picks a band: Lightness Gradient's 8 gradients of \
         Cycle Steps lightnesses, each 45 degrees of hue on, Hue Wheel's Cycle Steps hues, Black And \
         White's two, or Solid Color's one colour for the set. Edge Detect works a pixel Factor by \
         Factor where its band differs from a neighbour's, Brute Force works every pixel so; at \
         Factor 1 Edge Highlight whitens the pixels whose band differs from the left or above. \
         Overlay ghosts the other set and draws its centre's cross. Every expected pixel is \
         `Fixtures/fractal/expected_fractal.json`, written by `tools/fractal_reference.py` before this \
         code existed and printed in document 25 as FX-FRACTALSET-001 to 042. Tolerance 2e-5.\n",
    );

    t.heading("FX-FRACTALSET-001 to 042 (document 25)");
    t.fixtures("expected_fractal.json");

    t.heading("The file");
    let all = files("fx_fractal", 42);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_fractal_030.json");
    t.row(
        "fx_fractal_030.json is saved with its counts as written, 3.7 and 100.9, though only their whole parts count",
        &saved.to_string(),
        saved["cycle_steps"] == 3.7 && saved["mandelbrot_escape_limit"] == 100.9 && saved["mandelbrot_center"] == json!([-0.75, 0])
            && saved["set_choice"] == "mandelbrot" && saved["oversample_method"] == "edge_detect" && saved["oversample_factor"] == 2,
    );
    why(
        &mut t,
        &[
            (
                "fx_fractal_031.json",
                "Fractal's set choice is \"mandelbrot\", \"mandelbrot_inverse\", \"mandelbrot_over_julia\", \"mandelbrot_inverse_over_julia\", \"julia\" or \"julia_inverse\", and this is \"burning_ship\".",
            ),
            ("fx_fractal_032.json", "Fractal's equation is \"z2\", \"z3\", \"z4\", \"z5\" or \"z6\", and this is \"z7\"."),
            ("fx_fractal_033.json", "Fractal's mandelbrot escape limit runs from 1 to 10000, and this is 0."),
            ("fx_fractal_034.json", "Fractal's cycle offset runs from 0 to 1000, and this is 1001."),
            ("fx_fractal_035.json", "Fractal's mandelbrot magnification runs from -10 to 40, and this is 41."),
            ("fx_fractal_036.json", "Fractal's julia center runs from -10 to 10, and this is 11."),
            ("fx_fractal_037.json", "Fractal's palette is \"lightness_gradient\", \"hue_wheel\", \"black_and_white\" or \"solid_color\", and this is \"rainbow\"."),
            ("fx_fractal_038.json", "Fractal's cycle steps runs from 1 to 1000, and this is 0."),
            ("fx_fractal_039.json", "Fractal's oversample factor runs from 1 to 8, and this is 9."),
            ("fx_fractal_041.json", "Fractal's oversample method is \"edge_detect\" or \"brute_force\", and this is \"fast\"."),
            ("fx_fractal_042.json", "Fractal's transparency is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    let base = r##""set_choice": "mandelbrot", "equation": "z2", "mandelbrot_magnification": 0, "mandelbrot_escape_limit": 100, "julia_center": [0, 0], "julia_magnification": 0, "julia_escape_limit": 100, "overlay": "off", "transparency": "off", "palette": "lightness_gradient", "hue": 0, "cycle_steps": 10, "cycle_offset": 0, "edge_highlight": "off", "oversample_method": "edge_detect""##;
    t.shape_refused("fx_fractal_001.json", "a Fractal with no `oversample_factor`", &format!(r#"{{{base}, "mandelbrot_center": [-0.75, 0]}}"#));
    t.shape_refused("fx_fractal_001.json", "a Fractal with a factor in words", &format!(r#"{{{base}, "mandelbrot_center": [-0.75, 0], "oversample_factor": "two"}}"#));
    t.shape_refused("fx_fractal_001.json", "a Fractal whose centre is one number", &format!(r#"{{{base}, "mandelbrot_center": [-0.75], "oversample_factor": 2}}"#));

    t.heading("Commands");
    let mut document = t.load("fx_fractal_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("set choice \"burning_ship\"", set(fractal("burning_ship", PLAIN))),
            ("escape limit 0", set(fractal("mandelbrot", [0.0, 0.0, 0.0, 10.0, 2.0]))),
            ("oversample factor 9", set(fractal("mandelbrot", [0.0, 100.0, 0.0, 10.0, 9.0]))),
            ("magnification keyed to 41", keys("mandelbrot_magnification", &[(0, &[0.0]), (4, &[41.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_fractal_001.json",
        vec![
            ("Julia, hue 120, cycle steps 3, factor 1", set(fractal("julia", [0.0, 100.0, 120.0, 3.0, 1.0]))),
            ("magnification keyed from 0 to 4", keys("mandelbrot_magnification", &[(0, &[0.0]), (4, &[4.0])])),
            ("Mandelbrot centre keyed from -0.75, 0 to -0.75, 0.1", keys("mandelbrot_center", &[(0, &[-0.75, 0.0]), (4, &[-0.75, 0.1])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_fractal_001.json", 0), ("fx_fractal_002.json", 0), ("fx_fractal_004.json", 0), ("fx_fractal_013.json", 0), ("fx_fractal_020.json", 0), ("fx_fractal_027.json", 0), ("fx_fractal_028.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_fractal", 42, &[31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42], is_fractal);
    card_reference(
        &mut t,
        &mut gpu,
        "Fractal",
        "core.fractal",
        &[
            ("as it starts (the Mandelbrot set, Lightness Gradient, Edge Detect 2 by 2)", json!({})),
            ("Julia, Hue Wheel, hue 200", json!({"set_choice": "julia", "palette": "hue_wheel", "hue": 200})),
            ("Brute Force 4 by 4, Black And White", json!({"oversample_method": "brute_force", "oversample_factor": 4, "palette": "black_and_white"})),
            ("factor 1 with Edge Highlight, Transparency", json!({"oversample_factor": 1, "edge_highlight": "on", "transparency": "on"})),
            ("Overlay, Mandelbrot Over Julia from 0.3, 0.2", json!({"set_choice": "mandelbrot_over_julia", "julia_center": [0.3, 0.2], "overlay": "on"})),
            ("z^4, Julia Inverse, Solid Color", json!({"equation": "z4", "set_choice": "julia_inverse", "palette": "solid_color"})),
            ("magnification 30 at the seahorse valley, Escape Limit 500", json!({"mandelbrot_center": [-0.743643887037151, 0.131825904205330], "mandelbrot_magnification": 30, "mandelbrot_escape_limit": 500})),
            ("Mandelbrot Inverse at 0, 0, cycle steps 3, offset 2", json!({"set_choice": "mandelbrot_inverse", "mandelbrot_center": [0, 0], "cycle_steps": 3, "cycle_offset": 2})),
        ],
        is_fractal,
    );
    // D-416's guard: 1920 by 1080, 5 + 2 * 2 escapes a pixel, Escape Limit 10000, is 1.9e11
    // steps, past the card's 1e10; at 5, 0 every point escapes at once, so the processor is quick.
    let heavy = reference(|id| json!([fx("core.fractal", &format!("b295-{id}"), &json!({"mandelbrot_center": [5, 0], "mandelbrot_escape_limit": 10000}))]));
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (d, r, a, b) = both(&mut gpu, &heavy, &comp, &root, 100, quality);
        let card = on_card(&heavy, &comp, &root, 100, quality, is_fractal);
        t.row(
            &format!("the reference shot, Escape Limit 10000 on three layers (1.9e11 steps at Full, past the card's 1e10), frame 100, {}: drawn on the processor", quality.label()),
            &format!("largest difference {} of 255; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0),
            d.0 <= 1 && !r && a == b && card == 0,
        );
    }

    street(
        &mut t,
        "D-416",
        "core.fractal",
        &[
            ("as_added", json!({}), "as it starts: the Mandelbrot set, black, in red and orange bands, in place of the street", solid),
            ("julia_hue_wheel", json!({"set_choice": "julia", "palette": "hue_wheel"}), "the Julia set of -0.75, its outside going round the colour wheel", solid),
            ("seahorse", json!({"mandelbrot_center": [-0.743643887037151, 0.131825904205330], "mandelbrot_magnification": 12, "mandelbrot_escape_limit": 1000, "cycle_steps": 30}), "deep in the seahorse valley, magnification 12, Escape Limit 1000", solid),
            ("solid_clear", json!({"palette": "solid_color", "hue": 200}), "the set alone in blue, the rest clear", gaps),
            ("overlay", json!({"overlay": "on", "oversample_factor": 1, "edge_highlight": "on"}), "Edge Highlight on its bands, the Julia set ghosted and the white cross at its centre", solid),
        ],
    );

    t.finish("D-416_fractal_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B295_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-295: a measurement, run deliberately with --release --ignored"]
fn b295_fractal_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B295_CPU").is_ok();
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
        ("Noise, then Fractal as added (Escape Limit 100, Edge Detect 2 by 2)", Some(("core.fractal", json!({})))),
        ("Noise, then Fractal, Julia, Hue Wheel", Some(("core.fractal", json!({"set_choice": "julia", "palette": "hue_wheel"})))),
        ("Noise, then Fractal, Brute Force 4 by 4", Some(("core.fractal", json!({"oversample_method": "brute_force", "oversample_factor": 4})))),
        ("Noise, then Fractal, magnification 20 at the seahorse valley, Escape Limit 1000", Some(("core.fractal", json!({"mandelbrot_center": [-0.743643887037151, 0.131825904205330], "mandelbrot_magnification": 20, "mandelbrot_escape_limit": 1000})))),
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
    let out = std::env::var("B295_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-295_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
