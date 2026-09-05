//! B-02 exit tests: T-04 (over-composite of known alpha values, transparent edges) and
//! T-09 (known linear/sRGB values, straight/premultiplied conversion, zero alpha, no
//! duplicate display transform), for requirement R-10.
//!
//! Expected values are independently derived. The FX-A-* rows come from
//! `Fixtures/fixture_manifest.json` and document 25. The transfer-function rows were computed
//! at IEEE 754 binary64 in `verification/derive_b02_expected.py`, directly from the published
//! IEC 61966-2-1 formulae, and transcribed here. No expected value in this file was produced
//! by running the implementation under test.
//!
//! Running this test writes `verification/B-02_fixture_table.md`, which is the artifact
//! document 15 requires for B-02: expected versus actual to full precision.

use std::fmt::Write as _;

use anime_compositor::composite::{over_pixel, premultiply, unpremultiply};
use anime_compositor::{
    color, composite, AlphaMode, BufferError, ColorSpace, ImageBuffer, WorkingBuffer,
};

/// Tolerance for values that pass through `powf`.
///
/// Derived, not fitted: f32 carries a 24-bit mantissa, so relative epsilon is 2^-24 =
/// 5.96e-8. `powf` is typically accurate to a small number of ulp, giving an absolute error
/// on the order of 1e-7 for values in 0..1, and the f32 result is compared against an f64
/// derivation that contributes its own rounding of the same order. 1e-6 leaves roughly an
/// order of magnitude of margin and matches the document 25 baseline for float32 fixtures.
/// Rows that are exact in f32 use the tighter manifest tolerances unchanged.
const TF_TOL: f64 = 1e-6;

/// The fixture manifest, embedded so that a change to it breaks the build's tests loudly.
const MANIFEST: &str = include_str!("../Fixtures/fixture_manifest.json");

fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// Line-ending-independent hash of the manifest.
fn manifest_hash() -> u64 {
    fnv1a(MANIFEST.replace("\r\n", "\n").as_bytes())
}

/// Fails if `Fixtures/fixture_manifest.json` changes without this test being re-derived.
///
/// CLAUDE.md: "`Fixtures/` expected values are read-only to implementation work. Proposing a
/// change to one is a specification decision." The expected values below were transcribed
/// from that file by hand, so a manifest edit must land together with a re-derivation, and
/// this constant updated deliberately rather than the drift going unnoticed.
#[test]
fn manifest_unchanged() {
    const EXPECTED: u64 = 0x2d1c_fb54_7e56_507f;
    assert_eq!(
        manifest_hash(),
        EXPECTED,
        "Fixtures/fixture_manifest.json changed. Expected values in this test were transcribed \
         from it and must be re-derived before this constant is updated."
    );
}

struct Row {
    id: &'static str,
    what: String,
    expected: String,
    actual: String,
    tol: String,
    pass: bool,
}

fn f32s(v: &[f32]) -> String {
    v.iter().map(|x| format!("{x:.8e}")).collect::<Vec<_>>().join(", ")
}

fn f64s(v: &[f64]) -> String {
    v.iter().map(|x| format!("{x:.16e}")).collect::<Vec<_>>().join(", ")
}

fn row(
    rows: &mut Vec<Row>,
    id: &'static str,
    what: &str,
    actual: &[f32],
    expected: &[f64],
    tol: f64,
) {
    let pass = actual.len() == expected.len()
        && actual.iter().zip(expected).all(|(a, e)| (*a as f64 - *e).abs() <= tol);
    rows.push(Row {
        id,
        what: what.to_string(),
        expected: f64s(expected),
        actual: f32s(actual),
        tol: format!("{tol:e} absolute"),
        pass,
    });
}

fn exact_row(rows: &mut Vec<Row>, id: &'static str, what: &str, actual: String, expected: String) {
    let pass = actual == expected;
    rows.push(Row { id, what: what.to_string(), expected, actual, tol: "exact".into(), pass });
}

