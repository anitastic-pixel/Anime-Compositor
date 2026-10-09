//! B-238: D-359, Lens Blur's blur map, after After Effects' Camera Lens Blur Blur Map group:
//! another layer's luminance or alpha says how far out of focus each pixel is ("Blur & Sharpen |
//! Camera Lens Blur" and pick #12 in `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-359_lens_blur_map_table.md`.
//!
//! Every expected pixel is `Fixtures/lens_blur/expected_lens_blur_map.json`, written by
//! `tools/lens_blur_map_reference.py` before this code existed, FX-LENS-045 to 069. Tolerance
//! 2e-5. Nothing here is a snapshot of a run. FX-LENS-001 to 044 stay B-59's and B-64's.
//!
//! It also blurs B-35's town by a ramp, sky out of focus and road sharp, into
//! `verification/D-359 pictures/`, twice enlarged.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// Radius 4, a round iris, no highlights, and the blur map's five.
fn lens(layer: &str, fit: &str, channel: &str, focal_distance: f64, invert: &str) -> Effect {
    Effect::LensBlur {
        radius: 4.0,
        edges: "transparent".into(),
        iris: "circle".into(),
        roundness: 0.0,
        rotation: 0.0,
        aspect: 1.0,
        highlight_gain: 0.0,
        highlight_threshold: 100.0,
        layer: J::from(layer),
        fit: fit.into(),
        channel: channel.into(),
        focal_distance,
        invert: invert.into(),
        map: None,
    }
}

fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

fn holder_effect(document: &anime_compositor::command::Document) -> String {
    format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0])
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
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

/// The Lens Blurs the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::LensBlur { .. })))
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers; the fourth is
/// left as it is, to be their blur map.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// A layer's one Lens Blur, as written in a file.
fn lens_json(id: &str, p: &J) -> J {
    json!({"instance_id": format!("b238-{id}"), "type_id": "core.lens_blur", "enabled": true, "parameters": p.clone()})
}

/// The town with `effects` on it and the ramp, white at the bottom, as its other layer; drawn,
/// straight 8-bit, and twice enlarged.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(effect_table::repo("Fixtures/lens_blur/fx_lens_046.json")).unwrap()).unwrap();
    project["assets"] = json!([
        {"id": "asset-bars", "kind": "still", "name": "town", "path": "town.png", "interpretation": {"color_space": "srgb", "alpha": "straight"}},
        {"id": "asset-ramp", "kind": "still", "name": "ramp", "path": "ramp.png", "interpretation": {"color_space": "srgb", "alpha": "straight"}}
    ]);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["layer_order"] = json!(["holder", "ramp"]);
    let layers = comp["layers"].as_array_mut().unwrap();
    layers.truncate(2);
    layers[0]["effects"] = effects;
    layers[1]["transform"]["position"]["base"] = json!([0, 0]);
    layers[1]["transform"]["scale"]["base"] = json!([100, 100]);
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    let bytes = frame.to_srgb8_straight();
    let (w, h) = TOWN;
    let big: Vec<u8> = (0..h * 2).flat_map(|y| (0..w * 2).map(move |x| (y / 2 * w + x / 2) * 4)).flat_map(|i| bytes[i..i + 4].to_vec()).collect();
    (bytes, big, said)
}

