//! B-521, D-642 (S-4 of D-638): Noise Gate, the first sound effect. Each project in
//! `Fixtures/noise_gate` is mixed by `audio::mix` and held to `tools/noise_gate_reference.py`'s
//! numbers: the length, the mix at named places, each channel's peak, the warnings, and when the
//! last gate opens and shuts (read off the mix as what it keeps of what reached it). Refused
//! projects, an unknown sound effect kept as written, and decision 3 (sound effects on sound
//! layers only) through the commands. Pictures before and after go to
//! `verification/D-642 pictures/`.
//!
//! Writes `verification/D-642_noise_gate_table.md`.

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

fn picture(name: &str, mix: &[Vec<f64>; 2]) {
    let (w, h) = (900usize, 200usize);
    let n = mix[0].len().max(1);
    let mut px = vec![255u8; w * h * 3];
    let mut put = |x: usize, y: usize, c: [u8; 3]| px[(y.min(h - 1) * w + x) * 3..][..3].copy_from_slice(&c);
    for (c, half) in [(0usize, 0usize), (1, h / 2)] {
        for x in 0..w {
            let (a, b) = (x * n / w, ((x + 1) * n / w).max(x * n / w + 1).min(n));
            let (lo, hi) = (a..b).map(|i| mix[c].get(i).copied().unwrap_or(0.0)).fold((0f64, 0f64), |(lo, hi), v| (lo.min(v), hi.max(v)));
            let row = |v: f64| half + (((1.0 - v) / 2.0) * (h / 2 - 1) as f64).round() as usize;
            for y in row(hi)..=row(lo) {
                put(x, y, [30, 60, 160]);
            }
            put(x, half + h / 4, [150, 150, 150]);
        }
    }
    let dir = repo("verification/D-642 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{name}.png"))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&px).unwrap();
}

/// The project with the named effects switched off, through the command a person uses.
fn without(project: &Project, off: &[Id]) -> Project {
    let mut doc = Document::new(project.clone());
    let comp = project.compositions[0].clone();
    for layer in comp.layers_in_order() {
        for fx in layer.effects.iter().filter(|e| off.contains(&e.instance_id)) {
            doc.apply(Command::SetEffectEnabled {
                composition: comp.id.clone(),
                layer_id: layer.id.clone(),
                instance_id: fx.instance_id.clone(),
                enabled: false,
            })
            .expect("a gate can be switched off");
        }
    }
    doc.project().clone()
}

/// The last gate that runs: switched on and inside its ranges.
fn last_gate(project: &Project) -> Option<(Id, f64)> {
    project.compositions[0]
        .layers_in_order()
        .flat_map(|l| l.effects.iter())
        .filter(|e| e.enabled && e.effect.is_sound() && e.fault().is_none())
        .last()
        .map(|e| match e.effect {
            Effect::NoiseGate { hold, .. } => (e.instance_id.clone(), hold),
            _ => unreachable!(),
        })
}

/// When the last gate first rises, is full, last hears its threshold, starts closing and is shut,
/// from its gain: what the mix keeps of what reached it, held where nothing reached it.
fn timing(project: &Project, root: &std::path::Path, upto: u32) -> Option<J> {
    let (gate, hold) = last_gate(project)?;
    let comp = &project.compositions[0];
    let out = audio::mix(project, root, comp, 0, upto).0;
    let before = without(project, &[gate]);
    let inn = audio::mix(&before, root, &before.compositions[0], 0, upto).0;
    let gains: Vec<f64> = (0..out[0].len())
        .map(|i| {
            let c = if inn[0][i].abs() >= inn[1][i].abs() { 0 } else { 1 };
            // Where nothing reaches the gate (an earlier gate shut, or before the layer) it is
            // read as shut.
            if inn[c][i] != 0.0 { out[c][i] / inn[c][i] } else { 0.0 }
        })
        .collect();
    let find = |from: Option<usize>, cond: &dyn Fn(f64) -> bool| from.and_then(|f| (f..gains.len()).find(|&i| cond(gains[i])));
    let rise = find(Some(0), &|v| v > 0.0);
    let full = find(rise, &|v| v == 1.0);
    let closing = find(full, &|v| v < 1.0);
    let shut = find(closing, &|v| v == 0.0);
    // The rule: the hold counts down from the last sample at or above the threshold.
    let last_open = closing.map(|c| c as i64 - (hold * 48.0 + 0.5).floor() as i64 - 1);
    let j = |v: Option<usize>| v.map_or(J::Null, |v| J::from(v as i64));
    Some(serde_json::json!({"rise": j(rise), "full": j(full), "last_open": last_open, "closing": j(closing), "shut": j(shut)}))
}

