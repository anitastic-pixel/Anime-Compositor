//! B-338: D-458, after CycoreFX's CC Simple Wire Removal (Keying): a thin wire or rig between
//! two points painted out, by fading it clear (Fade), filling it from another frame of the layer
//! (Frame Offset), or covering it with the picture either side of it (Displace, and Displace
//! Horizontal along the rows), with Thickness, Slope and Mirror Blend. The Cycore FX manual names
//! the controls but publishes no formula: the rule is ours, stated in
//! `tools/simple_wire_removal_reference.py`.
//!
//! Writes `verification/D-458_simple_wire_removal_table.md` and draws pictures into
//! `verification/D-458 pictures/`.
//!
//! Every expected pixel is `Fixtures/simple_wire_removal/expected_simple_wire_removal.json`,
//! written by the tool before this code existed and printed in document 25 as FX-WIRE-001 to 030.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, on_card, reference, repo, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A CC Simple Wire Removal with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"point_a": [25, 50], "point_b": [75, 50], "removal_style": "displace", "thickness": 5,
        "slope": 50, "mirror_blend": 25, "frame_offset": -1});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.simple_wire_removal", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 3 at Full and Draft. The
/// files numbered in `none` leave nothing for the card on any frame.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=30 {
        let file = format!("fx_wire_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..4 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::SimpleWireRemoval { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 3 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 8 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` on the card at each of the three frames.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, [usize; 3])]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b338-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, CC Simple Wire Removal {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for (i, frame) in [0, 100, 239].into_iter().enumerate() {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::SimpleWireRemoval { .. }));
                t.row(
                    &format!("the reference shot, CC Simple Wire Removal {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == want[i],
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

/// The wire drawn across the street: a dark line 2 pixels wide from (60, 50) to (420, 140).
const FROM: (f64, f64) = (60.0, 50.0);
const TO: (f64, f64) = (420.0, 140.0);

/// How far the pixel (x, y)'s centre is from the wire's segment.
fn off_wire(x: usize, y: usize) -> f64 {
    let (px, py) = (x as f64 + 0.5 - FROM.0, y as f64 + 0.5 - FROM.1);
    let (vx, vy) = (TO.0 - FROM.0, TO.1 - FROM.1);
    let t = ((px * vx + py * vy) / (vx * vx + vy * vy)).clamp(0.0, 1.0);
    (px - t * vx).hypot(py - t * vy)
}

/// The street with the wire drawn on it.
fn wired() -> Vec<u8> {
    let (w, h) = TOWN;
    let mut bytes = town();
    for y in 0..h {
        for x in 0..w {
            if off_wire(x, y) <= 1.0 {
                bytes[(y * w + x) * 4..][..3].copy_from_slice(&[30, 30, 30]);
            }
        }
    }
    bytes
}

/// The mean, over the pixels the wire covers, of how far `a` is from the clean street, in levels.
fn on_the_wire(a: &[u8], clean: &[u8]) -> f64 {
    let (w, h) = TOWN;
    let (mut sum, mut n) = (0.0, 0);
    for y in 0..h {
        for x in 0..w {
            if off_wire(x, y) <= 1.0 {
                let i = (y * w + x) * 4;
                sum += (0..3).map(|c| a[i + c].abs_diff(clean[i + c]) as f64).sum::<f64>() / 3.0;
                n += 1;
            }
        }
    }
    sum / n as f64
}

/// The pixels more than `far` from the wire are the same in `a` and `b`.
fn far_kept(a: &[u8], b: &[u8], far: f64) -> bool {
    let (w, h) = TOWN;
    (0..h).all(|y| (0..w).filter(|&x| off_wire(x, y) > far).all(|x| a[(y * w + x) * 4..][..4] == b[(y * w + x) * 4..][..4]))
}

/// Set the `holder` layer's effect.
fn set(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-1"), effect }
}

/// Key a setting of the `holder` layer's effect, linearly.
fn keys(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-1"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

/// Each refused command on `file` is refused with a sentence and leaves it as it was; then each
/// taken one is taken, and as many undos give back `file`'s frame 0 exactly.
fn holder_commands(t: &mut Table, file: &str, refused: Vec<(&str, Command)>, taken: Vec<(&str, Command)>) {
    let mut document = t.load(file).document;
    let held = format!("{:?}", document.project());
    for (what, command) in refused {
        let why = document.apply(command).err();
        let untouched = format!("{:?}", document.project()) == held;
        t.row(&format!("{what} is refused with a sentence, and nothing changes"), &why.as_ref().map_or("taken".into(), |d| d.message.clone()), why.is_some() && untouched);
    }
    let n = taken.len();
    for (what, command) in taken {
        let ok = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if ok { "taken" } else { "refused" }, ok);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}

/// The `holder` layer's effect in `file`, changed by `f`.
fn changed(t: &Table, file: &str, f: impl FnOnce(&mut Effect)) -> Effect {
    let d = t.load(file).document;
    let mut e = d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].effect.clone();
    f(&mut e);
    e
}

#[test]
fn b338_simple_wire_removal() {
    let mut t = Table::new(
        "simple_wire_removal",
        "# D-458: CC Simple Wire Removal\n\nB-338, after CycoreFX's CC Simple Wire Removal \
         (Keying): a thin wire or rig between two points painted out. The Cycore FX manual names \
         the controls but publishes no formula, so the rule is ours, stated in \
         `tools/simple_wire_removal_reference.py`: a band Thickness wide along the wire, its edge \
         softened by Slope, becomes clear (Fade), the layer at another frame (Frame Offset), or \
         the picture either side of the band mixed across it with Mirror Blend of its reflection \
         (Displace; Displace Horizontal samples along the rows). Every expected pixel is \
         `Fixtures/simple_wire_removal/expected_simple_wire_removal.json`, written by the tool \
         before this code existed and printed in document 25 as FX-WIRE-001 to 030. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-WIRE-001 to 030 (document 25)");
    // The fixtures' composition is 4 frames long, so the warnings are read at its last frame.
    t.fixtures_warned_at("expected_simple_wire_removal.json", 1..=30, 3);

    t.heading("The file");
    let files: Vec<String> = (1..=30).map(|n| format!("fx_wire_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_wire_003.json");
    t.row(
        "fx_wire_003.json is saved with its seven settings, Frame Offset style and offset 1, and no picture",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 7 && saved["removal_style"] == "frame_offset" && saved["frame_offset"] == 1 && saved.get("picture").is_none(),
    );
    for (file, want) in [
        ("fx_wire_023.json", "removal style"),
        ("fx_wire_024.json", "removal style"),
        ("fx_wire_025.json", "thickness"),
        ("fx_wire_026.json", "slope"),
        ("fx_wire_027.json", "mirror blend"),
        ("fx_wire_028.json", "frame offset"),
        ("fx_wire_029.json", "point a"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.to_lowercase().contains(want));
    }
    let rest = r#""point_b": [75, 50], "slope": 50, "mirror_blend": 25, "frame_offset": -1"#;
    t.shape_refused("fx_wire_001.json", "a CC Simple Wire Removal whose thickness is a word", &format!(r#"{{"point_a": [25, 50], "removal_style": "displace", "thickness": "thin", {rest}}}"#));
    t.shape_refused("fx_wire_001.json", "a CC Simple Wire Removal without its removal style", &format!(r#"{{"point_a": [25, 50], "thickness": 5, {rest}}}"#));
    t.shape_refused("fx_wire_001.json", "a CC Simple Wire Removal whose point A is one number", &format!(r#"{{"point_a": [25], "removal_style": "displace", "thickness": 5, {rest}}}"#));

    t.heading("Commands");
    let over = changed(&t, "fx_wire_001.json", |e| if let Effect::SimpleWireRemoval { thickness, .. } = e { *thickness = 101.0 });
    let word = changed(&t, "fx_wire_001.json", |e| if let Effect::SimpleWireRemoval { removal_style, .. } = e { *removal_style = "clone".into() });
    let fade = changed(&t, "fx_wire_001.json", |e| if let Effect::SimpleWireRemoval { removal_style, .. } = e { *removal_style = "fade".into() });
    holder_commands(
        &mut t,
        "fx_wire_001.json",
        vec![("thickness 101", set(over)), ("removal style \"clone\"", set(word)), ("slope keyed to 150", keys("slope", &[(0, &[50.0]), (3, &[150.0])]))],
        vec![("Fade", set(fade)), ("thickness keyed from 0 to 10", keys("thickness", &[(0, &[0.0]), (3, &[10.0])]))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_wire_001.json", 0),
        ("fx_wire_002.json", 0),
        ("fx_wire_003.json", 0),
        ("fx_wire_008.json", 0),
        ("fx_wire_016.json", 0),
        ("fx_wire_017.json", 0),
        ("fx_wire_020.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Thickness 0 (011), A on B (012), Displace Horizontal on a flat wire (013), Frame Offset on
    // an adjustment layer (021) and refused (023 to 030) leave nothing for the card; an
    // adjustment layer's stack (022) is the card's too but is listed apart from the layers'.
    let none: Vec<u32> = [11, 12, 13, 21, 22].into_iter().chain(23..=30).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("Displace as added", json!({}), [3, 3, 3]),
            ("Fade, a slanting wire, thickness 12, slope 0", json!({"removal_style": "fade", "point_a": [10, 20], "point_b": [90, 70], "thickness": 12, "slope": 0}), [3, 3, 3]),
            (
                "Displace Horizontal, a slanting wire, thickness 9, mirror blend 70",
                json!({"removal_style": "displace_horizontal", "point_a": [30, 10], "point_b": [60, 95], "thickness": 9, "mirror_blend": 70}),
                [3, 3, 3],
            ),
            (
                "Frame Offset -1, corner to corner, thickness 60 (frame 0 has no frame before it)",
                json!({"removal_style": "frame_offset", "point_a": [0, 0], "point_b": [100, 100], "thickness": 60}),
                [0, 3, 3],
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-458 pictures/`");
    let dir = repo("verification/D-458 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", TOWN, &town());
    write("wired.png", TOWN, &wired());
    let (clean, _) = picture(&dir, "town.png", TOWN, json!([]));
    let (before, said) = picture(&dir, "wired.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    let wire_was = on_the_wire(&before, &clean);
    t.row(
        "1_before.png, the street with a dark wire drawn across it from (60, 50) to (420, 140); draws cleanly",
        &format!("{said:?}, the wire is {wire_was:.1} levels from the clean street on average"),
        said.is_empty() && wire_was > 40.0,
    );
    let along = json!({"point_a": [FROM.0 / 4.8, FROM.1 / 2.7], "point_b": [TO.0 / 4.8, TO.1 / 2.7], "thickness": 4, "slope": 50});
    let shot = |extra: J| {
        let mut p = along.clone();
        for (k, v) in extra.as_object().unwrap() {
            p[k] = v.clone();
        }
        picture(&dir, "wired.png", TOWN, json!([fx("fx-0-0", &p)]))
    };

    let (after, said) = shot(json!({}));
    write("2_displace.png", TOWN, &after);
    let left = on_the_wire(&after, &clean);
    t.row(
        "2_displace.png, Displace along the wire, thickness 4, slope 50: the wire painted over with the street either side of it, under a quarter of its difference from the clean street left; nothing more than 4 pixels (h + r) from it touched; draws cleanly",
        &format!("{said:?}, {left:.1} levels left of {wire_was:.1}, far pixels untouched: {}", far_kept(&after, &before, 4.0)),
        said.is_empty() && left < wire_was / 4.0 && far_kept(&after, &before, 4.0),
    );

    let (after, said) = shot(json!({"removal_style": "fade"}));
    write("3_fade.png", TOWN, &after);
    let gone = (0..TOWN.1).all(|y| (62..418).filter(|&x| off_wire(x, y) <= 1.0).all(|x| after[(y * TOWN.0 + x) * 4 + 3] == 0));
    t.row(
        "3_fade.png, Fade: the wire made clear (a see-through line where it was), with a soft edge; every wire pixel between its ends clear; nothing more than 4 pixels from it touched; draws cleanly",
        &format!("{said:?}, every wire pixel clear: {gone}, far pixels untouched: {}", far_kept(&after, &before, 4.0)),
        said.is_empty() && gone && far_kept(&after, &before, 4.0),
    );

    let (after, said) = shot(json!({"removal_style": "displace_horizontal"}));
    write("4_displace_horizontal.png", TOWN, &after);
    let left_h = on_the_wire(&after, &clean);
    t.row(
        "4_displace_horizontal.png, Displace Horizontal: the same wire painted over from the street left and right of it along each row; under a quarter of its difference left; draws cleanly",
        &format!("{said:?}, {left_h:.1} levels left of {wire_was:.1}"),
        said.is_empty() && left_h < wire_was / 4.0,
    );

    let (after, said) = shot(json!({"mirror_blend": 100}));
    write("5_mirror_100.png", TOWN, &after);
    let left_m = on_the_wire(&after, &clean);
    t.row(
        "5_mirror_100.png, Displace with mirror blend 100: the band filled with the street reflected across it; the wire still gone; draws cleanly",
        &format!("{said:?}, {left_m:.1} levels left of {wire_was:.1}"),
        said.is_empty() && left_m < wire_was / 4.0,
    );

    let (after, said) = shot(json!({"thickness": 0}));
    t.row(
        "thickness 0 changes nothing: the wired street byte for byte; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&after, &before).1),
        said.is_empty() && after == before,
    );

    // By hand, the rule on one pixel: a flat grey 8 by 8 with a white row 4; a level wire through
    // the middle of row 4, thickness 2, slope 100, Displace: the row becomes the grey above and below.
    let mut flat: Vec<u8> = [100u8, 100, 100, 255].repeat(64);
    for x in 0..8 {
        flat[(4 * 8 + x) * 4..][..3].copy_from_slice(&[255, 255, 255]);
    }
    write("flat.png", (8, 8), &flat);
    let (after, said) = picture(&dir, "flat.png", (8, 8), json!([fx("fx-0-0", &json!({"point_a": [0, 56.25], "point_b": [100, 56.25], "thickness": 2, "slope": 100}))]));
    let row: Vec<u8> = (0..8).map(|x| after[(4 * 8 + x) * 4]).collect();
    t.row(
        "by hand: a white row through flat grey 100, a level wire along it, thickness 2, slope 100, Displace: the row becomes 100",
        &format!("{said:?}, row 4 red: {row:?}"),
        said.is_empty() && row.iter().all(|v| v.abs_diff(100) <= 1),
    );

    t.finish("D-458_simple_wire_removal_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B338_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-338: a measurement, run deliberately with --release --ignored"]
fn b338_simple_wire_removal_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B338_CPU").is_ok();
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
        ("Noise, then CC Simple Wire Removal, Displace, thickness 20", Some(json!({"thickness": 20}))),
        ("Noise, then CC Simple Wire Removal, Frame Offset -1, thickness 20", Some(json!({"removal_style": "frame_offset", "thickness": 20}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}w"), p));
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
    let out = std::env::var("B338_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-338_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
