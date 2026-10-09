//! D-263: a text layer's words, drawn from a font's own outlines.
//!
//! The outlines are read by `ttf-parser` and filled here through the same sixteen samples a
//! pixel that masks and shapes use (ADR-016), so text, shapes and masks have one edge quality.
//! Fonts fill by the nonzero rule rather than the even-odd rule masks use: a letter may be built
//! from outlines that overlap, and the overlap is ink, not a hole.
//!
//! Characters are placed one after another by their advance widths, plus D-264's tracking and,
//! when asked, the font's own kerning pairs (GPOS `kern`, else the old `kern` table).
//! ponytail: no shaping, which Latin and Japanese do without; Arabic or the Indic scripts would
//! need a shaper (`harfrust`, named in document 15) before they joined correctly.
//!
//! D-264 adds what After Effects, Premiere Pro and Resolve put in their text panels: tracking,
//! kerning, leading, all caps, faux bold and italic, a box the words wrap in with justified lines,
//! and a stroke, a background box and a shadow, painted in that order under the fill.
//!
//! D-350 adds text animators (`effects::Effect::TextAnimator`): each character moved, scaled,
//! turned, faded, recoloured and spaced by how much each animator's range selector picks it. The
//! letters stay outlines drawn here, on the processor, as all of a text layer is; what comes
//! after the drawing runs where it always did.

use std::collections::HashMap;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use rayon::prelude::*;

use crate::mask::SAMPLES_PER_SIDE;
use crate::WorkingBuffer;

/// Which end of each line sits at the text's place; in a box (D-264), which side of the box.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
    /// D-264: each line of a box stretched to both sides, the last line of a paragraph left, as
    /// the editors' "Justify Last Left". Without a box it is drawn as Left.
    Justify,
}

impl Align {
    pub fn as_str(self) -> &'static str {
        match self {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
            Align::Justify => "justify",
        }
    }

    pub fn parse(s: &str) -> Option<Align> {
        match s {
            "left" => Some(Align::Left),
            "center" => Some(Align::Center),
            "right" => Some(Align::Right),
            "justify" => Some(Align::Justify),
            _ => None,
        }
    }
}

/// D-264: a line round the letters, `width` pixels out from their edge, under the fill.
#[derive(Clone, PartialEq, Debug)]
pub struct Stroke {
    pub color: [f64; 3],
    pub width: f64,
}

/// D-264: a box behind the words, `padding` pixels out from them, corners rounded by
/// `roundness` pixels.
#[derive(Clone, PartialEq, Debug)]
pub struct Background {
    pub color: [f64; 3],
    pub opacity: f64,
    pub padding: f64,
    pub roundness: f64,
}

/// D-264: the letters and their stroke again, `distance` pixels away in `angle` degrees (After
/// Effects' compass: 90 is to the right, 135 down and to the right) and blurred by `softness`.
#[derive(Clone, PartialEq, Debug)]
pub struct Shadow {
    pub color: [f64; 3],
    pub opacity: f64,
    pub angle: f64,
    pub distance: f64,
    pub softness: f64,
}

/// D-263's record: what a text layer says and how.
#[derive(Clone, PartialEq, Debug)]
pub struct Text {
    /// The words; a line break starts a new line below.
    pub text: String,
    /// A font file's name, as it is in the Windows fonts folder, or [`Text::BUNDLED_FONT`].
    pub font: String,
    /// The font's em in pixels: roughly the height from the bottom of a "g" to the top of an "A".
    pub size: f64,
    /// Linear, 0 to 1, as a solid's colour is.
    pub color: [f64; 3],
    /// Where the first line's baseline meets its aligned end, in the layer's own pixels, which
    /// are the composition's (the layer's anchor and position are both its centre, as a shape
    /// layer's are).
    /// In a box (D-264), the box's top left corner instead.
    pub at: [f64; 2],
    pub align: Align,
    /// D-264, from here on; each is written only when it is not its default, so a D-263 record
    /// saves as it did. Thousandths of the size, added after each character, -1000 to 1000.
    pub tracking: f64,
    /// Pixels from one baseline to the next; 0 is the font's own line height.
    pub leading: f64,
    /// The font's own kerning pairs, which the editors call "Metrics".
    pub kerning: bool,
    pub all_caps: bool,
    /// The outlines thickened by a fiftieth of the size, for a font with no bold of its own.
    pub faux_bold: bool,
    /// The outlines slanted 12 degrees, for a font with no italic of its own.
    pub faux_italic: bool,
    /// The width of a box the words wrap in; 0 is point text, which never wraps.
    pub box_width: f64,
    pub stroke: Option<Stroke>,
    pub background: Option<Background>,
    pub shadow: Option<Shadow>,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: String::new(),
            font: Self::BUNDLED_FONT.to_string(),
            size: 100.0,
            color: [1.0; 3],
            at: [0.0; 2],
            align: Align::Left,
            tracking: 0.0,
            leading: 0.0,
            kerning: false,
            all_caps: false,
            faux_bold: false,
            faux_italic: false,
            box_width: 0.0,
            stroke: None,
            background: None,
            shadow: None,
        }
    }
}

impl Text {
    /// The font that comes with the program: M PLUS Rounded 1c, under the SIL Open Font
    /// License (`docs/third_party/MPLUSRounded1c-OFL.txt`). It has Latin, kana and kanji.
    pub const BUNDLED_FONT: &'static str = "MPLUSRounded1c-Regular.ttf";
    pub const MAX_SIZE: f64 = 2000.0;
    /// The widest box, the widest composition (document 19).
    pub const MAX_BOX: f64 = 16384.0;

