//! G0 spike SP-01: save and reopen a minimal document; interrupt the save and verify the
//! previous file survives intact on the target filesystem.
//!
//! Quarantined per document 06. Not production code.
//!
//! ADR-010 specifies: write to a temporary sibling, flush and close, then atomically
//! replace. This spike checks that the claim actually holds on the volume the project
//! lives on, under a hard process kill at every stage of the write.
//!
//! Fixture reference: FX-IO-001 (interrupted replacement retains the last valid project).
//! FX-IO-002 (disk-full) is NOT covered here; see the report.
//!
//! Usage:
//!   sp01_atomic_save            run the full matrix in <cwd>/spike-output/sp01
//!   sp01_atomic_save child <dir> <abort_after_bytes>    internal; aborts mid-write

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Stand-in for a saved project. Content does not matter to the question being asked;
/// only that it is a known byte sequence we can prove survived unchanged.
const GOOD: &str = r#"{"schema_version":0,"name":"minimal","compositions":[],"marker":"ORIGINAL"}"#;

/// The replacement the interrupted save was trying to write. Deliberately much larger,
/// so a partial write is a realistic torn file rather than a coincidental match.
fn replacement() -> String {
    let filler = "x".repeat(600_000);
    format!(
        "{{\"schema_version\":0,\"name\":\"minimal\",\"compositions\":[],\"marker\":\"REPLACEMENT\",\"filler\":\"{filler}\"}}"
    )
}

/// The save ADR-010 describes: temp sibling, write, flush, sync, close, atomic rename.
/// `abort_after` injects a hard process abort once that many bytes have been written,
/// simulating a crash or power loss with no unwinding and no flush.
fn atomic_save(target: &Path, body: &[u8], abort_after: Option<usize>) -> std::io::Result<()> {
    let tmp = target.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp)?;
        let chunk = 64 * 1024;
        let mut written = 0usize;
        for part in body.chunks(chunk) {
            f.write_all(part)?;
            written += part.len();
            if let Some(limit) = abort_after {
                if written >= limit {
                    // No close, no rename. The process ends here, as a crash would.
                    // std::process::exit rather than abort: identical effect on the file
                    // (writes already went to the OS, the rename never happens) and it
                    // does not raise a CRT abort dialog. This is a crash simulation, not
                    // a power-loss test; see the NOT RUN notes.
                    std::process::exit(101);
                }
            }
        }
        f.flush()?;
        f.sync_all()?;
    } // closed here
    if let Some(limit) = abort_after {
        // Abort in the window between a complete temp file and the rename.
        if limit == usize::MAX {
            std::process::exit(101);
        }
    }
    fs::rename(&tmp, target)?;
    Ok(())
}

struct Case {
    name: &'static str,
    /// None = no interruption. Some(n) = abort after n bytes. Some(MAX) = abort just
    /// before the rename, with a complete temp file on disk.
    abort_after: Option<usize>,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Child mode: perform one interrupted save inside the directory we are given.
    if args.len() >= 4 && args[1] == "child" {
        let dir = PathBuf::from(&args[2]);
        let n: usize = args[3].parse().expect("abort_after");
        let target = dir.join("project.json");
        let _ = atomic_save(&target, replacement().as_bytes(), Some(n));
        // Only reached if the abort did not fire.
        println!("child completed without aborting");
        return;
    }

