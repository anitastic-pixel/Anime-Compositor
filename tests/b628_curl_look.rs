//! B-628: D-749, Curl Noise's look after After Effects (audit rows 8, 13 and 14): the noise
//! carried along a curl flow, Flow Softness crisper as it goes up, and the drift adding up the
//! speed frame by frame.
//!
//! Writes `verification/B-628_curl_look_table.md` and draws the street into
//! `verification/B-628 pictures/`.
//!
//! FX-CURL-078's expected pixels are `Fixtures/curl_noise/expected_curl_noise.json`, written by
//! `tools/curl_noise_reference.py`; every other number here is measured and printed as measured.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{distance, repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Curl Noise as a new one is added, with the settings `p` changed.
fn curl(p: &J) -> J {
    let mut all = json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
        "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
        "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
        "clip_hdr_results": "on", "channel": "rgb"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!([{"instance_id": "fx-0-0", "type_id": "core.curl_noise", "enabled": true, "parameters": all}])
}

/// The street, a two-second shot, with `effects` on it.
fn street(effects: J) -> Project {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    persist::load_str(&project.to_string())
        .expect("the street's project reads")
        .document
        .project()
        .clone()
}

/// `project` drawn at `frame` from `root`, straight 8-bit, with what it warned of.
fn draw(project: &Project, root: &Path, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut log = FrameLog::new(3);
    let b = render_frame(project, &Id::new(MAIN), frame, root, 64, &mut log).expect("it draws");
    let said = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
    (b.to_srgb8_straight(), said)
}

/// The mean step in red between neighbouring pixels across, in levels: how crisp a picture is.
fn crispness(a: &[u8], width: usize) -> f64 {
    let (mut sum, mut n) = (0.0, 0usize);
    for row in a.chunks_exact(width * 4) {
        for x in 1..width {
            sum += (row[x * 4] as f64 - row[x * 4 - 4] as f64).abs();
            n += 1;
        }
    }
    sum / n as f64
}

/// The largest difference and pixels differing between `a` and `b` moved `dx` pixels right,
/// over the columns both cover.
fn moved(a: &[u8], b: &[u8], width: usize, dx: usize) -> (u8, usize) {
    let (mut most, mut n) = (0u8, 0usize);
    for (ra, rb) in a.chunks_exact(width * 4).zip(b.chunks_exact(width * 4)) {
        for x in dx..width {
            // The card's own covering does not move; only where both are fully covered.
            if ra[x * 4 + 3] < 255 || rb[(x - dx) * 4 + 3] < 255 {
                continue;
            }
            let d = (0..4)
                .map(|c| ra[x * 4 + c].abs_diff(rb[(x - dx) * 4 + c]))
                .max()
                .unwrap();
            most = most.max(d);
            n += (d > 1) as usize;
        }
    }
    (most, n)
}

