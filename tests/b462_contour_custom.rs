//! B-462: D-584, Contour Paint's Match to Nearest Custom Color, after After Effects' Reduction
//! Method of that name and its Custom Color Set: `reduction_method`
//! "match_to_nearest_custom_color" and D-87's chosen colours, `colors`, as the set.
//!
//! Writes `verification/D-584_contour_custom_table.md` and draws pictures into
//! `verification/D-584 pictures/`.
//!
//! Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by
//! `tools/contour_paint_reference.py` before this code existed and printed in document 25 as
//! FX-CONTOUR-033 to 041. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, median, on_card, reference, repo, set, strings, time_passes, town, Table, MAIN, NINE, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

const CUSTOM: &str = "match_to_nearest_custom_color";

/// A Contour Paint with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"view": "final_result", "stylization": 50, "cleanliness": 50, "smear_iterations": 3,
        "apply_to_alpha": "off", "draw_color_fills": "none", "reduction_method": "k_means", "posterization_level": 8,
        "low_strength": 0, "high_strength": 100, "offset_flow_angles": 0, "quantize_flow_angles": 0, "edge_map_layer": ""});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.contour_paint", "enabled": true, "parameters": all})
}

/// The `holder` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

/// `command` aimed at the `holder` layer's effect `fx-1`; the fixtures' drawing is `holder`, not `art`.
fn holder(mut command: Command) -> Command {
    if let Command::SetEffectParameters { layer_id, instance_id, .. } = &mut command {
        *layer_id = Id::new("holder");
        *instance_id = Id::new("fx-1");
    }
    command
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at frame 0,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
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
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

fn rgb(hex: &str) -> [u8; 3] {
    let b = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).unwrap();
    [b(1), b(3), b(5)]
}

