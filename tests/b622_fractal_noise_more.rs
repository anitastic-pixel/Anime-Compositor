//! B-622: D-743, Fractal Noise's Rotation, Sub Settings and Overflow, which its engine drew since
//! D-647 but its file never held.
//!
//! Writes `verification/B-622_fractal_noise_more_table.md` and draws pictures into
//! `verification/B-622 pictures/`.
//!
//! No fixture is added or changed. Turbulent Noise is drawn by Fractal Noise's engine (D-445), so
//! `Fixtures/turbulent_noise/expected_turbulent_more.json`, written by
//! `tools/turbulent_more_reference.py` (FX-TURBNOISE-033 to 060), already holds the reference's
//! pixels for these settings. Each case is drawn here as a Fractal Noise with the same settings
//! and held to the same tolerance, 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// Turbulent Noise's fixture `file` with its effect made a Fractal Noise of the same settings:
/// Fractal Noise's own speed 0 and black-to-white colours, as Turbulent Noise draws it (D-445).
fn as_fractal(t: &Table, file: &str) -> J {
    let mut j: J = serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
    for layer in j["compositions"][0]["layers"].as_array_mut().unwrap() {
        for fx in layer["effects"].as_array_mut().into_iter().flatten() {
            if fx["type_id"] == "core.turbulent_noise" {
                fx["type_id"] = J::from("core.fractal_noise");
                let p = &mut fx["parameters"];
                p["speed"] = json!(0);
                p["dark_color"] = json!("#000000");
                p["light_color"] = json!("#ffffff");
            }
        }
    }
    j
}

/// The first effect on the `art` layer of a loaded project.
fn art_effect(loaded: &persist::Loaded) -> Effect {
    loaded
        .document
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .effect
        .clone()
}

