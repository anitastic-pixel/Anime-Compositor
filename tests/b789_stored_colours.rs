//! B-789, D-910: colour management's stored colours, display-value rules and depths (ADR-023
//! step C4). Writes `verification/B-789_stored_colours_table.md` and the pictures in
//! `verification/B-789 pictures/`.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build): every
//! number was written by `tools/c4_reference.py` into `Fixtures/colour/c4/expected_c4.json`
//! before this build's code (FX-COLC4-001 to 005, document 25): the published rules worked in
//! double precision in linear Rec.709, and OpenColorIO 2.5.1's matrix from Linear Rec.709 to
//! ACEScg for the same picture working in ACEScg.

use std::path::PathBuf;

use anime_compositor::cache::CelCache;
use anime_compositor::color::{self, Gamut};
use anime_compositor::command::Command;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::view;
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn dir() -> PathBuf {
    repo("Fixtures/colour/c4")
}

fn open(name: &str) -> persist::Loaded {
    persist::load(&dir().join(name)).unwrap_or_else(|d| panic!("{name}: {} {}", d.message, d.detail))
}

fn frame(project: &Project) -> WorkingBuffer {
    let mut log = FrameLog::new(3);
    preview::preview_frame_cached(
        project,
        &Id::new("comp-main"),
        0,
        &dir(),
        PreviewQuality::Full,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut CelCache::none(),
    )
    .unwrap_or_else(|d| panic!("frame 0: {}", d.message))
}

fn bits(b: &WorkingBuffer) -> Vec<u32> {
    b.data().iter().map(|v| v.to_bits()).collect()
}

const BAR: f64 = 1e-5;

/// The worst |build - expected| over every pixel's three channels; `expected` is one row of 16.
fn worst(image: &WorkingBuffer, expected: &J) -> f64 {
    let row = expected.as_array().unwrap();
    image
        .data()
        .chunks_exact(4)
        .enumerate()
        .flat_map(|(i, px)| {
            let x = row[i % 16].as_array().unwrap();
            (0..3).map(move |c| (px[c] as f64 - x[c].as_f64().unwrap()).abs())
        })
        .fold(0.0, f64::max)
}

/// Today's view of a picture: sRGB curve, 8 bits, opaque here.
fn view(image: &WorkingBuffer) -> Vec<[u8; 3]> {
    image.to_srgb8_straight().chunks_exact(4).map(|p| [p[0], p[1], p[2]]).collect()
}

/// The 16 x 4 picture's view, each pixel as a 16 x 16 square.
fn picture(name: &str, image: &WorkingBuffer) {
    let small = image.to_srgb8_straight();
    let (w, h, s) = (16usize, 4usize, 16usize);
    let mut big = Vec::with_capacity(w * s * h * s * 4);
    for y in 0..h * s {
        for x in 0..w * s {
            let i = ((y / s) * w + x / s) * 4;
            big.extend_from_slice(&small[i..i + 4]);
        }
    }
    let folder = repo("verification/B-789 pictures");
    std::fs::create_dir_all(&folder).unwrap();
    png_out::write_rgba(&folder.join(format!("{name}.png")), w * s, h * s, OutputDepth::Eight, &[], &big)
        .unwrap();
}

#[derive(Default)]
struct Table {
    checks: Vec<(String, String, String)>,
    cases: Vec<String>,
}

impl Table {
    fn check(&mut self, what: &str, expected: impl ToString, actual: impl ToString) {
        self.checks.push((what.to_string(), expected.to_string(), actual.to_string()));
    }
}

fn settings(project: &Project, float_depth: bool, eight_bpc: bool, ae_32bpc: bool) -> Command {
    let comp = &project.compositions[0];
    Command::SetCompositionSettings {
        composition: comp.id.clone(),
        name: comp.name.clone(),
        width: comp.width,
        height: comp.height,
        frame_rate: comp.frame_rate,
        duration_frames: comp.duration_frames,
        sheet_details: comp.sheet_details.clone(),
        background_color: comp.background_color,
        float_depth,
        eight_bpc,
        ae_32bpc,
    }
}

