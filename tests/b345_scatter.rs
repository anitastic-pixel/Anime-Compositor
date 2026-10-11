//! B-345: D-465, After Effects' Scatter under `core.scatter`: Scatter Amount, Grain and
//! Randomize Every Frame.
//!
//! Writes `verification/D-465_scatter_table.md` and draws pictures into
//! `verification/D-465 pictures/`.
//!
//! Every expected pixel is `Fixtures/scatter/expected_scatter.json`, written by
//! `tools/scatter_reference.py` before this code existed and printed in document 25 as
//! FX-SCATTER-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Scatters the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Scatter { .. })))
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

/// A Scatter with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"scatter_amount": 10, "grain": "both", "randomize": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.scatter", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=19 {
        let file = format!("fx_scatter_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b345-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Scatter {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Scatter {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
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
fn b345_scatter() {
    let mut t = Table::new(
        "scatter",
        "# D-465: Scatter\n\nB-345: `core.scatter` takes After Effects' Scatter controls (Scatter \
         Amount, Grain, Randomize Every Frame). Every expected pixel is \
         `Fixtures/scatter/expected_scatter.json`, written by `tools/scatter_reference.py` before \
         this code existed and printed in document 25 as FX-SCATTER-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-SCATTER-001 to 019 (document 25)");
    t.fixtures_numbered("expected_scatter.json", 1..=19);

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_scatter_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_scatter_009.json");
    t.row(
        "fx_scatter_009.json is saved with its three settings, and not the frame",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3 && saved["scatter_amount"] == 2.0 && saved["grain"] == "both" && saved["randomize"] == "on",
    );
    for (file, want) in [
        ("fx_scatter_015.json", "scatter amount"),
        ("fx_scatter_016.json", "scatter amount"),
        ("fx_scatter_017.json", "grain"),
        ("fx_scatter_018.json", "randomize every frame"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Scatter and its {want}"), &why, why.contains("Scatter") && why.contains(want));
    }
    let full = r#"{"scatter_amount": 10, "grain": "both", "randomize": "off"}"#;
    t.shape_refused("fx_scatter_001.json", "a Scatter whose amount is a word", &full.replace("\"scatter_amount\": 10", "\"scatter_amount\": \"lots\""));
    t.shape_refused("fx_scatter_001.json", "a Scatter without its randomize", &full.replace(", \"randomize\": \"off\"", ""));
    t.shape_refused("fx_scatter_001.json", "a Scatter whose grain is a number", &full.replace("\"both\"", "3"));

    t.heading("Commands");
    let mut document = t.load("fx_scatter_001.json").document;
    let over = changed(&t, "fx_scatter_001.json", |e| if let Effect::Scatter { scatter_amount, .. } = e { *scatter_amount = 128.0 });
    let word = changed(&t, "fx_scatter_001.json", |e| if let Effect::Scatter { grain, .. } = e { *grain = "diagonal".into() });
    let odd = changed(&t, "fx_scatter_001.json", |e| if let Effect::Scatter { randomize, .. } = e { *randomize = "sometimes".into() });
    t.refused(
        &mut document,
        vec![
            ("scatter amount 128", set(over)),
            ("grain diagonal", set(word)),
            ("randomize sometimes", set(odd)),
            ("scatter amount keyed to 200", keys("scatter_amount", &[(0, &[0.0]), (4, &[200.0])])),
        ],
    );
    let moved = changed(&t, "fx_scatter_001.json", |e| {
        if let Effect::Scatter { scatter_amount, grain, randomize, .. } = e {
            *scatter_amount = 7.5;
            *grain = "vertical".into();
            *randomize = "on".into();
        }
    });
    t.taken(
        &mut document,
        "fx_scatter_001.json",
        vec![
            ("amount 7.5, vertical, a new jumble each frame", set(moved)),
            ("scatter amount keyed from 0 to 127", keys("scatter_amount", &[(0, &[0.0]), (4, &[127.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_scatter_001.json", 0),
        ("fx_scatter_003.json", 0),
        ("fx_scatter_004.json", 0),
        ("fx_scatter_009.json", 2),
        ("fx_scatter_014.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // No reach (006, 007) leaves the effect out; refused ones (015 to 019) leave nothing for the
    // card.
    let none: Vec<u32> = [6, 7].into_iter().chain(15..=19).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added, amount 10", json!({}), 3),
            ("amount 40, Horizontal", json!({"scatter_amount": 40, "grain": "horizontal"}), 3),
            ("amount 127, Vertical", json!({"scatter_amount": 127, "grain": "vertical"}), 3),
            ("amount 25, Randomize Every Frame on, frame 3", json!({"scatter_amount": 25, "randomize": "on"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-465 pictures/`");
    let dir = repo("verification/D-465 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let all = w * h;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J, frame: i32| picture(&dir, "town.png", TOWN, TOWN, json!([fx("fx-0-0", &p)]), frame);
    let (before, said) = picture(&dir, "town.png", TOWN, TOWN, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street filling the frame; draws cleanly", &format!("{said:?}, covered {} of {all}", covered(&before)), said.is_empty() && covered(&before) == all);
    // Every pixel of `p` is the street's own from within `n` across and `m` down, or clear.
    let from_near = |p: &[u8], n: i64, m: i64| {
        (0..h).all(|y| {
            (0..w).all(|x| {
                let q = at(p, w, x, y);
                q[3] == 0
                    || (-m..=m).any(|dy| {
                        (-n..=n).any(|dx| {
                            let (u, v) = (x as i64 + dx, y as i64 + dy);
                            (0..w as i64).contains(&u) && (0..h as i64).contains(&v) && at(&before, w, u as usize, v as usize) == q
                        })
                    })
            })
        })
    };

    let (added, said) = street(json!({}), 0);
    write("2_as_added.png", &added);
    let same = (0..all).filter(|&i| added[4 * i..4 * i + 4] == before[4 * i..4 * i + 4]).count();
    t.row(
        "2_as_added.png, Scatter as added (amount 10): the street jumbled, every pixel the street's own from within 10 pixels; draws cleanly",
        &format!("{said:?}, unmoved {same} of {all}, from within 10 {}", from_near(&added, 10, 10)),
        said.is_empty() && same < all / 2 && from_near(&added, 10, 10),
    );

    let (far, said) = street(json!({"scatter_amount": 40}), 0);
    write("3_amount_40.png", &far);
    t.row(
        "3_amount_40.png, amount 40: jumbled further, not 2_as_added.png, the frame's edge frayed (some clear pixels along it, about 11 in 100 of the frame for a reach of 40 on 480 by 270, so more than 85 in 100 covered); draws cleanly",
        &format!("{said:?}, different {}, covered {} of {all}", far != added, covered(&far)),
        said.is_empty() && far != added && covered(&far) < all && covered(&far) > all * 85 / 100,
    );

    let (across, said) = street(json!({"scatter_amount": 40, "grain": "horizontal"}), 0);
    write("4_horizontal_40.png", &across);
    t.row(
        "4_horizontal_40.png, amount 40, Grain Horizontal: every pixel from its own row, within 40 across; draws cleanly",
        &format!("{said:?}, from its row {}", from_near(&across, 40, 0)),
        said.is_empty() && from_near(&across, 40, 0) && across != far,
    );

    let (down, said) = street(json!({"scatter_amount": 40, "grain": "vertical"}), 0);
    write("5_vertical_40.png", &down);
    t.row(
        "5_vertical_40.png, amount 40, Grain Vertical: every pixel from its own column, within 40 down; draws cleanly",
        &format!("{said:?}, from its column {}", from_near(&down, 0, 40)),
        said.is_empty() && from_near(&down, 0, 40) && down != far && down != across,
    );

    let (still0, _) = street(json!({"scatter_amount": 10}), 0);
    let (still3, said) = street(json!({"scatter_amount": 10}), 3);
    let (live0, _) = street(json!({"scatter_amount": 10, "randomize": "on"}), 0);
    let (live3, said2) = street(json!({"scatter_amount": 10, "randomize": "on"}), 3);
    write("6_randomize_frame_3.png", &live3);
    t.row(
        "6_randomize_frame_3.png: with Randomize Every Frame off, frame 3 is frame 0; on, frame 3 is a new jumble and frame 0 is 2_as_added.png; draw cleanly",
        &format!("{said:?} {said2:?}, off the same {}, on different {}, on frame 0 as added {}", still0 == still3, live0 != live3, live0 == added),
        said.is_empty() && said2.is_empty() && still0 == still3 && live0 != live3 && live0 == added,
    );

    t.finish("D-465_scatter_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B345_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-345: a measurement, run deliberately with --release --ignored"]
fn b345_scatter_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B345_CPU").is_ok();
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
        ("Noise, then Scatter as added (amount 10)", Some(json!({}))),
        ("the same, amount 127, Randomize Every Frame on", Some(json!({"scatter_amount": 127, "randomize": "on"}))),
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
    let out = std::env::var("B345_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-345_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
