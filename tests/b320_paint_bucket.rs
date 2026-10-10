//! B-320: D-440 Paint Bucket, after After Effects' Paint Bucket ("Generate" in
//! `docs/effects/EFFECTS.md`): the area of similar colour holding a point, filled with a colour.
//!
//! Writes `verification/D-440_paint_bucket_table.md`.
//!
//! Every expected pixel is `Fixtures/paint_bucket/expected_paint_bucket.json`, written by
//! `tools/paint_bucket_reference.py` before this code existed and printed in document 25 as
//! FX-PAINTBUCKET-001 to 045. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-440 pictures/`.

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

/// A Paint Bucket: the point, [tolerance, spread radius, stroke width, feather softness,
/// opacity], [selector, view threshold, stroke, invert fill, colour, blending mode].
fn bucket(fill_point: [f64; 2], [tolerance, spread_radius, stroke_width, feather_softness, opacity]: [f64; 5], words: [&str; 6]) -> Effect {
    let [fill_selector, view_threshold, stroke, invert_fill, color, blending_mode] = words.map(str::to_string);
    Effect::PaintBucket {
        fill_point,
        fill_selector,
        tolerance,
        view_threshold,
        stroke,
        invert_fill,
        spread_radius,
        stroke_width,
        feather_softness,
        color,
        opacity,
        blending_mode,
    }
}

const ADDED: [f64; 5] = [10.0, 3.0, 3.0, 10.0, 100.0];

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 6] {
    let mut w = ["color_and_alpha", "off", "antialias", "off", "#ff0000", "normal"];
    for &(i, v) in change {
        w[i] = v;
    }
    w
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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.paint_bucket" => json!({"fill_point": [50, 50], "fill_selector": "color_and_alpha", "tolerance": 10, "view_threshold": "off",
            "stroke": "antialias", "invert_fill": "off", "spread_radius": 3, "stroke_width": 3, "feather_softness": 10, "color": "#ff0000",
            "opacity": 100, "blending_mode": "normal"}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b320-{id}"), p)]));
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
            &format!("{said:?}, {} pixels changed of {}", distance(&bytes, &before).1, TOWN.0 * TOWN.1),
            said.is_empty() && check(&bytes, &before),
        );
    }
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

// --- Paint Bucket ---------------------------------------------------------------------------

fn is_bucket(e: &Effect) -> bool {
    matches!(e, Effect::PaintBucket { .. })
}

/// Some of the street changed and some of it did not: an area was filled, not the whole.
fn in_part(a: &[u8], b: &[u8]) -> bool {
    let changed = distance(a, b).1;
    changed > 1000 && changed < a.len() / 4 * 9 / 10
}

/// Every pixel opaque and either white or black, both showing.
fn two_tone(a: &[u8], _: &[u8]) -> bool {
    let white = a.chunks_exact(4).filter(|p| *p == [255, 255, 255, 255]).count();
    let black = a.chunks_exact(4).filter(|p| *p == [0, 0, 0, 255]).count();
    white > 0 && black > 0 && white + black == a.len() / 4
}

