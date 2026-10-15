//! B-481: D-602, After Effects' Bezier Warp under `core.bezier_warp`: the drawing's outline as
//! four cubic edges (four vertices, eight tangents) and Quality, drawn through D-601's forward
//! mesh.
//!
//! Writes `verification/D-602_bezier_warp_table.md` and draws pictures into
//! `verification/D-602 pictures/`.
//!
//! Every expected pixel is `Fixtures/bezier_warp/expected_bezier_warp.json`, written by
//! `tools/bezier_warp_reference.py` before this code existed and printed in document 25 as
//! FX-BEZIER-001 to 017. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, BEZIER_POINTS, BEZIER_START};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Bezier Warp with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"quality": 8});
    for (name, at) in BEZIER_POINTS.iter().zip(BEZIER_START) {
        all[*name] = json!(at);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.bezier_warp", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=17 {
        let file = format!("fx_bezier_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::BezierWarp { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b481-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Bezier Warp {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::BezierWarp { .. }));
                t.row(
                    &format!("the reference shot, Bezier Warp {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Whether the pixel (x, y) of a picture `w` wide is clear.
fn clear(p: &[u8], w: usize, x: usize, y: usize) -> bool {
    p[(y * w + x) * 4 + 3] == 0
}

#[test]
fn b481_bezier_warp() {
    let mut t = Table::new(
        "bezier_warp",
        "# D-602: Bezier Warp\n\nB-481: `core.bezier_warp` takes After Effects' Bezier Warp \
         controls (the four vertices, the eight tangents, Quality) and bends the drawing to the \
         outline they make, through D-601's forward mesh. Every expected pixel is \
         `Fixtures/bezier_warp/expected_bezier_warp.json`, written by \
         `tools/bezier_warp_reference.py` before this code existed and printed in document 25 as \
         FX-BEZIER-001 to 017. Tolerance 2e-5.\n",
    );

    t.heading("FX-BEZIER-001 to 017 (document 25)");
    t.fixtures_numbered("expected_bezier_warp.json", 1..=17);

    t.heading("The file");
    let files: Vec<String> = (1..=17).map(|n| format!("fx_bezier_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_bezier_006.json");
    t.row(
        "fx_bezier_006.json is saved with its thirteen settings, each point two numbers",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 13 && saved["top_right_tangent"][0] == 200.0 / 3.0 && saved["top_right_tangent"][1] == -50.0 && saved["quality"] == 8.0,
    );
    for (file, want) in [
        ("fx_bezier_014.json", "quality"),
        ("fx_bezier_015.json", "quality"),
        ("fx_bezier_016.json", "top left vertex"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Bezier Warp and its {want}"), &why, why.contains("Bezier Warp") && why.contains(want));
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused("fx_bezier_001.json", "a Bezier Warp whose quality is a word", &full.replace("\"quality\":8", "\"quality\":\"high\""));
    t.shape_refused("fx_bezier_001.json", "a Bezier Warp without its top left vertex", &full.replace("\"top_left_vertex\":[0.0,0.0],", ""));
    t.shape_refused("fx_bezier_001.json", "a Bezier Warp whose right top vertex is one number", &full.replace("\"right_top_vertex\":[100.0,0.0]", "\"right_top_vertex\":100"));

    t.heading("Commands");
    let mut document = t.load("fx_bezier_001.json").document;
    let rough = changed(&t, "fx_bezier_001.json", |e| if let Effect::BezierWarp { quality, .. } = e { *quality = 0.5 });
    let far = changed(&t, "fx_bezier_001.json", |e| if let Effect::BezierWarp { points, .. } = e { points[6] = [100.0, 501.0] });
    t.refused(
        &mut document,
        vec![
            ("quality 0.5", set(rough)),
            ("bottom right vertex at (100, 501)", set(far)),
            ("top left tangent keyed to (-500, 0)", keys("top_left_tangent", &[(0, &[100.0 / 3.0, 0.0]), (4, &[-500.0, 0.0])])),
        ],
    );
    let sag = changed(&t, "fx_bezier_001.json", |e| {
        if let Effect::BezierWarp { points, quality } = e {
            points[1][1] = 60.0;
            points[2][1] = 60.0;
            *quality = 3.0;
        }
    });
    t.taken(
        &mut document,
        "fx_bezier_001.json",
        vec![
            ("top tangents at 60 per cent down, quality 3", set(sag)),
            ("left bottom vertex keyed from (0, 100) to (20, 80)", keys("left_bottom_vertex", &[(0, &[0.0, 100.0]), (4, &[20.0, 80.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_bezier_002.json", 0),
        ("fx_bezier_006.json", 0),
        ("fx_bezier_009.json", 2),
        ("fx_bezier_011.json", 0),
        ("fx_bezier_013.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001 changes nothing, so nothing is left to the card; the refused (014 to 017) neither;
    // 009 changes nothing at frame 0 only, so it is on the card in 8 of its 10 frames.
    let none: Vec<u32> = std::iter::once(1).chain(14..=17).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("the top edge sagging (top tangents at 60 per cent)", json!({"top_left_tangent": [100.0 / 3.0, 60], "top_right_tangent": [200.0 / 3.0, 60]}), 3),
            ("an S at quality 2", json!({"top_left_tangent": [100.0 / 3.0, 50], "top_right_tangent": [200.0 / 3.0, -50], "bottom_left_tangent": [100.0 / 3.0, 150], "bottom_right_tangent": [200.0 / 3.0, 50], "quality": 2}), 3),
            ("folded (bottom right vertex at (-20, -20))", json!({"bottom_right_vertex": [-20, -20]}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-602 pictures/`");
    let dir = repo("verification/D-602 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let (w, h) = TOWN;
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    let open = (0..w * h).filter(|&i| before[i * 4 + 3] == 0).count();
    t.row("1_before.png, the street with no effect, every pixel covered; draws cleanly", &format!("{said:?}, {open} clear pixels"), said.is_empty() && open == 0);

    let (added, said) = street(json!({}));
    t.row(
        "as added, every point where it starts: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&added, &before).1),
        said.is_empty() && added == before,
    );

    let (sag, said) = street(json!({"top_left_tangent": [100.0 / 3.0, 60], "top_right_tangent": [200.0 / 3.0, 60]}));
    write("2_sag.png", TOWN, &sag);
    let depth = |x: usize| (0..h).take_while(|&y| clear(&sag, w, x, y)).count();
    t.row(
        "2_sag.png, both top tangents pulled down to 60 per cent: the top edge sags in a curve, clear above it, deepest in the middle and nothing cleared at the corners; draws cleanly",
        &format!("{said:?}, clear rows from the top at the left, a quarter, the middle: {}, {}, {}", depth(0), depth(w / 4), depth(w / 2)),
        said.is_empty() && depth(0) == 0 && depth(w / 4) > 0 && depth(w / 2) > depth(w / 4),
    );

    let (squeeze, said) = street(json!({"top_left_vertex": [25, 25], "top_left_tangent": [125.0 / 3.0, 25], "top_right_tangent": [175.0 / 3.0, 25], "right_top_vertex": [75, 25],
        "right_top_tangent": [75, 125.0 / 3.0], "right_bottom_tangent": [75, 175.0 / 3.0], "bottom_right_vertex": [75, 75], "bottom_right_tangent": [175.0 / 3.0, 75],
        "bottom_left_tangent": [125.0 / 3.0, 75], "left_bottom_vertex": [25, 75], "left_bottom_tangent": [25, 175.0 / 3.0], "left_top_tangent": [25, 125.0 / 3.0]}));
    write("3_half_size.png", TOWN, &squeeze);
    let covered = (0..w * h).filter(|&i| squeeze[i * 4 + 3] != 0).count();
    t.row(
        "3_half_size.png, every point moved into the square from 25 to 75 per cent: the street at half size in the middle, a quarter of the frame covered, the rest clear; draws cleanly",
        &format!("{said:?}, {covered} of {} pixels covered", w * h),
        said.is_empty() && covered.abs_diff(w * h / 4) <= w + h && clear(&squeeze, w, 0, 0) && !clear(&squeeze, w, w / 2, h / 2),
    );

    let (s_curve, said) = street(json!({"top_left_tangent": [100.0 / 3.0, 50], "top_right_tangent": [200.0 / 3.0, -50], "bottom_left_tangent": [100.0 / 3.0, 150], "bottom_right_tangent": [200.0 / 3.0, 50]}));
    write("4_s_curve.png", TOWN, &s_curve);
    let top = |x: usize| (0..h).take_while(|&y| clear(&s_curve, w, x, y)).count();
    t.row(
        "4_s_curve.png, the top edge's tangents at 50 and -50 per cent down and the bottom's at 150 and 50: the street waved into an S, clear above the top edge's dip on the left and none on the right; draws cleanly",
        &format!("{said:?}, clear rows from the top at a quarter and three quarters across: {}, {}", top(w / 4), top(3 * w / 4)),
        said.is_empty() && top(w / 4) > 10 && top(3 * w / 4) == 0,
    );

    let (corner, said) = street(json!({"top_left_vertex": [25, 25]}));
    write("5_corner_pulled.png", TOWN, &corner);
    let (low, said1) = street(json!({"top_left_vertex": [25, 25], "quality": 1}));
    write("5b_corner_pulled_quality_1.png", TOWN, &low);
    t.row(
        "5_corner_pulled.png and 5b_corner_pulled_quality_1.png, the top left vertex pulled to (25, 25): the corner it left is clear and the far corner kept (two pixels in, past the edge's smoothing); at Quality 1 the bend is coarser, so the two differ; both draw cleanly",
        &format!("{said:?} {said1:?}, {} pixels differ between the qualities", distance(&corner, &low).1),
        said.is_empty() && said1.is_empty() && clear(&corner, w, 2, 2) && corner[((h - 3) * w + w - 3) * 4..][..4] == before[((h - 3) * w + w - 3) * 4..][..4] && corner != low,
    );

    t.finish("D-602_bezier_warp_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B481_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-481: a measurement, run deliberately with --release --ignored"]
fn b481_bezier_warp_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B481_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Bezier Warp with the top edge sagging", Some(json!({"top_left_tangent": [100.0 / 3.0, 60], "top_right_tangent": [200.0 / 3.0, 60]}))),
        ("Noise, then Bezier Warp folded (bottom right vertex at (-20, -20))", Some(json!({"bottom_right_vertex": [-20, -20]}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B481_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-481_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
