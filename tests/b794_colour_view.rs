//! B-794, D-915: the viewer's colour view with colour management on (ADR-023 step C5). Writes
//! `verification/B-794_colour_view_table.md`.
//!
//! Where the expected values come from (ADR-009, none read off a run of this build): every level
//! is OpenColorIO 2.5.1's, written by `tools/view_reference.py` into
//! `Fixtures/colour/view/expected_view.json` before this build's code (FX-COLVIEW-001 to 006,
//! document 25). The card is judged against this build's processor, as every card table is.

use std::path::PathBuf;
use std::sync::atomic::AtomicBool;

use anime_compositor::cache::CelCache;
use anime_compositor::color::Gamut;
use anime_compositor::command::Command;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::export::{export_sequence, ExportChoices, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::view::{self, Shown, View};
use anime_compositor::{persist, OutputAlpha, OutputDepth, WorkingBuffer};
use serde_json::Value as J;

/// SHA-256 (FIPS 180-4), here so the test needs no new dependency.
fn sha256(bytes: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let mut m = bytes.to_vec();
    m.push(0x80);
    while m.len() % 64 != 56 {
        m.push(0);
    }
    m.extend_from_slice(&((bytes.len() as u64) * 8).to_be_bytes());
    for block in m.chunks_exact(64) {
        let mut w = [0u32; 64];
        for i in 0..16 {
            w[i] = u32::from_be_bytes(block[i * 4..i * 4 + 4].try_into().unwrap());
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16].wrapping_add(s0).wrapping_add(w[i - 7]).wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let t1 = hh.wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(maj);
            (hh, g, f, e, d, c, b, a) = (g, f, e, d.wrapping_add(t1), c, b, a, t1.wrapping_add(t2));
        }
        for (x, y) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *x = x.wrapping_add(y);
        }
    }
    h.iter().map(|x| format!("{x:08x}")).collect()
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn dir() -> PathBuf {
    repo("Fixtures/colour/view")
}

fn open(name: &str) -> persist::Loaded {
    persist::load(&dir().join(name)).unwrap_or_else(|d| panic!("{name}: {} {}", d.message, d.detail))
}

fn text(name: &str) -> String {
    std::fs::read_to_string(dir().join(name)).unwrap()
}

/// The largest channel difference of two RGBA byte pictures, and how many pixels are over 1.
fn levels(a: &[u8], b: &[u8]) -> (u8, usize) {
    let (mut largest, mut over1) = (0u8, 0usize);
    for (a, b) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        over1 += (d > 1) as usize;
    }
    (largest, over1)
}

struct Table {
    checks: Vec<(String, String, String)>,
    cases: Vec<String>,
    card: Vec<String>,
}

impl Table {
    fn check(&mut self, what: &str, expected: impl ToString, actual: impl ToString) {
        self.checks.push((what.to_string(), expected.to_string(), actual.to_string()));
    }
}

/// The frame drawn by the processor and by the card through the project's view.
fn both(project: &Project, gpu: &mut Gpu) -> (WorkingBuffer, Vec<u8>, Vec<u8>, bool) {
    let id = Id::new("comp-main");
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let cpu = preview::preview_frame_cached(project, &id, 0, &dir(), PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("the processor: {}", d.message));
    let bytes = view::encode8(&cpu, Shown::of(project));
    let mut log = FrameLog::new(3);
    let (card, ..) = preview::preview_frame_srgb8(project, &id, 0, &dir(), PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("the card: {}", d.message));
    let said = log.finish();
    let refused = said.iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
    for d in said.iter().filter(|d| d.id == DiagnosticId::GpuPreviewOnCpu) {
        eprintln!("on the processor: {} {}", d.message, d.detail);
    }
    (cpu, bytes, card, refused)
}

