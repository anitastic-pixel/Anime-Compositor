//! B-949: D-1069, the Text Animator's Expression Selector, as After Effects' (iOknL8pPZGg 4:25 to
//! 5:40, G2 of the wave 12 report). The checks read how much the animator picks each letter
//! (`text::placed`) the way compose works it out, draw frames through compose, and save again.

mod effect_table;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::text::{self, Placed};
use anime_compositor::{expr, persist, png_out, OutputDepth, WorkingBuffer};

const W: usize = 1280;
const H: usize = 480;
const WORDS: &str = "Bounce";

/// The tutorial's bounce, word for word but for the `linear` line's arguments spelled out.
const BOUNCE: &str = "freq = 1; decay = 5; duration = 0.10; \
    retard = textIndex*thisComp.frameDuration*1; t = time - (inPoint + retard); \
    startVal = [100,100,100]; endVal = [0,0,0]; \
    if (t < duration) { linear(t, 0, duration, startVal, endVal) } \
    else { amp = (endVal - startVal)/duration; w = freq*Math.PI*2; \
    endVal + amp*(Math.sin((t - duration)*w)/Math.exp(decay*(t - duration))/w) }";

/// The title with an animator moving picked letters 200 pixels up, the Range Selector picking
/// every letter, and the Expression Selector on with `expr` on its Amount, `p` over it.
fn file(expr: Option<&str>, p: &J) -> J {
    let mut a = json!({"position": [0, -200], "scale": [100, 100], "rotation": 0, "opacity": 100, "fill": "off",
        "color": [1, 0, 0], "tracking": 0, "start": 0, "end": 100, "offset": 0, "amount": 100,
        "based_on": "characters", "shape": "square", "smoothness": 100, "ease_high": 0, "ease_low": 0,
        "expression_selector": "on", "expression_amount": 100});
    for (k, v) in p.as_object().unwrap() {
        if v.is_null() {
            a.as_object_mut().unwrap().remove(k);
        } else {
            a[k] = v.clone();
        }
    }
    if let Some(e) = expr {
        // The expression sits on the Amount `p` gives, 100 when it gives none.
        let base = a.get("expression_amount").cloned().unwrap_or(json!(100));
        a["expression_amount"] = json!({"base": base, "keyframes": [], "expression": {"text": e, "enabled": true}});
    }
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects = json!([{"instance_id": "fx-0", "type_id": "core.text_animator", "enabled": true, "parameters": a}]);
    let words = json!({"text": WORDS, "font": "MPLUSRounded1c-Regular.ttf", "size": 168, "color": [1, 1, 1], "at": [360, 340], "align": "left"});
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 48,
        "transform": {"anchor": t(json!([W / 2, H / 2])), "position": t(json!([W / 2, H / 2])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": words});
    json!({
        "schema_version": 0, "project_id": "proj-b949",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 48,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 48}, "layer_order": ["art"], "layers": [layer]}]
    })
}

fn load(e: Option<&str>, p: &J) -> persist::Loaded {
    persist::load_str(&file(e, p).to_string()).unwrap_or_else(|d| panic!("the project reads: {}", d.message))
}

fn project(e: Option<&str>, p: &J) -> Project {
    load(e, p).document.project().clone()
}

/// What each letter is picked at frame `f`, worked out as compose does, and the expression
/// errors effect_at reports.
fn picks(e: Option<&str>, p: &J, f: i32) -> (Vec<f64>, Vec<String>) {
    let project = project(e, p);
    let comp = project.composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let (now, failed) = expr::effect_at(comp, &layer.id, &layer.effects[0], f, f as f64);
    let words = layer.text.clone().unwrap();
    let mut fx = now.effect;
    let values = text::selector_values(&words, &fx);
    if let Ok(a) = expr::selector(comp, &layer.id, &layer.effects[0], f, &values) {
        if let Effect::TextAnimator { expression_values, .. } = &mut fx {
            *expression_values = a;
        }
    }
    let mut l: Vec<Placed> = text::placed(&words, &[fx]).expect("the font is there");
    l.sort_by_key(|c| c.index);
    (
        l.iter().filter(|c| c.ch != ' ').map(|c| c.amounts[0]).collect(),
        failed.iter().map(|(n, x)| format!("{n}: {}", x.message)).collect(),
    )
}

fn amounts(e: Option<&str>, p: &J, f: i32) -> Vec<f64> {
    picks(e, p, f).0
}

fn show(v: &[f64]) -> String {
    v.iter().map(|a| format!("{a:.3}")).collect::<Vec<_>>().join(", ")
}

fn near(a: &[f64], b: &[f64]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-9)
}

fn frame(e: Option<&str>, p: &J, f: i32) -> WorkingBuffer {
    let loaded = load(e, p);
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), f, &repo("Fixtures"), 1, &mut log)
        .unwrap_or_else(|d| panic!("frame {f} renders: {}", d.message))
}

