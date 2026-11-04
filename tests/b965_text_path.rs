//! B-965: D-1085, After Effects' Path Options: a text layer's words set along one of its masks,
//! with Reverse Path, Perpendicular To Path, Force Alignment and keyable First and Last Margins.
//!
//! Every expected number is `Fixtures/text_path/expected_text_path.json`, written by
//! `tools/text_path_reference.py` (fontTools; curved masks sampled finely) before this code
//! existed. The table is `verification/B-965_text_path_table.md`.

mod effect_table;

use std::fs;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::{plan_frame, render_frame};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::{persist, text, WorkingBuffer};

fn expected() -> J {
    serde_json::from_str(&fs::read_to_string(repo("Fixtures/text_path/expected_text_path.json")).unwrap()).unwrap()
}

/// A 1280 by 720 composition of one text layer with one mask, Path Options and animators.
fn project(case: &J) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let base = json!({"position": [0, 0], "scale": [100, 100], "rotation": 0, "opacity": 100, "fill": "off",
        "color": [1, 0, 0], "tracking": 0, "start": 0, "end": 100, "offset": 0, "amount": 100,
        "based_on": "characters", "shape": "square", "smoothness": 100, "ease_high": 0, "ease_low": 0});
    let mut effects = vec![json!({"instance_id": "fx-path", "type_id": "core.text_path", "enabled": true,
        "parameters": case["options"]})];
    for (i, a) in case["animators"].as_array().unwrap().iter().enumerate() {
        let mut p = base.clone();
        for (k, v) in a.as_object().unwrap() {
            p[k] = v.clone();
        }
        effects.push(json!({"instance_id": format!("fx-{i}"), "type_id": "core.text_animator", "enabled": true, "parameters": p}));
    }
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 4,
        "transform": {"anchor": t(json!([640, 360])), "position": t(json!([640, 360])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [case["mask"]], "source_text": case["text"]});
    let p = json!({
        "schema_version": 0, "project_id": "proj-b965",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"}, "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": 1280, "height": 720, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 4,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 4}, "layer_order": ["art"], "layers": [layer]}]
    });
    persist::load_str(&p.to_string())
        .unwrap_or_else(|d| panic!("the project reads: {}", d.message))
        .document
        .project()
        .clone()
}

/// The `art` layer's text and its effects as the frame hands them to the drawing: Path Options'
/// mask filled in from the layer's first mask, as compose fills it.
fn parts(project: &Project, frame: i32) -> (anime_compositor::text::Text, Vec<Effect>) {
    let comp = project.composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let mask = layer.masks[0].at_time(frame as f64);
    let effects = layer
        .effects
        .iter()
        .map(|i| {
            let mut e = anime_compositor::expr::effect_at(comp, &layer.id, i, frame, frame as f64).0.effect;
            if let Effect::TextPath { line, .. } = &mut e {
                *line = Some((mask.path(), mask.closed));
            }
            e
        })
        .collect();
    (layer.text.clone().unwrap(), effects)
}