    let root = std::env::current_dir().unwrap().join("spike-output").join("sp01");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).expect("create spike-output/sp01");

    println!("SP-01 atomic save under interrupted write");
    println!("target volume: {}", root.display());
    println!("filesystem   : {}", volume_info(&root));
    println!();

    let cases = [
        Case { name: "abort after 64 KiB of temp write", abort_after: Some(64 * 1024) },
        Case { name: "abort after 256 KiB of temp write", abort_after: Some(256 * 1024) },
        Case { name: "abort after temp complete, before rename", abort_after: Some(usize::MAX) },
    ];

    let mut rows: Vec<(String, String, String, String, bool)> = Vec::new();

    // Control: an uninterrupted save must actually replace the file, or the interrupted
    // cases below prove nothing.
    {
        let dir = root.join("control");
        fs::create_dir_all(&dir).unwrap();
        let target = dir.join("project.json");
        fs::write(&target, GOOD).unwrap();
        atomic_save(&target, replacement().as_bytes(), None).expect("control save");
        let after = fs::read(&target).unwrap();
        let ok = after == replacement().as_bytes();
        let strays = stray_temps(&dir);
        rows.push((
            "control: uninterrupted save".into(),
            "file replaced, no temp left".into(),
            format!("{} bytes, marker={}, strays={}", after.len(), marker(&after), strays),
            if ok && strays == 0 { "PASS".into() } else { "FAIL".into() },
            ok && strays == 0,
        ));
    }

    for (i, c) in cases.iter().enumerate() {
        let dir = root.join(format!("case{i}"));
        fs::create_dir_all(&dir).unwrap();
        let target = dir.join("project.json");
        fs::write(&target, GOOD).unwrap();
        let before = fs::read(&target).unwrap();

        let out = Command::new(std::env::current_exe().unwrap())
            .arg("child")
            .arg(&dir)
            .arg(if c.abort_after == Some(usize::MAX) {
                usize::MAX.to_string()
            } else {
                c.abort_after.unwrap().to_string()
            })
            .output()
            .expect("spawn child");

        let aborted = !out.status.success();
        let after = fs::read(&target).unwrap_or_default();
        let intact = after == before;
        let strays = stray_temps(&dir);

        rows.push((
            c.name.into(),
            "original intact, byte for byte".into(),
            format!(
                "child_aborted={} original_intact={} bytes={} marker={} stray_temp_files={}",
                aborted,
                intact,
                after.len(),
                marker(&after),
                strays
            ),
            if aborted && intact { "PASS".into() } else { "FAIL".into() },
            aborted && intact,
        ));
    }

    // Reopen check: the surviving file must still parse as the document we saved.
    println!("{:<44} {:<32} {}", "CASE", "EXPECTED", "ACTUAL");
    println!("{}", "-".repeat(140));
    let mut all = true;
    for (name, expected, actual, verdict, ok) in &rows {
        all &= ok;
        println!("{name:<44} {expected:<32} {actual}   [{verdict}]");
    }
    println!();
    println!("SP-01 result: {}", if all { "PASS" } else { "FAIL" });
    println!();
    println!("NOT RUN: FX-IO-002 disk-full. Simulating a full volume needs a dedicated");
    println!("         small volume; it is not covered by this spike and B-09 must cover it.");
    println!("NOT RUN: power loss with write caching enabled. fsync is issued; whether the");
    println!("         drive honours it is a hardware property this spike cannot observe.");

    std::process::exit(if all { 0 } else { 1 });
}

fn marker(bytes: &[u8]) -> &'static str {
    let s = String::from_utf8_lossy(bytes);
    if s.contains("ORIGINAL") {
        "ORIGINAL"
    } else if s.contains("REPLACEMENT") {
        "REPLACEMENT"
    } else {
        "UNREADABLE"
    }
}

fn stray_temps(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().ends_with(".tmp"))
                .count()
        })
        .unwrap_or(0)
}

fn volume_info(p: &Path) -> String {
    // Recorded, not asserted. The filesystem is part of the SP-01 answer: atomic
    // replacement is a filesystem guarantee, not a Rust one.
    let root = p.components().next().map(|c| c.as_os_str().to_string_lossy().to_string());
    match root {
        Some(r) => {
            let out = Command::new("cmd")
                .args(["/C", &format!("fsutil fsinfo volumeinfo {r}\\")])
                .output();
            match out {
                Ok(o) => String::from_utf8_lossy(&o.stdout)
                    .lines()
                    .find(|l| l.to_lowercase().contains("file system name"))
                    .map(|l| format!("{} ({})", l.trim(), r))
                    .unwrap_or_else(|| format!("unknown ({r})")),
                Err(_) => format!("unknown ({r})"),
            }
        }
        None => "unknown".into(),
    }
}