/// A Fractal Noise on the street with `p` over its D-128 start.
fn street(dir: &std::path::Path, p: &J) -> Vec<u8> {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    let mut all = json!({"size": 40, "complexity": 6, "contrast": 100, "brightness": 0, "evolution": 0, "speed": 0,
        "seed": 0, "dark_color": "#000000", "light_color": "#ffffff", "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    layer["effects"] = json!([{"instance_id": "fx-1", "type_id": "core.fractal_noise", "enabled": true, "parameters": all}]);
    let loaded = persist::load_str(&project.to_string()).expect("the street reads");
    let mut log = FrameLog::new(3);
    render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        0,
        dir,
        64,
        &mut log,
    )
    .expect("the street draws")
    .to_srgb8_straight()
}

#[test]
fn b622_fractal_noise_more() {
    let mut t = Table::new(
        "turbulent_noise",
        "# B-622: Fractal Noise's Rotation, Sub Settings and Overflow (D-743)\n\n\
         `core.fractal_noise` now keeps in its file, and shows in the panel, the settings its engine \
         drew since D-647 but always at their start: Overflow, Rotation, Perspective Offset, Sub Influence, \
         Sub Scaling, Sub Rotation, Sub Offset and Center Subscale. A file without them reads Allow HDR \
         Results, 0, off, 50, 50, 0, (0, 0), off, which draws every Fractal Noise before D-743 as before \
         (FX-FRACTAL-001 to 038 rerun unchanged in `b71_fractal_noise`). One added from now on starts at \
         After Effects' Sub Influence 70 and Sub Scaling 56 (the app's test \
         `b622_fractal_noise_sub_settings_typed_in_the_panel`). No fixture is added or changed: \
         Turbulent Noise is Fractal Noise's engine (D-445), so each of Turbulent Noise's reference cases \
         FX-TURBNOISE-033 to 058 (`tools/turbulent_more_reference.py`) is drawn below as a Fractal \
         Noise with the same settings and held to its numbers, tolerance 2e-5. Uniform Scaling (FX-TURBNOISE-038, \
         059) and Turbulent Noise's further blending modes (049 to 053, 060) are not Fractal Noise's.\n",
    );

    t.heading("Turbulent Noise's reference cases drawn as Fractal Noise");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_turbulent_more.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();
    let cases: Vec<u32> = (33..=37).chain(39..=48).chain(54..=58).collect();
    for n in &cases {
        let name = format!("FX-TURBNOISE-{n:03}");
        let case = &expected["cases"][&name];
        let file = case["project"].as_str().unwrap();
        let loaded = persist::load_str(&as_fractal(&t, file).to_string())
            .unwrap_or_else(|d| panic!("{file} as Fractal Noise opens: {}", d.message));
        for (frame, pixels) in case["frames"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let d =
                effect_table::largest_difference(&t.render(&loaded.document, frame, 64), pixels);
            t.row(
                &format!(
                    "{name} as Fractal Noise, frame {frame}: {}",
                    case["says"].as_str().unwrap()
                ),
                &format!("largest difference {d:.1e}"),
                d <= tolerance,
            );
        }
        let on_open: Vec<String> = loaded
            .warnings
            .iter()
            .map(|d| d.id.as_str().to_string())
            .collect();
        let mut log = FrameLog::new(8);
        let _ = plan_frame(
            loaded.document.project(),
            &Id::new(MAIN),
            4,
            &t.root,
            &mut log,
        );
        let at_frame: Vec<String> = log
            .finish()
            .iter()
            .map(|d| d.id.as_str().to_string())
            .collect();
        let want: Vec<String> = case
            .get("warning")
            .and_then(J::as_str)
            .map(String::from)
            .into_iter()
            .collect();
        let words = if want.is_empty() {
            String::new()
        } else {
            art_effect(&loaded).why_invalid()
        };
        let named = want.is_empty() || words.contains("Fractal Noise");
        t.row(
            &format!("{name} as Fractal Noise: what opening it and frame 4 warn of, in Fractal Noise's name"),
            &format!("{on_open:?} and {at_frame:?} {words:?}"),
            on_open == want && at_frame == want && named,
        );
    }

    t.heading("The file");
    let old = persist::load(&repo("Fixtures/fractal_noise/fx_fractal_001.json")).unwrap();
    let saved = effect_table::saved(&old);
    let params = &saved["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    let new_keys = [
        "overflow",
        "rotation",
        "uniform_scaling",
        "perspective_offset",
        "sub_influence",
        "sub_scaling",
        "sub_rotation",
        "sub_offset",
        "center_subscale",
    ];
    t.row(
        "fx_fractal_001.json, from before D-743, saves none of the new settings",
        &params.to_string(),
        new_keys.iter().all(|k| params.get(k).is_none()),
    );
    let more = match art_effect(&old) {
        Effect::FractalNoise { more, .. } => more,
        _ => panic!("fx_fractal_001.json holds a Fractal Noise"),
    };
    t.row(
        "and reads them as before D-743: Allow HDR Results, Rotation 0, Sub Influence 50, Sub Scaling 50, Sub Rotation 0, Sub Offset (0, 0), Center Subscale off",
        &format!("{more:?}"),
        more == Default::default(),
    );
    for n in [37u32, 47] {
        let file = format!("fx_turbnoise_{n:03}.json");
        let written = as_fractal(&t, &file);
        let loaded = persist::load_str(&written.to_string()).unwrap();
        let same = effect_table::same_json(&effect_table::saved(&loaded), &written);
        t.row(
            &format!("{file} as Fractal Noise opened and saved holds what it held, keys included"),
            if same { "the same" } else { "differs" },
            same,
        );
    }

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    for n in [36u32, 37, 41, 47, 48] {
        let file = format!("fx_turbnoise_{n:03}.json");
        let project = persist::load_str(&as_fractal(&t, &file).to_string())
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree) = (0, false, true);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) =
                    both(&mut gpu, &project, &Id::new(MAIN), &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
            }
        }
        t.row(
            &format!("{file} as Fractal Noise, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree,
        );
    }

    t.heading("Pictures: the street, in `verification/B-622 pictures/`");
    let dir = repo("verification/B-622 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(
            &dir.join(name),
            TOWN.0,
            TOWN.1,
            OutputDepth::Eight,
            &[],
            bytes,
        )
        .unwrap()
    };
    write("town.png", &town());
    let mut seen: Vec<Vec<u8>> = Vec::new();
    for (name, what, p) in [
        ("1_older_project_50_50.png", "a project from before D-743: Sub Influence 50, Sub Scaling 50, as it always drew", json!({})),
        ("2_new_70_56.png", "a Fractal Noise added from now on: After Effects' Sub Influence 70 and Sub Scaling 56, more fine detail", json!({"sub_influence": 70, "sub_scaling": 56})),
        ("3_sub_77_60.png", "Sub Influence 77.5, Sub Scaling 59.7, as in Cinecom's tutorial: busier still", json!({"sub_influence": 77.5, "sub_scaling": 59.7})),
        ("4_rotation_36.png", "Rotation 36: the clouds turned round the middle", json!({"rotation": 36, "sub_influence": 70, "sub_scaling": 56})),
    ] {
        let img = street(&dir, &p);
        write(name, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, {what}; unlike every picture before"), &format!("its own: {new}"), new);
    }

    t.finish("B-622_fractal_noise_more_table.md");
}
