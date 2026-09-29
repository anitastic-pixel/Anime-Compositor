//! B-124c: a matted layer at Draft, the fault B-124's playtest sheet reported "found along the
//! way". The Draft step shrank each layer's placement to the smaller frame and not its matte's,
//! so the matte landed in the wrong place and the layer was lost.
//!
//! Writes `verification/B-124c_draft_matte_table.md` and `verification/B-124c pictures/`.
//!
//! No fixture holds a Draft frame with a matte, so the check is by what must be equal: an orange
//! layer the size of the frame, alpha-matted by a square, is exactly an orange square placed
//! where the matte is, at Full and at Draft alike. The Full frame is what an export writes and
//! the fault never touched it.

mod effect_table;

use std::fs;

use effect_table::{largest_difference, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::Document;
use anime_compositor::compose;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render;
use anime_compositor::{png_out, OutputDepth, WorkingBuffer};

const ORANGE: [f64; 3] = [1.0, 0.35, 0.05];

fn solid(id: &str, color: [f64; 3], (w, h): (u32, u32), at: [f64; 2], scale: f64, turn: f64) -> J {
    serde_json::json!({
        "id": id, "kind": "solid", "name": id, "enabled": true, "locked": false,
        "in_frame": 0, "out_frame": 1,
        "solid": {"color": color, "width": w, "height": h},
        "transform": {
            "anchor": {"base": [w as f64 / 2.0, h as f64 / 2.0], "keyframes": []},
            "position": {"base": at, "keyframes": []},
            "scale": {"base": [scale, scale], "keyframes": []},
            "rotation": {"base": turn, "keyframes": []},
            "opacity": {"base": 1, "keyframes": []}
        },
        "mask": null, "matte": null, "blend_mode": "normal", "effects": []
    })
}

fn composition(id: &str, layers: Vec<J>) -> J {
    let order: Vec<J> = layers.iter().map(|l| l["id"].clone()).collect();
    serde_json::json!({
        "id": id, "name": id, "width": 960, "height": 540, "pixel_aspect_ratio": 1,
        "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
        "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
        "layer_order": order, "layers": layers
    })
}

/// A dark ground and, over it, either the orange frame matted by a white 200 by 200 square
/// placed at `at`, scaled and turned, the square switched off and used only as the matte; or the
/// orange square itself placed so. `nested` puts the same inside a composition layer.
fn shot(matted: bool, at: [f64; 2], scale: f64, turn: f64, nested: bool) -> Document {
    let mut layers = vec![solid("ground", [0.02, 0.03, 0.06], (960, 540), [480.0, 270.0], 100.0, 0.0)];
    if matted {
        let mut picture = solid("picture", ORANGE, (960, 540), [480.0, 270.0], 100.0, 0.0);
        picture["matte"] = serde_json::json!({"layer_id": "square", "mode": "alpha", "matte_only": true});
        let mut square = solid("square", [1.0, 1.0, 1.0], (200, 200), at, scale, turn);
        square["enabled"] = J::Bool(false);
        layers.extend([picture, square]);
    } else {
        layers.push(solid("square", ORANGE, (200, 200), at, scale, turn));
    }
    let compositions = if nested {
        let mut inner = solid("inner", [0.0; 3], (960, 540), [480.0, 270.0], 100.0, 0.0);
        let record = inner.as_object_mut().unwrap();
        record.remove("solid");
        record.insert("kind".into(), "composition".into());
        record.insert("composition_id".into(), "comp-inner".into());
        record.insert("source_offset_frames".into(), 0.into());
        vec![composition(MAIN, vec![inner]), composition("comp-inner", layers)]
    } else {
        vec![composition(MAIN, layers)]
    };
    let project = serde_json::json!({
        "schema_version": 0, "project_id": "proj-b124c",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [], "compositions": compositions
    });
    persist::load_str(&project.to_string()).expect("the shot reads").document
}

fn draw(document: &Document, quality: PreviewQuality) -> WorkingBuffer {
    let root = effect_table::repo("Fixtures");
    let mut log = FrameLog::new(8);
    let frame = preview::preview_frame(document.project(), &Id::new(MAIN), 0, &root, quality, 64, &mut log)
        .expect("the frame draws");
    assert!(log.finish().is_empty(), "the shot draws without a word");
    frame
}

/// The Draft frame as it was drawn before this fix: every placement shrunk, the matte's not.
fn draft_before(document: &Document) -> WorkingBuffer {
    let root = effect_table::repo("Fixtures");
    let plan = compose::plan_frame(document.project(), &Id::new(MAIN), 0, &root, &mut FrameLog::new(8))
        .expect("the frame plans");
    let mut before = preview::scale_plan(plan.clone(), PreviewQuality::Draft);
    for (small, full) in before.layers.iter_mut().zip(&plan.layers) {
        if let (Some(small), Some(full)) = (&mut small.matte, &full.matte) {
            small.transform = full.transform;
        }
    }
    render::render(&before, 64)
}

/// The pixels within 1e-6 of the orange.
fn orange(frame: &WorkingBuffer) -> usize {
    frame
        .data()
        .chunks_exact(4)
        .filter(|p| (0..3).all(|c| (p[c] as f64 - ORANGE[c]).abs() <= 1e-6))
        .count()
}

/// Each Draft pixel drawn 4 times bigger, so it lies beside the Full frame at the same size.
fn enlarged(frame: &WorkingBuffer, by: usize) -> WorkingBuffer {
    let (w, h) = (frame.width() * by, frame.height() * by);
    let mut out = WorkingBuffer::transparent(w, h);
    for (i, px) in out.data_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (i % w / by, i / w / by);
        px.copy_from_slice(&frame.data()[(y * frame.width() + x) * 4..][..4]);
    }
    out
}

