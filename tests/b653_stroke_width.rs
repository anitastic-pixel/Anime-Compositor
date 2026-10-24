//! B-653: D-773, Path Stroke's Brush Size is the brush's radius, as After Effects draws it: the
//! line is twice Brush Size across and the dabs are Spacing per cent of that width apart.
//!
//! Writes `verification/B-653_stroke_width_table.md`. The fixtures' numbers are the reference's
//! (`tools/stroke_reference.py`), whose cases' sizes D-773 halved so their pixels stay; the widths
//! here are counted on a hard brush along a long straight edge, the pitch on a string of dabs.

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
const H: usize = 200;

/// A 240 by 200 solid with one mask, a box from (20, 40) to (220, 160), and Path Stroke on it,
/// On Transparent, with `p` over its settings.
fn shot(p: J) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let pt = |x: f64, y: f64| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]});
    let mut stroke = json!({"mask": 1, "all_masks": "off", "stroke_sequentially": "off", "color": "#ffffff",
        "brush_size": 10, "brush_hardness": 100, "opacity": 100, "start": 0, "end": 100, "spacing": 0,
        "paint_style": "on_transparent"});
    for (k, v) in p.as_object().unwrap() {
        stroke[k] = v.clone();
    }
    let path = t(json!({"points": [pt(20.0, 40.0), pt(220.0, 40.0), pt(220.0, 160.0), pt(20.0, 160.0)]}));
    let mask = json!({"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
        "feather_px": 0.0, "expansion_px": 0.0, "path": path});
    json!({
        "schema_version": 0, "project_id": "proj-b653",
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
                "solid": {"color": [0.2, 0.3, 0.4], "width": W, "height": H},
                "transform": {
                    "anchor": t(json!([120, 100])), "position": t(json!([120, 100])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "masks": [mask],
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.stroke", "enabled": true, "parameters": stroke}]
            }]
        }]
    })
    .to_string()
}

/// The covering (alpha) at every pixel.
fn alpha(project: &str) -> Vec<f32> {
    let loaded = persist::load_str(project).expect("the shot reads");
    let mut log = FrameLog::new(8);
    let f = render_frame(loaded.document.project(), &Id::new(MAIN), 0, std::path::Path::new("."), 64, &mut log)
        .expect("the shot draws");
    (0..W * H).map(|i| f.pixel(i % W, i / W)[3]).collect()
}

/// The rows of column 120 above row 100 covered at least half: the top edge's line, across.
fn width(project: &str) -> usize {
    let a = alpha(project);
    (0..100).filter(|y| a[y * W + 120] >= 0.5).count()
}

/// Where each covered run along row `y` starts, from x 40 to 200.
fn starts(project: &str, y: usize) -> Vec<usize> {
    let a = alpha(project);
    (40..200).filter(|&x| a[y * W + x] >= 0.5 && a[y * W + x - 1] < 0.5).collect()
}

#[test]
fn b653_stroke_width() {
    let mut t = Table::new(
        "stroke",
        "# B-653: Path Stroke's Brush Size is the brush's radius (D-773)\n\n\
         Seven tutorials from seven creators read After Effects' Stroke line 1.64 to 2.2 times Brush \
         Size across (median 1.91; 1.91 to 1.96 at Hardness 100), where ours drew 1 times. Path Stroke's \
         brush now has radius r = Brush Size, so the line is 2 Brush Size across, and its dabs are \
         Spacing per cent of that width apart. The widths are counted on a hard brush (Hardness 100, \
         Spacing 0) along the top edge of a box mask, as the rows of one column covered at least half; \
         the fixtures' numbers are the reference's, whose sizes D-773 halved so their pixels stay.\n",
    );

    t.heading("The line's width, in pixels covered at least half across the top edge");
    for size in [2.0, 10.0, 25.0] {
        let w = width(&shot(json!({"brush_size": size})));
        let want = (2.0 * size) as usize;
        t.row(
            &format!("Brush Size {size}, Hardness 100: {want} pixels across, twice the size (it was {size} before D-773)"),
            &format!("{w} pixels"),
            w == want,
        );
    }

    t.heading("The dabs: Spacing per cent of the width, 2 Brush Size, apart");
    for (size, spacing, pitch) in [(10.0, 100.0, 20usize), (10.0, 50.0, 10), (6.0, 100.0, 12)] {
        // A row inside the dabs' reach but off the path, where each dab covers its own run.
        let y = (40.0 + 0.9 * size) as usize;
        let s = starts(&shot(json!({"brush_size": size, "spacing": spacing})), y);
        let gaps: Vec<usize> = s.windows(2).map(|p| p[1] - p[0]).collect();
        t.row(
            &format!("Brush Size {size}, Spacing {spacing}: a dab every {pitch} pixels along the edge (row {y})"),
            &format!("runs start at {s:?}"),
            gaps.len() >= 3 && gaps.iter().all(|&g| g == pitch),
        );
    }

    t.heading("The fixtures, at the reference's numbers (tolerance 2e-5)");
    t.fixtures("expected_stroke.json");

    t.heading("An old project");
    let file = t.root.join("fx_stroke_001.json");
    let now = fs::read_to_string(&file).unwrap();
    let old = now.replace("\"brush_size\": 1.5", "\"brush_size\": 3");
    let loaded = persist::load_str(&old).expect("the old fx_stroke_001.json opens");
    let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
    t.row(
        "fx_stroke_001.json as saved before D-773 (Brush Size 3, its file at a259ec22) opens with no warning",
        &format!("{on_open:?}"),
        old != now && on_open.is_empty(),
    );
    let written: J = serde_json::from_str(&old).unwrap();
    let same = effect_table::same_json(&effect_table::saved(&loaded), &written);
    t.row(
        "and saves back holding what it held: Brush Size still 3, nothing added",
        if same { "the same" } else { "differs" },
        same,
    );
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_stroke.json")).unwrap()).unwrap();
    let mut log = FrameLog::new(8);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, &t.root, 64, &mut log).unwrap();
    let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    let d = largest_difference(&frame, &expected["cases"]["FX-STROKE-010"]["frames"]["0"]);
    t.row(
        "and draws a line twice as wide, with no warning: FX-STROKE-010's frame (the 6-pixel brush Brush Size 6 drew before)",
        &format!("largest difference {d:.1e}; warnings {said:?}"),
        d <= 2e-5 && said.is_empty(),
    );

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cases: Vec<(String, anime_compositor::model::Project, std::path::PathBuf)> = Vec::new();
    for (name, p) in [
        ("Brush Size 10, Hardness 100, Spacing 0", json!({})),
        ("Brush Size 25, Hardness 50, Spacing 0", json!({"brush_size": 25, "brush_hardness": 50})),
        ("Brush Size 10, Spacing 100, On Original Image", json!({"spacing": 100, "paint_style": "on_original"})),
        ("Brush Size 6, Hardness 79, Spacing 15", json!({"brush_size": 6, "brush_hardness": 79, "spacing": 15})),
    ] {
        let project = persist::load_str(&shot(p)).unwrap().document.project().clone();
        cases.push((name.to_string(), project, std::path::PathBuf::from(".")));
    }
    for n in [1u32, 6, 10, 22] {
        let name = format!("fx_stroke_{n:03}.json");
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

    t.finish("B-653_stroke_width_table.md");
}
