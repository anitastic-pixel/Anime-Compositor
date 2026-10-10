//! B-302: D-423 Light Rays with CC Light Rays' controls ("Generate" in
//! `docs/effects/EFFECTS.md`): rays streaming from a round or square light source.
//!
//! Writes `verification/D-423_light_rays_cc_table.md`.
//!
//! Every expected pixel is `Fixtures/light_rays_cc/expected_light_rays_cc.json`, written by
//! `tools/light_rays_cc_reference.py` before this code existed and printed in document 25 as
//! FX-RAYSCC-001 to 039. Tolerance 2e-5. Nothing here is a snapshot of a run. A Light Rays saved
//! before D-423 opens as D-124's form, saves as written and draws as before: `Fixtures/light_rays`
//! keeps pinning it (tests/b67_light_rays.rs, tests/b76_gpu_fx.rs).
//!
//! Also draws the street into `verification/D-423 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, same_json, saved, set, town, Table, MAIN, TOWN};
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
        "core.light_rays" => json!({"intensity": 100, "center": [50, 50], "radius": 50, "warp_softness": 50, "shape": "round", "direction": 0, "color_from_source": "on", "allow_brightening": "on", "color": "#ffffff", "transfer_mode": "none"}),
        // D-124's form, as an older project holds it.
        "old" => json!({"center": [50, 50], "length": 50, "threshold": 0, "intensity": 1, "color": "#ffffff"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    let type_id = if type_id == "old" { "core.light_rays" } else { type_id };
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
        let project = reference(|id| json!([fx(type_id, &format!("b302-{id}"), p)]));
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

// --- Light Rays, CC's controls ---------------------------------------------------------------

fn is_cc(e: &Effect) -> bool {
    matches!(e, Effect::CcLightRays { .. })
}

/// A Light Rays with CC's controls: `n` intensity, radius, warp softness, direction; `w` shape,
/// colour from source, allow brightening, colour, transfer mode.
fn cc(center: [f64; 2], n: [f64; 4], w: [&str; 5]) -> Effect {
    Effect::CcLightRays {
        intensity: n[0],
        center,
        radius: n[1],
        warp_softness: n[2],
        shape: w[0].to_string(),
        direction: n[3],
        color_from_source: w[1].to_string(),
        allow_brightening: w[2].to_string(),
        color: w[3].to_string(),
        transfer_mode: w[4].to_string(),
    }
}

const MID: [f64; 2] = [50.0, 50.0];
const START: [f64; 4] = [100.0, 50.0, 50.0, 0.0];
const WORDS: [&str; 5] = ["round", "on", "on", "#ffffff", "none"];

#[test]
fn b302_light_rays_cc() {
    let mut t = Table::new(
        "light_rays_cc",
        "# D-423: Light Rays with CC Light Rays' controls\n\nB-302: the light is the layer times \
         each pixel's share of a round or square source of Radius pixels about the Center (the \
         square turned by Direction), white at its covering with Color from Source off; the rays \
         are that light through Spin & Zoom Blur's zoom at amount 100, then its spin of Warp \
         Softness / 10 degrees, Intensity / 100 times (held at 1 with Allow Brightening off), laid \
         on by the Transfer Mode: None over the layer, Add as D-124's last step, Screen, Lighten. \
         The manual gives no formula, ranges or defaults, so those are ours. Every expected pixel \
         is `Fixtures/light_rays_cc/expected_light_rays_cc.json`, written by \
         `tools/light_rays_cc_reference.py` before this code existed and printed in document 25 \
         as FX-RAYSCC-001 to 039. Tolerance 2e-5. A Light Rays saved before D-423 opens as \
         D-124's form and draws as before.\n",
    );

    t.heading("FX-RAYSCC-001 to 039 (document 25)");
    t.fixtures("expected_light_rays_cc.json");

    t.heading("Older projects: D-124's form, as before");
    let old_root = repo("Fixtures/light_rays");
    for n in 1..=24 {
        let file = format!("fx_rays_{n:03}.json");
        let text = fs::read_to_string(old_root.join(&file)).unwrap();
        // fx_rays_010.json's colour is written #FF8000 and is saved in small letters, as before D-423.
        let old: J = serde_json::from_str(&text.replace("#FF8000", "#ff8000")).unwrap();
        let loaded = persist::load_str(&text).unwrap();
        let effect = &loaded.document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect;
        let is_old = matches!(effect, Effect::LightRays { .. });
        let kept = same_json(&saved(&loaded), &old);
        t.row(
            &format!("{file} (D-124's length and threshold) opens as D-124's form and is saved as it was written (a colour in small letters), with no new settings"),
            &format!("{}; saved {}", if is_old { "D-124's form" } else { "the new form" }, if kept { "the same" } else { "differently" }),
            is_old && kept,
        );
    }
    let dir = repo("verification/D-423 pictures");
    fs::create_dir_all(&dir).unwrap();
    for (what, new, old) in [
        ("as it starts", json!({"radius": 10000, "warp_softness": 0, "transfer_mode": "add"}), json!({"length": 100})),
        ("round 30, 70", json!({"radius": 10000, "warp_softness": 0, "transfer_mode": "add", "center": [30, 70]}), json!({"length": 100, "center": [30, 70]})),
    ] {
        let (a, _) = picture(&dir, json!([fx("core.light_rays", "fx-0-0", &new)]));
        let (c, _) = picture(&dir, json!([fx("old", "fx-0-0", &old)]));
        let d = distance(&a, &c);
        t.row(
            &format!("the street, {what}: the new form with a source over the whole layer, no warp, Add, against D-124's form at length 100, threshold 0, intensity 1"),
            &format!("largest difference {} of 255, {} pixels differ", d.0, d.1),
            d.0 == 0,
        );
    }

    t.heading("How far it reaches");
    let got = cc([0.0, 0.0], [2000.0, 10000.0, 1000.0, 0.0], ["square", "on", "on", "#ffffff", "add"]).bounds_expansion();
    t.row("it grows the drawing's bounds by nothing: rays past the layer's edge are cut", &got.to_string(), got == 0);
    let mut draft = cc([30.0, 70.0], [250.0, 80.0, 120.0, 30.0], ["square", "off", "off", "#ff8000", "screen"]);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves the radius alone: the centre is a share of the drawing, the warp and direction angles",
        &format!("{draft:?}"),
        draft == cc([30.0, 70.0], [250.0, 40.0, 120.0, 30.0], ["square", "off", "off", "#ff8000", "screen"]),
    );

    t.heading("The file");
    // fx_rayscc_011.json's capitals are saved in small letters, the row after next.
    let all: Vec<String> = files("fx_rayscc", 39).into_iter().filter(|f| f != "fx_rayscc_011.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_rayscc_026.json");
    t.row(
        "fx_rayscc_026.json is saved with all ten settings",
        &saved.to_string(),
        saved["intensity"] == 180
            && saved["center"] == json!([40, 60])
            && saved["radius"] == 4
            && saved["warp_softness"] == 120
            && saved["shape"] == "square"
            && saved["direction"] == 30
            && saved["color_from_source"] == "off"
            && saved["allow_brightening"] == "on"
            && saved["color"] == "#40c0ff"
            && saved["transfer_mode"] == "screen"
            && saved.as_object().unwrap().len() == 10,
    );
    let saved = t.saved_parameters("fx_rayscc_011.json");
    t.row("fx_rayscc_011.json's colour written #FF8000 is read and saved in small letters", &saved.to_string(), saved["color"] == "#ff8000");
    why(
        &mut t,
        &[
            ("fx_rayscc_027.json", "Light Rays's intensity runs from 0 to 2000, and this is -1."),
            ("fx_rayscc_028.json", "Light Rays's intensity runs from 0 to 2000, and this is 2001."),
            ("fx_rayscc_030.json", "Light Rays's center runs from -1000 to 1000, and this is -1001."),
            ("fx_rayscc_031.json", "Light Rays's radius runs from 0 to 10000, and this is -1."),
            ("fx_rayscc_032.json", "Light Rays's radius runs from 0 to 10000, and this is 10001."),
            ("fx_rayscc_033.json", "Light Rays's warp softness runs from 0 to 1000, and this is 1001."),
            ("fx_rayscc_034.json", "Light Rays's direction runs from -3600 to 3600, and this is 3601."),
            ("fx_rayscc_035.json", "Light Rays's shape is \"round\" or \"square\", and this is \"triangle\"."),
            ("fx_rayscc_036.json", "Light Rays's colour from source is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_rayscc_037.json", "Light Rays's allow brightening is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_rayscc_038.json", "Light Rays's transfer mode is \"none\", \"add\", \"lighten\" or \"screen\", and this is \"multiply\"."),
            ("fx_rayscc_039.json", "Light Rays's colour is written #rrggbb, and this is \"#12345\"."),
        ],
    );
    let rest = r##""radius": 50, "warp_softness": 50, "shape": "round", "direction": 0, "color_from_source": "on", "allow_brightening": "on", "color": "#ffffff", "transfer_mode": "none""##;
    t.shape_refused("fx_rayscc_001.json", "a Light Rays with no `center`", &format!("{{\"intensity\": 100, {rest}}}"));
    t.shape_refused("fx_rayscc_001.json", "a Light Rays with one number for its centre", &format!("{{\"intensity\": 100, \"center\": [50], {rest}}}"));
    t.shape_refused(
        "fx_rayscc_001.json",
        "a Light Rays with no `radius`",
        r##"{"intensity": 100, "center": [50, 50], "warp_softness": 50, "shape": "round", "direction": 0, "color_from_source": "on", "allow_brightening": "on", "color": "#ffffff", "transfer_mode": "none"}"##,
    );
    t.shape_refused(
        "fx_rayscc_001.json",
        "a Light Rays with its shape as a number",
        r##"{"intensity": 100, "center": [50, 50], "radius": 50, "warp_softness": 50, "shape": 1, "direction": 0, "color_from_source": "on", "allow_brightening": "on", "color": "#ffffff", "transfer_mode": "none"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rayscc_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 10001", set(cc(MID, [100.0, 10001.0, 50.0, 0.0], WORDS))),
            ("intensity 2001", set(cc(MID, [2001.0, 50.0, 50.0, 0.0], WORDS))),
            ("warp softness -1", set(cc(MID, [100.0, 50.0, -1.0, 0.0], WORDS))),
            ("direction -3601", set(cc(MID, [100.0, 50.0, 50.0, -3601.0], WORDS))),
            ("centre 50, 1001", set(cc([50.0, 1001.0], START, WORDS))),
            ("shape \"triangle\"", set(cc(MID, START, ["triangle", "on", "on", "#ffffff", "none"]))),
            ("transfer mode \"multiply\"", set(cc(MID, START, ["round", "on", "on", "#ffffff", "multiply"]))),
            ("colour \"orange\"", set(cc(MID, START, ["round", "off", "on", "orange", "none"]))),
            ("radius keyed to 20000", keys("radius", &[(0, &[50.0]), (4, &[20000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rayscc_001.json",
        vec![
            (
                "the tops: intensity 2000, radius 10000, warp 1000, direction 3600, centre 1000, 1000",
                set(cc([1000.0, 1000.0], [2000.0, 10000.0, 1000.0, 3600.0], ["square", "off", "off", "#000000", "lighten"])),
            ),
            (
                "the bottoms: intensity 0, radius 0, warp 0, direction -3600, centre -1000, -1000",
                set(cc([-1000.0, -1000.0], [0.0, 0.0, 0.0, -3600.0], ["round", "on", "on", "#ffffff", "add"])),
            ),
            ("radius keyed from 0 to 100", keys("radius", &[(0, &[0.0]), (4, &[100.0])])),
            ("direction keyed from 0 to 90", keys("direction", &[(0, &[0.0]), (4, &[90.0])])),
            ("centre keyed from 50, 50 to 0, 0", keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_rayscc_001.json", 0),
        ("fx_rayscc_004.json", 0),
        ("fx_rayscc_005.json", 0),
        ("fx_rayscc_007.json", 0),
        ("fx_rayscc_013.json", 0),
        ("fx_rayscc_022.json", 2),
        ("fx_rayscc_026.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_rayscc", 39, &[2, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39], is_cc);
    card_reference(
        &mut t,
        &mut gpu,
        "Light Rays",
        "core.light_rays",
        &[
            ("as it starts (round, radius 50, warp 50, none)", json!({})),
            ("radius 400, warp 300, Add", json!({"radius": 400, "warp_softness": 300, "transfer_mode": "add"})),
            (
                "square 300 turned 30, Color from Source off #ff8000, intensity 250, Screen",
                json!({"shape": "square", "radius": 300, "direction": 30, "color_from_source": "off", "color": "#ff8000", "intensity": 250, "transfer_mode": "screen"}),
            ),
            (
                "Lighten, intensity 300 held at 100, radius 600, round 30, 40",
                json!({"transfer_mode": "lighten", "intensity": 300, "allow_brightening": "off", "radius": 600, "center": [30, 40]}),
            ),
        ],
        is_cc,
    );

    street(
        &mut t,
        "D-423",
        "core.light_rays",
        &[
            ("as_added", json!({}), "as it starts: a round source 50 pixels about the middle, its rays streaming outward in the street's colours, laid over it", changed),
            ("big_source", json!({"radius": 300, "warp_softness": 200}), "radius 300, warp 200: a bigger source, longer and brighter rays, melted together", changed),
            ("square", json!({"shape": "square", "radius": 200, "direction": 30, "warp_softness": 0}), "a square source 200 turned 30 degrees, no warp: the rays fan out from a turned square", changed),
            (
                "orange_screen",
                json!({"color_from_source": "off", "color": "#ff8000", "radius": 250, "transfer_mode": "screen", "center": [30, 40]}),
                "Color from Source off #ff8000, radius 250, screen, round 30, 40: orange rays screened over the street",
                changed,
            ),
            ("add_bright", json!({"radius": 250, "intensity": 300, "transfer_mode": "add"}), "intensity 300, radius 250, add: the rays added three times, the middle blown out", changed),
        ],
    );

    t.finish("D-423_light_rays_cc_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B302_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-302: a measurement, run deliberately with --release --ignored"]
fn b302_light_rays_cc_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B302_CPU").is_ok();
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
        ("Noise, then Light Rays as added (round, radius 50, warp 50, none)", Some(("core.light_rays", json!({})))),
        ("Noise, then Light Rays, radius 400, warp 300, add", Some(("core.light_rays", json!({"radius": 400, "warp_softness": 300, "transfer_mode": "add"})))),
        (
            "Noise, then Light Rays, square 300, warp 0, Color from Source off, screen",
            Some(("core.light_rays", json!({"shape": "square", "radius": 300, "direction": 30, "warp_softness": 0, "color_from_source": "off", "color": "#ff8000", "transfer_mode": "screen"}))),
        ),
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
    let out = std::env::var("B302_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-302_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
