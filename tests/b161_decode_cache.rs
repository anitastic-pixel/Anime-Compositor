//! B-161, D-232: decoded drawings kept on disk, and read back instead of decoded again.
//!
//! Writes `verification/B-161_decode_cache_table.md` (correctness, every run) and
//! `verification/B-161_decode_cache_timing.md` (the measurement, under `--release --ignored`).
//!
//! The one claim is document 27's: "A cold render and a fully warm render for the same immutable
//! request must produce equivalent pixels and diagnostics." A copy on disk is a warm render that
//! survives closing the window, so every row below asks whether reading the copy changed anything.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build):
//! - **The image counts** are the number of image files under `Fixtures/`, listed by this test
//!   from the folder, and the 56 PNGs of the reference shot its README lists (1 + 24 + 11 + 20).
//! - **"Identical" means every bit of every sample**, compared as `f32::to_bits`, so the expected
//!   value is the whole count and NaN or negative zero cannot hide a difference.
//! - **A second read is answered by the disk** by the definition of a copy: the first read wrote
//!   it, so the number of disk answers on the second pass is the number of files that decode,
//!   less the EXRs. An EXR decodes straight to floats, has no 8-bit bytes to keep, and has none.
//! - **A copy is 32 + width x height x 4 bytes**: a four-word header and the decoded 8-bit RGBA
//!   bytes, before any arithmetic. Three 16x16 copies are 1,056 bytes each, so a cap of 2,112 holds two.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime};

use anime_compositor::cache::{CelCache, DiskCache};
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Interpretation, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, AlphaMode, ColorSpace, WorkingBuffer};

const COMP: &str = "comp-reference-shot";
const IMAGE_EXTENSIONS: [&str; 9] = ["png", "exr", "tga", "webp", "bmp", "tif", "tiff", "jpg", "jpeg"];
/// The reference shot's drawings, from its README: 1 + 24 + 12 - 1 missing + 20.
const REFERENCE_PNGS: usize = 56;
const COPY_16: u64 = 32 + 16 * 16 * 4;

struct Report {
    rows: Vec<(String, String, String)>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push((check.to_string(), expected.to_string(), actual.to_string()));
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// A folder of its own for each section, empty at the start.
fn scratch(name: &str) -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("b161").join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn disk(folder: &Path, cap: u64) -> DiskCache {
    DiskCache { folder: folder.to_path_buf(), cap }
}

fn with_disk(disk: &DiskCache) -> CelCache {
    let mut cache = CelCache::none();
    cache.set_disk(Some(disk.clone()));
    cache
}

fn bits(buffer: &WorkingBuffer) -> Vec<u32> {
    buffer.data().iter().map(|v| v.to_bits()).collect()
}

fn same(a: &WorkingBuffer, b: &WorkingBuffer) -> bool {
    a.width() == b.width() && a.height() == b.height() && bits(a) == bits(b)
}

fn copies(folder: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = fs::read_dir(folder)
        .map(|list| list.flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "cel")).collect())
        .unwrap_or_default();
    found.sort();
    found
}

fn images(dir: &Path, into: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            images(&path, into);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| IMAGE_EXTENSIONS.contains(&e.to_ascii_lowercase().as_str()))
        {
            into.push(path);
        }
    }
}

fn fixture_images() -> Vec<PathBuf> {
    let mut all = Vec::new();
    images(&repo("Fixtures"), &mut all);
    all.sort();
    all
}

fn interpretation(path: &Path) -> Interpretation {
    Interpretation::for_file(&path.to_string_lossy())
}

fn fresh(path: &Path, interpretation: Interpretation) -> WorkingBuffer {
    (*CelCache::none().decoded(path, interpretation).unwrap()).clone()
}

/// A 16x16 PNG of one colour, for the eviction section, which needs copies small enough to count.
fn write_png(path: &Path, rgba: [u8; 4]) {
    let file = fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), 16, 16);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&rgba.repeat(256)).unwrap();
}

