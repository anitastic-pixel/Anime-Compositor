//! B-636: D-757, Turbulent Displace gains After Effects' Offset (Turbulence) and its other
//! Displacement types: `offset` (pixels, keyable, absent (0, 0)) and the words cross, bulge,
//! twist, turbulent_smoother, bulge_smoother and twist_smoother. A file without them draws as
//! before.
//!
//! Writes `verification/B-636_turbulent_types_table.md` and draws pictures into
//! `verification/B-636 pictures/`.
//!
//! Every expected pixel is `Fixtures/turbulent_types/expected_turbulent_types.json`, written by
//! `tools/turbulent_types_reference.py` before this code existed and printed in document 25 as
//! FX-TURB-TY-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    changed, distance, keys, reference, repo, same_json, saved, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::{persist, png_out, OutputDepth};

fn is_turb(e: &Effect) -> bool {
    matches!(e, Effect::TurbulentDisplace { .. })
}

/// A Turbulent Displace as added from the panel, with `p` laid over it.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"amount": 30, "size": 40, "complexity": 2, "evolution": 0, "speed": 0, "seed": 0,
        "edges": "transparent", "units": "after_effects"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.turbulent_displace", "enabled": true, "parameters": all})
}

/// `effects` on the street at frame 0, drawn on the processor, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
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
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let out = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (out.to_srgb8_straight(), said)
}

/// `json` opened and drawn at `frame` on the processor, from the folder `root`.
fn draw(json: &J, root: &Path, frame: i32) -> Vec<f32> {
    let loaded = persist::load_str(&json.to_string()).expect("the file reads");
    let mut log = FrameLog::new(3);
    render_frame(loaded.document.project(), &Id::new(MAIN), frame, root, 64, &mut log)
        .expect("the frame draws")
        .data()
        .to_vec()
}