    /// What is outside D-263's and D-264's ranges, as a sentence, or `None` for words that may
    /// be drawn.
    pub fn problem(&self) -> Option<String> {
        if !(1.0..=Self::MAX_SIZE).contains(&self.size) {
            return Some(format!("a size from 1 to {} pixels, not {}", Self::MAX_SIZE, self.size));
        }
        let colours = [Some(&self.color), self.stroke.as_ref().map(|s| &s.color), self.background.as_ref().map(|b| &b.color), self.shadow.as_ref().map(|s| &s.color)];
        if let Some(c) = colours.into_iter().flatten().flatten().find(|c| !(0.0..=1.0).contains(*c)) {
            return Some(format!("a colour of three numbers from 0 to 1, not {c}"));
        }
        let within = |what: &str, v: f64, low: f64, high: f64| {
            (!(low..=high).contains(&v)).then(|| format!("{what} from {low} to {high}, not {v}"))
        };
        let mut checks = vec![
            within("a tracking", self.tracking, -1000.0, 1000.0),
            within("a leading", self.leading, 0.0, 10000.0),
            within("a box width", self.box_width, 0.0, Self::MAX_BOX),
        ];
        if let Some(s) = &self.stroke {
            if !(s.width > 0.0 && s.width <= 500.0) {
                checks.push(Some(format!("a stroke width above 0 and up to 500, not {}", s.width)));
            }
        }
        if let Some(b) = &self.background {
            checks.push(within("a background opacity", b.opacity, 0.0, 1.0));
            checks.push(within("a background padding", b.padding, 0.0, 1000.0));
            checks.push(within("a background roundness", b.roundness, 0.0, 1000.0));
        }
        if let Some(s) = &self.shadow {
            checks.push(within("a shadow opacity", s.opacity, 0.0, 1.0));
            checks.push(within("a shadow angle", s.angle, -36000.0, 36000.0));
            checks.push(within("a shadow distance", s.distance, 0.0, 1000.0));
            checks.push(within("a shadow softness", s.softness, 0.0, 500.0));
        }
        if let Some(p) = checks.into_iter().flatten().next() {
            return Some(p);
        }
        if !self.at.iter().all(|v| v.is_finite()) {
            return Some("a place of two numbers".to_string());
        }
        // A font is named, never pointed at: the project must not reach into folders.
        let f = &self.font;
        let lower = f.to_ascii_lowercase();
        if f.is_empty()
            || f.starts_with('.')
            || f.contains(['/', '\\', ':'])
            || ![".ttf", ".otf", ".ttc"].iter().any(|e| lower.ends_with(e))
        {
            return Some(format!(
                "a font's file name ending .ttf, .otf or .ttc, not \"{f}\""
            ));
        }
        None
    }
}

static BUNDLED: &[u8] = include_bytes!("../assets/fonts/MPLUSRounded1c-Regular.ttf");

/// The font's bytes, read once a run. `None` when this machine does not have it. The window
/// asks for them too (D-264), to type on the picture in the font the frame is drawn in.
///
/// ponytail: a font read is kept for the run, so a font installed or replaced while the program
/// is open is seen after a restart; nothing in a frame can tell the two files apart otherwise.
pub fn font_bytes(name: &str) -> Option<&'static [u8]> {
    if name == Text::BUNDLED_FONT {
        return Some(BUNDLED);
    }
    // P-25: named, never pointed at, by the window as by a project: a name the project could
    // not hold reads nothing.
    if (Text { font: name.to_string(), ..Text::default() }).problem().is_some() {
        return None;
    }
    static READ: OnceLock<Mutex<HashMap<String, &'static [u8]>>> = OnceLock::new();
    let mut read = READ.get_or_init(Default::default).lock().ok()?;
    if let Some(bytes) = read.get(name) {
        return Some(bytes);
    }
    let bytes = installed_fonts()
        .into_iter()
        .find_map(|dir| std::fs::read(dir.join(name)).ok())?;
    // Kept for the run, so leaked: a project names a handful of fonts at most.
    let bytes: &'static [u8] = Box::leak(bytes.into_boxed_slice());
    read.insert(name.to_string(), bytes);
    Some(bytes)
}

/// Where Windows keeps fonts: for everyone, and for the person signed in.
pub fn installed_fonts() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(windows) = std::env::var_os("WINDIR") {
        dirs.push(PathBuf::from(windows).join("Fonts"));
    }
    if let Some(local) = std::env::var_os("LOCALAPPDATA") {
        dirs.push(PathBuf::from(local).join("Microsoft").join("Windows").join("Fonts"));
    }
    dirs
}

/// One font file as the editors' font menus show it: a family and a style within it.
#[derive(Clone, PartialEq, Debug)]
pub struct FontName {
    pub file: String,
    pub family: String,
    pub style: String,
}

/// D-264: every font this program can draw, by family and style, the bundled one first, then
/// the rest by family. Read once a run, from each file's name table alone: the fonts folder
/// holds some hundreds of megabytes, which a menu has no need to read. Of a collection (.ttc),
/// its first font, which is the one a project naming the file draws.
pub fn fonts() -> &'static [FontName] {
    static LIST: OnceLock<Vec<FontName>> = OnceLock::new();
    LIST.get_or_init(|| {
        let mut list: Vec<FontName> = Vec::new();
        let mut add = |file: String, table: &[u8]| {
            if let Some((family, style)) = names(table) {
                if !list.iter().any(|f| f.file.eq_ignore_ascii_case(&file)) {
                    list.push(FontName { file, family, style });
                }
            }
        };
        if let Some(table) = ttf_parser::Face::parse(BUNDLED, 0).ok().and_then(|f| f.raw_face().table(ttf_parser::Tag::from_bytes(b"name"))) {
            add(Text::BUNDLED_FONT.to_string(), table);
        }
        for dir in installed_fonts() {
            let Ok(entries) = std::fs::read_dir(&dir) else { continue };
            let mut files: Vec<_> = entries.flatten().map(|e| e.path()).collect();
            files.sort();
            for path in files {
                let Some(file) = path.file_name().and_then(|n| n.to_str()).map(str::to_string) else { continue };
                if (Text { font: file.clone(), ..Text::default() }).problem().is_some() {
                    continue;
                }
                if let Some(table) = name_table(&path) {
                    add(file, &table);
                }
            }
        }
        let bundled = list.len().min(1);
        list[bundled..].sort_by(|a, b| (a.family.to_lowercase(), &a.style).cmp(&(b.family.to_lowercase(), &b.style)));
        list
    })
}

/// A font file's `name` table, found through its table directory without reading the outlines.
fn name_table(path: &std::path::Path) -> Option<Vec<u8>> {
    let mut f = std::fs::File::open(path).ok()?;
    let mut read = |at: u64, n: usize| -> Option<Vec<u8>> {
        let mut buf = vec![0; n];
        f.seek(SeekFrom::Start(at)).ok()?;
        f.read_exact(&mut buf).ok()?;
        Some(buf)
    };
    let be32 = |b: &[u8], i: usize| u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
    let head = read(0, 16)?;
    let start = if &head[..4] == b"ttcf" { be32(&head, 12) as u64 } else { 0 };
    let tables = u16::from_be_bytes(read(start + 4, 2)?.try_into().ok()?) as usize;
    let dir = read(start + 12, tables * 16)?;
    let record = dir.chunks_exact(16).find(|r| &r[..4] == b"name")?;
    let (at, len) = (be32(record, 8) as u64, be32(record, 12) as usize);
    // A name table is a few kilobytes; one claiming more than a megabyte is not read.
    (len <= 1 << 20).then_some(())?;
    read(at, len)
}

