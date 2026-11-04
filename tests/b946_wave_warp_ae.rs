//! B-946: D-1066, Wave Warp after After Effects: the `rule` word. Under `after_effects` Width is
//! half a wave and Speed waves a second, and Noise and Smooth Noise are jagged streaks lying
//! along the wave (G5 of the wave 12 report, the liquid gradient tutorial lHQvi94dpvM). A file
//! without the word keeps D-149's rule: FX-WAVE-001 to 047 are unchanged and pass in B-92,
//! B-624 and B-634.
//!
//! Writes `verification/B-946_wave_warp_ae_table.md` and draws pictures into
//! `verification/B-946 pictures/`.

mod effect_table;

use std::fs;

use effect_table::{both, distance, on_card, reference, repo, saved, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// A Wave Warp as added (After Effects' rule and settings), with `p` over it.
fn wave(p: &J) -> J {
    let mut all = json!({"shape": "sine", "height": 10, "width": 40, "direction": 90, "speed": 1,
        "phase": 0, "edges": "transparent", "pinning": "none", "rule": "after_effects"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": "w", "type_id": "core.wave_warp", "enabled": true, "parameters": all})
}

/// The street still (480 by 270, 24 frames a second) with `effects`, as a project.
fn project(effects: J) -> J {
    let size = (480, 270);
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("1_left.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    comp["duration_frames"] = J::from(96);
    comp["work_area"]["end_frame_exclusive"] = J::from(96);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(96);
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    project
}

/// `effects` on the street still at `frame`, drawn on the processor, straight 8-bit, with what
/// it warned of.
fn picture(effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let loaded = persist::load_str(&project(effects).to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        &repo("verification/D-506 pictures"),
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (frame.to_srgb8_straight(), said)
}

fn write(name: &str, bytes: &[u8]) {
    let out = repo("verification/B-946 pictures");
    fs::create_dir_all(&out).unwrap();
    png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], bytes)
        .unwrap();
}

#[test]
fn b946_wave_warp_ae() {
    let mut t = Table::new(
        "wave_warp",
        "# D-1066: Wave Warp after After Effects\n\nB-946. Wave Warp's new `rule` word: \
         `after_effects` (as added) reads Width as half a wave and Speed as waves a second, and \
         draws Noise and Smooth Noise as jagged streaks along the wave; a file without the word \
         keeps D-149's rule. FX-WAVE-001 to 047 are unchanged and pass in B-92, B-624 and B-634.\n",
    );

    t.heading("The file");
    let old = t.saved_parameters("fx_wave_001.json");
    t.row(
        "fx_wave_001.json, written before the rule, is saved as it was: no rule",
        &format!("{} settings; rule {}", old.as_object().unwrap().len(), old["rule"]),
        old.get("rule").is_none(),
    );
    let loaded = persist::load_str(&project(json!([wave(&json!({}))])).to_string()).unwrap();
    let s = saved(&loaded);
    let p = &s["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "a Wave Warp as added is saved with its rule, after_effects",
        &format!("rule {}, width {}, speed {}", p["rule"], p["width"], p["speed"]),
        p["rule"] == "after_effects",
    );
    let (_, said) = picture(json!([wave(&json!({"rule": "spin"}))]), 6);
    let said = said.join(" ");
    t.row(
        "a Wave Warp whose rule is \"spin\" is left out with a warning",
        &said,
        said.contains("EFFECT_PARAMETER_INVALID"),
    );

    t.heading("The units: After Effects' Width 40 and Speed 1 at 24 frames a second, frame 6");
    for shape in ["sine", "triangle", "circle", "square", "sawtooth", "semicircle"] {
        let ae = picture(json!([wave(&json!({"shape": shape, "height": 20}))]), 6);
        let before = picture(
            json!([wave(&json!({"shape": shape, "height": 20, "rule": "d149", "width": 80, "speed": 15}))]),
            6,
        );
        let d = distance(&ae.0, &before.0);
        t.row(
            &format!("{shape}: the same as D-149's rule at Width 80 and Speed 15 degrees a frame"),
            &format!("largest difference {} of 255, {} pixels differ; warnings {:?} {:?}", d.0, d.1, ae.1, before.1),
            d.1 == 0 && ae.1.is_empty() && before.1.is_empty(),
        );
    }
    let first = picture(json!([wave(&json!({"height": 20}))]), 0).0;
    let a_second = picture(json!([wave(&json!({"height": 20}))]), 24).0;
    let d = distance(&first, &a_second);
    t.row(
        "Speed 1: frame 24 (one second on) is frame 0 again, the wave moved one whole wave (within 1 level, the rounding of a whole turn)",
        &format!("largest difference {} of 255, {} pixels differ", d.0, d.1),
        d.0 <= 1,
    );

    t.heading("Noise and Smooth Noise (the tutorial's Height 500 over Width 526, scaled to Height 20 over 21)");
    let street = picture(json!([]), 0).0;
    for shape in ["noise", "smooth_noise"] {
        let p = json!({"shape": shape, "height": 20, "width": 21, "direction": 45, "speed": 0});
        let ae = picture(json!([wave(&p)]), 0);
        let mut old = p.clone();
        old["rule"] = J::from("d149");
        old["width"] = J::from(42);
        let before = picture(json!([wave(&old)]), 0);
        write(&format!("1_before_{shape}"), &before.0);
        write(&format!("2_after_effects_{shape}"), &ae.0);
        t.row(
            &format!("2_after_effects_{shape}.png: drawn, and not D-149's {shape} (1_before_{shape}.png)"),
            &format!(
                "{} pixels moved from the street, {} pixels differ from D-149's; warnings {:?}",
                distance(&ae.0, &street).1,
                distance(&ae.0, &before.0).1,
                ae.1
            ),
            ae.1.is_empty() && distance(&ae.0, &street).1 > 0 && distance(&ae.0, &before.0).1 > 0,
        );
    }

    t.heading("On the card");
    let mut gpu = Gpu::new().expect("a graphics card");
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added", json!({})),
        ("noise", json!({"shape": "noise", "height": 30, "width": 60, "direction": 45})),
        ("smooth noise", json!({"shape": "smooth_noise", "height": 30, "width": 60, "direction": 45})),
        ("the D-149 rule, smooth noise", json!({"shape": "smooth_noise", "height": 30, "rule": "d149"})),
    ] {
        let project = reference(|id| {
            let mut m = wave(&p);
            m["instance_id"] = J::from(format!("b946-{id}"));
            json!([m])
        });
        for frame in [100, 239] {
            let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, PreviewQuality::Full);
            let card = on_card(&project, &ref_comp, &ref_root, frame, PreviewQuality::Full, |e| {
                matches!(e, Effect::WaveWarp { .. })
            });
            t.row(
                &format!("the reference shot, Wave Warp {what} on three layers, frame {frame}, Full"),
                &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                d.0 <= 1 && !r && a == b && card == 3,
            );
        }
    }
    t.finish("B-946_wave_warp_ae_table.md");
}
