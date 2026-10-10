//! B-326: D-446 Curl Noise, after After Effects' Curl Noise (26.3 beta; "Noise & Grain" in
//! `docs/effects/EFFECTS.md`): swirling grey flow lines along the curl of a smooth noise.
//!
//! Writes `verification/D-446_curl_noise_table.md`.
//!
//! Every expected pixel is `Fixtures/curl_noise/expected_curl_noise.json`, written by
//! `tools/curl_noise_reference.py` before this code existed and printed in document 25 as
//! FX-CURL-001 to 044. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-446 pictures/`.

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

/// A Curl Noise: [speed, direction, size, offset x, offset y, evolution, turbulence speed, swirl,
/// density, smoothness, vertical bias, sample count, sample radius, flow softness, edge
/// definition, flow falloff, contrast, brightness], [source, view, clip HDR results, channel].
fn curl(n: [f64; 18], w: [&str; 4]) -> Effect {
    let [speed, direction, size, ox, oy, evolution, turbulence_speed, swirl, density, smoothness, vertical_bias, sample_count, sample_radius, flow_softness, edge_definition, flow_falloff, contrast, brightness] = n;
    let [source, view, clip_hdr_results, channel] = w.map(str::to_string);
    Effect::CurlNoise {
        source,
        speed,
        direction,
        size,
        offset: [ox, oy],
        evolution,
        turbulence_speed,
        swirl,
        density,
        smoothness,
        vertical_bias,
        sample_count,
        sample_radius,
        flow_softness,
        edge_definition,
        flow_falloff,
        view,
        contrast,
        brightness,
        clip_hdr_results,
        channel,
        frame: 0,
        float: false,
    }
}

const ADDED: [f64; 18] = [10.0, 0.0, 100.0, 0.0, 0.0, 0.0, 20.0, 45.0, 0.0, 50.0, 50.0, 12.0, 30.0, 20.0, 50.0, 0.0, 100.0, 0.0];
const WORDS: [&str; 4] = ["internal", "final_render", "on", "rgb"];

