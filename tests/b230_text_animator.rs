//! B-230: D-350, After Effects' text animators (EFFECTS.md P0-7, part 1): an animator with
//! Position, Scale, Rotation, Opacity, Fill Color and Tracking, and its Range Selector (Start,
//! End, Offset, Amount, Based On characters or words, the six shapes, Smoothness, Ease High and
//! Low), applied to each character by how far the selector picks it.
//!
//! Every expected number is `Fixtures/text_animator/expected_text_animator.json`, written by
//! `tools/text_animator_reference.py` (fontTools) before this code existed. The letters stay
//! outlines drawn on the processor, as D-263's are; what comes after them runs where it did.

mod effect_table;

use std::fs;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Expression, Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::text::{self, Text};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const W: usize = 1280;
const H: usize = 360;

fn expected() -> J {
    serde_json::from_str(&fs::read_to_string(repo("Fixtures/text_animator/expected_text_animator.json")).unwrap()).unwrap()
}

/// A 1280 by 360 composition of one text layer `words`, carrying `animators` as written in the
/// fixture, under an identity transform.
fn project(words: &J, animators: &[J], frames: u32) -> Project {
    sized(words, animators, frames, (W, H))
}

fn sized(words: &J, animators: &[J], frames: u32, (w, h): (usize, usize)) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects: Vec<J> = animators
        .iter()
        .enumerate()
        .map(|(i, p)| json!({"instance_id": format!("fx-{i}"), "type_id": "core.text_animator", "enabled": true, "parameters": p}))
        .collect();
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": frames,
        "transform": {"anchor": t(json!([w / 2, h / 2])), "position": t(json!([w / 2, h / 2])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": words});
    let p = json!({
        "schema_version": 0, "project_id": "proj-b230",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": w, "height": h, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": frames,
            "work_area": {"start_frame": 0, "end_frame_exclusive": frames}, "layer_order": ["art"], "layers": [layer]}]
    });
    persist::load_str(&p.to_string()).unwrap_or_else(|d| panic!("the project reads: {}", d.message)).document.project().clone()
}

/// The text and the animators of the `art` layer, read from the file as the build reads them.
fn parts(project: &Project) -> (Text, Vec<Effect>) {
    let layer = project.composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap();
    (layer.text.clone().unwrap(), layer.effects.iter().map(|i| i.effect.clone()).collect())
}

/// The animators of `document`'s `art` layer as they are at `frame`, keys and expressions read.
fn at_frame(project: &Project, frame: i32) -> Vec<Effect> {
    let comp = project.composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    layer
        .effects
        .iter()
        .map(|i| anime_compositor::expr::effect_at(comp, &layer.id, i, frame, frame as f64).0.effect)
        .collect()
}

