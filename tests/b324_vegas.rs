//! B-324: D-444, Vegas, after After Effects' Vegas ("Generate" in `docs/effects/EFFECTS.md`):
//! dashes running along the layer's masks or a shape layer's paths, drawn with P0-22's machinery.
//! The owner's decision of 2026-10-10, "Masks only for now": Image Contours is refused.
//!
//! Writes `verification/D-444_vegas_table.md`.
//!
//! Every expected pixel is `Fixtures/vegas/expected_vegas.json`, written by
//! `tools/vegas_reference.py` before this code existed and printed in document 25 as
//! FX-VEGAS-001 to 060. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-444 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// `base` with one setting changed.
fn with(base: &Effect, change: impl Fn(&mut Effect)) -> Effect {
    let mut e = base.clone();
    change(&mut e);
    e
}

/// The picture's circle: centre, radius.
const CIRCLE: ((f64, f64), f64) = ((240.0, 135.0), 100.0);

/// A circle as four curved points, the way a mask tool draws one.
fn circle(((cx, cy), r): ((f64, f64), f64)) -> J {
    let k = 0.552_284_749_8 * r;
    json!({"points": [
        {"point": [cx, cy - r], "in": [-k, 0], "out": [k, 0]},
        {"point": [cx + r, cy], "in": [0, -k], "out": [0, k]},
        {"point": [cx, cy + r], "in": [k, 0], "out": [-k, 0]},
        {"point": [cx - r, cy], "in": [0, k], "out": [0, -k]},
    ]})
}

/// A setting keyed linearly from `a` at frame 0 to `b` at frame 24.
fn keyed(a: f64, b: f64) -> J {
    json!({"base": a, "keyframes": [{"frame": 0, "value": a, "interp": "linear"}, {"frame": 24, "value": b, "interp": "linear"}]})
}

