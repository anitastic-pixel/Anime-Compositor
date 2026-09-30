//! B-08b: the bounded cache of decoded cels (R-06b), specified by document 27.
//!
//! This is the one module in the crate that exists for speed rather than for correctness, and
//! document 27 states the rule that follows from that: "Caching may improve interaction but must
//! never define correctness. A cold render and a fully warm render for the same immutable request
//! must produce equivalent pixels and diagnostics." So the shape here is deliberately small.
//!
//! **What it holds.** One decoded cel, after document 21's step 1: decoded, re-tagged with the
//! asset's interpretation, and converted into the working space. Not a finished frame. D-37
//! measured the reason: 75.15 ms of an 81.69 ms draft preview frame is reading and decoding the
//! four cels it needs, and 6.53 ms is rendering them into a picture. A cache of frames would
//! chase the 6.53.
//!
//! **What it is keyed on.** Document 27: "File path alone is not sufficient media identity. At
//! minimum include observed size/mtime for interactive invalidation." So the key is the path, the
//! file's length, its modification time, and the interpretation the buffer was tagged with — the
//! last because the conversion into the working space is inside what is being remembered, so two
//! assets reading the same file differently must not share an entry. A file whose metadata cannot
//! be read is decoded and not stored, because an entry that cannot notice the file changing is
//! worse than no entry.
//!
//! **How it is bounded.** In bytes actually held, not in entries. A cel is 1920x1080 f32 RGBA,
//! about 33 MB, which is four times what the PNG decoder writes for the same cel and is the
//! figure `verification/D-37_decode_cost.md` now quotes, having originally priced a held cel at
//! the decoder's 8-bit output. Eviction is least-recently-used and changes performance only, which is document 27's
//! requirement and is checked as a byte comparison rather than asserted.
//!
//! **Where it is not.** Export never sees one. [`crate::compose::plan_frame`] and
//! [`crate::export`] use [`CelCache::none`], whose budget is zero and which therefore cannot store
//! anything; only the preview path is handed a real one. A full-resolution preview and an export
//! of the same frame differ in 0 of 8,294,400 samples, and a cache is the obvious way to lose
//! that. ADR-015 makes keeping it out of the export path part of the decision rather than an
//! implementation choice.
//!
//! **What outlives the window** (B-161, D-232). A cel the viewer had to decode also has its
//! decoded 8-bit pixels written to disk, the bytes the decoder gave before any arithmetic, and the
//! next miss for the same key - in this session or a later one - reads those back instead of
//! unpacking the file again, then converts them exactly as a decode does. It is the same buffer to
//! the bit, which `verification/B-161_decode_cache_table.md` checks on every fixture image, and it
//! is the viewer's alone: [`CelCache::none`] never has one, so export still decodes. The 8-bit
//! bytes and not the working buffer's floats, because four times the bytes took longer to read
//! than the file took to decode: 49.0 ms against 44.5 ms a frame of the reference shot when this
//! was first built that way (2026-09-29, provisional).

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};
use std::time::{Duration, SystemTime};

use rayon::prelude::*;

use crate::compose::retag;
use crate::diagnostics::Diagnostic;
use crate::model::Id;
use crate::effects::{Bypassed, EffectInstance};
use crate::mask::Mask;
use crate::media;
use crate::model::Interpretation;
use crate::{ImageBuffer, WorkingBuffer};

/// The budget the viewer uses when nobody says otherwise.
///
/// Document 27: "Define a configurable cache budget after measuring the reference machine." It was
/// measured, and `verification/B-08b_cache_budget.md` is that measurement: forty-eight consecutive
/// frames of the reference shot at four budgets, on the reference machine, in a release build.
///
/// The first number came out of that table rather than out of a guess about it: 128 MB took a
/// draft preview frame of the reference shot from 100.29 ms to 42.55 ms, and 512 MB, four times
/// the memory, took it to 42.16 ms. On a four-layer shot the gap is smaller than the run-to-run
/// noise, because playback is sequential - a cel is asked for again within a few frames of first
/// use or not for a long time, so what has to fit is the reuse distance and not the shot.
///
/// **That table measured the wrong shot, and `verification/T-06_declared_fixture.md` measured the
/// right one.** Document 08 line 41 declares a ten-layer fixture. One cel of it costs 33,177,600
/// bytes to hold and one frame needs ten of them, which is 316.4 MiB, so at 128 MB the cache
/// could not hold a single frame of the shot the project's own performance target is written
/// against: showing a frame evicted the cels that made it, every loop decoded 2,340 cels against
/// 480 from memory, and asking for the frame just shown cost 371.71 ms at the median. At a budget
/// with room for one frame the same request cost 190.15 ms. That is the whole of what this number
/// decides, and it is worth about half the cost of a repeated frame on the declared fixture.
///
/// One gibibyte is 32 cels of that size: a frame of the heaviest fixture this project declares,
/// plus two more frames' worth of neighbourhood to scrub inside. It is a ceiling and not an
/// allocation - the cache holds what it has been given, so a four-layer shot still holds four
/// layers' worth and this costs it nothing. Reaching the ceiling on the reference machine's 32 GB
/// is about 1.3 GB of working set.
///
/// **What it does not buy is the target.** 190.15 ms is not document 08 line 41's 100 ms, and no
/// budget reaches it: with all ten cels in memory the remaining cost is compositing, not decoding.
/// Raising this closes half of D-40 and leaves the other half open for the owner.
pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;

/// The share of [`DEFAULT_BUDGET_BYTES`] the evaluated-effect cache is given (P-11).
///
/// `verification/P-09_effect_reuse.md` counted the declared fixture's blur at twelve distinct
/// results and priced them at 0.37 GiB, and P-11's own split of the effect stage in `src/perf.rs`
/// measured that blur at 111 to 123 ms of every declared-fixture frame against 1.8 ms for the
/// exposure and the tint together. A blurred result is larger than the cel it came from, because
/// a blur grows the buffer by its kernel radius on every side, so this is those twelve results
/// plus the room that growth takes.
///
/// **It is taken out of D-40's gibibyte and not added to it**, which is what document 15's P-11
/// requires: the viewer builds one cache with this many bytes for effect results and the rest for
/// cels, so the total the window holds is what D-40 already set.
pub const DEFAULT_EFFECT_BUDGET_BYTES: usize = 448 * 1024 * 1024;

/// The machine's memory in bytes, as Windows reports it (B-48, D-105).
#[cfg(windows)]
pub fn installed_memory() -> Option<u64> {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    let mut status = MEMORYSTATUSEX { dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32, ..Default::default() };
    // SAFETY: `status` is ours and its length is set, as the call requires.
    unsafe { GlobalMemoryStatusEx(&mut status) }.ok()?;
    Some(status.ullTotalPhys)
}

#[cfg(not(windows))]
pub fn installed_memory() -> Option<u64> {
    None
}

/// D-105: what the viewer's cache may hold when the memory setting is Automatic. A quarter of the
/// machine's memory, and never less than D-40's gibibyte.
pub fn automatic_budget() -> usize {
    installed_memory().map_or(DEFAULT_BUDGET_BYTES, |m| ((m / 4) as usize).max(DEFAULT_BUDGET_BYTES))
}

/// D-105: the most a Custom setting may give the viewer's cache: three quarters of the machine's
/// memory, so Windows and every other program keep a quarter.
pub fn largest_budget() -> usize {
    installed_memory().map_or(DEFAULT_BUDGET_BYTES, |m| ((m / 4 * 3) as usize).max(DEFAULT_BUDGET_BYTES))
}

/// The way a budget is written in the pages that quote one, so a page and the constant it is
/// quoting cannot drift apart. Every artifact that names the default calls this rather than
/// spelling the number, which is how "128 MB" ended up in four committed pages.
pub fn budget_label(bytes: usize) -> String {
    let mib = bytes / (1024 * 1024);
    if mib >= 1024 && mib.is_multiple_of(1024) {
        format!("{} GiB", mib / 1024)
    } else {
        format!("{mib} MiB")
    }
}

