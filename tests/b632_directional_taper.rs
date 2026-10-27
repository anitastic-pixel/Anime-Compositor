//! B-632: Directional Blur's streak tapers at its ends, against D-753.
//!
//! Writes `verification/B-632_directional_taper_table.md`.
//!
//! Every expected pixel is `Fixtures/directional_blur/expected_directional_blur.json` (and, for
//! Bloom, `Fixtures/bloom/expected_bloom.json`), written by the tools under `tools/` before this
//! code and printed in document 25. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{set, Table};

use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;

fn dirblur(direction: f64, length: f64) -> Effect {
    Effect::DirectionalBlur {
        direction,
        length,
        edges: "transparent".into(),
        taper: "on".into(),
    }
}

/// D-753's weight of the column `j` away in a streak `h` columns each way, worked straight from
/// document 21's closed form.
fn tapered(j: i32, h: f64) -> f64 {
    let j = j.abs() as f64;
    if j == 0.0 {
        return if h >= 1.0 { (h - 1.0 / 3.0) / (h * h) } else { 1.0 - h / 3.0 };
    }
    let c = |v: f64| v.max(0.0).powi(3);
    let x = h - j;
    (c(x + 1.0) - 2.0 * c(x) + c(x - 1.0)) / (6.0 * h * h)
}

#[test]
fn b632_directional_taper() {
    let mut t = Table::new(
        "directional_blur",
        "# B-632: Directional Blur's streak tapers at its ends\n\nD-753, accepted on 2026-10-23 \
         by the owner's message \"Proceed, if an effect is more like a inspiration/modified \
         version of a AE effect, only improve it. But it is to be a 1 to 1 match, then go for \
         it.\" Each streak is now a tent `length` long in all, heaviest at the drawing and \
         fading to nothing at its two ends, as After Effects' does, where it was a flat average \
         with hard ends. Every expected pixel is written by `tools/directional_blur_reference.py` \
         (and `tools/bloom_reference.py`) before this code and printed in document 25. The \
         build's frame is compared sample by sample against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-DIRBLUR-001 to 015 (document 25), on the processor");
    t.fixtures("expected_directional_blur.json");

    t.heading("The streak's shape: one half-covered red pixel streaked up and down, length 8");
    // FX-DIRBLUR-001's drawing: the red pixel at (12, 4) is alone in its column, so column 12 of
    // the blurred frame is its covering times the weights, h = 4 rows each way.
    let mut document = t.load("fx_dirblur_001.json").document;
    let taken = document.apply(set(dirblur(0.0, 8.0))).is_ok();
    t.row("direction 0, length 8 is taken", if taken { "taken" } else { "refused" }, taken);
    let frame = t.render(&document, 0, 64);
    let w = frame.width();
    let alpha = |y: usize| frame.data()[(y * w + 12) * 4 + 3] as f64;
    let red = 128.0 / 255.0;
    let mut previous = f64::INFINITY;
    for y in 4..10usize {
        let j = y as i32 - 4;
        let (got, want) = (alpha(y) / red, tapered(j, 4.0));
        let fades = got < previous;
        previous = got;
        t.row(
            &format!("{j} rows below the pixel its covering is {want:.6} of the pixel's, and less than the row before"),
            &format!("{got:.6}"),
            (got - want).abs() < 2e-5 && fades,
        );
    }
    let below = alpha(3) / red;
    t.row(
        "the streak is the same above as below: 1 row above equals 1 row below",
        &format!("{below:.6} against {:.6}", alpha(5) / red),
        (below - alpha(5) / red).abs() < 2e-5,
    );
    let sum: f64 = (0..10).map(|y| alpha(y) / red).sum();
    t.row(
        "the whole column keeps the pixel's covering: the weights sum to 1",
        &format!("{sum:.6}"),
        (sum - 1.0).abs() < 2e-5,
    );
    t.row(
        "five rows out, past the streak's end at four, nothing",
        &format!("{:.6}", alpha(9)),
        alpha(9) == 0.0,
    );

    t.heading("Bloom keeps its own streaks: FX-BLOOM-001 to 028 (document 25), unmoved by D-753");
    t.root = effect_table::repo("Fixtures/bloom");
    t.fixtures("expected_bloom.json");
    t.root = effect_table::repo("Fixtures/directional_blur");

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The card draws a Directional Blur as its own step (`OnCard::Directional`), not an effect
    // step, so the count of effect steps is 0 for every case; the pixels are what is compared.
    // D-919: 016 to 029 too, 016 to 026 with Taper Ends off.
    t.card_fixtures(
        &mut gpu,
        "expected_directional_blur.json",
        &(1..=29).collect::<Vec<u32>>(),
        |e| matches!(e, Effect::DirectionalBlur { .. }),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_dirblur_004.json", 0), ("fx_dirblur_007.json", 0), ("fx_dirblur_011.json", 2)]);

    t.finish("B-632_directional_taper_table.md");
}
