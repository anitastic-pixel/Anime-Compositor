//! B-786 (D-907, PROPOSED, off): export on the graphics card, as a trial the owner asked to be
//! measured first ("Measure first", 2026-10-26).
//!
//! `export::ON_CARD` is false unless a test sets it, and not in the export window. Set, each
//! frame is planned as the viewer's card plans it at Full, drawn by `Gpu::draw_held` and read back
//! as the processor's working floats (`Gpu::sum_back`), then encoded as the processor's frame is.
//! A frame the card refuses is drawn by the processor and named in the report; an effect the card
//! does not draw is run on the processor first and named too.
//!
//! - `b786_card_export_says_what_the_processor_drew`: frames 0 to 4 of the reference shot, with
//!   a horizontal Gaussian Blur on layer 1 (the card blurs both ways only, D-303) and an
//!   adjustment layer with a Glow on frames 0 and 1 (the card refuses those frames, B-156). The
//!   report must name both, frames 0 and 1 must be the processor's files byte for byte, and the
//!   card's frames must be within 1 level.
//! - `b786_card_export_table` (ignored, run by name in release): whole-export times, card
//!   against processor in alternating rounds, and the per-frame difference, for the reference
//!   shot and a heavier 25-layer project; writes `verification/B-786_card_export_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Instant;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::export::{
    export_sequence, ExportChoices, ExportReport, ExportRequest, MissingSource, OutputFormat,
    CARDS, ON_CARD,
};
use anime_compositor::exr_io::{self, ExrSamples};
use anime_compositor::gpu::Gpu;
use anime_compositor::media;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::{OutputAlpha, OutputDepth};
use serde_json::{json, Value};

mod common;
use common::repo;

/// `ON_CARD` is one switch for the whole process.
static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

fn comp() -> Id {
    Id::new("comp-reference-shot")
}

/// The reference shot as B-08a wrote it, as JSON, to be changed.
fn shot_json() -> Value {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    serde_json::from_str(&text).expect("the reference shot is JSON")
}

fn load(j: &Value) -> Project {
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("{}", d.message))
        .document
        .project()
        .clone()
}

fn blur(id: &str, sigma: f64, dimensions: &str) -> Value {
    json!({"instance_id": id, "type_id": "core.gaussian_blur", "enabled": true,
           "parameters": {"sigma_px": sigma, "dimensions": dimensions}})
}

/// The heavier project: the reference shot's four layers and 21 more copies of them, each moved,
/// scaled, turned, faded and laid in a mode of its own, as wave 10's 25-layer lag project is,
/// with a Gaussian Blur (both ways, which the card draws) on every fifth copy.
fn heavy() -> Project {
    let mut j = shot_json();
    let c = &mut j["compositions"][0];
    let base: Vec<Value> = c["layers"].as_array().unwrap().clone();
    let modes = ["overlay", "normal", "lighten", "normal", "screen", "multiply", "add"];
    for k in 0..21usize {
        let mut l = base[k % 4].clone();
        let id = format!("layer-{}", k + 5);
        l["id"] = json!(id);
        l["name"] = json!(id);
        let key = |v: Value| json!({"base": v, "keyframes": []});
        let s = 90 + (3 + 7 * k) % 25;
        l["transform"]["position"] = key(json!([
            ((128 + 37 * k) % 300) as i64 - 130,
            ((12 + 53 * k) % 200) as i64 - 100
        ]));
        l["transform"]["scale"] = key(json!([s, s]));
        l["transform"]["rotation"] = key(json!(((14 + 11 * k) % 30) as i64 - 15));
        l["transform"]["opacity"] = key(json!((60 + (8 + 12 * k) % 40) as f64 / 100.0));
        l["blend_mode"] = json!(modes[k % 7]);
        if k % 5 == 0 {
            l["effects"] = json!([blur(&format!("b786-blur-{k}"), 6.0, "both")]);
        }
        c["layers"].as_array_mut().unwrap().push(l);
        c["layer_order"].as_array_mut().unwrap().push(json!(id));
    }
    load(&j)
}

