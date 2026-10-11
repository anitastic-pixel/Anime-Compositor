//! B-409 (D-529): Numbers' and Timecode's words drawn faster with no picture changed. Each
//! character is drawn only over the rows it reaches (`text::draw_rows`), and the last few word
//! pictures are kept by their words and settings. The checks: `draw_rows` gives exactly `draw`'s
//! pixels; Numbers and Timecode frames are bit for bit the frames B-408's engine drew (their
//! fingerprints, written by that engine, in `verification/B-409_before_fingerprints.txt`); and a
//! kept picture is only used for the same words drawn the same way.

mod effect_table;

use std::fs;

use effect_table::{Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::text::{self, Align, Background, Shadow, Stroke, Text};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

fn plates() -> std::path::PathBuf {
    std::env::temp_dir().join("b409_plates")
}

/// A one-layer composition `w` by `h` at 24 frames a second of a still picture (written to
/// [`plates`]): a dark blue to purple ramp with lighter bars, carrying `effects`.
fn plate((w, h): (usize, usize), frames: u32, effects: J) -> Project {
    let dir = plates();
    fs::create_dir_all(&dir).unwrap();
    let file = format!("plate_{w}x{h}.png");
    let pixels: Vec<u8> = (0..w * h)
        .flat_map(|i| {
            let (x, y) = (i % w, i / w);
            let bar = if (x / 40) % 2 == 0 { 0 } else { 24 };
            [(20 + 80 * x / w + bar) as u8, (30 + 40 * y / h) as u8, (90 + bar) as u8, 255]
        })
        .collect();
    png_out::write_rgba(&dir.join(&file), w, h, OutputDepth::Eight, &[], &pixels).unwrap();
    let t = |v: J| json!({"base": v, "keyframes": []});
    let c = json!([w as f64 / 2.0, h as f64 / 2.0]);
    let project = json!({
        "schema_version": 0, "project_id": "proj-b409",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "asset-plate", "kind": "still", "name": "plate", "path": file,
                    "interpretation": {"color_space": "srgb", "alpha": "straight"}}],
        "compositions": [{
            "id": MAIN, "name": "Main", "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames},
            "layer_order": ["plate"],
            "layers": [{"id": "plate", "kind": "raster", "name": "plate", "asset_id": "asset-plate", "enabled": true,
                "locked": false, "in_frame": 0, "out_frame": frames, "source_offset_frames": 0,
                "transform": {"anchor": t(c.clone()), "position": t(c), "scale": t(json!([100, 100])),
                              "rotation": t(json!(0)), "opacity": t(json!(1))},
                "exposure_spans": [], "mask": null, "matte": null, "blend_mode": "normal", "effects": effects}]
        }]
    });
    persist::load_str(&project.to_string()).expect("the project reads").document.project().clone()
}

