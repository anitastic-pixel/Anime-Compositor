//! B-515: D-635, the wireframe views of Wave World and Shatter, after After Effects' Wireframe
//! Preview (Wave World) and its four wireframe views (Shatter). Both are previews that never
//! change the simulation; the drawing rule, the colours and the settings when added are ours,
//! stated in `tools/wave_world_reference.py` (step 8) and `tools/shatter_reference.py` (step 9).
//!
//! Writes `verification/D-635_wireframes_table.md` and draws pictures into
//! `verification/D-635 pictures/`, over black so the white lines show.
//!
//! Every expected pixel is in `Fixtures/wave_world/expected_wave_world.json` (FX-WW-049 to 062)
//! and `Fixtures/shatter/expected_shatter.json` (FX-SH-063 and 084 to 092), written by the tools
//! before this code existed. FX-SH-063 is checked by B-513's test. Tolerance 2e-5. Nothing here
//! is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{repo, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

const WIRE_KEYS: [&str; 4] = [
    "view",
    "horizontal_rotation",
    "vertical_rotation",
    "vertical_scale",
];

/// The `holder` layer's effect in `file`.
fn effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    d.project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("holder"))
        .unwrap()
        .effects[0]
        .effect
        .clone()
}

fn files(prefix: &str, numbers: std::ops::RangeInclusive<u32>) -> Vec<String> {
    numbers.map(|n| format!("{prefix}_{n:03}.json")).collect()
}

/// `Fixtures/{folder}/{file}` at the street's size in `dir` (the street the holder's drawing,
/// Wave World's logo grown to the street as B-510's pictures have it), the holder's settings
/// changed by `p`; `frame` drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, folder: &str, file: &str, p: &J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo(&format!("Fixtures/{folder}/{file}"))).unwrap(),
    )
    .unwrap();
    let a = project["assets"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|a| a["id"] == "asset-shot")
        .unwrap();
    a["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    for l in comp["layers"].as_array_mut().unwrap() {
        if l["id"] == "logo" {
            l["transform"]["scale"]["base"] = json!([1500, 1500]);
        }
    }
    let holder = comp["layers"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|l| l["id"] == "holder")
        .unwrap();
    let parameters = &mut holder["effects"][0]["parameters"];
    for (k, v) in p.as_object().unwrap() {
        parameters[k] = v.clone();
    }
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        dir,
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (drawn.to_srgb8_straight(), said)
}

/// One effect's new cases: the fixtures, the refusals and the round trips.
fn section(
    t: &mut Table,
    name: &str,
    folder: &str,
    prefix: &str,
    numbers: std::ops::RangeInclusive<u32>,
    refused: &[(&str, &str)],
) {
    t.root = repo(&format!("Fixtures/{folder}"));
    let (a, b) = (*numbers.start(), *numbers.end());
    t.heading(&format!(
        "{name}: {}-{a:03} to {b:03} (document 25)",
        prefix.to_uppercase().replace('_', "-")
    ));
    t.fixtures_numbered(&format!("expected_{folder}.json"), numbers.clone());
    for (file, want) in refused {
        // A setting keyed out of range is refused when the file is read, by the warning.
        let mut why = effect(t, file).why_invalid();
        if why.is_empty() {
            why = t
                .load(file)
                .warnings
                .iter()
                .map(|d| format!("{} {}", d.message, d.detail))
                .collect::<Vec<_>>()
                .join(" ");
        }
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.to_lowercase().contains(want),
        );
    }
    let list = files(prefix, numbers);
    t.round_trips(&list.iter().map(String::as_str).collect::<Vec<_>>());
}

/// The pixels of each colour the wireframes draw in: white, red, blue; and any other.
fn colours(bytes: &[u8]) -> [usize; 4] {
    let mut n = [0; 4];
    for p in bytes.chunks_exact(4) {
        match p {
            [_, _, _, 0] => {}
            [255, 255, 255, 255] => n[0] += 1,
            [255, 0, 0, 255] => n[1] += 1,
            [0, 0, 255, 255] => n[2] += 1,
            _ => n[3] += 1,
        }
    }
    n
}

