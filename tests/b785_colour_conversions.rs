//! B-785 (D-906, ADR-023 step C2): the build's named colour spaces against OpenColorIO 2.5.1's
//! own numbers in `Fixtures/colour/expected_conversions.json` (written by
//! `tools/ocio_reference.py`, never by the build). Writes
//! `verification/B-785_conversion_table.md`.

use anime_compositor::color::{self, Gamut, Space};
use serde_json::Value;
use std::fmt::Write as _;
use std::path::Path;

const BAR: f64 = 1e-6;

/// Conversions where OpenColorIO itself does not follow the published formula, and why. For
/// these the build follows the formula, so the row is judged against the formula column instead.
const KNOWN: [(&[&str], &str, &str); 3] = [
    (
        &["srgb_rec709>linear_rec709", "linear_rec709>srgb_rec709"],
        "srgb",
        "OCIO's sRGB is a smooth power curve whose straight bottom piece ends at 0.0393 \
         (slope 12.923), the standard's at 0.04045 (slope 12.92). Between 0 and 1 the two are at \
         most 6.6e-7 apart in linear light and 6.3e-6 encoded, under a hundredth of a level of \
         255; below 0, where both carry the straight piece on, they differ by 2.5e-4 of the \
         number. The build keeps the standard curve it has used since D-17.",
    ),
    (
        &["acescc>acescg", "acescg>acescc"],
        "acescc",
        "OCIO's ACEScc is a 4096-step table that clamps below -0.36 and above 1.5 and sets \
         negatives to 0 (about 3e-6 off mid-range, 3e-4 when encoding); the build works the \
         published formula exactly.",
    ),
    (
        &["acescg>acescct"],
        "acescct",
        "one sample, linear -0.01, where the straight bottom piece 10.54 x + 0.0729 nearly \
         cancels to -0.0325: OCIO works it in single precision and lands 3.8e-8 off the formula \
         (1.2e-6 of the answer); the build works it in double precision and lands 3.4e-8 from \
         the formula after rounding to single. Every other sample is within the bar.",
    ),
];

fn space(id: &str) -> Space {
    match id {
        "linear_rec709" => color::LINEAR_REC709,
        "acescg" => color::ACESCG,
        "aces2065_1" => color::ACES2065_1,
        "linear_rec2020" => color::LINEAR_REC2020,
        "linear_p3_d65" => color::LINEAR_P3_D65,
        "srgb_rec709" => color::SRGB_REC709,
        "bt1886_rec709" => color::BT1886_REC709,
        "gamma22_rec709" => color::GAMMA22_REC709,
        "acescct" => color::ACESCCT,
        "acescc" => color::ACESCC,
        other => panic!("unknown space {other}"),
    }
}

fn gamut(id: &str) -> Gamut {
    space(id).gamut
}

fn nums(v: &Value) -> Vec<f64> {
    v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect()
}

/// Largest |ours - theirs| / scale and largest |ours - theirs| over all samples.
fn compare(input: &[f64], ours: &[f64], theirs: &[f64], per_channel: bool) -> (f64, f64) {
    let (mut rel, mut abs) = (0f64, 0f64);
    for i in 0..ours.len() {
        let d = (ours[i] - theirs[i]).abs();
        let s = if per_channel {
            input[i].abs().max(theirs[i].abs())
        } else {
            let t = i - i % 3;
            (t..t + 3).fold(0f64, |m, j| m.max(input[j].abs()).max(theirs[j].abs()))
        };
        abs = abs.max(d);
        rel = rel.max(if d == 0.0 { 0.0 } else { d / s });
    }
    (rel, abs)
}

