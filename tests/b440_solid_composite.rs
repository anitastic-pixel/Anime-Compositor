//! B-440: D-560, a fixture folder for `core.solid_composite` (D-312, B-195), after After Effects'
//! Solid Composite (Channel). No code changes: the rule is document 21's, pinned here against
//! numbers worked a second way.
//!
//! Writes `verification/D-560_solid_composite_table.md` and draws pictures into
//! `verification/D-560 pictures/`.
//!
//! Every expected pixel is `Fixtures/solid_composite/expected_solid_composite.json`, written by
//! `tools/solid_composite_reference.py` before this code existed and printed in document 25 as
//! FX-SOLIDC-001 to 019. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
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

/// A Solid Composite with the settings `p`, every one `p` leaves out at its start.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"source_opacity": 100, "color": "#ffffff", "opacity": 100, "blend": "normal"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.solid_composite", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=19 {
        let file = format!("fx_solidc_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::SolidComposite { .. }));
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
        let project = reference(|id| json!([fx(&format!("b440-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Solid Composite {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::SolidComposite { .. }));
                t.row(
                    &format!("the reference shot, Solid Composite {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
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

/// The pixels of `before` and `after` side by side, channel by channel (alpha left out).
fn channels<'a>(before: &'a [u8], after: &'a [u8]) -> impl Iterator<Item = (i32, i32)> + 'a {
    before.chunks_exact(4).zip(after.chunks_exact(4)).flat_map(|(b, a)| (0..3).map(move |c| (b[c] as i32, a[c] as i32)))
}

#[test]
fn b440_solid_composite() {
    let mut t = Table::new(
        "solid_composite",
        "# D-560: Solid Composite, its fixture folder\n\nB-440: `core.solid_composite` (D-312, built \
         in B-195 from P-26's Advanced Electric tutorial) lays a layer, at its own opacity, on a \
         solid colour at another, blended normal, add, screen or multiply, after After Effects' \
         Solid Composite (Channel). B-195 pinned it on hand-worked solids; this unit gives it a \
         fixture folder. Every expected pixel is \
         `Fixtures/solid_composite/expected_solid_composite.json`, written by \
         `tools/solid_composite_reference.py` before this code existed and printed in document \
         25 as FX-SOLIDC-001 to 019. No code changed. Tolerance 2e-5.\n",
    );

    t.heading("FX-SOLIDC-001 to 019 (document 25)");
    t.fixtures_numbered("expected_solid_composite.json", 1..=19);

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_solidc_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_solidc_010.json");
    t.row(
        "fx_solidc_010.json is saved with its four settings",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 4
            && saved["source_opacity"] == 70.0
            && saved["color"] == "#ff8040"
            && saved["opacity"] == 60.0
            && saved["blend"] == "multiply",
    );
    for (file, want) in [
        ("fx_solidc_014.json", "opacity"),
        ("fx_solidc_015.json", "source_opacity"),
        ("fx_solidc_016.json", "blend is \"normal\", \"add\", \"screen\" or \"multiply\""),
        ("fx_solidc_017.json", "colour is written #rrggbb"),
        ("fx_solidc_018.json", "colour is written #rrggbb"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        let says = why.contains(want) || why.contains(&want.replace('_', " "));
        t.row(&format!("{file} is refused in a sentence saying {want}"), &why, says);
    }
    t.shape_refused("fx_solidc_001.json", "a Solid Composite whose opacity is a word", r##"{"source_opacity": 100, "color": "#ffffff", "opacity": "full", "blend": "normal"}"##);
    t.shape_refused("fx_solidc_001.json", "a Solid Composite without its blend", r##"{"source_opacity": 100, "color": "#ffffff", "opacity": 100}"##);

    t.heading("Commands");
    let mut document = t.load("fx_solidc_001.json").document;
    let edit = |f: fn(&mut String, &mut String)| {
        changed(&t, "fx_solidc_001.json", move |e| {
            if let Effect::SolidComposite { blend, color, .. } = e {
                f(blend, color);
            }
        })
    };
    let overlay = edit(|b, _| *b = "overlay".into());
    let red = edit(|_, c| *c = "red".into());
    let screen = edit(|b, _| *b = "screen".into());
    let blue = edit(|_, c| *c = "#3366cc".into());
    t.refused(&mut document, vec![("blend overlay", set(overlay)), ("colour red", set(red))]);
    t.taken(&mut document, "fx_solidc_001.json", vec![("blend screen", set(screen)), ("colour #3366cc", set(blue))]);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_solidc_001.json", 0),
        ("fx_solidc_003.json", 0),
        ("fx_solidc_007.json", 0),
        ("fx_solidc_009.json", 0),
        ("fx_solidc_010.json", 0),
        ("fx_solidc_013.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Refused (014 to 019) leave nothing for the card.
    let none: Vec<u32> = (14..=19).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("colour #3366cc, source opacity 50", json!({"color": "#3366cc", "source_opacity": 50})),
            ("multiply, colour #ff8040", json!({"blend": "multiply", "color": "#ff8040"})),
            ("add, colour #404040 at opacity 60", json!({"blend": "add", "color": "#404040", "opacity": 60})),
        ],
    );

    t.heading("Pictures: in `verification/D-560 pictures/`");
    let dir = repo("verification/D-560 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    let street = |p: J| picture(&dir, "town.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "town.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let (same, said) = street(json!({}));
    t.row(
        "the settings as they start (white, normal, both at 100) change nothing on the solid street: the street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&same, &before).1),
        said.is_empty() && same == before,
    );

    let (half, said) = street(json!({"color": "#3366cc", "source_opacity": 50}));
    write("2_half_over_blue.png", TOWN, &half);
    let between = channels(&before, &half).zip([0x33, 0x66, 0xcc].into_iter().cycle()).all(|((b, a), s)| a + 1 >= b.min(s) && a <= b.max(s) + 1);
    t.row(
        "2_half_over_blue.png, source opacity 50 over #3366cc: the street half seen through a blue; every channel between the street's and the blue's, within 1 level; draws cleanly",
        &format!("{said:?}, between: {between}"),
        said.is_empty() && between && half != before,
    );

    let (warm, said) = street(json!({"blend": "multiply", "color": "#ff8040"}));
    write("3_multiply_orange.png", TOWN, &warm);
    let darker = channels(&before, &warm).all(|(b, a)| a <= b + 1);
    let reds = before.chunks_exact(4).zip(warm.chunks_exact(4)).all(|(b, a)| a[0].abs_diff(b[0]) <= 1);
    t.row(
        "3_multiply_orange.png, multiply with #ff8040: the street warmer and darker, no channel lighter, the reds kept (the orange's red is full), within 1 level; draws cleanly",
        &format!("{said:?}, none lighter: {darker}, reds kept: {reds}"),
        said.is_empty() && darker && reds && warm != before,
    );

    let (pale, said) = street(json!({"blend": "screen", "color": "#808080"}));
    write("4_screen_grey.png", TOWN, &pale);
    let lighter = channels(&before, &pale).all(|(b, a)| a + 1 >= b.max(128));
    t.row(
        "4_screen_grey.png, screen with #808080: the street paler, every channel at least the street's and at least 128, within 1 level; draws cleanly",
        &format!("{said:?}, lighter: {lighter}"),
        said.is_empty() && lighter && pale != before,
    );

    t.finish("D-560_solid_composite_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B440_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-440: a measurement, run deliberately with --release --ignored"]
fn b440_solid_composite_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B440_CPU").is_ok();
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
        ("Noise, then Solid Composite, #3366cc, source opacity 50", Some(json!({"color": "#3366cc", "source_opacity": 50}))),
        ("Noise, then Solid Composite, multiply, #ff8040 at opacity 60", Some(json!({"blend": "multiply", "color": "#ff8040", "opacity": 60}))),
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
    let out = std::env::var("B440_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-440_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