/// A name table's family and style in English: the typographic pair (16 and 17) when there is
/// one, else the legacy pair (1 and 2), as the editors' menus group them.
fn names(table: &[u8]) -> Option<(String, String)> {
    let table = ttf_parser::name::Table::parse(table)?;
    let find = |id: u16| {
        let english = |n: &ttf_parser::name::Name| {
            (n.platform_id == ttf_parser::PlatformId::Windows && n.language_id == 0x0409)
                || (n.platform_id == ttf_parser::PlatformId::Macintosh && n.language_id == 0)
        };
        let all: Vec<_> = table.names.into_iter().filter(|n| n.name_id == id).collect();
        all.iter().filter(|n| english(n)).chain(all.iter()).find_map(|n| n.to_string()).filter(|s| !s.is_empty())
    };
    let family = find(16).or_else(|| find(1))?;
    let style = find(17).or_else(|| find(2)).unwrap_or_else(|| "Regular".to_string());
    Some((family, style))
}

/// Collects a glyph's outlines in the picture's pixels, curves cut into short straight lines.
struct Pen<'a> {
    x: f64,
    y: f64,
    scale: f64,
    /// D-264's faux italic: how far right a point moves for each unit it is above the baseline.
    slant: f64,
    /// D-350: the character's own move, scale and turn about its anchor; `None` when it has none.
    /// A curve's control points are moved with it, which moves the curve exactly.
    turn: Option<Turn>,
    now: Vec<(f64, f64)>,
    into: &'a mut Vec<Vec<(f64, f64)>>,
    /// D-373: a path for Path Stroke rather than a shape to fill: the outline's points and curve
    /// handles are kept, a quadratic raised to the cubic it is, and each contour is cut into
    /// pieces by `mask::flatten`, as a mask's or a shape's path is.
    path: Option<Vec<crate::mask::MaskPoint>>,
}

impl Pen<'_> {
    fn at(&self, x: f32, y: f32) -> (f64, f64) {
        // Font units go up; pixels go down.
        let p = (self.x + (x as f64 + y as f64 * self.slant) * self.scale, self.y - y as f64 * self.scale);
        let Some(t) = &self.turn else { return p };
        let (dx, dy) = ((p.0 - t.anchor[0]) * t.scale[0], (p.1 - t.anchor[1]) * t.scale[1]);
        // Clockwise on the picture, whose y grows downwards.
        (
            t.anchor[0] + t.cos * dx - t.sin * dy + t.moved[0],
            t.anchor[1] + t.sin * dx + t.cos * dy + t.moved[1],
        )
    }

    fn end(&mut self) {
        if let Some(nodes) = &mut self.path {
            self.now.clear();
            // The font closes a contour by coming back to its first point: one point, not two.
            if nodes.len() > 1 && nodes[0].point == nodes[nodes.len() - 1].point {
                let last = nodes.pop().unwrap();
                nodes[0].in_handle = last.in_handle;
            }
            let nodes = std::mem::take(nodes);
            if nodes.len() >= 2 {
                self.into.push(crate::mask::flatten(&nodes, true));
            }
            return;
        }
        let now = std::mem::take(&mut self.now);
        if now.len() >= 3 {
            self.into.push(now);
        }
    }

    /// D-373: a path node at `to`, the curve to it leaving the node before by `out` and arriving
    /// by `into`, both as points (the node itself for a straight line).
    fn node(&mut self, out: (f64, f64), into: (f64, f64), to: (f64, f64)) {
        let Some(nodes) = &mut self.path else { return };
        if let Some(last) = nodes.last_mut() {
            last.out_handle = (out.0 - last.point.0, out.1 - last.point.1);
        }
        nodes.push(crate::mask::MaskPoint { point: to, in_handle: (into.0 - to.0, into.1 - to.1), out_handle: (0.0, 0.0) });
    }

    /// A curve through `control`, ending at `to`, as straight pieces about two pixels long.
    fn curve(&mut self, control: &[(f64, f64)], to: (f64, f64)) {
        let from = *self.now.last().unwrap_or(&to);
        let mut hull = vec![from];
        hull.extend_from_slice(control);
        hull.push(to);
        let reach: f64 = hull.windows(2).map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1)).sum();
        let pieces = (reach / 2.0).ceil().clamp(1.0, 64.0) as usize;
        for i in 1..=pieces {
            let t = i as f64 / pieces as f64;
            // De Casteljau on the hull.
            let mut p = hull.clone();
            while p.len() > 1 {
                p = p.windows(2).map(|w| (w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t)).collect();
            }
            self.now.push(p[0]);
        }
    }
}

impl ttf_parser::OutlineBuilder for Pen<'_> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.end();
        let p = self.at(x, y);
        self.now.push(p);
        if let Some(nodes) = &mut self.path {
            nodes.push(crate::mask::MaskPoint { point: p, in_handle: (0.0, 0.0), out_handle: (0.0, 0.0) });
        }
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.at(x, y);
        self.now.push(p);
        let last = self.path.as_ref().and_then(|n| n.last()).map_or(p, |l| l.point);
        self.node(last, p, p);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (c, to) = (self.at(x1, y1), self.at(x, y));
        if self.path.is_some() {
            // The cubic a quadratic is: its handles two thirds of the way to the control point.
            let from = self.path.as_ref().and_then(|n| n.last()).map_or(to, |l| l.point);
            let third = |a: (f64, f64)| (a.0 + (c.0 - a.0) * 2.0 / 3.0, a.1 + (c.1 - a.1) * 2.0 / 3.0);
            self.node(third(from), third(to), to);
            return;
        }
        self.curve(&[c], to);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (c1, c2, to) = (self.at(x1, y1), self.at(x2, y2), self.at(x, y));
        if self.path.is_some() {
            self.node(c1, c2, to);
            return;
        }
        self.curve(&[c1, c2], to);
    }
    fn close(&mut self) {
        self.end();
    }
}

#[derive(Clone, Copy)]
struct Turn {
    anchor: [f64; 2],
    scale: [f64; 2],
    cos: f64,
    sin: f64,
    moved: [f64; 2],
}

/// D-350: one character as the text animators leave it. `index` counts the layer's characters
/// from 0, line breaks left out; `anchor` is on its baseline halfway across its advance; `moved`,
/// `scale` (a factor each way) and `turn` (degrees, clockwise) are about the anchor; `amounts`
/// is how much each animator picked it; `bounds` is its outline's box, left, top, right, bottom,
/// as drawn, `None` for a character with no outline.
#[derive(Clone, PartialEq, Debug)]
pub struct Placed {
    pub index: usize,
    pub ch: char,
    pub anchor: [f64; 2],
    pub moved: [f64; 2],
    pub scale: [f64; 2],
    pub turn: f64,
    pub opacity: f64,
    pub color: [f64; 3],
    pub amounts: Vec<f64>,
    pub bounds: Option<[f64; 4]>,
}

/// D-350: every character laid out with the animators, in the order drawn (D-371: a
/// right-to-left run's from the right; `index` is each one's place in the words). `None` when
/// the font is not on this machine.
pub fn placed(text: &Text, animators: &[crate::effects::Effect]) -> Option<Vec<Placed>> {
    lay_out(text, animators, false).map(|l| l.chars.into_iter().map(|c| c.0).collect())
}

