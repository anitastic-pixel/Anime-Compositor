//! B-744 (P5, D-864): export reads a frame's footage side by side, as the viewer already did.
//!
//! `CelCache::prewarm` used to return at once for a cache with no budget, which is what export
//! holds, so an export frame read and unpacked its cels one after another. It now reads them
//! across the thread pool first; the layer loop then takes each one as it comes to it.
//!
//! What has to stay the same, and is written to `verification/B-744_export_read_ahead_table.md`
//! so the build before and the build after can be compared file to file:
//! - every exported file's bytes (FNV-1a of each file),
//! - the export's diagnostics, in order, with footage missing and footage that cannot be read
//!   on layers in the middle of the stack (document 28),
//! - how many frames the progress count reached, and the status,
//! - a cancelled export still writes nothing.
//!
//! `b744_export_frame_reads_ahead` is the check that the read-ahead really runs on the export
//! path, so it cannot quietly stop: with the stages on, an export frame of the reference shot
//! decodes in the read-ahead and opens no cel file inside the layer loop.
//!
//! `b744_timing` is a measurement, run deliberately (`--release --ignored`), one shot per run so
//! the peak working set is that shot's: `W9_CASES` (`name|project|media root|frames` a line),
//! `W9_CASE` the one to run, `W9_LABEL` what to call the build.
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::time::Instant;

use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::export::{self, ExportChoices, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::perf::{self, Stage};
use anime_compositor::{OutputAlpha, OutputDepth};

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

fn request(comp: &Id, first: i32, last: i32, dir: &Path, missing: MissingSource) -> ExportRequest {
    ExportRequest {
        composition: comp.clone(),
        first_frame: first,
        last_frame: last,
        output_dir: dir.to_path_buf(),
        naming: "f_%04d.png".into(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: DEFAULT_TILE_SIZE,
        missing,
        format: OutputFormat::Png,
        choices: ExportChoices::default(),
    }
}

fn reference_shot() -> Project {
    let path = repo("verification/B-08a_project.json");
    persist::load(&path)
        .unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message))
        .document
        .project()
        .clone()
}

/// The declared fixture (ten layers) with footage broken in the middle of the stack: layer 3's
/// files gone, layer 6's files not pictures, layer 8's folder gone.
fn damaged() -> (Project, PathBuf) {
    let (project, root, _) = build_fixture("b744");
    for gone in ["copy3", "copy8"] {
        std::fs::remove_dir_all(root.join(gone)).expect("remove a copy");
    }
    for entry in std::fs::read_dir(root.join("copy6")).expect("read copy6") {
        std::fs::write(entry.unwrap().path(), b"not a picture").expect("spoil a file");
    }
    (project, root)
}

/// One export, as rows: status, progress, each file's digest, each diagnostic in order.
fn export_rows(name: &str, project: &Project, root: &Path, req: &ExportRequest, cancel: bool) -> String {
    let _ = std::fs::remove_dir_all(&req.output_dir);
    std::fs::create_dir_all(&req.output_dir).unwrap();
    let done = AtomicUsize::new(0);
    let report =
        export::export_sequence_counting(project, root, req, &AtomicBool::new(cancel), &done);
    let mut rows = format!(
        "| {name} | status | {:?}, {} of {} files, progress {} |\n",
        report.status,
        report.written.len(),
        report.files_expected,
        done.load(std::sync::atomic::Ordering::SeqCst)
    );
    for file in &report.written {
        let bytes = std::fs::read(file).expect("read an exported file");
        let short = file.file_name().unwrap().to_string_lossy();
        writeln!(rows, "| {name} | {short} | {:016x} ({} bytes) |", fnv1a(&bytes), bytes.len()).unwrap();
    }
    let root_text = root.display().to_string();
    for (i, d) in report.diagnostics.iter().enumerate() {
        // The roots differ between checkouts; the order and the words are what is compared.
        let message = d.message.replace(&root_text, "<root>").replace('|', "/");
        writeln!(rows, "| {name} | diagnostic {} | {} {:?}: {message} |", i + 1, d.id, d.severity).unwrap();
    }
    rows
}

