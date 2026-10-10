//! B-279: D-400, after After Effects' Shadow/Highlight: the shadows lifted and the highlights
//! brought down by each pixel's surroundings, by darktable's shadows and highlights rule.
//!
//! Writes `verification/D-400_shadow_highlight_table.md` and draws pictures into
//! `verification/D-400 pictures/`.
//!
//! Every expected pixel is `Fixtures/shadow_highlight/expected_shadow_highlight.json`, written by
//! `tools/shadow_highlight_reference.py` before this code existed and printed in document 25 as
//! FX-SHHI-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Shadow/Highlights the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ShadowHighlight { .. })))
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

/// A Shadow/Highlight with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({
        "shadow_amount": 50, "highlight_amount": 0, "shadow_tonal_width": 50, "shadow_radius": 30,
        "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20
    });
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.shadow_highlight", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=21 {
        let file = format!("fx_shhi_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b279-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Shadow/Highlight {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Shadow/Highlight {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
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

/// Covered pixels of `before` and `after` side by side.
fn pairs<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (&'a [u8], &'a [u8])> {
    before.chunks_exact(4).zip(after.chunks_exact(4)).filter(|(b, _)| b[3] > 0)
}

#[test]
fn b279_shadow_highlight() {
    let mut t = Table::new(
        "shadow_highlight",
        "# D-400: Shadow/Highlight\n\nB-279, after After Effects' Shadow/Highlight: the shadows \
         lifted and the highlights brought down by each pixel's surroundings, by darktable's \
         shadows and highlights rule (src/iop/shadhi.c, Gaussian softening): the lightness \
         blurred and inverted, laid on each pixel's own lightness by an overlay, gated by the \
         tonal width, the colour scaled to follow. Every expected pixel is \
         `Fixtures/shadow_highlight/expected_shadow_highlight.json`, written by \
         `tools/shadow_highlight_reference.py` before this code existed and printed in document \
         25 as FX-SHHI-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-SHHI-001 to 021 (document 25)");
    t.fixtures_numbered("expected_shadow_highlight.json", 1..=21);

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_shhi_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_shhi_011.json");
    t.row(
        "fx_shhi_011.json is saved with its seven settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7
            && saved["shadow_amount"] == 70.0
            && saved["shadow_radius"] == 2.0
            && saved["highlight_amount"] == 60.0
            && saved["highlight_radius"] == 6.0
            && saved["color_correction"] == 20.0,
    );
    for (file, want) in [
        ("fx_shhi_017.json", "shadow_amount"),
        ("fx_shhi_018.json", "highlight_amount"),
        ("fx_shhi_019.json", "shadow_tonal_width"),
        ("fx_shhi_020.json", "highlight_radius"),
        ("fx_shhi_021.json", "color_correction"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let want = want.replace('_', " ");
        t.row(&format!("{file} is refused in a sentence naming its {want}"), &why, why.contains(&want));
    }
    let rest = r#""highlight_amount": 0, "shadow_tonal_width": 50, "shadow_radius": 30, "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20"#;
    t.shape_refused("fx_shhi_001.json", "a Shadow/Highlight whose shadow amount is a word", &format!(r#"{{"shadow_amount": "fifty", {rest}}}"#));
    t.shape_refused("fx_shhi_001.json", "a Shadow/Highlight without its shadow amount", &format!("{{{rest}}}"));

    t.heading("Commands");
    let mut document = t.load("fx_shhi_001.json").document;
    let over = changed(&t, "fx_shhi_001.json", |e| if let Effect::ShadowHighlight { shadow_amount, .. } = e { *shadow_amount = 101.0 });
    let wide = changed(&t, "fx_shhi_001.json", |e| if let Effect::ShadowHighlight { highlight_radius, .. } = e { *highlight_radius = 501.0 });
    t.refused(
        &mut document,
        vec![
            ("shadow amount 101", set(over)),
            ("highlight radius 501", set(wide)),
            ("shadow amount keyed to 150", keys("shadow_amount", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let lights = changed(&t, "fx_shhi_001.json", |e| {
        if let Effect::ShadowHighlight { highlight_amount, highlight_radius, .. } = e {
            *highlight_amount = 70.0;
            *highlight_radius = 4.0;
        }
    });
    t.taken(
        &mut document,
        "fx_shhi_001.json",
        vec![
            ("highlight 70 at radius 4", set(lights)),
            ("shadow amount keyed from 0 to 100", keys("shadow_amount", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shhi_001.json", 0),
        ("fx_shhi_006.json", 0),
        ("fx_shhi_011.json", 0),
        ("fx_shhi_014.json", 0),
        ("fx_shhi_015.json", 2),
        ("fx_shhi_016.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Both amounts 0 (002) and refused (017 to 021) leave nothing for the card.
    let none: Vec<u32> = std::iter::once(2).chain(17..=21).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("shadow 80, highlight 60, radii 12 and 40", json!({"shadow_amount": 80, "highlight_amount": 60, "shadow_radius": 12, "highlight_radius": 40}), 3),
            ("shadow 100, widths 100, colour correction 100", json!({"shadow_amount": 100, "shadow_tonal_width": 100, "highlight_tonal_width": 100, "color_correction": 100}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-400 pictures/`");
    let dir = repo("verification/D-400 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({"shadow_amount": 0}));
    t.row(
        "both amounts 0 changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;

    // As added: the shadows lift; nothing darkens by more than a level.
    let (after, said) = street(json!({}));
    write("2_as_added.png", TOWN, &after);
    let darker = pairs(&before, &after).filter(|(b, a)| luma(a) < luma(b) - 1.0).count();
    let lifted = pairs(&before, &after).filter(|(b, a)| luma(a) > luma(b) + 1.0).count();
    t.row(
        "2_as_added.png, as added (shadow 50, radius 30): the dark doorways and shade lifted; no pixel more than a level darker; draws cleanly",
        &format!("{said:?}, {lifted} pixels lifted, {darker} darker"),
        said.is_empty() && lifted > 0 && darker == 0,
    );

    let (after, said) = street(json!({"shadow_amount": 100}));
    write("3_shadow_100.png", TOWN, &after);
    t.row(
        "3_shadow_100.png, shadow 100: lifted further; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&after, &before).1),
        said.is_empty() && after != before,
    );

    // Highlights only, small radius: the lights come down; nothing brightens by more than a level.
    let (after, said) = street(json!({"shadow_amount": 0, "highlight_amount": 80, "highlight_radius": 4}));
    write("4_highlight_80.png", TOWN, &after);
    let brighter = pairs(&before, &after).filter(|(b, a)| luma(a) > luma(b) + 1.0).count();
    let lowered = pairs(&before, &after).filter(|(b, a)| luma(a) < luma(b) - 1.0).count();
    t.row(
        "4_highlight_80.png, shadow 0, highlight 80 at radius 4: the sky and lit walls brought down; no pixel more than a level brighter; draws cleanly",
        &format!("{said:?}, {lowered} pixels lowered, {brighter} brighter"),
        said.is_empty() && lowered > 0 && brighter == 0,
    );

    let (after, said) = street(json!({"shadow_amount": 70, "highlight_amount": 60, "highlight_radius": 10}));
    write("5_both.png", TOWN, &after);
    t.row(
        "5_both.png, shadow 70 and highlight 60 at radius 10: the street's range drawn in from both ends; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&after, &before).1),
        said.is_empty() && after != before,
    );

    let (grey, said0) = street(json!({"shadow_amount": 100, "color_correction": 0}));
    let (rich, said1) = street(json!({"shadow_amount": 100, "color_correction": 100}));
    write("6_colour_0.png", TOWN, &grey);
    write("7_colour_100.png", TOWN, &rich);
    let sat = |p: &[u8]| p[..3].iter().max().unwrap() - p[..3].iter().min().unwrap();
    let total = |v: &[u8]| v.chunks_exact(4).filter(|p| p[3] > 0).map(|p| sat(p) as u64).sum::<u64>();
    t.row(
        "6_colour_0.png and 7_colour_100.png, shadow 100 with colour correction 0 and 100: the lifted shadows paler at 0, more coloured at 100; draw cleanly",
        &format!("{said0:?} {said1:?}, total saturation {} and {}", total(&grey), total(&rich)),
        said0.is_empty() && said1.is_empty() && total(&grey) < total(&rich),
    );

    t.finish("D-400_shadow_highlight_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B279_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-279: a measurement, run deliberately with --release --ignored"]
fn b279_shadow_highlight_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B279_CPU").is_ok();
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
        ("Noise, then Shadow/Highlight as added (shadow 50, radius 30)", Some(json!({}))),
        ("Noise, then Shadow/Highlight, shadow 80, highlight 60, radii 12 and 40", Some(json!({"shadow_amount": 80, "highlight_amount": 60, "shadow_radius": 12, "highlight_radius": 40}))),
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
    let out = std::env::var("B279_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-279_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
