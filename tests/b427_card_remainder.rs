//! B-427: D-547, the card's remainders. This machine's card (RTX 4070 Ti SUPER, Vulkan) takes
//! the remainder of a negative whole number as if it had no sign: -7 % 6 gives 3, not -1, so
//! `((z % n) + n) % n` gives 3 where 5 is meant. A divisor that is a power of two comes out right
//! by luck. Every `%` in the card's code was read: the one that can see a negative number with
//! any divisor is the noise's period, which Noise Alpha's Cycle Noise and Fractal Noise's Cycle
//! Evolution wrap the depth with. A negative Noise Phase or Evolution with a cycle that is not a
//! power of two drew a different noise on the card. The processor is the reference.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::Table;
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        // B-233's reason: a pixel clear in both has no colour to compare.
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::NoiseAlpha { .. } | Effect::FractalNoise { .. })))
        .count()
}

/// The processor's and the card's frame, as sRGB bytes, and whether the card refused.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> (Vec<u8>, Vec<u8>, bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (c.to_srgb8_straight(), g, refused, said_cpu, said_gpu)
}

/// The reference shot with `type_id` of `parameters` on its first three layers.
fn reference(type_id: &str, parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b427-{id}"), "type_id": type_id, "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

fn noise_alpha(phase: f64, cycle: f64) -> J {
    json!({"noise": "uniform_animation", "amount": 60, "original_alpha": "scale", "overflow": "wrap_back",
           "random_seed": 0, "noise_phase": phase, "cycle_noise": "on", "cycle": cycle})
}

fn fractal_noise(evolution: f64, cycle: f64, turbulent: bool) -> J {
    let mut p = json!({"size": 40, "complexity": 4, "contrast": 130, "brightness": 5, "evolution": evolution, "speed": 0,
                       "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 70, "blend": "screen", "cycle": cycle});
    if turbulent {
        p["fractal_type"] = json!("turbulent");
        p["noise_type"] = json!("block");
    }
    p
}

#[test]
fn b427_card_remainder() {
    let mut t = Table::new(
        "reference_shot",
        "# B-427: the card's remainders\n\n\
         D-547: this machine's graphics card takes the remainder of a negative whole number as if \
         it had no sign (-7 % 6 gave 3, not -1; -2 wrapped into 7 gave 2, not 5). Every `%` in the \
         card's code was read. The noise's period, used by Noise Alpha's Cycle Noise and Fractal \
         Noise's Cycle Evolution, was the only one that could meet a negative number with any \
         divisor: a negative Noise Phase or Evolution with a cycle that is not a power of two drew \
         a different noise on the card than on the processor. Each case below is the reference \
         shot with the effect on its first three layers, at frames 0, 100 and 239, Full and \
         Draft, the card's picture against the processor's, within 1 level of 255 (ADR-006, \
         D-100). The cases with a cycle of 2 or a positive depth agreed before the fix too.\n",
    );
    let comp = Id::new("comp-reference-shot");
    let root = effect_table::repo("Fixtures/reference_shot");
    let mut gpu = Gpu::new().expect("a usable card");
    let cases: Vec<(&str, &str, J, J)> = vec![
        ("Noise Alpha, phase -100, cycle 3", "core.noise_alpha", noise_alpha(-100.0, 3.0), noise_alpha(-100.0, 0.0)),
        ("Noise Alpha, phase -1000, cycle 7", "core.noise_alpha", noise_alpha(-1000.0, 7.0), noise_alpha(-1000.0, 0.0)),
        ("Noise Alpha, phase -100, cycle 2 (agreed before)", "core.noise_alpha", noise_alpha(-100.0, 2.0), noise_alpha(-100.0, 0.0)),
        ("Noise Alpha, phase 500, cycle 3 (agreed before)", "core.noise_alpha", noise_alpha(500.0, 3.0), noise_alpha(500.0, 0.0)),
        ("Fractal Noise, evolution -500, cycle 3", "core.fractal_noise", fractal_noise(-500.0, 3.0, false), fractal_noise(-500.0, 0.0, false)),
        ("Fractal Noise, turbulent block, evolution -2000, cycle 5", "core.fractal_noise", fractal_noise(-2000.0, 5.0, true), fractal_noise(-2000.0, 0.0, true)),
        ("Fractal Noise, evolution -500, cycle 2 (agreed before)", "core.fractal_noise", fractal_noise(-500.0, 2.0, false), fractal_noise(-500.0, 0.0, false)),
        ("Fractal Noise, evolution 1500, cycle 3 (agreed before)", "core.fractal_noise", fractal_noise(1500.0, 3.0, false), fractal_noise(1500.0, 0.0, false)),
    ];
    let dir = effect_table::repo("verification/D-547 pictures");
    fs::create_dir_all(&dir).unwrap();
    for (i, (what, type_id, p, uncycled)) in cases.iter().enumerate() {
        t.heading(what);
        let project = reference(type_id, p);
        let plain = reference(type_id, uncycled);
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(project, &comp, 0, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
                .expect("the reference shot draws")
                .to_srgb8_straight()
        };
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            "the processor's frame 0 differs from the same with no cycle, so the cycle is in play",
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (c, g, r, a, b) = both(&mut gpu, &project, &comp, &root, frame, quality);
                let d = distance(&c, &g);
                let card = on_card(&project, &comp, &root, frame, quality);
                t.row(
                    &format!("frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
                // The first two cases' frame 0, for the eye: the card's picture beside the processor's.
                if i < 2 && frame == 0 && quality == PreviewQuality::Full {
                    let size = project.composition(&comp).unwrap();
                    let (w, h) = (size.width as usize, size.height as usize);
                    let name = if i == 0 { "noise_alpha" } else { "noise_alpha_7" };
                    png_out::write_rgba(&dir.join(format!("{name}_processor.png")), w, h, OutputDepth::Eight, &[], &c).unwrap();
                    png_out::write_rgba(&dir.join(format!("{name}_card.png")), w, h, OutputDepth::Eight, &[], &g).unwrap();
                }
            }
        }
    }
    t.finish("D-547_card_remainder_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-233's way: the reference shot with the noise on its first three layers,
/// every eighth frame asked for as the viewer asks, on the card.
#[test]
#[ignore = "B-427: a measurement, run deliberately with --release --ignored"]
fn b427_card_remainder_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let moving = |cycle: f64| {
        let mut p = fractal_noise(-3000.0, cycle, false);
        p["speed"] = json!(15);
        p
    };
    let shots: [(&str, &str, J); 4] = [
        ("Fractal Noise, evolution -3000 moving, no cycle", "core.fractal_noise", moving(0.0)),
        ("Fractal Noise, evolution -3000 moving, cycle 3", "core.fractal_noise", moving(3.0)),
        ("Noise Alpha, phase -100, no cycle", "core.noise_alpha", noise_alpha(-100.0, 0.0)),
        ("Noise Alpha, phase -100, cycle 3", "core.noise_alpha", noise_alpha(-100.0, 3.0)),
    ];
    let (comp, root) = (Id::new("comp-reference-shot"), effect_table::repo("Fixtures/reference_shot"));
    for (name, type_id, p) in shots {
        let project = reference(type_id, &p);
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B427_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| effect_table::repo("verification/B-427_card_remainder_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