#[test]
fn b789_stored_colours() {
    let expected: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("expected_c4.json")).unwrap()).unwrap();
    let mut t = Table::default();

    // ---- FX-COLC4-001 to 004: the same picture in linear Rec.709 and in ACEScg -------------
    for (case, v) in expected["cases"].as_object().unwrap() {
        let names = [v["projects"][0].as_str().unwrap(), v["projects"][1].as_str().unwrap()];
        let rec = open(names[0]);
        let aces = open(names[1]);
        let (rec, aces) = (rec.document.project(), aces.document.project());
        let (a, b) = (frame(rec), frame(aces));
        let (wa, wb) = (worst(&a, &v["rec709"]), worst(&b, &v["acescg"]));
        // The ACEScg picture taken back to Rec.709 and shown through today's view.
        let mut back = b.clone();
        color::convert_rgba(back.data_mut(), Gamut::Ap1, Gamut::Rec709);
        let bytes: Vec<[u8; 3]> = v["view"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| {
                let p = p.as_array().unwrap();
                [0, 1, 2].map(|c| p[c].as_u64().unwrap() as u8)
            })
            .collect();
        let want: Vec<[u8; 3]> = (0..4).flat_map(|_| bytes.iter().copied()).collect();
        let (va, vb) = (view(&a) == want, view(&back) == want);
        // Colour management on in Rec.709 is colour management off, bit for bit.
        let mut off = rec.clone();
        off.color_management = None;
        let same_off = bits(&frame(&off)) == bits(&a);
        let pass = wa <= BAR && wb <= BAR && va && vb && same_off;
        t.cases.push(format!(
            "| {case} | {wa:.1e} | {wb:.1e} | {} | {} | {} | {} |",
            if va { "same" } else { "DIFFERENT" },
            if vb { "same" } else { "DIFFERENT" },
            if same_off { "same bits" } else { "DIFFERENT" },
            if pass { "pass" } else { "FAIL" }
        ));
        t.check(
            &format!("{case}: Rec.709 and ACEScg within {BAR:.0e} of expected, both the expected bytes through today's view, Rec.709 on = off bit for bit"),
            "pass",
            if pass { "pass" } else { "FAIL" },
        );
        picture(&format!("{case}_rec709"), &a);
        picture(&format!("{case}_acescg_back_to_rec709"), &back);
    }

    // ---- FX-COLC4-005: the two After Effects depths refused with colour management on -------
    for name in expected["refused"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        let said = match persist::load(&dir().join(name)) {
            Ok(_) => "opened".to_string(),
            Err(d) => format!("{}, {}", d.id.as_str(), d.detail.contains("/compositions/0")),
        };
        t.check(
            &format!("{name} is refused at opening, the composition named"),
            "COLOR_DEPTH_REFUSED, true",
            said,
        );
    }
    for name in expected["opens"].as_array().unwrap() {
        let name = name.as_str().unwrap();
        t.check(&format!("{name} (Float, ACEScg) opens"), "true", persist::load(&dir().join(name)).is_ok());
    }
    let loaded = open("float_acescg.json");
    let mut doc = loaded.document;
    let before = persist::to_json(doc.project(), &Default::default());
    for (what, eight, ae) in [("8 bpc (AE)", true, false), ("32 bpc (AE)", false, true)] {
        let project = doc.project().clone();
        let said = match doc.apply(settings(&project, ae, eight, ae)) {
            Ok(_) => "accepted".to_string(),
            Err(d) => d.id.as_str().to_string(),
        };
        let unchanged = persist::to_json(doc.project(), &Default::default()) == before;
        t.check(
            &format!("the composition settings asking for {what} with colour management on are refused and the project is unchanged"),
            "COLOR_DEPTH_REFUSED, true",
            format!("{said}, {unchanged}"),
        );
    }
    for (what, float) in [("Display", false), ("Float", true)] {
        let project = doc.project().clone();
        let said = doc.apply(settings(&project, float, false, false)).is_ok();
        t.check(&format!("and {what} is accepted"), "true", said);
    }
    let off = open("../../float_depth/fx_blend_addf_001.json");
    let mut doc = off.document;
    let project = doc.project().clone();
    t.check(
        "with colour management off, 8 bpc (AE) is accepted as before",
        "true",
        doc.apply(settings(&project, false, true, false)).is_ok(),
    );

    // ---- Gap 8 of B-788: an unknown word saved back with no base record --------------------
    let unknown = persist::load(&repo("Fixtures/colour/interpret/unknown_space.json")).unwrap();
    let text = persist::to_json(unknown.document.project(), &Default::default());
    t.check(
        "an asset tagged with a word this build does not have, saved with no record of the file it came from, keeps the word",
        "true",
        text.contains("\"arri-logc4-awg4\""),
    );

    // ---- The card hands an ACEScg frame to the processor -----------------------------------
    match Gpu::new() {
        Err(why) => t.check("the card hands an ACEScg frame to the processor", "NOT RUN", format!("NOT RUN: no usable card ({why})")),
        Ok(mut gpu) => {
            let loaded = open("overlay_acescg.json");
            let project = loaded.document.project();
            // D-915: the viewer shows the frame through the project's view.
            let cpu = view::encode8(&frame(project), view::Shown::of(project));
            let mut log = FrameLog::new(3);
            let mut cache = CelCache::viewer();
            let (card, ..) = preview::preview_frame_srgb8(
                project,
                &Id::new("comp-main"),
                0,
                &dir(),
                PreviewQuality::Full,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut cache,
                &mut gpu,
            )
            .unwrap();
            let said = log
                .finish()
                .iter()
                .any(|d| d.id == DiagnosticId::GpuPreviewOnCpu && d.message.contains("ACEScg"));
            t.check(
                "the card hands an ACEScg frame to the processor, says so (GPU_PREVIEW_ON_CPU), and the bytes are the processor's",
                "true, true",
                format!("{said}, {}", card == cpu),
            );
        }
    }

    write(&t);
    let failed: Vec<&String> = t
        .checks
        .iter()
        .filter(|(_, e, a)| e != a && e != "NOT RUN")
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

