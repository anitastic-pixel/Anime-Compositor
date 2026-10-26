//! What the tables of the 2026-09-25 batch of effects share (B-35 to B-41): each renders its
//! fixtures against the reference tool's numbers, opens and saves its files, refuses and takes
//! commands, and cuts a frame into tiles, and writes the answers as a table for the owner.
//!
//! Every item is `#[allow(dead_code)]` for the reason `tests/common/mod.rs` gives.
#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value as J;

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, plan_frame, render_frame};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::WorkingBuffer;
use anime_compositor::{persist, render};

pub const MAIN: &str = "comp-main";

pub fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// A table of checks, each with the build's answer and whether it matches.
pub struct Table {
    pub root: PathBuf,
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    /// `fixtures` is the folder under `Fixtures/`; `intro` the Markdown above the first table.
    pub fn new(fixtures: &str, intro: &str) -> Table {
        Table {
            root: repo(&format!("Fixtures/{fixtures}")),
            out: intro.to_string(),
            checks: 0,
            passed: 0,
        }
    }

    pub fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    pub fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }

    /// Write the table to `verification/{file}` and fail the test on any row that does not match.
    pub fn finish(mut self, file: &str) {
        self.out.push_str(&format!(
            "\n## Result\n\n{} of {} checks pass.\n",
            self.passed, self.checks
        ));
        fs::write(repo(&format!("verification/{file}")), &self.out).unwrap();
        assert_eq!(self.passed, self.checks, "see verification/{file}");
    }

    pub fn load(&self, file: &str) -> persist::Loaded {
        persist::load(&self.root.join(file))
            .unwrap_or_else(|d| panic!("{file} opens: {}", d.message))
    }

    pub fn render(&self, document: &Document, frame: i32, tile: usize) -> WorkingBuffer {
        let mut log = FrameLog::new(8);
        render_frame(
            document.project(),
            &Id::new(MAIN),
            frame,
            &self.root,
            tile,
            &mut log,
        )
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
    }

    /// Every case of `expected`: each frame against the reference's numbers, and what opening
    /// the file and planning frame 4 warn of.
    pub fn fixtures(&mut self, expected: &str) {
        self.fixtures_numbered(expected, 1..=999);
    }

    /// The cases of `expected` whose number is in `numbers`, as [`fixtures`](Self::fixtures).
    pub fn fixtures_numbered(&mut self, expected: &str, numbers: std::ops::RangeInclusive<u32>) {
        self.fixtures_warned_at(expected, numbers, 4);
    }

    /// As [`fixtures_numbered`](Self::fixtures_numbered), what each frame warns of read at
    /// `warn_frame`, for a fixture whose composition ends before frame 4 (D-458's has 4 frames).
    pub fn fixtures_warned_at(
        &mut self,
        expected: &str,
        numbers: std::ops::RangeInclusive<u32>,
        warn_frame: i32,
    ) {
        let expected: J =
            serde_json::from_str(&fs::read_to_string(self.root.join(expected)).unwrap()).unwrap();
        let tolerance = expected["tolerance"].as_f64().unwrap();
        for (name, case) in expected["cases"].as_object().unwrap() {
            let number = name.rsplit('-').next().and_then(|n| n.parse().ok());
            if !number.is_some_and(|n| numbers.contains(&n)) {
                continue;
            }
            let says = case["says"].as_str().unwrap();
            let loaded = self.load(case["project"].as_str().unwrap());
            for (frame, pixels) in case["frames"].as_object().unwrap() {
                let frame: i32 = frame.parse().unwrap();
                let d = largest_difference(&self.render(&loaded.document, frame, 64), pixels);
                self.row(
                    &format!("{name} frame {frame}: {says}"),
                    &format!("largest difference {d:.1e}"),
                    d <= tolerance,
                );
            }
            let on_open: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
            let mut log = FrameLog::new(8);
            let _ = plan_frame(
                loaded.document.project(),
                &Id::new(MAIN),
                warn_frame,
                &self.root,
                &mut log,
            );
            let at_frame: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
            let want: Vec<&str> = case
                .get("warning")
                .and_then(J::as_str)
                .into_iter()
                .collect();
            // D-348: a warning said on each frame only, not on opening.
            let mut want_at_frame = want.clone();
            want_at_frame.extend(case.get("frame_warning").and_then(J::as_str));
            self.row(
                &format!("{name}: what opening it warns of, and what frame {warn_frame} warns of"),
                &format!("{on_open:?} and {at_frame:?}"),
                on_open == want && at_frame == want_at_frame,
            );
        }
    }

    /// Each file opened and saved holds what it held.
    pub fn round_trips(&mut self, files: &[&str]) {
        for file in files {
            let original: J =
                serde_json::from_str(&fs::read_to_string(self.root.join(file)).unwrap()).unwrap();
            let same = same_json(&saved(&self.load(file)), &original);
            self.row(
                &format!(
                    "{file} opened and saved holds what it held, out-of-range values, wrong \
                     words and keys included"
                ),
                if same { "the same" } else { "differs" },
                same,
            );
        }
    }

    /// The parameters `file` is saved with.
    pub fn saved_parameters(&self, file: &str) -> J {
        let saved = saved(&self.load(file));
        let layers = saved["compositions"][0]["layers"].as_array().unwrap();
        let holder = layers
            .iter()
            .position(|l| l["effects"].as_array().is_some_and(|e| !e.is_empty()))
            .unwrap_or(0);
        layers[holder]["effects"][0]["parameters"].clone()
    }

    /// `file` with its effect's parameters replaced by `parameters` is refused on opening as a
    /// fault in its shape.
    pub fn shape_refused(&mut self, file: &str, what: &str, parameters: &str) {
        let mut json: J =
            serde_json::from_str(&fs::read_to_string(self.root.join(file)).unwrap()).unwrap();
        // The `art` layer, which is not the bottom one when a case has a layer beneath it.
        let layers = json["compositions"][0]["layers"].as_array().unwrap();
        let art = layers
            .iter()
            .position(|l| l["id"] == "art")
            .or_else(|| {
                layers
                    .iter()
                    .position(|l| l["effects"].as_array().is_some_and(|e| !e.is_empty()))
            })
            .unwrap_or(0);
        json["compositions"][0]["layers"][art]["effects"][0]["parameters"] =
            serde_json::from_str(parameters).unwrap();
        let path = std::env::temp_dir().join(format!("effect_table_shape_{}.json", self.checks));
        fs::write(&path, json.to_string()).unwrap();
        let refused = persist::load(&path).err();
        self.row(
            &format!("a file with {what} is refused as a fault in its shape"),
            &refused
                .as_ref()
                .map_or("opened".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }

    /// A fixture case the build does not follow, pending `decision`: the file is refused on
    /// opening, as every effect's number written as a word is, where the case expects it kept.
    pub fn in_dispute(&mut self, file: &str, case: &str, decision: &str) {
        let refused = persist::load(&self.root.join(file)).err();
        self.row(
            &format!(
                "{case}, in dispute ({decision}): the build refuses {file} as a fault in its \
                 shape, as it does a number written as a word in every effect; the case expects \
                 it kept with a warning"
            ),
            &refused
                .as_ref()
                .map_or("opened".to_string(), |d| d.message.clone()),
            refused.is_some(),
        );
    }

    /// A fixture case a later decision superseded: its value is inside the range now, so the
    /// file opens with no warning, where the case expected one. The case is kept as written.
    pub fn superseded(&mut self, file: &str, case: &str, decision: &str) {
        let warnings: Vec<String> = self
            .load(file)
            .warnings
            .iter()
            .map(|d| d.id.as_str().to_string())
            .collect();
        self.row(
            &format!("{case}, superseded by {decision}: {file}'s value is in range now and opens with no warning"),
            &format!("{warnings:?}"),
            warnings.is_empty(),
        );
    }

    /// Each command is refused with a sentence and leaves the effect as it was.
    pub fn refused(&mut self, document: &mut Document, commands: Vec<(&str, Command)>) {
        let held = settings(document);
        for (what, command) in commands {
            let refused = document.apply(command).err();
            let untouched = settings(document) == held;
            self.row(
                &format!("{what} is refused with a sentence, and nothing changes"),
                &refused
                    .as_ref()
                    .map_or("taken".to_string(), |d| d.message.clone()),
                refused.is_some() && untouched,
            );
        }
    }

    /// Each command is taken; then as many undos give back `file`'s frame 0 exactly.
    pub fn taken(&mut self, document: &mut Document, file: &str, commands: Vec<(&str, Command)>) {
        let n = commands.len();
        for (what, command) in commands {
            let taken = document.apply(command).is_ok();
            self.row(
                &format!("{what} is taken"),
                if taken { "taken" } else { "refused" },
                taken,
            );
        }
        let before = self.render(&self.load(file).document, 0, 64);
        for _ in 0..n {
            document.undo();
        }
        let same = self.render(document, 0, 64).data() == before.data();
        self.row(
            &format!("undo {n} times: frame 0 is the frame it was"),
            if same { "byte-identical" } else { "differ" },
            same,
        );
    }

    /// The frame is the same cut into tiles of 1 pixel and of 64.
    pub fn tiles(&mut self, cases: &[(&str, i32)]) {
        for &(file, frame) in cases {
            let d = self.load(file).document;
            let same = self.render(&d, frame, 64).data() == self.render(&d, frame, 1).data();
            self.row(
                &format!("{file} frame {frame} in tiles of 1 and of 64"),
                if same { "byte-identical" } else { "differ" },
                same,
            );
        }
    }
}

pub const TOWN: (usize, usize) = (480, 270);

/// A street at a quarter of 1920 by 1080: a sky lightening downward, a row of houses with lit
/// and dark windows, and a road with white markings.
pub fn town() -> Vec<u8> {
    let (w, h) = TOWN;
    let walls = [
        [180, 90, 70],
        [90, 140, 170],
        [200, 180, 120],
        [120, 160, 100],
    ];
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let t = y as f64 / h as f64;
            let mut c = [
                (120.0 + 100.0 * t) as u8,
                (170.0 + 50.0 * t) as u8,
                (230.0 - 30.0 * t) as u8,
            ];
            let (house, lx) = (x / 48, x % 48);
            let top = 110 + (house * 37 % 5) * 16;
            if (top..210).contains(&y) && lx < 45 {
                c = walls[house % 4];
                let ly = y - top;
                if (6..40).contains(&lx)
                    && (lx - 6) % 12 < 6
                    && ly >= 10
                    && (ly - 10) % 16 < 8
                    && y < 200
                {
                    c = if (x / 12 + y / 16) % 3 != 0 {
                        [255, 240, 170]
                    } else {
                        [40, 50, 70]
                    };
                }
            }
            if y >= 210 {
                c = if (230..234).contains(&y) && x % 40 < 22 {
                    [250, 250, 250]
                } else {
                    [60, 60, 66]
                };
            }
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The largest difference between a frame and the reference's pixels.
pub fn largest_difference(buffer: &WorkingBuffer, expected: &J) -> f64 {
    let expected: Vec<f64> = expected
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|p| p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()))
        .collect();
    assert_eq!(expected.len(), buffer.data().len(), "the extents agree");
    buffer
        .data()
        .iter()
        .zip(&expected)
        .map(|(a, b)| (*a as f64 - b).abs())
        .fold(0.0, f64::max)
}