/// B-161, D-232: what the disk copies of decoded drawings may take when Preferences says nothing
/// else: five gigabytes as Preferences counts them, about six hundred 1920x1080 drawings at
/// 8,294,432 bytes each.
pub const DEFAULT_DISK_CAP_BYTES: u64 = 5_000_000_000;

/// B-161: raise this whenever a change to [`media::decode_8bit`] could change the bytes it gives
/// for a file, such as a decoder crate that reads a file differently, so that no copy written
/// before the change is read after it. The colour arithmetic after it needs no number: it runs
/// on every copy read, as it does on every decode.
const DECODER_VERSION: u32 = 1;

/// The first eight bytes of every copy. A file that does not start with them is not one.
const MAGIC: &[u8; 8] = b"TNAEcel1";
/// The magic, the width, the height and the checksum: four eight-byte words.
const HEADER: usize = 32;
/// B-171: the first eight bytes of every kept composition frame.
const FRAME_MAGIC: &[u8; 8] = b"TNAEfrm1";
/// The magic, the width, the height, the length of the text, and the text's and the pixels'
/// checksums: six eight-byte words.
const FRAME_HEADER: usize = 48;
/// B-171 (D-243): a composition frame is written to disk only when drawing it took at least as
/// long as reading its bytes back would, at this many bytes a millisecond: 16.6 ms for a
/// 1920x1080 frame at Full, 4.1 ms at Draft. Lighter frames are only kept in memory.
// ponytail: one figure for every disk; measure the folder's own speed if a slow disk makes
// reading back slower than drawing.
const DISK_BYTES_PER_MS: f64 = 2_000_000.0;

/// B-171b: a composition frame on its way to disk: the folder, its file, its key and file list,
/// the frame, and where a failure to write it is noted for the cache that kept it.
type Job = (DiskCache, PathBuf, String, String, Arc<WorkingBuffer>, Arc<Mutex<Vec<String>>>);

/// B-171b: the one worker that writes composition frames to disk, so the frame that drew one does
/// not wait for the disk, and the files it has still to write, so a reader waits for one rather
/// than drawing it again. Started by the first frame written.
struct Writer {
    jobs: std::sync::mpsc::SyncSender<Job>,
    pending: Mutex<Vec<PathBuf>>,
    done: Condvar,
}

impl Writer {
    /// `file` is no longer on its way to disk, written or not.
    fn finished(&self, file: &Path) {
        let mut pending = self.pending.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(at) = pending.iter().position(|f| f == file) {
            pending.remove(at);
        }
        self.done.notify_all();
    }
}

static WRITER: OnceLock<Writer> = OnceLock::new();

fn writer() -> &'static Writer {
    WRITER.get_or_init(|| {
        // ponytail: eight frames queued at most (265 MB at Full); past that a frame is kept in
        // memory only. Queue by bytes, or write when the window is idle, if a slow disk loses many.
        let (jobs, queue) = std::sync::mpsc::sync_channel::<Job>(8);
        std::thread::Builder::new()
            .name("composition frames to disk".into())
            .spawn(move || {
                // Its checksums on a thread of its own, not the pool the frames are drawn on
                // ([`checksum`] gives the same answer on any number of threads).
                let own = rayon::ThreadPoolBuilder::new().num_threads(1).build().expect("a one-thread pool");
                for (disk, file, key, text, picture, notes) in queue {
                    if let Err(e) = own.install(|| disk.store_frame(&file, &key, &text, &picture)) {
                        notes.lock().unwrap_or_else(|p| p.into_inner()).push(format!(
                            "FRAME_CACHE_NOT_WRITTEN: a composition frame was drawn but could not be kept on disk ({e}); it will be drawn again next time."
                        ));
                    }
                    writer().finished(&file);
                }
            })
            .expect("start the worker that writes composition frames");
        Writer { jobs, pending: Mutex::new(Vec::new()), done: Condvar::new() }
    })
}

/// B-171b: wait until nothing is still being written to `file`, or with `None` to any file.
fn written(file: Option<&Path>) {
    let Some(w) = WRITER.get() else { return };
    let mut pending = w.pending.lock().unwrap_or_else(|p| p.into_inner());
    while pending.iter().any(|f| file.is_none_or(|file| f == file)) {
        pending = w.done.wait(pending).unwrap_or_else(|p| p.into_inner());
    }
}

/// B-171b: wait until every composition frame on its way to disk is there.
pub fn disk_writes_done() {
    written(None);
}

/// B-161, D-232: where decoded cels are kept on disk between sessions, and how much they may take.
///
/// Document 27 line 58: "Temporary cache storage, if added later, must be separate, disposable and
/// versioned." Separate: its own folder, and it only ever reads, writes or deletes files ending
/// `.cel`, `.frame` or `.part` there, so a folder chosen by mistake loses nothing else. Disposable:
/// every file is a copy of something the drawing file can produce again, and a copy that fails its
/// checks is deleted and decoded fresh. Versioned: [`DECODER_VERSION`] is in every cel's name, and
/// the program's own identity in every composition frame's (B-171).
#[derive(Clone, Debug, PartialEq)]
pub struct DiskCache {
    pub folder: PathBuf,
    /// The most bytes of copies the folder may hold. Least recently used go first past it.
    pub cap: u64,
}

impl DiskCache {
    /// The file a cel with `key` is kept in. `None` for a file dated before 1970, which has no
    /// time that can be written into a name.
    fn file(&self, key: &Key) -> Option<PathBuf> {
        let since = key.modified.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_nanos();
        // Document 27 line 29's key: the media's identity (path, length, time), its
        // interpretation, and the decoder's version.
        let identity = format!(
            "{}\n{}\n{since}\n{:?}\n{:?}\n{DECODER_VERSION}",
            key.path.to_string_lossy(),
            key.len,
            key.interpretation.color_space,
            key.interpretation.alpha,
        );
        Some(self.folder.join(format!("{}.cel", crate::sha256::hex(identity.as_bytes()))))
    }

