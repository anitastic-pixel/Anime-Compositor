//! B-300: D-421 Audio Waveform, after After Effects' Audio Waveform ("Generate" in
//! `docs/effects/EFFECTS.md`): the wave of a sound layer drawn as strokes, a line or dots along
//! a line or round a mask.
//!
//! Writes `verification/D-421_audio_waveform_table.md`.
//!
//! Every expected pixel is `Fixtures/audio_waveform/expected_audio_waveform.json`, written by
//! `tools/audio_waveform_reference.py` before this code existed and printed in document 25 as
//! FX-AWAVE-001 to 049. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-421 pictures/`, to the piece of music B-299 makes.

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

/// An Audio Waveform with the starting settings but for `change`, given as the file writes them.
fn waveform(change: J) -> Effect {
    let mut all = defaults();
    for (k, v) in change.as_object().unwrap() {
        all[k] = v.clone();
    }
    let n = |k: &str| all[k].as_f64().unwrap();
    let p = |k: &str| [all[k][0].as_f64().unwrap(), all[k][1].as_f64().unwrap()];
    let w = |k: &str| all[k].as_str().unwrap().to_string();
    Effect::AudioWaveform {
        audio_layer: all["audio_layer"].clone(),
        start_point: p("start_point"),
        end_point: p("end_point"),
        path: n("path"),
        displayed_samples: n("displayed_samples"),
        maximum_height: n("maximum_height"),
        audio_duration: n("audio_duration"),
        audio_offset: n("audio_offset"),
        thickness: n("thickness"),
        softness: n("softness"),
        random_seed: n("random_seed"),
        inside_color: w("inside_color"),
        outside_color: w("outside_color"),
        waveform_options: w("waveform_options"),
        display_options: w("display_options"),
        composite: w("composite"),
        levels: None,
        outline: None,
    }
}

