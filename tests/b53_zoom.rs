//! B-53: the pictures a person judges D-110 by, the Radial Blur's zoom smearing outward only.
//!
//! Frame 0 of the reference shot, drawn by the CPU as an export is: as it is, and with a zoom of
//! 30 about the middle of its background. `zoom_before.png`, beside them in
//! `verification/B-53 pictures/`, was drawn by this same test under D-95's rule, from the code
//! of commit cf7a2d9, and is not rewritten. The fixtures FX-RADIAL-002, 005 and 007 pin the rule
//! itself.

use std::fs;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Id;
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::json;

mod common;
use common::repo;

#[test]
fn b53_pictures() {
    let pictures = repo("verification/B-53 pictures");
    fs::create_dir_all(&pictures).expect("make the pictures folder");
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let plain: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let mut zoomed = plain.clone();
    zoomed["compositions"][0]["layers"][0]["effects"] = json!([{"instance_id": "b53", "type_id": "core.radial_blur", "enabled": true,
        "parameters": {"type": "zoom", "amount": 30.0, "center": [50.0, 50.0]}}]);
    for (file, j) in [("plain.png", plain), ("zoom_after.png", zoomed)] {
        let project = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{file}: {}", d.message)).document.project().clone();
        let mut log = FrameLog::new(3);
        let frame = preview::preview_frame_cached(&project, &Id::new("comp-reference-shot"), 0, &repo("Fixtures/reference_shot"), PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .unwrap_or_else(|d| panic!("{file}: {}", d.message));
        png_out::write_rgba(&pictures.join(file), frame.width(), frame.height(), OutputDepth::Eight, &[], &frame.to_srgb8_straight()).expect("write a picture");
    }
}
