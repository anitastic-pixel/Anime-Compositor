//! B-426: D-546, pick #15 (EFFECTS.md row 262), Glint Streaks: Cross Glare gains a colour per
//! ray (one colour, two alternating, or a gradient round the rays) and shimmer, each ray dimming
//! and coming back on its own beat, as Glint Streaks merged into Cross Glare rather than a new
//! effect.
//!
//! Every expected pixel is `Fixtures/glint_streaks/expected_glint_streaks.json`, written by
//! `tools/glint_streaks_reference.py` before this code existed. D-161's fixtures are run again,
//! unchanged, since Cross Glare must draw as before. The rays are drawn on the card too, and
//! checked against the processor within 1 level of 255 (ADR-006, D-100).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, median, said, set, time_passes, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Document;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn effect_of(d: &Document) -> Effect {
    let comp = d.project().composition(&Id::new(MAIN)).unwrap();
    comp.layer(&Id::new("art")).unwrap().effects[0]
        .effect
        .clone()
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        // B-233's reason: a pixel clear in both has no colour to compare.
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p
            .iter()
            .zip(q)
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

/// A Cross Glare as added with `changes` laid over it.
fn glare(changes: J) -> J {
    let mut p = json!({"threshold": 80, "length": 40, "points": 4, "angle": 45, "intensity": 1, "color": "#ffffff"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    p
}

/// The reference shot with Cross Glare of `parameters` on its first three layers.
fn reference(parameters: &J) -> Project {
    let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json"))
        .expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = json!([{"instance_id": format!("b426-{id}"), "type_id": "core.cross_glare", "enabled": true, "parameters": parameters}]);
    }
    persist::load_str(&j.to_string())
        .unwrap_or_else(|d| panic!("the reference shot: {}", d.message))
        .document
        .project()
        .clone()
}

fn on_card(
    project: &Project,
    comp: &Id,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(
        project,
        comp,
        frame,
        root,
        quality,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c, render::OnCard::Fx(f) if matches!(f.instance.effect, Effect::CrossGlare { .. })))
        .count()
}

