//! B-160: faster export. D-231 renders several frames at once and must not change one byte of
//! any exported file. (D-230, the graphics card's video encoder, was rejected by the owner on
//! 2026-09-29 and removed in B-160b.)
//!
//! # How "not one byte changed" is shown
//!
//! Every export below is hashed (SHA-256, `src/sha256.rs`), file by file, together with the
//! report the export returned. The old build is this file cut at the "D-231's own
//! choices" line, compiled against the commit before B-160 and run with `B160_LABEL=before` and
//! `B160_REPO` set to this checkout: it writes `target/b160/hashes_before.txt`. This build writes
//! `hashes_after.txt`, and `b160_table` compares the two line by line. The exports are:
//!
//! - the reference shot (`Fixtures/reference_shot`, 1920x1080, all four layers at their own
//!   cadence, as B-10 builds it): all 240 frames as a PNG sequence and as an MP4, and frames 0 to
//!   47 as 16-bit premultiplied PNG, EXR half, EXR float, a GIF and an animated PNG;
//! - every fixture project in `Fixtures/` that opens: each of its compositions, every frame, as
//!   an 8-bit PNG sequence and as EXR half.
//!
//! The expected value is the old build's own output, which is the point: D-231 is a change of
//! speed only, so its expected value is "what the one-at-a-time build wrote".
//!
//! An MP4 is compared with its dates set aside ([`mp4_dates_aside`]): Windows stamps the moment
//! into the file, so no two runs of any build write the same MP4, and the table shows that too.
//!
//! These are slow, so they are `#[ignore]`d and run by name, as B-10's shot is. The order is
//! `b160_every_export_hashed` and `b160_timing` for the old build and this one, then `b160_table`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use anime_compositor::command::{Command, Document, Target};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::exr_io::ExrSamples;
use anime_compositor::export::{export_sequence, ExportChoices, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, Composition, Id, Layer, Project, Prop, Value};
use anime_compositor::persist;
use anime_compositor::sha256;
use anime_compositor::time::{ExposureMap, FrameRate};
use anime_compositor::{OutputAlpha, OutputDepth};

/// `B160_REPO` lets the old build, compiled in a checkout of its own, read and write here, so
/// the paths its reports name are the same as the new build's.
fn repo(rel: &str) -> PathBuf {
    std::env::var("B160_REPO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .join(rel)
}

/// An MP4 with the moment it was made set to nought. Windows' writer stamps the time into three
/// boxes of the index (`mvhd`, `tkhd`, `mdhd`), so the same build exporting the same shot twice
/// writes two different files; everything else in them is the encoder's.
pub fn mp4_dates_aside(mut b: Vec<u8>) -> Vec<u8> {
    fn walk(b: &mut [u8], mut at: usize, end: usize) {
        while at + 8 <= end {
            let size = u32::from_be_bytes(b[at..at + 4].try_into().unwrap()) as usize;
            if size < 8 || at + size > end {
                return;
            }
            match &b[at + 4..at + 8] {
                b"moov" | b"trak" | b"mdia" => walk(b, at + 8, at + size),
                b"mvhd" | b"tkhd" | b"mdhd" => {
                    // version 0: two 4-byte times; version 1: two 8-byte times.
                    let n = if b[at + 8] == 1 { 16 } else { 8 };
                    b[at + 12..at + 12 + n].fill(0);
                }
                _ => {}
            }
            at += size;
        }
    }
    let end = b.len();
    walk(&mut b, 0, end);
    b
}

fn out_dir(rel: &str) -> PathBuf {
    let dir = repo("target/b160").join(rel);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

// ---- the reference shot, as B-10 builds it ------------------------------------------------

const SHOT: &str = "comp-full-shot";

fn shot_root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

fn shot_asset(layer: u32) -> Asset {
    let dir = shot_root().join(format!("layer{layer}"));
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let imported = import_sequence(&files).asset.unwrap();
    let frames: BTreeMap<u32, String> = imported
        .frames()
        .iter()
        .map(|(n, p)| (*n, format!("layer{layer}/{}", p.file_name().unwrap().to_str().unwrap())))
        .collect();
    Asset {
        id: Id::new(&format!("asset-layer{layer}")),
        kind: AssetKind::ImageSequence,
        name: format!("layer{layer}"),
        path: None,
        pattern: Some(imported.pattern().to_string()),
        frames,
        interpretation: Default::default(),
        redistribute: true,
    }
}

fn shot() -> Project {
    let sheet: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(shot_root().join("exposure_sheet.json")).unwrap()).unwrap();
    let array = |key: &str| -> Vec<u32> {
        sheet[key].as_array().unwrap().iter().map(|v| v.as_u64().unwrap() as u32).collect()
    };
    let mut project = Project::new(Id::new("proj-b160-shot"));
    let rate = FrameRate::new(24, 1).unwrap();
    project.compositions.push(Composition::new(Id::new(SHOT), "reference shot", 1920, 1080, rate, 0, 240));
    let mut doc = Document::new(project);
    for n in 1..=4u32 {
        let exposures: Vec<(u32, u32)> = match n {
            1 => vec![(0, 240)],
            2 => (0..240).map(|f| (f % 24, 1)).collect(),
            3 => (0..120).map(|k| (k % 12, 2)).collect(),
            _ => array("layer4_exposure_drawing_ids").into_iter().zip(array("layer4_exposure_lengths")).collect(),
        };
        let asset = shot_asset(n);
        let mut layer = Layer::new(Id::new(&format!("layer-{n}")), format!("layer{n}"), asset.id.clone(), 0, 240);
        layer.exposure_spans = ExposureMap::from_lengths(&exposures).unwrap().spans().to_vec();
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer { composition: Id::new(SHOT), layer: Box::new(layer), index: (n - 1) as usize },
        ])
        .unwrap();
    }
    doc.apply_all(vec![Command::SetPropertyBase {
        composition: Id::new(SHOT),
        target: Target::Layer(Id::new("layer-4")),
        prop: Prop::Opacity,
        value: Value::Scalar(1.0),
    }])
    .unwrap();
    doc.project().clone()
}