    /// The drawing kept in `file`, as [`media::from_8bit`] makes it: `Ok(None)` when there is none,
    /// `Err` with the reason when the file is there but is not a whole, unchanged copy.
    fn load(&self, file: &Path) -> Result<Option<ImageBuffer>, String> {
        let mut f = match std::fs::OpenOptions::new().read(true).write(true).open(file) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("it could not be opened ({e})")),
        };
        let mut head = [0u8; HEADER];
        f.read_exact(&mut head).map_err(|_| "it is shorter than its own header".to_string())?;
        if &head[..8] != MAGIC {
            return Err("it does not begin the way a copy does".into());
        }
        let word = |at: usize| u64::from_le_bytes(head[at..at + 8].try_into().expect("eight bytes"));
        let (w, h, sum) = (word(8) as usize, word(16) as usize, word(24));
        // Checked against the file's length before anything is allocated, so a damaged header
        // cannot ask for more memory than the file holds.
        let bytes = w.checked_mul(h).and_then(|n| n.checked_mul(4));
        let length = f.metadata().map_err(|e| format!("its length could not be read ({e})"))?.len();
        let whole = bytes.and_then(|n| n.checked_add(HEADER)).map(|n| n as u64);
        if whole != Some(length) {
            return Err(format!("it holds {length} bytes, not the {} a {w}x{h} copy holds", whole.unwrap_or(0)));
        }
        let mut rgba = vec![0u8; bytes.expect("checked above")];
        f.read_exact(&mut rgba).map_err(|e| format!("it could not be read to the end ({e})"))?;
        if checksum(&rgba) != sum {
            return Err("its pixels do not match the checksum written with them".into());
        }
        // Least recently used is judged by this time, so reading a copy counts as using it.
        let _ = f.set_modified(SystemTime::now());
        media::from_8bit(file, w, h, &rgba).map(Some).map_err(|d| format!("its header is not a picture's ({})", d.detail))
    }

    /// Keep `buffer` in `file`.
    fn store(&self, file: &Path, w: usize, h: usize, rgba: &[u8]) -> std::io::Result<()> {
        let mut head = [0u8; HEADER];
        head[..8].copy_from_slice(MAGIC);
        head[8..16].copy_from_slice(&(w as u64).to_le_bytes());
        head[16..24].copy_from_slice(&(h as u64).to_le_bytes());
        head[24..].copy_from_slice(&checksum(rgba).to_le_bytes());
        self.put(file, &[&head, rgba])
    }

    /// B-171 (G12): the file the composition frame kept under `key` is in.
    fn frame_file(&self, key: &str) -> PathBuf {
        self.folder.join(format!("{}.frame", crate::sha256::hex(key.as_bytes())))
    }

    /// B-171: the composition frame kept in `file` under `key`, and the files it was drawn from, as
    /// [`load`](Self::load) reads a cel: `Ok(None)` when there is none, `Err` with the reason when
    /// the file is there but is not a whole copy of that frame. The key is kept whole in the file
    /// and compared, so two keys whose names happened to match could not be mistaken for each other.
    fn load_frame(&self, file: &Path, key: &str) -> Result<Option<(WorkingBuffer, Vec<Stamp>)>, String> {
        let mut f = match std::fs::OpenOptions::new().read(true).write(true).open(file) {
            Ok(f) => f,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(format!("it could not be opened ({e})")),
        };
        let mut head = [0u8; FRAME_HEADER];
        f.read_exact(&mut head).map_err(|_| "it is shorter than its own header".to_string())?;
        if &head[..8] != FRAME_MAGIC {
            return Err("it does not begin the way a copy does".into());
        }
        let word = |at: usize| u64::from_le_bytes(head[at..at + 8].try_into().expect("eight bytes"));
        let (w, h, text, text_sum, pixel_sum) = (word(8) as usize, word(16) as usize, word(24) as usize, word(32), word(40));
        // Checked against the file's length before anything is allocated, as a cel's is.
        let length = f.metadata().map_err(|e| format!("its length could not be read ({e})"))?.len();
        let whole = w
            .checked_mul(h)
            .and_then(|n| n.checked_mul(16))
            .and_then(|n| n.checked_add(FRAME_HEADER))
            .and_then(|n| n.checked_add(text))
            .map(|n| n as u64);
        if whole != Some(length) {
            return Err(format!("it holds {length} bytes, not the {} a {w}x{h} frame holds", whole.unwrap_or(0)));
        }
        let mut words = vec![0u8; text];
        f.read_exact(&mut words).map_err(|e| format!("it could not be read to the end ({e})"))?;
        let mut picture = WorkingBuffer::transparent(w, h);
        f.read_exact(bytemuck::cast_slice_mut(picture.data_mut()))
            .map_err(|e| format!("it could not be read to the end ({e})"))?;
        if checksum(&words) != text_sum || checksum(bytemuck::cast_slice(picture.data())) != pixel_sum {
            return Err("its contents do not match the checksums written with them".into());
        }
        let words = String::from_utf8(words).map_err(|_| "its text is not text".to_string())?;
        let (kept, stamps) = words.split_once('\0').ok_or("its text has no end to its key")?;
        if kept != key {
            return Err("it was kept for another frame".into());
        }
        let files = stamps_of(stamps).ok_or("its list of files cannot be read")?;
        let _ = f.set_modified(SystemTime::now());
        Ok(Some((picture, files)))
    }

    /// B-171: keep `picture`, drawn from `files`, in `file` under `key`. Its floats are written as
    /// they are in memory, which is little-endian on every machine this program runs on.
    fn store_frame(&self, file: &Path, key: &str, files: &str, picture: &WorkingBuffer) -> std::io::Result<()> {
        let text = format!("{key}\0{files}");
        let pixels: &[u8] = bytemuck::cast_slice(picture.data());
        let mut head = [0u8; FRAME_HEADER];
        head[..8].copy_from_slice(FRAME_MAGIC);
        for (at, word) in [picture.width() as u64, picture.height() as u64, text.len() as u64, checksum(text.as_bytes()), checksum(pixels)]
            .into_iter()
            .enumerate()
        {
            head[8 + at * 8..16 + at * 8].copy_from_slice(&word.to_le_bytes());
        }
        self.put(file, &[&head, text.as_bytes(), pixels])
    }

    /// Write `parts` to `file`, whole or not at all: written under another name and renamed, so a
    /// copy that is half written is never read as one.
    fn put(&self, file: &Path, parts: &[&[u8]]) -> std::io::Result<()> {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        std::fs::create_dir_all(&self.folder)?;
        let part = file.with_extension(format!(
            "{}-{}.part",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let written = (|| {
            let mut out = std::fs::File::create(&part)?;
            for bytes in parts {
                out.write_all(bytes)?;
            }
            drop(out);
            std::fs::rename(&part, file)
        })();
        if written.is_err() {
            let _ = std::fs::remove_file(&part);
        }
        written?;
        self.evict();
        Ok(())
    }

    /// The copies in the folder, oldest use first, with their lengths. A `.part` more than an
    /// hour old is what a closed or crashed window left half written, and is deleted on the way.
    fn copies(&self) -> Vec<(SystemTime, u64, PathBuf)> {
        let Ok(list) = std::fs::read_dir(&self.folder) else { return Vec::new() };
        let mut copies = Vec::new();
        for entry in list.flatten() {
            let path = entry.path();
            let Ok(meta) = entry.metadata() else { continue };
            let time = meta.modified().unwrap_or(SystemTime::UNIX_EPOCH);
            match path.extension().and_then(|e| e.to_str()) {
                Some("cel" | "frame") => copies.push((time, meta.len(), path)),
                Some("part") if time.elapsed().is_ok_and(|age| age > Duration::from_secs(3600)) => {
                    let _ = std::fs::remove_file(&path);
                }
                _ => {}
            }
        }
        copies.sort();
        copies
    }

    /// Delete the least recently used copies until what is left fits the cap.
    ///
    /// ponytail: the folder is listed on every store, which is a few hundred entries against an
    /// 8 MB write; keep a running total if a folder of many thousands ever makes it show.
    fn evict(&self) {
        let copies = self.copies();
        let mut total: u64 = copies.iter().map(|(_, len, _)| len).sum();
        for (_, len, path) in copies {
            if total <= self.cap {
                break;
            }
            if std::fs::remove_file(&path).is_ok() {
                total -= len;
            }
        }
    }

    /// How many bytes of copies the folder holds now.
    pub fn held(&self) -> u64 {
        self.copies().iter().map(|(_, len, _)| len).sum()
    }
}

/// FNV-1a over eight bytes at a time, a slice of the pool at a time and then over the slices'
/// results in order, so the answer does not depend on how many threads there are. Each step
/// multiplies by an odd number, which cannot map two values to one, so any one changed byte
/// changes the sum.
fn checksum(bytes: &[u8]) -> u64 {
    let fnv = |h: u64, v: u64| (h ^ v).wrapping_mul(0x0000_0100_0000_01b3);
    let parts: Vec<u64> = bytes
        .par_chunks(1 << 18)
        .map(|chunk| {
            let words = chunk.chunks_exact(8);
            let tail = words.remainder().iter().fold(0, |h, b| h << 8 | u64::from(*b));
            let h = words.fold(0xcbf2_9ce4_8422_2325, |h, w| fnv(h, u64::from_le_bytes(w.try_into().expect("eight"))));
            fnv(h, tail)
        })
        .collect();
    parts.into_iter().fold(0xcbf2_9ce4_8422_2325, fnv)
}

/// B-161: the cel at `path` in the working space, from `disk` when a good copy is there, decoded
/// when not, and written there after a decode. The flag is whether the disk answered. A copy that
/// fails its checks, or cannot be written, adds a line to `notes` for the session log (document
/// 28: the preview's pixels are unchanged either way, so it is a note and not a warning).
fn working_cel(
    disk: Option<&DiskCache>,
    key: Option<&Key>,
    path: &Path,
    interpretation: Interpretation,
    notes: &mut Vec<String>,
) -> Result<(WorkingBuffer, bool), Diagnostic> {
    let name = path.file_name().map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into_owned());
    let working = |image: ImageBuffer| retag(image, interpretation).into_working();
    // An EXR decodes straight to floats and has no 8-bit bytes to keep.
    let copy = disk
        .zip(key)
        .filter(|_| !crate::exr_io::is_exr(&path.to_string_lossy()))
        .and_then(|(disk, key)| Some((disk, disk.file(key)?)));
    if let Some((disk, file)) = &copy {
        match crate::perf::time(crate::perf::Stage::FileRead, || disk.load(file)) {
            Ok(Some(image)) => return Ok((working(image), true)),
            Ok(None) => {}
            Err(reason) => {
                let _ = std::fs::remove_file(file);
                notes.push(format!(
                    "DECODE_CACHE_DISCARDED: the disk copy of {name} was deleted and the drawing decoded again, because {reason}."
                ));
            }
        }
    }
    // `media::decode` is these two steps; they are taken apart here to keep the bytes between.
    let image = match media::decode_8bit(path) {
        None => media::decode(path)?,
        Some(pixels) => {
            let (w, h, rgba) = pixels?;
            if let Some((disk, file)) = &copy {
                if let Err(e) = disk.store(file, w, h, &rgba) {
                    notes.push(format!(
                        "DECODE_CACHE_NOT_WRITTEN: {name} was decoded but could not be kept on disk ({e}); it will be decoded again next time."
                    ));
                }
            }
            media::from_8bit(path, w, h, &rgba)?
        }
    };
    Ok((working(image), false))
}

