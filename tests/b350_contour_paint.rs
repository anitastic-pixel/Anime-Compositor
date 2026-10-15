//! B-350: D-470, After Effects' Contour Paint (Stylize) under `core.contour_paint`: View,
//! Stylization, Cleanliness, Smear Iterations, Apply to Alpha, Draw Color Fills, Reduction Method,
//! Posterization Level, Low and High Strength, Offset and Quantize Flow Angles, External Edge Map.
//!
//! Writes `verification/D-470_contour_paint_table.md` and draws pictures into
//! `verification/D-470 pictures/`.
//!
//! Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by
//! `tools/contour_paint_reference.py` before this code existed and printed in document 25 as
//! FX-CONTOUR-001 to 032. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

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

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=32 {
        let file = format!("fx_contour_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ContourPaint { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b350-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Contour Paint {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ContourPaint { .. }));
                t.row(
                    &format!("the reference shot, Contour Paint {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), with the still `texture` (in `dir`)
/// beneath it as the hidden layer `bricks`, drawn on the processor at frame 0, straight 8-bit,
/// with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), texture: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let mut bricks_asset = project["assets"][0].clone();
    bricks_asset["id"] = J::from("asset-bricks");
    bricks_asset["name"] = J::from("bricks");
    bricks_asset["path"] = J::from(texture);
    project["assets"].as_array_mut().unwrap().push(bricks_asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut hidden = layer.clone();
    hidden["id"] = J::from("bricks");
    hidden["name"] = J::from("bricks");
    hidden["asset_id"] = J::from("asset-bricks");
    hidden["enabled"] = J::from(false);
    hidden["transform"]["anchor"]["base"] = json!([0, 0]);
    hidden["transform"]["position"]["base"] = json!([0, 0]);
    hidden["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().push(hidden);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("bricks"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// A 32 by 16 brick: a light face with a dark 2-pixel mortar line along its top and left, and
/// one down its middle on the lower half, so tiled it makes a running bond.
fn brick() -> Vec<u8> {
    let mut bytes = Vec::with_capacity(32 * 16 * 4);
    for y in 0..16 {
        for x in 0..32 {
            let mortar = y % 8 < 2 || (y < 8 && x < 2) || (y >= 8 && (16..18).contains(&x));
            let v = if mortar { 60 } else { 210 };
            bytes.extend_from_slice(&[v, v, v, 255]);
        }
    }
    bytes
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
    if let Command::SetEffectParameters { layer_id, instance_id, .. } | Command::SetEffectKeys { layer_id, instance_id, .. } = &mut command {
        *layer_id = Id::new("holder");
        *instance_id = Id::new("fx-1");
    }
    command
}

#[test]
fn b350_contour_paint() {
    let mut t = Table::new(
        "contour_paint",
        "# D-470: Contour Paint\n\nB-350: `core.contour_paint` takes After Effects' Contour Paint controls \
         (View, Stylization, Cleanliness, Smear Iterations, Apply to Alpha, Draw Color Fills, Reduction Method, \
         Posterization Level, Low and High Strength, Offset and Quantize Flow Angles, External Edge Map). Every \
         expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by \
         `tools/contour_paint_reference.py` before this code existed and printed in document 25 as \
         FX-CONTOUR-001 to 032. Tolerance 2e-5.\n",
    );

    t.heading("FX-CONTOUR-001 to 032 (document 25)");
    t.fixtures_numbered("expected_contour_paint.json", 1..=32);

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_contour_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_contour_017.json");
    t.row(
        "fx_contour_017.json is saved with its thirteen settings, and nothing the program works out for itself",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 13
            && saved["edge_map_layer"] == "ramp"
            && saved["view"] == "final_result"
            && saved["smear_iterations"] == 3.0
            && saved["reduction_method"] == "k_means"
            && saved.get("palette").is_none()
            && saved.get("px").is_none(),
    );
    for (file, want) in [
        ("fx_contour_023.json", "stylization"),
        ("fx_contour_024.json", "smear iterations"),
        ("fx_contour_025.json", "view"),
        ("fx_contour_026.json", "draw color fills"),
        ("fx_contour_027.json", "reduction method"),
        ("fx_contour_028.json", "posterization level"),
        ("fx_contour_029.json", "quantize flow angles"),
        ("fx_contour_030.json", "external edge map"),
        ("fx_contour_031.json", "apply to alpha"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Contour Paint and its {want}"), &why, why.contains("Contour Paint") && why.contains(want));
    }
    let full = r##"{"view": "final_result", "stylization": 50, "cleanliness": 50, "smear_iterations": 3, "apply_to_alpha": "off", "draw_color_fills": "none", "reduction_method": "k_means", "posterization_level": 8, "low_strength": 0, "high_strength": 100, "offset_flow_angles": 0, "quantize_flow_angles": 0, "edge_map_layer": ""}"##;
    t.shape_refused("fx_contour_001.json", "a Contour Paint whose stylization is a word", &full.replace("\"stylization\": 50", "\"stylization\": \"strong\""));
    t.shape_refused("fx_contour_001.json", "a Contour Paint without its cleanliness", &full.replace(", \"cleanliness\": 50", ""));
    t.shape_refused("fx_contour_001.json", "a Contour Paint whose view is a number", &full.replace("\"final_result\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_contour_001.json").document;
    let over = changed(&t, "fx_contour_001.json", |e| if let Effect::ContourPaint { stylization, .. } = e { *stylization = 120.0 });
    let view = changed(&t, "fx_contour_001.json", |e| if let Effect::ContourPaint { view, .. } = e { *view = "arrows".into() });
    let number = changed(&t, "fx_contour_001.json", |e| if let Effect::ContourPaint { edge_map_layer, .. } = e { *edge_map_layer = json!(3) });
    let effect_of = |d: &Document| d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].clone();
    let held = format!("{:?}", effect_of(&document));
    for (what, command) in [
        ("stylization 120", set(over)),
        ("view arrows", set(view)),
        ("external edge map the number 3", set(number)),
        ("smear iterations keyed to 150", keys("smear_iterations", &[(0, &[3.0]), (4, &[150.0])])),
    ] {
        let refused = document.apply(holder(command)).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let moved = changed(&t, "fx_contour_001.json", |e| {
        if let Effect::ContourPaint { edge_map_layer, offset_flow_angles, quantize_flow_angles, draw_color_fills, .. } = e {
            *edge_map_layer = json!("ramp");
            *offset_flow_angles = 30.0;
            *quantize_flow_angles = 45.0;
            *draw_color_fills = "only".into();
        }
    });
    t.taken(
        &mut document,
        "fx_contour_001.json",
        vec![
            ("the ramp as the edge map, the flow turned 30 and snapped to 45, fills only", holder(set(moved))),
            ("posterization level keyed from 2 to 32", holder(keys("posterization_level", &[(0, &[2.0]), (4, &[32.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_contour_001.json", 0),
        ("fx_contour_008.json", 0),
        ("fx_contour_010.json", 0),
        ("fx_contour_014.json", 0),
        ("fx_contour_017.json", 0),
        ("fx_contour_020.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Smear Iterations 0 with no fills (002) leaves the effect out, snapped flow (016) is drawn
    // by the processor; refused ones (023 to 032) leave nothing for the card.
    let none: Vec<u32> = [2, 16].into_iter().chain(23..=32).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "Stylization 80, Apply to Alpha on, fills Over Smeared Image by Median in 6 colours, the flow turned 30",
                json!({"stylization": 80, "apply_to_alpha": "on", "draw_color_fills": "over_smeared_image", "reduction_method": "median",
                    "posterization_level": 6, "offset_flow_angles": 30}),
                3,
            ),
            ("the flow snapped to 15 degrees: a step, drawn by the processor", json!({"quantize_flow_angles": 15}), 0),
            ("View Edge Map with layer4 as the External Edge Map", json!({"view": "edge_map", "edge_map_layer": "layer-4"}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-470 pictures/`");
    let dir = repo("verification/D-470 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    // The brick tiled over the whole street, so as an edge map it steers every pixel.
    let (w, h) = TOWN;
    let one = brick();
    let wall: Vec<u8> = (0..w * h).flat_map(|i| { let (x, y) = (i % w % 32, i / w % 16); one[(y * 32 + x) * 4..(y * 32 + x) * 4 + 4].to_vec() }).collect();
    write("brick_wall.png", TOWN, &wall);
    let street = |p: J| picture(&dir, "town.png", TOWN, "brick_wall.png", json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, "brick_wall.png", json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let coverings = |p: &[u8]| p.chunks_exact(4).map(|q| q[3]).collect::<Vec<_>>();
    // How far a picture is from the street, summed over every colour.
    let away = |p: &[u8]| p.iter().zip(&before).map(|(a, b)| a.abs_diff(*b) as u64).sum::<u64>();
    // The distinct colours of a picture's pixels that are not clear.
    let colours = |p: &[u8]| p.chunks_exact(4).filter(|q| q[3] > 0).map(|q| [q[0], q[1], q[2]]).collect::<HashSet<_>>().len();

    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, Contour Paint as added: the street smeared along its own edges into strokes, the covering exactly as before; draws cleanly",
        &format!("{said:?}, changed {}, coverings the same {}", added != before, coverings(&added) == coverings(&before)),
        said.is_empty() && added != before && coverings(&added) == coverings(&before),
    );

    let (long, said) = street(json!({"stylization": 100, "smear_iterations": 6}));
    write("3_long_strokes.png", TOWN, &long);
    t.row(
        "3_long_strokes.png, Stylization 100 and 6 smears: longer strokes, further from the street than 2_as_added.png; draws cleanly",
        &format!("{said:?}, distance from the street {} against {}", away(&long), away(&added)),
        said.is_empty() && away(&long) > away(&added),
    );

    let (filled, said) = street(json!({"draw_color_fills": "over_smeared_image", "posterization_level": 6}));
    write("4_fills_6_colours.png", TOWN, &filled);
    t.row(
        "4_fills_6_colours.png, Draw Color Fills Over Smeared Image, K-Means, 6 colours: the street in at most 6 flat colours; draws cleanly",
        &format!("{said:?}, {} colours against {} in 2_as_added.png", colours(&filled), colours(&added)),
        said.is_empty() && colours(&filled) <= 6 && colours(&added) > 6,
    );

    let (strong, said) = street(json!({"draw_color_fills": "only", "posterization_level": 6, "low_strength": 10}));
    write("5_fills_only_strong_edges.png", TOWN, &strong);
    let clear = strong.chunks_exact(4).filter(|q| q[3] == 0).count();
    t.row(
        "5_fills_only_strong_edges.png, Fills Only, Low Strength 10: flat colours along the buildings' and road's edges only, clear between (6_edge_map.png shows the strengths); draws cleanly",
        &format!("{said:?}, {clear} of {} pixels clear, {} colours", w * h, colours(&strong)),
        said.is_empty() && clear > 0 && clear < w * h && colours(&strong) <= 6,
    );

    let (edges, said) = street(json!({"view": "edge_map"}));
    write("6_edge_map.png", TOWN, &edges);
    let cyan = edges.chunks_exact(4).filter(|q| q[1] as i32 - q[0] as i32 > 60 && q[2] as i32 - q[0] as i32 > 60).count();
    let opaque = edges.chunks_exact(4).all(|q| q[3] == 255);
    t.row(
        "6_edge_map.png, View Edge Map: the edges' strength in grey, opaque everywhere, with short cyan lines along the flow; draws cleanly",
        &format!("{said:?}, opaque {opaque}, {cyan} cyan pixels"),
        said.is_empty() && opaque && cyan > 0,
    );

    let (bricks, said) = street(json!({"edge_map_layer": "bricks", "stylization": 100}));
    let (own, _) = street(json!({"stylization": 100}));
    write("7_steered_by_bricks.png", TOWN, &bricks);
    t.row(
        "7_steered_by_bricks.png, a brick wall (brick_wall.png) as the External Edge Map, Stylization 100: the street smeared along the wall's mortar lines instead of its own edges, so it differs from the same smear steered by the street; draws cleanly",
        &format!("{said:?}, differs from the street's own flow {}", bricks != own),
        said.is_empty() && bricks != own && bricks != before,
    );

    t.finish("D-470_contour_paint_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B350_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-350: a measurement, run deliberately with --release --ignored"]
fn b350_contour_paint_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B350_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Contour Paint as added (three smears)", Some(json!({}))),
        ("the same with fills Over Smeared Image, K-Means, 8 colours", Some(json!({"draw_color_fills": "over_smeared_image"}))),
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
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame"));
                } else {
                    drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                }
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B350_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-350_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