#[test]
fn b462_contour_custom() {
    let mut t = Table::new(
        "contour_paint",
        "# D-584: Contour Paint, Match to Nearest Custom Color\n\nB-462: `core.contour_paint`'s Reduction Method \
         takes After Effects' Match to Nearest Custom Color as `match_to_nearest_custom_color`, with D-87's chosen \
         colours (`colors`, up to eight `#rrggbb`) as the Custom Color Set, each pixel filled with the custom colour \
         nearest it in CIE L*a*b* (CIE76). Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, \
         written by `tools/contour_paint_reference.py` before this code existed and printed in document 25 as \
         FX-CONTOUR-033 to 041. Tolerance 2e-5.\n",
    );

    t.heading("FX-CONTOUR-033 to 041 (document 25)");
    t.fixtures_numbered("expected_contour_paint.json", 33..=41);

    t.heading("The file");
    t.round_trips(&["fx_contour_033.json", "fx_contour_034.json", "fx_contour_035.json", "fx_contour_036.json", "fx_contour_037.json", "fx_contour_038.json"]);
    let saved = t.saved_parameters("fx_contour_033.json");
    t.row(
        "fx_contour_033.json is saved with its fourteen settings, the four custom colours among them, and no palette",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 14
            && saved["reduction_method"] == CUSTOM
            && saved["colors"] == json!(["#c03030", "#3050c0", "#808080", "#f0f0c0"])
            && saved.get("palette").is_none(),
    );
    let old = t.saved_parameters("fx_contour_010.json");
    t.row("fx_contour_010.json, from before the set, opens with none and saves without it", &old.to_string(), old.get("colors").is_none());
    for (file, want) in [("fx_contour_039.json", "\"#c0303\""), ("fx_contour_040.json", "up to eight colours"), ("fx_contour_041.json", "\"red\"")] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let full = r##"{"view": "final_result", "stylization": 50, "cleanliness": 50, "smear_iterations": 3, "apply_to_alpha": "off", "draw_color_fills": "over_smeared_image", "reduction_method": "match_to_nearest_custom_color", "posterization_level": 8, "low_strength": 0, "high_strength": 100, "offset_flow_angles": 0, "quantize_flow_angles": 0, "edge_map_layer": "", "colors": "#c03030"}"##;
    t.shape_refused("fx_contour_033.json", "a Contour Paint whose custom colours are one word, not a list", full);

    t.heading("Commands");
    let mut document = t.load("fx_contour_010.json").document;
    let nine = changed(&t, "fx_contour_010.json", |e| if let Effect::ContourPaint { colors, reduction_method, .. } = e { *colors = strings(NINE); *reduction_method = CUSTOM.into() });
    let bad = changed(&t, "fx_contour_010.json", |e| if let Effect::ContourPaint { colors, reduction_method, .. } = e { *colors = strings(&["#12"]); *reduction_method = CUSTOM.into() });
    let effect_of = |d: &anime_compositor::command::Document| format!("{:?}", d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]);
    let held = effect_of(&document);
    for (what, command) in [("nine custom colours", holder(set(nine))), ("a custom colour \"#12\"", holder(set(bad)))] {
        let refused = document.apply(command).err();
        let untouched = effect_of(&document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let matched = changed(&t, "fx_contour_010.json", |e| if let Effect::ContourPaint { colors, reduction_method, .. } = e { *colors = strings(&["#ffffff", "#000000"]); *reduction_method = CUSTOM.into() });
    t.taken(&mut document, "fx_contour_010.json", vec![("Match to Nearest Custom Color with white and black", holder(set(matched)))]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_contour_033.json", 0), ("fx_contour_034.json", 0), ("fx_contour_037.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in 33..=41u32 {
        let file = format!("fx_contour_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ContourPaint { .. }));
            }
        }
        // The refused ones (039 to 041) leave nothing for the card.
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (n >= 39),
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    let five = json!(["#1d1b2e", "#3a5a8c", "#e8c39e", "#c0392b", "#f4f1e8"]);
    for (what, p) in [
        ("fills Over Smeared Image matched to five custom colours", json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": five})),
        ("Fills Only, Low Strength 10, matched to two custom colours", json!({"draw_color_fills": "only", "low_strength": 10, "reduction_method": CUSTOM, "colors": ["#202040", "#f0e0c0"]})),
    ] {
        let project = reference(|id| json!([fx(&format!("b462-{id}"), &p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Contour Paint {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ContourPaint { .. }));
                t.row(
                    &format!("the reference shot, Contour Paint {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: in `verification/D-584 pictures/`");
    let dir = repo("verification/D-584 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let colours = |p: &[u8]| p.chunks_exact(4).filter(|q| q[3] > 0).map(|q| [q[0], q[1], q[2]]).collect::<BTreeSet<_>>();
    let set_of = |list: &[&str]| list.iter().map(|h| rgb(h)).collect::<BTreeSet<_>>();

    let (k_means, said) = street(json!({"draw_color_fills": "over_smeared_image", "posterization_level": 5}));
    write("2_k_means_5_colours.png", &k_means);
    t.row(
        "2_k_means_5_colours.png, fills by K-Means in 5 colours, for comparison: colours the program picks from the street; draws cleanly",
        &format!("{said:?}, {} colours", colours(&k_means).len()),
        said.is_empty() && colours(&k_means).len() <= 5,
    );

    let night = ["#1d1b2e", "#3a5a8c", "#e8c39e", "#c0392b", "#f4f1e8"];
    let (custom, said) = street(json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": night}));
    write("3_custom_5_colours.png", &custom);
    t.row(
        "3_custom_5_colours.png, Match to Nearest Custom Color with five chosen colours (a night navy, a blue, a skin tone, a red, an off-white): every pixel one of those five; draws cleanly",
        &format!("{said:?}, colours used {:?}", colours(&custom).len()),
        said.is_empty() && colours(&custom).is_subset(&set_of(&night)) && colours(&custom).len() >= 2,
    );

    let two = ["#202040", "#f0e0c0"];
    let (pair, said) = street(json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": two}));
    write("4_custom_2_colours.png", &pair);
    t.row(
        "4_custom_2_colours.png, only a dark blue and a cream: the street in two tones, dark where it is dark and cream where it is light; draws cleanly",
        &format!("{said:?}, colours {:?}", colours(&pair)),
        said.is_empty() && colours(&pair) == set_of(&two),
    );

    let (none, said) = street(json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": []}));
    let (smear, _) = street(json!({}));
    write("5_custom_no_colours.png", &none);
    t.row(
        "5_custom_no_colours.png, Match to Nearest Custom Color with no colours chosen: nothing is filled, the smear alone, the same as the effect without fills; draws cleanly",
        &format!("{said:?}, the same as the smear alone {}", none == smear),
        said.is_empty() && none == smear && none != before,
    );

    t.finish("D-584_contour_custom_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B462_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-462: a measurement, run deliberately with --release --ignored"]
fn b462_contour_custom_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B462_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n- Loops: {passes}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cpu { "the processor" } else { "the card" },
    );
    let eight = json!(["#1d1b2e", "#3a5a8c", "#e8c39e", "#c0392b", "#f4f1e8", "#5d7a3a", "#8e6c8a", "#d9a441"]);
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        (
            "Noise, then Contour Paint, fills Over Smeared Image matched to 2 custom colours",
            Some(json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": ["#202040", "#f0e0c0"]})),
        ),
        ("the same matched to 8 custom colours", Some(json!({"draw_color_fills": "over_smeared_image", "reduction_method": CUSTOM, "colors": eight}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}b"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B462_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-462_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
