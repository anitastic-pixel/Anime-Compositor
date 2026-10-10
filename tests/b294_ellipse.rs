//! B-294: D-415 Ellipse, after After Effects' Ellipse ("Generate" in `docs/effects/EFFECTS.md`):
//! the outline of an ellipse about a centre, drawn as Beam's line is, over the layer or alone.
//!
//! Writes `verification/D-415_ellipse_table.md`.
//!
//! Every expected pixel is `Fixtures/ellipse/expected_ellipse.json`, written by
//! `tools/ellipse_reference.py` before this code existed and printed in document 25 as
//! FX-ELLIPSE-001 to 027. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-415 pictures/`.

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

fn ellipse(center: [f64; 2], [width, height, thickness, softness]: [f64; 4], words: [&str; 3]) -> Effect {
    let [inside_color, outside_color, composite] = words.map(str::to_string);
    Effect::Ellipse { center, width, height, thickness, softness, inside_color, outside_color, composite }
}

const PLAIN: [&str; 3] = ["#ffffff", "#3c8cff", "on"];

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
        "core.ellipse" => json!({"center": [50, 50], "width": 200, "height": 200, "thickness": 8, "softness": 50, "inside_color": "#ffffff", "outside_color": "#3c8cff", "composite": "on"}),
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
        let project = reference(|id| json!([fx(type_id, &format!("b294-{id}"), p)]));
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

// --- Ellipse --------------------------------------------------------------------------------

fn is_ellipse(e: &Effect) -> bool {
    matches!(e, Effect::Ellipse { .. })
}

/// Changed, and some of it left clear.
fn gaps(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).any(|p| p[3] == 0)
}

/// Changed, and nothing clear that was not clear before.
fn kept(a: &[u8], b: &[u8]) -> bool {
    changed(a, b) && a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] > 0 || q[3] == 0)
}

