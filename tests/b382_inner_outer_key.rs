//! B-382: D-502 Inner/Outer Key, after After Effects' Inner/Outer Key ("Keying" in
//! `docs/effects/EFFECTS.md`): a foreground mask kept, a background mask's outside gone (or, with
//! one mask, a band of the highlight radius about it worked out), the pixels between them made
//! see-through as much as they are the background's colour and that colour taken back out, then
//! Edge Thin, Edge Feather, Edge Threshold and Blend With Original.
//!
//! Writes `verification/D-502_inner_outer_key_table.md`.
//!
//! Every expected pixel is `Fixtures/inner_outer_key/expected_inner_outer_key.json`, written by
//! `tools/inner_outer_key_reference.py` before this code existed and printed in document 25 as
//! FX-IOK-001 to 039. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-502 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, keys, median, on_card, repo, set, time_passes, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// An Inner/Outer Key: foreground, background, radius, thin, feather, threshold, blend; invert.
fn iok([foreground, background, single_mask_highlight_radius, edge_thin, edge_feather, edge_threshold, blend_with_original]: [f64; 7], invert: &str) -> Effect {
    Effect::InnerOuterKey {
        foreground,
        background,
        single_mask_highlight_radius,
        edge_thin,
        edge_feather,
        edge_threshold,
        invert_extraction: invert.to_string(),
        blend_with_original,
        paths: None,
    }
}

/// A circle as a closed path of four curves.
fn circle(((cx, cy), r): ((f64, f64), f64)) -> J {
    let k = 0.552_284_749_8 * r;
    json!({"points": [
        {"point": [cx, cy - r], "in": [-k, 0], "out": [k, 0]},
        {"point": [cx + r, cy], "in": [0, -k], "out": [0, k]},
        {"point": [cx, cy + r], "in": [k, 0], "out": [-k, 0]},
        {"point": [cx - r, cy], "in": [0, k], "out": [0, -k]},
    ]})
}

/// Masks that cut nothing (mode None), for the key to choose from.
fn masks(circles: &[((f64, f64), f64)]) -> J {
    J::Array(
        circles
            .iter()
            .enumerate()
            .map(|(m, &c)| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": circle(c), "keyframes": []}}))
            .collect(),
    )
}

/// The reference shot (1920 by 1080) with four masks (mode None: a small circle inside a large
/// one, and two more) and `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let d = 40.0 * i as f64;
        layers[i]["masks"] = masks(&[((960.0 + d, 540.0), 250.0), ((960.0 + d, 540.0), 400.0), ((560.0 + d, 400.0), 220.0), ((1400.0 + d, 620.0), 300.0)]);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.inner_outer_key" => json!({"foreground": 0, "background": 0, "single_mask_highlight_radius": 10, "edge_thin": 0, "edge_feather": 0,
            "edge_threshold": 0, "invert_extraction": "off", "blend_with_original": 0}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// key is drawn on the processor (the card draws what follows it), so it is never on the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 0,
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, the card's frame against the processor's.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(type_id, &format!("b382-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 0,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of. The street layer has
/// three masks that cut nothing: a circle about the middle, a larger one about it, and one to
/// the upper left.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["masks"] = masks(&[((240.0, 135.0), 70.0), ((240.0, 135.0), 110.0), ((110.0, 80.0), 50.0)]);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The street drawn plain and with each of `shots` of `type_id`, written as numbered pictures
/// into `verification/{d} pictures/`; a row each that it draws cleanly and changes what it says
/// it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[(&str, J, &str, fn(&[u8], &[u8]) -> bool)]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect (its three masks cut nothing); draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed of {}", distance(&bytes, &before).1, TOWN.0 * TOWN.1),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

fn is_iok(e: &Effect) -> bool {
    matches!(e, Effect::InnerOuterKey { .. })
}

/// The share of pixels that changed, and whether no pixel's alpha rose (a key only takes away).
fn taken_away(a: &[u8], b: &[u8]) -> (f64, bool) {
    let share = distance(a, b).1 as f64 / (a.len() / 4) as f64;
    (share, a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] <= q[3]))
}

/// Most pixels changed but not all, none made less see-through.
fn mostly(a: &[u8], b: &[u8]) -> bool {
    let (share, ok) = taken_away(a, b);
    share > 0.6 && share < 0.98 && ok
}