/// D-373: the words' outlines as paths for Path Stroke, in the layer's pixels with the
/// animators as they are: every contour closed, glyph by glyph in the order the words are read
/// (a cluster's at its first character typed), each glyph's contours in the font's order, as
/// `mask::flatten` cuts a path. `None` when the font is not on this machine.
pub fn outlines(text: &Text, animators: &[crate::effects::Effect]) -> Option<Vec<Vec<(f64, f64)>>> {
    let mut laid = lay_out(text, animators, true)?;
    laid.chars.sort_by_key(|c| c.0.index);
    Some(laid.chars.iter().flat_map(|c| laid.contours[c.1.clone()].iter().cloned()).collect())
}

/// D-350: how much one animator's range selector picks each of `chars`, the layer's characters
/// with line breaks left out, times its Amount. Document 21 and `tools/text_animator_reference.py`
/// give the rule.
fn picks(chars: &[char], animator: &crate::effects::Effect) -> Vec<f64> {
    let crate::effects::Effect::TextAnimator { start, end, offset, amount, based_on, shape, smoothness, ease_high, ease_low, .. } = animator else {
        return vec![0.0; chars.len()];
    };
    // Each character's unit, if it is one, and how many units there are.
    let (mut unit, mut n, mut inside) = (Vec::with_capacity(chars.len()), 0usize, false);
    for c in chars {
        let space = c.is_whitespace();
        match based_on.as_str() {
            "characters" => {
                unit.push(Some(n));
                n += 1;
            }
            "characters_excluding_spaces" => {
                unit.push((!space).then_some(n));
                n += usize::from(!space);
            }
            _ => {
                unit.push((!space).then_some(n));
                n += usize::from(space && inside);
                inside = !space;
            }
        }
    }
    n += usize::from(inside);
    let count = n as f64;
    let (mut s, mut e) = ((start + offset) / 100.0 * count, (end + offset) / 100.0 * count);
    if s > e {
        std::mem::swap(&mut s, &mut e);
    }
    let pick = |k: usize| {
        let k = k as f64;
        let v = if shape == "square" {
            let c = (e.min(k + 1.0) - s.max(k)).clamp(0.0, 1.0);
            let w = smoothness / 100.0;
            if w == 0.0 {
                if c >= 0.5 { 1.0 } else { 0.0 }
            } else {
                ((c - 0.5) / w + 0.5).clamp(0.0, 1.0)
            }
        } else {
            let m = k + 0.5;
            let u = if e > s { (m - s) / (e - s) } else if m >= s { f64::INFINITY } else { f64::NEG_INFINITY };
            let within = (0.0..=1.0).contains(&u);
            match shape.as_str() {
                "ramp_up" => u.clamp(0.0, 1.0),
                "ramp_down" => 1.0 - u.clamp(0.0, 1.0),
                "triangle" if within => 1.0 - (2.0 * u - 1.0).abs(),
                "round" if within => (1.0 - (2.0 * u - 1.0).powi(2)).max(0.0).sqrt(),
                "smooth" if within => (1.0 - (std::f64::consts::TAU * u).cos()) / 2.0,
                _ => 0.0,
            }
        };
        // Document 20's ease curve, Ease Low the handle at the low end.
        let v = if *ease_high != 0.0 || *ease_low != 0.0 {
            crate::model::solve(ease_low / 100.0, 0.0, 1.0 - ease_high / 100.0, 1.0, v)
        } else {
            v
        };
        v * amount / 100.0
    };
    unit.into_iter().map(|u| u.filter(|_| n > 0).map_or(0.0, pick)).collect()
}

/// D-350: character `index` as the animators, top first, leave it, and the room they add after it.
fn animate(index: usize, ch: char, text: &Text, animators: &[crate::effects::Effect], picked: &[Vec<f64>]) -> (Placed, f64) {
    let mut p = Placed {
        index,
        ch,
        anchor: [0.0; 2],
        moved: [0.0; 2],
        scale: [1.0; 2],
        turn: 0.0,
        opacity: 1.0,
        color: text.color,
        amounts: Vec::new(),
        bounds: None,
    };
    let mut room = 0.0;
    for (a, row) in animators.iter().zip(picked) {
        let crate::effects::Effect::TextAnimator { position, scale, rotation, opacity, fill, color, tracking, .. } = a else { continue };
        let v = row[index];
        p.amounts.push(v);
        p.moved = [p.moved[0] + v * position[0], p.moved[1] + v * position[1]];
        p.turn += v * rotation;
        room += v * tracking / 1000.0 * text.size;
        p.scale = [p.scale[0] * (1.0 + v * (scale[0] / 100.0 - 1.0)), p.scale[1] * (1.0 + v * (scale[1] / 100.0 - 1.0))];
        p.opacity = (p.opacity * (1.0 + v * (opacity / 100.0 - 1.0))).clamp(0.0, 1.0);
        if fill == "on" {
            let f = v.clamp(0.0, 1.0);
            for j in 0..3 {
                p.color[j] += f * (color[j] - p.color[j]);
            }
        }
    }
    (p, room)
}

/// Document 21 step 1 for a text layer: the composition's size in transparent black with the
/// words drawn into it. `None` when the font is not on this machine or is not a font, which the
/// caller says per frame; nothing is drawn in another font's place.
pub fn draw(text: &Text, width: usize, height: usize) -> Option<WorkingBuffer> {
    animated(text, &[], width, height)
}

