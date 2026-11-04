//! D-1089 / B-969: Convert Audio to Keyframes, a composition's loudness as three keyed sliders.
//!
//! Writes `verification/B-969_audio_keyframes_table.md`.
//!
//! The expected keys are `Fixtures/audio_keyframes/expected_audio_keyframes.json`, written by
//! `tools/audio_keyframes_reference.py` from `tools/sound_mix_reference.py`'s mix, before the
//! code and sharing nothing with `src/audio.rs`.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::audio::{amplitude, amplitude_layer};
use anime_compositor::command::{Command, Document};
use anime_compositor::expr::{evaluate, Target};
use anime_compositor::model::{Id, Project, Prop};
use anime_compositor::persist::{self, to_json};
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/audio_keyframes")
}

fn load(file: &str) -> persist::Loaded {
    match persist::load(&root().join(file)) {
        Ok(l) => l,
        Err(d) => panic!("{file} did not open: {} {}", d.message, d.detail),
    }
}

fn open(data: &J) -> Result<Project, String> {
    persist::load_str(&data.to_string())
        .map(|l| l.document.project().clone())
        .map_err(|d| d.id.to_string())
}

const NAMES: [&str; 3] = ["Left Channel", "Right Channel", "Both Channels"];

#[test]
fn fx_akf_cases_match_the_reference() {
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root().join("expected_audio_keyframes.json")).unwrap())
            .unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let mut out = String::from(
        "# B-969: Convert Audio to Keyframes (D-1089)\n\n\
         Each project in `Fixtures/audio_keyframes/` is turned into keys by the build \
         (`tests/b969_audio_keyframes.rs`) and held to the numbers \
         `tools/audio_keyframes_reference.py` wrote before the code: on each frame of the work \
         area, Left Channel is 100 times the mean of the left samples' sizes, Right Channel the \
         same for the right, Both Channels their mean (document 21). Tolerance 0.002.\n\n\
         | Case | What | Frames | Frame | Expected L, R, Both | Build L, R, Both | Result |\n\
         |---|---|---|---|---|---|---|\n",
    );
    let mut failed = Vec::new();
    for case in expected["cases"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        let what = case["what"].as_str().unwrap();
        let loaded = load(file);
        let project = loaded.document.project();
        let comp = &project.compositions[0];
        let got = amplitude(project, &root(), comp);
        if case["refused"].as_bool().unwrap() {
            let ok = got.is_err();
            let said = got.err().unwrap_or_else(|| "keys were made".into());
            out += &format!(
                "| {name} | {what} | none | - | refused | {said} | {} |\n",
                if ok { "pass" } else { "FAIL" }
            );
            if !ok {
                failed.push(name.to_string());
            }
            continue;
        }
        let (from, keys, _) = got.unwrap_or_else(|e| panic!("{name}: {e}"));
        let want: Vec<[f64; 3]> = case["keys"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| {
                let k: Vec<f64> = k.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
                [k[0], k[1], k[2]]
            })
            .collect();
        let a = case["from"].as_i64().unwrap() as i32;
        let span = format!("{a} to {}", a + want.len() as i32 - 1);
        if from != a || keys.len() != want.len() {
            failed.push(format!("{name}: frames {from}+{} against {span}", keys.len()));
        }
        let worst = keys
            .iter()
            .zip(&want)
            .flat_map(|(g, w)| (0..3).map(move |c| (g[c] - w[c]).abs()))
            .fold(0.0, f64::max);
        if worst > tol {
            failed.push(format!("{name}: off by {worst}"));
        }
        let three = |v: &[f64; 3]| format!("{:.3}, {:.3}, {:.3}", v[0], v[1], v[2]);
        for n in [0usize, 6, want.len() - 1] {
            if n >= want.len() || n >= keys.len() {
                continue;
            }
            let ok = (0..3).all(|c| (keys[n][c] - want[n][c]).abs() <= tol);
            out += &format!(
                "| {name} | {what} | {span} | {} | {} | {} | {} |\n",
                a + n as i32,
                three(&want[n]),
                three(&keys[n]),
                if ok { "pass" } else { "FAIL" }
            );
        }
        out += &format!(
            "| {name} | every frame | {span} | all {} | | largest difference {worst:.6} | {} |\n",
            want.len(),
            if worst <= tol { "pass" } else { "FAIL" }
        );
    }
    fs::write(repo("verification/B-969_audio_keyframes_table.md"), out).unwrap();
    assert!(failed.is_empty(), "{failed:?}");
}

