//! B-321: D-441 Write-on, after After Effects' Write-on ("Generate" in
//! `docs/effects/EFFECTS.md`): a brush that paints wherever its Brush Position has been.
//!
//! Writes `verification/D-441_write_on_table.md`.
//!
//! Every expected pixel is `Fixtures/writeon/expected_writeon.json`, written by
//! `tools/writeon_reference.py` before this code existed and printed in document 25 as
//! FX-WRITEON-001 to 041. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-441 pictures/`.

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

/// A Write-on: the position, [size, hardness, opacity, stroke length, spacing], [colour, paint
/// time properties, brush time properties, paint style].
fn write_on(brush_position: [f64; 2], [brush_size, brush_hardness, brush_opacity, stroke_length, brush_spacing]: [f64; 5], words: [&str; 4]) -> Effect {
    let [color, paint_time_properties, brush_time_properties, paint_style] = words.map(str::to_string);
    Effect::WriteOn {
        brush_position,
        color,
        brush_size,
        brush_hardness,
        brush_opacity,
        stroke_length,
        brush_spacing,
        paint_time_properties,
        brush_time_properties,
        paint_style,
        marks: Vec::new(),
    }
}

const ADDED: [f64; 5] = [6.0, 75.0, 100.0, 0.0, 0.01];

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 4] {
    let mut w = ["#ffffff", "none", "none", "on_original"];
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
        "core.write_on" => json!({"brush_position": [50, 50], "color": "#ffffff", "brush_size": 6, "brush_hardness": 75, "brush_opacity": 100,
            "stroke_length": 0, "brush_spacing": 0.01, "paint_time_properties": "none", "brush_time_properties": "none", "paint_style": "on_original"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
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
        let project = reference(|id| json!([fx(type_id, &format!("b321-{id}"), p)]));
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

/// `effects` on the street at `frame` of a two-second shot, drawn, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

type Shot<'a> = (&'a str, J, i32, &'a str, fn(&[u8], &[u8]) -> bool);

/// The street drawn plain and with each of `shots` of `type_id` at its frame, written as numbered
/// pictures into `verification/{d} pictures/`; a row each that it draws cleanly and changes what
/// it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[Shot]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, frame {frame}, {what}; draws cleanly"),
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

// --- Write-on -------------------------------------------------------------------------------

fn is_write_on(e: &Effect) -> bool {
    matches!(e, Effect::WriteOn { .. })
}

/// A brush mark and no more: some pixels changed, fewer than a thousand.
fn a_dot(a: &[u8], b: &[u8]) -> bool {
    let changed = distance(a, b).1;
    changed > 0 && changed < 1000
}

/// A line: more than a thousand pixels changed, under a quarter of the street.
fn a_line(a: &[u8], b: &[u8]) -> bool {
    let changed = distance(a, b).1;
    changed > 1000 && changed < a.len() / 4 / 4
}

/// The line alone: most of the street clear, more than a thousand pixels showing.
fn alone(a: &[u8], _: &[u8]) -> bool {
    let clear = a.chunks_exact(4).filter(|p| p[3] == 0).count();
    let shown = a.len() / 4 - clear;
    clear > a.len() / 4 / 2 && shown > 1000
}

#[test]
fn b321_write_on() {
    let mut t = Table::new(
        "writeon",
        "# D-441: Write-on\n\nB-321, after After Effects' Write-on: a brush mark laid every Brush \
         Spacing seconds of the layer's time from its in point, where Brush Position then was, each \
         kept for Stroke Length seconds (0 for ever); each mark's size and hardness taken when it was \
         laid or now as Brush Time Properties says, its opacity as Paint Time Properties says (the \
         colour is always the one now, since colours are not keyable here); Path Stroke's round brush \
         at each mark, the mark covering most winning, laid on the layer, on transparent or revealing \
         the layer. The layer never grows. Every expected pixel is `Fixtures/writeon/expected_writeon.json`, \
         written by `tools/writeon_reference.py` before this code existed and printed in document 25 \
         as FX-WRITEON-001 to 041. Tolerance 2e-5.\n",
    );

    t.heading("FX-WRITEON-001 to 041 (document 25)");
    t.fixtures_numbered("expected_writeon.json", 1..=41);

    t.heading("The file");
    let all = files("fx_writeon", 41);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_writeon_009.json");
    t.row(
        "fx_writeon_009.json is saved with its words and numbers as written, the position's keys kept and no marks",
        &saved.to_string(),
        saved["brush_position"]["keyframes"][1]["value"] == json!([90, 50]) && saved["color"] == "#ffffff" && saved["brush_size"] == 3
            && saved["brush_hardness"] == 75 && saved["brush_opacity"] == 100 && saved["stroke_length"] == 0
            && saved["brush_spacing"] == 0.02 && saved["paint_time_properties"] == "none" && saved["brush_time_properties"] == "none"
            && saved["paint_style"] == "on_transparent" && saved.get("marks").is_none(),
    );
    why(
        &mut t,
        &[
            ("fx_writeon_028.json", "Write-on's brush position runs from -1000 to 1000, and this is 1001."),
            ("fx_writeon_029.json", "Write-on's brush size runs from 0 to 200, and this is 201."),
            ("fx_writeon_030.json", "Write-on's brush size runs from 0 to 200, and this is -1."),
            ("fx_writeon_031.json", "Write-on's brush hardness runs from 0 to 100, and this is 101."),
            ("fx_writeon_032.json", "Write-on's brush opacity runs from 0 to 100, and this is 101."),
            ("fx_writeon_033.json", "Write-on's stroke length runs from 0 to 3600, and this is -1."),
            ("fx_writeon_034.json", "Write-on's stroke length runs from 0 to 3600, and this is 3601."),
            ("fx_writeon_035.json", "Write-on's brush spacing runs from 0.001 to 10, and this is 0."),
            ("fx_writeon_036.json", "Write-on's brush spacing runs from 0.001 to 10, and this is 11."),
            ("fx_writeon_037.json", "Write-on's paint time properties is one of none, color, opacity, color_and_opacity, and this is \"size\"."),
            ("fx_writeon_038.json", "Write-on's brush time properties is one of none, size, hardness, size_and_hardness, and this is \"color\"."),
            ("fx_writeon_039.json", "Write-on's paint style is one of on_original, on_transparent, reveal, and this is \"glow\"."),
            ("fx_writeon_040.json", "Write-on's colour is written #rrggbb, and this is \"#12345\"."),
        ],
    );
    t.shape_refused("fx_writeon_001.json", "a Write-on with no `brush_spacing`", r##"{"brush_position": [50, 50], "color": "#ffffff", "brush_size": 6, "brush_hardness": 75, "brush_opacity": 100, "stroke_length": 0, "paint_time_properties": "none", "brush_time_properties": "none", "paint_style": "on_original"}"##);
    t.shape_refused("fx_writeon_001.json", "a Write-on whose brush position is one number", r##"{"brush_position": 50, "color": "#ffffff", "brush_size": 6, "brush_hardness": 75, "brush_opacity": 100, "stroke_length": 0, "brush_spacing": 0.01, "paint_time_properties": "none", "brush_time_properties": "none", "paint_style": "on_original"}"##);
    t.shape_refused("fx_writeon_001.json", "a Write-on whose paint style is true", r##"{"brush_position": [50, 50], "color": "#ffffff", "brush_size": 6, "brush_hardness": 75, "brush_opacity": 100, "stroke_length": 0, "brush_spacing": 0.01, "paint_time_properties": "none", "brush_time_properties": "none", "paint_style": true}"##);

    t.heading("Commands");
    let mut document = t.load("fx_writeon_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("brush position 1001, 50", set(write_on([1001.0, 50.0], ADDED, words(&[])))),
            ("brush size 201", set(write_on([50.0, 50.0], [201.0, 75.0, 100.0, 0.0, 0.01], words(&[])))),
            ("brush spacing 0", set(write_on([50.0, 50.0], [6.0, 75.0, 100.0, 0.0, 0.0], words(&[])))),
            ("paint time properties \"size\"", set(write_on([50.0, 50.0], ADDED, words(&[(1, "size")])))),
            ("brush time properties \"color\"", set(write_on([50.0, 50.0], ADDED, words(&[(2, "color")])))),
            ("paint style \"glow\"", set(write_on([50.0, 50.0], ADDED, words(&[(3, "glow")])))),
            ("colour \"red\"", set(write_on([50.0, 50.0], ADDED, words(&[(0, "red")])))),
            ("brush size keyed to 300", keys("brush_size", &[(0, &[6.0]), (4, &[300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_writeon_001.json",
        vec![
            (
                "position 25, 60, size 12, hardness 40, opacity 70, stroke length 2, spacing 0.05, blue, Opacity, Size & Hardness, reveal",
                set(write_on([25.0, 60.0], [12.0, 40.0, 70.0, 2.0, 0.05], ["#3080ff", "opacity", "size_and_hardness", "reveal"])),
            ),
            ("brush position keyed from 10, 50 to 90, 50", keys("brush_position", &[(0, &[10.0, 50.0]), (4, &[90.0, 50.0])])),
            ("brush size keyed from 2 to 8", keys("brush_size", &[(0, &[2.0]), (4, &[8.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_writeon_001.json", 0),
        ("fx_writeon_002.json", 8),
        ("fx_writeon_009.json", 4),
        ("fx_writeon_014.json", 8),
        ("fx_writeon_021.json", 8),
        ("fx_writeon_023.json", 8),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-WRITEON-011 (Brush Size 0 on the layer) changes nothing and 028 to 041 are refused, so
    // the card is not asked.
    let none: Vec<u32> = [11].into_iter().chain(28..=41).collect();
    card_fixtures(&mut t, &mut gpu, "fx_writeon", 41, &none, is_write_on);
    let across = keyed(&[(0, json!([10, 20])), (239, json!([90, 80]))]);
    card_reference(
        &mut t,
        &mut gpu,
        "Write-on",
        "core.write_on",
        &[
            ("as added (a dot in the middle)", json!({})),
            ("brush keyed across, size 40, hardness 50, spacing 0.05", json!({"brush_position": across, "brush_size": 40, "brush_hardness": 50, "brush_spacing": 0.05})),
            (
                "keyed across, On Transparent, stroke length 2, red, size 24",
                json!({"brush_position": across, "paint_style": "on_transparent", "stroke_length": 2, "color": "#ff3020", "brush_size": 24}),
            ),
            (
                "keyed across, Reveal, size keyed 10 to 80 kept per mark, opacity keyed 100 to 30 kept per mark",
                json!({"brush_position": across, "paint_style": "reveal", "brush_size": keyed(&[(0, json!(10)), (239, json!(80))]),
                    "brush_opacity": keyed(&[(0, json!(100)), (239, json!(30))]), "brush_time_properties": "size", "paint_time_properties": "opacity"}),
            ),
        ],
        is_write_on,
    );

    let vee = keyed(&[(0, json!([10, 30])), (24, json!([50, 75])), (47, json!([90, 30]))]);
    street(
        &mut t,
        "D-441",
        "core.write_on",
        &[
            ("as_added", json!({}), 0, "as added: a white dot in the middle", a_dot),
            (
                "vee_half",
                json!({"brush_position": vee, "brush_size": 12}),
                24,
                "brush keyed through a V over two seconds, size 12, halfway: the first stroke of the V written in white",
                a_line,
            ),
            ("vee_whole", json!({"brush_position": vee, "brush_size": 12}), 47, "the same at the end: the whole V written", a_line),
            (
                "vee_transparent_tail",
                json!({"brush_position": vee, "brush_size": 16, "stroke_length": 0.5, "color": "#ff3020", "paint_style": "on_transparent"}),
                40,
                "the V in red on transparent, stroke length 0.5 seconds: only the last half second of the line, the street gone",
                alone,
            ),
            (
                "vee_reveal_swelling",
                json!({"brush_position": vee, "brush_size": keyed(&[(0, json!(4)), (47, json!(40))]), "brush_time_properties": "size", "paint_style": "reveal"}),
                47,
                "the V revealing the street, each mark keeping its size, 4 growing to 40: the street seen through a line that swells",
                alone,
            ),
        ],
    );

    t.finish("D-441_write_on_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B321_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-321: a measurement, run deliberately with --release --ignored"]
fn b321_write_on_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B321_CPU").is_ok();
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
    let across = keyed(&[(0, json!([10, 20])), (239, json!([90, 80]))]);
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Write-on as added", Some(json!({}))),
        ("Noise, then Write-on keyed across, size 40 (up to 1000 marks)", Some(json!({"brush_position": across, "brush_size": 40}))),
        (
            "Noise, then Write-on keyed across, size keyed 10 to 80 per mark, stroke length 2",
            Some(json!({"brush_position": across, "brush_size": keyed(&[(0, json!(10)), (239, json!(80))]), "brush_time_properties": "size", "stroke_length": 2})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.write_on", &format!("{id}c"), p));
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
    let out = std::env::var("B321_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-321_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
