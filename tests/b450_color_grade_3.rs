//! B-450: D-571, Color Grade's third unit, after Lumetri Color under our own name: the hue
//! versus hue, hue versus luma, luma versus saturation and saturation versus saturation curves,
//! and HSL Secondary's key denoise and blur and its three wheels.
//!
//! Writes `verification/D-571_color_grade_3_table.md` and draws pictures into
//! `verification/D-571 pictures/`.
//!
//! Every expected pixel is `Fixtures/color_grade_3/expected_color_grade_3.json`, written by
//! `tools/color_grade_3_reference.py` before this code existed and printed in document 25 as
//! FX-GRADE3-001 to 034. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
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
/// files numbered in `none` leave nothing for the card; FX-GRADE3-019's key blur is 0 at frame 0
/// only, so the card draws that frame alone.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=34 {
        let file = format!("fx_grade3_{n:03}.json");
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
        let want = if n == 19 { card == 2 } else { (card == 0) == none.contains(&n) };
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && want,
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)]) {
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
        let project = reference(|id| json!([fx(&format!("b450-{id}"), p)]));
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
                    d.0 <= 1 && !r && a == b && card == 3,
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

/// The covered pixels that are grey, their three channels equal.
fn greys(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0 && q[0] == q[1] && q[1] == q[2]).count()
}

/// The covered pixels.
fn covered(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0).count()
}

/// The covered pixels of a mask that are neither black nor white: the key's soft edge.
fn soft(p: &[u8]) -> usize {
    p.chunks_exact(4).filter(|q| q[3] > 0 && q[0] > 0 && q[0] < 255).count()
}

