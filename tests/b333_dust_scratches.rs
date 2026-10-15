//! B-333: D-453 Dust & Scratches, after After Effects' Dust & Scratches ("Noise & Grain" in
//! `docs/effects/EFFECTS.md`): D-203's Median with a threshold.
//!
//! Writes `verification/D-453_dust_scratches_table.md`.
//!
//! Every expected pixel is `Fixtures/dust_scratches/expected_dust_scratches.json`, written by
//! `tools/dust_scratches_reference.py` before this code existed and printed in document 25 as
//! FX-DUST-001 to 018. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-453 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, distance, keys, median, on_card, reference, repo, set, time_passes, town, Table, MAIN, TOWN,
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

fn dust(radius: f64, threshold: f64, operate_on_alpha: &str) -> Effect {
    Effect::DustScratches { radius, threshold, operate_on_alpha: operate_on_alpha.to_string() }
}

/// An effect of `type_id` with the settings `p`; for the ones here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.dust_scratches" => json!({"radius": 1, "threshold": 0, "operate_on_alpha": "off"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out, so the card is not asked.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(type_id, &format!("b333-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street at `frame` of a two-second shot, drawn, straight 8-bit, with what it
/// warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

type Shot<'a> = (&'a str, J, i32, &'a str, fn(&[u8], &[u8]) -> bool);

/// The street drawn plain and with each of `shots` of `type_id` at its frame, written as numbered
/// pictures into `verification/{d} pictures/`; a row each that it draws cleanly and changes what
/// it says it changes.
fn street(t: &mut Table, d: &str, type_id: &str, shots: &[Shot]) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    for (i, (name, p, frame, what, check)) in shots.iter().enumerate() {
        let file = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, json!([fx(type_id, "fx-0-0", p)]), *frame);
        write(&file, &bytes);
        t.row(
            &format!("{file}, frame {frame}, {what}; draws cleanly"),
            &format!("{said:?}, {} pixels changed of {}, the largest by {} of 255", distance(&bytes, &before).1, TOWN.0 * TOWN.1, distance(&bytes, &before).0),
            said.is_empty() && check(&bytes, &before),
        );
    }
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

fn files(stem: &str, count: u32) -> Vec<String> {
    (1..=count).map(|n| format!("{stem}_{n:03}.json")).collect()
}

// --- Dust & Scratches -----------------------------------------------------------------------

fn is_dust(e: &Effect) -> bool {
    matches!(e, Effect::DustScratches { .. })
}

/// Pixels changed, and the covering of every pixel kept.
fn cleaned(a: &[u8], b: &[u8]) -> (usize, bool) {
    let kept = a.chunks_exact(4).zip(b.chunks_exact(4)).all(|(p, q)| p[3] == q[3]);
    (distance(a, b).1, kept)
}

