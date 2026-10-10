//! B-322: D-442 Scribble, after After Effects' Scribble ("Generate" in
//! `docs/effects/EFFECTS.md`): lines zigzagging over a closed mask.
//!
//! Writes `verification/D-442_scribble_table.md`.
//!
//! Every expected pixel is `Fixtures/scribble/expected_scribble.json`, written by
//! `tools/scribble_reference.py` before this code existed and printed in document 25 as
//! FX-SCRIBBLE-001 to 071. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-442 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// A Scribble: [mask, edge width, opacity, angle, stroke width, curviness, curviness variation,
/// spacing, spacing variation, path overlap, path overlap variation, start, end, wiggles per
/// second, random seed], [scribble, fill type, colour, fill paths sequentially, wiggle type,
/// composite].
fn scribble(n: [f64; 15], w: [&str; 6]) -> Effect {
    let [mask, edge_width, opacity, angle, stroke_width, curviness, curviness_variation, spacing, spacing_variation, path_overlap, path_overlap_variation, start, end, wiggles_per_second, random_seed] = n;
    let [scribble, fill_type, color, fill_paths_sequentially, wiggle_type, composite] = w.map(str::to_string);
    Effect::Scribble {
        scribble,
        mask,
        fill_type,
        edge_width,
        color,
        opacity,
        angle,
        stroke_width,
        curviness,
        curviness_variation,
        spacing,
        spacing_variation,
        path_overlap,
        path_overlap_variation,
        start,
        end,
        fill_paths_sequentially,
        wiggle_type,
        wiggles_per_second,
        random_seed,
        composite,
        masks: None,
        time: 0.0,
    }
}

const ADDED: [f64; 15] = [1.0, 10.0, 100.0, 45.0, 5.0, 5.0, 1.0, 5.0, 0.5, 0.0, 2.0, 0.0, 100.0, 5.0, 1.0];
const WORDS: [&str; 6] = ["single_mask", "inside", "#ffffff", "on", "smooth", "on_original"];

fn numbers(change: &[(usize, f64)]) -> [f64; 15] {
    let mut n = ADDED;
    for &(i, v) in change {
        n[i] = v;
    }
    n
}

