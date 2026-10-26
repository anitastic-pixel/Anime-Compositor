//! B-754 (D-874): Float and 32 bpc (After Effects) frames on the graphics card, against the CPU.
//!
//! Every fixture project, with each composition set to Float and again to 32 bpc (After
//! Effects), and every fixture saved in either as it was saved, is drawn by the CPU and by the
//! card at its first and middle frames, Full and Draft. Each effect the card drew is held over
//! all the pixels of its fixtures to D-874's bar: at 8 bits within 1 level of 255 on at least
//! 99.9% of them and within 2 everywhere; in floats, the card's own frame (linear premultiplied,
//! before it is turned to 8 bits) within 1e-4 of the CPU's on solid areas (slope under 1e-3 a
//! pixel) and no edge moved by more than 0.05 px (the difference over the slope, where the slope
//! is at least 0.01 a pixel). A frame the card refuses must be the CPU's exactly. A scene pushed
//! past white and below black through Normal, Multiply, Screen and Add, Exposure and Fractal
//! Noise must come back from the card as the CPU keeps it.
//!
//! Writes `verification/B-754_float_card_table.md`. `B754_ONLY` limits the fixtures by path.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render::OnCard;
use serde_json::{json, Value};

mod effect_table;
use effect_table::{repo, Table};

/// The fixture project files, as `b222_gpu_colour` walks them.
fn files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for folder in fs::read_dir(repo("Fixtures")).expect("read Fixtures") {
        let folder = folder.expect("a folder").path();
        if !folder.is_dir() {
            continue;
        }
        for f in fs::read_dir(&folder).expect("read a fixture folder") {
            let f = f.expect("a file").path();
            let name = f.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if name.ends_with(".json") && !name.starts_with("expected") {
                files.push(f);
            }
        }
    }
    files.sort();
    files
}

/// The names of the effects the card is left on the frame's layers.
fn on_card(project: &Project, comp: &Id, frame: i32, root: &Path, q: PreviewQuality) -> Vec<String> {
    let mut log = FrameLog::new(3);
    let Ok(plan) = compose::plan_frame_for_card(project, comp, frame, root, q, &mut log, &mut CelCache::viewer()) else {
        return Vec::new();
    };
    let mut names: Vec<String> = plan
        .layers
        .iter()
        .flat_map(|l| l.on_card.iter())
        .map(|c| match c.unmixed() {
            OnCard::Fx(f) => f.instance.effect.name().to_string(),
            other => format!("{other:?}").split(['(', ' ']).next().unwrap_or("?").to_string(),
        })
        .collect();
    names.sort();
    names.dedup();
    names
}

#[derive(Default, Clone, Copy)]
struct Float {
    /// Largest difference anywhere, on solid areas, and the largest edge shift in pixels.
    any: f64,
    solid: f64,
    shift: f64,
}

impl Float {
    fn of(cpu: &[f32], card: &[f32], w: usize, h: usize) -> Float {
        let mut f = Float::default();
        for y in 0..h {
            for x in 0..w {
                for c in 0..4 {
                    let i = (y * w + x) * 4 + c;
                    let e = (cpu[i] as f64 - card[i] as f64).abs();
                    f.any = f.any.max(e);
                    if x == 0 || y == 0 || x + 1 == w || y + 1 == h {
                        continue;
                    }
                    let at = |x: usize, y: usize| cpu[(y * w + x) * 4 + c] as f64;
                    let slope = ((at(x + 1, y) - at(x - 1, y)) / 2.0).hypot((at(x, y + 1) - at(x, y - 1)) / 2.0);
                    if slope < 1e-3 {
                        f.solid = f.solid.max(e);
                    } else if slope >= 0.01 {
                        f.shift = f.shift.max(e / slope);
                    }
                }
            }
        }
        f
    }
    fn max(self, o: Float) -> Float {
        Float { any: self.any.max(o.any), solid: self.solid.max(o.solid), shift: self.shift.max(o.shift) }
    }
    fn ok(self) -> bool {
        self.solid <= 1e-4 && self.shift <= 0.05
    }
}

#[derive(Default)]
struct Stats {
    fixtures: usize,
    frames: usize,
    pixels: usize,
    over1: usize,
    over2: usize,
    largest: u8,
    float: Float,
}