/// Some pixels changed but not most, none made less see-through.
fn in_part(a: &[u8], b: &[u8]) -> bool {
    let (share, ok) = taken_away(a, b);
    share > 0.0 && share < 0.6 && ok
}

#[test]
fn b382_inner_outer_key() {
    let mut t = Table::new(
        "inner_outer_key",
        "# D-502: Inner/Outer Key\n\nB-382, after After Effects' Inner/Outer Key, the rule ours (Adobe \
         publishes none): the foreground mask (its number's floor, on with two points or more) covers \
         the kept pixels by ADR-016's 4 by 4 even-odd rule; the background mask's outside is gone, or \
         with no background the pixels whose centres lie within the highlight radius of the outline are \
         unknown, the rest kept inside it and gone outside; Invert Extraction swaps kept and gone. Each \
         unknown pixel takes the mean colour of the kept and of the gone pixels in the square reaching \
         the nearest of each, is made see-through as much as it is the gone colour along the line \
         between them, and has that colour taken back out. Then Edge Thin (a disc's minimum, or \
         maximum when negative), Edge Feather (a Gaussian of sigma = feather / 2), Edge Threshold, \
         and Blend With Original. Foreground 0 keys nothing; a mask named and missing leaves the layer \
         as it is with a warning each frame. The layer never grows. Every expected pixel is \
         `Fixtures/inner_outer_key/expected_inner_outer_key.json`, written by \
         `tools/inner_outer_key_reference.py` before this code existed and printed in document 25 as \
         FX-IOK-001 to 039. Tolerance 2e-5.\n",
    );

    t.heading("FX-IOK-001 to 039 (document 25)");
    t.fixtures_numbered("expected_inner_outer_key.json", 1..=39);

    t.heading("The file");
    let all = files("fx_iok", 39);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_iok_017.json");
    t.row(
        "fx_iok_017.json is saved with its numbers and word as written, no paths",
        &saved.to_string(),
        saved["foreground"] == 1.5 && saved["background"] == 2.7 && saved["single_mask_highlight_radius"] == 10
            && saved["invert_extraction"] == "off" && saved["blend_with_original"] == 0 && saved.get("paths").is_none(),
    );
    why(
        &mut t,
        &[
            ("fx_iok_030.json", "Inner/Outer Key's foreground runs from 0 to 1000, and this is -1."),
            ("fx_iok_031.json", "Inner/Outer Key's background runs from 0 to 1000, and this is 1001."),
            ("fx_iok_032.json", "Inner/Outer Key's single mask highlight radius runs from 0 to 1000, and this is -1."),
            ("fx_iok_033.json", "Inner/Outer Key's edge thin runs from -100 to 100, and this is -101."),
            ("fx_iok_034.json", "Inner/Outer Key's edge thin runs from -100 to 100, and this is 101."),
            ("fx_iok_035.json", "Inner/Outer Key's edge feather runs from 0 to 1000, and this is 1001."),
            ("fx_iok_036.json", "Inner/Outer Key's edge threshold runs from 0 to 100, and this is 101."),
            ("fx_iok_037.json", "Inner/Outer Key's blend with original runs from 0 to 100, and this is 101."),
            ("fx_iok_038.json", "Inner/Outer Key's invert extraction is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    let ok = r#""single_mask_highlight_radius": 10, "edge_thin": 0, "edge_feather": 0, "edge_threshold": 0"#;
    t.shape_refused("fx_iok_002.json", "an Inner/Outer Key with no `blend_with_original`", &format!(r#"{{"foreground": 1, "background": 2, {ok}, "invert_extraction": "off"}}"#));
    t.shape_refused("fx_iok_002.json", "an Inner/Outer Key with a foreground in words", &format!(r#"{{"foreground": "first", "background": 2, {ok}, "invert_extraction": "off", "blend_with_original": 0}}"#));
    t.shape_refused("fx_iok_002.json", "an Inner/Outer Key whose invert extraction is true", &format!(r#"{{"foreground": 1, "background": 2, {ok}, "invert_extraction": true, "blend_with_original": 0}}"#));

    t.heading("Commands");
    let mut document = t.load("fx_iok_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("foreground -1", set(iok([-1.0, 2.0, 10.0, 0.0, 0.0, 0.0, 0.0], "off"))),
            ("edge thin 101", set(iok([1.0, 2.0, 10.0, 101.0, 0.0, 0.0, 0.0], "off"))),
            ("edge feather 1001", set(iok([1.0, 2.0, 10.0, 0.0, 1001.0, 0.0, 0.0], "off"))),
            ("invert extraction \"yes\"", set(iok([1.0, 2.0, 10.0, 0.0, 0.0, 0.0, 0.0], "yes"))),
            ("blend with original keyed to 101", keys("blend_with_original", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_iok_002.json",
        vec![
            ("foreground 2, background 1, radius 3, thin -2, feather 4, threshold 30, inverted, blend 20", set(iok([2.0, 1.0, 3.0, -2.0, 4.0, 30.0, 20.0], "on"))),
            ("edge feather keyed from 0 to 6", keys("edge_feather", &[(0, &[0.0]), (4, &[6.0])])),
            ("single mask highlight radius keyed from 0 to 5", keys("single_mask_highlight_radius", &[(0, &[0.0]), (4, &[5.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_iok_002.json", 0), ("fx_iok_003.json", 0), ("fx_iok_008.json", 0), ("fx_iok_011.json", 0), ("fx_iok_014.json", 2), ("fx_iok_016.json", 0), ("fx_iok_021.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255 (the key is drawn on the processor; the card draws what follows it)");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_iok", 39, is_iok);
    card_reference(
        &mut t,
        &mut gpu,
        "Inner/Outer Key",
        "core.inner_outer_key",
        &[
            ("foreground 1 (a circle of radius 250), background 2 (radius 400 about it)", json!({"foreground": 1, "background": 2})),
            ("foreground 1 alone, radius 10", json!({"foreground": 1})),
            ("foreground 3 alone, radius 40, inverted", json!({"foreground": 3, "single_mask_highlight_radius": 40, "invert_extraction": "on"})),
            ("foreground 1, background 2, thin 3, feather 12, threshold 20", json!({"foreground": 1, "background": 2, "edge_thin": 3, "edge_feather": 12, "edge_threshold": 20})),
            ("foreground 4 alone, radius 25, thin -4, blend 40", json!({"foreground": 4, "single_mask_highlight_radius": 25, "edge_thin": -4, "blend_with_original": 40})),
        ],
        is_iok,
    );

    street(
        &mut t,
        "D-502",
        "core.inner_outer_key",
        &[
            ("two_masks", json!({"foreground": 1, "background": 2}), "foreground 1 (a circle of radius 70), background 2 (110 about it): the inner disc kept, outside the outer one gone, the ring between see-through as much as it is the colour outside", mostly),
            ("one_mask", json!({"foreground": 1}), "foreground 1 alone, radius as added (10): the disc kept with a soft rim, the rest gone", mostly),
            ("inverted", json!({"foreground": 1, "background": 2, "invert_extraction": "on"}), "the two masks with Invert Extraction: the inner disc gone, the street outside the outer one kept", in_part),
            ("feathered", json!({"foreground": 1, "background": 2, "edge_thin": 4, "edge_feather": 16}), "the two masks, thin 4, feather 16: the kept disc a little smaller, its edge soft", mostly),
            ("blended", json!({"foreground": 3, "single_mask_highlight_radius": 6, "blend_with_original": 50}), "foreground 3 (the upper left circle) alone, radius 6, blend 50: the street half see-through outside it", mostly),
        ],
    );

    t.finish("D-502_inner_outer_key_table.md");
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B382_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-382: a measurement, run deliberately with --release --ignored"]
fn b382_inner_outer_key_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B382_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Inner/Outer Key, two masks", Some(("core.inner_outer_key", json!({"foreground": 1, "background": 2})))),
        ("Noise, then Inner/Outer Key, one mask, radius 10", Some(("core.inner_outer_key", json!({"foreground": 1})))),
        ("Noise, then Inner/Outer Key, two masks, thin 3, feather 12, threshold 20", Some(("core.inner_outer_key", json!({"foreground": 1, "background": 2, "edge_thin": 3, "edge_feather": 12, "edge_threshold": 20})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B382_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-382_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