/// [`draw`] with D-350's text animators, top first. Characters of one opacity and colour are
/// filled together, as all of them are without animators; every stroke is drawn before every
/// fill, and the shadow is cast by them all.
pub fn animated(text: &Text, animators: &[crate::effects::Effect], width: usize, height: usize) -> Option<WorkingBuffer> {
    let laid = lay_out(text, animators, false)?;
    let (w, h) = (width, height);
    let mut picture = WorkingBuffer::transparent(w, h);
    if let Some(b) = &text.background {
        let p = b.padding;
        let [l, t, r, bottom] = laid.bounds;
        let field = fill(&[rounded_box(l - p, t - p, r + p, bottom + p, b.roundness)], w, h);
        crate::shape::paint(picture.data_mut(), &field, w, b.color, b.opacity, None);
    }
    let bold = if text.faux_bold { text.size * 0.02 } else { 0.0 };
    // The characters by opacity and colour, in the order each first comes; one not seen is left.
    let mut groups: Vec<(f64, [f64; 3], Vec<Vec<(f64, f64)>>)> = Vec::new();
    for (p, range) in &laid.chars {
        if p.opacity <= 0.0 {
            continue;
        }
        let i = match groups.iter().position(|g| g.0 == p.opacity && g.1 == p.color) {
            Some(i) => i,
            None => {
                groups.push((p.opacity, p.color, Vec::new()));
                groups.len() - 1
            }
        };
        groups[i].2.extend_from_slice(&laid.contours[range.clone()]);
    }
    // Each group's fill and stroke cover, and the rows they reach: nothing outside them is painted.
    let reach = bold + text.stroke.as_ref().map_or(0.0, |s| s.width);
    type Field = (f64, [f64; 3], Vec<f32>, Option<Vec<f32>>, std::ops::Range<usize>);
    let fields: Vec<Field> = groups
        .iter()
        .map(|(opacity, color, contours)| {
            let mut ink = fill(contours, w, h);
            if bold > 0.0 {
                union(&mut ink, &around(contours, bold, w, h));
            }
            let lined = text.stroke.as_ref().map(|s| {
                let mut f = around(contours, bold + s.width, w, h);
                union(&mut f, &ink);
                f
            });
            let ys = || contours.iter().flatten().map(|p| p.1);
            let top = (ys().fold(f64::INFINITY, f64::min) - reach - 2.0).floor().clamp(0.0, h as f64) as usize;
            let bottom = (ys().fold(f64::NEG_INFINITY, f64::max) + reach + 3.0).ceil().clamp(0.0, h as f64) as usize;
            (*opacity, *color, ink, lined, top..bottom.max(top))
        })
        .collect();
    let rows = |data: &mut [f32], field: &[f32], band: &std::ops::Range<usize>, color: [f64; 3], opacity: f64| {
        crate::shape::paint(&mut data[band.start * w * 4..band.end * w * 4], &field[band.start * w..band.end * w], w, color, opacity, None);
    };
    if let (Some(s), false) = (&text.shadow, fields.is_empty()) {
        // Every group's letters, each as seen.
        let mut under = vec![0.0f32; w * h];
        for (opacity, _, ink, lined, band) in &fields {
            let f = lined.as_ref().unwrap_or(ink);
            for i in band.start * w..band.end * w {
                under[i] = under[i].max(f[i] * *opacity as f32);
            }
        }
        // After Effects' compass: 0 is up, 90 is right, and y grows downwards in pixels.
        let a = s.angle.to_radians();
        let (dx, dy) = (a.sin() * s.distance, -a.cos() * s.distance);
        // D-265: only the part of the frame the moved letters reach is worked. Outside it the
        // shadow is empty, the blur skips empty pixels and adds its taps in the same order
        // wherever the part starts, so every pixel is the one the whole frame gave.
        if let Some([l, t, r, b]) = moved_box(&under, w, h, dx, dy) {
            let pw = r - l;
            let mut shadow = WorkingBuffer::transparent(pw, b - t);
            let part = shifted(&under, w, h, dx, dy, [l, t, r, b]);
            crate::shape::paint(shadow.data_mut(), &part, pw, s.color, s.opacity, None);
            // The blur grows the part by its radius on every side.
            let grow = if s.softness > 0.0 { crate::effects::blur(&mut shadow, s.softness / 2.0) } else { 0 };
            let sw = shadow.width();
            let sh = shadow.data();
            let data = picture.data_mut();
            for y in t.saturating_sub(grow)..(b + grow).min(h) {
                for x in l.saturating_sub(grow)..(r + grow).min(w) {
                    let j = ((y + grow - t) * sw + x + grow - l) * 4;
                    let px = &mut data[(y * w + x) * 4..(y * w + x) * 4 + 4];
                    for c in 0..4 {
                        px[c] = sh[j + c] + px[c] * (1.0 - sh[j + 3]);
                    }
                }
            }
        }
    }
    if let Some(s) = &text.stroke {
        for (opacity, _, _, lined, band) in &fields {
            if let Some(f) = lined {
                rows(picture.data_mut(), f, band, s.color, *opacity);
            }
        }
    }
    for (opacity, color, ink, _, band) in &fields {
        rows(picture.data_mut(), ink, band, *color, *opacity);
    }
    Some(picture)
}

/// D-265: [`draw`], keeping the last few pictures drawn. Text settings have no keys, so a text
/// layer is the same picture on every frame, and playing or scrubbing it draws it once. A
/// picture is handed out shared; the render copies it before an effect or mask changes it.
// ponytail: four 1080p pictures, about 130 MB; key by layer if a project shows more text layers
// than that redrawing on every frame.
// D-350: with its animators as they are on the frame, so a still animator draws once too.
pub fn drawn(text: &Text, animators: &[crate::effects::Effect], width: usize, height: usize) -> Option<Arc<WorkingBuffer>> {
    type Kept = Vec<(Text, Vec<crate::effects::Effect>, usize, usize, Arc<WorkingBuffer>)>;
    static KEPT: Mutex<Kept> = Mutex::new(Vec::new());
    let hit = |kept: &mut Kept| {
        let i = kept.iter().position(|(t, a, w, h, _)| t == text && a == animators && *w == width && *h == height)?;
        let found = kept.remove(i);
        let picture = found.4.clone();
        kept.push(found);
        Some(picture)
    };
    if let Some(p) = hit(&mut KEPT.lock().unwrap_or_else(|e| e.into_inner())) {
        return Some(p);
    }
    let picture = Arc::new(animated(text, animators, width, height)?);
    let mut kept = KEPT.lock().unwrap_or_else(|e| e.into_inner());
    if hit(&mut kept).is_none() {
        if kept.len() == 4 {
            kept.remove(0);
        }
        kept.push((text.clone(), animators.to_vec(), width, height, picture.clone()));
    }
    Some(picture)
}

/// D-264: the words' box in the layer's pixels, left, top, right, bottom: from the first line's
/// ascender to the last line's descender and across every line (a box text's whole box), grown
/// to hold any outline that reaches past it. The background is drawn round it, and the window
/// outlines and picks a text layer by it. `None` when the font is not on this machine.
pub fn bounds(text: &Text) -> Option<[f64; 4]> {
    lay_out(text, &[], false).map(|l| l.bounds)
}

/// The words placed: every glyph's outline in the layer's pixels, and their box; D-350: and each
/// character with the outlines that are its own.
struct Laid {
    contours: Vec<Vec<(f64, f64)>>,
    bounds: [f64; 4],
    chars: Vec<(Placed, std::ops::Range<usize>)>,
}

