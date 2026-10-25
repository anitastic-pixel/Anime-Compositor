//! B-718: D-838, Radial Blur's zoom smears both ways along the line from the centre, a quarter of
//! its amount either side of each pixel, as After Effects' does (it was D-110's whole amount,
//! outward only).
//!
//! Writes `verification/B-718_radial_zoom_table.md`. The fixtures' numbers are the references'
//! (`tools/radial_blur_reference.py`, `tools/edges_reference.py`); Light Rays, Spin & Zoom Blur
//! and Fast Zoom Blur keep D-110's outward zoom and their fixtures do not move.

mod effect_table;

use std::fs;

use effect_table::{both, largest_difference, repo, Table, MAIN};
use serde_json::Value as J;


use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;

fn expected(folder: &str, file: &str) -> J {
    serde_json::from_str(&fs::read_to_string(repo(&format!("Fixtures/{folder}/{file}"))).unwrap()).unwrap()
}

#[test]
fn b718_radial_zoom() {
    let mut t = Table::new(
        "radial_blur",
        "# B-718: Radial Blur's zoom smears both ways about each pixel (D-838)\n\n\
         Seven creators' After Effects frames fit a zoom centred on each pixel and half its amount \
         long 3.8 to 4.6 times better than D-110's, which smeared the whole amount from the pixel \
         back toward the centre. A zoom now samples from 1 - Amount / 400 to 1 + Amount / 400 of the \
         pixel's distance from the centre, on the processor and the card. Spin is unchanged, and \
         Light Rays, Spin & Zoom Blur and Fast Zoom Blur keep the outward zoom.\n",
    );

    t.heading("Radial Blur's fixtures, at the reference's numbers (tolerance 2e-5)");
    t.fixtures("expected_radial_blur.json");
    let radial = t.root.clone();
    t.root = repo("Fixtures/edges");
    t.heading("FX-EDGES-008, a zoom with Edges Repeat (tolerance 2e-5)");
    t.fixtures_numbered("expected_edges.json", 8..=8);

    t.heading("The zoom smears both ways now: FX-RADIAL-002 against D-110's outward zoom");
    t.root = radial.clone();
    let now = expected("radial_blur", "expected_radial_blur.json");
    let outward = expected("spin_zoom_blur", "expected_spin_zoom_blur.json");
    let centred = now["cases"]["FX-RADIAL-002"]["frames"]["0"].as_array().unwrap();
    let out = outward["cases"]["FX-SPINZOOM-001"]["frames"]["0"].as_array().unwrap();
    let a = |px: &J| px[3].as_f64().unwrap();
    let inward = centred.iter().zip(out).filter(|(c, o)| a(c) > 0.05 && a(o) == 0.0).count();
    t.row(
        "Pixels the centred zoom 30 covers that D-110's outward zoom 30 (FX-SPINZOOM-001, the same \
         drawing) leaves empty: the smear reaching inward, toward the centre",
        &format!("{inward} pixels"),
        inward > 0,
    );

    t.heading("An old project");
    let loaded = t.load("fx_radial_002.json");
    let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
    t.row(
        "fx_radial_002.json, the same bytes as at c4b0696c (before D-838; no project file changed), \
         opens with no warning",
        &format!("{on_open:?}"),
        on_open.is_empty(),
    );
    t.round_trips(&["fx_radial_002.json", "fx_radial_007.json"]);

    let frame = t.render(&loaded.document, 0, 64);
    let d_out = largest_difference(&frame, &outward["cases"]["FX-SPINZOOM-001"]["frames"]["0"]);
    let d_now = largest_difference(&frame, &now["cases"]["FX-RADIAL-002"]["frames"]["0"]);

    t.row(
        "and draws the centred zoom (FX-RADIAL-002), not D-110's outward one it drew before",
        &format!("largest difference from FX-RADIAL-002 {d_now:.1e}, from D-110's {d_out:.2}"),
        d_now <= 2e-5 && d_out > 0.1,
    );

    t.heading("Light Rays, Spin & Zoom Blur and Fast Zoom Blur keep D-110's outward zoom (tolerance 2e-5)");
    t.root = repo("Fixtures/light_rays");
    t.fixtures("expected_light_rays.json");
    t.root = repo("Fixtures/spin_zoom_blur");
    t.fixtures("expected_spin_zoom_blur.json");
    t.root = repo("Fixtures/fast_zoom_blur");
    t.fixtures("expected_fast_zoom_blur.json");

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let cases = [
        (radial.clone(), "fx_radial_002.json"),
        (radial.clone(), "fx_radial_005.json"),
        (radial.clone(), "fx_radial_007.json"),
        (repo("Fixtures/edges"), "fx_edges_008.json"),
    ];
    for (root, name) in &cases {
        let project = persist::load(&root.join(name)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree) = (0, false, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let (d, r, a, b) = both(&mut gpu, &project, &Id::new(MAIN), root, 0, quality);
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

    t.finish("B-718_radial_zoom_table.md");
}