fn words<'a>(change: &[(usize, &'a str)]) -> [&'a str; 6] {
    let mut w = WORDS;
    for &(i, v) in change {
        w[i] = v;
    }
    w
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
        .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// A closed mask of mode None through `points`, straight sides.
fn mask(points: &[[f64; 2]]) -> J {
    let points: Vec<J> = points.iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect();
    json!({"name": "Mask 1", "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0, "feather_px": 0.0, "expansion_px": 0.0,
        "path": {"base": {"points": points}, "keyframes": []}})
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers, each with a
/// six-sided mask of mode None for the scribble to fill.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
        layers[i]["masks"] = json!([mask(&[[400.0, 150.0], [1500.0, 220.0], [1700.0, 560.0], [1450.0, 950.0], [500.0, 880.0], [250.0, 520.0]])]);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.scribble" => json!({"scribble": "single_mask", "mask": 1, "fill_type": "inside", "edge_width": 10, "color": "#ffffff", "opacity": 100,
            "angle": 45, "stroke_width": 5, "curviness": 5, "curviness_variation": 1, "spacing": 5, "spacing_variation": 0.5, "path_overlap": 0,
            "path_overlap_variation": 2, "start": 0, "end": 100, "fill_paths_sequentially": "on", "wiggle_type": "smooth", "wiggles_per_second": 5,
            "random_seed": 1, "composite": "on_original"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
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
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
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
        let project = reference(|id| json!([fx(type_id, &format!("b322-{id}"), p)]));
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
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at `frame` of a two-second shot, the layer holding a star-like mask of
/// mode None, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["masks"] = json!([mask(&[[240.0, 20.0], [290.0, 100.0], [400.0, 110.0], [320.0, 170.0], [350.0, 250.0], [240.0, 200.0], [130.0, 250.0], [160.0, 170.0], [80.0, 110.0], [190.0, 100.0]])]);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

type Shot<'a> = (&'a str, J, i32, &'a str, fn(&[u8], &[u8]) -> bool);

/// The street drawn plain and with each of `shots` of `type_id` at its frame, written as numbered
/// pictures into `verification/{d} pictures/`; a row each that it draws cleanly and changes what
/// it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[Shot]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect (its star mask of mode None changes nothing); draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, frame {frame}, {what}; draws cleanly"),
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

// --- Scribble -------------------------------------------------------------------------------

fn is_scribble(e: &Effect) -> bool {
    matches!(e, Effect::Scribble { .. })
}

/// A fill: more than two thousand pixels changed, under half the street.
fn a_fill(a: &[u8], b: &[u8]) -> bool {
    let changed = distance(a, b).1;
    changed > 2000 && changed < a.len() / 4 / 2
}

/// The scribble alone: most of the street clear, more than two thousand pixels showing.
fn alone(a: &[u8], _: &[u8]) -> bool {
    let clear = a.chunks_exact(4).filter(|p| p[3] == 0).count();
    let shown = a.len() / 4 - clear;
    clear > a.len() / 4 / 2 && shown > 2000
}

#[test]
fn b322_scribble() {
    let mut t = Table::new(
        "scribble",
        "# D-442: Scribble\n\nB-322, after After Effects' Scribble: the chosen closed masks (one, all, or \
         all combined by their modes), or a band along their outline, crossed by lines Spacing apart at \
         Angle, each run Path Overlap past the edge, joined at alternate ends by turns as round as \
         Curviness says, every variation drawn from Random Seed and changed Wiggles/Second times a \
         second (Static never, Jumpy at once, Smooth gliding); trimmed from Start to End along the \
         scribble's length (all masks as one length when Fill Paths Sequentially is on) and drawn with \
         Path Stroke's round brush Stroke Width across, on the layer, on transparent or revealing the \
         layer. With no usable mask it draws nothing and says EFFECT_PATH_MISSING every frame. Every \
         expected pixel is `Fixtures/scribble/expected_scribble.json`, written by \
         `tools/scribble_reference.py` before this code existed and printed in document 25 as \
         FX-SCRIBBLE-001 to 071. Tolerance 2e-5.\n",
    );

    t.heading("FX-SCRIBBLE-001 to 071 (document 25)");
    t.fixtures_numbered("expected_scribble.json", 1..=71);

    t.heading("The file");
    // FX-SCRIBBLE-048's colour is written in capitals and saved in small letters, as every colour is.
    let all: Vec<String> = files("fx_scribble", 71).into_iter().filter(|f| f != "fx_scribble_048.json").collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let caps = t.saved_parameters("fx_scribble_048.json");
    t.row("fx_scribble_048.json is saved with its colour in small letters", &caps["color"].to_string(), caps["color"] == "#ff3020");
    let saved = t.saved_parameters("fx_scribble_017.json");
    t.row(
        "fx_scribble_017.json is saved with its words and numbers as written, End's keys kept, and no masks or time",
        &saved.to_string(),
        saved["end"]["keyframes"][1]["value"] == json!(100) && saved["scribble"] == "single_mask" && saved["mask"] == 1
            && saved["fill_type"] == "inside" && saved["color"] == "#ff3020" && saved["spacing"] == 1.5 && saved["wiggle_type"] == "static"
            && saved["fill_paths_sequentially"] == "on" && saved.get("masks").is_none() && saved.get("time").is_none(),
    );
    why(
        &mut t,
        &[
            ("fx_scribble_054.json", "Scribble's mask runs from 1 to 1000, and this is 0."),
            ("fx_scribble_055.json", "Scribble's edge width runs from 0 to 1000, and this is -1."),
            ("fx_scribble_056.json", "Scribble's opacity runs from 0 to 100, and this is 101."),
            ("fx_scribble_057.json", "Scribble's angle runs from -3600 to 3600, and this is 3601."),
            ("fx_scribble_058.json", "Scribble's stroke width runs from 0 to 1000, and this is 1001."),
            ("fx_scribble_059.json", "Scribble's curviness runs from 0 to 100, and this is 101."),
            ("fx_scribble_060.json", "Scribble's spacing runs from 1 to 1000, and this is 0.5."),
            ("fx_scribble_061.json", "Scribble's path overlap runs from -1000 to 1000, and this is -1001."),
            ("fx_scribble_062.json", "Scribble's end runs from 0 to 100, and this is 101."),
            ("fx_scribble_063.json", "Scribble's wiggles per second runs from 0 to 100, and this is -1."),
            ("fx_scribble_064.json", "Scribble's random seed runs from 0 to 100000, and this is 100001."),
            ("fx_scribble_065.json", "Scribble's scribble is one of single_mask, all_masks, all_masks_using_modes, and this is \"some_masks\"."),
            ("fx_scribble_066.json", "Scribble's fill type is one of inside, centered_edge, inside_edge, outside_edge, left_edge, right_edge, and this is \"outline\"."),
            ("fx_scribble_067.json", "Scribble's fill paths sequentially is \"off\" or \"on\", and this is \"yes\"."),
            ("fx_scribble_068.json", "Scribble's wiggle type is one of static, jumpy, smooth, and this is \"wobbly\"."),
            ("fx_scribble_069.json", "Scribble's composite is one of on_original, on_transparent, reveal, and this is \"glow\"."),
            ("fx_scribble_070.json", "Scribble's colour is written #rrggbb, and this is \"#12345\"."),
        ],
    );
    let p = r##""scribble": "single_mask", "mask": 1, "fill_type": "inside", "edge_width": 10, "color": "#ffffff", "opacity": 100, "angle": 45, "stroke_width": 5, "curviness": 5, "curviness_variation": 1, "spacing": 5, "spacing_variation": 0.5, "path_overlap": 0, "path_overlap_variation": 2, "start": 0, "end": 100, "wiggle_type": "smooth", "wiggles_per_second": 5, "random_seed": 1"##;
    t.shape_refused("fx_scribble_001.json", "a Scribble with no `composite`", &format!("{{{p}, \"fill_paths_sequentially\": \"on\"}}"));
    t.shape_refused("fx_scribble_001.json", "a Scribble whose composite is true", &format!("{{{p}, \"fill_paths_sequentially\": \"on\", \"composite\": true}}"));
    t.shape_refused("fx_scribble_001.json", "a Scribble whose fill paths sequentially is true", &format!("{{{p}, \"fill_paths_sequentially\": true, \"composite\": \"on_original\"}}"));

    t.heading("Commands");
    let mut document = t.load("fx_scribble_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("mask 0", set(scribble(numbers(&[(0, 0.0)]), WORDS))),
            ("spacing 0.5", set(scribble(numbers(&[(7, 0.5)]), WORDS))),
            ("end 101", set(scribble(numbers(&[(12, 101.0)]), WORDS))),
            ("scribble \"some_masks\"", set(scribble(ADDED, words(&[(0, "some_masks")])))),
            ("fill type \"outline\"", set(scribble(ADDED, words(&[(1, "outline")])))),
            ("colour \"red\"", set(scribble(ADDED, words(&[(2, "red")])))),
            ("wiggle type \"wobbly\"", set(scribble(ADDED, words(&[(4, "wobbly")])))),
            ("stroke width keyed to 2000", keys("stroke_width", &[(0, &[5.0]), (4, &[2000.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_scribble_001.json",
        vec![
            (
                "All Masks Using Modes, mask 2, Outside Edge 6, blue, opacity 70, angle -30, stroke 3, curviness 40 and 10, spacing 7 and 2, overlap -1 and 3, 10 to 90, off, Jumpy 12, seed 7, reveal",
                set(scribble(
                    [2.0, 6.0, 70.0, -30.0, 3.0, 40.0, 10.0, 7.0, 2.0, -1.0, 3.0, 10.0, 90.0, 12.0, 7.0],
                    ["all_masks_using_modes", "outside_edge", "#3080ff", "off", "jumpy", "reveal"],
                )),
            ),
            ("end keyed from 0 to 100", keys("end", &[(0, &[0.0]), (4, &[100.0])])),
            ("angle keyed from 0 to 90", keys("angle", &[(0, &[0.0]), (4, &[90.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_scribble_001.json", 4),
        ("fx_scribble_006.json", 0),
        ("fx_scribble_025.json", 0),
        ("fx_scribble_029.json", 0),
        ("fx_scribble_039.json", 2),
        ("fx_scribble_045.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-SCRIBBLE-049 to 053 find no mask and 054 to 071 are refused, so the card is not asked.
    let none: Vec<u32> = (49..=71).collect();
    card_fixtures(&mut t, &mut gpu, "fx_scribble", 71, &none, is_scribble);
    card_reference(
        &mut t,
        &mut gpu,
        "Scribble",
        "core.scribble",
        &[
            ("as added (white lines 5 across over the mask)", json!({})),
            (
                "stroke 12, spacing 30, curviness 60 and 20, spacing variation 8, overlap 10 and 20, Jumpy, red, On Transparent",
                json!({"stroke_width": 12, "spacing": 30, "curviness": 60, "curviness_variation": 20, "spacing_variation": 8, "path_overlap": 10,
                    "path_overlap_variation": 20, "wiggle_type": "jumpy", "color": "#ff3020", "composite": "on_transparent"}),
            ),
            (
                "Outside Edge 80, Reveal, stroke 8, spacing 16, End keyed 0 to 100",
                json!({"fill_type": "outside_edge", "edge_width": 80, "composite": "reveal", "stroke_width": 8, "spacing": 16,
                    "end": keyed(&[(0, json!(0)), (239, json!(100))])}),
            ),
            (
                "angle keyed -30 to 120, Smooth 2 a second, opacity 60, spacing 20, stroke 6",
                json!({"angle": keyed(&[(0, json!(-30)), (239, json!(120))]), "wiggles_per_second": 2, "opacity": 60, "spacing": 20, "stroke_width": 6}),
            ),
        ],
        is_scribble,
    );

    street(
        &mut t,
        "D-442",
        "core.scribble",
        &[
            ("as_added", json!({}), 0, "as added: white lines 5 across at 45 degrees over the star", a_fill),
            (
                "red_loose",
                json!({"color": "#ff3020", "stroke_width": 3, "spacing": 12, "curviness": 60, "curviness_variation": 30, "spacing_variation": 4, "path_overlap_variation": 10}),
                0,
                "red, stroke 3, spacing 12, curviness 60 and 30, varied: a loose hand-drawn hatch over the star",
                a_fill,
            ),
            (
                "outline_transparent",
                json!({"fill_type": "centered_edge", "edge_width": 24, "spacing": 4, "stroke_width": 2, "angle": 0, "color": "#ffd040", "composite": "on_transparent"}),
                0,
                "Centered Edge 24, yellow, On Transparent: a hatched band along the star's outline, the street gone",
                alone,
            ),
            (
                "reveal_half",
                json!({"stroke_width": 8, "spacing": 10, "end": keyed(&[(0, json!(0)), (47, json!(100))]), "composite": "reveal"}),
                24,
                "Reveal, End keyed 0 to 100 over two seconds, halfway: half the star's scribble, the street seen only through it",
                alone,
            ),
        ],
    );

    t.finish("D-442_scribble_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B322_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-322: a measurement, run deliberately with --release --ignored"]
fn b322_scribble_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B322_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Scribble as added (spacing 5, about 200 lines)", Some(json!({}))),
        ("Noise, then Scribble stroke 12, spacing 30, curviness 60, Jumpy", Some(json!({"stroke_width": 12, "spacing": 30, "curviness": 60, "wiggle_type": "jumpy"}))),
        ("Noise, then Scribble Centered Edge 80, spacing 2, stroke 2", Some(json!({"fill_type": "centered_edge", "edge_width": 80, "spacing": 2, "stroke_width": 2}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.scribble", &format!("{id}c"), p));
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
    let out = std::env::var("B322_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-322_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