fn lay_out(text: &Text, animators: &[crate::effects::Effect], path: bool) -> Option<Laid> {
    let face = rustybuzz::Face::from_slice(font_bytes(&text.font)?, 0)?;
    let scale = text.size / face.units_per_em() as f64;
    let natural = (face.ascender() as f64 - face.descender() as f64 + face.line_gap() as f64) * scale;
    let line = if text.leading > 0.0 { text.leading } else { natural };
    let boxed = text.box_width > 0.0;
    let ascent = face.ascender() as f64 * scale;
    let first = if boxed { text.at[1] + ascent } else { text.at[1] };
    let track = text.tracking / 1000.0 * text.size;
    let words = if text.all_caps { text.text.to_uppercase() } else { text.text.clone() };
    let slant = if text.faux_italic { 0.2126 } else { 0.0 };
    let mut contours = Vec::new();
    let mut across = [f64::INFINITY, f64::NEG_INFINITY];
    let mut n = 0;
    // D-350: each animator's pick of every character, line breaks left out.
    let flat: Vec<char> = words.split('\n').flat_map(|p| p.trim_end_matches('\r').chars()).collect();
    let picked: Vec<Vec<f64>> = animators.iter().map(|a| picks(&flat, a)).collect();
    let mut placed = Vec::new();
    let mut k = 0;
    for paragraph in words.split('\n') {
        let chars: Vec<char> = paragraph.trim_end_matches('\r').chars().collect();
        let mut states: Vec<(Placed, f64)> = chars.iter().enumerate().map(|(i, c)| animate(k + i, *c, text, animators, &picked)).collect();
        k += chars.len();
        // D-371: shaped. A glyph the font lacks is its "missing" glyph, usually a box: seen, not
        // hidden. A cluster (a ligature, a letter and its marks) is one unit from here on, held by
        // its first character; the others in it take no room and draw nothing of their own.
        let glyphs = shaped(&face, &chars, text.kerning);
        let mut head: Vec<usize> = (0..chars.len()).collect();
        let mut units = [vec![0i32; chars.len()], vec![0i32; chars.len()]];
        for g in &glyphs {
            units[0][g.ch] += i32::from(face.glyph_hor_advance(g.id).unwrap_or(0));
            units[1][g.ch] += g.advance;
        }
        let starts: Vec<usize> = { let mut s: Vec<usize> = glyphs.iter().map(|g| g.ch).collect(); s.sort_unstable(); s.dedup(); s };
        for (i, h) in head.iter_mut().enumerate() {
            *h = starts[..starts.partition_point(|s| *s <= i)].last().copied().unwrap_or(i);
        }
        // Each unit's own advance, and what kerning adds to it.
        let raw: Vec<f64> = units[0].iter().map(|u| *u as f64 * scale).collect();
        // How far the pen moves after each unit: its advance, the tracking, and the kern.
        let step: Vec<f64> = (0..chars.len())
            .map(|i| if head[i] != i { 0.0 } else { raw[i] + track + (units[1][i] - units[0][i]) as f64 * scale + states[i].1 })
            .collect();
        let mut from = vec![0.0];
        for s in &step {
            from.push(from.last().unwrap() + s);
        }
        // A line's width, from its first glyph's origin to its last glyph's advance, without the
        // tracking and kern that lead to a next glyph.
        let width = |a: usize, e: usize| if e > a { from[head[e - 1]] - from[a] + raw[head[e - 1]] } else { 0.0 };
        let mut lines = Vec::new();
        if boxed {
            let space = |i: usize| chars[i].is_whitespace();
            // CJK is broken between any two characters (ponytail: without kinsoku, so a 。 may
            // start a line).
            let cjk = |i: usize| chars[i] as u32 >= 0x2E80;
            let trimmed = |a: usize, mut e: usize| {
                while e > a && space(e - 1) {
                    e -= 1;
                }
                e
            };
            let mut start = 0;
            while start < chars.len() {
                let mut end = chars.len();
                let mut chance = None;
                for k in start + 1..=chars.len() {
                    if k - 1 > start && width(start, trimmed(start, k)) > text.box_width {
                        end = chance.unwrap_or(k - 1);
                        break;
                    }
                    if k < chars.len() && !space(k) && (space(k - 1) || cjk(k - 1) || cjk(k)) {
                        chance = Some(k);
                    }
                }
                lines.push((start, end, end == chars.len()));
                start = end;
                while start < chars.len() && space(start) {
                    start += 1;
                }
            }
        }
        if lines.is_empty() {
            lines.push((0, chars.len(), true));
        }
        for (a, end, last) in lines {
            let e = if boxed { (a..end).rev().find(|i| !chars[*i].is_whitespace()).map_or(a, |i| i + 1) } else { end };
            let w = width(a, e);
            // In a justified line the extra room goes to its spaces, or between every two
            // characters when it has none, as Japanese has none.
            let stretch = boxed && text.align == Align::Justify && !last && e > a + 1;
            let gaps = (a..e.saturating_sub(1)).filter(|i| chars[*i].is_whitespace()).count();
            let extra = if stretch { (text.box_width - w).max(0.0) } else { 0.0 };
            let per = |i: usize| match (stretch, gaps) {
                (false, _) => 0.0,
                (true, 0) => extra / (e - a - 1) as f64,
                (true, g) => if chars[i].is_whitespace() { extra / g as f64 } else { 0.0 },
            };
            let left = if boxed {
                text.at[0]
                    + match text.align {
                        Align::Left | Align::Justify => 0.0,
                        Align::Center => (text.box_width - w) / 2.0,
                        Align::Right => text.box_width - w,
                    }
            } else {
                text.at[0]
                    - match text.align {
                        Align::Left | Align::Justify => 0.0,
                        Align::Center => w / 2.0,
                        Align::Right => w,
                    }
            };
            let y = first + n as f64 * line;
            let mut x = left;
            // D-371: the line's characters in the order their glyphs are drawn, each cluster's
            // characters together.
            let mut order: Vec<usize> = Vec::with_capacity(e - a);
            for g in glyphs.iter().filter(|g| (a..e).contains(&g.ch)) {
                if order.last().is_none_or(|l| head[*l] != g.ch) {
                    order.extend((g.ch..e).take_while(|j| head[*j] == g.ch));
                }
            }
            for i in order {
                let p = &mut states[i].0;
                p.anchor = [x + raw[i] / 2.0, y];
                let turn = (p.moved != [0.0; 2] || p.turn != 0.0 || p.scale != [1.0; 2]).then(|| {
                    let (sin, cos) = p.turn.to_radians().sin_cos();
                    Turn { anchor: p.anchor, scale: p.scale, cos, sin, moved: p.moved }
                });
                let from = contours.len();
                let mut gx = 0;
                for g in glyphs.iter().filter(|g| g.ch == i) {
                    // D-372 (a), the owner's choice of 2026-10-09: a TrueType outline slides
                    // sideways so its left edge meets the side bearing in hmtx, its lsb less the
                    // xMin in the glyph's own header, as FreeType, HarfBuzz and Windows draw it.
                    // A CFF font has no glyf table and is drawn as it is.
                    let slid = face.tables().glyf.and_then(|t| Some(i32::from(face.glyph_hor_side_bearing(g.id)?) - i32::from(t.bbox(g.id)?.x_min))).unwrap_or(0);
                    let (gx_at, gy) = ((gx + g.offset.0 + slid) as f64 * scale, g.offset.1 as f64 * scale);
                    let mut pen = Pen { x: x + gx_at, y: y - gy, scale, slant, turn, now: Vec::new(), into: &mut contours, path: path.then(Vec::new) };
                    face.outline_glyph(g.id, &mut pen);
                    pen.end();
                    gx += g.advance;
                }
                p.bounds = contours[from..].iter().flatten().fold(None, |b: Option<[f64; 4]>, &(px, py)| {
                    Some(b.map_or([px, py, px, py], |b| [b[0].min(px), b[1].min(py), b[2].max(px), b[3].max(py)]))
                });
                placed.push((p.clone(), from..contours.len()));
                x += step[i] + per(i);
            }
            across = [across[0].min(left), across[1].max(left + w + extra)];
            n += 1;
        }
    }
    if boxed {
        across = [text.at[0], text.at[0] + text.box_width];
    }
    let last = first + (n.max(1) - 1) as f64 * line;
    let mut bounds = [across[0], first - ascent, across[1], last - face.descender() as f64 * scale];
    for &(x, y) in contours.iter().flatten() {
        bounds = [bounds[0].min(x), bounds[1].min(y), bounds[2].max(x), bounds[3].max(y)];
    }
    if !bounds[0].is_finite() {
        bounds[0] = text.at[0];
        bounds[2] = text.at[0];
    }
    Some(Laid { contours, bounds, chars: placed })
}