#[test]
fn b628_curl_look() {
    let mut t = Table::new(
        "curl_noise",
        "# B-628: Curl Noise's look (D-749)\n\nAudit rows 8, 13 and 14: After Effects' large, smooth \
         marbling that follows Size and swirls further with Sample Radius; Flow Softness crisper as it \
         goes up; the speeds added up frame by frame, so a keyed Speed drifts from where it was. The \
         rule is document 21's and `tools/curl_noise_reference.py`'s; FX-CURL-001 to 077 hold it in \
         B-326, B-461 and B-525's tables. Here: FX-CURL-078 (a keyed Speed), the drift at other \
         frame rates, Draft against Full, and the street.\n",
    );

    t.heading("FX-CURL-078: a keyed Speed adds up");
    t.fixtures_numbered("expected_curl_noise.json", 78..=78);
    t.round_trips(&["fx_curl_078.json"]);

    // Row 14 at other frame rates: Speed 20 is 2 pixels a frame at 24 fps, so 4 at 12 fps and 1
    // at 48 fps, the same distance each second. Fixture 011 is 16 by 10, Direction 90 (right).
    t.heading("The drift at other frame rates (fixture 011, Speed 20, Direction 90, Input Noise)");
    let text = fs::read_to_string(t.root.join("fx_curl_011.json")).unwrap();
    for (fps, dx) in [(24, 2usize), (12, 4), (48, 1)] {
        let mut j: J = serde_json::from_str(&text).unwrap();
        j["compositions"][0]["frame_rate"] = json!({"numerator": fps, "denominator": 1});
        let project = persist::load_str(&j.to_string()).unwrap().document.project().clone();
        let (f0, _) = draw(&project, &t.root, 0);
        let (f1, said) = draw(&project, &t.root, 1);
        let d = moved(&f1, &f0, 16, dx);
        let still = distance(&f1, &f0).1;
        t.row(
            &format!("{fps} fps: frame 1 is frame 0 moved {dx} pixels right, within 1 level, and not the same as frame 0"),
            &format!("largest difference {} of 255 on {} pixels; {still} pixels differ from frame 0; warnings {said:?}", d.0, d.1),
            d.0 <= 1 && still > 0 && said.is_empty(),
        );
    }

    // Draft draws at a quarter size with Size, Offset, Speed and the drift scaled with it, so the
    // same noise in fewer pixels; Sample Radius is trace time, not pixels, and is not scaled.
    t.heading("Draft against Full (the street, Curl Noise as added, frame 12)");
    let project = street(curl(&json!({})));
    let root = repo("verification/B-628 pictures");
    fs::create_dir_all(&root).unwrap();
    png_out::write_rgba(&root.join("town.png"), TOWN.0, TOWN.1, OutputDepth::Eight, &[], &town()).unwrap();
    let preview = |q| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(&project, &Id::new(MAIN), 12, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the street draws")
    };
    let full = preview(PreviewQuality::Full);
    let draft = preview(PreviewQuality::Draft);
    let (fw, fh, dw) = (full.width(), full.height(), draft.width());
    let k = fw / dw;
    let (fb, db) = (full.to_srgb8_straight(), draft.to_srgb8_straight());
    let (mut sum, mut n) = (0.0, 0usize);
    for y in 0..fh / k {
        for x in 0..dw {
            let mut block = 0.0;
            for dy in 0..k {
                for dx in 0..k {
                    block += fb[((k * y + dy) * fw + k * x + dx) * 4] as f64;
                }
            }
            sum += (block / (k * k) as f64 - db[(y * dw + x) * 4] as f64).abs();
            n += 1;
        }
    }
    let mean = sum / n as f64;
    t.row(
        "Draft's pixels against Full's averaged in blocks: the same marbling, a mean difference under 8 levels",
        &format!("Draft {dw} by {}, Full {fw} by {fh}, blocks {k} by {k}; mean difference {mean:.2} levels", draft.height()),
        dw * k == fw && mean < 8.0,
    );

    t.heading("Pictures: the street, in `verification/B-628 pictures/`");
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&root.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    let shot = |p: J| draw(&street(curl(&p)), &root, 0);
    let mut pictures = vec![];
    for (name, p) in [
        ("1_as_added", json!({})),
        ("2_size_300", json!({"size": 300})),
        ("3_flow_softness_0", json!({"flow_softness": 0})),
        ("4_flow_softness_100", json!({"flow_softness": 100})),
        ("5_sample_radius_0", json!({"sample_radius": 0})),
        ("6_sample_radius_80", json!({"sample_radius": 80})),
    ] {
        let (bytes, said) = shot(p);
        write(&format!("{name}.png"), &bytes);
        let c = crispness(&bytes, TOWN.0);
        t.row(
            &format!("{name}.png draws cleanly"),
            &format!("warnings {said:?}; crispness (mean step across) {c:.2} levels"),
            said.is_empty(),
        );
        pictures.push((bytes, c));
    }
    let c = |i: usize| pictures[i].1;
    t.row(
        "Row 8: Size 300's marbling is larger and smoother than Size 100's (smaller steps across)",
        &format!("{:.2} against {:.2} levels", c(1), c(0)),
        c(1) < c(0),
    );
    t.row(
        "Row 13: Flow Softness 100 is crisper than Flow Softness 0 (larger steps across)",
        &format!("{:.2} against {:.2} levels", c(3), c(2)),
        c(3) > c(2),
    );
    let swirled = distance(&pictures[5].0, &pictures[4].0).1;
    t.row(
        "Row 8: Sample Radius 80 carries the noise further than Sample Radius 0, so most pixels differ",
        &format!("{swirled} of {} pixels differ", TOWN.0 * TOWN.1),
        swirled > TOWN.0 * TOWN.1 / 2,
    );

    t.finish("B-628_curl_look_table.md");
}
