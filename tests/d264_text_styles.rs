//! D-264: text styled as After Effects, Premiere Pro and DaVinci Resolve style it.
//!
//! Writes `verification/D-264_text_styles_table.md`.
//!
//! The owner asked on 2026-10-02 for the Text tool and its properties to be "similar to
//! Premiere/Davinci/AE's text tools". These are the engine half's checks:
//!
//! 1. The new settings save and read back exactly, are written only when they are used (so a
//!    D-263 text record saves as it did), and what is outside their ranges is refused.
//! 2. Character: tracking, the font's own kerning, leading, all caps, faux bold and faux italic
//!    each move or change the letters by what they say.
//! 3. Paragraph: a box the words wrap in, Japanese between any two characters, and left, centre,
//!    right and justified lines in it.
//! 4. Appearance: a stroke round the letters, a box behind them and a shadow, in that order under
//!    the fill.
//! 5. A D-263 text layer is drawn byte for byte as 2edc62d drew it, the fonts can be listed by
//!    name, a styled layer goes into an exported frame, and every fixture project saves byte for
//!    byte as on e395d8b.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

use anime_compositor::command::{Command, Document};
use anime_compositor::export::{export_sequence, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::model::{Id, Layer, Project};
use anime_compositor::persist::Preserved;
use anime_compositor::text::{self, Align, Background, Shadow, Stroke, Text};
use anime_compositor::{persist, sha256, OutputAlpha, OutputDepth, WorkingBuffer};

/// Taken on e395d8b (D-253's check), as D-261's and D-263's checks have them.
const FIXTURES_OPENED: usize = 2297;
const FIXTURES_SAVED: &str = "aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d";

/// Three D-263 pictures, taken on 2edc62d: every pixel's four numbers, SHA-256.
const D263_PICTURES: [(&str, [f64; 2], Align, &str); 3] = [
    ("Text \u{3042}\nCut 012", [200.0, 600.0], Align::Left, "d87262ee1bfc63b078b0e2e930113843efc92b8fc94bee1907a42cab23eeb845"),
    ("AV To\nWAVE", [960.0, 500.0], Align::Center, "bb6976575e3dee8c49101f80f5981b9b16a23b24272b0e8e95e05f780dc604df"),
    ("Right", [1700.0, 300.0], Align::Right, "adbbacf2ffcf3bc6f2257c1852275352caa9b4e89e5f3efc3e40fd3479d21b85"),
];

const W: usize = 1920;
const H: usize = 1080;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn projects(dir: &Path, found: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            projects(&path, found);
        } else if path.extension().is_some_and(|e| e == "json") {
            found.push(path);
        }
    }
}

fn words(text: &str, size: f64, at: [f64; 2], align: Align) -> Text {
    Text { text: text.into(), size, color: [1.0, 0.9, 0.2], at, align, ..Text::default() }
}

fn draw(t: &Text) -> WorkingBuffer {
    text::draw(t, W, H).unwrap_or_else(|| WorkingBuffer::transparent(W, H))
}

/// Where the ink is: the leftmost, rightmost, top and bottom pixel with any cover, and how many.
fn ink(picture: &WorkingBuffer) -> Option<(usize, usize, usize, usize, usize)> {
    let w = picture.width();
    let mut found: Option<(usize, usize, usize, usize, usize)> = None;
    for (i, px) in picture.data().chunks_exact(4).enumerate() {
        if px[3] > 0.0 {
            let (x, y) = (i % w, i / w);
            found = Some(match found {
                None => (x, x, y, y, 1),
                Some((l, r, t, b, n)) => (l.min(x), r.max(x), t.min(y), b.max(y), n + 1),
            });
        }
    }
    found
}

fn box_of(p: &WorkingBuffer) -> String {
    ink(p).map(|(l, r, t, b, n)| format!("{l}..{r} across, {t}..{b} down, {n} pixels")).unwrap_or("no ink".into())
}

/// One pixel's four numbers, premultiplied, rounded to two places.
fn at(p: &WorkingBuffer, x: usize, y: usize) -> [f32; 4] {
    let i = (y * p.width() + x) * 4;
    let d = &p.data()[i..i + 4];
    [d[0], d[1], d[2], d[3]].map(|v| (v * 100.0).round() / 100.0)
}

