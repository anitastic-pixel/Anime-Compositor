//! B-12b item 6: the fixture document 08 line 41 declares, built, and the envelope re-measured
//! against it.
//!
//! Writes `verification/T-06_declared_fixture.md` and `verification/T-06_declared_fixture.json`
//! under `--release --ignored`.
//!
//! # Why this file exists
//!
//! `tests/t06_envelope.rs` measures the reference shot and says at the top of its own artifact
//! that the reference shot is **not** the fixture document 08 asks about. Line 41 declares
//! *"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple effect
//! instances"*; the reference shot has four raster layers, no mattes and no effects. That gap is
//! **D-41**, and D-41 says in its own words what closes it: *"the re-measurement against the real
//! fixture after B-07, not an edit to line 41."* B-06 and B-07 have landed, so the fixture is
//! buildable, so this is that re-measurement.
//!
//! **D-40 is the other entry this touches.** The owner decided on 2026-09-06 to leave the preview
//! cache budget at 128 MB, and gave a reason with a condition attached: the target line 41 asks
//! about is met at that budget *on a four-layer shot*, and *"if B-06 and B-07 make the reference
//! shot heavy enough that the measured p95 crosses 100 ms, that is a new measurement and reopens
//! this entry."* This file is the measurement that decides that. It states the verdict and does
//! not act on it: reopening a decision is a note in the register, and the register is the owner's.
//!
//! # What was built, and the one way it is not the real thing
//!
//! Ten raster layers over **ten distinct image sequences on disk**, two alpha mattes, three effect
//! instances, in the reference shot's own composition — 1920x1080, 24 fps, 240 frames.
//!
//! The ten sequences are **copies of the reference shot's four**, laid down under `target/` by
//! this test. So the fixture has ten layers of drawings and only four layers of *pictures*: the
//! same art appears more than once. That is a real limitation and it is stated here rather than
//! buried, but it is a limitation of what the shot looks like and not of what it costs. Nothing in
//! the render path shares work between two layers that read different files. `src/cache.rs` keys a
//! decoded cel on the file's path, length, modification time and interpretation, so ten sequences
//! are ten decodes; `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an 81.69 ms
//! draft frame, which is the part being multiplied. Drawing the owner six more layers of art would
//! change the pictures in this artifact and would not change a number in it.
//!
//! **The two mattes are alpha mattes by construction, not by a setting.** `model::MatteReference`
//! holds a layer and a `matte_only` flag and nothing else; document 21 composites a matte from the
//! matte layer's alpha. There is no luma matte in this build to choose instead.
//!
//! **`matte_only` is false on both.** True would keep the matte layer out of the visible stack,
//! which is the commoner way to use one and the cheaper one to render. False keeps all ten layers
//! composited, which is the reading of "ten raster layers, two alpha mattes" that costs more, and
//! a floor should cost more rather than less.
//!
//! # What is asserted and what is only reported
//!
//! The same division `tests/t06_envelope.rs` makes, for the same reasons, and the same four
//! assertions — a timing is reported and never asserted, because a test that fails on a busy
//! machine teaches nothing.
//!
//! # What this does not measure, and why not
//!
//! **Full resolution.** T-06 walks the shot again at full resolution because an export writes
//! full-resolution frames. Ten layers at full resolution is roughly four times the work of the
//! draft walk here and document 08 line 41 does not ask for it; `verification/B-10_export.md` is
//! where an export is timed.
//!
//! **A cache with room for the whole shot.** T-06 has that row at 2 GB. This fixture has 166
//! distinct drawings, which is about 5.5 GB held, and this file will not allocate that to make a
//! table row. The seek table below tops out at a budget that holds one frame of this fixture,
//! which is the more useful comparison anyway: it is what the viewer would need to make a scrub
//! stop re-decoding.

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{json, Value as J};

