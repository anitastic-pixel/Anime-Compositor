//! B-775 (D-895): Lens Blur worked by Fourier transform on the viewer's card, held against the
//! CPU's exact gather by wave 8's bar on what a viewer can see (D-894): within 1 level of 255 on
//! at least 99.9% of the pixels and within 2 everywhere, a pixel fully transparent on both sides
//! not graded. The transform is switched on for every radius here, whatever `LENS_FFT_FROM` says.
//!
//! Writes `verification/B-775_fft_lens_blur_table.md`; the timing, run deliberately, writes
//! `verification/B-775_timing_table.md`.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::{Gpu, LENS_FFT_FROM};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::png_out;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::OutputDepth;
use serde_json::json;

mod effect_table;
use effect_table::{hidden, median, repo, seen, shown, Table};

const RADII: [f64; 4] = [25.0, 50.0, 100.0, 200.0];
const FRAME: i32 = 100;

/// D-222's Lens Blur: hexagon iris, roundness 20, rotation 15, aspect 1.3, gain 2, threshold 80.
fn lens(radius: f64) -> serde_json::Value {
    json!({"type_id": "core.lens_blur", "parameters": {"radius": radius, "edges": "transparent", "iris": "hexagon",
           "roundness": 20, "rotation": 15, "aspect": 1.3, "highlight_gain": 2, "highlight_threshold": 80}})
}

/// The reference shot with `e` on its first three layers, as B-164 times it; no effect for `None`.
fn shot(e: Option<serde_json::Value>) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: serde_json::Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    if let Some(e) = e {
        for (i, layer) in j["compositions"][0]["layers"].as_array_mut().expect("layers").iter_mut().take(3).enumerate() {
            let mut e = e.clone();
            e["instance_id"] = json!(format!("b775-{i}"));
            e["enabled"] = json!(true);
            layer["effects"] = json!([e]);
        }
    }
    persist::load_str(&j.to_string()).expect("the reference shot loads").document.project().clone()
}

/// Wave 8's bar over the pixels a viewer can see.
#[derive(Default)]
struct Stats {
    frames: usize,
    seen: usize,
    over1: usize,
    over2: usize,
    largest: u8,
    excused: usize,
    excused_shown: u8,
}

impl Stats {
    fn add(&mut self, cpu: &[u8], card: &[u8]) {
        assert_eq!(cpu.len(), card.len());
        self.frames += 1;
        for (a, b) in cpu.chunks_exact(4).zip(card.chunks_exact(4)) {
            if a[3] == 0 && b[3] == 0 {
                if hidden(a, b) {
                    self.excused += 1;
                    self.excused_shown = self.excused_shown.max(shown(a, b));
                }
                continue;
            }
            let d = seen(a, b);
            self.seen += 1;
            self.largest = self.largest.max(d);
            self.over1 += (d > 1) as usize;
            self.over2 += (d > 2) as usize;
        }
    }
    fn ok(&self) -> bool {
        self.frames > 0 && self.over2 == 0 && (self.seen - self.over1) * 1000 >= self.seen * 999 && self.excused_shown == 0
    }
    fn text(&self) -> String {
        format!(
            "{} frames, {} pixels seen: largest {}, {} over 1, {} over 2, {:.4}% within 1; {} fully transparent pixels with different hidden colours not graded, largest difference over black or white {}",
            self.frames,
            self.seen,
            self.largest,
            self.over1,
            self.over2,
            100.0 * (self.seen - self.over1) as f64 / self.seen.max(1) as f64,
            self.excused,
            self.excused_shown
        )
    }
}

