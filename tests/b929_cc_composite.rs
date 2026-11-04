//! B-929: D-1049, `core.cc_composite`, after After Effects' CC Composite (Channel): the layer as
//! it was before its first effect laid back on what the effects made, in front, behind, or by
//! add, multiply or screen, at an opacity, optionally keeping the effects' alpha (RGB Only).
//!
//! Writes `verification/D-1049_cc_composite_table.md` and draws pictures into
//! `verification/D-1049 pictures/`.
//!
//! Every expected pixel is `Fixtures/cc_composite/expected_cc_composite.json`, written by
//! `tools/cc_composite_reference.py` before this code existed and printed in document 25 as
//! FX-CCCOMP-001 to 016. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, distance, on_card, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::Id;
use anime_compositor::preview::PreviewQuality;
use anime_compositor::{persist, png_out, OutputDepth};

/// A CC Composite with the settings `p`, every one `p` leaves out at its start.
fn cc(id: &str, p: &J) -> J {
    let mut all = json!({"opacity": 100, "composite_original": "in_front", "rgb_only": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.cc_composite", "enabled": true, "parameters": all})
}

/// A solid blue, #3366cc, in place of the layer (Solid Composite, source opacity 0).
fn blue(id: &str) -> J {
    json!({"instance_id": id, "type_id": "core.solid_composite", "enabled": true,
           "parameters": {"source_opacity": 0, "color": "#3366cc", "opacity": 100, "blend": "normal"}})
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
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
fn b929_cc_composite() {
    let mut t = Table::new(
        "cc_composite",
        "# D-1049: CC Composite\n\nB-929: `core.cc_composite` lays the layer as it was before \
         its first effect back on what its effects made: in front, behind, or by add, multiply \
         or screen, at an opacity, and with RGB Only the effects' alpha kept. After After \
         Effects' CC Composite (Channel), as lane w12g's long-shadow tutorial (wWtZXibe2QI at \
         1:55) uses it to put the logo back on top of its shadow. Every expected pixel is \
         `Fixtures/cc_composite/expected_cc_composite.json`, written by \
         `tools/cc_composite_reference.py` before this code existed and printed in document 25 \
         as FX-CCCOMP-001 to 016. Tolerance 2e-5.\n",
    );

    t.heading("FX-CCCOMP-001 to 016 (document 25)");
    t.fixtures_numbered("expected_cc_composite.json", 1..=16);

    t.heading("The file");
    let files: Vec<String> = (1..=16).map(|n| format!("fx_cccomp_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_cccomp_010.json");
    t.row(
        "fx_cccomp_010.json is saved with its three settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 3
            && saved["opacity"] == 100.0
            && saved["composite_original"] == "in_front"
            && saved["rgb_only"] == "on",
    );
    for (file, want) in [
        ("fx_cccomp_013.json", "opacity"),
        (
            "fx_cccomp_014.json",
            "composite original is \"in_front\", \"behind\", \"add\", \"multiply\" or \"screen\"",
        ),
        ("fx_cccomp_015.json", "RGB only is \"off\" or \"on\""),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence saying {want}"),
            &why,
            why.contains(want),
        );
    }
    t.shape_refused(
        "fx_cccomp_001.json",
        "a CC Composite whose opacity is a word",
        r#"{"opacity": "full", "composite_original": "in_front", "rgb_only": "off"}"#,
    );
    t.shape_refused(
        "fx_cccomp_001.json",
        "a CC Composite without its RGB only",
        r#"{"opacity": 100, "composite_original": "in_front"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_cccomp_001.json").document;
    let edit = |f: fn(&mut String, &mut String)| {
        changed(&t, "fx_cccomp_001.json", move |e| {
            if let Effect::CcComposite { composite_original, rgb_only, .. } = e {
                f(composite_original, rgb_only);
            }
        })
    };
    let stencil = edit(|m, _| *m = "stencil".into());
    let yes = edit(|_, r| *r = "yes".into());
    let behind = edit(|m, _| *m = "behind".into());
    let on = edit(|_, r| *r = "on".into());
    t.refused(
        &mut document,
        vec![("composite original stencil", set(stencil)), ("RGB only yes", set(yes))],
    );
    t.taken(
        &mut document,
        "fx_cccomp_001.json",
        vec![("composite original behind", set(behind)), ("RGB only on", set(on))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_cccomp_002.json", 0),
        ("fx_cccomp_003.json", 0),
        ("fx_cccomp_007.json", 0),
        ("fx_cccomp_009.json", 0),
        ("fx_cccomp_012.json", 3),
    ]);

    t.heading("The card: a CC Composite and every effect above it are drawn by the processor");
    let mut gpu = Gpu::new().expect("a usable card");
    let none: Vec<u32> = (1..=16).collect();
    t.card_fixtures(&mut gpu, "expected_cc_composite.json", &none, |e| {
        matches!(e, Effect::CcComposite { .. })
    });
    let comp = Id::new(MAIN);
    let project = t.load("fx_cccomp_002.json").document.project().clone();
    let above = on_card(&project, &comp, &t.root, 0, PreviewQuality::Full, |e| {
        matches!(e, Effect::SolidComposite { .. })
    });
    t.row(
        "fx_cccomp_002.json: the Solid Composite above the CC Composite is not on the card",
        &format!("{above} on the card"),
        above == 0,
    );
    let mut after: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("fx_cccomp_002.json")).unwrap(),
    )
    .unwrap();
    let mut second = blue("fx-0-2");
    second["parameters"]["source_opacity"] = J::from(100);
    second["parameters"]["opacity"] = J::from(40);
    after["compositions"][0]["layers"][0]["effects"]
        .as_array_mut()
        .unwrap()
        .push(second);
    let after = persist::load_str(&after.to_string()).unwrap().document.project().clone();
    let below = on_card(&after, &comp, &t.root, 0, PreviewQuality::Full, |e| {
        matches!(e, Effect::SolidComposite { .. })
    });
    let (d, refused, a, b) = both(&mut gpu, &after, &comp, &t.root, 0, PreviewQuality::Full);
    t.row(
        "fx_cccomp_002.json with a second Solid Composite after the CC Composite: that one goes to the card, and the card's frame is the processor's within 1 level",
        &format!("{below} on the card; largest difference {} of 255; refused: {refused}; warnings CPU [{a}] GPU [{b}]", d.0),
        below == 1 && d.0 <= 1 && !refused && a == b,
    );

    t.heading("Pictures: in `verification/D-1049 pictures/`");
    let dir = repo("verification/D-1049 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap()
    };
    write("town.png", &town());
    let street = |e: J| picture(&dir, "town.png", TOWN, e);
    let (before, said) = street(json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (solid, said) = street(json!([blue("fx-0-0")]));
    write("2_blue.png", &solid);
    let all_blue = solid.chunks_exact(4).all(|p| p[0].abs_diff(0x33) <= 1 && p[1].abs_diff(0x66) <= 1 && p[2].abs_diff(0xcc) <= 1);
    t.row("2_blue.png, a Solid Composite that leaves only #3366cc: every pixel the blue, within 1 level; draws cleanly", &format!("{said:?}, all blue: {all_blue}"), said.is_empty() && all_blue);

    let (front, said) = street(json!([blue("fx-0-0"), cc("fx-0-1", &json!({}))]));
    write("3_blue_then_in_front.png", &front);
    t.row(
        "3_blue_then_in_front.png, the blue then CC Composite in front: the street back on top, byte for byte 1_before.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 1_before.png", distance(&front, &before).1),
        said.is_empty() && front == before,
    );

    let (back, said) = street(json!([blue("fx-0-0"), cc("fx-0-1", &json!({"composite_original": "behind"}))]));
    write("4_blue_then_behind.png", &back);
    t.row(
        "4_blue_then_behind.png, the same behind: the blue covers the street, byte for byte 2_blue.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 2_blue.png", distance(&back, &solid).1),
        said.is_empty() && back == solid,
    );

    let (half, said) = street(json!([blue("fx-0-0"), cc("fx-0-1", &json!({"opacity": 50}))]));
    write("5_blue_then_in_front_50.png", &half);
    let between = before
        .chunks_exact(4)
        .zip(half.chunks_exact(4))
        .flat_map(|(b, h)| (0..3).map(move |c| (b[c] as i32, h[c] as i32)))
        .zip([0x33i32, 0x66, 0xcc].into_iter().cycle())
        .all(|((b, h), s)| h + 1 >= b.min(s) && h <= b.max(s) + 1);
    t.row(
        "5_blue_then_in_front_50.png, in front at opacity 50: the street half seen over the blue; every channel between the street's and the blue's, within 1 level; draws cleanly",
        &format!("{said:?}, between: {between}"),
        said.is_empty() && between && half != before && half != solid,
    );

    let (dark, said) = street(json!([blue("fx-0-0"), cc("fx-0-1", &json!({"composite_original": "multiply"}))]));
    write("6_blue_then_multiply.png", &dark);
    let darker = before
        .chunks_exact(4)
        .zip(dark.chunks_exact(4))
        .all(|(b, d)| (0..3).all(|c| d[c] as i32 <= b[c] as i32 + 1));
    t.row(
        "6_blue_then_multiply.png, multiply: the street tinted blue and darker, no channel lighter than the street's, within 1 level; draws cleanly",
        &format!("{said:?}, none lighter: {darker}"),
        said.is_empty() && darker && dark != before,
    );

    t.finish("D-1049_cc_composite_table.md");
}
