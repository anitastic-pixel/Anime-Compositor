//! D-915 (ADR-023 step C5): the viewer's colour view with colour management on.
//!
//! Applied once, at the viewer's 8-bit encode, after the unpremultiply (document 21, "The view").
//! Exports, renders to file, trace images and thumbnails never take it. The graphics card's
//! encode pass does the same steps from the same table (`src/gpu.rs`, `encode`).

use crate::color::{self, Curve, Gamut};
use crate::model::Project;
use crate::{composite, perf, WorkingBuffer};
use rayon::prelude::*;
use std::sync::OnceLock;

/// A view of `color_settings.view`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum View {
    /// OpenColorIO's "ACES 2.0 - SDR 100 nits (Rec.709)" of "sRGB - Display": the default when on.
    #[default]
    Aces2Sdr,
    /// OpenColorIO's "Un-tone-mapped": to linear Rec.709, then the sRGB curve, clipped.
    UnToneMapped,
    /// Today's encode: the sRGB curve on the working numbers.
    Standard,
    /// A word this build does not have (`COLOR_VIEW_UNAVAILABLE`): kept in the file, shown as Standard.
    Unknown,
}

impl View {
    pub const ALL: [View; 3] = [View::Aces2Sdr, View::UnToneMapped, View::Standard];

    pub fn word(self) -> Option<&'static str> {
        match self {
            View::Aces2Sdr => Some("aces-2.0-sdr"),
            View::UnToneMapped => Some("un-tone-mapped"),
            View::Standard => Some("standard"),
            View::Unknown => None,
        }
    }

    pub fn from_word(word: &str) -> View {
        View::ALL
            .into_iter()
            .find(|v| v.word() == Some(word))
            .unwrap_or(View::Unknown)
    }

    pub fn name(self) -> &'static str {
        match self {
            View::Aces2Sdr => "ACES 2.0 SDR",
            View::UnToneMapped => "Un-tone-mapped",
            View::Standard => "Standard",
            View::Unknown => "unknown",
        }
    }
}

/// Points a side of the ACES table.
pub const SIZE: usize = 97;

/// `assets/colour/aces2_sdr_srgb_97.f32`, written by `tools/view_reference.py`: OpenColorIO
/// 2.5.1's bake from ACEScct, little-endian f32 RGB, red fastest. Its SHA-256 is in
/// `Fixtures/colour/view/expected_view.json` and checked by `tests/b794_colour_view.rs`.
pub static TABLE_BYTES: &[u8] = include_bytes!("../assets/colour/aces2_sdr_srgb_97.f32");

/// The table as numbers, read once.
pub fn table() -> &'static [f32] {
    static TABLE: OnceLock<Vec<f32>> = OnceLock::new();
    TABLE.get_or_init(|| {
        assert_eq!(TABLE_BYTES.len(), SIZE * SIZE * SIZE * 12, "the ACES view's table");
        TABLE_BYTES
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect()
    })
}

/// Tetrahedral lookup of an ACEScct colour, the same steps as the card's `tetra`.
pub fn lookup(e: [f32; 3]) -> [f32; 3] {
    let t = table();
    let n = (SIZE - 1) as f32;
    let mut i = [0usize; 3];
    let mut f = [0f32; 3];
    for c in 0..3 {
        let u = e[c].clamp(0.0, 1.0) * n;
        let u = if u.is_nan() { 0.0 } else { u };
        i[c] = (u.floor() as usize).min(SIZE - 2);
        f[c] = u - i[c] as f32;
    }
    let corner = |dr: usize, dg: usize, db: usize| {
        let k = 3 * ((i[0] + dr) + SIZE * ((i[1] + dg) + SIZE * (i[2] + db)));
        [t[k], t[k + 1], t[k + 2]]
    };
    let [fr, fg, fb] = f;
    let (a, b, w) = if fr > fg {
        if fg > fb {
            (corner(1, 0, 0), corner(1, 1, 0), [fr, fg, fb])
        } else if fr > fb {
            (corner(1, 0, 0), corner(1, 0, 1), [fr, fb, fg])
        } else {
            (corner(0, 0, 1), corner(1, 0, 1), [fb, fr, fg])
        }
    } else if fb > fg {
        (corner(0, 0, 1), corner(0, 1, 1), [fb, fg, fr])
    } else if fb > fr {
        (corner(0, 1, 0), corner(0, 1, 1), [fg, fb, fr])
    } else {
        (corner(0, 1, 0), corner(1, 1, 0), [fg, fr, fb])
    };
    let (c0, c1) = (corner(0, 0, 0), corner(1, 1, 1));
    std::array::from_fn(|c| {
        (1.0 - w[0]) * c0[c] + (w[0] - w[1]) * a[c] + (w[1] - w[2]) * b[c] + w[2] * c1[c]
    })
}

