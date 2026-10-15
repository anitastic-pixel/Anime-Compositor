//! B-343: D-463, After Effects' CC Plastic under `core.plastic`: Bump Layer, Property, Softness,
//! Height, Cut Min, Cut Max, the light, Ambient, Diffuse, Dust, Specular, Roughness and Metal.
//!
//! Writes `verification/D-463_plastic_table.md` and draws pictures into
//! `verification/D-463 pictures/`.
//!
//! Every expected pixel is `Fixtures/plastic/expected_plastic.json`, written by
//! `tools/plastic_reference.py` before this code existed and printed in document 25 as
//! FX-PLASTIC-001 to 031. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN,
    TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Plastic with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50, "cut_min": 0, "cut_max": 100,
        "light_intensity": 100, "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": [30, 30],
        "light_direction": -45, "ambient_light_color": "#ffffff", "ambient": 25, "diffuse": 75, "dust": 0, "specular": 50, "roughness": 0.05, "metal": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.plastic", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=31 {
        let file = format!("fx_plastic_{n:03}.json");
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
                    matches!(e, Effect::Plastic { .. })
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
        let project = reference(|id| json!([fx(&format!("b343-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Plastic {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::Plastic { .. })
                });
                t.row(
                    &format!("the reference shot, Plastic {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor at `frame`,
/// straight 8-bit, with what it warned of.
fn picture(
    dir: &Path,
    asset: &str,
    size: (usize, usize),
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

/// The `art` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("holder"))
        .unwrap()
        .effects[0]
        .effect
        .clone();
    f(&mut e);
    e
}

/// Pixel (x, y) of a picture `w` wide.
fn at(p: &[u8], w: usize, x: usize, y: usize) -> &[u8] {
    &p[(y * w + x) * 4..(y * w + x) * 4 + 4]
}

/// `command` aimed at the `holder` layer's effect `fx-1`; the fixtures' drawing is `holder`, not `art`.
fn holder(mut command: Command) -> Command {
    if let Command::SetEffectParameters {
        layer_id,
        instance_id,
        ..
    }
    | Command::SetEffectKeys {
        layer_id,
        instance_id,
        ..
    } = &mut command
    {
        *layer_id = Id::new("holder");
        *instance_id = Id::new("fx-1");
    }
    command
}

#[test]
fn b343_plastic() {
    let mut t = Table::new(
        "plastic",
        "# D-463: Plastic\n\nB-343: `core.plastic` takes After Effects' CC Plastic controls \
         (Bump Layer, Property, Softness, Height, Cut Min, Cut Max, the light, Ambient, Diffuse, Dust, \
         Specular, Roughness, Metal). Every expected pixel is `Fixtures/plastic/expected_plastic.json`, \
         written by `tools/plastic_reference.py` before this code existed and printed in document 25 as \
         FX-PLASTIC-001 to 031. Tolerance 2e-5.\n",
    );

    t.heading("FX-PLASTIC-001 to 031 (document 25)");
    t.fixtures_numbered("expected_plastic.json", 1..=31);

    t.heading("The file");
    let files: Vec<String> = (1..=31)
        .map(|n| format!("fx_plastic_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_plastic_013.json");
    t.row(
        "fx_plastic_013.json is saved with its twenty settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 20
            && saved["layer"] == "ramp"
            && saved["fit"] == "stretch"
            && saved["property"] == "luminance"
            && saved["softness"] == 0.0
            && saved["height"] == 100.0
            && saved["cut_min"] == 30.0
            && saved["cut_max"] == 70.0
            && saved["light_intensity"] == 100.0
            && saved["light_color"] == "#ffffff"
            && saved["light_type"] == "distant"
            && saved["light_height"] == 100.0
            && saved["light_position"] == json!([30, 30])
            && saved["light_direction"] == -45.0
            && saved["ambient_light_color"] == "#ffffff"
            && saved["ambient"] == 25.0
            && saved["diffuse"] == 75.0
            && saved["dust"] == 0.0
            && saved["specular"] == 50.0
            && saved["roughness"] == 0.05
            && saved["metal"] == 0.0,
    );
    for (file, want) in [
        ("fx_plastic_023.json", "softness"),
        ("fx_plastic_024.json", "height"),
        ("fx_plastic_025.json", "cut min"),
        ("fx_plastic_026.json", "property"),
        ("fx_plastic_027.json", "light type"),
        ("fx_plastic_028.json", "roughness"),
        ("fx_plastic_029.json", "bump layer"),
        ("fx_plastic_030.json", "ambient light colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming Plastic and its {want}"),
            &why,
            why.contains("Plastic") && why.contains(want),
        );
    }
    let full = r##"{"layer": "", "fit": "stretch", "property": "alpha", "softness": 10, "height": 50, "cut_min": 0, "cut_max": 100, "light_intensity": 100, "light_color": "#ffffff", "light_type": "distant", "light_height": 100, "light_position": [30, 30], "light_direction": -45, "ambient_light_color": "#ffffff", "ambient": 25, "diffuse": 75, "dust": 0, "specular": 50, "roughness": 0.05, "metal": 0}"##;
    t.shape_refused(
        "fx_plastic_001.json",
        "a Plastic whose softness is a word",
        &full.replace("\"softness\": 10", "\"softness\": \"soft\""),
    );
    t.shape_refused(
        "fx_plastic_001.json",
        "a Plastic without its metal",
        &full.replace(", \"metal\": 0", ""),
    );
    t.shape_refused(
        "fx_plastic_001.json",
        "a Plastic whose light position is one number",
        &full.replace("[30, 30]", "30"),
    );
    t.shape_refused(
        "fx_plastic_001.json",
        "a Plastic whose ambient light colour is a number",
        &full.replace(
            "\"ambient_light_color\": \"#ffffff\"",
            "\"ambient_light_color\": 4",
        ),
    );

    t.heading("Commands");
    let mut document = t.load("fx_plastic_001.json").document;
    let over = changed(&t, "fx_plastic_001.json", |e| {
        if let Effect::Plastic { softness, .. } = e {
            *softness = 101.0
        }
    });
    let far = changed(&t, "fx_plastic_001.json", |e| {
        if let Effect::Plastic { light_position, .. } = e {
            *light_position = [50.0, -1001.0]
        }
    });
    let word = changed(&t, "fx_plastic_001.json", |e| {
        if let Effect::Plastic { light_type, .. } = e {
            *light_type = "spot".into()
        }
    });
    let effect_of = |d: &Document| {
        d.project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("holder"))
            .unwrap()
            .effects[0]
            .clone()
    };
    let held = format!("{:?}", effect_of(&document));
    for (what, command) in [
        ("softness 101", set(over)),
        ("light position 50, -1001", set(far)),
        ("light type spot", set(word)),
        (
            "metal keyed to 150",
            keys("metal", &[(0, &[0.0]), (4, &[150.0])]),
        ),
    ] {
        let refused = document.apply(holder(command)).err();
        let untouched = format!("{:?}", effect_of(&document)) == held;
        t.row(
            &format!("{what} is refused with a sentence, and nothing changes"),
            &refused
                .as_ref()
                .map_or("taken".to_string(), |d| d.message.clone()),
            refused.is_some() && untouched,
        );
    }
    let moved = changed(&t, "fx_plastic_001.json", |e| {
        if let Effect::Plastic {
            layer,
            fit,
            property,
            softness,
            cut_min,
            light_type,
            light_color,
            metal,
            ..
        } = e
        {
            *layer = json!("ramp");
            *fit = "tile".into();
            *property = "luminance".into();
            *softness = 3.0;
            *cut_min = 20.0;
            *light_type = "point".into();
            *light_color = "#ffcc88".into();
            *metal = 60.0;
        }
    });
    t.taken(
        &mut document,
        "fx_plastic_001.json",
        vec![
            ("the ramp tiled, Luminance, softness 3, cut min 20, an orange point light, metal 60", holder(set(moved))),
            ("light position keyed from 0, 0 to 100, 100", holder(keys("light_position", &[(0, &[0.0, 0.0]), (4, &[100.0, 100.0])]))),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_plastic_001.json", 0),
        ("fx_plastic_006.json", 0),
        ("fx_plastic_012.json", 0),
        ("fx_plastic_013.json", 0),
        ("fx_plastic_016.json", 0),
        ("fx_plastic_019.json", 2),
        ("fx_plastic_020.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused ones (023 to 031) leave nothing for the card.
    let none: Vec<u32> = (23..=31).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({}), 3),
            (
                "layer4 as the bump, Luminance, softness 20, height -60, cut 20 to 80, a point light, dust 40, metal 100",
                json!({"layer": "layer-4", "property": "luminance", "softness": 20, "height": -60, "cut_min": 20, "cut_max": 80, "light_type": "point", "light_position": [20, 70], "light_height": 60, "dust": 40, "metal": 100}),
                3,
            ),
            (
                "Lightness, softness 0, cut 50 to 50, a blue ambient light, roughness 0.5",
                json!({"property": "lightness", "softness": 0, "cut_min": 50, "cut_max": 50, "ambient_light_color": "#4080ff", "roughness": 0.5}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-463 pictures/`");
    let dir = repo("verification/D-463 pictures");
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
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]), 0);
    write("1_before.png", TOWN, &before);
    t.row(
        "1_before.png, the street with no effect; draws cleanly",
        &format!("{said:?}"),
        said.is_empty(),
    );
    let (w, h) = TOWN;
    let coverings = |p: &[u8]| p.chunks_exact(4).map(|q| q[3]).collect::<Vec<_>>();
    let middle = |p: &[u8]| at(p, w, w / 2, h / 2).to_vec();

    let (added, said) = street(json!({}));
    write("2_as_added.png", TOWN, &added);
    t.row(
        "2_as_added.png, as added (its own covering as the bump, softness 10, height 50): the picture's edge rounded like a lit plastic tile, light from the top left, its covering exactly as before; draws cleanly",
        &format!("{said:?}, changed {}, coverings the same {}", added != before, coverings(&added) == coverings(&before)),
        said.is_empty() && added != before && coverings(&added) == coverings(&before),
    );

    let relief = json!({"property": "luminance", "softness": 4, "height": 60});
    let (lum, said) = street(relief.clone());
    write("3_luminance_relief.png", TOWN, &lum);
    t.row(
        "3_luminance_relief.png, Luminance, softness 4, height 60: the street's light and dark parts standing out as a shiny relief, not 2_as_added.png, the covering kept; draws cleanly",
        &format!("{said:?}, different {}, coverings the same {}", lum != added, coverings(&lum) == coverings(&before)),
        said.is_empty() && lum != added && coverings(&lum) == coverings(&before),
    );

    let mut metal_p = relief.clone();
    metal_p["metal"] = json!(100);
    metal_p["light_color"] = json!("#ffd080");
    let mut plastic_p = metal_p.clone();
    plastic_p["metal"] = json!(0);
    let (metal, said) = street(metal_p);
    let (orange, said2) = street(plastic_p);
    write("4_metal.png", TOWN, &metal);
    write("5_plastic_orange_light.png", TOWN, &orange);
    t.row(
        "4_metal.png and 5_plastic_orange_light.png, 3 under an orange light with Metal 100 and 0: the shine takes the street's own colours in 4 and the light's orange in 5, so they differ; draw cleanly",
        &format!("{said:?} {said2:?}, different {}", metal != orange),
        said.is_empty() && said2.is_empty() && metal != orange,
    );

    let (flat, said) = street(json!({"ambient": 100, "diffuse": 0, "specular": 0}));
    write("6_flat.png", TOWN, &flat);
    t.row(
        "6_flat.png, Ambient 100, Diffuse 0, Specular 0: the street unchanged within 1 level at its middle pixel; draws cleanly",
        &format!("{said:?}, middle {:?} against {:?}", middle(&flat), middle(&before)),
        said.is_empty() && middle(&flat).iter().zip(middle(&before)).all(|(a, b)| a.abs_diff(b) <= 1),
    );

    t.finish("D-463_plastic_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B343_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-343: a measurement, run deliberately with --release --ignored"]
fn b343_plastic_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B343_CPU").is_ok();
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
        ("Noise, then Plastic as added", Some(json!({}))),
        (
            "the same, Luminance, softness 30, a point light, metal 100",
            Some(
                json!({"property": "luminance", "softness": 30, "light_type": "point", "metal": 100}),
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
                v.push(fx(&format!("{id}b"), p));
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
    let out = std::env::var("B343_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-343_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
