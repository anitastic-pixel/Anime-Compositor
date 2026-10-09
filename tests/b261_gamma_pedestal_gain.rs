//! B-261: D-382, after After Effects' Gamma/Pedestal/Gain: a black stretch that lifts the dark
//! values of every channel, then for red, green and blue apart a gamma for the middle, a pedestal
//! for the lowest value and a gain for the highest.
//!
//! Writes `verification/D-382_gamma_pedestal_gain_table.md` and draws pictures into
//! `verification/D-382 pictures/`.
//!
//! Every expected pixel is `Fixtures/gamma_pedestal_gain/expected_gamma_pedestal_gain.json`,
//! written by `tools/gamma_pedestal_gain_reference.py` before this code existed and printed in
//! document 25 as FX-GPG-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

fn is_gpg(e: &Effect) -> bool {
    matches!(e, Effect::GammaPedestalGain { .. })
}

/// The Gamma/Pedestal/Gains the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_gpg(&f.instance.effect)))
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

/// A Gamma/Pedestal/Gain with the settings `p`, every one `p` leaves out at its start.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"black_stretch": 1});
    for c in ["red", "green", "blue"] {
        all[format!("{c}_gamma")] = json!(1);
        all[format!("{c}_pedestal")] = json!(0);
        all[format!("{c}_gain")] = json!(1);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.gamma_pedestal_gain", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=21 {
        let file = format!("fx_gpg_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b261-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Gamma/Pedestal/Gain {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Gamma/Pedestal/Gain {what} on three layers, frame {frame}, {}", quality.label()),
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
fn b261_gamma_pedestal_gain() {
    let mut t = Table::new(
        "gamma_pedestal_gain",
        "# D-382: Gamma/Pedestal/Gain\n\nB-261, after After Effects' Gamma/Pedestal/Gain: a black \
         stretch that lifts the dark values of every channel, then for red, green and blue apart a \
         gamma for the middle, a pedestal for the lowest value and a gain for the highest. Every \
         expected pixel is `Fixtures/gamma_pedestal_gain/expected_gamma_pedestal_gain.json`, \
         written by `tools/gamma_pedestal_gain_reference.py` before this code existed and printed \
         in document 25 as FX-GPG-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-GPG-001 to 021 (document 25)");
    t.fixtures_numbered("expected_gamma_pedestal_gain.json", 1..=21);

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_gpg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_gpg_012.json");
    t.row(
        "fx_gpg_012.json is saved with its ten settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 10 && saved["black_stretch"] == 1.5 && saved["green_pedestal"] == 0.1 && saved["blue_gamma"] == 1.5,
    );
    for (file, want) in [
        ("fx_gpg_015.json", "black stretch"),
        ("fx_gpg_016.json", "black stretch"),
        ("fx_gpg_017.json", "green gamma"),
        ("fx_gpg_018.json", "blue gamma"),
        ("fx_gpg_019.json", "red pedestal"),
        ("fx_gpg_020.json", "green gain"),
        ("fx_gpg_021.json", "blue gain"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_gpg_001.json",
        "a Gamma/Pedestal/Gain whose red gain is a word",
        r#"{"black_stretch": 1, "red_gamma": 1, "red_pedestal": 0, "red_gain": "1",
            "green_gamma": 1, "green_pedestal": 0, "green_gain": 1, "blue_gamma": 1, "blue_pedestal": 0, "blue_gain": 1}"#,
    );
    t.shape_refused(
        "fx_gpg_001.json",
        "a Gamma/Pedestal/Gain without its blue pedestal",
        r#"{"black_stretch": 1, "red_gamma": 1, "red_pedestal": 0, "red_gain": 1,
            "green_gamma": 1, "green_pedestal": 0, "green_gain": 1, "blue_gamma": 1, "blue_gain": 1}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_gpg_001.json").document;
    let gamma = changed(&t, "fx_gpg_001.json", |e| if let Effect::GammaPedestalGain { gamma, .. } = e { gamma[0] = 0.05 });
    let gain = changed(&t, "fx_gpg_001.json", |e| if let Effect::GammaPedestalGain { gain, .. } = e { gain[2] = 4.5 });
    t.refused(
        &mut document,
        vec![
            ("red gamma 0.05", set(gamma)),
            ("blue gain 4.5", set(gain)),
            ("black stretch keyed to 5", keys("black_stretch", &[(0, &[1.0]), (4, &[5.0])])),
        ],
    );
    let curve = changed(&t, "fx_gpg_001.json", |e| {
        if let Effect::GammaPedestalGain { black_stretch, pedestal, .. } = e {
            *black_stretch = 2.0;
            *pedestal = [1.0, 1.0, 1.0];
        }
    });
    t.taken(
        &mut document,
        "fx_gpg_001.json",
        vec![("black stretch 2, every pedestal 1", set(curve)), ("blue gain keyed from 1 to 0", keys("blue_gain", &[(0, &[1.0]), (4, &[0.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_gpg_003.json", 0), ("fx_gpg_009.json", 0), ("fx_gpg_012.json", 0), ("fx_gpg_013.json", 2), ("fx_gpg_014.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Untouched (001) and refused (015 to 021) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(1).chain(15..=21).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("black stretch 2, gammas 1.4, 1, 0.7", json!({"black_stretch": 2, "red_gamma": 1.4, "blue_gamma": 0.7}), 3),
            ("pedestals 0.1, gains 0.9, 1.1, 1.3", json!({"red_pedestal": 0.1, "green_pedestal": 0.1, "blue_pedestal": 0.1, "red_gain": 0.9, "green_gain": 1.1, "blue_gain": 1.3}), 3),
            ("every pedestal 1 and gain 0, the negative", json!({"red_pedestal": 1, "green_pedestal": 1, "blue_pedestal": 1, "red_gain": 0, "green_gain": 0, "blue_gain": 0}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-382 pictures/`");
    let dir = repo("verification/D-382 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "the effect as it is added changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let (lifted, said) = picture(&dir, one(json!({"black_stretch": 3})));
    write("2_black_stretch_3.png", &lifted);
    let lighter = before.chunks_exact(4).zip(lifted.chunks_exact(4)).all(|(b, l)| (0..3).all(|c| l[c] >= b[c]));
    t.row(
        "2_black_stretch_3.png, black stretch 3: the shadows opened up, no pixel darker; draws cleanly",
        &format!("{said:?}, no channel darker: {lighter}"),
        said.is_empty() && lighter && lifted != before,
    );
    let (warm, said) = picture(&dir, one(json!({"red_gain": 1.2, "red_gamma": 1.2, "blue_gain": 0.8, "blue_pedestal": 0.05})));
    write("3_warm.png", &warm);
    t.row(
        "3_warm.png, red gamma and gain 1.2, blue gain 0.8 and pedestal 0.05: the street warmer, its blacks a little blue; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&warm, &before).1),
        said.is_empty() && warm != before,
    );
    let (neg, said) = picture(&dir, one(json!({"red_pedestal": 1, "green_pedestal": 1, "blue_pedestal": 1, "red_gain": 0, "green_gain": 0, "blue_gain": 0})));
    write("4_negative.png", &neg);
    let turned = before.chunks_exact(4).zip(neg.chunks_exact(4)).all(|(b, n)| (0..3).all(|c| b[c].abs_diff(255 - n[c]) <= 1));
    t.row(
        "4_negative.png, every pedestal 1 and gain 0: the street's negative, each channel 255 less itself within 1 level; draws cleanly",
        &format!("{said:?}, every channel turned over: {turned}"),
        said.is_empty() && turned,
    );

    t.finish("D-382_gamma_pedestal_gain_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B261_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-261: a measurement, run deliberately with --release --ignored"]
fn b261_gamma_pedestal_gain_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B261_CPU").is_ok();
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
        ("Noise, then Gamma/Pedestal/Gain, black stretch 1.5 and a gain", Some(json!({"black_stretch": 1.5, "red_gain": 1.2}))),
        ("Noise, then Gamma/Pedestal/Gain, every channel its own curve", Some(json!({"black_stretch": 1.5, "red_gain": 1.2, "green_pedestal": 0.1, "blue_gamma": 1.5}))),
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
    let out = std::env::var("B261_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-261_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
