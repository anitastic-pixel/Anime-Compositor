//! B-445: D-565, Energy Stroke's extras (docs/effects/EFFECTS.md row 53, P0-22): the shape of
//! each end of the core (Start and End Roundness), the noise read on the layer instead of along
//! the path (Noise Along Path off), and the layer's masks cutting the core, the glow or both
//! (Alpha Mode, Invert Masks). The core from two points and the size at each end were D-508's.
//! The rules are ours, stated in `tools/energy_stroke_extras_reference.py`. Open masks stay a gap
//! (document 19): every mask here is closed.
//!
//! Writes `verification/D-565_energy_stroke_extras_table.md` and draws pictures into
//! `verification/D-565 pictures/`.
//!
//! Every expected pixel is `Fixtures/energy_stroke_extras/expected_energy_stroke_extras.json`,
//! written by the tool before this code existed and printed in document 25 as FX-ENERGYX-001 to
//! 033. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
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

/// The Energy Strokes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::EnergyStroke { .. })))
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
    let said_gpu = said_gpu.split(", ").filter(|id| *id != DiagnosticId::GpuPreviewOnCpu.as_str()).collect::<Vec<_>>().join(", ");
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers.

fn reference(stack: impl Fn(&str) -> J) -> Project {
    reference_masked(stack, None)
}

/// As [`reference`], each of the three layers also given `masks` when there are some.
fn reference_masked(stack: impl Fn(&str) -> J, masks: Option<J>) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
        if let Some(m) = &masks {
            layers[i]["masks"] = m.clone();
        }
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}


