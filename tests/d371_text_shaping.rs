//! B-250: D-371, text shaping (EFFECTS.md P0-7, part 2): a text layer's words shaped by
//! `rustybuzz` (D-354), so Arabic joins, Devanagari builds its conjuncts, a combining accent sits
//! on its letter, and the font's standard ligatures are on (the owner, 2026-10-09).
//!
//! Every expected number is `Fixtures/text_shaping/expected_text_shaping.json`, written by
//! `tools/text_shaping_reference.py` with HarfBuzz itself (`uharfbuzz`) and fontTools, never the
//! build's code. The letters stay outlines filled on the processor, as D-263's are.

mod effect_table;

use std::fs;

use effect_table::{repo, Table};
use serde_json::Value as J;

use anime_compositor::text::{self, Text};
use anime_compositor::{png_out, OutputDepth, WorkingBuffer};

fn font(name: &str) -> Vec<u8> {
    if name == Text::BUNDLED_FONT {
        return text::font_bytes(name).unwrap().to_vec();
    }
    fs::read(repo(&format!("Fixtures/text_shaping/fonts/{name}"))).unwrap()
}

fn nums(v: &J) -> Vec<f64> {
    v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect()
}

fn far(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

/// The words over a dark grey, so white letters show on any page.
fn picture(file: &str, b: &WorkingBuffer) {
    let dir = repo("verification/D-371 pictures");
    fs::create_dir_all(&dir).unwrap();
    let mut over = WorkingBuffer::opaque(b.width(), b.height());
    for (o, p) in over.data_mut().chunks_mut(4).zip(b.data().chunks(4)) {
        for k in 0..3 {
            o[k] = p[k] + (1.0 - p[3]) * 0.02;
        }
        o[3] = 1.0;
    }
    png_out::write_rgba(&dir.join(format!("{file}.png")), b.width(), b.height(), OutputDepth::Eight, &[], &over.to_srgb8_straight()).unwrap();
}

#[test]
fn d371_text_shaping() {
    // The layout checks name the Noto fonts by file name, as a project does; the build looks for
    // a font in the signed-in person's fonts folder, so this run's is pointed at a copy of the
    // fixture fonts. Nothing on the machine is changed.
    let fonts = repo("target/d371_fonts/Microsoft/Windows/Fonts");
    fs::create_dir_all(&fonts).unwrap();
    for name in ["NotoSansArabic-Regular.ttf", "NotoSansDevanagari-Regular.ttf"] {
        fs::write(fonts.join(name), font(name)).unwrap();
    }
    std::env::set_var("LOCALAPPDATA", repo("target/d371_fonts"));

    let mut t = Table::new(
        "text_shaping",
        "# B-250: text shaping\n\nD-371 (EFFECTS.md P0-7, part 2): a text layer's words shaped by \
         `rustybuzz` (D-354), ligatures on by the owner's choice of 2026-10-09. Every expected \
         number is `Fixtures/text_shaping/expected_text_shaping.json`, written by \
         `tools/text_shaping_reference.py` with HarfBuzz itself (`uharfbuzz` 0.53.3) and fontTools \
         before this code was committed, and printed in document 25 as FX-SHAPE-001 to 023. The \
         shaping checks compare every glyph, left to right: its number in the font, its advance, \
         its offsets and the character its cluster starts at, all whole font units, exactly. The \
         layout checks compare each cluster's anchor within 1e-6 pixel and its drawn box within a \
         quarter of a pixel (the build cuts curves into pieces about 2 pixels long; the reference \
         takes each curve's exact extent), and that every other character of a cluster draws \
         nothing of its own.\n\nBoth the reference and the build slide each TrueType outline so its \
         left edge meets the side bearing the font's hmtx table gives, as FreeType and HarfBuzz \
         do: D-372 (a), the owner's choice of 2026-10-09, built B-251. Before it, FX-SHAPE-020 \
         was in dispute, its \"y\" 0.4 pixel from the reference's.\n",
    );
    let e: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_text_shaping.json")).unwrap()).unwrap();
    let tol = e["tolerance"].as_f64().unwrap();
    let box_tol = e["box_tolerance"].as_f64().unwrap();

    t.heading("FX-SHAPE-001 to 012: every glyph against HarfBuzz (document 25)");
    for (name, case) in e["shaping"].as_object().unwrap() {
        let bytes = font(case["font"].as_str().unwrap());
        let face = rustybuzz::Face::from_slice(&bytes, 0).unwrap();
        let chars: Vec<char> = case["text"].as_str().unwrap().chars().collect();
        let built: Vec<[i64; 5]> = text::shaped(&face, &chars, case["kerning"].as_bool().unwrap())
            .iter()
            .map(|g| [i64::from(g.id.0), i64::from(g.advance), i64::from(g.offset.0), i64::from(g.offset.1), g.ch as i64])
            .collect();
        let want: Vec<[i64; 5]> = case["glyphs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|g| {
                let g: Vec<i64> = g.as_array().unwrap().iter().map(|v| v.as_i64().unwrap()).collect();
                [g[0], g[1], g[2], g[3], g[4]]
            })
            .collect();
        let differ = built.iter().zip(&want).filter(|(b, w)| b != w).count() + built.len().abs_diff(want.len());
        let shown = |gs: &[[i64; 5]]| gs.iter().map(|g| g[0].to_string()).collect::<Vec<_>>().join(" ");
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &if differ == 0 {
                format!("{} glyphs, all the same: {}", built.len(), shown(&built))
            } else {
                format!("{differ} of {} glyphs differ; built {:?}, expected {:?}", want.len(), built, want)
            },
            differ == 0,
        );
    }

    t.heading("FX-SHAPE-020 to 023: laid out and drawn (document 25)");
    for (name, case) in e["layout"].as_object().unwrap() {
        let w = &case["text"];
        let words = Text {
            text: w["text"].as_str().unwrap().into(),
            font: w["font"].as_str().unwrap().into(),
            size: w["size"].as_f64().unwrap(),
            at: [nums(&w["at"])[0], nums(&w["at"])[1]],
            tracking: w["tracking"].as_f64().unwrap(),
            kerning: w["kerning"].as_bool().unwrap(),
            ..Text::default()
        };
        let read = text::font_bytes(&words.font).map(<[u8]>::to_vec);
        let ours = read.as_deref() == Some(font(&words.font).as_slice());
        let placed = text::placed(&words, &[]).unwrap_or_default();
        let want = case["chars"].as_array().unwrap();
        let (mut worst, mut worst_box, mut ok) = (0f64, 0f64, ours && placed.len() == want.len());
        for c in want {
            let i = c["index"].as_u64().unwrap() as usize;
            let Some(b) = placed.iter().find(|p| p.index == i) else {
                ok = false;
                continue;
            };
            ok &= b.ch.to_string() == c["char"].as_str().unwrap();
            if c["head"].as_bool().unwrap() {
                worst = worst.max(far(&b.anchor, &nums(&c["anchor"])));
            }
            worst_box = worst_box.max(match (b.bounds, c["box"].is_null()) {
                (None, true) => 0.0,
                (Some(r), false) => far(&r, &nums(&c["box"])),
                _ => f64::INFINITY,
            });
        }
        let heads = want.iter().filter(|c| c["head"].as_bool().unwrap()).count();
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!(
                "{} characters, {heads} clusters; anchors within {worst:.1e}, boxes within {worst_box:.3} px{}",
                placed.len(),
                if ours { "" } else { "; the font read was not the fixture's copy" }
            ),
            ok && worst <= tol && worst_box <= box_tol,
        );
        if let Some(p) = text::draw(&words, 1000, 400) {
            picture(&name.to_lowercase().replace('-', "_"), &p);
        }
    }
    t.finish("D-371_text_shaping_table.md");
}

