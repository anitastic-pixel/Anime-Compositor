//! B-227: D-347, Moment Map, after After Effects' Time Displacement (EFFECTS.md P0-4).
//!
//! Each pixel of the layer is taken from a different moment of the layer, chosen by a map
//! layer's brightness. Every expected pixel is
//! `Fixtures/time_displacement/expected_time_displacement.json`, written by
//! `tools/time_displacement_reference.py` before this code existed.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::command::Command;
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

/// `base` with the max time `max` and the fit `fit`.
fn with(base: &Effect, max: f64, fit: &str) -> Effect {
    let mut e = base.clone();
    if let Effect::MomentMap { max_time, fit: f, .. } = &mut e {
        *max_time = max;
        *f = fit.to_string();
    }
    e
}

/// Set the `holder` layer's Moment Map; the fixtures' drawing is `holder`, not `art`.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        effect,
    }
}

/// Frames of the pictures' drawing: the street sliding 8 pixels left a frame.
const DRAWINGS: usize = 25;

/// The street drawn `DRAWINGS` times, sliding, as an image sequence on a layer carrying
/// `parameters`' Moment Map, with a black-to-white ramp under it switched off; frame 12.
fn picture(dir: &Path, parameters: Option<J>) -> (Vec<u8>, Vec<String>) {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let transform = json!({
        "anchor": t(json!([w as f64 / 2.0, h as f64 / 2.0])), "position": t(json!([w as f64 / 2.0, h as f64 / 2.0])),
        "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
    });
    let effects = parameters.map_or(json!([]), |p| {
        json!([{"instance_id": "fx-0-0", "type_id": "core.moment_map", "enabled": true, "parameters": p}])
    });
    let frames: serde_json::Map<String, J> =
        (1..=DRAWINGS).map(|n| (n.to_string(), json!(format!("run/{n}.png")))).collect();
    let spans: Vec<J> = (0..DRAWINGS)
        .map(|f| json!({"start_frame": f, "end_frame_exclusive": f + 1, "drawing_number": f + 1}))
        .collect();
    let project = json!({
        "schema_version": 0, "project_id": "proj-b227-picture",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [
            {"id": "asset-run", "kind": "image_sequence", "name": "run", "pattern": "run/####.png", "frames": frames,
             "interpretation": {"color_space": "srgb", "alpha": "straight"}},
            {"id": "asset-ramp", "kind": "still", "name": "ramp", "path": "ramp.png",
             "interpretation": {"color_space": "srgb", "alpha": "straight"}}
        ],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": DRAWINGS,
            "work_area": {"start_frame": 0, "end_frame_exclusive": DRAWINGS},
            "layer_order": ["street", "ramp"],
            "layers": [
                {"id": "street", "kind": "raster", "name": "street", "asset_id": "asset-run", "enabled": true,
                 "locked": false, "in_frame": 0, "out_frame": DRAWINGS, "source_offset_frames": 0,
                 "transform": transform, "exposure_spans": spans, "mask": null, "matte": null,
                 "blend_mode": "normal", "effects": effects},
                {"id": "ramp", "kind": "raster", "name": "ramp", "asset_id": "asset-ramp", "enabled": false,
                 "locked": false, "in_frame": 0, "out_frame": DRAWINGS, "source_offset_frames": 0,
                 "transform": transform, "exposure_spans": [], "mask": null, "matte": null,
                 "blend_mode": "normal", "effects": []}
            ]
        }]
    });
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), 12, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

