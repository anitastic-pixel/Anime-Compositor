//! B-870: D-990, the Text Animator's Blur (After Effects' animator Blur property): each letter
//! blurred by Blur times how much the Range Selector picks it, a Gaussian of sigma half the Blur.
//! oTvDbIZsQoE (11:12 to 11:45) brings its title in this way. Adobe publishes no formula for the
//! property, so the checks are what the rule says, not numbers from After Effects.

mod effect_table;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::text::{self, Text};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const W: usize = 1280;
const H: usize = 360;

fn file(animators: &[J]) -> J {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects: Vec<J> = animators
        .iter()
        .enumerate()
        .map(|(i, p)| json!({"instance_id": format!("fx-{i}"), "type_id": "core.text_animator", "enabled": true, "parameters": p}))
        .collect();
    let words = json!({"text": "BLUR IN", "font": "MPLUSRounded1c-Regular.ttf", "size": 160, "color": [1, 1, 1], "at": [160, 240], "align": "left"});
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 24,
        "transform": {"anchor": t(json!([W / 2, H / 2])), "position": t(json!([W / 2, H / 2])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": words});
    json!({
        "schema_version": 0, "project_id": "proj-b870",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24}, "layer_order": ["art"], "layers": [layer]}]
    })
}

fn animator(blur: Option<[f64; 2]>, start: f64) -> J {
    let mut a = json!({"position": [0, 0], "scale": [100, 100], "rotation": 0, "opacity": 100, "fill": "off",
        "color": [1, 0, 0], "tracking": 0, "start": start, "end": 100, "offset": 0, "amount": 100,
        "based_on": "characters", "shape": "square", "smoothness": 0, "ease_high": 0, "ease_low": 0});
    if let Some(b) = blur {
        a["blur"] = json!(b);
    }
    a
}

fn load(p: &J) -> Project {
    persist::load_str(&p.to_string())
        .unwrap_or_else(|d| panic!("the project reads: {}", d.message))
        .document
        .project()
        .clone()
}

fn parts(project: &Project) -> (Text, Vec<Effect>) {
    let layer = project
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap();
    (
        layer.text.clone().unwrap(),
        layer.effects.iter().map(|i| i.effect.clone()).collect(),
    )
}

fn drawn(blur: Option<[f64; 2]>, start: f64) -> WorkingBuffer {
    let (words, fx) = parts(&load(&file(&[animator(blur, start)])));
    text::animated(&words, &fx, W, H).expect("the font is there")
}

fn saved(p: &J) -> J {
    effect_table::saved(&persist::load_str(&p.to_string()).unwrap())
}

/// The columns and rows holding alpha over 1/255, as left, top, right, bottom.
fn reach(b: &WorkingBuffer) -> [usize; 4] {
    let mut r = [usize::MAX, usize::MAX, 0, 0];
    for y in 0..b.height() {
        for x in 0..b.width() {
            if b.pixel(x, y)[3] > 1.0 / 255.0 {
                r = [r[0].min(x), r[1].min(y), r[2].max(x), r[3].max(y)];
            }
        }
    }
    r
}

fn alpha_sum(b: &WorkingBuffer) -> f64 {
    b.data().chunks(4).map(|p| p[3] as f64).sum()
}

fn most(b: &WorkingBuffer) -> f32 {
    b.data().chunks(4).map(|p| p[3]).fold(0.0, f32::max)
}

/// Whether columns `x0..x1` of `a` and `b` are the same to the bit.
fn same_columns(a: &WorkingBuffer, b: &WorkingBuffer, x0: usize, x1: usize) -> bool {
    (0..a.height()).all(|y| (x0..x1).all(|x| a.pixel(x, y) == b.pixel(x, y)))
}

