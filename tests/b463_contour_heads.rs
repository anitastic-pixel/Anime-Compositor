//! B-463: D-585, Contour Paint's Arrowheads, after the arrows After Effects' Edge Map view
//! draws along the flow: `arrowheads` "on" or "off", "on" when added, "off" for a file without it.
//!
//! Writes `verification/D-585_contour_heads_table.md` and draws pictures into
//! `verification/D-585 pictures/`.
//!
//! Every expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by
//! `tools/contour_paint_reference.py` before this code existed and printed in document 25 as
//! FX-CONTOUR-042 to 047. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, on_card, reference, repo, set, town, Table, MAIN, TOWN};
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

/// The cyan-leaning pixels of a straight 8-bit picture: where the lines and heads are drawn.
fn cyan(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0 && (q[0] as i32) + 8 < q[1] as i32 && q[1] == q[2]).count()
}

#[test]
fn b463_contour_heads() {
    let mut t = Table::new(
        "contour_paint",
        "# D-585: Contour Paint, Arrowheads\n\nB-463: `core.contour_paint`'s Edge Map view draws a head at the end \
         of each flow line, as After Effects' Edge Map draws arrows: two strokes a fifth of the cell long, back \
         along the line turned 30 degrees each way. `arrowheads` is `on` or `off`, `on` when added, `off` for a \
         file without it (so a file from before D-585 draws as it did, D-304's way), written only when `on`. Every \
         expected pixel is `Fixtures/contour_paint/expected_contour_paint.json`, written by \
         `tools/contour_paint_reference.py` before this code existed and printed in document 25 as FX-CONTOUR-042 \
         to 047. Tolerance 2e-5.\n",
    );

    t.heading("FX-CONTOUR-042 to 047 (document 25)");
    t.fixtures_numbered("expected_contour_paint.json", 42..=47);
    t.heading("The cases from before D-585 still pass");
    t.fixtures_numbered("expected_contour_paint.json", 14..=14);
    t.fixtures_numbered("expected_contour_paint.json", 18..=18);

    t.heading("The file");
    t.round_trips(&["fx_contour_042.json", "fx_contour_043.json", "fx_contour_044.json", "fx_contour_045.json", "fx_contour_046.json", "fx_contour_047.json", "fx_contour_014.json"]);
    let saved = t.saved_parameters("fx_contour_042.json");
    t.row(
        "fx_contour_042.json is saved with its fourteen settings, Arrowheads on among them",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 14 && saved["arrowheads"] == "on",
    );
    let off = t.saved_parameters("fx_contour_043.json");
    t.row(
        "fx_contour_043.json, Arrowheads written off, is saved with it as written, so the file holds what it held",
        &off.to_string(),
        off["arrowheads"] == "off",
    );
    let old = t.saved_parameters("fx_contour_014.json");
    let heads = |e: &Effect| if let Effect::ContourPaint { arrowheads, .. } = e { arrowheads.clone() } else { String::new() };
    let opened = heads(&changed(&t, "fx_contour_014.json", |_| {}));
    t.row(
        "fx_contour_014.json, from before the heads, opens with Arrowheads off and saves without them",
        &format!("opens with {opened:?}; saved {old}"),
        opened == "off" && old.get("arrowheads").is_none(),
    );
    let why = changed(&t, "fx_contour_047.json", |_| {}).why_invalid();
    t.row("fx_contour_047.json is refused in a sentence naming \"yes\"", &why, why.contains("\"yes\"") && why.contains("arrowheads"));
    let full = r##"{"view": "edge_map", "stylization": 50, "cleanliness": 50, "smear_iterations": 3, "apply_to_alpha": "off", "draw_color_fills": "none", "reduction_method": "k_means", "posterization_level": 8, "low_strength": 0, "high_strength": 100, "offset_flow_angles": 0, "quantize_flow_angles": 0, "edge_map_layer": "", "arrowheads": 1}"##;
    t.shape_refused("fx_contour_042.json", "a Contour Paint whose arrowheads is a number, not a word", full);

    t.heading("Commands");
    let mut document = t.load("fx_contour_014.json").document;
    let bad = changed(&t, "fx_contour_014.json", |e| if let Effect::ContourPaint { arrowheads, .. } = e { *arrowheads = "yes".into() });
    let effect_of = |d: &anime_compositor::command::Document| format!("{:?}", d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0]);
    let held = effect_of(&document);
    let refused = document.apply(holder(set(bad))).err();
    let untouched = effect_of(&document) == held;
    t.row(
        "Arrowheads \"yes\" is refused with a sentence, and nothing changes",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some() && untouched,
    );
    let on = changed(&t, "fx_contour_014.json", |e| if let Effect::ContourPaint { arrowheads, .. } = e { *arrowheads = "on".into() });
    t.taken(&mut document, "fx_contour_014.json", vec![("Arrowheads on", holder(set(on)))]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_contour_042.json", 0), ("fx_contour_044.json", 0), ("fx_contour_045.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in [14u32, 18, 42, 43, 44, 45, 46, 47] {
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
        // The refused one (047) leaves nothing for the card.
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == (n == 47),
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
    let lines = cpu(&reference(|id| json!([fx(&format!("b463-{id}"), &json!({"view": "edge_map"}))])));
    for (what, p) in [
        ("Edge Map with Arrowheads on", json!({"view": "edge_map", "arrowheads": "on"})),
        ("Edge Map with Arrowheads on, Offset Flow Angles 90, Stylization 80", json!({"view": "edge_map", "arrowheads": "on", "offset_flow_angles": 90, "stylization": 80})),
    ] {
        let project = reference(|id| json!([fx(&format!("b463-{id}"), &p)]));
        let changed = distance(&cpu(&project), &lines).1;
        t.row(
            &format!("the reference shot, Contour Paint {what}: the processor's frame 100 differs from the lines without heads, so the comparisons below test the heads"),
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

    t.heading("Pictures: in `verification/D-585 pictures/`");
    let dir = repo("verification/D-585 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));

    let (plain, said) = street(json!({"view": "edge_map"}));
    write("1_edge_map_no_heads.png", &plain);
    t.row(
        "1_edge_map_no_heads.png, the street's Edge Map with Arrowheads off, as a file from before them: grey strength with short cyan lines; draws cleanly",
        &format!("{said:?}, {} cyan pixels", cyan(&plain)),
        said.is_empty() && cyan(&plain) > 0,
    );
    let (headed, said) = street(json!({"view": "edge_map", "arrowheads": "on"}));
    write("2_edge_map_arrowheads.png", &headed);
    t.row(
        "2_edge_map_arrowheads.png, the same with Arrowheads on: each line now ends in a small V-shaped head, so there is more cyan and the flow's direction can be read; draws cleanly",
        &format!("{said:?}, {} cyan pixels", cyan(&headed)),
        said.is_empty() && cyan(&headed) > cyan(&plain),
    );
    let (turned, said) = street(json!({"view": "edge_map", "arrowheads": "on", "offset_flow_angles": 180}));
    write("3_arrowheads_offset_180.png", &turned);
    t.row(
        "3_arrowheads_offset_180.png, Arrowheads on and Offset Flow Angles 180: the same lines, every head at the other end; draws cleanly",
        &format!("{said:?}, differs from 2: {}", turned != headed),
        said.is_empty() && turned != headed,
    );
    let (result, said) = street(json!({"arrowheads": "on"}));
    let (result_off, _) = street(json!({}));
    write("4_final_result_unchanged.png", &result);
    t.row(
        "4_final_result_unchanged.png, Arrowheads on with View Final Result: the heads belong to the Edge Map, so the painted street is the same as with them off; draws cleanly",
        &format!("{said:?}, the same as with them off: {}", result == result_off),
        said.is_empty() && result == result_off,
    );

    t.finish("D-585_contour_heads_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B463_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-463: a measurement, run deliberately with --release --ignored"]
fn b463_contour_heads_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B463_CPU").is_ok();
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
        ("Noise, then Contour Paint, View Edge Map, Arrowheads off", Some(json!({"view": "edge_map"}))),
        ("the same with Arrowheads on", Some(json!({"view": "edge_map", "arrowheads": "on"}))),
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
    let out = std::env::var("B463_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-463_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