/// The same JSON, a whole number and the same number with a point taken as equal.
fn same(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => x.as_f64() == y.as_f64(),
        (J::Array(x), J::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(x, y)| same(x, y)),
        (J::Object(x), J::Object(y)) => x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| same(v, w))),
        _ => a == b,
    }
}

fn ids(list: &[&anime_compositor::diagnostics::Diagnostic]) -> Vec<String> {
    list.iter().map(|d| d.id.as_str().to_string()).collect()
}

#[test]
fn b521_noise_gate() {
    let mut t = Table::new(
        "noise_gate",
        "# D-642: Noise Gate (B-521)\n\nEach project is mixed by the core and held to the numbers \
         `tools/noise_gate_reference.py` wrote before the code, by document 21's rule: a gain from 0 \
         (shut) at the layer's first sample rises by 1/attack while the louder channel is at or above \
         the threshold or the hold is counting down, and falls by 1/release after; the times are whole \
         samples at 48,000 a second; a keyed setting is read straight between its values on this frame \
         and the next. The gate works before the layer's level; gates run in order; one switched off or \
         outside its ranges lets the sound through. Tolerance 2e-5. The timing is read off the mix: the \
         last gate's gain is what the mix keeps of what reached that gate (shut where nothing reached it). Pictures: `verification/D-642 \
         pictures/` (left channel above, right below, full level at the edges), `_before` with the \
         gates switched off.\n",
    );
    let expected: J = serde_json::from_str(&std::fs::read_to_string(t.root.join("expected_noise_gate.json")).unwrap()).unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut level = Vec::new();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        t.heading(&format!("{name}: {}", case["what"].as_str().unwrap()));
        let loaded = t.load(file);
        let project = loaded.document.project();
        let comp = &project.compositions[0];
        let (from, frames) = (case["from"].as_i64().unwrap() as i32, case["frames"].as_u64().unwrap() as u32);
        let (mix, said) = audio::mix(project, &t.root, comp, from, frames);
        let n = case["samples"].as_u64().unwrap() as usize;
        t.row(&format!("frames {from} to {}: {n} samples", from + frames as i32), &format!("{} and {}", mix[0].len(), mix[1].len()), mix[0].len() == n && mix[1].len() == n);
        let want_w: Vec<String> = case["warnings"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
        let opened = ids(&loaded.warnings.iter().collect::<Vec<_>>());
        let mixed = ids(&said.iter().map(|(_, d)| d).collect::<Vec<_>>());
        let words = |v: &[String]| if v.is_empty() { "none".to_string() } else { v.join(", ") };
        t.row(&format!("opening warns of: {}", words(&want_w)), &words(&opened), opened == want_w);
        t.row(&format!("mixing warns of: {}", words(&want_w)), &words(&mixed), mixed == want_w);
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
        let got_t = timing(project, &t.root, (from + frames as i32) as u32).unwrap_or(J::Null);
        let want_t = &case["timing"];
        let tshow = |v: &J| if v.is_null() { "no gate runs".to_string() } else {
            ["rise", "full", "last_open", "closing", "shut"].iter().map(|k| format!("{k} {}", v[k])).collect::<Vec<_>>().join(", ")
        };
        t.row(&format!("timing: {}", tshow(want_t)), &tshow(&got_t), got_t == *want_t);
        if ["fx_gate_001", "fx_gate_004", "fx_gate_007", "fx_gate_013"].contains(&name) {
            let all: Vec<Id> = comp.layers_in_order().flat_map(|l| l.effects.iter().map(|e| e.instance_id.clone())).collect();
            let off = without(project, &all);
            picture(&format!("{name}_before"), &audio::mix(&off, &t.root, &off.compositions[0], from, frames).0);
            picture(&format!("{name}_after"), &mix);
        }
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
        t.row(&format!("{file}: {}", r["what"].as_str().unwrap()), &got, got == r["diagnostic"].as_str().unwrap());
    }

    let u = &expected["unknown"];
    let file = u["project"].as_str().unwrap();
    t.heading(&format!("{file}: {}", u["what"].as_str().unwrap()));
    let loaded = t.load(file);
    let project = loaded.document.project();
    let (mix, said) = audio::mix(project, &t.root, &project.compositions[0], 0, u["frames"].as_u64().unwrap() as u32);
    let want_w: Vec<String> = u["warnings"].as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
    let opened = ids(&loaded.warnings.iter().collect::<Vec<_>>());
    let mixed = ids(&said.iter().map(|(_, d)| d).collect::<Vec<_>>());
    t.row("opening warns of EFFECT_UNSUPPORTED", &opened.join(", "), opened == want_w);
    t.row("mixing warns of EFFECT_UNSUPPORTED", &mixed.join(", "), mixed == want_w);
    let mut worst = 0f64;
    for (i, v) in u["places"].as_array().unwrap().iter().zip(u["values"].as_array().unwrap()) {
        for (c, v) in v.as_array().unwrap().iter().enumerate() {
            worst = worst.max((mix[c][i.as_u64().unwrap() as usize] - v.as_f64().unwrap()).abs());
        }
    }
    t.row(&format!("the sound unchanged at the named places, within {tol}"), &format!("largest difference {worst:.2e}"), worst <= tol);
    let layer_fx = |j: &J| j["compositions"][0]["layers"].as_array().unwrap().iter().find(|l| l["kind"] == "audio").unwrap()["effects"].clone();
    let written: J = serde_json::from_str(&persist::to_json(project, &loaded.preserved)).unwrap();
    let original: J = serde_json::from_str(&std::fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
    let kept = same(&layer_fx(&written), &layer_fx(&original));
    t.row("saved again, the layer's effects are as written", &if kept { "as written".to_string() } else { layer_fx(&written).to_string() }, kept);
    let gate_file = t.load("fx_gate_012.json");
    let written: J = serde_json::from_str(&persist::to_json(gate_file.document.project(), &gate_file.preserved)).unwrap();
    let original: J = serde_json::from_str(&std::fs::read_to_string(t.root.join("fx_gate_012.json")).unwrap()).unwrap();
    let kept = same(&layer_fx(&written), &layer_fx(&original));
    t.row("fx_gate_012 saved again: its keyed gate as written", &if kept { "as written".to_string() } else { layer_fx(&written).to_string() }, kept);

    t.heading("Decision 3: sound effects on sound layers only, through the commands");
    let mut doc = Document::new(t.load("fx_gate_011.json").document.project().clone());
    let comp = doc.project().compositions[0].clone();
    let sound = comp.layers_in_order().find(|l| l.kind == LayerKind::Audio).unwrap().id.clone();
    let add = |effect: Effect, at: &str| Command::AddEffect {
        composition: comp.id.clone(),
        layer_id: Id::new(at),
        effect: EffectInstance::new(Id::new("fx-new"), effect),
        index: None,
    };
    let gate = Effect::NoiseGate { threshold: -60.0, attack: 10.0, hold: 40.0, release: 220.0 };
    let r = doc.apply(add(Effect::Exposure { stops: 1.0, offset: 0.0, gamma: 1.0, bypass: "off".into() }, sound.as_str()));
    t.row("Exposure added to a sound layer", &r.as_ref().map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()), r.is_err());
    let r = doc.apply(add(gate.clone(), sound.as_str())).map(|_| ());
    t.row("Noise Gate added to a sound layer", &r.as_ref().map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()), r.is_ok());
    // fx_gate_102's solid with its gate taken off, so it opens.
    let mut j: J = serde_json::from_str(&std::fs::read_to_string(t.root.join("fx_gate_102.json")).unwrap()).unwrap();
    j["compositions"][0]["layers"][0].as_object_mut().unwrap().remove("effects");
    let mut solid = Document::new(persist::load_str(&j.to_string()).expect("the solid opens").document.project().clone());
    let r = solid.apply(Command::AddEffect {
        composition: comp.id.clone(),
        layer_id: Id::new("solid0"),
        effect: EffectInstance::new(Id::new("fx-new"), gate),
        index: None,
    });
    t.row("Noise Gate added to a solid", &r.as_ref().map_or_else(|d| format!("refused: {}", d.message), |_| "added".into()), r.is_err());

    t.heading("Speed (release build, this machine)");
    let p = t.load("fx_gate_001.json");
    let project = p.document.project();
    let off = without(project, &[Id::new("gate-0")]);
    let time = |pr: &Project| {
        let start = std::time::Instant::now();
        for _ in 0..5 {
            std::hint::black_box(audio::mix(pr, &t.root, &pr.compositions[0], 0, 48));
        }
        start.elapsed().as_secs_f64() * 1000.0 / 5.0
    };
    let (a, b) = (time(&off), time(project));
    t.row("fx_gate_001, two seconds mixed: without the gate, with it (milliseconds, recorded only)", &format!("{a:.1} ms, {b:.1} ms"), true);
    t.finish("D-642_noise_gate_table.md");
}