/// The leftmost and rightmost inked pixel on one row.
fn row_ink(p: &WorkingBuffer, y: usize) -> Option<(usize, usize)> {
    let w = p.width();
    let row = &p.data()[y * w * 4..(y + 1) * w * 4];
    let inked: Vec<usize> = (0..w).filter(|x| row[x * 4 + 3] > 0.0).collect();
    Some((*inked.first()?, *inked.last()?))
}

/// How many separate runs of inked rows there are: one per line of words.
fn lines(p: &WorkingBuffer) -> usize {
    let w = p.width();
    let inked: Vec<bool> = p.data().chunks_exact(w * 4).map(|row| row.chunks_exact(4).any(|px| px[3] > 0.0)).collect();
    inked.windows(2).filter(|pair| !pair[0] && pair[1]).count() + usize::from(inked.first() == Some(&true))
}

fn hash(p: &WorkingBuffer) -> String {
    let bytes: Vec<u8> = p.data().iter().flat_map(|v| v.to_le_bytes()).collect();
    sha256::hex(&bytes)
}

fn near(v: f64, want: f64, by: f64) -> bool {
    (v - want).abs() <= by
}

#[test]
fn d264_text_is_styled_as_the_editors_style_it() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| rows.push((what.into(), expected.into(), actual));
    let yes = |ok: bool, otherwise: String| if ok { "yes".to_string() } else { otherwise };
    let said = |r: Result<(), String>| if r.is_ok() { "taken".to_string() } else { "refused".to_string() };

    // Row 1, the record.
    let mut doc = persist::load(&repo("verification/B-08a_project.json")).unwrap_or_else(|d| panic!("{}", d.message)).document;
    let without = doc.project().clone();
    let composition = without.compositions[0].id.clone();
    let apply = |doc: &mut Document, command: Command| doc.apply(command).map(|_| ()).map_err(|d| d.message);
    let title = Id::new("layer-text");
    let plain = words("Cut 012", 120.0, [200.0, 600.0], Align::Left);
    let layer = Layer::text(title.clone(), "Text 1", plain.clone(), 1920, 1080, 0, 240);
    let _ = apply(&mut doc, Command::AddLayer { composition: composition.clone(), layer: Box::new(layer), index: 4 });
    let saved = persist::to_json(doc.project(), &Preserved::none());
    let value: serde_json::Value = serde_json::from_str(&saved).unwrap();
    let record = |v: &serde_json::Value| {
        v["compositions"][0]["layers"].as_array().unwrap().iter().find(|l| l["id"] == "layer-text").map(|l| l["source_text"].clone()).unwrap_or_default()
    };
    let mut keys: Vec<String> = record(&value).as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
    keys.sort();
    row(
        "A text layer using none of the new settings saves only D-263's six lines",
        "align, at, color, font, size, text",
        keys.join(", "),
    );
    let set = |text: Text| Command::SetText { composition: composition.clone(), layer_id: title.clone(), text };
    let styled = Text {
        text: "Cut 012\nあいう".into(),
        align: Align::Justify,
        tracking: 50.0,
        leading: 160.0,
        kerning: true,
        all_caps: true,
        faux_bold: true,
        faux_italic: true,
        box_width: 600.0,
        stroke: Some(Stroke { color: [0.0, 0.0, 0.0], width: 6.0 }),
        background: Some(Background { color: [0.0, 0.1, 0.4], opacity: 0.75, padding: 24.0, roundness: 12.0 }),
        shadow: Some(Shadow { color: [0.0, 0.0, 0.0], opacity: 0.5, angle: 135.0, distance: 8.0, softness: 6.0 }),
        ..plain.clone()
    };
    row("Every new setting at once is taken", "taken", said(apply(&mut doc, set(styled.clone()))));
    let saved = persist::to_json(doc.project(), &Preserved::none());
    let value: serde_json::Value = serde_json::from_str(&saved).unwrap();
    let r = record(&value);
    row(
        "Saved, they read back as set",
        "justify, 50, 160, true, true, true, true, 600, {\"color\":[0,0,0],\"width\":6}, {\"color\":[0,0.1,0.4],\"opacity\":0.75,\"padding\":24,\"roundness\":12}, {\"angle\":135,\"color\":[0,0,0],\"distance\":8,\"opacity\":0.5,\"softness\":6}",
        format!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
            r["align"].as_str().unwrap_or("?"),
            r["tracking"],
            r["leading"],
            r["kerning"],
            r["all_caps"],
            r["faux_bold"],
            r["faux_italic"],
            r["box_width"],
            r["stroke"],
            r["background"],
            r["shadow"],
        ),
    );
    match persist::load_str(&saved) {
        Ok(back) => {
            let same = back.document.project().compositions[0].layer(&title).and_then(|l| l.text.clone()) == Some(styled.clone());
            row("Opened again, the text is the same", "the same", if same { "the same" } else { "different" }.into());
            row(
                "Saved again, the file is the same, byte for byte",
                "the same",
                if persist::to_json(back.document.project(), &back.preserved) == saved { "the same" } else { "different" }.into(),
            );
        }
        Err(d) => {
            row("Opened again, the text is the same", "the same", d.message.clone());
            row("Saved again, the file is the same, byte for byte", "the same", d.message);
        }
    }
    let mut later = value.clone();
    for l in later["compositions"][0]["layers"].as_array_mut().unwrap() {
        if l["id"] == "layer-text" {
            l["source_text"]["stroke"]["join"] = serde_json::json!("round");
            l["source_text"]["shadow"]["spread"] = serde_json::json!(3);
        }
    }
    let kept = match persist::load_str(&later.to_string()) {
        Ok(k) => {
            let again: serde_json::Value = serde_json::from_str(&persist::to_json(k.document.project(), &k.preserved)).unwrap();
            let r = record(&again);
            if r["stroke"]["join"] == "round" && r["shadow"]["spread"] == 3 { "kept".to_string() } else { r.to_string() }
        }
        Err(d) => d.message,
    };
    row("A line inside the stroke or the shadow that no build writes yet is kept", "kept", kept);
    let _ = apply(&mut doc, set(Text { stroke: None, background: None, shadow: None, ..styled.clone() }));
    let r = record(&serde_json::from_str(&persist::to_json(doc.project(), &Preserved::none())).unwrap());
    row(
        "Stroke, background and shadow taken off are no longer written",
        "none written",
        match ["stroke", "background", "shadow"].iter().filter(|k| r.get(**k).is_some()).copied().collect::<Vec<_>>() {
            v if v.is_empty() => "none written".into(),
            v => v.join(", "),
        },
    );
    doc.undo();
    let now = |doc: &Document| doc.project().compositions[0].layer(&title).and_then(|l| l.text.clone());
    row("Undo puts them back", "back", if now(&doc) == Some(styled.clone()) { "back".into() } else { format!("{:?}", now(&doc)) });
    let stroke = Stroke { color: [0.0; 3], width: 6.0 };
    let background = Background { color: [0.0; 3], opacity: 0.5, padding: 10.0, roundness: 0.0 };
    let shadow = Shadow { color: [0.0; 3], opacity: 0.5, angle: 135.0, distance: 5.0, softness: 5.0 };
    for (what, bad) in [
        ("Tracking of 5000 is refused (-1000 to 1000)", Text { tracking: 5000.0, ..plain.clone() }),
        ("Leading below 0 is refused", Text { leading: -10.0, ..plain.clone() }),
        ("A box narrower than 0 is refused", Text { box_width: -1.0, ..plain.clone() }),
        ("A stroke 0 pixels wide is refused", Text { stroke: Some(Stroke { width: 0.0, ..stroke.clone() }), ..plain.clone() }),
        ("A stroke colour above 1 is refused", Text { stroke: Some(Stroke { color: [2.0, 0.0, 0.0], ..stroke.clone() }), ..plain.clone() }),
        ("A background opacity above 1 is refused", Text { background: Some(Background { opacity: 1.5, ..background.clone() }), ..plain.clone() }),
        ("A background padding below 0 is refused", Text { background: Some(Background { padding: -1.0, ..background.clone() }), ..plain.clone() }),
        ("A shadow opacity below 0 is refused", Text { shadow: Some(Shadow { opacity: -0.1, ..shadow.clone() }), ..plain.clone() }),
        ("A shadow distance below 0 is refused", Text { shadow: Some(Shadow { distance: -5.0, ..shadow.clone() }), ..plain.clone() }),
        ("A shadow softness of 1000 is refused (0 to 500)", Text { shadow: Some(Shadow { softness: 1000.0, ..shadow.clone() }), ..plain.clone() }),
        ("A shadow angle that is not a number is refused", Text { shadow: Some(Shadow { angle: f64::NAN, ..shadow.clone() }), ..plain.clone() }),
    ] {
        row(what, "refused", said(apply(&mut doc, set(bad))));
    }
    let refused = |change: &dyn Fn(&mut serde_json::Value)| {
        let mut v = value.clone();
        for l in v["compositions"][0]["layers"].as_array_mut().unwrap() {
            if l["id"] == "layer-text" {
                change(&mut l["source_text"]);
            }
        }
        if persist::load_str(&v.to_string()).is_err() { "refused".to_string() } else { "opened".to_string() }
    };
    row("A file aligning \"justified\", which is no alignment, is refused", "refused", refused(&|t| t["align"] = serde_json::json!("justified")));
    row("A file with tracking \"wide\" is refused", "refused", refused(&|t| t["tracking"] = serde_json::json!("wide")));
    row("A file with a stroke with no width is refused", "refused", refused(&|t| { t["stroke"].as_object_mut().unwrap().remove("width"); }));
    row("A file with kerning 1 rather than true or false is refused", "refused", refused(&|t| t["kerning"] = serde_json::json!(1)));

    // Row 2, character.
    let size = 100.0;
    let width = |t: &Text| ink(&draw(t)).map(|(l, r, ..)| (r - l) as f64).unwrap_or(0.0);
    let hhhh = words("HHHH", size, [100.0, 400.0], Align::Left);
    let spread = width(&Text { tracking: 100.0, ..hhhh.clone() }) - width(&hhhh);
    row(
        "Tracking 100 (thousandths of the size) puts 10 pixels after each of HHHH's first three letters: 30 wider",
        "yes",
        yes(near(spread, 30.0, 2.0), format!("{spread} wider")),
    );
    let av = words("AV", size, [100.0, 400.0], Align::Left);
    let pulled = width(&av) - width(&Text { kerning: true, ..av.clone() });
    row(
        "With kerning, A and V close up by the font's own 100 units of 1000: 10 pixels",
        "yes",
        yes(near(pulled, 10.0, 1.5), format!("{pulled} closer")),
    );
    let hh = words("HH", size, [100.0, 400.0], Align::Left);
    row(
        "Kerning changes nothing between letters the font does not kern",
        "the same",
        if hash(&draw(&hh)) == hash(&draw(&Text { kerning: true, ..hh.clone() })) { "the same" } else { "different" }.into(),
    );
    let one = draw(&words("H", size, [100.0, 300.0], Align::Left));
    let two = draw(&Text { leading: 200.0, ..words("H\nH", size, [100.0, 300.0], Align::Left) });
    let drop = match (ink(&one), ink(&two)) {
        (Some((_, _, _, b1, _)), Some((_, _, _, b2, _))) => b2 as f64 - b1 as f64,
        _ => 0.0,
    };
    row("Leading 200 puts the second line's baseline 200 pixels below the first", "yes", yes(near(drop, 200.0, 1.0), format!("{drop} below")));
    row(
        "All caps draws \"abc\" byte for byte as \"ABC\"",
        "the same",
        if hash(&draw(&Text { all_caps: true, ..words("abc", size, [100.0, 400.0], Align::Left) })) == hash(&draw(&words("ABC", size, [100.0, 400.0], Align::Left))) {
            "the same"
        } else {
            "different"
        }
        .into(),
    );
    let upright = draw(&words("I", size, [100.0, 400.0], Align::Left));
    let leaning = draw(&Text { faux_italic: true, ..words("I", size, [100.0, 400.0], Align::Left) });
    let lean = match (ink(&leaning), ink(&upright)) {
        (Some((_, _, t, b, _)), Some(_)) => match (row_ink(&leaning, t + 2), row_ink(&leaning, b - 2)) {
            (Some((tl, tr)), Some((bl, br))) => (tl + tr) as f64 / 2.0 - (bl + br) as f64 / 2.0,
            _ => 0.0,
        },
        _ => 0.0,
    };
    row("Faux italic leans an I: its top is at least 10 pixels right of its foot", "yes", yes(lean >= 10.0, format!("{lean} pixels")));
    let heavy = draw(&Text { faux_bold: true, ..words("I", size, [100.0, 400.0], Align::Left) });
    row(
        "Faux bold thickens an I: a fifth more ink or better, its middle where it was",
        "yes",
        match (ink(&heavy), ink(&upright)) {
            (Some((hl, hr, _, _, hn)), Some((ul, ur, _, _, un))) => {
                yes(hn as f64 >= un as f64 * 1.2 && near((hl + hr) as f64 / 2.0, (ul + ur) as f64 / 2.0, 1.0), format!("{hn} against {un} pixels"))
            }
            _ => "no ink".into(),
        },
    );

    // Row 3, paragraph.
    let boxed = |words_: &str, align: Align| Text { box_width: 400.0, ..words(words_, size * 0.6, [100.0, 200.0], align) };
    let wrapped = draw(&boxed("one two three four five six seven", Align::Left));
    row(
        "In a 400 pixel box from x 100, words wrap onto three lines or more and stay inside it",
        "yes",
        match ink(&wrapped) {
            Some((l, r, ..)) => yes(lines(&wrapped) >= 3 && l >= 100 && r <= 500, format!("{} lines, {}", lines(&wrapped), box_of(&wrapped))),
            None => "no ink".into(),
        },
    );
    let long = draw(&boxed("abcdefghijklmnopqrstuvwxyz", Align::Left));
    row(
        "A word longer than the box is broken inside it",
        "yes",
        match ink(&long) {
            Some((_, r, ..)) => yes(lines(&long) >= 2 && r <= 500, format!("{} lines, {}", lines(&long), box_of(&long))),
            None => "no ink".into(),
        },
    );
    let japanese = draw(&boxed("あいうえおかきくけこさしすせそ", Align::Left));
    row(
        "Japanese, which has no spaces, wraps between any two characters",
        "yes",
        match ink(&japanese) {
            Some((_, r, ..)) => yes(lines(&japanese) >= 2 && r <= 500, format!("{} lines, {}", lines(&japanese), box_of(&japanese))),
            None => "no ink".into(),
        },
    );
    let middle = |t: &Text| ink(&draw(t)).map(|(l, r, ..)| (l + r) as f64 / 2.0).unwrap_or(0.0);
    let right_end = |t: &Text| ink(&draw(t)).map(|(_, r, ..)| r as f64).unwrap_or(0.0);
    let centred = middle(&boxed("Hi", Align::Center));
    row("Centred in the box, a line's middle is the box's middle, 300", "yes", yes(near(centred, 300.0, 6.0), format!("{centred}")));
    let righted = right_end(&boxed("Hi", Align::Right));
    row("Aligned right in the box, a line ends at the box's right side, 500", "yes", yes(near(righted, 500.0, 8.0), format!("{righted}")));
    let spaced = "the quick brown fox jumps over the lazy dog";
    let justified = draw(&boxed(spaced, Align::Justify));
    let lefted = draw(&boxed(spaced, Align::Left));
    let first_line = |p: &WorkingBuffer| ink(p).and_then(|(_, _, t, ..)| row_ink(p, t + (size * 0.6 * 0.3) as usize));
    row(
        "Justified, the first line reaches both sides of the box; aligned left it falls short",
        "yes",
        match (first_line(&justified), first_line(&lefted)) {
            (Some((jl, jr)), Some((_, lr))) => yes(jl <= 106 && jr >= 490 && lr < 480, format!("justified {jl}..{jr}, left ends {lr}")),
            _ => "no ink".into(),
        },
    );
    let last_line = |p: &WorkingBuffer| ink(p).and_then(|(_, _, _, b, _)| row_ink(p, b - (size * 0.6 * 0.15) as usize));
    row(
        "Justified, the last line stays aligned left, as the editors' Justify Last Left does",
        "the same",
        if last_line(&justified) == last_line(&lefted) { "the same".into() } else { format!("{:?} against {:?}", last_line(&justified), last_line(&lefted)) },
    );
    let top = ink(&draw(&Text { box_width: 400.0, ..words("H", size, [100.0, 200.0], Align::Left) })).map(|(_, _, t, ..)| t).unwrap_or(0);
    row(
        "In a box, the place is the box's top left corner: an H's top is a little below y 200, not above it",
        "yes",
        yes((200..=260).contains(&top), format!("top at {top}")),
    );

    // Row 4, appearance.
    let i_at = words("I", 200.0, [300.0, 500.0], Align::Left);
    let bare = draw(&i_at);
    let outlined = draw(&Text { stroke: Some(Stroke { color: [0.0, 0.0, 0.0], width: 8.0 }), ..i_at.clone() });
    let (bl, br, bt, bb, _) = ink(&bare).unwrap_or((0, 0, 0, 0, 0));
    let mid_y = (bt + bb) / 2;
    row(
        "A stroke 8 pixels wide reaches 8 pixels beyond the letter on each side",
        "yes",
        match ink(&outlined) {
            Some((ol, or, ..)) => yes(near(bl as f64 - ol as f64, 8.0, 1.0) && near(or as f64 - br as f64, 8.0, 1.0), format!("{} against {}", box_of(&outlined), box_of(&bare))),
            None => "no ink".into(),
        },
    );
    row(
        "Outside the letter the stroke is its own colour, black and solid; inside, the fill is on top",
        "[0.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0]",
        format!("{:?}, {:?}", at(&outlined, bl - 4, mid_y), at(&outlined, (bl + br) / 2, mid_y)),
    );
    let bounds = text::bounds(&i_at).unwrap_or([0.0; 4]);
    row(
        "The words' box holds every inked pixel",
        "yes",
        yes(bounds[0] <= bl as f64 + 1.0 && bounds[2] >= br as f64 && bounds[1] <= bt as f64 + 1.0 && bounds[3] >= bb as f64, format!("{bounds:?} against {}", box_of(&bare))),
    );
    let behind = draw(&Text { background: Some(Background { color: [0.0, 0.0, 1.0], opacity: 0.5, padding: 20.0, roundness: 0.0 }), ..i_at.clone() });
    let (x0, y0) = (bounds[0].floor() as usize, bounds[1].floor() as usize);
    row(
        "A background 20 pixels out from the words' box, half seen: blue at half inside it, nothing beyond it",
        "[0.0, 0.0, 0.5, 0.5], [0.0, 0.0, 0.0, 0.0]",
        format!("{:?}, {:?}", at(&behind, x0 - 10, mid_y), at(&behind, x0 - 30, mid_y)),
    );
    let rounded = draw(&Text { background: Some(Background { color: [0.0, 0.0, 1.0], opacity: 1.0, padding: 20.0, roundness: 30.0 }), ..i_at.clone() });
    let square = draw(&Text { background: Some(Background { color: [0.0, 0.0, 1.0], opacity: 1.0, padding: 20.0, roundness: 0.0 }), ..i_at.clone() });
    row(
        "With roundness 30 the background's corner is cut away; square, it is not",
        "0 rounded, 1 square",
        format!("{} rounded, {} square", at(&rounded, x0 - 18, y0 - 18)[3], at(&square, x0 - 18, y0 - 18)[3]),
    );
    let shadowed = draw(&Text { shadow: Some(Shadow { color: [0.0, 0.0, 0.0], opacity: 1.0, angle: 135.0, distance: 20.0, softness: 0.0 }), ..i_at.clone() });
    row(
        "A shadow at 135 degrees, 20 pixels away, lies down and to the right: black just right of the letter",
        "[0.0, 0.0, 0.0, 1.0]",
        format!("{:?}", at(&shadowed, br + 8, mid_y)),
    );
    row("Nothing of the shadow up and to the left", "[0.0, 0.0, 0.0, 0.0]", format!("{:?}", at(&shadowed, bl - 8, mid_y)));
    let soft = draw(&Text { shadow: Some(Shadow { color: [0.0, 0.0, 0.0], opacity: 1.0, angle: 135.0, distance: 20.0, softness: 10.0 }), ..i_at.clone() });
    let edge = at(&soft, br + 14, mid_y)[3];
    row("With softness 10 the shadow's edge is part seen", "yes", yes(edge > 0.05 && edge < 0.95, format!("{edge}")));
    let everything = Text {
        stroke: Some(Stroke { color: [1.0, 0.0, 0.0], width: 6.0 }),
        background: Some(Background { color: [0.0, 0.0, 1.0], opacity: 1.0, padding: 40.0, roundness: 0.0 }),
        shadow: Some(Shadow { color: [0.0, 1.0, 0.0], opacity: 1.0, angle: 90.0, distance: 30.0, softness: 0.0 }),
        ..i_at.clone()
    };
    let stacked = draw(&everything);
    row(
        "All three at once, from the bottom: background, shadow, stroke, fill",
        "[0.0, 0.0, 1.0, 1.0], [0.0, 1.0, 0.0, 1.0], [1.0, 0.0, 0.0, 1.0], [1.0, 0.9, 0.2, 1.0]",
        format!(
            "{:?}, {:?}, {:?}, {:?}",
            at(&stacked, bl - 30, mid_y),
            at(&stacked, br + 20, mid_y),
            at(&stacked, bl - 3, mid_y),
            at(&stacked, (bl + br) / 2, mid_y)
        ),
    );
    row("Drawn a second time, every pixel is the same", "the same", if hash(&draw(&everything)) == hash(&stacked) { "the same" } else { "different" }.into());

    // Row 5.
    for (words_, place, align, pinned) in D263_PICTURES {
        let t = Text { font: Text::BUNDLED_FONT.into(), ..words(words_, 120.0, place, align) };
        row(
            &format!("A D-263 text layer, \"{}\" {}, is drawn byte for byte as on 2edc62d", words_.replace('\n', "\\n"), align.as_str()),
            pinned,
            hash(&draw(&t)),
        );
    }
    let fonts = text::fonts();
    row(
        "The fonts are listed by the names they give themselves: the one that comes with the program is Rounded Mplus 1c, Regular",
        "Rounded Mplus 1c, Regular",
        fonts.iter().find(|f| f.file == Text::BUNDLED_FONT).map(|f| format!("{}, {}", f.family, f.style)).unwrap_or("not listed".into()),
    );
    row(
        "And this machine's own, such as Arial, Bold in arialbd.ttf",
        "Arial, Bold",
        fonts.iter().find(|f| f.file.eq_ignore_ascii_case("arialbd.ttf")).map(|f| format!("{}, {}", f.family, f.style)).unwrap_or("not listed".into()),
    );
    row(
        "A listed font's file can be read for the window to type in",
        "the same",
        if text::font_bytes(Text::BUNDLED_FONT).map(<[u8]>::to_vec) == fs::read(repo("assets/fonts/MPLUSRounded1c-Regular.ttf")).ok() { "the same" } else { "different" }.into(),
    );
    let frame = |project: &Project, name: &str| {
        let dir = repo("target/d264_text_styles").join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let request = ExportRequest {
            composition: project.compositions[0].id.clone(),
            first_frame: 10,
            last_frame: 10,
            output_dir: dir.clone(),
            naming: "d264_%04d.png".to_string(),
            depth: OutputDepth::Eight,
            alpha: OutputAlpha::Straight,
            tile_size: 128,
            missing: MissingSource::Block,
            format: OutputFormat::Png,
            choices: Default::default(),
        };
        let report = export_sequence(project, &repo("Fixtures/reference_shot"), &request, &AtomicBool::new(false));
        (fs::read(dir.join("d264_0010.png")).unwrap_or_default(), report)
    };
    let _ = apply(&mut doc, set(Text { box_width: 0.0, align: Align::Left, ..plain.clone() }));
    let (unstyled, _) = frame(doc.project(), "plain");
    let _ = apply(&mut doc, set(styled.clone()));
    let (fancy, report) = frame(doc.project(), "styled");
    row(
        "A styled text layer goes into exported frame 10, which differs from the unstyled one, and nothing is said",
        "different, nothing said",
        format!(
            "{}, {}",
            if !fancy.is_empty() && fancy != unstyled { "different" } else { "the same" },
            if report.diagnostics.is_empty() { "nothing said".to_string() } else { format!("{:?}", report.diagnostics.iter().map(|d| d.id.as_str()).collect::<Vec<_>>()) },
        ),
    );
    let mut found = Vec::new();
    projects(&repo("Fixtures"), &mut found);
    found.sort();
    let mut all = String::new();
    let mut opened = 0;
    for path in &found {
        let Ok(text) = fs::read_to_string(path) else { continue };
        if let Ok(loaded) = persist::load_str(&text) {
            opened += 1;
            all.push_str(&persist::to_json(loaded.document.project(), &loaded.preserved));
        }
    }
    row("Fixture projects that open", &FIXTURES_OPENED.to_string(), opened.to_string());
    row("Their saved text, all of it, is byte for byte as before (SHA-256)", FIXTURES_SAVED, sha256::hex(all.as_bytes()));

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut md = format!(
        "# D-264: text styles\n\nWritten by `tests/d264_text_styles.rs`. The D-263 pictures were taken on 2edc62d and the fixture hash on e395d8b.\n\n**{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, e, a) in &rows {
        let _ = writeln!(md, "| {check} | {e} | {a} | {} |", if e == a { "pass" } else { "FAIL" });
    }
    fs::write(repo("verification/D-264_text_styles_table.md"), md).unwrap();
    let failed: Vec<_> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(failed.is_empty(), "{failed:?}");
}
