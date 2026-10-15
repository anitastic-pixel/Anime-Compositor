//! B-523, D-644 (S-6 of D-638): Distortion, the third sound effect. Each project in
//! `Fixtures/distortion` is mixed by `audio::mix` and held to `tools/distortion_reference.py`'s
//! numbers: the length, the mix at named places, each channel's peak, the warnings, each type's
//! input-to-output curve on the ramp, a sine's 2nd and 3rd harmonics, and Resolution and
//! Downsample exactly. Refused projects, settings kept and saved as written, and decision 3
//! through the commands. Pictures go to `verification/D-644 pictures/`.
//!
//! Writes `verification/D-644_distortion_table.md`.

mod effect_table;

use anime_compositor::audio;
use anime_compositor::command::{Command, Document};
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::model::{Id, LayerKind, Project};
use anime_compositor::persist;
use effect_table::{repo, Table};
use serde_json::Value as J;

fn db(v: f64) -> Option<f64> {
    (v != 0.0).then(|| 20.0 * v.log10())
}

fn shown(v: Option<f64>) -> String {
    v.map_or("none".to_string(), |v| format!("{v:.2} dB"))
}

fn save_png(name: &str, w: usize, h: usize, px: &[u8]) {
    let dir = repo("verification/D-644 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{name}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(px).unwrap();
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
    save_png(name, w, h, &px);
}

/// A curve: sample in across (-1 to +1), sample out up (-1.5 to +1.5); grey axes and the
/// diagonal (no change), the measured ramp in blue.
fn curve_chart(name: &str, pts: &[(f64, f64)]) {
    let (w, h) = (401usize, 601usize);
    let mut px = vec![255u8; w * h * 3];
    let mut put = |x: i64, y: i64, c: [u8; 3]| {
        if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
            px[(y as usize * w + x as usize) * 3..][..3].copy_from_slice(&c);
        }
    };
    let ax = |v: f64| ((v + 1.0) * 200.0).round() as i64;
    let ay = |v: f64| ((1.5 - v) * 200.0).round() as i64;
    for i in 0..601 {
        put(200, i, [200, 200, 200]);
        put(i, 300, [200, 200, 200]);
        put(i, 100, [230, 230, 230]);
        put(i, 500, [230, 230, 230]);
    }
    for i in 0..=400 {
        let v = i as f64 / 200.0 - 1.0;
        put(ax(v), ay(v), [150, 150, 150]);
    }
    for &(x, y) in pts {
        for d in -1..=1 {
            put(ax(x), ay(y) + d, [30, 60, 160]);
        }
    }
    save_png(name, w, h, &px);
}

/// The project with every effect switched off, through the command a person uses.
fn without(project: &Project) -> Project {
    let mut doc = Document::new(project.clone());
    let comp = project.compositions[0].clone();
    for layer in comp.layers_in_order() {
        for fx in &layer.effects {
            doc.apply(Command::SetEffectEnabled {
                composition: comp.id.clone(),
                layer_id: layer.id.clone(),
                instance_id: fx.instance_id.clone(),
                enabled: false,
            })
            .expect("an effect can be switched off");
        }
    }
    doc.project().clone()
}