/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=33 {
        let file = format!("fx_energyx_{n:03}.json");
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, Option<J>, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, masks, want) in settings {
        let project = reference_masked(|id| json!([fx(&format!("b445-{id}"), p)]), masks.clone());
        let plain = if masks.is_some() { cpu(&reference_masked(|_| json!([]), masks.clone())) } else { plain.clone() };
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Energy Stroke {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Energy Stroke {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` (and `masks`) on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), masks: J, effects: J) -> (Vec<u8>, Vec<String>) {
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
    layer["masks"] = masks;
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

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}


/// An Energy Stroke with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source": "points", "core_start": [25, 50], "core_end": [75, 50], "evolution": 0,
        "start_offset": 0, "end_offset": 100, "start_size": 100, "end_size": 100, "core_size": 8,
        "core_softness": 30, "glow_color": "#3080ff", "glow_intensity": 100, "glow_spread": 40,
        "flicker_amount": 0, "flicker_speed": 20, "distortion_amount": 0, "distortion_scale": 60,
        "distortion_speed": 20, "random_seed": 0, "composite": "add"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.energy_stroke", "enabled": true, "parameters": all})
}

/// A closed mask of mode None through `points`, as the fixtures write one.
fn mask(points: &[[f64; 2]]) -> J {
    let points: Vec<J> = points.iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect();
    json!([{"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": {"points": points}, "keyframes": []}}])
}

#[test]
fn b445_energy_stroke_extras() {
    let mut t = Table::new(
        "energy_stroke_extras",
        "# D-565: Energy Stroke's extras\n\nB-445, docs/effects/EFFECTS.md row 53 (P0-22): \
         Start and End Roundness shape each end of the core, from round (100, as added and as \
         D-508 drew) to flat (0); Noise Along Path off reads the wobble's noise on the layer, so \
         it stays put while the beam moves through it; Alpha Mode lets every mask of the layer \
         that is on, whatever its mode, cut the core, the glow or both, turned over by Invert \
         Masks. The core from two points and the size at each end were D-508's. The rules are \
         ours, stated in `tools/energy_stroke_extras_reference.py`. Every expected pixel is \
         `Fixtures/energy_stroke_extras/expected_energy_stroke_extras.json`, written by the tool \
         before this code existed and printed in document 25 as FX-ENERGYX-001 to 033. Tolerance \
         2e-5. Open masks stay a gap (document 19). The masks' cut runs on the processor, which \
         holds the masks; the ends and the noise on the layer run on the card too.\n",
    );

    t.heading("FX-ENERGYX-001 to 033 (document 25)");
    t.fixtures_numbered("expected_energy_stroke_extras.json", 1..=33);

    t.heading("The file");
    let files: Vec<String> = (1..=33).map(|n| format!("fx_energyx_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_energyx_020.json");
    t.row(
        "fx_energyx_020.json is saved with its twenty-five settings, Alpha Mode masks, Invert Masks on",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 25 && saved["alpha_mode"] == "masks" && saved["invert_masks"] == "on",
    );
    // D-304: a file from before is saved as it was, without the five new settings.
    let old_root = repo("Fixtures/energy_stroke");
    for file in ["fx_energy_001.json", "fx_energy_009.json", "fx_energy_032.json"] {
        let original: J = serde_json::from_str(&fs::read_to_string(old_root.join(file)).unwrap()).unwrap();
        let loaded = persist::load(&old_root.join(file)).unwrap();
        let same = effect_table::same_json(&effect_table::saved(&loaded), &original);
        t.row(
            &format!("D-508's {file} opened and saved holds what it held, none of the five new settings written"),
            if same { "the same" } else { "differs" },
            same,
        );
    }
    let mut e = changed(&t, "fx_energyx_001.json", |_| {});
    if let Effect::EnergyStroke { start_roundness, alpha_mode, .. } = &mut e {
        *start_roundness = 30.0;
        *alpha_mode = "glow".into();
    }
    t.row("a Start Roundness of 30 and Alpha Mode glow are valid", &e.why_invalid(), e.why_invalid().is_empty());
    for (file, want) in [
        ("fx_energyx_029.json", "start_roundness"),
        ("fx_energyx_030.json", "end_roundness"),
        ("fx_energyx_031.json", "noise along path"),
        ("fx_energyx_032.json", "alpha mode"),
        ("fx_energyx_033.json", "invert masks"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let named = why.to_lowercase().contains(want) || why.to_lowercase().contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence naming {}", want.replace('_', " ")), &why, named);
    }
    t.shape_refused(
        "fx_energyx_001.json",
        "an Energy Stroke whose Start Roundness is a word",
        r##"{"source": "points", "core_start": [25, 50], "core_end": [75, 50], "evolution": 0, "start_offset": 0, "end_offset": 100, "start_size": 100, "end_size": 100, "core_size": 8, "core_softness": 30, "glow_color": "#3080ff", "glow_intensity": 100, "glow_spread": 40, "flicker_amount": 0, "flicker_speed": 20, "distortion_amount": 0, "distortion_scale": 60, "distortion_speed": 20, "random_seed": 0, "composite": "add", "start_roundness": "flat"}"##,
    );

    t.heading("On the card (D-565)");
    let mut gpu = Gpu::new().expect("a graphics card");
    // 011 is a shape layer, which has no drawing for the card (B-236). 017 to 020 and 022 to 028
    // have Alpha Mode on, which the processor draws since it holds the masks. 029 to 033 are
    // refused.
    let none: Vec<u32> = [11].into_iter().chain(17..=20).chain(22..=33).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    let window = mask(&[[300.0, 200.0], [1500.0, 200.0], [1500.0, 800.0], [300.0, 800.0]]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("core 30, Start Roundness 0, End Roundness 40", json!({"core_size": 30, "start_roundness": 0, "end_roundness": 40}), None, 3),
            ("Noise Along Path off, distortion 20 at scale 80, speed 40", json!({"noise_along_path": "off", "distortion_amount": 20, "distortion_scale": 80, "distortion_speed": 40}), None, 3),
            ("Alpha Mode core, a mask of mode None on each layer", json!({"alpha_mode": "core", "core_size": 20}), Some(window), 0),
            ("Alpha Mode masks, Invert Masks on, a small mask in the corner", json!({"alpha_mode": "masks", "invert_masks": "on"}), Some(mask(&[[0.0, 0.0], [300.0, 0.0], [300.0, 300.0], [0.0, 300.0]])), 0),
        ],
    );

    t.heading("Pictures (verification/D-565 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-565 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, "1_left.png", (480, 270), json!([]), json!([])).0;
    let middle = mask(&[[160.0, 60.0], [320.0, 60.0], [320.0, 210.0], [160.0, 210.0]]);
    let saber = json!({"glow_color": "#ff6010", "glow_intensity": 200, "core_size": 14, "glow_spread": 30, "core_start": [15, 50], "core_end": [85, 50]});
    let with = |extra: J| {
        let mut p = saber.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    // Each picture also differs from the one before it, so each setting is seen to do something.
    let mut before = street.clone();
    for (name, masks, p, on) in [
        ("1_round_ends_as_added", json!([]), with(json!({})), (72, 135)),
        ("2_flat_ends", json!([]), with(json!({"start_roundness": 0, "end_roundness": 0})), (72, 135)),
        ("3_flat_start_pointed_end", json!([]), with(json!({"start_roundness": 0, "end_roundness": 30, "end_size": 30})), (240, 135)),
        ("4_noise_along_path", json!([]), with(json!({"distortion_amount": 10, "distortion_scale": 40})), (240, 135)),
        ("5_noise_on_the_layer", json!([]), with(json!({"noise_along_path": "off", "distortion_amount": 10, "distortion_scale": 40})), (240, 135)),
        ("6_masks_cut_the_core", middle.clone(), with(json!({"alpha_mode": "core"})), (240, 135)),
        ("7_masks_cut_both_inverted", middle, with(json!({"alpha_mode": "masks", "invert_masks": "on"})), (100, 135)),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), masks, json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let lit = at(&bytes, on);
        let moved = distance(&bytes, &before).1;
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed, lit at {on:?}, unlike the picture before"),
            &format!("{} pixels changed; at {on:?} {lit:?}; {moved} pixels unlike the picture before; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0 && lit[3] > 0 && moved > 0,
        );
        before = bytes;
    }
    t.finish("D-565_energy_stroke_extras_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B445_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-445: a measurement, run deliberately with --release --ignored"]
fn b445_energy_stroke_extras_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B445_CPU").is_ok();
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
    let window = mask(&[[300.0, 200.0], [1500.0, 200.0], [1500.0, 800.0], [300.0, 800.0]]);
    let shots: [(&str, Option<J>, Option<J>); 5] = [
        ("Noise alone", None, None),
        ("Noise, then Energy Stroke as added", Some(json!({})), None),
        ("Noise, then Energy Stroke, core 30, both ends flat", Some(json!({"core_size": 30, "start_roundness": 0, "end_roundness": 0})), None),
        ("Noise, then Energy Stroke, noise on the layer, distortion 20", Some(json!({"noise_along_path": "off", "distortion_amount": 20, "distortion_scale": 80})), None),
        ("Noise, then Energy Stroke, Alpha Mode masks, one mask", Some(json!({"alpha_mode": "masks"})), Some(window)),
    ];
    for (name, e, masks) in shots {
        let project = reference_masked(
            |id| {
                let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                    "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
                if let Some(p) = &e {
                    v.push(fx(&format!("{id}u"), p));
                }
                J::Array(v)
            },
            masks,
        );
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
    let out = std::env::var("B445_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-445_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