use anime_compositor::cache::{CelCache, DEFAULT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

mod common;
use common::{peak_working_set, repo, working_set};

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
/// One decoded cel held in the working space: RGBA f32 at the composition's extent.
const ONE_CEL: usize = WIDTH * HEIGHT * 4 * std::mem::size_of::<f32>();
const FIRST: i32 = 0;
const LAST: i32 = 239;
/// The number of repeated work-area loops document 08 line 41 asks for.
const LOOPS: usize = 10;
/// Ten cels, which is one frame of this fixture: every layer contributes one. This is the smallest
/// budget at which asking for the frame just shown is a hit rather than ten decodes.
const ONE_FRAME_BYTES: usize = 10 * ONE_CEL;
/// Coprime with the 240 frames of the work area, so stepping by it visits every frame exactly once
/// in an order that is nowhere near sequential, and the same order every run.
const SCATTER: i32 = 97;
/// 24 fps as a per-frame budget in milliseconds.
const FRAME_BUDGET_MS: f64 = 1000.0 / 24.0;

/// Which of the reference shot's four sequences each of the ten layers reads a copy of. One
/// background and nine cel layers, which is the shape of the shot the reference already is.
const SOURCES: [usize; 10] = [1, 2, 3, 4, 2, 3, 4, 2, 3, 4];
/// Layer 5 is matted by layer 4, layer 8 by layer 7. Two alpha mattes, no cycle.
const MATTES: [(usize, usize); 2] = [(5, 4), (8, 7)];

// ---------------------------------------------------------------------------------------
// Building the fixture
// ---------------------------------------------------------------------------------------

/// Each test that builds the fixture gets a root of its own, because cargo runs the tests in one
/// binary in parallel and two of them laying down the same copies would race.
fn workload_root(name: &str) -> PathBuf {
    repo("target/b12b_declared_fixture").join(name)
}

/// Copy one of the reference shot's sequences to a directory of its own, so that the render path
/// treats it as a different sequence. Emptied first: a stale copy is a fixture nobody declared.
fn lay_down_copy(source: &Path, into: &Path) {
    if into.exists() {
        fs::remove_dir_all(into).unwrap_or_else(|e| panic!("empty {}: {e}", into.display()));
    }
    fs::create_dir_all(into).unwrap_or_else(|e| panic!("make {}: {e}", into.display()));
    for entry in fs::read_dir(source).unwrap_or_else(|e| panic!("read {}: {e}", source.display())) {
        let entry = entry.expect("a directory entry of the reference shot");
        let to = into.join(entry.file_name());
        fs::copy(entry.path(), &to).unwrap_or_else(|e| panic!("copy to {}: {e}", to.display()));
    }
}

/// The declared fixture, as a project file this build can open, plus the root its drawings are
/// under.
///
/// Built out of `verification/B-08a_project.json` rather than written from nothing, so that every
/// exposure sheet here is one this repository already renders — including layer 3's missing
/// drawing 7 and layer 4's out-of-order re-exposure, both of which the copies inherit.
fn build_fixture(name: &str) -> (Project, PathBuf, String) {
    let base_text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read verification/B-08a_project.json");
    let base: J = serde_json::from_str(&base_text).expect("B-08a's artifact is JSON");
    let root = workload_root(name);
    fs::create_dir_all(&root).unwrap_or_else(|e| panic!("make {}: {e}", root.display()));

    let mut assets = Vec::new();
    let mut layers = Vec::new();
    let mut order = Vec::new();

    for (index, source) in SOURCES.iter().enumerate() {
        let n = index + 1;
        let from = format!("layer{source}");
        let to = format!("copy{n}");
        lay_down_copy(
            &repo("Fixtures/reference_shot").join(&from),
            &root.join(&to),
        );

        let mut asset = base["assets"][source - 1].clone();
        asset["id"] = json!(format!("asset-{n}"));
        asset["name"] = json!(to);
        asset["pattern"] = json!(asset["pattern"]
            .as_str()
            .expect("every asset of the reference shot is a sequence")
            .replace(&from, &to));
        let frames: BTreeMap<String, J> = asset["frames"]
            .as_object()
            .expect("an image sequence has an explicit frame list")
            .iter()
            .map(|(drawing, file)| {
                let file = file.as_str().expect("a frame maps a drawing to a file");
                (
                    drawing.clone(),
                    json!(file.replacen(&format!("{from}/"), &format!("{to}/"), 1)),
                )
            })
            .collect();
        asset["frames"] = json!(frames);
        assets.push(asset);

        let mut layer = base["compositions"][0]["layers"][source - 1].clone();
        layer["id"] = json!(format!("layer-{n}"));
        layer["name"] = json!(to);
        layer["asset_id"] = json!(format!("asset-{n}"));
        layer["matte"] = J::Null;
        layer["effects"] = json!([]);
        layers.push(layer);
        order.push(format!("layer-{n}"));
    }

    for (on, from) in MATTES {
        layers[on - 1]["matte"] = json!({
            // The only mode the format has: `persist` accepts "alpha" and nothing else.
            "mode": "alpha",
            "layer_id": format!("layer-{from}"),
            "matte_only": false,
        });
    }

    // Three simple effect instances, one of each kind this build has, on three different layers.
    layers[1]["effects"] = json!([{
        "instance_id": "fx-exposure",
        "type_id": "core.exposure",
        "enabled": true,
        "parameters": { "stops": 0.5 },
    }]);
    layers[5]["effects"] = json!([{
        "instance_id": "fx-blur",
        "type_id": "core.gaussian_blur",
        "enabled": true,
        "parameters": { "sigma_px": 4.0 },
    }]);
    layers[8]["effects"] = json!([{
        "instance_id": "fx-tint",
        "type_id": "core.tint",
        "enabled": true,
        "parameters": { "color": [0.9, 0.7, 0.5], "amount": 0.3 },
    }]);

    let mut project = base.clone();
    project["project_id"] = json!("proj-t06-declared-fixture");
    project["assets"] = json!(assets);
    project["compositions"][0]["layer_order"] = json!(order);
    project["compositions"][0]["layers"] = json!(layers);

    let text = serde_json::to_string_pretty(&project).expect("the fixture serialises");

    let loaded = persist::load_str(&text).unwrap_or_else(|d| {
        panic!(
            "the fixture this test wrote, this build cannot open: {} -- {}",
            d.message, d.detail
        )
    });
    (loaded.document.project().clone(), root, text)
}

// ---------------------------------------------------------------------------------------
// Measuring
// ---------------------------------------------------------------------------------------

fn render_ms(
    project: &Project,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
    cache: &mut CelCache,
) -> f64 {
    let comp = Id::new(COMP);
    let mut log = FrameLog::new(3);
    let at = Instant::now();
    preview::preview_frame_cached(
        project,
        &comp,
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message));
    at.elapsed().as_secs_f64() * 1000.0
}

