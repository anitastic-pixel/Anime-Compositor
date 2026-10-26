//! B-748 (D-868): every layer blend mode on the graphics card, against the CPU.
//!
//! Before this the card drew Normal, Multiply, Screen and Add only, and a frame with any other
//! mode was drawn on the CPU whole. Three sets of frames are drawn by both and held to the bar
//! (within 1 level of 255 on at least 99.9% of pixels, within 2 everywhere); a frame the card
//! refuses must be the CPU's exactly:
//! 1. Every fixture saved with a mode beyond the four, as saved and set to 8 bpc, first and
//!    middle frames, Full and Draft. These are also the projects saved before B-748: they open.
//! 2. Each of the 16 modes, with Preserve Transparency off and on, in linear light and 8 bpc,
//!    a 384 by 384 layer of mixed pixels (0, 0.5, 1 and in between, at alphas from clear to
//!    opaque, at 70% opacity) set 100 px in over a 512 by 512 backdrop of the same kind, so part
//!    of the frame lies outside the layer, where a stencil clears it. A stencil scaled to
//!    nothing, which clears the whole frame, is the CPU's (the card refuses it).
//! 3. Every fixture project with two layers or more, each layer above the first set to one of
//!    the twelve modes beyond the four (in turn, by file), first frame, Full, as saved and in
//!    8 bpc.
//!
//! Writes `verification/B-748_card_blend_modes_table.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{BlendMode, Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render::{self, Affine, FramePlan, LayerDraw};
use anime_compositor::WorkingBuffer;
use serde_json::Value;

mod effect_table;
use effect_table::{repo, Table};

const CORE: [&str; 4] = ["normal", "multiply", "screen", "add"];

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

#[derive(Default)]
struct Stats {
    frames: usize,
    pixels: usize,
    over1: usize,
    over2: usize,
    largest: u8,
}

impl Stats {
    fn add(&mut self, a: &[u8], b: &[u8]) -> u8 {
        let mut largest = 0u8;
        for (a, b) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
            let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
            largest = largest.max(d);
            self.over1 += (d > 1) as usize;
            self.over2 += (d > 2) as usize;
        }
        self.frames += 1;
        self.pixels += a.len() / 4;
        self.largest = self.largest.max(largest);
        largest
    }

    fn ok(&self) -> bool {
        self.over2 == 0 && (self.pixels - self.over1) * 1000 >= self.pixels * 999
    }

    fn text(&self) -> String {
        format!(
            "{} frames, {} pixels: largest {}, {} over 1, {} over 2, {:.4}% within 1",
            self.frames,
            self.pixels,
            self.largest,
            self.over1,
            self.over2,
            100.0 * (self.pixels - self.over1) as f64 / self.pixels.max(1) as f64
        )
    }
}

