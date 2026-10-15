//! B-380: D-500, after After Effects' Color Range (Keying): the colours inside a box chosen in
//! Lab, YUV or RGB keyed out, with a fuzziness that softens the box's edge. Adobe describes the
//! effect but publishes no formula and no defaults we found: the rule and the settings when added
//! are ours, stated in `tools/color_range_reference.py`.
//!
//! Writes `verification/D-500_color_range_table.md` and draws pictures into
//! `verification/D-500 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_range/expected_color_range.json`, written by the tool
//! before this code existed and printed in document 25 as FX-CRANGE-001 to 018. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, Table, MAIN,
    TOWN,
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

/// A Color Range with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"color_space": "lab", "fuzziness": 0, "min_1": 0, "max_1": 0, "min_2": 0, "max_2": 0, "min_3": 0, "max_3": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.color_range", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=18 {
        let file = format!("fx_crange_{n:03}.json");
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
                    matches!(e, Effect::ColorRange { .. })
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
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
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b380-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Color Range {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::ColorRange { .. })
                });
                t.row(
                    &format!("the reference shot, Color Range {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
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
    let frame = render_frame(
        loaded.document.project(),
        &Id::new(MAIN),
        0,
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

const SCREEN: [u8; 3] = [40, 180, 60];
const SHIRT: [u8; 3] = [40, 60, 160];

/// A green-screen shot at the street's size: a lit green screen, a face whose outer 14 pixels
/// take on more and more of the screen's green (the spill), and a blue shirt below. Color
/// Difference Key's (B-339).
fn portrait() -> Vec<u8> {
    let (w, h) = TOWN;
    let skin = [214.0, 172.0, 140.0];
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 - 240.0, y as f64 - 110.0);
            let r = (dx * dx / (70.0 * 70.0) + dy * dy / (90.0 * 90.0)).sqrt();
            let c = if y >= 200 && (130..350).contains(&x) {
                SHIRT
            } else if r < 1.0 {
                // The spill, from none 14 pixels in to 60% of the screen at the edge.
                let spill = (1.0 - (1.0 - r) * 70.0 / 14.0).clamp(0.0, 1.0) * 0.6;
                std::array::from_fn(|c| {
                    (skin[c] + (SCREEN[c] as f64 - skin[c]) * spill).round() as u8
                })
            } else {
                SCREEN
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], x: usize, y: usize) -> &[u8] {
    let i = (y * TOWN.0 + x) * 4;
    &bytes[i..i + 4]
}

#[test]
fn b380_color_range() {
    let mut t = Table::new(
        "color_range",
        "# D-500: Color Range\n\nB-380, after After Effects' Color Range (Keying): the colours \
         inside a box chosen in Lab, YUV or RGB keyed out, with a fuzziness that softens the box's \
         edge. Adobe describes the effect and names its controls but publishes no formula and no \
         defaults we found, so the rule and the settings when added are ours, stated in \
         `tools/color_range_reference.py`: each pixel's three components on a 0 to 255 scale, how \
         far the farthest lies outside the box, over the fuzziness, is the share of the pixel kept. \
         Every expected pixel is `Fixtures/color_range/expected_color_range.json`, written by the \
         tool before this code existed and printed in document 25 as FX-CRANGE-001 to 018. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-CRANGE-001 to 018 (document 25)");
    t.fixtures_numbered("expected_color_range.json", 1..=18);

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_crange_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_crange_005.json");
    t.row(
        "fx_crange_005.json is saved with its eight settings, colour space rgb, fuzziness 40 and the top of green 255",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8 && saved["color_space"] == "rgb" && saved["fuzziness"] == 40.0 && saved["max_2"] == 255.0,
    );
    for (file, want) in [
        ("fx_crange_013.json", "fuzziness"),
        ("fx_crange_014.json", "fuzziness"),
        ("fx_crange_015.json", "min 1"),
        ("fx_crange_016.json", "max 3"),
        ("fx_crange_017.json", "colour space"),
        ("fx_crange_018.json", "colour space"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    let numbers = r##""min_1": 0, "max_1": 0, "min_2": 0, "max_2": 0, "min_3": 0"##;
    t.shape_refused(
        "fx_crange_001.json",
        "a Color Range whose fuzziness is a word",
        &format!(r##"{{"color_space": "lab", "fuzziness": "soft", {numbers}, "max_3": 0}}"##),
    );
    t.shape_refused(
        "fx_crange_001.json",
        "a Color Range without its colour space",
        &format!(r##"{{"fuzziness": 0, {numbers}, "max_3": 0}}"##),
    );
    t.shape_refused(
        "fx_crange_001.json",
        "a Color Range without its max 3",
        &format!(r##"{{"color_space": "lab", "fuzziness": 0, {numbers}}}"##),
    );

    t.heading("Commands");
    let mut document = t.load("fx_crange_001.json").document;
    let over = changed(&t, "fx_crange_001.json", |e| {
        if let Effect::ColorRange { fuzziness, .. } = e {
            *fuzziness = 256.0
        }
    });
    let word = changed(&t, "fx_crange_001.json", |e| {
        if let Effect::ColorRange { color_space, .. } = e {
            *color_space = "hsl".into()
        }
    });
    t.refused(
        &mut document,
        vec![
            ("fuzziness 256", set(over)),
            ("colour space \"hsl\"", set(word)),
            (
                "max 3 keyed to 300",
                keys("max_3", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    let yuv = changed(&t, "fx_crange_001.json", |e| {
        if let Effect::ColorRange {
            color_space, range, ..
        } = e
        {
            *color_space = "yuv".into();
            *range = [20.0, 120.0, 170.0, 230.0, 80.0, 125.0];
        }
    });
    t.taken(
        &mut document,
        "fx_crange_001.json",
        vec![
            (
                "colour space yuv with a box about the blue screen",
                set(yuv),
            ),
            (
                "fuzziness keyed from 0 to 80",
                keys("fuzziness", &[(0, &[0.0]), (4, &[80.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_crange_002.json", 0),
        ("fx_crange_003.json", 0),
        ("fx_crange_004.json", 0),
        ("fx_crange_007.json", 0),
        ("fx_crange_010.json", 2),
        ("fx_crange_012.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The refused ones (013 to 018) leave nothing for the card.
    let none: Vec<u32> = (13..=18).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "RGB, the whole cube but the darkest reds, fuzziness 60",
                json!({"color_space": "rgb", "fuzziness": 60, "min_1": 40, "max_1": 255, "max_2": 255, "max_3": 255}),
                3,
            ),
            (
                "Lab, the lighter half, a* and b* within 40 of grey, fuzziness 30",
                json!({"fuzziness": 30, "min_1": 128, "max_1": 255, "min_2": 88, "max_2": 168, "min_3": 88, "max_3": 168}),
                3,
            ),
            (
                "YUV, the darker half, fuzziness 0",
                json!({"color_space": "yuv", "max_1": 128, "max_2": 255, "max_3": 255}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-500 pictures/`");
    let dir = repo("verification/D-500 pictures");
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
    write("portrait.png", TOWN, &portrait());
    let shot = |p: J| picture(&dir, "portrait.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "portrait.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = shot(json!({}));
    t.row(
        "as added (Lab, every end 0) nothing is keyed: the shot byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    // The places to look: the screen, the face's middle (no spill), a spilled pixel near the
    // face's top edge, (156, 175, 113), the shirt.
    let (screen, face, edge, shirt) = ((20, 20), (240, 110), (240, 28), (240, 250));
    let cover = |b: &[u8], p: (usize, usize)| at(b, p.0, p.1)[3];
    let colour_kept = |after: &[u8]| {
        after
            .chunks_exact(4)
            .zip(before.chunks_exact(4))
            .filter(|(a, _)| a[3] >= 16)
            .all(|(a, b)| (0..3).all(|c| a[c].abs_diff(b[c]) <= 1))
    };

    // A Lab box about the screen's green (it is L 164, a 68, b 177 here; the spilled pixel is
    // L 175, a 111, b 158, skin a 139): L 120 to 210, a 40 to 95, b 150 to 210.
    let lab =
        json!({"min_1": 120, "max_1": 210, "min_2": 40, "max_2": 95, "min_3": 150, "max_3": 210});
    let (after, said) = shot(lab.clone());
    write("2_lab_hard.png", TOWN, &after);
    let (s, f, e, sh) = (
        cover(&after, screen),
        cover(&after, face),
        cover(&after, edge),
        cover(&after, shirt),
    );
    t.row(
        "2_lab_hard.png, a Lab box about the screen's green, fuzziness 0: the screen see-through, the face, the spilled pixel (a 111, outside the box) and the shirt fully covered; colour kept wherever anything is left; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, spilled {e}, shirt {sh}, colour kept {}", colour_kept(&after)),
        said.is_empty() && s == 0 && f == 255 && e == 255 && sh == 255 && colour_kept(&after),
    );

    // By hand, from the tool: with fuzziness 40 the spilled pixel is 15.85 past a's top end, 255
    // x 15.85 / 40 = 101.
    let mut soft = lab.clone();
    soft["fuzziness"] = J::from(40);
    let (after, said) = shot(soft);
    write("3_lab_soft.png", TOWN, &after);
    let (s, f, e, sh) = (
        cover(&after, screen),
        cover(&after, face),
        cover(&after, edge),
        cover(&after, shirt),
    );
    t.row(
        "3_lab_soft.png, the same box with fuzziness 40: the screen still see-through, the face and the shirt still whole, the spilled green edge now fading (covering 101 by hand at the spilled pixel); draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, spilled {e}, shirt {sh}"),
        said.is_empty() && s == 0 && f == 255 && sh == 255 && e.abs_diff(101) <= 2,
    );

    // A YUV box about the shirt (Y 65, U 181, V 110 here): the shirt keyed, nothing else.
    let (after, said) = shot(
        json!({"color_space": "yuv", "fuzziness": 20, "min_1": 40, "max_1": 110, "min_2": 160, "max_2": 220, "min_3": 90, "max_3": 140}),
    );
    write("4_yuv_shirt.png", TOWN, &after);
    let (s, f, e, sh) = (
        cover(&after, screen),
        cover(&after, face),
        cover(&after, edge),
        cover(&after, shirt),
    );
    t.row(
        "4_yuv_shirt.png, a YUV box about the blue shirt, fuzziness 20: the shirt see-through, the screen, the face and its edge fully covered; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, spilled {e}, shirt {sh}"),
        said.is_empty() && sh == 0 && s == 255 && f == 255 && e == 255,
    );

    // By hand on single colours, an RGB box about green (R 0 to 90, G 150 to 255, B 0 to 110),
    // fuzziness 30: pure green all gone, (0, 200, 120) 10 past blue's top, 255 x 10 / 30 = 85,
    // pure blue all kept.
    let rgb = json!({"color_space": "rgb", "fuzziness": 30, "min_1": 0, "max_1": 90, "min_2": 150, "max_2": 255, "min_3": 0, "max_3": 110});
    for (file, name, c, want) in [
        ("green.png", "pure green", [0u8, 255, 0], 0u8),
        (
            "sea_green.png",
            "the sea green (0, 200, 120)",
            [0, 200, 120],
            85,
        ),
        ("blue.png", "pure blue", [0, 0, 255], 255),
    ] {
        write(file, (4, 4), &[c[0], c[1], c[2], 255].repeat(16));
        let (after, said) = picture(&dir, file, (4, 4), json!([fx("fx-0-0", &rgb)]));
        t.row(
            &format!("by hand: {name}, in an RGB box about green with fuzziness 30, keeps covering {want} of 255; draws cleanly"),
            &format!("{said:?}, covering {}", after[3]),
            said.is_empty() && after[3].abs_diff(want) <= 2,
        );
    }

    t.finish("D-500_color_range_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B380_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-380: a measurement, run deliberately with --release --ignored"]
fn b380_color_range_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B380_CPU").is_ok();
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
            "Noise, then Color Range, RGB, fuzziness 60",
            Some(
                json!({"color_space": "rgb", "fuzziness": 60, "min_1": 40, "max_1": 255, "max_2": 255, "max_3": 255}),
            ),
        ),
        (
            "Noise, then Color Range, Lab, fuzziness 30",
            Some(
                json!({"fuzziness": 30, "min_1": 128, "max_1": 255, "min_2": 88, "max_2": 168, "min_3": 88, "max_3": 168}),
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
    let out = std::env::var("B380_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-380_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