/// One untimed frame, so no measured frame pays for the first touch of freshly allocated pages.
fn warm_up(project: &Project, root: &Path) {
    let _ = render_ms(
        project,
        root,
        FIRST,
        PreviewQuality::Draft,
        &mut CelCache::none(),
    );
}

/// A cel that cannot fit the budget is refused rather than admitted and immediately thrown out.
/// Both behaviours end with an empty cache, so the eviction count is the only thing that separates
/// them. Returns the eviction count for the artifact to report.
fn refuses_what_it_cannot_hold(project: &Project, root: &Path) -> u64 {
    let mut cache = CelCache::with_budget(ONE_CEL - 1);
    let _ = render_ms(project, root, FIRST, PreviewQuality::Draft, &mut cache);
    assert_eq!(
        cache.len(),
        0,
        "a budget below one cel held {} cels",
        cache.len()
    );
    assert_eq!(
        cache.held_bytes(),
        0,
        "a budget below one cel held {} bytes",
        cache.held_bytes()
    );
    assert_eq!(
        cache.evictions(),
        0,
        "a budget below one cel evicted {} cels, so it admitted what it should have refused",
        cache.evictions()
    );
    cache.evictions()
}

fn sorted(mut ms: Vec<f64>) -> Vec<f64> {
    ms.sort_by(|a, b| a.partial_cmp(b).expect("no NaN in a measured duration"));
    ms
}

/// Nearest rank on the sorted sample, as `tests/t06_envelope.rs` does it.
fn percentile(sorted: &[f64], p: f64) -> f64 {
    let rank = (p * sorted.len() as f64).ceil().max(1.0) as usize;
    sorted[rank.min(sorted.len()) - 1]
}

fn median(sorted: &[f64]) -> f64 {
    let n = sorted.len();
    if n % 2 == 0 {
        (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0
    } else {
        sorted[n / 2]
    }
}

fn work_area() -> Vec<i32> {
    (FIRST..=LAST).collect()
}

fn scattered() -> Vec<i32> {
    let length = LAST - FIRST + 1;
    (0..length)
        .map(|i| FIRST + (i * SCATTER) % length)
        .collect()
}

struct Loop {
    ms: Vec<f64>,
    total_ms: f64,
    decodes: u64,
    hits: u64,
    held: usize,
    working_set: usize,
}

fn one_loop(project: &Project, root: &Path, cache: &mut CelCache) -> Loop {
    let decodes_before = cache.misses();
    let hits_before = cache.hits();
    let mut ms = Vec::new();
    let start = Instant::now();
    for frame in work_area() {
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            cache,
        ));
    }
    Loop {
        total_ms: start.elapsed().as_secs_f64() * 1000.0,
        ms,
        decodes: cache.misses() - decodes_before,
        hits: cache.hits() - hits_before,
        held: cache.held_bytes(),
        working_set: working_set(),
    }
}