#[test]
fn b785_named_colour_spaces_match_opencolorio() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(root.join("Fixtures/colour/expected_conversions.json"))
        .expect("fixture");
    let doc: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(doc["white_adaptation"], "Bradford");

    let mut md = String::new();
    let _ = writeln!(md, "# B-785: the named colour spaces against OpenColorIO\n");
    let _ = writeln!(
        md,
        "Expected numbers: OpenColorIO {} ({}, {}), from `Fixtures/colour/expected_conversions.json`.\n\
         A row passes when every sample is within {BAR:e} of the sample's size. Nothing in the \
         app uses these spaces yet; colour management is off.\n",
        doc["ocio_version"].as_str().unwrap(),
        doc["config"].as_str().unwrap(),
        doc["optimization"].as_str().unwrap()
    );

    // 1. The matrices, worked out from the primaries, against OCIO's own.
    let _ = writeln!(md, "## Matrices to ACES2065-1 (Bradford white adaptation)\n");
    let _ = writeln!(md, "| From | Largest difference | Result |\n|---|---|---|");
    let mut failures = Vec::new();
    for (id, m) in doc["matrices_to_aces2065_1"].as_object().unwrap() {
        let theirs = nums(m);
        let ours = color::gamut_matrix(gamut(id), Gamut::Ap0);
        let d = (0..9).fold(0f64, |a, k| a.max((ours[k / 3][k % 3] - theirs[k]).abs()));
        let ok = d <= 1e-12;
        if !ok {
            failures.push(format!("matrix {id}: {d:e}"));
        }
        let _ = writeln!(md, "| {id} | {d:.1e} | {} |", if ok { "pass" } else { "FAIL" });
    }

    // 2. Every conversion, sample by sample.
    let samples = |k: &str| -> Vec<[f32; 3]> {
        doc["samples"][k]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| {
                let v = nums(s);
                [v[0] as f32, v[1] as f32, v[2] as f32]
            })
            .collect()
    };
    let (linear, encoded) = (samples("linear"), samples("encoded"));
    let _ = writeln!(
        md,
        "\n## Conversions\n\n| From | To | Samples | Largest difference (of size) | Largest difference | Result |\n|---|---|---|---|---|---|"
    );
    let (mut rows, mut passed, mut notes) = (0, 0, Vec::new());
    for pair in doc["pairs"].as_array().unwrap() {
        let (from, to) = (pair["from"].as_str().unwrap(), pair["to"].as_str().unwrap());
        let pts = if pair["samples"] == "linear" { &linear } else { &encoded };
        let input: Vec<f64> = pts.iter().flatten().map(|&c| c as f64).collect();
        let ours: Vec<f64> = pts
            .iter()
            .flat_map(|&p| color::convert(p, space(from), space(to)))
            .map(f64::from)
            .collect();
        let curve = pair["curve"].as_str();
        let (rel, abs) = compare(&input, &ours, &nums(&pair["ocio"]), curve.is_some());
        let key = format!("{from}>{to}");
        let known = KNOWN.iter().find(|k| k.0.contains(&key.as_str()));
        let result = if rel <= BAR {
            if known.is_some() {
                failures.push(format!("{from} -> {to} now matches OCIO; drop it from KNOWN"));
            }
            "pass".to_string()
        } else if let Some((_, name, why)) = known {
            let (frel, fabs) = compare(&input, &ours, &nums(&pair["formula"]), true);
            if frel > BAR {
                failures.push(format!("{from} -> {to}: formula {frel:e}"));
            }
            if !notes.iter().any(|(n, _)| n == name) {
                notes.push((*name, *why));
            }
            format!(
                "{} by the published formula ({frel:.1e} of size, {fabs:.1e}); OCIO differs, see note {name}",
                if frel <= BAR { "pass" } else { "FAIL" }
            )
        } else {
            failures.push(format!("{from} -> {to}: {rel:e}"));
            "FAIL".to_string()
        };
        rows += 1;
        passed += result.starts_with("pass") as usize;
        let _ = writeln!(
            md,
            "| {from} | {to} | {} | {rel:.1e} | {abs:.1e} | {result} |",
            pts.len()
        );
    }
    let _ = writeln!(md, "\n{passed} of {rows} conversions pass.\n");
    for (name, why) in notes {
        let _ = writeln!(md, "Note {name}: {why}\n");
    }
    std::fs::write(root.join("verification/B-785_conversion_table.md"), md).unwrap();
    assert!(
        failures.is_empty(),
        "{failures:?}; see verification/B-785_conversion_table.md"
    );
}
