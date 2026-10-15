//! B-570: D-700, the gaps D-605's Liquify left against After Effects' Liquify: the brushes it
//! lacked (Turbulence, Reflection, Clone, Reconstruction with its four modes), Freeze Area and
//! Thaw Area, a freeze area mask, and strokes brushed before the last one, kept with the effect.
//!
//! Writes `verification/D-700_liquify_brushes_table.md` and draws pictures into
//! `verification/D-700 pictures/`.
//!
//! Every expected pixel is `Fixtures/liquify_brushes/expected_liquify_brushes.json`, written by
//! `tools/liquify_brushes_reference.py` before this code existed and printed in document 25 as
//! FX-LIQUIFY-BRUSHES-001 to 028. Tolerance 2e-5. Nothing here is a snapshot of a run. D-605's
//! own fixtures stand unchanged and are B-484's.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{changed, keys, median, repo, set, time_passes, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

fn mine(e: &Effect) -> bool {
    matches!(e, Effect::Liquify { .. })
}

/// A rectangle from (x0, y0) to (x1, y1), corner points only.
fn rect([x0, y0, x1, y1]: [f64; 4]) -> J {
    let points: Vec<J> = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]].iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect();
    json!({ "points": points })
}

/// Masks that cut nothing (mode None), for the freeze area mask to choose from.
fn masks(rects: &[[f64; 4]]) -> J {
    J::Array(
        rects
            .iter()
            .enumerate()
            .map(|(m, &r)| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": rect(r), "keyframes": []}}))
            .collect(),
    )
}

/// A Liquify with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"tool": "warp", "brush_size": 64, "brush_pressure": 50, "stroke": [], "distortion_percentage": 100, "distortion_mesh_offset": [0, 0]});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.liquify", "enabled": true, "parameters": all})
}

/// One earlier stroke: tool number, brush size, pressure, jitter 70, no clone offset, Revert,
/// then the points.
fn earlier(tool: u32, size: u32, pressure: u32, points: &[f64]) -> J {
    let mut e = vec![tool as f64, size as f64, pressure as f64, 70.0, 0.0, 0.0, 0.0];
    e.extend_from_slice(points);
    json!(e)
}

/// The reference shot (1920 by 1080) with a box mask (mode None) about its middle and `stack`
/// on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["masks"] = masks(&[[860.0, 440.0, 1060.0, 640.0]]);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// `effects` on the street (in `dir`), drawn on the processor at frame 0, straight 8-bit, with
/// what it warned of. The layer carries `rects` as masks that cut nothing.
fn picture(dir: &Path, rects: &[[f64; 4]], effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["masks"] = masks(rects);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The pixels that differ between two straight 8-bit pictures.
fn differing(a: &[u8], b: &[u8]) -> usize {
    a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, q)| p != q).count()
}

