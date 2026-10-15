//! B-388: D-508, Energy Stroke, after the purpose of Video Copilot's Saber (docs/effects/PLUGINS.md
//! pick #11), built separately from Path Stroke as that file recommends: a glowing beam, a white
//! core inside a coloured glow, between two points or along the layer's masks, shapes or text
//! outlines. Saber's controls are named in its tutorials, but no formula and no settings when
//! added are published: the rule and those settings are ours, stated in
//! `tools/energy_stroke_reference.py`.
//!
//! Writes `verification/D-508_energy_stroke_table.md` and draws pictures into
//! `verification/D-508 pictures/`.
//!
//! Every expected pixel is `Fixtures/energy_stroke/expected_energy_stroke.json`, written by the
//! tool before this code existed and printed in document 25 as FX-ENERGY-001 to 053. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, distance, median, on_card, reference, repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// The reference shot (1920 by 1080) with `stack` on its first three layers.


/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=53 {
        let file = format!("fx_energy_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::EnergyStroke { .. }));
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
        let project = reference(|id| json!([fx(&format!("b388-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Energy Stroke {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::EnergyStroke { .. }));
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
fn b388_energy_stroke() {
    let mut t = Table::new(
        "energy_stroke",
        "# D-508: Energy Stroke\n\nB-388, after the purpose of Video Copilot's Saber \
         (docs/effects/PLUGINS.md pick #11), built separately from Path Stroke: a glowing beam, a \
         white core inside a coloured glow, between two points or along the layer's masks, shapes \
         or text outlines. Saber's controls are named in its tutorials but no formula and no \
         settings when added are published, so the rule and those settings are ours, stated in \
         `tools/energy_stroke_reference.py`: the core a smoothstep round each bit of the path, the \
         glow four soft rings each half the last, flicker and distortion from value noise. Every \
         expected pixel is `Fixtures/energy_stroke/expected_energy_stroke.json`, written by the \
         tool before this code existed and printed in document 25 as FX-ENERGY-001 to 053. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-ENERGY-001 to 053 (document 25)");
    t.fixtures_numbered("expected_energy_stroke.json", 1..=53);

    t.heading("The file");
    let files: Vec<String> = (1..=53).map(|n| format!("fx_energy_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_energy_009.json");
    t.row(
        "fx_energy_009.json is saved with its twenty settings, an orange glow at intensity 400, Core Start [25, 50]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 20
            && saved["glow_color"] == "#ff8020"
            && saved["glow_intensity"] == 400
            && saved["core_start"] == json!([25, 50]),
    );
    for (file, want) in [
        ("fx_energy_040.json", "core size"),
        ("fx_energy_041.json", "core softness"),
        ("fx_energy_042.json", "glow spread"),
        ("fx_energy_043.json", "glow intensity"),
        ("fx_energy_044.json", "start offset"),
        ("fx_energy_045.json", "end size"),
        ("fx_energy_046.json", "distortion scale"),
        ("fx_energy_047.json", "flicker amount"),
        ("fx_energy_048.json", "core start"),
        ("fx_energy_049.json", "evolution"),
        ("fx_energy_050.json", "random seed"),
        ("fx_energy_051.json", "composite"),
        ("fx_energy_052.json", "path from"),
        ("fx_energy_053.json", "glow colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    t.shape_refused(
        "fx_energy_001.json",
        "an Energy Stroke whose core size is a word",
        r##"{"source": "points", "core_start": [25, 50], "core_end": [75, 50], "evolution": 0, "start_offset": 0, "end_offset": 100, "start_size": 100, "end_size": 100, "core_size": "big", "core_softness": 30, "glow_color": "#3080ff", "glow_intensity": 100, "glow_spread": 40, "flicker_amount": 0, "flicker_speed": 20, "distortion_amount": 0, "distortion_scale": 60, "distortion_speed": 20, "random_seed": 0, "composite": "add"}"##,
    );
    t.shape_refused(
        "fx_energy_001.json",
        "an Energy Stroke whose Core End is one number",
        r##"{"source": "points", "core_start": [25, 50], "core_end": 75, "evolution": 0, "start_offset": 0, "end_offset": 100, "start_size": 100, "end_size": 100, "core_size": 8, "core_softness": 30, "glow_color": "#3080ff", "glow_intensity": 100, "glow_spread": 40, "flicker_amount": 0, "flicker_speed": 20, "distortion_amount": 0, "distortion_scale": 60, "distortion_speed": 20, "random_seed": 0, "composite": "add"}"##,
    );

    t.heading("On the card (D-508)");
    let mut gpu = Gpu::new().expect("a graphics card");
    // A shape or text layer has no drawing, and the card is handed effects on drawings only
    // (B-236 and B-252, Path Stroke's): FX-ENERGY-029 to 031, 034 and 035 are drawn by the
    // processor in both paths. 036 to 053 draw nothing or are refused.
    let none: Vec<u32> = [29, 30, 31, 34, 35].into_iter().chain(36..=53).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (a blue beam across the middle, added)", json!({}), 3),
            ("orange, intensity 300, diagonal, transparent", json!({"glow_color": "#ff8020", "glow_intensity": 300, "core_start": [10, 10], "core_end": [90, 90], "composite": "transparent"}), 3),
            ("distortion 20 at scale 80, flicker 60 at speed 40, black", json!({"distortion_amount": 20, "distortion_scale": 80, "flicker_amount": 60, "flicker_speed": 40, "composite": "black"}), 3),
            ("tapered, End Offset keyed from 0 to 100", json!({"end_size": 0, "end_offset": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 100, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-508 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-508 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, "1_left.png", (480, 270), json!([]), json!([])).0;
    let frame = mask(&[[60.0, 40.0], [420.0, 40.0], [420.0, 230.0], [60.0, 230.0]]);
    for (name, masks, p, on) in [
        ("1_as_added", json!([]), json!({}), (240, 135)),
        ("2_orange_saber", json!([]), json!({"glow_color": "#ff6010", "glow_intensity": 200, "core_size": 6, "glow_spread": 30, "core_start": [15, 80], "core_end": [85, 20]}), (240, 135)),
        ("3_round_a_mask", frame.clone(), json!({"source": "masks", "glow_color": "#40ff80", "end_offset": 70, "core_size": 5, "glow_spread": 25}), (240, 40)),
        ("4_wobble", frame, json!({"source": "masks", "distortion_amount": 8, "distortion_scale": 50, "glow_color": "#c040ff", "core_size": 5, "glow_spread": 25}), (240, 135)),
        ("5_over_black", json!([]), json!({"composite": "black", "glow_spread": 60}), (240, 135)),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), masks, json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let lit = at(&bytes, on);
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed, lit at {on:?}"),
            &format!("{} pixels changed; at {on:?} {lit:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0 && lit[3] > 0,
        );
    }
    t.finish("D-508_energy_stroke_table.md");
}


/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B388_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-388: a measurement, run deliberately with --release --ignored"]
fn b388_energy_stroke_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B388_CPU").is_ok();
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
        ("Noise, then Energy Stroke as added", Some(json!({}))),
        ("Noise, then Energy Stroke, orange, intensity 300, spread 120", Some(json!({"glow_color": "#ff8020", "glow_intensity": 300, "glow_spread": 120}))),
        ("Noise, then Energy Stroke, distortion 20, flicker 60", Some(json!({"distortion_amount": 20, "distortion_scale": 80, "flicker_amount": 60}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
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
    let out = std::env::var("B388_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-388_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