#[test]
fn b238_lens_blur_map() {
    let mut t = Table::new(
        "lens_blur",
        "# D-359: Lens Blur's blur map\n\nB-238, after After Effects' Camera Lens Blur Blur Map \
         group: another layer's luminance or alpha, lying on the layer, says how far out of focus \
         each pixel is. A value at Blur Focal Distance is sharp; the further from it, the wider the \
         iris, to the full radius 255 levels away. Every expected pixel is \
         `Fixtures/lens_blur/expected_lens_blur_map.json`, written by \
         `tools/lens_blur_map_reference.py` before this code existed, FX-LENS-045 to 069. The \
         build's frame is compared sample by sample against the tolerance of 2e-5. FX-LENS-001 to \
         044 are B-59's and B-64's and must not move.\n",
    );

    t.heading("FX-LENS-045 to 069");
    t.fixtures_numbered("expected_lens_blur_map.json", 45..=69);
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_lens_blur_map.json")).unwrap()).unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let got = persist::load(&t.root.join(file)).err();
        let id = case["refused"].as_str().unwrap();
        t.row(
            &format!("{file}: {}", case["says"].as_str().unwrap()),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("With no map, Lens Blur is the Lens Blur it was");
    let frame = |file: &str| t.render(&t.load(file).document, 0, 64).data().to_vec();
    let (plain, unset, white) = (frame("fx_lens_007.json"), frame("fx_lens_045.json"), frame("fx_lens_050.json"));
    t.row(
        "fx_lens_045.json, every Blur Map setting at its start, against fx_lens_007.json, the same blur without them",
        if unset == plain { "byte-identical" } else { "differ" },
        unset == plain,
    );
    t.row(
        "fx_lens_050.json, a white solid read by its alpha so every pixel is at the full radius, against fx_lens_007.json: \
         the map's last level is Lens Blur's own sums, to the bit",
        if white == plain { "byte-identical" } else { "differ" },
        white == plain,
    );
    let old = t.saved_parameters("fx_lens_001.json");
    let clean = ["layer", "fit", "channel", "focal_distance", "invert"].iter().all(|k| old.get(k).is_none());
    t.row("fx_lens_001.json, from before D-359, is saved without any of the new settings", &old.to_string(), clean);

    t.heading("How far it reaches");
    let got = lens("ramp", "center", "luminance", 0.0, "off").bounds_expansion();
    t.row("with a map, the layer may grow as far as the full radius's iris reaches, 4", &got.to_string(), got == 4);
    let mut draft = lens("ramp", "center", "luminance", 128.0, "off");
    draft.scale_distances(|d| d * 0.5);
    let mut want = lens("ramp", "center", "luminance", 128.0, "off");
    if let Effect::LensBlur { radius, .. } = &mut want {
        *radius = 2.0;
    }
    t.row(
        "a half-size draft halves the radius and keeps Blur Focal Distance, which is a map value, not a distance",
        &format!("{draft:?}"),
        draft == want,
    );

    t.heading("The file");
    let files: Vec<String> = (45..=69).map(|n| format!("fx_lens_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    t.shape_refused(
        "fx_lens_046.json",
        "a Lens Blur whose Blur Focal Distance is a word",
        r#"{"radius": 4, "edges": "transparent", "layer": "ramp", "focal_distance": "near"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_lens_046.json").document;
    let held = holder_effect(&document);
    for (what, effect) in [
        ("Blur Focal Distance 256", lens("ramp", "center", "luminance", 256.0, "off")),
        ("Placement \"tile\"", lens("ramp", "tile", "luminance", 0.0, "off")),
        ("Channel \"red\"", lens("ramp", "center", "red", 0.0, "off")),
        ("Invert \"yes\"", lens("ramp", "center", "luminance", 0.0, "yes")),
    ] {
        let refused = document.apply(set(effect)).err();
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && holder_effect(&document) == held,
        );
    }
    let before = t.render(&document, 0, 64);
    let commands = [
        ("Stretch, Alpha, Blur Focal Distance 128, Invert on, reading the card layer", set(lens("card", "stretch", "alpha", 128.0, "on"))),
        (
            "Blur Focal Distance keyed from 0 to 255",
            Command::SetEffectKeys {
                composition: Id::new(MAIN),
                layer_id: Id::new("holder"),
                instance_id: Id::new("fx-1"),
                setting: "focal_distance".into(),
                keys: [(0, 0.0), (4, 255.0)].iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
            },
        ),
        ("no layer, which turns the map off", set(lens("", "center", "luminance", 0.0, "off"))),
    ];
    let n = commands.len();
    for (what, command) in commands {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lens_046.json", 0), ("fx_lens_053.json", 0), ("fx_lens_055.json", 0), ("fx_lens_056.json", 2), ("fx_lens_059.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 45..=62 {
        let file = format!("fx_lens_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        // FX-LENS-057 names a layer that is not there, so the layer is drawn without the effect.
        let none = n == 57;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        ("Radius 10, layer 4's luminance, Blur Focal Distance 0", json!({"radius": 10, "edges": "transparent", "layer": "layer-4"})),
        (
            "Radius 20, a hexagon with highlights, layer 4 stretched, Blur Focal Distance 128, inverted",
            json!({"radius": 20, "edges": "transparent", "iris": "hexagon", "highlight_gain": 2, "highlight_threshold": 70, "layer": "layer-4", "fit": "stretch", "focal_distance": 128, "invert": "on"}),
        ),
        ("Radius 8, repeat edges, layer 4's alpha, Blur Focal Distance 40", json!({"radius": 8, "edges": "repeat", "layer": "layer-4", "channel": "alpha", "focal_distance": 40})),
    ] {
        let project = reference(|id| json!([lens_json(id, &p)]));
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let mut unmapped = p.clone();
        unmapped["layer"] = J::from("");
        let changed = distance(&cpu(&project), &cpu(&reference(|id| json!([lens_json(id, &unmapped)])))).1;
        t.row(
            &format!("the reference shot, Lens Blur {what}: the processor's frame 100 differs from the same blur without the map, so the comparisons below test the map"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Lens Blur {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: a town with a depth ramp, in `verification/D-359 pictures/`, twice enlarged");
    let dir = effect_table::repo("verification/D-359 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8], scale: usize| png_out::write_rgba(&dir.join(name), w * scale, h * scale, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &effect_table::town(), 1);
    // The ramp: black at the top, white at the bottom, opaque.
    let ramp: Vec<u8> = (0..h).flat_map(|y| (0..w).flat_map(move |_| {
        let v = (y * 255 / (h - 1)) as u8;
        [v, v, v, 255]
    })).collect();
    write("ramp.png", &ramp, 1);
    let one = |p: J| json!([{"instance_id": "fx-1", "type_id": "core.lens_blur", "enabled": true, "parameters": p}]);
    // Sharpness: how much each pixel differs from its right and lower neighbours, summed over a
    // band of rows, away from the sides, where the transparent edges fade.
    let sharp = |b: &[u8], rows: std::ops::Range<usize>| {
        let at = |x: usize, y: usize, k: usize| b[(y * w + x) * 4 + k];
        rows.flat_map(|y| (8..w - 8).map(move |x| (x, y)))
            .map(|(x, y)| (0..3).map(|k| (at(x, y, k).abs_diff(at(x + 1, y, k)) as u64).pow(2) + (at(x, y, k).abs_diff(at(x, y + 1, k)) as u64).pow(2)).sum::<u64>())
            .sum::<u64>()
    };
    let (house, road) = (115..205, 220..262);

    let (before, big, said) = picture(&dir, J::Array(vec![]));
    write("before.png", &big, 2);
    t.row("before.png, the town with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (flat, big, said) = picture(&dir, one(json!({"edges": "transparent", "radius": 6})));
    write("lens_blur_no_map.png", &big, 2);
    t.row(
        "lens_blur_no_map.png, Lens Blur radius 6 with no map: everything equally soft, houses and road markings alike",
        &format!("{said:?}; houses {} from {}, road {} from {}", sharp(&flat, house.clone()), sharp(&before, house.clone()), sharp(&flat, road.clone()), sharp(&before, road.clone())),
        said.is_empty() && sharp(&flat, house.clone()) < sharp(&before, house.clone()) && sharp(&flat, road.clone()) < sharp(&before, road.clone()),
    );

    let (near, big, said) = picture(&dir, one(json!({"edges": "transparent", "radius": 6, "layer": "ramp", "focal_distance": 255})));
    write("lens_blur_road_sharp.png", &big, 2);
    t.row(
        "lens_blur_road_sharp.png, the ramp as the blur map, Blur Focal Distance 255: the road at the bottom, white on the map, \
         nearly sharp, the houses higher up softer and the sky softest, as a camera focused on the near ground",
        &format!("{said:?}; houses {}, road {} of {} before", sharp(&near, house.clone()), sharp(&near, road.clone()), sharp(&before, road.clone())),
        said.is_empty() && sharp(&near, road.clone()) > sharp(&flat, road.clone()) && sharp(&near, house.clone()) < sharp(&before, house.clone()),
    );

    let (far, big, said) = picture(&dir, one(json!({"edges": "transparent", "radius": 6, "layer": "ramp", "focal_distance": 140})));
    write("lens_blur_houses_sharp.png", &big, 2);
    t.row(
        "lens_blur_houses_sharp.png, Blur Focal Distance 140, the houses' grey on the map: the houses kept sharper than with no map, \
         the road blurred more than in lens_blur_road_sharp.png",
        &format!("{said:?}; houses {} (no map {}), road {} (road sharp {})", sharp(&far, house.clone()), sharp(&flat, house.clone()), sharp(&far, road.clone()), sharp(&near, road.clone())),
        said.is_empty() && sharp(&far, house.clone()) > sharp(&flat, house.clone()) && sharp(&far, road.clone()) < sharp(&near, road.clone()),
    );

    t.finish("D-359_lens_blur_map_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-237's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole, with Draw on: GPU. The first loop's median
/// is "first"; the loops after it give "again". With `B238_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-238: a measurement, run deliberately with --release --ignored"]
fn b238_lens_blur_map_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B238_CPU").is_ok();
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
        ("Noise, then Lens Blur radius 10, no map", Some(json!({"radius": 10, "edges": "transparent"}))),
        ("Noise, then Lens Blur radius 10, layer 4's luminance as the blur map", Some(json!({"radius": 10, "edges": "transparent", "layer": "layer-4"}))),
        ("Noise, then Lens Blur radius 20, layer 4's luminance as the blur map", Some(json!({"radius": 20, "edges": "transparent", "layer": "layer-4"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(lens_json(id, p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
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
    let out = std::env::var("B238_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-238_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
