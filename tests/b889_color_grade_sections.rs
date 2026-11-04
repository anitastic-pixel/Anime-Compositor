//! B-889: D-1009, Color Grade's switch per section.
//!
//! Writes `verification/D-1009_color_grade_6_table.md` and draws pictures into
//! `verification/D-1009 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_6/expected_color_grade_6.json`, written by
//! `tools/color_grade_6_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE6-001 to 011. Nothing here is a snapshot of a run. The white balance eyedropper's
//! numbers (`expected_white_balance.json`) are checked in the page by B-889's page check.

mod effect_table;

use std::fs;

use effect_table::{both, changed, distance, on_card, repo, saved, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, COLOR_GRADE_SECTIONS, COLOR_GRADE_SETTINGS};
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
fn b889_color_grade_sections() {
    let mut t = Table::new(
        "color_grade_6",
        "# D-1009: Color Grade, the sixth unit\n\nB-889: a switch per section (Basic Correction, \
         Creative, Curves, Color Wheels, HSL Secondary, Vignette). A section switched off draws as \
         if its settings were at their start. Every expected pixel is \
         `Fixtures/color_grade_6/expected_color_grade_6.json`, written by \
         `tools/color_grade_6_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE6-001 to 011.\n",
    );

    t.heading("FX-GRADE6-001 to 011 (document 25)");
    t.fixtures_numbered("expected_color_grade_6.json", 1..=11);

    t.heading("The file");
    let files: Vec<String> = (1..=10).map(|n| format!("fx_grade6_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let why = changed(&t, "fx_grade6_011.json", |_| {}).why_invalid();
    t.row(
        "fx_grade6_011.json, curves' switch \"yes\", is refused in a sentence naming the switch",
        &why,
        why.contains("curves switch"),
    );
    let old = fs::read_to_string(t.root.join("fx_grade6_010.json")).unwrap();
    let saved = saved(&t.load("fx_grade6_010.json")).to_string();
    let named = COLOR_GRADE_SECTIONS.iter().filter(|s| saved.contains(*s)).count();
    t.row(
        "fx_grade6_010.json, a file from before with no switches, saves with none written",
        &format!("switches in the saved file: {named}; in the file read: {}", COLOR_GRADE_SECTIONS.iter().filter(|s| old.contains(*s)).count()),
        named == 0,
    );

    t.heading("Commands");
    let mut document = t.load("fx_grade6_001.json").document;
    let word = |w: &str| {
        changed(&t, "fx_grade6_001.json", |e| {
            if let Effect::ColorGrade { sections, .. } = e {
                sections[2] = w.into();
            }
        })
    };
    let [wrong, off, on] = ["Off", "off", "on"].map(word);
    t.refused(&mut document, vec![("curves switch \"Off\"", set(wrong))]);
    t.taken(
        &mut document,
        "fx_grade6_001.json",
        vec![("curves switch off", set(off)), ("curves switch on", set(on))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade6_001.json", 0),
        ("fx_grade6_003.json", 0),
        ("fx_grade6_006.json", 0),
        ("fx_grade6_009.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 1..=11u32 {
        let file = format!("fx_grade6_{n:03}.json");
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
        // Refused (011) leaves nothing for the card.
        let want = n != 11 || card == 0;
        t.row(
            &format!("{file}, frames 0 to 2 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 6 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && want,
        );
    }

    t.heading("Pictures: in `verification/D-1009 pictures/`");
    let dir = repo("verification/D-1009 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    t.row("the street with no effect draws cleanly", &format!("{said:?}"), said.is_empty());
    // A warm, contrasty grade with a dark vignette.
    let grade = json!({"temperature": 40, "contrast": 30, "vignette_amount": -2.5, "midtone_wheel_hue": 40, "midtone_wheel_amount": 30});
    let with = |extra: J| {
        let mut p = grade.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        fx(&p)
    };
    let (all_on, said) = picture(&dir, with(json!({})));
    write("1_all_on.png", &all_on);
    t.row("1_all_on.png, warm, contrasty, dark corners; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (no_vignette, said) = picture(&dir, with(json!({"vignette_on": "off"})));
    write("2_vignette_off.png", &no_vignette);
    t.row("2_vignette_off.png, the same with Vignette off: the corners as bright as the middle's grade; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (no_basic, said) = picture(&dir, with(json!({"basic_on": "off"})));
    write("3_basic_off.png", &no_basic);
    t.row("3_basic_off.png, Basic Correction off: no warmth or contrast, the wheel's tint and vignette kept; draws cleanly", &format!("{said:?}"), said.is_empty());
    let off: serde_json::Map<String, J> = COLOR_GRADE_SECTIONS.iter().map(|s| (s.to_string(), J::from("off"))).collect();
    let (none, said) = picture(&dir, with(J::Object(off)));
    write("4_all_off.png", &none);
    let d = distance(&none, &before);
    t.row(
        "4_all_off.png, every section off, is the street exactly as it is",
        &format!("largest difference {} of 255; {said:?}", d.0),
        d.0 <= 1 && said.is_empty(),
    );
    t.row(
        "each switch changes the picture",
        &format!("all on against vignette off: {} pixels; against basic off: {}", distance(&all_on, &no_vignette).1, distance(&all_on, &no_basic).1),
        distance(&all_on, &no_vignette).1 > 0 && distance(&all_on, &no_basic).1 > 0,
    );

    t.finish("D-1009_color_grade_6_table.md");
}