#[test]
fn b227_moment_map() {
    let mut t = Table::new(
        "time_displacement",
        "# B-227: Moment Map\n\nD-347, after After Effects' Time Displacement (EFFECTS.md P0-4). \
         Every expected pixel is `Fixtures/time_displacement/expected_time_displacement.json`, \
         written by `tools/time_displacement_reference.py` before this code existed and printed \
         in document 25 as FX-TDISP-001 to 028. The build's frame is compared sample by sample; \
         the answer is the largest difference over all of them, against the catalogue's \
         tolerance of 2e-5.\n",
    );

    t.heading("FX-TDISP-001 to 028 (document 25)");
    t.fixtures("expected_time_displacement.json");

    t.heading("The file");
    let files: Vec<String> = (1..=28).filter(|&n| n != 14).map(|n| format!("fx_tdisp_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    // FX-TDISP-014's mask is written in the file's first form, as FX-ECHO-019's (B-130): it is
    // saved as one mask in the current form, not dropped.
    let loaded = t.load("fx_tdisp_014.json");
    let saved = effect_table::saved(&loaded);
    let again = persist::load_str(&saved.to_string()).expect("the saved file opens").document;
    let masks = &saved["compositions"][0]["layers"][0]["masks"];
    let same = t.render(&again, 6, 64).data() == t.render(&loaded.document, 6, 64).data();
    t.row(
        "fx_tdisp_014.json, its mask in the first form, is saved as one mask of the same four \
         corners in the current form, and the saved file draws frame 6 the same",
        &format!("{} mask(s), frame 6 {}", masks.as_array().map_or(0, Vec::len), if same { "byte-identical" } else { "differs" }),
        masks.as_array().is_some_and(|m| m.len() == 1 && m[0]["path"]["base"]["points"].as_array().is_some_and(|p| p.len() == 4))
            && same,
    );
    let params = t.saved_parameters("fx_tdisp_002.json");
    t.row(
        "the moments it lays are never saved",
        &format!("{:?} {:?}", params.get("picture"), params.get("map")),
        params.get("picture").is_none() && params.get("map").is_none(),
    );
    let mut document = t.load("fx_tdisp_002.json").document;
    let effect_of = |d: &anime_compositor::command::Document| {
        d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone()
    };
    let base = effect_of(&document);
    for (file, said) in [
        ("fx_tdisp_026.json", "Moment Map's fit is \"center\", \"stretch\" or \"tile\", and this is \"fill\"."),
        ("fx_tdisp_027.json", "Moment Map's map is the name of a layer of this composition, and this is 3."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == said);
    }

    t.heading("Commands");
    for (what, effect) in [("max 10.5 seconds", with(&base, 10.5, "stretch")), ("fit \"fill\"", with(&base, 0.5, "fill"))] {
        let refused = document.apply(set(effect)).err();
        let untouched = effect_of(&document) == base;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    t.taken(
        &mut document,
        "fx_tdisp_002.json",
        vec![("max -2 seconds,", set(with(&base, -2.0, "stretch"))), ("fit tile,", set(with(&base, -2.0, "tile")))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_tdisp_001.json", 6), ("fx_tdisp_002.json", 6), ("fx_tdisp_010.json", 6), ("fx_tdisp_014.json", 6)]);

    t.heading("Pictures: a sliding street, in `verification/D-347 pictures/`");
    let dir = effect_table::repo("verification/D-347 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    // The drawings and the ramp are made here and removed after: only the frames are kept.
    fs::create_dir_all(dir.join("run")).unwrap();
    let street = town();
    for n in 1..=DRAWINGS {
        let shift = 8 * (n - 1);
        let bytes: Vec<u8> = (0..w * h)
            .flat_map(|i| {
                let (x, y) = (i % w, i / w);
                street[(y * w + (x + shift) % w) * 4..][..4].to_vec()
            })
            .collect();
        png_out::write_rgba(&dir.join(format!("run/{n}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
    }
    let ramp: Vec<u8> = (0..w * h)
        .flat_map(|i| {
            let v = ((i % w) as f64 / (w - 1) as f64 * 255.0).round() as u8;
            [v, v, v, 255]
        })
        .collect();
    png_out::write_rgba(&dir.join("ramp.png"), w, h, OutputDepth::Eight, &[], &ramp).unwrap();
    let (before, said) = picture(&dir, None);
    png_out::write_rgba(&dir.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the street at frame 12, draws cleanly", &format!("{said:?}"), said.is_empty());
    let changed = |b: &[u8]| (0..w * h).filter(|&i| b[i * 4..][..4] != before[i * 4..][..4]).count();
    let shots = [
        ("as_added", "as added: Max 1 second, Time Resolution 60, its own brightness: the bright sky from later, the dark road from a second back, before the street began, so the road is clear", json!({"max_time": 1, "resolution": 60, "layer": "", "fit": "stretch"}), true),
        ("squeeze", "the ramp, Max 0.5: the left edge from half a second back, the right edge from half a second on, so the sliding street comes out squeezed, more buildings across the frame", json!({"max_time": 0.5, "resolution": 24, "layer": "ramp", "fit": "stretch"}), true),
        ("still", "Max 0: the street as it is, the same as before.png", json!({"max_time": 0, "resolution": 60, "layer": "ramp", "fit": "stretch"}), false),
    ];
    for (name, what, p, moves) in shots {
        let (bytes, said) = picture(&dir, Some(p));
        png_out::write_rgba(&dir.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes).unwrap();
        let n = changed(&bytes);
        t.row(&format!("{name}.png, {what}"), &format!("{said:?}, {n} pixels changed"), said.is_empty() && (n > 0) == moves);
    }
    fs::remove_dir_all(dir.join("run")).unwrap();
    fs::remove_file(dir.join("ramp.png")).unwrap();

    t.finish("D-347_moment_map_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-226's way: the reference shot (1920 by 1080, 24 a second) with the effect
/// on its first three layers, then a Noise that changes every frame so nothing is kept, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop starts
/// with empty caches and its 30 frames' median is "first"; then seven loops are timed, the
/// median of their 210 frames is "again". Moment Map is drawn by the CPU, as Echo is.
#[test]
#[ignore = "B-227: a measurement, run deliberately with --release --ignored"]
fn b227_moment_map_timing() {
    use anime_compositor::cache::CelCache;
    use anime_compositor::compose::DEFAULT_TILE_SIZE;
    use anime_compositor::gpu::Gpu;
    use anime_compositor::preview::{self, PreviewQuality};
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let fx = |id: &str, type_id: &str, p: J| json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": p});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Moment Map, its own brightness, Max 1 s, Resolution 60, then Noise", Some(json!({"max_time": 1, "resolution": 60, "layer": "", "fit": "stretch"}))),
        ("Moment Map, layer-4 as the map, Max 0.5 s, Resolution 24, then Noise", Some(json!({"max_time": 0.5, "resolution": 24, "layer": "layer-4", "fit": "stretch"}))),
    ];
    for (name, p) in shots {
        let stack = |id: &str| {
            let mut v = Vec::new();
            if let Some(p) = &p {
                v.push(fx(&format!("{id}m"), "core.moment_map", p.clone()));
            }
            v.push(fx(&format!("{id}n"), "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})));
            v
        };
        let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let layers = &mut j["compositions"][0]["layers"];
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            layers[i]["effects"] = J::Array(stack(id));
        }
        let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let (project, comp, root) = (loaded.document.project().clone(), Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B227_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-227_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