#[test]
fn b744_exports_are_the_same_with_the_read_ahead() {
    let comp = Id::new(COMP);
    let out = repo("target/b744_export");
    let reference = reference_shot();
    let reference_root = repo("Fixtures/reference_shot");
    let (broken, broken_root) = damaged();

    let mut rows = String::new();
    // Frame 14 is the reference shot's missing drawing.
    rows += &export_rows(
        "reference shot 10-17, render transparent",
        &reference,
        &reference_root,
        &request(&comp, 10, 17, &out, MissingSource::RenderTransparent),
        false,
    );
    rows += &export_rows(
        "reference shot 0-7, block",
        &reference,
        &reference_root,
        &request(&comp, 0, 7, &out, MissingSource::Block),
        false,
    );
    rows += &export_rows(
        "reference shot 10-17, block",
        &reference,
        &reference_root,
        &request(&comp, 10, 17, &out, MissingSource::Block),
        false,
    );
    rows += &export_rows(
        "broken footage 0-5, render transparent",
        &broken,
        &broken_root,
        &request(&comp, 0, 5, &out, MissingSource::RenderTransparent),
        false,
    );
    rows += &export_rows(
        "broken footage 0-5, block",
        &broken,
        &broken_root,
        &request(&comp, 0, 5, &out, MissingSource::Block),
        false,
    );
    rows += &export_rows(
        "reference shot 0-7, cancelled",
        &reference,
        &reference_root,
        &request(&comp, 0, 7, &out, MissingSource::RenderTransparent),
        true,
    );
    let _ = std::fs::remove_dir_all(&out);

    let page = format!(
        "# B-744 (D-864): export reads a frame's footage side by side\n\n\
         Written by `tests/b744_export_read_ahead.rs`. Each export below was run by the build \
         before B-744 and by this one, and this page is the same, word for word, from both: the \
         same files to the byte (FNV-1a), the same diagnostics in the same order, the same status \
         and progress count. \"Broken footage\" is the ten-layer declared fixture with layer 3's \
         and layer 8's drawings gone and layer 6's files replaced by text, so the missing and \
         unreadable footage sits in the middle of the stack.\n\n\
         | Export | Row | Result |\n|---|---|---|\n{rows}"
    );
    std::fs::write(repo("verification/B-744_export_read_ahead_table.md"), page).unwrap();
    assert!(rows.contains("MEDIA_MISSING"), "the broken footage was reported");
}

#[test]
fn b744_export_frame_reads_ahead() {
    let project = reference_shot();
    let root = repo("Fixtures/reference_shot");
    perf::enable();
    perf::reset();
    let mut log = FrameLog::new(3);
    compose::render_frame(&project, &Id::new(COMP), 0, &root, DEFAULT_TILE_SIZE, &mut log)
        .expect("frame 0 renders");
    let stages = perf::snapshot();
    let calls = |stage: Stage| {
        stages
            .iter()
            .find(|(s, ..)| *s == stage)
            .map_or(0, |(_, _, calls)| *calls)
    };
    assert!(calls(Stage::Prewarm) >= 1, "the export frame read its cels ahead");
    assert_eq!(
        calls(Stage::FileRead),
        0,
        "no cel was opened inside the layer loop: every one was read ahead"
    );
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

#[test]
#[ignore = "B-744: a measurement, run deliberately with --release --ignored"]
fn b744_timing() {
    let list = std::fs::read_to_string(std::env::var("W9_CASES").expect("W9_CASES")).unwrap();
    let want = std::env::var("W9_CASE").expect("W9_CASE");
    let label = std::env::var("W9_LABEL").unwrap_or_else(|_| "?".into());
    let line = list
        .lines()
        .find(|l| l.split('|').next() == Some(want.as_str()))
        .expect("the case is listed");
    let p: Vec<&str> = line.split('|').collect();
    let project = persist::load(Path::new(p[1])).expect("open").document.project().clone();
    let root = PathBuf::from(p[2]);
    let frames: Vec<i32> = p[3].split(',').map(|f| f.parse().unwrap()).collect();
    let comp = project.compositions[0].id.clone();

    let one: Vec<f64> = frames
        .iter()
        .map(|&f| {
            let mut log = FrameLog::new(3);
            let t = Instant::now();
            drop(compose::render_frame(&project, &comp, f, &root, DEFAULT_TILE_SIZE, &mut log).unwrap());
            t.elapsed().as_secs_f64() * 1000.0
        })
        .collect();
    let n = (project.composition(&comp).unwrap().duration_frames as i32).min(8);
    let dir = std::env::temp_dir().join(format!("b744_{want}_{label}"));
    let mut cells = Vec::new();
    let mut digest = 0u64;
    for m in [MissingSource::Block, MissingSource::RenderTransparent] {
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let req = request(&comp, 0, n - 1, &dir, m);
        let t = Instant::now();
        let r = export::export_sequence(&project, &root, &req, &AtomicBool::new(false));
        cells.push(t.elapsed().as_secs_f64() * 1000.0);
        assert!(r.succeeded(), "{want}: {:?}", r.diagnostics);
        for f in &r.written {
            digest = fnv1a(&[digest.to_le_bytes().as_slice(), &std::fs::read(f).unwrap()].concat());
        }
    }
    let _ = std::fs::remove_dir_all(&dir);
    eprintln!(
        "{label} | {want} | frame {:.1} | block {n} {:.0} | transparent {n} {:.0} | peak {} MB | files {digest:016x}",
        med(one),
        cells[0],
        cells[1],
        common::peak_working_set() >> 20
    );
}