#[test]
fn the_layer_is_a_null_with_three_named_sliders_an_expression_can_read() {
    let mut loaded = load("fx_akf_001.json");
    let project = loaded.document.project().clone();
    let comp = &project.compositions[0];
    let fx = [Id::new("fx-1"), Id::new("fx-2"), Id::new("fx-3")];
    let (layer, left_out) =
        amplitude_layer(&project, &root(), comp, Id::new("amp"), fx).unwrap();
    assert!(left_out.is_empty());
    assert_eq!(layer.name, "Audio Amplitude");
    let names: Vec<_> = layer.effects.iter().map(|e| e.name.clone().unwrap()).collect();
    assert_eq!(names, NAMES);
    let keys = amplitude(&project, &root(), comp).unwrap().1;
    for (k, e) in layer.effects.iter().enumerate() {
        let track = &e.tracks["slider"][0];
        for f in [0, 6, 12, 47] {
            let v = track.value_at(f).as_scalar().unwrap();
            assert!((v - keys[f as usize][k]).abs() < 1e-9, "{} at {f}", NAMES[k]);
        }
    }
    let at = comp.len();
    loaded
        .document
        .apply(Command::AddLayer {
            composition: comp.id.clone(),
            layer: Box::new(layer),
            index: at,
        })
        .unwrap();
    // The tutorial's own expression, through the layer's name and the effect's name.
    let mut data: J =
        serde_json::from_str(&to_json(loaded.document.project(), &loaded.preserved)).unwrap();
    // The owner's sheet starts from this project, as the command leaves it.
    if let Ok(path) = std::env::var("B969_PROJECT") {
        fs::write(path, data.to_string()).unwrap();
    }
    let text = "thisComp.layer(\"Audio Amplitude\").effect(\"Both Channels\")(\"Slider\")";
    for l in data["compositions"][0]["layers"].as_array_mut().unwrap() {
        if l["id"] == "amp" {
            l["transform"]["rotation"]["expression"] =
                serde_json::json!({ "text": text, "enabled": true });
        }
    }
    let reopened = open(&data).unwrap();
    let comp = &reopened.compositions[0];
    for f in [0, 6, 12] {
        let v = evaluate(comp, &Target::Layer(Id::new("amp")), Prop::Rotation, f)
            .unwrap()
            .as_scalar()
            .unwrap();
        assert!((v - keys[f as usize][2]).abs() < 1e-6, "rotation at {f}: {v}");
    }
}

#[test]
fn an_effect_name_saves_and_an_older_project_is_unchanged() {
    let loaded = load("fx_akf_001.json");
    let project = loaded.document.project();
    let comp = &project.compositions[0];
    let fx = [Id::new("fx-1"), Id::new("fx-2"), Id::new("fx-3")];
    let (layer, _) = amplitude_layer(project, &root(), comp, Id::new("amp"), fx).unwrap();
    let mut doc = Document::new(project.clone());
    doc.apply(Command::AddLayer {
        composition: comp.id.clone(),
        layer: Box::new(layer),
        index: 0,
    })
    .unwrap();
    let saved = to_json(doc.project(), &loaded.preserved);
    let data: J = serde_json::from_str(&saved).unwrap();
    let reopened = open(&data).unwrap();
    let names: Vec<_> = reopened.compositions[0]
        .layer(&Id::new("amp"))
        .unwrap()
        .effects
        .iter()
        .map(|e| e.name.clone())
        .collect();
    assert_eq!(names, NAMES.map(|n| Some(n.to_string())));

    // An older project: its effects have no name, open as before and save without one.
    let mut old = data.clone();
    for l in old["compositions"][0]["layers"].as_array_mut().unwrap() {
        for e in l.get_mut("effects").and_then(J::as_array_mut).into_iter().flatten() {
            e.as_object_mut().unwrap().remove("name");
        }
    }
    let old_project = open(&old).unwrap();
    let amp = old_project.compositions[0].layer(&Id::new("amp")).unwrap();
    assert!(amp.effects.iter().all(|e| e.name.is_none()));
    assert!(!to_json(&old_project, &loaded.preserved).contains("\"name\": \"Left Channel\""));

    // A name that is empty or not a text is refused, never dropped.
    for bad in [J::from(""), J::from(5)] {
        let mut d = data.clone();
        for l in d["compositions"][0]["layers"].as_array_mut().unwrap() {
            if l["id"] == "amp" {
                l["effects"][0]["name"] = bad.clone();
            }
        }
        assert_eq!(open(&d).err().as_deref(), Some("PROJECT_SCHEMA_INVALID"), "{bad}");
    }
}