fn project() -> Project {
    let path = repo("verification/B-08a_project.json");
    persist::load(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message)).document.project().clone()
}

#[test]
fn b161_disk_copies_change_nothing() {
    let mut report = Report { rows: Vec::new() };

    // --- Every fixture image: decoded fresh, then written, then read back from the copy. -------
    let all = fixture_images();
    let folder = scratch("every_image");
    let every = disk(&folder, u64::MAX);
    let (mut decoded, mut refused, mut identical, mut first_identical, mut from_disk) = (0, 0, 0, 0, 0);
    let (mut refused_alike, mut exr, mut exr_copies) = (0, 0, 0);
    let mut reference = (0, 0);
    for path in &all {
        let reading = interpretation(path);
        let Ok(plain) = CelCache::none().decoded(path, reading) else {
            refused += 1;
            let before = copies(&folder).len();
            let plain_id = CelCache::none().decoded(path, reading).err().unwrap().id;
            let through = with_disk(&every).decoded(path, reading).err().map(|d| d.id);
            if through == Some(plain_id) && copies(&folder).len() == before {
                refused_alike += 1;
            }
            continue;
        };
        decoded += 1;
        let before = copies(&folder).len();
        let mut writing = with_disk(&every);
        let first = writing.decoded(path, reading).unwrap();
        if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("exr")) {
            exr += 1;
            exr_copies += copies(&folder).len() - before;
        }
        let mut reading_back = with_disk(&every);
        let second = reading_back.decoded(path, reading).unwrap();
        from_disk += reading_back.disk_hits() - writing.disk_hits();
        first_identical += usize::from(same(&plain, &first));
        identical += usize::from(same(&plain, &second));
        if path.starts_with(repo("Fixtures/reference_shot")) {
            reference.0 += 1;
            reference.1 += usize::from(same(&plain, &second));
        }
    }
    report.check(
        "Image files under Fixtures/ that were read (listed from the folder)",
        all.len(),
        decoded + refused,
    );
    report.check(
        "A drawing read back from its disk copy is the fresh decode, every bit of every sample",
        format!("{decoded} of {decoded}"),
        format!("{identical} of {decoded}"),
    );
    report.check(
        "The read that wrote the copy hands back the fresh decode too",
        format!("{decoded} of {decoded}"),
        format!("{first_identical} of {decoded}"),
    );
    report.check(
        "The second read was answered by the disk copy, not by decoding (every image but the EXRs)",
        decoded - exr,
        from_disk,
    );
    report.check(
        "The EXRs among them are decoded every time and no copy is written for them",
        format!("{exr} EXRs, 0 copies"),
        format!("{exr} EXRs, {exr_copies} copies"),
    );
    report.check(
        "The reference shot's 56 drawings, of those: read back bit for bit",
        format!("{REFERENCE_PNGS} of {REFERENCE_PNGS}"),
        format!("{} of {}", reference.1, reference.0),
    );
    report.check(
        "A file the decoder refuses is refused the same way with copies on, and no copy is written for it",
        format!("{refused} of {refused}"),
        format!("{refused_alike} of {refused}"),
    );
    let _ = fs::remove_dir_all(&folder);

    // --- A damaged copy: found out, deleted, decoded fresh, noted, written again. ---------------
    let drawing = repo("Fixtures/reference_shot/layer2/layer2_001.png");
    let reading = interpretation(&drawing);
    let plain = fresh(&drawing, reading);
    let damages: [(&str, fn(&Path)); 5] = [
        ("cut to half its length", |f| {
            let len = fs::metadata(f).unwrap().len();
            fs::OpenOptions::new().write(true).open(f).unwrap().set_len(len / 2).unwrap();
        }),
        ("one byte of one pixel changed", |f| {
            let mut bytes = fs::read(f).unwrap();
            let at = bytes.len() / 2;
            bytes[at] ^= 0x01;
            fs::write(f, bytes).unwrap();
        }),
        ("its first bytes overwritten", |f| {
            let mut bytes = fs::read(f).unwrap();
            bytes[..8].copy_from_slice(b"garbage!");
            fs::write(f, bytes).unwrap();
        }),
        ("emptied to nothing", |f| fs::write(f, b"").unwrap()),
        ("a width in its header that its length disagrees with", |f| {
            let mut bytes = fs::read(f).unwrap();
            bytes[8] = bytes[8].wrapping_add(1);
            fs::write(f, bytes).unwrap();
        }),
    ];
    for (what, damage) in damages {
        let folder = scratch("damaged");
        let cache_disk = disk(&folder, u64::MAX);
        with_disk(&cache_disk).decoded(&drawing, reading).unwrap();
        let copy = copies(&folder).pop().expect("the first read wrote a copy");
        damage(&copy);
        let mut after = with_disk(&cache_disk);
        let result = after.decoded(&drawing, reading);
        let notes = after.take_disk_notes();
        let pixels_same = result.as_ref().is_ok_and(|b| same(&plain, b));
        let noted = notes.len() == 1 && notes[0].starts_with("DECODE_CACHE_DISCARDED: the disk copy of layer2_001.png");
        let mut again = with_disk(&cache_disk);
        let rewritten = again.decoded(&drawing, reading).is_ok_and(|b| same(&plain, &b)) && again.disk_hits() == 1;
        report.check(
            &format!("A copy {what}: decoded fresh with the same bits, one session-log line, written whole again"),
            "decoded, not from disk, 1 line, whole",
            format!(
                "{}, {}, {} line{}, {}",
                if pixels_same { "decoded" } else { "WRONG PIXELS OR ERROR" },
                if after.disk_hits() == 0 { "not from disk" } else { "FROM THE DAMAGED COPY" },
                notes.len(),
                if notes.len() == 1 { "" } else { "s" },
                if rewritten && noted { "whole" } else { "not whole" },
            ),
        );
        let _ = fs::remove_dir_all(&folder);
    }

    // --- What makes two requests the same: the file's length and time, and the interpretation. --
    let folder = scratch("identity");
    let media = scratch("identity_media");
    let cache_disk = disk(&folder, u64::MAX);
    let file = media.join("drawing.png");
    fs::copy(repo("Fixtures/reference_shot/layer2/layer2_001.png"), &file).unwrap();
    with_disk(&cache_disk).decoded(&file, reading).unwrap();
    // Replaced by a different drawing under the same name, as a repaint is.
    fs::copy(repo("Fixtures/reference_shot/layer2/layer2_002.png"), &file).unwrap();
    let later = SystemTime::now() + Duration::from_secs(5);
    fs::File::options().write(true).open(&file).unwrap().set_modified(later).unwrap();
    let mut replaced = with_disk(&cache_disk);
    let new = replaced.decoded(&file, reading).unwrap();
    report.check(
        "A drawing repainted under the same name is decoded again, and the new drawing is what is shown",
        "decoded, the new drawing's bits",
        format!(
            "{}, {}",
            if replaced.disk_hits() == 0 { "decoded" } else { "OLD COPY USED" },
            if same(&fresh(&file, reading), &new) { "the new drawing's bits" } else { "WRONG BITS" }
        ),
    );
    // The same bytes, only a later time: still a different key, as document 27 says.
    let touched = later + Duration::from_secs(5);
    fs::File::options().write(true).open(&file).unwrap().set_modified(touched).unwrap();
    let mut retimed = with_disk(&cache_disk);
    retimed.decoded(&file, reading).unwrap();
    report.check("The same drawing given a later time is decoded again", 0, retimed.disk_hits());
    let linear = Interpretation { color_space: ColorSpace::LinearLight, alpha: AlphaMode::Premultiplied };
    let mut other = with_disk(&cache_disk);
    let as_linear = other.decoded(&file, linear).unwrap();
    let mut other_again = with_disk(&cache_disk);
    let as_linear_again = other_again.decoded(&file, linear).unwrap();
    report.check(
        "The same file read under a second interpretation has a copy of its own, read back bit for bit",
        "decoded, then from disk, same bits as fresh",
        format!(
            "{}, then {}, {}",
            if other.disk_hits() == 0 { "decoded" } else { "SRGB COPY USED" },
            if other_again.disk_hits() == 1 { "from disk" } else { "decoded" },
            if same(&fresh(&file, linear), &as_linear) && same(&as_linear, &as_linear_again) {
                "same bits as fresh"
            } else {
                "WRONG BITS"
            }
        ),
    );
    let _ = fs::remove_dir_all(&folder);
    let _ = fs::remove_dir_all(&media);

    // --- The size limit: least recently used goes first, and nothing that is not a copy. -------
    let folder = scratch("evict");
    let media = scratch("evict_media");
    let small = disk(&folder, 2 * COPY_16);
    let [a, b, c] = ["a.png", "b.png", "c.png"].map(|n| media.join(n));
    write_png(&a, [255, 0, 0, 255]);
    write_png(&b, [0, 255, 0, 255]);
    write_png(&c, [0, 0, 255, 128]);
    let keep = folder.join("not a copy.txt");
    fs::write(&keep, b"left alone").unwrap();
    let step = || std::thread::sleep(Duration::from_millis(30));
    let reading = Interpretation::default();
    with_disk(&small).decoded(&a, reading).unwrap();
    step();
    with_disk(&small).decoded(&b, reading).unwrap();
    step();
    // A is used again, so B is now the one used longest ago.
    let mut a_again = with_disk(&small);
    a_again.decoded(&a, reading).unwrap();
    step();
    with_disk(&small).decoded(&c, reading).unwrap();
    let held: u64 = copies(&folder).iter().map(|p| fs::metadata(p).unwrap().len()).sum();
    report.check("One 16x16 copy is 32 + 16 x 16 x 4 bytes", COPY_16, fs::metadata(&copies(&folder)[0]).unwrap().len());
    report.check("With a cap of two copies, a third leaves two", format!("2 copies, {} bytes", 2 * COPY_16), format!("{} copies, {held} bytes", copies(&folder).len()));
    let hit = |p: &Path| {
        let mut cache = with_disk(&small);
        cache.decoded(p, reading).unwrap();
        cache.disk_hits()
    };
    // Asked in this order so that asking cannot itself evict what is asked next.
    let (a_kept, c_kept) = (hit(&a), hit(&c));
    report.check(
        "The one let go is the one used longest ago (B), not the one written first (A)",
        "A kept, C kept",
        format!("A {}, C {}", if a_kept == 1 { "kept" } else { "LET GO" }, if c_kept == 1 { "kept" } else { "LET GO" }),
    );
    report.check("A file in the folder that is not a copy is left alone", true, keep.exists());
    let _ = fs::remove_dir_all(&folder);
    let _ = fs::remove_dir_all(&media);

    // --- The viewer's own path: frames, read ahead included, with the copies on. ---------------
    let project = project();
    let comp = Id::new(COMP);
    let root = repo("Fixtures/reference_shot");
    let frames = [0, 12, 25, 100];
    let render = |cache: &mut CelCache| -> Vec<Vec<u32>> {
        let mut log = FrameLog::new(3);
        frames
            .iter()
            .map(|&f| {
                let buffer = preview::preview_frame_cached(&project, &comp, f, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, cache)
                    .unwrap();
                bits(&buffer)
            })
            .collect()
    };
    let folder = scratch("viewer");
    let viewer_disk = disk(&folder, u64::MAX);
    let plain_frames = render(&mut CelCache::none());
    let mut first = CelCache::viewer_sized(64 * 33_177_600);
    first.set_disk(Some(viewer_disk.clone()));
    let first_frames = render(&mut first);
    let mut second = CelCache::viewer_sized(64 * 33_177_600);
    second.set_disk(Some(viewer_disk.clone()));
    // P-18's read-ahead is `prewarm`, which is what `plan_frame` calls first; this is that call.
    let comp_model = project.composition(&comp).unwrap();
    second.prewarm(&compose::cels_at(&project, comp_model, frames[0], &root));
    let second_frames = render(&mut second);
    let samples = plain_frames.iter().map(Vec::len).sum::<usize>();
    let agree = |x: &Vec<Vec<u32>>| x.iter().flatten().zip(plain_frames.iter().flatten()).filter(|(p, q)| p == q).count();
    report.check(
        "Four full frames of the reference shot, first session (copies written): same bits as no copies",
        samples,
        agree(&first_frames),
    );
    report.check(
        "The same four frames in a second session (copies read): same bits as no copies",
        samples,
        agree(&second_frames),
    );
    report.check(
        "The second session decoded nothing: every drawing it needed came from a copy, read ahead or not",
        format!("{} decodes asked for, {} answered by copies", second.misses(), second.misses()),
        format!("{} decodes asked for, {} answered by copies", second.misses(), second.disk_hits()),
    );
    report.check("The cache export uses has no disk copies to read (CelCache::none)", 0, {
        let mut none = CelCache::none();
        none.decoded(&repo("Fixtures/reference_shot/layer2/layer2_001.png"), Interpretation::default()).unwrap();
        none.disk_hits()
    });
    let _ = fs::remove_dir_all(&folder);

    // --- The table. ----------------------------------------------------------------------------
    let failed: Vec<_> = report.rows.iter().filter(|(_, e, a)| e != a).collect();
    let mut out = String::from(
        "# B-161: decoded drawings kept on disk, and what they must not change\n\n\
         D-232 (owner delegated 2026-09-29): the first time the viewer decodes a drawing it also keeps the decoded \
         pixels on disk, and later - this session or after a restart - it reads that copy instead of decoding the PNG \
         again. That is only allowed if the copy is the decode, exactly. This table is produced by \
         `tests/b161_decode_cache.rs` and asks that question on every image under `Fixtures/`, the reference \
         shot's 56 drawings among them.\n\n\
         Read the first rows first: they compare every bit of every sample of a fresh decode with the copy read back. \
         The damaged-copy rows break a copy five different ways and check that the viewer notices, throws the copy \
         away, decodes the drawing again with the same bits, writes one line for the session log, and writes a whole \
         copy back. The last rows render real frames of the reference shot with the copies on and off.\n\n\
         ## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
    );
    for (check, expected, actual) in &report.rows {
        let result = if expected == actual { "pass" } else { "FAIL" };
        out.push_str(&format!("| {check} | {expected} | {actual} | {result} |\n"));
    }
    out.push_str(&format!(
        "\n{} of {} checks pass.\n\n\
         ## What is not checked here\n\n\
         - The speed. It is a measurement, in `verification/B-161_decode_cache_timing.md`.\n\
         - A drawing replaced by a different one of exactly the same length and exactly the same modified time. \
         Document 27 accepts length and time as the identity for interactive work, as B-08b's memory cache does; \
         a copy tool that keeps times, putting back an older drawing of exactly the same length, would be missed.\n",
        report.rows.len() - failed.len(),
        report.rows.len()
    ));
    fs::write(repo("verification/B-161_decode_cache_table.md"), out).unwrap();
    assert!(failed.is_empty(), "B-161 checks failed: {failed:#?}");
}

