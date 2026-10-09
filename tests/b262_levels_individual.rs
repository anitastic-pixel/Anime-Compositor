//! B-262: D-383, the owner's "Both names, one engine": Levels with a set of input black, input
//! white, gamma, output black and output white for RGB, red, green, blue and alpha, its Channel
//! menu choosing which set its controls show, and Levels (Individual Controls) over the same rule
//! showing all 25 at once. Each channel's own set first, then the RGB set; the alpha set on the
//! covering with the colour kept.
//!
//! Writes `verification/D-383_levels_individual_table.md` and draws pictures into
//! `verification/D-383 pictures/`.
//!
//! Every expected pixel is `Fixtures/levels_individual/expected_levels_individual.json`, written
//! by `tools/levels_individual_reference.py` before this code existed and printed in document 25
//! as FX-LVLIC-001 to 020. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, LEVELS_NAMES};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

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

/// The Levels the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ChannelLevels { .. } | Effect::Levels { .. })))
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

/// A Levels (Individual Controls) with the settings `p`, every one `p` leaves out at its start.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({});
    for names in LEVELS_NAMES {
        for (n, start) in names.iter().zip([0.0, 255.0, 1.0, 0.0, 255.0]) {
            all[*n] = json!(start);
        }
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.levels_individual", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=20 {
        let file = format!("fx_lvlic_{n:03}.json");
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b262-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Levels {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Levels {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

#[test]
fn b262_levels_individual() {
    let mut t = Table::new(
        "levels_individual",
        "# D-383: Levels with a set for each channel, and Levels (Individual Controls)\n\nB-262, the \
         owner's \"Both names, one engine\": a set of input black, input white, gamma, output black \
         and output white for RGB, red, green, blue and alpha; Levels' Channel menu chooses the set \
         its controls show, Levels (Individual Controls) shows all 25. Each channel's own set first, \
         then the RGB set; the alpha set on the covering with the colour kept. Every expected pixel \
         is `Fixtures/levels_individual/expected_levels_individual.json`, written by \
         `tools/levels_individual_reference.py` before this code existed and printed in document 25 \
         as FX-LVLIC-001 to 020. A Levels saved before D-383, with its five settings, opens as the \
         RGB set and draws as it did: `tests/b55_levels.rs` and its fixtures are unchanged.\n",
    );

    t.heading("FX-LVLIC-001 to 020 (document 25)");
    t.fixtures_numbered("expected_levels_individual.json", 1..=20);

    t.heading("The file");
    let files: Vec<String> = (1..=20).filter(|n| *n != 12).map(|n| format!("fx_lvlic_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_lvlic_012.json");
    t.row(
        "fx_lvlic_012.json, a Levels with its Channel menu and only the RGB five, is saved with its channel and all 25 settings, the ones it left out at their starts",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 26 && saved["channel"] == "alpha" && saved["input_white"] == 200 && saved["alpha_output_white"] == 255,
    );
    let saved = t.saved_parameters("fx_lvlic_014.json");
    t.row(
        "fx_lvlic_014.json, Levels (Individual Controls), is saved with its 25 settings and no channel",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 25 && saved.get("channel").is_none() && saved["red_input_white"] == 230 && saved["alpha_output_white"] == 200,
    );
    for (file, want) in [
        ("fx_lvlic_016.json", "red input black"),
        ("fx_lvlic_017.json", "green gamma"),
        ("fx_lvlic_018.json", "blue output white"),
        ("fx_lvlic_019.json", "alpha gamma"),
        ("fx_lvlic_020.json", "\"luma\""),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let rgb = r#""input_black": 0, "input_white": 255, "gamma": 1, "output_black": 0, "output_white": 255"#;
    t.shape_refused("fx_lvlic_001.json", "a Levels (Individual Controls) without its red gamma", &format!("{{{rgb}}}"));
    t.shape_refused("fx_lvlic_011.json", "a Levels whose red input white is a word", &format!("{{\"channel\": \"red\", {rgb}, \"red_input_white\": \"192\"}}"));
    t.shape_refused("fx_lvlic_011.json", "a Levels whose channel is a number", &format!("{{\"channel\": 1, {rgb}}}"));

    t.heading("Commands");
    let mut document = t.load("fx_lvlic_001.json").document;
    let black = changed(&t, "fx_lvlic_001.json", |e| if let Effect::ChannelLevels { sets, .. } = e { sets[1][0] = 300.0 });
    let gamma = changed(&t, "fx_lvlic_001.json", |e| if let Effect::ChannelLevels { sets, .. } = e { sets[4][2] = 0.0 });
    t.refused(
        &mut document,
        vec![
            ("red input black 300", set(black)),
            ("alpha gamma 0", set(gamma)),
            ("green output white keyed to 400", keys("green_output_white", &[(0, &[255.0]), (4, &[400.0])])),
        ],
    );
    let blue = changed(&t, "fx_lvlic_001.json", |e| {
        if let Effect::ChannelLevels { sets, .. } = e {
            sets[3] = [20.0, 230.0, 1.5, 10.0, 250.0];
        }
    });
    t.taken(
        &mut document,
        "fx_lvlic_001.json",
        vec![("the blue set 20, 230, 1.5, 10, 250", set(blue)), ("alpha output white keyed from 255 to 0", keys("alpha_output_white", &[(0, &[255.0]), (4, &[0.0])]))],
    );
    let mut levels = t.load("fx_lvlic_011.json").document;
    let luma = changed(&t, "fx_lvlic_011.json", |e| if let Effect::ChannelLevels { channel, .. } = e { *channel = "luma".into() });
    t.refused(&mut levels, vec![("Levels' channel \"luma\"", set(luma))]);
    let green = changed(&t, "fx_lvlic_011.json", |e| if let Effect::ChannelLevels { channel, .. } = e { *channel = "green".into() });
    t.taken(&mut levels, "fx_lvlic_011.json", vec![("Levels' Channel menu on Green", set(green))]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lvlic_005.json", 0), ("fx_lvlic_008.json", 0), ("fx_lvlic_010.json", 0), ("fx_lvlic_013.json", 2), ("fx_lvlic_014.json", 0), ("fx_lvlic_015.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Untouched (001), an alpha set (007 to 009, 014, 015), a threshold (010) and refused (016
    // to 020) leave nothing for the card.
    let none: Vec<u32> = [1, 7, 8, 9, 10, 14, 15].into_iter().chain(16..=20).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("RGB input 16 to 235 and gamma 1.1", json!({"input_black": 16, "input_white": 235, "gamma": 1.1}), 3),
            (
                "red input white 220, green gamma 1.3, blue output 20 to 235, then RGB gamma 0.9",
                json!({"red_input_white": 220, "green_gamma": 1.3, "blue_output_black": 20, "blue_output_white": 235, "gamma": 0.9}),
                3,
            ),
            ("red, green and blue each turned over", json!({"red_output_black": 255, "red_output_white": 0, "green_output_black": 255, "green_output_white": 0, "blue_output_black": 255, "blue_output_white": 0}), 3),
            ("alpha output white 200, on the processor", json!({"alpha_output_white": 200}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-383 pictures/`");
    let dir = repo("verification/D-383 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "Levels (Individual Controls) as it is added changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let old = json!({"input_black": 20, "input_white": 230, "gamma": 1.2, "output_black": 10, "output_white": 245});
    let (five, said) = picture(&dir, json!([{"instance_id": "fx-0-0", "type_id": "core.levels", "enabled": true, "parameters": old}]));
    write("2_levels_as_before.png", &five);
    let (as_set, _) = picture(&dir, one(old.clone()));
    t.row(
        "2_levels_as_before.png, a Levels saved before D-383 with its five settings: the same street, byte for byte, as Levels (Individual Controls) with those five as its RGB set; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&five, &as_set).1),
        said.is_empty() && five == as_set && five != before,
    );
    let (warm, said) = picture(&dir, one(json!({"red_input_white": 220, "blue_output_white": 210})));
    write("3_warm.png", &warm);
    let warmer = before.chunks_exact(4).zip(warm.chunks_exact(4)).all(|(b, w)| w[0] >= b[0] && w[1] == b[1] && w[2] <= b[2]);
    t.row(
        "3_warm.png, red input white 220 and blue output white 210: the street warmer, no red darker, green untouched, no blue lighter; draws cleanly",
        &format!("{said:?}, warmer everywhere: {warmer}"),
        said.is_empty() && warmer && warm != before,
    );
    let channel = json!({"channel": "blue", "input_black": 0, "input_white": 255, "gamma": 1, "output_black": 0, "output_white": 255, "blue_gamma": 1.6});
    let (menu, said) = picture(&dir, json!([{"instance_id": "fx-0-0", "type_id": "core.levels", "enabled": true, "parameters": channel}]));
    write("4_channel_blue_gamma.png", &menu);
    let (individual, _) = picture(&dir, one(json!({"blue_gamma": 1.6})));
    t.row(
        "4_channel_blue_gamma.png, Levels with its Channel menu on Blue and blue gamma 1.6: the street bluer in its middles, byte for byte Levels (Individual Controls) with blue gamma 1.6; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&menu, &individual).1),
        said.is_empty() && menu == individual && menu != before,
    );
    let (faint, said) = picture(&dir, one(json!({"alpha_output_white": 128})));
    write("5_alpha_half.png", &faint);
    let half = faint.chunks_exact(4).all(|p| p[3].abs_diff(128) <= 1);
    let kept = before.chunks_exact(4).zip(faint.chunks_exact(4)).all(|(b, f)| (0..3).all(|c| b[c].abs_diff(f[c]) <= 1));
    t.row(
        "5_alpha_half.png, alpha output white 128: the street half covering everywhere, its colours kept within 1 level; draws cleanly",
        &format!("{said:?}, every covering 128: {half}, colours kept: {kept}"),
        said.is_empty() && half && kept,
    );

    t.finish("D-383_levels_individual_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B262_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-262: a measurement, run deliberately with --release --ignored"]
fn b262_levels_individual_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B262_CPU").is_ok();
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
        // A Levels as saved before D-383, its five settings and no channel: the shot timed before.
        ("Noise, then Levels as saved before, five settings", Some(json!({"type_id": "core.levels",
            "parameters": {"input_black": 16, "input_white": 235, "gamma": 1.1, "output_black": 0, "output_white": 255}}))),
        ("Noise, then Levels (Individual Controls), RGB, red, green and blue sets", Some(json!({"input_black": 16, "gamma": 1.1, "red_input_white": 220, "green_gamma": 1.3, "blue_output_black": 20}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            match &e {
                Some(p) if p.get("type_id").is_some() => v.push(json!({"instance_id": format!("{id}c"), "type_id": p["type_id"], "enabled": true, "parameters": p["parameters"]})),
                Some(p) => v.push(fx(&format!("{id}c"), p)),
                None => {}
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
    let out = std::env::var("B262_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-262_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
