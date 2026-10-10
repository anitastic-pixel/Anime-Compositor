//! B-288: D-409, Chromatic Aberration's new form (PLUGINS.md pick #3, "Lens Chromatic
//! Aberration", with its RGB Separation row as a mode): a lens falloff, a scale for each
//! channel, fringe blur and a straight offset, on `core.chromatic_aberration`. A file without
//! `mode` is D-120's form and draws as before.
//!
//! Writes `verification/D-409_lens_chromatic_aberration_table.md` and draws pictures into
//! `verification/D-409 pictures/`.
//!
//! Every expected pixel is
//! `Fixtures/lens_chromatic_aberration/expected_lens_chromatic_aberration.json`, written by
//! `tools/lens_chromatic_aberration_reference.py` before this code existed and printed in
//! document 25 as FX-LENSCA-001 to 026. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, same_json, saved, set, town, Table, MAIN, TOWN};
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

fn is_lens(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::LensChromaticAberration { .. }))
}

/// The new-form aberrations the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_lens(c.unmixed())).count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_lens(c.unmixed()))
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

/// The new form's settings as added, with `p` laid over them.
fn new_form(p: &J) -> J {
    let mut all = json!({"mode": "radial", "amount": 3, "center": [50, 50], "angle": 90, "falloff": 0,
        "red_scale": 100, "green_scale": 0, "blue_scale": -100, "fringe_blur": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.chromatic_aberration", "enabled": true, "parameters": new_form(p)})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing to the card: untouched or refused.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=26 {
        let file = format!("fx_lensca_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
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
/// and Draft, on the card against the processor, all three on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
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
        let project = reference(|id| json!([fx(&format!("b288-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Chromatic Aberration {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Chromatic Aberration {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street, frame 0, drawn on the processor, straight 8-bit, with what it warned
/// of.
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log).expect("the frame draws").data().to_vec()
}

#[test]
fn b288_lens_chromatic_aberration() {
    let mut t = Table::new(
        "lens_chromatic_aberration",
        "# D-409: Lens Chromatic Aberration\n\nB-288: `core.chromatic_aberration`'s new form, \
         PLUGINS.md's pick #3 with its RGB Separation row as a mode: radial (a lens falloff, weak \
         in the middle and strong at the edges) or offset (a straight line at an angle), a scale \
         for each channel, and fringe blur. A file without `mode` is D-120's form and draws by \
         D-120's code, untouched. Every expected pixel is \
         `Fixtures/lens_chromatic_aberration/expected_lens_chromatic_aberration.json`, written by \
         `tools/lens_chromatic_aberration_reference.py` before this code existed and printed in \
         document 25 as FX-LENSCA-001 to 026. Tolerance 2e-5.\n",
    );

    t.heading("FX-LENSCA-001 to 026 (document 25)");
    t.fixtures_numbered("expected_lens_chromatic_aberration.json", 1..=26);

    t.heading("Older projects: D-120's form, as before");
    let old_root = repo("Fixtures/chromatic_aberration");
    for n in 1..=11 {
        let file = format!("fx_chroma_{n:03}.json");
        let text = fs::read_to_string(old_root.join(&file)).unwrap();
        let old: J = serde_json::from_str(&text).unwrap();
        let loaded = persist::load_str(&text).unwrap();
        let effect = &loaded.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect;
        let kept = same_json(&saved(&loaded), &old);
        t.row(
            &format!("{file} (no mode) opens as D-120's form and is saved as it was written, with no new settings"),
            &format!("{}; saved {}", if matches!(effect, Effect::ChromaticAberration { .. }) { "D-120's form" } else { "the new form" }, if kept { "the same" } else { "differently" }),
            matches!(effect, Effect::ChromaticAberration { .. }) && kept,
        );
        // Its twin in the new form at the starting values draws the same numbers, bit for bit.
        let mut twin = old.clone();
        let p = &mut twin["compositions"][0]["layers"][0]["effects"][0]["parameters"];
        let mut all = new_form(&json!({}));
        all["amount"] = p["amount"].clone();
        all["center"] = p["center"].clone();
        *p = all;
        let frames: Vec<i32> = if [5, 6].contains(&n) { vec![0, 2, 4] } else if n == 7 { vec![0, 3] } else { vec![0] };
        let same = frames.iter().all(|&f| draw(&old, &old_root, f) == draw(&twin, &old_root, f));
        t.row(
            &format!("{file} and its twin in the new form at the starting values (radial, falloff 0, scales 100, 0, -100, no blur) draw the same frames, bit for bit"),
            if same { "bit-identical" } else { "differ" },
            same,
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=26).map(|n| format!("fx_lensca_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved_p = t.saved_parameters("fx_lensca_011.json");
    t.row(
        "fx_lensca_011.json is saved with its nine settings, in offset mode",
        &saved_p.to_string(),
        saved_p.as_object().unwrap().len() == 9 && saved_p["mode"] == "offset" && saved_p["fringe_blur"] == 100.0,
    );
    for (file, want) in [
        ("fx_lensca_019.json", "mode"),
        ("fx_lensca_020.json", "falloff"),
        ("fx_lensca_021.json", "red_scale"),
        ("fx_lensca_022.json", "blue_scale"),
        ("fx_lensca_023.json", "fringe_blur"),
        ("fx_lensca_024.json", "angle"),
        ("fx_lensca_025.json", "amount"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, says);
    }
    let all = r##""mode": "radial", "amount": 3, "center": [50, 50], "angle": 90, "red_scale": 100, "green_scale": 0, "blue_scale": -100, "fringe_blur": 0"##;
    t.shape_refused("fx_lensca_001.json", "a new-form Chromatic Aberration without its falloff", &format!("{{{all}}}"));
    t.shape_refused("fx_lensca_001.json", "a new-form Chromatic Aberration whose falloff is a word", &format!("{{{all}, \"falloff\": \"soft\"}}"));
    t.shape_refused("fx_lensca_001.json", "a new-form Chromatic Aberration whose mode is a number", r##"{"mode": 1, "amount": 3, "center": [50, 50], "angle": 90, "falloff": 0, "red_scale": 100, "green_scale": 0, "blue_scale": -100, "fringe_blur": 0}"##);

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_lensca_009.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let halved = matches!(draft, Effect::LensChromaticAberration { amount, .. } if amount == 1.0);
    t.row(
        "a half-size draft preview halves the amount, a distance, and leaves the scales, shares of it",
        &format!("{draft:?}"),
        halved,
    );
    t.row("it grows the drawing's bounds by nothing", &draft.bounds_expansion().to_string(), draft.bounds_expansion() == 0);

    t.heading("Commands");
    let mut document = t.load("fx_lensca_001.json").document;
    let spiral = changed(&t, "fx_lensca_001.json", |e| if let Effect::LensChromaticAberration { mode, .. } = e { *mode = "spiral".into() });
    let falloff = changed(&t, "fx_lensca_001.json", |e| if let Effect::LensChromaticAberration { falloff, .. } = e { *falloff = 101.0 });
    let red = changed(&t, "fx_lensca_001.json", |e| if let Effect::LensChromaticAberration { red_scale, .. } = e { *red_scale = 201.0 });
    t.refused(
        &mut document,
        vec![
            ("mode spiral", set(spiral)),
            ("falloff 101", set(falloff)),
            ("red scale 201", set(red)),
            ("fringe blur keyed to 150", keys("fringe_blur", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let offset = changed(&t, "fx_lensca_001.json", |e| {
        if let Effect::LensChromaticAberration { mode, fringe_blur, .. } = e {
            *mode = "offset".into();
            *fringe_blur = 60.0;
        }
    });
    t.taken(
        &mut document,
        "fx_lensca_001.json",
        vec![
            ("offset mode with fringe blur 60", set(offset)),
            ("falloff keyed from 0 to 100", keys("falloff", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_lensca_001.json", 0),
        ("fx_lensca_003.json", 0),
        ("fx_lensca_007.json", 0),
        ("fx_lensca_008.json", 0),
        ("fx_lensca_009.json", 0),
        ("fx_lensca_011.json", 0),
        ("fx_lensca_013.json", 2),
        ("fx_lensca_015.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Amount 0 (002) leaves nothing to draw; refused (019 to 026).
    let none: Vec<u32> = [2].into_iter().chain(19..=26).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("radial, amount 20, falloff 100", json!({"amount": 20, "falloff": 100})),
            ("radial about (30, 60), amount 20, falloff 60, red 150, green 40, fringe blur 50", json!({"amount": 20, "center": [30, 60], "falloff": 60, "red_scale": 150, "green_scale": 40, "fringe_blur": 50})),
            ("offset at 30 degrees, amount 8, fringe blur 100", json!({"mode": "offset", "amount": 8, "angle": 30, "fringe_blur": 100})),
        ],
    );

    t.heading("Pictures: in `verification/D-409 pictures/`");
    let dir = repo("verification/D-409 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    // How much red and blue moved inside a box of the street, against the picture before.
    let moved = |p: &[u8], x0: usize, x1: usize, y0: usize, y1: usize| -> u64 {
        let mut s = 0;
        for y in y0..y1 {
            for x in x0..x1 {
                let i = (y * TOWN.0 + x) * 4;
                s += p[i].abs_diff(before[i]) as u64 + p[i + 2].abs_diff(before[i + 2]) as u64;
            }
        }
        s
    };
    let (w, h) = TOWN;
    let (mid, edge) = ((w * 3 / 8, w * 5 / 8, h * 3 / 8, h * 5 / 8), (0, w / 8, 0, h / 4));
    let (flat, said_flat) = street(json!({"amount": 12}));
    write("2_no_falloff.png", &flat);
    let (lens, said) = street(json!({"amount": 12, "falloff": 100}));
    write("3_lens_falloff_100.png", &lens);
    let m = |p: &[u8], b: (usize, usize, usize, usize)| moved(p, b.0, b.1, b.2, b.3);
    t.row(
        "2_no_falloff.png and 3_lens_falloff_100.png, amount 12: with falloff 100 the middle of the street barely splits while the corners still do; both draw cleanly",
        &format!("{said_flat:?} {said:?}; the middle moved {} without falloff, {} with; the top left corner {} and {}", m(&flat, mid), m(&lens, mid), m(&flat, edge), m(&lens, edge)),
        said_flat.is_empty() && said.is_empty() && m(&lens, mid) * 4 < m(&flat, mid) && m(&lens, edge) > 0,
    );

    let (offset, said) = street(json!({"mode": "offset", "amount": 6, "angle": 90}));
    write("4_offset_rgb_separation.png", &offset);
    // On the covered street, red is the picture's red six pixels to the left, blue six to the right.
    let (mut red_ok, mut blue_ok, mut seen) = (0, 0, 0);
    for y in 0..h {
        for x in 6..w - 6 {
            let i = (y * w + x) * 4;
            let (l, r) = (i - 24, i + 24);
            if before[i + 3] == 255 && before[l + 3] == 255 && before[r + 3] == 255 {
                seen += 1;
                red_ok += (offset[i] == before[l]) as usize;
                blue_ok += (offset[i + 2] == before[r + 2]) as usize;
            }
        }
    }
    t.row(
        "4_offset_rgb_separation.png, offset mode, amount 6, angle 90: red is the street's red six pixels to the left, blue six to the right, everywhere the street covers; draws cleanly",
        &format!("{said:?}; red {red_ok} and blue {blue_ok} of {seen} pixels"),
        said.is_empty() && seen > 0 && red_ok == seen && blue_ok == seen,
    );

    let (soft, said) = street(json!({"amount": 12, "falloff": 100, "fringe_blur": 100}));
    write("5_fringe_blur_100.png", &soft);
    let edges = |p: &[u8]| p.chunks_exact(4).collect::<Vec<_>>().windows(2).map(|w| w[0][0].abs_diff(w[1][0]) as u64 + w[0][2].abs_diff(w[1][2]) as u64).sum::<u64>();
    t.row(
        "5_fringe_blur_100.png, 3_lens_falloff_100.png with fringe blur 100: the coloured fringes smeared outward, softer; draws cleanly",
        &format!("{said:?}, red and blue sideways contrast {} against {}", edges(&soft), edges(&lens)),
        said.is_empty() && edges(&soft) < edges(&lens),
    );

    let (green, said) = street(json!({"amount": 8, "red_scale": 0, "green_scale": 100, "blue_scale": 0}));
    write("6_green_only.png", &green);
    let rb_kept = green.chunks_exact(4).zip(before.chunks_exact(4)).filter(|(p, q)| q[3] == 255 && (p[0] != q[0] || p[2] != q[2])).count();
    let g_moved = green.chunks_exact(4).zip(before.chunks_exact(4)).filter(|(p, q)| p[1] != q[1]).count();
    t.row(
        "6_green_only.png, scales red 0, green 100, blue 0: only green moves, outward; red and blue stay where the street covers; draws cleanly",
        &format!("{said:?}; {rb_kept} covered pixels changed red or blue, {g_moved} changed green"),
        said.is_empty() && rb_kept == 0 && g_moved > 0,
    );

    t.finish("D-409_lens_chromatic_aberration_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B288_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-288: a measurement, run deliberately with --release --ignored"]
fn b288_lens_chromatic_aberration_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B288_CPU").is_ok();
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
        ("Noise, then radial amount 10, falloff 100", Some(json!({"amount": 10, "falloff": 100}))),
        ("Noise, then radial amount 10, falloff 100, fringe blur 100 (11 samples a channel)", Some(json!({"amount": 10, "falloff": 100, "fringe_blur": 100}))),
        ("Noise, then offset amount 6 at 90 degrees", Some(json!({"mode": "offset", "amount": 6}))),
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
    let out = std::env::var("B288_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-288_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