#[test]
fn b570_liquify_brushes() {
    let mut t = Table::new(
        "liquify_brushes",
        "# D-700: Liquify's other brushes, freeze areas and earlier strokes\n\nB-570: `core.liquify` \
         takes After Effects' remaining Liquify brushes (Turbulence, Reflection, Clone, \
         Reconstruction with Revert, Displace, Amplitwist and Affine), Freeze Area and Thaw Area, \
         Turbulent Jitter, Clone Offset, Reconstruction Mode, a Freeze Area Mask, and the strokes \
         brushed before the last, kept with the effect. Every expected pixel is \
         `Fixtures/liquify_brushes/expected_liquify_brushes.json`, written by \
         `tools/liquify_brushes_reference.py` before this code existed and printed in document 25 \
         as FX-LIQUIFY-BRUSHES-001 to 028. Tolerance 2e-5. D-605's own fixtures stand unchanged \
         (B-484).\n",
    );

    t.heading("FX-LIQUIFY-BRUSHES-001 to 028 (document 25)");
    t.fixtures("expected_liquify_brushes.json");

    t.heading("The file");
    let files: Vec<String> = (1..=28).map(|n| format!("fx_liquify_brushes_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = Table::new("liquify", "");
    let saved = old.saved_parameters("fx_liquify_002.json");
    t.row(
        "D-605's fx_liquify_002.json, from before D-700, is saved with its six settings and none of the new ones",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6 && saved.get("strokes").is_none() && saved.get("turbulent_jitter").is_none(),
    );
    let saved = t.saved_parameters("fx_liquify_brushes_013.json");
    t.row(
        "fx_liquify_brushes_013.json is saved with its earlier stroke as a list of lists",
        &saved["strokes"].to_string(),
        saved["strokes"].as_array().is_some_and(|a| a.len() == 1 && a[0].as_array().is_some_and(|e| e.len() == 11)),
    );
    for (file, want) in [
        ("fx_liquify_brushes_018.json", "tool"),
        ("fx_liquify_brushes_019.json", "turbulent_jitter"),
        ("fx_liquify_brushes_020.json", "reconstruction mode"),
        ("fx_liquify_brushes_021.json", "tool number"),
        ("fx_liquify_brushes_022.json", "numbers"),
        ("fx_liquify_brushes_023.json", "point"),
        ("fx_liquify_brushes_024.json", "brush size"),
        ("fx_liquify_brushes_025.json", "numbers"),
        ("fx_liquify_brushes_026.json", "freeze_area_mask"),
        ("fx_liquify_brushes_027.json", "clone_offset"),
        ("fx_liquify_brushes_028.json", "reconstruction mode number"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let ok = why.contains("Liquify") && why.to_lowercase().replace('_', " ").contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming Liquify and its {}", want.replace('_', " ")), &why, ok);
    }
    let full = fx("x", &json!({"strokes": [[0, 8, 50, 70, 0, 0, 0, 25, 50, 75, 50]]}))["parameters"].to_string();
    t.shape_refused("fx_liquify_brushes_001.json", "a Liquify whose earlier strokes are one flat list", &full.replace("[[0,8,50,70,0,0,0,25,50,75,50]]", "[0,8,50,70,0,0,0,25,50,75,50]"));
    t.shape_refused("fx_liquify_brushes_001.json", "a Liquify whose earlier stroke holds a word", &full.replace("[[0,8,", "[[\"warp\",8,"));
    t.shape_refused("fx_liquify_brushes_001.json", "a Liquify whose clone offset is one number", &full.replace("\"distortion_mesh_offset\":[0,0]", "\"distortion_mesh_offset\":[0,0],\"clone_offset\":[3]"));

    t.heading("Commands");
    let mut document = t.load("fx_liquify_brushes_013.json").document;
    let bad_tool = changed(&t, "fx_liquify_brushes_013.json", |e| if let Effect::Liquify { tool, .. } = e { *tool = "freeze".into() });
    let bad_mode = changed(&t, "fx_liquify_brushes_013.json", |e| if let Effect::Liquify { reconstruction_mode, .. } = e { *reconstruction_mode = "smooth".into() });
    let bad_stroke = changed(&t, "fx_liquify_brushes_013.json", |e| if let Effect::Liquify { strokes, .. } = e { strokes[0][0] = 12.0 });
    t.refused(
        &mut document,
        vec![
            ("tool freeze (Freeze Area is freeze_area)", set(bad_tool)),
            ("reconstruction mode smooth", set(bad_mode)),
            ("an earlier stroke with tool number 12", set(bad_stroke)),
            ("turbulent jitter keyed to end at 0", keys("turbulent_jitter", &[(0, &[70.0]), (4, &[0.0])])),
        ],
    );
    let more = changed(&t, "fx_liquify_brushes_013.json", |e| {
        if let Effect::Liquify { strokes, tool, .. } = e {
            strokes.push(vec![10.0, 6.0, 100.0, 70.0, 0.0, 0.0, 0.0, 50.0, 50.0]);
            *tool = "bloat".into();
        }
    });
    t.taken(
        &mut document,
        "fx_liquify_brushes_013.json",
        vec![
            ("a Freeze Area dab added as an earlier stroke, the last stroke Bloat", set(more)),
            ("clone offset keyed from (0, 0) to (5, -5)", keys("clone_offset", &[(0, &[0.0, 0.0]), (4, &[5.0, -5.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_liquify_brushes_001.json", 0),
        ("fx_liquify_brushes_003.json", 0),
        ("fx_liquify_brushes_007.json", 0),
        ("fx_liquify_brushes_010.json", 0),
        ("fx_liquify_brushes_012.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The missing mask (017) and the refused (018 to 028) leave nothing to the card.
    t.card_fixtures(&mut gpu, "expected_liquify_brushes.json", &(17..=28).collect::<Vec<_>>(), mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("Liquify Turbulence, brush 300, stroked across the middle", json!({"tool": "turbulence", "brush_size": 300, "stroke": [25, 50, 75, 50]})),
        (
            "Liquify Warp across the middle with a box about the middle as its freeze area mask",
            json!({"brush_size": 300, "stroke": [25, 50, 75, 50], "freeze_area_mask": 1}),
        ),
        (
            "Liquify Twirl then Reflection, Clone and Reconstruction (Amplitwist) over it",
            json!({"tool": "reconstruction", "reconstruction_mode": "amplitwist", "brush_size": 200, "brush_pressure": 100, "stroke": [45, 40, 60, 40],
                "strokes": [earlier(1, 600, 100, &[50.0, 50.0]), earlier(7, 300, 100, &[30.0, 60.0, 70.0, 60.0]), [8, 200, 100, 70, 0, 80, 0, 50, 30]]}),
        ),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(|id| json!([fx(&format!("b570-{id}"), &p)])), 3))
    .collect();
    t.card_reference(&mut gpu, &reference(|_| json!([])), &shots, mine);

    t.heading("Pictures: in `verification/D-700 pictures/`");
    let dir = repo("verification/D-700 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, &[], json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let box_ = [[180.0, 95.0, 300.0, 175.0]];
    let street = |p: J| picture(&dir, &box_, json!([fx("fx-0-0", &p)]));
    let across = json!({"brush_size": 120, "stroke": [20, 50, 80, 50]});
    let (pushed, _) = street(across.clone());
    write("2_warp.png", &pushed);

    let mut rows: Vec<(&str, &str, J, &str)> = vec![
        ("3_turbulence.png", "Turbulence, brush 120, pressure 100, across the middle: the street under the stroke scrambled smoothly", json!({"tool": "turbulence", "brush_size": 120, "brush_pressure": 100, "stroke": [20, 50, 80, 50]}), "1_before.png"),
        ("4_reflection.png", "Reflection, brush 160, pressure 100, across the middle going right: the street above the line mirrored below it", json!({"tool": "reflection", "brush_size": 160, "brush_pressure": 100, "stroke": [20, 50, 80, 50]}), "1_before.png"),
        ("5_clone.png", "2_warp.png's push, then Clone, brush 100, pressure 100, one dab low on the street with Clone Offset 80 up: the push copied down there", json!({"tool": "clone", "brush_size": 100, "brush_pressure": 100, "stroke": [50, 80], "clone_offset": [0, -80], "strokes": [earlier(0, 120, 50, &[20.0, 50.0, 80.0, 50.0])]}), "2_warp.png"),
        ("6_revert.png", "2_warp.png's push, then Reconstruction (Revert), brush 100, pressure 100, one dab in the middle: the push undone there", json!({"tool": "reconstruction", "brush_size": 100, "brush_pressure": 100, "stroke": [50, 50], "strokes": [earlier(0, 120, 50, &[20.0, 50.0, 80.0, 50.0])]}), "2_warp.png"),
        ("7_freeze_then_warp.png", "Freeze Area, brush 100, pressure 100, one dab in the middle, then 2_warp.png's push: the middle held still", json!({"brush_size": 120, "stroke": [20, 50, 80, 50], "strokes": [earlier(10, 100, 100, &[50.0, 50.0])]}), "2_warp.png"),
        ("8_freeze_mask.png", "2_warp.png's push with Freeze Area Mask 1, a box about the middle: the middle held still inside the box", json!({"brush_size": 120, "stroke": [20, 50, 80, 50], "freeze_area_mask": 1}), "2_warp.png"),
    ];
    for (name, says, p, against) in rows.drain(..) {
        let (bytes, said) = street(p);
        write(name, &bytes);
        let other = if against == "2_warp.png" { &pushed } else { &before };
        let n = differing(&bytes, other);
        t.row(&format!("{name}, {says}; differs from {against}; draws cleanly"), &format!("{said:?}, {n} pixels differ from {against}"), said.is_empty() && n > TOWN.0 * TOWN.1 / 200);
    }
    let (frozen, _) = street(json!({"brush_size": 120, "stroke": [20, 50, 80, 50], "freeze_area_mask": 1}));
    let middle = (TOWN.1 / 2 * TOWN.0 + TOWN.0 / 2) * 4;
    t.row(
        "8_freeze_mask.png: the pixel at the middle of the box is the street's, where 2_warp.png moved it",
        &format!("street {:?}, frozen {:?}, pushed {:?}", &before[middle..middle + 4], &frozen[middle..middle + 4], &pushed[middle..middle + 4]),
        frozen[middle..middle + 4] == before[middle..middle + 4] && pushed[middle..middle + 4] != before[middle..middle + 4],
    );
    let (missing, said) = street(json!({"brush_size": 120, "stroke": [20, 50, 80, 50], "freeze_area_mask": 2}));
    t.row(
        "Freeze Area Mask 2 of one: the street as it is, and EFFECT_PATH_MISSING said",
        &format!("{said:?}"),
        missing == before && said.iter().any(|s| s.starts_with("EFFECT_PATH_MISSING")),
    );

    t.finish("D-700_liquify_brushes_table.md");
}

/// The frame times, B-484's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B570_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-570: a measurement, run deliberately with --release --ignored"]
fn b570_liquify_brushes_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B570_CPU").is_ok();
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
        ("Noise, then Liquify Turbulence, brush 300, stroked across the middle", Some(json!({"tool": "turbulence", "brush_size": 300, "stroke": [25, 50, 75, 50]}))),
        ("Noise, then Liquify Warp with a freeze area mask", Some(json!({"brush_size": 300, "stroke": [25, 50, 75, 50], "freeze_area_mask": 1}))),
        (
            "Noise, then Liquify Twirl, Reflection and Clone, then Reconstruction (Amplitwist)",
            Some(json!({"tool": "reconstruction", "reconstruction_mode": "amplitwist", "brush_size": 200, "brush_pressure": 100, "stroke": [45, 40, 60, 40],
                "strokes": [earlier(1, 600, 100, &[50.0, 50.0]), earlier(7, 300, 100, &[30.0, 60.0, 70.0, 60.0]), [8, 200, 100, 70, 0, 80, 0, 50, 30]]})),
        ),
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
    let out = std::env::var("B570_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-570_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