fn request(dir: &Path, last: i32, format: OutputFormat) -> ExportRequest {
    let exr = matches!(format, OutputFormat::Exr(_));
    ExportRequest {
        composition: comp(),
        first_frame: 0,
        last_frame: last,
        output_dir: dir.to_path_buf(),
        naming: if exr { "f_%04d.exr" } else { "f_%04d.png" }.to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::RenderTransparent,
        format,
        choices: ExportChoices::default(),
    }
}

/// One export into a fresh `target/b786/<folder>`, timed: on the processor when `cards` is 0,
/// or on the card with that many connections to it.
fn export(project: &Project, folder: &str, last: i32, format: OutputFormat, cards: usize) -> (ExportReport, f64, PathBuf) {
    let dir = repo("target/b786").join(folder);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    ON_CARD.store(cards > 0, Ordering::SeqCst);
    CARDS.store(cards.max(1), Ordering::SeqCst);
    let start = Instant::now();
    let report = export_sequence(project, &root(), &request(&dir, last, format), &AtomicBool::new(false));
    let ms = start.elapsed().as_secs_f64() * 1000.0;
    ON_CARD.store(false, Ordering::SeqCst);
    CARDS.store(1, Ordering::SeqCst);
    assert!(report.succeeded(), "{folder}: {:?}", report.diagnostics);
    (report, ms, dir)
}

fn png(dir: &Path, frame: i32) -> Vec<u8> {
    let (_, _, px) = media::decode_8bit(&dir.join(format!("f_{frame:04}.png")))
        .expect("a PNG")
        .expect("it decodes");
    px
}

/// (largest difference in levels of 255, pixels more than 1 level apart, pixels). Colour where
/// both pictures are fully transparent is not compared, as in B-754.
fn levels(a: &[u8], b: &[u8]) -> (u8, usize, usize) {
    let (mut worst, mut over) = (0u8, 0usize);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = if p[3] == 0 && q[3] == 0 {
            0
        } else {
            (0..4).map(|i| p[i].abs_diff(q[i])).max().unwrap()
        };
        worst = worst.max(d);
        over += (d > 1) as usize;
    }
    (worst, over, a.len() / 4)
}

/// (largest float difference, mean float difference) of an EXR pair, skipping colour where both
/// are fully transparent.
fn floats(a: &Path, b: &Path) -> (f32, f64) {
    let a = exr_io::read(a).expect("an EXR").image;
    let b = exr_io::read(b).expect("an EXR").image;
    let (mut worst, mut sum, mut n) = (0f32, 0f64, 0usize);
    for (p, q) in a.data().chunks_exact(4).zip(b.data().chunks_exact(4)) {
        if p[3] == 0.0 && q[3] == 0.0 {
            continue;
        }
        for i in 0..4 {
            let d = (p[i] - q[i]).abs();
            worst = worst.max(d);
            sum += d as f64;
            n += 1;
        }
    }
    (worst, sum / n.max(1) as f64)
}

