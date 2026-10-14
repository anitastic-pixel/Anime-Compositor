//! B-347: D-467, After Effects' Texturize under `core.texturize`: Texture Layer, Light Direction,
//! Texture Contrast and Texture Placement.
//!
//! Writes `verification/D-467_texturize_table.md` and draws pictures into
//! `verification/D-467 pictures/`.
//!
//! Every expected pixel is `Fixtures/texturize/expected_texturize.json`, written by
//! `tools/texturize_reference.py` before this code existed and printed in document 25 as
//! FX-TEXTURIZE-001 to 020. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

/// The Texturizes the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::Texturize { .. })))
        .count()
}

/// A Texturize with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "", "fit": "tile", "light_direction": 135, "texture_contrast": 1});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.texturize", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=20 {
        let file = format!("fx_texturize_{n:03}.json");
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
        let project = reference(|id| json!([fx(&format!("b347-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Texturize {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, Texturize {what} on three layers, frame {frame}, {}", quality.label()),
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

/// Pixel (x, y) of a picture `w` wide.
fn at(p: &[u8], w: usize, x: usize, y: usize) -> &[u8] {
    &p[(y * w + x) * 4..(y * w + x) * 4 + 4]
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
fn b347_texturize() {
    let mut t = Table::new(
        "texturize",
        "# D-467: Texturize\n\nB-347: `core.texturize` takes After Effects' Texturize controls \
         (Texture Layer, Light Direction, Texture Contrast, Texture Placement). Every expected pixel is \
         `Fixtures/texturize/expected_texturize.json`, written by `tools/texturize_reference.py` before \
         this code existed and printed in document 25 as FX-TEXTURIZE-001 to 020. Tolerance 2e-5.\n",
    );

    t.heading("FX-TEXTURIZE-001 to 020 (document 25)");
    t.fixtures_numbered("expected_texturize.json", 1..=20);

    t.heading("The file");
    let files: Vec<String> = (1..=20).map(|n| format!("fx_texturize_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_texturize_009.json");
    t.row(
        "fx_texturize_009.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["layer"] == "spot"
            && saved["fit"] == "stretch"
            && saved["light_direction"] == 135.0
            && saved["texture_contrast"] == 1.0,
    );
    for (file, want) in [
        ("fx_texturize_015.json", "texture contrast"),
        ("fx_texturize_016.json", "texture contrast"),
        ("fx_texturize_017.json", "texture placement"),
        ("fx_texturize_018.json", "texture layer"),
        ("fx_texturize_019.json", "light direction"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming Texturize and its {want}"), &why, why.contains("Texturize") && why.contains(want));
    }
    let full = r##"{"layer": "", "fit": "tile", "light_direction": 135, "texture_contrast": 1}"##;
    t.shape_refused("fx_texturize_001.json", "a Texturize whose contrast is a word", &full.replace("\"texture_contrast\": 1", "\"texture_contrast\": \"strong\""));
    t.shape_refused("fx_texturize_001.json", "a Texturize without its light direction", &full.replace(", \"light_direction\": 135", ""));
    t.shape_refused("fx_texturize_001.json", "a Texturize whose placement is a number", &full.replace("\"tile\"", "2"));

    t.heading("Commands");
    let mut document = t.load("fx_texturize_001.json").document;
    let over = changed(&t, "fx_texturize_001.json", |e| if let Effect::Texturize { texture_contrast, .. } = e { *texture_contrast = 2.5 });
    let fit = changed(&t, "fx_texturize_001.json", |e| if let Effect::Texturize { fit, .. } = e { *fit = "mirror".into() });
    let number = changed(&t, "fx_texturize_001.json", |e| if let Effect::Texturize { layer, .. } = e { *layer = json!(3) });
    let effect_of = |d: &Document| d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].clone();
    let held = format!("{:?}", effect_of(&document));
    for (what, command) in [
        ("texture contrast 2.5", set(over)),
        ("texture placement mirror", set(fit)),
        ("texture layer the number 3", set(number)),
        ("light direction keyed to 4000", keys("light_direction", &[(0, &[0.0]), (4, &[4000.0])])),
    ] {
        let refused = document.apply(holder(command)).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let moved = changed(&t, "fx_texturize_001.json", |e| {
        if let Effect::Texturize { layer, fit, light_direction, texture_contrast, .. } = e {
            *layer = json!("ramp");
            *fit = "center".into();
            *light_direction = 30.0;
            *texture_contrast = 0.5;
        }
    });
    t.taken(
        &mut document,
        "fx_texturize_001.json",
        vec![
            ("the ramp centred, Light Direction 30, Texture Contrast 0.5", holder(set(moved))),
            ("texture contrast keyed from 0 to 2", holder(keys("texture_contrast", &[(0, &[0.0]), (4, &[2.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_texturize_002.json", 0),
        ("fx_texturize_006.json", 0),
        ("fx_texturize_008.json", 0),
        ("fx_texturize_010.json", 0),
        ("fx_texturize_011.json", 0),
        ("fx_texturize_013.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // No texture (001), Texture Contrast 0 (004) and a missing texture layer (012) leave the
    // effect out; refused ones (015 to 020) leave nothing for the card.
    let none: Vec<u32> = [1, 4, 12].into_iter().chain(15..=20).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("with layer4 tiled as the texture", json!({"layer": "layer-4"}), 3),
            ("with layer4 stretched, Light Direction 45, Texture Contrast 2", json!({"layer": "layer-4", "fit": "stretch", "light_direction": 45, "texture_contrast": 2}), 3),
            ("with layer4 centred, Light Direction -100, Texture Contrast 0.4", json!({"layer": "layer-4", "fit": "center", "light_direction": -100, "texture_contrast": 0.4}), 3),
        ],
    );

    t.heading("Pictures: in `verification/D-467 pictures/`");
    let dir = repo("verification/D-467 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    write("brick.png", (32, 16), &brick());
    let street = |p: J| picture(&dir, "town.png", TOWN, "brick.png", json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, "brick.png", json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (w, h) = TOWN;
    let coverings = |p: &[u8]| p.chunks_exact(4).map(|q| q[3]).collect::<Vec<_>>();
    // How far a picture is from the street, summed over every colour.
    let away = |p: &[u8]| p.iter().zip(&before).map(|(a, b)| a.abs_diff(*b) as u64).sum::<u64>();
    // Each pixel lighter (1), darker (-1) or the same (0) as the street, by its summed colours.
    let sign = |p: &[u8]| {
        (0..w * h)
            .map(|i| {
                let s = |q: &[u8]| q[4 * i..4 * i + 3].iter().map(|&c| c as i32).sum::<i32>();
                (s(p) - s(&before)).signum()
            })
            .collect::<Vec<_>>()
    };

    let (none_picked, said) = street(json!({}));
    write("2_as_added.png", TOWN, &none_picked);
    t.row(
        "2_as_added.png, Texturize as added, no Texture Layer picked: the street exactly as it was; draws cleanly",
        &format!("{said:?}, the same as 1_before.png {}", none_picked == before),
        said.is_empty() && none_picked == before,
    );

    let (bricks, said) = street(json!({"layer": "bricks"}));
    write("3_bricks.png", TOWN, &bricks);
    t.row(
        "3_bricks.png, the brick picture as the texture, tiled, the light from the top left: a brick wall pressed into the whole street, the street's colours still there, the covering exactly as before; draws cleanly",
        &format!("{said:?}, changed {}, coverings the same {}", bricks != before, coverings(&bricks) == coverings(&before)),
        said.is_empty() && bricks != before && coverings(&bricks) == coverings(&before),
    );

    let (turned, said) = street(json!({"layer": "bricks", "light_direction": -45}));
    write("4_light_from_bottom_right.png", TOWN, &turned);
    let (a, b) = (sign(&bricks), sign(&turned));
    let flipped = a.iter().zip(&b).filter(|(x, y)| **x != 0 && **y == -**x).count();
    let kept = a.iter().zip(&b).filter(|(x, y)| **x != 0 && **y == **x).count();
    t.row(
        "4_light_from_bottom_right.png, Light Direction -45: the light from the bottom right, so the mortar edges that 3_bricks.png lightened are darkened and the other way round, at more pixels than keep their way; draws cleanly",
        &format!("{said:?}, turned over at {flipped} pixels, kept at {kept}"),
        said.is_empty() && flipped > kept,
    );

    let (strong, said) = street(json!({"layer": "bricks", "texture_contrast": 2}));
    write("5_contrast_2.png", TOWN, &strong);
    t.row(
        "5_contrast_2.png, Texture Contrast 2: the same bricks pressed in harder, further from the street than 3_bricks.png; draws cleanly",
        &format!("{said:?}, distance from the street {} against {}", away(&strong), away(&bricks)),
        said.is_empty() && away(&strong) > away(&bricks),
    );

    let (centred, said) = street(json!({"layer": "bricks", "fit": "center"}));
    write("6_one_brick_centred.png", TOWN, &centred);
    let corners_kept = [(0, 0), (w - 1, 0), (0, h - 1), (w - 1, h - 1), (w / 4, h / 4)].iter().all(|&(x, y)| at(&centred, w, x, y) == at(&before, w, x, y));
    t.row(
        "6_one_brick_centred.png, Center Texture: one brick pressed into the middle, the corners and the rest of the street untouched; draws cleanly",
        &format!("{said:?}, the middle changed {}, the corners and a quarter point untouched {corners_kept}", centred != before),
        said.is_empty() && centred != before && corners_kept,
    );

    t.finish("D-467_texturize_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B347_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-347: a measurement, run deliberately with --release --ignored"]
fn b347_texturize_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B347_CPU").is_ok();
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
        ("Noise, then Texturize with layer4 tiled as the texture", Some(json!({"layer": "layer-4"}))),
        ("the same, stretched, Light Direction 45, Texture Contrast 2", Some(json!({"layer": "layer-4", "fit": "stretch", "light_direction": 45, "texture_contrast": 2}))),
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
    let out = std::env::var("B347_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-347_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