#[test]
fn b124c_draft_matte_table() {
    let mut t = Table::new(
        "",
        "# B-124c: a matted layer at Draft\n\n\
         Written by `tests/b124c_draft_matte.rs`. B-124's playtest sheet reported that a layer \
         with a matte disappears at Draft: the Draft step shrank each layer's placement to the \
         smaller frame, and not its matte's. It now shrinks both. Each shot is 960 by 540: a \
         dark ground, and an orange layer the size of the frame alpha-matted by a white 200 by \
         200 square, which must be exactly an orange 200 by 200 square placed where the matte is. \
         Full is what an export writes and was never touched. Tolerance 1e-6.\n",
    );
    let places: [(&str, [f64; 2], f64, f64, bool); 4] = [
        ("the square straight, its corner at (300, 148)", [400.0, 248.0], 100.0, 0.0, false),
        ("the square turned 30 degrees in the middle", [480.0, 270.0], 100.0, 30.0, false),
        ("the square at 150% turned 45 degrees, near a corner", [760.0, 140.0], 150.0, 45.0, false),
        ("the square turned 30 degrees, inside a composition layer", [480.0, 270.0], 100.0, 30.0, true),
    ];
    t.heading("The matted layer is the square it is matted by");
    for &(what, at, scale, turn, nested) in &places {
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let matted = draw(&shot(true, at, scale, turn, nested), quality);
            let square = draw(&shot(false, at, scale, turn, nested), quality);
            let d = largest_difference(&matted, &J::Array(
                square.data().chunks_exact(4).map(|p| p.iter().map(|&v| J::from(v as f64)).collect()).collect(),
            ));
            let size = (matted.width(), matted.height());
            let want = if quality == PreviewQuality::Full { (960, 540) } else { (240, 135) };
            t.row(
                &format!("{what}, at {quality:?}: {} by {}, and the same as the orange square", want.0, want.1),
                &format!("{} by {}, largest difference {d:.1e}", size.0, size.1),
                size == want && d <= 1e-6,
            );
        }
    }

    t.heading("Draft is Full a quarter of the size each way");
    let (_, at, scale, turn, nested) = places[0];
    let full = orange(&draw(&shot(true, at, scale, turn, nested), PreviewQuality::Full));
    let draft = orange(&draw(&shot(true, at, scale, turn, nested), PreviewQuality::Draft));
    let before = orange(&draft_before(&shot(true, at, scale, turn, nested)));
    t.row(
        "the straight square: 40,000 orange pixels at Full, and 40,000 / 16 = 2,500 at Draft",
        &format!("{full} at Full, {draft} at Draft"),
        full == 40_000 && draft == 2_500,
    );
    t.row(
        "the same Draft frame drawn as it was before the fix has no orange pixel: the fault",
        &format!("{before}"),
        before == 0,
    );

    // Full, then Draft before and after the fix enlarged to the same size, side by side.
    let (_, at, scale, turn, nested) = places[1];
    let document = shot(true, at, scale, turn, nested);
    let panels = [
        draw(&document, PreviewQuality::Full),
        enlarged(&draft_before(&document), 4),
        enlarged(&draw(&document, PreviewQuality::Draft), 4),
    ];
    let gap = 16;
    let (w, h) = (3 * 960 + 2 * gap, 540);
    let mut sheet = [255u8, 255, 255, 255].repeat(w * h);
    for (i, panel) in panels.iter().enumerate() {
        let rgba = panel.to_srgb8_straight();
        for y in 0..540 {
            let to = (y * w + i * (960 + gap)) * 4;
            sheet[to..to + 960 * 4].copy_from_slice(&rgba[y * 960 * 4..][..960 * 4]);
        }
    }
    let dir = effect_table::repo("verification/B-124c pictures");
    fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("before_after.png"), w, h, OutputDepth::Eight, &[], &sheet).unwrap();
    t.heading("The picture");
    t.row(
        "`verification/B-124c pictures/before_after.png`: the turned square at Full; at Draft \
         before the fix, empty; at Draft now, the same square; both Draft frames drawn 4 times \
         bigger",
        "written",
        true,
    );

    t.finish("B-124c_draft_matte_table.md");
}