/// The frame by the CPU and by the card; panics if the card refused it.
fn both(project: &Project, comp: &Id, frame: i32, root: &Path, q: PreviewQuality, gpu: &mut Gpu) -> (Vec<u8>, Vec<u8>) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let cpu = preview::preview_frame_cached(project, comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .expect("the CPU's frame")
        .to_srgb8_straight();
    let mut log = FrameLog::new(3);
    gpu.forget();
    let (card, ..) = preview::preview_frame_srgb8(project, comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .expect("the card's frame");
    let refused = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
    assert!(!refused, "the card refused a Lens Blur frame");
    (cpu, card)
}

/// The Draft frame at radius 200 by the CPU and by the card, in `verification/B-775 pictures/`,
/// and their difference: black where they agree, grey where a channel is 1 level apart, white
/// where 2 or more; a pixel nobody can see is black.
fn pictures(cpu: &[u8], card: &[u8]) {
    let (w, h) = (480, 270);
    assert_eq!(cpu.len(), w * h * 4, "a Draft frame of the reference shot is 480 by 270");
    let dir = repo("verification/B-775 pictures");
    fs::create_dir_all(&dir).expect("make the pictures folder");
    let diff: Vec<u8> = cpu
        .chunks_exact(4)
        .zip(card.chunks_exact(4))
        .flat_map(|(a, b)| {
            let v = seen(a, b).min(2) * 127;
            [v, v, v, 255]
        })
        .collect();
    for (name, bytes) in [("cpu.png", cpu), ("card.png", card), ("difference.png", &diff[..])] {
        png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).expect("write a picture");
    }
}

fn fixtures() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(repo("Fixtures/lens_blur"))
        .expect("read the Lens Blur fixtures")
        .map(|f| f.expect("a file").path())
        .filter(|f| {
            let name = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
            name.ends_with(".json") && !name.starts_with("expected")
        })
        .collect();
    files.sort();
    files
}

#[test]
fn b775_fft_lens_blur() {
    let mut gpu = Gpu::new().expect("a usable card");
    gpu.lens_fft(Some(1.0));
    let mut t = Table::new(
        "lens_blur",
        &format!(
            "# B-775 (D-895): Lens Blur by Fourier transform on the viewer's card\n\n\
             Card: {}. The card's Lens Blur with transparent edges and no blur map worked by \
             Fourier transform at every radius, against the CPU's exact gather, both as the page \
             receives them (8-bit straight sRGB). The bar (wave 8, D-894): within 1 level of 255 \
             on at least 99.9% of the pixels a viewer can see and within 2 everywhere; a pixel \
             fully transparent on both sides is not graded, and must look the same over black \
             and over white.\n",
            gpu.about()
        ),
    );
    t.heading("The reference shot at frame 100, D-222's Lens Blur on its first three layers");
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    for radius in RADII {
        let project = shot(Some(lens(radius)));
        for q in [PreviewQuality::Full, PreviewQuality::Draft] {
            let before = gpu.lens_ffts();
            let (cpu, card) = both(&project, &comp, FRAME, &root, q, &mut gpu);
            let worked = gpu.lens_ffts() - before;
            let mut s = Stats::default();
            s.add(&cpu, &card);
            if radius == 200.0 && q == PreviewQuality::Draft {
                pictures(&cpu, &card);
            }
            t.row(
                &format!("radius {radius}, {}", q.label()),
                &format!("{worked} Lens Blurs by transform; {}", s.text()),
                worked == 3 && s.ok(),
            );
        }
    }

    t.heading("Every Lens Blur fixture, first and middle frames, Full and Draft");
    let (mut all, mut worked_all, mut shots) = (Stats::default(), 0, 0);
    for file in fixtures() {
        // A fixture the loader refuses (the map cycle) has no frame to compare.
        let Ok(loaded) = persist::load(&file) else { continue };
        let project = loaded.document.project().clone();
        let root = file.parent().expect("a folder").to_path_buf();
        let Some(c) = project.compositions.first() else { continue };
        let (id, start, n) = (c.id.clone(), c.start_frame, c.duration_frames as i32);
        let frames = if n > 1 { vec![start, start + n / 2] } else { vec![start] };
        let before = gpu.lens_ffts();
        let mut s = Stats::default();
        for q in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &f in &frames {
                let (cpu, card) = both(&project, &id, f, &root, q, &mut gpu);
                s.add(&cpu, &card);
                all.add(&cpu, &card);
            }
        }
        let worked = gpu.lens_ffts() - before;
        worked_all += worked;
        shots += 1;
        let name = file.file_stem().unwrap_or_default().to_string_lossy().into_owned();
        t.row(&name, &format!("{worked} Lens Blurs by transform; {}", s.text()), s.ok());
    }
    t.row(
        &format!("all {shots} fixtures together"),
        &format!("{worked_all} Lens Blurs by transform; {}", all.text()),
        all.ok() && worked_all > 0,
    );

    t.heading("The switch");
    let mut fresh = Gpu::new().expect("a usable card");
    for (radius, q, want) in [
        (50.0, PreviewQuality::Full, 0),
        (100.0, PreviewQuality::Full, 3),
        (200.0, PreviewQuality::Full, 3),
        (200.0, PreviewQuality::Draft, 0),
    ] {
        let before = fresh.lens_ffts();
        let (cpu, card) = both(&shot(Some(lens(radius))), &comp, FRAME, &root, q, &mut fresh);
        let worked = fresh.lens_ffts() - before;
        let mut s = Stats::default();
        s.add(&cpu, &card);
        t.row(
            &format!("as shipped (from radius {LENS_FFT_FROM}), radius {radius}, {}", q.label()),
            &format!("{worked} Lens Blurs by transform (expected {want}); largest {}", s.largest),
            worked == want && s.ok(),
        );
    }
    gpu.lens_fft(None);
    let project = shot(Some(lens(200.0)));
    let before = gpu.lens_ffts();
    let (cpu, card) = both(&project, &comp, FRAME, &root, PreviewQuality::Full, &mut gpu);
    let (largest, _) = effect_table::distance(&cpu, &card);
    t.row(
        "switched off, radius 200, Full",
        &format!("{} Lens Blurs by transform; the exact gather, largest difference {largest}", gpu.lens_ffts() - before),
        gpu.lens_ffts() == before && largest <= 1,
    );
    t.finish("B-775_fft_lens_blur_table.md");
}