// ---- one export, hashed ------------------------------------------------------------------

struct Job<'a> {
    name: String,
    project: &'a Project,
    root: PathBuf,
    composition: Id,
    first: i32,
    last: i32,
    format: OutputFormat,
    depth: OutputDepth,
    alpha: OutputAlpha,
    naming: &'static str,
}

fn request(job: &Job, dir: &Path, choices: ExportChoices) -> ExportRequest {
    ExportRequest {
        composition: job.composition.clone(),
        first_frame: job.first,
        last_frame: job.last,
        output_dir: dir.to_path_buf(),
        naming: job.naming.to_string(),
        depth: job.depth,
        alpha: job.alpha,
        tile_size: DEFAULT_TILE_SIZE,
        missing: MissingSource::RenderTransparent,
        format: job.format,
        choices,
    }
}

/// `(what, sha-256)` for every file the job wrote, in name order, then its report. Runs that are
/// compared use the same `folder` under `target/b160`, so the paths a report names are the same.
fn hashes(job: &Job, choices: ExportChoices, folder: &str) -> Vec<(String, String)> {
    let dir = out_dir(folder);
    let report = export_sequence(job.project, &job.root, &request(job, &dir, choices), &AtomicBool::new(false));
    let mut names: Vec<PathBuf> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    names.sort();
    let mut out: Vec<(String, String)> = Vec::new();
    for p in &names {
        let what = format!("{} {}", job.name, p.file_name().unwrap().to_string_lossy());
        let bytes = fs::read(p).unwrap();
        out.push((what.clone(), sha256::hex(&bytes)));
        if p.extension().is_some_and(|e| e == "mp4") {
            out.push((format!("{what} (dates set aside)"), sha256::hex(&mp4_dates_aside(bytes))));
        }
    }
    let said = format!(
        "{:?} {} {:?} {} {:?}",
        report.status, report.files_expected, report.written, report.fidelity_incomplete, report.diagnostics
    );
    out.push((format!("{} report", job.name), sha256::hex(said.as_bytes())));
    out
}

