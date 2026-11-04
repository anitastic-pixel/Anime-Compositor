//! B-947: D-1067, the Text Animator's Based On Lines and Ease High and Low from -100 to 100, as
//! After Effects' Range Selector (iOknL8pPZGg 1:20 to 1:50, G3 of the wave 12 report). The checks
//! read how much the selector picks each letter (`text::placed`) and draw the two-line title.

mod effect_table;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::effects::Effect;
use anime_compositor::model::{Id, Project};
use anime_compositor::text::{self, Placed, Text};
use anime_compositor::{persist, png_out, OutputDepth, WorkingBuffer};

const W: usize = 1280;
const H: usize = 480;

fn file(words: &str, p: &J) -> J {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let mut a = json!({"position": [0, 120], "scale": [100, 100], "rotation": 0, "opacity": 0, "fill": "off",
        "color": [1, 0, 0], "tracking": 0, "start": 0, "end": 100, "offset": 0, "amount": 100,
        "based_on": "lines", "shape": "square", "smoothness": 0, "ease_high": 0, "ease_low": 0});
    for (k, v) in p.as_object().unwrap() {
        a[k] = v.clone();
    }
    let effects = json!([{"instance_id": "fx-0", "type_id": "core.text_animator", "enabled": true, "parameters": a}]);
    let words = json!({"text": words, "font": "MPLUSRounded1c-Regular.ttf", "size": 140, "color": [1, 1, 1], "at": [120, 180], "align": "left"});
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 24,
        "transform": {"anchor": t(json!([W / 2, H / 2])), "position": t(json!([W / 2, H / 2])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": words});
    json!({
        "schema_version": 0, "project_id": "proj-b947",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": W, "height": H, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 24,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 24}, "layer_order": ["art"], "layers": [layer]}]
    })
}

fn load(p: &J) -> Result<Project, String> {
    persist::load_str(&p.to_string())
        .map(|l| l.document.project().clone())
        .map_err(|d| d.message)
}

fn parts(project: &Project) -> (Text, Vec<Effect>) {
    let layer = project
        .composition(&Id::new(MAIN))
        .unwrap()
        .layer(&Id::new("art"))
        .unwrap();
    (
        layer.text.clone().unwrap(),
        layer
            .effects
            .iter()
            .filter(|i| i.is_valid())
            .map(|i| i.at(0).effect)
            .collect(),
    )
}

/// The letters laid out, in the order the words are read.
fn letters(words: &str, p: &J) -> Vec<Placed> {
    let (t, fx) = parts(&load(&file(words, p)).expect("the project reads"));
    let mut l = text::placed(&t, &fx).expect("the font is there");
    l.sort_by_key(|c| c.index);
    l
}

/// How much the animator picks each letter, as "A 1.00", in reading order.
fn picks(l: &[Placed]) -> String {
    l.iter()
        .map(|c| format!("{} {:.2}", c.ch, c.amounts[0]))
        .collect::<Vec<_>>()
        .join(", ")
}

fn picture(name: &str, words: &str, p: &J) {
    let (t, fx) = parts(&load(&file(words, p)).unwrap());
    let b = text::animated(&t, &fx, W, H).expect("the font is there");
    let dir = repo("verification/B-947 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let mut over = WorkingBuffer::opaque(W, H);
    for (o, p) in over.data_mut().chunks_mut(4).zip(b.data().chunks(4)) {
        for k in 0..3 {
            o[k] = p[k] + (1.0 - p[3]) * 0.02;
        }
        o[3] = 1.0;
    }
    png_out::write_rgba(&dir.join(format!("{name}.png")), W, H, OutputDepth::Eight, &[], &over.to_srgb8_straight())
        .unwrap();
}

#[test]
fn b947_text_lines_ease() {
    let mut t = Table::new(
        "text_animator",
        "# D-1067: Text Animator, Based On Lines and Ease from -100 to 100\n\nB-947. The animator \
         here moves picked letters down 120 pixels and fades them out, Shape Square unless a row \
         says, so a picked letter is gone. FX-TXA-001 to 023 are unchanged and pass in B-230.\n",
    );
    let two = "LIQUID\nDESIGN";

    t.heading("Based On Lines");
    let l = letters(two, &json!({"end": 50}));
    t.row(
        "two lines, Square, Start 0, End 50: the first line all picked, the second not",
        &picks(&l),
        l.iter().all(|c| c.amounts[0] == if c.index < 6 { 1.0 } else { 0.0 }),
    );
    let l = letters("ONE TWO\nTHREE", &json!({"start": 50}));
    t.row(
        "a line's spaces belong to it: \"ONE TWO\" then \"THREE\", Start 50: only THREE picked",
        &picks(&l),
        l.iter().all(|c| c.amounts[0] == if c.index >= 7 { 1.0 } else { 0.0 }),
    );
    let l = letters(two, &json!({"based_on": "words", "end": 50}));
    t.row(
        "Based On Words on the same title: the line break ends LIQUID, so one word of two is picked (before D-1067 both were one word)",
        &picks(&l),
        l.iter().all(|c| c.amounts[0] == if c.index < 6 { 1.0 } else { 0.0 }),
    );
    picture("1_lines_first_picked", two, &json!({"end": 50}));
    picture("2_lines_offset_half", two, &json!({"end": 50, "offset": 50}));
    t.row(
        "1_lines_first_picked.png: DESIGN only (LIQUID moved away and faded); 2_lines_offset_half.png: LIQUID only",
        "drawn",
        true,
    );

    t.heading("Ease from -100 to 100 (Ramp Up over eight letters, the third letter)");
    let word = "ABCDEFGH";
    let at = |p: J| letters(word, &p)[2].amounts[0];
    let ramp = json!({"based_on": "characters", "shape": "ramp_up"});
    let with = |k: &str, v: f64| {
        let mut p = ramp.clone();
        p[k] = J::from(v);
        p
    };
    let (none, plus, minus) = (at(ramp.clone()), at(with("ease_low", 50.0)), at(with("ease_low", -50.0)));
    t.row(
        "Ease Low 50 picks the third letter less than no ease, Ease Low -50 more",
        &format!("Ease Low -50: {minus:.3}; 0: {none:.3}; 50: {plus:.3}"),
        minus > none && none > plus,
    );
    let (hplus, hminus) = (at(with("ease_high", 50.0)), at(with("ease_high", -50.0)));
    t.row(
        "Ease High 50 picks it more than no ease, Ease High -50 less",
        &format!("Ease High -50: {hminus:.3}; 0: {none:.3}; 50: {hplus:.3}"),
        hplus > none && none > hminus,
    );
    let valid = |p: J| {
        let project = load(&file(word, &p)).unwrap();
        let c = project.composition(&Id::new(MAIN)).unwrap();
        c.layer(&Id::new("art")).unwrap().effects[0].is_valid()
    };
    let (edge, far) = (valid(with("ease_low", -100.0)), valid(with("ease_low", -150.0)));
    t.row(
        "Ease Low -100 is allowed; -150 is outside -100..100, so the animator is switched off with a message",
        &format!("-100 runs: {edge}; -150 runs: {far}"),
        edge && !far,
    );
    let all: Vec<f64> = (0..8).map(|k| letters(word, &with("ease_low", -80.0))[k].amounts[0]).collect();
    t.row(
        "Ease Low -80: the pick still rises letter by letter, from 0 to 1",
        &format!("{all:.3?}"),
        all.windows(2).all(|w| w[1] >= w[0]) && all[7] <= 1.0 && all[0] >= 0.0,
    );
    t.finish("B-947_text_lines_ease_table.md");
}
