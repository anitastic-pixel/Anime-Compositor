//! B-777, D-897: the viewer keeps a decoded drawing at its own 8 bits and widens it to floats
//! when a frame asks for it.
//!
//! Writes `verification/B-777_compact_cels_table.md` (correctness, every run) and, under
//! `--release --ignored`, one line per row of the timing to the file `B777_OUT` names, labelled
//! with `B777_LABEL`, so the build before and the build after can be run turn about with the
//! same test.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build):
//! - **"Identical" means every bit of every sample**, compared as `f32::to_bits`, against
//!   `CelCache::none()`, which keeps nothing and so decodes every time, as export does.
//! - **The 65,536 pairs** are every colour byte with every alpha byte, the whole of what an 8-bit
//!   drawing can hold in one channel, laid out as a 256 x 256 picture.
//! - **A budget of 8 bytes a pixel** has room for the drawing at 8 bits (4 bytes a pixel) and not
//!   at floats (16 bytes a pixel), so the second request is answered from memory only if the
//!   drawing is kept at 8 bits. The build before keeps floats and so decodes it again.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Interpretation, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, AlphaMode, ColorSpace, WorkingBuffer};

mod common;
use common::build_fixture;

const COMP: &str = "comp-reference-shot";
const IMAGE_EXTENSIONS: [&str; 9] = [
    "png", "exr", "tga", "webp", "bmp", "tif", "tiff", "jpg", "jpeg",
];
const GIB: usize = 1024 * 1024 * 1024;
const MIB: f64 = 1024.0 * 1024.0;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn bits(buffer: &WorkingBuffer) -> Vec<u32> {
    buffer.data().iter().map(|v| v.to_bits()).collect()
}

fn same(a: &WorkingBuffer, b: &WorkingBuffer) -> bool {
    a.width() == b.width() && a.height() == b.height() && bits(a) == bits(b)
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

fn is_exr(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("exr"))
}

/// The four ways a project may say a drawing is encoded.
fn interpretations() -> [Interpretation; 4] {
    [
        (ColorSpace::Srgb, AlphaMode::Straight),
        (ColorSpace::Srgb, AlphaMode::Premultiplied),
        (ColorSpace::LinearLight, AlphaMode::Straight),
        (ColorSpace::LinearLight, AlphaMode::Premultiplied),
    ]
    .map(|(color_space, alpha)| Interpretation { color_space, alpha })
}

/// Asked twice of a cache with room for the drawing at 8 bits and not at floats: whether the
/// second answer came from memory, and whether both answers are a fresh decode's to the bit.
fn twice(path: &Path, interpretation: Interpretation) -> Option<(bool, bool)> {
    let fresh = CelCache::none().decoded(path, interpretation).ok()?;
    let mut tight = CelCache::with_budget(fresh.width() * fresh.height() * 8);
    let first = tight.decoded(path, interpretation).ok()?;
    let first_same = same(&fresh, &first);
    // Let go, as a frame does once drawn, so the second answer cannot be these same floats.
    drop(first);
    let second = tight.decoded(path, interpretation).ok()?;
    Some((tight.hits() == 1, first_same && same(&fresh, &second)))
}

fn write_every_pair(path: &Path) {
    let mut rgba = Vec::with_capacity(256 * 256 * 4);
    for a in 0..=255u8 {
        for c in 0..=255u8 {
            rgba.extend_from_slice(&[c, 255 - c, c ^ 0x5a, a]);
        }
    }
    let file = fs::File::create(path).unwrap();
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), 256, 256);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&rgba).unwrap();
}

