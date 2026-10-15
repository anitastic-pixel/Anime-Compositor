//! B-385: D-505, after After Effects' Unmult (Keying, new in After Effects 26.0): a black or
//! white background turned into transparency, so fire, smoke or a flare shot on black lays over
//! a shot cleanly. Adobe names the controls but publishes no formula and no defaults we found:
//! the rule and the settings when added are ours, stated in `tools/unmult_reference.py`.
//!
//! Writes `verification/D-505_unmult_table.md` and draws pictures into
//! `verification/D-505 pictures/`.
//!
//! Every expected pixel is `Fixtures/unmult/expected_unmult.json`, written by the tool before
//! this code existed and printed in document 25 as FX-UNMULT-001 to 019. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, keys, median, on_card, reference, repo, set, time_passes, Table, MAIN,
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

/// An Unmult with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"background_color": "black", "black_level": 0, "white_level": 100, "softness": 0,
        "remove_color_matting": "on", "clip_hdr_results": "off"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.unmult", "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=19 {
        let file = format!("fx_unmult_{n:03}.json");
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
                    matches!(e, Effect::Unmult { .. })
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
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
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
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b385-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Unmult {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| {
                    matches!(e, Effect::Unmult { .. })
                });
                t.row(
                    &format!("the reference shot, Unmult {what} on three layers, frame {frame}, {}", quality.label()),
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

const ORANGE: [f64; 3] = [255.0, 160.0, 60.0];
const GREY: (usize, usize) = (50, 220);

/// A shot at the street's size on a plain `back`ground: in the middle a soft glow (white at its
/// core when `hot`), falling through `glow` to the background by 110 pixels out; and a mid-grey (128)
/// square at the lower left.
fn shot_on(back: f64, glow: [f64; 3], hot: bool) -> Vec<u8> {
    let (w, h) = TOWN;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let (dx, dy) = (x as f64 - 240.0, y as f64 - 135.0);
            let r = (dx * dx + dy * dy).sqrt();
            let g = (1.0 - r / 110.0).clamp(0.0, 1.0).powi(2);
            let core = if hot {
                (1.0 - r / 22.0).clamp(0.0, 1.0)
            } else {
                0.0
            };
            let c: [u8; 3] = if x.abs_diff(GREY.0) < 20 && y.abs_diff(GREY.1) < 20 {
                [128; 3]
            } else {
                std::array::from_fn(|c| {
                    let v = back + (glow[c] - back) * g;
                    (v + (255.0 - v) * core).round() as u8
                })
            };
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

fn to_linear(v: f64) -> f64 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn to_srgb(v: f64) -> f64 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

/// The straight 8-bit picture laid over a sky (blue at the top to pale at the bottom), in
/// linear light, for the eye only: what the Unmult is for.
fn over_sky(bytes: &[u8]) -> Vec<u8> {
    let (w, h) = TOWN;
    let mut out = Vec::with_capacity(bytes.len());
    for (i, p) in bytes.chunks_exact(4).enumerate() {
        let t = (i / w) as f64 / (h - 1) as f64;
        let sky = [40.0 + 120.0 * t, 90.0 + 110.0 * t, 170.0 + 60.0 * t];
        let a = p[3] as f64 / 255.0;
        for c in 0..3 {
            let v = to_linear(p[c] as f64 / 255.0) * a + to_linear(sky[c] / 255.0) * (1.0 - a);
            out.push((to_srgb(v) * 255.0).round() as u8);
        }
        out.push(255);
    }
    out
}

/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}

fn near(a: [u8; 4], b: [u8; 4]) -> bool {
    a.iter().zip(b).all(|(x, y)| x.abs_diff(y) <= 2)
}

#[test]
fn b385_unmult() {
    let mut t = Table::new(
        "unmult",
        "# D-505: Unmult\n\nB-385, after After Effects' Unmult (Keying, new in After Effects \
         26.0): a black or white background turned into transparency. Adobe names the controls \
         but publishes no formula and no defaults we found, so the rule and the settings when \
         added are ours, stated in `tools/unmult_reference.py`: the covering becomes how far the \
         pixel is from the background (over black its brightest channel, over white how far its \
         darkest channel falls below white), less the level, over the softness; Remove Color \
         Matting restores the colour from before the background. Worked in linear light. Every \
         expected pixel is `Fixtures/unmult/expected_unmult.json`, written by the tool before \
         this code existed and printed in document 25 as FX-UNMULT-001 to 019. Tolerance 2e-5.\n",
    );

    t.heading("FX-UNMULT-001 to 019 (document 25)");
    t.fixtures_numbered("expected_unmult.json", 1..=19);

    t.heading("The file");
    let files: Vec<String> = (1..=19).map(|n| format!("fx_unmult_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_unmult_003.json");
    t.row(
        "fx_unmult_003.json is saved with its six settings, background black, black level 10, softness 20, remove color matting on",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["background_color"] == "black"
            && saved["black_level"] == 10.0
            && saved["softness"] == 20.0
            && saved["remove_color_matting"] == "on",
    );
    for (file, want) in [
        ("fx_unmult_015.json", "black level"),
        ("fx_unmult_016.json", "softness"),
        ("fx_unmult_017.json", "background color"),
        ("fx_unmult_018.json", "remove color matting"),
        ("fx_unmult_019.json", "clip HDR results"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(
            &format!("{file} is refused in a sentence naming {want}"),
            &why,
            why.contains(want),
        );
    }
    t.shape_refused(
        "fx_unmult_001.json",
        "an Unmult whose softness is a word",
        r#"{"background_color": "black", "black_level": 0, "white_level": 100, "softness": "soft", "remove_color_matting": "on", "clip_hdr_results": "off"}"#,
    );
    t.shape_refused(
        "fx_unmult_001.json",
        "an Unmult without its background colour",
        r#"{"black_level": 0, "white_level": 100, "softness": 0, "remove_color_matting": "on", "clip_hdr_results": "off"}"#,
    );
    t.shape_refused(
        "fx_unmult_001.json",
        "an Unmult without its white level",
        r#"{"background_color": "black", "black_level": 0, "softness": 0, "remove_color_matting": "on", "clip_hdr_results": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_unmult_001.json").document;
    let over = changed(&t, "fx_unmult_001.json", |e| {
        if let Effect::Unmult { black_level, .. } = e {
            *black_level = 101.0
        }
    });
    let word = changed(&t, "fx_unmult_001.json", |e| {
        if let Effect::Unmult {
            background_color, ..
        } = e
        {
            *background_color = "grey".into()
        }
    });
    t.refused(
        &mut document,
        vec![
            ("black level 101", set(over)),
            ("background color \"grey\"", set(word)),
            (
                "softness keyed to 300",
                keys("softness", &[(0, &[0.0]), (4, &[300.0])]),
            ),
        ],
    );
    let white = changed(&t, "fx_unmult_001.json", |e| {
        if let Effect::Unmult {
            background_color,
            white_level,
            ..
        } = e
        {
            *background_color = "white".into();
            *white_level = 90.0;
        }
    });
    t.taken(
        &mut document,
        "fx_unmult_001.json",
        vec![
            ("background white, white level 90", set(white)),
            (
                "black level keyed from 0 to 40",
                keys("black_level", &[(0, &[0.0]), (4, &[40.0])]),
            ),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_unmult_002.json", 0),
        ("fx_unmult_003.json", 0),
        ("fx_unmult_006.json", 0),
        ("fx_unmult_010.json", 0),
        ("fx_unmult_012.json", 2),
        ("fx_unmult_014.json", 3),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // The refused ones (015 to 019) leave nothing for the card.
    let none: Vec<u32> = (15..=19).collect();
    card_fixtures(&mut t, &mut gpu, &none);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added (black)", json!({}), 3),
            (
                "white, white level 80, softness 20, remove color matting off",
                json!({"background_color": "white", "white_level": 80, "softness": 20, "remove_color_matting": "off"}),
                3,
            ),
            (
                "black level 20, softness 10, clip HDR results on",
                json!({"black_level": 20, "softness": 10, "clip_hdr_results": "on"}),
                3,
            ),
        ],
    );

    t.heading("Pictures: in `verification/D-505 pictures/`");
    let dir = repo("verification/D-505 pictures");
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
    write("flare.png", &shot_on(0.0, ORANGE, true));
    write("smoke.png", &shot_on(255.0, [55.0; 3], false));
    let flare = |p: J| picture(&dir, "flare.png", TOWN, json!([fx("fx-0-0", &p)]));
    let (before, said) = picture(&dir, "flare.png", TOWN, json!([]));
    write("1_flare_before.png", &before);
    write("1b_flare_before_over_sky.png", &over_sky(&before));
    t.row(
        "1_flare_before.png, a flare shot on black (white core, orange glow) and a mid-grey square; 1b lays it over a sky as it is, so the black hides the sky; draws cleanly",
        &format!("{said:?}"),
        said.is_empty(),
    );

    // The places to look: the black, the core, the glow 60 pixels out (53, 33, 12), the grey.
    let (black, core, glow) = ((5, 5), (240, 135), (300, 135));
    // By hand (scratchpad unmult_hand.py's sums, as the tool's rule): the glow pixel's brightest
    // channel 53 is 0.0356 in linear light, covering 9, straight colour (255, 175, 90); grey 128
    // is 0.2158, covering 55, straight white.
    let (after, said) = flare(json!({}));
    write("2_flare_unmult.png", &after);
    write("2b_flare_unmult_over_sky.png", &over_sky(&after));
    let (b, c, g, m) = (
        at(&after, black),
        at(&after, core),
        at(&after, glow),
        at(&after, GREY),
    );
    t.row(
        "2_flare_unmult.png, as added: the black see-through, the core whole and white, the glow pixel covering 9 of 255 at straight (255, 175, 90), the grey square covering 55 at straight white; 2b over the sky shows the glow with no black box; draws cleanly",
        &format!("{said:?}, black {b:?}, core {c:?}, glow {g:?}, grey {m:?}"),
        said.is_empty() && b[3] == 0 && c == [255; 4] && near(g, [255, 175, 90, 9]) && near(m, [255, 255, 255, 55]),
    );

    // By hand: level 10% leaves out the glow pixel (0.0356); the grey's share is (0.2158 - 0.1)
    // / 0.2 = 0.579, covering 0.125, 32 of 255.
    let (after, said) = flare(json!({"black_level": 10, "softness": 20}));
    write("3_black_level_10_softness_20.png", &after);
    write(
        "3b_black_level_10_softness_20_over_sky.png",
        &over_sky(&after),
    );
    let (c, g, m) = (at(&after, core), at(&after, glow), at(&after, GREY));
    t.row(
        "3_black_level_10_softness_20.png: the faint outer glow gone (the glow pixel covering 0), the core whole, the grey square covering 32 by hand; draws cleanly",
        &format!("{said:?}, core {c:?}, glow {g:?}, grey {m:?}"),
        said.is_empty() && g[3] == 0 && c == [255; 4] && m[3].abs_diff(32) <= 2,
    );

    let (after, said) = flare(json!({"remove_color_matting": "off"}));
    write("4_remove_color_matting_off.png", &after);
    write(
        "4b_remove_color_matting_off_over_sky.png",
        &over_sky(&after),
    );
    let (g, m) = (at(&after, glow), at(&after, GREY));
    t.row(
        "4_remove_color_matting_off.png: the same coverings, but each pixel keeps its own colour, so the glow and the grey stay dark (the grey straight 128 at covering 55, the glow straight (53, 33, 12) at 9); over the sky the glow looks dirtier; draws cleanly",
        &format!("{said:?}, glow {g:?}, grey {m:?}"),
        said.is_empty() && near(m, [128, 128, 128, 55]) && near(g, [53, 33, 12, 9]),
    );

    // By hand: smoke's darkest 55 is 0.0382 in linear light, so over white it covers
    // 1 - 0.0382, 245 of 255, straight black; grey 128 covers 1 - 0.2158, 200, straight black.
    let (before, said_before) = picture(&dir, "smoke.png", TOWN, json!([]));
    write("5_smoke_before.png", &before);
    let (after, said) = picture(
        &dir,
        "smoke.png",
        TOWN,
        json!([fx("fx-0-0", &json!({"background_color": "white"}))]),
    );
    write("5b_smoke_unmult_white.png", &after);
    write("5c_smoke_unmult_white_over_sky.png", &over_sky(&after));
    let (w, s, m) = (at(&after, black), at(&after, core), at(&after, GREY));
    t.row(
        "5_smoke_before.png, dark smoke on white; 5b with background white: the white see-through, the smoke's middle covering 245 at straight black, the grey square covering 200 at straight black, by hand; draws cleanly",
        &format!("{said_before:?} {said:?}, white {w:?}, smoke {s:?}, grey {m:?}"),
        said_before.is_empty() && said.is_empty() && w[3] == 0 && near(s, [0, 0, 0, 245]) && near(m, [0, 0, 0, 200]),
    );

    t.finish("D-505_unmult_table.md");
}

/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B385_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-385: a measurement, run deliberately with --release --ignored"]
fn b385_unmult_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B385_CPU").is_ok();
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
        ("Noise, then Unmult as added (black)", Some(json!({}))),
        (
            "Noise, then Unmult, white, white level 80, softness 20",
            Some(json!({"background_color": "white", "white_level": 80, "softness": 20})),
        ),
        (
            "Noise, then Unmult, black level 20, softness 10, matting off",
            Some(json!({"black_level": 20, "softness": 10, "remove_color_matting": "off"})),
        ),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![
                json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}}),
            ];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
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
    let out = std::env::var("B385_OUT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| repo("verification/B-385_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
