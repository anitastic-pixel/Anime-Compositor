//! B-443: D-563, Shadow/Highlight's Midtone Contrast and Black and White Clip, after After
//! Effects' Shadow/Highlight: after D-400's lift, Color Grade's contrast curve on the lightness,
//! then the lightness stretched by the layer's own count, the chroma following each change.
//!
//! Writes `verification/D-563_shadow_highlight_clip_table.md` and draws pictures into
//! `verification/D-563 pictures/`.
//!
//! Every expected pixel is `Fixtures/shadow_highlight_clip/expected_shadow_highlight_clip.json`,
//! written by `tools/shadow_highlight_clip_reference.py` before this code existed and printed in
//! document 25 as FX-SHCLIP-001 to 021. Tolerance 2e-5. Nothing here is a snapshot of a run.

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
        "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20,
        "midtone_contrast": 0, "black_clip": 0, "white_clip": 0
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
        let file = format!("fx_shclip_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b443-{id}"), p)]));
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

#[test]
fn b443_shadow_highlight_clip() {
    let mut t = Table::new(
        "shadow_highlight_clip",
        "# D-563: Shadow/Highlight's Midtone Contrast and Black and White Clip\n\nB-443, after \
         After Effects' Shadow/Highlight: after D-400's lift, Midtone Contrast is Color Grade's \
         contrast curve on the lightness; Black Clip and White Clip stretch the lightness from \
         the clip bins of the layer's own count to its darkest and lightest bins in use; the \
         chroma follows each change toward black or white. The rule is this program's own \
         (Adobe does not publish one). Every expected pixel is \
         `Fixtures/shadow_highlight_clip/expected_shadow_highlight_clip.json`, written by \
         `tools/shadow_highlight_clip_reference.py` before this code existed and printed in \
         document 25 as FX-SHCLIP-001 to 021. Tolerance 2e-5.\n",
    );

    t.heading("FX-SHCLIP-001 to 021 (document 25)");
    t.fixtures_numbered("expected_shadow_highlight_clip.json", 1..=21);

    t.heading("The file");
    let files: Vec<String> = (1..=21).map(|n| format!("fx_shclip_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_shclip_001.json");
    t.row(
        "fx_shclip_001.json, written without the three settings, is saved without them (D-304's way: only if moved, keyed or already in the file)",
        &old.to_string(),
        old.as_object().unwrap().len() == 7 && ["midtone_contrast", "black_clip", "white_clip"].iter().all(|k| old.get(*k).is_none()),
    );
    let saved = t.saved_parameters("fx_shclip_013.json");
    t.row(
        "fx_shclip_013.json is saved with its ten settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 10
            && saved["shadow_amount"] == 70.0
            && saved["highlight_amount"] == 30.0
            && saved["midtone_contrast"] == 40.0
            && saved["black_clip"] == 10.0
            && saved["white_clip"] == 10.0,
    );
    for (file, want) in [
        ("fx_shclip_018.json", "midtone_contrast"),
        ("fx_shclip_019.json", "midtone_contrast"),
        ("fx_shclip_020.json", "black_clip"),
        ("fx_shclip_021.json", "white_clip"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let want = want.replace('_', " ");
        t.row(&format!("{file} is refused in a sentence naming its {want}"), &why, why.contains(&want));
    }
    let rest = r#""shadow_amount": 50, "highlight_amount": 0, "shadow_tonal_width": 50, "shadow_radius": 30, "highlight_tonal_width": 50, "highlight_radius": 30, "color_correction": 20"#;
    t.shape_refused("fx_shclip_002.json", "a Shadow/Highlight whose midtone contrast is a word", &format!(r#"{{{rest}, "midtone_contrast": "more", "black_clip": 0, "white_clip": 0}}"#));
    t.shape_refused("fx_shclip_002.json", "a Shadow/Highlight whose black clip is a word", &format!(r#"{{{rest}, "midtone_contrast": 0, "black_clip": "some", "white_clip": 0}}"#));

    t.heading("Commands");
    let mut document = t.load("fx_shclip_002.json").document;
    let over = changed(&t, "fx_shclip_002.json", |e| if let Effect::ShadowHighlight { midtone_contrast, .. } = e { *midtone_contrast = 101.0 });
    let wide = changed(&t, "fx_shclip_002.json", |e| if let Effect::ShadowHighlight { white_clip, .. } = e { *white_clip = 51.0 });
    t.refused(
        &mut document,
        vec![
            ("midtone contrast 101", set(over)),
            ("white clip 51", set(wide)),
            ("black clip keyed to -5", keys("black_clip", &[(0, &[0.0]), (4, &[-5.0])])),
        ],
    );
    let both_ends = changed(&t, "fx_shclip_002.json", |e| {
        if let Effect::ShadowHighlight { midtone_contrast, black_clip, white_clip, .. } = e {
            *midtone_contrast = 30.0;
            *black_clip = 5.0;
            *white_clip = 5.0;
        }
    });
    t.taken(
        &mut document,
        "fx_shclip_002.json",
        vec![
            ("midtone contrast 30, both clips 5", set(both_ends)),
            ("midtone contrast keyed from -100 to 100", keys("midtone_contrast", &[(0, &[-100.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_shclip_004.json", 0),
        ("fx_shclip_008.json", 0),
        ("fx_shclip_010.json", 0),
        ("fx_shclip_013.json", 0),
        ("fx_shclip_015.json", 2),
        ("fx_shclip_016.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // A clip above 0 keeps the effect on the processor (008 to 013, 016, 017; 015 after frame
    // 0); refused files (018 to 021) leave nothing for the card.
    let none: Vec<u32> = (8..=13).chain([16, 17]).chain(18..=21).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added with midtone contrast 60", json!({"midtone_contrast": 60}), 3),
            ("shadow 0, midtone contrast -80", json!({"shadow_amount": 0, "midtone_contrast": -80}), 3),
            ("black clip 2 and white clip 2, on the processor", json!({"black_clip": 2, "white_clip": 2}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-563 pictures/`");
    let dir = repo("verification/D-563 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row("2_as_added.png, Shadow/Highlight as added (the three new settings 0); draws cleanly", &format!("{said:?}"), said.is_empty());
    let luma = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    let spread = |v: &[u8]| {
        let l: Vec<f64> = v.chunks_exact(4).filter(|p| p[3] > 0).map(luma).collect();
        let m = l.iter().sum::<f64>() / l.len() as f64;
        (l.iter().map(|x| (x - m) * (x - m)).sum::<f64>() / l.len() as f64).sqrt()
    };

    let (more, said0) = street(json!({"midtone_contrast": 80}));
    let (less, said1) = street(json!({"midtone_contrast": -80}));
    write("3_midtone_80.png", TOWN, &more);
    write("4_midtone_minus_80.png", TOWN, &less);
    t.row(
        "3_midtone_80.png and 4_midtone_minus_80.png, as added with midtone contrast 80 and -80: the street's brightness spread wider at 80 and drawn in at -80 than as added; draw cleanly",
        &format!("{said0:?} {said1:?}, spread of brightness {:.2}, {:.2} as added, {:.2}", spread(&more), spread(&added), spread(&less)),
        said0.is_empty() && said1.is_empty() && spread(&more) > spread(&added) && spread(&added) > spread(&less),
    );

    let (clipped, said) = street(json!({"black_clip": 5, "white_clip": 5}));
    write("5_clips_5.png", TOWN, &clipped);
    // The clips stretch the lightness to the street's own darkest and lightest in use, so the
    // darks fall below the as-added darkest and the lights reach white.
    let floor = added.chunks_exact(4).filter(|p| p[3] > 0).map(luma).fold(f64::INFINITY, f64::min);
    let dark = |v: &[u8]| v.chunks_exact(4).filter(|p| p[3] > 0 && luma(p) <= floor).count();
    let white = |v: &[u8]| v.chunks_exact(4).filter(|p| p[3] > 0 && p[..3] == [255, 255, 255]).count();
    t.row(
        "5_clips_5.png, as added with both clips 5: more of the street at or below the as-added darkest and more white than as added; draws cleanly",
        &format!("{said:?}, pixels at or below the as-added darkest {} against {}, white {} against {}", dark(&clipped), dark(&added), white(&clipped), white(&added)),
        said.is_empty() && dark(&clipped) > dark(&added) && white(&clipped) > white(&added),
    );

    let (alone, said) = street(json!({"shadow_amount": 0, "midtone_contrast": 50, "black_clip": 1, "white_clip": 1}));
    write("6_no_lift.png", TOWN, &alone);
    t.row(
        "6_no_lift.png, both amounts 0, midtone contrast 50, both clips 1: the new settings work without the lift; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&alone, &before).1),
        said.is_empty() && alone != before,
    );

    t.finish("D-563_shadow_highlight_clip_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B443_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-443: a measurement, run deliberately with --release --ignored"]
fn b443_shadow_highlight_clip_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B443_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Shadow/Highlight as added", Some(json!({}))),
        ("Noise, then Shadow/Highlight as added with midtone contrast 60", Some(json!({"midtone_contrast": 60}))),
        ("Noise, then Shadow/Highlight as added with both clips 1 (on the processor)", Some(json!({"black_clip": 1, "white_clip": 1}))),
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
    let out = std::env::var("B443_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-443_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
