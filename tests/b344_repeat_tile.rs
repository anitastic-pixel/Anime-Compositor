//! B-344: D-464, After Effects' CC RepeTile under `core.repeat_tile`: Expand Right, Expand
//! Left, Expand Down, Expand Up, Tiling and Blend Borders.
//!
//! Writes `verification/D-464_repeat_tile_table.md` and draws pictures into
//! `verification/D-464 pictures/`.
//!
//! Every expected pixel is `Fixtures/repeat_tile/expected_repeat_tile.json`, written by
//! `tools/repeat_tile_reference.py` before this code existed and printed in document 25 as
//! FX-REPEAT-001 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
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

/// A Repeat Tile with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"expand_right": 0, "expand_left": 0, "expand_down": 0, "expand_up": 0, "tiling": "unfold", "blend_borders": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.repeat_tile", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_repeat_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::RepeatTile { .. }));
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
        let project = reference(|id| json!([fx(&format!("b344-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Repeat Tile {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::RepeatTile { .. }));
                t.row(
                    &format!("the reference shot, Repeat Tile {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels) in the middle of a composition `room`
/// pixels, drawn on the processor at `frame`, straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), room: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(room.0);
    comp["height"] = J::from(room.1);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["position"]["base"] = json!([room.0 as f64 / 2.0, room.1 as f64 / 2.0]);
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

#[test]
fn b344_repeat_tile() {
    let mut t = Table::new(
        "repeat_tile",
        "# D-464: Repeat Tile\n\nB-344: `core.repeat_tile` takes After Effects' CC RepeTile controls \
         (Expand Right, Expand Left, Expand Down, Expand Up, Tiling, Blend Borders). Every expected \
         pixel is `Fixtures/repeat_tile/expected_repeat_tile.json`, written by \
         `tools/repeat_tile_reference.py` before this code existed and printed in document 25 as \
         FX-REPEAT-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-REPEAT-001 to 030 (document 25)");
    t.fixtures_numbered("expected_repeat_tile.json", 1..=30);

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_repeat_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_repeat_018.json");
    t.row(
        "fx_repeat_018.json is saved with its six settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["expand_right"] == 4.0
            && saved["expand_left"] == 1.0
            && saved["expand_down"] == 0.0
            && saved["expand_up"] == 2.0
            && saved["tiling"] == "repeat"
            && saved["blend_borders"] == 0.0,
    );
    for (file, want) in [
        ("fx_repeat_026.json", "expand right"),
        ("fx_repeat_027.json", "expand up"),
        ("fx_repeat_028.json", "tiling"),
        ("fx_repeat_029.json", "blend borders"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Repeat Tile and its {want}"), &why, why.contains("Repeat Tile") && why.contains(want));
    }
    let full = r#"{"expand_right": 0, "expand_left": 0, "expand_down": 0, "expand_up": 0, "tiling": "unfold", "blend_borders": 0}"#;
    t.shape_refused("fx_repeat_001.json", "a Repeat Tile whose expand right is a word", &full.replace("\"expand_right\": 0", "\"expand_right\": \"far\""));
    t.shape_refused("fx_repeat_001.json", "a Repeat Tile without its blend borders", &full.replace(", \"blend_borders\": 0", ""));
    t.shape_refused("fx_repeat_001.json", "a Repeat Tile whose tiling is a number", &full.replace("\"unfold\"", "3"));

    t.heading("Commands");
    let mut document = t.load("fx_repeat_001.json").document;
    let over = changed(&t, "fx_repeat_001.json", |e| if let Effect::RepeatTile { expand_down, .. } = e { *expand_down = 4001.0 });
    let much = changed(&t, "fx_repeat_001.json", |e| if let Effect::RepeatTile { blend_borders, .. } = e { *blend_borders = 101.0 });
    let word = changed(&t, "fx_repeat_001.json", |e| if let Effect::RepeatTile { tiling, .. } = e { *tiling = "wallpaper".into() });
    t.refused(
        &mut document,
        vec![
            ("expand down 4001", set(over)),
            ("blend borders 101", set(much)),
            ("tiling wallpaper", set(word)),
            ("expand left keyed to 5000", keys("expand_left", &[(0, &[0.0]), (4, &[5000.0])])),
        ],
    );
    let moved = changed(&t, "fx_repeat_001.json", |e| {
        if let Effect::RepeatTile { expand_right, expand_left, expand_down, expand_up, tiling, blend_borders } = e {
            *expand_right = 12.0;
            *expand_left = 3.0;
            *expand_down = 7.0;
            *expand_up = 0.5;
            *tiling = "brick".into();
            *blend_borders = 40.0;
        }
    });
    t.taken(
        &mut document,
        "fx_repeat_001.json",
        vec![
            ("Brick, grown 12 right, 3 left, 7 down and half a pixel up, blend 40", set(moved)),
            ("blend borders keyed from 0 to 100", keys("blend_borders", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_repeat_005.json", 0),
        ("fx_repeat_011.json", 0),
        ("fx_repeat_015.json", 0),
        ("fx_repeat_017.json", 0),
        ("fx_repeat_020.json", 0),
        ("fx_repeat_022.json", 4),
        ("fx_repeat_025.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001) the effect does nothing and is left out; refused ones (026 to 030) leave
    // nothing for the card.
    let none: Vec<u32> = [1].into_iter().chain(26..=30).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            // The shot's layers fill the frame from its corner, so what grows lies outside it; a blend
            // shows on the frame itself. The fixtures check the grown pixels on the card.
            (
                "Checker Flip H, grown 200 right and 100 down, blend 30",
                json!({"tiling": "checker_flip_h", "expand_right": 200, "expand_down": 100, "blend_borders": 30}),
                3,
            ),
            (
                "Brick, grown 150 all round, blend 40",
                json!({"tiling": "brick", "expand_right": 150, "expand_left": 150, "expand_down": 150, "expand_up": 150, "blend_borders": 40}),
                3,
            ),
            ("Random, grown 80 left and up, blend 100", json!({"tiling": "random", "expand_left": 80, "expand_up": 80, "blend_borders": 100}), 3),
            ("Repeat, nothing grown, blend 60", json!({"tiling": "repeat", "blend_borders": 60}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-464 pictures/`");
    let dir = repo("verification/D-464 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    // The street in the middle of a composition twice its size, so the copies have room.
    let room = (2 * w, 2 * h);
    let all = room.0 * room.1;
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, room, json!([fx("fx-0-0", &p)]), 0);
    let grown = |tiling: &str, more: J| {
        let mut p = json!({"tiling": tiling, "expand_right": w / 2, "expand_left": w / 2, "expand_down": h / 2, "expand_up": h / 2});
        for (k, v) in more.as_object().unwrap() {
            p[k] = v.clone();
        }
        street(p)
    };
    let (before, said) = picture(&dir, "town.png", TOWN, room, json!([]), 0);
    write("1_before.png", room, &before);
    let street_pixels = covered(&before);
    t.row(
        "1_before.png, the street in the middle of a composition twice its size, the rest clear; draws cleanly",
        &format!("{said:?}, covered {street_pixels} of {all}"),
        said.is_empty() && street_pixels == w * h,
    );
    let (x0, y0) = (w / 2, h / 2);

    let (added, said) = street(json!({}));
    t.row(
        "as added (Unfold, nothing grown, no blending): the picture as it was; draws cleanly",
        &format!("{said:?}, the same {}", added == before),
        said.is_empty() && added == before,
    );

    let (repeat, said) = grown("repeat", json!({}));
    write("2_repeat.png", room, &repeat);
    let copies = (0..room.1).flat_map(|y| (0..room.0 - w).map(move |x| (x, y))).filter(|&(x, y)| at(&repeat, room.0, x, y) == at(&repeat, room.0, x + w, y)).count();
    let middle = (0..h).all(|y| (0..w).all(|x| at(&repeat, room.0, x0 + x, y0 + y) == at(&before, room.0, x0 + x, y0 + y)));
    t.row(
        "2_repeat.png, Repeat, grown half the street each way: the whole composition covered, the street kept in the middle, and the picture the same one street to the right everywhere; draws cleanly",
        &format!("{said:?}, covered {} of {all}, middle kept {middle}, the same a street to the right at {copies} of {}", covered(&repeat), room.1 * (room.0 - w)),
        said.is_empty() && covered(&repeat) == all && middle && copies == room.1 * (room.0 - w),
    );

    let (unfold, said) = grown("unfold", json!({}));
    write("3_unfold.png", room, &unfold);
    // The copy on the right is the street mirrored about its right edge.
    let mirrored = (0..h).all(|y| (0..x0).all(|k| at(&unfold, room.0, x0 + w + k, y0 + y) == at(&unfold, room.0, x0 + w - 1 - k, y0 + y)));
    t.row(
        "3_unfold.png, Unfold: the copies mirrored about the street's edges, so the right copy is the street flipped, whole composition covered; draws cleanly",
        &format!("{said:?}, mirrored {mirrored}, covered {} of {all}", covered(&unfold)),
        said.is_empty() && mirrored && covered(&unfold) == all,
    );

    let (brick, said) = grown("brick", json!({}));
    write("4_brick.png", room, &brick);
    // The rows above and below are pushed half a street across.
    let pushed = (0..y0).all(|y| (0..x0).all(|x| at(&brick, room.0, x0 + x + w / 2, y) == at(&before, room.0, x0 + x, y0 + h - y0 + y)));
    t.row(
        "4_brick.png, Brick: the rows above and below the street pushed half a street across, whole composition covered; draws cleanly",
        &format!("{said:?}, pushed {pushed}, not 2_repeat.png {}, covered {} of {all}", brick != repeat, covered(&brick)),
        said.is_empty() && pushed && brick != repeat && covered(&brick) == all,
    );

    let (blended, said) = grown("repeat", json!({"blend_borders": 100}));
    write("5_repeat_blend_100.png", room, &blended);
    let copies = (0..room.1).flat_map(|y| (0..room.0 - w).map(move |x| (x, y))).filter(|&(x, y)| at(&blended, room.0, x, y) == at(&blended, room.0, x + w, y)).count();
    t.row(
        "5_repeat_blend_100.png, Repeat with Blend Borders 100: not 2_repeat.png, still the same one street to the right everywhere, whole composition covered; draws cleanly",
        &format!("{said:?}, different {}, the same a street to the right at {copies} of {}, covered {} of {all}", blended != repeat, room.1 * (room.0 - w), covered(&blended)),
        said.is_empty() && blended != repeat && copies == room.1 * (room.0 - w) && covered(&blended) == all,
    );

    let (right, said) = street(json!({"expand_right": w / 2}));
    write("6_unfold_right_only.png", room, &right);
    let left_clear = (0..room.1).all(|y| (0..x0).all(|x| at(&right, room.0, x, y)[3] == 0));
    t.row(
        "6_unfold_right_only.png, Unfold grown half a street to the right only: the street and half a mirrored street beside it, nothing on the left; draws cleanly",
        &format!("{said:?}, covered {} (want {}), left clear {left_clear}", covered(&right), w * h + w / 2 * h),
        said.is_empty() && covered(&right) == w * h + w / 2 * h && left_clear,
    );

    let (random, said) = grown("random", json!({}));
    write("7_random.png", room, &random);
    let (twist, said2) = grown("twist", json!({}));
    write("8_twist.png", room, &twist);
    t.row(
        "7_random.png and 8_twist.png: neither is 2_repeat.png nor each other, whole composition covered; draw cleanly",
        &format!(
            "{said:?} {said2:?}, different {} {} {}, covered {} {}",
            random != repeat,
            twist != repeat,
            random != twist,
            covered(&random),
            covered(&twist)
        ),
        said.is_empty() && said2.is_empty() && random != repeat && twist != repeat && random != twist && covered(&random) == all && covered(&twist) == all,
    );

    t.finish("D-464_repeat_tile_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B344_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-344: a measurement, run deliberately with --release --ignored"]
fn b344_repeat_tile_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B344_CPU").is_ok();
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
        ("Noise, then Repeat Tile, Unfold grown 200 each way", Some(json!({"expand_right": 200, "expand_left": 200, "expand_down": 200, "expand_up": 200}))),
        (
            "the same, Brick with Blend Borders 50",
            Some(json!({"expand_right": 200, "expand_left": 200, "expand_down": 200, "expand_up": 200, "tiling": "brick", "blend_borders": 50})),
        ),
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
    let out = std::env::var("B344_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-344_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
