//! B-776: D-896's precision rule, single precision by default and double only where it is
//! proven needed, applied to the processor's (CPU's) effects.
//!
//! Writes `verification/B-776_precision_table.md`: every fixture of the effects whose weights moved
//! to single precision (Bilateral Blur, and Cartoon, which uses it), read again against the
//! reference's numbers at the fixtures' own tolerance (2e-5). The times before and after and
//! the effects kept in double precision, with why, are written above the checks from
//! `b776_precision_intro.md`; they were measured with `perf_audit::perf_audit_timing` and are
//! not worked out here.

mod effect_table;

use effect_table::{repo, Table};

const INTRO: &str = include_str!("b776_precision_intro.md");

#[test]
fn b776_precision() {
    let mut t = Table::new("bilateral_blur", INTRO);
    t.heading("Bilateral Blur's fixtures (FX-BILAT), its weights in single precision");
    t.fixtures("expected_bilateral_blur.json");
    t.root = repo("Fixtures/cartoon");
    t.heading("Cartoon's fixtures (FX-CARTOON), its smoothing is Bilateral Blur's");
    t.fixtures_numbered("expected_cartoon.json", 1..=32);
    t.finish("B-776_precision_table.md");
}