/// A scattered walk of the whole work area at one budget, warmed by an identical walk first when
/// asked, so that the measured pass is the "cached" seek document 08 means.
fn seek_walk(
    project: &Project,
    root: &Path,
    budget: usize,
    warm_first: bool,
) -> (Vec<f64>, usize, usize) {
    let mut cache = CelCache::with_budget(budget);
    if warm_first {
        for frame in scattered() {
            let _ = render_ms(project, root, frame, PreviewQuality::Draft, &mut cache);
        }
    }
    let mut ms = Vec::new();
    for frame in scattered() {
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            &mut cache,
        ));
    }
    (sorted(ms), cache.held_bytes(), cache.len())
}

/// The same frame asked for twice in a row, everywhere in the shot: the warmest seek there is.
fn reseek(project: &Project, root: &Path, budget: usize) -> Vec<f64> {
    let mut cache = CelCache::with_budget(budget);
    let mut ms = Vec::new();
    for frame in scattered() {
        let _ = render_ms(project, root, frame, PreviewQuality::Draft, &mut cache);
        ms.push(render_ms(
            project,
            root,
            frame,
            PreviewQuality::Draft,
            &mut cache,
        ));
    }
    sorted(ms)
}

// ---------------------------------------------------------------------------------------
// The measurement
// ---------------------------------------------------------------------------------------

/// Document 08 line 41's declared fixture, built and measured.
///
/// ```text
/// cargo test --release --test b12b_declared_fixture -- --ignored --nocapture
/// ```
#[test]
#[ignore = "timing; builds a workload on disk and runs for several minutes under --release"]
fn t06_against_the_fixture_document_08_declares() {
    let (project, root, text) = build_fixture("measured");
    let path = repo("verification/T-06_declared_fixture.json");
    fs::write(&path, &text).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));

    warm_up(&project, &root);
    let evictions_below_one_cel = refuses_what_it_cannot_hold(&project, &root);

    // --- Ten loops over the work area at the viewer's own budget. -----------------------------
    let mut cache = CelCache::with_budget(DEFAULT_BUDGET_BYTES);
    let mut loops = Vec::new();
    for _ in 0..LOOPS {
        loops.push(one_loop(&project, &root, &mut cache));
    }
    for (n, run) in loops.iter().enumerate() {
        assert!(
            run.held <= DEFAULT_BUDGET_BYTES,
            "loop {} held {} bytes, over its budget of {DEFAULT_BUDGET_BYTES}",
            n + 1,
            run.held
        );
    }
    let peak_after_loops = peak_working_set();

    // --- Seeking. -----------------------------------------------------------------------------
    let reseek_default = reseek(&project, &root, DEFAULT_BUDGET_BYTES);
    let (scatter_default, _, _) = seek_walk(&project, &root, DEFAULT_BUDGET_BYTES, false);
    let (scatter_frame, filled_bytes, filled_cels) =
        seek_walk(&project, &root, ONE_FRAME_BYTES, true);
    let reseek_frame = reseek(&project, &root, ONE_FRAME_BYTES);

    // The budget assertion above believes whatever the cache says it holds. This is the same
    // figure derived a second way, from a count of cels.
    assert_eq!(
        filled_bytes,
        filled_cels * ONE_CEL,
        "the cache reports {filled_bytes} bytes held in {filled_cels} cels, which is not \
         {filled_cels} cels of {ONE_CEL} bytes"
    );

    write_artifact(
        &loops,
        &Seeks {
            reseek_default,
            scatter_default,
            scatter_frame,
            reseek_frame,
            filled_bytes,
            filled_cels,
            evictions_below_one_cel,
        },
        peak_after_loops,
        text.len(),
    );

    // --- The one claim document 08 asks for that is a check rather than a number. --------------
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let growth = ended.saturating_sub(baseline);
    assert!(
        growth <= ONE_CEL,
        "the process grew {growth} bytes between the end of loop 2 ({baseline}) and the end of \
         loop {LOOPS} ({ended}), which is more than one cel ({ONE_CEL}); see \
         verification/T-06_declared_fixture.md"
    );
}

struct Seeks {
    reseek_default: Vec<f64>,
    scatter_default: Vec<f64>,
    scatter_frame: Vec<f64>,
    reseek_frame: Vec<f64>,
    filled_bytes: usize,
    filled_cels: usize,
    evictions_below_one_cel: u64,
}

fn mib(bytes: usize) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}

