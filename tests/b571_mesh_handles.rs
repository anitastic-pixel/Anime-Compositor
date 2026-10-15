//! B-571: D-701, Mesh Warp's vertex handles, the gap D-603 left against After Effects' Mesh
//! Warp: each vertex's right, left, lower and upper Bezier handle, moved from where the smooth
//! mesh has it, bending the drawing even where no vertex has moved.
//!
//! Writes `verification/D-701_mesh_handles_table.md` and draws pictures into
//! `verification/D-701 pictures/`.
//!
//! Every expected pixel is `Fixtures/mesh_handles/expected_mesh_handles.json`, written by
//! `tools/mesh_handles_reference.py` before this code existed and printed in document 25 as
//! FX-MESHH-001 to 013. Tolerance 2e-5. Nothing here is a snapshot of a run. D-603's own
//! fixtures stand unchanged and are B-482's.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{changed, distance, keys, median, reference, repo, set, time_passes, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

fn mine(e: &Effect) -> bool {
    matches!(e, Effect::MeshWarp { .. })
}

/// No handle moved but those in `moves`: (i, j, which) to (x, y) per cent, `which` 0 right,
/// 1 left, 2 lower, 3 upper.
fn handles(rows: usize, cols: usize, moves: &[((usize, usize, usize), (f64, f64))]) -> Vec<f64> {
    let mut h = vec![0.0; 8 * (rows + 1) * (cols + 1)];
    for &((i, j, which), (x, y)) in moves {
        let n = 8 * (j * (cols + 1) + i) + 2 * which;
        h[n] = x;
        h[n + 1] = y;
    }
    h
}

/// A 2 by 2 mesh at rest, its middle vertex's right handle lifted 20 per cent, as FX-MESHH-003.
fn lift() -> J {
    json!({"rows": 2, "columns": 2, "vertex_handles": handles(2, 2, &[((1, 1, 0), (0.0, -20.0))])})
}

/// A 1 by 1 mesh, the top edge's two handles pulled 30 per cent down, as FX-MESHH-005.
fn sag() -> J {
    json!({"rows": 1, "columns": 1, "vertex_handles": handles(1, 1, &[((0, 0, 0), (0.0, 30.0)), ((1, 0, 1), (0.0, 30.0))])})
}

/// The middle vertex pushed to 75 per cent, its lower handle swung left and upper right, as
/// FX-MESHH-006: an S.
fn s_curve() -> J {
    json!({"rows": 2, "columns": 2,
        "distortion_mesh": [0, 0, 50, 0, 100, 0, 0, 50, 75, 50, 100, 50, 0, 100, 50, 100, 100, 100],
        "vertex_handles": handles(2, 2, &[((1, 1, 2), (-15.0, 0.0)), ((1, 1, 3), (15.0, 0.0))])})
}

/// A Mesh Warp with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"rows": 7, "columns": 7, "quality": 8, "distortion_mesh": []});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.mesh_warp", "enabled": true, "parameters": all})
}

/// `effects` on the street (in `dir`), drawn on the processor at frame 0, straight 8-bit, with
/// what it warned of.
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

