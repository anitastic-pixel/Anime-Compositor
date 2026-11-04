//! B-945: D-1065, Mercury after After Effects' CC Mr. Mercury: the `rule` word. Under
//! `after_effects` the numbers are After Effects' and every drop lives, showing the layer under
//! it, so a producer of radius 0 over a clear part of the layer still makes drops (G4 of the
//! wave 12 report, the "LIQUID DESIGN" tutorial 9BTluTNBsGQ). A file without the word keeps
//! D-674's rule, numbers and pictures: FX-MERCURY-001 to 038 are unchanged and pass in B-553.
//!
//! Writes `verification/B-945_mercury_ae_table.md` and draws pictures into
//! `verification/B-945 pictures/`.

mod effect_table;

use std::fs;

use effect_table::{both, distance, on_card, reference, repo, saved, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, MERCURY_SETTINGS, MERCURY_WORDS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// A Mercury's settings, every one `p` leaves out as it is added (the After Effects rule).
fn mercury(p: &J) -> J {
    let mut all = json!({});
    for (name, _, _, start) in MERCURY_SETTINGS {
        all[name] = J::from(start);
    }
    for (name, _, start) in MERCURY_WORDS {
        all[name] = J::from(start);
    }
    all["light_color"] = J::from("#ffffff");
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": "m", "type_id": "core.mercury", "enabled": true, "parameters": all})
}

/// A linear wipe taking 55% of the layer away, the middle with it, as the gaps between the
/// tutorial's letters.
fn wipe() -> J {
    json!({"instance_id": "w", "type_id": "core.linear_wipe", "enabled": true,
        "parameters": {"completion": 55, "angle": 90, "feather": 0}})
}

/// The street still (480 by 270) with `effects`, as a project.
fn project(effects: J) -> J {
    let size = (480, 270);
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from("1_left.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    comp["duration_frames"] = J::from(96);
    comp["work_area"]["end_frame_exclusive"] = J::from(96);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(96);
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    project
}

/// `effects` on the street still at `frame`, drawn on the processor, straight 8-bit, with what
/// it warned of.
fn picture(effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let loaded = persist::load_str(&project(effects).to_string()).expect("the project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        &repo("verification/D-506 pictures"),
        64,
        &mut log,
    )
    .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (frame.to_srgb8_straight(), said)
}

fn write(name: &str, bytes: &[u8]) {
    let out = repo("verification/B-945 pictures");
    fs::create_dir_all(&out).unwrap();
    png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], bytes)
        .unwrap();
}