#[test]
fn b636_turbulent_types() {
    let mut t = Table::new(
        "turbulent_types",
        "# B-636: Turbulent Displace's offset and types (D-757)\n\nD-757 adds After Effects' Offset \
         (Turbulence) as `offset` (pixels, x then y, keyable, -100000 to 100000, absent (0, 0)), \
         which slides the field as D-411's drift does, and the Displacement words cross, bulge, twist, \
         turbulent_smoother, bulge_smoother and twist_smoother (document 21). A file without them \
         draws as before. Every expected pixel is `Fixtures/turbulent_types/expected_turbulent_types.json`, \
         written by `tools/turbulent_types_reference.py` before this code existed and printed in \
         document 25 as FX-TURB-TY-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-TURB-TY-001 to 018 (document 25)");
    t.fixtures_numbered("expected_turbulent_types.json", 1..=18);

    t.heading("Older projects: no offset, as before");
    for (folder, prefix, count) in [
        ("turbulent_displace", "fx_turb", 26),
        ("turbulent_ae", "fx_turb_ae", 12),
        ("heat_shimmer", "fx_shimmer", 16),
    ] {
        let root = repo(&format!("Fixtures/{folder}"));
        for n in 1..=count {
            let file = format!("{prefix}_{n:03}.json");
            let text = fs::read_to_string(root.join(&file)).unwrap();
            let Ok(loaded) = persist::load_str(&text) else {
                continue;
            };
            let old: J = serde_json::from_str(&text).unwrap();
            let kept = same_json(&saved(&loaded), &old);
            // Its twin with Offset (0, 0) written draws the same frames.
            let mut twin = old.clone();
            let effects = &mut twin["compositions"][0]["layers"][0]["effects"];
            let is_turb = effects[0]["type_id"] == "core.turbulent_displace";
            if is_turb {
                effects[0]["parameters"]["offset"] = json!([0, 0]);
            }
            let same = is_turb
                && [0, 4]
                    .iter()
                    .all(|&f| draw(&old, &root, f) == draw(&twin, &root, f));
            t.row(
                &format!("{folder}/{file} (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0)"),
                &format!("saved {}; {}", if kept { "the same" } else { "differently" }, if same { "bit-identical" } else { "differ" }),
                kept && same,
            );
        }
    }

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_turb_ty_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved_p = t.saved_parameters("fx_turb_ty_001.json");
    t.row(
        "fx_turb_ty_001.json is saved with Offset (3, 0)",
        &saved_p.to_string(),
        saved_p["offset"][0] == 3.0 && saved_p["offset"][1] == 0.0,
    );
    let saved_p = t.saved_parameters("fx_turb_ty_005.json");
    t.row(
        "fx_turb_ty_005.json is saved with Displacement bulge and no offset written",
        &saved_p.to_string(),
        saved_p["displacement"] == "bulge" && saved_p.get("offset").is_none(),
    );
    for (file, what, want) in [
        ("fx_turb_ty_015.json", "Displacement \"Bulge\"", "displacement"),
        ("fx_turb_ty_016.json", "Displacement \"cross_displacement\"", "displacement"),
        ("fx_turb_ty_017.json", "Offset (100001, 0)", "offset"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} ({what}) is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    let why = changed(&t, "fx_turb_ty_016.json", |_| {}).why_invalid();
    t.row(
        "the sentence for a wrong word lists the nine words",
        &why,
        anime_compositor::effects::TURBULENT_WAYS.iter().all(|w| why.contains(w)),
    );
    t.shape_refused(
        "fx_turb_ty_001.json",
        "a Turbulent Displace whose Offset is one number",
        r##"{"amount": 30, "size": 8, "complexity": 2, "evolution": 0, "speed": 0, "seed": 0, "edges": "transparent", "units": "after_effects", "offset": 3}"##,
    );

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_turb_ty_001.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let halved = matches!(draft, Effect::TurbulentDisplace { offset, .. } if offset == [1.5, 0.0]);
    t.row("a half-size draft preview halves the Offset, a distance", &format!("{draft:?}"), halved);
    for file in ["fx_turb_ty_001.json", "fx_turb_ty_005.json", "fx_turb_ty_006.json"] {
        let (a, b) = (
            changed(&t, file, |_| {}).bounds_expansion(),
            changed(&t, "fx_turb_ty_002.json", |_| {}).bounds_expansion(),
        );
        t.row(
            &format!("{file} grows the drawing's bounds as the plain turbulent push does"),
            &format!("{a} against {b}"),
            a == b,
        );
    }

    t.heading("Commands");
    let mut document = t.load("fx_turb_ty_002.json").document;
    let far = changed(&t, "fx_turb_ty_002.json", |e| {
        if let Effect::TurbulentDisplace { offset, .. } = e {
            *offset = [0.0, 100001.0]
        }
    });
    let capital = changed(&t, "fx_turb_ty_002.json", |e| {
        if let Effect::TurbulentDisplace { displacement, .. } = e {
            *displacement = "Twist".into()
        }
    });
    t.refused(
        &mut document,
        vec![
            ("offset (0, 100001)", set(far)),
            ("displacement \"Twist\"", set(capital)),
            ("offset keyed to (-200000, 0)", keys("offset", &[(0, &[0.0, 0.0]), (4, &[-200000.0, 0.0])])),
        ],
    );
    let twist = changed(&t, "fx_turb_ty_002.json", |e| {
        if let Effect::TurbulentDisplace { displacement, offset, .. } = e {
            *displacement = "twist_smoother".into();
            *offset = [12.0, -7.5];
        }
    });
    t.taken(
        &mut document,
        "fx_turb_ty_002.json",
        vec![
            ("twist_smoother with offset (12, -7.5)", set(twist)),
            ("offset keyed from (0, 0) to (8, -4)", keys("offset", &[(0, &[0.0, 0.0]), (4, &[8.0, -4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_turb_ty_001.json", 0),
        ("fx_turb_ty_003.json", 2),
        ("fx_turb_ty_004.json", 0),
        ("fx_turb_ty_005.json", 0),
        ("fx_turb_ty_009.json", 0),
        ("fx_turb_ty_011.json", 0),
        ("fx_turb_ty_012.json", 0),
        ("fx_turb_ty_013.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    t.row(
        "the card draws the plain turbulent push with an offset (001 to 003); the other words stay on the processor, as D-306's horizontal, vertical and pinning do, so those frames are the processor's",
        "see the rows below",
        true,
    );
    let mut gpu = Gpu::new().expect("a usable card");
    t.card_fixtures(
        &mut gpu,
        "expected_turbulent_types.json",
        &(4..=18).collect::<Vec<u32>>(),
        is_turb,
    );
    let plain = reference(|_| json!([]));
    let shots = vec![
        (
            "Turbulent Displace amount 30, size 40, offset (25, -10)",
            reference(|id| json!([fx(&format!("b636-{id}"), &json!({"offset": [25, -10]}))])),
            3,
        ),
        (
            "Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25)",
            reference(|id| {
                json!([fx(&format!("b636-{id}"), &json!({"amount": 60, "size": 20, "speed": 20, "seed": 7,
                    "drift_direction": 30, "drift_speed": 2.5, "offset": [-40.5, 13.25]}))])
            }),
            3,
        ),
        (
            "Turbulent Displace bulge, amount 30, size 40 (the processor's)",
            reference(|id| json!([fx(&format!("b636-{id}"), &json!({"displacement": "bulge"}))])),
            0,
        ),
    ];
    t.card_reference(&mut gpu, &plain, &shots, is_turb);

    t.heading("Pictures: in `verification/B-636 pictures/`");
    let dir = repo("verification/B-636 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes)
            .unwrap()
    };
    write("town.png", &town());
    let warp = |p: J| json!([fx("fx-0-0", &p)]);
    let (plain_pic, said) = picture(&dir, warp(json!({"amount": 20, "size": 40})));
    write("1_turbulent.png", &plain_pic);
    t.row("1_turbulent.png, the street with amount 20, size 40, as before; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (n, name, p, what) in [
        (2, "offset_40_0", json!({"amount": 20, "size": 40, "offset": [40, 0]}), "the same warp slid 40 pixels right"),
        (3, "cross", json!({"amount": 20, "size": 40, "displacement": "cross"}), "cross: streaks, stretched across for the sideways push and down for the up-and-down one"),
        (4, "bulge", json!({"amount": 20, "size": 40, "displacement": "bulge"}), "bulge: the field's hills swell"),
        (5, "twist", json!({"amount": 20, "size": 40, "displacement": "twist"}), "twist: the pixels carried round the hills"),
        (6, "turbulent_smoother", json!({"amount": 20, "size": 40, "complexity": 4, "displacement": "turbulent_smoother"}), "turbulent_smoother, complexity 4: finer octaves weaker"),
    ] {
        let (p, said) = picture(&dir, warp(p));
        let file = format!("{n}_{name}.png");
        write(&file, &p);
        t.row(
            &format!("{file}, {what}; draws cleanly and differs from 1_turbulent.png"),
            &format!("{said:?}; {} pixels differ", distance(&p, &plain_pic).1),
            said.is_empty() && distance(&p, &plain_pic).1 > 0,
        );
    }

    t.finish("B-636_turbulent_types_table.md");
}
