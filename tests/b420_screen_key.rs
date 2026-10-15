//! B-420: D-540, Screen Key, this program's own green and blue screen keyer (roadmap item L-03,
//! un-parked by the owner on 2026-10-11): a colour difference matte against a picked screen
//! colour, clip levels, despill, shrink or grow, softness and an edge refine. Not a copy of
//! Keylight: the rule is ours, stated in `tools/screen_key_reference.py`.
//!
//! Writes `verification/D-540_screen_key_table.md` and draws pictures into
//! `verification/D-540 pictures/`.
//!
//! Every expected pixel is `Fixtures/screen_key/expected_screen_key.json`, written by the tool
//! before this code existed and printed in document 25 as FX-SKEY-001 to 024. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, Table, MAIN, TOWN,
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

/// A Screen Key with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"view": "final", "screen_color": "#00ff00", "screen_gain": 100, "screen_balance": 50, "clip_black": 0,
        "clip_white": 100, "despill_bias": 50, "despill": 100, "shrink_grow": 0, "softness": 0, "edge_refine": 0});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.screen_key", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=24 {
        let file = format!("fx_skey_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::ScreenKey { .. }));
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
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
        let project = reference(|id| json!([fx(&format!("b420-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Screen Key {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::ScreenKey { .. }));
                t.row(
                    &format!("the reference shot, Screen Key {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
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

const SCREEN: [u8; 3] = [40, 180, 60];
const SHIRT: [u8; 3] = [40, 60, 160];

/// A green-screen shot at the street's size: a lit green screen, a face whose outer 14 pixels
/// take on more and more of the screen's green (the spill), and a blue shirt below. Color
/// Difference Key's (B-339).
fn portrait() -> Vec<u8> {
    let (w, h) = TOWN;
    let skin = [214.0, 172.0, 140.0];
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 - 240.0, y as f64 - 110.0);
            let r = (dx * dx / (70.0 * 70.0) + dy * dy / (90.0 * 90.0)).sqrt();
            let c = if y >= 200 && (130..350).contains(&x) {
                SHIRT
            } else if r < 1.0 {
                // The spill, from none 14 pixels in to 60% of the screen at the edge.
                let spill = (1.0 - (1.0 - r) * 70.0 / 14.0).clamp(0.0, 1.0) * 0.6;
                std::array::from_fn(|c| (skin[c] + (SCREEN[c] as f64 - skin[c]) * spill).round() as u8)
            } else {
                SCREEN
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], x: usize, y: usize) -> &[u8] {
    let i = (y * TOWN.0 + x) * 4;
    &bytes[i..i + 4]
}

#[test]
fn b420_screen_key() {
    let mut t = Table::new(
        "screen_key",
        "# D-540: Screen Key\n\nB-420, this program's own green and blue screen keyer (roadmap item \
         L-03; not a copy of Keylight, whose code and text are licensed). A colour difference \
         matte against a picked screen colour, with gain and balance; clip levels; despill by \
         holding the screen channel to a blend of the other two; shrink or grow on Simple \
         Choker's disc, softness on document 21's Gaussian, and an edge refine by Key Cleaner's \
         fit; views Final Result, Screen Matte and Status. The rule is stated in \
         `tools/screen_key_reference.py`. Every expected pixel is \
         `Fixtures/screen_key/expected_screen_key.json`, written by the tool before this code \
         existed and printed in document 25 as FX-SKEY-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-SKEY-001 to 024 (document 25)");
    t.fixtures_numbered("expected_screen_key.json", 1..=24);

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_skey_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_skey_004.json");
    t.row(
        "fx_skey_004.json is saved with its eleven settings, view final, screen colour #28b43c and screen balance 0",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 11
            && saved["view"] == "final"
            && saved["screen_color"] == "#28b43c"
            && saved["screen_balance"] == 0.0,
    );
    for (file, want) in [
        ("fx_skey_020.json", "screen gain"),
        ("fx_skey_021.json", "clip black"),
        ("fx_skey_022.json", "shrink grow"),
        ("fx_skey_023.json", "view"),
        ("fx_skey_024.json", "screen colour"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_skey_001.json",
        "a Screen Key whose gain is a word",
        r##"{"view": "final", "screen_color": "#00ff00", "screen_gain": "lots", "screen_balance": 50, "clip_black": 0, "clip_white": 100, "despill_bias": 50, "despill": 100, "shrink_grow": 0, "softness": 0, "edge_refine": 0}"##,
    );
    t.shape_refused(
        "fx_skey_001.json",
        "a Screen Key without its screen colour",
        r##"{"view": "final", "screen_gain": 100, "screen_balance": 50, "clip_black": 0, "clip_white": 100, "despill_bias": 50, "despill": 100, "shrink_grow": 0, "softness": 0, "edge_refine": 0}"##,
    );
    t.shape_refused(
        "fx_skey_001.json",
        "a Screen Key without its edge refine",
        r##"{"view": "final", "screen_color": "#00ff00", "screen_gain": 100, "screen_balance": 50, "clip_black": 0, "clip_white": 100, "despill_bias": 50, "despill": 100, "shrink_grow": 0, "softness": 0}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_skey_001.json").document;
    let over = changed(&t, "fx_skey_001.json", |e| if let Effect::ScreenKey { screen_gain, .. } = e { *screen_gain = 201.0 });
    let word = changed(&t, "fx_skey_001.json", |e| if let Effect::ScreenKey { view, .. } = e { *view = "Status".into() });
    t.refused(
        &mut document,
        vec![
            ("screen gain 201", set(over)),
            ("view \"Status\"", set(word)),
            ("softness keyed to 300", keys("softness", &[(0, &[0.0]), (4, &[300.0])])),
        ],
    );
    let picked = changed(&t, "fx_skey_001.json", |e| {
        if let Effect::ScreenKey { screen_color, clip_white, .. } = e {
            *screen_color = "#28b43c".into();
            *clip_white = 70.0;
        }
    });
    t.taken(
        &mut document,
        "fx_skey_001.json",
        vec![
            ("the green screen's colour picked, clip white 70", set(picked)),
            ("clip black keyed from 0 to 40", keys("clip_black", &[(0, &[0.0]), (4, &[40.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_skey_002.json", 0),
        ("fx_skey_006.json", 0),
        ("fx_skey_011.json", 0),
        ("fx_skey_014.json", 0),
        ("fx_skey_015.json", 0),
        ("fx_skey_016.json", 0),
        ("fx_skey_017.json", 0),
        ("fx_skey_018.json", 2),
        ("fx_skey_019.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // Left to the processor: a hard clip (007), the status view (013), shrink or grow (014,
    // 015, 019), softness (016) and the edge refine (017); the refused ones (020 to 024) leave
    // nothing.
    let none: Vec<u32> = [7, 13, 14, 15, 16, 17, 19].into_iter().chain(20..=24).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("screen colour orange, gain 120, clip 10 to 90", json!({"screen_color": "#ff8000", "screen_gain": 120, "clip_black": 10, "clip_white": 90}), 3),
            ("screen colour sky blue, balance 95, the screen matte", json!({"view": "screen_matte", "screen_color": "#3070e0", "screen_balance": 95}), 3),
            ("screen colour orange, shrink 2 and softness 2, left to the processor", json!({"screen_color": "#ff8000", "shrink_grow": -2, "softness": 2}), 0),
        ],
    );

    t.heading("Pictures: in `verification/D-540 pictures/`");
    let dir = repo("verification/D-540 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, size: (usize, usize), bytes: &[u8]| png_out::write_rgba(&dir.join(name), size.0, size.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("portrait.png", TOWN, &portrait());
    let shot = |p: J| picture(&dir, "portrait.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "portrait.png", TOWN, json!([]));
    write("1_before.png", TOWN, &before);
    t.row("1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly", &format!("{said:?}"), said.is_empty());

    // The places to look: the screen, the face's middle (no spill), a spilled pixel near the
    // face's top edge, (156, 175, 113), the shirt.
    let (screen, face, edge, shirt) = ((20, 20), (240, 110), (240, 28), (240, 250));
    let cover = |b: &[u8], p: (usize, usize)| at(b, p.0, p.1)[3];
    let px = |b: &[u8], p: (usize, usize)| at(b, p.0, p.1).to_vec();
    let near = |a: &[u8], b: [u8; 4]| a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 1);

    // By hand: the screen's green stands 180 - (40 + 60) / 2 = 130 above, pure green's 255, so
    // as added it keeps 1 - 130 / 255 of its covering, 125.
    let (after, said) = shot(json!({}));
    write("2_as_added.png", TOWN, &after);
    let (s, f, sh) = (cover(&after, screen), cover(&after, face), cover(&after, shirt));
    t.row(
        "2_as_added.png, as added (screen colour pure green): the screen, less saturated than pure green, half see-through (covering 125 by hand); the face and the shirt whole; draws cleanly",
        &format!("{said:?}, screen {s}, face {f}, shirt {sh}"),
        said.is_empty() && s.abs_diff(125) <= 1 && f == 255 && sh == 255,
    );

    // By hand at the spilled pixel (156, 175, 113): its green stands 175 - 134.5 = 40.5 above,
    // the screen's 130, so it keeps 1 - 40.5 / 130 = 0.688 of its covering, 175; the despill
    // holds its green to the average of red and blue, 134.5.
    let picked = json!({"screen_color": "#28b43c"});
    let (after, said) = shot(picked.clone());
    write("3_picked.png", TOWN, &after);
    let (s, f, sh, e) = (cover(&after, screen), px(&after, face), px(&after, shirt), px(&after, edge));
    t.row(
        "3_picked.png, the screen colour picked from the screen: the screen see-through; the face's middle and the shirt untouched; the spilled edge pixel partly see-through (covering 175 by hand) with its green spill taken out (green 134 or 135 by hand, red 156 and blue 113 kept); draws cleanly",
        &format!("{said:?}, screen {s}, face {f:?}, shirt {sh:?}, spilled {e:?}"),
        said.is_empty()
            && s == 0
            && near(&f, [214, 172, 140, 255])
            && near(&sh, [SHIRT[0], SHIRT[1], SHIRT[2], 255])
            && e[3].abs_diff(175) <= 1
            && e[0].abs_diff(156) <= 1
            && e[1].abs_diff(135) <= 1
            && e[2].abs_diff(113) <= 1,
    );

    // By hand: clip 30 to 70 stretches 0.688 to (0.688 - 0.3) / 0.4 = 0.97, covering 247.
    let mut clipped = picked.clone();
    clipped["clip_black"] = J::from(30);
    clipped["clip_white"] = J::from(70);
    let (after, said) = shot(clipped.clone());
    write("4_clipped.png", TOWN, &after);
    let (s, e) = (cover(&after, screen), cover(&after, edge));
    t.row(
        "4_clipped.png, clip black 30 and clip white 70: the screen see-through, the spilled edge pixel nearly solid (covering 247 by hand); draws cleanly",
        &format!("{said:?}, screen {s}, spilled {e}"),
        said.is_empty() && s == 0 && e.abs_diff(247) <= 2,
    );

    let mut matte = clipped.clone();
    matte["view"] = J::from("screen_matte");
    let (after, said) = shot(matte);
    write("5_screen_matte.png", TOWN, &after);
    let (s, f, sh) = (px(&after, screen), px(&after, face), px(&after, shirt));
    t.row(
        "5_screen_matte.png, the same as View Screen Matte: the screen black, the face and the shirt white, all fully covered; draws cleanly",
        &format!("{said:?}, screen {s:?}, face {f:?}, shirt {sh:?}"),
        said.is_empty() && s == [0, 0, 0, 255] && f == [255, 255, 255, 255] && sh == [255, 255, 255, 255],
    );

    let mut status = clipped.clone();
    status["view"] = J::from("status");
    let (after, said) = shot(status);
    write("6_status.png", TOWN, &after);
    let (s, f, e) = (px(&after, screen), px(&after, face), px(&after, edge));
    t.row(
        "6_status.png, the same as View Status: the screen black, the face white, the spilled edge pixel (matte 0.97) the one grey, encoded 128; draws cleanly",
        &format!("{said:?}, screen {s:?}, face {f:?}, spilled {e:?}"),
        said.is_empty() && s == [0, 0, 0, 255] && f == [255, 255, 255, 255] && e[0].abs_diff(128) <= 1 && e[3] == 255,
    );

    let mut edged = clipped.clone();
    edged["shrink_grow"] = J::from(-2);
    edged["softness"] = J::from(1.5);
    edged["edge_refine"] = J::from(3);
    let (after, said) = shot(edged);
    write("7_edge.png", TOWN, &after);
    let (s, f, sh) = (cover(&after, screen), px(&after, face), cover(&after, shirt));
    t.row(
        "7_edge.png, then shrink 2 pixels, softness 1.5 and edge refine 3: the screen still see-through, the face's middle and the shirt still whole, the face's edge pulled in and softened; draws cleanly",
        &format!("{said:?}, screen {s}, face {f:?}, shirt {sh}"),
        said.is_empty() && s == 0 && near(&f, [214, 172, 140, 255]) && sh == 255,
    );

    t.finish("D-540_screen_key_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B420_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-420: a measurement, run deliberately with --release --ignored"]
fn b420_screen_key_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B420_CPU").is_ok();
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
        ("Noise, then Screen Key, screen colour orange, gain 120, clip 10 to 90 (on the card)", Some(json!({"screen_color": "#ff8000", "screen_gain": 120, "clip_black": 10, "clip_white": 90}))),
        ("Noise, then Screen Key, the same as the screen matte (on the card)", Some(json!({"view": "screen_matte", "screen_color": "#ff8000", "screen_gain": 120, "clip_black": 10, "clip_white": 90}))),
        ("Noise, then Screen Key, the same with shrink 2, softness 1.5 and edge refine 3 (the processor)", Some(json!({"screen_color": "#ff8000", "screen_gain": 120, "clip_black": 10, "clip_white": 90, "shrink_grow": -2, "softness": 1.5, "edge_refine": 3}))),
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
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B420_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-420_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