fn reference() -> Project {
    persist::load(&repo("verification/B-08a_project.json"))
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

fn frame(
    project: &Project,
    root: &Path,
    f: i32,
    q: PreviewQuality,
    cache: &mut CelCache,
) -> WorkingBuffer {
    let mut log = FrameLog::new(3);
    preview::preview_frame_cached(
        project,
        &Id::new(COMP),
        f,
        root,
        q,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame {f}: {}", d.message))
}

#[test]
fn b777_compact_cels_are_exact() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: String, actual: String| {
        rows.push((what.to_string(), expected, actual));
    };

    // --- Every image the fixtures hold, read all four ways. -----------------------------------
    let mut all = Vec::new();
    images(&repo("Fixtures"), &mut all);
    all.sort();
    let (mut asked, mut eight_bit, mut from_memory, mut exact) = (0, 0, 0, 0);
    let (mut exr, mut exr_from_memory) = (0, 0);
    for path in &all {
        for interpretation in interpretations() {
            let Some((hit, identical)) = twice(path, interpretation) else {
                continue;
            };
            asked += 1;
            exact += usize::from(identical);
            if is_exr(path) {
                exr += 1;
                exr_from_memory += usize::from(hit);
            } else {
                eight_bit += 1;
                from_memory += usize::from(hit);
            }
        }
    }
    check(
        "Every fixture image that decodes, read four ways, twice: both answers are a fresh decode's, bit for bit",
        format!("{asked} of {asked}"),
        format!("{exact} of {asked}"),
    );
    check(
        "and an 8-bit drawing's second answer came from memory with room for it at 8 bits only",
        format!("{eight_bit} of {eight_bit}"),
        format!("{from_memory} of {eight_bit}"),
    );
    check(
        "An EXR is floats already and stays floats, so with room for 8 bytes a pixel it is not kept",
        format!("0 of {exr}"),
        format!("{exr_from_memory} of {exr}"),
    );

    // --- Every colour byte with every alpha byte. ----------------------------------------------
    let scratch = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("b777");
    fs::create_dir_all(&scratch).unwrap();
    let pairs = scratch.join("every_pair.png");
    write_every_pair(&pairs);
    for interpretation in interpretations() {
        let (hit, identical) = twice(&pairs, interpretation).expect("the pairs picture decodes");
        check(
            &format!(
                "All 65,536 colour and alpha byte pairs, {:?} {:?}: widened from memory, bit for bit a fresh decode",
                interpretation.color_space, interpretation.alpha
            ),
            "from memory, identical".into(),
            format!(
                "{}, {}",
                if hit { "from memory" } else { "**decoded again**" },
                if identical { "identical" } else { "**different**" }
            ),
        );
    }

    // --- Whole frames, Full, from a cache that can only hold drawings at 8 bits. ---------------
    let project = reference();
    let root = repo("Fixtures/reference_shot");
    // Four drawings at 8 bits: 4 x 1920 x 1080 x 4 bytes, one float drawing's worth.
    let mut tight = CelCache::with_budget(4 * 1920 * 1080 * 4);
    // Fourteen drawings at 8 bits, or two at floats and six at 8 bits: floats are kept, let go
    // and written over by the next drawing widened.
    let mut some = CelCache::with_budget(14 * 1920 * 1080 * 4);
    let mut none = CelCache::none();
    let frames = [0, 12, 14, 25, 100, 101];
    let (mut identical, mut some_identical) = (0, 0);
    for _round in 0..2 {
        for f in frames {
            let a = frame(&project, &root, f, PreviewQuality::Full, &mut tight);
            let b = frame(&project, &root, f, PreviewQuality::Full, &mut none);
            let c = frame(&project, &root, f, PreviewQuality::Full, &mut some);
            identical += usize::from(same(&a, &b));
            some_identical += usize::from(same(&c, &b));
        }
    }
    check(
        "Reference shot frames 0, 12, 14, 25, 100, 101 at Full, twice round, from a cache of four 8-bit drawings: the uncached frame, bit for bit",
        "12 of 12".into(),
        format!("{identical} of 12"),
    );
    check(
        "and that cache answered from memory on the second round",
        "hits > 0".into(),
        if tight.hits() > 0 { "hits > 0".into() } else { "**0 hits**".into() },
    );
    check(
        "The same frames from a cache with room for some drawings at floats too: the uncached frame, bit for bit",
        "12 of 12".into(),
        format!("{some_identical} of 12"),
    );

    // --- The card finds a drawing it already holds by the cache's name for it. -----------------
    let cel = root.join("layer1/layer1_000.png");
    let mut named = CelCache::with_budget(1920 * 1080 * 8);
    let a = named.decoded(&cel, Interpretation::default()).unwrap();
    let first_name = named.name_of(&a);
    drop(a);
    let b = named.decoded(&cel, Interpretation::default()).unwrap();
    let second_name = named.name_of(&b);
    check(
        "A drawing widened again after its floats were let go keeps the name the card holds it under",
        "named, the same name".into(),
        format!(
            "{}, {}",
            if second_name.is_some() { "named" } else { "**no name**" },
            if first_name.is_some() && first_name == second_name {
                "the same name"
            } else {
                "**a different name**"
            }
        ),
    );
    let c = named.decoded(&cel, Interpretation::default()).unwrap();
    check(
        "and while a frame still holds those floats, asking again hands back the same floats, not a second copy",
        "the same floats".into(),
        if Arc::ptr_eq(&b, &c) {
            "the same floats".into()
        } else {
            "**a second copy**".into()
        },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut page = format!(
        "# B-777: drawings kept at 8 bits, widened when asked for\n\n\
         Written by `tests/b777_compact_cels.rs` (`cargo test --test b777_compact_cels`), for D-897. \
         **{passed} of {} pass.**\n\n\
         \"Identical\" is every bit of every sample (`f32::to_bits`), against `CelCache::none()`, which keeps \
         nothing and decodes every time, as export does.\n\n\
         | Check | Expected | Actual | |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        let _ = writeln!(
            page,
            "| {what} | {expected} | {actual} | {} |",
            if expected == actual { "pass" } else { "**FAIL**" }
        );
    }
    fs::write(repo("verification/B-777_compact_cels_table.md"), page).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "failed: {failed:?}");
}

