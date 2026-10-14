//! B-346: D-466, After Effects' Strobe Light under `core.strobe_light`: Strobe Color, Blend With
//! Original, Strobe Duration, Strobe Period, Random Strobe Probability, Strobe, Strobe Operator and
//! Random Seed.
//!
//! Writes `verification/D-466_strobe_light_table.md` and draws pictures into
//! `verification/D-466 pictures/`.
//!
//! Every expected pixel is `Fixtures/strobe_light/expected_strobe_light.json`, written by
//! `tools/strobe_light_reference.py` before this code existed and printed in document 25 as
//! FX-STROBE-001 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Strobe Lights the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::StrobeLight { .. })))
        .count()
}

/// A Strobe Light with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({
        "strobe_color": "#ffffff",
        "blend_with_original": 0,
        "strobe_duration": 0.1,
        "strobe_period": 1,
        "random_strobe_probability": 0,
        "strobe": "color_only",
        "strobe_operator": "copy",
        "random_seed": 0
    });
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.strobe_light", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_strobe_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b346-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Strobe Light {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Strobe Light {what} on three layers, frame {frame}, {}", quality.label()),
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

#[test]
fn b346_strobe_light() {
    let mut t = Table::new(
        "strobe_light",
        "# D-466: Strobe Light\n\nB-346: `core.strobe_light` takes After Effects' Strobe Light controls \
         (Strobe Color, Blend With Original, Strobe Duration, Strobe Period, Random Strobe Probability, \
         Strobe, Strobe Operator, Random Seed). Every expected pixel is \
         `Fixtures/strobe_light/expected_strobe_light.json`, written by \
         `tools/strobe_light_reference.py` before this code existed and printed in document 25 as \
         FX-STROBE-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-STROBE-001 to 030 (document 25)");
    t.fixtures_numbered("expected_strobe_light.json", 1..=30);

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_strobe_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_strobe_013.json");
    t.row(
        "fx_strobe_013.json is saved with its eight settings, and not the time",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8 && saved["strobe_color"] == "#ff0000" && saved["blend_with_original"] == 50.0 && saved["strobe_operator"] == "copy",
    );
    for (file, want) in [
        ("fx_strobe_025.json", "blend with original"),
        ("fx_strobe_026.json", "strobe duration"),
        ("fx_strobe_027.json", "strobe operator"),
        ("fx_strobe_028.json", "strobe is"),
        ("fx_strobe_029.json", "strobe colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Strobe Light and its {want}"), &why, why.contains("Strobe Light") && why.contains(want));
    }
    let full = r##"{"strobe_color": "#ffffff", "blend_with_original": 0, "strobe_duration": 0.1, "strobe_period": 1, "random_strobe_probability": 0, "strobe": "color_only", "strobe_operator": "copy", "random_seed": 0}"##;
    t.shape_refused("fx_strobe_001.json", "a Strobe Light whose duration is a word", &full.replace("\"strobe_duration\": 0.1", "\"strobe_duration\": \"short\""));
    t.shape_refused("fx_strobe_001.json", "a Strobe Light without its random seed", &full.replace(", \"random_seed\": 0", ""));
    t.shape_refused("fx_strobe_001.json", "a Strobe Light whose colour is a number", &full.replace("\"#ffffff\"", "3"));

    t.heading("Commands");
    let mut document = t.load("fx_strobe_001.json").document;
    let over = changed(&t, "fx_strobe_001.json", |e| if let Effect::StrobeLight { blend_with_original, .. } = e { *blend_with_original = 101.0 });
    let op = changed(&t, "fx_strobe_001.json", |e| if let Effect::StrobeLight { strobe_operator, .. } = e { *strobe_operator = "screen".into() });
    let mode = changed(&t, "fx_strobe_001.json", |e| if let Effect::StrobeLight { strobe, .. } = e { *strobe = "flash".into() });
    let colour = changed(&t, "fx_strobe_001.json", |e| if let Effect::StrobeLight { strobe_color, .. } = e { *strobe_color = "white".into() });
    t.refused(
        &mut document,
        vec![
            ("blend with original 101", set(over)),
            ("strobe operator screen", set(op)),
            ("strobe flash", set(mode)),
            ("strobe colour white", set(colour)),
            ("random strobe probability keyed to 150", keys("random_strobe_probability", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let moved = changed(&t, "fx_strobe_001.json", |e| {
        if let Effect::StrobeLight { strobe_color, blend_with_original, strobe_period, strobe, strobe_operator, random_seed, .. } = e {
            *strobe_color = "#00ff80".into();
            *blend_with_original = 12.5;
            *strobe_period = 0.5;
            *strobe = "transparent".into();
            *strobe_operator = "xor".into();
            *random_seed = 42.0;
        }
    });
    t.taken(
        &mut document,
        "fx_strobe_001.json",
        vec![
            ("green, blend 12.5, every half second, Makes Layer Transparent, Xor, seed 42", set(moved)),
            ("blend with original keyed from 0 to 100", keys("blend_with_original", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_strobe_001.json", 0),
        ("fx_strobe_008.json", 0),
        ("fx_strobe_011.json", 0),
        ("fx_strobe_015.json", 0),
        ("fx_strobe_023.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Strobe Duration 0 (017) is never on, so the effect is left out; refused ones (025 to 030)
    // leave nothing for the card.
    let none: Vec<u32> = [17].into_iter().chain(25..=30).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Copy in purple, always on", json!({"strobe_color": "#8040c0", "strobe_period": 0.01}), 3),
            ("Difference in orange, Blend With Original 30, always on", json!({"strobe_color": "#ff8800", "strobe_operator": "difference", "blend_with_original": 30, "strobe_period": 0.01}), 3),
            ("Xor in teal, always on", json!({"strobe_color": "#33cc99", "strobe_operator": "xor", "strobe_period": 0.01}), 3),
            ("Makes Layer Transparent, Blend With Original 40, always on", json!({"strobe": "transparent", "blend_with_original": 40, "strobe_period": 0.01}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-466 pictures/`");
    let dir = repo("verification/D-466 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let all = w * h;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J, frame: i32| picture(&dir, "town.png", TOWN, TOWN, json!([fx("fx-0-0", &p)]), frame);
    let (before, said) = picture(&dir, "town.png", TOWN, TOWN, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street filling the frame; draws cleanly", &format!("{said:?}, covered {} of {all}", covered(&before)), said.is_empty() && covered(&before) == all);
    let every = |p: &[u8], f: &dyn Fn(&[u8], &[u8]) -> bool| (0..all).all(|i| f(&p[4 * i..4 * i + 4], &before[4 * i..4 * i + 4]));

    let (flash, said) = street(json!({}), 0);
    write("2_as_added_frame_0.png", &flash);
    t.row(
        "2_as_added_frame_0.png, Strobe Light as added at frame 0, inside the first strobe: the whole street white; draws cleanly",
        &format!("{said:?}, all white {}", every(&flash, &|p, _| p == [255, 255, 255, 255])),
        said.is_empty() && every(&flash, &|p, _| p == [255, 255, 255, 255]),
    );

    let (gap, said) = street(json!({}), 3);
    write("3_as_added_frame_3.png", &gap);
    t.row(
        "3_as_added_frame_3.png, as added at frame 3 (0.125 seconds, past the 0.1-second strobe): the street as it was; draws cleanly",
        &format!("{said:?}, the same as 1_before.png {}", gap == before),
        said.is_empty() && gap == before,
    );

    let (diff, said) = street(json!({"strobe_color": "#8040c0", "strobe_operator": "difference"}), 0);
    write("4_difference_purple.png", &diff);
    let same = (0..all).filter(|&i| diff[4 * i..4 * i + 3] == before[4 * i..4 * i + 3]).count();
    t.row(
        "4_difference_purple.png, Difference with purple: the street's colours turned over against purple, nearly every pixel changed, still opaque; draws cleanly",
        &format!("{said:?}, unchanged {same} of {all}, opaque {}", covered(&diff) == all && every(&diff, &|p, _| p[3] == 255)),
        said.is_empty() && same < all / 100 && every(&diff, &|p, _| p[3] == 255),
    );

    let (red, said) = street(json!({"strobe_color": "#ff0000", "blend_with_original": 50}), 0);
    write("5_red_blend_50.png", &red);
    let toward = every(&red, &|p, q| p[0] >= q[0] && p[1] <= q[1] && p[2] <= q[2] && p[3] == 255);
    t.row(
        "5_red_blend_50.png, Copy in red with Blend With Original 50: halfway to red, so every pixel at least as red and no more green or blue than the street; draws cleanly",
        &format!("{said:?}, toward red everywhere {toward}, not the street {}", red != before),
        said.is_empty() && toward && red != before,
    );

    let (faded, said) = street(json!({"strobe": "transparent", "blend_with_original": 25}), 0);
    write("6_transparent_blend_25.png", &faded);
    let quarter = every(&faded, &|p, q| (63..=65).contains(&p[3]) && (0..3).all(|c| p[c].abs_diff(q[c]) <= 1));
    t.row(
        "6_transparent_blend_25.png, Makes Layer Transparent with Blend With Original 25: the street at a quarter strength (alpha 64 of 255), its colours kept; draws cleanly",
        &format!("{said:?}, a quarter everywhere {quarter}"),
        said.is_empty() && quarter,
    );

    let on = |seed: i32| -> Vec<i32> {
        (0..5)
            .filter(|&f| street(json!({"strobe_duration": 0, "strobe_period": 100, "random_strobe_probability": 50, "random_seed": seed}), f).0 != before)
            .collect()
    };
    let (three, four) = (on(3), on(4));
    t.row(
        "Random Strobe Probability 50 on the street, frames 0 to 4: seed 3 strobes at 1 and 4, seed 4 at 0, 1 and 2, as FX-STROBE-018 and 019 say",
        &format!("seed 3 {three:?}, seed 4 {four:?}"),
        three == [1, 4] && four == [0, 1, 2],
    );

    t.finish("D-466_strobe_light_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B346_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-346: a measurement, run deliberately with --release --ignored"]
fn b346_strobe_light_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B346_CPU").is_ok();
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
        ("Noise, then Strobe Light always on (Copy, Strobe Period 0.01)", Some(json!({"strobe_period": 0.01}))),
        ("the same, Xor with Blend With Original 30", Some(json!({"strobe_period": 0.01, "strobe_operator": "xor", "blend_with_original": 30}))),
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
    let out = std::env::var("B346_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-346_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
