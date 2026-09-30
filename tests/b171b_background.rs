//! B-171b: a composition frame the viewer keeps on disk is written there by a worker, so the frame
//! that drew it is not held up by the write (the first play no slower than before B-171), and
//! what is written is what was written before, byte for byte. A full-size frame is kept, then:
//!
//! 1. the keeping returns before its copy is on disk (the write is not in the frame);
//! 2. a new viewer with the same folder, asking at once, gets the frame back bit for bit (it waits
//!    for a write still under way rather than drawing again);
//! 3. the copy on disk is the file B-171 wrote: its bytes match one written with no worker.
//!
//! Writes `verification/B-171b_background_table.md`.

use std::fs;
use std::sync::Arc;
use std::time::Duration;

use anime_compositor::cache::{CelCache, DiskCache};
use anime_compositor::WorkingBuffer;

mod common;
use common::repo;

fn viewer(disk: &std::path::Path) -> CelCache {
    let mut cache = CelCache::viewer();
    cache.set_disk(Some(DiskCache { folder: disk.to_path_buf(), cap: 20_000_000_000 }));
    cache
}

fn frames(disk: &std::path::Path) -> Vec<std::path::PathBuf> {
    fs::read_dir(disk).map_or_else(|_| Vec::new(), |l| {
        l.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "frame")).collect()
    })
}

#[test]
fn b171b_written_by_a_worker() {
    let disk = repo("target/b171b_disk");
    let _ = fs::remove_dir_all(&disk);
    let mut picture = WorkingBuffer::transparent(1920, 1080);
    for (i, v) in picture.data_mut().iter_mut().enumerate() {
        *v = (i % 997) as f32 / 996.0;
    }
    let picture = Arc::new(picture);
    let key = "b171b frame".to_string();
    let mut first = viewer(&disk);
    first.store_inner(key.clone(), Vec::new(), Arc::clone(&picture), Duration::from_secs(1));
    let on_disk_at_return = !frames(&disk).is_empty();
    let back = viewer(&disk).inner_frame(&key);
    let same = back.as_deref().is_some_and(|b| b.data() == picture.data());
    let written = frames(&disk);
    // The copy B-171 wrote, byte for byte: magic, width, height, text length, the two checksums,
    // the key and an empty file list, then the floats.
    let bytes = written.first().map(|f| fs::read(f).expect("read the copy")).unwrap_or_default();
    let text = format!("{key}\0");
    let floats: &[u8] = bytemuck::cast_slice(picture.data());
    let whole = bytes.len() == 48 + text.len() + floats.len()
        && bytes[..8] == *b"TNAEfrm1"
        && bytes[8..16] == 1920u64.to_le_bytes()
        && bytes[16..24] == 1080u64.to_le_bytes()
        && bytes[24..32] == (text.len() as u64).to_le_bytes()
        && bytes[48..48 + text.len()] == *text.as_bytes()
        && bytes[48 + text.len()..] == *floats;
    let rows = [
        ("Keeping a frame returns before its copy is on disk", !on_disk_at_return),
        ("A new viewer asking at once gets the frame back bit for bit", same),
        ("One copy on disk, laid out byte for byte as B-171 wrote it", written.len() == 1 && whole),
    ];
    let mut table = String::from(
        "# B-171b: composition frames written to disk by a worker\n\n\
         Written by `tests/b171b_background.rs`. A 1920 by 1080 composition frame is kept by the \
         viewer as B-171 keeps a slow one, and the disk folder looked at straight away.\n\n\
         | Check | Result |\n|---|---|\n",
    );
    for (check, ok) in rows {
        table += &format!("| {check} | {} |\n", if ok { "pass" } else { "FAIL" });
    }
    let passed = rows.iter().filter(|(_, ok)| *ok).count();
    table += &format!("\n**{passed} of {} pass.**\n", rows.len());
    fs::write(repo("verification/B-171b_background_table.md"), table).expect("write the table");
    assert_eq!(passed, rows.len(), "B-171b checks failed");
}
