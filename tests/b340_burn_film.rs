//! B-340: D-460, After Effects' CC Burn Film under `core.burn_film`: Burn, Center and Random
//! Seed.
//!
//! Writes `verification/D-460_burn_film_table.md` and draws pictures into
//! `verification/D-460 pictures/`.
//!
//! Every expected pixel is `Fixtures/burn_film/expected_burn_film.json`, written by
//! `tools/burn_film_reference.py` before this code existed and printed in document 25 as
//! FX-BURN-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Burn Films the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::BurnFilm { .. })))
        .count()
}

/// A Burn Film with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"burn": 0, "center": [50, 50], "seed": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.burn_film", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=20 {
        let file = format!("fx_burn_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b340-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Burn Film {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Burn Film {what} on three layers, frame {frame}, {}", quality.label()),
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

#[test]
fn b340_burn_film() {
    let mut t = Table::new(
        "burn_film",
        "# D-460: Burn Film\n\nB-340: `core.burn_film` takes After Effects' CC Burn Film controls \
         (Burn, Center, Random Seed). Every expected pixel is \
         `Fixtures/burn_film/expected_burn_film.json`, written by `tools/burn_film_reference.py` \
         before this code existed and printed in document 25 as FX-BURN-001 to 020. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-BURN-001 to 020 (document 25)");
    t.fixtures_numbered("expected_burn_film.json", 1..=20);

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_burn_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_burn_008.json");
    t.row(
        "fx_burn_008.json is saved with its three settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3 && saved["burn"] == 50.0 && saved["seed"] == 7.0 && saved["center"] == json!([50, 50]),
    );
    for (file, want) in [
        ("fx_burn_015.json", "burn"),
        ("fx_burn_016.json", "burn"),
        ("fx_burn_017.json", "center"),
        ("fx_burn_018.json", "seed"),
        ("fx_burn_019.json", "seed"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Burn Film and its {want}"), &why, why.contains("Burn Film") && why.contains(want));
    }
    let full = r#"{"burn": 0, "center": [50, 50], "seed": 0}"#;
    t.shape_refused("fx_burn_001.json", "a Burn Film whose burn is a word", &full.replace("\"burn\": 0", "\"burn\": \"lots\""));
    t.shape_refused("fx_burn_001.json", "a Burn Film without its seed", &full.replace(", \"seed\": 0", ""));
    t.shape_refused("fx_burn_001.json", "a Burn Film whose center is one number", &full.replace("[50, 50]", "50"));

    t.heading("Commands");
    let mut document = t.load("fx_burn_001.json").document;
    let over = changed(&t, "fx_burn_001.json", |e| if let Effect::BurnFilm { burn, .. } = e { *burn = 101.0 });
    let far = changed(&t, "fx_burn_001.json", |e| if let Effect::BurnFilm { center, .. } = e { *center = [50.0, -1001.0] });
    t.refused(
        &mut document,
        vec![
            ("burn 101", set(over)),
            ("center 50, -1001", set(far)),
            ("burn keyed to 150", keys("burn", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let moved = changed(&t, "fx_burn_001.json", |e| {
        if let Effect::BurnFilm { burn, center, seed } = e {
            *burn = 40.0;
            *center = [20.0, 30.0];
            *seed = 12.0;
        }
    });
    t.taken(
        &mut document,
        "fx_burn_001.json",
        vec![
            ("burn 40, center 20, 30, seed 12", set(moved)),
            ("center keyed from 0, 0 to 100, 100", keys("center", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_burn_003.json", 0),
        ("fx_burn_004.json", 0),
        ("fx_burn_008.json", 0),
        ("fx_burn_011.json", 2),
        ("fx_burn_013.json", 0),
        ("fx_burn_014.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Burn 0 (001) leaves nothing to do, and refused ones (015 to 020) nothing for the card.
    let mut none: Vec<u32> = vec![1];
    none.extend(15..=20);
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("burn 30", json!({"burn": 30}), 3),
            ("burn 60, center 20, 30, seed 5", json!({"burn": 60, "center": [20, 30], "seed": 5}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-460 pictures/`");
    let dir = repo("verification/D-460 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (w, h) = TOWN;
    let corners = [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1)];

    let (added, said) = street(json!({}));
    t.row("as added (Burn 0): the street byte for byte; draws cleanly", &format!("{said:?}, the same: {}", added == before), said.is_empty() && added == before);

    let (thirty, said) = street(json!({"burn": 30}));
    write("2_burn_30.png", TOWN, &thirty);
    // Char: black with some but not all of its cover; scorched: its cover kept, darker, not black.
    let pairs = || thirty.chunks_exact(4).zip(before.chunks_exact(4));
    let char_line = pairs().filter(|(p, q)| p[..3] == [0, 0, 0] && p[3] > 0 && p[3] < q[3]).count();
    let scorched = pairs().filter(|(p, q)| p[3] == q[3] && p[..3] != [0, 0, 0] && p != q && p[..3].iter().zip(&q[..3]).all(|(a, b)| a <= b)).count();
    let untouched = corners.iter().all(|&(x, y)| at(&thirty, w, x, y) == at(&before, w, x, y));
    t.row(
        "2_burn_30.png, Burn 30: a hole through the middle (the middle pixel clear), ringed by a black char line and a scorched edge, the four corners untouched; draws cleanly",
        &format!("{said:?}, middle cover {}, char pixels {char_line}, scorched pixels {scorched}, corners untouched {untouched}", at(&thirty, w, w / 2, h / 2)[3]),
        said.is_empty() && at(&thirty, w, w / 2, h / 2)[3] == 0 && char_line > 0 && scorched > 0 && untouched,
    );

    let (sixty, said) = street(json!({"burn": 60}));
    write("3_burn_60.png", TOWN, &sixty);
    t.row(
        "3_burn_60.png, Burn 60: more burned away than at 30; draws cleanly",
        &format!("{said:?}, pixels with any cover: before {}, burn 30 {}, burn 60 {}", covered(&before), covered(&thirty), covered(&sixty)),
        said.is_empty() && covered(&sixty) < covered(&thirty) && covered(&thirty) < covered(&before),
    );

    let (corner, said) = street(json!({"burn": 30, "center": [0, 0]}));
    write("4_from_the_corner.png", TOWN, &corner);
    t.row(
        "4_from_the_corner.png, Center 0, 0, Burn 30: the burn starts at the top left (that corner clear), and the picture is not 2_burn_30.png; draws cleanly",
        &format!("{said:?}, top-left cover {}, different: {}", at(&corner, w, 0, 0)[3], corner != thirty),
        said.is_empty() && at(&corner, w, 0, 0)[3] == 0 && corner != thirty,
    );

    let (seven, said) = street(json!({"burn": 30, "seed": 7}));
    write("5_seed_7.png", TOWN, &seven);
    t.row(
        "5_seed_7.png, Random Seed 7, Burn 30: ragged differently from 2_burn_30.png; draws cleanly",
        &format!("{said:?}, different: {}", seven != thirty),
        said.is_empty() && seven != thirty,
    );

    let (gone, said) = street(json!({"burn": 100}));
    t.row("Burn 100: every pixel clear; draws cleanly", &format!("{said:?}, pixels with any cover {}", covered(&gone)), said.is_empty() && covered(&gone) == 0);

    t.finish("D-460_burn_film_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B340_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-340: a measurement, run deliberately with --release --ignored"]
fn b340_burn_film_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B340_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 2] = [
        ("Noise alone", None),
        ("Noise, then Burn Film burn 50", Some(json!({"burn": 50}))),
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
    let out = std::env::var("B340_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-340_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
