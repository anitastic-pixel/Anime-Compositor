//! B-446: D-567, After Effects' whole Fractal Type and Noise Type lists for Turbulent Noise
//! (D-445) and Fractal Noise (D-128, D-299), one engine: seventeen fractal types and five noise
//! types, each past the four already built this program's own rule after its name.
//!
//! Writes `verification/D-567_turbulent_noise_types_table.md` and draws pictures into
//! `verification/D-567 pictures/`.
//!
//! Every expected pixel is `Fixtures/turbulent_noise_types/expected_turbulent_noise_types.json`,
//! written by `tools/turbulent_noise_types_reference.py` before this code existed and printed in
//! document 25 as FX-TURBTYPE-001 to 025; Turbulent Noise's FX-TURBNOISE-001 to 032 (029 and 030
//! renamed under D-566) and Fractal Noise's FX-FRACTAL cases rerun unchanged. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, saved, set, time_passes, town,
    Table, MAIN, TOWN,
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

const TYPES: [&str; 17] = [
    "basic",
    "turbulent",
    "turbulent_smooth",
    "turbulent_sharp",
    "dynamic",
    "dynamic_progressive",
    "dynamic_twist",
    "max",
    "smeary",
    "swirly",
    "rocky",
    "cloudy",
    "terrain",
    "subscale",
    "small_bumps",
    "strings",
    "threads",
];
const NOISES: [&str; 5] = ["smooth", "block", "linear", "soft_linear", "spline"];

