//! B-524, D-645 (S-7 of D-638): a keyed level, pan, and sound through composition layers. Each
//! project in `Fixtures/sound_layer` is mixed by `audio::mix` and held to
//! `tools/sound_layer_reference.py`'s numbers: the length, the mix at named places, each
//! channel's peak, the warnings, and where every click lands (the click method). Refused
//! projects, levels and pans saved as written, the level command on a keyed level, and which
//! compositions have sound. Pictures go to `verification/D-645 pictures/`.
//!
//! Writes `verification/D-645_sound_layer_table.md`.

mod effect_table;

use anime_compositor::audio;
use anime_compositor::command::{Command, Document};
use anime_compositor::model::LayerKind;
use anime_compositor::persist;
use effect_table::{repo, Table};
use serde_json::Value as J;

fn db(v: f64) -> Option<f64> {
    (v != 0.0).then(|| 20.0 * v.log10())
}

fn shown(v: Option<f64>) -> String {
    v.map_or("silent".to_string(), |v| format!("{v:.2} dB"))
}

/// A waveform: left channel above, right below, full level at the edges of each half.
fn picture(name: &str, mix: &[Vec<f64>; 2]) {
    let (w, h) = (900usize, 200usize);
    let n = mix[0].len().max(1);
    let mut px = vec![255u8; w * h * 3];
    let mut put =
        |x: usize, y: usize, c: [u8; 3]| px[(y.min(h - 1) * w + x) * 3..][..3].copy_from_slice(&c);
    for (c, half) in [(0usize, 0usize), (1, h / 2)] {
        for x in 0..w {
            let (a, b) = (x * n / w, ((x + 1) * n / w).max(x * n / w + 1).min(n));
            // One sample past the column too, so neighbouring columns join up.
            let (lo, hi) = (a..(b + 1).min(n))
                .map(|i| mix[c].get(i).copied().unwrap_or(0.0))
                .fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));
            let row = |v: f64| {
                half + (((1.0 - v.clamp(-1.0, 1.0)) / 2.0) * (h / 2 - 1) as f64).round() as usize
            };
            put(x, half + h / 4, [150, 150, 150]);
            for y in row(hi)..=row(lo) {
                put(x, y, [30, 60, 160]);
            }
        }
    }
    let dir = repo("verification/D-645 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{name}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&px).unwrap();
}

/// The same JSON, a whole number and the same number with a point taken as equal.
fn same(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => x.as_f64() == y.as_f64(),
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(x, y)| same(x, y))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w)))
        }
        _ => a == b,
    }
}

fn ids(list: &[&anime_compositor::diagnostics::Diagnostic]) -> Vec<String> {
    list.iter().map(|d| d.id.as_str().to_string()).collect()
}

/// Every audio layer's level and pan, as the file says them.
fn sound_fields(j: &J) -> Vec<J> {
    let mut out = Vec::new();
    for c in j["compositions"].as_array().unwrap() {
        for l in c["layers"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|l| l["kind"] == "audio")
        {
            out.push(l["gain_db"].clone());
            out.push(l.get("pan").cloned().unwrap_or(J::Null));
        }
    }
    out
}

