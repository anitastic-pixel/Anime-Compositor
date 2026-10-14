//! B-341: D-461, After Effects' CC HexTile under `core.hex_tile`: Render, Radius, Center, Lock
//! Center Tile, Rotate and Smearing.
//!
//! Writes `verification/D-461_hex_tile_table.md` and draws pictures into
//! `verification/D-461 pictures/`.
//!
//! Every expected pixel is `Fixtures/hex_tile/expected_hex_tile.json`, written by
//! `tools/hex_tile_reference.py` before this code existed and printed in document 25 as
//! FX-HEX-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// The Hex Tiles the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::HexTile { .. })))
        .count()
}

/// A Hex Tile with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"render": "tile", "radius": 50, "center": [50, 50], "lock_center": "off", "rotate": 0, "smearing": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.hex_tile", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=22 {
        let file = format!("fx_hex_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b341-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Hex Tile {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Hex Tile {what} on three layers, frame {frame}, {}", quality.label()),
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

/// How many pixels of `p` are covered at all.
fn covered(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0).count()
}

/// Pixel (x, y) of a picture `w` wide.
fn at(p: &[u8], w: usize, x: usize, y: usize) -> &[u8] {
    &p[(y * w + x) * 4..(y * w + x) * 4 + 4]
}

/// How many pixels of `p` and `q` are within one level of each other, out of those of `p`.
fn alike(p: &[u8], q: &[u8]) -> usize {
    p.chunks_exact(4).zip(q.chunks_exact(4)).filter(|(a, b)| a.iter().zip(*b).all(|(x, y)| x.abs_diff(*y) <= 1)).count()
}