/// Two files hold the same thing. A number is compared as a number, because the reference
/// writes `0` where the build writes `0.0`.
pub fn same_json(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => x.as_f64() == y.as_f64(),
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(u, v)| same_json(u, v))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, u)| y.get(k).is_some_and(|v| same_json(u, v)))
        }
        _ => a == b,
    }
}

pub fn saved(loaded: &persist::Loaded) -> J {
    serde_json::from_str(&persist::to_json(
        loaded.document.project(),
        &loaded.preserved,
    ))
    .unwrap()
}

/// The `art` layer's first effect and every key it holds, to see that nothing changed.
pub fn settings(document: &Document) -> String {
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    format!("{:?}", comp.layer(&Id::new("art")).unwrap().effects[0])
}

/// Set the `art` layer's first effect.
pub fn set(effect: Effect) -> Command {
    Command::SetEffectParameters {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        effect,
    }
}

/// Key a setting of the `art` layer's first effect, linearly; each value is one number or a
/// point's two.
pub fn keys(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("art"),
        instance_id: Id::new("fx-0-0"),
        setting: setting.to_string(),
        keys: values
            .iter()
            .map(|&(frame, v)| EffectKey {
                frame,
                value: v.to_vec(),
                interp: Interp::Linear,
            })
            .collect(),
    }
}

