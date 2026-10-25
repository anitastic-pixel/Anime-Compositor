//! B-709: D-829, Drop Shadow's softness up to 2000, as After Effects' panel takes it (500 was
//! the top before).
//!
//! Writes `verification/B-709_drop_shadow_softness_2000_table.md`. The fixtures' numbers are the
//! reference's (`tools/drop_shadow_reference.py`, FX-SHADOW-031 to 033, and FX-SHADOW-020 now at
//! 2001); the timings are this machine's, measured here, not a claim beyond it.
#![recursion_limit = "256"]

mod effect_table;

use std::fs;
use std::time::Instant;

use effect_table::{both, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

const PAGE: &str = include_str!("../app/ui/index.html");

/// A 1920 by 1080 composition holding a white 960 by 200 solid in its middle, a title's size,
/// with Drop Shadow on it: black, Opacity 100, Direction 135, Distance 20, Softness `softness`.
fn title(softness: f64) -> String {
    let t = |v: J| json!({"base": v, "keyframes": []});
    json!({
        "schema_version": 0, "project_id": "proj-b709",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{
            "id": MAIN, "name": MAIN, "width": 1920, "height": 1080, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 1, "work_area": {"start_frame": 0, "end_frame_exclusive": 1},
            "layer_order": ["art"],
            "layers": [{
                "id": "art", "kind": "solid", "name": "art", "enabled": true, "locked": false,
                "in_frame": 0, "out_frame": 1,
                "solid": {"color": [1.0, 1.0, 1.0], "width": 960, "height": 200},
                "transform": {
                    "anchor": t(json!([480, 100])), "position": t(json!([960, 540])),
                    "scale": t(json!([100, 100])), "rotation": t(json!(0)), "opacity": t(json!(1))
                },
                "matte": null, "blend_mode": "normal",
                "effects": [{"instance_id": "f", "type_id": "core.drop_shadow", "enabled": true,
                    "parameters": {"color": "#000000", "opacity": 100, "direction": 135,
                                   "distance": 20, "softness": softness}}]
            }]
        }]
    })
    .to_string()
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
fn b709_drop_shadow_softness_2000() {
    let mut t = Table::new(
        "drop_shadow",
        "# B-709: Drop Shadow's softness up to 2000\n\nD-829. After Effects' Drop Shadow panel \
         takes Softness 2000 (Smertimba, https://www.youtube.com/watch?v=gsihkt5sO2w, 2:03); ours \
         stopped at 500 and refused more. The rule is unchanged, sigma softness / 5 (D-774). The \
         fixtures' numbers are `tools/drop_shadow_reference.py`'s, worked before this code; every \
         FX-SHADOW case is also run by B-58's table.\n",
    );

    t.heading("FX-SHADOW-020 and 031 to 033 (document 25)");
    t.fixtures_numbered("expected_drop_shadow.json", 31..=33);
    t.fixtures_numbered("expected_drop_shadow.json", 20..=20);

    // On the 16 by 10 frame a sigma-400 shadow is faint, near the 2e-5 tolerance; held here to a
    // thousandth of each pixel's own covering as well.
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_drop_shadow.json")).unwrap(),
    )
    .unwrap();
    for n in 31..=33 {
        let case = &expected["cases"][format!("FX-SHADOW-0{n}")];
        let loaded = t.load(case["project"].as_str().unwrap());
        let f = t.render(&loaded.document, 0, 64);
        let want = case["frames"]["0"].as_array().unwrap();
        let worst = want
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let w = p[3].as_f64().unwrap();
                (f.pixel(i % 16, i / 16)[3] as f64 - w).abs() / w
            })
            .fold(0.0, f64::max);
        t.row(
            &format!("FX-SHADOW-0{n}: every pixel's covering within a thousandth of its own value"),
            &format!("largest share off {worst:.1e}"),
            worst < 1e-3,
        );
    }

    t.heading("The panel");
    t.row(
        "the Effect Controls hint reads 0 to 2000",
        "core.drop_shadow's Softness row",
        PAGE.contains("['softness', 'Softness', 1, 0.5, 'pixels, 0 to 2000']]"),
    );

    t.heading("What it costs: a 960 by 200 title in a 1920 by 1080 frame, distance 20");
    let mut gpu = Gpu::new().ok();
    for softness in [100.0, 500.0, 1000.0, 2000.0] {
        let loaded = persist::load_str(&title(softness)).expect("the title reads");
        let project = loaded.document.project();
        let comp = Id::new(MAIN);
        let root = std::path::Path::new(".");
        let cpu = median(
            (0..3)
                .map(|_| {
                    let mut log = FrameLog::new(8);
                    let start = Instant::now();
                    render_frame(project, &comp, 0, root, DEFAULT_TILE_SIZE, &mut log)
                        .expect("the CPU frame draws");
                    start.elapsed().as_secs_f64() * 1000.0
                })
                .collect(),
        );
        let card = match gpu.as_mut() {
            None => "no card".to_string(),
            Some(gpu) => {
                let ((levels, pixels), refused, _, said) =
                    both(gpu, project, &comp, root, 0, PreviewQuality::Full);
                let mut cache = CelCache::viewer();
                let ms = median(
                    (0..3)
                        .map(|_| {
                            let mut log = FrameLog::new(8);
                            let start = Instant::now();
                            preview::preview_frame_srgb8(
                                project,
                                &comp,
                                0,
                                root,
                                PreviewQuality::Full,
                                DEFAULT_TILE_SIZE,
                                &mut log,
                                &mut cache,
                                gpu,
                            )
                            .expect("the card's frame draws");
                            cache = CelCache::viewer();
                            start.elapsed().as_secs_f64() * 1000.0
                        })
                        .collect(),
                );
                let ok = !refused && levels <= 1;
                t.row(
                    &format!("softness {softness}: the card draws it, within 1 level of the CPU"),
                    &format!("{levels} levels at most, {pixels} pixels differ; {said}"),
                    ok,
                );
                format!("card {ms:.0} ms")
            }
        };
        t.row(
            &format!("softness {softness} (sigma {}): time to draw the frame, median of 3", softness / 5.0),
            &format!("CPU {cpu:.0} ms, {card}"),
            true,
        );
    }

    t.finish("B-709_drop_shadow_softness_2000_table.md");
}
