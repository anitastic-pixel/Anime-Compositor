//! B-655: D-775, a mask's feather blurs at sigma 0.375 x feather, After Effects' mask edge as
//! seven of eight tutorials measure it (it was feather / 2).
//!
//! Writes `verification/B-655_mask_feather_table.md`. The fixtures' numbers are the reference's
//! (`tools/mask_reference.py`); the edge is read off a white solid's rectangle mask.

mod effect_table;

use std::fs;

use effect_table::{both, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

const W: usize = 240;
const H: usize = 240;

/// A 240 by 240 composition holding a white 240 by 240 solid with one rectangle mask over
/// rows and columns 70 to 170, `m` over the mask's settings.
fn shot(m: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let pt = |x: f64, y: f64| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]});
    let mut mask = json!({
        "name": "Mask 1", "enabled": true, "inverted": false, "mode": "add", "opacity": 1.0,
        "feather_px": 10.0, "expansion_px": 0.0,
        "path": {"base": {"points": [pt(70.0, 70.0), pt(170.0, 70.0), pt(170.0, 170.0), pt(70.0, 170.0)]}, "keyframes": []}
    });
    for (k, v) in m.as_object().unwrap() {
        mask[k] = v.clone();
    }
    json!({
        "schema_version": 0, "project_id": "proj-b655",
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
                "solid": {"color": [1.0, 1.0, 1.0], "width": W, "height": H},
                "transform": {
                    "anchor": t(json!([120, 120])), "position": t(json!([120, 120])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [mask],
                "matte": null, "blend_mode": "normal", "effects": []
            }]
        }]
    })
    .to_string()
}

/// The mask's edge: how far above its top edge (y 70), along column 120, the covering falls to
/// 0.1587, one sigma out on a Gaussian-blurred edge, read between pixel centres.
fn sigma(project: &str) -> f64 {
    let loaded = persist::load_str(project).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log)
        .expect("the shot draws");
    let a = |k: usize| f.pixel(120, 69 - k)[3] as f64; // k + 0.5 above the edge
    let level = 0.158_655;
    if a(0) < level {
        return 0.0; // nothing above the edge: a hard one
    }
    for k in 0..69 {
        if a(k + 1) < level {
            return k as f64 + 0.5 + (a(k) - level) / (a(k) - a(k + 1));
        }
    }
    f64::NAN
}

#[test]
fn b655_mask_feather() {
    let mut t = Table::new(
        "masks",
        "# B-655: A mask's feather blurs at sigma 0.375 x feather (D-775)\n\n\
         Eight tutorials from eight creators fit After Effects' mask edge; seven put its sigma at \
         0.345 to 0.398 x feather (about 0.375), and ours, at feather / 2, read 1.27 to 1.43 times as \
         wide (median 1.335; Graphician 1.94, an outlier on the same side). A mask's covering is now \
         blurred at sigma 0.375 x feather. The edge is read off a white solid's 100 by 100 rectangle \
         mask: how far out its covering falls to 0.1587, one sigma on a Gaussian's edge. The \
         fixtures' numbers are the reference's. Display Mix's FX-MIX8-009 and 010 (8 bpc and Display) \
         are checked by b642_display_mix.\n",
    );

    t.heading("The mask's edge: sigma, in pixels, within 3 per cent");
    for feather in [4.0, 8.0, 20.0, 40.0] {
        let s = sigma(&shot(json!({"feather_px": feather})));
        let want = 0.375 * feather;
        t.row(
            &format!("Feather {feather}: sigma {want} (it was {} before D-775)", feather / 2.0),
            &format!("{s:.2} pixels"),
            (s / want - 1.0).abs() <= 0.03,
        );
    }
    let s = sigma(&shot(json!({"feather_px": 0.0})));
    t.row("Feather 0: a hard edge, as before (nothing covered above it)", &format!("{s:.2} pixels"), s == 0.0);

    t.heading("The fixtures, at the reference's numbers (tolerance 1e-6 and 2e-5)");
    // FX-MSK-018 and 019 (no feather; a diagnosed outline) are b24b_masks' own checks.
    t.fixtures_numbered("expected_masks.json", 1..=17);
    t.fixtures_numbered("expected_masks.json", 31..=35);
    let masks = t.root.clone();
    t.root = effect_table::repo("Fixtures/energy_stroke_extras");
    t.fixtures_numbered("expected_energy_stroke_extras.json", 22..=22);
    t.root = masks;

    t.heading("An old project");
    let file = fs::read_to_string(t.root.join("fx_msk_014.json")).unwrap();
    let loaded = persist::load_str(&file).expect("fx_msk_014.json opens");
    let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
    t.row(
        "fx_msk_014.json (feather 2, its file unchanged since before D-775) opens with no warning",
        &format!("{on_open:?}"),
        on_open.is_empty(),
    );
    let written: J = serde_json::from_str(&file).unwrap();
    let same = effect_table::same_json(&effect_table::saved(&loaded), &written);
    t.row(
        "and saves back holding what it held: feather still 2, nothing added",
        if same { "the same" } else { "differs" },
        same,
    );
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, &t.root, 64, &mut log).unwrap();
    let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    let a = frame.pixel(1, 0)[3] as f64;
    t.row(
        "and draws a tighter edge (sigma 0.75): column 1's covering 0.7275, not the 0.5662 it drew at sigma 1, with no warning",
        &format!("{a:.4}; warnings {said:?}"),
        (a - 0.7275).abs() < 1e-3 && said.is_empty(),
    );

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cases: Vec<(String, anime_compositor::model::Project, std::path::PathBuf)> = Vec::new();
    for (name, m) in [
        ("Feather 10", json!({})),
        ("Feather 40, expansion 6", json!({"feather_px": 40, "expansion_px": 6})),
        ("Feather 25, inverted, opacity 0.6", json!({"feather_px": 25, "inverted": true, "opacity": 0.6})),
    ] {
        let project = persist::load_str(&shot(m)).unwrap().document.project().clone();
        cases.push((name.to_string(), project, std::path::PathBuf::from(".")));
    }
    let p = t.load("fx_msk_014.json").document.project().clone();
    cases.push(("fx_msk_014.json".into(), p, t.root.clone()));
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

    t.finish("B-655_mask_feather_table.md");
}
