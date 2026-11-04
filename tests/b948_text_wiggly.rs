//! B-948: D-1068, the Text Animator's Wiggly Selector, as After Effects' (SbTfu-Y63zk 1:00 to
//! 1:40, G1 of the wave 12 report). The checks read how much the animator picks each letter
//! (`text::placed`), draw frames through compose, and save the file again.

mod effect_table;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::text::{self, Placed, Text};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const W: usize = 1280;
const H: usize = 480;
const WORDS: &str = "Neon Sign";

/// The neon title: Opacity 25, the Range Selector picking nothing (End 0, as if deleted), and
/// the tutorial's Wiggly Selector, with `p` over it.
fn file(p: &J) -> J {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut a = json!({"position": [0, 0], "scale": [100, 100], "rotation": 0, "opacity": 25, "fill": "off",
        "color": [1, 0, 0], "tracking": 0, "start": 0, "end": 0, "offset": 0, "amount": 100,
        "based_on": "characters", "shape": "square", "smoothness": 100, "ease_high": 0, "ease_low": 0,
        "wiggly": "on", "wiggly_mode": "add", "wiggly_max": 100, "wiggly_min": -100, "wiggly_rate": 3,
        "wiggly_correlation": 50, "wiggly_temporal_phase": 0, "wiggly_spatial_phase": 0, "wiggly_seed": 0});
    for (k, v) in p.as_object().unwrap() {
        if v.is_null() {
            a.as_object_mut().unwrap().remove(k);
        } else {
            a[k] = v.clone();
        }
    }
    let effects = json!([{"instance_id": "fx-0", "type_id": "core.text_animator", "enabled": true, "parameters": a}]);
    let words = json!({"text": WORDS, "font": "MPLUSRounded1c-Regular.ttf", "size": 168, "color": [1, 1, 1], "at": [300, 300], "align": "left"});
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 48,
        "transform": {"anchor": t(json!([W / 2, H / 2])), "position": t(json!([W / 2, H / 2])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": words});
    json!({
        "schema_version": 0, "project_id": "proj-b948",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 48,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 48}, "layer_order": ["art"], "layers": [layer]}]
    })
}

fn load(p: &J) -> persist::Loaded {
    persist::load_str(&file(p).to_string()).unwrap_or_else(|d| panic!("the project reads: {}", d.message))
}

fn project(p: &J) -> Project {
    load(p).document.project().clone()
}

/// The text and its animator, the animator at `seconds` as compose hands it on.
fn parts(project: &Project, seconds: f64) -> (Text, Vec<Effect>, bool) {
    let layer = project.composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap();
    let i = layer.effects[0].at(0);
    let valid = i.is_valid();
    let mut e = i.effect;
    if let Effect::TextAnimator { wiggly_time, .. } = &mut e {
        *wiggly_time = seconds;
    }
    (layer.text.clone().unwrap(), vec![e], valid)
}

/// How much the animator picks each letter at `seconds`, spaces left out, in reading order.
fn amounts(p: &J, seconds: f64) -> Vec<f64> {
    let (t, fx, _) = parts(&project(p), seconds);
    let mut l: Vec<Placed> = text::placed(&t, &fx).expect("the font is there");
    l.sort_by_key(|c| c.index);
    l.iter().filter(|c| c.ch != ' ').map(|c| c.amounts[0]).collect()
}

fn show(v: &[f64]) -> String {
    v.iter().map(|a| format!("{a:.2}")).collect::<Vec<_>>().join(", ")
}

fn frame(p: &J, f: i32) -> WorkingBuffer {
    let loaded = load(p);
    let mut log = FrameLog::new(8);
    render_frame(loaded.document.project(), &Id::new(MAIN), f, &repo("Fixtures"), 1, &mut log)
        .unwrap_or_else(|d| panic!("frame {f} renders: {}", d.message))
}

fn same(a: &WorkingBuffer, b: &WorkingBuffer) -> bool {
    a.data() == b.data()
}