fn said(report: &ExportReport) -> String {
    report
        .diagnostics
        .iter()
        .map(|d| format!("{} {}", d.message, d.detail))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn b786_card_export_says_what_the_processor_drew() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let mut j = shot_json();
    let c = &mut j["compositions"][0];
    c["layers"][0]["effects"] = json!([blur("b786-h", 4.0, "horizontal")]);
    let glow = json!({"instance_id": "b786-glow", "type_id": "core.glow", "enabled": true,
        "parameters": {"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0,
                       "radius": 30, "intensity": 1, "operation": "add", "tint": ""}});
    let key = |v: Value| json!({"base": v, "keyframes": []});
    let adjustment = json!({
        "id": "b786-adj", "kind": "adjustment", "name": "b786 adjustment", "enabled": true,
        "locked": false, "in_frame": 0, "out_frame": 2,
        "transform": {"anchor": key(json!([960, 540])), "position": key(json!([960, 540])),
                      "scale": key(json!([100, 100])), "rotation": key(json!(0)),
                      "opacity": key(json!(1.0))},
        "matte": null, "blend_mode": "normal", "effects": [glow],
    });
    c["layers"].as_array_mut().unwrap().push(adjustment);
    c["layer_order"].as_array_mut().unwrap().push(json!("b786-adj"));
    let project = load(&j);

    let (processor, _, cpu) = export(&project, "say_cpu", 4, OutputFormat::Png, 0);
    assert!(!said(&processor).contains("graphics card"), "off says nothing of the card: {}", said(&processor));
    let (report, _, gpu) = export(&project, "say_card", 4, OutputFormat::Png, 3);
    let text = said(&report);
    println!("{text}");
    if let Err(why) = Gpu::new() {
        assert!(text.contains("no card could be opened"), "{why}: {text}");
        for f in 0..=4 {
            assert_eq!(png(&cpu, f), png(&gpu, f), "frame {f} without a card");
        }
        return;
    }
    assert!(text.contains("the card drew 3 of 5 frames"), "{text}");
    assert!(text.contains("The card drew frames 2 to 4."), "{text}");
    assert!(text.contains("The processor drew frames 0 to 1"),"{text}");
    assert!(text.contains("adjustment layer has an effect the GPU does not draw"), "{text}");
    assert!(text.contains("the processor ran these effects first: Gaussian Blur."), "{text}");
    for f in 0..=1 {
        assert_eq!(
            fs::read(cpu.join(format!("f_{f:04}.png"))).unwrap(),
            fs::read(gpu.join(format!("f_{f:04}.png"))).unwrap(),
            "frame {f}, refused by the card, is the processor's file"
        );
    }
    for f in 2..=4 {
        let (worst, over, _) = levels(&png(&cpu, f), &png(&gpu, f));
        assert!(worst <= 1, "frame {f}: {worst} levels, {over} pixels over 1");
    }
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v[v.len() / 2]
}

/// The ways an export is timed: 0 the processor, otherwise the card with that many connections.
const WAYS: [usize; 3] = [0, 1, 3];

/// Whole-export times, `rounds` of the three ways taking turns (the first to go moving on each
/// round), and the PNG frames of each card way's last round compared with the processor's.
fn timed(out: &mut String, name: &str, project: &Project, last: i32, rounds: usize) {
    let mut times: Vec<Vec<f64>> = vec![Vec::new(); WAYS.len()];
    let mut said = vec![String::new(); WAYS.len()];
    for r in 0..rounds {
        for k in 0..WAYS.len() {
            let w = (r + k) % WAYS.len();
            let (report, ms, _) = export(project, &format!("{name}_{}", WAYS[w]), last, OutputFormat::Png, WAYS[w]);
            times[w].push(ms);
            said[w] = report
                .diagnostics
                .iter()
                .filter(|d| d.message.contains("graphics card") || d.message.contains("CPU drew"))
                .map(|d| format!("{} {}", d.message, d.detail))
                .collect::<Vec<_>>()
                .join(" / ");
        }
    }
    let frames = last + 1;
    let c = median(times[0].clone());
    let cpu = repo("target/b786").join(format!("{name}_0"));
    let ms = |v: &[f64]| v.iter().map(|m| format!("{m:.0}")).collect::<Vec<_>>().join(", ");
    let _ = writeln!(
        out,
        "| {name}, frames 0 to {last}, 8-bit PNG, processor | {c:.0} ms ({:.1} ms a frame) | 1.00x | | | |\n|  | rounds: {} | | | | |",
        c / frames as f64,
        ms(&times[0])
    );
    for w in 1..WAYS.len() {
        let g = median(times[w].clone());
        let gpu = repo("target/b786").join(format!("{name}_{}", WAYS[w]));
        let (mut worst, mut over, mut px, mut worst_frame, mut exact) = (0u8, 0usize, 0usize, 0, 0);
        for f in 0..frames {
            let (a, b) = (png(&cpu, f), png(&gpu, f));
            exact += (a == b) as usize;
            let (d, o, n) = levels(&a, &b);
            if d > worst {
                worst = d;
                worst_frame = f;
            }
            over += o;
            px += n;
        }
        let _ = writeln!(
            out,
            "| {name}, card, {} connection{} | {g:.0} ms ({:.1} ms a frame) | {:.2}x | {} of {px} ({:.4}%) | {worst} (frame {worst_frame}) | {exact} of {frames} |\n|  | rounds: {} | | | | |",
            WAYS[w],
            if WAYS[w] == 1 { "" } else { "s" },
            g / frames as f64,
            c / g,
            px - over,
            100.0 * (px - over) as f64 / px as f64,
            ms(&times[w])
        );
        println!("{name}: processor {c:.0} ms, card x{} {g:.0} ms; worst {worst}; report: {}", WAYS[w], said[w]);
    }
    out.push_str(&format!("\n<!-- {name} report, card: {} -->\n\n", said[WAYS.len() - 1]));
}