fn pct(sorted: &[f64], p: f64) -> f64 {
    sorted[((p * sorted.len() as f64).ceil() as usize).clamp(1, sorted.len()) - 1]
}

/// D-897's measurement: document 08's ten-layer shot (166 distinct drawings) on the processor at
/// Draft, with the viewer's cache at four memory settings (16 GiB is about this machine's
/// Automatic, 15.4 GiB, D-105; 8 GiB is a 32 GB machine's). One JSON line per setting.
#[test]
#[ignore = "timing; run under --release with --ignored"]
fn b777_compact_cels_timing() {
    let label = std::env::var("B777_LABEL").unwrap_or_else(|_| "unlabelled".into());
    let out = std::env::var("B777_OUT").map(PathBuf::from).ok();
    let (project, root, _) = build_fixture("b777");
    let mut lines = String::new();
    for total in [GIB, 4 * GIB, 8 * GIB, 16 * GIB] {
        let mut cache = CelCache::viewer_sized(total);
        let mut loops = Vec::new();
        let mut decodes = Vec::new();
        for _ in 0..3 {
            let misses = cache.misses();
            let mut ms: Vec<f64> = (0..240)
                .map(|f| {
                    let at = Instant::now();
                    frame(&project, &root, f, PreviewQuality::Draft, &mut cache);
                    at.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
            loops.push(ms);
            decodes.push(cache.misses() - misses);
        }
        let mut seek: Vec<f64> = (0..240)
            .map(|i| {
                let at = Instant::now();
                frame(&project, &root, (i * 97) % 240, PreviewQuality::Draft, &mut cache);
                at.elapsed().as_secs_f64() * 1000.0
            })
            .collect();
        seek.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!(cache.held_bytes() <= cache.budget());
        let third = &loops[2];
        let line = format!(
            "{{\"label\":\"{label}\",\"total_gib\":{},\"cel_budget_mib\":{:.0},\"drawings_held\":{},\"held_mib\":{:.0},\
             \"decodes_loop2\":{},\"decodes_loop3\":{},\"loop3_total_ms\":{:.0},\"loop3_median\":{:.2},\"loop3_p95\":{:.2},\
             \"seek_median\":{:.2},\"seek_p95\":{:.2}}}\n",
            total / GIB,
            cache.budget() as f64 / MIB,
            cache.len(),
            cache.held_bytes() as f64 / MIB,
            decodes[1],
            decodes[2],
            third.iter().sum::<f64>(),
            pct(third, 0.5),
            pct(third, 0.95),
            pct(&seek, 0.5),
            pct(&seek, 0.95),
        );
        print!("{line}");
        lines.push_str(&line);
    }
    if let Some(out) = out {
        use std::io::Write;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(out)
            .unwrap();
        f.write_all(lines.as_bytes()).unwrap();
    }
}