#[test]
fn b945_mercury_ae() {
    let mut t = Table::new(
        "mercury",
        "# D-1065: Mercury after After Effects\n\nB-945. Mercury's new `rule` word: \
         `after_effects` (as added) reads CC Mr. Mercury's numbers and every drop lives, showing \
         the layer under it; a file without the word keeps D-674's rule. FX-MERCURY-001 to 038 \
         are unchanged and pass in B-553 (`verification/D-674_mercury_table.md`).\n",
    );

    t.heading("The file");
    let old = t.saved_parameters("fx_mercury_023.json");
    t.row(
        "fx_mercury_023.json, written before the rule, is saved as it was: 28 settings and no rule",
        &format!("{} settings; rule {}", old.as_object().unwrap().len(), old["rule"]),
        old.as_object().unwrap().len() == 28 && old.get("rule").is_none(),
    );
    let loaded = persist::load_str(&project(json!([mercury(&json!({}))])).to_string()).unwrap();
    let s = saved(&loaded);
    let p = &s["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "a Mercury as added is saved with its rule, after_effects, and CC Mr. Mercury's numbers",
        &format!(
            "rule {}, birth rate {}, radius {} by {}, velocity {}, gravity {}, birth size {}, death size {}",
            p["rule"], p["birth_rate"], p["radius_x"], p["radius_y"], p["velocity"], p["gravity"],
            p["birth_size"], p["death_size"]
        ),
        p["rule"] == "after_effects"
            && p["birth_rate"] == 1.0
            && p["radius_x"] == 5.0
            && p["velocity"] == 1.0
            && p["gravity"] == 1.0
            && p["birth_size"] == 0.15
            && p["death_size"] == 0.75,
    );
    let (_, said) = picture(json!([mercury(&json!({"rule": "spin"}))]), 24);
    let said = said.join(" ");
    t.row(
        "a Mercury whose rule is \"spin\" is left out with a warning",
        &said,
        said.contains("EFFECT_PARAMETER_INVALID"),
    );

    t.heading("A producer of radius 0 over a clear part of the layer (G4)");
    let empty = picture(json!([wipe(), mercury(&json!({"birth_rate": 0}))]), 24).0;
    for (name, p, drops) in [
        (
            "1_before_radius_0",
            json!({"rule": "d674", "birth_rate": 20, "radius_x": 0, "radius_y": 0, "velocity": 25, "extra": 50, "gravity": 10, "birth_size": 15, "death_size": 75}),
            false,
        ),
        ("2_after_effects_radius_0", json!({"radius_x": 0, "radius_y": 0}), true),
        ("3_after_effects_line", json!({"radius_x": 200, "radius_y": 0}), true),
    ] {
        let (bytes, said) = picture(json!([wipe(), mercury(&p)]), 24);
        write(name, &bytes);
        let changed = distance(&bytes, &empty).1;
        t.row(
            &format!(
                "{name}.png, frame 24, the middle wiped away: {}",
                if drops { "drops drawn" } else { "no drops, as D-674 drew it" }
            ),
            &format!("{changed} pixels differ from no drops; warnings {said:?}"),
            said.is_empty() && (changed > 0) == drops,
        );
    }

    t.heading("The animations (frame 24, After Effects rule)");
    let street = picture(json!([]), 0).0;
    let mut drawn: Vec<(String, Vec<u8>)> = vec![];
    {
        for way in MERCURY_WORDS[0].1 {
            let p = json!({"animation": way, "birth_rate": 3, "velocity": 2});
            let (bytes, said) = picture(json!([mercury(&p)]), 24);
            write(&format!("4_{way}"), &bytes);
            let same = drawn
                .iter()
                .find(|(_, b)| distance(b, &bytes).1 == 0)
                .map(|(n, _)| n.clone());
            t.row(
                &format!("4_{way}.png: drawn without a warning, unlike the street and every animation before it"),
                &format!(
                    "{} pixels changed; warnings {said:?}; the same as {}",
                    distance(&bytes, &street).1,
                    same.as_deref().unwrap_or("none")
                ),
                said.is_empty() && distance(&bytes, &street).1 > 0 && same.is_none(),
            );
            drawn.push((way.to_string(), bytes));
        }
    }

    t.heading("On the card");
    let mut gpu = Gpu::new().expect("a graphics card");
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    for (what, p) in [
        ("as added", json!({})),
        ("fire", json!({"animation": "fire", "birth_rate": 3})),
        ("jet sideways, a line producer", json!({"animation": "jet_sideways", "radius_x": 0, "radius_y": 300})),
        ("the D-674 rule", json!({"rule": "d674"})),
    ] {
        let project = reference(|id| {
            let mut m = mercury(&p);
            m["instance_id"] = J::from(format!("b945-{id}"));
            json!([m])
        });
        for frame in [100, 239] {
            let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, PreviewQuality::Full);
            let card = on_card(&project, &ref_comp, &ref_root, frame, PreviewQuality::Full, |e| {
                matches!(e, Effect::Mercury { .. })
            });
            t.row(
                &format!("the reference shot, Mercury {what} on three layers, frame {frame}, Full"),
                &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                d.0 <= 1 && !r && a == b && card == 3,
            );
        }
    }
    t.finish("B-945_mercury_ae_table.md");
}
