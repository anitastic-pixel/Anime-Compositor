//! B-329: D-449, After Effects' CC Block Load under `core.block_load`: Completion, Scans, Start
//! Cleared and Bilinear.
//!
//! Writes `verification/D-449_block_load_table.md` and draws pictures into
//! `verification/D-449 pictures/`.
//!
//! Every expected pixel is `Fixtures/block_load/expected_block_load.json`, written by
//! `tools/block_load_reference.py` before this code existed and printed in document 25 as
//! FX-BLOCKLOAD-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// A Block Load with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "scans": 4, "start_cleared": "on", "bilinear": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.block_load", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=24 {
        let file = format!("fx_blockload_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::BlockLoad { .. }));
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
        let project = reference(|id| json!([fx(&format!("b329-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Block Load {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::BlockLoad { .. }));
                t.row(
                    &format!("the reference shot, Block Load {what} on three layers, frame {frame}, {}", quality.label()),
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

/// How many neighbouring pixel pairs differ by more than `step` levels in red, across and down.
fn edges(p: &[u8], (w, h): (usize, usize), step: u8) -> usize {
    let r = |x: usize, y: usize| p[(y * w + x) * 4];
    let mut n = 0;
    for y in 0..h - 1 {
        for x in 0..w - 1 {
            n += (r(x, y).abs_diff(r(x + 1, y)) > step) as usize + (r(x, y).abs_diff(r(x, y + 1)) > step) as usize;
        }
    }
    n
}

/// Rows `ys` of two pictures `w` wide, byte for byte.
fn rows_same(a: &[u8], b: &[u8], w: usize, ys: std::ops::Range<usize>) -> bool {
    a[ys.start * w * 4..ys.end * w * 4] == b[ys.start * w * 4..ys.end * w * 4]
}

#[test]
fn b329_block_load() {
    let mut t = Table::new(
        "block_load",
        "# D-449: Block Load\n\nB-329: `core.block_load` takes After Effects' CC Block Load controls \
         (Completion, Scans, Start Cleared, Bilinear). Every expected pixel is \
         `Fixtures/block_load/expected_block_load.json`, written by `tools/block_load_reference.py` \
         before this code existed and printed in document 25 as FX-BLOCKLOAD-001 to 024. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-BLOCKLOAD-001 to 024 (document 25)");
    t.fixtures_numbered("expected_block_load.json", 1..=24);

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_blockload_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_blockload_011.json");
    t.row(
        "fx_blockload_011.json is saved with its four settings, and not the draft's pixel width",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4 && saved["scans"] == 2.0 && saved["start_cleared"] == "off" && saved["bilinear"] == "on" && saved.get("pixel").is_none(),
    );
    for (file, want) in [
        ("fx_blockload_018.json", "completion"),
        ("fx_blockload_019.json", "completion"),
        ("fx_blockload_020.json", "scans"),
        ("fx_blockload_021.json", "scans"),
        ("fx_blockload_022.json", "start cleared"),
        ("fx_blockload_023.json", "bilinear"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Block Load and its {want}"), &why, why.contains("Block Load") && why.contains(want));
    }
    let full = r#"{"completion": 0, "scans": 4, "start_cleared": "on", "bilinear": "off"}"#;
    t.shape_refused("fx_blockload_001.json", "a Block Load whose scans is a word", &full.replace("\"scans\": 4", "\"scans\": \"many\""));
    t.shape_refused("fx_blockload_001.json", "a Block Load without its completion", &full.replace("\"completion\": 0, ", ""));
    t.shape_refused("fx_blockload_001.json", "a Block Load whose bilinear is a number", &full.replace("\"off\"", "0"));

    t.heading("Commands");
    let mut document = t.load("fx_blockload_001.json").document;
    let few = changed(&t, "fx_blockload_001.json", |e| if let Effect::BlockLoad { scans, .. } = e { *scans = 0.5 });
    let bad = changed(&t, "fx_blockload_001.json", |e| if let Effect::BlockLoad { bilinear, .. } = e { *bilinear = "On".into() });
    t.refused(
        &mut document,
        vec![
            ("scans 0.5", set(few)),
            ("bilinear On (a capital)", set(bad)),
            ("completion keyed to 150", keys("completion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let smooth = changed(&t, "fx_blockload_001.json", |e| {
        if let Effect::BlockLoad { completion, bilinear, .. } = e {
            *completion = 60.0;
            *bilinear = "on".into();
        }
    });
    t.taken(
        &mut document,
        "fx_blockload_001.json",
        vec![
            ("completion 60, bilinear on", set(smooth)),
            ("scans keyed from 1 to 16", keys("scans", &[(0, &[1.0]), (4, &[16.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_blockload_005.json", 0),
        ("fx_blockload_011.json", 0),
        ("fx_blockload_012.json", 0),
        ("fx_blockload_014.json", 1),
        ("fx_blockload_016.json", 0),
        ("fx_blockload_017.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Every pass done (002, 013) leaves nothing to do, and refused ones (018 to 024) nothing for
    // the card.
    let mut none: Vec<u32> = vec![2, 13];
    none.extend(18..=24);
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (nothing yet)", json!({}), 3),
            ("completion 50", json!({"completion": 50}), 3),
            ("start cleared off, scans 6, completion 30, bilinear", json!({"start_cleared": "off", "scans": 6, "completion": 30, "bilinear": "on"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-449 pictures/`");
    let dir = repo("verification/D-449 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let mosaic = |size: f64| picture(&dir, "town.png", TOWN, json!([{"instance_id": "fx-0-0", "type_id": "core.mosaic", "enabled": true, "parameters": {"size": size}}]), 0).0;
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let w = TOWN.0;

    let (added, said) = street(json!({}));
    t.row(
        "as added (Completion 0, Start Cleared on): nothing shows yet, every pixel clear; draws cleanly (no picture: it would be empty)",
        &format!("{said:?}, pixels with any cover {}", added.chunks_exact(4).filter(|p| p[3] > 0).count()),
        said.is_empty() && added.chunks_exact(4).all(|p| p[3] == 0),
    );

    let (coarse, said) = street(json!({"start_cleared": "off"}));
    write("2_coarsest.png", TOWN, &coarse);
    t.row(
        "2_coarsest.png, Start Cleared off, Completion 0, Scans 4: the street in blocks 16 pixels square, the same picture as Mosaic size 16; draws cleanly",
        &format!("{said:?}, the same as Mosaic 16: {}", coarse == mosaic(16.0)),
        said.is_empty() && coarse == mosaic(16.0),
    );

    let (half, said) = street(json!({"completion": 50}));
    write("3_half_way.png", TOWN, &half);
    // Five passes; half way is the third (blocks 4 wide) half done: 34 of its 68 rows of blocks.
    t.row(
        "3_half_way.png, Completion 50: the top 136 rows in blocks 4 pixels square (Mosaic 4), the rows below still in blocks 8 square (Mosaic 8); draws cleanly",
        &format!("{said:?}, top as Mosaic 4: {}, below as Mosaic 8: {}", rows_same(&half, &mosaic(4.0), w, 0..136), rows_same(&half, &mosaic(8.0), w, 136..TOWN.1)),
        said.is_empty() && rows_same(&half, &mosaic(4.0), w, 0..136) && rows_same(&half, &mosaic(8.0), w, 136..TOWN.1),
    );

    let (blended, said) = street(json!({"completion": 50, "bilinear": "on"}));
    write("4_half_way_bilinear.png", TOWN, &blended);
    t.row(
        "4_half_way_bilinear.png, Completion 50, Bilinear: 3_half_way.png with the blocks blended smoothly, so fewer sudden jumps (more than 24 levels) between neighbours; draws cleanly",
        &format!("{said:?}, neighbour pairs differing by more than 24 in red: flat {}, blended {}", edges(&half, TOWN, 24), edges(&blended, TOWN, 24)),
        said.is_empty() && edges(&blended, TOWN, 24) < edges(&half, TOWN, 24),
    );

    let (late, said) = street(json!({"completion": 90}));
    write("5_nearly_done.png", TOWN, &late);
    // The last pass (blocks 1 wide, the street itself) half done: rows 0 to 134 as drawn.
    t.row(
        "5_nearly_done.png, Completion 90: the top 135 rows the street as it is, the rows below in blocks 2 pixels square (Mosaic 2); draws cleanly",
        &format!("{said:?}, top as the street: {}, below as Mosaic 2: {}", rows_same(&late, &before, w, 0..135), rows_same(&late, &mosaic(2.0), w, 135..TOWN.1)),
        said.is_empty() && rows_same(&late, &before, w, 0..135) && rows_same(&late, &mosaic(2.0), w, 135..TOWN.1),
    );

    let (done, said) = street(json!({"completion": 100}));
    t.row("Completion 100: the street byte for byte; draws cleanly", &format!("{said:?}, the same: {}", done == before), said.is_empty() && done == before);

    t.finish("D-449_block_load_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B329_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-329: a measurement, run deliberately with --release --ignored"]
fn b329_block_load_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B329_CPU").is_ok();
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
        ("Noise, then Block Load completion 50", Some(json!({"completion": 50}))),
        ("Noise, then Block Load completion 50, scans 8, bilinear (two blended pictures, blocks 16 and 32 wide)", Some(json!({"completion": 50, "scans": 8, "bilinear": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}b"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B329_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-329_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
