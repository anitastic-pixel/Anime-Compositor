//! D-259: a middle preview quality, Half, beside Full and Draft.
//!
//! Writes `verification/D-259_half_table.md` and `verification/D-259_half_frame_10.png`.
//!
//! The owner accepted Half on 2026-10-02 with three conditions, each a row here:
//!
//! 1. Full and Draft stay byte for byte as they were. Their hashes below were taken from the
//!    build before Half existed (258a0de), on frames 10 and 100 of the reference shot.
//! 2. Half's frame 10 is recorded as a new expected picture: its hash below was taken from the
//!    first build that had Half, and the picture itself is the file named above, to be looked at.
//! 3. Half is at least as quick as Full on this machine, measured, never assumed.

use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, sha256, OutputAlpha, OutputDepth};

const COMP: &str = "comp-reference-shot";

/// Taken on 258a0de, before Half existed.
const BEFORE: [(PreviewQuality, i32, &str); 4] = [
    (PreviewQuality::Full, 10, "06ad045e059c4f492c71d761f034f56aaf6ec034931bc731e33f33a20711d284"),
    (PreviewQuality::Full, 100, "833970a6fab53413553b4cdb2a350f1048c2fa039c6f8af2684700a3d19e5c8d"),
    (PreviewQuality::Draft, 10, "6c32af698fb03b107625be15a9b3e4ab7a901ac4da8ac73a2d69c6806288505e"),
    (PreviewQuality::Draft, 100, "46bc3314f00bc927f67c5108563e4bc9b4f580b12acfc9de2c6aa5c4525b575e"),
];

/// Half's frame 10, recorded from the first build with Half.
const HALF_10: &str = "1888530bc056fd7456c2302f1842539f954e48b0788ec33056d90ba4458a3df8";

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn project() -> Project {
    let path = repo("verification/B-08a_project.json");
    persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)).document.project().clone()
}

fn frame_at(project: &Project, frame: i32, quality: PreviewQuality) -> (usize, usize, Vec<u8>) {
    let b = preview::preview_frame(project, &Id::new(COMP), frame, &repo("Fixtures/reference_shot"), quality, DEFAULT_TILE_SIZE, &mut FrameLog::new(3))
        .unwrap_or_else(|d| panic!("preview frame {frame}: {}", d.message));
    (b.width(), b.height(), b.encode(OutputDepth::Eight, OutputAlpha::Straight))
}

fn median_ms(project: &Project, quality: PreviewQuality) -> f64 {
    frame_at(project, 10, quality);
    let mut ms: Vec<f64> = (0..7)
        .map(|_| {
            let t = Instant::now();
            frame_at(project, 10, quality);
            t.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    ms.sort_by(f64::total_cmp);
    ms[3]
}

#[test]
fn d259_half_beside_full_and_draft() {
    let project = project();
    let mut rows: Vec<(String, String, String)> = Vec::new();

    for (quality, frame, hash) in BEFORE {
        let (w, h, samples) = frame_at(&project, frame, quality);
        rows.push((format!("{} frame {frame}, {w} by {h}, is byte for byte as before Half", quality.label()), hash.into(), sha256::hex(&samples)));
    }

    let (w, h, half) = frame_at(&project, 10, PreviewQuality::Half);
    rows.push(("Half is half the composition each way".into(), "960 by 540".into(), format!("{w} by {h}")));
    rows.push(("Half says it differs from an export (the orange \"not final\")".into(), "true".into(), PreviewQuality::Half.differs_from_export().to_string()));
    rows.push(("Half frame 10 is the recorded picture".into(), HALF_10.into(), sha256::hex(&half)));
    let path = repo("verification/D-259_half_frame_10.png");
    png_out::write_rgba(&path, w, h, OutputDepth::Eight, &[("Source", "frame 10 of verification/B-08a_project.json at Half, by tests/d259_half.rs".to_string())], &half)
        .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));

    let full = median_ms(&project, PreviewQuality::Full);
    let halfms = median_ms(&project, PreviewQuality::Half);
    let draft = median_ms(&project, PreviewQuality::Draft);
    rows.push((
        format!("Half is at least as quick as Full (median of 7, frame 10, CPU: Full {full:.1} ms, Half {halfms:.1} ms, Draft {draft:.1} ms)"),
        "true".into(),
        (halfms <= full).to_string(),
    ));

    let mut md = String::from("# D-259: Half beside Full and Draft\n\nWritten by `tests/d259_half.rs` from `verification/B-08a_project.json`. The Full and Draft hashes were taken on the build before Half existed (258a0de). Times are this machine's, on the CPU, with nothing remembered between runs.\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, e, a) in &rows {
        let _ = writeln!(md, "| {check} | {e} | {a} | {} |", if e == a { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/D-259_half_table.md"), md).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "{failed:?}");
}