#[test]
fn b524_sound_layer() {
    let mut t = Table::new(
        "sound_layer",
        "# D-645: a keyed level, pan, and sound through composition layers (B-524)\n\nEach project is \
         mixed by the core and held to the numbers `tools/sound_layer_reference.py` wrote before the \
         code, by document 21's rules: an audio layer's level is a number or keys (-96 to +12 dB), its \
         pan absent, a number or keys (-100 all left to +100 all right; left times min(1, 1 - p/100), \
         right times min(1, 1 + p/100), so the centre is unchanged); a keyed one moves in a straight \
         line across each frame's samples (the level in dB), after the layer's sound effects. A \
         composition layer switched on is heard: frame f plays the inner composition's frame \
         floor(source time) from its first sample, nothing outside the inner composition's frames. \
         Tolerance 2e-5. Clicks are single samples of 0.5 twice a second, so where a click lands says \
         where the sound landed (the click method). Pictures: `verification/D-645 pictures/`, \
         waveforms with the left channel above and the right below.\n",
    );
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(t.root.join("expected_sound_layer.json")).unwrap(),
    )
    .unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut level = Vec::new();
    let mut clicks = Vec::new();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        t.heading(&format!("{name}: {}", case["what"].as_str().unwrap()));
        let loaded = t.load(file);
        let project = loaded.document.project();
        let comp = &project.compositions[0];
        let (from, frames) = (
            case["from"].as_i64().unwrap() as i32,
            case["frames"].as_u64().unwrap() as u32,
        );
        let (mix, said) = audio::mix(project, &t.root, comp, from, frames);
        let n = case["samples"].as_u64().unwrap() as usize;
        t.row(
            &format!("frames {from} to {}: {n} samples", from + frames as i32),
            &format!("{} and {}", mix[0].len(), mix[1].len()),
            mix[0].len() == n && mix[1].len() == n,
        );
        let want_w: Vec<String> = case["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect();
        let opened = ids(&loaded.warnings.iter().collect::<Vec<_>>());
        let mixed = ids(&said.iter().map(|(_, d)| d).collect::<Vec<_>>());
        let words = |v: &[String]| {
            if v.is_empty() {
                "none".to_string()
            } else {
                v.join(", ")
            }
        };
        t.row(
            &format!("opening warns of: {}", words(&want_w)),
            &words(&opened),
            opened == want_w,
        );
        t.row("mixing warns of: none", &words(&mixed), mixed.is_empty());
        let mut worst = 0f64;
        let places = case["places"].as_array().unwrap();
        for (i, v) in places.iter().zip(case["values"].as_array().unwrap()) {
            let i = i.as_u64().unwrap() as usize;
            for (c, v) in v.as_array().unwrap().iter().enumerate() {
                worst = worst.max(
                    mix[c]
                        .get(i)
                        .map_or(f64::INFINITY, |g| (g - v.as_f64().unwrap()).abs()),
                );
            }
        }
        t.row(
            &format!("{} named places within {tol}", places.len()),
            &format!("largest difference {worst:.2e}"),
            worst <= tol,
        );
        let want: Vec<Option<f64>> = case["peak_db"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_f64())
            .collect();
        let got: Vec<Option<f64>> = mix
            .iter()
            .map(|c| db(c.iter().fold(0f64, |m, v| m.max(v.abs()))))
            .collect();
        let ok = want.iter().zip(&got).all(|(w, g)| match (w, g) {
            (Some(w), Some(g)) => (w - g).abs() <= 0.01,
            (None, None) => true,
            _ => false,
        });
        level.push((
            format!("{name}: left {}, right {}", shown(want[0]), shown(want[1])),
            format!("{}, {}", shown(got[0]), shown(got[1])),
            ok,
        ));
        if let Some(list) = case.get("clicks").and_then(|c| c.as_array()) {
            let s0 = case["first_sample"].as_i64().unwrap();
            let want: Vec<(i64, f64)> = list
                .iter()
                .map(|c| (c["sample"].as_i64().unwrap(), c["left"].as_f64().unwrap()))
                .collect();
            let got: Vec<(i64, f64)> = mix[0]
                .iter()
                .enumerate()
                .filter(|(_, v)| v.abs() > 0.1)
                .map(|(i, v)| (s0 + i as i64, *v))
                .collect();
            let ok = want.len() == got.len()
                && want
                    .iter()
                    .zip(&got)
                    .all(|(w, g)| w.0 == g.0 && (w.1 - g.1).abs() <= tol);
            let say = |v: &[(i64, f64)]| {
                v.iter()
                    .map(|(s, x)| format!("{s} ({x:.4})"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            clicks.push((
                format!("{name}: {}", say(&want)),
                if ok {
                    "the same samples".to_string()
                } else {
                    say(&got)
                },
                ok,
            ));
        }
        if [
            "fx_slay_001",
            "fx_slay_002",
            "fx_slay_004",
            "fx_slay_005",
            "fx_slay_008",
            "fx_slay_010",
            "fx_slay_011",
            "fx_slay_013",
            "fx_slay_017",
        ]
        .contains(&name)
        {
            picture(name, &mix);
        }
    }

    t.heading("The click table: every sample louder than 0.1, exactly where the reference has it");
    for (what, built, ok) in clicks {
        t.row(&what, &built, ok);
    }
    t.heading("The level table: each channel's loudest sample, within 0.01 dB");
    for (what, built, ok) in level {
        t.row(&what, &built, ok);
    }

    t.heading("Refused projects");
    for r in expected["refused"].as_array().unwrap() {
        let file = r["project"].as_str().unwrap();
        let got = match persist::load(&t.root.join(file)) {
            Ok(_) => "opened".to_string(),
            Err(d) => d.id.as_str().to_string(),
        };
        t.row(
            &format!("{file}: {}", r["what"].as_str().unwrap()),
            &got,
            got == r["diagnostic"].as_str().unwrap(),
        );
    }

    t.heading("Saved again: every level and pan as written");
    for case in expected["cases"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let loaded = t.load(file);
        let written: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        let original: J =
            serde_json::from_str(&std::fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
        let (w, o) = (sound_fields(&written), sound_fields(&original));
        let kept = w.len() == o.len() && w.iter().zip(&o).all(|(a, b)| same(a, b));
        t.row(
            &format!("{file}"),
            &if kept {
                "as written".to_string()
            } else {
                J::Array(w).to_string()
            },
            kept,
        );
    }

    t.heading("The level command on a keyed level");
    let mut doc = Document::new(t.load("fx_slay_001.json").document.project().clone());
    let comp = doc.project().compositions[0].clone();
    let sound = comp
        .layers_in_order()
        .find(|l| l.kind == LayerKind::Audio)
        .unwrap()
        .id
        .clone();
    let r = doc
        .apply(Command::SetAudioGain {
            composition: comp.id.clone(),
            layer_id: sound.clone(),
            value: -3.0,
        })
        .is_ok();
    let after = doc.project().compositions[0]
        .layers_in_order()
        .find(|l| l.id == sound)
        .unwrap()
        .clone();
    let keys = after.gain_keys.as_ref().map_or(0, |k| k.keyframes().len());
    let base = after.gain_keys.as_ref().and_then(|k| k.base().as_scalar());
    t.row(
        "set to -3 dB: the number and the keys' base are -3, the two keys stay",
        &format!(
            "{}; number {}, base {base:?}, {keys} keys",
            if r { "set" } else { "refused" },
            after.gain_db
        ),
        r && after.gain_db == -3.0 && base == Some(-3.0) && keys == 2,
    );
    let a = audio::mix(
        doc.project(),
        &t.root,
        &doc.project().compositions[0],
        0,
        48,
    )
    .0;
    let b = audio::mix(
        t.load("fx_slay_001.json").document.project(),
        &t.root,
        &comp,
        0,
        48,
    )
    .0;
    t.row(
        "and the sound is the keys' sound, unchanged",
        if a == b { "unchanged" } else { "changed" },
        a == b,
    );

    t.heading("Which compositions have sound (an MP4 gets a sound track when one does)");
    for (file, want) in [
        ("fx_slay_007.json", true),
        ("fx_slay_011.json", true),
        ("fx_slay_012.json", false),
        ("fx_slay_018.json", true),
    ] {
        let p = t.load(file);
        let project = p.document.project();
        let got = audio::has_sound(project, &project.compositions[0]);
        t.row(
            &format!("{file}: {}", if want { "has sound" } else { "has none" }),
            if got { "has sound" } else { "has none" },
            got == want,
        );
    }
    t.heading("Speed (release build, this machine)");
    let time = |file: &str| {
        let p = t.load(file);
        let pr = p.document.project();
        let start = std::time::Instant::now();
        for _ in 0..5 {
            std::hint::black_box(audio::mix(pr, &t.root, &pr.compositions[0], 0, 48));
        }
        start.elapsed().as_secs_f64() * 1000.0 / 5.0
    };
    let (a, b, c) = (
        time("fx_slay_006.json"),
        time("fx_slay_001.json"),
        time("fx_slay_011.json"),
    );
    t.row("two seconds mixed: a plain level and pan, a keyed level, two compositions deep (milliseconds, recorded only)", &format!("{a:.1} ms, {b:.1} ms, {c:.1} ms"), true);
    t.finish("D-645_sound_layer_table.md");
}