fn write_artifact(loops: &[Loop], seeks: &Seeks, peak: usize, project_bytes: usize) {
    let mut s = String::from("# T-06 again, against the fixture document 08 actually declares\n\n");
    s.push_str(
        "`verification/T-06_performance_envelope.md` measures the reference shot and says at the \
         top of itself that the reference shot is not the fixture document 08 line 41 asks about: \
         *\"1080p, 24 fps, 240 frames, ten raster layers, two alpha mattes and three simple \
         effect instances\"*, against a shot with four layers, no mattes and no effects. That gap \
         is **D-41**, and D-41 names what closes it - the re-measurement against the real fixture \
         once B-06 and B-07 have landed, not an edit to line 41. Both have landed. This is that \
         measurement. Produced by `tests/b12b_declared_fixture.rs`, which is `#[ignore]`d in \
         normal runs.\n\n",
    );

    s.push_str("## Machine, build and configuration\n\n");
    s.push_str(
        "- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads\n\
         - OS: Microsoft Windows 11 Education, 10.0.26200\n\
         - Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`\n\
         - Workload: `verification/T-06_declared_fixture.json`, at draft resolution\n\
         - Tile size: `compose::DEFAULT_TILE_SIZE`\n",
    );
    let _ = writeln!(
        s,
        "- Work area: frames {FIRST} to {LAST}, the whole shot\n\
         - Percentiles are by nearest rank on the sorted sample\n\n\
         Debug assertions in this build: {}. A run with `true` there is a debug build, and its \
         numbers say more about the compiler than about the renderer.\n",
        cfg!(debug_assertions)
    );

    // --- The fixture. --------------------------------------------------------------------------
    let _ = writeln!(
        s,
        "\n## The fixture, and the one way it is not the real thing\n\n\
         | What line 41 declares | What was built |\n|---|---|\n\
         | 1080p | 1920 x 1080 |\n\
         | 24 fps | 24 fps |\n\
         | 240 frames | 240, frames {FIRST} to {LAST} |\n\
         | Ten raster layers | Ten, over ten separate image sequences on disk |\n\
         | Two alpha mattes | Layer 5 matted by layer 4, layer 8 by layer 7 |\n\
         | Three simple effect instances | An exposure on layer 2, a Gaussian blur on layer 6, a \
         tint on layer 9 |\n\n\
         The project file is written out beside this one as \
         `verification/T-06_declared_fixture.json`, {project_bytes} bytes, and it is a project \
         this build opens: the test builds it, saves it, and reads it back through \
         `persist::load_str` before measuring anything.\n\n\
         **The ten sequences are copies of the reference shot's four.** The test lays them down \
         under `target/b12b_declared_fixture/` as `copy1` to `copy10`. So this shot has ten layers \
         of drawings and four layers of pictures - the same art appears more than once, and \
         anybody looking at a frame of it would see that.\n\n\
         That is a limitation of what it looks like and not of what it costs, and the difference \
         matters because this file is about cost. Nothing in the render path shares work between \
         two layers reading different files: `src/cache.rs` keys a decoded cel on the file's path, \
         its length, its modification time and its interpretation, so ten sequences are ten \
         decodes and not four. `verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an \
         81.69 ms draft frame, which is the part being multiplied here. Six more layers of \
         owner-drawn art would change the pictures and would not change a number below.\n\n\
         Two smaller choices, both made towards the heavier reading rather than the cheaper one. \
         The mattes are **alpha** mattes because that is the only kind this build has - \
         `model::MatteReference` holds a layer and a flag, and document 21 composites from the \
         matte layer's alpha. And `matte_only` is **false** on both, so all ten layers still \
         composite; true would keep the two matte layers out of the visible stack and cost less. \
         A floor should cost more rather than less.\n\n\
         Each copied layer keeps the exposure sheet of the reference layer it came from, \
         including layer 3's deliberately missing drawing 7 and layer 4's out-of-order \
         re-exposure. Nothing here is a new timing path; it is the same paths, ten times over.\n",
    );

    // --- Ten loops. ---------------------------------------------------------------------------
    s.push_str(
        "\n## Ten repeated work-area loops\n\n\
         | Loop | Total ms | Median ms | p95 ms | Slowest ms | Frames per second at the median | \
         Decodes | From memory | Cache held (MiB) | Process working set (MiB) |\n\
         |---|---|---|---|---|---|---|---|---|---|\n",
    );
    for (n, run) in loops.iter().enumerate() {
        let ms = sorted(run.ms.clone());
        let m = median(&ms);
        let _ = writeln!(
            s,
            "| {} | {:.1} | {:.2} | {:.2} | {:.2} | {:.1} | {} | {} | {:.1} | {:.1} |",
            n + 1,
            run.total_ms,
            m,
            percentile(&ms, 0.95),
            ms[ms.len() - 1],
            1000.0 / m,
            run.decodes,
            run.hits,
            mib(run.held),
            mib(run.working_set),
        );
    }

    let first = sorted(loops[0].ms.clone());
    let last = sorted(loops[LOOPS - 1].ms.clone());
    let work_area_budget_ms = (LAST - FIRST + 1) as f64 * FRAME_BUDGET_MS;
    let under = loops
        .iter()
        .filter(|run| run.total_ms <= work_area_budget_ms)
        .count();
    let baseline = loops[1].working_set;
    let ended = loops[LOOPS - 1].working_set;
    let dropped = loops[LOOPS - 1]
        .ms
        .iter()
        .filter(|m| **m > FRAME_BUDGET_MS)
        .count();
    let _ = writeln!(
        s,
        "\n**Cold-render throughput** is the first row: {:.1} ms for {} frames, a median of \
         {:.2} ms and {:.1} frames per second. It is the only loop that pays for reading every \
         drawing off the disk for the first time.\n\n\
         **Warm playback at 24 fps.** A 24 fps clock allows this 240-frame work area {:.1} ms, and \
         **{} of the ten loops came in under it**; the tenth loop's median frame cost {:.2} ms \
         against the {:.1} ms one frame is allowed, and **{} of its {} frames** cost more than \
         that. Those are the frames D-32 drops rather than running the shot slow. End to end, on a \
         real window, playback is counted in `verification/B-08_window_shell.md`.\n\n\
         **Memory across the ten loops.** {:.1} MiB at the end of the second loop, {:.1} MiB at \
         the end of the tenth, a difference of {:.1} MiB across 1,920 renders. The test fails if \
         that exceeds one cel, 33,177,600 bytes. Peak working set across the ten loops was \
         {:.1} MiB, read before the seek section below so that the large cache the seek table \
         allocates does not get reported as the program's appetite.\n\n\
         **Peak VRAM is not reported.** There is no GPU path in this build; a VRAM figure would \
         be the desktop's.\n",
        loops[0].total_ms,
        loops[0].ms.len(),
        median(&first),
        1000.0 / median(&first),
        work_area_budget_ms,
        under,
        median(&last),
        FRAME_BUDGET_MS,
        dropped,
        loops[LOOPS - 1].ms.len(),
        mib(baseline),
        mib(ended),
        mib(ended.saturating_sub(baseline)),
        mib(peak),
    );

    // --- Seeks. -------------------------------------------------------------------------------
    s.push_str(
        "\n## Seeking\n\n\
         A seek is a jump to a frame that is not the next one. The walk steps through the work \
         area 97 frames at a time; 97 is coprime with 240, so it reaches every frame exactly once \
         in an order nowhere near sequential, and it is the same order every run.\n\n\
         | Seek | Budget | Median ms | p95 ms | Slowest ms | Within 100 ms at p95 |\n\
         |---|---|---|---|---|---|\n",
    );
    let rows: [(&str, &str, &Vec<f64>); 4] = [
        (
            "The frame just shown, again",
            "128 MB (the viewer's default)",
            &seeks.reseek_default,
        ),
        (
            "A scattered walk of the whole shot",
            "128 MB (the viewer's default)",
            &seeks.scatter_default,
        ),
        (
            "The frame just shown, again",
            "332 MB (room for one frame)",
            &seeks.reseek_frame,
        ),
        (
            "The same walk, second pass",
            "332 MB (room for one frame)",
            &seeks.scatter_frame,
        ),
    ];
    for (label, budget, ms) in rows {
        let p95 = percentile(ms, 0.95);
        let _ = writeln!(
            s,
            "| {label} | {budget} | {:.2} | {p95:.2} | {:.2} | {} |",
            median(ms),
            ms[ms.len() - 1],
            if p95 <= 100.0 { "yes" } else { "**no**" },
        );
    }

    let p95_default = percentile(&seeks.scatter_default, 0.95);
    let _ = writeln!(
        s,
        "\n### How to read the seek table\n\n\
         **At ten layers the viewer's default budget cannot hold a single frame, and that is \
         arithmetic rather than a defect.** One cel of this composition costs 33,177,600 bytes to \
         hold and every frame of this fixture needs ten of them, which is 316.4 MiB. The default \
         budget is 128 MB. So the first row - asking for the frame that was just shown - is not a \
         warm seek at this fixture at all: showing the frame evicts the cels that made it. The \
         third row is the same request at a budget with room for one frame, and the difference \
         between the two rows is the whole of what the budget decides.\n\n\
         The fourth row is the scattered walk with that same one-frame budget, walked twice, the \
         second pass measured. It still re-decodes on every jump - one frame of headroom cannot \
         hold a neighbourhood - so it is a lower bound on the cost of scrubbing this shot and not \
         a picture of a warm cache. Holding every distinct drawing of this fixture is 166 cels, \
         about 5.5 GB, and this file will not allocate that to fill in a table row. The cache in \
         the measured row held {:.1} MiB in {} cels.\n\n\
         **Every figure here is seek-to-buffer, not seek-to-display.** It stops at a finished \
         picture in memory; the transport into the window is the window's, and \
         `verification/B-08_window_shell.md` is where a real one is watched.\n",
        mib(seeks.filled_bytes),
        seeks.filled_cels,
    );

    // --- The verdict. -------------------------------------------------------------------------
    let met = p95_default <= 100.0;
    let _ = writeln!(
        s,
        "\n## What this settles, and what it hands back to the owner\n\n\
         **D-41 - \"the performance fixture document 08 declares cannot be built in this build\".** \
         It can now, and it has been. The entry says what closes it: the re-measurement against \
         the real fixture after B-07. This file is it, and D-41 can be closed. What stays true and \
         should be written into the closing note is the sentence above about the art: ten \
         sequences of drawings, four sequences of pictures. Every figure in this file is a real \
         ten-layer cost and no figure in it is a picture of a ten-layer shot.\n\n\
         **D-40 - the 128 MB preview cache budget.** The owner decided on 2026-09-06 to leave it \
         where it is, and attached a condition: *\"if B-06 and B-07 make the reference shot heavy \
         enough that the measured p95 crosses 100 ms, that is a new measurement and reopens this \
         entry rather than contradicting it.\"* Here is that measurement, on the declared fixture, \
         at the default budget, scrubbing:\n\n\
         | | |\n|---|---|\n\
         | p95 of a scattered seek, default budget | **{:.2} ms** |\n\
         | Document 08 line 41's target | 100 ms |\n\
         | Crossed | **{}** |\n\n\
         {}\n\n\
         **And a third thing, which is not in the register at all.** D-40 is about the cache \
         budget and D-41 was about the fixture not existing. Neither of them is the biggest \
         number on this page. Document 08 line 41 also asks for *warm-cache playback at 24 fps*, \
         and on the fixture it declares this build renders **{:.2} ms a frame at draft against \
         the {:.1} ms a 24 fps clock allows** - {} of the ten loops came in under the deadline, \
         and {} of the tenth loop's {} frames were over it. That is a factor of about {:.0}, and \
         a factor is not a margin.\n\n\
         Read next to `verification/T-06_performance_envelope.md`, which has the four-layer \
         reference shot sitting *on* the 24 fps deadline and flipping either side of it between \
         runs, this says something specific and worth saying plainly: **nothing here is a \
         regression, and the shot document 08 declares is simply more work than this build does \
         in real time.** Two and a half times the layers costs about nine times the frame, which \
         is more than the layer count alone accounts for and is not explained by anything \
         measured here. `verification/D-37_decode_cost.md` is where the obvious suspect is - \
         decoding was 75.15 ms of an 81.69 ms four-cel draft frame - and the loop table above is \
         consistent with it: every loop decodes {} cels and only {} come from memory, because ten \
         cels of frame do not fit a 128 MB cache and never will.\n\n\
         What that costs the person using it is already decided and needs no new decision: D-32 \
         says the viewer holds real time and drops the frames it cannot make, so this shot plays \
         at the right speed and shows fewer frames rather than playing slowly. What is not \
         decided is whether a 24 fps target for a ten-layer shot is one this project keeps, \
         lowers, or reaches by doing the decoding differently. That is a register entry somebody \
         has to open, and opening it is the owner's.\n\n\
         This file does not act on any of it. Reopening or opening a decision is a note in \
         `Markdown/14_Decisions_Risks.md`, and the register is the owner's.\n",
        p95_default,
        if met { "no" } else { "yes" },
        if met {
            "So the condition D-40 attached has not fired. The target is still met at the default \
             budget, now on the fixture document 08 declares rather than on a four-layer floor, \
             and the decision stands as taken."
        } else {
            "So the condition D-40 attached **has fired**, and D-40 reopens. What reopens it is a \
             measurement and not a disagreement: the owner's reason was that the target was met at \
             this budget, and on the declared fixture it is not. The three answers the entry lists \
             are unchanged - leave the default and let scrubbing pay for itself, raise it to hold a \
             working neighbourhood, or make it a setting with a stated cost - and the third row of \
             the seek table above is what raising it buys."
        },
        median(&last),
        FRAME_BUDGET_MS,
        under,
        dropped,
        loops[LOOPS - 1].ms.len(),
        median(&last) / FRAME_BUDGET_MS,
        loops[LOOPS - 1].decodes,
        loops[LOOPS - 1].hits,
    );

    let _ = writeln!(
        s,
        "\n## What document 08 line 41 asked for, and what came back\n\n\
         | Asked for | Answer |\n|---|---|\n\
         | The ten-layer, two-matte, three-effect fixture | **Built**, and measured here. D-41 |\n\
         | Warm-cache playback at 24 fps | {} of the ten loops came in under the {:.1} ms a 24 fps \
         clock allows 240 frames |\n\
         | p95 cached seek-to-display at or below 100 ms | **{}** at the viewer's default budget, \
         as seek-to-buffer: {:.2} ms. D-40 |\n\
         | No unbounded memory growth after ten repeated work-area loops | Measured against the \
         operating system's working set, and asserted rather than reported |\n\
         | Dropped frames | {} of the tenth loop's {} frames cost more than one frame's budget; \
         counted end to end by photograph in `B-08_window_shell.md` |\n\
         | Cold-render throughput | The first loop above |\n\
         | Peak RAM | {:.1} MiB peak working set |\n\
         | Peak VRAM | **Not measurable**: there is no GPU path in this build |\n\n\
         ## Two checks on the cache that every number above rests on\n\n\
         Every figure in this file is read out of the cache, so a fault in the cache's own \
         accounting would move all of them at once without failing anything. Both of these are \
         asserted, and the run that wrote this file passed them.\n\n\
         - **The held bytes are the cel count times the cel size.** The cache held {:.1} MiB in \
         {} cels, checked against {} cels of {} bytes - the same figure derived a second way.\n\
         - **A budget below one cel refuses rather than churns.** Given {} bytes, one less than a \
         cel, the cache held nothing and evicted {}. A cache that admits what it cannot hold and \
         throws it straight back out also ends up empty; the eviction count is the only thing that \
         tells the two apart.\n",
        under,
        work_area_budget_ms,
        if met { "Met" } else { "Not met" },
        p95_default,
        dropped,
        loops[LOOPS - 1].ms.len(),
        mib(peak),
        mib(seeks.filled_bytes),
        seeks.filled_cels,
        seeks.filled_cels,
        ONE_CEL,
        ONE_CEL - 1,
        seeks.evictions_below_one_cel,
    );

    let path = repo("verification/T-06_declared_fixture.md");
    fs::write(&path, s).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}