/// `name` with its composition set to a depth: "" as saved, else the key set true; and its
/// working space set to `working` where the file has colour management.
fn at_depth(name: &str, key: &str, working: &str) -> Project {
    let mut j: J = serde_json::from_str(&text(name)).unwrap();
    if let Some(c) = j.get_mut("color_settings").and_then(|c| c.as_object_mut()).filter(|c| c.contains_key("color_management")) {
        c.insert("working_space".into(), J::from(working));
    }
    if !key.is_empty() {
        for c in j["compositions"].as_array_mut().unwrap() {
            c.as_object_mut().unwrap().insert(key.into(), J::from(true));
        }
    }
    let loaded = persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
    loaded.document.project().clone()
}

#[test]
fn b794_colour_view() {
    let expected: J = serde_json::from_str(&text("expected_view.json")).unwrap();
    let mut t = Table { checks: Vec::new(), cases: Vec::new(), card: Vec::new() };

    // ---- FX-COLVIEW-001: the table ---------------------------------------------------------
    t.check("SHA-256 of \"abc\" (the standard's own example)", "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad", sha256(b"abc"));
    let sha = sha256(view::TABLE_BYTES);
    t.check("the built-in ACES table's SHA-256", expected["table"]["sha256"].as_str().unwrap(), &sha);
    t.check("its colours (97 cubed)", 97 * 97 * 97, view::table().len() / 3);

    // ---- FX-COLVIEW-002 and 003: the processor against OpenColorIO ---------------------------
    for case in expected["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let working = match case["working"].as_str().unwrap() {
            "acescg" => Gamut::Ap1,
            _ => Gamut::Rec709,
        };
        let barred = case["barred"].as_bool().unwrap();
        let pixels = case["pixels"].as_array().unwrap();
        let mut data = Vec::with_capacity(pixels.len() * 4);
        for p in pixels {
            data.extend(p.as_array().unwrap().iter().map(|v| v.as_f64().unwrap() as f32));
            data.push(1.0);
        }
        let mut buffer = WorkingBuffer::opaque(pixels.len(), 1);
        buffer.data_mut().copy_from_slice(&data);
        for (word, v) in [("aces-2.0-sdr", View::Aces2Sdr), ("un-tone-mapped", View::UnToneMapped)] {
            let ours = view::encode8(&buffer, Shown { view: v, working });
            let want: Vec<u8> = case[word]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|p| {
                    let p: Vec<u8> = p.as_array().unwrap().iter().map(|x| x.as_u64().unwrap() as u8).collect();
                    [p[0], p[1], p[2], 255]
                })
                .collect();
            let (largest, over1) = levels(&ours, &want);
            let within = 100.0 * (pixels.len() - over1) as f64 / pixels.len() as f64;
            // Un-tone-mapped is barred on every case (FX-COLVIEW-003); ACES on the barred ones.
            let judged = barred || v == View::UnToneMapped;
            let pass = within >= 99.9;
            t.cases.push(format!(
                "| {name} | {} | {} | {} | {largest} | {within:.2}% | {} |",
                case["working"].as_str().unwrap(),
                v.name(),
                pixels.len(),
                match (judged, pass) {
                    (true, true) => "pass",
                    (true, false) => "FAIL",
                    (false, _) => "reported",
                }
            ));
            if judged {
                t.check(&format!("{} view, {name}, {}: within 1 level on at least 99.9%", v.name(), case["working"].as_str().unwrap()), "true", pass);
            }
        }
        // Standard is today's encode, whatever the working space.
        let standard = view::encode8(&buffer, Shown { view: View::Standard, working });
        if standard != buffer.to_srgb8_straight() {
            t.check(&format!("Standard on {name} is today's encode"), "true", "false");
        }
    }

    // ---- FX-COLVIEW-004: the card against the processor -------------------------------------
    let mut gpu = Gpu::new().expect("a usable card");
    let about = gpu.about().to_string();
    for name in ["hdr_scene_aces.json", "hdr_scene_un_tone_mapped.json", "hdr_scene_standard.json", "hdr_scene_off.json"] {
        // D-910: an ACEScg frame is drawn by the processor; Rec.709 working keeps the card's view drawn.
        for working in ["acescg", "linear-srgb"] {
        if working == "linear-srgb" && name == "hdr_scene_off.json" {
            continue;
        }
        for (depth, key) in [("8-bit", ""), ("Float", "float_depth"), ("8 bpc (After Effects)", "eight_bpc")] {
            // D-910 refuses 8 bpc (AE) with colour management on at opening; only the off file takes it.
            if key == "eight_bpc" && name != "hdr_scene_off.json" {
                continue;
            }
            let project = at_depth(name, key, working);
            let depth = if working == "acescg" { depth.to_string() } else { format!("{depth}, Rec.709 working") };
            let (cpu, bytes, card, refused) = both(&project, &mut gpu);
            let past1: Vec<usize> = cpu
                .data()
                .chunks_exact(4)
                .enumerate()
                .filter(|(_, p)| p[..3].iter().any(|c| *c > 1.0))
                .map(|(i, _)| i)
                .collect();
            let (largest, over1) = levels(&bytes, &card);
            let largest_past1 = past1
                .iter()
                .map(|&i| levels(&bytes[i * 4..i * 4 + 4], &card[i * 4..i * 4 + 4]).0)
                .max()
                .unwrap_or(0);
            let off_today = if name == "hdr_scene_off.json" { bytes == cpu.to_srgb8_straight() } else { true };
            // Only an 8 bpc (After Effects) frame with a view goes to the processor, on purpose.
            let on_purpose = (key == "eight_bpc" && Shown::of(&project).code() != 0) || project.color_management == Some(Gamut::Ap1);
            let ok = largest <= 1 && refused == on_purpose && (!refused || largest == 0) && off_today;
            t.card.push(format!(
                "| {name} | {} | {depth} | {} | {} | {largest} | {largest_past1} | {over1} | {} |",
                view::Shown::of(&project).view.name(),
                if refused { "processor (refused)" } else { "card" },
                past1.len(),
                if ok { "pass" } else { "FAIL" }
            ));
            t.check(&format!("{name}, {depth}: the card within 1 level of the processor on every pixel"), "true", ok);
            if key.is_empty() && working == "acescg" && name != "hdr_scene_off.json" {
                picture(&cpu, &bytes, Shown::of(&project).view.word().unwrap_or("unknown"));
            }
        }
        }
    }

    // ---- Exports never take the view, on the processor or the card --------------------------
    for (cards, on) in [(0usize, "the processor"), (1, "the card")] {
        let file = |name: &str| {
            let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("b794_export").join(format!("{name}_{cards}"));
            let _ = std::fs::remove_dir_all(&out);
            std::fs::create_dir_all(&out).unwrap();
            let request = ExportRequest {
                composition: Id::new("comp-main"),
                first_frame: 0,
                last_frame: 0,
                output_dir: out.clone(),
                naming: "f_%04d.png".to_string(),
                depth: OutputDepth::Eight,
                alpha: OutputAlpha::Straight,
                tile_size: DEFAULT_TILE_SIZE,
                missing: MissingSource::RenderTransparent,
                format: OutputFormat::Png,
                choices: ExportChoices { on_card: cards > 0, ..ExportChoices::default() },
            };
            let report = export_sequence(open(name).document.project(), &dir(), &request, &AtomicBool::new(false));
            assert!(report.succeeded(), "{name}: {:?}", report.diagnostics);
            std::fs::read(out.join("f_0000.png")).unwrap()
        };
        let aces = file("hdr_scene_aces.json");
        t.check(
            &format!("an export on {on}: ACES 2.0 SDR and Un-tone-mapped files are byte for byte the Standard file"),
            "true, true",
            format!("{}, {}", aces == file("hdr_scene_standard.json"), aces == file("hdr_scene_un_tone_mapped.json")),
        );
    }

    // ---- FX-COLVIEW-005: the project files ---------------------------------------------------
    let shown = |name: &str| Shown::of(open(name).document.project()).view.name();
    t.check("hdr_scene_aces.json (no view) shows", "ACES 2.0 SDR", shown("hdr_scene_aces.json"));
    t.check("hdr_scene_un_tone_mapped.json shows", "Un-tone-mapped", shown("hdr_scene_un_tone_mapped.json"));
    t.check("hdr_scene_standard.json shows", "Standard", shown("hdr_scene_standard.json"));
    t.check("hdr_scene_off.json shows", "Standard", shown("hdr_scene_off.json"));
    t.check("hdr_scene_off_with_view.json shows", "Standard", shown("hdr_scene_off_with_view.json"));
    for name in [
        "hdr_scene_aces.json",
        "hdr_scene_un_tone_mapped.json",
        "hdr_scene_standard.json",
        "hdr_scene_off.json",
        "hdr_scene_off_with_view.json",
        "hdr_scene_unknown_view.json",
    ] {
        let loaded = open(name);
        t.check(&format!("{name} saves back byte for byte"), "true", persist::to_json(loaded.document.project(), &loaded.preserved) == text(name));
    }
    let mut loaded = open("hdr_scene_aces.json");
    let applied = loaded.document.apply(Command::SetColorView { view: View::Standard }).is_ok();
    let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
    t.check("project.set_view standard on hdr_scene_aces.json writes view after alpha_mode", "true, true", format!("{applied}, {}", saved.contains("\"alpha_mode\": \"premultiplied\",\n    \"view\": \"standard\"")));
    loaded.document.apply(Command::SetColorView { view: View::Aces2Sdr }).unwrap();
    t.check(
        "and back to ACES 2.0 SDR the file is as it was, no view",
        "true",
        persist::to_json(loaded.document.project(), &loaded.preserved) == text("hdr_scene_aces.json"),
    );
    let mut loaded = open("hdr_scene_un_tone_mapped.json");
    loaded.document.apply(Command::SetColorView { view: View::Aces2Sdr }).unwrap();
    t.check(
        "a file that had a view keeps the word when set to ACES 2.0 SDR",
        "true",
        persist::to_json(loaded.document.project(), &loaded.preserved).contains("\"view\": \"aces-2.0-sdr\""),
    );
    for name in ["hdr_scene_off.json", "hdr_scene_off_with_view.json"] {
        let mut loaded = open(name);
        let said = match loaded.document.apply(Command::SetColorView { view: View::Standard }) {
            Ok(_) => "applied".to_string(),
            Err(d) => d.message,
        };
        t.check(
            &format!("{name}: project.set_view is refused, the file unchanged"),
            "Colour views need colour management, which this project does not have on., true",
            format!("{said}, {}", persist::to_json(loaded.document.project(), &loaded.preserved) == text(name)),
        );
    }
    let mut loaded = open("hdr_scene_aces.json");
    t.check(
        "a view this build does not have is refused",
        "false",
        loaded.document.apply(Command::SetColorView { view: View::Unknown }).is_ok(),
    );

    // ---- FX-COLVIEW-006: a word this build does not have ------------------------------------
    let loaded = open("hdr_scene_unknown_view.json");
    let said: Vec<&String> = loaded.warnings.iter().filter(|d| d.id == DiagnosticId::ColorViewUnavailable).map(|d| &d.message).collect();
    t.check(
        "hdr_scene_unknown_view.json opens with one COLOR_VIEW_UNAVAILABLE naming aces-1.3-sdr",
        "1, true",
        format!("{}, {}", said.len(), said.first().is_some_and(|m| m.contains("aces-1.3-sdr"))),
    );
    t.check("and shows", "Standard", Shown::of(loaded.document.project()).view.name());
    let mut j: J = serde_json::from_str(&text("hdr_scene_aces.json")).unwrap();
    j["color_settings"]["view"] = J::from(3);
    t.check(
        "a view that is not a word is refused at /color_settings/view",
        "PROJECT_SCHEMA_INVALID, true",
        match persist::load_str(&j.to_string()) {
            Ok(_) => "opened".to_string(),
            Err(d) => format!("{}, {}", d.id.as_str(), d.detail.contains("/color_settings/view")),
        },
    );

    write(&t, &about);
    let failed: Vec<&String> = t.checks.iter().filter(|(_, e, a)| e != a).map(|(c, _, _)| c).collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// The processor's 8-bit frame through one view, blown up (nearest neighbour) for the owner.
