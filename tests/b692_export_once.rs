//! B-692 (D-812): an export that blocks on a missing drawing draws each frame once. Before it,
//! the range was planned whole to look for missing drawings and then drawn again to be written;
//! now a frame whose plan is whole is drawn and encoded the first time, its file held until the
//! range is known whole (up to 1 GiB of files), and written from there.
//!
//! A change of speed only, so its expected value is the old build's own output: `b692_hashes`
//! writes, for every export below, each file's SHA-256 and the report's, to
//! `target/b692/hashes_<B692_LABEL>.txt`. The old build is this file compiled against the
//! commit before B-692 with `B692_LABEL=before` and `B692_REPO` set to this checkout, so the
//! paths a report names are the same. The exports, all with the default missing-drawing policy
//! (Block):
//!
//! - the reference shot (`verification/B-08a_project.json`): frames 0 to 47 as 8-bit PNG,
//!   16-bit premultiplied PNG and EXR half, frames 0 to 23 as a GIF and an animated PNG;
//! - every fixture project in `Fixtures/` that opens: each composition, every frame, as 8-bit
//!   PNG. Those with a missing drawing are blocked, so their reports are compared too.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::time::Instant;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::export::{
    export_sequence, ExportChoices, ExportReport, ExportRequest, MissingSource, OutputFormat,
};
use anime_compositor::exr_io::ExrSamples;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::sha256;
use anime_compositor::{OutputAlpha, OutputDepth};

fn repo(rel: &str) -> PathBuf {
    std::env::var("B692_REPO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .join(rel)
}

fn out_dir(rel: &str) -> PathBuf {
    let dir = repo("target/b692").join(rel);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

struct Job {
    name: String,
    project: Project,
    root: PathBuf,
    composition: Id,
    first: i32,
    last: i32,
    format: OutputFormat,
    depth: OutputDepth,
    alpha: OutputAlpha,
    naming: &'static str,
}

fn export(job: &Job, dir: &Path, missing: MissingSource, choices: ExportChoices) -> ExportReport {
    let request = ExportRequest {
        composition: job.composition.clone(),
        first_frame: job.first,
        last_frame: job.last,
        output_dir: dir.to_path_buf(),
        naming: job.naming.to_string(),
        depth: job.depth,
        alpha: job.alpha,
        tile_size: DEFAULT_TILE_SIZE,
        missing,
        format: job.format,
        choices,
    };
    export_sequence(&job.project, &job.root, &request, &AtomicBool::new(false))
}

/// `(what, sha-256)` of every file the job wrote, in name order, then of its report.
fn hashes(job: &Job, missing: MissingSource, choices: ExportChoices) -> Vec<(String, String)> {
    let dir = out_dir("work");
    let report = export(job, &dir, missing, choices);
    let mut names: Vec<PathBuf> = fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()).collect();
    names.sort();
    let mut out: Vec<(String, String)> = names
        .iter()
        .map(|p| {
            (
                format!("{} {}", job.name, p.file_name().unwrap().to_string_lossy()),
                sha256::hex(&fs::read(p).unwrap()),
            )
        })
        .collect();
    let said = format!(
        "{:?} {} {:?} {} {:?}",
        report.status,
        report.files_expected,
        report.written,
        report.fidelity_incomplete,
        report.diagnostics
    );
    if std::env::var("B692_SAY").is_ok() {
        eprintln!("SAID {}: {said}", job.name);
    }
    out.push((format!("{} report", job.name), sha256::hex(said.as_bytes())));
    out
}

fn shot() -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).unwrap();
    persist::load_str(&text).ok().unwrap().document.project().clone()
}

fn shot_job(name: &str, last: i32, format: OutputFormat, depth: OutputDepth, alpha: OutputAlpha, naming: &'static str) -> Job {
    let project = shot();
    Job {
        name: format!("reference shot {name}"),
        composition: project.compositions[0].id.clone(),
        project,
        root: repo("Fixtures/reference_shot"),
        first: 0,
        last,
        format,
        depth,
        alpha,
        naming,
    }
}

