//! B-301: D-422 Light Burst, our name for CC Light Burst 2.5 ("Generate" in
//! `docs/effects/EFFECTS.md`): rays bursting outward from a point, taken from the image.
//!
//! Writes `verification/D-422_light_burst_table.md`.
//!
//! Every expected pixel is `Fixtures/light_burst/expected_light_burst.json`, written by
//! `tools/light_burst_reference.py` before this code existed and printed in document 25 as
//! FX-BURST-001 to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-422 pictures/`.

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
        "core.light_burst" => json!({"center": [50, 50], "intensity": 100, "ray_length": 50, "burst": "straight", "set_color": "off", "color": "#ffffff"}),
        "core.light_rays" => json!({"center": [50, 50], "length": 50, "threshold": 0, "intensity": 1, "color": "#ffffff"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b301-{id}"), p)]));
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

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures
/// into `verification/{d} pictures/`; a row each that it draws cleanly and changes what it says
/// it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed", distance(&bytes, &before).1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn changed(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > 0
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

// --- Light Burst ----------------------------------------------------------------------------

fn is_burst(e: &Effect) -> bool {
    matches!(e, Effect::LightBurst { .. })
}

fn burst(center: [f64; 2], intensity: f64, ray_length: f64, kind: &str, set_color: &str, color: &str) -> Effect {
    Effect::LightBurst {
        center,
        intensity,
        ray_length,
        burst: kind.to_string(),
        set_color: set_color.to_string(),
        color: color.to_string(),
    }
}

#[test]
fn b301_light_burst() {
    let mut t = Table::new(
        "light_burst",
        "# D-422: Light Burst\n\nB-301, our name for CC Light Burst 2.5: the whole layer is the light, \
         zoomed outward about the Center by Spin & Zoom Blur's straight, fading or centered zoom of \
         amount Ray Length, and added on as Light Rays' rays are, Intensity / 100 times; with Set \
         Color on, the rays are the Color at their covering. The manual gives no formula, ranges or \
         defaults, so those are ours. Every expected pixel is \
         `Fixtures/light_burst/expected_light_burst.json`, written by \
         `tools/light_burst_reference.py` before this code existed and printed in document 25 as \
         FX-BURST-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-BURST-001 to 030 (document 25)");
    t.fixtures("expected_light_burst.json");

    t.heading("How far it reaches");
    let got = burst([0.0, 0.0], 2000.0, 100.0, "straight", "off", "#ffffff").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: rays past the layer's edge are cut", &got.to_string(), got == 0);
    let mut draft = burst([30.0, 70.0], 250.0, 80.0, "fade", "on", "#ff8000");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the length and the centre are shares of the drawing",
        &format!("{draft:?}"),
        draft == burst([30.0, 70.0], 250.0, 80.0, "fade", "on", "#ff8000"),
    );

    t.heading("The file");
    // fx_burst_012.json's capitals are saved in small letters, the row after next.
    let all: Vec<String> = files("fx_burst", 30).into_iter().filter(|f| f != "fx_burst_012.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_burst_020.json");
    t.row(
        "fx_burst_020.json is saved with all six settings",
        &saved.to_string(),
        saved["center"] == json!([25, 75])
            && saved["intensity"] == 300
            && saved["ray_length"] == 50
            && saved["burst"] == "fade"
            && saved["set_color"] == "on"
            && saved["color"] == "#40c0ff",
    );
    let saved = t.saved_parameters("fx_burst_012.json");
    t.row("fx_burst_012.json's colour written #FF8000 is read and saved in small letters", &saved.to_string(), saved["color"] == "#ff8000");
    why(
        &mut t,
        &[
            ("fx_burst_022.json", "Light Burst's ray length runs from 0 to 100, and this is 101."),
            ("fx_burst_023.json", "Light Burst's intensity runs from 0 to 2000, and this is -1."),
            ("fx_burst_024.json", "Light Burst's intensity runs from 0 to 2000, and this is 2001."),
            ("fx_burst_026.json", "Light Burst's center runs from -1000 to 1000, and this is -1001."),
            ("fx_burst_027.json", "Light Burst's burst is \"straight\", \"fade\" or \"center\", and this is \"sideways\"."),
            ("fx_burst_028.json", "Light Burst's set colour is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_burst_029.json", "Light Burst's colour is written #rrggbb, and this is \"#12345\"."),
            ("fx_burst_030.json", "Light Burst's colour is written #rrggbb, and this is \"orange\"."),
        ],
    );
    let full = r##""intensity": 100, "ray_length": 50, "burst": "straight", "set_color": "off", "color": "#ffffff""##;
    t.shape_refused("fx_burst_001.json", "a Light Burst with no `center`", &format!("{{{full}}}"));
    t.shape_refused("fx_burst_001.json", "a Light Burst with one number for its centre", &format!("{{\"center\": [50], {full}}}"));
    t.shape_refused(
        "fx_burst_001.json",
        "a Light Burst with its burst as a number",
        r##"{"center": [50, 50], "intensity": 100, "ray_length": 50, "burst": 1, "set_color": "off", "color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_burst_001.json",
        "a Light Burst with no `set_color`",
        r##"{"center": [50, 50], "intensity": 100, "ray_length": 50, "burst": "straight", "color": "#ffffff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_burst_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("ray length 101", set(burst([50.0, 50.0], 100.0, 101.0, "straight", "off", "#ffffff"))),
            ("intensity 2001", set(burst([50.0, 50.0], 2001.0, 50.0, "straight", "off", "#ffffff"))),
            ("centre 50, 1001", set(burst([50.0, 1001.0], 100.0, 50.0, "straight", "off", "#ffffff"))),
            ("burst \"sideways\"", set(burst([50.0, 50.0], 100.0, 50.0, "sideways", "off", "#ffffff"))),
            ("set colour \"yes\"", set(burst([50.0, 50.0], 100.0, 50.0, "straight", "yes", "#ffffff"))),
            ("colour \"orange\"", set(burst([50.0, 50.0], 100.0, 50.0, "straight", "on", "orange"))),
            ("intensity keyed to 3000", keys("intensity", &[(0, &[100.0]), (4, &[3000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_burst_001.json",
        vec![
            ("the tops: intensity 2000, ray length 100, centre 1000, 1000, centered", set(burst([1000.0, 1000.0], 2000.0, 100.0, "center", "on", "#000000"))),
            ("the bottoms: intensity 0, ray length 0, centre -1000, -1000, fade", set(burst([-1000.0, -1000.0], 0.0, 0.0, "fade", "off", "#ffffff"))),
            ("ray length keyed from 0 to 100", keys("ray_length", &[(0, &[0.0]), (4, &[100.0])])),
            ("centre keyed from 50, 50 to 0, 0", keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_burst_001.json", 0),
        ("fx_burst_004.json", 0),
        ("fx_burst_005.json", 0),
        ("fx_burst_009.json", 0),
        ("fx_burst_016.json", 2),
        ("fx_burst_020.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_burst", 30, &[2, 22, 23, 24, 25, 26, 27, 28, 29, 30], is_burst);
    card_reference(
        &mut t,
        &mut gpu,
        "Light Burst",
        "core.light_burst",
        &[
            ("as it starts (straight, intensity 100, ray length 50)", json!({})),
            ("as the manual's tutorial (intensity 1200, ray length 35)", json!({"intensity": 1200, "ray_length": 35})),
            ("fade, Set Color on #ff8000, intensity 300, round 30, 40", json!({"burst": "fade", "set_color": "on", "color": "#ff8000", "intensity": 300, "center": [30, 40]})),
            ("center, ray length 100, round -10, 50, off the edge", json!({"burst": "center", "ray_length": 100, "center": [-10, 50]})),
        ],
        is_burst,
    );

    t.heading("Straight with Set Color off is Light Rays with no bright test");
    let dir = repo("verification/D-422 pictures");
    fs::create_dir_all(&dir).unwrap();
    for (what, b, r) in [
        ("as it starts", json!({}), json!({})),
        ("ray length 80 round 30, 70", json!({"ray_length": 80, "center": [30, 70]}), json!({"length": 80, "center": [30, 70]})),
    ] {
        let (a, _) = picture(&dir, json!([fx("core.light_burst", "fx-0-0", &b)]));
        let (c, _) = picture(&dir, json!([fx("core.light_rays", "fx-0-0", &r)]));
        let d = distance(&a, &c);
        t.row(
            &format!("the street, Light Burst {what} against Light Rays at threshold 0 and intensity 1, the same length and centre"),
            &format!("largest difference {} of 255, {} pixels differ", d.0, d.1),
            d.0 == 0,
        );
    }

    street(
        &mut t,
        "D-422",
        "core.light_burst",
        &[
            ("as_added", json!({}), "as it starts: every house streaked outward from the middle in its own colours, and brightened", changed),
            ("tutorial", json!({"intensity": 1200, "ray_length": 35}), "the manual's tutorial settings, intensity 1200, ray length 35: the street blown out in white streaks", changed),
            ("fade", json!({"burst": "fade", "ray_length": 80}), "fade, ray length 80: the streaks fade as they go out", changed),
            ("center", json!({"burst": "center", "ray_length": 80}), "center, ray length 80: the streaks run both in and out from each house", changed),
            ("set_color", json!({"set_color": "on", "color": "#ff8000", "intensity": 150, "center": [25, 40]}), "Set Color on #ff8000, intensity 150, round 25, 40: the street covers its whole frame, so its light is even and the picture is washed orange with no streaks (the rays take the layer's shape only where it is see-through, as FX-BURST-009's lamp shows)", changed),
        ],
    );

    t.finish("D-422_light_burst_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B301_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-301: a measurement, run deliberately with --release --ignored"]
fn b301_light_burst_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B301_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Light Burst as added (straight, ray length 50)", Some(("core.light_burst", json!({})))),
        ("Noise, then Light Burst, fade, ray length 100", Some(("core.light_burst", json!({"burst": "fade", "ray_length": 100})))),
        ("Noise, then Light Burst, center, Set Color on", Some(("core.light_burst", json!({"burst": "center", "set_color": "on", "color": "#ff8000"})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B301_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-301_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