#[test]
fn b333_dust_scratches() {
    let mut t = Table::new(
        "dust_scratches",
        "# D-453: Dust & Scratches\n\nB-333, after After Effects' Dust & Scratches: D-203's Median \
         with a threshold. Each channel of a pixel's straight colour takes the median of the taps \
         within Radius that show when their 8-bit values (D-88's, through the sRGB curve) are more \
         than Threshold apart, and keeps its own otherwise; with Operate on Alpha on the covering \
         is worked the same way against the median of every tap's. So specks go and grain within \
         Threshold stays. Every expected pixel is \
         `Fixtures/dust_scratches/expected_dust_scratches.json`, written by \
         `tools/dust_scratches_reference.py` before this code existed and printed in document 25 \
         as FX-DUST-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-DUST-001 to 018 (document 25)");
    t.fixtures("expected_dust_scratches.json");

    t.heading("The file");
    let all = files("fx_dust", 18);
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_dust_009.json");
    t.row(
        "fx_dust_009.json is saved with its word and numbers as written, and the threshold's keys kept",
        &saved.to_string(),
        saved["radius"] == 2 && saved["operate_on_alpha"] == "off" && saved["threshold"]["keyframes"][1]["value"] == json!(16),
    );
    let saved = t.saved_parameters("fx_dust_007.json");
    t.row(
        "fx_dust_007.json is saved with its word and numbers as written",
        &saved.to_string(),
        saved["radius"] == 2 && saved["threshold"] == 16 && saved["operate_on_alpha"] == "on",
    );
    why(
        &mut t,
        &[
            ("fx_dust_012.json", "Dust & Scratches's radius runs from 0 to 10, and this is 11."),
            ("fx_dust_013.json", "Dust & Scratches's radius runs from 0 to 10, and this is -1."),
            ("fx_dust_015.json", "Dust & Scratches's threshold runs from 0 to 255, and this is 256."),
            ("fx_dust_016.json", "Dust & Scratches's threshold runs from 0 to 255, and this is -1."),
            ("fx_dust_018.json", "Dust & Scratches's operate on alpha is \"off\" or \"on\", and this is \"sometimes\"."),
        ],
    );
    t.shape_refused("fx_dust_001.json", "a Dust & Scratches with no `operate_on_alpha`", "{\"radius\": 1, \"threshold\": 0}");
    t.shape_refused("fx_dust_001.json", "a Dust & Scratches whose operate on alpha is a number", "{\"radius\": 1, \"threshold\": 0, \"operate_on_alpha\": 1}");
    t.shape_refused("fx_dust_001.json", "a Dust & Scratches with no `threshold`", "{\"radius\": 1, \"operate_on_alpha\": \"off\"}");

    t.heading("Commands");
    let mut document = t.load("fx_dust_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("radius 10.5", set(dust(10.5, 0.0, "off"))),
            ("radius -0.5", set(dust(-0.5, 0.0, "off"))),
            ("threshold 255.5", set(dust(1.0, 255.5, "off"))),
            ("threshold -2", set(dust(1.0, -2.0, "off"))),
            ("operate on alpha \"On\"", set(dust(1.0, 0.0, "On"))),
            ("threshold keyed to 400", keys("threshold", &[(0, &[0.0]), (4, &[400.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_dust_001.json",
        vec![
            ("radius 4, threshold 30, operate on alpha on", set(dust(4.0, 30.0, "on"))),
            ("radius keyed from 0 to 10", keys("radius", &[(0, &[0.0]), (4, &[10.0])])),
            ("threshold keyed from 0 to 255", keys("threshold", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_dust_001.json", 0),
        ("fx_dust_004.json", 0),
        ("fx_dust_005.json", 0),
        ("fx_dust_007.json", 0),
        ("fx_dust_009.json", 2),
        ("fx_dust_010.json", 4),
        ("fx_dust_011.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-DUST-002 is Radius 0 and 006 Threshold 255, which change nothing, and 012 to 018 are
    // refused, so the card is not asked.
    let mut none: Vec<u32> = vec![2, 6];
    none.extend(12..=18);
    card_fixtures(&mut t, &mut gpu, "fx_dust", 18, &none, is_dust);
    // Each at Radius 4 or more, so the reference shot's Draft (a quarter of its size) still reaches 1 and the card is asked.
    card_reference(
        &mut t,
        &mut gpu,
        "Dust & Scratches",
        "core.dust_scratches",
        &[
            ("Radius 4, Threshold 0", json!({"radius": 4})),
            ("Radius 6, Threshold 20", json!({"radius": 6, "threshold": 20})),
            ("Radius 4, Threshold 8, Operate on Alpha on", json!({"radius": 4, "threshold": 8, "operate_on_alpha": "on"})),
            ("Radius 4, Threshold keyed 0 to 60", json!({"radius": 4, "threshold": keyed(&[(0, json!(0)), (239, json!(60))])})),
        ],
        is_dust,
    );

    street(
        &mut t,
        "D-453",
        "core.dust_scratches",
        &[
            ("radius_2", json!({"radius": 2}), 0, "Radius 2, Threshold 0: fine detail evened with its neighbours, the covering kept", |a, b| {
                let (n, kept) = cleaned(a, b);
                n > 0 && kept
            }),
            ("radius_3", json!({"radius": 3}), 0, "Radius 3, Threshold 0: more evened, as Median at 3, the covering kept", |a, b| {
                let (n, kept) = cleaned(a, b);
                n > 0 && kept
            }),
            ("radius_3_threshold_40", json!({"radius": 3, "threshold": 40}), 0, "Radius 3, Threshold 40: only what stands out by more than 40 is changed, so far fewer pixels than at 0", |a, b| {
                let (n, kept) = cleaned(a, b);
                n > 0 && kept
            }),
            ("threshold_255", json!({"radius": 3, "threshold": 255}), 0, "Radius 3, Threshold 255: the street untouched", |a, b| distance(a, b).1 == 0),
        ],
    );
    let dir = repo("verification/D-453 pictures");
    let at = |p: J| picture(&dir, json!([fx("core.dust_scratches", "fx-0-0", &p)]), 0).0;
    let (before, _) = picture(&dir, json!([]), 0);
    let zero = distance(&at(json!({"radius": 3})), &before).1;
    let forty = distance(&at(json!({"radius": 3, "threshold": 40})), &before).1;
    t.row(
        "3_radius_3.png changes more pixels than 4_radius_3_threshold_40.png: the threshold keeps what is close to its neighbours",
        &format!("{zero} pixels changed at Threshold 0, {forty} at 40"),
        forty < zero,
    );

    t.finish("D-453_dust_scratches_table.md");
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B333_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-333: a measurement, run deliberately with --release --ignored"]
fn b333_dust_scratches_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B333_CPU").is_ok();
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
        ("Noise, then Dust & Scratches as added (Radius 1, Threshold 0)", Some(json!({}))),
        ("Noise, then Dust & Scratches Radius 3, Threshold 20", Some(json!({"radius": 3, "threshold": 20}))),
        ("Noise, then Dust & Scratches Radius 5, Threshold 8, Operate on Alpha on", Some(json!({"radius": 5, "threshold": 8, "operate_on_alpha": "on"}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some(p) = &e {
                v.push(fx("core.dust_scratches", &format!("{id}d"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B333_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-333_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
