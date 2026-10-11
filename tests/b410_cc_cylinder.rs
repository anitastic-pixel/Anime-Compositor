//! B-410: D-530, after CycoreFX's CC Cylinder (Perspective): the layer wrapped round an open
//! cylinder, turned and lit, in its own fake 3D. The CycoreFX manual names the controls but
//! publishes no formula and no settings when added: the rule and those settings are ours, stated
//! in `tools/cc_cylinder_reference.py`. The composition camera and 3D layers are not used.
//!
//! Writes `verification/D-530_cc_cylinder_table.md` and draws pictures into
//! `verification/D-530 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_cylinder/expected_cc_cylinder.json`, written by the
//! tool before this code existed and printed in document 25 as FX-CYL-001 to 027. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

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

/// The CC Cylinders the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CcCylinder { .. })))
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

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=27 {
        let file = format!("fx_cyl_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b410-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Cylinder {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, CC Cylinder {what} on three layers, frame {frame}, {}", quality.label()),
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

/// A CC Cylinder with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"radius": 50, "position": [50, 50], "position_z": 0, "rotation_x": 0, "rotation_y": 0,
        "rotation_z": 0, "render": "full", "light_intensity": 100, "light_color": "#ffffff", "light_height": 50,
        "light_direction": -45, "ambient": 40, "diffuse": 60, "specular": 50, "roughness": 10, "metal": 100});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_cylinder", "enabled": true, "parameters": all})
}

#[test]
fn b410_cc_cylinder() {
    let mut t = Table::new(
        "cc_cylinder",
        "# D-530: CC Cylinder\n\nB-410, after CycoreFX's CC Cylinder (Perspective): the layer \
         wrapped round an open cylinder, turned and lit, in its own fake 3D. The CycoreFX manual \
         names the controls but publishes no formula and no settings when added, so the rule and \
         those settings are ours, stated in `tools/cc_cylinder_reference.py`: a fixed eye (a 50 mm \
         lens on a 36 mm frame) looks at the cylinder, turned X then Y then Z; the outside is the \
         nearer hit and the inside the farther; a directional light (Light Height and Direction) \
         gives ambient, diffuse and a Blinn-Phong highlight whose colour Metal mixes between the \
         layer's and the light's. The composition camera and 3D layers are not used. Every \
         expected pixel is `Fixtures/cc_cylinder/expected_cc_cylinder.json`, written by the tool \
         before this code existed and printed in document 25 as FX-CYL-001 to 027. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CYL-001 to 027 (document 25)");
    t.fixtures_numbered("expected_cc_cylinder.json", 1..=27);

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_cyl_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cyl_013.json");
    t.row(
        "fx_cyl_013.json is saved with its sixteen settings, an orange light, metal 0, Position [50, 50]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 16
            && saved["light_color"] == "#ff8000"
            && saved["metal"] == json!(0)
            && saved["position"] == json!([50, 50]),
    );
    for (file, want) in [
        ("fx_cyl_019.json", "radius"),
        ("fx_cyl_020.json", "radius"),
        ("fx_cyl_021.json", "position z"),
        ("fx_cyl_022.json", "rotation y"),
        ("fx_cyl_023.json", "render"),
        ("fx_cyl_024.json", "light colour"),
        ("fx_cyl_025.json", "ambient"),
        ("fx_cyl_026.json", "roughness"),
        ("fx_cyl_027.json", "light height"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_cyl_001.json",
        "a CC Cylinder whose radius is a word",
        r##"{"radius": "wide", "position": [50, 50], "position_z": 0, "rotation_x": 0, "rotation_y": 0, "rotation_z": 0, "render": "full", "light_intensity": 100, "light_color": "#ffffff", "light_height": 50, "light_direction": -45, "ambient": 40, "diffuse": 60, "specular": 50, "roughness": 10, "metal": 100}"##,
    );
    t.shape_refused(
        "fx_cyl_001.json",
        "a CC Cylinder whose Position is one number",
        r##"{"radius": 50, "position": 50, "position_z": 0, "rotation_x": 0, "rotation_y": 0, "rotation_z": 0, "render": "full", "light_intensity": 100, "light_color": "#ffffff", "light_height": 50, "light_direction": -45, "ambient": 40, "diffuse": 60, "specular": 50, "roughness": 10, "metal": 100}"##,
    );

    t.heading("On the card (D-530)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[19, 20, 21, 22, 23, 24, 25, 26, 27]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            ("render inside, tipped, an orange light, metal 0", json!({"render": "inside", "rotation_x": 30, "light_color": "#ff8000", "metal": 0}), 3),
            ("radius 100, lying on its side, rough", json!({"radius": 100, "rotation_z": 90, "roughness": 50, "specular": 100}), 3),
            ("Rotation Y keyed one whole turn", json!({"rotation_y": {"base": 0, "keyframes": [{"frame": 0, "value": 0, "interp": "linear"}, {"frame": 239, "value": 360, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-530 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-530 pictures");
    fs::create_dir_all(&out).unwrap();
    let street = picture(&dir, "1_left.png", (480, 270), json!([])).0;
    for (name, p) in [
        ("1_as_added", json!({})),
        ("2_inside", json!({"render": "inside"})),
        ("3_turned_a_quarter", json!({"rotation_y": 90})),
        ("4_tipped_towards_you", json!({"rotation_x": 30})),
        ("5_on_its_side_orange", json!({"rotation_z": 90, "light_color": "#ffa040", "metal": 0})),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let middle = at(&bytes, (240, 135));
        t.row(
            &format!("{name}.png: drawn without a warning, the street changed, the middle drawn"),
            &format!("{} pixels changed; middle {middle:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && distance(&bytes, &street).1 > 0 && middle[3] > 0,
        );
    }
    t.finish("D-530_cc_cylinder_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B410_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-410: a measurement, run deliberately with --release --ignored"]
fn b410_cc_cylinder_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B410_CPU").is_ok();
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
        ("Noise, then CC Cylinder as added", Some(json!({}))),
        ("Noise, then CC Cylinder, render inside, tipped", Some(json!({"render": "inside", "rotation_x": 30}))),
        ("Noise, then CC Cylinder, radius 100, on its side", Some(json!({"radius": 100, "rotation_z": 90}))),
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
    let out = std::env::var("B410_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-410_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
