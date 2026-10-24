//! B-654: D-774, Drop Shadow's softness blurs at sigma softness / 5, After Effects' shadow edge
//! as seven tutorials measure it (it was softness / 3).
//!
//! Writes `verification/B-654_drop_shadow_softness_table.md`. The fixtures' numbers are the
//! reference's (`tools/drop_shadow_reference.py`), whose softened cases' softness D-774 raised by
//! 5 / 3 so their pixels stay; the edge is read off a white square's black shadow.

mod effect_table;

use std::fs;

use effect_table::{both, largest_difference, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

const W: usize = 240;
const H: usize = 240;

/// A 240 by 240 composition holding a white 100 by 100 solid in its middle (rows and columns 70
/// to 169) with Drop Shadow on it, black, Opacity 100, Distance 0, `p` over its settings.
fn shot(p: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut shadow = json!({"color": "#000000", "opacity": 100, "direction": 135, "distance": 0, "softness": 10});
    for (k, v) in p.as_object().unwrap() {
        shadow[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b654",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1.0, 1.0, 1.0], "width": 100, "height": 100},
                "transform": {
                    "anchor": t(json!([50, 50])), "position": t(json!([120, 120])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.drop_shadow", "enabled": true, "parameters": shadow}]
            }]
        }]
    })
    .to_string()
}

/// The shadow's edge: how far above the square's top edge (row 70), along column 120, its
/// covering falls to 0.1587, one sigma out on a Gaussian's edge, read between pixel centres.
fn sigma(project: &str) -> f64 {
    let loaded = persist::load_str(project).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log)
        .expect("the shot draws");
    let a = |k: usize| f.pixel(120, 69 - k)[3] as f64; // k + 0.5 above the edge
    let level = 0.158_655;
    for k in 0..69 {
        if a(k + 1) < level {
            return k as f64 + 0.5 + (a(k) - level) / (a(k) - a(k + 1));
        }
    }
    f64::NAN
}

#[test]
fn b654_drop_shadow_softness() {
    let mut t = Table::new(
        "drop_shadow",
        "# B-654: Drop Shadow's softness blurs at sigma softness / 5 (D-774)\n\n\
         Seven tutorials from seven creators fit After Effects' Drop Shadow edge at sigma softness / 5.04 \
         (ours, at softness / 3, read 1.57 to 1.83 times as wide, median 1.68). The shadow is now the \
         drawing's covering blurred at sigma softness / 5 and grows the layer by that blur's reach. The \
         edge is read off a white 100 by 100 square's black shadow at Opacity 100 and Distance 0: how \
         far out its covering falls to 0.1587, one sigma on a Gaussian's edge; the fixtures' numbers are \
         the reference's, whose softened cases' softness D-774 raised by 5 / 3 so their pixels stay.\n",
    );

    t.heading("The shadow's edge: sigma, in pixels, within 3 per cent");
    for softness in [10.0, 25.0, 50.0, 100.0] {
        let s = sigma(&shot(json!({"softness": softness})));
        let want = softness / 5.0;
        t.row(
            &format!("Softness {softness}: sigma {want} (it was {:.2} before D-774)", softness / 3.0),
            &format!("{s:.2} pixels"),
            (s / want - 1.0).abs() <= 0.03,
        );
    }

    t.heading("The fixtures, at the reference's numbers (tolerance 2e-5)");
    t.fixtures("expected_drop_shadow.json");

    t.heading("An old project");
    let now = fs::read_to_string(t.root.join("fx_shadow_008.json")).unwrap();
    let old = now.replace("\"softness\": 5", "\"softness\": 3");
    let loaded = persist::load_str(&old).expect("the old fx_shadow_008.json opens");
    let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
    t.row(
        "fx_shadow_008.json as saved before D-774 (softness 3, its file at 621e141b) opens with no warning",
        &format!("{on_open:?}"),
        old != now && on_open.is_empty(),
    );
    let written: J = serde_json::from_str(&old).unwrap();
    let same = effect_table::same_json(&effect_table::saved(&loaded), &written);
    t.row(
        "and saves back holding what it held: softness still 3, nothing added",
        if same { "the same" } else { "differs" },
        same,
    );
    let expected: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("expected_drop_shadow.json")).unwrap()).unwrap();
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, &t.root, 64, &mut log).unwrap();
    let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    let d = largest_difference(&frame, &expected["cases"]["FX-SHADOW-008"]["frames"]["0"]);
    t.row(
        "and draws a tighter shadow (sigma 0.6), not FX-SHADOW-008's sigma 1, with no warning",
        &format!("largest difference from FX-SHADOW-008 {d:.1e}; warnings {said:?}"),
        d > 1e-3 && said.is_empty(),
    );

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cases: Vec<(String, anime_compositor::model::Project, std::path::PathBuf)> = Vec::new();
    for (name, p) in [
        ("Softness 10, Distance 0", json!({})),
        ("Softness 50, Distance 20, Opacity 60", json!({"softness": 50, "distance": 20, "opacity": 60})),
        ("Softness 100, Distance 7.5, direction 200, colour #2040a0",
         json!({"softness": 100, "distance": 7.5, "direction": 200, "color": "#2040a0"})),
        ("Softness 25, Shadow Only on", json!({"softness": 25, "distance": 10, "shadow_only": "on"})),
    ] {
        let project = persist::load_str(&shot(p)).unwrap().document.project().clone();
        cases.push((name.to_string(), project, std::path::PathBuf::from(".")));
    }
    for n in [8u32, 9, 27] {
        let name = format!("fx_shadow_{n:03}.json");
        let p = t.load(&name).document.project().clone();
        cases.push((name, p, t.root.clone()));
    }
    for (name, project, root) in &cases {
        let (mut largest, mut refused, mut agree) = (0, false, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let (d, r, a, b) = both(&mut gpu, project, &Id::new(MAIN), root, 0, quality);
            largest = largest.max(d.0);
            refused |= r;
            agree &= a == b;
        }
        t.row(
            &format!("{name}, frame 0 at Full and Draft"),
            &format!("largest difference {largest} of 255; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree,
        );
    }

    t.finish("B-654_drop_shadow_softness_table.md");
}
