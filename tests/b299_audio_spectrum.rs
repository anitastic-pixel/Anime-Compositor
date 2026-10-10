//! B-299: D-420 Audio Spectrum, after After Effects' Audio Spectrum ("Generate" in
//! `docs/effects/EFFECTS.md`): the frequencies of a sound layer drawn as bars, a line or dots
//! along a line, round a point or round a mask.
//!
//! Writes `verification/D-420_audio_spectrum_table.md`.
//!
//! Every expected pixel is `Fixtures/audio_spectrum/expected_audio_spectrum.json`, written by
//! `tools/audio_spectrum_reference.py` before this code existed and printed in document 25 as
//! FX-ASPEC-001 to 053. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-420 pictures/`, to a piece of music made here.

mod effect_table;

use std::fs;
use std::path::{Path, PathBuf};

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

/// An Audio Spectrum with the starting settings but for `change`, given as the file writes them.
fn spectrum(change: J) -> Effect {
    let mut all = defaults();
    for (k, v) in change.as_object().unwrap() {
        all[k] = v.clone();
    }
    let n = |k: &str| all[k].as_f64().unwrap();
    let p = |k: &str| [all[k][0].as_f64().unwrap(), all[k][1].as_f64().unwrap()];
    let w = |k: &str| all[k].as_str().unwrap().to_string();
    Effect::AudioSpectrum {
        audio_layer: all["audio_layer"].clone(),
        start_point: p("start_point"),
        end_point: p("end_point"),
        path: n("path"),
        use_polar_path: w("use_polar_path"),
        start_frequency: n("start_frequency"),
        end_frequency: n("end_frequency"),
        frequency_bands: n("frequency_bands"),
        maximum_height: n("maximum_height"),
        audio_duration: n("audio_duration"),
        audio_offset: n("audio_offset"),
        thickness: n("thickness"),
        softness: n("softness"),
        inside_color: w("inside_color"),
        outside_color: w("outside_color"),
        blend_overlapping_colors: w("blend_overlapping_colors"),
        hue_interpolation: n("hue_interpolation"),
        dynamic_hue_phase: w("dynamic_hue_phase"),
        color_symmetry: w("color_symmetry"),
        display_options: w("display_options"),
        side_options: w("side_options"),
        duration_averaging: w("duration_averaging"),
        composite: w("composite"),
        levels: None,
        outline: None,
    }
}

/// After Effects' starting settings, as the window adds the effect.
fn defaults() -> J {
    json!({"audio_layer": "", "start_point": [10, 50], "end_point": [90, 50], "path": 0, "use_polar_path": "off",
        "start_frequency": 20, "end_frequency": 2000, "frequency_bands": 64, "maximum_height": 200, "audio_duration": 90,
        "audio_offset": 0, "thickness": 3, "softness": 50, "inside_color": "#ffffff", "outside_color": "#3c8cff",
        "blend_overlapping_colors": "off", "hue_interpolation": 0, "dynamic_hue_phase": "off", "color_symmetry": "off",
        "display_options": "digital", "side_options": "side_a_b", "duration_averaging": "off", "composite": "off"})
}

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

fn is_spectrum(e: &Effect) -> bool {
    matches!(e, Effect::AudioSpectrum { .. })
}

/// The Audio Spectrums the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_spectrum(&f.instance.effect)))
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

