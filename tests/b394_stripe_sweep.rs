//! B-394: D-514, after CycoreFX's CC Line Sweep (Transition): the layer wiped away line by line,
//! the lines' thickness, overlap and angle set. The CycoreFX manual names the controls but
//! publishes no formula and no settings when added: the rule and those settings are ours, stated
//! in `tools/stripe_sweep_reference.py`.
//!
//! Writes `verification/D-514_stripe_sweep_table.md` and draws pictures into
//! `verification/D-514 pictures/`.
//!
//! Every expected pixel is `Fixtures/stripe_sweep/expected_stripe_sweep.json`, written by the
//! tool before this code existed and printed in document 25 as FX-STRIPE-001 to 024. Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, distance, median, reference, repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// The Stripe Sweeps the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::StripeSweep { .. })))
        .count()
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers.


/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=24 {
        let file = format!("fx_stripe_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b394-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Stripe Sweep {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Stripe Sweep {what} on three layers, frame {frame}, {}", quality.label()),
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


/// A Stripe Sweep with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "direction": 0, "thickness": 20, "slant": 50, "flip": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.stripe_sweep", "enabled": true, "parameters": all})
}

#[test]
fn b394_stripe_sweep() {
    let mut t = Table::new(
        "stripe_sweep",
        "# D-514: Stripe Sweep\n\nB-394, after CycoreFX's CC Line Sweep (Transition): the layer \
         wiped away line by line. The CycoreFX manual names the controls but publishes no formula \
         and no settings when added, so the rule and those settings are ours, stated in \
         `tools/stripe_sweep_reference.py`: the drawing cut into lines Thickness pixels thick \
         across the Direction, each wiped from one end to the other (the other, flipped), the \
         lines starting one after another, the Slant how much they overlap (0 one at a time, 99 \
         all together). Every expected pixel is \
         `Fixtures/stripe_sweep/expected_stripe_sweep.json`, written by the tool before this code \
         existed and printed in document 25 as FX-STRIPE-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-STRIPE-001 to 024 (document 25)");
    t.fixtures_numbered("expected_stripe_sweep.json", 1..=24);

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_stripe_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_stripe_014.json");
    t.row(
        "fx_stripe_014.json is saved with its five settings, flipped, direction -135, slant 20, thickness 2, completion 65",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5
            && saved["flip"] == "on"
            && saved["direction"] == json!(-135)
            && saved["slant"] == json!(20)
            && saved["thickness"] == json!(2)
            && saved["completion"] == json!(65),
    );
    for (file, want) in [
        ("fx_stripe_015.json", "completion"),
        ("fx_stripe_016.json", "completion"),
        ("fx_stripe_017.json", "direction"),
        ("fx_stripe_018.json", "thickness"),
        ("fx_stripe_019.json", "thickness"),
        ("fx_stripe_020.json", "slant"),
        ("fx_stripe_021.json", "slant"),
        ("fx_stripe_022.json", "flip"),
        ("fx_stripe_023.json", "flip"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_stripe_001.json",
        "a Stripe Sweep whose thickness is a word",
        r##"{"completion": 0, "direction": 0, "thickness": "thick", "slant": 50, "flip": "off"}"##,
    );
    t.shape_refused(
        "fx_stripe_001.json",
        "a Stripe Sweep whose flip is a number",
        r##"{"completion": 0, "direction": 0, "thickness": 20, "slant": 50, "flip": 1}"##,
    );

    t.heading("On the card (D-514)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 50 (as added otherwise)", json!({"completion": 50}), 3),
            ("completion 40, thickness 6, slant 0, direction 30", json!({"completion": 40, "thickness": 6, "slant": 0, "direction": 30}), 3),
            ("completion 60, flipped, slant 99, direction -90, thickness 100", json!({"completion": 60, "flip": "on", "slant": 99, "direction": -90, "thickness": 100}), 3),
            ("completion keyed 10 to 90", json!({"completion": {"base": 10, "keyframes": [{"frame": 0, "value": 10, "interp": "linear"}, {"frame": 239, "value": 90, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-514 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-514 pictures");
    fs::create_dir_all(&out).unwrap();
    let (street, _) = picture(&dir, "1_left.png", (480, 270), json!([]));
    png_out::write_rgba(&out.join("0_the_street.png"), 480, 270, OutputDepth::Eight, &[], &street).unwrap();
    // name, settings, a pixel that must be gone and one that must be as the street's (worked by
    // the reference's rule at 480 by 270).
    for (name, p, gone, kept) in [
        ("1_stripes", json!({"completion": 40}), (240, 2), (240, 268)),
        ("2_one_at_a_time", json!({"completion": 40, "slant": 0}), (240, 2), (240, 268)),
        ("3_all_together", json!({"completion": 40, "slant": 99}), (2, 135), (470, 135)),
        ("4_columns_flipped", json!({"completion": 40, "direction": 90, "thickness": 40, "flip": "on"}), (470, 135), (2, 135)),
        ("5_turned", json!({"completion": 50, "direction": 30}), (240, 135), (470, 268)),
        ("6_done", json!({"completion": 100}), (2, 2), (240, 135)),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let (g, k) = (at(&bytes, gone), at(&bytes, kept));
        let done = name == "6_done";
        t.row(
            &format!("{name}.png: drawn without a warning, the pixel at {gone:?} gone, {}", if done { "every pixel gone".to_string() } else { format!("the one at {kept:?} as the street's") }),
            &format!("{} pixels changed; at {gone:?} {g:?}; at {kept:?} {k:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && g[3] == 0 && if done { bytes.chunks_exact(4).all(|p| p[3] == 0) } else { k == at(&street, kept) },
        );
    }
    t.finish("D-514_stripe_sweep_table.md");
}


/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B394_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-394: a measurement, run deliberately with --release --ignored"]
fn b394_stripe_sweep_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B394_CPU").is_ok();
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
        ("Noise, then Stripe Sweep, completion 50 (as added otherwise)", Some(json!({"completion": 50}))),
        ("Noise, then Stripe Sweep, completion 50, thickness 4, slant 0, direction 30", Some(json!({"completion": 50, "thickness": 4, "slant": 0, "direction": 30}))),
        ("Noise, then Stripe Sweep, completion 50, flipped, thickness 200, slant 99", Some(json!({"completion": 50, "flip": "on", "thickness": 200, "slant": 99}))),
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
    let out = std::env::var("B394_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-394_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