#[test]
fn b02_fixture_table() {
    let mut rows: Vec<Row> = Vec::new();

    // ---- T-04: normal over with known alpha values (document 21, "Normal composite") ----

    // FX-A-001: a transparent source leaves the destination untouched.
    row(
        &mut rows,
        "FX-A-001",
        "transparent source-over: S=(0,0,0,0) over D=(0.2,0.4,0.6,1)",
        &over_pixel([0.0, 0.0, 0.0, 0.0], [0.2, 0.4, 0.6, 1.0]),
        &[0.2, 0.4, 0.6, 1.0],
        1e-7,
    );

    // FX-A-002: an opaque source replaces the destination.
    row(
        &mut rows,
        "FX-A-002",
        "opaque source-over: S=(0.8,0.1,0.2,1) over D=(0.2,0.4,0.6,1)",
        &over_pixel([0.8, 0.1, 0.2, 1.0], [0.2, 0.4, 0.6, 1.0]),
        &[0.8, 0.1, 0.2, 1.0],
        1e-7,
    );

    // FX-A-003: straight (1,0,0) at alpha 0.5, premultiplied, over opaque blue.
    row(
        &mut rows,
        "FX-A-003",
        "partial alpha over opaque: straight S=(1,0,0) As=0.5 over D=(0,0,1,1)",
        &over_pixel(premultiply([1.0, 0.0, 0.0, 0.5]), [0.0, 0.0, 1.0, 1.0]),
        &[0.5, 0.0, 0.5, 1.0],
        1e-6,
    );

    // FX-A-004: zero-alpha unpremultiply is zero and finite, never NaN or Inf.
    let z = unpremultiply([0.0, 0.0, 0.0, 0.0]);
    exact_row(
        &mut rows,
        "FX-A-004",
        "zero-alpha unpremultiply of C=(0,0,0) A=0 is zero and finite",
        format!("{} finite={}", f32s(&z), z.iter().all(|c| c.is_finite())),
        format!("{} finite=true", f32s(&[0.0, 0.0, 0.0, 0.0])),
    );

    // A premultiplied pixel carrying alpha zero but nonzero RGB must also unpremultiply to
    // zero rather than dividing. Document 21 line 9 states the rule without restricting it to
    // the already-zero case, and this is the input shape that actually produces Inf.
    let z2 = unpremultiply([0.7, 0.2, 0.1, 0.0]);
    exact_row(
        &mut rows,
        "B02-ZEROA",
        "zero-alpha unpremultiply of C=(0.7,0.2,0.1) A=0 does not divide",
        format!("{} finite={}", f32s(&z2), z2.iter().all(|c| c.is_finite())),
        format!("{} finite=true", f32s(&[0.0, 0.0, 0.0, 0.0])),
    );

    // T-04 transparent edges, at buffer level: a transparent border composites away to
    // nothing while the covered interior lands exactly on the source.
    let mut src_px = vec![0.0f32; 3 * 3 * 4];
    let centre = (1 * 3 + 1) * 4;
    src_px[centre] = 1.0;
    src_px[centre + 3] = 1.0;
    let src_buf = ImageBuffer::new(3, 3, ColorSpace::LinearLight, AlphaMode::Premultiplied, src_px)
        .expect("3x3 source")
        .into_working();
    let mut dst_buf = ImageBuffer::new(
        3,
        3,
        ColorSpace::LinearLight,
        AlphaMode::Premultiplied,
        [0.0f32, 0.0, 1.0, 1.0].repeat(9),
    )
    .expect("3x3 opaque blue destination")
    .into_working();
    composite::over(&src_buf, &mut dst_buf).expect("matching extents");
    exact_row(
        &mut rows,
        "B02-EDGE",
        "3x3 transparent border over opaque blue: border untouched, centre replaced",
        format!("corner={} centre={}", f32s(&dst_buf.pixel(0, 0)), f32s(&dst_buf.pixel(1, 1))),
        format!("corner={} centre={}", f32s(&[0.0, 0.0, 1.0, 1.0]), f32s(&[1.0, 0.0, 0.0, 1.0])),
    );

    let mismatch = composite::over(&src_buf, &mut WorkingBuffer::transparent(4, 3));
    exact_row(
        &mut rows,
        "B02-EXTENT",
        "compositing mismatched extents is refused rather than silently clipped",
        format!("{:?}", mismatch.err()),
        format!(
            "{:?}",
            Some(composite::CompositeError::ExtentMismatch { src: (3, 3), dst: (4, 3) })
        ),
    );

    let bad_len = ImageBuffer::new(2, 2, ColorSpace::Srgb, AlphaMode::Straight, vec![0.0; 15]);
    exact_row(
        &mut rows,
        "B02-LEN",
        "a buffer whose data length contradicts its extent is refused at construction",
        format!("{:?}", bad_len.err()),
        format!("{:?}", Some(BufferError::LengthMismatch { expected: 16, actual: 15 })),
    );

    // ---- T-09: known linear and sRGB values ----
    // Expected values from verification/derive_b02_expected.py, computed at binary64.

    for (id, name, input, expected) in [
        ("T09-S2L-a", "sRGB 0.0", 0.0f32, 0.0f64),
        ("T09-S2L-b", "sRGB 10/255, on the linear segment", 10.0 / 255.0, 0.003_035_269_835_488_374_8),
        ("T09-S2L-c", "sRGB 0.04045, the kink", 0.04045, 0.003_130_804_953_560_371_3),
        ("T09-S2L-d", "sRGB 0.05, just above the kink", 0.05, 0.003_935_939_504_088_967),
        ("T09-S2L-e", "sRGB 128/255", 128.0 / 255.0, 0.215_860_500_113_899_26),
        ("T09-S2L-f", "sRGB 0.5", 0.5, 0.214_041_140_482_232_55),
        ("T09-S2L-g", "sRGB 1.0", 1.0, 1.0),
    ] {
        row(
            &mut rows,
            id,
            &format!("sRGB to linear: {name}"),
            &[color::srgb_to_linear(input)],
            &[expected],
            TF_TOL,
        );
    }

    for (id, name, input, expected) in [
        ("T09-L2S-a", "linear 0.0", 0.0f32, 0.0f64),
        ("T09-L2S-b", "linear 0.0031308, the kink", 0.0031308, 0.040_449_936),
        ("T09-L2S-c", "linear 0.18", 0.18, 0.461_356_129_500_441_64),
        ("T09-L2S-d", "linear 0.5", 0.5, 0.735_356_983_052_449_45),
        ("T09-L2S-e", "linear 1.0", 1.0, 0.999_999_999_999_999_89),
    ] {
        row(
            &mut rows,
            id,
            &format!("linear to sRGB: {name}"),
            &[color::linear_to_srgb(input)],
            &[expected],
            TF_TOL,
        );
    }

    // ---- T-09: straight and premultiplied conversion through the tagged buffer ----

    // Full-intensity red at 50% alpha, decoded exactly as document 21 specifies G1 PNG input:
    // sRGB encoded, straight alpha. The tags drive both conversions; nothing is assumed.
    let decoded = ImageBuffer::from_srgb8_straight(1, 1, &[255, 0, 0, 128]).expect("1x1 decode");
    exact_row(
        &mut rows,
        "T09-TAG",
        "PNG input is tagged sRGB and straight before anything touches it",
        format!("{:?} {:?}", decoded.color_space(), decoded.alpha_mode()),
        format!("{:?} {:?}", ColorSpace::Srgb, AlphaMode::Straight),
    );
    let working = decoded.into_working();
    row(
        &mut rows,
        "T09-PREMUL",
        "sRGB8 (255,0,0,128) straight to linear premultiplied",
        &working.pixel(0, 0),
        &[0.501_960_784_313_725_48, 0.0, 0.0, 0.501_960_784_313_725_48],
        TF_TOL,
    );
    row(
        &mut rows,
        "T09-STRAIGHT",
        "recovering straight linear RGB from that premultiplied pixel",
        &unpremultiply(working.pixel(0, 0)),
        &[1.0, 0.0, 0.0, 0.501_960_784_313_725_48],
        TF_TOL,
    );
    exact_row(
        &mut rows,
        "T09-RT8",
        "the same pixel encoded back out to straight sRGB 8-bit",
        format!("{:?}", working.to_srgb8_straight()),
        format!("{:?}", vec![255u8, 0, 0, 128]),
    );

    // ---- T-09: no duplicate display transform ----

    // A buffer already tagged as working space is not transformed a second time. This is
    // structural rather than conventional: into_working consults the tags.
    let already = vec![0.2f32, 0.4, 0.6, 1.0];
    let again =
        ImageBuffer::new(1, 1, ColorSpace::LinearLight, AlphaMode::Premultiplied, already.clone())
            .expect("1x1")
            .into_working();
    exact_row(
        &mut rows,
        "T09-ONCE",
        "a linear premultiplied buffer entering the working space is bit-identical",
        format!("{:?}", again.data().iter().map(|c| c.to_bits()).collect::<Vec<_>>()),
        format!("{:?}", already.iter().map(|c| c.to_bits()).collect::<Vec<_>>()),
    );

    // Every 8-bit code survives decode, premultiply at full alpha and re-encode. A second,
    // accidental transfer function anywhere on that path would move most of these codes.
    let opaque: Vec<u8> = (0..256u32).flat_map(|v| [v as u8, v as u8, v as u8, 255]).collect();
    let survived = ImageBuffer::from_srgb8_straight(256, 1, &opaque)
        .expect("256x1")
        .into_working()
        .to_srgb8_straight();
    let drifted: Vec<usize> = opaque
        .iter()
        .zip(&survived)
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(i, _)| i)
        .collect();
    exact_row(
        &mut rows,
        "T09-NODUP",
        "all 256 sRGB codes round-trip through the working space unchanged",
        format!("{} channels drifted: {:?}", drifted.len(), drifted),
        "0 channels drifted: []".to_string(),
    );

    // ---- Artifact ----

    let failed: Vec<&Row> = rows.iter().filter(|r| !r.pass).collect();
    let mut md = String::new();
    writeln!(md, "# B-02 fixture table").unwrap();
    writeln!(md).unwrap();
    writeln!(
        md,
        "Generated by `cargo test --test b02_color_alpha`. Requirement R-10; tests T-04 \
         (over-composite of known alpha values, transparent edges) and T-09 (colour and alpha)."
    )
    .unwrap();
    writeln!(md).unwrap();
    writeln!(
        md,
        "Expected values are independently derived, at binary64, in \
         `verification/derive_b02_expected.py` and from `Fixtures/fixture_manifest.json`. None \
         was produced by running the code under test. Actual values print to full f32 \
         precision and expected values to full f64 precision, which is why the two columns \
         differ in digit count."
    )
    .unwrap();
    writeln!(md).unwrap();
    writeln!(md, "Fixture manifest FNV-1a: `{:#018x}`", manifest_hash()).unwrap();
    writeln!(md).unwrap();
    writeln!(md, "**{} of {} rows pass.**", rows.len() - failed.len(), rows.len()).unwrap();
    writeln!(md).unwrap();
    writeln!(md, "| ID | Check | Expected | Actual | Tolerance | Result |").unwrap();
    writeln!(md, "|---|---|---|---|---|---|").unwrap();
    for r in &rows {
        writeln!(
            md,
            "| {} | {} | `{}` | `{}` | {} | {} |",
            r.id,
            r.what,
            r.expected,
            r.actual,
            r.tol,
            if r.pass { "PASS" } else { "**FAIL**" }
        )
        .unwrap();
    }
    writeln!(md).unwrap();
    writeln!(md, "## Not run by this test").unwrap();
    writeln!(md).unwrap();
    writeln!(
        md,
        "These fixtures exist in document 25 and the manifest but belong to later tasks. They \
         are listed so their absence is a stated scope boundary rather than a silent gap."
    )
    .unwrap();
    writeln!(md).unwrap();
    for (id, owner) in [
        (
            "FX-B-001, FX-B-002, FX-B-003 (multiply, screen, add)",
            "blend modes, document 21 line 63; B-02 covers normal-over only",
        ),
        ("FX-E-001, FX-E-002 (exposure)", "effects, R-05, G1-rest"),
        ("FX-T-001, FX-T-002 (tint)", "effects, R-05, G1-rest"),
        ("FX-XF-001 to FX-XF-004 (transforms)", "R-03, task B-05"),
        ("FX-MATTE-001 (matte cycle rejection)", "R-04, task B-06, parked under D-12"),
        ("The matte-mapping half of T-04", "R-04, task B-06, parked under D-12"),
    ] {
        writeln!(md, "- {id} - {owner}").unwrap();
    }
    std::fs::create_dir_all("verification").unwrap();
    std::fs::write("verification/B-02_fixture_table.md", md).unwrap();

    assert!(
        failed.is_empty(),
        "{} fixture rows failed; see verification/B-02_fixture_table.md:\n{}",
        failed.len(),
        failed
            .iter()
            .map(|r| format!("  {}: expected {} got {}", r.id, r.expected, r.actual))
            .collect::<Vec<_>>()
            .join("\n")
    );
}
