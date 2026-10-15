//! B-522, D-643 (S-5 of D-638): Compressor, the second sound effect. Each project in
//! `Fixtures/compressor` is mixed by `audio::mix` and held to `tools/compressor_reference.py`'s
//! numbers: the length, the mix at named places, each channel's peak, the warnings, the steady
//! gain curve, and how long a step takes to reach 63% and 90% of its change (read off the mix as
//! what the last compressor makes of what reached it; the test sounds are square waves, the
//! same size on every sample). Refused projects, settings kept and saved as written, and
//! decision 3 through the commands. Pictures go to `verification/D-643 pictures/`.
//!
//! Writes `verification/D-643_compressor_table.md`.

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
    v.map_or("silent".to_string(), |v| format!("{v:.2} dB"))
}

fn save_png(name: &str, w: usize, h: usize, px: &[u8]) {
    let dir = repo("verification/D-643 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{name}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(px).unwrap();
}

/// A waveform: left channel above, right below, full level at the edges.
fn picture(name: &str, mix: &[Vec<f64>; 2]) {
    let (w, h) = (900usize, 200usize);
    let n = mix[0].len().max(1);
    let mut px = vec![255u8; w * h * 3];
    let mut put =
        |x: usize, y: usize, c: [u8; 3]| px[(y.min(h - 1) * w + x) * 3..][..3].copy_from_slice(&c);
    for (c, half) in [(0usize, 0usize), (1, h / 2)] {
        for x in 0..w {
            let (a, b) = (x * n / w, ((x + 1) * n / w).max(x * n / w + 1).min(n));
            let (lo, hi) = (a..b)
                .map(|i| mix[c].get(i).copied().unwrap_or(0.0))
                .fold((0f64, 0f64), |(lo, hi), v| (lo.min(v), hi.max(v)));
            let row = |v: f64| half + (((1.0 - v) / 2.0) * (h / 2 - 1) as f64).round() as usize;
            for y in row(hi)..=row(lo) {
                put(x, y, [30, 60, 160]);
            }
            put(x, half + h / 4, [150, 150, 150]);
        }
    }
    save_png(name, w, h, &px);
}

/// The gain curve: level in across, level out up, both -60 to +12 dB; grey lines every 12 dB,
/// the diagonal (no change) darker, the rule's curve in red, the measured steps as blue dots.
fn curve_chart(name: &str, rows: &[(f64, f64, f64)]) {
    let s = 6usize;
    let (w, h) = (72 * s + 1, 72 * s + 1);
    let mut px = vec![255u8; w * h * 3];
    let mut put = |x: i64, y: i64, c: [u8; 3]| {
        if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h {
            px[(y as usize * w + x as usize) * 3..][..3].copy_from_slice(&c);
        }
    };
    let at = |d: f64| ((d + 60.0) * s as f64).round() as i64;
    for k in 0..=6 {
        for i in 0..w as i64 {
            put(k * 12 * s as i64, i, [220, 220, 220]);
            put(i, (h as i64 - 1) - k * 12 * s as i64, [220, 220, 220]);
        }
    }
    for i in 0..w as i64 {
        put(i, h as i64 - 1 - i, [150, 150, 150]);
    }
    for pair in rows.windows(2) {
        let ((x0, want0, _), (x1, want1, _)) = (pair[0], pair[1]);
        for t in 0..=40 {
            let f = t as f64 / 40.0;
            let (x, y) = (
                x0 + (x1 - x0) * f,
                (x0 + want0) + ((x1 + want1) - (x0 + want0)) * f,
            );
            put(at(x), h as i64 - 1 - at(y), [200, 40, 40]);
        }
    }
    for &(x, _, got) in rows {
        let (cx, cy) = (at(x), h as i64 - 1 - at(x + got));
        for dx in -2..=2 {
            for dy in -2..=2 {
                put(cx + dx, cy + dy, [30, 60, 160]);
            }
        }
    }
    save_png(name, w, h, &px);
}

/// The project with the named effects switched off, through the command a person uses.
fn without(project: &Project, off: &[Id]) -> Project {
    let mut doc = Document::new(project.clone());
    let comp = project.compositions[0].clone();
    for layer in comp.layers_in_order() {
        for fx in layer
            .effects
            .iter()
            .filter(|e| off.contains(&e.instance_id))
        {
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

/// The last compressor that runs: switched on and inside its ranges.
fn last_compressor(project: &Project) -> Option<&EffectInstance> {
    project.compositions[0]
        .layers_in_order()
        .flat_map(|l| l.effects.iter())
        .filter(|e| {
            e.enabled && matches!(e.effect, Effect::Compressor { .. }) && e.fault().is_none()
        })
        .last()
}

/// The last compressor's gain in dB at each sample: what the mix makes of what reached it, on
/// the louder channel (none where nothing reached it).
fn gains(project: &Project, root: &std::path::Path, upto: u32) -> Option<Vec<Option<f64>>> {
    let id = last_compressor(project)?.instance_id.clone();
    let out = audio::mix(project, root, &project.compositions[0], 0, upto).0;
    let before = without(project, &[id]);
    let inn = audio::mix(&before, root, &before.compositions[0], 0, upto).0;
    Some(
        (0..out[0].len())
            .map(|i| {
                let c = if inn[0][i].abs() >= inn[1][i].abs() {
                    0
                } else {
                    1
                };
                (inn[c][i] != 0.0).then(|| 20.0 * (out[c][i] / inn[c][i]).abs().log10())
            })
            .collect(),
    )
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
fn b522_compressor() {
    let mut t = Table::new(
        "compressor",
        "# D-643: Compressor (B-522)\n\nEach project is mixed by the core and held to the numbers \
         `tools/compressor_reference.py` wrote before the code, by document 21's rule: the louder \
         channel's level in dB, a knee curve centred on the threshold giving the wanted gain change, \
         the change eased toward it per sample by exp(-1/t), t the attack when it must fall and the \
         release when it may rise (a tenth of the release while Auto Release is on and it is more than \
         3 dB below), then Makeup Gain and an instant Output Limit on the louder channel. A keyed \
         setting is read straight between its values on this frame and the next. The compressor works \
         before the layer's level; effects run in order; one switched off or outside its ranges lets \
         the sound through. Tolerance 2e-5. The test sounds are 1 kHz square waves, so the gain is read \
         exactly off the mix: what the last compressor makes of what reached it. Pictures: \
         `verification/D-643 pictures/`: waveforms (left channel above, right below, `_before` with \
         the effects switched off) and gain curves (`_curve`: level in across, level out up, -60 to \
         +12 dB, grey lines every 12 dB, the diagonal is no change, red the rule, blue the measured \
         steps).\n",
    );
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(t.root.join("expected_compressor.json")).unwrap(),
    )
    .unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut level = Vec::new();
    let mut curves = Vec::new();
    let mut times = Vec::new();
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

        let upto = (from + frames as i32) as u32;
        let measured = gains(project, &t.root, upto);
        if let Some(rows) = case.get("curve").and_then(|c| c.as_array()) {
            let g = measured.as_ref().expect("a compressor runs");
            let (makeup, limit) = match &last_compressor(project).unwrap().effect {
                Effect::Compressor {
                    makeup,
                    output_limit,
                    ..
                } => (*makeup, *output_limit),
                _ => unreachable!(),
            };
            let mut worst = 0f64;
            let mut points = Vec::new();
            for r in rows {
                let (s, x, c) = (
                    r["sample"].as_u64().unwrap() as usize,
                    r["in_db"].as_f64().unwrap(),
                    r["gain_db"].as_f64().unwrap(),
                );
                // The rule's Output Limit holds the louder channel at the limit.
                let want = (c + makeup).min(limit - x);
                let got = g[s].unwrap_or(f64::NAN);
                worst = worst.max((got - want).abs());
                points.push((x, want, got));
            }
            t.row(&format!("the steady curve: {} steps within 0.01 dB of the rule (after the Output Limit)", rows.len()), &format!("largest difference {worst:.2e} dB"), worst <= 0.01);
            for &(x, want, got) in points.iter().filter(|p| (p.0.round() as i64) % 6 == 0) {
                curves.push((
                    format!("{name}: in {x:.1} dB, change {want:.3} dB"),
                    format!("{got:.3} dB"),
                    (got - want).abs() <= 0.01,
                ));
            }
            curve_chart(&format!("{name}_curve"), &points);
        }
        if let Some(rows) = case["timing"].as_array() {
            let g = measured.as_ref().expect("a compressor runs");
            for r in rows {
                let step = r["step"].as_u64().unwrap() as usize;
                let to = r["to_db"].as_f64().unwrap();
                let before = g[step - 1].unwrap();
                let reach = |share: f64| {
                    (step..g.len())
                        .find(|&j| (g[j].unwrap() - before) / (to - before) >= share)
                        .map(|j| j - step)
                };
                let (n63, n90) = (reach(0.63), reach(0.90));
                let want = (
                    r["n63"].as_u64().map(|v| v as usize),
                    r["n90"].as_u64().map(|v| v as usize),
                );
                let ms = |v: Option<usize>| {
                    v.map_or("never".to_string(), |v| {
                        format!("{v} samples ({:.1} ms)", v as f64 / 48.0)
                    })
                };
                times.push((
                    format!("{name}: step at sample {step}, {before:.2} to {to:.2} dB: 63% after {}, 90% after {}", ms(want.0), ms(want.1)),
                    format!("{}, {}", ms(n63), ms(n90)),
                    (n63, n90) == want,
                ));
            }
        }
        if ["fx_comp_004", "fx_comp_007", "fx_comp_013", "fx_comp_016"].contains(&name) {
            let all: Vec<Id> = comp
                .layers_in_order()
                .flat_map(|l| l.effects.iter().map(|e| e.instance_id.clone()))
                .collect();
            let off = without(project, &all);
            picture(
                &format!("{name}_before"),
                &audio::mix(&off, &t.root, &off.compositions[0], from, frames).0,
            );
            picture(&format!("{name}_after"), &mix);
        }
    }

    t.heading("The timing table: how many samples a step takes to cover 63% and 90% of its change");
    for (what, built, ok) in times {
        t.row(&what, &built, ok);
    }
    t.heading("The curve table: every sixth step of each staircase, the steady gain change");
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
    for file in ["fx_comp_009.json", "fx_comp_015.json", "fx_comp_013.json"] {
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
    let mut doc = Document::new(t.load("fx_comp_012.json").document.project().clone());
    let comp = doc.project().compositions[0].clone();
    let sound = comp
        .layers_in_order()
        .find(|l| l.kind == LayerKind::Audio)
        .unwrap()
        .id
        .clone();
    let compressor = Effect::Compressor {
        threshold: -16.0,
        ratio: 3.0,
        knee: 15.0,
        attack: 6.0,
        release: 440.0,
        auto_release: "on".into(),
        makeup: 0.0,
        output_limit: 0.0,
    };
    let r = doc.apply(Command::AddEffect {
        composition: comp.id.clone(),
        layer_id: sound,
        effect: EffectInstance::new(Id::new("fx-new"), compressor.clone()),
        index: None,
    });
    t.row(
        "Compressor added to a sound layer",
        &r.as_ref()
            .map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()),
        r.is_ok(),
    );
    let mut j: J =
        serde_json::from_str(&std::fs::read_to_string(t.root.join("fx_comp_101.json")).unwrap())
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
        effect: EffectInstance::new(Id::new("fx-new"), compressor),
        index: None,
    });
    t.row(
        "Compressor added to a solid",
        &r.as_ref()
            .map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()),
        r.is_err(),
    );

    t.heading("Speed (release build, this machine)");
    let p = t.load("fx_comp_004.json");
    let project = p.document.project();
    let off = without(project, &[Id::new("comp-0")]);
    let time = |pr: &Project| {
        let start = std::time::Instant::now();
        for _ in 0..5 {
            std::hint::black_box(audio::mix(pr, &t.root, &pr.compositions[0], 0, 96));
        }
        start.elapsed().as_secs_f64() * 1000.0 / 5.0
    };
    let (a, b) = (time(&off), time(project));
    t.row("fx_comp_004, four seconds mixed: without the compressor, with it (milliseconds, recorded only)", &format!("{a:.1} ms, {b:.1} ms"), true);
    t.finish("D-643_compressor_table.md");
}
