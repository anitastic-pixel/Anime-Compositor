//! B-688 (D-808): 8 bpc (After Effects) frames on the graphics card, against the CPU.
//!
//! Every fixture project, every composition set to 8 bpc, whose frame leaves an effect to the
//! card, and every fixture saved in 8 bpc as it was saved, is drawn by the CPU and by the card at
//! its first and middle frames, Full and Draft. Each effect the card drew is held to D-808's bar
//! over all the pixels of its fixtures: within 1 level of 255 on at least 99.9% of them and
//! within 2 everywhere. A frame the card refuses must be the CPU's exactly.
//!
//! Writes `verification/B-688_eight_card_table.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render::OnCard;
use serde_json::Value;

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

/// The names of the effects the card is left in 8 bpc on the frame's layers.
fn on_card(project: &anime_compositor::model::Project, comp: &anime_compositor::model::Id, frame: i32, root: &std::path::Path, q: PreviewQuality) -> Vec<String> {
    let mut log = FrameLog::new(3);
    let Ok(plan) = compose::plan_frame_for_card(project, comp, frame, root, q, &mut log, &mut CelCache::viewer()) else {
        return Vec::new();
    };
    let mut names: Vec<String> = plan
        .layers
        .iter()
        .flat_map(|l| l.on_card.iter())
        .filter_map(|c| match c {
            OnCard::Eight(inner, _) => match inner.unmixed() {
                OnCard::Fx(f) => Some(f.instance.effect.name().to_string()),
                other => Some(format!("{other:?}").split(['(', ' ']).next().unwrap_or("?").to_string()),
            },
            _ => None,
        })
        .collect();
    names.sort();
    names.dedup();
    names
}

#[derive(Default)]
struct Stats {
    fixtures: usize,
    frames: usize,
    pixels: usize,
    over1: usize,
    over2: usize,
    largest: u8,
}

#[test]
fn b688_eight_card() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut by_effect: BTreeMap<String, Stats> = BTreeMap::new();
    let (mut drawn, mut refused, mut refused_exact, mut shots) = (0usize, 0usize, 0usize, 0usize);
    let mut t = Table::new(
        "eight_bpc",
        &format!(
            "# B-688 (D-808): 8 bpc (After Effects) frames on the graphics card\n\n\
             Card: {}. Every fixture project with each composition set to 8 bpc whose frame leaves \
             an effect to the card, and every fixture saved in 8 bpc as saved, drawn by the CPU \
             and by the card at its first and middle frames, Full and Draft. D-808's bar for each \
             effect over all its fixtures' pixels: within 1 level of 255 on at least 99.9% and \
             within 2 everywhere. A frame the card refuses must be the CPU's exactly.\n",
            gpu.about()
        ),
    );
    let mut old_style = Vec::new();
    let only = std::env::var("B688_ONLY").ok();
    for file in files() {
        if only.as_ref().is_some_and(|o| !file.to_string_lossy().contains(o.as_str())) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&file) else { continue };
        let Ok(mut j) = serde_json::from_str::<Value>(&text) else { continue };
        let saved_eight = text.contains("\"eight_bpc\": true");
        let Some(comps) = j["compositions"].as_array_mut() else { continue };
        for c in comps.iter_mut() {
            let Some(o) = c.as_object_mut() else { continue };
            o.remove("float_depth");
            o.remove("ae_32bpc");
            o.insert("eight_bpc".into(), Value::from(true));
        }
        let Ok(loaded) = persist::load_str(&j.to_string()) else { continue };
        let project = loaded.document.project().clone();
        let root = file.parent().expect("a folder").to_path_buf();
        let Some(comp) = project.compositions.first() else { continue };
        let (id, start, n) = (comp.id.clone(), comp.start_frame, comp.duration_frames as i32);
        let frames = if n > 1 { vec![start, start + n / 2] } else { vec![start] };
        let names = |q| frames.iter().flat_map(|&f| on_card(&project, &id, f, &root, q)).collect::<Vec<_>>();
        if !saved_eight && names(PreviewQuality::Full).is_empty() && names(PreviewQuality::Draft).is_empty() {
            continue;
        }
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
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let Ok(c) = preview::preview_frame_cached(&project, &id, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache) else {
                    continue;
                };
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&project, &id, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the card: {}", d.message));
                let cpu = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
                let (mut largest, mut over1, mut over2) = (0u8, 0usize, 0usize);
                for (a, b) in c.chunks_exact(4).zip(g.chunks_exact(4)) {
                    let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
                    largest = largest.max(d);
                    over1 += (d > 1) as usize;
                    over2 += (d > 2) as usize;
                }
                if cpu {
                    refused += 1;
                    refused_exact += (largest == 0) as usize;
                    old_ok &= largest == 0;
                    continue;
                }
                drawn += 1;
                if largest > 1 {
                    eprintln!("{name} frame {frame} {}: largest {largest}, {over1} over 1", q.label());
                }
                old_ok &= largest <= 1;
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
                }
            }
        }
        if saved_eight {
            old_style.push((name, old_ok));
        }
    }
    t.heading("Each effect the card drew in 8 bpc, over all its fixtures");
    for (e, s) in &by_effect {
        let within = 100.0 * (s.pixels - s.over1) as f64 / s.pixels.max(1) as f64;
        let ok = s.over2 == 0 && (s.pixels - s.over1) * 1000 >= s.pixels * 999;
        t.row(
            e,
            &format!(
                "{} fixtures, {} frames, {} pixels: largest {}, {} over 1, {} over 2, {within:.4}% within 1",
                s.fixtures, s.frames, s.pixels, s.largest, s.over1, s.over2
            ),
            ok,
        );
    }
    t.heading("Projects saved in 8 bpc before B-688 open and draw as saved");
    for (name, ok) in &old_style {
        t.row(name, if *ok { "opens; the card's frame within 1 level of the CPU's, or the CPU's exactly" } else { "differs" }, *ok);
    }
    t.heading("All the frames");
    t.row(
        &format!("{shots} fixture projects"),
        &format!("{drawn} frames on the card; {refused} the card refused, {refused_exact} of them the CPU's exactly"),
        refused == refused_exact && drawn > 0,
    );
    t.finish("B-688_eight_card_table.md");
}
