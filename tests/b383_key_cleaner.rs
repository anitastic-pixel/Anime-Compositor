//! B-383: D-503 Key Cleaner, after After Effects' Key Cleaner ("Keying" in
//! `docs/effects/EFFECTS.md`): put after a key, it fits the covering near the keyed edge to the
//! picture's colours (D-352's Refine Soft Matte fit at the Additional Edge Radius), steepens the
//! part-covered pixels by Alpha Contrast and is backed off by Strength. Reduce Chatter is not
//! built.
//!
//! Writes `verification/D-503_key_cleaner_table.md`.
//!
//! Every expected pixel is `Fixtures/key_cleaner/expected_key_cleaner.json`, written by
//! `tools/key_cleaner_reference.py` before this code existed and printed in document 25 as
//! FX-KCLEAN-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-503 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN,
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

fn kc(additional_edge_radius: f64, alpha_contrast: f64, strength: f64) -> Effect {
    Effect::KeyCleaner {
        additional_edge_radius,
        alpha_contrast,
        strength,
    }
}

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.key_cleaner" => {
            json!({"additional_edge_radius": 10, "alpha_contrast": 0, "strength": 100})
        }
        "core.extract" => {
            json!({"channel": "luminance", "black_point": 0, "white_point": 255, "black_softness": 0,
            "white_softness": 0, "invert": "off"})
        }
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A luma key first (Extract: the darker pixels gone, a soft edge), then Key Cleaner with `p`.
fn keyed_then(id: &str, p: &J) -> J {
    json!([
        fx(
            "core.extract",
            &format!("{id}k"),
            &json!({"black_point": 70, "black_softness": 60})
        ),
        fx("core.key_cleaner", &format!("{id}c"), p)
    ])
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// cleaner is drawn on the processor (the card draws what follows it), so it is never on the card.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
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
                    matches!(e, Effect::KeyCleaner { .. })
                });
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 0,
        );
    }
}

/// The reference shot with each stack on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, the card's frame against the processor's.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, fn(&str) -> J)]) {
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
    for (what, stack) in settings {
        let project = reference(|id| stack(&format!("b383-{id}")));
        // Without the cleaner: the stack's other effects alone.
        let without = reference(|id| {
            let J::Array(mut v) = stack(&format!("b383-{id}")) else {
                unreachable!()
            };
            v.retain(|e| e["type_id"] != "core.key_cleaner");
            J::Array(v)
        });
        let changed = distance(&cpu(&project), &cpu(&without)).1;
        t.row(
            &format!("the reference shot, {what}: the processor's frame 100 differs from the shot without the Key Cleaner, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::KeyCleaner { .. })
                });
                t.row(
                    &format!("the reference shot, {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 0,
                );
            }
        }
    }
}

/// `effects` on the street, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(
        &fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap(),
    )
    .unwrap();
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

/// Whether some pixels changed but fewer than half.
fn some(a: &[u8], b: &[u8]) -> bool {
    let share = distance(a, b).1 as f64 / (a.len() / 4) as f64;
    share > 0.0 && share < 0.5
}

/// Every pixel's covering 0 or 255.
fn hard(a: &[u8], b: &[u8]) -> bool {
    some(a, b) && a.chunks_exact(4).all(|p| p[3] == 0 || p[3] == 255)
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc
            .project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("art"))
            .unwrap()
            .effects[0]
            .effect
            .why_invalid();
        t.row(
            &format!("{file} is refused in a sentence"),
            &why,
            why == *want,
        );
    }
}