fn both(
    gpu: &mut Gpu,
    project: &Project,
    comp: &Id,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(
        project,
        comp,
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(
        project,
        comp,
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut cache,
        gpu,
    )
    .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (
        distance(&c.to_srgb8_straight(), &g),
        refused,
        said_cpu,
        said_gpu,
    )
}

/// B-233's framed town: the town (480 by 270) in a composition 160 pixels wider and taller.
fn framed(effects: J) -> Project {
    let (w, h) = TOWN;
    let t = |v: J| json!({"base": v, "keyframes": []});
    let project = json!({
        "schema_version": 0, "project_id": "proj-b426",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-town", "kind": "still", "name": "town", "path": "town.png",
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w + 160, "height": h + 160, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24},
            "layer_order": ["art"],
            "layers": [{"id": "art", "kind": "raster", "name": "art", "asset_id": "asset-town", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": 24, "source_offset_frames": 0,
                "transform": {"anchor": t(json!([0, 0])), "position": t(json!([80, 80])), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string())
        .unwrap_or_else(|d| panic!("the framed town opens: {}", d.message))
        .document
        .project()
        .clone()
}

#[test]
fn b426_glint_streaks() {
    let mut t = Table::new(
        "cross_glare",
        "# B-426: Glint Streaks (pick #15)\n\n\
         D-546 (EFFECTS.md row 262): Cross Glare gains Glint Streaks' controls. Ray Colours: one \
         colour, as before; Alternate, every other ray in the Second Colour; or Gradient, the \
         rays going from the Colour to the Second Colour round the star. Shimmer: each ray dims \
         and comes back over Shimmer Period frames, each on its own beat, as the rays of a real \
         glint flicker; Shimmer 0 holds them still. Cross Glare draws as before with one colour \
         and no shimmer, so D-161's 29 fixtures are run again here unchanged. Every expected \
         pixel is `Fixtures/glint_streaks/expected_glint_streaks.json`, written by \
         `tools/glint_streaks_reference.py` before this code existed (FX-GLINT-001 to 020), \
         compared sample by sample against the catalogue's tolerance of 2e-5. The rays are drawn \
         on the card too: the card's picture against the processor's, within 1 level of 255 \
         (ADR-006, D-100).\n",
    );

    t.heading("D-161's FX-GLARE-001 to 029, unchanged: Cross Glare draws as before");
    t.fixtures("expected_cross_glare.json");
    t.round_trips(&[
        "fx_glare_001.json",
        "fx_glare_005.json",
        "fx_glare_015.json",
    ]);

    t.root = effect_table::repo("Fixtures/glint_streaks");
    t.heading("FX-GLINT-001 to 020");
    t.fixtures("expected_glint_streaks.json");

    t.heading("The file");
    // 008 writes its second colour in capitals; the colour is lowercased on reading, as
    // Cross Glare's own colour is (b104 leaves its capitals fixture, 016, out the same way).
    let files: Vec<String> = (1..=20)
        .filter(|&n| n != 8)
        .map(|n| format!("fx_glint_{n:03}.json"))
        .collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p1 = t.saved_parameters("fx_glint_001.json");
    t.shape_refused("fx_glint_001.json", "a shimmer written as a word", &{
        let mut o = p1.clone();
        o["shimmer"] = json!("lots");
        o.to_string()
    });
    t.shape_refused("fx_glint_001.json", "ray colours written as a number", &{
        let mut o = p1.clone();
        o["ray_colors"] = json!(2);
        o.to_string()
    });
    for (file, said) in [
        ("fx_glint_015.json", "Cross Glare's shimmer runs from 0 to 100, and this is 101."),
        ("fx_glint_016.json", "Cross Glare's shimmer period runs from 1 to 1000, and this is 0.5."),
        ("fx_glint_017.json", "Cross Glare's shimmer period runs from 1 to 1000, and this is 1001."),
        ("fx_glint_018.json", "Cross Glare's ray colours is \"one\", \"alternate\" or \"gradient\", and this is \"rainbow\"."),
        ("fx_glint_019.json", "Cross Glare's second colour is written #rrggbb, and this is \"#12345\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming it"), &why, why == said);
    }

    t.heading("Commands");
    let mut document = t.load("fx_glint_001.json").document;
    let base = effect_of(&document);
    let with = |f: &dyn Fn(&mut Effect)| {
        let mut e = base.clone();
        f(&mut e);
        e
    };
    t.refused(
        &mut document,
        vec![
            (
                "shimmer 101",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.shimmer = 101.0
                    }
                })),
            ),
            (
                "shimmer period 0.5",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.shimmer_period = 0.5
                    }
                })),
            ),
            (
                "ray colours \"rainbow\"",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.ray_colors = "rainbow".into()
                    }
                })),
            ),
            (
                "second colour \"#12345\"",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.color_2 = "#12345".into()
                    }
                })),
            ),
            (
                "shimmer keyed to 120",
                keys("shimmer", &[(0, &[0.0]), (4, &[120.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_glint_001.json",
        vec![
            (
                "ray colours gradient, second colour #ff0000, shimmer 100, period 1000",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.ray_colors = "gradient".into();
                        glint.color_2 = "#ff0000".into();
                        glint.shimmer = 100.0;
                        glint.shimmer_period = 1000.0;
                    }
                })),
            ),
            (
                "ray colours alternate, shimmer period 1",
                set(with(&|e| {
                    if let Effect::CrossGlare { glint, .. } = e {
                        glint.ray_colors = "alternate".into();
                        glint.shimmer_period = 1.0;
                    }
                })),
            ),
            (
                "shimmer keyed 0 to 100 over frames 0 to 4",
                keys("shimmer", &[(0, &[0.0]), (4, &[100.0])]),
            ),
        ],
    );
    // D-68: a setting keyed is saved with its keys, even at its start value.
    let loaded = t.load("fx_glint_002.json");
    let mut keyed = loaded.document;
    keyed
        .apply(keys("shimmer_period", &[(0, &[24.0]), (4, &[48.0])]))
        .expect("shimmer period keyed");
    let back = persist::load_str(&persist::to_json(keyed.project(), &loaded.preserved))
        .expect("opened again")
        .document;
    // D-68: the keys and the constants; a keyed setting's unused base is not compared.
    let fx = |d: &Document| {
        d.project()
            .composition(&Id::new(MAIN))
            .unwrap()
            .layer(&Id::new("art"))
            .unwrap()
            .effects[0]
            .clone()
    };
    let (a, b) = (fx(&back), fx(&keyed));
    let kept = a.effect == b.effect
        && a.keys("shimmer_period").len() == 2
        && a.keys("shimmer_period") == b.keys("shimmer_period");
    t.row("shimmer period keyed 24 to 48 on a file without it, saved and opened again: the keys are kept", if kept { "kept" } else { "lost" }, kept);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_glint_002.json", 0),
        ("fx_glint_003.json", 0),
        ("fx_glint_006.json", 0),
        ("fx_glint_009.json", 2),
        ("fx_glint_014.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    let fixture_root = effect_table::repo("Fixtures/glint_streaks");
    for n in 1..=14 {
        let file = format!("fx_glint_{n:03}.json");
        let project = persist::load(&fixture_root.join(&file))
            .unwrap()
            .document
            .project()
            .clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..9 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &fixture_root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &fixture_root, frame, quality);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 8 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 18 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && card == 18,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = effect_table::repo("Fixtures/reference_shot");
    for (what, p) in [
        (
            "Cross Glare, ray colours alternate, second colour #ff8000",
            glare(json!({"ray_colors": "alternate", "color_2": "#ff8000"})),
        ),
        (
            "Cross Glare, six rays, gradient to #4060ff, shimmer 100, period 24",
            glare(
                json!({"points": 6, "ray_colors": "gradient", "color_2": "#4060ff", "shimmer": 100, "shimmer_period": 24}),
            ),
        ),
    ] {
        let project = reference(&p);
        let mut none = p.clone();
        none["ray_colors"] = json!("one");
        none["shimmer"] = json!(0);
        let plain = reference(&none);
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
        let changed = distance(&cpu(&project), &cpu(&plain)).1;
        t.row(
            &format!("the reference shot, {what}: the processor's frame 100 differs from the same with one colour and no shimmer, so the comparisons below test the new settings"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality);
                t.row(
                    &format!("the reference shot, {what}, on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the town, set in a larger frame, in `verification/D-546 pictures/`");
    let dir = effect_table::repo("verification/D-546 pictures");
    fs::create_dir_all(&dir).unwrap();
    let (w, h) = TOWN;
    png_out::write_rgba(
        &dir.join("town.png"),
        w,
        h,
        OutputDepth::Eight,
        &[],
        &effect_table::town(),
    )
    .unwrap();
    let (fw, fh) = (w + 160, h + 160);
    let draw = |effects: J, frame: i32| {
        let mut log = FrameLog::new(8);
        let b = render_frame(&framed(effects), &Id::new(MAIN), frame, &dir, 64, &mut log)
            .expect("the town draws");
        (
            b.to_srgb8_straight(),
            log.finish()
                .iter()
                .map(|d| d.id.as_str().to_string())
                .collect::<Vec<_>>(),
        )
    };
    let fx = |p: J| json!([{"instance_id": "fx-0-0", "type_id": "core.cross_glare", "enabled": true, "parameters": p}]);
    let lit = |changes: J| {
        let mut p = glare(json!({"threshold": 50, "length": 30}));
        for (k, v) in changes.as_object().unwrap() {
            p[k] = v.clone();
        }
        fx(p)
    };
    let pictures = [
        ("town_glare.png", "Cross Glare (threshold 50, length 30), one colour, white: for comparison", lit(json!({})), 0),
        ("town_alternate.png", "Ray Colours Alternate, the second colour orange #ff8000: every other ray orange", lit(json!({"ray_colors": "alternate", "color_2": "#ff8000"})), 0),
        ("town_gradient.png", "eight rays, Ray Colours Gradient from red #ff4040 to blue #4060ff: the rays go red to blue round the star",
            lit(json!({"points": 8, "color": "#ff4040", "ray_colors": "gradient", "color_2": "#4060ff"})), 0),
        ("town_shimmer_0.png", "Shimmer 100, period 24, frame 0: some rays bright, some dim", lit(json!({"shimmer": 100})), 0),
        ("town_shimmer_6.png", "the same at frame 6, a quarter period on: each ray has moved on its own beat", lit(json!({"shimmer": 100})), 6),
    ];
    let (bare, _) = draw(json!([]), 0);
    let mut drawn = Vec::new();
    for (file, what, effects, frame) in pictures {
        let (bytes, said) = draw(effects, frame);
        png_out::write_rgba(&dir.join(file), fw, fh, OutputDepth::Eight, &[], &bytes).unwrap();
        drawn.push((file, what, bytes, said));
    }
    for (i, (file, what, bytes, said)) in drawn.iter().enumerate() {
        // The first against the town without the effect, the last against the one before, the
        // rest against the first.
        let (against, other) = match i {
            0 => (&bare, "the town alone"),
            4 => (&drawn[3].2, "frame 0"),
            _ => (&drawn[0].2, "one colour and no shimmer"),
        };
        let changed = distance(bytes, against).1;
        t.row(
            &format!("{file}, {what}"),
            &format!("{changed} pixels differ from {other}; warnings {said:?}"),
            said.is_empty() && changed > 0,
        );
    }
    fs::remove_file(dir.join("town.png")).unwrap();

    t.finish("D-546_glint_streaks_table.md");
}

/// The frame times, B-233's way: the reference shot with a changing Noise on its first three
/// layers, then the effect, every eighth frame asked for as the viewer asks, with Draw on: GPU.
#[test]
#[ignore = "B-426: a measurement, run deliberately with --release --ignored"]
fn b426_glint_streaks_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        },
    );
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Cross Glare as added", Some(glare(json!({})))),
        (
            "Noise, then Cross Glare, ray colours alternate",
            Some(glare(json!({"ray_colors": "alternate"}))),
        ),
        (
            "Noise, then Cross Glare, gradient, shimmer 100",
            Some(glare(json!({"ray_colors": "gradient", "shimmer": 100}))),
        ),
    ];
    for (name, e) in shots {
        let stack = |id: &str| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true, "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(json!({"instance_id": format!("{id}g"), "type_id": "core.cross_glare", "enabled": true, "parameters": p}));
            }
            v
        };
        let text = fs::read_to_string(effect_table::repo("verification/B-08a_project.json"))
            .expect("read the reference shot");
        let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
        let layers = &mut j["compositions"][0]["layers"];
        for (i, id) in ["a", "b", "c"].iter().enumerate() {
            layers[i]["effects"] = J::Array(stack(id));
        }
        let loaded =
            persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{name}: {}", d.message));
        let (project, comp, root) = (
            loaded.document.project().clone(),
            Id::new("comp-reference-shot"),
            effect_table::repo("Fixtures/reference_shot"),
        );
        let (first, times) = time_passes(
            &mut gpu,
            false,
            8,
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
    let out = std::env::var("B426_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| effect_table::repo("verification/B-426_glint_streaks_timing_table.md"));
    fs::write(out, s).expect("write the timing table");
}