/// The town with a circle mask of mode None carrying `parameters`' Vegas, at `frame`.
fn picture(dir: &Path, parameters: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let masks = json!([{"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(CIRCLE), "keyframes": []}}]);
    let project = json!({
        "schema_version": 0, "project_id": "proj-b324-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
            "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 25,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 25},
            "layer_order": ["town"],
            "layers": [{
                "id": "town", "kind": "raster", "name": "town", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 25, "source_offset_frames": 0,
                "transform": {
                    "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "exposure_spans": [],
                "masks": masks,
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "fx-0-0", "type_id": "core.vegas", "enabled": true, "parameters": parameters}]
            }]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {} {}", d.id.as_str(), d.message, d.detail)).collect();
    (drawn.to_srgb8_straight(), said)
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
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

/// The Vegases the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Vegas { .. })))
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers, each with three
/// circle masks of mode None to draw along.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let masks: Vec<J> = [((960.0, 540.0), 400.0), ((560.0, 400.0), 220.0), ((1400.0, 620.0), 300.0)]
            .iter()
            .enumerate()
            .map(|(m, &((x, y), r))| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(((x + 40.0 * i as f64, y), r)), "keyframes": []}}))
            .collect();
        layers[i]["masks"] = J::Array(masks);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// Vegas's parameters: the effect's values as added with `changes` laid over them.
fn vegas_of(changes: J) -> J {
    let mut p = json!({"stroke": "masks", "mask": 1, "all_masks": "off", "segments": 32, "length": 1, "segment_distribution": "bunched",
        "rotation": 0, "random_phase": "off", "random_seed": 1, "blend_mode": "over", "color": "#ffffff", "width": 2, "hardness": 0,
        "start_opacity": 1, "mid_point_opacity": 0, "mid_point_position": 0.5, "end_opacity": 0});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// Frames side by side, a white gap between.
fn strip(frames: &[Vec<u8>]) -> (usize, Vec<u8>) {
    let (w, h) = TOWN;
    let gap = 8;
    let wide = frames.len() * (w + gap) - gap;
    let mut out = vec![255u8; wide * h * 4];
    for (n, f) in frames.iter().enumerate() {
        for y in 0..h {
            let at = (y * wide + n * (w + gap)) * 4;
            out[at..at + w * 4].copy_from_slice(&f[y * w * 4..(y + 1) * w * 4]);
        }
    }
    (wide, out)
}

#[test]
fn b324_vegas() {
    let mut t = Table::new(
        "vegas",
        "# B-324: Vegas\n\nD-444, after After Effects' Vegas (Generate): dashes along the \
         layer's masks or a shape layer's paths, each fading from its start to its end, turning \
         round as Rotation moves. The owner's decision of 2026-10-10, \"Masks only for now\": \
         Image Contours, After Effects' default, is refused with a sentence. Every expected pixel \
         is `Fixtures/vegas/expected_vegas.json`, written by `tools/vegas_reference.py` before \
         this code existed and printed in document 25 as FX-VEGAS-001 to 060. The build's frame \
         is compared sample by sample; the answer is the largest difference over all of them, \
         against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-VEGAS-001 to 060 (document 25)");
    t.fixtures("expected_vegas.json");

    t.heading("The file");
    let files: Vec<String> = (1..=60).map(|n| format!("fx_vegas_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
    };
    let saved = t.saved_parameters("fx_vegas_001.json");
    t.row("the paths compose finds are not saved", &saved.to_string(), saved.get("paths").is_none());
    for (file, said) in [
        ("fx_vegas_042.json", "Vegas's Image Contours stroke is not built yet: it draws only along masks and shape paths, so set Stroke to Masks or Shapes."),
        ("fx_vegas_043.json", "Vegas's stroke is \"masks\", \"shapes\" or \"image_contours\", and this is \"edges\"."),
        ("fx_vegas_053.json", "Vegas's blend mode is \"over\", \"under\", \"transparent\" or \"stencil\", and this is \"add\"."),
        ("fx_vegas_054.json", "Vegas's segment distribution is \"bunched\" or \"even\", and this is \"random\"."),
        ("fx_vegas_055.json", "Vegas's random phase is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_vegas_056.json", "Vegas's colour is written #rrggbb, and this is \"#12345\"."),
        ("fx_vegas_057.json", "Vegas's all masks is \"off\" or \"on\", and this is \"maybe\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == said);
    }
    let fx42 = t.saved_parameters("fx_vegas_042.json");
    t.row("fx_vegas_042.json's Image Contours is kept as written", &fx42.to_string(), fx42["stroke"] == "image_contours");
    t.shape_refused("fx_vegas_001.json", "a width written as a word", r##"{"stroke": "masks", "mask": 1, "all_masks": "off", "segments": 4, "length": 1, "segment_distribution": "bunched", "rotation": 0, "random_phase": "off", "random_seed": 1, "blend_mode": "over", "color": "#ffffff", "width": "2", "hardness": 0, "start_opacity": 1, "mid_point_opacity": 0, "mid_point_position": 0.5, "end_opacity": 0}"##);
    t.shape_refused("fx_vegas_001.json", "no blend mode", r##"{"stroke": "masks", "mask": 1, "all_masks": "off", "segments": 4, "length": 1, "segment_distribution": "bunched", "rotation": 0, "random_phase": "off", "random_seed": 1, "color": "#ffffff", "width": 2, "hardness": 0, "start_opacity": 1, "mid_point_opacity": 0, "mid_point_position": 0.5, "end_opacity": 0}"##);

    t.heading("Commands");
    let mut document = t.load("fx_vegas_001.json").document;
    let base = effect_of(&document);
    t.refused(
        &mut document,
        vec![
            ("width 201", set(with(&base, |e| if let Effect::Vegas { width, .. } = e { *width = 201.0 }))),
            ("stroke \"image_contours\"", set(with(&base, |e| if let Effect::Vegas { stroke, .. } = e { *stroke = "image_contours".into() }))),
            ("blend mode \"Over\"", set(with(&base, |e| if let Effect::Vegas { blend_mode, .. } = e { *blend_mode = "Over".into() }))),
            ("Segments keyed to 1001", keys("segments", &[(0, &[4.0]), (4, &[1001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_vegas_001.json",
        vec![
            ("Rotation 90, Even,", set(with(&base, |e| if let Effect::Vegas { rotation, segment_distribution, .. } = e { *rotation = 90.0; *segment_distribution = "even".into() }))),
            ("Rotation keyed from 0 to 360,", keys("rotation", &[(0, &[0.0]), (4, &[360.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_vegas_001.json", 0), ("fx_vegas_007.json", 2), ("fx_vegas_009.json", 0), ("fx_vegas_029.json", 2), ("fx_vegas_031.json", 0), ("fx_vegas_035.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/vegas");
    for n in 1..=41 {
        let file = format!("fx_vegas_{n:03}.json");
        let project = persist::load(&fixture_root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &fixture_root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &fixture_root, frame, quality);
            }
        }
        // FX-VEGAS-034 to 037 are shape layers: a shape layer has no drawing, and the card is
        // handed effects on drawings only (B-236's way); FX-VEGAS-038 to 041 have no path: the layer is left as it
        // is, and the card not asked.
        let expect = if n >= 34 { 0 } else { 10 };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames (expected {expect}); the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == expect,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added (mask 1, 32 segments, white, Width 2)", vegas_of(json!({}))),
        ("All Masks, Segments 12, Length 0.5, Even, orange, Width 24, Hardness 0.5", vegas_of(json!({"all_masks": "on", "segments": 12, "length": 0.5, "segment_distribution": "even", "color": "#ff8800", "width": 24, "hardness": 0.5}))),
        ("mask 2, Segments 5, Rotation 77, Random Phase, Width 40, Under, Mid-point Opacity 0.6", vegas_of(json!({"mask": 2, "segments": 5, "rotation": 77, "random_phase": "on", "width": 40, "blend_mode": "under", "mid_point_opacity": 0.6}))),
        ("All Masks, Segments 3, Width 60, Transparent", vegas_of(json!({"all_masks": "on", "segments": 3, "width": 60, "blend_mode": "transparent"}))),
        ("All Masks, Segments 8, Length 0.7, Width 50, Stencil", vegas_of(json!({"all_masks": "on", "segments": 8, "length": 0.7, "width": 50, "blend_mode": "stencil"}))),
    ] {
        let project = reference(|id| json!([{"instance_id": format!("b324-{id}"), "type_id": "core.vegas", "enabled": true, "parameters": p.clone()}]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&reference(|_| json!([])))).1;
        t.row(
            &format!("the reference shot, Vegas {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Vegas {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a street with a circle mask, in `verification/D-444 pictures/`");
    let dir = effect_table::repo("verification/D-444 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let street = town();
    png_out::write_rgba(&dir.join("town.png"), w, h, OutputDepth::Eight, &[], &street).unwrap();
    let ((cx, cy), r) = CIRCLE;
    // Pixels changed, and how many of them are further from the circle than the dashes reach.
    let changed = |b: &[u8], reach: f64| {
        let all: Vec<usize> = (0..w * h).filter(|&i| b[i * 4..][..4] != street[i * 4..][..4]).collect();
        let off = all.iter().filter(|&&i| (((i % w) as f64 + 0.5 - cx).hypot((i / w) as f64 + 0.5 - cy) - r).abs() > reach).count();
        (all.len(), off)
    };
    let chase = vegas_of(json!({"segments": 5, "length": 0.5, "segment_distribution": "even", "rotation": keyed(0.0, 360.0), "color": "#ff8800", "width": 10, "hardness": 0.6}));
    let mut frames = Vec::new();
    let mut counts = Vec::new();
    let mut ok = true;
    for frame in [0, 6, 12, 18, 24] {
        let (bytes, said) = picture(&dir, &chase, frame);
        let (all, off) = changed(&bytes, 6.0);
        ok &= said.is_empty() && off == 0 && all > 0;
        counts.push(all);
        png_out::write_rgba(&dir.join(format!("chase_f{frame:02}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        frames.push(bytes);
    }
    let round = distance(&frames[0], &frames[4]);
    let moved = distance(&frames[0], &frames[1]).1;
    let (wide, bytes) = strip(&frames);
    png_out::write_rgba(&dir.join("chase_strip.png"), wide, h, OutputDepth::Eight, &[], &bytes).unwrap();
    t.row(
        "chase_strip.png (frames 0, 6, 12, 18 and 24, each also alone as chase_fNN.png), five orange dashes spread round the circle, Rotation keyed from 0 to 360: they chase once round, frame 24 back where frame 0 was; nothing changes off the circle",
        &format!("pixels changed per frame {counts:?}; frame 6 differs from frame 0 in {moved} pixels; frame 24 against frame 0, largest difference {} of 255", round.0),
        ok && moved > 0 && round.0 <= 1,
    );
    for (name, what, p, reach) in [
        ("tails", "Segments 4, Width 16, Hardness 0.5: four dashes end to end, each whole at its start and fading to nothing at its end, comet tails", vegas_of(json!({"segments": 4, "width": 16, "hardness": 0.5, "color": "#40c0ff"})), 9.0),
        ("stencil", "Segments 6, Length 0.6, Width 30, Stencil: the street shows only inside the dashes", vegas_of(json!({"segments": 6, "length": 0.6, "width": 30, "blend_mode": "stencil", "end_opacity": 1})), 16.0),
    ] {
        let (bytes, said) = picture(&dir, &p, 0);
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let (all, off) = changed(&bytes, reach);
        let pass = said.is_empty() && all > 0 && (name == "stencil" || off == 0);
        t.row(&format!("{name}.png, {what}"), &format!("{said:?}, {all} pixels changed, {off} off the circle"), pass);
    }
    let (_, said) = picture(&dir, &vegas_of(json!({"stroke": "image_contours"})), 0);
    t.row(
        "the street with Stroke Image Contours: drawn without the effect, and a sentence says why",
        &format!("{said:?}"),
        said.iter().any(|s| s.contains("Image Contours stroke is not built yet")),
    );

    t.finish("D-444_vegas_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-233's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again". With `B324_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-324: a measurement, run deliberately with --release --ignored"]
fn b324_vegas_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B324_CPU").is_ok();
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n\n\
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
        ("Noise, then Vegas as added (mask 1, 32 segments, Width 2)", Some(vegas_of(json!({})))),
        ("Noise, then Vegas, All Masks, 64 segments, Width 24", Some(vegas_of(json!({"all_masks": "on", "segments": 64, "width": 24})))),
        ("Noise, then Vegas, All Masks, 8 segments, Length 0.5, Even, Width 60", Some(vegas_of(json!({"all_masks": "on", "segments": 8, "length": 0.5, "segment_distribution": "even", "width": 60})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}v"), "type_id": "core.vegas", "enabled": true, "parameters": p.clone()}));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
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
    let out = std::env::var("B324_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-324_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