/// The frame by the CPU and by the card, and whether the card refused it.
fn both(
    project: &Project,
    comp: &Id,
    frame: i32,
    root: &std::path::Path,
    q: PreviewQuality,
    gpu: &mut Gpu,
) -> Option<(Vec<u8>, Vec<u8>, bool)> {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let cpu = preview::preview_frame_cached(project, comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .ok()?
        .to_srgb8_straight();
    let mut log = FrameLog::new(3);
    let (card, ..) =
        preview::preview_frame_srgb8(project, comp, frame, root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
            .ok()?;
    let refused = log.finish().iter().any(|d| d.id.as_str() == DiagnosticId::GpuPreviewOnCpu.as_str());
    Some((cpu, card, refused))
}

/// `j`'s compositions set to 8 bpc.
fn eight(j: &mut Value) {
    for c in j["compositions"].as_array_mut().into_iter().flatten() {
        if let Some(o) = c.as_object_mut() {
            o.remove("float_depth");
            o.remove("ae_32bpc");
            o.insert("eight_bpc".into(), Value::from(true));
        }
    }
}

/// A `w` by `h` picture of mixed premultiplied pixels: each channel 0, 0.5, 1 or in between, at
/// alphas 0, 0.5, 1 or in between, from `seed`.
fn mixed(w: usize, h: usize, seed: u64) -> WorkingBuffer {
    let mut s = seed;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        let r = (s >> 11) as f32 / (1u64 << 53) as f32;
        match s % 5 {
            0 => 0.0,
            1 => 0.5,
            2 => 1.0,
            _ => r,
        }
    };
    let mut b = WorkingBuffer::transparent(w, h);
    for px in b.data_mut().chunks_exact_mut(4) {
        let a = next();
        for c in &mut px[..3] {
            *c = next() * a;
        }
        px[3] = a;
    }
    b
}

fn layer(id: &str, source: WorkingBuffer, transform: Affine, opacity: f32, blend: BlendMode, preserve: bool) -> LayerDraw {
    LayerDraw {
        id: Id::new(id),
        source: Arc::new(source),
        transform,
        opacity,
        matte: None,
        blend,
        adjust: None,
        nested: None,
        on_card: Vec::new(),
        motion_blur: false,
        preserve,
        moments: Vec::new(),
        mixed: false,
        wrap: Vec::new(),
    }
}

#[test]
fn b748_card_blend_modes() {
    let mut gpu = Gpu::new().expect("a usable card");
    let mut t = Table::new(
        "layer_blend_modes",
        &format!(
            "# B-748 (D-868): every layer blend mode on the graphics card\n\n\
             Card: {}. Written by `tests/b748_card_blend_modes.rs`. Each frame is drawn by the CPU \
             and by the card. The bar: within 1 level of 255 on at least 99.9% of pixels and \
             within 2 everywhere. A frame the card refuses must be the CPU's exactly.\n",
            gpu.about()
        ),
    );
    let only = std::env::var("B748_ONLY").ok();
    let all = files();

    // 1. The fixtures saved with a mode beyond the four.
    t.heading("Fixtures saved with a mode beyond Normal, Multiply, Screen and Add (saved before B-748), as saved and in 8 bpc");
    for file in &all {
        let text = fs::read_to_string(file).unwrap_or_default();
        let Ok(j) = serde_json::from_str::<Value>(&text) else { continue };
        let modes: Vec<String> = j["compositions"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|c| c["layers"].as_array().into_iter().flatten())
            .filter_map(|l| l["blend_mode"].as_str())
            .filter(|m| !CORE.contains(m))
            .map(str::to_string)
            .collect();
        if modes.is_empty() {
            continue;
        }
        let root = file.parent().unwrap().to_path_buf();
        let name = format!(
            "{}/{}",
            root.file_name().unwrap_or_default().to_string_lossy(),
            file.file_stem().unwrap_or_default().to_string_lossy()
        );
        for depth in ["as saved", "8 bpc"] {
            let mut j = j.clone();
            if depth == "8 bpc" {
                eight(&mut j);
            }
            let loaded = persist::load_str(&j.to_string());
            let known = modes.iter().all(|m| BlendMode::ALL.iter().any(|b| b.as_str() == m));
            let loaded = match loaded {
                Ok(loaded) => loaded,
                Err(d) => {
                    // FX-LBLEND-009: a mode this build does not know is refused when the file
                    // opens (B-627), never drawn as another.
                    t.row(&format!("{name} ({}), {depth}", modes.join(", ")), &format!("refused when it opens: {}", d.message), !known);
                    continue;
                }
            };
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let (id, start, n) = (comp.id.clone(), comp.start_frame, comp.duration_frames as i32);
            let frames = if n > 1 { vec![start, start + n / 2] } else { vec![start] };
            let (mut s, mut refused, mut exact) = (Stats::default(), 0, 0);
            for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                for &f in &frames {
                    let (cpu, card, r) = both(&project, &id, f, &root, q, &mut gpu).expect("the frame draws");
                    if r {
                        refused += 1;
                        exact += (cpu == card) as usize;
                    } else {
                        s.add(&cpu, &card);
                    }
                }
            }
            let mut modes = modes.clone();
            modes.sort();
            modes.dedup();
            t.row(
                &format!("{name} ({}), {depth}", modes.join(", ")),
                &format!("opens with {} warnings; on the card {}; refused {refused}, {exact} of them the CPU's exactly", loaded.warnings.len(), s.text()),
                s.ok() && refused == exact && s.frames > 0,
            );
        }
    }

    // 2. The sweep.
    t.heading("Each mode, Preserve Transparency off and on, linear light and 8 bpc: a 384 px layer of mixed pixels at 70% over a 512 px backdrop");
    let backdrop = mixed(512, 512, 0x9E37_79B9_7F4A_7C15);
    let top = mixed(384, 384, 0xD1B5_4A32_D192_ED03);
    for mode in BlendMode::ALL {
        for display in [false, true] {
            for preserve in [false, true] {
                let plan = FramePlan {
                    width: 512,
                    height: 512,
                    float: false,
                    display,
                    layers: vec![
                        layer("under", backdrop.clone(), Affine::IDENTITY, 1.0, BlendMode::Normal, false),
                        layer("top", top.clone(), Affine::translation(100.0, 100.0), 0.7, mode, preserve),
                    ],
                };
                let cpu = render::render(&plan, DEFAULT_TILE_SIZE).to_srgb8_straight();
                let card = gpu.draw(&plan, &CelCache::viewer());
                let what = format!(
                    "{}, {}, Preserve Transparency {}",
                    mode.as_str(),
                    if display { "8 bpc" } else { "linear" },
                    if preserve { "on" } else { "off" }
                );
                match card {
                    Ok(card) => {
                        let mut s = Stats::default();
                        s.add(&cpu, &card);
                        t.row(&what, &format!("on the card: {}", s.text()), s.ok());
                    }
                    Err(d) => t.row(&what, &format!("the card refused: {}", d.message), false),
                }
            }
        }
    }
    for mode in [BlendMode::StencilAlpha, BlendMode::StencilLuma] {
        let plan = FramePlan {
            width: 512,
            height: 512,
            float: false,
            display: false,
            layers: vec![
                layer("under", backdrop.clone(), Affine::IDENTITY, 1.0, BlendMode::Normal, false),
                layer("top", top.clone(), Affine::scaling(0.0, 0.0), 1.0, mode, false),
            ],
        };
        let refused = gpu.draw(&plan, &CelCache::viewer()).is_err();
        let cpu = render::render(&plan, DEFAULT_TILE_SIZE);
        let clear = cpu.data().iter().all(|&v| v == 0.0);
        t.row(
            &format!("{} scaled to nothing", mode.as_str()),
            &format!(
                "{}; the CPU's frame {}",
                if refused { "the card refuses it, so the CPU draws it" } else { "the card drew it" },
                if clear { "is clear" } else { "is not clear" }
            ),
            refused && clear,
        );
    }

    // 3. Every fixture with two layers or more, its upper layers in one of the twelve modes.
    t.heading("Every fixture project with two layers or more, its upper layers set to one of the twelve modes, first frame, Full");
    let new: Vec<BlendMode> = BlendMode::ALL.into_iter().filter(|m| !CORE.contains(&m.as_str())).collect();
    let mut by_mode: BTreeMap<(&str, &str), Stats> = BTreeMap::new();
    let (mut shots, mut refused, mut exact) = (0usize, 0usize, 0usize);
    for file in &all {
        if only.as_ref().is_some_and(|o| !file.to_string_lossy().contains(o.as_str())) {
            continue;
        }
        let text = fs::read_to_string(file).unwrap_or_default();
        let Ok(mut j) = serde_json::from_str::<Value>(&text) else { continue };
        let Some(c) = j["compositions"].get_mut(0) else { continue };
        let Some(bottom) = c["layer_order"].get(0).and_then(Value::as_str).map(str::to_string) else { continue };
        let Some(layers) = c["layers"].as_array_mut() else { continue };
        if layers.len() < 2 {
            continue;
        }
        let mode = new[shots % new.len()];
        for l in layers.iter_mut() {
            if l["id"].as_str() != Some(bottom.as_str()) && l.get("blend_mode").is_some() {
                l["blend_mode"] = Value::from(mode.as_str());
            }
        }
        let root = file.parent().unwrap().to_path_buf();
        let mut counted = false;
        for depth in ["as saved", "8 bpc"] {
            let mut j = j.clone();
            if depth == "8 bpc" {
                eight(&mut j);
            }
            let Ok(loaded) = persist::load_str(&j.to_string()) else { continue };
            let project = loaded.document.project().clone();
            let comp = &project.compositions[0];
            let Some((cpu, card, r)) = both(&project, &comp.id.clone(), comp.start_frame, &root, PreviewQuality::Full, &mut gpu) else {
                continue;
            };
            counted = true;
            if r {
                refused += 1;
                exact += (cpu == card) as usize;
                continue;
            }
            let largest = by_mode.entry((mode.as_str(), depth)).or_default().add(&cpu, &card);
            if largest > 1 {
                eprintln!("{} {} {depth}: largest {largest}", file.display(), mode.as_str());
            }
        }
        shots += counted as usize;
    }
    for ((mode, depth), s) in &by_mode {
        t.row(&format!("{mode}, {depth}"), &format!("on the card: {}", s.text()), s.ok());
    }
    t.row(
        &format!("{shots} fixture projects"),
        &format!("{refused} frames the card refused, {exact} of them the CPU's exactly"),
        refused == exact && shots > 0,
    );
    t.finish("B-748_card_blend_modes_table.md");
}
