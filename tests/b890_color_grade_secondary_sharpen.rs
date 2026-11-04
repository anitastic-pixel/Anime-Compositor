//! B-890: D-1010, Color Grade's HSL Secondary Sharpen.
//!
//! Writes `verification/D-1010_color_grade_7_table.md` and draws pictures into
//! `verification/D-1010 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_7/expected_color_grade_7.json`, written by
//! `tools/color_grade_7_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE7-001 to 010. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;

use effect_table::{both, changed, distance, on_card, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, COLOR_GRADE_SETTINGS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// A Color Grade with the settings `p`, every one `p` leaves out as it is added.
fn fx(p: &J) -> J {
    let straight = json!([[0, 0], [255, 255]]);
    let mut all = json!({"look": "", "curve_master": straight, "curve_red": straight, "curve_green": straight, "curve_blue": straight,
        "hue_saturation": [], "hue_hue": [], "hue_luma": [], "luma_saturation": [], "saturation_saturation": [],
        "secondary_wheels": "one", "key_invert": "off", "key_view": "off"});
    for (name, _, _, start) in COLOR_GRADE_SETTINGS {
        all[name] = json!(start);
    }
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!([{"instance_id": "fx-0-0", "type_id": "core.color_grade", "enabled": true, "parameters": all}])
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &std::path::Path, effects: J) -> (Vec<u8>, Vec<String>) {
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
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log)
        .expect("the picture draws");
    let said = log
        .finish()
        .iter()
        .map(|d| format!("{} {}", d.id.as_str(), d.message))
        .collect();
    (frame.to_srgb8_straight(), said)
}

#[test]
fn b890_color_grade_secondary_sharpen() {
    let mut t = Table::new(
        "color_grade_7",
        "# D-1010: Color Grade, the seventh unit\n\nB-890: HSL Secondary's Sharpen, D-637's sharpen \
         mixed in by the secondary's key after its correction. Every expected pixel is \
         `Fixtures/color_grade_7/expected_color_grade_7.json`, written by \
         `tools/color_grade_7_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE7-001 to 010.\n",
    );

    t.heading("FX-GRADE7-001 to 010 (document 25)");
    t.fixtures_numbered("expected_color_grade_7.json", 1..=10);

    t.heading("The file");
    let files: Vec<String> = (1..=9).map(|n| format!("fx_grade7_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let why = changed(&t, "fx_grade7_010.json", |_| {}).why_invalid();
    t.row(
        "fx_grade7_010.json, secondary sharpen 101, is refused in a sentence naming the setting",
        &why,
        why.contains("secondary sharpen") || why.contains("secondary_sharpen"),
    );

    t.heading("Commands");
    let mut document = t.load("fx_grade7_001.json").document;
    let to = |x: f64| {
        changed(&t, "fx_grade7_001.json", |e| {
            if let Effect::ColorGrade { values, .. } = e {
                values[55] = x;
            }
        })
    };
    let [wrong, low, none] = [-101.0, -100.0, 0.0].map(to);
    t.refused(&mut document, vec![("secondary sharpen -101", set(wrong))]);
    t.taken(
        &mut document,
        "fx_grade7_001.json",
        vec![("secondary sharpen -100", set(low)), ("secondary sharpen 0", set(none))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade7_001.json", 0),
        ("fx_grade7_004.json", 0),
        ("fx_grade7_005.json", 0),
        ("fx_grade7_006.json", 4),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    // A secondary sharpen is the processor's, as D-637's sharpen.
    for n in 1..=10u32 {
        let file = format!("fx_grade7_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..3 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::ColorGrade { .. })
                });
            }
        }
        t.row(
            &format!("{file}, frames 0 to 2 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 6 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree,
        );
    }

    t.heading("Pictures: in `verification/D-1010 pictures/`");
    let dir = repo("verification/D-1010 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    t.row("the street with no effect draws cleanly", &format!("{said:?}"), said.is_empty());
    // A key on the warm colours (the brick), as Set would make it.
    let key = |extra: J| {
        let mut p = json!({"key_hue": 20, "key_hue_range": 30, "key_hue_softness": 15});
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        fx(&p)
    };
    let (mask, _) = picture(&dir, key(json!({"key_view": "mask"})));
    write("1_key.png", &mask);
    let mut shots = Vec::new();
    for (file, s, what) in [
        ("2_sharper.png", 100.0, "Secondary Sharpen 100: the warm buildings' edges harder"),
        ("3_softer.png", -100.0, "Secondary Sharpen -100: the warm buildings softened"),
    ] {
        let (p, said) = picture(&dir, key(json!({"secondary_sharpen": s})));
        write(file, &p);
        t.row(&format!("{file}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        shots.push(p);
    }
    // Where the key is 0 nothing changes; inside it the picture does.
    let (mut outside, mut moved, mut zero) = (true, 0, 0);
    for (i, m) in mask.chunks_exact(4).enumerate() {
        let (a, b, s) = (&shots[0][i * 4..][..3], &shots[1][i * 4..][..3], &before[i * 4..][..3]);
        if m[0] == 0 {
            zero += 1;
            outside &= a.iter().zip(s).chain(b.iter().zip(s)).all(|(x, y)| x.abs_diff(*y) <= 1);
        } else if a != s {
            moved += 1;
        }
    }
    t.row(
        "where the key is 0 both pictures are the street's own colour; inside it the sharpened one differs",
        &format!("{zero} pixels outside the key, unchanged: {outside}; {moved} pixels inside changed; sharper against softer: {} pixels", distance(&shots[0], &shots[1]).1),
        zero > 0 && outside && moved > 0,
    );

    t.finish("D-1010_color_grade_7_table.md");
}