/// What makes two requests for a decoded cel the same request.
#[derive(Clone, PartialEq, Eq, Debug)]
struct Key {
    path: PathBuf,
    len: u64,
    modified: SystemTime,
    interpretation: Interpretation,
}

impl Key {
    /// `None` when the file's metadata cannot be read, which makes the request uncacheable rather
    /// than an error: the decode that follows will report the problem properly if there is one.
    fn of(path: &Path, interpretation: Interpretation) -> Option<Key> {
        let meta = looked_at(path).ok()?;
        Some(Key {
            path: path.to_path_buf(),
            len: meta.len(),
            modified: meta.modified().ok()?,
            interpretation,
        })
    }
}

/// What makes two evaluations of an effect stack the same evaluation (P-11).
///
/// ADR-017 runs the stack whole-layer, on the cel's own pixels, before the frame plan exists, so
/// an evaluation's whole input is the decoded cel, the masks that were drawn into it first, and
/// the stack itself. All three are held here **by value rather than as a hash**: document 27 line 29
/// requires every input to be in the key, and a comparison of the inputs themselves cannot
/// collide the way a digest of them can. A mask is a handful of points and a stack is a handful
/// of parameters, so this costs nothing next to the 33 MB it protects.
///
/// The cel half is the same [`Key`] the decoded cel is held under - path, length, modification
/// time and interpretation - so a cel file that changes on disk invalidates the effect result
/// computed from it by the same rule that invalidates the cel.
#[derive(Clone, PartialEq, Debug)]
struct EffectKey {
    cel: Key,
    /// Empty when the layer has no mask, and holding only the masks that could be drawn: one
    /// that cannot writes nothing into the cel, so it cannot change what the stack reads.
    masks: Vec<Mask>,
    effects: Vec<EffectInstance>,
    /// D-99: 1 for a full-size cel, the draft divisor for a cel taken down to draft size before
    /// its stack ran. Without it a stack with no distance in it and no mask - an exposure alone -
    /// would find the full-size result of the same cel at draft, a buffer four times too large.
    divisor: usize,
}

/// What a buffer this cache holds is, for a store that keeps its own copy after this cache lets
/// go (B-44b: the graphics card's). It is the key the cache holds the buffer under, so the same
/// file read again, or the same stack run again, has the same name and the card need not be sent
/// it again. A buffer the cache does not hold - a cel a mask was drawn into - has none.
#[derive(Clone, PartialEq, Debug)]
pub struct Name(Named);

#[derive(Clone, PartialEq, Debug)]
enum Named {
    Cel(Key),
    Effect(EffectKey),
}

/// What one evaluation of an effect stack produced, in full.
///
/// The buffer and the offset are what the renderer needs. `bypassed` is what document 28 needs:
/// an effect this build cannot run raises a warning **per frame**, and a frame served from this
/// cache has to raise the same warnings as the frame that filled it, or the cache would have
/// changed the diagnostics - which document 27 forbids in the same sentence that allows caching
/// at all. The entries are positions in the stack, so replaying one names the same instance.
#[derive(Clone)]
pub struct EffectResult {
    pub buffer: Arc<WorkingBuffer>,
    pub offset: (usize, usize),
    pub bypassed: Vec<(usize, Bypassed)>,
}

/// A bounded, least-recently-used cache of decoded cels in the working space.
///
/// Constructed either [`with_budget`](Self::with_budget) or as [`none`](Self::none), which is a
/// cache that can never hold anything and is what every path outside the preview uses.
pub struct CelCache {
    budget: usize,
    held: usize,
    /// Least recently used first. ponytail: a linear scan, because this holds tens of entries and
    /// a frame asks it four questions; a map plus an intrusive list if a real project ever makes
    /// the scan measurable.
    entries: Vec<(Key, Arc<WorkingBuffer>)>,
    /// Cels [`prewarm`](Self::prewarm) decoded ahead of the layer loop and that no request has
    /// collected yet (P-03(b)). Not part of `held`: a pending cel has not been admitted, and
    /// [`decoded`](Self::decoded) admits it through [`store`](Self::store) like any other miss,
    /// in the order the layer loop asks for it, so the budget, the eviction order and the hit
    /// and miss counts are exactly what they were when every decode was serial.
    pending: Vec<(Key, Arc<WorkingBuffer>)>,
    hits: u64,
    misses: u64,
    evicted: u64,
    /// The evaluated effect results (P-11), with a budget of their own rather than a share of the
    /// cel list's. `verification/P-09_effect_reuse.md` is the reason they are not one list: the
    /// request stream cycles, least-recently-used is at its worst against a cycle, and effect
    /// results competing with cels in one list would be evicted exactly before they are wanted.
    effect_budget: usize,
    effect_held: usize,
    /// With when each was last used, counted in [`tick`](Self::tick)s.
    effect_entries: Vec<(EffectKey, EffectResult, u64)>,
    /// B-171 (G12, D-243): frames of compositions shown inside another, as the viewer drew them,
    /// with the files each was drawn from and when it was last used. Least recently used first.
    /// Held under the effect budget beside the effect results, and let go with them in the order
    /// the two were last used, so neither kind can push out the other's newest.
    inner: Vec<(String, Vec<Stamp>, Arc<WorkingBuffer>, u64)>,
    /// A count of uses, the clock the two lists above are let go by.
    tick: u64,
    effect_hits: u64,
    effect_misses: u64,
    effect_evicted: u64,
    /// B-161: where decoded cels are also kept on disk, for the viewer only. `None` everywhere
    /// else, and always in [`none`](Self::none), so export decodes every drawing itself.
    disk: Option<DiskCache>,
    /// Misses the disk copy answered instead of a decode.
    disk_hits: u64,
    /// Session-log lines about disk copies, waiting for the window to collect them.
    disk_notes: Vec<String>,
    /// B-171b: the same, from the worker writing composition frames.
    late_notes: Arc<Mutex<Vec<String>>>,
    /// B-159 (G10): the frame last drawn, and the picture below its last edit.
    // ponytail: one picture the frame's size, outside the budget; count it if a budget is tight.
    below: (Option<(Id, i32)>, crate::render::Below),
}

impl CelCache {
    /// A cache that may hold up to `budget` bytes of decoded cels.
    pub fn with_budget(budget: usize) -> CelCache {
        CelCache::with_budgets(budget, 0)
    }

