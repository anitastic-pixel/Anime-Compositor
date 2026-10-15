//! B-386: D-506, 3D Glasses under `core.three_d_glasses`, after After Effects' 3D Glasses
//! (Perspective): a left and a right view, each another layer named by D-189's layer setting or
//! the layer itself, made into one stereo picture: an anaglyph, the pair side by side or over
//! under, or interlaced. Adobe names the controls but publishes no formula and no defaults we
//! found: the rule and the settings when added are ours, stated in
//! `tools/three_d_glasses_reference.py`.
//!
//! Writes `verification/D-506_three_d_glasses_table.md` and draws pictures into
//! `verification/D-506 pictures/`.
//!
//! Every expected pixel is `Fixtures/three_d_glasses/expected_three_d_glasses.json`, written by
//! the tool before this code existed and printed in document 25 as FX-GLASSES-001 to 037.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, reference, repo, time_passes, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn is_ours(c: &render::OnCard) -> bool {
    matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::ThreeDGlasses { .. }))
}

/// The 3D Glasses the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers.iter().flat_map(|l| &l.on_card).filter(|c| is_ours(c.unmixed())).count()
        // An adjustment layer's run is made from its stack when the card draws it.
        + plan.layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .filter_map(|stack| compose::adjust_run(stack, (plan.width, plan.height)))
            .flatten()
            .filter(|c| is_ours(c.unmixed()))
            .count()
}

/// A 3D Glasses with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"left_view": "", "right_view": "", "scene_convergence": 0, "vertical_alignment": 0,
        "units": "pixels", "swap_left_right": "off", "three_d_view": "balanced_colored_red_blue", "balance": 0.75});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.three_d_glasses", "enabled": true, "parameters": all})
}

