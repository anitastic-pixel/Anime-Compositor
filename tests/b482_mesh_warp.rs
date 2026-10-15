//! B-482: D-603, After Effects' Mesh Warp under `core.mesh_warp`: a grid of rows by columns
//! whose vertices are moved, the drawing bent smoothly with them (a Catmull-Rom surface through
//! the vertices) and drawn through D-601's forward mesh.
//!
//! Writes `verification/D-603_mesh_warp_table.md` and draws pictures into
//! `verification/D-603 pictures/`.
//!
//! Every expected pixel is `Fixtures/mesh_warp/expected_mesh_warp.json`, written by
//! `tools/mesh_warp_reference.py` before this code existed and printed in document 25 as
//! FX-MESH-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

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
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// The mesh at rest, per cent, flat: x then y for each vertex, row by row.
fn rest(rows: usize, cols: usize) -> Vec<f64> {
    (0..=rows).flat_map(|j| (0..=cols).flat_map(move |i| [i as f64 / cols as f64 * 100.0, j as f64 / rows as f64 * 100.0])).collect()
}

/// The mesh at rest with the vertices in `moves` ((i, j) to (x, y) per cent) moved.
fn moved(rows: usize, cols: usize, moves: &[((usize, usize), (f64, f64))]) -> Vec<f64> {
    let mut v = rest(rows, cols);
    for &((i, j), (x, y)) in moves {
        let n = 2 * (j * (cols + 1) + i);
        v[n] = x;
        v[n + 1] = y;
    }
    v
}

/// The 2 by 2 mesh with its middle vertex pushed to 75 per cent across, as FX-MESH-003.
fn push() -> J {
    json!({"rows": 2, "columns": 2, "distortion_mesh": moved(2, 2, &[((1, 1), (75.0, 50.0))])})
}

/// A 3 by 3 mesh whose top edge waves, as FX-MESH-005, at `quality`.
fn wave(quality: f64) -> J {
    json!({"rows": 3, "columns": 3, "quality": quality, "distortion_mesh": moved(3, 3, &[((1, 0), (100.0 / 3.0, 20.0)), ((2, 0), (200.0 / 3.0, -15.0)),
        ((1, 3), (100.0 / 3.0, 115.0)), ((2, 3), (200.0 / 3.0, 80.0))])})
}

/// The 2 by 2 mesh folded, its middle vertex out at 150 per cent, as FX-MESH-009.
fn fold() -> J {
    json!({"rows": 2, "columns": 2, "distortion_mesh": moved(2, 2, &[((1, 1), (150.0, 50.0))])})
}