fn shot_jobs(project: &Project) -> Vec<Job<'_>> {
    let job = |name: &str, last: i32, format, depth, alpha, naming| Job {
        name: format!("reference shot {name}"),
        project,
        root: shot_root(),
        composition: Id::new(SHOT),
        first: 0,
        last,
        format,
        depth,
        alpha,
        naming,
    };
    use OutputAlpha::*;
    use OutputDepth::*;
    vec![
        job("PNG 8-bit", 239, OutputFormat::Png, Eight, Straight, "shot_%04d.png"),
        job("PNG 16-bit premultiplied", 47, OutputFormat::Png, Sixteen, Premultiplied, "shot_%04d.png"),
        job("EXR half", 47, OutputFormat::Exr(ExrSamples::Half), Eight, Straight, "shot_%04d.exr"),
        job("EXR float", 47, OutputFormat::Exr(ExrSamples::Float), Eight, Straight, "shot_%04d.exr"),
        job("GIF", 47, OutputFormat::Gif, Eight, Straight, "shot.gif"),
        job("animated PNG", 47, OutputFormat::Apng, Eight, Straight, "shot.png"),
        job("MP4", 239, OutputFormat::Mp4, Eight, Straight, "shot.mp4"),
    ]
}

/// Every fixture project that opens, with the folder its media is relative to.
fn fixtures() -> Vec<(String, Project, PathBuf)> {
    let mut files: Vec<PathBuf> = Vec::new();
    for dir in fs::read_dir(repo("Fixtures")).unwrap() {
        let dir = dir.unwrap().path();
        if !dir.is_dir() {
            continue;
        }
        for f in fs::read_dir(&dir).unwrap() {
            let f = f.unwrap().path();
            let name = f.file_name().unwrap().to_string_lossy().to_string();
            if name.ends_with(".json") && !name.starts_with("expected") && !name.contains("manifest") {
                files.push(f);
            }
        }
    }
    files.sort();
    files
        .into_iter()
        .filter_map(|f| {
            let loaded = persist::load(&f).ok()?;
            let rel = f.strip_prefix(repo("Fixtures")).unwrap().to_string_lossy().replace('\\', "/");
            Some((rel, loaded.document.project().clone(), f.parent().unwrap().to_path_buf()))
        })
        .collect()
}

fn fixture_jobs<'a>(fixtures: &'a [(String, Project, PathBuf)]) -> Vec<Job<'a>> {
    let mut jobs = Vec::new();
    for (rel, project, root) in fixtures {
        for c in &project.compositions {
            let last = c.start_frame + c.duration_frames.max(1) as i32 - 1;
            for (kind, format, naming) in [
                ("PNG", OutputFormat::Png, "f_%04d.png"),
                ("EXR half", OutputFormat::Exr(ExrSamples::Half), "f_%04d.exr"),
            ] {
                jobs.push(Job {
                    name: format!("{rel} {} {kind}", c.id.as_str()),
                    project,
                    root: root.clone(),
                    composition: c.id.clone(),
                    first: c.start_frame,
                    last,
                    format,
                    depth: OutputDepth::Eight,
                    alpha: OutputAlpha::Straight,
                    naming,
                });
            }
        }
    }
    jobs
}

fn label() -> String {
    std::env::var("B160_LABEL").unwrap_or_else(|_| "after".to_string())
}

#[test]
#[ignore]
fn b160_every_export_hashed() {
    let project = shot();
    let fixtures = fixtures();
    let mut lines: Vec<String> = Vec::new();
    let started = Instant::now();
    for job in shot_jobs(&project).iter().chain(fixture_jobs(&fixtures).iter()) {
        for (what, sha) in hashes(job, ExportChoices::default(), "work") {
            lines.push(format!("{what}\t{sha}"));
        }
    }
    let file = repo("target/b160").join(format!("hashes_{}.txt", label()));
    fs::write(&file, lines.join("\n") + "\n").unwrap();
    eprintln!("{} lines in {:.0} s, {} fixture projects", lines.len(), started.elapsed().as_secs_f64(), fixtures.len());
}