#[test]
#[ignore]
fn b786_card_export_table() {
    let _one = ONE_AT_A_TIME.lock().unwrap_or_else(|e| e.into_inner());
    let path = repo("verification/B-786_card_export_table.md");
    if let Err(why) = Gpu::new() {
        fs::write(&path, format!("# B-786: export on the graphics card\n\n**NOT RUN.** No usable card: {why}\n")).unwrap();
        panic!("no card: {why}");
    }
    let rounds: usize = std::env::var("B786_ROUNDS").ok().and_then(|r| r.parse().ok()).unwrap_or(5);
    let shot = load(&shot_json());
    let heavy = heavy();
    let mut rows = String::new();
    timed(&mut rows, "reference shot", &shot, 239, rounds);
    timed(&mut rows, "25 layers", &heavy, 47, rounds);

    // EXR float: the card's working floats against the processor's, as written.
    let mut exr = String::new();
    for (name, project, last) in [("reference shot", &shot, 47), ("25 layers", &heavy, 23)] {
        let (_, _, cpu) = export(project, &format!("{name}_exr_cpu"), last, OutputFormat::Exr(ExrSamples::Float), 0);
        let (_, _, gpu) = export(project, &format!("{name}_exr_card"), last, OutputFormat::Exr(ExrSamples::Float), 3);
        let (mut worst, mut mean, mut at) = (0f32, 0f64, 0);
        for f in 0..=last {
            let (w, m) = floats(&cpu.join(format!("f_{f:04}.exr")), &gpu.join(format!("f_{f:04}.exr")));
            if w > worst {
                worst = w;
                at = f;
            }
            mean += m / (last + 1) as f64;
        }
        let _ = writeln!(exr, "| {name}, frames 0 to {last}, EXR float | {worst:.6} (frame {at}) | {mean:.2e} |");
    }

    let text = format!(
        "# B-786: export on the graphics card against the processor (D-907, PROPOSED, off)\n\n\
         Written by `tests/b786_card_export.rs` (`b786_card_export_table`, release, run by name). \
         Each export is the whole job as the export window runs it (`export_sequence`, frames \
         drawn several at once, PNG encoded beside), timed from the call to its report. The \
         processor, the card with one connection (one frame on the card at a time) and the card \
         with three (three frames at once) take turns, {rounds} rounds, the first to go moving on \
         each round; the median is shown, and each round below it. The machine is shared with other lanes of wave 11, \
         so the times are the machine's load as well as the work.\n\n\
         The pixel columns compare each card frame with the processor's frame of the last round, \
         8-bit straight sRGB as written; colour where both are fully transparent is not compared \
         (B-754). The viewer's bar is 1 level of 255 (D-165).\n\n\
         | Export | Whole export (median) | Faster than the processor by | Pixels within 1 level of the processor's | Largest difference (levels) | Frames the same bytes |\n\
         |---|---|---|---|---|---|\n{rows}\n\
         The 25-layer project is the reference shot's four layers and 21 copies, moved, scaled, \
         turned, faded and laid in overlay, lighten, screen, multiply and add, with a Gaussian \
         Blur of 6 on every fifth copy (both ways, which the card draws).\n\n\
         ## EXR float\n\n\
         The working floats as written, linear and premultiplied, card against processor.\n\n\
         | Export | Largest difference | Mean difference |\n|---|---|---|\n{exr}"
    );
    fs::write(&path, text).unwrap();
    let _ = fs::remove_dir_all(repo("target/b786"));
}
