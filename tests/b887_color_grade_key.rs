//! B-887: D-1007, Color Grade's Show Key as Color/Gray and Color/Black.
//!
//! Writes `verification/D-1007_color_grade_5_table.md` and draws pictures into
//! `verification/D-1007 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_5/expected_color_grade_5.json`, written by
//! `tools/color_grade_5_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE5-001 to 009. Nothing here is a snapshot of a run. The key tools' numbers
//! (`expected_key_tools.json`) are checked in the page by B-887's page check.

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
fn b887_color_grade_key() {
    let mut t = Table::new(
        "color_grade_5",
        "# D-1007: Color Grade, the fifth unit\n\nB-887: HSL Secondary's Show Key as Color/Gray \
         and Color/Black (the colour where keyed, over grey or black elsewhere). Every expected \
         pixel is `Fixtures/color_grade_5/expected_color_grade_5.json`, written by \
         `tools/color_grade_5_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE5-001 to 009.\n",
    );

    t.heading("FX-GRADE5-001 to 009 (document 25)");
    t.fixtures_numbered("expected_color_grade_5.json", 1..=9);

    t.heading("The file");
    let files: Vec<String> = (1..=8).map(|n| format!("fx_grade5_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let why = changed(&t, "fx_grade5_009.json", |_| {}).why_invalid();
    t.row(
        "fx_grade5_009.json, key view \"color\", is refused in a sentence naming the key view",
        &why,
        why.contains("key view"),
    );

    t.heading("Commands");
    let mut document = t.load("fx_grade5_001.json").document;
    let word = |w: &str| {
        changed(&t, "fx_grade5_001.json", |e| {
            if let Effect::ColorGrade { key_view, .. } = e {
                *key_view = w.into();
            }
        })
    };
    let [wrong, black, mask, off] = ["Color/Gray", "color_black", "mask", "off"].map(word);
    t.refused(&mut document, vec![("key view \"Color/Gray\"", set(wrong))]);
    t.taken(
        &mut document,
        "fx_grade5_001.json",
        vec![
            ("key view color_black", set(black)),
            ("key view mask", set(mask)),
            ("key view off", set(off)),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade5_001.json", 0),
        ("fx_grade5_004.json", 0),
        ("fx_grade5_006.json", 0),
        ("fx_grade5_007.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    // A blurred key (006: the processor's, D-571) and refused (009) leave nothing for the card.
    for n in 1..=9u32 {
        let file = format!("fx_grade5_{n:03}.json");
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
        let want = (card == 0) == [6, 9].contains(&n);
        t.row(
            &format!("{file}, frames 0 to 2 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 6 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && want,
        );
    }

    t.heading("Pictures: in `verification/D-1007 pictures/`");
    let dir = repo("verification/D-1007 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    // A wide warm key, as Set would make it on the street's brick.
    let key = |view: &str| {
        fx(&json!({"key_hue": 20, "key_hue_range": 30, "key_hue_softness": 15, "key_view": view}))
    };
    let mut views = Vec::new();
    for (file, view, what) in [
        ("2_white_black.png", "mask", "White/Black, D-398's: the key as grey"),
        ("3_color_gray.png", "color_gray", "Color/Gray: the warm colours as they are, the rest mid grey"),
        ("4_color_black.png", "color_black", "Color/Black: the warm colours as they are, the rest black"),
    ] {
        let (p, said) = picture(&dir, key(view));
        write(file, &p);
        t.row(&format!("{file}, {what}; draws cleanly"), &format!("{said:?}"), said.is_empty());
        views.push(p);
    }
    // Where the key is 0, Color/Gray is 0.5 encoded (127 or 128) and Color/Black is 0; where it
    // is whole, both are the street's own colour.
    let (mut grey, mut black, mut kept, mut zero, mut whole) = (true, true, true, 0, 0);
    for (i, m) in views[0].chunks_exact(4).enumerate() {
        let (g, b, s) = (&views[1][i * 4..][..3], &views[2][i * 4..][..3], &before[i * 4..][..3]);
        if m[0] == 0 {
            zero += 1;
            grey &= g.iter().all(|v| (127..=128).contains(v));
            black &= b.iter().all(|v| *v == 0);
        } else if m[0] == 255 {
            whole += 1;
            kept &= g.iter().zip(s).chain(b.iter().zip(s)).all(|(a, s)| a.abs_diff(*s) <= 1);
        }
    }
    t.row(
        "where the key is 0, Color/Gray is mid grey and Color/Black black; where it is whole, both are the street's own colour",
        &format!("{zero} pixels outside, {whole} inside; grey {grey}, black {black}, the street's own {kept}; Color/Gray differs from White/Black in {} pixels", distance(&views[1], &views[0]).1),
        zero > 0 && whole > 0 && grey && black && kept,
    );

    t.finish("D-1007_color_grade_5_table.md");
}
