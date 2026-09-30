//! B-170: a masked drawing held for a second frame is not masked again, and not sent to the card
//! again (GPU plan G11, D-242). The reference shot with every layer held on twos and its second
//! layer masked, with no effect, with a Gaussian Blur and with an Exposure, at Full and Draft,
//! frames 0 to 11, drawn by one viewer that keeps what it drew:
//!
//! 1. every frame is, byte for byte, the frame a viewer that keeps nothing draws;
//! 2. on a held frame (an odd one) no mask is drawn;
//! 3. on a held frame the card is sent no drawing.
//!
//! Writes `verification/B-170_held_table.md`.

use std::fmt::Write as _;
use std::fs;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::perf::{self, Stage};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

const SHOTS: [&str; 3] = ["mask", "mask and blur", "mask and exposure"];

fn shot(kind: &str) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = j["compositions"][0]["layers"].as_array_mut().expect("layers");
    for l in layers.iter_mut() {
        l["exposure_spans"] = (0..120)
            .map(|k| json!({"start_frame": 2 * k, "end_frame_exclusive": 2 * k + 2, "drawing_number": 2 * k}))
            .collect();
    }
    let corner = |x: i32, y: i32| json!({"point": [x, y], "in": [0, 0], "out": [0, 0]});
    layers[1]["masks"] = json!([{"name": "M", "enabled": true, "inverted": false, "mode": "add", "opacity": 1.0,
        "feather_px": 4.0, "expansion_px": 0.0,
        "path": {"base": {"points": [corner(100, 100), corner(1500, 100), corner(1500, 900), corner(100, 900)]}, "keyframes": []}}]);
    match kind {
        "mask and blur" => {
            layers[1]["effects"] = json!([{"instance_id": "b170-b", "type_id": "core.gaussian_blur", "enabled": true,
                "parameters": {"sigma_px": 6.0}}])
        }
        "mask and exposure" => {
            layers[1]["effects"] = json!([{"instance_id": "b170-x", "type_id": "core.exposure", "enabled": true,
                "parameters": {"stops": 0.5}}])
        }
        _ => {}
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{kind}: {}", d.message)).document.project().clone()
}

#[test]
fn a_held_masked_drawing_is_kept() {
    let out = repo("verification/B-170_held_table.md");
    let mut gpu = match Gpu::new() {
        Ok(gpu) => gpu,
        Err(why) => {
            fs::write(&out, format!("# B-170: a held masked drawing kept\n\n**NOT RUN.** No usable card: {why}\n\nNo check in this table was run, so none of them passes.\n"))
                .expect("write the B-170 table");
            return;
        }
    };
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let draw = |project: &Project, frame: i32, quality: PreviewQuality, cache: &mut CelCache, gpu: &mut Gpu| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_srgb8(project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, cache, gpu)
            .expect("the frame draws")
            .0
    };
    perf::enable();
    let mut rows = String::new();
    let (mut checks, mut passed) = (0, 0);
    for kind in SHOTS {
        let project = shot(kind);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            let fresh: Vec<Vec<u8>> = (0..12)
                .map(|frame| {
                    gpu.forget();
                    draw(&project, frame, quality, &mut CelCache::viewer(), &mut gpu)
                })
                .collect();
            gpu.forget();
            let mut cache = CelCache::viewer();
            for frame in 0..12 {
                let sent = gpu.sent();
                perf::reset();
                let bytes = draw(&project, frame, quality, &mut cache, &mut gpu);
                let (sent, masked) = (gpu.sent() - sent, perf::read(Stage::Mask).1);
                let same = bytes == fresh[frame as usize];
                let held = frame % 2 == 1;
                let ok = same && (!held || (sent == 0 && masked == 0));
                checks += 1;
                passed += ok as usize;
                writeln!(
                    rows,
                    "| {kind} | {} | {frame} | {} | {sent} | {masked} | {} | {} |",
                    quality.label(),
                    if held { "held" } else { "new" },
                    if same { "yes" } else { "NO" },
                    if ok { "pass" } else { "FAIL" }
                )
                .unwrap();
            }
        }
    }
    perf::disable();
    let table = format!(
        "# B-170: a held masked drawing kept\n\n\
         The reference shot with every layer held on twos (a new drawing on even frames, the same \
         one again on odd frames) and its second layer masked, with no effect, with a Gaussian \
         Blur and with an Exposure. One viewer draws frames 0 to 11 in turn, keeping what it drew. \
         A row passes when the frame is, byte for byte, the frame a viewer that keeps nothing draws, \
         and, on a held frame, no mask is drawn and the card is sent no drawing.\n\n\
         **{passed} of {checks} pass.**\n\n\
         | Shot | Quality | Frame | Drawing | Drawings sent to the card | Masks drawn | Same bytes | Result |\n\
         |---|---|---:|---|---:|---:|---|---|\n{rows}"
    );
    fs::write(&out, table).expect("write the B-170 table");
    assert_eq!(passed, checks, "see verification/B-170_held_table.md");
}

/// The time a frame takes, held and new, by the median of frames 4 to 23. Run by hand; writes
/// `verification/B-170_timing_raw.md`.
#[test]
#[ignore]
fn b170_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let mut rows = String::new();
    for kind in SHOTS {
        let project = shot(kind);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            gpu.forget();
            let mut cache = CelCache::viewer();
            let (mut new, mut held) = (Vec::new(), Vec::new());
            for frame in 0..24 {
                let mut log = FrameLog::new(3);
                let at = std::time::Instant::now();
                preview::preview_frame_srgb8(&project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .expect("the frame draws");
                let ms = at.elapsed().as_secs_f64() * 1000.0;
                if frame >= 4 {
                    if frame % 2 == 1 { held.push(ms) } else { new.push(ms) }
                }
            }
            new.sort_by(f64::total_cmp);
            held.sort_by(f64::total_cmp);
            writeln!(rows, "| {kind} | {} | {:.1} | {:.1} |", quality.label(), new[new.len() / 2], held[held.len() / 2]).unwrap();
        }
    }
    fs::write(
        repo("verification/B-170_timing_raw.md"),
        format!("| Shot | Quality | New drawing (ms) | Held (ms) |\n|---|---|---:|---:|\n{rows}"),
    )
    .expect("write the timing");
}
