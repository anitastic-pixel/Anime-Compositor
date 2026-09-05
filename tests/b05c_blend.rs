//! R-03 / R-10 / B-05c: the multiply, screen and add blend modes.
//!
//! Writes `verification/B-05c_blend_table.md`.
//!
//! # Where the expected values come from
//!
//! Document 21, "Blend modes", gives the whole contract:
//!
//! ```text
//! multiply: B = cs * cd
//! screen:   B = cs + cd - cs*cd
//! add:      B = min(1, cs + cd)          "the bounded G1 display blend"
//!
//! Co = (1-As)*Cd + (1-Ad)*Cs + As*Ad*B(cs,cd)
//! Ao = As + Ad - As*Ad
//! ```
//!
//! where `cs` and `cd` are the *straight* colours recovered from the premultiplied operands,
//! and "Zero-alpha straight colors are zero".
//!
//! The first three rows below are document 25's FX-B-001, FX-B-002 and FX-B-003, copied from
//! the catalogue. Every other expected value in this file was worked out by hand from the
//! equations above, and the arithmetic is written into the comment above each one so it can be
//! checked on paper. Nothing here was captured from a run of the code under test (ADR-009).
//!
//! # What is deliberately not tested here
//!
//! Nothing assembles a [`FramePlan`] from a [`Project`] yet, so no test can show a layer's
//! stored `blend_mode` reaching the renderer through real application code — that assembly is
//! B-08's. The seam is checked instead: the last section shows that all four stored modes are
//! the same four values `LayerDraw` carries, so the wiring is a move and not a conversion.

use std::fs;
use std::path::PathBuf;

use anime_compositor::composite::{blend_pixel, over_pixel};
use anime_compositor::model::{BlendMode, Id, Layer};
use anime_compositor::render::{render, Affine, FramePlan, LayerDraw};
use anime_compositor::WorkingBuffer;

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

/// A pixel written to six decimal places.
///
/// Document 25 asks for the FX-B fixtures to 1e-6, and six places is that tolerance made
/// visible: two values that agree to six places print the same string, and a reader can see
/// the number rather than the word "within tolerance".
fn q(p: [f32; 4]) -> String {
    format!(
        "({:.6}, {:.6}, {:.6}, {:.6})",
        p[0] as f64, p[1] as f64, p[2] as f64, p[3] as f64
    )
}