#[test]
fn b450_color_grade_3() {
    let mut t = Table::new(
        "color_grade_3",
        "# D-571: Color Grade, the third unit\n\nB-450, after Lumetri Color under our own name: \
         the hue versus hue, hue versus luma, luma versus saturation and saturation versus \
         saturation curves after hue versus saturation, and HSL Secondary's key denoise and blur \
         and its three wheels. Every expected pixel is \
         `Fixtures/color_grade_3/expected_color_grade_3.json`, written by \
         `tools/color_grade_3_reference.py` before this code existed and printed in document 25 \
         as FX-GRADE3-001 to 034.\n",
    );

    t.heading("FX-GRADE3-001 to 034 (document 25)");
    t.fixtures_numbered("expected_color_grade_3.json", 1..=34);

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_grade3_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_grade3_021.json");
    t.row(
        "fx_grade3_021.json is saved with its 67 settings: the look, 54 numbers, four RGB curves, five hue and saturation curves and three words",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 67
            && effect_table::same_json(&saved["hue_hue"], &json!([[30.0, -10.0], [200.0, 20.0]]))
            && saved["secondary_wheels"] == "three"
            && saved["key_denoise"] == 12.0,
    );
    let old = t.saved_parameters("fx_grade3_022.json");
    t.row(
        "fx_grade3_022.json, a D-398 file without the new settings, is saved as before: 54 settings, none of the new ones written while each is where it starts (D-121's rule)",
        &old.to_string(),
        old.as_object().unwrap().len() == 54
            && ["hue_hue", "hue_luma", "luma_saturation", "saturation_saturation", "secondary_wheels", "secondary_shadow_wheel_hue", "secondary_highlight_lightness", "key_denoise", "key_blur"]
                .iter()
                .all(|k| old.get(*k).is_none()),
    );
    for (file, want) in [
        ("fx_grade3_023.json", "hue versus hue"),
        ("fx_grade3_024.json", "hue versus hue"),
        ("fx_grade3_025.json", "hue versus luma"),
        ("fx_grade3_026.json", "hue versus luma"),
        ("fx_grade3_027.json", "luma versus saturation"),
        ("fx_grade3_028.json", "luma versus saturation"),
        ("fx_grade3_029.json", "saturation versus saturation"),
        ("fx_grade3_030.json", "saturation versus saturation"),
        ("fx_grade3_031.json", "secondary wheels"),
        ("fx_grade3_032.json", "key denoise"),
        ("fx_grade3_033.json", "key blur"),
        ("fx_grade3_034.json", "secondary highlight lightness"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let mut word = fx("x", &json!({}))["parameters"].clone();
    word["secondary_wheels"] = J::from(3);
    t.shape_refused("fx_grade3_001.json", "a Color Grade whose secondary wheels is a number", &word.to_string());
    let mut flat = fx("x", &json!({}))["parameters"].clone();
    flat["hue_luma"] = json!([0, 50]);
    t.shape_refused("fx_grade3_001.json", "a Color Grade whose hue versus luma is numbers, not points", &flat.to_string());

    t.heading("How far it reaches");
    let mut draft = changed(&t, "fx_grade3_021.json", |_| {});
    draft.scale_distances(|d| d * 0.5);
    let at = |name: &str| COLOR_GRADE_SETTINGS.iter().position(|s| s.0 == name).unwrap();
    let halved = matches!(&draft, Effect::ColorGrade { values, .. }
        if values[at("key_denoise")] == 6.0 && values[at("key_blur")] == 4.0 && values[at("secondary_shadow_wheel_amount")] == 20.0);
    t.row(
        "a half-size draft preview halves the key denoise and blur, distances, and leaves the other settings",
        &format!("key denoise {}, key blur {}", if let Effect::ColorGrade { values, .. } = &draft { values[at("key_denoise")] } else { -1.0 }, if let Effect::ColorGrade { values, .. } = &draft { values[at("key_blur")] } else { -1.0 }),
        halved,
    );
    t.row("it grows the drawing's bounds by nothing", &draft.bounds_expansion().to_string(), draft.bounds_expansion() == 0);

    t.heading("Commands");
    let mut document = t.load("fx_grade3_001.json").document;
    let shift = changed(&t, "fx_grade3_001.json", |e| {
        if let Effect::ColorGrade { more_curves, .. } = e {
            more_curves[0] = vec![vec![0.0, 181.0]];
        }
    });
    let two = changed(&t, "fx_grade3_001.json", |e| {
        if let Effect::ColorGrade { secondary_wheels, .. } = e {
            *secondary_wheels = "two".into();
        }
    });
    let falling = changed(&t, "fx_grade3_001.json", |e| {
        if let Effect::ColorGrade { more_curves, .. } = e {
            more_curves[2] = vec![vec![80.0, 50.0], vec![20.0, 150.0]];
        }
    });
    t.refused(
        &mut document,
        vec![
            ("hue versus hue with a shift of 181", set(shift)),
            ("secondary wheels \"two\"", set(two)),
            ("luma versus saturation with its lumas falling", set(falling)),
            ("key blur keyed to 101", keys("key_blur", &[(0, &[0.0]), (4, &[101.0])])),
        ],
    );
    let graded = changed(&t, "fx_grade3_001.json", |e| {
        if let Effect::ColorGrade { values, more_curves, secondary_wheels, .. } = e {
            more_curves[1] = vec![vec![0.0, -20.0], vec![180.0, 20.0]];
            more_curves[3] = vec![vec![0.0, 100.0], vec![100.0, 60.0]];
            *secondary_wheels = "three".into();
            values[at("secondary_shadow_wheel_amount")] = 40.0;
        }
    });
    t.taken(
        &mut document,
        "fx_grade3_001.json",
        vec![
            ("hue versus luma, saturation versus saturation and three secondary wheels", set(graded)),
            ("key denoise keyed from 0 to 40", keys("key_denoise", &[(0, &[0.0]), (4, &[40.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_grade3_002.json", 0),
        ("fx_grade3_008.json", 0),
        ("fx_grade3_011.json", 0),
        ("fx_grade3_015.json", 0),
        ("fx_grade3_017.json", 0),
        ("fx_grade3_019.json", 4),
        ("fx_grade3_020.json", 0),
        ("fx_grade3_021.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // As added (001), flat curves (004), unused wheels (014), a key denoised or blurred (015
    // to 018, 020, 021: the processor's, D-571) and refused (023 to 034) leave nothing for the
    // card.
    let none: Vec<u32> = [1, 4, 14, 15, 16, 17, 18, 20, 21].into_iter().chain(23..=34).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "the four new curves: greens toward cyan, reds darker, the darks duller, the dull colours stronger",
                json!({"hue_hue": [[0, 0], [120, 30], [240, 0]], "hue_luma": [[0, -20], [180, 20]],
                    "luma_saturation": [[10, 60], [60, 100]], "saturation_saturation": [[0, 140], [50, 100]]}),
            ),
            (
                "HSL secondary with three wheels: the warm tones keyed, their shadows teal and highlights lowered",
                json!({"key_hue": 25, "key_hue_range": 15, "key_hue_softness": 25, "key_saturation_low": 20, "key_softness": 15,
                    "secondary_wheels": "three", "secondary_wheel_hue": 40, "secondary_wheel_amount": 20,
                    "secondary_shadow_wheel_hue": 195, "secondary_shadow_wheel_amount": 50, "secondary_highlight_lightness": -15}),
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-571 pictures/`");
    let dir = repo("verification/D-571 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let one = |p: J| json!([fx("fx-0-0", &p)]);
    let (before, said) = picture(&dir, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = picture(&dir, one(json!({})));
    t.row(
        "Color Grade as added, the new settings written, changes nothing: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );
    for (file, what, p) in [
        ("2_hue_vs_hue.png", "hue versus hue, the greens turned toward cyan and the blues toward purple", json!({"hue_hue": [[0, 0], [120, 40], [220, 30], [300, 0]]})),
        ("3_hue_vs_luma.png", "hue versus luma, the blue sky darker and the warm walls lighter", json!({"hue_luma": [[30, 30], [210, -40]]})),
        ("4_luma_vs_saturation.png", "luma versus saturation, the shadows grey and the lights stronger", json!({"luma_saturation": [[15, 0], [60, 100], [90, 160]]})),
        ("5_saturation_vs_saturation.png", "saturation versus saturation, the dull colours stronger and the strong ones duller", json!({"saturation_saturation": [[0, 180], [40, 100], [100, 60]]})),
        ("6_three_wheels.png", "every colour keyed, three secondary wheels: teal shadows, warm highlights, the midtones as they were", json!({"secondary_wheels": "three", "secondary_shadow_wheel_hue": 195, "secondary_shadow_wheel_amount": 60, "secondary_highlight_wheel_hue": 35, "secondary_highlight_wheel_amount": 50})),
    ] {
        let (after, said) = picture(&dir, one(p));
        write(file, &after);
        t.row(&format!("{file}, {what}: the street changed; draws cleanly"), &format!("{said:?}, {} pixels changed", distance(&after, &before).1), said.is_empty() && after != before);
    }
    let key = json!({"key_hue": 0, "key_hue_range": 20, "key_hue_softness": 15, "key_view": "mask"});
    let (rough, said) = picture(&dir, one(key.clone()));
    write("7_key_mask.png", &rough);
    t.row("7_key_mask.png, the reds keyed, shown as the mask: every pixel grey; draws cleanly", &format!("{said:?}, grey pixels {} of {}", greys(&rough), covered(&rough)), said.is_empty() && greys(&rough) == covered(&rough));
    let mut denoised = key.clone();
    denoised["key_denoise"] = J::from(30);
    let (clean, said) = picture(&dir, one(denoised));
    write("8_key_denoised.png", &clean);
    t.row(
        "8_key_denoised.png, the same key denoised 30 (a median of 3 pixels round): the street has no specks to remove, so the key's square corners come out rounded, still hard-edged, every pixel black or white; draws cleanly",
        &format!("{said:?}, {} pixels changed from 7, soft-edge pixels {}", distance(&clean, &rough).1, soft(&clean)),
        said.is_empty() && clean != rough && soft(&clean) == 0 && greys(&clean) == covered(&clean),
    );
    let mut blurred = key;
    blurred["key_blur"] = J::from(30);
    let (smooth, said) = picture(&dir, one(blurred));
    write("9_key_blurred.png", &smooth);
    t.row(
        "9_key_blurred.png, the same key blurred 30 (a blur of 3 pixels): a wider soft edge than 7; draws cleanly",
        &format!("{said:?}, soft-edge pixels {} before, {} after", soft(&rough), soft(&smooth)),
        said.is_empty() && soft(&smooth) > soft(&rough),
    );

    t.finish("D-571_color_grade_3_table.md");
}

/// The frame times, B-253's way, as B-277's: the reference shot with a Noise that changes every
/// frame on its first three layers, then the effect, every eighth frame asked for as the viewer
/// asks. With `B450_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-450: a measurement, run deliberately with --release --ignored"]
fn b450_color_grade_3_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B450_CPU").is_ok();
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
            "Noise, then Color Grade, the four new curves and three secondary wheels",
            Some(json!({"hue_hue": [[0, 0], [120, 30], [240, 0]], "hue_luma": [[0, -20], [180, 20]],
                "luma_saturation": [[10, 60], [60, 100]], "saturation_saturation": [[0, 140], [50, 100]],
                "key_hue": 25, "key_hue_range": 15, "key_hue_softness": 25, "secondary_wheels": "three",
                "secondary_shadow_wheel_hue": 195, "secondary_shadow_wheel_amount": 50})),
        ),
        (
            "Noise, then Color Grade, an HSL secondary with its key denoised 12 and blurred 8 (drawn by the processor)",
            Some(json!({"key_hue": 25, "key_hue_range": 15, "key_hue_softness": 25, "key_saturation_low": 20, "key_softness": 15,
                "secondary_saturation": 60, "secondary_wheel_hue": 200, "secondary_wheel_amount": 40, "key_denoise": 12, "key_blur": 8})),
        ),
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B450_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-450_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