fn write(t: &Table) {
    let passed = t.checks.iter().filter(|(_, e, a)| e == a).count();
    let mut s = String::new();
    s.push_str("# B-789: stored colours, display-value rules and depths with colour management\n\n");
    s.push_str(
        "Written by `tests/b789_stored_colours.rs` (D-910, ADR-023 step C4). The expected numbers \
         were written by `tools/c4_reference.py` into `Fixtures/colour/c4/expected_c4.json` before \
         the code: the published rules in double precision in linear Rec.709, and OpenColorIO \
         2.5.1's Linear Rec.709 to ACEScg matrix for the same picture working in ACEScg. The bar: \
         |build - expected| at most 1e-5 in linear light, at every pixel, and the two working \
         spaces' pictures the same bytes through today's view.\n\n",
    );
    s.push_str(&format!("**{passed} of {} checks pass.**\n\n", t.checks.len()));
    s.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (c, e, a) in &t.checks {
        let r = if e == a { "pass" } else if e == "NOT RUN" { "not run" } else { "FAIL" };
        s.push_str(&format!("| {c} | {e} | {a} | {r} |\n"));
    }
    s.push_str(
        "\n## The four pictures, case by case\n\nThe worst difference from the expected numbers \
         working in linear Rec.709 and in ACEScg; whether today's view (sRGB curve, 8 bits) gives \
         the expected bytes from the Rec.709 picture and from the ACEScg picture taken back to \
         Rec.709; and whether colour management on in Rec.709 draws the same bits as off. The \
         pictures are in `verification/B-789 pictures/`, each pixel shown 16 times as wide and \
         tall.\n\n\
         | Case | Worst, Rec.709 | Worst, ACEScg | View, Rec.709 | View, ACEScg back | On = off | Result |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for row in &t.cases {
        s.push_str(row);
        s.push('\n');
    }
    s.push_str(
        "\nCOLOR_DEPTH_REFUSED (document 28) is the refusal both at opening and in the \
         composition settings.\n",
    );
    std::fs::write(repo("verification/B-789_stored_colours_table.md"), s).unwrap();
}
