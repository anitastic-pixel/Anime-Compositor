//! P-13: the blur spread across the thread pool draws the same picture.
//!
//! `verification/P-01_frame_trace.md` ranked the stages of a preview frame, and on the fixture
//! document 08 line 41 declares - ten layers, two mattes, three effects - with everything warm,
//! the effect stack is 65.2% of a draft frame. The tile loop beside it, which has been spread
//! across the rayon pool since B-08a, is 2.1%. The blur inside that effect stack was a plain
//! `for` loop on one thread. This is the unit that spread it, and this file is the evidence that
//! spreading it changed no pixel.
//!
//! # What is compared
//!
//! One buffer the size of a cel of the declared fixture, blurred at that fixture's own
//! `sigma_px` of 4.0, twice: once inside a rayon pool built with a single thread, once on the
//! pool this machine would use. Every sample of both results is compared **by its bits**, not by
//! a tolerance. Document 25 line 57 allows a filter 2e-5 of error; this asks for none, because
//! the arithmetic did not move. A destination row reads the source and writes only itself, so
//! each destination pixel is still the same taps accumulated into the same `acc` in the same
//! order, whichever thread carries the row. A difference of a single bit here would mean that
//! claim is wrong.
//!
//! Nothing here has an expected value of its own (ADR-009): the expected result is the result the
//! same code produces on one thread, which is what the single-threaded pool is for. Whether the
//! blur is the blur document 21 specifies is `tests/b07_effects.rs`, against weights generated a
//! second way, and that question is not reopened here.
//!
//! # What is asserted, and what is only reported
//!
//! The sample comparison is asserted, on every build, and that is all this file does. No
//! duration is measured here: `tests/p01_frame_trace.rs` already times the blur as a stage of a
//! whole frame on the recorded machine, which is the number that matters and the one
//! `verification/P-13_parallel_blur.md` reports. Timings are never asserted anywhere in this
//! project, for document 12's reason - a threshold that fails because the machine was busy
//! teaches nothing - and the page this test writes carries no duration and no thread count, so
//! it is the same bytes on every machine and CI can check it.

use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::WorkingBuffer;

mod common;
use common::repo;

/// A cel of the declared fixture, and that fixture's own blur.
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const SIGMA: f64 = 4.0;

/// Something with an edge in it, because a blur over a flat field would agree with itself no
/// matter how the taps were added up. Premultiplied linear RGBA, built from the coordinates so
/// that it is the same buffer on every machine and every run.
fn cel() -> WorkingBuffer {
    let mut buffer = WorkingBuffer::transparent(WIDTH, HEIGHT);
    let data = buffer.data_mut();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let inside = (x / 37 + y / 41) % 2 == 0 && x > 12 && y > 9;
            let a = if inside { 1.0 } else { 0.0 };
            let i = (y * WIDTH + x) * 4;
            data[i] = a * (x % 251) as f32 / 251.0;
            data[i + 1] = a * (y % 239) as f32 / 239.0;
            data[i + 2] = a * ((x ^ y) % 197) as f32 / 197.0;
            data[i + 3] = a;
        }
    }
    buffer
}

fn blurred() -> WorkingBuffer {
    let mut buffer = cel();
    let stack = [EffectInstance::new(
        Id::new("p12-blur"),
        Effect::GaussianBlur { sigma_px: SIGMA },
    )];
    apply_stack(&mut buffer, &stack, |at, _, why| {
        panic!("the blur was bypassed at position {at}: {why:?}")
    });
    buffer
}

/// Runs `f` on a pool of exactly `threads` threads, so that the same call is measured and
/// compared with the work spread and with it not spread.
fn on_pool<T: Send>(threads: usize, f: impl FnOnce() -> T + Send) -> T {
    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .build()
        .expect("a thread pool of a fixed size")
        .install(f)
}

/// How many samples of the two buffers differ in their bits, and the first place they do.
fn difference(a: &WorkingBuffer, b: &WorkingBuffer) -> (usize, Option<usize>) {
    assert_eq!((a.width(), a.height()), (b.width(), b.height()));
    let (x, y) = (a.data(), b.data());
    let mut differing = 0;
    let mut first = None;
    for (i, (p, q)) in x.iter().zip(y.iter()).enumerate() {
        if p.to_bits() != q.to_bits() {
            differing += 1;
            first.get_or_insert(i);
        }
    }
    (differing, first)
}

#[test]
fn p13_parallel_blur_matches_one_thread() {
    let one = on_pool(1, blurred);
    let many = blurred();

    let (differing, first) = difference(&one, &many);
    let samples = one.data().len();
    assert_eq!(
        differing, 0,
        "{differing} of {samples} samples differ, the first at index {first:?}; \
         spreading the blur across the pool was supposed to move no bit"
    );
    // The buffer grew by the kernel radius on all four sides, which is what makes this a blur and
    // not a copy: a comparison of two buffers that were never filtered would also find no
    // difference.
    assert!(
        one.width() > WIDTH && one.height() > HEIGHT,
        "the blur did not expand the buffer, so nothing was filtered"
    );

    let out = repo("verification/P-13_parallel_blur_table.md");
    let page = format!(
        "# P-13: the blur on one thread and on every thread, sample by sample\n\n\
         Written by `p13_parallel_blur_matches_one_thread` in `tests/p13_parallel_blur.rs`, which \
         runs on every build. It carries no duration and no thread count on purpose, so that it \
         is the same bytes on every machine; what the spreading is worth in milliseconds is \
         `verification/P-13_parallel_blur.md`, out of P-01's timer on the recorded \
         machine.\n\n\
         One buffer of {WIDTH}x{HEIGHT}, the size of a cel of the fixture document 08 line 41 \
         declares, blurred at that fixture's own sigma of {SIGMA}. Once inside a pool built with \
         a single thread, once on the pool this machine would use. Compared by bits and not by a \
         tolerance, because the arithmetic did not move: a destination row reads the source and \
         writes only itself, so a pixel is the same taps accumulated in the same order whichever \
         thread carries its row.\n\n\
         | What was compared | Samples | Differing |\n\
         |---|---|---|\n\
         | the blurred buffer, f32 and bit for bit | {samples} | {differing} |\n\n\
         The buffer grew from {WIDTH}x{HEIGHT} to {}x{} - the kernel radius on all four sides, \
         which document 21 requires and which is also how this page knows a filter ran at all \
         rather than two untouched copies being compared.\n",
        one.width(),
        one.height(),
    );
    std::fs::write(&out, page).unwrap_or_else(|e| panic!("write {}: {e}", out.display()));
}