#[test]
fn b383_key_cleaner() {
    let mut t = Table::new(
        "key_cleaner",
        "# D-503: Key Cleaner\n\nB-383, after After Effects' Key Cleaner, the rule ours (Adobe publishes \
         none): D-352's Refine Soft Matte fit at the Additional Edge Radius (its floor): over the edge \
         region, the pixels whose square of that radius holds a covered pixel and a pixel not fully \
         covered, the covering becomes the guided filter of the covering by the straight sRGB colours, \
         held inside 0 to 1; then Alpha Contrast steepens it, ramp(a, 0.5, 1 - contrast / 100); an empty \
         pixel that gains covering takes the covering-weighted average colour about it. Strength mixes \
         the result back toward the layer, premultiplied. Reduce Chatter, a smoothing over time, is not \
         built. The layer never grows. Every expected pixel is \
         `Fixtures/key_cleaner/expected_key_cleaner.json`, written by `tools/key_cleaner_reference.py` \
         before this code existed and printed in document 25 as FX-KCLEAN-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-KCLEAN-001 to 018 (document 25)");
    t.fixtures_numbered("expected_key_cleaner.json", 1..=18);

    t.heading("The file");
    let all: Vec<String> = (1..=18).map(|n| format!("fx_kclean_{n:03}.json")).collect();
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_kclean_001.json");
    t.row(
        "fx_kclean_001.json is saved with its three numbers as written",
        &saved.to_string(),
        saved == json!({"additional_edge_radius": 10, "alpha_contrast": 0, "strength": 100}),
    );
    why(
        &mut t,
        &[
            (
                "fx_kclean_012.json",
                "Key Cleaner's additional edge radius runs from 0 to 100, and this is 101.",
            ),
            (
                "fx_kclean_013.json",
                "Key Cleaner's additional edge radius runs from 0 to 100, and this is -1.",
            ),
            (
                "fx_kclean_014.json",
                "Key Cleaner's alpha contrast runs from 0 to 100, and this is 101.",
            ),
            (
                "fx_kclean_015.json",
                "Key Cleaner's alpha contrast runs from 0 to 100, and this is -1.",
            ),
            (
                "fx_kclean_016.json",
                "Key Cleaner's strength runs from 0 to 100, and this is 101.",
            ),
            (
                "fx_kclean_017.json",
                "Key Cleaner's strength runs from 0 to 100, and this is -1.",
            ),
        ],
    );
    t.shape_refused(
        "fx_kclean_001.json",
        "a Key Cleaner with no `strength`",
        r#"{"additional_edge_radius": 10, "alpha_contrast": 0}"#,
    );
    t.shape_refused(
        "fx_kclean_001.json",
        "a Key Cleaner with a radius in words",
        r#"{"additional_edge_radius": "ten", "alpha_contrast": 0, "strength": 100}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_kclean_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("additional edge radius 101", set(kc(101.0, 0.0, 100.0))),
            ("alpha contrast -1", set(kc(10.0, -1.0, 100.0))),
            ("strength 101", set(kc(10.0, 0.0, 101.0))),
            (
                "strength keyed to 101",
                keys("strength", &[(0, &[100.0]), (4, &[101.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_kclean_001.json",
        vec![
            (
                "radius 4, contrast 30, strength 75",
                set(kc(4.0, 30.0, 75.0)),
            ),
            (
                "additional edge radius keyed from 0 to 6",
                keys("additional_edge_radius", &[(0, &[0.0]), (4, &[6.0])]),
            ),
        ],
    );
    // Draft halves the radius, a distance; the contrast and strength stay.
    let mut draft = kc(8.0, 30.0, 75.0);
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview halves Additional Edge Radius and keeps Alpha Contrast and Strength",
        &format!("{draft:?}"),
        draft == kc(4.0, 30.0, 75.0),
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_kclean_001.json", 0),
        ("fx_kclean_005.json", 0),
        ("fx_kclean_007.json", 0),
        ("fx_kclean_009.json", 3),
        ("fx_kclean_011.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255 (the cleaner is drawn on the processor; the card draws what follows it)");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_kclean", 18);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Key Cleaner as added on the cels", |id| {
                json!([fx("core.key_cleaner", id, &json!({}))])
            }),
            (
                "Extract (black point 70, softness 60), then Key Cleaner as added",
                |id| keyed_then(id, &json!({})),
            ),
            (
                "Extract, then Key Cleaner radius 4, contrast 40, strength 60",
                |id| {
                    keyed_then(
                        id,
                        &json!({"additional_edge_radius": 4, "alpha_contrast": 40, "strength": 60}),
                    )
                },
            ),
        ],
    );

    t.heading("Pictures: the street, in `verification/D-503 pictures/`");
    let dir = repo("verification/D-503 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| {
        png_out::write_rgba(
            &dir.join(name),
            TOWN.0,
            TOWN.1,
            OutputDepth::Eight,
            &[],
            bytes,
        )
        .unwrap()
    };
    write("town.png", &town());
    let (keyed, said) = picture(
        &dir,
        json!([fx(
            "core.extract",
            "k",
            &json!({"black_point": 70, "black_softness": 60})
        )]),
    );
    write("1_keyed.png", &keyed);
    t.row("1_keyed.png, the street keyed by Extract (black point 70, softness 60), no cleaner; draws cleanly", &format!("{said:?}"), said.is_empty());
    let shots: [(&str, J, &str, fn(&[u8], &[u8]) -> bool); 4] = [
        ("cleaned", json!({}), "then Key Cleaner as added (radius 10): the soft edges of the key fitted to the street's colours", some),
        ("radius_3", json!({"additional_edge_radius": 3}), "then Key Cleaner radius 3: a tighter fit", some),
        ("contrast_100", json!({"alpha_contrast": 100}), "then Key Cleaner with Alpha Contrast 100: every pixel kept or gone", hard),
        ("strength_50", json!({"strength": 50}), "then Key Cleaner at Strength 50: halfway back to the key alone", some),
    ];
    for (i, (name, p, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, keyed_then("s", p));
        write(&file, &bytes);
        t.row(
            &format!("{file}, {what}; draws cleanly"),
            &format!(
                "{said:?}, {} pixels changed of {}",
                distance(&bytes, &keyed).1,
                TOWN.0 * TOWN.1
            ),
            said.is_empty() && check(&bytes, &keyed),
        );
    }

    t.finish("D-503_key_cleaner_table.md");
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B383_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-383: a measurement, run deliberately with --release --ignored"]
fn b383_key_cleaner_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B383_CPU").is_ok();
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
        (
            "Noise, then Key Cleaner as added (radius 10)",
            Some(json!({})),
        ),
        (
            "Noise, then Key Cleaner radius 4, contrast 40",
            Some(json!({"additional_edge_radius": 4, "alpha_contrast": 40})),
        ),
        (
            "Noise, then Key Cleaner radius 30, strength 60",
            Some(json!({"additional_edge_radius": 30, "strength": 60})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx(
                "core.noise",
                &format!("{id}n"),
                &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}),
            )];
            if let Some(p) = &e {
                v.push(fx("core.key_cleaner", &format!("{id}c"), p));
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
    let out = std::env::var("B383_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-383_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