    /// A cache that may hold `cels` bytes of decoded cels and `effects` bytes of evaluated effect
    /// results (P-11).
    ///
    /// Two budgets and not one, for the reason `verification/P-09_effect_reuse.md` measured: a
    /// single least-recently-used list holding both would let the cel stream, which cycles
    /// through far more distinct drawings than any budget holds, evict the effect results just
    /// before they are asked for again. The viewer splits D-40's gibibyte between the two; every
    /// other caller passes zero for the second and gets exactly the cache it had before P-11.
    pub fn with_budgets(cels: usize, effects: usize) -> CelCache {
        CelCache {
            budget: cels,
            held: 0,
            entries: Vec::new(),
            pending: Vec::new(),
            hits: 0,
            misses: 0,
            evicted: 0,
            effect_budget: effects,
            effect_held: 0,
            effect_entries: Vec::new(),
            inner: Vec::new(),
            tick: 0,
            effect_hits: 0,
            effect_misses: 0,
            effect_evicted: 0,
            disk: None,
            disk_hits: 0,
            disk_notes: Vec::new(),
            late_notes: Default::default(),
            below: Default::default(),
        }
    }

    /// B-161: keep decoded cels on disk as well, or stop (`None`). The viewer's cache only.
    pub fn set_disk(&mut self, disk: Option<DiskCache>) {
        self.disk = disk;
    }

    /// The cache the viewer runs with: D-40's gibibyte, split between decoded cels and evaluated
    /// effect results (P-11).
    ///
    /// **The total is unchanged.** [`DEFAULT_EFFECT_BUDGET_BYTES`] is taken out of
    /// [`DEFAULT_BUDGET_BYTES`], not added to it, so the window holds what document 40 said it
    /// holds and no more. This is one function rather than the split written out at each of the
    /// three places that need it - the window, P-01's first-playthrough harness and P-03's
    /// byte-equality proof - so that "what the viewer holds" has one definition to change.
    pub fn viewer() -> CelCache {
        CelCache::viewer_sized(DEFAULT_BUDGET_BYTES)
    }

    /// D-105: the viewer's cache holding `total` bytes, split as D-40's gibibyte is: seven
    /// sixteenths for effect results, which is [`DEFAULT_EFFECT_BUDGET_BYTES`] of a gibibyte.
    pub fn viewer_sized(total: usize) -> CelCache {
        CelCache::with_budgets(total - total / 16 * 7, total / 16 * 7)
    }

    /// D-105: give a cache the viewer is already using a new total, split as
    /// [`viewer_sized`](Self::viewer_sized) splits it. What no longer fits is let go, oldest first.
    pub fn resize(&mut self, total: usize) {
        self.budget = total - total / 16 * 7;
        self.effect_budget = total / 16 * 7;
        while self.held > self.budget {
            let (_, evicted) = self.entries.remove(0);
            self.held -= bytes_of(&evicted);
            self.evicted += 1;
        }
        self.trim_effects();
    }

    /// Let go of effect results and composition frames, least recently used first, until what is
    /// held fits the effect budget.
    fn trim_effects(&mut self) {
        while self.effect_held > self.effect_budget {
            let inner_older = match (self.inner.first(), self.effect_entries.first()) {
                (Some(inner), Some(effect)) => inner.3 < effect.2,
                (Some(_), None) => true,
                (None, Some(_)) => false,
                (None, None) => break,
            };
            self.effect_held -= if inner_older {
                bytes_of(&self.inner.remove(0).2)
            } else {
                self.effect_evicted += 1;
                bytes_of(&self.effect_entries.remove(0).1.buffer)
            };
        }
    }

    /// B-171 (G12, D-243): the frame of a composition shown inside another kept under `key`, from
    /// memory or from the disk folder, while every file it was drawn from is as it was. The
    /// viewer's alone: `None` without an effect budget, so an export draws every inner frame
    /// itself (ADR-015).
    pub fn inner_frame(&mut self, key: &str) -> Option<Arc<WorkingBuffer>> {
        if self.effect_budget == 0 {
            return None;
        }
        self.tick += 1;
        if let Some(at) = self.inner.iter().position(|(k, ..)| k == key) {
            let mut entry = self.inner.remove(at);
            if !unchanged(&entry.1) {
                self.effect_held -= bytes_of(&entry.2);
                return None;
            }
            entry.3 = self.tick;
            let picture = Arc::clone(&entry.2);
            self.inner.push(entry);
            return Some(picture);
        }
        let disk = self.disk.as_ref()?;
        let file = disk.frame_file(key);
        written(Some(&file));
        match crate::perf::time(crate::perf::Stage::FileRead, || disk.load_frame(&file, key)) {
            Ok(Some((picture, files))) if unchanged(&files) => {
                let picture = Arc::new(picture);
                self.keep_inner(key.to_string(), files, Arc::clone(&picture));
                Some(picture)
            }
            Ok(Some(_)) => {
                // Drawn from files that have changed since: it can never be right again.
                let _ = std::fs::remove_file(&file);
                None
            }
            Ok(None) => None,
            Err(reason) => {
                let _ = std::fs::remove_file(&file);
                self.disk_notes.push(format!(
                    "FRAME_CACHE_DISCARDED: the disk copy of a composition frame was deleted and the frame drawn again, because {reason}."
                ));
                None
            }
        }
    }

    /// B-171: keep a composition frame the viewer drew from `files` in `took`: in memory, and in
    /// the disk folder too when drawing it took longer than reading it back would
    /// ([`DISK_BYTES_PER_MS`]). A frame that read a file with no modification time is not kept.
    /// B-171b: the disk copy is written by a worker, and this returns before it is.
    pub fn store_inner(&mut self, key: String, files: Vec<Stamp>, picture: Arc<WorkingBuffer>, took: Duration) {
        if self.effect_budget == 0 || !files.iter().all(|(_, was)| was.is_none_or(|(_, modified)| modified.is_some())) {
            return;
        }
        let heavy = took.as_secs_f64() * 1000.0 * DISK_BYTES_PER_MS >= bytes_of(&picture) as f64;
        if let (Some(disk), true, Some(text)) = (&self.disk, heavy, stamps_text(&files)) {
            let (w, file) = (writer(), disk.frame_file(&key));
            w.pending.lock().unwrap_or_else(|p| p.into_inner()).push(file.clone());
            let job = (disk.clone(), file.clone(), key.clone(), text, Arc::clone(&picture), Arc::clone(&self.late_notes));
            if w.jobs.try_send(job).is_err() {
                // The disk is behind: this frame is kept in memory only, rather than the frame
                // waiting for the disk.
                w.finished(&file);
            }
        }
        self.keep_inner(key, files, picture);
    }

    fn keep_inner(&mut self, key: String, files: Vec<Stamp>, picture: Arc<WorkingBuffer>) {
        let bytes = bytes_of(&picture);
        if bytes > self.effect_budget {
            return;
        }
        self.tick += 1;
        self.inner.push((key, files, picture, self.tick));
        self.effect_held += bytes;
        self.trim_effects();
    }

    /// A cache that holds nothing, ever. Export and every non-preview caller use this.
    ///
    /// It is a real cache with a zero budget rather than an `Option<&mut CelCache>` at every call
    /// site, so there is exactly one code path through [`decoded`](Self::decoded) and the "no
    /// cache" case is exercised by every test that renders a frame.
    pub fn none() -> CelCache {
        CelCache::with_budget(0)
    }

