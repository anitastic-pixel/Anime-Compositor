//! What B-749 and B-750 share (D-869, D-870): the fixture projects a test picks, each
//! composition set to 8 bpc (After Effects), drawn by the CPU and by the card at the first and
//! middle frames, Full and Draft, and held to wave 8's bar.
//!
//! Every item is `#[allow(dead_code)]` for the reason `tests/common/mod.rs` gives.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::render::OnCard;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::Value;

use crate::effect_table::{hidden, seen, shown};

pub fn repo(rel: &str) -> PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Pixels held to the bar: within 1 level of 255 on at least 99.9% and within 2 everywhere.
#[derive(Default, Clone)]
pub struct Stats {
    pub fixtures: usize,
    pub frames: usize,
    pub pixels: usize,
    pub over1: usize,
    pub over2: usize,
    pub largest: u8,
    pub exact_frames: usize,
    /// Frames whose plan leaves the group's effect to the card in 8 bpc.
    pub on_card: usize,
    /// D-894: pixels fully transparent on both sides whose hidden colours differ, not graded,
    /// and the largest difference they show over black or white (0 by the rule's proof).
    pub excused: usize,
    pub excused_shown: u8,
}

impl Stats {
    pub fn ok(&self) -> bool {
        self.over2 == 0 && (self.pixels - self.over1) * 1000 >= self.pixels * 999
    }
    /// `effect`: whether the group is an effect, whose frames with it on the card are counted.
    pub fn text(&self, effect: bool) -> String {
        let on = if effect { format!("{} with it on the card, ", self.on_card) } else { String::new() };
        format!(
            "{} fixtures, {} frames drawn on the card ({on}{} byte for byte), {} pixels: largest {}, {} over 1, {} over 2, {:.4}% within 1",
            self.fixtures,
            self.frames,
            self.exact_frames,
            self.pixels,
            self.largest,
            self.over1,
            self.over2,
            100.0 * (self.pixels - self.over1) as f64 / self.pixels.max(1) as f64
        ) + &excused(self.excused, self.excused_shown)
    }
}

/// D-894: what a table row says of the pixels the rule did not grade, when there are any.
pub fn excused(n: usize, shown: u8) -> String {
    if n == 0 {
        String::new()
    } else {
        format!("; {n} fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white {shown}")
    }
}

/// What a walk found: per group, the frames the card drew; the card's refusals by reason, and
/// whether each refused frame was the CPU's exactly.
#[derive(Default)]
pub struct Walk {
    pub drawn: BTreeMap<String, Stats>,
    pub refused: BTreeMap<String, (usize, usize)>,
    pub shots: usize,
    pub worst: Vec<String>,
}

/// Every fixture project whose text `pick` takes, set to 8 bpc and changed by `edit`, its frames
/// counted under each of the groups `groups` names for that text.
pub fn walk(
    gpu: &mut Gpu,
    pick: impl Fn(&str) -> bool,
    edit: impl Fn(&mut Value),
    groups: impl Fn(&str) -> Vec<String>,
) -> Walk {
    let mut w = Walk::default();
    let only = std::env::var("EIGHT_ONLY").ok();
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
    for file in files {
        if only.as_ref().is_some_and(|o| !file.to_string_lossy().contains(o.as_str())) {
            continue;
        }
        let Ok(text) = fs::read_to_string(&file) else { continue };
        if !pick(&text) {
            continue;
        }
        let Ok(mut j) = serde_json::from_str::<Value>(&text) else { continue };
        let Some(comps) = j["compositions"].as_array_mut() else { continue };
        for c in comps.iter_mut() {
            let Some(o) = c.as_object_mut() else { continue };
            o.remove("float_depth");
            o.remove("ae_32bpc");
            o.insert("eight_bpc".into(), Value::from(true));
        }
        edit(&mut j);
        let Ok(loaded) = persist::load_str(&j.to_string()) else { continue };
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
        w.shots += 1;
        let groups = groups(&text);
        let mut counted = Vec::new();
        for q in [PreviewQuality::Full, PreviewQuality::Draft] {
            for &frame in &frames {
                let mut cache = CelCache::viewer();
                let mut log = FrameLog::new(3);
                let Ok(c) = preview::preview_frame_cached(&project, &id, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache) else {
                    continue;
                };
                let c = c.to_srgb8_straight();
                let mut log = FrameLog::new(3);
                let (g, ..) = preview::preview_frame_srgb8(&project, &id, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
                    .unwrap_or_else(|d| panic!("{name} frame {frame} on the card: {}", d.message));
                let why = log
                    .finish()
                    .iter()
                    .find(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str())
                    .map(|d| d.message.clone());
                let (mut largest, mut over1, mut over2, mut every) = (0u8, 0usize, 0usize, 0u8);
                let (mut excused, mut excused_shown) = (0usize, 0u8);
                for (a, b) in c.chunks_exact(4).zip(g.chunks_exact(4)) {
                    let d = seen(a, b);
                    largest = largest.max(d);
                    every = every.max(a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0));
                    over1 += (d > 1) as usize;
                    over2 += (d > 2) as usize;
                    if hidden(a, b) {
                        excused += 1;
                        excused_shown = excused_shown.max(shown(a, b));
                    }
                }
                if let Some(why) = why {
                    let r = w.refused.entry(why).or_default();
                    r.0 += 1;
                    r.1 += (every == 0) as usize;
                    continue;
                }
                if largest > 1 {
                    w.worst.push(format!("{name} frame {frame} {}: largest {largest}, {over1} over 1", q.label()));
                }
                let here = on_card(&project, &id, frame, &root, q);
                for e in &groups {
                    let s = w.drawn.entry(e.clone()).or_default();
                    s.on_card += here.contains(e) as usize;
                    if !counted.contains(e) {
                        s.fixtures += 1;
                        counted.push(e.clone());
                    }
                    s.frames += 1;
                    s.exact_frames += (every == 0) as usize;
                    s.excused += excused;
                    s.excused_shown = s.excused_shown.max(excused_shown);
                    s.pixels += c.len() / 4;
                    s.over1 += over1;
                    s.over2 += over2;
                    s.largest = s.largest.max(largest);
                }
            }
        }
    }
    w
}

/// The names of the effects the card is left in 8 bpc on the frame's layers, as `b688` has them.
fn on_card(
    project: &anime_compositor::model::Project,
    comp: &anime_compositor::model::Id,
    frame: i32,
    root: &std::path::Path,
    q: PreviewQuality,
) -> Vec<String> {
    let mut log = FrameLog::new(3);
    let Ok(plan) = compose::plan_frame_for_card(project, comp, frame, root, q, &mut log, &mut CelCache::viewer()) else {
        return Vec::new();
    };
    plan.layers
        .iter()
        .flat_map(|l| l.on_card.iter())
        .filter_map(|c| match c {
            OnCard::Eight(inner, _) => match inner.unmixed() {
                OnCard::Fx(f) => Some(f.instance.effect.name().to_string()),
                other => Some(format!("{other:?}").split(['(', ' ']).next().unwrap_or("?").to_string()),
            },
            _ => None,
        })
        .collect()
}