fn picture(file: &str, b: &WorkingBuffer) {
    let dir = repo("verification/B-870 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let mut over = WorkingBuffer::opaque(b.width(), b.height());
    for (o, p) in over.data_mut().chunks_mut(4).zip(b.data().chunks(4)) {
        for k in 0..3 {
            o[k] = p[k] + (1.0 - p[3]) * 0.02;
        }
        o[3] = 1.0;
    }
    png_out::write_rgba(
        &dir.join(format!("{file}.png")),
        b.width(),
        b.height(),
        OutputDepth::Eight,
        &[],
        &over.to_srgb8_straight(),
    )
    .unwrap();
}

#[test]
fn b870_text_animator_blur() {
    let mut t = Table::new(
        "text_animator",
        "# B-870: the Text Animator's Blur\n\nD-990: After Effects' animator Blur, x then y in \
         pixels, each letter blurred by Blur times how much the Range Selector picks it (a \
         Gaussian of sigma half the Blur, as the text shadow's Softness). The words are \"BLUR \
         IN\" at 160 pixels on a 1280 by 360 frame. Pictures are in `verification/B-870 \
         pictures/`.\n",
    );
    let sharp = drawn(None, 0.0);

    t.heading("A file from before Blur");
    let old = file(&[animator(None, 0.0)]);
    let (_, fx) = parts(&load(&old));
    let read = matches!(fx[0], Effect::TextAnimator { blur, .. } if blur == [0.0; 2]);
    t.row("reads with Blur 0, 0", &read.to_string(), read);
    let none = !saved(&old).to_string().contains("\"blur\"");
    t.row("saves with no blur key, as before", &none.to_string(), none);
    let zero = drawn(Some([0.0, 0.0]), 0.0);
    let same = zero.data() == sharp.data();
    t.row("Blur 0, 0 draws the same pixels as no Blur", &same.to_string(), same);

    t.heading("Blur 24, 24 on every letter");
    let soft = drawn(Some([24.0, 24.0]), 0.0);
    picture("blur_none", &sharp);
    picture("blur_24_all", &soft);
    let (a, b) = (alpha_sum(&sharp), alpha_sum(&soft));
    let kept = (b / a - 1.0).abs() < 0.01;
    t.row(
        "the letters' coverage is kept (within 1 per cent)",
        &format!("{:.4} of the sharp", b / a),
        kept,
    );
    let (m0, m1) = (most(&sharp), most(&soft));
    t.row(
        "no pixel is fully covered any more",
        &format!("{m0:.3} then {m1:.3}"),
        m0 > 0.99 && m1 < 0.95,
    );
    let (r0, r1) = (reach(&sharp), reach(&soft));
    let grew = r1[0] + 20 < r0[0] && r1[1] + 20 < r0[1] && r1[2] > r0[2] + 20 && r1[3] > r0[3] + 20;
    t.row(
        "the letters spread past their edges on all four sides",
        &format!("{r0:?} then {r1:?}"),
        grew,
    );

    t.heading("Blur 24, 0: across only");
    let across = drawn(Some([24.0, 0.0]), 0.0);
    picture("blur_24_0_all", &across);
    let r2 = reach(&across);
    let ok = r2[1] == r0[1] && r2[3] == r0[3] && r2[0] + 20 < r0[0] && r2[2] > r0[2] + 20;
    t.row(
        "the rows reached stay; the columns spread",
        &format!("{r0:?} then {r2:?}"),
        ok,
    );

    t.heading("Start at 60 per cent: the selector picks the last letters only");
    let half = drawn(Some([24.0, 24.0]), 60.0);
    picture("blur_24_start_60", &half);
    let (words, fx) = parts(&load(&file(&[animator(Some([24.0, 24.0]), 60.0)])));
    let placed = text::placed(&words, &fx).unwrap();
    let blurs: Vec<String> = placed
        .iter()
        .map(|p| format!("{}{}", p.ch, p.blur[0]))
        .collect();
    let want = ["B0", "L0", "U0", "R0", " 24", "I24", "N24"];
    t.row(
        "each letter's blur",
        &blurs.join(" "),
        blurs.iter().map(String::as_str).eq(want),
    );
    // The first two letters are well clear of the blurred ones' spread.
    let l = placed.iter().find(|p| p.ch == 'U').unwrap().bounds.unwrap();
    let edge = l[0].floor() as usize;
    let sharp_left = same_columns(&half, &sharp, 0, edge);
    t.row(
        "\"B\" and \"L\" are the sharp pixels to the bit",
        &sharp_left.to_string(),
        sharp_left,
    );
    let n = placed.iter().find(|p| p.ch == 'N').unwrap().bounds.unwrap();
    let blurred_right = !same_columns(&half, &sharp, n[0] as usize, n[2] as usize);
    t.row("\"N\" is blurred", &blurred_right.to_string(), blurred_right);

    t.heading("Saved, read again, and set");
    let with = file(&[animator(Some([24.0, 12.0]), 0.0)]);
    let back = saved(&with)["compositions"][0]["layers"][0]["effects"][0]["parameters"]["blur"].clone();
    t.row("Blur 24, 12 is saved", &back.to_string(), back == json!([24, 12]));
    let again = load(&saved(&with));
    let (_, fx) = parts(&again);
    let kept = matches!(fx[0], Effect::TextAnimator { blur, .. } if blur == [24.0, 12.0]);
    t.row("and read again as it was", &kept.to_string(), kept);
    let big = animator(Some([1200.0, 0.0]), 0.0);
    let (_, fx) = parts(&load(&file(&[big])));
    let refused = !fx[0].is_valid();
    t.row(
        "Blur past 1,000 pixels is refused, not drawn",
        &refused.to_string(),
        refused,
    );
    // The tutorial's own settings at 11:38: Tracking Amount 50, Blur 37, 37, Start part way.
    let mut cine = file(&[]);
    let mut a = animator(Some([37.0, 37.0]), 56.0);
    a["tracking"] = json!(50);
    let layer = &mut cine["compositions"][0]["layers"][0];
    layer["source_text"]["text"] = json!("CINEMATIC");
    layer["source_text"]["size"] = json!(110);
    layer["source_text"]["at"] = json!([300, 220]);
    layer["effects"] = json!([{"instance_id": "fx-0", "type_id": "core.text_animator", "enabled": true, "parameters": a}]);
    let (words, fx) = parts(&load(&cine));
    picture("tutorial_cinematic", &text::animated(&words, &fx, W, H).unwrap());
    t.finish("B-870_text_animator_blur_table.md");
}