/// D-371: the font's standard ligatures (`liga`, `clig`), such as the bundled font's "fi", on by
/// the owner's choice ("ligatures on by default", 2026-10-09). Required ligatures (Arabic's
/// lam-alef) are on regardless.
const LIGATURES: bool = true;

/// D-371: one glyph of a shaped paragraph. Its advance and offsets are in font units, the
/// advance with any kern in it; `ch` is the paragraph's character its cluster starts at.
pub struct Glyph {
    pub id: ttf_parser::GlyphId,
    pub advance: i32,
    pub offset: (i32, i32),
    pub ch: usize,
}

/// D-371: a paragraph shaped by `rustybuzz`, its glyphs in the order they are drawn, left to
/// right. The paragraph is cut into runs of one script and direction, which are shaped apart:
/// a mark, a space or a sign that is in no script goes with the run before it (the first run,
/// at the start), and figures in a right-to-left run are a left-to-right run of their own. A
/// paragraph whose first run reads right to left lays its runs out from the right.
/// ponytail: that is the bidirectional algorithm's commonest case, not the algorithm (UAX #9):
/// a sign between runs of two directions sides with the run before it.
pub fn shaped(face: &rustybuzz::Face, chars: &[char], kerning: bool) -> Vec<Glyph> {
    use rustybuzz::{script, Direction, Feature, Script, UnicodeBuffer};
    let tag = ttf_parser::Tag::from_bytes;
    let features = [
        // Over every character but not "global": rustybuzz 0.20.1 skips turning a right-to-left
        // run back round when a global kern is off in a font with an old `kern` table
        // (`kerning.rs`, the `continue` before the second `reverse`), and Arabic comes out
        // backwards. Off over a range does the same as off everywhere.
        Feature::new(tag(b"kern"), u32::from(kerning), 0..u32::MAX as usize),
        Feature::new(tag(b"liga"), u32::from(LIGATURES), ..),
        Feature::new(tag(b"clig"), u32::from(LIGATURES), ..),
    ];
    // The script and direction each character asks for, `None` for one that asks for neither.
    let wants = |c: char, in_rtl: bool| -> Option<(Script, Direction)> {
        if c.is_ascii_digit() {
            return in_rtl.then_some((script::COMMON, Direction::LeftToRight));
        }
        let mut b = UnicodeBuffer::new();
        b.add(c, 0);
        b.guess_segment_properties();
        let s = b.script();
        // Japanese and Chinese mix kanji, kana and bopomofo in one run, as one script.
        let s = if [script::HIRAGANA, script::KATAKANA, script::BOPOMOFO].contains(&s) { script::HAN } else { s };
        (s != script::UNKNOWN).then(|| (s, b.direction()))
    };
    let mut runs: Vec<(usize, usize, Script, Direction)> = Vec::new();
    for (i, c) in chars.iter().enumerate() {
        let in_rtl = runs.last().is_some_and(|r| r.3 == Direction::RightToLeft);
        match (wants(*c, in_rtl), runs.last_mut()) {
            (Some(w), Some(r)) if (r.2, r.3) == w || r.2 == script::UNKNOWN => {
                (r.2, r.3) = w;
                r.1 = i + 1;
            }
            (None, Some(r)) => r.1 = i + 1,
            (w, _) => {
                let (s, d) = w.unwrap_or((script::UNKNOWN, Direction::LeftToRight));
                runs.push((i, i + 1, s, d));
            }
        }
    }
    if runs.first().is_some_and(|r| r.3 == Direction::RightToLeft) {
        runs.reverse();
    }
    let mut glyphs = Vec::new();
    for (a, e, s, d) in runs {
        let words: String = chars[a..e].iter().collect();
        // Each character's byte in the run, as `rustybuzz` names clusters.
        let bytes: Vec<usize> = words.char_indices().map(|(b, _)| b).collect();
        let mut b = UnicodeBuffer::new();
        b.push_str(&words);
        if s != script::UNKNOWN {
            b.set_script(s);
        }
        b.set_direction(d);
        let out = rustybuzz::shape(face, &features, b);
        for (info, pos) in out.glyph_infos().iter().zip(out.glyph_positions()) {
            glyphs.push(Glyph {
                id: ttf_parser::GlyphId(info.glyph_id as u16),
                advance: pos.x_advance,
                offset: (pos.x_offset, pos.y_offset),
                ch: a + bytes.partition_point(|b| *b < info.cluster as usize),
            });
        }
    }
    glyphs
}

/// A box with its corners rounded by `r` pixels, as one closed outline.
fn rounded_box(l: f64, t: f64, r_: f64, b: f64, round: f64) -> Vec<(f64, f64)> {
    let r = round.min((r_ - l) / 2.0).min((b - t) / 2.0).max(0.0);
    let corners = [(r_ - r, t + r, -90.0), (r_ - r, b - r, 0.0), (l + r, b - r, 90.0), (l + r, t + r, 180.0)];
    let pieces = if r > 0.0 { (r / 2.0).ceil().clamp(2.0, 64.0) as usize } else { 1 };
    let mut outline = Vec::new();
    for (cx, cy, from) in corners {
        for i in 0..=pieces {
            let a = (from + 90.0 * i as f64 / pieces as f64).to_radians();
            outline.push((cx + r * a.cos(), cy + r * a.sin()));
        }
    }
    outline
}