/// The fixture is what document 08 line 41 declares, and this build renders a frame of it.
///
/// Cheap, and not `#[ignore]`d, because it is the only thing standing between the artifact above
/// and a fixture that quietly stopped being ten layers. It checks the project this test *wrote*
/// rather than the values that went into writing it, and it renders one frame so that a fixture
/// that can be assembled but not drawn fails here rather than twenty minutes into a timing run.
#[test]
fn the_fixture_is_ten_layers_two_mattes_and_three_effects_and_renders() {
    let (project, root, _) = build_fixture("shape");
    // The fixture is what it claims to be, checked against the file that was written rather than
    // against the values that went into it.
    let comp = project
        .composition(&Id::new(COMP))
        .expect("the fixture has the reference shot's composition");
    assert_eq!(comp.len(), 10, "the declared fixture has ten raster layers");
    assert_eq!(comp.width, 1920);
    assert_eq!(comp.height, 1080);
    assert_eq!(comp.duration_frames, 240);
    let mattes = comp.layers_in_order().filter(|l| l.matte.is_some()).count();
    assert_eq!(mattes, 2, "the declared fixture has two alpha mattes");
    let effects: usize = comp.layers_in_order().map(|l| l.effects.len()).sum();
    assert_eq!(
        effects, 3,
        "the declared fixture has three effect instances"
    );
    let distinct = project.assets.len();
    assert_eq!(distinct, 10, "ten layers over ten sequences on disk");

    let mut cache = CelCache::none();
    let ms = render_ms(&project, &root, FIRST, PreviewQuality::Draft, &mut cache);
    assert!(
        ms.is_finite() && ms > 0.0,
        "frame {FIRST} of the declared fixture did not render"
    );
}