/// The 2nd and 3rd harmonics against the 1 kHz one, in dB (none below -100), left channel,
/// samples 4,800 to 23,999: the reference tool's measure.
fn harmonics(m: &[f64]) -> [Option<f64>; 2] {
    let w = &m[4800..24000];
    let size = |h: f64| {
        let (mut re, mut im) = (0f64, 0f64);
        for (t, v) in w.iter().enumerate() {
            let a = -2.0 * std::f64::consts::PI * 1000.0 * h * t as f64 / 48000.0;
            re += v * a.cos();
            im += v * a.sin();
        }
        re.hypot(im)
    };
    let f = size(1.0);
    [2.0, 3.0].map(|h| {
        let r = size(h) / f;
        (r > 1e-5).then(|| 20.0 * r.log10())
    })
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

#[test]
fn b523_distortion() {
    let mut t = Table::new(
        "distortion",
        "# D-644: Distortion (B-523)\n\nEach project is mixed by the core and held to the numbers \
         `tools/distortion_reference.py` wrote before the code, by document 21's rule: each channel's \
         sample times 10^(drive / 50) (Drive 0 to 100% is 0 to +40 dB) through the type's curve (Soft \
         Clip tanh, Hard Clip held to -1..1, Saturation 1 u/(1+|u|), Saturation 2 (2/pi) atan(pi u/2), \
         Tube a lopsided tanh, Fuzz a square root held to 1), then Resolution's 2^(bits-1) steps and \
         Downsample's hold counted from the layer's first sample; that times gain / 25, mixed with the \
         clean sound by Mix, then times volume / 11. A keyed setting is read straight between its \
         values on this frame and the next. The effect works before the layer's level; effects run in \
         order; one switched off or outside its ranges lets the sound through. No oversampling: a high \
         Drive folds harmonics above 24 kHz back down (a logged ceiling). Tolerance 2e-5. The test \
         sound is a 1 kHz sine at -6 dB for half a second, then a ramp from -1 to +1 over a second. \
         Pictures: `verification/D-644 pictures/`: each type's curve (`_curve`: sample in across, -1 \
         to +1, out up, -1.5 to +1.5, grey the diagonal, blue the measured ramp), two cycles of the \
         sine (`_sine`: distorted above, clean below), and waveforms (left channel above, right below, `_before` with the effects \
         switched off).\n",
    );
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(t.root.join("expected_distortion.json")).unwrap(),
    )
    .unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut level = Vec::new();
    let mut curves = Vec::new();
    let mut harms = Vec::new();
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
        t.row(
            &format!("mixing warns of: {}", words(&want_w)),
            &words(&mixed),
            mixed == want_w,
        );
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

        if let Some(rows) = case.get("curve").and_then(|c| c.as_array()) {
            let mut worst = 0f64;
            for r in rows {
                let (s, x, y) = (
                    r["sample"].as_u64().unwrap() as usize,
                    r["in"].as_f64().unwrap(),
                    r["out"].as_f64().unwrap(),
                );
                let g = mix[0][s];
                worst = worst.max((g - y).abs());
                curves.push((
                    format!("{name}: in {x:+.3}, out {y:+.5}"),
                    format!("{g:+.5}"),
                    (g - y).abs() <= tol,
                ));
            }
            t.row(
                &format!("the curve: {} ramp samples within {tol}", rows.len()),
                &format!("largest difference {worst:.2e}"),
                worst <= tol,
            );
            let off = without(project);
            let clean = audio::mix(&off, &t.root, &off.compositions[0], from, frames).0;
            let pts: Vec<(f64, f64)> = (24000..72000)
                .step_by(20)
                .map(|s| (clean[0][s], mix[0][s]))
                .collect();
            curve_chart(&format!("{name}_curve"), &pts);
            picture(
                &format!("{name}_sine"),
                &[mix[0][4800..4896].to_vec(), clean[0][4800..4896].to_vec()],
            );
            let want: Vec<Option<f64>> = case["harmonics_db"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_f64())
                .collect();
            let got = harmonics(&mix[0]);
            let ok = want.iter().zip(&got).all(|(w, g)| match (w, g) {
                (Some(w), Some(g)) => (w - g).abs() <= 0.01,
                (None, None) => true,
                _ => false,
            });
            harms.push((
                format!("{name}: 2nd {}, 3rd {}", shown(want[0]), shown(want[1])),
                format!("{}, {}", shown(got[0]), shown(got[1])),
                ok,
            ));
        }
        if name == "fx_dist_009" {
            // Resolution 4 bits: every sample a whole number of eighths.
            let off = mix
                .iter()
                .flatten()
                .map(|v| (v * 8.0 - (v * 8.0).round()).abs())
                .fold(0f64, f64::max);
            t.row(
                "Resolution 4: every sample on a step of 1/8 (exactly)",
                &format!("largest distance from a step {off:.1e}"),
                off <= 1e-12,
            );
        }
        if name == "fx_dist_010" {
            // Downsample 4: samples 4k to 4k + 3 the same, in both channels.
            let held = (0..mix[0].len()).all(|i| (0..2).all(|c| mix[c][i] == mix[c][i - i % 4]));
            let moved = (0..mix[0].len() / 4 - 1)
                .filter(|k| mix[0][4 * k] != mix[0][4 * k + 4])
                .count();
            t.row("Downsample 4: each run of four samples from the layer's first is one value (exactly)", &format!("{}; {moved} of {} runs differ from the next", if held { "held" } else { "not held" }, mix[0].len() / 4 - 1), held && moved > 0);
        }
        if [
            "fx_dist_005",
            "fx_dist_008",
            "fx_dist_009",
            "fx_dist_011",
            "fx_dist_012",
            "fx_dist_015",
        ]
        .contains(&name)
        {
            let off = without(project);
            picture(
                &format!("{name}_before"),
                &audio::mix(&off, &t.root, &off.compositions[0], from, frames).0,
            );
            picture(&format!("{name}_after"), &mix);
        }
    }

    t.heading("The harmonic table: a 1 kHz sine at -6 dB through each type at the starting values, within 0.01 dB");
    for (what, built, ok) in harms {
        t.row(&what, &built, ok);
    }
    t.heading("The curve table: each type's output on the ramp at 21 samples");
    for (what, built, ok) in curves {
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

    t.heading("Saved again");
    let layer_fx = |j: &J| {
        j["compositions"][0]["layers"]
            .as_array()
            .unwrap()
            .iter()
            .find(|l| l["kind"] == "audio")
            .unwrap()["effects"]
            .clone()
    };
    for file in [
        "fx_dist_008.json",
        "fx_dist_012.json",
        "fx_dist_017.json",
        "fx_dist_018.json",
    ] {
        let loaded = t.load(file);
        let written: J = serde_json::from_str(&persist::to_json(
            loaded.document.project(),
            &loaded.preserved,
        ))
        .unwrap();
        let original: J =
            serde_json::from_str(&std::fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
        let kept = same(&layer_fx(&written), &layer_fx(&original));
        t.row(
            &format!("{file}: the layer's effects as written"),
            &if kept {
                "as written".to_string()
            } else {
                layer_fx(&written).to_string()
            },
            kept,
        );
    }

    t.heading("Decision 3: sound effects on sound layers only, through the commands");
    let mut doc = Document::new(t.load("fx_dist_001.json").document.project().clone());
    let comp = doc.project().compositions[0].clone();
    let sound = comp
        .layers_in_order()
        .find(|l| l.kind == LayerKind::Audio)
        .unwrap()
        .id
        .clone();
    let distortion = Effect::Distortion {
        kind: "soft_clip".into(),
        drive: 25.0,
        gain: 25.0,
        wet: 100.0,
        volume: 11.0,
        resolution: 24.0,
        downsample: 1.0,
    };
    let r = doc.apply(Command::AddEffect {
        composition: comp.id.clone(),
        layer_id: sound,
        effect: EffectInstance::new(Id::new("fx-new"), distortion.clone()),
        index: None,
    });
    t.row(
        "Distortion added to a sound layer",
        &r.as_ref()
            .map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()),
        r.is_ok(),
    );
    let mut j: J =
        serde_json::from_str(&std::fs::read_to_string(t.root.join("fx_dist_101.json")).unwrap())
            .unwrap();
    j["compositions"][0]["layers"][0]
        .as_object_mut()
        .unwrap()
        .remove("effects");
    let mut solid = Document::new(
        persist::load_str(&j.to_string())
            .expect("the solid opens")
            .document
            .project()
            .clone(),
    );
    let r = solid.apply(Command::AddEffect {
        composition: comp.id.clone(),
        layer_id: Id::new("solid0"),
        effect: EffectInstance::new(Id::new("fx-new"), distortion),
        index: None,
    });
    t.row(
        "Distortion added to a solid",
        &r.as_ref()
            .map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()),
        r.is_err(),
    );

    t.heading("Speed (release build, this machine)");
    let p = t.load("fx_dist_001.json");
    let project = p.document.project();
    let off = without(project);
    let time = |pr: &Project| {
        let start = std::time::Instant::now();
        for _ in 0..5 {
            std::hint::black_box(audio::mix(pr, &t.root, &pr.compositions[0], 0, 48));
        }
        start.elapsed().as_secs_f64() * 1000.0 / 5.0
    };
    let (a, b) = (time(&off), time(project));
    t.row("fx_dist_001, two seconds mixed: without the distortion, with it (milliseconds, recorded only)", &format!("{a:.1} ms, {b:.1} ms"), true);
    t.finish("D-644_distortion_table.md");
}