/// Ten seconds of music made here, mono, 16 bits at 48000 a second: eight notes from 110 to
/// 1760 hertz, each swelling and fading at its own pace, written once to the temporary folder.
fn music() -> PathBuf {
    let path = std::env::temp_dir().join("b299_music.wav");
    let rate = 48000u32;
    let n = rate as usize * 10;
    let notes = [110.0, 220.0, 330.0, 440.0, 660.0, 880.0, 1320.0, 1760.0];
    let mut data = Vec::with_capacity(n * 2);
    for i in 0..n {
        let t = i as f64 / rate as f64;
        let v: f64 = notes
            .iter()
            .enumerate()
            .map(|(k, f)| {
                let swell = 0.5 + 0.5 * (std::f64::consts::TAU * (0.3 + 0.17 * k as f64) * t + k as f64).sin();
                0.12 * swell * (std::f64::consts::TAU * f * t).sin()
            })
            .sum();
        data.extend(((v * 32767.0).round() as i16).to_le_bytes());
    }
    let mut wav = Vec::new();
    wav.extend(b"RIFF");
    wav.extend((36 + data.len() as u32).to_le_bytes());
    wav.extend(b"WAVEfmt ");
    wav.extend(16u32.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(1u16.to_le_bytes());
    wav.extend(rate.to_le_bytes());
    wav.extend((rate * 2).to_le_bytes());
    wav.extend(2u16.to_le_bytes());
    wav.extend(16u16.to_le_bytes());
    wav.extend(b"data");
    wav.extend((data.len() as u32).to_le_bytes());
    wav.extend(data);
    if fs::read(&path).ok().as_deref() != Some(&wav[..]) {
        fs::write(&path, wav).expect("write the music");
    }
    path
}

/// `project` given the music as a sound layer `sound`, playing from its first frame for `frames`.
fn with_music(project: &mut J, frames: i64) {
    project["assets"].as_array_mut().unwrap().push(json!({"id": "asset-music", "kind": "audio", "name": "music", "path": music().to_string_lossy()}));
    let comp = &mut project["compositions"][0];
    comp["layers"].as_array_mut().unwrap().push(json!({"id": "sound", "kind": "audio", "name": "sound", "asset_id": "asset-music", "enabled": true,
        "locked": false, "in_frame": 0, "out_frame": frames, "source_offset_frames": 0, "gain_db": 0.0}));
    if let Some(order) = comp["layer_order"].as_array_mut() {
        order.push(J::from("sound"));
    }
}

/// The reference shot (1920 by 1080) with the music and `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        j["compositions"][0]["layers"][i]["effects"] = stack(id);
    }
    with_music(&mut j, 240);
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`, the rest as a new one takes them.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.audio_spectrum" => defaults(),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` draw nothing or are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, count: u32, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("fx_aspec_{n:03}.json");
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
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with the music and each setting on its first three layers, frames 0, 100
/// and 239 at Full and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx("core.audio_spectrum", &format!("b299-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Audio Spectrum {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Audio Spectrum {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street with the music, frame `frame` drawn, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(240);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["out_frame"] = J::from(240);
    layer["effects"] = effects;
    with_music(&mut project, 240);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
}

/// Changed, and some of it left clear.
fn gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

/// Changed, and nothing clear that was not clear before.
fn kept(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] > 0 || q[3] == 0)
}

