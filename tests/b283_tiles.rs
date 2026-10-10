//! B-283: D-404 Tiles, after CycoreFX's CC Tiler ("Distort" in `docs/effects/EFFECTS.md`):
//! the layer shrunk to Scale and repeated in a grid across its own size, by Motion Tile's rule
//! (D-304), then mixed with the original by Blend w. Original.
//!
//! Writes `verification/D-404_tiles_table.md`.
//!
//! Every expected pixel is `Fixtures/tiles/expected_tiles.json`, written by
//! `tools/tiles_reference.py` before this code existed and printed in document 25 as
//! FX-TILES-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run. Tiles with no blend
//! must be Motion Tile's sized tile byte for byte.
//!
//! Also draws the street into `verification/D-404 pictures/`.

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

fn tiles(scale: f64, center: [f64; 2], blend: f64) -> Effect {
    Effect::Tiles { scale, center, blend }
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
        "core.tiles" => json!({"scale": 50, "center": [50, 50], "blend": 0}),
        "core.motion_tile" => json!({"output_width": 100, "output_height": 100, "mirror": "off", "tile_center": [50, 50], "tile_width": 100, "tile_height": 100}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b283-{id}"), p)]));
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
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
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

// --- Tiles ----------------------------------------------------------------------------------

fn is_tiles(e: &Effect) -> bool {
    matches!(e, Effect::Tiles { .. })
}

#[test]
fn b283_tiles() {
    let mut t = Table::new(
        "tiles",
        "# D-404: Tiles\n\nB-283, after CycoreFX's CC Tiler: the layer shrunk to Scale per cent and \
         repeated in a grid across its own size, one tile centred on Center, by Motion Tile's rule \
         (D-304: each pixel the mean of a grid of points, a pixel's worth of the layer each), then \
         mixed with the untiled layer by Blend w. Original. The layer never grows. Every expected \
         pixel is `Fixtures/tiles/expected_tiles.json`, written by `tools/tiles_reference.py` before \
         this code existed and printed in document 25 as FX-TILES-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-TILES-001 to 021 (document 25)");
    t.fixtures("expected_tiles.json");

    t.heading("Tiles is Motion Tile's sized tile");
    let dir = repo("verification/D-404 pictures");
    fs::create_dir_all(&dir).unwrap();
    for (what, p, q) in [
        ("as it starts, scale 50", json!({}), json!({"tile_width": 50, "tile_height": 50})),
        ("scale 25, centre 30, 40", json!({"scale": 25, "center": [30, 40]}), json!({"tile_width": 25, "tile_height": 25, "tile_center": [30, 40]})),
        ("scale 7", json!({"scale": 7}), json!({"tile_width": 7, "tile_height": 7})),
    ] {
        let (a, _) = picture(&dir, json!([fx("core.tiles", "fx-0-0", &p)]));
        let (b, _) = picture(&dir, json!([fx("core.motion_tile", "fx-0-0", &q)]));
        t.row(&format!("Tiles {what}, blend 0, on the street is Motion Tile's picture with the same tile byte for byte"), &format!("{} pixels differ", distance(&a, &b).1), a == b);
    }

    t.heading("The file");
    let all = files("fx_tiles", 21);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_tiles_009.json");
    t.row(
        "fx_tiles_009.json is saved with its scale, centre and blend",
        &saved.to_string(),
        saved["scale"] == 50 && saved["center"] == json!([50, 50]) && saved["blend"] == 50,
    );
    why(
        &mut t,
        &[
            ("fx_tiles_016.json", "Tiles's scale runs from 1 to 100, and this is 0.5."),
            ("fx_tiles_017.json", "Tiles's scale runs from 1 to 100, and this is 100.5."),
            ("fx_tiles_018.json", "Tiles's blend runs from 0 to 100, and this is -1."),
            ("fx_tiles_019.json", "Tiles's blend runs from 0 to 100, and this is 101."),
            ("fx_tiles_020.json", "Tiles's center runs from -1000 to 1000, and this is 1001."),
        ],
    );
    t.shape_refused("fx_tiles_001.json", "a Tiles with no `blend`", r#"{"scale": 50, "center": [50, 50]}"#);
    t.shape_refused("fx_tiles_001.json", "a Tiles with a centre of one number", r#"{"scale": 50, "center": 50, "blend": 0}"#);

    t.heading("Commands");
    let mut document = t.load("fx_tiles_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("scale 0.5", set(tiles(0.5, [50.0, 50.0], 0.0))),
            ("blend 100.5", set(tiles(50.0, [50.0, 50.0], 100.5))),
            ("scale keyed to 0", keys("scale", &[(0, &[50.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_tiles_001.json",
        vec![
            ("scale 25, centre 30, 40, blend 20", set(tiles(25.0, [30.0, 40.0], 20.0))),
            ("scale keyed from 100 to 20", keys("scale", &[(0, &[100.0]), (4, &[20.0])])),
            ("centre keyed from 50, 50 to 10, 90", keys("center", &[(0, &[50.0, 50.0]), (4, &[10.0, 90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_tiles_001.json", 0), ("fx_tiles_004.json", 0), ("fx_tiles_006.json", 0), ("fx_tiles_007.json", 0), ("fx_tiles_009.json", 0), ("fx_tiles_015.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_tiles", 21, &[2, 10, 16, 17, 18, 19, 20, 21], is_tiles);
    card_reference(
        &mut t,
        &mut gpu,
        "Tiles",
        "core.tiles",
        &[
            ("as it starts (scale 50, centre 50, 50, blend 0)", json!({})),
            ("scale 25, centre 30, 40, blend 20", json!({"scale": 25, "center": [30, 40], "blend": 20})),
            ("scale 7, centre -120, 300", json!({"scale": 7, "center": [-120, 300]})),
        ],
        is_tiles,
    );

    street(
        &mut t,
        "D-404",
        "core.tiles",
        &[
            ("as_added", json!({}), "as it starts: four copies of the street at half size, 2 by 2", changed),
            ("scale_25", json!({"scale": 25}), "scale 25: 4 by 4 copies at a quarter size", changed),
            ("off_centre", json!({"scale": 33, "center": [20, 30]}), "scale 33, centre 20, 30: the grid slid up and left, tiles cut at the edges", changed),
            ("blend_50", json!({"scale": 25, "blend": 50}), "scale 25, blend 50: the grid of copies seen through the whole street", changed),
            ("scale_3", json!({"scale": 3}), "scale 3: copies so small the street becomes a fine texture", changed),
        ],
    );

    t.finish("D-404_tiles_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B283_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-283: a measurement, run deliberately with --release --ignored"]
fn b283_tiles_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B283_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Motion Tile, tiles 25 per cent", Some(("core.motion_tile", json!({"tile_width": 25, "tile_height": 25})))),
        ("Noise, then Tiles, scale 25", Some(("core.tiles", json!({"scale": 25})))),
        ("Noise, then Tiles, scale 7, blend 20", Some(("core.tiles", json!({"scale": 7, "blend": 20})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B283_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-283_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