    /// The cel at `path`, tagged as `interpretation` says and converted into the working space.
    ///
    /// Decoded on a miss, remembered if it fits, returned from memory on a hit. The result is the
    /// same buffer either way: this function has one path that produces pixels and one that hands
    /// back pixels it already produced.
    ///
    /// **It is shared, not copied** (P-03(a)). A held cel is 33,177,600 bytes and
    /// `verification/P-01_frame_trace.md` measured copying it at up to 55.6% of a warm frame, for
    /// a copy almost no caller needed: a layer with no mask and no effects only ever reads its
    /// source. A caller that does need to write on it says so with [`Arc::make_mut`], which copies
    /// exactly then and never otherwise — `src/compose.rs` is the one caller that does, and
    /// `tests/b06_mask.rs`'s cache isolation check is what proves the copy really happens rather
    /// than trusting this paragraph.
    pub fn decoded(
        &mut self,
        path: &Path,
        interpretation: Interpretation,
    ) -> Result<Arc<WorkingBuffer>, Diagnostic> {
        let key = Key::of(path, interpretation);

        if let Some(key) = &key {
            let hit = crate::perf::time(crate::perf::Stage::CacheHit, || {
                let at = self.entries.iter().position(|(k, _)| k == key)?;
                let entry = self.entries.remove(at);
                let buffer = Arc::clone(&entry.1);
                self.entries.push(entry);
                Some(buffer)
            });
            if let Some(buffer) = hit {
                self.hits += 1;
                return Ok(buffer);
            }
        }

        self.misses += 1;
        // A cel `prewarm` decoded for this frame is still a miss: it was decoded, just earlier
        // and on another thread. Counting it as a hit would report a cache that answered a
        // request it never held, and `verification/B-08b_cache_table.md` is a table of exactly
        // those counts.
        let ready = key
            .as_ref()
            .and_then(|key| self.pending.iter().position(|(k, _)| k == key))
            .map(|at| self.pending.remove(at).1);
        let buffer = match ready {
            Some(buffer) => buffer,
            None => {
                let (buffer, from_disk) =
                    working_cel(self.disk.as_ref(), key.as_ref(), path, interpretation, &mut self.disk_notes)?;
                self.disk_hits += u64::from(from_disk);
                Arc::new(buffer)
            }
        };
        if let Some(key) = key {
            crate::perf::time(crate::perf::Stage::CacheStore, || {
                self.store(key, Arc::clone(&buffer))
            });
        }
        Ok(buffer)
    }

    /// Decode the cels this frame is about to ask for, all at once, across the thread pool
    /// (P-03(b)).
    ///
    /// `verification/P-01_frame_trace.md` measured the read, the byte-to-float pass and the
    /// transfer function at three quarters of a cold draft frame, and a frame's cels are
    /// independent files: nothing about decoding one depends on another. What forbade doing them
    /// together was this `&mut CelCache`, threaded through `plan_frame_cached`'s layer loop, which
    /// makes the loop the only place a decode can happen. This moves the decode ahead of the loop
    /// and leaves the loop's shape alone.
    ///
    /// Three things it deliberately does not do.
    ///
    /// **It does not decide anything.** Every cel it decodes is one the loop was going to ask
    /// for; a key it fails on, or never had metadata for, is simply absent from `pending` and
    /// decodes serially in [`decoded`](Self::decoded) a moment later, where the diagnostic is
    /// raised and logged against the right layer exactly as before. That is why P-03(b)'s
    /// requirement to collect a `FrameLog` per layer and merge it in composition order does not
    /// appear here: no diagnostic is raised on a worker thread, so there is no order to restore.
    ///
    /// **It does not admit anything.** Nothing here touches `held`, the eviction order, or the
    /// hit and miss counts.
    ///
    /// **It does nothing at all without a budget.** [`none`](Self::none) is what export and every
    /// non-preview caller hold, and ADR-015 keeps the cache off that path; decoding in advance
    /// for a cache that cannot store would be work thrown away twice over.
    ///
    /// ponytail: the whole `wanted` set is decoded in one fan-out, so the transient peak is one
    /// frame's cels held at once. That is the frame's own working set, which is what the budget
    /// is chosen to hold; chunk it if a composition ever has more layers than the budget has
    /// room for.
    pub fn prewarm(&mut self, wanted: &[(PathBuf, Interpretation)]) {
        if self.budget == 0 || wanted.is_empty() {
            return;
        }
        let keys: Vec<Key> = wanted
            .iter()
            .filter_map(|(path, interpretation)| Key::of(path, *interpretation))
            .fold(Vec::new(), |mut keys, key| {
                // A matte and the layer that uses it name the same file: decode it once.
                if !keys.contains(&key) {
                    keys.push(key);
                }
                keys
            });
        let todo: Vec<Key> = keys
            .iter()
            .filter(|key| !self.entries.iter().any(|(k, _)| k == *key))
            .filter(|key| !self.pending.iter().any(|(k, _)| k == *key))
            .cloned()
            .collect();
        if todo.is_empty() {
            // P-18: everything is held or already read ahead. The window asks again for the
            // frame on screen until the clock moves on, and such a request must not throw away
            // what was read ahead for the next one.
            return;
        }
        // P-18: a frame with something new to read. What was read ahead for a frame that is not
        // this one goes, so `pending` never holds more than one frame's cels.
        self.pending.retain(|(k, _)| keys.contains(k));
        let disk = self.disk.as_ref();
        let decoded: Vec<(Key, Arc<WorkingBuffer>, bool, Vec<String>)> =
            crate::perf::time(crate::perf::Stage::Prewarm, || {
                todo.into_par_iter()
                    .filter_map(|key| {
                        // `untimed` because these three stages are running on however many
                        // threads rayon gave them, and adding their core time to the same
                        // counters the frame's wall-clock is measured against would make the
                        // stage table sum to more than the frame.
                        let mut notes = Vec::new();
                        let (buffer, from_disk) = crate::perf::untimed(|| {
                            working_cel(disk, Some(&key), &key.path, key.interpretation, &mut notes).ok()
                        })?;
                        Some((key, Arc::new(buffer), from_disk, notes))
                    })
                    .collect()
            });
        for (key, buffer, from_disk, notes) in decoded {
            self.pending.push((key, buffer));
            self.disk_hits += u64::from(from_disk);
            self.disk_notes.extend(notes);
        }
    }

    /// The result of running `effects` over the cel at `path`, masked by `masks`, if this cache
    /// already has it (P-11).
    ///
    /// `None` on a miss, and `None` whenever the cel's metadata cannot be read, which is the same
    /// condition that makes the cel itself uncacheable: a key that cannot notice its input
    /// changing is worse than no key.
    pub fn effect_result(
        &mut self,
        path: &Path,
        interpretation: Interpretation,
        masks: &[Mask],
        effects: &[EffectInstance],
        divisor: usize,
    ) -> Option<EffectResult> {
        if self.effect_budget == 0 {
            return None;
        }
        let key = self.effect_key(path, interpretation, masks, effects, divisor)?;
        crate::perf::time(crate::perf::Stage::EffectCache, || {
            match self.effect_entries.iter().position(|(k, ..)| *k == key) {
                Some(at) => {
                    let mut entry = self.effect_entries.remove(at);
                    self.tick += 1;
                    entry.2 = self.tick;
                    let result = entry.1.clone();
                    self.effect_entries.push(entry);
                    self.effect_hits += 1;
                    Some(result)
                }
                None => {
                    self.effect_misses += 1;
                    None
                }
            }
        })
    }

    /// Remember what an effect stack evaluated to (P-11). A no-op without an effect budget, which
    /// is every caller but the viewer, and a no-op for a cel whose metadata could not be read.
    pub fn store_effect(
        &mut self,
        path: &Path,
        interpretation: Interpretation,
        masks: &[Mask],
        effects: &[EffectInstance],
        divisor: usize,
        result: EffectResult,
    ) {
        if self.effect_budget == 0 {
            return;
        }
        let Some(key) = self.effect_key(path, interpretation, masks, effects, divisor) else {
            return;
        };
        crate::perf::time(crate::perf::Stage::EffectCache, || {
            let bytes = bytes_of(&result.buffer);
            if bytes > self.effect_budget {
                return;
            }
            self.tick += 1;
            self.effect_entries.push((key, result, self.tick));
            self.effect_held += bytes;
            self.trim_effects();
        });
    }

    fn effect_key(
        &self,
        path: &Path,
        interpretation: Interpretation,
        masks: &[Mask],
        effects: &[EffectInstance],
        divisor: usize,
    ) -> Option<EffectKey> {
        Some(EffectKey {
            cel: Key::of(path, interpretation)?,
            masks: masks.to_vec(),
            effects: effects.to_vec(),
            divisor,
        })
    }

