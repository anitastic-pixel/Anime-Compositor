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
    now: Vec<(f64, f64)>,
    into: &'a mut Vec<Vec<(f64, f64)>>,
}

impl Pen<'_> {
    fn at(&self, x: f32, y: f32) -> (f64, f64) {
        // Font units go up; pixels go down.
        (self.x + (x as f64 + y as f64 * self.slant) * self.scale, self.y - y as f64 * self.scale)
    }

    fn end(&mut self) {
        let now = std::mem::take(&mut self.now);
        if now.len() >= 3 {
            self.into.push(now);
        }
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
    }
    fn line_to(&mut self, x: f32, y: f32) {
        let p = self.at(x, y);
        self.now.push(p);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        let (c, to) = (self.at(x1, y1), self.at(x, y));
        self.curve(&[c], to);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        let (c1, c2, to) = (self.at(x1, y1), self.at(x2, y2), self.at(x, y));
        self.curve(&[c1, c2], to);
    }
    fn close(&mut self) {
        self.end();
    }
}

/// Document 21 step 1 for a text layer: the composition's size in transparent black with the
/// words drawn into it. `None` when the font is not on this machine or is not a font, which the
/// caller says per frame; nothing is drawn in another font's place.
pub fn draw(text: &Text, width: usize, height: usize) -> Option<WorkingBuffer> {
    let laid = lay_out(text)?;
    let (w, h) = (width, height);
    let mut picture = WorkingBuffer::transparent(w, h);
    if let Some(b) = &text.background {
        let p = b.padding;
        let [l, t, r, bottom] = laid.bounds;
        let field = fill(&[rounded_box(l - p, t - p, r + p, bottom + p, b.roundness)], w, h);
        crate::shape::paint(picture.data_mut(), &field, w, b.color, b.opacity, None);
    }
    let mut ink = fill(&laid.contours, w, h);
    let bold = if text.faux_bold { text.size * 0.02 } else { 0.0 };
    if bold > 0.0 {
        union(&mut ink, &around(&laid.contours, bold, w, h));
    }
    let lined = text.stroke.as_ref().map(|s| {
        let mut f = around(&laid.contours, bold + s.width, w, h);
        union(&mut f, &ink);
        f
    });
    if let Some(s) = &text.shadow {
        let under = lined.as_ref().unwrap_or(&ink);
        // After Effects' compass: 0 is up, 90 is right, and y grows downwards in pixels.
        let a = s.angle.to_radians();
        let (dx, dy) = (a.sin() * s.distance, -a.cos() * s.distance);
        // D-265: only the part of the frame the moved letters reach is worked. Outside it the
        // shadow is empty, the blur skips empty pixels and adds its taps in the same order
        // wherever the part starts, so every pixel is the one the whole frame gave.
        if let Some([l, t, r, b]) = moved_box(under, w, h, dx, dy) {
            let pw = r - l;
            let mut shadow = WorkingBuffer::transparent(pw, b - t);
            let part = shifted(under, w, h, dx, dy, [l, t, r, b]);
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
    if let (Some(s), Some(f)) = (&text.stroke, &lined) {
        crate::shape::paint(picture.data_mut(), f, w, s.color, 1.0, None);
    }
    crate::shape::paint(picture.data_mut(), &ink, w, text.color, 1.0, None);
    Some(picture)
}

/// D-265: [`draw`], keeping the last few pictures drawn. Text settings have no keys, so a text
/// layer is the same picture on every frame, and playing or scrubbing it draws it once. A
/// picture is handed out shared; the render copies it before an effect or mask changes it.
// ponytail: four 1080p pictures, about 130 MB; key by layer if a project shows more text layers
// than that redrawing on every frame.
pub fn drawn(text: &Text, width: usize, height: usize) -> Option<Arc<WorkingBuffer>> {
    static KEPT: Mutex<Vec<(Text, usize, usize, Arc<WorkingBuffer>)>> = Mutex::new(Vec::new());
    let hit = |kept: &mut Vec<(Text, usize, usize, Arc<WorkingBuffer>)>| {
        let i = kept.iter().position(|(t, w, h, _)| t == text && *w == width && *h == height)?;
        let found = kept.remove(i);
        let picture = found.3.clone();
        kept.push(found);
        Some(picture)
    };
    if let Some(p) = hit(&mut KEPT.lock().unwrap_or_else(|e| e.into_inner())) {
        return Some(p);
    }
    let picture = Arc::new(draw(text, width, height)?);
    let mut kept = KEPT.lock().unwrap_or_else(|e| e.into_inner());
    if hit(&mut kept).is_none() {
        if kept.len() == 4 {
            kept.remove(0);
        }
        kept.push((text.clone(), width, height, picture.clone()));
    }
    Some(picture)
}

/// D-264: the words' box in the layer's pixels, left, top, right, bottom: from the first line's
/// ascender to the last line's descender and across every line (a box text's whole box), grown
/// to hold any outline that reaches past it. The background is drawn round it, and the window
/// outlines and picks a text layer by it. `None` when the font is not on this machine.
pub fn bounds(text: &Text) -> Option<[f64; 4]> {
    lay_out(text).map(|l| l.bounds)
}

/// The words placed: every glyph's outline in the layer's pixels, and their box.
struct Laid {
    contours: Vec<Vec<(f64, f64)>>,
    bounds: [f64; 4],
}

fn lay_out(text: &Text) -> Option<Laid> {
    let face = ttf_parser::Face::parse(font_bytes(&text.font)?, 0).ok()?;
    let scale = text.size / face.units_per_em() as f64;
    let natural = (face.ascender() as f64 - face.descender() as f64 + face.line_gap() as f64) * scale;
    let line = if text.leading > 0.0 { text.leading } else { natural };
    let boxed = text.box_width > 0.0;
    let ascent = face.ascender() as f64 * scale;
    let first = if boxed { text.at[1] + ascent } else { text.at[1] };
    let track = text.tracking / 1000.0 * text.size;
    let pairs = if text.kerning { kern_lookups(&face) } else { Vec::new() };
    let words = if text.all_caps { text.text.to_uppercase() } else { text.text.clone() };
    let slant = if text.faux_italic { 0.2126 } else { 0.0 };
    let mut contours = Vec::new();
    let mut across = [f64::INFINITY, f64::NEG_INFINITY];
    let mut n = 0;
    for paragraph in words.split('\n') {
        let chars: Vec<char> = paragraph.trim_end_matches('\r').chars().collect();
        // A glyph the font lacks is its "missing" glyph, usually a box: seen, not hidden.
        let glyphs: Vec<ttf_parser::GlyphId> =
            chars.iter().map(|c| face.glyph_index(*c).unwrap_or(ttf_parser::GlyphId(0))).collect();
        let raw: Vec<f64> = glyphs.iter().map(|g| face.glyph_hor_advance(*g).unwrap_or(0) as f64 * scale).collect();
        // How far the pen moves after each glyph: its advance, the tracking, and the pair kern.
        let step: Vec<f64> = (0..glyphs.len())
            .map(|i| {
                let kern = match glyphs.get(i + 1) {
                    Some(next) if text.kerning => kern(&face, &pairs, glyphs[i], *next) * scale,
                    _ => 0.0,
                };
                raw[i] + track + kern
            })
            .collect();
        let mut from = vec![0.0];
        for s in &step {
            from.push(from.last().unwrap() + s);
        }
        // A line's width, from its first glyph's origin to its last glyph's advance, without the
        // tracking and kern that lead to a next glyph.
        let width = |a: usize, e: usize| if e > a { from[e - 1] - from[a] + raw[e - 1] } else { 0.0 };
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
            for i in a..e {
                let mut pen = Pen { x, y, scale, slant, now: Vec::new(), into: &mut contours };
                face.outline_glyph(glyphs[i], &mut pen);
                pen.end();
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
    Some(Laid { contours, bounds })
}

/// The `kern` feature's lookups in the font's GPOS table, each once.
fn kern_lookups(face: &ttf_parser::Face) -> Vec<u16> {
    let mut found = Vec::new();
    if let Some(gpos) = face.tables().gpos {
        for feature in gpos.features {
            if feature.tag == ttf_parser::Tag::from_bytes(b"kern") {
                found.extend(feature.lookup_indices);
            }
        }
    }
    found.sort_unstable();
    found.dedup();
    found
}

/// How far, in font units, the pair `l` then `r` is kerned: each lookup's first pair subtable
/// that holds `l`, summed; a font with no GPOS kern feature is asked its old `kern` table.
fn kern(face: &ttf_parser::Face, lookups: &[u16], l: ttf_parser::GlyphId, r: ttf_parser::GlyphId) -> f64 {
    use ttf_parser::gpos::{PairAdjustment, PositioningSubtable};
    let Some(gpos) = face.tables().gpos.filter(|_| !lookups.is_empty()) else {
        let Some(table) = face.tables().kern else { return 0.0 };
        return table
            .subtables
            .into_iter()
            .filter(|s| s.horizontal && !s.variable)
            .filter_map(|s| s.glyphs_kerning(l, r))
            .map(f64::from)
            .sum();
    };
    let mut total = 0.0;
    for &index in lookups {
        let Some(lookup) = gpos.lookups.get(index) else { continue };
        for j in 0..lookup.subtables.len() {
            let found = match lookup.subtables.get::<PositioningSubtable>(j) {
                Some(PositioningSubtable::Pair(PairAdjustment::Format1 { coverage, sets })) => coverage
                    .get(l)
                    .map(|i| sets.get(i).and_then(|set| set.get(r)).map_or(0, |v| v.0.x_advance)),
                Some(PositioningSubtable::Pair(PairAdjustment::Format2 { coverage, classes, matrix })) => coverage
                    .get(l)
                    .map(|_| matrix.get((classes.0.get(l), classes.1.get(r))).map_or(0, |v| v.0.x_advance)),
                _ => None,
            };
            if let Some(v) = found {
                total += f64::from(v);
                break;
            }
        }
    }
    total
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