/// Nine distinct colours, one more than a chosen-colour list takes.
pub const NINE: &[&str] = &[
    "#000001", "#000002", "#000003", "#000004", "#000005", "#000006", "#000007", "#000008",
    "#000009",
];

pub fn strings(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// D-617: the largest difference, of 255, between two straight 8-bit pictures over the pixels
/// either covers, and the pixels differing.
pub fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = seen(p, q);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

/// D-894: the difference, of 255, a viewer can see between two straight 8-bit pixels of the
/// card's and the CPU's pictures. None where both alpha bytes are 0 (alpha below 1/255 on both
/// sides): such a pixel shows only what is behind it, over any background, whatever colour it
/// keeps. Else the largest of the four channels. Only for the card-against-CPU checks; fixture,
/// export and byte-for-byte checks still compare every channel.
pub fn seen(p: &[u8], q: &[u8]) -> u8 {
    if p[3] == 0 && q[3] == 0 {
        return 0;
    }
    p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0)
}

/// D-894: whether `seen` excused the pair: both alpha bytes 0, the hidden colours different.
pub fn hidden(p: &[u8], q: &[u8]) -> bool {
    p[3] == 0 && q[3] == 0 && p[..3] != q[..3]
}

/// D-894's proof, pixel by pixel: the largest difference, of 255, between the two pixels
/// composited over black and over white, rounded to 8 bits.
pub fn shown(p: &[u8], q: &[u8]) -> u8 {
    let over = |x: &[u8], c: usize, back: u32| (x[c] as u32 * x[3] as u32 + back * (255 - x[3] as u32) + 127) / 255;
    (0..3)
        .flat_map(|c| [0, 255].map(|b| over(p, c, b).abs_diff(over(q, c, b)) as u8))
        .max()
        .unwrap_or(0)
}