fn render(project: &Project, frame: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(project, &Id::new(MAIN), frame, &repo("Fixtures/text_animator"), 64, &mut log)
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

fn warned(project: &Project, frame: i32) -> Vec<String> {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(project, &Id::new(MAIN), frame, &repo("Fixtures/text_animator"), &mut log);
    log.finish().iter().map(|d| d.id.as_str().to_string()).collect()
}

fn nums(v: &J) -> Vec<f64> {
    match v {
        J::Array(a) => a.iter().map(|x| x.as_f64().unwrap()).collect(),
        x => vec![x.as_f64().unwrap()],
    }
}

fn far(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

/// The largest alpha inside `[x0, y0, x1, y1]`.
fn most_alpha(b: &WorkingBuffer, r: [f64; 4]) -> f32 {
    let mut m = 0f32;
    for y in r[1].floor().max(0.0) as usize..(r[3].ceil() as usize).min(b.height()) {
        for x in r[0].floor().max(0.0) as usize..(r[2].ceil() as usize).min(b.width()) {
            m = m.max(b.pixel(x, y)[3]);
        }
    }
    m
}

/// The frame over a dark grey, so white letters show on any page.
fn picture(dir: &std::path::Path, file: &str, b: &WorkingBuffer) {
    let mut over = WorkingBuffer::opaque(b.width(), b.height());
    for (o, p) in over.data_mut().chunks_mut(4).zip(b.data().chunks(4)) {
        for k in 0..3 {
            o[k] = p[k] + (1.0 - p[3]) * 0.02;
        }
        o[3] = 1.0;
    }
    png_out::write_rgba(&dir.join(format!("{file}.png")), b.width(), b.height(), OutputDepth::Eight, &[], &over.to_srgb8_straight()).unwrap();
}

#[test]
fn b230_text_animator() {
    let mut t = Table::new(
        "text_animator",
        "# B-230: text animators\n\nD-350 (EFFECTS.md P0-7, part 1): After Effects' text animator, \
         with Position, Scale, Rotation, Opacity, Fill Color and Tracking, and its Range Selector, \
         applied to each character by how far the selector picks it. Every expected number is \
         `Fixtures/text_animator/expected_text_animator.json`, written by \
         `tools/text_animator_reference.py` before this code existed and printed in document 25 as \
         FX-TXA-001 to 023. Per character the build gives the foot it turns and scales about, the \
         move, scale, turn, opacity and colour, and each animator's amount, compared within 1e-6, \
         and the box of its drawn outline, within a quarter of a pixel (the build cuts curves into \
         pieces about 2 pixels long; the reference takes each curve's exact extent).\n",
    );
    let e = expected();
    let tol = e["tolerance"].as_f64().unwrap();
    let box_tol = e["box_tolerance"].as_f64().unwrap();

    t.heading("FX-TXA-001 to 013: every character (document 25)");
    let mut drawn = std::collections::BTreeMap::new();
    for (name, case) in e["cases"].as_object().unwrap() {
        let animators: Vec<J> = case["animators"].as_array().unwrap().clone();
        let p = project(&case["text"], &animators, 1);
        let (words, fx) = parts(&p);
        let placed = text::placed(&words, &fx).expect("the font is there");
        let want = case["chars"].as_array().unwrap();
        let (mut worst, mut worst_box, mut same_chars) = (0f64, 0f64, placed.len() == want.len());
        for (b, w) in placed.iter().zip(want) {
            same_chars &= b.index as u64 == w["index"].as_u64().unwrap() && b.ch.to_string() == w["char"].as_str().unwrap();
            for d in [
                far(&b.anchor, &nums(&w["anchor"])),
                far(&b.moved, &nums(&w["move"])),
                far(&b.scale, &nums(&w["scale"])),
                far(&[b.turn], &nums(&w["turn"])),
                far(&[b.opacity], &nums(&w["opacity"])),
                far(&b.color, &nums(&w["color"])),
                far(&b.amounts, &nums(&w["amounts"])),
            ] {
                worst = worst.max(d);
            }
            worst_box = worst_box.max(match (b.bounds, w["box"].is_null()) {
                (None, true) => 0.0,
                (Some(r), false) => far(&r, &nums(&w["box"])),
                _ => f64::INFINITY,
            });
        }
        t.row(
            &format!("{name}: {}", case["says"].as_str().unwrap()),
            &format!("{} characters; numbers within {worst:.1e}, boxes within {worst_box:.3} px", placed.len()),
            same_chars && worst <= tol && worst_box <= box_tol,
        );
        drawn.insert(name.clone(), (words, fx, placed, p));
    }

    t.heading("The pictures the numbers make");
    let plain = |words: &Text| text::draw(words, W, H).unwrap();
    {
        // Typewriter: left of the seventh letter as with no animator, right of it nothing.
        let (words, fx, placed, _) = &drawn["FX-TXA-001"];
        let a = text::animated(words, fx, W, H).unwrap();
        let p = plain(words);
        let cut = ((placed[5].bounds.unwrap()[2] + placed[6].bounds.unwrap()[0]) / 2.0) as usize;
        let (mut left_same, mut right_empty) = (true, true);
        for y in 0..H {
            for x in 0..W {
                if x < cut {
                    left_same &= a.pixel(x, y) == p.pixel(x, y);
                } else {
                    right_empty &= a.pixel(x, y)[3] == 0.0;
                }
            }
        }
        t.row(
            &format!("FX-TXA-001: left of x = {cut} the line as with no animator, pixel for pixel; right of it nothing"),
            &format!("left {}, right {}", if left_same { "the same" } else { "differs" }, if right_empty { "empty" } else { "has ink" }),
            left_same && right_empty,
        );
    }
    {
        // Soft typewriter: the seventh letter at its opacity.
        let (words, fx, placed, _) = &drawn["FX-TXA-002"];
        let (a, p) = (text::animated(words, fx, W, H).unwrap(), plain(words));
        let c = &placed[6];
        let r = c.bounds.unwrap();
        let (got, full) = (most_alpha(&a, r), most_alpha(&p, r));
        t.row(
            &format!("FX-TXA-002: the seventh letter, at opacity {:.4}, has that share of its ink", c.opacity),
            &format!("strongest alpha {got:.4} of {full:.4}"),
            (got as f64 - c.opacity * full as f64).abs() <= 1e-5,
        );
    }
    {
        // Fill colour: the last letter, wholly picked, is green where it is solid.
        let (words, fx, placed, _) = &drawn["FX-TXA-008"];
        let a = text::animated(words, fx, W, H).unwrap();
        let c = placed.last().unwrap();
        let r = c.bounds.unwrap();
        let (mut solid, mut worst) = (0, 0f32);
        for y in r[1] as usize..r[3] as usize {
            for x in r[0] as usize..r[2] as usize {
                let px = a.pixel(x, y);
                if px[3] == 1.0 {
                    solid += 1;
                    for k in 0..3 {
                        worst = worst.max((px[k] - c.color[k] as f32).abs());
                    }
                }
            }
        }
        t.row(
            &format!("FX-TXA-008: the last letter's solid pixels are its Fill Color {:?}", c.color),
            &format!("{solid} solid pixels, largest difference {worst:.1e}"),
            solid > 50 && worst <= 1e-6,
        );
    }
    {
        // A wave moves each letter's ink by its move.
        let (words, fx, placed, _) = &drawn["FX-TXA-006"];
        let (a, p) = (text::animated(words, fx, W, H).unwrap(), plain(words));
        let top = |b: &WorkingBuffer, r: [f64; 4]| {
            (0..H).find(|&y| (r[0] as usize..r[2] as usize).any(|x| b.pixel(x, y)[3] > 0.5)).unwrap_or(H) as f64
        };
        let mut worst = 0f64;
        for c in placed.iter().filter(|c| c.bounds.is_some()) {
            let r = c.bounds.unwrap();
            let still = [r[0] - c.moved[0], r[1] - c.moved[1], r[2] - c.moved[0], r[3] - c.moved[1]];
            worst = worst.max((top(&a, r) - top(&p, still) - c.moved[1]).abs());
        }
        t.row(
            "FX-TXA-006: each letter's top edge is raised by its own move",
            &format!("largest difference {worst:.0} px"),
            worst <= 1.0,
        );
    }

    t.heading("FX-TXA-020 to 023: the files");
    let files = ["fx_txa_020.json", "fx_txa_021.json", "fx_txa_022.json", "fx_txa_023.json"];
    for (name, case) in e["projects"].as_object().unwrap() {
        let loaded = t.load(case["project"].as_str().unwrap());
        let doc = loaded.document.project().clone();
        for (frame, want) in case["visible"].as_object().unwrap() {
            let frame: i32 = frame.parse().unwrap();
            let (words, _) = parts(&doc);
            let fx = at_frame(&doc, frame);
            let shown: Vec<u64> = text::placed(&words, &fx).unwrap().iter().filter(|c| c.opacity > 0.0).map(|c| c.index as u64).collect();
            let want: Vec<u64> = want.as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
            let same = render(&doc, frame).data() == text::animated(&words, &fx, W, H).unwrap().data();
            t.row(
                &format!("{name} frame {frame}: {}", case["says"].as_str().unwrap()),
                &format!("characters shown {shown:?}; the frame {} the animated line", if same { "is" } else { "is not" }),
                shown == want && same,
            );
        }
        let on_open: Vec<String> = loaded.warnings.iter().map(|d| d.id.as_str().to_string()).collect();
        let at = warned(&doc, 4);
        let want_open: Vec<String> = case["warning"].as_str().into_iter().map(String::from).collect();
        let want_at: Vec<String> = case["frame_warning"].as_str().into_iter().map(String::from).collect();
        t.row(
            &format!("{name}: what opening it warns of, and what frame 4 warns of"),
            &format!("{on_open:?} and {at:?}"),
            on_open == want_open && at == want_at,
        );
    }
    {
        let doc = t.load("fx_txa_022.json").document.project().clone();
        let (words, _) = parts(&doc);
        let same = render(&doc, 4).data() == text::draw(&words, W, H).unwrap().data();
        t.row("fx_txa_022.json: with a shape it does not know, the line is drawn as with no animator", if same { "the same" } else { "differs" }, same);
        let doc = t.load("fx_txa_023.json").document.project().clone();
        let mut json: J = serde_json::from_str(&fs::read_to_string(t.root.join("fx_txa_023.json")).unwrap()).unwrap();
        json["compositions"][0]["layers"][0]["effects"] = json!([]);
        let bare = persist::load_str(&json.to_string()).unwrap().document.project().clone();
        let same = render(&doc, 4).data() == render(&bare, 4).data();
        t.row("fx_txa_023.json: the solid is drawn as with no animator", if same { "the same" } else { "differs" }, same);
    }
    t.round_trips(&files);
    let kept = t.saved_parameters("fx_txa_021.json");
    t.row("fx_txa_021.json: the unknown \"selector_mode\" is saved as written", &kept["selector_mode"].to_string(), kept["selector_mode"] == "add");
    t.shape_refused("fx_txa_021.json", "a Start written as a word", r#"{"position":[0,0],"scale":[100,100],"rotation":0,"opacity":0,"fill":"off","color":[1,0,0],"tracking":0,"start":"forty","end":100,"offset":0,"amount":100,"based_on":"characters","shape":"square","smoothness":0,"ease_high":0,"ease_low":0}"#);
    t.shape_refused("fx_txa_021.json", "a Position of one number", r#"{"position":5,"scale":[100,100],"rotation":0,"opacity":0,"fill":"off","color":[1,0,0],"tracking":0,"start":40,"end":100,"offset":0,"amount":100,"based_on":"characters","shape":"square","smoothness":0,"ease_high":0,"ease_low":0}"#);
    t.shape_refused("fx_txa_021.json", "no End", r#"{"position":[0,0],"scale":[100,100],"rotation":0,"opacity":0,"fill":"off","color":[1,0,0],"tracking":0,"start":40,"offset":0,"amount":100,"based_on":"characters","shape":"square","smoothness":0,"ease_high":0,"ease_low":0}"#);

    t.heading("Commands: keys, an expression, a refusal, undo");
    let mut document: Document = t.load("fx_txa_021.json").document;
    let (comp, layer, fx) = (Id::new(MAIN), Id::new("art"), Id::new("fx-0"));
    let first = text::animated(&parts(document.project()).0, &at_frame(document.project(), 0), W, H).unwrap();
    let key = |frame, value: &[f64]| EffectKey { frame, value: value.to_vec(), interp: Interp::Linear };
    let keys = |setting: &str, k: Vec<EffectKey>| Command::SetEffectKeys {
        composition: comp.clone(),
        layer_id: layer.clone(),
        instance_id: fx.clone(),
        setting: setting.to_string(),
        keys: k,
    };
    let steps: Vec<(&str, Command, i32, Box<dyn Fn(&Effect) -> bool>)> = vec![
        (
            "Position keyed 0, 0 at frame 0 to 0, -80 at frame 8: at frame 4 it is 0, -40",
            keys("position", vec![key(0, &[0.0, 0.0]), key(8, &[0.0, -80.0])]),
            4,
            Box::new(|e| matches!(e, Effect::TextAnimator { position, .. } if far(position, &[0.0, -40.0]) < 1e-9)),
        ),
        (
            "Fill colour keyed red at 0 to blue at 8: at frame 4 it is half of each",
            keys("color", vec![key(0, &[1.0, 0.0, 0.0]), key(8, &[0.0, 0.0, 1.0])]),
            4,
            Box::new(|e| matches!(e, Effect::TextAnimator { color, .. } if far(color, &[0.5, 0.0, 0.5]) < 1e-9)),
        ),
        (
            "Offset keyed -100 at 0 to 100 at 8: at frame 2 it is -50",
            keys("offset", vec![key(0, &[-100.0]), key(8, &[100.0])]),
            2,
            Box::new(|e| matches!(e, Effect::TextAnimator { offset, .. } if (offset + 50.0).abs() < 1e-9)),
        ),
        (
            "Start given the expression \"time*24*10\": at frame 3 it is 30",
            Command::SetEffectExpression {
                composition: comp.clone(),
                layer_id: layer.clone(),
                instance_id: fx.clone(),
                setting: "start".to_string(),
                expression: Some(Expression { text: "time*24*10".to_string(), enabled: true }),
            },
            3,
            Box::new(|e| matches!(e, Effect::TextAnimator { start, .. } if (start - 30.0).abs() < 1e-9)),
        ),
    ];
    let n = steps.len();
    for (what, command, frame, check) in steps {
        let taken = document.apply(command).is_ok();
        let now = at_frame(document.project(), frame);
        let ok = taken && check(&now[0]);
        t.row(what, &format!("{} ; {:?}", if taken { "taken" } else { "refused" }, now[0]), ok);
    }
    let past = document.apply(keys("opacity", vec![key(0, &[150.0])])).err();
    t.row(
        "Opacity keyed 150 (past its 0 to 100) is refused with a sentence",
        &past.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        past.is_some(),
    );
    let mut wrong = parts(document.project()).1[0].clone();
    if let Effect::TextAnimator { shape, .. } = &mut wrong {
        *shape = "wiggly".to_string();
    }
    let refused = document
        .apply(Command::SetEffectParameters { composition: comp.clone(), layer_id: layer.clone(), instance_id: fx.clone(), effect: wrong })
        .err();
    t.row(
        "the shape \"wiggly\" is refused with a sentence",
        &refused.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        refused.is_some(),
    );
    for _ in 0..n {
        document.undo();
    }
    let back = text::animated(&parts(document.project()).0, &at_frame(document.project(), 0), W, H).unwrap();
    t.row(&format!("undo {n} times: frame 0 is what it was"), if back.data() == first.data() { "byte-identical" } else { "differs" }, back.data() == first.data());

    t.heading("The card: the letters drawn on the processor, the frame shown through the card");
    let root = repo("Fixtures/text_animator");
    let mut gpu = Gpu::new().expect("a usable card");
    for file in ["fx_txa_020.json", "fx_txa_023.json"] {
        let doc = t.load(file).document.project().clone();
        for frame in [0, 6, 15] {
            let mut cache = CelCache::viewer();
            let mut log = FrameLog::new(3);
            let c = preview::preview_frame_cached(&doc, &Id::new(MAIN), frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache)
                .unwrap()
                .to_srgb8_straight();
            let mut log = FrameLog::new(3);
            let (g, ..) = preview::preview_frame_srgb8(&doc, &Id::new(MAIN), frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).unwrap();
            let said: Vec<String> = log.finish().iter().map(|d| d.id.as_str().to_string()).collect();
            let worst = c.iter().zip(&g).map(|(a, b)| a.abs_diff(*b)).max().unwrap_or(0);
            t.row(
                &format!("{file} frame {frame}: the card's frame against the processor's"),
                &format!("largest {worst} level(s); warnings {said:?}"),
                worst <= 1 && !said.iter().any(|s| s == DiagnosticId::GpuPreviewOnCpu.as_str()),
            );
        }
    }

    t.heading("Pictures, in `verification/D-350 pictures/`");
    let dir = repo("verification/D-350 pictures");
    fs::create_dir_all(&dir).unwrap();
    let doc = t.load("fx_txa_020.json").document.project().clone();
    for frame in [0, 3, 6, 10, 15] {
        let file = format!("typewriter_frame_{frame:02}");
        picture(&dir, &file, &render(&doc, frame));
        t.row(&format!("{file}.png: the typewriter at frame {frame} of 15, Start {}", frame * 100 / 15), "written", true);
    }
    for (case, file, what) in [
        ("FX-TXA-002", "typewriter_soft", "Smoothness 100: the seventh letter half faded"),
        ("FX-TXA-003", "fade_in_by_character", "Ramp Up, eased: bright on the left, fading to nothing on the right"),
        ("FX-TXA-004", "fade_out_by_character", "Ramp Down: the other way round"),
        ("FX-TXA-005", "word_by_word", "Based On Words: whole words lowered and faded together"),
        ("FX-TXA-006", "wave", "Triangle: the letters in the range raised, most in its middle"),
        ("FX-TXA-007", "humps", "Smooth and Round: a smooth hump and a swelling in the middle"),
        ("FX-TXA-008", "turn_scale_colour", "turned, squashed, spaced and the second half green"),
        ("FX-TXA-011", "thirty_characters", "the timing line: a wave and a fade together"),
    ] {
        let (words, fx, ..) = &drawn[case];
        picture(&dir, file, &text::animated(words, fx, W, H).unwrap());
        t.row(&format!("{file}.png ({case}): {what}"), "written", true);
    }

    t.finish("D-350_text_animator_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times: FX-TXA-011's 30-character line in a 1920 by 1080 composition, its two
/// animators' Offsets keyed from -100 at frame 0 to 100 at frame 47 so every frame is new.
/// First the drawing of the line alone (`text::animated`, which the viewer keeps no copy of here)
/// with no animator and with the two at each frame's values; then the viewer's whole frame, as it
/// asks for it, with Draw on: GPU. Each is 48 frames untimed once, then four loops timed; the
/// median of the 192 is given, and for the viewer the first loop's median too.
#[test]
#[ignore = "B-230: a measurement, run deliberately with --release --ignored"]
fn b230_text_animator_timing() {
    use std::fmt::Write as _;
    use std::time::Instant;
    let e = expected();
    let case = &e["cases"]["FX-TXA-011"];
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n\
         | Shot | First | Again |\n|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    let animators: Vec<J> = case["animators"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| {
            let mut p = a.clone();
            p["offset"] = json!({"base": -100, "keyframes": [{"frame": 0, "value": -100, "interp": "linear"},
                {"frame": 47, "value": 100, "interp": "linear"}]});
            p
        })
        .collect();
    let project = sized(&case["text"], &animators, 48, (1920, 1080));
    let (words, _) = parts(&project);
    let frames: Vec<Vec<Effect>> = (0..48).map(|f| at_frame(&project, f)).collect();
    for (name, with) in [("drawing the line, no animator", false), ("drawing the line, two animators", true)] {
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..5 {
            for fx in &frames {
                let fx: &[Effect] = if with { fx } else { &[] };
                let t = Instant::now();
                drop(text::animated(&words, fx, 1920, 1080).unwrap());
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | {:.1} | {:.1} |", median(first), median(times));
    }
    let root = repo("Fixtures/text_animator");
    let mut cache = CelCache::viewer();
    gpu.forget();
    let (mut first, mut times) = (Vec::new(), Vec::new());
    for pass in 0..5 {
        for frame in 0..48 {
            let mut log = FrameLog::new(3);
            let t = Instant::now();
            drop(preview::preview_frame_srgb8(&project, &Id::new(MAIN), frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
            let ms = t.elapsed().as_secs_f64() * 1000.0;
            assert!(log.finish().is_empty(), "the frame draws without a warning");
            if pass > 0 { times.push(ms) } else { first.push(ms) }
        }
    }
    let _ = writeln!(s, "| the viewer's whole frame, two animators, Full, Draw on: GPU | {:.1} | {:.1} |", median(first), median(times));
    let out = std::env::var("B230_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-230_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