/// Seven exports of the whole reference shot as PNG and as MP4, and their median.
#[test]
#[ignore]
fn b160_timing() {
    let project = shot();
    let jobs = shot_jobs(&project);
    let mut out = String::new();
    for job in jobs.iter().filter(|j| j.last == 239) {
        let dir = out_dir("timing");
        let mut ms: Vec<f64> = (0..7)
            .map(|_| {
                let t = Instant::now();
                let r = export_sequence(job.project, &job.root, &request(job, &dir, ExportChoices::default()), &AtomicBool::new(false));
                assert!(r.succeeded(), "{:?}", r.diagnostics);
                t.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out.push_str(&format!("{}\t{:.0}\t{:?}\n", job.name, ms[3], ms.iter().map(|m| m.round()).collect::<Vec<_>>()));
    }
    fs::write(repo("target/b160").join(format!("timing_{}.txt", label())), &out).unwrap();
    eprintln!("{out}");
}

// ---- D-231's own choices from here down. The old build does not have them, so the copy of
// ---- this file the old build is compiled with is cut at this line.

fn choices(frames_at_once: usize) -> ExportChoices {
    ExportChoices { frames_at_once, ..ExportChoices::default() }
}

/// How many lines of two hash lists differ. An MP4's whole-file line is left out: Windows stamps
/// the moment into the file, so it differs between any two runs; its dates-aside line is kept.
fn differing(a: &[(String, String)], b: &[(String, String)]) -> usize {
    if a.len() != b.len() {
        return a.len().max(b.len());
    }
    a.iter().zip(b).filter(|(x, y)| !x.0.ends_with(".mp4") && x != y).count()
}

struct Row {
    check: String,
    expected: String,
    actual: String,
}

fn row(check: impl Into<String>, expected: impl Into<String>, actual: impl Into<String>) -> Row {
    Row { check: check.into(), expected: expected.into(), actual: actual.into() }
}

/// The reference shot's seven jobs, cut to frames 0 to 23, drawn one at a time, 5 at once and as
/// many at once as the export picks (15 at this size on 24 threads). Neither 5 nor 15 divides 24,
/// so each ends on a short batch.
fn frames_at_once_rows(project: &Project) -> Vec<Row> {
    let mut rows = Vec::new();
    for mut job in shot_jobs(project) {
        job.last = 23;
        let one = hashes(&job, choices(1), "at_once");
        for (n, said) in [(5, "5 at once"), (0, "as many as the export picks")] {
            let other = hashes(&job, choices(n), "at_once");
            let d = differing(&one, &other);
            rows.push(row(
                format!("{}, frames 0 to 23: drawn {said}, against one at a time", job.name),
                "the same bytes and report",
                if d == 0 { "the same bytes and report".to_string() } else { format!("{d} lines differ") },
            ));
        }
    }
    rows
}

#[test]
fn b160_frames_at_once_writes_the_same_bytes() {
    let project = shot();
    for r in frames_at_once_rows(&project) {
        assert_eq!(r.expected, r.actual, "{}", r.check);
    }
}

fn mp4_job(project: &Project, last: i32) -> Job<'_> {
    let mut job = shot_jobs(project).pop().unwrap();
    assert!(matches!(job.format, OutputFormat::Mp4));
    job.last = last;
    job
}

fn median_ms(runs: usize, mut once: impl FnMut()) -> (f64, Vec<f64>) {
    let mut ms: Vec<f64> = (0..runs)
        .map(|_| {
            let t = Instant::now();
            once();
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (ms[runs / 2], ms)
}

fn read_lines(name: &str) -> Vec<(String, String)> {
    fs::read_to_string(repo("target/b160").join(name))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .map(|(a, b)| (a.to_string(), b.to_string()))
        .collect()
}

/// Runs after `b160_every_export_hashed` and `b160_timing` have run for the old build
/// (`B160_LABEL=before`) and this one. Writes `verification/B-160_faster_export_table.md`.
#[test]
#[ignore]
fn b160_table() {
    let project = shot();
    let mut checks: Vec<Row> = Vec::new();
    let mut measured: Vec<(String, String)> = Vec::new();

    // 1. Every export, old build against new.
    let before = read_lines("hashes_before.txt");
    let after = read_lines("hashes_after.txt");
    checks.push(row("lines hashed: every file and every report, old build and new", format!("{}", before.len()), format!("{}", after.len())));
    checks.push(row(
        "the same files, named the same, in the same order",
        "the same",
        if before.iter().map(|l| &l.0).eq(after.iter().map(|l| &l.0)) { "the same" } else { "not the same" },
    ));
    let d = differing(&before, &after);
    let compared = before.iter().filter(|l| !l.0.ends_with(".mp4")).count();
    checks.push(row(
        "files and reports whose bytes changed (the MP4 compared with its dates set aside)",
        format!("0 of {compared}"),
        format!("{d} of {compared}"),
    ));
    let projects: std::collections::BTreeSet<&str> =
        before.iter().filter(|l| !l.0.starts_with("reference shot")).map(|l| l.0.split(' ').next().unwrap()).collect();
    measured.push(("fixture projects exported, every composition, as PNG and EXR half".into(), format!("{}", projects.len())));

    // 2. The MP4's whole-file hash, the same build twice.
    let job = mp4_job(&project, 47);
    let first = hashes(&job, choices(0), "twice");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let second = hashes(&job, choices(0), "twice");
    let whole = |h: &[(String, String)]| h.iter().find(|l| l.0.ends_with(".mp4")).unwrap().1.clone();
    let aside = |h: &[(String, String)]| h.iter().find(|l| l.0.ends_with("(dates set aside)")).unwrap().1.clone();
    checks.push(row(
        "the same build, the same MP4 twice, a second apart: whole files, then dates set aside",
        "different, then the same",
        format!(
            "{}, then {}",
            if whole(&first) == whole(&second) { "the same" } else { "different" },
            if aside(&first) == aside(&second) { "the same" } else { "different" }
        ),
    ));

    // 3. How many frames at once changes no byte.
    checks.extend(frames_at_once_rows(&project));

    // 4. Speed. PROVISIONAL: other builds shared the machine.
    let mut timing: Vec<(String, String)> = Vec::new();
    for label in ["before", "after"] {
        for (what, median) in read_lines(&format!("timing_{label}.txt")) {
            timing.push((format!("{what}, {}", if label == "before" { "old build, one frame at a time" } else { "new build, several frames at once" }), median.split('\t').next().unwrap().to_string() + " ms"));
        }
    }
    let png_job = shot_jobs(&project).into_iter().next().unwrap();
    let dir = out_dir("timing");
    let (one, _) = median_ms(7, || {
        assert!(export_sequence(&project, &png_job.root, &request(&png_job, &dir, choices(1)), &AtomicBool::new(false)).succeeded());
    });
    timing.push(("reference shot PNG 8-bit, new build told to draw one frame at a time".into(), format!("{one:.0} ms")));

    // The table.
    let passed = checks.iter().filter(|r| r.expected == r.actual).count();
    let mut md = String::new();
    md.push_str("# B-160: faster export\n\n");
    md.push_str("Written by `cargo test --release --test b160_faster_export -- --ignored b160_table`, after `b160_every_export_hashed` and `b160_timing` have run for the old build (`B160_LABEL=before`, compiled from the commit before B-160) and for this one.\n\n");
    md.push_str("**D-231 (bit-exact):** an export now draws several frames at once and writes them in order. Every file it writes must be the file the old build wrote, byte for byte, and every report the same report. **D-230 (REJECTED by the owner on 2026-09-29):** the graphics card's video encoder was no faster on this machine and wrote bigger files, so B-160b removed it; every MP4 comes from the software encoder, as before B-160.\n\n");
    md.push_str(&format!("## Checks: {passed} of {} pass\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n", checks.len()));
    for r in &checks {
        md.push_str(&format!("| {} | {} | {} | {} |\n", r.check, r.expected, r.actual, if r.expected == r.actual { "pass" } else { "FAIL" }));
    }
    md.push_str("\n## Measurements (not pass or fail)\n\n| What | Measured |\n|---|---|\n");
    for (what, value) in &measured {
        md.push_str(&format!("| {what} | {value} |\n"));
    }
    md.push_str(&format!(
        "\n## Speed, median of 7 (PROVISIONAL)\n\nMachine: {}, {} threads; release build (opt-level 3). Other builds were running on the machine at the same time, so these are provisional until a quiet re-measure.\n\n| Export of the whole reference shot, 240 frames at 1920x1080 | Median |\n|---|---|\n",
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_default(),
        std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
    ));
    for (what, ms) in &timing {
        md.push_str(&format!("| {what} | {ms} |\n"));
    }
    fs::write(repo("verification/B-160_faster_export_table.md"), md).unwrap();
    let failed: Vec<&Row> = checks.iter().filter(|r| r.expected != r.actual).collect();
    assert!(failed.is_empty(), "{:#?}", failed.iter().map(|r| (&r.check, &r.expected, &r.actual)).collect::<Vec<_>>());
}
