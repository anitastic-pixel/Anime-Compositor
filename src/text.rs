//! D-263: a text layer's words, drawn from a font's own outlines.
//!
//! The outlines are read by `ttf-parser` and filled here through the same sixteen samples a
//! pixel that masks and shapes use (ADR-016), so text, shapes and masks have one edge quality.
//! Fonts fill by the nonzero rule rather than the even-odd rule masks use: a letter may be built
//! from outlines that overlap, and the overlap is ink, not a hole.
//!
//! Characters are placed one after another by their advance widths. ponytail: no kerning and no
//! shaping, which Latin and Japanese do without; Arabic or the Indic scripts would need a shaper
//! (`harfrust`, named in document 15) before they joined correctly.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use rayon::prelude::*;

use crate::mask::SAMPLES_PER_SIDE;
use crate::WorkingBuffer;

/// Which end of each line sits at the text's place.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Align {
    pub fn as_str(self) -> &'static str {
        match self {
            Align::Left => "left",
            Align::Center => "center",
            Align::Right => "right",
        }
    }

    pub fn parse(s: &str) -> Option<Align> {
        match s {
            "left" => Some(Align::Left),
            "center" => Some(Align::Center),
            "right" => Some(Align::Right),
            _ => None,
        }
    }
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
    pub at: [f64; 2],
    pub align: Align,
}

impl Text {
    /// The font that comes with the program: M PLUS Rounded 1c, under the SIL Open Font
    /// License (`docs/third_party/MPLUSRounded1c-OFL.txt`). It has Latin, kana and kanji.
    pub const BUNDLED_FONT: &'static str = "MPLUSRounded1c-Regular.ttf";
    pub const MAX_SIZE: f64 = 2000.0;

    /// What is outside D-263's ranges, as a sentence, or `None` for words that may be drawn.
    pub fn problem(&self) -> Option<String> {
        if !(1.0..=Self::MAX_SIZE).contains(&self.size) {
            return Some(format!("a size from 1 to {} pixels, not {}", Self::MAX_SIZE, self.size));
        }
        if let Some(c) = self.color.iter().find(|c| !(0.0..=1.0).contains(*c)) {
            return Some(format!("a colour of three numbers from 0 to 1, not {c}"));
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

/// The font's bytes, read once a run. `None` when this machine does not have it.
///
/// ponytail: a font read is kept for the run, so a font installed or replaced while the program
/// is open is seen after a restart; nothing in a frame can tell the two files apart otherwise.
fn font(name: &str) -> Option<&'static [u8]> {
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

/// Collects a glyph's outlines in the picture's pixels, curves cut into short straight lines.
struct Pen<'a> {
    x: f64,
    y: f64,
    scale: f64,
    now: Vec<(f64, f64)>,
    into: &'a mut Vec<Vec<(f64, f64)>>,
}

impl Pen<'_> {
    fn at(&self, x: f32, y: f32) -> (f64, f64) {
        // Font units go up; pixels go down.
        (self.x + x as f64 * self.scale, self.y - y as f64 * self.scale)
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
    let face = ttf_parser::Face::parse(font(&text.font)?, 0).ok()?;
    let scale = text.size / face.units_per_em() as f64;
    let line = (face.ascender() as f64 - face.descender() as f64 + face.line_gap() as f64) * scale;
    let mut contours = Vec::new();
    for (n, words) in text.text.split('\n').enumerate() {
        // A glyph the font lacks is its "missing" glyph, usually a box: seen, not hidden.
        let glyphs: Vec<ttf_parser::GlyphId> = words
            .trim_end_matches('\r')
            .chars()
            .map(|c| face.glyph_index(c).unwrap_or(ttf_parser::GlyphId(0)))
            .collect();
        let advance = |g: ttf_parser::GlyphId| face.glyph_hor_advance(g).unwrap_or(0) as f64 * scale;
        let across: f64 = glyphs.iter().map(|g| advance(*g)).sum();
        let mut x = text.at[0]
            - match text.align {
                Align::Left => 0.0,
                Align::Center => across / 2.0,
                Align::Right => across,
            };
        let y = text.at[1] + n as f64 * line;
        for g in glyphs {
            let mut pen = Pen { x, y, scale, now: Vec::new(), into: &mut contours };
            face.outline_glyph(g, &mut pen);
            pen.end();
            x += advance(g);
        }
    }
    let field = fill(&contours, width, height);
    let mut picture = WorkingBuffer::transparent(width, height);
    crate::shape::paint(picture.data_mut(), &field, width, text.color, 1.0, None);
    Some(picture)
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
