//! B-429: D-549, After Effects' Reduce Interlace Flicker (Stylize, Obsolete) under
//! `core.reduce_interlace_flicker`: Softness, a second name over Gaussian Blur's one-way blur
//! (D-303), vertical with its edges repeated, as D-383 and D-394.
//!
//! Writes `verification/D-549_reduce_interlace_flicker_table.md` and draws pictures into
//! `verification/D-549 pictures/`.
//!
//! Every expected pixel is
//! `Fixtures/reduce_interlace_flicker/expected_reduce_interlace_flicker.json`, written by
//! `tools/reduce_interlace_flicker_reference.py` before this code existed and printed in document
//! 25 as FX-RIF-001 to 009. Tolerance 2e-5. Nothing here is a snapshot of a run.

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

/// The Reduce Interlace Flickers the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ReduceInterlaceFlicker { .. })))
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

/// A Reduce Interlace Flicker with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"softness": 1});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.reduce_interlace_flicker", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning or stay on the processor.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=9 {
        let file = format!("fx_rif_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b429-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Reduce Interlace Flicker {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Reduce Interlace Flicker {what} on three layers, frame {frame}, {}", quality.label()),
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

/// 64 by 64: on the left, thin lines across, white and black rows in turn, the picture that
/// flickers on interlaced video; on the right, thin lines up and down, white and black columns
/// in turn.
const STRIPES: (usize, usize) = (64, 64);
fn stripes() -> Vec<u8> {
    let mut b = Vec::with_capacity(STRIPES.0 * STRIPES.1 * 4);
    for y in 0..STRIPES.1 {
        for x in 0..STRIPES.0 {
            let white = if x < STRIPES.0 / 2 { y % 2 == 0 } else { x % 2 == 0 };
            let v = if white { 255 } else { 0 };
            b.extend_from_slice(&[v, v, v, 255]);
        }
    }
    b
}

fn at(b: &[u8], x: usize, y: usize, width: usize) -> &[u8] {
    &b[(y * width + x) * 4..][..4]
}

