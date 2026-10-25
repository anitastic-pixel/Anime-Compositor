//! B-670 (D-790): an expression on a colour, a point or a word opens, is kept and is said.
//!
//! Lane w7e's replay of Smertimba's soft neon glow put `effect("Color Control")("Color")` on
//! seven Drop Shadows' Color, and the project would not open (PROJECT_SCHEMA_INVALID). Each check
//! here compares the frame with the expression against the frame of the same file without it,
//! drawn by the code that existed before D-790, so no expected pixel comes from the new code.

mod effect_table;

use std::path::PathBuf;

use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::export::{
    export_sequence, ExportRequest, ExportStatus, MissingSource, OutputFormat,
};
use anime_compositor::model::Id;
use anime_compositor::{persist, OutputAlpha, OutputDepth};

use effect_table::{repo, same_json, Table};

const MAIN: &str = "comp-main";

fn dir() -> PathBuf {
    repo("Fixtures/effect_mix")
}

/// The bands fixture with `effects` on its one layer.
fn with(effects: J) -> J {
    let mut p: J =
        serde_json::from_str(&std::fs::read_to_string(dir().join("fx_mix_001.json")).unwrap())
            .unwrap();
    p["compositions"][0]["layers"][0]["effects"] = effects;
    p
}

fn held(base: J, keys: J, text: &str, enabled: bool) -> J {
    json!({"base": base, "keyframes": keys, "expression": {"text": text, "enabled": enabled}})
}

/// The frame's samples and the ids and messages it said.
fn draw(project: &J, frame: i32) -> (Vec<f32>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let buffer = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        &dir(),
        64,
        &mut log,
    )
    .expect("the frame draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (buffer.data().to_vec(), said)
}

fn tint(color: J) -> J {
    with(json!([{"instance_id": "fx-0-0", "type_id": "core.tint", "enabled": true,
        "parameters": {"color": color, "amount": 1.0}}]))
}

/// D-820: a Gaussian Blur whose Units, a choice from a list, carries `units`.
fn blur_units(units: J) -> J {
    with(json!([{"instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": 2, "units": units}}]))
}

fn twirl(center: J) -> J {
    with(json!([{"instance_id": "fx-0-0", "type_id": "core.twirl", "enabled": true,
        "parameters": {"angle": 90, "radius": 40, "center": center}}]))
}

