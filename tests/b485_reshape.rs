//! B-485: D-606, After Effects' Reshape under `core.reshape`: the drawing inside one of the
//! layer's masks pulled into the shape of another, held at a third or at the drawing's edge,
//! drawn through D-601's backward field.
//!
//! Writes `verification/D-606_reshape_table.md` and draws pictures into `verification/D-606 pictures/`.
//!
//! Every expected pixel is `Fixtures/reshape/expected_reshape.json`, written by
//! `tools/reshape_reference.py` before this code existed and printed in document 25 as
//! FX-RESHAPE-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, repo, set, time_passes, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A rectangle from (x0, y0) to (x1, y1), corner points only.
fn rect([x0, y0, x1, y1]: [f64; 4]) -> J {
    let points: Vec<J> = [[x0, y0], [x1, y0], [x1, y1], [x0, y1]]
        .iter()
        .map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]}))
        .collect();
    json!({ "points": points })
}

/// Masks that cut nothing (mode None), for Reshape to choose from.
fn masks(rects: &[[f64; 4]]) -> J {
    J::Array(
        rects
            .iter()
            .enumerate()
            .map(|(m, &r)| json!({"name": format!("Mask {}", m + 1), "enabled": true, "inverted": false, "mode": "none", "opacity": 1.0,
                "feather_px": 0.0, "expansion_px": 0.0, "path": {"base": rect(r), "keyframes": []}}))
            .collect(),
    )
}

/// The reference shot (1920 by 1080) with three masks (mode None: a square, the same square
/// moved 200 right, and a large boundary about both) and `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        let d = 40.0 * i as f64;
        layers[i]["masks"] = masks(&[
            [760.0 + d, 340.0, 1160.0 + d, 740.0],
            [960.0 + d, 340.0, 1360.0 + d, 740.0],
            [400.0, 100.0, 1520.0, 980.0],
        ]);
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