/// One frame by the CPU and by the card: the CPU's floats and bytes, the card's bytes, its floats
/// when it drew the frame, or `None` when it refused it.
fn both(project: &Project, id: &Id, frame: i32, root: &Path, q: PreviewQuality, gpu: &mut Gpu, name: &str) -> Option<(Vec<f32>, usize, usize, Vec<u8>, Vec<u8>, Option<Vec<f32>>)> {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, id, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache).ok()?;
    let (w, h) = (c.width(), c.height());
    let bytes = c.to_srgb8_straight();
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, id, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("{name} frame {frame} on the card: {}", d.message));
    let refused = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
    let sum = (!refused).then(|| gpu.sum_back().expect("the card's frame back"));
    Some((c.data().to_vec(), w, h, bytes, g, sum))
}

fn levels(a: &[u8], b: &[u8]) -> (u8, usize, usize) {
    let (mut largest, mut over1, mut over2) = (0u8, 0usize, 0usize);
    for (a, b) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        // A pixel whose alpha is 0 on both has no colour to see: the CPU's alpha there is exactly
        // 0 and its colour 0, the card's a few 1e-9 and its colour divided out of it. Its alpha
        // is still compared, and its floats are in the float bar.
        let n = if a[3] == 0 && b[3] == 0 { 3 } else { 0 };
        let d = a.iter().zip(b).skip(n).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        over1 += (d > 1) as usize;
        over2 += (d > 2) as usize;
    }
    (largest, over1, over2)
}

