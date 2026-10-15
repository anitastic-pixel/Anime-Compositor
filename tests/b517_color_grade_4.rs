//! B-517: D-637, Color Grade's fourth unit, after Lumetri Color under our own name: the
//! Creative section's Sharpen, and the vignette's roundness below 0, its squarer shapes.
//!
//! Writes `verification/D-637_color_grade_4_table.md` and draws pictures into
//! `verification/D-637 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_4/expected_color_grade_4.json`, written by
//! `tools/color_grade_4_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE4-001 to 016. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, COLOR_GRADE_SETTINGS};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Color Grade with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
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
    json!({"instance_id": id, "type_id": "core.color_grade", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` leave nothing for the card; FX-GRADE4-006's sharpen is 0 at frame 2
/// only, so the card may draw that frame alone, or leave it.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=16 {
        let file = format!("fx_grade4_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ColorGrade { .. }));
            }
        }
        let want = if n == 6 { card <= 2 } else { (card == 0) == none.contains(&n) };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && want,
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `card` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b517-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Color Grade {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ColorGrade { .. }));
                t.row(
                    &format!("the reference shot, Color Grade {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
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
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// The mean luma of the 8-bit picture's pixel at (x, y), 0 to 255.
fn luma(p: &[u8], x: usize, y: usize) -> f64 {
    let q = &p[(y * TOWN.0 + x) * 4..][..3];
    0.2126 * q[0] as f64 + 0.7152 * q[1] as f64 + 0.0722 * q[2] as f64
}

/// How much darker the corner (8, 8) is than the left edge's middle (8, height / 2), as a
/// share of each one's luma before.
fn corner_vs_edge(before: &[u8], after: &[u8]) -> (f64, f64) {
    let (y_mid, y_top) = (TOWN.1 / 2, 8);
    (1.0 - luma(after, 8, y_top) / luma(before, 8, y_top), 1.0 - luma(after, 8, y_mid) / luma(before, 8, y_mid))
}

/// The sum over every pixel of how far it is from its right-hand neighbour in linear light (the
/// light the sharpen works in), in millionths: larger is harder.
fn edges(p: &[u8]) -> u64 {
    let lin = |v: u8| {
        let c = v as f64 / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    };
    let px: Vec<&[u8]> = p.chunks_exact(4).collect();
    px.windows(2).map(|w| (0..3).map(|c| ((lin(w[0][c]) - lin(w[1][c])).abs() * 1e6) as u64).sum::<u64>()).sum()
}

#[test]
fn b517_color_grade_4() {
    let mut t = Table::new(
        "color_grade_4",
        "# D-637: Color Grade, the fourth unit\n\nB-517, after Lumetri Color under our own name: \
         the Creative section's Sharpen (-100 to 100; below 0 it softens), after the faded film \
         and before the vibrance, and the vignette's roundness below 0, its squarer shapes. Every \
         expected pixel is `Fixtures/color_grade_4/expected_color_grade_4.json`, written by \
         `tools/color_grade_4_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE4-001 to 016.\n",
    );

    t.heading("FX-GRADE4-001 to 016 (document 25)");
    t.fixtures_numbered("expected_color_grade_4.json", 1..=16);

    t.heading("The file");
    let files: Vec<String> = (1..=14).map(|n| format!("fx_grade4_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_grade4_013.json");
    t.row(
        "fx_grade4_013.json is saved with its 68 settings: the look, 55 numbers, four RGB curves, five hue and saturation curves and three words",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 68 && saved["sharpen"] == 30.0 && saved["vignette_roundness"] == -60.0,
    );
    let old = t.saved_parameters("fx_grade4_014.json");
    t.row(
        "fx_grade4_014.json, a D-571 file without sharpen, is saved as before: 67 settings, sharpen not written while it is where it starts (D-121's rule)",
        &old.to_string(),
        old.as_object().unwrap().len() == 67 && old.get("sharpen").is_none(),
    );
    for file in ["fx_grade4_015.json", "fx_grade4_016.json"] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming sharpen"), &why, why.contains("sharpen"));
    }
    let at = |name: &str| COLOR_GRADE_SETTINGS.iter().position(|s| s.0 == name).unwrap();
    let below = changed(&t, "fx_grade4_001.json", |e| {
        if let Effect::ColorGrade { values, .. } = e {
            values[at("vignette_roundness")] = -101.0;
        }
    })
    .why_invalid();
    t.row("a roundness of -101 is refused in a sentence naming vignette roundness", &below, below.contains("vignette roundness"));

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_grade4_013.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let kept = matches!(&draft, Effect::ColorGrade { values, .. } if values[at("sharpen")] == 30.0 && values[at("vignette_roundness")] == -60.0);
    t.row(
        "a half-size draft preview leaves the sharpen and the roundness as they are (the sharpen's blur is 1 pixel of the picture drawn, D-637)",
        &format!("{kept}"),
        kept,
    );
    t.row("it grows the drawing's bounds by nothing", &draft.bounds_expansion().to_string(), draft.bounds_expansion() == 0);

    t.heading("Commands");
    let mut document = t.load("fx_grade4_001.json").document;
    let square = changed(&t, "fx_grade4_001.json", |e| {
        if let Effect::ColorGrade { values, .. } = e {
            values[at("vignette_roundness")] = -101.0;
        }
    });
    t.refused(
        &mut document,
        vec![
            ("vignette roundness -101", set(square)),
            ("sharpen keyed to 101", keys("sharpen", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    let soft = changed(&t, "fx_grade4_001.json", |e| {
        if let Effect::ColorGrade { values, .. } = e {
            values[at("sharpen")] = -50.0;
            values[at("vignette_amount")] = -2.0;
            values[at("vignette_roundness")] = -80.0;
        }
    });
    t.taken(
        &mut document,
        "fx_grade4_001.json",
        vec![
            ("sharpen -50 with a vignette at roundness -80", set(soft)),
            ("roundness keyed from -100 to 100", keys("vignette_roundness", &[(0, &[-100.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade4_002.json", 0),
        ("fx_grade4_004.json", 0),
        ("fx_grade4_006.json", 4),
        ("fx_grade4_008.json", 0),
        ("fx_grade4_009.json", 0),
        ("fx_grade4_010.json", 0),
        ("fx_grade4_013.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001), a sharpen (002 to 005, 007, 008, 013: the processor's, D-637) and refused
    // (015, 016) leave nothing for the card.
    let none: Vec<u32> = vec![1, 2, 3, 4, 5, 7, 8, 13, 15, 16];
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("a dark vignette at roundness -100, a rectangle", json!({"vignette_amount": -3, "vignette_roundness": -100, "vignette_feather": 30}), 3),
            ("a light vignette at roundness -40", json!({"vignette_amount": 2, "vignette_midpoint": 30, "vignette_roundness": -40}), 3),
            ("sharpen 60 (drawn by the processor)", json!({"sharpen": 60}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-637 pictures/`");
    let dir = repo("verification/D-637 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "Color Grade as added, sharpen written at 0, changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    let (hard, said) = picture(&dir, one(json!({"sharpen": 100})));
    write("2_sharpen_100.png", &hard);
    t.row(
        "2_sharpen_100.png, sharpen 100: harder edges, the neighbour-to-neighbour differences larger than the street's; draws cleanly",
        &format!("{said:?}, edges {} before, {} after", edges(&before), edges(&hard)),
        said.is_empty() && edges(&hard) > edges(&before),
    );
    let (softer, said) = picture(&dir, one(json!({"sharpen": -100})));
    write("3_sharpen_minus_100.png", &softer);
    t.row(
        "3_sharpen_minus_100.png, sharpen -100: softer, the neighbour-to-neighbour differences smaller; draws cleanly",
        &format!("{said:?}, edges {} before, {} after", edges(&before), edges(&softer)),
        said.is_empty() && edges(&softer) < edges(&before),
    );
    for (file, m, what) in [
        ("4_vignette_round_0.png", 0, "roundness 0, the frame's own oval: the corner darkened more than the edge's middle"),
        ("5_vignette_round_minus_50.png", -50, "roundness -50, squarer: the corner still darker than the edge's middle, by less"),
        ("6_vignette_round_minus_100.png", -100, "roundness -100, a rectangle: the corner and the edge's middle darkened alike"),
    ] {
        let (after, said) = picture(&dir, one(json!({"vignette_amount": -4, "vignette_midpoint": 40, "vignette_feather": 60, "vignette_roundness": m})));
        write(file, &after);
        let (corner, edge) = corner_vs_edge(&before, &after);
        let ok = match m {
            0 => corner > edge + 0.05,
            -50 => corner > edge,
            _ => (corner - edge).abs() < 0.05,
        };
        t.row(&format!("{file}, {what}; draws cleanly"), &format!("{said:?}, corner {:.0}% darker, edge middle {:.0}% darker", corner * 100.0, edge * 100.0), said.is_empty() && ok);
    }

    t.finish("D-637_color_grade_4_table.md");
}

/// The frame times, B-253's way, as B-450's: the reference shot with a Noise that changes every
/// frame on its first three layers, then the effect, every eighth frame asked for as the viewer
/// asks. With `B517_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-517: a measurement, run deliberately with --release --ignored"]
fn b517_color_grade_4_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B517_CPU").is_ok();
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
        ("Noise, then Color Grade, a dark vignette at roundness -60", Some(json!({"vignette_amount": -3, "vignette_roundness": -60}))),
        ("Noise, then Color Grade, sharpen 50 (drawn by the processor)", Some(json!({"sharpen": 50}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}c"), p));
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
    let out = std::env::var("B517_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-517_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