#[test]
fn b320_paint_bucket() {
    let mut t = Table::new(
        "paint_bucket",
        "# D-440: Paint Bucket\n\nB-320, after After Effects' Paint Bucket: the pixel holding the Fill \
         Point (per cent of the drawing); the pixels that match it within Tolerance by the Fill \
         Selector (Color & Alpha, Straight Color, Transparency, Opacity or Alpha Channel), joined to it \
         up, down, left and right (every match for Alpha Channel), turned over by Invert Fill; spread, \
         choked or stroked by distances between pixel centres; blurred by a box three pixels wide (or \
         Feather Softness's Gaussian for Feather), edges held; then the colour laid on by the blending \
         mode at Opacity (Fill Only the fill alone). View Threshold shows the matches white on black. \
         The layer never grows. Every expected pixel is `Fixtures/paint_bucket/expected_paint_bucket.json`, \
         written by `tools/paint_bucket_reference.py` before this code existed and printed in document \
         25 as FX-PAINTBUCKET-001 to 045. Tolerance 2e-5.\n",
    );

    t.heading("FX-PAINTBUCKET-001 to 045 (document 25)");
    t.fixtures_numbered("expected_paint_bucket.json", 1..=45);

    t.heading("The file");
    let all = files("fx_paintbucket", 45);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_paintbucket_018.json");
    t.row(
        "fx_paintbucket_018.json is saved with its words and numbers as written",
        &saved.to_string(),
        saved["fill_point"] == json!([50, 50]) && saved["fill_selector"] == "color_and_alpha" && saved["tolerance"] == 10
            && saved["view_threshold"] == "off" && saved["stroke"] == "antialias" && saved["invert_fill"] == "off"
            && saved["spread_radius"] == 3 && saved["stroke_width"] == 3 && saved["feather_softness"] == 10
            && saved["color"] == "#3080ff" && saved["opacity"] == 100 && saved["blending_mode"] == "multiply",
    );
    why(
        &mut t,
        &[
            ("fx_paintbucket_032.json", "Paint Bucket's fill point runs from -1000 to 1000, and this is 1001."),
            ("fx_paintbucket_033.json", "Paint Bucket's tolerance runs from 0 to 100, and this is -1."),
            ("fx_paintbucket_034.json", "Paint Bucket's tolerance runs from 0 to 100, and this is 101."),
            ("fx_paintbucket_035.json", "Paint Bucket's spread radius runs from 0 to 10000, and this is 10001."),
            ("fx_paintbucket_036.json", "Paint Bucket's stroke width runs from 0 to 10000, and this is -1."),
            ("fx_paintbucket_037.json", "Paint Bucket's feather softness runs from 0 to 10000, and this is 10001."),
            ("fx_paintbucket_038.json", "Paint Bucket's opacity runs from 0 to 100, and this is 101."),
            ("fx_paintbucket_039.json", "Paint Bucket's fill selector is one of color_and_alpha, straight_color, transparency, opacity, alpha_channel, and this is \"luma\"."),
            ("fx_paintbucket_040.json", "Paint Bucket's stroke is one of antialias, feather, spread, choke, stroke, and this is \"glow\"."),
            ("fx_paintbucket_041.json", "Paint Bucket's view threshold is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_paintbucket_042.json", "Paint Bucket's invert fill is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_paintbucket_043.json", "Paint Bucket's blending mode is one of normal, add, multiply, screen, overlay, soft_light, fill_only, and this is \"darken\"."),
            ("fx_paintbucket_044.json", "Paint Bucket's colour is written #rrggbb, and this is \"#12345\"."),
        ],
    );
    t.shape_refused("fx_paintbucket_001.json", "a Paint Bucket with no `tolerance`", r##"{"fill_point": [50, 50], "fill_selector": "color_and_alpha", "view_threshold": "off", "stroke": "antialias", "invert_fill": "off", "spread_radius": 3, "stroke_width": 3, "feather_softness": 10, "color": "#ff0000", "opacity": 100, "blending_mode": "normal"}"##);
    t.shape_refused("fx_paintbucket_001.json", "a Paint Bucket whose fill point is one number", r##"{"fill_point": 50, "fill_selector": "color_and_alpha", "tolerance": 10, "view_threshold": "off", "stroke": "antialias", "invert_fill": "off", "spread_radius": 3, "stroke_width": 3, "feather_softness": 10, "color": "#ff0000", "opacity": 100, "blending_mode": "normal"}"##);
    t.shape_refused("fx_paintbucket_001.json", "a Paint Bucket whose invert fill is true", r##"{"fill_point": [50, 50], "fill_selector": "color_and_alpha", "tolerance": 10, "view_threshold": "off", "stroke": "antialias", "invert_fill": true, "spread_radius": 3, "stroke_width": 3, "feather_softness": 10, "color": "#ff0000", "opacity": 100, "blending_mode": "normal"}"##);

    t.heading("Commands");
    let mut document = t.load("fx_paintbucket_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("fill point 1001, 50", set(bucket([1001.0, 50.0], ADDED, words(&[])))),
            ("tolerance 101", set(bucket([50.0, 50.0], [101.0, 3.0, 3.0, 10.0, 100.0], words(&[])))),
            ("spread radius -1", set(bucket([50.0, 50.0], [10.0, -1.0, 3.0, 10.0, 100.0], words(&[])))),
            ("fill selector \"luma\"", set(bucket([50.0, 50.0], ADDED, words(&[(0, "luma")])))),
            ("stroke \"glow\"", set(bucket([50.0, 50.0], ADDED, words(&[(2, "glow")])))),
            ("blending mode \"darken\"", set(bucket([50.0, 50.0], ADDED, words(&[(5, "darken")])))),
            ("colour \"red\"", set(bucket([50.0, 50.0], ADDED, words(&[(4, "red")])))),
            ("tolerance keyed to 101", keys("tolerance", &[(0, &[10.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_paintbucket_001.json",
        vec![
            (
                "point 15.625, 55, Straight Color, tolerance 20, Stroke width 2, blue, screen",
                set(bucket([15.625, 55.0], [20.0, 3.0, 2.0, 10.0, 100.0], words(&[(0, "straight_color"), (2, "stroke"), (4, "#3080ff"), (5, "screen")]))),
            ),
            ("fill point keyed from 50, 50 to 25, 50", keys("fill_point", &[(0, &[50.0, 50.0]), (4, &[25.0, 50.0])])),
            ("feather softness keyed from 0 to 6", keys("feather_softness", &[(0, &[0.0]), (4, &[6.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_paintbucket_001.json", 0),
        ("fx_paintbucket_005.json", 0),
        ("fx_paintbucket_012.json", 0),
        ("fx_paintbucket_013.json", 0),
        ("fx_paintbucket_022.json", 2),
        ("fx_paintbucket_026.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-PAINTBUCKET-021 (opacity 0) changes nothing and 032 to 045 are refused, so the card is
    // not asked.
    let none: Vec<u32> = [21].into_iter().chain(32..=45).collect();
    card_fixtures(&mut t, &mut gpu, "fx_paintbucket", 45, &none, is_bucket);
    card_reference(
        &mut t,
        &mut gpu,
        "Paint Bucket",
        "core.paint_bucket",
        &[
            ("as added (the middle)", json!({})),
            ("point 30, 40, Straight Color, tolerance 30, Feather 8, blue, multiply", json!({"fill_point": [30, 40], "fill_selector": "straight_color", "tolerance": 30, "stroke": "feather", "feather_softness": 8, "color": "#3080ff", "blending_mode": "multiply"})),
            ("Transparency at the corner, tolerance 50, Spread 6", json!({"fill_point": [1, 1], "fill_selector": "transparency", "tolerance": 50, "stroke": "spread", "spread_radius": 6})),
            ("Alpha Channel, Invert Fill, Fill Only, opacity 60", json!({"fill_selector": "alpha_channel", "invert_fill": "on", "blending_mode": "fill_only", "opacity": 60})),
            ("Opacity, Stroke width 4, View Threshold", json!({"fill_selector": "opacity", "stroke": "stroke", "stroke_width": 4, "view_threshold": "on"})),
            ("Choke 5, tolerance 40, soft light", json!({"stroke": "choke", "spread_radius": 5, "tolerance": 40, "blending_mode": "soft_light"})),
        ],
        is_bucket,
    );

    street(
        &mut t,
        "D-440",
        "core.paint_bucket",
        &[
            ("as_added", json!({}), "as added: the house wall at the middle filled red", in_part),
            (
                "road_feather_blue",
                json!({"fill_point": [12.5, 90], "stroke": "feather", "feather_softness": 6, "color": "#3080ff", "blending_mode": "multiply"}),
                "point on the road, Feather 6, blue, multiply: the road tinted blue, its markings and edge softened",
                in_part,
            ),
            ("sky_threshold", json!({"fill_point": [50, 5], "tolerance": 20, "view_threshold": "on"}), "point in the sky, tolerance 20, View Threshold: what matches white, the rest black", two_tone),
            (
                "sky_stroke",
                json!({"fill_point": [50, 5], "tolerance": 20, "stroke": "stroke", "stroke_width": 4, "color": "#ffffff"}),
                "point in the sky, tolerance 20, Stroke width 4, white: a white band round the edge of the sky",
                in_part,
            ),
        ],
    );

    t.finish("D-440_paint_bucket_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B320_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-320: a measurement, run deliberately with --release --ignored"]
fn b320_paint_bucket_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B320_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Paint Bucket as added", Some(json!({}))),
        ("Noise, then Paint Bucket, tolerance 30, Feather 20", Some(json!({"tolerance": 30, "stroke": "feather", "feather_softness": 20}))),
        ("Noise, then Paint Bucket, Alpha Channel, Choke 40", Some(json!({"fill_selector": "alpha_channel", "stroke": "choke", "spread_radius": 40}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.paint_bucket", &format!("{id}c"), p));
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
    let out = std::env::var("B320_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-320_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