#[test]
fn b754_float_card() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut t = Table::new(
        "float_depth",
        &format!(
            "# B-754 (D-874): Float and 32 bpc (After Effects) frames on the graphics card\n\n\
             Card: {}. Every fixture project with each composition set to Float, and again to 32 \
             bpc (After Effects), and every fixture saved in either as saved, drawn by the CPU and \
             by the card at its first and middle frames, Full and Draft. D-874's bar for each effect \
             over all its fixtures' pixels: at 8 bits within 1 level of 255 on at least 99.9% and \
             within 2 everywhere; in floats (the card's own linear frame before it is turned to 8 \
             bits) within 1e-4 on solid areas (slope under 1e-3 a pixel), and no edge moved by more \
             than 0.05 px (the difference over the slope where it is at least 0.01). A frame the \
             card refuses must be the CPU's exactly.\n",
            gpu.about()
        ),
    );
    let only = std::env::var("B754_ONLY").ok();
    let mut old_style = Vec::new();
    for (depth, ae) in [("Float", false), ("32 bpc (After Effects)", true)] {
        let mut by_effect: BTreeMap<String, Stats> = BTreeMap::new();
        let (mut drawn, mut refused, mut refused_exact, mut shots) = (0usize, 0usize, 0usize, 0usize);
        let mut why: BTreeMap<String, usize> = BTreeMap::new();
        for file in files() {
            if only.as_ref().is_some_and(|o| !file.to_string_lossy().contains(o.as_str())) {
                continue;
            }
            let Ok(text) = fs::read_to_string(&file) else { continue };
            let Ok(mut j) = serde_json::from_str::<Value>(&text) else { continue };
            let saved = if ae { text.contains("\"ae_32bpc\": true") } else { text.contains("\"float_depth\": true") && !text.contains("\"ae_32bpc\": true") };
            let Some(comps) = j["compositions"].as_array_mut() else { continue };
            for c in comps.iter_mut() {
                let Some(o) = c.as_object_mut() else { continue };
                o.remove("eight_bpc");
                o.remove("ae_32bpc");
                o.insert("float_depth".into(), Value::from(true));
                if ae {
                    o.insert("ae_32bpc".into(), Value::from(true));
                }
            }
            // A file saved in this depth is opened from disk as saved, the old-style check.
            let loaded = if saved { persist::load(&file) } else { persist::load_str(&j.to_string()) };
            let Ok(loaded) = loaded else { continue };
            let project = loaded.document.project().clone();
            let root = file.parent().expect("a folder").to_path_buf();
            let Some(comp) = project.compositions.first() else { continue };
            let (id, start, n) = (comp.id.clone(), comp.start_frame, comp.duration_frames as i32);
            let frames = if n > 1 { vec![start, start + n / 2] } else { vec![start] };
            let name = format!(
                "{}/{}",
                root.file_name().unwrap_or_default().to_string_lossy(),
                file.file_stem().unwrap_or_default().to_string_lossy()
            );
            shots += 1;
            let mut counted: Vec<String> = Vec::new();
            let mut old_ok = true;
            for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                for &frame in &frames {
                    let Some((cpu, w, h, c, g, sum)) = both(&project, &id, frame, &root, q, &mut gpu, &name) else { continue };
                    let (largest, over1, over2) = levels(&c, &g);
                    let Some(sum) = sum else {
                        refused += 1;
                        refused_exact += (largest == 0) as usize;
                        old_ok &= largest == 0;
                        let mut log = FrameLog::new(3);
                        let _ = preview::preview_frame_srgb8(&project, &id, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer(), &mut gpu);
                        if let Some(d) = log.finish().iter().find(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str()) {
                            *why.entry(d.message.clone()).or_default() += 1;
                        }
                        continue;
                    };
                    drawn += 1;
                    let f = Float::of(&cpu, &sum, w, h);
                    if largest > 1 || !f.ok() {
                        eprintln!("{depth} {name} frame {frame} {}: largest {largest}, {over1} over 1; float {:.2e} solid {:.2e} shift {:.3}", q.label(), f.any, f.solid, f.shift);
                    }
                    old_ok &= largest <= 1 && f.ok();
                    let mut here = on_card(&project, &id, frame, &root, q);
                    if here.is_empty() {
                        here.push("(layers only, no effect on the card)".into());
                    }
                    for e in here {
                        let s = by_effect.entry(e.clone()).or_default();
                        if !counted.contains(&e) {
                            s.fixtures += 1;
                            counted.push(e);
                        }
                        s.frames += 1;
                        s.pixels += c.len() / 4;
                        s.over1 += over1;
                        s.over2 += over2;
                        s.largest = s.largest.max(largest);
                        s.float = s.float.max(f);
                    }
                }
            }
            if saved {
                old_style.push((format!("{name} ({depth})"), old_ok));
            }
        }
        t.heading(&format!("Each effect the card drew in {depth}, over all its fixtures"));
        for (e, s) in &by_effect {
            let within = 100.0 * (s.pixels - s.over1) as f64 / s.pixels.max(1) as f64;
            let ok = s.over2 == 0 && (s.pixels - s.over1) * 1000 >= s.pixels * 999 && s.float.ok();
            t.row(
                e,
                &format!(
                    "{} fixtures, {} frames, {} pixels: largest {}, {} over 1, {} over 2, {within:.4}% within 1; float largest {:.2e}, solid {:.2e}, edge shift {:.4} px",
                    s.fixtures, s.frames, s.pixels, s.largest, s.over1, s.over2, s.float.any, s.float.solid, s.float.shift
                ),
                ok,
            );
        }
        t.heading(&format!("All the frames in {depth}"));
        t.row(
            &format!("{shots} fixture projects"),
            &format!("{drawn} frames on the card; {refused} the card refused, {refused_exact} of them the CPU's exactly"),
            refused == refused_exact && drawn > 0,
        );
        for (message, count) in &why {
            t.row(&format!("Refused ({count} frames)"), message, true);
        }
    }
    t.heading("Projects saved in Float or 32 bpc (After Effects) before B-754 open and draw as saved");
    for (name, ok) in &old_style {
        t.row(name, if *ok { "opens; the card's frame within D-874's bar of the CPU's, or the CPU's exactly" } else { "differs" }, *ok);
    }
    over_range(&mut t, &mut gpu);
    t.finish("B-754_float_card_table.md");
}