#[test]
fn b670_held_expressions() {
    let mut t = Table::new(
        "effect_mix",
        "# D-790: an expression on a colour, a point or a word\n\nB-670, after lane w7e's replay of \
         Smertimba's soft neon glow, whose seven Drop Shadows' Color read a Color Control by \
         `effect(\"Color Control\")(\"Color\")` and whose project would not open \
         (PROJECT_SCHEMA_INVALID). Now the project opens; the expression's text is kept and \
         saved as read; the setting is drawn at its keys or constant, and every frame says \
         EXPRESSION_TYPE naming the setting. Each expected frame is the same file without the \
         expression, drawn as before D-790.\n\nD-820 runs the expression on a colour, a point and a colour word, so lane \
         w7e's neon now draws its Color Control's colour (B-700). What is still kept and said is \
         an expression that cannot give the setting what it holds: three numbers for a colour, \
         which After Effects holds as four; three for a point, which is two; and anything on a \
         choice from a list.\n",
    );
    let cases = [
        (
            "Tint's Color, a linear triple, `[1, 0, 0]`",
            tint(held(json!([0, 0, 1]), json!([]), "[1, 0, 0]", true)),
            tint(json!([0, 0, 1])),
            "color",
            2,
        ),
        (
            "Tint's Color keyed from blue to red, with an expression, at frame 2",
            tint(held(
                json!([0, 0, 1]),
                json!([{"frame": 0, "value": [0, 0, 1], "interp": "linear"},
                       {"frame": 4, "value": [1, 0, 0], "interp": "linear"}]),
                "[0, 1, 0]",
                true,
            )),
            tint(json!({"base": [0, 0, 1], "keyframes": [
                {"frame": 0, "value": [0, 0, 1], "interp": "linear"},
                {"frame": 4, "value": [1, 0, 0], "interp": "linear"}]})),
            "color",
            2,
        ),
        (
            "Gaussian Blur's Units, a choice from a list, `1`",
            blur_units(held(json!("sigma"), json!([]), "1", true)),
            blur_units(json!("sigma")),
            "units",
            0,
        ),
        (
            "Twirl's Center, a point, three numbers, `[4, 2, 0]`",
            twirl(held(json!([8, 5]), json!([]), "[4, 2, 0]", true)),
            twirl(json!([8, 5])),
            "center",
            1,
        ),
    ];

    t.heading("It opens, draws as without it, and says so each frame");
    for (what, project, plain, setting, frame) in &cases {
        let opens = persist::load_str(&project.to_string());
        t.row(
            &format!("{what}: the project opens"),
            &opens
                .as_ref()
                .map_or_else(|d| d.message.clone(), |_| "opens".into()),
            opens.is_ok(),
        );
        if opens.is_err() {
            continue;
        }
        let (got, said) = draw(project, *frame);
        let (want, plain_said) = draw(plain, *frame);
        t.row(
            &format!("{what}: frame {frame} is the frame without the expression"),
            if got == want { "sample-identical" } else { "differs" },
            got == want && got.iter().any(|v| *v != 0.0),
        );
        let named = said.iter().any(|s| {
            s.starts_with("EXPRESSION_TYPE") && s.contains(&format!(" {setting} does not work"))
        });
        t.row(
            &format!("{what}: frame {frame} says EXPRESSION_TYPE on {setting}, the plain file nothing"),
            &format!("{said:?} / {plain_said:?}"),
            named && said.len() == 1 && plain_said.is_empty(),
        );
    }

    t.heading("It is saved as it was read");
    for (what, project, _, setting, _) in &cases {
        let loaded = persist::load_str(&project.to_string()).unwrap();
        let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
        let again: J = serde_json::from_str(&saved).unwrap();
        let effects = again["compositions"][0]["layers"][0]["effects"].as_array().unwrap();
        let written = &effects[effects.len() - 1]["parameters"][*setting];
        let read = &project["compositions"][0]["layers"][0]["effects"]
            [effects.len() - 1]["parameters"][*setting];
        t.row(
            &format!("{what}: the setting's record is saved as read, expression text and all"),
            &written.to_string(),
            same_json(written, read),
        );
        let reloaded = persist::load_str(&saved).unwrap();
        let twice = persist::to_json(reloaded.document.project(), &reloaded.preserved);
        t.row(
            &format!("{what}: opened and saved again, the same bytes"),
            if twice == saved { "byte-identical" } else { "differs" },
            twice == saved,
        );
    }

    t.heading("Switched off, it is kept and nothing is said");
    let off = tint(held(json!([0, 0, 1]), json!([]), "[1, 0, 0]", false));
    let (got, said) = draw(&off, 2);
    let (want, _) = draw(&tint(json!([0, 0, 1])), 2);
    t.row(
        "Tint's Color with its expression switched off: frame 2 as without it, nothing said",
        &format!("{said:?}"),
        got == want && said.is_empty(),
    );

    t.heading("An export meets it");
    let loaded = persist::load_str(&cases[2].1.to_string()).unwrap();
    let out = std::env::temp_dir().join("b670_export");
    let _ = std::fs::remove_dir_all(&out);
    std::fs::create_dir_all(&out).unwrap();
    let report = export_sequence(
        loaded.document.project(),
        &dir(),
        &ExportRequest {
            composition: Id::new(MAIN),
            first_frame: 0,
            last_frame: 2,
            output_dir: out.clone(),
            naming: "f_%04d.png".to_string(),
            depth: OutputDepth::Eight,
            alpha: OutputAlpha::Straight,
            tile_size: 64,
            missing: MissingSource::Block,
            format: OutputFormat::Png,
            choices: Default::default(),
        },
        &std::sync::atomic::AtomicBool::new(false),
    );
    let _ = std::fs::remove_dir_all(&out);
    let said: Vec<&str> = report.diagnostics.iter().map(|d| d.id.as_str()).collect();
    t.row(
        "the Units project's export is refused with EXPRESSION_TYPE, as any failing expression's is (document 28)",
        &format!("{:?} {said:?}", report.status),
        report.status == ExportStatus::Blocked && said.contains(&"EXPRESSION_TYPE"),
    );

    t.heading("An old project");
    let old = repo("Fixtures/effect_mix/fx_mix_001.json");
    let text = std::fs::read_to_string(&old).unwrap();
    let loaded = persist::load_str(&text).unwrap();
    let saved: J =
        serde_json::from_str(&persist::to_json(loaded.document.project(), &loaded.preserved))
            .unwrap();
    let same = same_json(&saved, &serde_json::from_str(&text).unwrap());
    t.row(
        "fx_mix_001.json, written before D-790, opens and saves holding what it held",
        if same { "the same" } else { "differs" },
        same,
    );
    t.finish("B-670_held_expressions_table.md");
}