/// Straight RGBA to premultiplied, done here rather than by the crate so the operands of every
/// check are independent of the code under test.
fn premul(r: f32, g: f32, b: f32, a: f32) -> [f32; 4] {
    [r * a, g * a, b * a, a]
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// A flat buffer of one colour, used as a layer source.
fn solid(width: usize, height: usize, p: [f32; 4]) -> WorkingBuffer {
    let mut buf = WorkingBuffer::transparent(width, height);
    for px in buf.data_mut().chunks_exact_mut(4) {
        px.copy_from_slice(&p);
    }
    buf
}

fn draw(source: WorkingBuffer, opacity: f32, blend: BlendMode) -> LayerDraw {
    LayerDraw {
        id: Id::new("layer"),
        source,
        transform: Affine::IDENTITY,
        opacity,
        blend,
    }
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b05c_blend_fixtures() {
    let mut report = Report::default();

    // ---- document 25's three FX-B rows, copied from the catalogue -------------------------

    // FX-B-001: "multiply opaque | red over 50% gray | B=(.5,0,0), A=1".
    // cs=(1,0,0), cd=(.5,.5,.5), both alphas 1, so Co = B = (1*.5, 0*.5, 0*.5).
    report.check(
        "FX-B-001 multiply, opaque red over 50% grey",
        q([0.5, 0.0, 0.0, 1.0]),
        q(blend_pixel(
            BlendMode::Multiply,
            premul(1.0, 0.0, 0.0, 1.0),
            premul(0.5, 0.5, 0.5, 1.0),
        )),
    );

    // FX-B-002: "screen opaque | .5 gray over .5 gray | .75 gray, A=1".
    // B = .5 + .5 - .25 = .75.
    report.check(
        "FX-B-002 screen, 50% grey over 50% grey",
        q([0.75, 0.75, 0.75, 1.0]),
        q(blend_pixel(
            BlendMode::Screen,
            premul(0.5, 0.5, 0.5, 1.0),
            premul(0.5, 0.5, 0.5, 1.0),
        )),
    );

    // FX-B-003: "add opaque | .7 + .6 gray | 1.0 clamped, A=1".
    // B = min(1, 1.3) = 1.
    report.check(
        "FX-B-003 add, 70% grey over 60% grey, clamped",
        q([1.0, 1.0, 1.0, 1.0]),
        q(blend_pixel(
            BlendMode::Add,
            premul(0.7, 0.7, 0.7, 1.0),
            premul(0.6, 0.6, 0.6, 1.0),
        )),
    );

    // The same operands under screen come to .7 + .6 - .42 = .88, so this pair is what
    // separates add from screen. Without the clamp add would read 1.3 here, and without the
    // clamp *being part of the mode* the two modes would be indistinguishable at .7 over .6.
    report.check(
        "screen on the same operands is 0.88, so add's clamp is doing the work",
        q([0.88, 0.88, 0.88, 1.0]),
        q(blend_pixel(
            BlendMode::Screen,
            premul(0.7, 0.7, 0.7, 1.0),
            premul(0.6, 0.6, 0.6, 1.0),
        )),
    );

    // ---- the equation where the alphas are not 1 ------------------------------------------

    // Half-alpha red over opaque 50% grey, multiply.
    //   Cs = (.5,0,0,.5), cs = (1,0,0);  Cd = (.5,.5,.5,1), cd = (.5,.5,.5)
    //   B  = (.5, 0, 0)
    //   Co_r = (1-.5)*.5 + (1-1)*.5 + .5*1*.5 = .25 + 0 + .25 = .5
    //   Co_g = (1-.5)*.5 + 0 + .5*1*0        = .25
    //   Co_b = .25
    //   Ao   = .5 + 1 - .5 = 1
    report.check(
        "multiply, half-alpha red over opaque 50% grey",
        q([0.5, 0.25, 0.25, 1.0]),
        q(blend_pixel(
            BlendMode::Multiply,
            premul(1.0, 0.0, 0.0, 0.5),
            premul(0.5, 0.5, 0.5, 1.0),
        )),
    );

    // Half-alpha 80% grey over half-alpha 40% grey, screen.
    //   Cs = (.4,.4,.4,.5), cs = .8;  Cd = (.2,.2,.2,.5), cd = .4
    //   B  = .8 + .4 - .32 = .88
    //   Co = (1-.5)*.2 + (1-.5)*.4 + .5*.5*.88 = .1 + .2 + .22 = .52
    //   Ao = .5 + .5 - .25 = .75
    let partial_screen = blend_pixel(
        BlendMode::Screen,
        premul(0.8, 0.8, 0.8, 0.5),
        premul(0.4, 0.4, 0.4, 0.5),
    );
    report.check(
        "screen, half-alpha 80% grey over half-alpha 40% grey",
        q([0.52, 0.52, 0.52, 0.75]),
        q(partial_screen),
    );

    // And the straight colour that comes back out of it: .52 / .75 = .693333...
    report.check(
        "and its straight colour is 0.52/0.75",
        format!("{:.6}", 0.52_f64 / 0.75_f64),
        format!("{:.6}", partial_screen[0] as f64 / partial_screen[3] as f64),
    );

    // Alpha is the union of the two coverages and does not depend on the mode:
    // Ao = .5 + .5 - .25 = .75 for all three.
    let union: Vec<String> = [BlendMode::Multiply, BlendMode::Screen, BlendMode::Add]
        .iter()
        .map(|&m| {
            format!(
                "{:.6}",
                blend_pixel(m, premul(0.8, 0.8, 0.8, 0.5), premul(0.4, 0.4, 0.4, 0.5))[3]
            )
        })
        .collect();
    report.check(
        "output alpha is As+Ad-As*Ad in every mode",
        "0.750000, 0.750000, 0.750000",
        union.join(", "),
    );

    // ---- the two degenerate cases the equation has to survive ------------------------------

    // Over nothing at all, every mode leaves the source exactly where it is:
    //   Ad = 0, so cd = 0 and Co = (1-As)*0 + 1*Cs + As*0*B = Cs, Ao = As.
    // Worth a row of its own because it is the surprising one: multiplying a layer against an
    // empty background does not turn it black.
    let src = premul(1.0, 0.0, 0.0, 0.5);
    let over_nothing: Vec<String> = [BlendMode::Multiply, BlendMode::Screen, BlendMode::Add]
        .iter()
        .map(|&m| q(blend_pixel(m, src, [0.0; 4])))
        .collect();
    report.check(
        "over a transparent background, every mode leaves the source untouched",
        [q(src), q(src), q(src)].join(" "),
        over_nothing.join(" "),
    );

    // A transparent source changes nothing, which is FX-A-001's rule applied to a blend mode:
    //   As = 0, so cs = 0, B = 0, Co = 1*Cd + (1-Ad)*0 + 0 = Cd.
    report.check(
        "a fully transparent source leaves the destination unchanged",
        q([0.2, 0.4, 0.6, 1.0]),
        q(blend_pixel(
            BlendMode::Multiply,
            [0.0; 4],
            premul(0.2, 0.4, 0.6, 1.0),
        )),
    );

    // Document 21 line 9 again: zero alpha must not produce NaN or Inf through the divide that
    // recovers straight colour.
    let both_empty = blend_pixel(BlendMode::Screen, [0.0; 4], [0.0; 4]);
    report.check(
        "two empty pixels blend to a finite result, not NaN",
        "(0.000000, 0.000000, 0.000000, 0.000000) all finite",
        format!(
            "{} {}",
            q(both_empty),
            if both_empty.iter().all(|c| c.is_finite()) {
                "all finite"
            } else {
                "NOT FINITE"
            }
        ),
    );

    // ---- normal is untouched ---------------------------------------------------------------

    // Document 21 gives normal separately as Co = Cs + Cd*(1-As), Ao = As + Ad*(1-As):
    //   Co = .4 + .2*.5 = .5,  Ao = .5 + .5*.5 = .75
    report.check(
        "normal still follows document 21's own source-over formula",
        q([0.5, 0.5, 0.5, 0.75]),
        q(blend_pixel(
            BlendMode::Normal,
            premul(0.8, 0.8, 0.8, 0.5),
            premul(0.4, 0.4, 0.4, 0.5),
        )),
    );
    report.check(
        "and it is the same pixel B-02's over_pixel produces",
        q(over_pixel(
            premul(0.8, 0.8, 0.8, 0.5),
            premul(0.4, 0.4, 0.4, 0.5),
        )),
        q(blend_pixel(
            BlendMode::Normal,
            premul(0.8, 0.8, 0.8, 0.5),
            premul(0.4, 0.4, 0.4, 0.5),
        )),
    );

    // ---- through the renderer --------------------------------------------------------------

    // Two 2x2 layers, identity transforms. Bottom: opaque 50% grey. Top: red at 50% layer
    // opacity, which document 21 applies at step 6, before the blend at step 7. So the blend
    // sees exactly the operands of the "half-alpha red over opaque 50% grey" row above, and
    // every pixel of the frame must equal that row's answer.
    let plan_for = |mode: BlendMode| FramePlan {
        width: 2,
        height: 2,
        layers: vec![
            draw(
                solid(2, 2, premul(0.5, 0.5, 0.5, 1.0)),
                1.0,
                BlendMode::Normal,
            ),
            draw(solid(2, 2, premul(1.0, 0.0, 0.0, 1.0)), 0.5, mode),
        ],
    };
    let frame = render(&plan_for(BlendMode::Multiply), 8);
    let pixels: Vec<String> = frame
        .data()
        .chunks_exact(4)
        .map(|p| q([p[0], p[1], p[2], p[3]]))
        .collect();
    report.check(
        "the renderer applies the layer's mode: all four pixels of a multiply frame",
        vec![q([0.5, 0.25, 0.25, 1.0]); 4].join(" "),
        pixels.join(" "),
    );

    // The same plan under normal. Co_r = .5 + .5*(1-.5) = .75, Co_g = 0 + .5*.5 = .25,
    // Ao = .5 + 1*.5 = 1. A build that ignored the mode would print this for multiply too.
    let normal = render(&plan_for(BlendMode::Normal), 8);
    report.check(
        "the same stack under normal is a different pixel, so the mode is being read",
        q([0.75, 0.25, 0.25, 1.0]),
        q([
            normal.data()[0],
            normal.data()[1],
            normal.data()[2],
            normal.data()[3],
        ]),
    );

    // Screen: cs=(1,0,0), cd=(.5,.5,.5), B=(1,.5,.5), both weights .5*1.
    //   Co_r = .5*.5 + 0 + .5*1   = .75
    //   Co_g = .5*.5 + 0 + .5*.5  = .5
    let screen = render(&plan_for(BlendMode::Screen), 8);
    report.check(
        "and under screen it is a third one",
        q([0.75, 0.5, 0.5, 1.0]),
        q([
            screen.data()[0],
            screen.data()[1],
            screen.data()[2],
            screen.data()[3],
        ]),
    );

    // ADR-011: a tiled render and a whole-frame render must be byte-identical, and blending is
    // per-pixel so adding a mode cannot change that.
    let reference = render(&plan_for(BlendMode::Multiply), 64);
    let identical = [1usize, 2, 3, 8, 64]
        .iter()
        .all(|&size| render(&plan_for(BlendMode::Multiply), size).data() == reference.data());
    report.check(
        "a blended frame is byte-identical at every tile size (ADR-011)",
        "identical",
        if identical { "identical" } else { "differs" },
    );

    // ---- the seam between the stored mode and the rendered one -----------------------------

    // Nothing builds a FramePlan from a Project yet; that is B-08. What can be shown now is
    // that there is no conversion to get wrong: the four modes a layer stores are the four a
    // draw carries, under the four names the schema spells.
    let seam: Vec<String> = [
        BlendMode::Normal,
        BlendMode::Multiply,
        BlendMode::Screen,
        BlendMode::Add,
    ]
    .iter()
    .map(|&mode| {
        let mut layer = Layer::new(Id::new("l"), "l", Id::new("a"), 0, 1);
        layer.blend_mode = mode;
        let carried = draw(solid(1, 1, [0.0; 4]), 1.0, layer.blend_mode).blend;
        carried.as_str().to_string()
    })
    .collect();
    report.check(
        "a layer's stored blend mode is the value a draw carries, unconverted",
        "normal, multiply, screen, add",
        seam.join(", "),
    );

    write_report(&report);
    let failed = report.rows.iter().filter(|r| !r.pass()).count();
    assert_eq!(
        failed,
        0,
        "B-05c blend fixtures: {failed} of {} checks failed\n{}",
        report.rows.len(),
        report
            .rows
            .iter()
            .filter(|r| !r.pass())
            .map(|r| format!(
                "  {}\n    expected {}\n    actual   {}",
                r.check, r.expected, r.actual
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-05c — the multiply, screen and add blend modes\n\n\
         **{passed} of {} checks passed.**\n\n\
         Generated by `tests/b05c_blend.rs`. Covers requirements R-03 and R-10, document 21's \
         \"Blend modes\" section, and document 25's fixtures FX-B-001, FX-B-002 and FX-B-003.\n\n\
         ## What to look at\n\n\
         A blend mode decides how a layer's colour is combined with what is already underneath \
         it. Multiply darkens, screen lightens, add brightens and stops at white. Every number \
         in the table below was worked out on paper from the two equations in document 21 \
         before the code was run, and the working is written into the test beside each one.\n\n\
         Three rows are worth reading on their own:\n\n\
         - **Over a transparent background, every mode leaves the source untouched.** This is \
         the one that surprises people: a layer set to multiply, with nothing under it, does \
         not go black. If it did, the bottom layer of every stack would disappear the moment \
         its mode was changed.\n\
         - **The same stack under normal is a different pixel.** Three rows render the same two \
         layers three times and get three different answers. A build that stored the mode but \
         ignored it would print the same answer three times.\n\
         - **Add's clamp is doing the work.** 70% over 60% comes to 1.0 under add and 0.88 \
         under screen. Without the clamp document 21 calls \"the bounded G1 display blend\", \
         add would read 1.3 and the two modes would be hard to tell apart.\n\n\
         ## What this does not cover\n\n\
         Nothing assembles a render from a saved project yet, so no test here can show a mode \
         travelling from a file to the screen; that assembly is B-08. The last row shows the \
         seam instead: the four modes a layer stores are the four a render carries, under the \
         four names the schema spells, so the wiring is a move rather than a conversion.\n\n\
         ## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        report.rows.len(),
    ));
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    fs::write(repo("verification/B-05c_blend_table.md"), out).expect("write report");
}
