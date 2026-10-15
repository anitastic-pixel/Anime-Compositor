//! B-387: D-507, after CycoreFX's CC Spotlight (Perspective): a spotlight cone shining down on
//! the layer. The CycoreFX manual names the controls but publishes no formula and no settings
//! when added: the rule and those settings are ours, stated in `tools/cc_spotlight_reference.py`.
//! The four Gel renders are D-519's (`tests/b399_cc_spotlight_gel.rs`).
//!
//! Writes `verification/D-507_cc_spotlight_table.md` and draws pictures into
//! `verification/D-507 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_spotlight/expected_cc_spotlight.json`, written by the
//! tool before this code existed and printed in document 25 as FX-SPOT-001 to 027. Tolerance
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
    for n in 1..=27 {
        let file = format!("fx_spot_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::CcSpotlight { .. }));
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
        let project = reference(|id| json!([fx(&format!("b387-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Spotlight {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::CcSpotlight { .. }));
                t.row(
                    &format!("the reference shot, CC Spotlight {what} on three layers, frame {frame}, {}", quality.label()),
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


/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}


/// A CC Spotlight with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"from": [25, 25], "to": [50, 50], "height": 50, "cone_angle": 40, "edge_softness": 20,
        "color": "#ffffff", "intensity": 100, "render": "light_shadow"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_spotlight", "enabled": true, "parameters": all})
}

#[test]
fn b387_cc_spotlight() {
    let mut t = Table::new(
        "cc_spotlight",
        "# D-507: CC Spotlight\n\nB-387, after CycoreFX's CC Spotlight (Perspective): a spotlight \
         cone shining down on the layer. The CycoreFX manual names the controls but publishes no \
         formula and no settings when added, so the rule and those settings are ours, stated in \
         `tools/cc_spotlight_reference.py`: a lamp at From, Height above the drawing, pointed at \
         To; a pixel's share of the light is 1 inside the inner angle, 0 outside half the Cone \
         Angle, smoothstep between; Light Only, Light Add, Light Add+ and Light Shadow use it. \
         The four Gel renders are D-519's. Every expected pixel is \
         `Fixtures/cc_spotlight/expected_cc_spotlight.json`, written by the tool before this code \
         existed and printed in document 25 as FX-SPOT-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-SPOT-001 to 027 (document 25)");
    t.fixtures_numbered("expected_cc_spotlight.json", 1..=27);

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_spot_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_spot_009.json");
    t.row(
        "fx_spot_009.json is saved with its eight settings, an orange light, render light add, From [25, 25]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8
            && saved["color"] == "#ff8000"
            && saved["render"] == "light_add"
            && saved["from"] == json!([25, 25]),
    );
    for (file, want) in [
        ("fx_spot_019.json", "from"),
        ("fx_spot_020.json", "height"),
        ("fx_spot_021.json", "cone angle"),
        ("fx_spot_022.json", "edge softness"),
        ("fx_spot_023.json", "intensity"),
        ("fx_spot_024.json", "render"),
        ("fx_spot_025.json", "render"),
        ("fx_spot_026.json", "colour"),
        ("fx_spot_027.json", "colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_spot_001.json",
        "a CC Spotlight whose height is a word",
        r##"{"from": [25, 25], "to": [50, 50], "height": "high", "cone_angle": 40, "edge_softness": 20, "color": "#ffffff", "intensity": 100, "render": "light_shadow"}"##,
    );
    t.shape_refused(
        "fx_spot_001.json",
        "a CC Spotlight whose From is one number",
        r##"{"from": 25, "to": [50, 50], "height": 50, "cone_angle": 40, "edge_softness": 20, "color": "#ffffff", "intensity": 100, "render": "light_shadow"}"##,
    );

    t.heading("On the card (D-507)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[19, 20, 21, 22, 23, 24, 25, 26, 27]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (light shadow)", json!({}), 3),
            ("light add, orange, intensity 250, edge softness 0", json!({"render": "light_add", "color": "#ff8000", "intensity": 250, "edge_softness": 0}), 3),
            ("light add plus, low and wide", json!({"render": "light_add_plus", "height": 10, "cone_angle": 90}), 3),
            ("light only, From keyed", json!({"render": "light_only", "from": {"base": [0, 0], "keyframes": [{"frame": 0, "value": [0, 0], "interp": "linear"}, {"frame": 239, "value": [100, 0], "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-507 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-507 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, "1_left.png", (480, 270), json!([])).0;
    for (name, p) in [
        ("1_as_added", json!({})),
        ("2_light_only", json!({"render": "light_only"})),
        ("3_light_add_orange", json!({"render": "light_add", "color": "#ffa040", "intensity": 150})),
        ("4_low_and_wide", json!({"height": 10, "cone_angle": 70})),
        ("5_hard_edge", json!({"edge_softness": 0, "from": [50, 50]})),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let middle = at(&bytes, (240, 135));
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed, the middle lit"),
            &format!("{} pixels changed; middle {middle:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0 && middle[3] > 0,
        );
    }
    t.finish("D-507_cc_spotlight_table.md");
}


/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B387_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-387: a measurement, run deliberately with --release --ignored"]
fn b387_cc_spotlight_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B387_CPU").is_ok();
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
        ("Noise, then CC Spotlight as added (light shadow)", Some(json!({}))),
        ("Noise, then CC Spotlight, light add, orange, intensity 250", Some(json!({"render": "light_add", "color": "#ff8000", "intensity": 250}))),
        ("Noise, then CC Spotlight, light add plus, low and wide", Some(json!({"render": "light_add_plus", "height": 10, "cone_angle": 90}))),
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
    let out = std::env::var("B387_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-387_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