/// The bounce's answer for letter `k` (from 1) at frame `f`, in this test's own words.
fn bounce(k: f64, f: f64) -> f64 {
    let t = f / 24.0 - k / 24.0;
    if t < 0.1 {
        100.0 * (1.0 - (t / 0.1).clamp(0.0, 1.0))
    } else {
        let w = std::f64::consts::PI * 2.0;
        -1000.0 * ((t - 0.1) * w).sin() / (5.0 * (t - 0.1)).exp() / w
    }
}

#[test]
fn b949_text_expression() {
    let mut t = Table::new(
        "text_animator",
        "# D-1069: Text Animator, the Expression Selector\n\nB-949. The title is \"Bounce\" with an \
         animator moving picked letters 200 pixels up, the Range Selector picking every letter, and \
         the Expression Selector on, with the expression a row gives on its Amount. Each number is \
         how much the animator picks a letter (1 is fully, 200 pixels up; 0 is in its place; below 0 \
         is below its place). FX-TXA-001 to 023 are unchanged and pass in B-230.\n",
    );
    let none = json!({});

    t.heading("Off is as before");
    let old = amounts(None, &json!({"expression_selector": null, "expression_amount": null, "end": 50, "smoothness": 0}), 0);
    let off = amounts(Some("0"), &json!({"expression_selector": "off", "end": 50, "smoothness": 0}), 0);
    t.row(
        "a file with no Expression words and one with it off (and an expression of 0) pick the same: End 50, the first half",
        &show(&old),
        old == off && old[..3].iter().all(|a| *a == 1.0) && old[3..].iter().all(|a| *a == 0.0),
    );

    t.heading("What the expression is told");
    let index = amounts(Some("textIndex*10"), &none, 0);
    t.row("textIndex*10: 10, 20, ... 60 per cent", &show(&index), near(&index, &[0.1, 0.2, 0.3, 0.4, 0.5, 0.6]));
    let total = amounts(Some("textTotal*10"), &none, 0);
    t.row("textTotal*10: six letters, 60 per cent each", &show(&total), total.iter().all(|a| (a - 0.6).abs() < 1e-9));
    let sel = amounts(Some("selectorValue[0]*textIndex/textTotal"), &json!({"end": 50, "smoothness": 0}), 0);
    t.row(
        "selectorValue[0]*textIndex/textTotal with End 50: the first half ramps, the rest 0",
        &show(&sel),
        near(&sel, &[1.0 / 6.0, 2.0 / 6.0, 0.5, 0.0, 0.0, 0.0]),
    );
    let list = amounts(Some("[30, 90, 90]"), &none, 0);
    t.row("a list answer takes its first number: 30 per cent", &show(&list), list.iter().all(|a| (a - 0.3).abs() < 1e-9));
    let held = amounts(Some("textIndex*50 - 200"), &none, 0);
    t.row(
        "answers past 100 or -100 are held there: -150, -100, -50, 0, 50, 100",
        &show(&held),
        near(&held, &[-1.0, -1.0, -0.5, 0.0, 0.5, 1.0]),
    );
    let plain = amounts(None, &json!({"expression_amount": 40}), 0);
    t.row("no expression: every letter takes the Amount, 40", &show(&plain), plain.iter().all(|a| (a - 0.4).abs() < 1e-9));
    let half = amounts(Some("textIndex*10"), &json!({"amount": 50}), 0);
    t.row(
        "the animator's own Amount 50 comes after: half of each",
        &show(&half),
        near(&half, &[0.05, 0.1, 0.15, 0.2, 0.25, 0.3]),
    );
    let ip = amounts(Some("inPoint*100 + thisLayer.inPoint*100 + 10"), &json!({}), 0);
    let ip_later = {
        let mut f = file(Some("inPoint*100"), &none);
        f["compositions"][0]["layers"][0]["in_frame"] = json!(12);
        let l = persist::load_str(&f.to_string()).unwrap();
        let comp = l.document.project().composition(&Id::new(MAIN)).unwrap().clone();
        let layer = comp.layer(&Id::new("art")).unwrap().clone();
        expr::selector(&comp, &layer.id, &layer.effects[0], 12, &[100.0]).unwrap()[0]
    };
    t.row(
        "inPoint is the layer's first frame in seconds: 0 here, 0.5 for a layer starting at frame 12",
        &format!("{} | {ip_later}", show(&ip)),
        ip.iter().all(|a| (a - 0.1).abs() < 1e-9) && (ip_later - 50.0).abs() < 1e-9,
    );
    let ex = amounts(Some("Math.exp(0)*20 + Math.exp(1)"), &none, 0);
    t.row(
        "Math.exp: e to the power, 20 + 2.718 per cent",
        &show(&ex),
        ex.iter().all(|a| (a - (20.0 + std::f64::consts::E) / 100.0).abs() < 1e-9),
    );

    t.heading("The tutorial's bounce, letter by letter (24 frames a second)");
    let mut all_ok = true;
    for f in [0, 1, 2, 3, 4, 6, 9, 14, 24, 40] {
        let got = amounts(Some(BOUNCE), &none, f);
        let want: Vec<f64> = (1..=6).map(|k| (bounce(k as f64, f as f64) / 100.0).clamp(-1.0, 1.0)).collect();
        let ok = near(&got, &want);
        all_ok &= ok;
        t.row(&format!("frame {f}"), &show(&got), ok);
    }
    let f2 = amounts(Some(BOUNCE), &none, 2);
    t.row(
        "at frame 2 the first letter has started down and the second has not",
        &show(&f2),
        f2[0] < 1.0 && f2[1] == 1.0,
    );
    let f8 = amounts(Some(BOUNCE), &none, 8);
    t.row(
        "the first letter overshoots below its place before it settles",
        &show(&f8),
        f8[0] < -0.05,
    );
    let f40 = amounts(Some(BOUNCE), &none, 40);
    t.row(
        "by frame 40 every letter has settled to within 1 per cent of its place",
        &show(&f40),
        all_ok && f40.iter().all(|a| a.abs() < 0.01),
    );

    t.heading("Errors");
    let (bad, failed) = picks(Some("Math.sqrt(textIndex - 7)"), &json!({"expression_amount": 30}), 0);
    t.row(
        "an expression giving not-a-number (the square root of a negative) puts every letter at the Amount (30) and is reported",
        &format!("{}; {}", show(&bad), failed.join("; ")),
        bad.iter().all(|a| (a - 0.3).abs() < 1e-9) && failed.len() == 1 && failed[0].starts_with("expression_amount"),
    );
    let (_, elsewhere) = picks(
        None,
        &json!({"rotation": {"base": 0, "keyframes": [], "expression": {"text": "textIndex", "enabled": true}}}),
        0,
    );
    t.row(
        "textIndex on another setting is an error that says where it has a value",
        &elsewhere.join("; "),
        elsewhere.len() == 1 && elsewhere[0].contains("Expression Selector"),
    );
    let refused = project(None, &json!({"expression_selector": "maybe"}));
    let valid = refused.composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].is_valid();
    t.row("an unknown Expression Selector word switches the animator off", &format!("runs: {valid}"), !valid);

    t.heading("Through compose, and saved");
    let still = frame(Some(BOUNCE), &none, 40);
    let rested = frame(None, &json!({"expression_amount": 0}), 40);
    let mean = |a: &WorkingBuffer, b: &WorkingBuffer| {
        a.data().iter().zip(b.data()).map(|(x, y)| (x - y).abs() as f64).sum::<f64>() / a.data().len() as f64
    };
    let settled = mean(&still, &rested);
    let moving = mean(&frame(Some(BOUNCE), &none, 4), &rested);
    t.row(
        "frame 40 of the bounce is nearly the letters at rest (under a pixel off, edges only); frame 4 is far from it",
        &format!("mean difference {settled:.5} at frame 40, {moving:.5} at frame 4"),
        settled < moving / 20.0,
    );
    t.row(
        "frame 2 of the bounce differs from frame 40",
        "compared",
        frame(Some(BOUNCE), &none, 2).data() != still.data(),
    );
    let saved = |e: Option<&str>, p: &J| {
        let l = load(e, p);
        let j: J = serde_json::from_str(&persist::to_json(l.document.project(), &l.preserved)).unwrap();
        j["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let on = saved(Some(BOUNCE), &none);
    t.row(
        "saved with it on, both words are written, the expression with them",
        &format!("selector {}, amount expression kept {}", on["expression_selector"], on["expression_amount"]["expression"]["text"] == BOUNCE),
        on["expression_selector"] == "on" && on["expression_amount"]["expression"]["text"] == BOUNCE && on.get("expression_values").is_none(),
    );
    let kept = saved(None, &json!({"expression_selector": null, "expression_amount": null}));
    t.row(
        "a file from before it saves again with no Expression words",
        "no expression_ words",
        kept.as_object().unwrap().keys().all(|k| !k.starts_with("expression")),
    );

    picture();
    t.row(
        "bounce_frames.png: frames 0, 2, 4, 6, 9 and 24 from the top; the letters drop in one after another and bounce",
        "drawn",
        true,
    );
    t.finish("B-949_text_expression_table.md");
}

/// Six frames of the bounce, one under the other on dark grey.
fn picture() {
    let rows = 0..H;
    let h = rows.len();
    let frames = [0, 2, 4, 6, 9, 24];
    let mut out = WorkingBuffer::opaque(W, h * frames.len());
    for (n, f) in frames.into_iter().enumerate() {
        let b = frame(Some(BOUNCE), &json!({}), f);
        for y in rows.clone() {
            for x in 0..W {
                let s = &b.data()[(y * W + x) * 4..][..4];
                let o = ((n * h + y - rows.start) * W + x) * 4;
                let d = out.data_mut();
                for k in 0..3 {
                    d[o + k] = s[k] + (1.0 - s[3]) * 0.02;
                }
                d[o + 3] = 1.0;
            }
        }
    }
    let dir = repo("verification/B-949 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("bounce_frames.png"), W, h * frames.len(), OutputDepth::Eight, &[], &out.to_srgb8_straight())
        .unwrap();
}