/// The measurement: per frame of the reference shot, decoding its drawings against reading their
/// copies. `cargo test --release --test b161_decode_cache -- --ignored`.
#[test]
#[ignore]
fn b161_timing() {
    const RUNS: usize = 7;
    const FRAMES: i32 = 48;
    let project = project();
    let comp = project.composition(&Id::new(COMP)).unwrap().clone();
    let root = repo("Fixtures/reference_shot");
    let folder = scratch("timing");
    let cache_disk = disk(&folder, u64::MAX);
    let wanted: Vec<_> = (0..FRAMES).map(|f| compose::cels_at(&project, &comp, f, &root)).collect();
    // Write every copy once, and time that too: it is what the first pass pays.
    let mut writes = Vec::new();
    for cels in &wanted {
        let started = Instant::now();
        for (path, reading) in cels {
            let mut cache = with_disk(&cache_disk);
            let _ = cache.decoded(path, *reading);
        }
        writes.push(started.elapsed().as_secs_f64() * 1000.0);
    }
    let once = |cels: &Vec<(PathBuf, Interpretation)>, use_disk: bool| {
        let started = Instant::now();
        for (path, reading) in cels {
            let mut cache = if use_disk { with_disk(&cache_disk) } else { CelCache::none() };
            let _ = cache.decoded(path, *reading);
        }
        started.elapsed().as_secs_f64() * 1000.0
    };
    // Each run times a decode and then a copy read of the same frame, one straight after the
    // other, so whatever else the machine is doing at that moment weighs on both columns alike.
    let (mut cold, mut hit) = (Vec::new(), Vec::new());
    for cels in &wanted {
        let (mut decodes, mut reads): (Vec<f64>, Vec<f64>) = (0..RUNS).map(|_| (once(cels, false), once(cels, true))).unzip();
        decodes.sort_by(f64::total_cmp);
        reads.sort_by(f64::total_cmp);
        cold.push(decodes[RUNS / 2]);
        hit.push(reads[RUNS / 2]);
    }
    let median = |v: &[f64]| {
        let mut v = v.to_vec();
        v.sort_by(f64::total_cmp);
        v[v.len() / 2]
    };
    let cels: usize = wanted.iter().map(Vec::len).sum();
    let held: u64 = copies(&folder).iter().map(|p| fs::metadata(p).unwrap().len()).sum();
    let mut out = format!(
        "# B-161: decoding a drawing against reading its disk copy\n\n\
         Produced by `cargo test --release --test b161_decode_cache -- --ignored`, with no other build running on \
         the machine; a busy machine gives noisy numbers.\n\n\
         ## Machine, build and configuration\n\n\
         - Machine: the reference machine (Windows 11, {} logical processors).\n\
         - Build: release (`opt-level = 3`).\n\
         - Shot: the reference shot, composition frames 0 to {} ({} drawings asked for, {} distinct copies, {:.1} MiB on disk).\n\
         - Each frame's drawings are read one after another on one thread, as `CelCache::decoded` does; the viewer's \
         read-ahead does them side by side, which divides both columns alike.\n\
         - Decode: `CelCache::none`, so no memory cache and no copies. Copy: a cache with no memory budget and the \
         copies on, so every drawing comes from its copy.\n\
         - The copies were read while Windows still had them in its file cache, which is the case for playing a shot \
         again in the same sitting. After a restart the first read comes from the SSD itself and is slower.\n\
         - {RUNS} runs per frame, each a decode and then a copy read one straight after the other, median of the \
         runs; then the median over the {FRAMES} frames. The first pass is timed once, before the runs.\n\n\
         ## Result\n\n\
         | Per frame (median over frames) | ms |\n|---|---|\n\
         | Decode the drawings | {:.2} |\n\
         | Read their copies | {:.2} |\n\
         | First pass: decode and write the copies | {:.2} |\n\n\
         Reading the copies took {:.0}% of the decode time.\n\n\
         ## Every frame\n\n| Frame | Decode ms | Copy ms | First pass ms |\n|---|---|---|---|\n",
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        FRAMES - 1,
        cels,
        copies(&folder).len(),
        held as f64 / (1024.0 * 1024.0),
        median(&cold),
        median(&hit),
        median(&writes),
        100.0 * median(&hit) / median(&cold),
    );
    for f in 0..FRAMES as usize {
        out.push_str(&format!("| {f} | {:.2} | {:.2} | {:.2} |\n", cold[f], hit[f], writes[f]));
    }
    fs::write(repo("verification/B-161_decode_cache_timing.md"), out).unwrap();
    let _ = fs::remove_dir_all(&folder);
}
