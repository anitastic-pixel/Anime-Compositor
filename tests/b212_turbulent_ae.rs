//! B-212: D-328, Turbulent Displace in After Effects' units.
//!
//! Writes `verification/D-328_turbulent_ae_table.md`.
//!
//! Every expected pixel is `Fixtures/turbulent_ae/expected_turbulent_ae.json`, written by
//! `tools/turbulent_ae_reference.py` before this code existed and printed in document 25 as
//! FX-TURB-AE-001 to 012. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also warps a soft glow the old way and the new into `verification/D-328 pictures/`, over
//! black, at tutorial 2's Amount 80, Size 2.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{set, Table};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{turbulent_push, Effect};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// Tutorial 2's setting, held still.
fn turb(units: &str) -> Effect {
    Effect::TurbulentDisplace {
        amount: 80.0,
        size: 2.0,
        complexity: 2.0,
        evolution: 0.0,
        speed: 0.0,
        seed: 0.0,
        edges: "transparent".into(),
        frame: 0,
        displacement: "turbulent".into(),
        pinning: "none".into(),
        units: units.into(),
        new_seed_every: 0.0,
        drift_direction: 0.0,
        drift_speed: 0.0,
    }
}

/// A 16:9 plate holding a soft gold glow round a white-hot middle, fading to clear.
const PLATE: (usize, usize) = (192, 108);

fn glow() -> Vec<u8> {
    let (w, h) = PLATE;
    (0..w * h)
        .flat_map(|i| {
            let (x, y) = ((i % w) as f64 + 0.5 - w as f64 / 2.0, (i / w) as f64 + 0.5 - h as f64 / 2.0);
            let r2 = x * x + y * y;
            let a = (-r2 / (2.0 * 22.0 * 22.0)).exp();
            let hot = (-r2 / (2.0 * 7.0 * 7.0)).exp();
            let c = |gold: f64| ((gold + (255.0 - gold) * hot).round()) as u8;
            [c(255.0), c(196.0), c(80.0), (a * 255.0).round() as u8]
        })
        .collect()
}

