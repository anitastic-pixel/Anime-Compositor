//! B-352: D-472, After Effects' Gaussian Blur (Legacy) (Obsolete) under
//! `core.gaussian_blur_legacy`, a second name over Gaussian Blur's engine in Blurriness (D-321), as
//! D-383 and D-394: Blurriness and Blur Dimensions, drawn as Gaussian Blur in Blurriness with its
//! edges always transparent.
//!
//! Writes `verification/D-472_gaussian_blur_legacy_table.md` and draws pictures into
//! `verification/D-472 pictures/`.
//!
//! Every expected pixel is `Fixtures/gaussian_blur_legacy/expected_gaussian_blur_legacy.json`,
//! written by `tools/gaussian_blur_legacy_reference.py` before this code existed and printed in
//! document 25 as FX-GAUSSLEG-001 to 011. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn legacy(blurriness: f64, dimensions: &str) -> Effect {
    Effect::GaussianBlurLegacy { blurriness, dimensions: dimensions.into() }
}

/// The Gaussian Blurs the card's plan leaves to the card, over every layer: a Gaussian Blur
/// (Legacy) goes to the card as the Gaussian Blur it is, and nothing else here has one.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Gaussian(_)))
        .count()
}

/// A Gaussian Blur (Legacy) with the settings `p` over Blurriness 10, both ways.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"blurriness": 10, "dimensions": "both"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.gaussian_blur_legacy", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left off the card; FX-GAUSSLEG-007 is a Float composition, which
/// the card refuses whole (D-319), so the processor draws it and the card's frame says so.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=11 {
        let file = format!("fx_gaussleg_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && if n == 7 { refused } else { !refused && agree && (card == 0) == none.contains(&n) },
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor, 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in cases {
        let project = reference(|id| json!([fx(&format!("b352-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Gaussian Blur (Legacy) {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Gaussian Blur (Legacy) {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
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

#[test]
fn b352_gaussian_blur_legacy() {
    let mut t = Table::new(
        "gaussian_blur_legacy",
        "# D-472: Gaussian Blur (Legacy)\n\nB-352: `core.gaussian_blur_legacy` takes After Effects' Gaussian Blur \
         (Legacy) controls (Blurriness, Blur Dimensions) and draws them with Gaussian Blur's engine in Blurriness, \
         its edges always transparent (D-321; D-383 and D-394, both names, one engine). Every expected pixel is \
         `Fixtures/gaussian_blur_legacy/expected_gaussian_blur_legacy.json`, written by \
         `tools/gaussian_blur_legacy_reference.py` before this code existed and printed in document 25 as \
         FX-GAUSSLEG-001 to 011. Tolerance 2e-5.\n",
    );

    t.heading("FX-GAUSSLEG-001 to 011 (document 25)");
    t.fixtures_numbered("expected_gaussian_blur_legacy.json", 1..=11);

    t.heading("One engine");
    let text = fs::read_to_string(t.root.join("fx_gaussleg_001.json")).unwrap();
    let mut j: J = serde_json::from_str(&text).unwrap();
    j["compositions"][0]["layers"][0]["effects"][0] = json!({"instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": 10, "units": "blurriness"}});
    let gauss = persist::load_str(&j.to_string()).expect("the Gaussian Blur project reads").document;
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_gaussian_blur_legacy.json")).unwrap()).unwrap();
    let d = effect_table::largest_difference(&t.render(&gauss, 0, 64), &expected["cases"]["FX-GAUSSLEG-001"]["frames"]["0"]);
    t.row(
        "FX-GAUSSLEG-001's drawing with Gaussian Blur, Blurriness 10 in Blurriness, in its place draws FX-GAUSSLEG-001's frame (Blurriness 10)",
        &format!("largest difference {d:.1e}"),
        d <= 2e-5,
    );

    t.heading("The file");
    let files: Vec<String> = (1..=7).map(|n| format!("fx_gaussleg_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_gaussleg_003.json");
    t.row(
        "fx_gaussleg_003.json is saved with its blurriness and dimensions, and no sigma, units or edges of its own",
        &saved.to_string(),
        saved["blurriness"] == 10.0
            && saved["dimensions"] == "horizontal"
            && saved.get("sigma_px").is_none()
            && saved.get("units").is_none()
            && saved.get("edges").is_none(),
    );
    for (file, want) in [("fx_gaussleg_008.json", "blurriness"), ("fx_gaussleg_009.json", "blurriness"), ("fx_gaussleg_010.json", "dimensions")] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Gaussian Blur (Legacy) and its {want}"), &why, why.contains("Gaussian Blur (Legacy)") && why.contains(want));
    }
    let full = r##"{"blurriness": 10, "dimensions": "both"}"##;
    t.shape_refused("fx_gaussleg_001.json", "a Gaussian Blur (Legacy) whose blurriness is a word", &full.replace("10", "\"soft\""));
    t.shape_refused("fx_gaussleg_001.json", "a Gaussian Blur (Legacy) whose dimensions is a number", &full.replace("\"both\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_gaussleg_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("blurriness 501", set(legacy(501.0, "both"))),
            ("blurriness -1", set(legacy(-1.0, "both"))),
            ("Blur Dimensions \"Both\", written with a capital", set(legacy(10.0, "Both"))),
            ("blurriness keyed to 600", keys("blurriness", &[(0, &[10.0]), (4, &[600.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_gaussleg_001.json",
        vec![
            ("blurriness 20, vertical", set(legacy(20.0, "vertical"))),
            ("blurriness keyed from 0 to 500", keys("blurriness", &[(0, &[0.0]), (4, &[500.0])])),
        ],
    );

    t.heading("A half-size draft");
    let mut draft = legacy(20.0, "both");
    draft.scale_distances(|v| v / 2.0);
    t.row("A half-size draft halves the blurriness", &format!("{draft:?}"), draft == legacy(10.0, "both"));

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_gaussleg_001.json", 0), ("fx_gaussleg_003.json", 0), ("fx_gaussleg_004.json", 0), ("fx_gaussleg_006.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // One way only (003, 004) is the processor's, as Gaussian Blur's (D-303); Blurriness 0 (005)
    // leaves the effect out; refused ones (008 to 011) leave nothing for the card.
    let none: Vec<u32> = [3, 4, 5].into_iter().chain(8..=11).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(&mut t, &mut gpu, &[("Blurriness 10", json!({})), ("Blurriness 60", json!({"blurriness": 60}))]);

    t.heading("Pictures: in `verification/D-472 pictures/`");
    let dir = repo("verification/D-472 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let street = |e: J| picture(&dir, "town.png", TOWN, json!([e]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (legacy20, said) = street(fx("fx-0-0", &json!({"blurriness": 20})));
    write("2_blurriness_20.png", &legacy20);
    let (boxed, said_box) = street(json!({"instance_id": "fx-0-0", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": 20, "units": "blurriness"}}));
    write("3_gaussian_blur_20.png", &boxed);
    t.row(
        "2_blurriness_20.png, Gaussian Blur (Legacy) Blurriness 20, and 3_gaussian_blur_20.png, Gaussian Blur at Blurriness 20: the same soft street, byte for byte; both draw cleanly",
        &format!("{said:?} {said_box:?}, changed {}, the same {}", legacy20 != before, legacy20 == boxed),
        said.is_empty() && said_box.is_empty() && legacy20 != before && legacy20 == boxed,
    );

    let (across, said) = street(fx("fx-0-0", &json!({"blurriness": 40, "dimensions": "horizontal"})));
    write("4_horizontal_40.png", &across);
    // A row's sums of differences against the street, across and down.
    let (w, h) = TOWN;
    let step = |p: &[u8], dx: usize, dy: usize| {
        let mut s = 0u64;
        for y in 0..h - dy {
            for x in 0..w - dx {
                let (a, b) = ((y * w + x) * 4, ((y + dy) * w + x + dx) * 4);
                s += p[a].abs_diff(p[b]) as u64;
            }
        }
        s
    };
    t.row(
        "4_horizontal_40.png, Blurriness 40, Blur Dimensions horizontal: smeared sideways only, so neighbours across differ far less than in the street while neighbours down differ about as much; draws cleanly",
        &format!("{said:?}, across {} against {}, down {} against {}", step(&across, 1, 0), step(&before, 1, 0), step(&across, 0, 1), step(&before, 0, 1)),
        said.is_empty() && step(&across, 1, 0) * 3 < step(&before, 1, 0) && step(&across, 0, 1) * 2 > step(&before, 0, 1),
    );

    t.finish("D-472_gaussian_blur_legacy_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B352_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-352: a measurement, run deliberately with --release --ignored"]
fn b352_gaussian_blur_legacy_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B352_CPU").is_ok();
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
        ("Noise, then Gaussian Blur (Legacy) Blurriness 10", Some(json!({}))),
        ("the same at Blurriness 100", Some(json!({"blurriness": 100}))),
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
    let out = std::env::var("B352_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-352_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