/// The street plain and with each shot, as numbered pictures in `verification/D-420 pictures/`.
fn street(t: &mut Table, shots: &[(&str, J, i32, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading("Pictures: the street, in `verification/D-420 pictures/`");
    let dir = repo("verification/D-420 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx("core.audio_spectrum", "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

#[test]
fn b299_audio_spectrum() {
    let mut t = Table::new(
        "audio_spectrum",
        "# D-420: Audio Spectrum\n\nB-299, after After Effects' Audio Spectrum: the levels of a sound \
         layer at Frequency Bands frequencies evenly spaced from Start to End Frequency, each 2 |sum x_k \
         w_k e^(-2 pi i f k / rate)| / sum w_k over Audio Duration from the frame's first sample plus \
         Audio Offset (Hann window, channels averaged, the layer's gain applied; Duration Averaging the \
         mean over three windows half a window apart), drawn as bars (Digital), a line through their \
         tips (Analog Lines) or dots (Analog Dots), Maximum Height times the level (at most 1) tall, \
         along the line from Start to End Point, round Start Point (Use Polar Path) or round a mask \
         (Path), each mark Beam's line (D-207). Every expected pixel is \
         `Fixtures/audio_spectrum/expected_audio_spectrum.json`, written by \
         `tools/audio_spectrum_reference.py` before this code existed and printed in document 25 as \
         FX-ASPEC-001 to 053. Tolerance 2e-5. The levels are worked out on the processor and the marks \
         drawn on the card.\n",
    );

    t.heading("FX-ASPEC-001 to 053 (document 25)");
    t.fixtures("expected_audio_spectrum.json");

    t.heading("The file");
    let all: Vec<String> = (1..=53).map(|n| format!("fx_aspec_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_aspec_002.json");
    t.row(
        "fx_aspec_002.json is saved with every setting as written, the levels it heard not among them",
        &saved.to_string(),
        saved["audio_layer"] == "sound" && saved["frequency_bands"] == 4 && saved["side_options"] == "side_a" && saved.get("levels").is_none()
            && saved.get("outline").is_none() && saved.as_object().unwrap().len() == 23,
    );
    why(
        &mut t,
        &[
            ("fx_aspec_035.json", "Audio Spectrum's frequency bands runs from 1 to 4096, and this is 0."),
            ("fx_aspec_036.json", "Audio Spectrum's frequency bands runs from 1 to 4096, and this is 4097."),
            ("fx_aspec_037.json", "Audio Spectrum's start frequency runs from 1 to 20000, and this is 0."),
            ("fx_aspec_038.json", "Audio Spectrum's end frequency runs from 1 to 20000, and this is 20001."),
            ("fx_aspec_039.json", "Audio Spectrum's maximum height runs from 0 to 10000, and this is -1."),
            ("fx_aspec_040.json", "Audio Spectrum's audio duration runs from 1 to 30000, and this is 0."),
            ("fx_aspec_041.json", "Audio Spectrum's audio offset runs from -30000 to 30000, and this is 30001."),
            ("fx_aspec_042.json", "Audio Spectrum's thickness runs from 0 to 10000, and this is -1."),
            ("fx_aspec_043.json", "Audio Spectrum's softness runs from 0 to 100, and this is 101."),
            ("fx_aspec_044.json", "Audio Spectrum's hue interpolation runs from -3600 to 3600, and this is 3601."),
            ("fx_aspec_045.json", "Audio Spectrum's path runs from 0 to 1000, and this is -1."),
            ("fx_aspec_046.json", "Audio Spectrum's start point runs from -1000 to 1000, and this is 1001."),
            ("fx_aspec_047.json", "Audio Spectrum's display options is \"digital\", \"analog_lines\" or \"analog_dots\", and this is \"bars\"."),
            ("fx_aspec_048.json", "Audio Spectrum's side options is \"side_a\", \"side_b\" or \"side_a_b\", and this is \"side_c\"."),
            ("fx_aspec_049.json", "Audio Spectrum's composite is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_aspec_050.json", "Audio Spectrum's inside colour is written #rrggbb, and this is \"#12345\"."),
            ("fx_aspec_051.json", "Audio Spectrum's use polar path is \"off\" or \"on\", and this is \"maybe\"."),
            ("fx_aspec_052.json", "Audio Spectrum's audio layer is the name of a layer of this composition, and this is 5."),
        ],
    );
    let mut no_bands = defaults();
    no_bands.as_object_mut().unwrap().remove("frequency_bands");
    t.shape_refused("fx_aspec_001.json", "an Audio Spectrum with no `frequency_bands`", &no_bands.to_string());
    let mut worded = defaults();
    worded["thickness"] = J::from("thick");
    t.shape_refused("fx_aspec_001.json", "an Audio Spectrum with a thickness in words", &worded.to_string());
    let mut one = defaults();
    one["end_point"] = json!([90]);
    t.shape_refused("fx_aspec_001.json", "an Audio Spectrum whose end point is one number", &one.to_string());

    t.heading("Commands");
    let mut document = t.load("fx_aspec_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("frequency bands 0", set(spectrum(json!({"audio_layer": "sound", "frequency_bands": 0})))),
            ("display options \"bars\"", set(spectrum(json!({"audio_layer": "sound", "display_options": "bars"})))),
            ("audio layer 5", set(spectrum(json!({"audio_layer": 5})))),
            ("maximum height keyed to 10001", keys("maximum_height", &[(0, &[8.0]), (4, &[10001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_aspec_002.json",
        vec![
            ("32 bands as analog lines round the middle, orange, blended", set(spectrum(json!({"audio_layer": "sound", "frequency_bands": 32, "use_polar_path": "on", "display_options": "analog_lines", "inside_color": "#ff8000", "blend_overlapping_colors": "on", "maximum_height": 4})))),
            ("start point keyed from 10, 80 to 20, 50", keys("start_point", &[(0, &[10.0, 80.0]), (4, &[20.0, 50.0])])),
            ("thickness keyed from 1 to 3", keys("thickness", &[(0, &[1.0]), (4, &[3.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_aspec_002.json", 2), ("fx_aspec_006.json", 0), ("fx_aspec_008.json", 0), ("fx_aspec_012.json", 0), ("fx_aspec_017.json", 0), ("fx_aspec_019.json", 0), ("fx_aspec_031.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let none: Vec<u32> = [20, 21, 22].into_iter().chain(35..=53).collect();
    card_fixtures(&mut t, &mut gpu, 53, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as it starts, listening to the music (64 bars both ways along the middle, alone)", json!({"audio_layer": "sound"})),
            ("in silence, no Audio Layer (64 dots along the middle), thickness 10", json!({"thickness": 10})),
            ("Analog Lines, Side A, 1500 tall, thickness 6, softness 80", json!({"audio_layer": "sound", "display_options": "analog_lines", "side_options": "side_a", "maximum_height": 1500, "thickness": 6, "softness": 80})),
            ("Analog Dots round the middle, 128 bands, 800 tall, thickness 8", json!({"audio_layer": "sound", "use_polar_path": "on", "start_point": [50, 50], "display_options": "analog_dots", "frequency_bands": 128, "maximum_height": 800, "thickness": 8})),
            ("32 bars blended, thickness 40, Hue Interpolation 360, Dynamic Hue Phase and Color Symmetry", json!({"audio_layer": "sound", "frequency_bands": 32, "blend_overlapping_colors": "on", "thickness": 40, "maximum_height": 1200, "hue_interpolation": 360, "dynamic_hue_phase": "on", "color_symmetry": "on", "inside_color": "#ff8000", "outside_color": "#6450a0"})),
            ("Side B over the layer, Duration Averaging, 200 ms from 40 ms before", json!({"audio_layer": "sound", "side_options": "side_b", "composite": "on", "duration_averaging": "on", "audio_duration": 200, "audio_offset": -40, "maximum_height": 1000, "start_point": [5, 20], "end_point": [95, 90]})),
            ("1024 bands, hairlines 1 thick, softness 0, 2000 tall", json!({"audio_layer": "sound", "frequency_bands": 1024, "thickness": 1, "softness": 0, "maximum_height": 2000, "end_frequency": 4000})),
        ],
    );

    street(
        &mut t,
        &[
            ("as_added", json!({"audio_layer": "sound", "maximum_height": 600}), 0, "the music as it starts, 600 tall: white bars edged blue both ways along the middle, alone", gaps),
            ("over_the_street", json!({"audio_layer": "sound", "maximum_height": 600, "side_options": "side_a", "start_point": [5, 95], "end_point": [95, 95], "thickness": 4, "composite": "on"}), 48, "bars standing up from the foot of the street, over it, two seconds in", kept),
            ("neon_line", json!({"audio_layer": "sound", "display_options": "analog_lines", "side_options": "side_a", "maximum_height": 900, "thickness": 5, "softness": 80, "inside_color": "#fff0c0", "outside_color": "#ff8000", "composite": "on"}), 96, "an orange neon line over the street, four seconds in", kept),
            ("ring_of_dots", json!({"audio_layer": "sound", "use_polar_path": "on", "start_point": [50, 50], "display_options": "analog_dots", "frequency_bands": 96, "maximum_height": 2000, "thickness": 6, "hue_interpolation": 360, "inside_color": "#ff4040", "outside_color": "#ff4040", "composite": "on"}), 144, "a rainbow ring of dots round the middle, over the street, six seconds in", kept),
        ],
    );

    t.finish("D-420_audio_spectrum_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop's 30 frames' median is
/// "first"; the median of the loops after it is "again". With `B299_CPU` set, the processor draws.
#[test]
#[ignore = "B-299: a measurement, run deliberately with --release --ignored"]
fn b299_audio_spectrum_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B299_CPU").is_ok();
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
        ("Noise, then Audio Spectrum as added, listening to the music (64 bands, 90 ms)", Some(json!({"audio_layer": "sound"}))),
        ("Noise, then Audio Spectrum, 1024 bands to 4000 Hz, hairlines, 2000 tall", Some(json!({"audio_layer": "sound", "frequency_bands": 1024, "thickness": 1, "softness": 0, "maximum_height": 2000, "end_frequency": 4000}))),
        ("Noise, then Audio Spectrum, Analog Dots round the middle, 128 bands, averaged over 200 ms", Some(json!({"audio_layer": "sound", "use_polar_path": "on", "start_point": [50, 50], "display_options": "analog_dots", "frequency_bands": 128, "maximum_height": 800, "thickness": 8, "audio_duration": 200, "duration_averaging": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.audio_spectrum", &format!("{id}c"), p));
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
    let out = std::env::var("B299_OUT").map(PathBuf::from).unwrap_or_else(|_| repo("verification/B-299_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
