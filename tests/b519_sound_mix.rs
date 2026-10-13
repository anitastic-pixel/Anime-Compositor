//! B-519, D-640 (S-2 of D-638): mixing a composition's sound. Each project in
//! `Fixtures/sound_mix` is mixed by `audio::mix` for the frames its case names and held to
//! `tools/sound_mix_reference.py`'s numbers: the length at 48,000 a second, the mix at named
//! places in both channels, each channel's peak in decibels (the level table) and the warnings.
//! A picture of each mix, with a grey line at each frame's first sample, goes to
//! `verification/D-640 pictures/`.
//!
//! Writes `verification/D-640_sound_mix_table.md`.

mod effect_table;

use anime_compositor::audio;
use effect_table::{repo, Table};
use serde_json::Value as J;

fn db(v: f64) -> Option<f64> {
    (v != 0.0).then(|| 20.0 * v.log10())
}

fn shown(v: Option<f64>) -> String {
    v.map_or("silent".to_string(), |v| format!("{v:.2} dB"))
}

fn picture(name: &str, mix: &[Vec<f64>; 2], ticks: &[usize]) {
    let (w, h) = (900usize, 200usize);
    let n = mix[0].len().max(1);
    let peak = mix.iter().flatten().fold(1f64, |m, v| m.max(v.abs()));
    let mut px = vec![255u8; w * h * 3];
    let mut put = |x: usize, y: usize, c: [u8; 3]| px[(y.min(h - 1) * w + x) * 3..][..3].copy_from_slice(&c);
    for &t in ticks {
        for y in 0..h {
            put(t * w / n, y, [215, 215, 215]);
        }
    }
    for (c, half) in [(0usize, 0usize), (1, h / 2)] {
        for x in 0..w {
            let (a, b) = (x * n / w, ((x + 1) * n / w).max(x * n / w + 1).min(n));
            let (lo, hi) = (a..b).map(|i| mix[c].get(i).copied().unwrap_or(0.0)).fold((0f64, 0f64), |(lo, hi), v| (lo.min(v), hi.max(v)));
            let row = |v: f64| half + (((1.0 - v / peak) / 2.0) * (h / 2 - 1) as f64).round() as usize;
            for y in row(hi)..=row(lo) {
                put(x, y, [30, 60, 160]);
            }
            put(x, half + h / 4, [150, 150, 150]);
        }
    }
    let dir = repo("verification/D-640 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{name}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&px).unwrap();
}

#[test]
fn b519_sound_mix() {
    let mut t = Table::new(
        "sound_mix",
        "# D-640: mixing a composition's sound (B-519)\n\nEach project is mixed by the core for the \
         frames its case names and held to the numbers `tools/sound_mix_reference.py` wrote before \
         the code, by document 21's rule: each sample belongs to its frame (ADR-018), an audio layer \
         that is on reads its working sound from its local frame's first sample, times its level; \
         the layers are added and nothing is clipped. The window plays this mix. Tolerance 2e-5. \
         Pictures: `verification/D-640 pictures/` (left channel above, right below, a grey line at \
         each frame's first sample, drawn to the loudest sample).\n",
    );
    let expected: J = serde_json::from_str(&std::fs::read_to_string(t.root.join("expected_sound_mix.json")).unwrap()).unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut level = Vec::new();
    let mut mixes = std::collections::HashMap::new();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        t.heading(&format!("{name}: {}", case["what"].as_str().unwrap()));
        let loaded = t.load(file);
        let project = loaded.document.project();
        let comp = &project.compositions[0];
        let (from, frames) = (case["from"].as_i64().unwrap() as i32, case["frames"].as_u64().unwrap() as u32);
        let (mix, left_out) = audio::mix(project, &t.root, comp, from, frames);
        let n = case["samples"].as_u64().unwrap() as usize;
        t.row(&format!("frames {from} to {}: {n} samples", from + frames as i32), &format!("{} and {}", mix[0].len(), mix[1].len()), mix[0].len() == n && mix[1].len() == n);
        let want_w: Vec<&str> = case["warnings"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let got_w: Vec<&str> = left_out.iter().map(|(_, d)| d.id.as_str()).collect();
        t.row(
            &format!("left out: {}", if want_w.is_empty() { "none".to_string() } else { want_w.join(", ") }),
            &if got_w.is_empty() { "none".to_string() } else { left_out.iter().map(|(l, d)| format!("{l} {}", d.id.as_str())).collect::<Vec<_>>().join(", ") },
            got_w == want_w,
        );
        let silent = mix.iter().flatten().all(|v| *v == 0.0);
        let want_silent = case["silent"].as_bool().unwrap();
        t.row(if want_silent { "every sample 0" } else { "not silent" }, if silent { "every sample 0" } else { "not silent" }, silent == want_silent);
        let mut worst = 0f64;
        let places = case["places"].as_array().unwrap();
        for (i, v) in places.iter().zip(case["values"].as_array().unwrap()) {
            let i = i.as_u64().unwrap() as usize;
            for (c, v) in v.as_array().unwrap().iter().enumerate() {
                worst = worst.max(mix[c].get(i).map_or(f64::INFINITY, |g| (g - v.as_f64().unwrap()).abs()));
            }
        }
        t.row(&format!("{} named places within {tol}", places.len()), &format!("largest difference {worst:.2e}"), worst <= tol);
        let want: Vec<Option<f64>> = case["peak_db"].as_array().unwrap().iter().map(|v| v.as_f64()).collect();
        let got: Vec<Option<f64>> = mix.iter().map(|c| db(c.iter().fold(0f64, |m, v| m.max(v.abs())))).collect();
        let ok = want.iter().zip(&got).all(|(w, g)| match (w, g) {
            (Some(w), Some(g)) => (w - g).abs() <= 0.01,
            (None, None) => true,
            _ => false,
        });
        level.push((format!("{name}: left {}, right {}", shown(want[0]), shown(want[1])), format!("{}, {}", shown(got[0]), shown(got[1])), ok));
        let (num, den) = (comp.frame_rate.numerator() as i64, comp.frame_rate.denominator() as i64);
        let first = |f: i64| f * 48000 * den / num;
        let s0 = first(from as i64);
        let ticks: Vec<usize> = (from as i64..from as i64 + frames as i64).map(|f| (first(f) - s0) as usize).collect();
        picture(name, &mix, &ticks);
        mixes.insert(name.to_string(), (s0 as usize, mix));
    }

    t.heading("A stretch alone is the whole mix there");
    let (whole_at, whole) = &mixes["fx_smix_003"];
    let (part_at, part) = &mixes["fx_smix_014"];
    let same = (0..part[0].len()).all(|i| (0..2).all(|c| part[c][i] == whole[c][part_at - whole_at + i]));
    t.row("fx_smix_014 (frames 10 to 14) against fx_smix_003 at the same samples", if same { "every sample the same" } else { "different" }, same);
    let again = audio::mix(t.load("fx_smix_003.json").document.project(), &t.root, &t.load("fx_smix_003.json").document.project().compositions[0], 0, 36).0;
    t.row("mixing again (the working sound kept) gives the same samples", if again == *whole { "the same" } else { "different" }, again == *whole);

    t.heading("The level table: each channel's loudest sample, within 0.01 dB");
    for (what, built, ok) in level {
        t.row(&what, &built, ok);
    }
    t.finish("D-640_sound_mix_table.md");
}