/// A Mesh Warp with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"rows": 7, "columns": 7, "quality": 8, "distortion_mesh": []});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.mesh_warp", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=18 {
        let file = format!("fx_mesh_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::MeshWarp { .. }));
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
        let project = reference(|id| json!([fx(&format!("b482-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Mesh Warp {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::MeshWarp { .. }));
                t.row(
                    &format!("the reference shot, Mesh Warp {what} on three layers, frame {frame}, {}", quality.label()),
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
fn b482_mesh_warp() {
    let mut t = Table::new(
        "mesh_warp",
        "# D-603: Mesh Warp\n\nB-482: `core.mesh_warp` takes After Effects' Mesh Warp controls \
         (Rows, Columns, Quality, Distortion Mesh) and bends the drawing smoothly with the \
         mesh's vertices, a Catmull-Rom surface through them, drawn through D-601's forward \
         mesh. Every expected pixel is `Fixtures/mesh_warp/expected_mesh_warp.json`, written by \
         `tools/mesh_warp_reference.py` before this code existed and printed in document 25 as \
         FX-MESH-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-MESH-001 to 018 (document 25)");
    t.fixtures_numbered("expected_mesh_warp.json", 1..=18);

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_mesh_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_mesh_003.json");
    let mesh = saved["distortion_mesh"].as_array().map_or(0, Vec::len);
    t.row(
        "fx_mesh_003.json is saved with its four settings, the mesh one flat list of 18 numbers, the middle vertex's x 75",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4 && saved["rows"] == 2.0 && saved["columns"] == 2.0 && mesh == 18 && saved["distortion_mesh"][8] == 75.0 && saved["quality"] == 8.0,
    );
    let empty = t.saved_parameters("fx_mesh_001.json");
    t.row(
        "fx_mesh_001.json, the mesh at rest, is saved with an empty list",
        &empty.to_string(),
        empty["distortion_mesh"].as_array().is_some_and(Vec::is_empty),
    );
    for (file, want) in [
        ("fx_mesh_013.json", "rows"),
        ("fx_mesh_014.json", "columns"),
        ("fx_mesh_015.json", "distortion mesh is none, or 18 numbers"),
        ("fx_mesh_016.json", "distortion mesh"),
        ("fx_mesh_017.json", "quality"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Mesh Warp and its {want}"), &why, why.contains("Mesh Warp") && why.contains(want));
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused("fx_mesh_001.json", "a Mesh Warp whose rows are a word", &full.replace("\"rows\":7", "\"rows\":\"seven\""));
    t.shape_refused("fx_mesh_001.json", "a Mesh Warp without its distortion mesh", &full.replace("\"distortion_mesh\":[],", ""));
    t.shape_refused("fx_mesh_001.json", "a Mesh Warp whose distortion mesh is one number", &full.replace("\"distortion_mesh\":[]", "\"distortion_mesh\":50"));

    t.heading("Commands");
    let mut document = t.load("fx_mesh_003.json").document;
    let tall = changed(&t, "fx_mesh_003.json", |e| if let Effect::MeshWarp { rows, .. } = e { *rows = 32.0 });
    let short = changed(&t, "fx_mesh_003.json", |e| if let Effect::MeshWarp { distortion_mesh, .. } = e { distortion_mesh.truncate(16) });
    let far = moved(2, 2, &[((1, 1), (600.0, 50.0))]);
    t.refused(
        &mut document,
        vec![
            ("rows 32", set(tall)),
            ("a 2 by 2 mesh of 16 numbers", set(short)),
            ("the middle vertex keyed out to 600 per cent", keys("distortion_mesh", &[(0, &rest(2, 2)), (4, &far)])),
        ],
    );
    let nudge = changed(&t, "fx_mesh_003.json", |e| {
        if let Effect::MeshWarp { distortion_mesh, quality, .. } = e {
            distortion_mesh[8] = 60.0;
            distortion_mesh[9] = 40.0;
            *quality = 3.0;
        }
    });
    let pushed = moved(2, 2, &[((1, 1), (75.0, 50.0))]);
    t.taken(
        &mut document,
        "fx_mesh_003.json",
        vec![
            ("the middle vertex at (60, 40), quality 3", set(nudge)),
            ("the mesh keyed from rest to the middle pushed", keys("distortion_mesh", &[(0, &rest(2, 2)), (4, &pushed)])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_mesh_003.json", 0),
        ("fx_mesh_005.json", 0),
        ("fx_mesh_007.json", 2),
        ("fx_mesh_009.json", 0),
        ("fx_mesh_010.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001 and 002 change nothing, so nothing is left to the card; the refused (013 to 018)
    // neither; 007 changes nothing at frame 0 only, so it is on the card in 8 of its 10 frames.
    let none: Vec<u32> = [1, 2].into_iter().chain(13..=18).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("2 by 2, the middle vertex pushed to 75 per cent across", push(), 3),
            ("3 by 3, the top and bottom edges waving, at quality 2", wave(2.0), 3),
            ("folded (the middle vertex out at 150 per cent)", fold(), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-603 pictures/`");
    let dir = repo("verification/D-603 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let (w, h) = TOWN;
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    let open = |p: &[u8]| (0..w * h).filter(|&i| p[i * 4 + 3] == 0).count();
    t.row("1_before.png, the street with no effect, every pixel covered; draws cleanly", &format!("{said:?}, {} clear pixels", open(&before)), said.is_empty() && open(&before) == 0);

    let (added, said) = street(json!({}));
    let (written, said1) = street(json!({"distortion_mesh": rest(7, 7)}));
    t.row(
        "as added, the mesh at rest, and the 7 by 7 mesh written out where it rests: the street byte for byte; both draw cleanly",
        &format!("{said:?} {said1:?}, {} and {} pixels differ", distance(&added, &before).1, distance(&written, &before).1),
        said.is_empty() && said1.is_empty() && added == before && written == before,
    );

    let (pushed, said) = street(push());
    write("2_middle_pushed.png", TOWN, &pushed);
    t.row(
        "2_middle_pushed.png, a 2 by 2 mesh with its middle vertex pushed to 75 per cent across: the street bowed right round the middle, the outline kept, so nothing cleared; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street, {} clear", distance(&pushed, &before).1, open(&pushed)),
        said.is_empty() && distance(&pushed, &before).1 > w * h / 10 && open(&pushed) == 0,
    );

    let (waved, said) = street(wave(8.0));
    write("3_edges_waving.png", TOWN, &waved);
    let top = |x: usize| (0..h).take_while(|&y| clear(&waved, w, x, y)).count();
    t.row(
        "3_edges_waving.png, a 3 by 3 mesh with the top edge's inner vertices at 20 and -15 per cent down: clear above the top edge where it dips at a third across, none at two thirds; draws cleanly",
        &format!("{said:?}, clear rows from the top at a third and two thirds across: {}, {}", top(w / 3), top(2 * w / 3)),
        said.is_empty() && top(w / 3) > 10 && top(2 * w / 3) == 0,
    );

    let (folded, said) = street(fold());
    write("4_folded.png", TOWN, &folded);
    t.row(
        "4_folded.png, the middle vertex dragged out to 150 per cent: the mesh folds over itself, unlike the push; draws cleanly",
        &format!("{said:?}, {} pixels differ from 2_middle_pushed.png", distance(&folded, &pushed).1),
        said.is_empty() && distance(&folded, &pushed).1 > 0,
    );

    let mut rough = push();
    rough["quality"] = json!(1);
    let (low, said) = street(rough);
    write("5_middle_pushed_quality_1.png", TOWN, &low);
    t.row(
        "5_middle_pushed_quality_1.png, 2_middle_pushed.png at Quality 1: each cell left whole, the bow drawn in straight pieces, so the two differ; draws cleanly",
        &format!("{said:?}, {} pixels differ between the qualities", distance(&pushed, &low).1),
        said.is_empty() && pushed != low,
    );

    t.finish("D-603_mesh_warp_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B482_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-482: a measurement, run deliberately with --release --ignored"]
fn b482_mesh_warp_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B482_CPU").is_ok();
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
    let seven = json!({"distortion_mesh": moved(7, 7, &[((3, 3), (60.0, 55.0)), ((4, 2), (50.0, 30.0)), ((2, 5), (25.0, 75.0))])});
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Mesh Warp 2 by 2 with the middle pushed", Some(push())),
        ("Noise, then Mesh Warp 7 by 7 with three vertices moved, Quality 8", Some(seven)),
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
    let out = std::env::var("B482_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-482_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