fn jobs() -> Vec<Job> {
    use OutputAlpha::*;
    use OutputDepth::*;
    let mut jobs = vec![
        shot_job("PNG 8-bit", 47, OutputFormat::Png, Eight, Straight, "shot_%04d.png"),
        shot_job("PNG 16-bit premultiplied", 47, OutputFormat::Png, Sixteen, Premultiplied, "shot_%04d.png"),
        shot_job("EXR half", 47, OutputFormat::Exr(ExrSamples::Half), Eight, Straight, "shot_%04d.exr"),
        shot_job("GIF", 23, OutputFormat::Gif, Eight, Straight, "shot.gif"),
        shot_job("animated PNG", 23, OutputFormat::Apng, Eight, Straight, "shot.png"),
    ];
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
    for f in files {
        let Ok(loaded) = persist::load(&f) else { continue };
        let project = loaded.document.project().clone();
        let rel = f.strip_prefix(repo("Fixtures")).unwrap().to_string_lossy().replace('\\', "/");
        for c in &project.compositions {
            jobs.push(Job {
                name: format!("{rel} {}", c.id.as_str()),
                project: project.clone(),
                root: f.parent().unwrap().to_path_buf(),
                composition: c.id.clone(),
                first: c.start_frame,
                last: c.start_frame + c.duration_frames.max(1) as i32 - 1,
                format: OutputFormat::Png,
                depth: OutputDepth::Eight,
                alpha: OutputAlpha::Straight,
                naming: "f_%04d.png",
            });
        }
    }
    jobs
}

fn label() -> String {
    std::env::var("B692_LABEL").unwrap_or_else(|_| "after".to_string())
}

#[test]
#[ignore = "B-692: slow, run by name, once on the old build and once on this one"]
fn b692_hashes() {
    let started = Instant::now();
    let mut lines: Vec<String> = Vec::new();
    let only = std::env::var("B692_SAY").unwrap_or_default();
    for job in jobs().into_iter().filter(|j| j.name.contains(&only)) {
        for (what, sha) in hashes(&job, MissingSource::Block, ExportChoices::default()) {
            lines.push(format!("{what}\t{sha}"));
        }
    }
    fs::write(repo("target/b692").join(format!("hashes_{}.txt", label())), lines.join("\n") + "\n").unwrap();
    eprintln!("{} lines in {:.0} s", lines.len(), started.elapsed().as_secs_f64());
}

/// Three exports of the reference shot as 8-bit PNG, in ms, appended as one line to
/// `target/b692/timing_<label>.txt`; run alternately with the old build: frames 0 to 13
/// blocking (the default; none is missing), the same writing the missing transparent, and
/// frames 0 to 47 blocking, which stops (frames 14, 15, 38 and 39 miss a drawing of layer 3).
#[test]
#[ignore = "B-692: a measurement, run by name"]
fn b692_timing() {
    let mut row = String::new();
    for (last, missing, whole) in [
        (13, MissingSource::Block, true),
        (13, MissingSource::RenderTransparent, true),
        (47, MissingSource::Block, false),
    ] {
        let job = shot_job("PNG 8-bit", last, OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, "shot_%04d.png");
        let dir = out_dir("timing");
        let t = Instant::now();
        let r = export(&job, &dir, missing, ExportChoices::default());
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(r.succeeded(), whole, "{:?}", r.diagnostics);
        row.push_str(&format!("{ms:.0}\t"));
    }
    let file = repo("target/b692").join(format!("timing_{}.txt", label()));
    let old = fs::read_to_string(&file).unwrap_or_default();
    fs::write(&file, format!("{old}{}\n", row.trim_end())).unwrap();
    eprintln!("{} {row}", label());
}

// ---- B-692's own checks from here down; the old build's copy of this file is cut here.

/// Blocking writes the same bytes as rendering the missing transparent when nothing is missing,
/// one frame at a time or as many at once as the export picks.
#[test]
fn b692_a_blocking_export_writes_what_drawing_once_writes() {
    let mut job = shot_job("PNG 8-bit", 11, OutputFormat::Png, OutputDepth::Eight, OutputAlpha::Straight, "shot_%04d.png");
    let files = |h: Vec<(String, String)>| h.into_iter().filter(|(w, _)| !w.ends_with("report")).collect::<Vec<_>>();
    let plain = files(hashes(&job, MissingSource::RenderTransparent, ExportChoices::default()));
    for n in [1, 0] {
        let choices = ExportChoices { frames_at_once: n, ..ExportChoices::default() };
        assert_eq!(files(hashes(&job, MissingSource::Block, choices)), plain, "{n} at once");
    }
    job.format = OutputFormat::Gif;
    job.naming = "shot.gif";
    let plain = files(hashes(&job, MissingSource::RenderTransparent, ExportChoices::default()));
    assert_eq!(files(hashes(&job, MissingSource::Block, ExportChoices::default())), plain, "GIF");
}