#[test]
fn b515_wireframes() {
    let mut t = Table::new(
        "wave_world",
        "# D-635: the wireframe views of Wave World and Shatter\n\nB-515, After Effects' Wireframe \
         Preview for Wave World and the four wireframe views for Shatter (Wireframe Front View, \
         Wireframe, and each + Forces), under the coordinator's approval of the D-635 design for \
         the owner's /loop request: \"both are previews that never change the simulation, so the \
         choices are ours to make\". Wave World's grid is drawn in white, seen from above at the \
         Vertical Rotation (30 degrees when added) with no perspective, its producers marked red \
         and blue; Shatter's pieces are drawn as white outlines, straight on or through the \
         effect's camera, its force spheres red and blue. The rules are stated in \
         `tools/wave_world_reference.py` (step 8) and `tools/shatter_reference.py` (step 9). \
         Every expected pixel was written by the tools before this code existed. Tolerance \
         2e-5.\n",
    );

    section(
        &mut t,
        "Wave World",
        "wave_world",
        "fx_ww",
        49..=62,
        &[
            ("fx_ww_059.json", "view"),
            ("fx_ww_060.json", "vertical rotation"),
            ("fx_ww_061.json", "vertical scale"),
            ("fx_ww_062.json", "horizontal rotation"),
        ],
    );
    let saved = t.saved_parameters("fx_ww_001.json");
    t.row(
        "fx_ww_001.json, from before D-635, opens as the height map as added and is saved without the view or the Wireframe Controls",
        &format!("{:?}", WIRE_KEYS.iter().filter(|k| saved.get(**k).is_some()).collect::<Vec<_>>()),
        WIRE_KEYS.iter().all(|k| saved.get(*k).is_none()),
    );
    let as_added = effect(&t, "fx_ww_001.json");
    t.row(
        "fx_ww_001.json reads the Wireframe Controls as added: horizontal rotation 0, vertical rotation 30, vertical scale 1",
        &format!("{:?} {:?} {:?}", as_added.get("horizontal_rotation"), as_added.get("vertical_rotation"), as_added.get("vertical_scale")),
        as_added.get("horizontal_rotation") == Some(vec![0.0]) && as_added.get("vertical_rotation") == Some(vec![30.0]) && as_added.get("vertical_scale") == Some(vec![1.0]),
    );
    let saved = t.saved_parameters("fx_ww_051.json");
    t.row(
        "fx_ww_051.json is saved with the view and the two controls it moved, the horizontal rotation it left alone not written",
        &WIRE_KEYS.iter().map(|k| format!("{k} {}", saved.get(*k).map_or("-".to_string(), J::to_string))).collect::<Vec<_>>().join("; "),
        saved["view"] == "wireframe_preview" && saved["vertical_rotation"] == json!(10) && saved["vertical_scale"] == json!(2) && saved.get("horizontal_rotation").is_none(),
    );

    section(
        &mut t,
        "Shatter",
        "shatter",
        "fx_sh",
        84..=92,
        &[("fx_sh_092.json", "view")],
    );

    t.heading("Pictures (verification/D-635 pictures)");
    let dir = std::env::temp_dir().join("b515_pictures");
    fs::create_dir_all(dir.join("media")).unwrap();
    png_out::write_rgba(
        &dir.join("town.png"),
        TOWN.0,
        TOWN.1,
        OutputDepth::Eight,
        &[],
        &effect_table::town(),
    )
    .unwrap();
    fs::copy(
        repo("Fixtures/wave_world/media/logo.png"),
        dir.join("media/logo.png"),
    )
    .unwrap();
    for m in ["back", "ramp", "colours", "cracks"] {
        fs::copy(
            repo(&format!("Fixtures/shatter/media/{m}.png")),
            dir.join(format!("media/{m}.png")),
        )
        .unwrap();
    }
    let out = repo("verification/D-635 pictures");
    fs::create_dir_all(&out).unwrap();
    let water = json!({"view": "wireframe_preview", "ground_layer": "", "grid_resolution": 40, "steepness": 0.25, "producer_1_position": [50, 50],
        "producer_1_height": 0.1, "producer_1_width": 0.1, "producer_1_amplitude": 0.5, "producer_1_frequency": 1});
    let with = |base: &J, extra: J| {
        let mut p = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        p
    };
    for (name, folder, file, p, frame, want) in [
        (
            "1_wave_world_wireframe_as_added",
            "wave_world",
            "fx_ww_049.json",
            water.clone(),
            24,
            [true, true, false],
        ),
        (
            "2_wave_world_wireframe_turned_two_producers",
            "wave_world",
            "fx_ww_049.json",
            with(
                &water,
                json!({"horizontal_rotation": 30, "vertical_rotation": 45, "vertical_scale": 3, "producer_1_position": [30, 50], "producer_2_position": [70, 50], "producer_2_amplitude": 0.5, "producer_2_frequency": 2.5}),
            ),
            36,
            [true, true, true],
        ),
        (
            "3_wave_world_wireframe_island_shore",
            "wave_world",
            "fx_ww_058.json",
            with(
                &water,
                json!({"ground_layer": "logo", "steepness": 0.6, "producer_1_position": [25, 50], "producer_1_amplitude": 1.5, "producer_1_frequency": 2, "vertical_scale": 2}),
            ),
            48,
            [true, true, false],
        ),
        (
            "4_shatter_wireframe_front",
            "shatter",
            "fx_sh_001.json",
            json!({"view": "wireframe_front"}),
            4,
            [true, false, false],
        ),
        (
            "5_shatter_wireframe_forces_turned_camera",
            "shatter",
            "fx_sh_001.json",
            json!({"view": "wireframe_forces", "camera_y_rotation": 30, "force_2_radius": 0.2, "force_2_position": [20, 30]}),
            4,
            [true, true, true],
        ),
    ] {
        let (bytes, said) = picture(&dir, folder, file, &p, frame);
        let n = colours(&bytes);
        // Saved over black, so the white lines show in any picture viewer; every drawn pixel is
        // opaque (checked below), so only the clear ones change.
        let shown: Vec<u8> = bytes
            .chunks_exact(4)
            .flat_map(|p| {
                if p[3] == 0 {
                    [0, 0, 0, 255]
                } else {
                    [p[0], p[1], p[2], p[3]]
                }
            })
            .collect();
        png_out::write_rgba(
            &out.join(format!("{name}.png")),
            TOWN.0,
            TOWN.1,
            OutputDepth::Eight,
            &[],
            &shown,
        )
        .unwrap();
        t.row(
            &format!("{name}.png: drawn without a warning, only in white, red and blue, each colour there as asked"),
            &format!("{} white, {} red, {} blue, {} of any other colour; warnings {said:?}", n[0], n[1], n[2], n[3]),
            said.is_empty() && n[3] == 0 && (0..3).all(|i| (n[i] > 0) == want[i]),
        );
    }
    t.finish("D-635_wireframes_table.md");
}