#[test]
fn b341_hex_tile() {
    let mut t = Table::new(
        "hex_tile",
        "# D-461: Hex Tile\n\nB-341: `core.hex_tile` takes After Effects' CC HexTile controls \
         (Render, Radius, Center, Lock Center Tile, Rotate, Smearing). Every expected pixel is \
         `Fixtures/hex_tile/expected_hex_tile.json`, written by `tools/hex_tile_reference.py` \
         before this code existed and printed in document 25 as FX-HEX-001 to 022. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-HEX-001 to 022 (document 25)");
    t.fixtures_numbered("expected_hex_tile.json", 1..=22);

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_hex_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_hex_014.json");
    t.row(
        "fx_hex_014.json is saved with its six settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["render"] == "fold_seamless"
            && saved["radius"] == 4.0
            && saved["center"] == json!([40, 60])
            && saved["lock_center"] == "off"
            && saved["rotate"] == 15.0
            && saved["smearing"] == 25.0,
    );
    for (file, want) in [
        ("fx_hex_015.json", "radius"),
        ("fx_hex_016.json", "radius"),
        ("fx_hex_017.json", "render"),
        ("fx_hex_018.json", "lock center"),
        ("fx_hex_019.json", "smearing"),
        ("fx_hex_020.json", "rotate"),
        ("fx_hex_021.json", "center"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Hex Tile and its {want}"), &why, why.contains("Hex Tile") && why.contains(want));
    }
    let full = r#"{"render": "tile", "radius": 50, "center": [50, 50], "lock_center": "off", "rotate": 0, "smearing": 0}"#;
    t.shape_refused("fx_hex_001.json", "a Hex Tile whose radius is a word", &full.replace("\"radius\": 50", "\"radius\": \"big\""));
    t.shape_refused("fx_hex_001.json", "a Hex Tile without its smearing", &full.replace(", \"smearing\": 0", ""));
    t.shape_refused("fx_hex_001.json", "a Hex Tile whose center is one number", &full.replace("[50, 50]", "50"));
    t.shape_refused("fx_hex_001.json", "a Hex Tile whose render is a number", &full.replace("\"tile\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_hex_001.json").document;
    let over = changed(&t, "fx_hex_001.json", |e| if let Effect::HexTile { radius, .. } = e { *radius = 4001.0 });
    let far = changed(&t, "fx_hex_001.json", |e| if let Effect::HexTile { center, .. } = e { *center = [50.0, -1001.0] });
    let word = changed(&t, "fx_hex_001.json", |e| if let Effect::HexTile { render, .. } = e { *render = "honeycomb".into() });
    t.refused(
        &mut document,
        vec![
            ("radius 4001", set(over)),
            ("center 50, -1001", set(far)),
            ("render honeycomb", set(word)),
            ("radius keyed to 5000", keys("radius", &[(0, &[50.0]), (4, &[5000.0])])),
        ],
    );
    let moved = changed(&t, "fx_hex_001.json", |e| {
        if let Effect::HexTile { render, radius, center, lock_center, rotate, smearing } = e {
            *render = "fold_aligned".into();
            *radius = 12.0;
            *center = [20.0, 30.0];
            *lock_center = "on".into();
            *rotate = 45.0;
            *smearing = 60.0;
        }
    });
    t.taken(
        &mut document,
        "fx_hex_001.json",
        vec![
            ("Fold Aligned, radius 12, center 20, 30, locked, rotate 45, smearing 60", set(moved)),
            ("center keyed from 0, 0 to 100, 100", keys("center", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_hex_002.json", 0),
        ("fx_hex_003.json", 0),
        ("fx_hex_004.json", 0),
        ("fx_hex_008.json", 0),
        ("fx_hex_010.json", 2),
        ("fx_hex_013.json", 0),
        ("fx_hex_014.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused ones (015 to 022) leave nothing for the card.
    let none: Vec<u32> = (15..=22).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("radius 40", json!({"radius": 40}), 3),
            (
                "Fold Seamlessly, radius 60, center 30, 40, rotate 20, smearing 30",
                json!({"render": "fold_seamless", "radius": 60, "center": [30, 40], "rotate": 20, "smearing": 30}),
                3,
            ),
            ("Fold Aligned, radius 25, locked, center 70, 20", json!({"render": "fold_aligned", "radius": 25, "lock_center": "on", "center": [70, 20]}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-461 pictures/`");
    let dir = repo("verification/D-461 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (w, h) = TOWN;
    let all = w * h;

    let (added, said) = street(json!({}));
    t.row(
        "as added (Tile, radius 50): the middle pixel kept, the picture changed, every pixel still covered; draws cleanly",
        &format!("{said:?}, middle kept {}, changed {}, covered {} of {all}", at(&added, w, w / 2, h / 2) == at(&before, w, w / 2, h / 2), added != before, covered(&added)),
        said.is_empty() && at(&added, w, w / 2, h / 2) == at(&before, w, w / 2, h / 2) && added != before && covered(&added) == all,
    );

    let (tile, said) = street(json!({"radius": 20}));
    write("2_tile_radius_20.png", TOWN, &tile);
    // A flat-topped honeycomb of radius 20 repeats every 60 pixels across.
    let repeats = (0..h).flat_map(|y| (0..w - 60).map(move |x| (x, y))).filter(|&(x, y)| alike(at(&tile, w, x, y), at(&tile, w, x + 60, y)) == 1).count();
    let checked = h * (w - 60);
    t.row(
        "2_tile_radius_20.png, Tile, radius 20: the street's middle repeated in a honeycomb, the picture the same 60 pixels to the right (within 1 level) almost everywhere, every pixel covered; draws cleanly",
        &format!("{said:?}, the same 60 to the right at {repeats} of {checked} pixels, covered {} of {all}", covered(&tile)),
        said.is_empty() && repeats * 100 >= checked * 99 && covered(&tile) == all,
    );

    let (aligned, said) = street(json!({"radius": 20, "render": "fold_aligned"}));
    write("3_fold_aligned.png", TOWN, &aligned);
    let (seamless, said2) = street(json!({"radius": 20, "render": "fold_seamless"}));
    write("4_fold_seamless.png", TOWN, &seamless);
    t.row(
        "3_fold_aligned.png and 4_fold_seamless.png, the folds at radius 20: neither is 2_tile_radius_20.png nor each other, every pixel covered; draw cleanly",
        &format!("{said:?} {said2:?}, different {} {} {}, covered {} {}", aligned != tile, seamless != tile, aligned != seamless, covered(&aligned), covered(&seamless)),
        said.is_empty() && said2.is_empty() && aligned != tile && seamless != tile && aligned != seamless && covered(&aligned) == all && covered(&seamless) == all,
    );

    let (turned, said) = street(json!({"radius": 20, "rotate": 30}));
    write("5_rotate_30.png", TOWN, &turned);
    t.row(
        "5_rotate_30.png, radius 20 turned 30 degrees: not 2_tile_radius_20.png, the middle pixel kept (within 1 level); draws cleanly",
        &format!("{said:?}, different {}, middle kept within 1 level {}", turned != tile, alike(at(&turned, w, w / 2, h / 2), at(&before, w, w / 2, h / 2)) == 1),
        said.is_empty() && turned != tile && alike(at(&turned, w, w / 2, h / 2), at(&before, w, w / 2, h / 2)) == 1,
    );

    let (smeared, said) = street(json!({"radius": 20, "smearing": 50}));
    write("6_smearing_50.png", TOWN, &smeared);
    let (whole, said2) = street(json!({"radius": 20, "smearing": 100}));
    t.row(
        "6_smearing_50.png, radius 20 smeared half way: neither the tiles nor the street; at Smearing 100 every tile shows its own place, the street within 1 level everywhere; draws cleanly",
        &format!("{said:?} {said2:?}, different {} {}, Smearing 100 within 1 level at {} of {all}", smeared != tile, smeared != before, alike(&whole, &before)),
        said.is_empty() && said2.is_empty() && smeared != tile && smeared != before && alike(&whole, &before) == all,
    );

    let (locked, said) = street(json!({"radius": 20, "center": [30, 40], "lock_center": "on"}));
    write("7_locked_center_30_40.png", TOWN, &locked);
    let (free, said2) = street(json!({"radius": 20, "center": [30, 40]}));
    t.row(
        "7_locked_center_30_40.png, the tiles showing the street at 30, 40 with the honeycomb held at the middle: not the unlocked picture, and its middle pixel is the street's at 30, 40; draws cleanly",
        &format!("{said:?} {said2:?}, different {}, middle is 30, 40's {}", locked != free, at(&locked, w, w / 2, h / 2) == at(&before, w, w * 30 / 100, h * 40 / 100)),
        said.is_empty() && said2.is_empty() && locked != free && at(&locked, w, w / 2, h / 2) == at(&before, w, w * 30 / 100, h * 40 / 100),
    );

    t.finish("D-461_hex_tile_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B341_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-341: a measurement, run deliberately with --release --ignored"]
fn b341_hex_tile_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B341_CPU").is_ok();
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
        ("Noise, then Hex Tile radius 50", Some(json!({"radius": 50}))),
        ("the same, Fold Seamlessly, rotate 20, smearing 30", Some(json!({"radius": 50, "render": "fold_seamless", "rotate": 20, "smearing": 30}))),
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
    let out = std::env::var("B341_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-341_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