/// The warnings a frame gave, by id, each once.
pub fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// D-617: the effects `mine` picks that the card's plan leaves to the card, over every layer.
pub fn on_card(
    project: &Project,
    comp: &Id,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
    mine: fn(&Effect) -> bool,
) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(
        project,
        comp,
        frame,
        root,
        quality,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if mine(&f.instance.effect)))
        .count()
}

/// D-617: the processor and the card, each drawing the frame the page receives: the largest
/// difference and the pixels differing, whether the card refused the frame, and each one's warnings.
pub fn both(
    gpu: &mut Gpu,
    project: &Project,
    comp: &Id,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(
        project,
        comp,
        frame,
        root,
        quality,
        compose::DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(
        project,
        comp,
        frame,
        root,
        quality,
        compose::DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
        gpu,
    )
    .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (
        distance(&c.to_srgb8_straight(), &g),
        refused,
        said_cpu,
        said_gpu,
    )
}

impl Table {
    /// D-617: every fixture file of `expected` on the card against the processor, at each frame
    /// its case draws, at Full and Draft. The files numbered in `none` (their effect left out)
    /// have nothing on the card; every other has its effect, the one `mine` picks, there.
    pub fn card_fixtures(
        &mut self,
        gpu: &mut Gpu,
        expected: &str,
        none: &[u32],
        mine: fn(&Effect) -> bool,
    ) {
        let comp = Id::new(MAIN);
        let expected: J =
            serde_json::from_str(&fs::read_to_string(self.root.join(expected)).unwrap()).unwrap();
        for (name, case) in expected["cases"].as_object().unwrap() {
            let n: u32 = name.rsplit('-').next().unwrap().parse().unwrap();
            let file = case["project"].as_str().unwrap();
            let frames: Vec<i32> = case["frames"]
                .as_object()
                .unwrap()
                .keys()
                .map(|f| f.parse().unwrap())
                .collect();
            let project = persist::load(&self.root.join(file))
                .unwrap()
                .document
                .project()
                .clone();
            let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
            for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                for &frame in &frames {
                    let (d, r, a, b) = both(gpu, &project, &comp, &self.root, frame, quality);
                    largest = largest.max(d.0);
                    refused |= r;
                    agree &= a == b;
                    card += on_card(&project, &comp, &self.root, frame, quality, mine);
                }
            }
            self.row(
                &format!("{name}, {file}, frames {frames:?} at Full and Draft"),
                &format!("largest difference {largest} of 255; on the card in {card} of {} frames; refused by the card: {refused}; the same warnings: {agree}", 2 * frames.len()),
                largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
            );
        }
    }

    /// D-617: each of `shots`, the reference shot with the effect (`mine`) on its `want` layers,
    /// frames 0, 100 and 239 at Full and Draft, on the card against the processor; first, that its
    /// processor frame 100 differs from `plain`, so the comparisons test the effect.
    pub fn card_reference(
        &mut self,
        gpu: &mut Gpu,
        plain: &Project,
        shots: &[(&str, Project, usize)],
        mine: fn(&Effect) -> bool,
    ) {
        let comp = Id::new("comp-reference-shot");
        let root = repo("Fixtures/reference_shot");
        let cpu = |project: &Project| {
            let mut log = FrameLog::new(3);
            preview::preview_frame_cached(
                project,
                &comp,
                100,
                &root,
                PreviewQuality::Full,
                compose::DEFAULT_TILE_SIZE,
                &mut log,
                &mut CelCache::viewer(),
            )
            .expect("the reference shot draws")
            .to_srgb8_straight()
        };
        let without = cpu(plain);
        for (what, project, want) in shots {
            let changed = distance(&cpu(project), &without).1;
            self.row(&format!("the reference shot, {what}: the processor's frame 100 differs from the shot without it"), &format!("{changed} pixels changed"), changed > 0);
            for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
                for frame in [0, 100, 239] {
                    let (d, r, a, b) = both(gpu, project, &comp, &root, frame, quality);
                    let card = on_card(project, &comp, &root, frame, quality, mine);
                    self.row(
                        &format!("the reference shot, {what}, frame {frame}, {}", quality.label()),
                        &format!("largest difference {} of 255, {} pixels differ; {card} of {want} on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                        d.0 <= 1 && !r && a == b && card == *want,
                    );
                }
            }
        }
    }
}