/// A Reshape with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source_mask": 0, "destination_mask": 0, "boundary_mask": 0, "percent": 100, "elasticity": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.reshape", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=22 {
        let file = format!("fx_reshape_{n:03}.json");
        let project = persist::load(&t.root.join(&file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| {
                    matches!(e, Effect::Reshape { .. })
                });
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
        preview::preview_frame_cached(
            project,
            &ref_comp,
            100,
            &ref_root,
            PreviewQuality::Full,
            DEFAULT_TILE_SIZE,
            &mut log,
            &mut CelCache::viewer(),
        )
        .expect("the reference shot draws")
        .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in cases {
        let project = reference(|id| json!([fx(&format!("b485-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Reshape {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::Reshape { .. })
                });
                t.row(
                    &format!("the reference shot, Reshape {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of. The layer carries `rects` as masks that cut nothing.
fn picture(
    dir: &Path,
    asset: &str,
    size: (usize, usize),
    rects: &[[f64; 4]],
    effects: J,
    frame: i32,
) -> (Vec<u8>, Vec<String>) {
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
    layer["masks"] = masks(rects);
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        frame,
        dir,
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

/// The street's masks: 1 a square about the middle, 2 the same square 60 right, 3 a tall strip
/// through the middle, 4 a boundary well inside the picture.
const STREET: [[f64; 4]; 4] = [
    [190.0, 85.0, 290.0, 185.0],
    [250.0, 85.0, 350.0, 185.0],
    [215.0, 35.0, 265.0, 235.0],
    [120.0, 20.0, 420.0, 250.0],
];

#[test]
fn b485_reshape() {
    let mut t = Table::new(
        "reshape",
        "# D-606: Reshape\n\nB-485: `core.reshape` takes After Effects' Reshape controls (Source Mask, \
         Destination Mask, Boundary Mask, Percent, Elasticity), pulls the drawing inside the source \
         mask into the destination's shape, the rest following and the boundary held, and draws it \
         through D-601's backward field. Every expected pixel is `Fixtures/reshape/expected_reshape.json`, \
         written by `tools/reshape_reference.py` before this code existed and printed in document 25 \
         as FX-RESHAPE-001 to 022. Tolerance 2e-5.\n",
    );

    t.heading("FX-RESHAPE-001 to 022 (document 25)");
    t.fixtures_numbered("expected_reshape.json", 1..=22);

    t.heading("The file");
    let files: Vec<String> = (1..=22)
        .map(|n| format!("fx_reshape_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_reshape_001.json");
    t.row(
        "fx_reshape_001.json is saved with its five settings, the elasticity as a word, no outlines",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5
            && saved["source_mask"] == 0.0
            && saved["destination_mask"] == 0.0
            && saved["boundary_mask"] == 0.0
            && saved["percent"] == 100.0
            && saved["elasticity"] == "normal",
    );
    for (file, want) in [
        ("fx_reshape_018.json", "elasticity"),
        ("fx_reshape_019.json", "percent"),
        ("fx_reshape_020.json", "source mask"),
        ("fx_reshape_021.json", "boundary mask"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming Reshape and its {want}"),
            &why,
            why.contains("Reshape") && why.contains(want),
        );
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused(
        "fx_reshape_001.json",
        "a Reshape whose elasticity is a number",
        &full.replace("\"elasticity\":\"normal\"", "\"elasticity\":3"),
    );
    t.shape_refused(
        "fx_reshape_001.json",
        "a Reshape whose source mask is a word",
        &full.replace("\"source_mask\":0", "\"source_mask\":\"one\""),
    );
    t.shape_refused(
        "fx_reshape_001.json",
        "a Reshape without its percent",
        &full.replace(",\"percent\":100", ""),
    );

    t.heading("Commands");
    let mut document = t.load("fx_reshape_002.json").document;
    let rubber = changed(&t, "fx_reshape_002.json", |e| {
        if let Effect::Reshape { elasticity, .. } = e {
            *elasticity = "rubber".into()
        }
    });
    let over = changed(&t, "fx_reshape_002.json", |e| {
        if let Effect::Reshape { percent, .. } = e {
            *percent = 101.0
        }
    });
    t.refused(
        &mut document,
        vec![
            ("elasticity rubber", set(rubber)),
            ("percent 101", set(over)),
            (
                "the source mask keyed to end at 1001",
                keys("source_mask", &[(0, &[1.0]), (4, &[1001.0])]),
            ),
        ],
    );
    let liquid = changed(&t, "fx_reshape_002.json", |e| {
        if let Effect::Reshape { elasticity, .. } = e {
            *elasticity = "liquid".into()
        }
    });
    t.taken(
        &mut document,
        "fx_reshape_002.json",
        vec![
            ("elasticity liquid", set(liquid)),
            (
                "percent keyed from 0 to 100",
                keys("percent", &[(0, &[0.0]), (4, &[100.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_reshape_002.json", 0),
        ("fx_reshape_003.json", 0),
        ("fx_reshape_005.json", 0),
        ("fx_reshape_009.json", 0),
        ("fx_reshape_010.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 001 (no masks chosen) changes nothing, so nothing is left to the card; the missing masks
    // (015 to 017) and the refused (018 to 022) neither; 010 changes nothing at frame 0 only, so
    // it is on the card in 8 of 10.
    let none: Vec<u32> = [1].into_iter().chain(15..=22).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "the square pulled to the square moved right",
                json!({"source_mask": 1, "destination_mask": 2}),
                3,
            ),
            (
                "the same within a boundary, Liquid, 60 per cent",
                json!({"source_mask": 1, "destination_mask": 2, "boundary_mask": 3, "percent": 60, "elasticity": "liquid"}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-606 pictures/`");
    let dir = repo("verification/D-606 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| {
        png_out::write_rgba(
            &dir.join(name),
            size.0,
            size.1,
            OutputDepth::Eight,
            &[],
            bytes,
        )
        .unwrap()
    };
    write("town.png", TOWN, &town());
    let (w, h) = TOWN;
    let street = |p: J| {
        picture(
            &dir,
            "town.png",
            TOWN,
            &STREET,
            json!([fx("fx-0-0", &p)]),
            0,
        )
    };
    let (before, said) = picture(&dir, "town.png", TOWN, &STREET, json!([]), 0);
    write("1_before.png", TOWN, &before);
    let open = |p: &[u8]| (0..w * h).filter(|&i| p[i * 4 + 3] == 0).count();
    t.row(
        "1_before.png, the street with no effect (its four masks cut nothing), every pixel covered; draws cleanly",
        &format!("{said:?}, {} clear pixels", open(&before)),
        said.is_empty() && open(&before) == 0,
    );

    let (added, said) = street(json!({}));
    t.row(
        "As added, with no masks chosen: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&added, &before).1),
        said.is_empty() && added == before,
    );

    let (pulled, said) = street(json!({"source_mask": 1, "destination_mask": 2}));
    write("2_pulled_right.png", TOWN, &pulled);
    t.row(
        "2_pulled_right.png, source mask 1 (a square about the middle) to destination mask 2 (the same square 60 pixels right): the middle of the street dragged right, the edges held; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street", distance(&pulled, &before).1),
        said.is_empty() && distance(&pulled, &before).1 > w * h / 100,
    );

    let outside = |p: &[u8]| {
        (0..h).all(|y| {
            (0..120)
                .chain(421..w)
                .all(|x| p[(y * w + x) * 4..][..4] == before[(y * w + x) * 4..][..4])
        })
    };
    let (fenced, said) =
        street(json!({"source_mask": 1, "destination_mask": 2, "boundary_mask": 4}));
    write("3_within_boundary.png", TOWN, &fenced);
    t.row(
        "3_within_boundary.png, the same held at boundary mask 4 (120 to 420 across): nothing left or right of it moves, so it differs from 2_pulled_right.png; draws cleanly",
        &format!("{said:?}, outside the boundary untouched: {}, {} pixels differ from 2_pulled_right.png", outside(&fenced), distance(&fenced, &pulled).1),
        said.is_empty() && outside(&fenced) && distance(&fenced, &pulled).1 > 0,
    );

    let (tall, said) = street(json!({"source_mask": 1, "destination_mask": 3}));
    write("4_squeezed_tall.png", TOWN, &tall);
    t.row(
        "4_squeezed_tall.png, the square pulled into mask 3, a tall strip: the middle squeezed across and stretched up and down, so it differs from 2_pulled_right.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 2_pulled_right.png", distance(&tall, &pulled).1),
        said.is_empty() && distance(&tall, &pulled).1 > w * h / 100,
    );

    let (stiff, said) =
        street(json!({"source_mask": 1, "destination_mask": 2, "elasticity": "stiff"}));
    write("5_stiff.png", TOWN, &stiff);
    let (liquid, said_liquid) =
        street(json!({"source_mask": 1, "destination_mask": 2, "elasticity": "liquid"}));
    write("6_liquid.png", TOWN, &liquid);
    t.row(
        "5_stiff.png and 6_liquid.png, 2_pulled_right.png at Elasticity Stiff (the pull spread wide) and Liquid (kept near the square's edges): the two differ; both draw cleanly",
        &format!("{said:?} {said_liquid:?}, {} pixels differ", distance(&stiff, &liquid).1),
        said.is_empty() && said_liquid.is_empty() && distance(&stiff, &liquid).1 > w * h / 100,
    );

    let (missing, said) = street(json!({"source_mask": 9, "destination_mask": 2}));
    t.row(
        "Source mask 9 of four: the street as it is, and EFFECT_PATH_MISSING said",
        &format!("{said:?}, {} pixels differ", distance(&missing, &before).1),
        missing == before && said.len() == 1 && said[0].starts_with("EFFECT_PATH_MISSING"),
    );

    t.finish("D-606_reshape_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers (with their three masks), so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B485_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-485: a measurement, run deliberately with --release --ignored"]
fn b485_reshape_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B485_CPU").is_ok();
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
        (
            "Noise, then Reshape, square to the square moved right",
            Some(json!({"source_mask": 1, "destination_mask": 2})),
        ),
        (
            "Noise, then Reshape, the same within a boundary, Liquid, 60 per cent",
            Some(
                json!({"source_mask": 1, "destination_mask": 2, "boundary_mask": 3, "percent": 60, "elasticity": "liquid"}),
            ),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (
            Id::new("comp-reference-shot"),
            repo("Fixtures/reference_shot"),
        );
        let (first, times) = time_passes(
            &mut gpu,
            cpu,
            passes,
            &project,
            &comp,
            &root,
            PreviewQuality::Full,
        );
        let _ = writeln!(
            s,
            "| {name} | Full | {:.1} | {:.1} |",
            median(first),
            median(times)
        );
    }
    let out = std::env::var("B485_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-485_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