/// What the viewer shows: the view, and the working space it is applied from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Shown {
    pub view: View,
    pub working: Gamut,
}

impl Shown {
    pub const STANDARD: Shown = Shown {
        view: View::Standard,
        working: Gamut::Rec709,
    };

    /// The project's view: Standard with colour management off or a word this build does not have.
    pub fn of(project: &Project) -> Shown {
        match (project.color_management, project.color_view) {
            (Some(working), view @ (View::Aces2Sdr | View::UnToneMapped)) => Shown { view, working },
            _ => Shown::STANDARD,
        }
    }

    /// The card's number for the view: 0 is today's encode.
    pub fn code(self) -> u32 {
        match self.view {
            View::UnToneMapped => 1,
            View::Aces2Sdr => 2,
            View::Standard | View::Unknown => 0,
        }
    }

    /// The matrix from the working space to the view's, `None` when they are the same.
    fn matrix(self) -> Option<[[f64; 3]; 3]> {
        let to = match self.view {
            View::Aces2Sdr => Gamut::Ap1,
            _ => Gamut::Rec709,
        };
        (to != self.working).then(|| color::gamut_matrix(self.working, to))
    }

    /// The matrix as the card reads it, row by row; exactly the identity when there is none.
    pub fn card_matrix(self) -> [f32; 9] {
        let m = self
            .matrix()
            .unwrap_or([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
        std::array::from_fn(|k| m[k / 3][k % 3] as f32)
    }
}

/// The view's display value of one straight linear colour, `m` its [`Shown::matrix`].
fn display(view: View, m: Option<&[[f64; 3]; 3]>, rgb: [f32; 3]) -> [f32; 3] {
    let rgb = m.map_or(rgb, |m| color::apply_f32(m, rgb));
    match view {
        View::Aces2Sdr => lookup(rgb.map(|c| Curve::AcesCct.from_linear(c))),
        _ => rgb.map(color::linear_to_srgb),
    }
}

/// The viewer's eight-bit straight RGBA through `shown`; Standard is
/// [`WorkingBuffer::to_srgb8_straight`] itself.
pub fn encode8(buffer: &WorkingBuffer, shown: Shown) -> Vec<u8> {
    if shown.code() == 0 {
        return buffer.to_srgb8_straight();
    }
    perf::time(perf::Stage::Encode, || {
        let m = shown.matrix();
        let mut out = vec![0u8; buffer.data().len()];
        out.par_chunks_mut(4)
            .zip(buffer.data().par_chunks_exact(4))
            .for_each(|(dst, px)| {
                let s = composite::unpremultiply([px[0], px[1], px[2], px[3]]);
                let e = display(shown.view, m.as_ref(), [s[0], s[1], s[2]]);
                for c in 0..3 {
                    dst[c] = color::quantise_u8(e[c]);
                }
                dst[3] = color::quantise_u8(s[3]);
            });
        out
    })
}

/// D-918: one colour as the picker, the eyedropper and Info show it with colour management on.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Readout {
    /// What a file stores (D-910): linear Rec.709, straight.
    pub stored: [f32; 3],
    /// The working space's numbers, straight.
    pub working: [f32; 3],
    /// What the viewer shows: [`encode8`]'s bytes.
    pub display: [u8; 3],
    pub alpha: f32,
}

impl Readout {
    /// A stored colour, opaque, as a picker holds it.
    pub fn of_stored(project: &Project, rgb: [f32; 3]) -> Readout {
        let mut px = [rgb[0], rgb[1], rgb[2], 1.0];
        if let Some(working) = project.working() {
            color::convert_rgba(&mut px, Gamut::Rec709, working);
        }
        Readout {
            stored: rgb,
            ..Readout::of_pixel(project, px)
        }
    }

    /// A premultiplied working pixel of a picture.
    pub fn of_pixel(project: &Project, px: [f32; 4]) -> Readout {
        let s = composite::unpremultiply(px);
        let mut stored = s;
        if let Some(working) = project.working() {
            color::convert_rgba(&mut stored, working, Gamut::Rec709);
        }
        let mut one = WorkingBuffer::transparent(1, 1);
        one.data_mut().copy_from_slice(&px);
        let bytes = encode8(&one, Shown::of(project));
        Readout {
            stored: [stored[0], stored[1], stored[2]],
            working: [s[0], s[1], s[2]],
            display: [bytes[0], bytes[1], bytes[2]],
            alpha: s[3],
        }
    }
}