/// The glow with one Turbulent Displace of `settings` (FX-TURB-AE-001's for the rest), over
/// black, 8-bit.
fn picture(dir: &Path, settings: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("Fixtures/turbulent_ae/fx_turb_ae_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("glow.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(PLATE.0);
    comp["height"] = J::from(PLATE.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([PLATE.0 as f64 / 2.0, PLATE.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let parameters = &mut layer["effects"][0]["parameters"];
    parameters.as_object_mut().unwrap().remove("units");
    for (k, v) in settings.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(effect_table::MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    // Over black in linear light: the premultiplied colour itself, held to white.
    let black = frame
        .data()
        .chunks(4)
        .flat_map(|p| {
            let over = |c: f32| (anime_compositor::color::linear_to_srgb(c.clamp(0.0, 1.0)) * 255.0).round() as u8;
            [over(p[0]), over(p[1]), over(p[2]), 255]
        })
        .collect();
    (black, said)
}

/// How grainy a picture is: the mean step in red from each pixel to the one on its right.
fn grain(bytes: &[u8]) -> f64 {
    let (w, h) = PLATE;
    let mut sum = 0.0;
    for y in 0..h {
        for x in 0..w - 1 {
            sum += bytes[(y * w + x) * 4].abs_diff(bytes[(y * w + x + 1) * 4]) as f64;
        }
    }
    sum / ((w - 1) * h) as f64
}

/// The mean difference in red from another picture, in levels of 255.
fn apart(a: &[u8], b: &[u8]) -> f64 {
    a.chunks(4).zip(b.chunks(4)).map(|(p, q)| p[0].abs_diff(q[0]) as f64).sum::<f64>() / (a.len() / 4) as f64
}

#[test]
fn b212_turbulent_ae() {
    let mut t = Table::new(
        "turbulent_ae",
        "# D-328: Turbulent Displace in After Effects' units\n\nFrom P-26's tutorial 2, where \
         After Effects' Turbulent Displace at Amount 80, Size 2 leaves a glow smooth with a fine \
         shimmer and D-127's rule breaks it into dust. No source found says how After Effects \
         scales Amount with Size, so this program reads it as a push of amount x min(size, 100) \
         / 100 pixels: at Size 100 and above it is D-127's, at Size 2 Amount 80 pushes 1.6. \
         Turbulent Displace gains Units: After Effects, which a new one takes, and Classic, what \
         a file without units means, D-127's rule as before. Every expected pixel is \
         `Fixtures/turbulent_ae/expected_turbulent_ae.json`, written by \
         `tools/turbulent_ae_reference.py` before the build had it, printed in document 25 as \
         FX-TURB-AE-001 to 012. Tolerance 2e-5.\n",
    );

    t.heading("FX-TURB-AE-001 to 012 (document 25)");
    t.fixtures("expected_turbulent_ae.json");

    t.heading("The rule");
    for (amount, size, units, push) in [(80.0, 2.0, "after_effects", 1.6), (80.0, 2.0, "classic", 80.0), (3.0, 100.0, "after_effects", 3.0), (3.0, 1000.0, "after_effects", 3.0), (1000.0, 1000.0, "after_effects", 1000.0)] {
        let got = turbulent_push(amount, size, units);
        t.row(&format!("Amount {amount}, size {size}, {units}: the push is {push} pixels"), &format!("{got}"), got == push);
    }
    let mut drafted = turb("after_effects");
    drafted.scale_distances(|d| d * 0.5);
    let (a, s) = match &drafted { Effect::TurbulentDisplace { amount, size, .. } => (*amount, *size), _ => unreachable!() };
    let push = turbulent_push(a, s, "after_effects");
    t.row("A half-size draft of amount 80, size 2, After Effects units, pushes half of 1.6 (its size held at 1)", &format!("amount {a}, size {s}, push {push}"), (push - 0.8).abs() < 1e-12 && s == 1.0);
    let mut drafted = turb("classic");
    drafted.scale_distances(|d| d * 0.5);
    let a = match &drafted { Effect::TurbulentDisplace { amount, .. } => *amount, _ => unreachable!() };
    t.row("A half-size draft in Classic units pushes 40, as before D-328", &format!("amount {a}"), a == 40.0);

    t.heading("The file");
    t.round_trips(&["fx_turb_ae_001.json", "fx_turb_ae_002.json", "fx_turb_ae_003.json", "fx_turb_ae_008.json", "fx_turb_ae_011.json"]);
    let saved = t.saved_parameters("fx_turb_ae_002.json");
    t.row("fx_turb_ae_002.json, a Turbulent Displace with no units as before D-328, is saved without units", &saved.to_string(), saved.get("units").is_none());
    let saved = t.saved_parameters("fx_turb_ae_001.json");
    t.row("One in After Effects units is saved as units: after_effects", &saved.to_string(), saved["units"] == "after_effects");
    let saved = t.saved_parameters("fx_turb_ae_003.json");
    t.row("One whose file says units: classic keeps saying it", &saved.to_string(), saved["units"] == "classic");
    let old = effect_table::repo("Fixtures/turbulent_displace/fx_turb_001.json");
    let loaded = persist::load_str(&fs::read_to_string(&old).unwrap()).unwrap();
    let saved = effect_table::saved(&loaded);
    let parameters = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row("FX-TURB-001's file, from before D-328, is saved without units", &parameters.to_string(), parameters.get("units").is_none());

    t.heading("Commands");
    let mut document = t.load("fx_turb_ae_002.json").document;
    t.refused(
        &mut document,
        vec![("units \"ae\"", set(turb("ae"))), ("units \"After_Effects\", written with capitals", set(turb("After_Effects")))],
    );
    t.taken(&mut document, "fx_turb_ae_002.json", vec![("FX-TURB-AE-002 set to After Effects units,", set(turb("after_effects")))]);
    let mut document = t.load("fx_turb_ae_002.json").document;
    document.apply(set(turb("after_effects"))).unwrap();
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_turbulent_ae.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&document, 0, 64), &expected["cases"]["FX-TURB-AE-001"]["frames"]["0"]);
    t.row("FX-TURB-AE-002 set to After Effects units by the command draws FX-TURB-AE-001's frame", &format!("largest difference {d:.1e}"), d <= 2e-5);

    t.heading("The preview card");
    match Gpu::new() {
        Err(why) => t.row("The card draws the warp", &format!("NOT RUN: no usable card ({why})"), false),
        Ok(mut gpu) => {
            for name in ["fx_turb_ae_001.json", "fx_turb_ae_002.json", "fx_turb_ae_004.json", "fx_turb_ae_005.json", "fx_turb_ae_007.json", "fx_turb_ae_009.json", "fx_turb_ae_010.json"] {
                let project = t.load(name).document.project().clone();
                let comp = Id::new("comp-main");
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let cpu = preview::preview_frame_cached(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).unwrap().to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (card, ..) = preview::preview_frame_srgb8(&project, &comp, 0, &t.root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
                let on_cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let worst = cpu.iter().zip(&card).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
                t.row(
                    &format!("{name}: drawn on the card, within 1 level of 255 of the CPU"),
                    &format!("{}, largest difference {worst} of 255", if on_cpu { "CPU" } else { "card" }),
                    !on_cpu && worst <= 1,
                );
            }
        }
    }

    t.heading("Pictures: a soft glow, in `verification/D-328 pictures/`, over black");
    let dir = effect_table::repo("verification/D-328 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = PLATE;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    write("glow.png", &glow());
    let mut drawn = Vec::new();
    for (name, settings, what) in [
        ("1_glow.png", json!({ "amount": 0 }), "the glow with no warp"),
        ("2_classic_amount80_size2.png", json!({}), "Amount 80, Size 2 in an older project (Classic): broken into dust"),
        ("3_ae_amount80_size2.png", json!({ "units": "after_effects" }), "the same in After Effects units: the glow stays smooth, a fine shimmer at its edge"),
        ("4_ae_amount80_size100.png", json!({ "units": "after_effects", "size": 100 }), "Amount 80, Size 100 in After Effects units: Classic's rule, the whole glow carried and bent"),
    ] {
        let (bytes, said) = picture(&dir, settings);
        write(name, &bytes);
        t.row(&format!("{name}, {what}; draws cleanly"), &format!("{said:?}, grain {:.2}", grain(&bytes)), said.is_empty());
        drawn.push(bytes);
    }
    let (plain, classic, ae) = (&drawn[0], &drawn[1], &drawn[2]);
    t.row(
        "After Effects units at Amount 80, Size 2 stay close to the plain glow; Classic does not",
        &format!("mean difference from the plain glow: After Effects {:.2}, Classic {:.2} levels of 255", apart(ae, plain), apart(classic, plain)),
        apart(ae, plain) < 2.0 && apart(classic, plain) > 10.0 * apart(ae, plain),
    );
    t.row(
        "Classic is grainy (dust); After Effects units are about as smooth as the plain glow",
        &format!("grain: plain {:.2}, After Effects {:.2}, Classic {:.2}", grain(plain), grain(ae), grain(classic)),
        grain(ae) < 2.0 * grain(plain) + 0.5 && grain(classic) > 4.0 * grain(ae),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_turb_ae_001.json", 0), ("fx_turb_ae_002.json", 0), ("fx_turb_ae_010.json", 0)]);

    t.finish("D-328_turbulent_ae_table.md");
}
