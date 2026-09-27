//! B-67: the light rays in the core, against D-124.
//!
//! Writes `verification/B-67_light_rays_table.md`.
//!
//! Every expected pixel is `Fixtures/light_rays/expected_light_rays.json`, written by
//! `tools/light_rays_reference.py` before this code existed and printed in document 25 as
//! FX-RAYS-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn rays(center: [f64; 2], length: f64, threshold: f64, intensity: f64, color: &str) -> Effect {
    Effect::LightRays {
        center,
        length,
        threshold,
        intensity,
        color: color.to_string(),
    }
}

#[test]
fn b67_light_rays() {
    let mut t = Table::new(
        "light_rays",
        "# B-67: light rays\n\nD-124, accepted by the owner on 2026-09-26, the second of the \
         second batch of ten. Every expected pixel is \
         `Fixtures/light_rays/expected_light_rays.json`, written by \
         `tools/light_rays_reference.py` before this code existed and printed in document 25 as \
         FX-RAYS-001 to 024. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-RAYS-001 to 024 (document 25)");
    t.fixtures("expected_light_rays.json");

    t.heading("How far it reaches");
    let got = rays([0.0, 0.0], 100.0, 0.0, 10.0, "#ffffff").bounds_expansion();
    t.row(
        "it grows the drawing's bounds by nothing: rays past the layer's edge are cut",
        &got.to_string(),
        got == 0,
    );
    let mut draft = rays([30.0, 70.0], 60.0, 70.0, 1.0, "#ffffff");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: the length and the centre are shares of \
         the drawing",
        &format!("{draft:?}"),
        draft == rays([30.0, 70.0], 60.0, 70.0, 1.0, "#ffffff"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_rays_001.json",
        "fx_rays_008.json",
        "fx_rays_012.json",
        "fx_rays_013.json",
        "fx_rays_015.json",
        "fx_rays_016.json",
        "fx_rays_017.json",
        "fx_rays_018.json",
        "fx_rays_021.json",
        "fx_rays_022.json",
        "fx_rays_023.json",
        "fx_rays_024.json",
    ]);
    t.shape_refused(
        "fx_rays_001.json",
        "no `center` at all",
        r##"{"length": 50, "threshold": 70, "intensity": 1, "color": "#ffffff"}"##,
    );
    t.shape_refused(
        "fx_rays_001.json",
        "a centre of one number",
        r##"{"center": 50, "length": 50, "threshold": 70, "intensity": 1, "color": "#ffffff"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_rays_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("length 101", set(rays([50.0, 50.0], 101.0, 70.0, 1.0, "#ffffff"))),
            ("threshold -1", set(rays([50.0, 50.0], 50.0, -1.0, 1.0, "#ffffff"))),
            ("intensity 11", set(rays([50.0, 50.0], 50.0, 70.0, 11.0, "#ffffff"))),
            ("centre 1001, 50", set(rays([1001.0, 50.0], 50.0, 70.0, 1.0, "#ffffff"))),
            ("colour \"orange\"", set(rays([50.0, 50.0], 50.0, 70.0, 1.0, "orange"))),
            ("intensity keyed to 20", keys("intensity", &[(0, &[1.0]), (4, &[20.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_rays_001.json",
        vec![
            (
                "length 100, threshold 100, intensity 10 and centre 1000, 1000, the tops,",
                set(rays([1000.0, 1000.0], 100.0, 100.0, 10.0, "#ffffff")),
            ),
            (
                "length 0, threshold 0, intensity 0 and centre -1000, -1000, the bottoms,",
                set(rays([-1000.0, -1000.0], 0.0, 0.0, 0.0, "#000000")),
            ),
            ("centre keyed from 50, 50 to 0, 0", keys("center", &[(0, &[50.0, 50.0]), (4, &[0.0, 0.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_rays_001.json", 0), ("fx_rays_017.json", 3)]);

    t.finish("B-67_light_rays_table.md");
}
