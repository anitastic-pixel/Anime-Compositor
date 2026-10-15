//! B-483: D-604, After Effects' Warp under `core.warp`: fifteen preset shapes (Arc, Flag,
//! Bulge, Twist and the rest) bent by Bend along either axis, leaned by the two Distortions, and
//! drawn through D-601's forward mesh.
//!
//! Writes `verification/D-604_warp_table.md` and draws pictures into `verification/D-604 pictures/`.
//!
//! Every expected pixel is `Fixtures/warp/expected_warp.json`, written by
//! `tools/warp_reference.py` before this code existed and printed in document 25 as FX-WARP-001
//! to 030. Tolerance 2e-5. Nothing here is a snapshot of a run.

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
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Warp with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"warp_style": "arc", "warp_axis": "horizontal", "bend": 50, "horizontal_distortion": 0, "vertical_distortion": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.warp", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_warp_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::Warp { .. }));
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
        let project = reference(|id| json!([fx(&format!("b483-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Warp {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::Warp { .. }));
                t.row(
                    &format!("the reference shot, Warp {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
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
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Whether the pixel (x, y) of a picture `w` wide is clear.
fn clear(p: &[u8], w: usize, x: usize, y: usize) -> bool {
    p[(y * w + x) * 4 + 3] == 0
}

#[test]
fn b483_warp() {
    let mut t = Table::new(
        "warp",
        "# D-604: Warp\n\nB-483: `core.warp` takes After Effects' Warp controls (Warp Style, Warp \
         Axis, Bend, Horizontal Distortion, Vertical Distortion) and bends the drawing into the \
         style's shape, drawn through D-601's forward mesh. Every expected pixel is \
         `Fixtures/warp/expected_warp.json`, written by `tools/warp_reference.py` before this \
         code existed and printed in document 25 as FX-WARP-001 to 030. Tolerance 2e-5.\n",
    );

    t.heading("FX-WARP-001 to 030 (document 25)");
    t.fixtures_numbered("expected_warp.json", 1..=30);

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_warp_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_warp_001.json");
    t.row(
        "fx_warp_001.json is saved with its five settings, the style and axis as words",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 5
            && saved["warp_style"] == "arc"
            && saved["warp_axis"] == "horizontal"
            && saved["bend"] == 50.0
            && saved["horizontal_distortion"] == 0.0
            && saved["vertical_distortion"] == 0.0,
    );
    for (file, want) in [
        ("fx_warp_026.json", "warp style"),
        ("fx_warp_027.json", "warp axis"),
        ("fx_warp_028.json", "bend"),
        ("fx_warp_029.json", "horizontal distortion"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Warp and its {want}"), &why, why.contains("Warp") && why.contains(want));
    }
    let full = fx("x", &json!({}))["parameters"].to_string();
    t.shape_refused("fx_warp_001.json", "a Warp whose style is a number", &full.replace("\"warp_style\":\"arc\"", "\"warp_style\":3"));
    t.shape_refused("fx_warp_001.json", "a Warp whose bend is a word", &full.replace("\"bend\":50", "\"bend\":\"half\""));
    t.shape_refused("fx_warp_001.json", "a Warp without its axis", &full.replace("\"warp_axis\":\"horizontal\",", ""));

    t.heading("Commands");
    let mut document = t.load("fx_warp_001.json").document;
    let spiral = changed(&t, "fx_warp_001.json", |e| if let Effect::Warp { warp_style, .. } = e { *warp_style = "spiral".into() });
    let over = changed(&t, "fx_warp_001.json", |e| if let Effect::Warp { bend, .. } = e { *bend = 101.0 });
    t.refused(
        &mut document,
        vec![
            ("warp style spiral", set(spiral)),
            ("bend 101", set(over)),
            ("vertical distortion keyed to 150", keys("vertical_distortion", &[(0, &[0.0]), (4, &[150.0])])),
        ],
    );
    let twist = changed(&t, "fx_warp_001.json", |e| {
        if let Effect::Warp { warp_style, warp_axis, bend, .. } = e {
            *warp_style = "twist".into();
            *warp_axis = "vertical".into();
            *bend = -70.0;
        }
    });
    t.taken(
        &mut document,
        "fx_warp_001.json",
        vec![
            ("twist, vertical, bend -70", set(twist)),
            ("bend keyed from 0 to 100", keys("bend", &[(0, &[0.0]), (4, &[100.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_warp_001.json", 0),
        ("fx_warp_010.json", 0),
        ("fx_warp_021.json", 0),
        ("fx_warp_022.json", 2),
        ("fx_warp_025.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 002 (Bend 0, no distortion) changes nothing, so nothing is left to the card; the refused
    // (026 to 030) neither; 022 changes nothing at frame 0 only, so it is on the card in 8 of 10.
    let none: Vec<u32> = [2].into_iter().chain(26..=30).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added, Arc at Bend 50", json!({}), 3),
            ("Flag, vertical, Bend -60", json!({"warp_style": "flag", "warp_axis": "vertical", "bend": -60}), 3),
            ("Twist 80 with both distortions 30", json!({"warp_style": "twist", "bend": 80, "horizontal_distortion": 30, "vertical_distortion": 30}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-604 pictures/`");
    let dir = repo("verification/D-604 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let (w, h) = TOWN;
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    let open = |p: &[u8]| (0..w * h).filter(|&i| p[i * 4 + 3] == 0).count();
    t.row("1_before.png, the street with no effect, every pixel covered; draws cleanly", &format!("{said:?}, {} clear pixels", open(&before)), said.is_empty() && open(&before) == 0);

    let (flat, said) = street(json!({"bend": 0}));
    t.row(
        "Bend 0 and no distortion: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels differ", distance(&flat, &before).1),
        said.is_empty() && flat == before,
    );

    let (arc, said) = street(json!({}));
    write("2_arc.png", TOWN, &arc);
    let corners = clear(&arc, w, 0, h - 1) && clear(&arc, w, w - 1, h - 1);
    t.row(
        "2_arc.png, as added (Arc, Bend 50): the street bent round a circle, the bottom corners lifted clear; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street, {} clear, both bottom corners clear: {corners}", distance(&arc, &before).1, open(&arc)),
        said.is_empty() && distance(&arc, &before).1 > w * h / 10 && corners,
    );

    let (flag, said) = street(json!({"warp_style": "flag"}));
    write("3_flag.png", TOWN, &flag);
    let top = |p: &[u8], x: usize| (0..h).take_while(|&y| clear(p, w, x, y)).count();
    t.row(
        "3_flag.png, Flag at Bend 50: the street rising then dropping, none clear above it a quarter across and clear three quarters across; draws cleanly",
        &format!("{said:?}, clear rows from the top at a quarter and three quarters across: {}, {}", top(&flag, w / 4), top(&flag, 3 * w / 4)),
        said.is_empty() && top(&flag, w / 4) == 0 && top(&flag, 3 * w / 4) > 0,
    );

    let (upright, said) = street(json!({"warp_style": "flag", "warp_axis": "vertical"}));
    write("4_flag_vertical.png", TOWN, &upright);
    t.row(
        "4_flag_vertical.png, the same Flag on the vertical axis: the wave runs down the street instead, so it differs from 3_flag.png; draws cleanly",
        &format!("{said:?}, {} pixels differ from 3_flag.png", distance(&upright, &flag).1),
        said.is_empty() && distance(&upright, &flag).1 > w * h / 10,
    );

    let (twist, said) = street(json!({"warp_style": "twist", "bend": 100}));
    write("5_twist.png", TOWN, &twist);
    t.row(
        "5_twist.png, Twist at Bend 100: the middle turned round, the corners nearly in place; draws cleanly",
        &format!("{said:?}, {} pixels differ from the street", distance(&twist, &before).1),
        said.is_empty() && distance(&twist, &before).1 > w * h / 10,
    );

    let (leaned, said) = street(json!({"bend": 0, "horizontal_distortion": 60}));
    write("6_horizontal_distortion_60.png", TOWN, &leaned);
    t.row(
        "6_horizontal_distortion_60.png, Bend 0, Horizontal Distortion 60: the right side taller than the left, so the top left corner clear and the top right not; draws cleanly",
        &format!("{said:?}, top left clear: {}, top right clear: {}", clear(&leaned, w, 0, 0), clear(&leaned, w, w - 1, 0)),
        said.is_empty() && clear(&leaned, w, 0, 0) && !clear(&leaned, w, w - 1, 0),
    );

    t.finish("D-604_warp_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B483_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-483: a measurement, run deliberately with --release --ignored"]
fn b483_warp_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B483_CPU").is_ok();
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
        ("Noise, then Warp as added (Arc, Bend 50)", Some(json!({}))),
        ("Noise, then Warp Twist 80 with both distortions 30", Some(json!({"warp_style": "twist", "bend": 80, "horizontal_distortion": 30, "vertical_distortion": 30}))),
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
    let out = std::env::var("B483_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-483_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