/// B-250's timing: a 30-character Latin line in the bundled font and a 31-character Arabic line
/// in Segoe UI (on every Windows machine), each laid out (`text::placed`) and drawn into a 1920 by
/// 1080 picture (`text::draw`) 200 times; the medians in milliseconds, to `D371_OUT`.
#[test]
#[ignore = "B-250: a measurement, run deliberately with --release --ignored"]
fn d371_text_shaping_timing() {
    let lines = [
        ("30-character Latin line, M PLUS Rounded 1c", "Typewriter effect, 30 letters!", Text::BUNDLED_FONT),
        (
            "31-character Arabic line, Segoe UI",
            "\u{0627}\u{0644}\u{0633}\u{0644}\u{0627}\u{0645} \u{0639}\u{0644}\u{064a}\u{0643}\u{0645} \u{0648}\u{0631}\u{062d}\u{0645}\u{0629} \u{0627}\u{0644}\u{0644}\u{0647} \u{0648}\u{0628}\u{0631}\u{0643}\u{0627}\u{062a}\u{0647}",
            "segoeui.ttf",
        ),
    ];
    let mut s = String::from("| Line | Characters | Laid out | Drawn |\n|---|---:|---:|---:|\n");
    for (name, words, font) in lines {
        let t = Text { text: words.into(), font: font.into(), size: 100.0, at: [100.0, 540.0], ..Text::default() };
        assert!(text::draw(&t, 1920, 1080).is_some(), "{font} is on this machine");
        let (mut lay, mut draw) = (Vec::new(), Vec::new());
        for _ in 0..200 {
            let a = std::time::Instant::now();
            std::hint::black_box(text::placed(&t, &[]));
            lay.push(a.elapsed().as_secs_f64() * 1000.0);
            let a = std::time::Instant::now();
            std::hint::black_box(text::draw(&t, 1920, 1080));
            draw.push(a.elapsed().as_secs_f64() * 1000.0);
        }
        lay.sort_by(f64::total_cmp);
        draw.sort_by(f64::total_cmp);
        s += &format!("| {name} | {} | {:.3} | {:.3} |\n", words.chars().count(), lay[100], draw[100]);
    }
    fs::write(std::env::var("D371_OUT").expect("D371_OUT names the file"), s).unwrap();
}