fn render(project: &Project, frame: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(project, &Id::new(MAIN), frame, &repo("Fixtures/text_path"), 64, &mut log)
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

fn warned(project: &Project, frame: i32) -> Vec<String> {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(project, &Id::new(MAIN), frame, &repo("Fixtures/text_path"), &mut log);
    log.finish().iter().map(|d| d.id.as_str().to_string()).collect()
}

/// The largest miss, in pixels and degrees, of the characters laid out against `chars`.
fn misses(placed: &[text::Placed], chars: &J) -> (f64, f64) {
    let mut by_index = placed.to_vec();
    by_index.sort_by_key(|p| p.index);
    let chars = chars.as_array().unwrap();
    assert_eq!(by_index.len(), chars.len());
    by_index.iter().zip(chars).fold((0.0f64, 0.0f64), |(d, a), (p, c)| {
        let at = [c["at"][0].as_f64().unwrap(), c["at"][1].as_f64().unwrap()];
        let turn = (p.turn - c["turn"].as_f64().unwrap() + 540.0).rem_euclid(360.0) - 180.0;
        (d.max((p.anchor[0] - at[0]).hypot(p.anchor[1] - at[1])), a.max(turn.abs()))
    })
}

/// Ink (alpha over 0.2) within `r` pixels of `p`.
fn inked(b: &WorkingBuffer, p: [f64; 2], r: i64) -> bool {
    let (x0, y0) = (p[0] as i64, p[1] as i64);
    (y0 - r..=y0 + r).any(|y| {
        (x0 - r..=x0 + r).any(|x| {
            x >= 0 && y >= 0 && (x as usize) < b.width() && (y as usize) < b.height()
                && b.pixel(x as usize, y as usize)[3] > 0.2
        })
    })
}

#[test]
fn b965_text_path() {
    let e = expected();
    let mut t = Table::new(
        "text_path",
        "# B-965: Path Options, text on a path (D-1085)\n\nWritten by `tests/b965_text_path.rs`. \
         Each case's characters against `Fixtures/text_path/expected_text_path.json` from \
         `tools/text_path_reference.py`: where each one's anchor lands and how far it is turned.\n",
    );
    t.heading("Where each character lands");
    for (case, c) in e["cases"].as_object().unwrap() {
        let p = project(c);
        let (words, effects) = parts(&p, 0);
        let placed = text::placed(&words, &effects).expect("the bundled font");
        let (d, a) = misses(&placed, &c["chars"]);
        let curved = c["curved"].as_bool().unwrap();
        let (dt, at) = if curved {
            (e["curve_tolerance"].as_f64().unwrap(), e["curve_turn_tolerance"].as_f64().unwrap())
        } else {
            (e["tolerance"].as_f64().unwrap(), e["tolerance"].as_f64().unwrap())
        };
        t.row(
            &format!("{case}: {}", c["says"].as_str().unwrap()),
            &format!("{} characters, at most {d:.4} px and {a:.4} degrees out", placed.len()),
            d <= dt && a <= at,
        );
        // The frame draws the words where the rule puts them.
        let picture = render(&p, 0);
        let all = c["chars"].as_array().unwrap().iter().filter(|x| x["char"] != " ").all(|x| {
            let (turn, at) = (x["turn"].as_f64().unwrap().to_radians(), &x["at"]);
            // A point 18 pixels above the baseline, in the character's own up.
            let up = [at[0].as_f64().unwrap() + 18.0 * turn.sin(), at[1].as_f64().unwrap() - 18.0 * turn.cos()];
            inked(&picture, up, 14)
        });
        t.row(&format!("{case}: the frame has ink at every letter"), if all { "yes" } else { "no" }, all);
    }

    t.heading("Projects");
    let keyed = t.load("fx_txp_020.json").document.project().clone();
    let frames = &e["projects"]["FX-TXP-020"]["frames"];
    for (frame, chars) in frames.as_object().unwrap() {
        let f: i32 = frame.parse().unwrap();
        let (words, effects) = parts(&keyed, f);
        let (d, a) = misses(&text::placed(&words, &effects).unwrap(), chars);
        t.row(
            &format!("fx_txp_020.json frame {f}: First Margin keyed, {} along", 50 * f),
            &format!("at most {d:.4} px and {a:.4} degrees out"),
            d <= 0.02 && a <= 0.02,
        );
    }
    let moved = render(&keyed, 0).data() != render(&keyed, 5).data();
    t.row("fx_txp_020.json: frames 0 and 5 are drawn differently", if moved { "they differ" } else { "the same" }, moved);
    let missing = t.load("fx_txp_021.json").document.project().clone();
    let said = warned(&missing, 0);
    let ok = said.iter().any(|s| s == "EFFECT_PATH_MISSING");
    t.row("fx_txp_021.json: mask 3 on a layer with one is said on the frame", &said.join(", "), ok);
    let mut json: J = serde_json::from_str(&fs::read_to_string(t.root.join("fx_txp_021.json")).unwrap()).unwrap();
    json["compositions"][0]["layers"][0]["effects"] = json!([]);
    let bare = persist::load_str(&json.to_string()).unwrap().document.project().clone();
    let same = render(&missing, 0).data() == render(&bare, 0).data();
    t.row(
        "fx_txp_021.json: its words are drawn where they are without a path",
        if same { "the same" } else { "differs" },
        same,
    );
    let none = {
        let mut j = json.clone();
        j["compositions"][0]["layers"][0]["masks"] = json!([]);
        persist::load_str(&j.to_string()).unwrap().document.project().clone()
    };
    let old = render(&none, 0).data() == render(&bare, 0).data();
    t.row("A project from before Path Options opens and draws as before", if old { "the same" } else { "differs" }, old);

    t.heading("The file");
    t.round_trips(&["fx_txp_020.json", "fx_txp_021.json"]);
    let saved = t.saved_parameters("fx_txp_021.json");
    let no_line = saved.get("line").is_none() && saved["path"] == 3;
    t.row("Saved: the six settings, never the mask's pieces", &saved.to_string(), no_line);
    t.shape_refused("fx_txp_021.json", "a First Margin written as a word", r#"{"path":1,"reverse":"off","perpendicular":"on","force_alignment":"off","first_margin":"ten","last_margin":0}"#);
    t.shape_refused("fx_txp_021.json", "no Last Margin", r#"{"path":1,"reverse":"off","perpendicular":"on","force_alignment":"off","first_margin":0}"#);
    let mut odd = json.clone();
    odd["compositions"][0]["layers"][0]["effects"] = serde_json::from_str(
        r#"[{"instance_id":"fx-0","type_id":"core.text_path","enabled":true,"parameters":{"path":1,"reverse":"maybe","perpendicular":"on","force_alignment":"off","first_margin":0,"last_margin":0}}]"#,
    )
    .unwrap();
    let loaded = persist::load_str(&odd.to_string()).unwrap();
    let said: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
    let ok = said.contains(&"EFFECT_PARAMETER_INVALID");
    t.row("Reverse Path written \"maybe\" is said on opening", &said.join(", "), ok);
    t.finish("B-965_text_path_table.md");
}