#[test]
fn b571_mesh_handles() {
    let mut t = Table::new(
        "mesh_handles",
        "# D-701: Mesh Warp's vertex handles\n\nB-571: `core.mesh_warp` gains `vertex_handles`, \
         eight numbers a vertex (its right, left, lower and upper Bezier handle, each moved from \
         where the smooth mesh has it, per cent of the drawing). Each cell is the Bezier patch of \
         D-603's surface with its control points moved by the handles; none moved is D-603's \
         frame. Every expected pixel is `Fixtures/mesh_handles/expected_mesh_handles.json`, \
         written by `tools/mesh_handles_reference.py` before this code existed and printed in \
         document 25 as FX-MESHH-001 to 013. Tolerance 2e-5.\n",
    );

    t.heading("FX-MESHH-001 to 013 (document 25)");
    t.fixtures_numbered("expected_mesh_handles.json", 1..=13);

    t.heading("The file");
    let files: Vec<String> = (1..=13).map(|n| format!("fx_meshh_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_meshh_003.json");
    let h = saved["vertex_handles"].as_array().map_or(0, Vec::len);
    t.row(
        "fx_meshh_003.json is saved with its five settings, the handles one flat list of 72 numbers, the middle vertex's right handle's y -20",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5 && h == 72 && saved["vertex_handles"][33] == -20.0,
    );
    let full = fx("x", &json!({}))["parameters"].to_string();
    for (file, want) in [("fx_meshh_011.json", "vertex handles are none, or 72 numbers"), ("fx_meshh_012.json", "vertex handles")] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Mesh Warp and its {want}"), &why, why.contains("Mesh Warp") && why.contains(want));
    }
    t.shape_refused("fx_meshh_003.json", "a Mesh Warp whose vertex handles are one number", &full.replace("\"distortion_mesh\":[]", "\"distortion_mesh\":[],\"vertex_handles\":5"));

    t.heading("Commands");
    let mut document = t.load("fx_meshh_003.json").document;
    let short = changed(&t, "fx_meshh_003.json", |e| if let Effect::MeshWarp { vertex_handles, .. } = e { vertex_handles.truncate(70) });
    let far = handles(2, 2, &[((1, 1, 0), (500.0, 0.0))]);
    t.refused(
        &mut document,
        vec![
            ("a 2 by 2 mesh's handles as 70 numbers", set(short)),
            ("a handle keyed out to 500 per cent", keys("vertex_handles", &[(0, &handles(2, 2, &[])), (4, &far)])),
        ],
    );
    let nudge = changed(&t, "fx_meshh_003.json", |e| {
        if let Effect::MeshWarp { vertex_handles, .. } = e {
            vertex_handles[32] = 10.0;
        }
    });
    let lifted = handles(2, 2, &[((1, 1, 0), (0.0, -20.0))]);
    t.taken(
        &mut document,
        "fx_meshh_003.json",
        vec![
            ("the middle's right handle also 10 per cent right", set(nudge)),
            ("the handles keyed from none moved to the middle's lifted", keys("vertex_handles", &[(0, &handles(2, 2, &[])), (4, &lifted)])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_meshh_003.json", 0), ("fx_meshh_005.json", 0), ("fx_meshh_006.json", 0), ("fx_meshh_007.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 002 changes nothing, so nothing is left to the card; the refused (011 to 013) neither.
    // 007 rests at frame 0 only; 009 and 010 move a handle, so the card draws them though the
    // frame comes out as the drawing.
    t.card_fixtures(&mut gpu, "expected_mesh_handles.json", &[2, 11, 12, 13], mine);
    let shots: Vec<(&str, Project, usize)> = [
        ("Mesh Warp 2 by 2 at rest, the middle's right handle lifted 20 per cent", lift()),
        ("Mesh Warp 1 by 1, the top edge's handles pulled 30 per cent down", sag()),
        ("Mesh Warp 2 by 2, the middle pushed and its handles turned into an S", s_curve()),
    ]
    .into_iter()
    .map(|(what, p)| (what, reference(|id| json!([fx(&format!("b571-{id}"), &p)])), 3))
    .collect();
    t.card_reference(&mut gpu, &reference(|_| json!([])), &shots, mine);

    t.heading("Pictures: in `verification/D-701 pictures/`");
    let dir = repo("verification/D-701 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (w, hh) = TOWN;
    let street = |p: J| picture(&dir, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    let open = |p: &[u8]| (0..w * hh).filter(|&i| p[i * 4 + 3] == 0).count();
    t.row("1_before.png, the street with no effect, every pixel covered; draws cleanly", &format!("{said:?}, {} clear pixels", open(&before)), said.is_empty() && open(&before) == 0);

    let (zero, said) = street(json!({"vertex_handles": handles(7, 7, &[])}));
    t.row(
        "every handle of the 7 by 7 mesh written and none moved: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&zero, &before).1),
        said.is_empty() && zero == before,
    );

    let (lifted, said) = street(lift());
    write("2_handle_lifted.png", &lifted);
    t.row(
        "2_handle_lifted.png, no vertex moved, the middle vertex's right handle lifted 20 per cent: the street right of the middle bends up, the outline kept; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street, {} clear", distance(&lifted, &before).1, open(&lifted)),
        said.is_empty() && distance(&lifted, &before).1 > w * hh / 20 && open(&lifted) == 0,
    );

    let (sagged, said) = street(sag());
    write("3_top_sags.png", &sagged);
    let top = |x: usize| (0..hh).take_while(|&y| sagged[(y * w + x) * 4 + 3] == 0).count();
    t.row(
        "3_top_sags.png, one cell, the top edge's handles pulled 30 per cent down: clear above the top edge, deepest in the middle (about 22 per cent), none at the corners; draws cleanly",
        &format!("{said:?}, clear rows from the top at the left edge, a quarter and half across: {}, {}, {}", top(0), top(w / 4), top(w / 2)),
        said.is_empty() && top(0) == 0 && top(w / 4) > 0 && top(w / 2) > top(w / 4) && top(w / 2) > hh / 6,
    );

    let (s, said) = street(s_curve());
    write("4_s_curve.png", &s);
    let (bow, _) = street(json!({"rows": 2, "columns": 2, "distortion_mesh": [0, 0, 50, 0, 100, 0, 0, 50, 75, 50, 100, 50, 0, 100, 50, 100, 100, 100]}));
    write("5_bow_without_handles.png", &bow);
    t.row(
        "4_s_curve.png, the middle pushed right and its handles turned: differs from 5_bow_without_handles.png, the same push with no handle moved; draws cleanly",
        &format!("{said:?}, {} pixels differ from the bow", distance(&s, &bow).1),
        said.is_empty() && distance(&s, &bow).1 > w * hh / 20,
    );

    t.finish("D-701_mesh_handles_table.md");
}

/// The frame times, B-253's way, as B-482's: the reference shot with a Noise that changes every
/// frame on its first three layers, then the effect, every eighth frame asked for whole. With
/// `B571_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-571: a measurement, run deliberately with --release --ignored"]
fn b571_mesh_handles_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B571_CPU").is_ok();
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
    let seven = json!({"vertex_handles": handles(7, 7, &[((3, 3, 0), (5.0, -5.0)), ((3, 3, 2), (-5.0, 0.0)), ((4, 2, 1), (0.0, 8.0)), ((2, 5, 3), (6.0, 0.0))])});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Mesh Warp 2 by 2, the middle pushed and its handles turned", Some(s_curve())),
        ("Noise, then Mesh Warp 7 by 7 at rest with four handles moved, Quality 8", Some(seven)),
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
    let out = std::env::var("B571_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-571_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