    /// How many effect-stack evaluations were answered from memory (P-11).
    pub fn effect_hits(&self) -> u64 {
        self.effect_hits
    }

    /// How many effect-stack evaluations had to run.
    pub fn effect_misses(&self) -> u64 {
        self.effect_misses
    }

    /// How many held effect results were dropped to stay inside the effect budget.
    pub fn effect_evictions(&self) -> u64 {
        self.effect_evicted
    }

    /// How many bytes of evaluated effect results are held right now.
    pub fn effect_held_bytes(&self) -> usize {
        self.effect_held
    }

    /// The effect-result budget this cache was built with.
    pub fn effect_budget(&self) -> usize {
        self.effect_budget
    }

    fn store(&mut self, key: Key, buffer: Arc<WorkingBuffer>) {
        let bytes = bytes_of(&buffer);
        // A cel larger than the whole budget is not stored at all. Evicting everything to hold
        // one thing that will be evicted by the next request is worse than not holding it.
        if bytes > self.budget {
            return;
        }
        self.entries.push((key, buffer));
        self.held += bytes;
        while self.held > self.budget {
            let (_, evicted) = self.entries.remove(0);
            self.held -= bytes_of(&evicted);
            self.evicted += 1;
        }
    }

    /// The name this cache holds `buffer` under, if it holds it (B-44b).
    pub fn name_of(&self, buffer: &Arc<WorkingBuffer>) -> Option<Name> {
        let cel = self.entries.iter().find(|(_, b)| Arc::ptr_eq(b, buffer));
        if let Some((key, _)) = cel {
            return Some(Name(Named::Cel(key.clone())));
        }
        self.effect_entries
            .iter()
            .find(|(_, r, _)| Arc::ptr_eq(&r.buffer, buffer))
            .map(|(key, ..)| Name(Named::Effect(key.clone())))
    }

    /// How many requests were answered from memory.
    pub fn hits(&self) -> u64 {
        self.hits
    }

    /// How many requests had to decode.
    pub fn misses(&self) -> u64 {
        self.misses
    }

    /// B-161: how many decodes a disk copy answered instead, read ahead or not.
    pub fn disk_hits(&self) -> u64 {
        self.disk_hits
    }

    /// B-159 (G10): what is kept below the last edit of `frame` of `composition`, forgotten when
    /// the frame asked for is another. `None` without an effect budget, which is every cache but
    /// the viewer's, so an export never starts from anything kept (ADR-015).
    pub fn below(&mut self, composition: &Id, frame: i32) -> Option<&mut crate::render::Below> {
        if self.effect_budget == 0 {
            return None;
        }
        if self.below.0.as_ref().is_none_or(|(c, f)| c != composition || *f != frame) {
            self.below.0 = Some((composition.clone(), frame));
            self.below.1.forget();
        }
        Some(&mut self.below.1)
    }

    /// B-159 (G10): how many frames were started from the layers below an edit as kept.
    pub fn below_reused(&self) -> u64 {
        self.below.1.reused()
    }

    /// B-161: the session-log lines about disk copies since the last call, emptied by it.
    pub fn take_disk_notes(&mut self) -> Vec<String> {
        let late = std::mem::take(&mut *self.late_notes.lock().unwrap_or_else(|p| p.into_inner()));
        self.disk_notes.extend(late);
        std::mem::take(&mut self.disk_notes)
    }

    /// How many held cels were dropped to stay inside the budget.
    pub fn evictions(&self) -> u64 {
        self.evicted
    }

    /// How many bytes of decoded cels are held right now. Never more than the budget.
    pub fn held_bytes(&self) -> usize {
        self.held
    }

    /// How many cels are held right now.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The budget this cache was built with.
    pub fn budget(&self) -> usize {
        self.budget
    }
}

/// What one decoded cel costs to hold: its samples, four bytes each.
fn bytes_of(buffer: &WorkingBuffer) -> usize {
    std::mem::size_of_val(buffer.as_image().data())
}

/// B-154 (G3): one file a frame looked at, as it was when it looked - its length and modification
/// time, or `None` when it was not there. Document 27's media identity, the same two facts [`Key`]
/// holds, taken for every file and not only the cels, because a finished frame also depends on the
/// files it found missing and the LUTs it read.
pub type Stamp = (PathBuf, Option<(u64, Option<SystemTime>)>);

thread_local! {
    /// The files looked at on this thread since [`files_read`] began, or `None` outside it.
    static LOOKED: std::cell::RefCell<Option<Vec<Stamp>>> = const { std::cell::RefCell::new(None) };
}

fn stamp(meta: &std::io::Result<std::fs::Metadata>) -> Option<(u64, Option<SystemTime>)> {
    meta.as_ref().ok().map(|m| (m.len(), m.modified().ok()))
}

/// `std::fs::metadata`, noted down for [`files_read`]. Every question the render path asks the
/// disk goes through here, so a finished frame knows every file it came from. Reading the file
/// itself is not asked separately: each read is preceded by one of these.
pub fn looked_at(path: &Path) -> std::io::Result<std::fs::Metadata> {
    let meta = std::fs::metadata(path);
    LOOKED.with(|l| {
        if let Some(list) = l.borrow_mut().as_mut() {
            list.push((path.to_path_buf(), stamp(&meta)));
        }
    });
    meta
}

/// Run `f` and say which files it looked at on this thread. Work done on other threads is not
/// seen, which is right for the read-ahead and holds for the render because every
/// [`looked_at`] in it runs on the calling thread (the parallel loops are over pixels).
pub fn files_read<T>(f: impl FnOnce() -> T) -> (T, Vec<Stamp>) {
    let outer = LOOKED.with(|l| l.replace(Some(Vec::new())));
    let out = f();
    let mut files = LOOKED.with(|l| l.replace(outer)).unwrap_or_default();
    files.sort();
    files.dedup();
    LOOKED.with(|l| {
        if let Some(outer) = l.borrow_mut().as_mut() {
            outer.extend(files.iter().cloned());
        }
    });
    (out, files)
}

/// Whether every file is as it was. A file with no modification time can never be trusted to be
/// unchanged, so a frame that read one is never kept. Each file is looked at through
/// [`looked_at`], so a frame that uses something kept still knows every file it came from (B-171).
pub fn unchanged(files: &[Stamp]) -> bool {
    files.iter().all(|(path, was)| {
        was.is_none_or(|(_, modified)| modified.is_some()) && stamp(&looked_at(path)) == *was
    })
}

/// B-171: `files` as lines of text, "length nanoseconds path", or "- - path" for a file that was
/// not there. `None` for a time before 1970 or a path that is not text, which is kept in memory
/// only.
fn stamps_text(files: &[Stamp]) -> Option<String> {
    let mut text = String::new();
    for (path, was) in files {
        let path = path.to_str().filter(|p| !p.contains('\n'))?;
        match was {
            None => text.push_str(&format!("- - {path}\n")),
            Some((len, modified)) => {
                let since = modified.as_ref()?.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_nanos();
                text.push_str(&format!("{len} {since} {path}\n"));
            }
        }
    }
    Some(text)
}

/// [`stamps_text`] read back.
fn stamps_of(text: &str) -> Option<Vec<Stamp>> {
    text.lines()
        .map(|line| {
            let mut words = line.splitn(3, ' ');
            let (len, since, path) = (words.next()?, words.next()?, words.next()?);
            let was = match len {
                "-" => None,
                _ => Some((len.parse().ok()?, Some(SystemTime::UNIX_EPOCH + Duration::from_nanos(since.parse().ok()?)))),
            };
            Some((PathBuf::from(path), was))
        })
        .collect()
}

