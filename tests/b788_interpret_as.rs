//! B-788, D-909: "Interpret as" per file and colour management's working space, converted at
//! decode (ADR-023 step C3). Writes `verification/B-788_interpret_as_table.md`.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build): every
//! number is OpenColorIO 2.5.1's, or the published curve worked in double precision with
//! OpenColorIO's matrix where D-906 found OpenColorIO leaves the formula, written by
//! `tools/interpret_reference.py` into `Fixtures/colour/interpret/expected_interpretation.json`
//! before this build's code (FX-COLINT-001 to 007, document 25). The chart's ColorChecker patches
//! were made from X-Rite's published sRGB bytes, so tagged ACES2065-1 today's view must give
//! those bytes back.

use std::path::{Path, PathBuf};

use anime_compositor::cache::CelCache;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::model::{Id, Interpretation, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{color, persist, AlphaMode, ColorSpace, WorkingBuffer};
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn dir() -> PathBuf {
    repo("Fixtures/colour/interpret")
}

fn open(name: &str) -> persist::Loaded {
    persist::load(&dir().join(name)).unwrap_or_else(|d| panic!("{name}: {} {}", d.message, d.detail))
}

/// A fresh decode, as export makes it.
fn fresh(path: &Path, read: Interpretation) -> WorkingBuffer {
    (*CelCache::none().decoded(path, read).expect("decode")).clone()
}

/// D-897's bytes path: asked twice of a cache with room for the drawing at 8 bits only, so the
/// second answer is widened from the kept bytes. Whether it came from memory, and its floats.
fn from_bytes(path: &Path, read: Interpretation) -> (bool, WorkingBuffer) {
    let n = fresh(path, read).data().len() / 4;
    let mut tight = CelCache::with_budget(n * 8);
    drop(tight.decoded(path, read).expect("decode"));
    let second = (*tight.decoded(path, read).expect("decode")).clone();
    (tight.hits() == 1, second)
}

fn bits(b: &WorkingBuffer) -> Vec<u32> {
    b.data().iter().map(|v| v.to_bits()).collect()
}

/// |ours - expected| over the colour's size (the largest of 1 and its channels' magnitudes).
fn off(ours: &[f32], expected: &[f64]) -> f64 {
    let scale = expected.iter().fold(1f64, |m, v| m.max(v.abs()));
    ours.iter()
        .zip(expected)
        .map(|(o, e)| (*o as f64 - e).abs())
        .fold(0.0, f64::max)
        / scale
}

const BAR: f64 = 1e-6;

fn numbers(v: &J) -> Vec<f64> {
    v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect()
}

fn frame(project: &Project, cache: &mut CelCache) -> (WorkingBuffer, Vec<DiagnosticId>) {
    let mut log = FrameLog::new(3);
    let image = preview::preview_frame_cached(
        project,
        &Id::new("comp-main"),
        0,
        &dir(),
        PreviewQuality::Full,
        anime_compositor::compose::DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame 0: {}", d.message));
    (image, log.ids_at(0))
}

struct Table {
    checks: Vec<(String, String, String)>,
    chart: Vec<String>,
    ramp: Vec<String>,
}

impl Table {
    fn check(&mut self, what: &str, expected: impl ToString, actual: impl ToString) {
        self.checks
            .push((what.to_string(), expected.to_string(), actual.to_string()));
    }
}

fn rgb(v: &[f64]) -> String {
    format!("{:.6} {:.6} {:.6}", v[0], v[1], v[2])
}

#[test]
fn b788_interpret_as() {
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(dir().join("expected_interpretation.json")).unwrap(),
    )
    .unwrap();
    let mut t = Table { checks: Vec::new(), chart: Vec::new(), ramp: Vec::new() };

    // ---- FX-COLINT-001 and 002: the ACES chart --------------------------------------------
    let chart = &expected["chart"];
    let (w, patch, columns) = (
        chart["width"].as_u64().unwrap() as usize,
        chart["patch"].as_u64().unwrap() as usize,
        chart["columns"].as_u64().unwrap() as usize,
    );
    let patches = chart["patches"].as_array().unwrap();
    for case in chart["cases"].as_array().unwrap() {
        let name = case["project"].as_str().unwrap();
        let (tag, working) = (case["tag"].as_str().unwrap(), case["working"].as_str().unwrap());
        let loaded = open(name);
        let project = loaded.document.project();
        let asset = &project.assets[0];
        let read = project.interpretation(asset);
        let path = dir().join(asset.path.as_deref().unwrap());
        let image = fresh(&path, read);
        let ocio: Vec<Vec<f64>> = case["ocio"].as_array().unwrap().iter().map(numbers).collect();
        let mut worst = vec![0f64; ocio.len()];
        for (i, px) in image.data().chunks_exact(4).enumerate() {
            let p = (i / w / patch) * columns + (i % w) / patch;
            worst[p] = worst[p].max(off(&px[..3], &ocio[p]));
        }
        let within = worst.iter().filter(|o| **o <= BAR).count();
        t.check(
            &format!("{name} (tagged {tag}, working {working}): every pixel of every patch within the bar of OpenColorIO"),
            format!("{} of {}", ocio.len(), ocio.len()),
            format!("{within} of {}", ocio.len()),
        );
        // The real pipeline: the frame drawn from it is the decoded drawing, sample for sample.
        let (drawn, ids) = frame(project, &mut CelCache::none());
        t.check(
            &format!("{name}: frame 0 drawn through the compositor equals the decoded drawing, bit for bit"),
            "true, no diagnostics",
            format!("{}, {}", bits(&drawn) == bits(&image), if ids.is_empty() { "no diagnostics".to_string() } else { format!("{ids:?}") }),
        );
        let view = case.get("view_bytes").and_then(J::as_array);
        let mut view_same = 0;
        for (p, want) in ocio.iter().enumerate() {
            let (x, y) = ((p % columns) * patch + patch / 2, (p / columns) * patch + patch / 2);
            let px = image.as_image().pixel(x, y);
            let shown: Vec<u8> = px[..3].iter().map(|c| color::quantise_u8(color::linear_to_srgb(*c))).collect();
            let expected_bytes: Option<Vec<u8>> = view.map(|v| {
                v[p].as_array().unwrap().iter().map(|b| b.as_u64().unwrap() as u8).collect()
            });
            view_same += usize::from(expected_bytes.as_ref() == Some(&shown));
            t.chart.push(format!(
                "| {name} | {} | {} | {} | {:.1e} | {} | {} |",
                patches[p]["name"].as_str().unwrap(),
                rgb(want),
                rgb(&px[..3].iter().map(|c| *c as f64).collect::<Vec<_>>()),
                worst[p],
                if worst[p] <= BAR { "pass" } else { "FAIL" },
                match &expected_bytes {
                    Some(b) => format!(
                        "{:?} / {:?} {}",
                        b,
                        shown,
                        if *b == shown { "pass" } else { "FAIL" }
                    ),
                    None => "(ACEScg numbers; no view until C5)".to_string(),
                },
            ));
        }
        if view.is_some() {
            t.check(
                &format!("{name}: today's view (sRGB curve, 8 bits) shows each patch as OpenColorIO's numbers give"),
                format!("{} of {}", ocio.len(), ocio.len()),
                format!("{view_same} of {}", ocio.len()),
            );
        }
        if tag == "aces2065-1" && working == "linear-srgb" {
            let mut xrite = 0;
            for (p, made) in patches.iter().enumerate().take(24) {
                let want: Vec<u8> = made["made_from"].as_str().unwrap().trim_start_matches("sRGB bytes ")
                    .split(' ').map(|b| b.parse().unwrap()).collect();
                let (x, y) = ((p % columns) * patch + patch / 2, (p / columns) * patch + patch / 2);
                let px = image.as_image().pixel(x, y);
                let shown: Vec<u8> = px[..3].iter().map(|c| color::quantise_u8(color::linear_to_srgb(*c))).collect();
                xrite += usize::from(shown == want);
            }
            t.check(
                "tagged ACES2065-1, today's view gives back X-Rite's own sRGB bytes for every ColorChecker patch",
                "24 of 24",
                format!("{xrite} of 24"),
            );
        }
        // Each chart project saves back byte for byte.
        let text = std::fs::read_to_string(dir().join(name)).unwrap();
        t.check(
            &format!("{name} saves back byte for byte"),
            "true",
            persist::to_json(project, &loaded.preserved) == text,
        );
    }

    // ---- FX-COLINT-003 and 004: the 8-bit ramp, both decode paths --------------------------
    let ramp = &expected["ramp"];
    for case in ramp["cases"].as_array().unwrap() {
        let (tag, working, judge) = (
            case["tag"].as_str().unwrap(),
            case["working"].as_str().unwrap(),
            case["judge"].as_str().unwrap(),
        );
        let name = if working == "acescg" { "ramp_tags_managed_acescg.json" } else { "ramp_tags.json" };
        let loaded = open(name);
        let project = loaded.document.project();
        let asset = project.assets.iter().find(|a| a.id.as_str() == format!("asset-{tag}")).unwrap();
        let read = project.interpretation(asset);
        let path = dir().join(asset.path.as_deref().unwrap());
        // The colour before alpha: the same file read as premultiplied, so nothing multiplies it.
        let colour = fresh(&path, Interpretation { alpha: AlphaMode::Premultiplied, ..read });
        let (ocio, formula) = (numbers(&case["ocio"]), numbers(&case["formula"]));
        let (mut worst_judge, mut worst_other) = (0f64, 0f64);
        for (i, px) in colour.data().chunks_exact(4).enumerate() {
            let (o, f) = (off(&px[..3], &ocio[i * 3..i * 3 + 3]), off(&px[..3], &formula[i * 3..i * 3 + 3]));
            let (j, other) = if judge == "ocio" { (o, f) } else { (f, o) };
            worst_judge = worst_judge.max(j);
            worst_other = worst_other.max(other);
        }
        // Straight alpha: the colour times alpha, in single precision, every sample.
        let straight = fresh(&path, read);
        let times_alpha = straight
            .data()
            .chunks_exact(4)
            .zip(colour.data().chunks_exact(4))
            .all(|(s, c)| (0..3).all(|k| s[k].to_bits() == (c[k] * c[3]).to_bits()) && s[3] == c[3]);
        let (hit, widened) = from_bytes(&path, read);
        let same = bits(&widened) == bits(&straight);
        let pass = worst_judge <= BAR && times_alpha && hit && same;
        t.ramp.push(format!(
            "| {tag} | {working} | {judge} | {worst_judge:.1e} | {worst_other:.1e} | {times_alpha} | {} | {} |",
            if hit && same { "same bits, from memory" } else { "DIFFERENT" },
            if pass { "pass" } else { "FAIL" }
        ));
        t.check(
            &format!("ramp tagged {tag}, working {working}: within the bar of the {judge} column, times alpha exact, the bytes path the same bits"),
            "pass",
            if pass { "pass" } else { "FAIL" },
        );
    }

    // ---- FX-COLINT-005: off is today -----------------------------------------------------
    let minimal = persist::load(&repo("Fixtures/projects/minimal_project.json")).unwrap();
    let text = std::fs::read_to_string(repo("Fixtures/projects/minimal_project.json")).unwrap();
    let project = minimal.document.project();
    t.check(
        "a project saved before D-909 (minimal_project.json) opens with colour management off, reads its drawings as recorded, and saves byte for byte",
        "off, true, true",
        format!(
            "{}, {}, {}",
            if project.color_management.is_none() { "off" } else { "on" },
            project.assets.iter().all(|a| project.interpretation(a) == a.interpretation),
            persist::to_json(project, &minimal.preserved) == text
        ),
    );
    t.check(
        "today's two words keep their names in the decoded-drawing key (document 27), so no kept drawing is thrown away",
        "Srgb, LinearLight",
        format!("{:?}, {:?}", ColorSpace::from_word("srgb").unwrap(), ColorSpace::from_word("linear-srgb").unwrap()),
    );
    let words: Vec<&str> = ColorSpace::WORDS.iter().map(|(w, _)| *w).collect();
    let back = words.iter().all(|w| ColorSpace::from_word(w).and_then(ColorSpace::word) == Some(*w));
    t.check("each of the ten words reads and writes as itself", "true", back);

    // ---- FX-COLINT-006: a word this build does not have -----------------------------------
    let loaded = open("unknown_space.json");
    let unknown: Vec<String> = loaded
        .warnings
        .iter()
        .filter(|d| d.id == DiagnosticId::ColorSpaceUnknown)
        .map(|d| d.message.clone())
        .collect();
    t.check(
        "unknown_space.json opens with one COLOR_SPACE_UNKNOWN naming the asset and the word",
        "1, true",
        format!("{}, {}", unknown.len(), unknown.first().is_some_and(|m| m.contains("arri-logc4-awg4") && m.contains("aces_chart"))),
    );
    let project = loaded.document.project();
    let (drawn, ids) = frame(project, &mut CelCache::none());
    t.check(
        "its frame 0 is transparent and its log says COLOR_SPACE_UNKNOWN (so an export is marked incomplete)",
        "transparent, true",
        format!(
            "{}, {}",
            if drawn.data().iter().all(|v| *v == 0.0) { "transparent" } else { "drawn" },
            ids.contains(&DiagnosticId::ColorSpaceUnknown)
        ),
    );
    let text = std::fs::read_to_string(dir().join("unknown_space.json")).unwrap();
    t.check(
        "and it saves back byte for byte, the word included",
        "true",
        persist::to_json(project, &loaded.preserved) == text,
    );

    // ---- FX-COLINT-007: refused at opening -----------------------------------------------
    for name in ["refused_acescg_without_management.json", "refused_unknown_working_space.json"] {
        let said = match persist::load(&dir().join(name)) {
            Ok(_) => "opened".to_string(),
            Err(d) => format!(
                "{}, {}",
                d.id.as_str(),
                d.detail.contains("/color_settings/working_space")
            ),
        };
        t.check(
            &format!("{name} is refused at /color_settings/working_space"),
            "PROJECT_SCHEMA_INVALID, true",
            said,
        );
    }

    // ---- Every interpretation project saves back byte for byte ----------------------------
    for name in ["ramp_tags.json", "ramp_tags_managed_acescg.json"] {
        let loaded = open(name);
        let text = std::fs::read_to_string(dir().join(name)).unwrap();
        t.check(
            &format!("{name} saves back byte for byte"),
            "true",
            persist::to_json(loaded.document.project(), &loaded.preserved) == text,
        );
    }

    write(&t);
    let failed: Vec<&String> = t.checks.iter().filter(|(_, e, a)| e != a).map(|(c, _, _)| c).collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

fn write(t: &Table) {
    let passed = t.checks.iter().filter(|(_, e, a)| e == a).count();
    let mut s = String::new();
    s.push_str("# B-788: Interpret as, checked against OpenColorIO\n\n");
    s.push_str(
        "Written by `tests/b788_interpret_as.rs` (D-909). The expected numbers are OpenColorIO \
         2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless), \
         written by `tools/interpret_reference.py` into \
         `Fixtures/colour/interpret/expected_interpretation.json` before the code. The bar: \
         |build - expected| at most 1e-6 of the colour's size (at least 1), at every pixel.\n\n",
    );
    s.push_str(&format!("**{passed} of {} checks pass.**\n\n", t.checks.len()));
    s.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (c, e, a) in &t.checks {
        s.push_str(&format!("| {c} | {e} | {a} | {} |\n", if e == a { "pass" } else { "FAIL" }));
    }
    s.push_str(
        "\n## The chart, patch by patch\n\nEach patch's colour as OpenColorIO gives it and as \
         this build decodes it (linear, in the working space), the worst pixel's difference over \
         the colour's size, and, in today's working space, the bytes today's view shows (sRGB \
         curve, 8 bits): OpenColorIO's / this build's. With ACEScg working there is no view to \
         judge until C5 (D-909 gap 2).\n\n\
         | Project | Patch | OpenColorIO | This build | Worst difference | Within the bar | View bytes |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for row in &t.chart {
        s.push_str(row);
        s.push('\n');
    }
    s.push_str(
        "\n## The 8-bit ramp, every word\n\n256 pixels, every byte as red, alphas 0 to 255. The \
         judge is the column named; the other column's worst difference is shown beside it (the \
         sRGB and ACEScc rows, where OpenColorIO does not compute the published formula, D-906). \
         \"Times alpha\" is the straight-alpha drawing equal to the colour times alpha in single \
         precision, every sample; \"bytes path\" is D-897's drawing kept at 8 bits and widened \
         again.\n\n\
         | Word | Working space | Judge | Worst vs judge | Worst vs the other | Times alpha | Bytes path | Result |\n\
         |---|---|---|---|---|---|---|---|\n",
    );
    for row in &t.ramp {
        s.push_str(row);
        s.push('\n');
    }
    std::fs::write(repo("verification/B-788_interpret_as_table.md"), s).unwrap();
}