/// Numbers as added with `changes` laid over its settings.
fn numbers(changes: J) -> J {
    let mut p = json!({"type": "number", "random_values": "off", "value": 0, "decimal_places": 0,
        "position": [50, 50], "display": "fill_only", "fill_color": "#ffffff", "stroke_color": "#ff3300",
        "stroke_width": 3, "size": 64, "tracking": 0, "proportional_spacing": "on",
        "composite_on_original": "on", "font": "MPLUSRounded1c-Regular.ttf", "alignment": "center"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({"instance_id": "fx-1", "type_id": "core.numbers", "enabled": true, "parameters": p})
}

/// Timecode as added with `changes` laid over its settings.
fn timecode(changes: J) -> J {
    let mut p = json!({"display_format": "smpte", "time_source": "layer_source", "time_units": 30,
        "drop_frame": "off", "starting_frame": 0, "text_position": [50, 85], "text_size": 36,
        "text_color": "#ffffff", "show_box": "on", "box_color": "#000000", "opacity": 100,
        "composite_on_original": "on"});
    for (k, v) in changes.as_object().unwrap() {
        p[k] = v.clone();
    }
    json!({"instance_id": "fx-1", "type_id": "core.timecode", "enabled": true, "parameters": p})
}

/// FNV-1a over every number's bits, with the size: two pictures share it only when they are the
/// same bit for bit (barring a collision, about 1 in 2^64).
fn fingerprint(b: &WorkingBuffer) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |v: u32| {
        for byte in v.to_le_bytes() {
            h = (h ^ byte as u64).wrapping_mul(0x100_0000_01b3);
        }
    };
    eat(b.width() as u32);
    eat(b.height() as u32);
    b.data().iter().for_each(|v| eat(v.to_bits()));
    format!("{h:016x}")
}

fn frame(project: &Project, at: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(project, &Id::new(MAIN), at, &plates(), 64, &mut log).expect("the frame draws")
}

/// The frames fingerprinted: Numbers and Timecode with their characters each drawn on their own
/// (proportional spacing off, and every Timecode), with strokes, at the frame's edges.
fn shots() -> Vec<(&'static str, Project, i32)> {
    let small = (960, 540);
    vec![
        ("Numbers as added (proportional)", plate(small, 60, json!([numbers(json!({}))])), 0),
        ("Numbers, monospaced long date, stroke over fill 6, size 60", plate(small, 60, json!([numbers(json!({"type": "long_date", "value": 4000, "size": 60, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 6}))])), 0),
        ("Numbers, monospaced timecode 30, fill over stroke, frame 37", plate(small, 60, json!([numbers(json!({"type": "timecode_30", "proportional_spacing": "off", "display": "fill_over_stroke"}))])), 37),
        ("Numbers, monospaced random with two places, stroke only, frame 9", plate(small, 60, json!([numbers(json!({"random_values": "on", "value": 500, "decimal_places": 2, "proportional_spacing": "off", "display": "stroke_only", "stroke_width": 5}))])), 9),
        ("Numbers, monospaced, stroke over fill, partly above the top", plate(small, 60, json!([numbers(json!({"value": 2026, "position": [50, 2], "size": 80, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 4}))])), 0),
        ("Numbers, monospaced, stroke over fill, partly below the bottom", plate(small, 60, json!([numbers(json!({"value": 2026, "position": [50, 99], "size": 80, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 4}))])), 0),
        ("Numbers, monospaced leading zeros, right, tracking 200, composite off", plate(small, 60, json!([numbers(json!({"type": "number_leading_zeros", "value": -42.5, "decimal_places": 1, "proportional_spacing": "off", "alignment": "right", "tracking": 200, "composite_on_original": "off"}))])), 0),
        ("Timecode as added, frame 50", plate(small, 60, json!([timecode(json!({}))])), 50),
        ("Timecode, custom drop frame, size 120, half-opacity box, composite off, 1920 by 1080", plate((1920, 1080), 60, json!([timecode(json!({"time_source": "custom", "drop_frame": "on", "starting_frame": 107892, "text_size": 120, "opacity": 50, "composite_on_original": "off"}))])), 3),
        ("Timecode, feet and frames 35mm, size 120, box past the top", plate(small, 60, json!([timecode(json!({"display_format": "feet_frames_35mm", "text_size": 120, "text_position": [50, 3]}))])), 21),
    ]
}

/// The words for [`text::draw_rows`] against [`text::draw`], on a 640 by 360 frame.
fn texts() -> Vec<(&'static str, Text)> {
    let t = |s: &str, size: f64, at: [f64; 2]| Text { text: s.into(), size, at, color: [1.0, 0.8, 0.2], ..Default::default() };
    let stroke = |width: f64| Some(Stroke { color: [0.9, 0.2, 0.0], width });
    vec![
        ("a 0 at size 64 in the middle", t("0", 64.0, [320.0, 180.0])),
        ("a date at size 40 from the left", t("Sunday, January 1, 1995", 40.0, [20.0, 100.0])),
        ("a 7 with a stroke of 6", Text { stroke: stroke(6.0), ..t("7", 120.0, [300.5, 200.25]) }),
        ("a W, faux bold and italic, a stroke of 3", Text { faux_bold: true, faux_italic: true, stroke: stroke(3.0), ..t("W", 90.0, [100.3, 150.7]) }),
        ("two lines, centred", Text { align: Align::Center, ..t("two\nlines", 50.0, [320.0, 120.0]) }),
        ("a justified box of words", Text { align: Align::Justify, box_width: 300.0, ..t("words in a box that wraps onto more lines", 30.0, [40.0, 40.0]) }),
        ("an 8 partly above the top, stroke 4", Text { stroke: stroke(4.0), ..t("8", 80.0, [50.0, 20.0]) }),
        ("an 8 partly below the bottom, stroke 4", Text { stroke: stroke(4.0), ..t("8", 80.0, [50.0, 380.0]) }),
        ("an 8 wholly above the frame", t("8", 40.0, [50.0, -60.0])),
        ("a space", t(" ", 64.0, [320.0, 180.0])),
        ("nothing", t("", 64.0, [320.0, 180.0])),
        ("abc, all caps, tracking 300", Text { all_caps: true, tracking: 300.0, ..t("abc", 48.0, [10.0, 300.0]) }),
        ("88 at size 500, taller than the frame", Text { stroke: stroke(10.0), ..t("88", 500.0, [320.0, 340.0]) }),
        ("with a background (the whole frame)", Text { background: Some(Background { color: [0.0, 0.0, 0.0], opacity: 0.5, padding: 8.0, roundness: 4.0 }), ..t("box", 48.0, [200.0, 200.0]) }),
        ("with a shadow (the whole frame)", Text { shadow: Some(Shadow { color: [0.0, 0.0, 0.0], opacity: 0.7, angle: 135.0, distance: 6.0, softness: 4.0 }), ..t("shade", 48.0, [200.0, 200.0]) }),
    ]
}

#[test]
fn b409_words() {
    let mut t = Table::new(
        "numbers",
        "# B-409: Numbers' and Timecode's words, drawn faster\n\nD-529: each character Numbers \
         (proportional spacing off) and Timecode draw is drawn only over the rows it reaches, and \
         the last four word pictures are kept by their words and settings. No picture may change: \
         the rows drawn are compared number by number with the whole frame drawn as before, and \
         Numbers and Timecode frames are compared bit for bit, by fingerprint, with the frames \
         B-408's engine drew (`verification/B-409_before_fingerprints.txt`, written by that \
         engine with `--ignored b409_words_fingerprints`).\n",
    );

    t.heading("The rows the words reach, against the whole frame (640 by 360)");
    let (w, h) = (640, 360);
    for (name, words) in texts() {
        let whole = text::draw(&words, w, h).expect("the bundled font draws");
        let (part, top) = text::draw_rows(&words, w, h).expect("the bundled font draws");
        let rows = top..top + part.height();
        let inside = part.width() == w
            && rows.end <= h
            && part.data().iter().zip(&whole.data()[top * w * 4..rows.end * w * 4]).all(|(a, b)| a.to_bits() == b.to_bits());
        let outside = whole.data().chunks_exact(w * 4).enumerate().filter(|(y, _)| !rows.contains(y)).all(|(_, row)| row.iter().all(|v| *v == 0.0));
        let covered = whole.data().chunks_exact(4).filter(|p| p[3] > 0.0).count();
        t.row(
            &format!("{name}: rows {}..{} of {h}, every number the same bits, nothing outside them", rows.start, rows.end),
            &format!("same inside: {inside}; transparent outside: {outside}; {covered} pixels covered"),
            inside && outside,
        );
    }

    t.heading("Numbers and Timecode frames, bit for bit against B-408's engine");
    let before = fs::read_to_string(effect_table::repo("verification/B-409_before_fingerprints.txt")).unwrap_or_default();
    let before: std::collections::HashMap<&str, &str> = before.lines().filter_map(|l| l.split_once('\t')).collect();
    for (name, project, at) in shots() {
        let now = fingerprint(&frame(&project, at));
        let was = before.get(name).copied().unwrap_or("missing");
        let what = if name.contains(", frame ") { name.to_string() } else { format!("{name}, frame {at}") };
        t.row(&what, &format!("now {now}, before {was}"), now == was);
    }

    t.heading("A kept picture is used only for the same words drawn the same way");
    let still = |fill: &str| plate((480, 160), 60, json!([numbers(json!({"type": "long_date", "value": 4000, "size": 30, "proportional_spacing": "off", "fill_color": fill}))]));
    let a = fingerprint(&frame(&still("#ffffff"), 0));
    let again = fingerprint(&frame(&still("#ffffff"), 0));
    let later = fingerprint(&frame(&still("#ffffff"), 7));
    let blue = fingerprint(&frame(&still("#3366ff"), 7));
    let back = fingerprint(&frame(&still("#ffffff"), 7));
    t.row("the same words drawn again (kept) are the same frame", &format!("{a} then {again}"), a == again);
    t.row("frame 7 shows the same date as frame 0: the same frame", &later, later == a);
    t.row("a blue fill is drawn afresh, not the white words kept", &blue, blue != a);
    t.row("white again is white", &back, back == a);
    let counting = plate((480, 160), 60, json!([numbers(json!({"type": "timecode_30", "proportional_spacing": "off"}))]));
    let (f3, f4) = (fingerprint(&frame(&counting, 3)), fingerprint(&frame(&counting, 4)));
    t.row("a counting timecode: frame 4 is not frame 3's words", &format!("{f3} and {f4}"), f3 != f4);

    t.finish("D-529_words_table.md");
}

/// Writes the fingerprints of [`shots`] to `B409_FINGERPRINTS_OUT`; run on B-408's engine it made
/// `verification/B-409_before_fingerprints.txt`.
#[test]
#[ignore = "B-409: writes the fingerprints file, run deliberately with --ignored"]
fn b409_words_fingerprints() {
    let mut s = String::new();
    for (name, project, at) in shots() {
        s.push_str(&format!("{name}\t{}\n", fingerprint(&frame(&project, at))));
    }
    fs::write(std::env::var("B409_FINGERPRINTS_OUT").expect("B409_FINGERPRINTS_OUT names the file"), s).unwrap();
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-229's way: a 1920 by 1080 still, the effect then a Noise that changes every
/// frame, every eighth frame of 240 asked for as the viewer asks, whole, with Draw on: GPU. The
/// first loop starts with empty caches and its 30 frames' median is "first"; then seven loops are
/// timed, the median of their 210 frames is "again".
#[test]
#[ignore = "B-409: a measurement, run deliberately with --release --ignored"]
fn b409_words_timing() {
    use std::fmt::Write as _;

    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let noise = json!({"instance_id": "fx-2", "type_id": "core.noise", "enabled": true,
                       "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}});
    let shots: [(&str, Option<J>); 6] = [
        ("Noise alone", None),
        ("Numbers, monospaced random number with three places, stroke over fill 6, size 120 (new words every frame), then Noise",
            Some(numbers(json!({"random_values": "on", "value": 30000, "decimal_places": 3, "size": 120, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 6})))),
        ("Numbers, monospaced long date, stroke over fill 6, size 120 (the same words every frame), then Noise",
            Some(numbers(json!({"type": "long_date", "value": 4000, "size": 120, "proportional_spacing": "off", "display": "stroke_over_fill", "stroke_width": 6})))),
        ("Numbers, timecode 30 at size 200 (proportional), then Noise", Some(numbers(json!({"type": "timecode_30", "size": 200})))),
        ("Timecode, composition source at size 200, then Noise", Some(timecode(json!({"time_source": "composition", "text_size": 200})))),
        ("Timecode, custom drop frame on a half-opacity box, composite off, then Noise", Some(timecode(json!({"time_source": "custom",
            "drop_frame": "on", "text_size": 120, "opacity": 50, "composite_on_original": "off"})))),
    ];
    let dir = plates();
    for (name, e) in shots {
        let effects: Vec<J> = e.into_iter().chain([noise.clone()]).collect();
        let project = plate((1920, 1080), 240, json!(effects));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..8 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(&project, &Id::new(MAIN), frame, &dir, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                assert!(log.finish().is_empty(), "{name}: the frame draws without a warning");
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    fs::write(std::env::var("B409_OUT").expect("B409_OUT names the file"), s).expect("write the timing table");
}