#[test]
fn b429_reduce_interlace_flicker() {
    let mut t = Table::new(
        "reduce_interlace_flicker",
        "# D-549: Reduce Interlace Flicker\n\nB-429: `core.reduce_interlace_flicker` takes After \
         Effects' Reduce Interlace Flicker (Stylize, Obsolete) control, Softness, as a second name \
         over Gaussian Blur's one-way blur (D-303, as D-383 and D-394): Softness is the sigma in \
         pixels of a blur up and down only, its edges repeated, so a thin line across no longer \
         sits on one field. Gaussian Blur leaves a one-way blur to the processor, so this does \
         too. Every expected pixel is \
         `Fixtures/reduce_interlace_flicker/expected_reduce_interlace_flicker.json`, written by \
         `tools/reduce_interlace_flicker_reference.py` before this code existed and printed in \
         document 25 as FX-RIF-001 to 009; Gaussian Blur's FX-BLURRY-001 to 009 rerun unchanged. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-RIF-001 to 009 (document 25)");
    t.fixtures_numbered("expected_reduce_interlace_flicker.json", 1..=9);

    t.heading("Gaussian Blur, the engine under the second name: its fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/blurriness");
    t.fixtures("expected_blurriness.json");
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=6).map(|n| format!("fx_rif_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_rif_003.json");
    t.row(
        "fx_rif_003.json is saved with its one setting, softness 2.5",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 1 && saved["softness"] == 2.5,
    );
    for file in ["fx_rif_007.json", "fx_rif_008.json"] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming softness"), &why, why.contains("softness"));
    }
    t.shape_refused("fx_rif_001.json", "a Reduce Interlace Flicker whose softness is a word", r#"{"softness": "soft"}"#);
    t.shape_refused("fx_rif_001.json", "a Reduce Interlace Flicker without its softness", r#"{}"#);

    t.heading("Commands");
    let mut document = t.load("fx_rif_001.json").document;
    let over = changed(&t, "fx_rif_001.json", |e| if let Effect::ReduceInterlaceFlicker { softness } = e { *softness = 501.0 });
    t.refused(
        &mut document,
        vec![("softness 501", set(over)), ("softness keyed to 600", keys("softness", &[(0, &[1.0]), (4, &[600.0])]))],
    );
    let soft = changed(&t, "fx_rif_001.json", |e| if let Effect::ReduceInterlaceFlicker { softness } = e { *softness = 2.5 });
    t.taken(
        &mut document,
        "fx_rif_001.json",
        vec![("softness 2.5", set(soft)), ("softness keyed from 0 to 2", keys("softness", &[(0, &[0.0]), (4, &[2.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rif_001.json", 0), ("fx_rif_003.json", 0), ("fx_rif_004.json", 0), ("fx_rif_005.json", 2), ("fx_rif_006.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255, the blur left to the processor");
    let mut gpu = Gpu::new().expect("a usable card");
    // Gaussian Blur's one-way blur stays on the processor, so no file puts this on the card.
    let none: Vec<u32> = (1..=9).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[("as added (softness 1), left to the processor", json!({}), 0), ("softness 2.5, left to the processor", json!({"softness": 2.5}), 0)],
    );

    t.heading("Pictures: in `verification/D-549 pictures/`");
    let dir = repo("verification/D-549 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("stripes.png", STRIPES, &stripes());
    write("town.png", TOWN, &town());

    let lines = |p: J| picture(&dir, "stripes.png", STRIPES, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "stripes.png", STRIPES, json!([]));
    write("1_before.png", STRIPES, &before);
    t.row(
        "1_before.png, thin lines with no effect: on the left white and black rows in turn, on the right white and black columns in turn; draws cleanly",
        &format!("{said:?}"),
        said.is_empty(),
    );
    let (same, said) = lines(json!({"softness": 0}));
    t.row(
        "softness 0 changes nothing: the lines byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // By hand at softness 1: a Gaussian of sigma 1 reaching 3 rows; a white row takes the
    // weights at 0 and 2 rows away, 0.507 of the light, encoded 189; a black row takes those at
    // 1 and 3 rows away, 0.493, encoded 186.
    let (after, said) = lines(json!({}));
    write("2_as_added.png", STRIPES, &after);
    let w = STRIPES.0;
    let (white, black) = (at(&after, 10, 32, w).to_vec(), at(&after, 10, 33, w).to_vec());
    let right_kept = (0..STRIPES.1).all(|y| (w / 2..w).all(|x| at(&after, x, y, w) == at(&before, x, y, w)));
    t.row(
        "2_as_added.png, as added (softness 1): the rows across on the left melt into one grey (a white row 189 and a black row 186 by hand, each within 1); the columns up and down on the right untouched byte for byte, since the blur never reaches sideways; draws cleanly",
        &format!("{said:?}, a white row {white:?}, a black row {black:?}, the right half kept: {right_kept}"),
        said.is_empty()
            && white[..3].iter().all(|v| v.abs_diff(189) <= 1)
            && black[..3].iter().all(|v| v.abs_diff(186) <= 1)
            && white[3] == 255
            && black[3] == 255
            && right_kept,
    );

    let (light, said) = lines(json!({"softness": 0.3}));
    write("3_softness_0_3.png", STRIPES, &light);
    let (lw, lb) = (at(&light, 10, 32, w)[0] as i32, at(&light, 10, 33, w)[0] as i32);
    let white = [white[0] as i32];
    let black = [black[0] as i32];
    t.row(
        "3_softness_0_3.png, softness 0.3: the rows across only softened, white rows still lighter than black ones by far more than in 2; draws cleanly",
        &format!("{said:?}, a white row {lw}, a black row {lb}"),
        said.is_empty() && lw > white[0] && lb < black[0] && lw - lb > 3 * (white[0] - black[0]),
    );

    // The second name: Gaussian Blur at the same sigma, vertical, edges repeated, draws the same
    // street byte for byte.
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (plain, _) = picture(&dir, "town.png", TOWN, json!([]));
    for softness in [0.5, 1.0, 2.5] {
        let blur = json!({"instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true, "parameters": {
            "sigma_px": softness, "edges": "repeat", "dimensions": "vertical", "units": "sigma"}});
        let (a, said0) = street(json!({"softness": softness}));
        let (b, said1) = picture(&dir, "town.png", TOWN, json!([blur]));
        if softness == 1.0 {
            write("4_street.png", TOWN, &a);
        }
        t.row(
            &format!("Reduce Interlace Flicker at softness {softness} against Gaussian Blur sigma {softness} pixels, vertical, edges repeated: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != plain,
        );
    }

    t.finish("D-549_reduce_interlace_flicker_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B429_CPU` set, the processor draws instead (the blur is the processor's either way).
#[test]
#[ignore = "B-429: a measurement, run deliberately with --release --ignored"]
fn b429_reduce_interlace_flicker_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B429_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Reduce Interlace Flicker as added (softness 1)", Some(json!({}))),
        ("Noise, then Reduce Interlace Flicker softness 2.5", Some(json!({"softness": 2.5}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
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
    let out = std::env::var("B429_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-429_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