/// B-171: this program, as its file on disk: its path, length and time. Every composition frame
/// kept on disk is filed under it, so a frame drawn by another build of the program is never read.
pub fn build() -> Option<&'static str> {
    static BUILD: std::sync::OnceLock<Option<String>> = std::sync::OnceLock::new();
    BUILD
        .get_or_init(|| {
            let exe = std::env::current_exe().ok()?;
            let meta = std::fs::metadata(&exe).ok()?;
            let since = meta.modified().ok()?.duration_since(SystemTime::UNIX_EPOCH).ok()?.as_nanos();
            Some(format!("{}\n{}\n{since}", exe.display(), meta.len()))
        })
        .as_deref()
}

/// B-154: everything a finished preview frame is made from, other than the files it read. Held by
/// value for the reason [`EffectKey`] is: a comparison of the inputs cannot collide. `card` is the
/// card's budget when the card drew it and `None` for the CPU, because the two are allowed to
/// differ by a level and a frame from one must never be sent as the other's.
#[derive(Clone, PartialEq, Debug)]
pub struct Sight {
    pub project: crate::model::Project,
    pub composition: crate::model::Id,
    pub root: PathBuf,
    pub quality: crate::preview::PreviewQuality,
    pub card: Option<usize>,
}

/// A finished frame as the viewer sends it: 8-bit straight sRGB, and what was said making it.
#[derive(Clone, Debug)]
pub struct Finished {
    pub pixels: Arc<Vec<u8>>,
    pub width: usize,
    pub height: usize,
    /// The card was asked and the CPU drew it.
    pub fell_back: bool,
    pub warnings: Vec<String>,
}

/// B-154 (G3, document 33's RAM preview): finished viewer frames, so a loop that has been played
/// once plays from memory. The viewer's alone, like [`CelCache`]: export never holds one
/// (ADR-015). A frame is kept with the [`Stamp`]s of the files it read and is only sent while all
/// of them are unchanged, so an edit (a new [`Sight`]) or a drawing replaced on disk is never
/// answered with the old picture.
pub struct FrameCache {
    budget: usize,
    held: usize,
    /// Least recently used first.
    sights: Vec<(Sight, Vec<(i32, Vec<Stamp>, Finished)>)>,
}

/// ponytail: sights are found by comparing whole projects, one after another; 16 keeps that
/// cheap, and an undo a few steps back still finds its frames. A map keyed on a digest if a
/// person ever needs more.
const MOST_SIGHTS: usize = 16;

impl FrameCache {
    pub fn with_budget(budget: usize) -> FrameCache {
        FrameCache { budget, held: 0, sights: Vec::new() }
    }

    /// A new budget, from the Preferences memory setting. What no longer fits goes, oldest sight
    /// first.
    pub fn resize(&mut self, budget: usize) {
        self.budget = budget;
        while self.held > self.budget && !self.sights.is_empty() {
            self.forget(0);
        }
    }

    fn forget(&mut self, at: usize) {
        let (_, frames) = self.sights.remove(at);
        self.held -= frames.iter().map(|(_, _, f)| f.pixels.len()).sum::<usize>();
    }

    /// The kept frame, if its files are all as they were; a kept frame whose files changed is
    /// dropped. Its sight becomes the most recently used.
    pub fn get(&mut self, sight: &Sight, frame: i32) -> Option<Finished> {
        let s = self.sights.iter().position(|(k, _)| k == sight)?;
        let f = self.sights[s].1.iter().position(|(n, _, _)| *n == frame)?;
        if !unchanged(&self.sights[s].1[f].1) {
            let (_, _, stale) = self.sights[s].1.remove(f);
            self.held -= stale.pixels.len();
            return None;
        }
        let found = self.sights[s].1[f].2.clone();
        let entry = self.sights.remove(s);
        self.sights.push(entry);
        Some(found)
    }

    /// Whether a frame is kept, without looking at the disk: for the render-ahead, which only
    /// needs to know what is still to do.
    pub fn holds(&self, sight: &Sight, frame: i32) -> bool {
        self.sights
            .iter()
            .any(|(k, frames)| k == sight && frames.iter().any(|(n, _, _)| *n == frame))
    }

    /// Keep a frame. Other sights make room, oldest first; this sight's own frames never do,
    /// because in a loop longer than the budget the frame thrown out would be the next one
    /// wanted. `false` when it did not fit, or a file it read has no modified time to check. A file
    /// changed while the frame was being made is not looked for here: [`get`](Self::get) looks at
    /// every file before sending, and looking twice cost the card's first pass (B-154).
    pub fn store(&mut self, sight: Sight, frame: i32, files: Vec<Stamp>, finished: Finished) -> bool {
        if files.iter().any(|(_, was)| was.is_some_and(|(_, modified)| modified.is_none())) {
            return false;
        }
        let s = match self.sights.iter().position(|(k, _)| *k == sight) {
            Some(s) => s,
            None => {
                self.sights.push((sight, Vec::new()));
                self.sights.len() - 1
            }
        };
        let entry = self.sights.remove(s);
        self.sights.push(entry);
        let mine = self.sights.len() - 1;
        if let Some(f) = self.sights[mine].1.iter().position(|(n, _, _)| *n == frame) {
            let (_, _, old) = self.sights[mine].1.remove(f);
            self.held -= old.pixels.len();
        }
        let bytes = finished.pixels.len();
        while (self.held + bytes > self.budget || self.sights.len() > MOST_SIGHTS) && self.sights.len() > 1 {
            self.forget(0);
        }
        if self.held + bytes > self.budget {
            if self.sights[self.sights.len() - 1].1.is_empty() {
                self.sights.pop();
            }
            return false;
        }
        self.held += bytes;
        let mine = self.sights.len() - 1;
        self.sights[mine].1.push((frame, files, finished));
        true
    }

    /// Bytes of pixels held, for the Preferences memory line.
    pub fn held_bytes(&self) -> usize {
        self.held
    }
}

#[cfg(test)]
mod frame_cache {
    use super::*;

    fn sight(quality: crate::preview::PreviewQuality) -> Sight {
        Sight {
            project: crate::model::Project::new(crate::model::Id::new("b154")),
            composition: crate::model::Id::new("comp"),
            root: PathBuf::from("."),
            quality,
            card: None,
        }
    }

    fn finished(bytes: usize) -> Finished {
        Finished { pixels: Arc::new(vec![7; bytes]), width: 1, height: 1, fell_back: false, warnings: Vec::new() }
    }

    #[test]
    fn b154_budget_rule() {
        use crate::preview::PreviewQuality::{Draft, Full};
        let mut c = FrameCache::with_budget(30);
        assert!(c.store(sight(Full), 0, Vec::new(), finished(10)));
        assert!(c.store(sight(Full), 1, Vec::new(), finished(10)));
        // Another sight makes room by forgetting the older one.
        assert!(c.store(sight(Draft), 0, Vec::new(), finished(20)));
        assert!(!c.holds(&sight(Full), 0) && c.holds(&sight(Draft), 0));
        assert_eq!(c.held_bytes(), 20);
        // The sight's own frames are never thrown out for another of its own frames.
        assert!(!c.store(sight(Draft), 1, Vec::new(), finished(20)));
        assert!(c.holds(&sight(Draft), 0));
        // The same frame again replaces what was there.
        assert!(c.store(sight(Draft), 0, Vec::new(), finished(30)));
        assert_eq!(c.held_bytes(), 30);
        // A file that has gone since the frame was made drops the frame.
        let file = std::env::temp_dir().join(format!("b154-budget-{}", std::process::id()));
        std::fs::write(&file, b"x").unwrap();
        let ((), files) = files_read(|| {
            looked_at(&file).unwrap();
        });
        assert_eq!(files.len(), 1);
        assert!(c.store(sight(Full), 5, files, finished(10)));
        assert!(c.get(&sight(Full), 5).is_some());
        std::fs::remove_file(&file).unwrap();
        assert!(c.get(&sight(Full), 5).is_none());
        assert_eq!(c.held_bytes(), 0);
        // A smaller budget lets go of what no longer fits.
        assert!(c.store(sight(Full), 0, Vec::new(), finished(10)));
        c.resize(5);
        assert_eq!(c.held_bytes(), 0);
    }
}