/// Every valid fixture file on the card against the processor, frames 0 to 4 at Full and Draft.
/// The files numbered in `none` leave nothing for the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=37 {
        let file = format!("fx_glasses_{n:03}.json");
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
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
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
        let project = reference(|id| json!([fx(&format!("b386-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, 3D Glasses {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, 3D Glasses {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the layer showing `1_left.png` (from `dir`), with a hidden layer `right` showing
/// `right_png` beneath it, drawn on the processor, straight 8-bit, with what it warned of.
fn picture(dir: &Path, right_png: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("1_left.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-right");
    asset["name"] = J::from("right");
    asset["path"] = J::from(right_png);
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    let layer = &mut comp["layers"][0];
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut right = layer.clone();
    right["id"] = J::from("right");
    right["name"] = J::from("right");
    right["asset_id"] = J::from("asset-right");
    right["enabled"] = J::from(false);
    right["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().insert(0, right);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("right"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Set the `holder` layer's first effect (the fixtures' own names).
fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's first effect, linearly.
fn keys_holder(setting: &str, values: &[(i32, f64)]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: vec![v], interp: Interp::Linear }).collect(),
    }
}

#[test]
fn b386_three_d_glasses() {
    let mut t = Table::new(
        "three_d_glasses",
        "# D-506: 3D Glasses\n\nB-386: `core.three_d_glasses`, after After Effects' 3D Glasses \
         (Perspective): a left and a right view, each another layer of the composition named by \
         D-189's layer setting (on the card through `map_texture`) or the layer itself, made into \
         one stereo picture. Adobe names the controls but publishes no formula and no defaults we \
         found, so the rule and the settings when added are ours, stated in \
         `tools/three_d_glasses_reference.py`: Scene Convergence slides the left view half its \
         distance right and the right view half left, Vertical Alignment half up and half down; \
         the red-green and red-blue views put the right view's brightness in red and the left's \
         in green (and blue); Balanced Colored Red Blue keeps the right's red and the left's green \
         and blue; Balance draws the two views towards their mean before that. Every expected \
         pixel is `Fixtures/three_d_glasses/expected_three_d_glasses.json`, written by the tool \
         before this code existed and printed in document 25 as FX-GLASSES-001 to 037. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-GLASSES-001 to 037 (document 25)");
    t.fixtures_numbered("expected_three_d_glasses.json", 1..=37);

    t.heading("Files whose layers read each other (document 25)");
    let expected: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/three_d_glasses/expected_three_d_glasses.json")).unwrap()).unwrap();
    for (file, case) in expected["loads"].as_object().unwrap() {
        let says = case["says"].as_str().unwrap();
        let id = case["refused"].as_str().unwrap();
        let got = persist::load(&t.root.join(file)).err();
        t.row(
            &format!("{file}: {says}"),
            &got.as_ref().map_or("opened".to_string(), |d| format!("{} {}", d.id.as_str(), d.message)),
            got.is_some_and(|d| d.id.as_str() == id),
        );
    }

    t.heading("The file");
    let files: Vec<String> = (1..=37).map(|n| format!("fx_glasses_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_glasses_002.json");
    t.row(
        "fx_glasses_002.json is saved with its eight settings, the views as written, red blue",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 8 && saved["left_view"] == "left" && saved["right_view"] == "right" && saved["three_d_view"] == "red_blue" && saved["balance"] == 0.75,
    );
    for (file, want) in [
        ("fx_glasses_027.json", "scene convergence"),
        ("fx_glasses_028.json", "scene convergence"),
        ("fx_glasses_029.json", "vertical alignment"),
        ("fx_glasses_030.json", "balance"),
        ("fx_glasses_031.json", "balance"),
        ("fx_glasses_032.json", "3D view"),
        ("fx_glasses_033.json", "units"),
        ("fx_glasses_034.json", "swap left-right"),
        ("fx_glasses_035.json", "left view"),
        ("fx_glasses_036.json", "right view"),
    ] {
        let why = holder_effect(&t, file).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    let whole = r#""left_view": "", "right_view": "", "vertical_alignment": 0, "units": "pixels", "swap_left_right": "off", "three_d_view": "red_blue", "balance": 0.75"#;
    t.shape_refused("fx_glasses_001.json", "a 3D Glasses whose convergence is a word", &format!(r#"{{{whole}, "scene_convergence": "near"}}"#));
    t.shape_refused("fx_glasses_001.json", "a 3D Glasses without its convergence", &format!(r#"{{{whole}}}"#));
    t.shape_refused(
        "fx_glasses_001.json",
        "a 3D Glasses without its right view",
        r#"{"left_view": "", "scene_convergence": 0, "vertical_alignment": 0, "units": "pixels", "swap_left_right": "off", "three_d_view": "red_blue", "balance": 0.75}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_glasses_002.json").document;
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = holder_effect(&t, "fx_glasses_002.json");
        f(&mut e);
        e
    };
    let over = with(&|e| if let Effect::ThreeDGlasses { scene_convergence, .. } = e { *scene_convergence = 1001.0 });
    let view = with(&|e| if let Effect::ThreeDGlasses { three_d_view, .. } = e { *three_d_view = "anaglyph".into() });
    let units = with(&|e| if let Effect::ThreeDGlasses { units, .. } = e { *units = "inches".into() });
    let lined = with(&|e| {
        if let Effect::ThreeDGlasses { scene_convergence, three_d_view, .. } = e {
            *scene_convergence = 2.0;
            *three_d_view = "balanced_red_green".into();
        }
    });
    // B-386b: Draft halves the distances when they are pixels, as every other distance, and
    // keeps them when they are a share of the layer, which is already halved.
    let draft = |units: &str, c: f64, v: f64| {
        with(&|e| {
            if let Effect::ThreeDGlasses { scene_convergence, vertical_alignment, units: u, .. } = e {
                (*scene_convergence, *vertical_alignment, *u) = (c, v, units.to_string());
            }
        })
    };
    let (mut pixels, mut percent) = (draft("pixels", 8.0, -4.0), draft("percent", 8.0, -4.0));
    pixels.scale_distances(|d| d * 0.5);
    percent.scale_distances(|d| d * 0.5);
    let halved = pixels == draft("pixels", 4.0, -2.0) && percent == draft("percent", 8.0, -4.0);
    refused(
        &mut t,
        &mut document,
        vec![
            ("scene convergence 1001", set_holder(over)),
            ("3D view \"anaglyph\"", set_holder(view)),
            ("units \"inches\"", set_holder(units)),
            ("balance keyed to 20", keys_holder("balance", &[(0, 0.0), (4, 20.0)])),
        ],
    );
    taken(
        &mut t,
        &mut document,
        "fx_glasses_002.json",
        vec![
            ("balanced red green at convergence 2", set_holder(lined)),
            ("vertical alignment keyed from 0 to 4", keys_holder("vertical_alignment", &[(0, 0.0), (4, 4.0)])),
        ],
    );

    t.row(
        "a half-size draft preview halves the convergence and alignment in pixels, and keeps them in per cent (B-386b)",
        &format!("{pixels:?} / {percent:?}"),
        halved,
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glasses_002.json", 0),
        ("fx_glasses_006.json", 0),
        ("fx_glasses_007.json", 0),
        ("fx_glasses_008.json", 0),
        ("fx_glasses_009.json", 0),
        ("fx_glasses_011.json", 0),
        ("fx_glasses_013.json", 0),
        ("fx_glasses_018.json", 2),
        ("fx_glasses_021.json", 0),
        ("fx_glasses_022.json", 0),
        ("fx_glasses_024.json", 0),
        ("fx_glasses_026.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // A view not in the composition (019) and refused (027 to 037) leave nothing for the card.
    let none: Vec<u32> = [19].into_iter().chain(27..=37).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("the layer itself against layer 4, red blue", json!({"right_view": "layer-4", "three_d_view": "red_blue"})),
            ("against layer 4, balanced colored, convergence 1.5 per cent, alignment -2, swapped", json!({"right_view": "layer-4", "units": "percent", "scene_convergence": 1.5, "vertical_alignment": -2, "swap_left_right": "on"})),
            ("the layer itself, interlaced, convergence 7", json!({"three_d_view": "interlace", "scene_convergence": 7})),
            ("against layer 4, side by side", json!({"right_view": "layer-4", "three_d_view": "side_by_side"})),
            ("against layer 4, over under", json!({"right_view": "layer-4", "three_d_view": "over_under"})),
        ],
    );

    t.heading("Pictures: in `verification/D-506 pictures/`");
    let dir = repo("verification/D-506 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), w, h, OutputDepth::Eight, &[], bytes).unwrap();
    // The left eye sees the street; the right eye, standing a little to the right, sees it 6
    // pixels further left. The last 6 columns, which the right eye would see past the edge, are
    // the street's own last column repeated.
    let street = town();
    let right: Vec<u8> = (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let j = (y * w + (x + 6).min(w - 1)) * 4;
            [street[j], street[j + 1], street[j + 2], street[j + 3]]
        })
        .collect();
    write("1_left.png", &street);
    write("2_right.png", &right);
    let grey = |p: &[u8]| p[0].abs_diff(p[1]) <= 1 && p[1].abs_diff(p[2]) <= 1;

    let (anaglyph, said) = picture(&dir, "2_right.png", json!([fx("fx-0-0", &json!({"right_view": "right", "three_d_view": "red_blue"}))]));
    write("3_red_blue.png", &anaglyph);
    let fringed = anaglyph.chunks_exact(4).filter(|p| !grey(p)).count();
    t.row(
        "3_red_blue.png, red blue as the two eyes stand: the street in grey with red and cyan fringes where the eyes disagree (watch it with red-cyan glasses); draws cleanly",
        &format!("{said:?}, {fringed} of {} pixels coloured", w * h),
        said.is_empty() && fringed > w * h / 20,
    );

    // Convergence -6 slides the left view 3 pixels left and the right 3 right, so the eyes agree.
    let (lined, said) = picture(&dir, "2_right.png", json!([fx("fx-0-0", &json!({"right_view": "right", "three_d_view": "red_blue", "scene_convergence": -6}))]));
    write("4_lined_up.png", &lined);
    let inside: Vec<&[u8]> = lined.chunks_exact(4).enumerate().filter(|(i, _)| (3..w - 3).contains(&(i % w))).map(|(_, p)| p).collect();
    let agree = inside.iter().filter(|p| grey(p) && p[3] == 255).count();
    t.row(
        "4_lined_up.png, the same at convergence -6: the eyes agree, so between the 3 columns at each edge (which only one eye sees) the picture is plain grey; draws cleanly",
        &format!("{said:?}, {agree} of {} pixels grey", inside.len()),
        said.is_empty() && agree == inside.len(),
    );

    let (colored, said) = picture(&dir, "2_right.png", json!([fx("fx-0-0", &json!({"right_view": "right"}))]));
    write("5_balanced_colored.png", &colored);
    let changed = distance(&colored, &street).1;
    t.row(
        "5_balanced_colored.png, as added but with the right eye named: the street in its own colours, fringed where the eyes disagree; draws cleanly",
        &format!("{said:?}, {changed} pixels differ from the left eye"),
        said.is_empty() && changed > 0,
    );

    let (pair, said) = picture(&dir, "2_right.png", json!([fx("fx-0-0", &json!({"right_view": "right", "three_d_view": "side_by_side"}))]));
    write("6_side_by_side.png", &pair);
    // The right half is the left half 3 pixels on (the eyes' 6 pixels, squeezed by half).
    let half = w / 2;
    let (mut same, mut all) = (0, 0);
    for y in 0..h {
        for x in 0..half - 4 {
            let (a, b) = ((y * w + x + 3) * 4, (y * w + x + half) * 4);
            all += 1;
            same += (distance(&pair[a..a + 4], &pair[b..b + 4]).0 <= 1) as usize;
        }
    }
    t.row(
        "6_side_by_side.png, side by side: the left eye squeezed into the left half, the right eye into the right half, which is the left half 3 pixels on; draws cleanly",
        &format!("{said:?}, {same} of {all} pixels match"),
        said.is_empty() && same == all,
    );

    let (gone, said) = picture(&dir, "2_right.png", json!([fx("fx-0-0", &json!({"right_view": "gone", "three_d_view": "red_blue"}))]));
    t.row(
        "a right view not in the composition: the street exactly as it was, with the warning",
        &format!("{said:?}, {} pixels changed", distance(&gone, &street).1),
        gone == street && said.iter().any(|s| s.starts_with("EFFECT_LAYER_MISSING")),
    );

    t.finish("D-506_three_d_glasses_table.md");
}

/// The `holder` layer's effect in `file`.
fn holder_effect(t: &Table, file: &str) -> Effect {
    let d = t.load(file).document;
    let layer = d.project().composition(&Id::new(MAIN)).unwrap().layers_in_order().find(|l| l.effects.iter().any(|e| matches!(e.effect, Effect::ThreeDGlasses { .. })));
    layer.expect("a layer with a 3D Glasses").effects[0].effect.clone()
}

/// The whole composition, to see that a refused command changed nothing (the effect is on
/// `holder`, not on the table's `art` layer).
fn everything(document: &anime_compositor::command::Document) -> String {
    format!("{:?}", document.project().composition(&Id::new(MAIN)).unwrap())
}

fn refused(t: &mut Table, document: &mut anime_compositor::command::Document, commands: Vec<(&str, Command)>) {
    let held = everything(document);
    for (what, command) in commands {
        let refused = document.apply(command).err();
        let untouched = everything(document) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
}

fn taken(t: &mut Table, document: &mut anime_compositor::command::Document, file: &str, commands: Vec<(&str, Command)>) {
    let n = commands.len();
    for (what, command) in commands {
        let taken = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if taken { "taken" } else { "refused" }, taken);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B386_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-386: a measurement, run deliberately with --release --ignored"]
fn b386_three_d_glasses_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B386_CPU").is_ok();
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
        ("Noise, then 3D Glasses against layer 4, balanced colored red blue", Some(json!({"right_view": "layer-4"}))),
        ("Noise, then 3D Glasses, the layer itself, red blue, convergence 7", Some(json!({"three_d_view": "red_blue", "scene_convergence": 7}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}g"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B386_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-386_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