/// Every pixel's cover by the band within `reach` of the outlines, which is shape strokes'
/// field asked only over the outlines' box.
fn around(contours: &[Vec<(f64, f64)>], reach: f64, w: usize, h: usize) -> Vec<f32> {
    let mut field = vec![0.0f32; w * h];
    let points = || contours.iter().flatten();
    if contours.is_empty() || w == 0 || h == 0 {
        return field;
    }
    let left = (points().map(|p| p.0).fold(f64::INFINITY, f64::min) - reach - 1.0).floor().clamp(0.0, w as f64) as usize;
    let top = (points().map(|p| p.1).fold(f64::INFINITY, f64::min) - reach - 1.0).floor().clamp(0.0, h as f64) as usize;
    let right = (points().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max) + reach + 2.0).ceil().clamp(0.0, w as f64) as usize;
    let bottom = (points().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max) + reach + 2.0).ceil().clamp(0.0, h as f64) as usize;
    if right <= left || bottom <= top {
        return field;
    }
    let (ox, oy) = (left as f64, top as f64);
    let segments: Vec<_> = contours
        .iter()
        .flat_map(|c| crate::shape::path_segments(c, true))
        .map(|(a, b)| ((a.0 - ox, a.1 - oy), (b.0 - ox, b.1 - oy)))
        .collect();
    let part = crate::shape::stroke_field(&segments, right - left, bottom - top, reach);
    for (y, row) in part.chunks_exact(right - left).enumerate() {
        field[(top + y) * w + left..(top + y) * w + right].copy_from_slice(row);
    }
    field
}

/// The larger cover of the two, pixel by pixel.
fn union(into: &mut [f32], other: &[f32]) {
    into.iter_mut().zip(other).for_each(|(a, b)| *a = a.max(*b));
}

/// D-265: the part of the frame, left, top, right, bottom, that a field moved by (dx, dy) can
/// cover: its covered pixels' box, moved, and one more pixel right and down for the sampling
/// between pixels. `None` when nothing is covered or it all moves out of the frame.
fn moved_box(field: &[f32], w: usize, h: usize, dx: f64, dy: f64) -> Option<[usize; 4]> {
    let rows: Vec<usize> = (0..h).filter(|y| field[y * w..(y + 1) * w].iter().any(|v| *v != 0.0)).collect();
    let (&top, &bottom) = (rows.first()?, rows.last()?);
    let row = |y: usize| &field[y * w..(y + 1) * w];
    let left = rows.iter().filter_map(|y| row(*y).iter().position(|v| *v != 0.0)).min()?;
    let right = rows.iter().filter_map(|y| row(*y).iter().rposition(|v| *v != 0.0)).max()?;
    let (fx, fy) = (dx.floor() as i64, dy.floor() as i64);
    let clamp = |v: i64, most: usize| v.clamp(0, most as i64) as usize;
    let (l, t) = (clamp(left as i64 + fx, w), clamp(top as i64 + fy, h));
    let (r, b) = (clamp(right as i64 + fx + 2, w), clamp(bottom as i64 + fy + 2, h));
    (r > l && b > t).then_some([l, t, r, b])
}

/// A field moved by (dx, dy) pixels, sampled between pixels bilinearly; what moves in from
/// outside is empty. Only the part `[l, t, r, b]` of the frame is given, row by row.
fn shifted(field: &[f32], w: usize, h: usize, dx: f64, dy: f64, [l, t, r, b]: [usize; 4]) -> Vec<f32> {
    let (fx, fy) = (dx.floor(), dy.floor());
    let (tx, ty) = ((dx - fx) as f32, (dy - fy) as f32);
    let at = |x: i64, y: i64| if x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h { field[y as usize * w + x as usize] } else { 0.0 };
    let (fx, fy) = (fx as i64, fy as i64);
    let mut out = vec![0.0f32; (r - l) * (b - t)];
    out.par_chunks_mut(r - l).enumerate().for_each(|(k, row)| {
        let y = t + k;
        for (i, v) in row.iter_mut().enumerate() {
            let x = l + i;
            let (sx, sy) = (x as i64 - fx, y as i64 - fy);
            let top = at(sx, sy) * (1.0 - tx) + at(sx - 1, sy) * tx;
            let low = at(sx, sy - 1) * (1.0 - tx) + at(sx - 1, sy - 1) * tx;
            *v = top * (1.0 - ty) + low * ty;
        }
    });
    out
}

/// Every pixel's cover by closed outlines under the nonzero rule, from ADR-016's 4 by 4 samples.
///
/// The mask scanline's walk (`mask::scanline_field`) with a count in place of a parity: each
/// edge crossed adds one going down and takes one going up, and a sample is inside when the
/// count is not zero. Only the rows the outlines reach are visited.
pub fn fill(contours: &[Vec<(f64, f64)>], w: usize, h: usize) -> Vec<f32> {
    let n = SAMPLES_PER_SIDE;
    let mut field = vec![0.0f32; w * h];
    let edges: Vec<[f64; 4]> = contours
        .iter()
        .flat_map(|c| (0..c.len()).map(move |i| {
            let (a, b) = (c[i], c[(i + 1) % c.len()]);
            [a.0, a.1, b.0, b.1]
        }))
        .filter(|e| e[1] != e[3])
        .collect();
    if edges.is_empty() || w == 0 {
        return field;
    }
    let top = edges.iter().map(|e| e[1].min(e[3])).fold(f64::INFINITY, f64::min);
    let bottom = edges.iter().map(|e| e[1].max(e[3])).fold(f64::NEG_INFINITY, f64::max);
    let first = (top.floor().max(0.0) as usize).min(h);
    let last = (bottom.ceil().max(0.0) as usize).min(h);
    field[first * w..last * w]
        .par_chunks_mut(w)
        .enumerate()
        .for_each_init(
            || (Vec::new(), vec![0u32; w]),
            |(crossings, hits): &mut (Vec<(f64, i32)>, Vec<u32>), (k, row)| {
                hits.iter_mut().for_each(|hit| *hit = 0);
                let y = (first + k) as f64;
                for j in 0..n {
                    let sy = y + (j as f64 + 0.5) / n as f64;
                    crossings.clear();
                    for &[x0, y0, x1, y1] in &edges {
                        if (y0 <= sy) != (y1 <= sy) {
                            let t = (sy - y0) / (y1 - y0);
                            crossings.push((x0 + t * (x1 - x0), if y1 > y0 { 1 } else { -1 }));
                        }
                    }
                    crossings.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("an edge crossing is never NaN"));
                    let (mut at, mut count) = (0, 0);
                    for (xi, hit) in hits.iter_mut().enumerate() {
                        for i in 0..n {
                            let sx = xi as f64 + (i as f64 + 0.5) / n as f64;
                            while at < crossings.len() && crossings[at].0 <= sx {
                                count += crossings[at].1;
                                at += 1;
                            }
                            if count != 0 {
                                *hit += 1;
                            }
                        }
                    }
                }
                for (v, hit) in row.iter_mut().zip(hits.iter()) {
                    *v = *hit as f32 / (n * n) as f32;
                }
            },
        );
    field
}