/// A card picture pushed four times past white and below black by Exposure, laid in each of the
/// four modes the card draws over a copy of itself, with Fractal Noise past white on top.
fn over_range(t: &mut Table, gpu: &mut Gpu) {
    t.heading("Values past white and below black survive the card as the CPU keeps them");
    let root = repo("Fixtures/float_depth");
    let base: Value = serde_json::from_str(&fs::read_to_string(root.join("fx_blend_addf_001.json")).unwrap()).unwrap();
    for mode in ["normal", "multiply", "screen", "add"] {
        let (mut high, mut low) = (f32::MIN, f32::MAX);
        for (noise, opacity) in [("none", 0.5), ("screen", 0.75), ("add", 1.0)] {
            let mut j = base.clone();
            let layers = j["compositions"][0]["layers"].as_array_mut().unwrap();
            layers[0]["effects"] = json!([{ "instance_id": "fx-0-0", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": -2, "offset": -0.5 } }]);
            layers[1]["blend_mode"] = json!(mode);
            layers[1]["transform"]["opacity"]["base"] = json!(opacity);
            layers[1]["transform"]["position"]["base"] = json!([9.5, 5.25]);
            let mut fx = vec![json!({ "instance_id": "fx-1-0", "type_id": "core.exposure", "enabled": true, "parameters": { "stops": 3, "offset": 0.0 } })];
            if noise != "none" {
                fx.push(json!({ "instance_id": "fx-1-1", "type_id": "core.fractal_noise", "enabled": true, "parameters": { "size": 4, "complexity": 4, "contrast": 300, "brightness": 50, "evolution": 0, "speed": 0, "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": noise } }));
            }
            layers[1]["effects"] = json!(fx);
            let project = persist::load_str(&j.to_string()).expect("the scene opens").document.project().clone();
            let id = project.compositions[0].id.clone();
            for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                let what = format!("{mode} over the pushed card, opacity {opacity}, Fractal Noise {noise}, {}", q.label());
                let Some((cpu, w, h, c, g, sum)) = both(&project, &id, 0, &root, q, gpu, &what) else {
                    t.row(&what, "the CPU drew nothing", false);
                    continue;
                };
                let Some(sum) = sum else {
                    t.row(&what, "the card refused it", false);
                    continue;
                };
                let (top, bottom) = cpu.iter().fold((f32::MIN, f32::MAX), |(a, b), v| (a.max(*v), b.min(*v)));
                (high, low) = (high.max(top), low.min(bottom));
                let (ctop, cbottom) = sum.iter().fold((f32::MIN, f32::MAX), |(a, b), v| (a.max(*v), b.min(*v)));
                let worst = cpu.iter().zip(&sum).map(|(a, b)| (*a as f64 - *b as f64).abs() / (a.abs() as f64).max(1.0)).fold(0.0, f64::max);
                let f = Float::of(&cpu, &sum, w, h);
                let (largest, ..) = levels(&c, &g);
                if opacity == 1.0 && q == PreviewQuality::Full {
                    pictures(mode, w, h, [("cpu", &c, &cpu), ("card", &g, &sum)]);
                }
                t.row(
                    &what,
                    &format!(
                        "CPU {bottom:.4} to {top:.4}, card {cbottom:.4} to {ctop:.4}; largest difference {worst:.2e} of the value (or of 1 under 1), solid {:.2e}, edge shift {:.4} px, {largest} levels at 8 bits",
                        f.solid, f.shift
                    ),
                    worst <= 1e-4 && f.ok() && largest <= 1,
                );
            }
        }
        t.row(&format!("{mode}: the CPU's frames went past white and below black"), &format!("{low:.4} to {high:.4}"), high > 1.0 && low < 0.0);
    }
}

/// `verification/B-754 pictures/`: each side's frame as shown, and its floats three stops down,
/// so what is past white shows; 24 times as large, each pixel a square.
fn pictures(mode: &str, w: usize, h: usize, sides: [(&str, &Vec<u8>, &Vec<f32>); 2]) {
    let dir = repo("verification/B-754 pictures");
    fs::create_dir_all(&dir).unwrap();
    let k = 24;
    let big = |px: &dyn Fn(usize) -> [u8; 4]| {
        let mut out = Vec::with_capacity(w * h * k * k * 4);
        for y in 0..h * k {
            for x in 0..w * k {
                out.extend(px((y / k) * w + x / k));
            }
        }
        out
    };
    for (side, bytes, floats) in sides {
        let shown = big(&|i| bytes[i * 4..i * 4 + 4].try_into().unwrap());
        let down = big(&|i| {
            let p = &floats[i * 4..i * 4 + 4];
            let a = p[3].max(1e-6);
            let c = |v: f32| (anime_compositor::color::linear_to_srgb((v / a / 8.0).clamp(0.0, 1.0)) * 255.0).round() as u8;
            [c(p[0]), c(p[1]), c(p[2]), (p[3].clamp(0.0, 1.0) * 255.0).round() as u8]
        });
        for (what, px) in [("as_shown", shown), ("3_stops_down", down)] {
            let path = dir.join(format!("{mode}_{side}_{what}.png"));
            anime_compositor::png_out::write_rgba(&path, w * k, h * k, anime_compositor::OutputDepth::Eight, &[], &px).unwrap();
        }
    }
}