/// A Turbulent Noise's settings as it is added, with `p` over them.
fn settings(p: &J) -> J {
    let mut all = json!({"fractal_type": "basic", "noise_type": "smooth", "invert": "off", "contrast": 100, "brightness": 0,
        "size": 100, "scale_width": 100, "scale_height": 100, "offset": [0, 0], "complexity": 6, "evolution": 0, "seed": 0,
        "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    all
}

/// A Turbulent Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    json!({"instance_id": id, "type_id": "core.turbulent_noise", "enabled": true, "parameters": settings(p)})
}

/// A Fractal Noise with Turbulent Noise's settings `p` and Fractal Noise's own `extra` (no speed,
/// black to white, never cycling unless `extra` says).
fn fractal(id: &str, p: &J, extra: &J) -> J {
    let mut all = settings(p);
    // D-756: Fractal Noise works its cell at two thirds of its size, Turbulent Noise at its size.
    all["size"] = json!(all["size"].as_f64().unwrap() * 1.5);
    for (k, v) in [
        ("speed", json!(0)),
        ("dark_color", json!("#000000")),
        ("light_color", json!("#ffffff")),
        ("cycle", json!(0)),
    ] {
        all[k] = v;
    }
    // A null in `extra` leaves that setting out of the file.
    for (k, v) in extra.as_object().unwrap() {
        if v.is_null() {
            all.as_object_mut().unwrap().remove(k);
        } else {
            all[k] = v.clone();
        }
    }
    json!({"instance_id": id, "type_id": "core.fractal_noise", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=25 {
        let file = format!("fx_turbtype_{n:03}.json");
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
                    matches!(
                        e,
                        Effect::TurbulentNoise { .. } | Effect::FractalNoise { .. }
                    )
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

/// The reference shot with each stack on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, cases: &[(&str, J)]) {
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
    for (what, e) in cases {
        let project = reference(|id| {
            let mut e = e.clone();
            e["instance_id"] = J::from(format!("b446-{id}"));
            json!([e])
        });
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(
                        e,
                        Effect::TurbulentNoise { .. } | Effect::FractalNoise { .. }
                    )
                });
                t.row(
                    &format!("the reference shot, {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
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

/// The fixture `file` with its Turbulent Noise turned into the Fractal Noise it names, `extra`
/// over Fractal Noise's own settings, opened and saved again: the saved parameters and the effect.
fn as_fractal(t: &Table, file: &str, extra: &J) -> (J, Effect) {
    let mut j: J = serde_json::from_str(&fs::read_to_string(t.root.join(file)).unwrap()).unwrap();
    let e = &mut j["compositions"][0]["layers"][0]["effects"][0];
    let p = e["parameters"].clone();
    let plain = |v: &J| v.get("base").cloned().unwrap_or_else(|| v.clone());
    let p = json!(p
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.clone(), plain(v)))
        .collect::<serde_json::Map<_, _>>());
    *e = fractal("fx-0-0", &p, extra);
    let loaded = persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("{file} as Fractal Noise: {}", d.message));
    let effect = loaded
        .document
        .project()
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap()
        .effects[0]
        .effect
        .clone();
    (
        saved(&loaded)["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone(),
        effect,
    )
}

#[test]
fn b446_turbulent_noise_types() {
    let mut t = Table::new(
        "turbulent_noise_types",
        "# D-567: Turbulent Noise's full type lists\n\nB-446: Turbulent Noise (D-445) and Fractal \
         Noise (D-128, D-299), one engine, take After Effects' whole Fractal Type list (Basic, \
         Turbulent Basic, Turbulent Smooth, Turbulent Sharp, Dynamic, Dynamic Progressive, Dynamic \
         Twist, Max, Smeary, Swirly, Rocky, Cloudy, Terrain, Subscale, Small Bumps, Strings, \
         Threads) and Noise Type list (Block, Linear, Soft Linear, Spline, with this program's \
         Smooth). Adobe publishes no formulas, so each type past the four already built is this \
         program's own rule after its name (document 21). Every expected pixel is \
         `Fixtures/turbulent_noise_types/expected_turbulent_noise_types.json`, written by \
         `tools/turbulent_noise_types_reference.py` before this code existed and printed in \
         document 25 as FX-TURBTYPE-001 to 025; Turbulent Noise's FX-TURBNOISE-001 to 032 (029 \
         and 030 renamed under D-566) and Fractal Noise's FX-FRACTAL cases rerun unchanged. \
         Tolerance 2e-5.\n",
    );

    t.heading("FX-TURBTYPE-001 to 025 (document 25)");
    t.fixtures_numbered("expected_turbulent_noise_types.json", 1..=25);

    t.heading("Turbulent Noise's and Fractal Noise's own fixtures, unchanged");
    let root = t.root.clone();
    t.root = repo("Fixtures/turbulent_noise");
    t.fixtures_numbered("expected_turbulent_noise.json", 1..=32);
    t.root = repo("Fixtures/fractal_noise");
    // D-318 widened complexity and brightness, superseding 022 and 024, as b71 says.
    t.fixtures_numbered("expected_fractal_noise.json", 1..=21);
    t.fixtures_numbered("expected_fractal_noise.json", 23..=23);
    t.fixtures_numbered("expected_fractal_noise.json", 25..=28);
    t.root = root;

    t.heading("The file");
    let files: Vec<String> = (1..=25)
        .map(|n| format!("fx_turbtype_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    for (k, kind) in TYPES[2..].iter().enumerate() {
        let file = format!("fx_turbtype_{:03}.json", k + 1);
        let p = t.saved_parameters(&file);
        t.row(
            &format!("{file} is saved with fractal type \"{kind}\""),
            &p["fractal_type"].to_string(),
            p["fractal_type"] == *kind,
        );
    }
    for (file, noise) in [
        ("fx_turbtype_016.json", "linear"),
        ("fx_turbtype_017.json", "soft_linear"),
        ("fx_turbtype_018.json", "spline"),
    ] {
        let p = t.saved_parameters(file);
        t.row(
            &format!("{file} is saved with noise type \"{noise}\""),
            &p["noise_type"].to_string(),
            p["noise_type"] == noise,
        );
    }
    for (file, want) in [
        ("fx_turbtype_024.json", "fractal type"),
        ("fx_turbtype_025.json", "noise type"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming Turbulent Noise and its {want}"),
            &why,
            why.contains("Turbulent Noise") && why.contains(want),
        );
    }
    let why = changed(&t, "fx_turbtype_024.json", |_| {}).why_invalid();
    t.row(
        "the refusal lists all seventeen fractal types by the word written",
        &why,
        TYPES.iter().all(|w| why.contains(&format!("\"{w}\""))),
    );
    let why = changed(&t, "fx_turbtype_025.json", |_| {}).why_invalid();
    t.row(
        "the refusal lists all five noise types by the word written",
        &why,
        NOISES.iter().all(|w| why.contains(&format!("\"{w}\""))),
    );

    // Fractal Noise, the same engine: the new words are taken, saved, and a wrong one refused.
    let (p, e) = as_fractal(&t, "fx_turbtype_019.json", &json!({"cycle": 2}));
    t.row(
        "fx_turbtype_019.json as Fractal Noise (dynamic_twist on spline, cycle 2) is taken and saved with its words",
        &format!("{p}; valid: {}", e.is_valid()),
        e.is_valid() && p["fractal_type"] == "dynamic_twist" && p["noise_type"] == "spline" && p["cycle"] == 2.0,
    );
    let (_, e) = as_fractal(&t, "fx_turbtype_024.json", &json!({}));
    let why = e.why_invalid();
    t.row(
        "fx_turbtype_024.json as Fractal Noise is refused in Fractal Noise's name",
        &why,
        why.starts_with("Fractal Noise's fractal type is one of"),
    );
    let (p, e) = as_fractal(
        &t,
        "fx_turbtype_001.json",
        &json!({"fractal_type": null, "noise_type": null}),
    );
    let plain = matches!(&e, Effect::FractalNoise { fractal_type, noise_type, .. } if fractal_type == "basic" && noise_type == "smooth");
    t.row(
        "a Fractal Noise whose file leaves out both words opens at basic and smooth and saves neither word, as before (D-299)",
        &format!("{p}; basic and smooth: {plain}"),
        plain && p.get("fractal_type").is_none() && p.get("noise_type").is_none(),
    );

    t.heading("Commands");
    let mut document = t.load("fx_turbtype_001.json").document;
    let bad = changed(&t, "fx_turbtype_001.json", |e| {
        if let Effect::TurbulentNoise { fractal_type, .. } = e {
            *fractal_type = "plasma".into()
        }
    });
    t.refused(&mut document, vec![("fractal type plasma", set(bad))]);
    let threads = changed(&t, "fx_turbtype_001.json", |e| {
        if let Effect::TurbulentNoise {
            fractal_type,
            noise_type,
            ..
        } = e
        {
            *fractal_type = "threads".into();
            *noise_type = "spline".into();
        }
    });
    t.taken(
        &mut document,
        "fx_turbtype_001.json",
        vec![
            ("threads on spline", set(threads)),
            (
                "evolution keyed from 0 to 720",
                keys("evolution", &[(0, &[0.0]), (4, &[720.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_turbtype_005.json", 0),
        ("fx_turbtype_018.json", 0),
        ("fx_turbtype_019.json", 2),
        ("fx_turbtype_023.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, &[24, 25]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            (
                "Turbulent Noise dynamic_twist on spline, size 40, complexity 20",
                fx(
                    "x",
                    &json!({"fractal_type": "dynamic_twist", "noise_type": "spline", "size": 40, "complexity": 20}),
                ),
            ),
            (
                "Turbulent Noise rocky on block, multiply at opacity 60",
                fx(
                    "x",
                    &json!({"fractal_type": "rocky", "noise_type": "block", "size": 30, "blend": "multiply", "opacity": 60}),
                ),
            ),
            (
                "Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen",
                fractal(
                    "x",
                    &json!({"fractal_type": "terrain", "noise_type": "soft_linear", "size": 50, "blend": "screen", "opacity": 80}),
                    &json!({"speed": 12, "cycle": 2, "dark_color": "#102040", "light_color": "#ffd890"}),
                ),
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-567 pictures/`");
    let dir = repo("verification/D-567 pictures");
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
    let (basic, said) = street(json!({"size": 40}));
    write("t01_basic.png", TOWN, &basic);
    t.row(
        "t01_basic.png, fractal type basic at size 40, for comparing; draws cleanly",
        &format!("{said:?}"),
        said.is_empty(),
    );
    let mut seen = vec![basic.clone()];
    for (k, kind) in TYPES.iter().enumerate().skip(1) {
        let (img, said) = street(json!({"size": 40, "fractal_type": kind}));
        let name = format!("t{:02}_{kind}.png", k + 1);
        write(&name, TOWN, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, fractal type {kind} at size 40: clouds of their own, unlike every picture before; draws cleanly"), &format!("{said:?}, its own: {new}"), said.is_empty() && new);
    }
    for (k, noise) in NOISES.iter().enumerate().skip(1) {
        let (img, said) = street(json!({"size": 40, "noise_type": noise}));
        let name = format!("n{:02}_{noise}.png", k + 1);
        write(&name, TOWN, &img);
        let new = !seen.contains(&img);
        seen.push(img);
        t.row(&format!("{name}, noise type {noise} at size 40, fractal type basic: unlike every picture before; draws cleanly"), &format!("{said:?}, its own: {new}"), said.is_empty() && new);
    }

    // The second name: Fractal Noise with the same settings, no speed, black to white and never
    // cycling, draws the same street byte for byte, for new types too.
    for (what, p) in [
        (
            "dynamic_twist on spline",
            json!({"size": 30, "fractal_type": "dynamic_twist", "noise_type": "spline"}),
        ),
        (
            "rocky on block, inverted, seed 9",
            json!({"size": 30, "fractal_type": "rocky", "noise_type": "block", "invert": "on", "seed": 9}),
        ),
        (
            "strings on linear, evolution 200, screen at 50",
            json!({"size": 25, "fractal_type": "strings", "noise_type": "linear", "evolution": 200, "blend": "screen", "opacity": 50}),
        ),
    ] {
        let (a, said0) = street(p.clone());
        let (b, said1) = picture(
            &dir,
            "town.png",
            TOWN,
            json!([fractal("fx-0-0", &p, &json!({}))]),
            0,
        );
        t.row(
            &format!("Turbulent Noise {what} against Fractal Noise with the same settings: the same street byte for byte; both draw cleanly"),
            &format!("{said0:?} {said1:?}, {} pixels differ", distance(&a, &b).1),
            said0.is_empty() && said1.is_empty() && a == b && a != basic,
        );
    }
    // Fractal Noise's Cycle Evolution with a new type: cycle 2 repeats every two turns.
    let cycled = |evolution: f64| {
        let p = json!({"size": 30, "fractal_type": "dynamic_progressive", "noise_type": "spline", "evolution": evolution});
        picture(
            &dir,
            "town.png",
            TOWN,
            json!([fractal("fx-0-0", &p, &json!({"cycle": 2}))]),
            0,
        )
        .0
    };
    let (a, b, c) = (cycled(90.0), cycled(810.0), cycled(450.0));
    write("c1_cycle_2_at_90.png", TOWN, &a);
    t.row(
        "c1_cycle_2_at_90.png, Fractal Noise dynamic_progressive on spline with cycle 2: evolution 810 draws the same street, 450 does not",
        &format!("810 the same: {}, 450 the same: {}", a == b, a == c),
        a == b && a != c,
    );

    t.finish("D-567_turbulent_noise_types_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B446_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-446: a measurement, run deliberately with --release --ignored"]
fn b446_turbulent_noise_types_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B446_CPU").is_ok();
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
        ("Noise, then Turbulent Noise as added (basic, smooth, complexity 6)", Some(json!({}))),
        ("Noise, then Turbulent Noise dynamic_twist on soft_linear, complexity 6", Some(json!({"fractal_type": "dynamic_twist", "noise_type": "soft_linear"}))),
        (
            "Noise, then Turbulent Noise dynamic_twist on spline, size 40, complexity 20 (the most)",
            Some(json!({"fractal_type": "dynamic_twist", "noise_type": "spline", "size": 40, "complexity": 20})),
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
    let out = std::env::var("B446_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-446_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