fn picture(frame: &WorkingBuffer, bytes: &[u8], word: &str) {
    let (w, h) = (frame.width(), frame.height());
    let k = (600 / w).max(1);
    let mut big = Vec::with_capacity(w * h * k * k * 4);
    for y in 0..h * k {
        for x in 0..w * k {
            let i = ((y / k) * w + x / k) * 4;
            big.extend_from_slice(&bytes[i..i + 4]);
        }
    }
    let out = repo("verification/B-794 pictures");
    std::fs::create_dir_all(&out).unwrap();
    let file = std::fs::File::create(out.join(format!("hdr_scene_{word}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), (w * k) as u32, (h * k) as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&big).unwrap();
}

fn write(t: &Table, about: &str) {
    let passed = t.checks.iter().filter(|(_, e, a)| e == a).count();
    let mut s = String::new();
    s.push_str("# B-794: the viewer's colour view, checked against OpenColorIO\n\n");
    s.push_str(
        "Written by `tests/b794_colour_view.rs` (D-915). The expected levels are OpenColorIO \
         2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless, \
         display \"sRGB - Display\"), written by `tools/view_reference.py` into \
         `Fixtures/colour/view/expected_view.json` before the code. The bar: within 1 level of \
         255 on every channel on at least 99.9% of a barred case's pixels. A view this build does \
         not have is kept, shown as Standard and warned `COLOR_VIEW_UNAVAILABLE`.\n\n",
    );
    s.push_str(&format!("**{passed} of {} checks pass.**\n\n", t.checks.len()));
    s.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (c, e, a) in &t.checks {
        s.push_str(&format!("| {c} | {e} | {a} | {} |\n", if e == a { "pass" } else { "FAIL" }));
    }
    s.push_str(
        "\n## The baked table and the processor against OpenColorIO, case by case\n\n\
         Each case's colours (in the working space named) through this build's processor view, \
         against OpenColorIO's own processor. The pure primary and secondary ramps and the stress \
         colours are reported for ACES 2.0 SDR, not barred (D-915: a 97-point table cannot follow \
         the gamut compression's edge there to 1 level).\n\n\
         | Case | Working space | View | Pixels | Largest difference | Within 1 level | Result |\n\
         |---|---|---|---|---|---|---|\n",
    );
    for row in &t.cases {
        s.push_str(row);
        s.push('\n');
    }
    s.push_str(&format!(
        "\n## The card against the processor\n\nCard: {about}. `media/hdr_scene.exr` through each \
         project, at each depth. 8 bpc (After Effects) frames with a view are drawn by the \
         processor (the card's 8 bpc sum is in display values), so they must match exactly; every \
         other frame must be the card's. Under a view each drawing goes to the card in full \
         precision, never half: in the first run the 8-bit Un-tone-mapped frame was 39 levels off \
         on 7,904 pixels, where the matrix to Rec.709 cancels bright channels. \
         \"Past 1\" counts the processor's pixels with a channel over 1.0, and the largest \
         difference among them.\n\n\
         | Project | View | Depth | Drawn by | Pixels past 1 | Largest difference | Largest past 1 | Pixels over 1 level | Result |\n\
         |---|---|---|---|---|---|---|---|---|\n"
    ));
    for row in &t.card {
        s.push_str(row);
        s.push('\n');
    }
    std::fs::write(repo("verification/B-794_colour_view_table.md"), s).unwrap();
}