/// The middle of `v` once sorted: the timing a table reports.
pub fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The loop of an effect's timing test: every eighth frame of 0 to 239 asked for as the viewer
/// asks, whole, `passes` times through one cache, after the card forgets what it held; drawn by
/// the processor when `cpu`, by the card otherwise. Each frame's milliseconds, the first pass's
/// apart from the passes after it.
pub fn time_passes(
    gpu: &mut Gpu,
    cpu: bool,
    passes: usize,
    project: &Project,
    comp: &Id,
    root: &Path,
    quality: PreviewQuality,
) -> (Vec<f64>, Vec<f64>) {
    let mut cache = CelCache::viewer();
    gpu.forget();
    let (mut first, mut times) = (Vec::new(), Vec::new());
    for pass in 0..passes {
        for frame in (0..240).step_by(8) {
            let mut log = FrameLog::new(3);
            let t = std::time::Instant::now();
            if cpu {
                drop(
                    preview::preview_frame_cached(
                        project,
                        comp,
                        frame,
                        root,
                        quality,
                        compose::DEFAULT_TILE_SIZE,
                        &mut log,
                        &mut cache,
                    )
                    .expect("CPU frame"),
                );
            } else {
                drop(
                    preview::preview_frame_srgb8(
                        project,
                        comp,
                        frame,
                        root,
                        quality,
                        compose::DEFAULT_TILE_SIZE,
                        &mut log,
                        &mut cache,
                        gpu,
                    )
                    .expect("GPU frame"),
                );
            }
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            if pass > 0 {
                times.push(ms)
            } else {
                first.push(ms)
            }
        }
    }
    (first, times)
}

/// The reference shot, its three layers given the effects `stack` makes for each.
pub fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

/// The first effect on layer `art` of fixture `file`, with `f` applied.
pub fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .effect
        .clone();
    f(&mut e);
    e
}