fn numbers(change: &[(usize, f64)]) -> [f64; 18] {
    let mut n = ADDED;
    for &(i, v) in change {
        n[i] = v;
    }
    n
}

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 4] {
    let mut w = WORDS;
    for &(i, v) in change {
        w[i] = v;
    }
    w
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

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
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

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.curl_noise" => json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
            "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
            "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
            "clip_hdr_results": "on", "channel": "rgb"}),
        "core.noise" => json!({}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out, so the card is not asked; those in `cpu` the card
/// refuses whole, so the processor draws them and says so.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], cpu: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        if cpu.contains(&n) {
            t.row(
                &format!("{file}, frames 0 to 4 at Full and Draft: refused by the card, drawn by the processor"),
                &format!("largest difference {largest} of 255; refused: {refused}"),
                largest <= 1 && refused,
            );
            continue;
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b326-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at `frame` of a two-second shot, drawn, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

type Shot<'a> = (&'a str, J, i32, &'a str, fn(&[u8], &[u8]) -> bool);

/// The street drawn plain and with each of `shots` of `type_id` at its frame, written as numbered
/// pictures into `verification/{d} pictures/`; a row each that it draws cleanly and changes what
/// it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[Shot]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, frame {frame}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed of {}, the largest by {} of 255", distance(&bytes, &before).1, TOWN.0 * TOWN.1, distance(&bytes, &before).0),
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

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Curl Noise -----------------------------------------------------------------------------

fn is_curl(e: &Effect) -> bool {
    matches!(e, Effect::CurlNoise { .. })
}

/// Most of the street changed.
fn covers(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > a.len() / 4 / 2
}

/// As `covers`, and every pixel a grey: its three channels equal.
fn greys(a: &[u8], b: &[u8]) -> bool {
    covers(a, b) && a.chunks_exact(4).all(|p| p[0] == p[1] && p[1] == p[2])
}

/// As `covers`, and red and green differ in most pixels: the flow drawn as colours.
fn coloured(a: &[u8], b: &[u8]) -> bool {
    covers(a, b) && a.chunks_exact(4).filter(|p| p[0].abs_diff(p[1]) > 8).count() > a.len() / 4 / 2
}

/// Holes: a tenth of the street or more less than half covered, and the colours of the street
/// kept where it still shows.
fn holes(a: &[u8], b: &[u8]) -> bool {
    let thin = a.chunks_exact(4).filter(|p| p[3] < 128).count();
    let kept = a.chunks_exact(4).zip(b.chunks_exact(4)).filter(|(p, _)| p[3] >= 32).all(|(p, q)| (0..3).all(|c| p[c].abs_diff(q[c]) <= 2));
    thin > a.len() / 4 / 10 && kept
}

#[test]
fn b326_curl_noise() {
    let mut t = Table::new(
        "curl_noise",
        "# D-446: Curl Noise\n\nB-326, after After Effects' Curl Noise (26.3 beta): a four-octave smooth \
         value noise Size pixels across, drifting Speed towards Direction and changing with Evolution \
         and Turbulence Speed; its slope turned a right angle (plus Swirl times the noise) is the \
         flow, which neither gathers nor spreads at Swirl 0, scaled across or down by Vertical Bias. \
         From each pixel the flow is followed both ways, Sample Count steps over Sample Radius pixels, \
         streaking a seed noise (blocky by Edge Definition) into grey flow lines, softened back \
         towards the noise by Flow Softness and faded where the noise is low by Flow Falloff. View \
         shows the lines, the noise, or the flow as colours; Contrast and Brightness follow, held in \
         0 and 1 unless Clip HDR Results is off in a Float composition; Channel puts the grey into \
         the colour, one channel or the covering. Source This Layer and Other Layer are refused in \
         a sentence, not built. The rule is this program's own; Adobe publishes none. Every expected \
         pixel is `Fixtures/curl_noise/expected_curl_noise.json`, written by \
         `tools/curl_noise_reference.py` before this code existed and printed in document 25 as \
         FX-CURL-001 to 044. Tolerance 2e-5.\n",
    );

    t.heading("FX-CURL-001 to 044 (document 25)");
    t.fixtures("expected_curl_noise.json");

    t.heading("The file");
    let all = files("fx_curl", 44);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_curl_032.json");
    t.row(
        "fx_curl_032.json is saved with its words and numbers as written, and no frame or depth",
        &saved.to_string(),
        saved["channel"] == "green" && saved["view"] == "final_render" && saved["swirl"] == -200 && saved["sample_radius"] == 6.5
            && saved["offset"] == json!([0, 0]) && saved.get("frame").is_none() && saved.get("float").is_none(),
    );
    let saved = t.saved_parameters("fx_curl_029.json");
    t.row("fx_curl_029.json is saved with Sample Radius's keys kept", &saved["sample_radius"].to_string(), saved["sample_radius"]["keyframes"][1]["value"] == json!(8));
    why(
        &mut t,
        &[
            ("fx_curl_033.json", "Curl Noise's This Layer source is not built yet: it makes its own noise only, so set Source to Internal."),
            ("fx_curl_034.json", "Curl Noise's Other Layer source is not built yet: it makes its own noise only, so set Source to Internal."),
            ("fx_curl_035.json", "Curl Noise's source is \"internal\", \"this_layer\" or \"other_layer\", and this is \"noise\"."),
            ("fx_curl_036.json", "Curl Noise's size runs from 1 to 1000, and this is 0.5."),
            ("fx_curl_037.json", "Curl Noise's speed runs from 0 to 100, and this is 101."),
            ("fx_curl_038.json", "Curl Noise's sample count runs from 3 to 24, and this is 2."),
            ("fx_curl_039.json", "Curl Noise's sample radius runs from 0 to 200, and this is 201."),
            ("fx_curl_040.json", "Curl Noise's swirl runs from -360 to 360, and this is 400."),
            ("fx_curl_041.json", "Curl Noise's view is one of final_render, input_noise, curl_generation, and this is \"lines\"."),
            ("fx_curl_042.json", "Curl Noise's channel is one of rgb, red, green, blue, alpha, and this is \"Red\"."),
            ("fx_curl_043.json", "Curl Noise's clip HDR results is \"on\" or \"off\", and this is \"yes\"."),
        ],
    );
    let p = r#""source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0, "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30, "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "contrast": 100, "brightness": 0, "clip_hdr_results": "on""#;
    t.shape_refused("fx_curl_001.json", "a Curl Noise with no `view`", &format!("{{{p}, \"channel\": \"rgb\"}}"));
    t.shape_refused("fx_curl_001.json", "a Curl Noise whose channel is a number", &format!("{{{p}, \"view\": \"final_render\", \"channel\": 1}}"));
    t.shape_refused(
        "fx_curl_001.json",
        "a Curl Noise whose offset has one number",
        &format!("{{{}, \"offset\": [3], \"view\": \"final_render\", \"channel\": \"rgb\"}}", p.replace(r#""offset": [0, 0], "#, "")),
    );

    t.heading("Commands");
    let mut document = t.load("fx_curl_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("size 0.5", set(curl(numbers(&[(2, 0.5)]), WORDS))),
            ("sample count 25", set(curl(numbers(&[(11, 25.0)]), WORDS))),
            ("source This Layer", set(curl(ADDED, words(&[(0, "this_layer")])))),
            ("view \"lines\"", set(curl(ADDED, words(&[(1, "lines")])))),
            ("channel \"Red\"", set(curl(ADDED, words(&[(3, "Red")])))),
            ("density keyed to 150", keys("density", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_curl_001.json",
        vec![
            (
                "speed 35, direction 200, size 6, offset 2 and -1, evolution 45, turbulence 60, swirl -200, density -30, smoothness 20, bias 70, 9 samples, radius 6.5, softness 10, edges 30, falloff 40, contrast 150, brightness -5, Curl Generation, clip off, green",
                set(curl([35.0, 200.0, 6.0, 2.0, -1.0, 45.0, 60.0, -200.0, -30.0, 20.0, 70.0, 9.0, 6.5, 10.0, 30.0, 40.0, 150.0, -5.0], ["internal", "curl_generation", "off", "green"])),
            ),
            ("sample radius keyed from 0 to 8", keys("sample_radius", &[(0, &[0.0]), (4, &[8.0])])),
            ("offset keyed from 0, 0 to 5, 3", keys("offset", &[(0, &[0.0, 0.0]), (4, &[5.0, 3.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_curl_001.json", 4),
        ("fx_curl_002.json", 2),
        ("fx_curl_004.json", 0),
        ("fx_curl_026.json", 0),
        ("fx_curl_031.json", 0),
        ("fx_curl_032.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-CURL-033 to 044 are refused, so the card is not asked; FX-CURL-024 is a Float
    // composition, which the card refuses whole (D-319), so the processor draws it.
    let none: Vec<u32> = (33..=44).collect();
    card_fixtures(&mut t, &mut gpu, "fx_curl", 44, &none, &[24], is_curl);
    card_reference(
        &mut t,
        &mut gpu,
        "Curl Noise",
        "core.curl_noise",
        &[
            ("as added (size 100, radius 30, 12 samples)", json!({})),
            ("size 20, Curl Generation, swirl 120, vertical bias 70", json!({"size": 20, "view": "curl_generation", "swirl": 120, "vertical_bias": 70})),
            (
                "size 40, radius 12, 8 samples, edges 100, falloff 60, contrast 200, brightness -10, channel Alpha",
                json!({"size": 40, "sample_radius": 12, "sample_count": 8, "edge_definition": 100, "flow_falloff": 60, "contrast": 200, "brightness": -10, "channel": "alpha"}),
            ),
            (
                "radius keyed 2 to 40 and evolution keyed 0 to 720, direction 135, speed 40, smoothness 10, channel Blue",
                json!({"sample_radius": keyed(&[(0, json!(2)), (239, json!(40))]), "evolution": keyed(&[(0, json!(0)), (239, json!(720))]),
                    "direction": 135, "speed": 40, "smoothness": 10, "channel": "blue"}),
            ),
        ],
        is_curl,
    );

    street(
        &mut t,
        "D-446",
        "core.curl_noise",
        &[
            ("as_added", json!({}), 0, "as added: soft swirling grey flow over the whole street", greys),
            (
                "fine_lines",
                json!({"size": 30, "sample_radius": 20, "sample_count": 16, "flow_softness": 0, "edge_definition": 100}),
                0,
                "size 30, radius 20, 16 samples, softness 0, edges 100: fine streaked flow lines",
                greys,
            ),
            ("curl_generation", json!({"size": 60, "view": "curl_generation"}), 0, "Curl Generation, size 60: the flow as red and green, the noise as blue", coloured),
            ("alpha", json!({"size": 40, "channel": "alpha", "contrast": 250}), 0, "Channel Alpha, size 40, contrast 250: the street shows through the light parts, holes in the dark", holes),
            ("frame_24", json!({}), 24, "as added at frame 24: the flow has drifted up and changed", greys),
        ],
    );

    t.finish("D-446_curl_noise_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B326_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-326: a measurement, run deliberately with --release --ignored"]
fn b326_curl_noise_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B326_CPU").is_ok();
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
        ("Noise, then Curl Noise as added (radius 30, 12 samples)", Some(json!({}))),
        ("Noise, then Curl Noise Input Noise (no lines)", Some(json!({"view": "input_noise"}))),
        ("Noise, then Curl Noise radius 100, 24 samples (the longest lines)", Some(json!({"sample_radius": 100, "sample_count": 24}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.curl_noise", &format!("{id}c"), p));
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
    let out = std::env::var("B326_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-326_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