/// The starting settings, as the window adds the effect.
fn defaults() -> J {
    json!({"audio_layer": "", "start_point": [10, 50], "end_point": [90, 50], "path": 0, "displayed_samples": 32,
        "maximum_height": 300, "audio_duration": 90, "audio_offset": 0, "thickness": 2, "softness": 50, "random_seed": 1,
        "inside_color": "#ffffff", "outside_color": "#3c8cff", "waveform_options": "mono", "display_options": "analog_lines",
        "composite": "off"})
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

fn is_waveform(e: &Effect) -> bool {
    matches!(e, Effect::AudioWaveform { .. })
}

/// The Audio Waveforms the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if is_waveform(&f.instance.effect)))
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
        "core.audio_waveform" => defaults(),
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
        let file = format!("fx_awave_{n:03}.json");
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
        let project = reference(|id| json!([fx("core.audio_waveform", &format!("b300-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Audio Waveform {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Audio Waveform {what} on three layers, frame {frame}, {}", quality.label()),
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

/// The street plain and with each shot, as numbered pictures in `verification/D-421 pictures/`.
fn street(t: &mut Table, shots: &[(&str, J, i32, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading("Pictures: the street, in `verification/D-421 pictures/`");
    let dir = repo("verification/D-421 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx("core.audio_waveform", "fx-0-0", p)]), *frame);
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
fn b300_audio_waveform() {
    let mut t = Table::new(
        "audio_waveform",
        "# D-421: Audio Waveform\n\nB-300, after After Effects' Audio Waveform: the samples of a sound \
         layer over Audio Duration from the frame's first sample plus Audio Offset (the layer's gain \
         applied, held to -1..1; Mono the channels' mean, Left channel 0, Right channel 1), cut into \
         Displayed Samples shares, each share's least and greatest drawn as a stroke from one to the \
         other (Digital), or one of them, picked by Random Seed, joined in a line (Analog Lines) or \
         dotted (Analog Dots), Maximum Height times the level from the line from Start to End Point \
         or round a mask (Path), each mark Beam's line (D-207), drawn as Audio Spectrum's (D-420). \
         Every expected pixel is `Fixtures/audio_waveform/expected_audio_waveform.json`, written by \
         `tools/audio_waveform_reference.py` before this code existed and printed in document 25 as \
         FX-AWAVE-001 to 049. Tolerance 2e-5. The least and greatest are worked out on the processor \
         and the marks drawn on the card, on Audio Spectrum's pass.\n",
    );

    t.heading("FX-AWAVE-001 to 049 (document 25)");
    t.fixtures("expected_audio_waveform.json");

    t.heading("The file");
    let all: Vec<String> = (1..=49).map(|n| format!("fx_awave_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_awave_002.json");
    t.row(
        "fx_awave_002.json is saved with every setting as written, the samples it heard not among them",
        &saved.to_string(),
        saved["audio_layer"] == "sound" && saved["displayed_samples"] == 8 && saved["display_options"] == "analog_lines" && saved.get("levels").is_none()
            && saved.get("outline").is_none() && saved.as_object().unwrap().len() == 16,
    );
    why(
        &mut t,
        &[
            ("fx_awave_034.json", "Audio Waveform's displayed samples runs from 1 to 4096, and this is 0."),
            ("fx_awave_035.json", "Audio Waveform's displayed samples runs from 1 to 4096, and this is 4097."),
            ("fx_awave_036.json", "Audio Waveform's maximum height runs from 0 to 10000, and this is -1."),
            ("fx_awave_037.json", "Audio Waveform's audio duration runs from 1 to 30000, and this is 0."),
            ("fx_awave_038.json", "Audio Waveform's audio offset runs from -30000 to 30000, and this is -30001."),
            ("fx_awave_039.json", "Audio Waveform's thickness runs from 0 to 10000, and this is -1."),
            ("fx_awave_040.json", "Audio Waveform's softness runs from 0 to 100, and this is 101."),
            ("fx_awave_041.json", "Audio Waveform's random seed runs from 0 to 100000, and this is -1."),
            ("fx_awave_042.json", "Audio Waveform's path runs from 0 to 1000, and this is 1001."),
            ("fx_awave_043.json", "Audio Waveform's end point runs from -1000 to 1000, and this is -1001."),
            ("fx_awave_044.json", "Audio Waveform's waveform options is \"mono\", \"left\" or \"right\", and this is \"stereo\"."),
            ("fx_awave_045.json", "Audio Waveform's display options is \"digital\", \"analog_lines\" or \"analog_dots\", and this is \"bars\"."),
            ("fx_awave_046.json", "Audio Waveform's composite is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_awave_047.json", "Audio Waveform's outside colour is written #rrggbb, and this is \"blue\"."),
            ("fx_awave_048.json", "Audio Waveform's audio layer is the name of a layer of this composition, and this is 5."),
        ],
    );
    let mut no_samples = defaults();
    no_samples.as_object_mut().unwrap().remove("displayed_samples");
    t.shape_refused("fx_awave_001.json", "an Audio Waveform with no `displayed_samples`", &no_samples.to_string());
    let mut worded = defaults();
    worded["random_seed"] = J::from("seven");
    t.shape_refused("fx_awave_001.json", "an Audio Waveform with a random seed in words", &worded.to_string());
    let mut one = defaults();
    one["start_point"] = json!([10]);
    t.shape_refused("fx_awave_001.json", "an Audio Waveform whose start point is one number", &one.to_string());

    t.heading("Commands");
    let mut document = t.load("fx_awave_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("displayed samples 0", set(waveform(json!({"audio_layer": "sound", "displayed_samples": 0})))),
            ("waveform options \"stereo\"", set(waveform(json!({"audio_layer": "sound", "waveform_options": "stereo"})))),
            ("audio layer 5", set(waveform(json!({"audio_layer": 5})))),
            ("random seed keyed to 100001", keys("random_seed", &[(0, &[1.0]), (4, &[100001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_awave_002.json",
        vec![
            ("16 samples as digital strokes, left channel, orange, over the layer", set(waveform(json!({"audio_layer": "sound", "displayed_samples": 16, "display_options": "digital", "waveform_options": "left", "inside_color": "#ff8000", "composite": "on", "maximum_height": 4})))),
            ("end point keyed from 90, 50 to 80, 20", keys("end_point", &[(0, &[90.0, 50.0]), (4, &[80.0, 20.0])])),
            ("random seed keyed from 1 to 9", keys("random_seed", &[(0, &[1.0]), (4, &[9.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_awave_002.json", 2), ("fx_awave_004.json", 0), ("fx_awave_005.json", 0), ("fx_awave_017.json", 0), ("fx_awave_028.json", 0), ("fx_awave_029.json", 0), ("fx_awave_031.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let none: Vec<u32> = [18, 19, 20].into_iter().chain(34..=49).collect();
    card_fixtures(&mut t, &mut gpu, 49, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("listening to the music, otherwise as it starts (32 points, an analog line along the middle, alone)", json!({"audio_layer": "sound"})),
            ("in silence, no Audio Layer (a flat line), thickness 10", json!({"thickness": 10})),
            ("Digital, 512 samples over 200 ms, 600 tall, hairlines, softness 0", json!({"audio_layer": "sound", "display_options": "digital", "displayed_samples": 512, "audio_duration": 200, "maximum_height": 600, "thickness": 1, "softness": 0})),
            ("Analog Dots, 128 samples, 800 tall, thickness 8, seed 9", json!({"audio_layer": "sound", "display_options": "analog_dots", "displayed_samples": 128, "maximum_height": 800, "thickness": 8, "random_seed": 9})),
            ("over the layer on a slant, 64 samples, thickness 30, orange to violet, from 40 ms before", json!({"audio_layer": "sound", "composite": "on", "displayed_samples": 64, "thickness": 30, "maximum_height": 900, "inside_color": "#ff8000", "outside_color": "#6450a0", "audio_offset": -40, "start_point": [5, 20], "end_point": [95, 90]})),
            ("4096 samples over 1000 ms, an analog line 2000 tall, softness 0", json!({"audio_layer": "sound", "displayed_samples": 4096, "audio_duration": 1000, "maximum_height": 2000, "softness": 0})),
        ],
    );

    street(
        &mut t,
        &[
            ("as_added", json!({"audio_layer": "sound", "maximum_height": 100, "thickness": 4}), 0, "the music as it starts, 100 tall: a white line edged blue zigzagging along the middle, alone", gaps),
            ("over_the_street", json!({"audio_layer": "sound", "maximum_height": 40, "displayed_samples": 120, "display_options": "digital", "audio_duration": 40, "thickness": 3, "start_point": [5, 88], "end_point": [95, 88], "composite": "on"}), 48, "digital strokes along the road, over the street, two seconds in", kept),
            ("neon_line", json!({"audio_layer": "sound", "displayed_samples": 240, "audio_duration": 20, "maximum_height": 80, "thickness": 5, "softness": 80, "inside_color": "#fff0c0", "outside_color": "#ff8000", "composite": "on"}), 96, "an orange neon wave over the street, 20 ms of the music, four seconds in", kept),
            ("slanted_dots", json!({"audio_layer": "sound", "display_options": "analog_dots", "displayed_samples": 48, "audio_duration": 30, "maximum_height": 60, "thickness": 10, "start_point": [10, 85], "end_point": [90, 15], "inside_color": "#ff4040", "outside_color": "#ff4040", "composite": "on"}), 144, "red dots on a slant up the street, over it, six seconds in", kept),
        ],
    );

    t.finish("D-421_audio_waveform_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop's 30 frames' median is
/// "first"; the median of the loops after it is "again". With `B300_CPU` set, the processor draws.
#[test]
#[ignore = "B-300: a measurement, run deliberately with --release --ignored"]
fn b300_audio_waveform_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B300_CPU").is_ok();
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
        ("Noise, then Audio Waveform as added, listening to the music (32 points, 90 ms)", Some(json!({"audio_layer": "sound"}))),
        ("Noise, then Audio Waveform, Digital, 1024 samples over 200 ms, hairlines, 300 tall", Some(json!({"audio_layer": "sound", "display_options": "digital", "displayed_samples": 1024, "audio_duration": 200, "thickness": 1, "softness": 0, "maximum_height": 300}))),
        ("Noise, then Audio Waveform, Analog Dots, 128 samples, 800 tall, thickness 8", Some(json!({"audio_layer": "sound", "display_options": "analog_dots", "displayed_samples": 128, "maximum_height": 800, "thickness": 8}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.audio_waveform", &format!("{id}c"), p));
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
    let out = std::env::var("B300_OUT").map(PathBuf::from).unwrap_or_else(|_| repo("verification/B-300_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