#[test]
fn b294_ellipse() {
    let mut t = Table::new(
        "ellipse",
        "# D-415: Ellipse\n\nB-294, after After Effects' Ellipse: the outline of the ellipse Width by \
         Height about Center, a share of the drawing's own size. A pixel's distance from the \
         outline is taken to first order, d = |k - 1| k / g with k the ellipse's level and g its \
         gradient (exact for a circle; min(a, b) at the centre), and the outline drawn as Beam's \
         line (D-207): Thickness wide, Softness per cent of it a ramp at least a pixel wide, Inside \
         Color along its middle and Outside Color at its edges, over the layer (Composite On \
         Original on) or alone. The layer never grows. Every expected pixel is \
         `Fixtures/ellipse/expected_ellipse.json`, written by `tools/ellipse_reference.py` before \
         this code existed and printed in document 25 as FX-ELLIPSE-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-ELLIPSE-001 to 027 (document 25)");
    t.fixtures("expected_ellipse.json");

    t.heading("The file");
    // FX-ELLIPSE-013's colours are written in capitals and saved in small letters, as every colour is.
    let all: Vec<String> = files("fx_ellipse", 27).into_iter().filter(|f| f != "fx_ellipse_013.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_ellipse_013.json");
    t.row(
        "fx_ellipse_013.json is saved with its colours in small letters, its words and numbers as written",
        &saved.to_string(),
        saved["inside_color"] == "#ff8000" && saved["outside_color"] == "#6450a0" && saved["composite"] == "off"
            && saved["width"] == 12 && saved["height"] == 8 && saved["thickness"] == 4 && saved["softness"] == 0 && saved["center"] == json!([50, 50]),
    );
    why(
        &mut t,
        &[
            ("fx_ellipse_019.json", "Ellipse's width runs from 1 to 10000, and this is 0."),
            ("fx_ellipse_020.json", "Ellipse's height runs from 1 to 10000, and this is 10001."),
            ("fx_ellipse_021.json", "Ellipse's thickness runs from 0 to 10000, and this is -1."),
            ("fx_ellipse_022.json", "Ellipse's softness runs from 0 to 100, and this is 101."),
            ("fx_ellipse_023.json", "Ellipse's center runs from -1000 to 1000, and this is 1001."),
            ("fx_ellipse_024.json", "Ellipse's composite is \"on\" or \"off\", and this is \"yes\"."),
            ("fx_ellipse_025.json", "Ellipse's inside colour is written #rrggbb, and this is \"#12345\"."),
            ("fx_ellipse_026.json", "Ellipse's outside colour is written #rrggbb, and this is \"red\"."),
        ],
    );
    t.shape_refused("fx_ellipse_001.json", "an Ellipse with no `composite`", r##"{"center": [50, 50], "width": 200, "height": 200, "thickness": 8, "softness": 50, "inside_color": "#ffffff", "outside_color": "#3c8cff"}"##);
    t.shape_refused("fx_ellipse_001.json", "an Ellipse with a width in words", r##"{"center": [50, 50], "width": "wide", "height": 200, "thickness": 8, "softness": 50, "inside_color": "#ffffff", "outside_color": "#3c8cff", "composite": "on"}"##);
    t.shape_refused("fx_ellipse_001.json", "an Ellipse whose centre is one number", r##"{"center": [50], "width": 200, "height": 200, "thickness": 8, "softness": 50, "inside_color": "#ffffff", "outside_color": "#3c8cff", "composite": "on"}"##);

    t.heading("Commands");
    let mut document = t.load("fx_ellipse_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("width 0", set(ellipse([50.0, 50.0], [0.0, 200.0, 8.0, 50.0], PLAIN))),
            ("composite \"yes\"", set(ellipse([50.0, 50.0], [200.0, 200.0, 8.0, 50.0], ["#ffffff", "#3c8cff", "yes"]))),
            ("inside colour \"white\"", set(ellipse([50.0, 50.0], [200.0, 200.0, 8.0, 50.0], ["white", "#3c8cff", "on"]))),
            ("width keyed to 10001", keys("width", &[(0, &[200.0]), (4, &[10001.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_ellipse_001.json",
        vec![
            ("12 by 8, thickness 3, softness 40, orange inside, violet outside, alone", set(ellipse([50.0, 50.0], [12.0, 8.0, 3.0, 40.0], ["#ff8000", "#6450a0", "off"]))),
            ("centre keyed from 50, 50 to 25, 50", keys("center", &[(0, &[50.0, 50.0]), (4, &[25.0, 50.0])])),
            ("width keyed from 8 to 16", keys("width", &[(0, &[8.0]), (4, &[16.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_ellipse_002.json", 0), ("fx_ellipse_004.json", 0), ("fx_ellipse_006.json", 0), ("fx_ellipse_009.json", 2), ("fx_ellipse_014.json", 0), ("fx_ellipse_016.json", 0), ("fx_ellipse_018.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_ellipse", 27, &[19, 20, 21, 22, 23, 24, 25, 26, 27], is_ellipse);
    card_reference(
        &mut t,
        &mut gpu,
        "Ellipse",
        "core.ellipse",
        &[
            ("as it starts (an outline 200 across in the middle, over the layer)", json!({})),
            ("900 by 400, thickness 30, softness 80", json!({"width": 900, "height": 400, "thickness": 30, "softness": 80})),
            ("600 by 600 alone, thickness 20", json!({"width": 600, "height": 600, "thickness": 20, "composite": "off"})),
            ("200 by 800, a hairline 1 thick, softness 0", json!({"width": 200, "height": 800, "thickness": 1, "softness": 0})),
            ("500 by 300 at 20, 70, thickness 60, orange and violet", json!({"center": [20, 70], "width": 500, "height": 300, "thickness": 60, "softness": 30, "inside_color": "#ff8000", "outside_color": "#6450a0"})),
            ("4000 by 1500, thickness 120, softness 100, past the frame", json!({"width": 4000, "height": 1500, "thickness": 120, "softness": 100})),
            ("1200 by 900 centred outside at 120, 50, thickness 40", json!({"center": [120, 50], "width": 1200, "height": 900, "thickness": 40})),
            ("300 by 150 alone, thickness 200, softness 100, thicker than it is tall", json!({"width": 300, "height": 150, "thickness": 200, "softness": 100, "composite": "off"})),
        ],
        is_ellipse,
    );

    street(
        &mut t,
        "D-415",
        "core.ellipse",
        &[
            ("as_added", json!({}), "as it starts: a glowing outline 200 across in the middle, white along its middle and blue at its edges, over the street", kept),
            ("neon_alone", json!({"width": 360, "height": 180, "thickness": 14, "softness": 70, "inside_color": "#fff0c0", "outside_color": "#ff8000", "composite": "off"}), "a wide orange neon oval alone, the rest clear", gaps),
            ("tall_violet", json!({"width": 120, "height": 240, "thickness": 6, "softness": 20, "outside_color": "#6450a0"}), "a tall thin oval, white fading to violet, over the street", kept),
            ("halo", json!({"center": [30, 40], "width": 300, "height": 300, "thickness": 60, "softness": 100}), "a thick soft halo centred left of the middle, over the street", kept),
        ],
    );

    t.finish("D-415_ellipse_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B294_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-294: a measurement, run deliberately with --release --ignored"]
fn b294_ellipse_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B294_CPU").is_ok();
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
        ("Noise, then Ellipse as added (200 across, thickness 8, over the layer)", Some(("core.ellipse", json!({})))),
        ("Noise, then Ellipse, 900 by 400, thickness 30, softness 80", Some(("core.ellipse", json!({"width": 900, "height": 400, "thickness": 30, "softness": 80})))),
        ("Noise, then Ellipse, 600 by 600 alone, thickness 20", Some(("core.ellipse", json!({"width": 600, "height": 600, "thickness": 20, "composite": "off"})))),
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
    let out = std::env::var("B294_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-294_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