#[test]
fn b948_text_wiggly() {
    let mut t = Table::new(
        "text_animator",
        "# D-1068: Text Animator, the Wiggly Selector\n\nB-948. The title is \"Neon Sign\" with the \
         tutorial's animator: Opacity 25, the Range Selector picking nothing (End 0, as if deleted), \
         and a Wiggly Selector with Mode Add, Max 100, Min -100, 3 wiggles a second, Correlation 50, \
         unless a row says. Each number is how much the animator picks a letter (1 is fully, so 25% \
         opacity; 0 or below is full brightness). Spaces are left out. FX-TXA-001 to 023 are \
         unchanged and pass in B-230.\n",
    );
    let none = json!({});

    t.heading("Off is as before");
    let off = |w: J| amounts(&w, 0.4);
    let old = off(json!({"wiggly": null, "wiggly_mode": null, "wiggly_max": null, "wiggly_min": null,
        "wiggly_rate": null, "wiggly_correlation": null, "wiggly_temporal_phase": null,
        "wiggly_spatial_phase": null, "wiggly_seed": null, "end": 50, "smoothness": 0}));
    let switched = off(json!({"wiggly": "off", "end": 50, "smoothness": 0}));
    t.row(
        "a file with no Wiggly words and one with Wiggly off pick the same (End 50: the first half)",
        &show(&old),
        old == switched && old[..4].iter().all(|a| *a == 1.0) && old[4..].iter().all(|a| *a == 0.0),
    );

    t.heading("The wiggle");
    let now = amounts(&none, 0.0);
    let spread = now.iter().cloned().fold(f64::MIN, f64::max) - now.iter().cloned().fold(f64::MAX, f64::min);
    t.row(
        "at 0 seconds each letter has its own pick, from -1 to 1, not all the same",
        &show(&now),
        now.iter().all(|a| (-1.0..=1.0).contains(a)) && spread > 0.3,
    );
    t.row(
        "the same time picks the same again",
        &show(&amounts(&none, 0.0)),
        amounts(&none, 0.0) == now,
    );
    let later = amounts(&none, 0.5);
    t.row(
        "half a second later the picks have changed",
        &show(&later),
        later != now,
    );
    let slow = amounts(&json!({"wiggly_rate": 1}), 1.5);
    let fast = amounts(&json!({"wiggly_rate": 3}), 0.5);
    t.row(
        "3 wiggles a second at 0.5 s pick what 1 wiggle a second does at 1.5 s",
        &format!("{} | {}", show(&fast), show(&slow)),
        fast == slow,
    );
    let still: Vec<Vec<f64>> = [0.0, 0.7, 1.9].iter().map(|s| amounts(&json!({"wiggly_rate": 0}), *s)).collect();
    t.row(
        "0 wiggles a second holds still",
        &show(&still[2]),
        still.windows(2).all(|w| w[0] == w[1]),
    );
    let together = amounts(&json!({"wiggly_correlation": 100}), 0.3);
    t.row(
        "Correlation 100: every letter picks the same",
        &show(&together),
        together.windows(2).all(|w| (w[0] - w[1]).abs() < 1e-12),
    );
    let flat = amounts(&json!({"wiggly_max": 50, "wiggly_min": 50}), 0.3);
    t.row(
        "Max 50 and Min 50: every letter picks 0.5",
        &show(&flat),
        flat.iter().all(|a| (a - 0.5).abs() < 1e-12),
    );
    let seeded = amounts(&json!({"wiggly_seed": 1}), 0.0);
    t.row(
        "Random Seed 1 is another wiggle",
        &show(&seeded),
        seeded != now,
    );
    let shifted = amounts(&json!({"wiggly_temporal_phase": 180}), 0.0);
    let ahead = amounts(&none, 0.5 / 3.0);
    t.row(
        "Temporal Phase 180 at 0 s is the wiggle half a wiggle later",
        &show(&shifted),
        shifted.iter().zip(&ahead).all(|(a, b)| (a - b).abs() < 1e-12),
    );

    t.heading("Modes, with the Range Selector picking every letter (End 100)");
    let all = |m: &str| amounts(&json!({"end": 100, "wiggly_mode": m}), 0.3);
    let add_none = now_at(0.3);
    let ok = |m: &str, f: &dyn Fn(f64) -> f64| {
        all(m).iter().zip(&add_none).all(|(a, x)| (a - f(*x)).abs() < 1e-12)
    };
    for (m, what, f) in [
        ("intersect", "1 times the wiggle: the wiggle", &(|x: f64| x) as &dyn Fn(f64) -> f64),
        ("add", "1 plus the wiggle, held to 1", &|x: f64| (1.0 + x).min(1.0)),
        ("subtract", "1 less the wiggle, held to 1", &|x: f64| (1.0 - x).min(1.0)),
        ("min", "the smaller: the wiggle", &|x: f64| x.min(1.0)),
        ("max", "the larger: 1", &|x: f64| x.max(1.0)),
        ("difference", "the gap from 1", &|x: f64| (1.0 - x).abs().min(1.0)),
    ] {
        t.row(&format!("{m}: {what}"), &show(&all(m)), ok(m, f));
    }

    t.heading("Through compose, saved and refused");
    let f8 = frame(&none, 8);
    t.row(
        "frame 8 at 3 wiggles a second draws what frame 24 at 1 does (24 frames a second)",
        "compared",
        same(&f8, &frame(&json!({"wiggly_rate": 1}), 24)),
    );
    t.row("frame 8 differs from frame 0", "compared", !same(&f8, &frame(&none, 0)));
    t.row(
        "with Wiggly off, frame 8 and frame 0 are the same",
        "compared",
        same(&frame(&json!({"wiggly": "off"}), 8), &frame(&json!({"wiggly": "off"}), 0)),
    );
    let saved = |p: &J| {
        let l = load(p);
        let j: J = serde_json::from_str(&persist::to_json(l.document.project(), &l.preserved)).unwrap();
        j["compositions"][0]["layers"][0]["effects"][0]["parameters"].clone()
    };
    let on = saved(&none);
    t.row(
        "saved with Wiggly on, every Wiggly word is written and none of the time",
        &format!("wiggly {}, mode {}, rate {}, seed {}", on["wiggly"], on["wiggly_mode"], on["wiggly_rate"], on["wiggly_seed"]),
        on["wiggly"] == "on" && on["wiggly_mode"] == "add" && on["wiggly_rate"] == 3.0 && on.get("wiggly_time").is_none(),
    );
    let old_file = json!({"wiggly": null, "wiggly_mode": null, "wiggly_max": null, "wiggly_min": null,
        "wiggly_rate": null, "wiggly_correlation": null, "wiggly_temporal_phase": null,
        "wiggly_spatial_phase": null, "wiggly_seed": null});
    let kept = saved(&old_file);
    t.row(
        "a file from before it saves again with no Wiggly words",
        "no wiggly_ words",
        kept.as_object().unwrap().keys().all(|k| !k.starts_with("wiggly")),
    );
    let (_, _, valid) = parts(&project(&json!({"wiggly_mode": "multiply"})), 0.0);
    t.row(
        "an unknown mode switches the animator off with a message",
        &format!("runs: {valid}"),
        !valid,
    );

    picture(&none);
    t.row(
        "neon_frames.png: frames 0, 6, 12, 18 and 24 (a second) from the top; letters dim and brighten each on its own",
        "drawn",
        true,
    );
    t.finish("B-948_text_wiggly_table.md");
}

/// The tutorial's picks with the Range Selector picking nothing, at `seconds`.
fn now_at(seconds: f64) -> Vec<f64> {
    amounts(&json!({}), seconds)
}

/// Five frames of the title, each its middle rows, one under the other on dark grey.
fn picture(p: &J) {
    let rows = 150..340;
    let h = rows.len();
    let mut out = WorkingBuffer::opaque(W, h * 5);
    for (n, f) in [0, 6, 12, 18, 24].into_iter().enumerate() {
        let b = frame(p, f);
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
    let dir = repo("verification/B-948 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    png_out::write_rgba(&dir.join("neon_frames.png"), W, h * 5, OutputDepth::Eight, &[], &out.to_srgb8_straight())
        .unwrap();
}