/// Run deliberately, in release: frame 100 of the reference shot with D-222's Lens Blur on three
/// layers, the exact gather and the transform turn about, `B775_ROUNDS` rounds (5 unless set),
/// each the middle of 11 drawings after the card let go of what it kept.
#[test]
#[ignore]
fn b775_fft_lens_blur_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let rounds: usize = std::env::var("B775_ROUNDS").ok().and_then(|r| r.parse().ok()).unwrap_or(5);
    let (root, comp) = (repo("Fixtures/reference_shot"), Id::new("comp-reference-shot"));
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- Build: {}\n- Rounds: {rounds}, gather and transform turn about; each the middle of 11 drawings\n- The transform is used at every radius here; the viewer uses it from radius {LENS_FFT_FROM} (D-895)\n- Other lanes were building and testing on the same machine, so single rounds can jump\n\n| Shot | Quality | Gather (each round) | Transform (each round) | Gather | Transform |\n|---|---|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let mut shots = vec![(String::from("none"), shot(None))];
    shots.extend(RADII.iter().map(|&r| (format!("Lens Blur {r}"), shot(Some(lens(r))))));
    for (name, project) in &shots {
        for q in [PreviewQuality::Draft, PreviewQuality::Full] {
            let mut cache = CelCache::viewer();
            let (mut gather, mut fft) = (Vec::new(), Vec::new());
            for round in 0..rounds * 2 {
                let on = round % 2 == 1;
                gpu.lens_fft(on.then_some(1.0));
                let mut times = Vec::new();
                for pass in 0..12 {
                    gpu.forget();
                    let mut log = FrameLog::new(3);
                    let t = std::time::Instant::now();
                    drop(
                        preview::preview_frame_srgb8(project, &comp, FRAME, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                            .expect("GPU frame"),
                    );
                    if pass > 0 {
                        times.push(t.elapsed().as_secs_f64() * 1000.0);
                    }
                }
                if on { &mut fft } else { &mut gather }.push(median(times));
            }
            let list = |v: &[f64]| v.iter().map(|x| format!("{x:.1}")).collect::<Vec<_>>().join(", ");
            let row = format!(
                "| {name} | {} | {} | {} | {:.1} | {:.1} |\n",
                q.label(),
                list(&gather),
                list(&fft),
                median(gather.clone()),
                median(fft.clone())
            );
            eprint!("{row}");
            s.push_str(&row);
        }
    }
    fs::write(repo("verification/B-775_timing_table.md"), s).expect("write the timing");
}
