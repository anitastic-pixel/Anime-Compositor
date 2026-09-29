//! B-125b: D-189's layer setting, the picture of a layer of the holder's own composition read by
//! an effect as a map. [`crate::compose::layer_map`] makes the picture, by document 21's steps 1
//! to 3 for the layer named; this is what is done to it after: cut back to its own rectangle, and
//! fitted to the holder.

use crate::WorkingBuffer;

/// The `size` rectangle of `picture` whose corner is at `origin`: a picture an effect grew, cut
/// back to what it was before (D-189's step 1). What lies outside `picture` is transparent.
pub(crate) fn cut(picture: &WorkingBuffer, origin: (usize, usize), size: (usize, usize)) -> WorkingBuffer {
    let mut out = WorkingBuffer::transparent(size.0, size.1);
    let (w, h) = (picture.width(), picture.height());
    let across = size.0.min(w.saturating_sub(origin.0));
    for y in 0..size.1.min(h.saturating_sub(origin.1)) {
        let from = ((origin.1 + y) * w + origin.0) * 4;
        out.data_mut()[y * size.0 * 4..][..across * 4]
            .copy_from_slice(&picture.data()[from..from + across * 4]);
    }
    out
}

/// Document 21's step 2 for a layer setting: `picture` fitted to the holder's `size` by `word`,
/// `center`, `tile` or `stretch`. An empty picture fits to transparent. `None` for any other
/// word: the effect holding it is refused by its own setting check before it asks.
pub(crate) fn fit(picture: &WorkingBuffer, word: &str, size: (usize, usize)) -> Option<WorkingBuffer> {
    if !matches!(word, "center" | "tile" | "stretch") {
        return None;
    }
    let (fw, fh) = size;
    let (w, h) = (picture.width(), picture.height());
    let mut out = WorkingBuffer::transparent(fw, fh);
    if w == 0 || h == 0 {
        return Some(out);
    }
    let dx = (fw as i64 - w as i64).div_euclid(2);
    let dy = (fh as i64 - h as i64).div_euclid(2);
    let src = picture.data();
    let at = |x: usize, y: usize| &src[(y * w + x) * 4..][..4];
    for (i, px) in out.data_mut().chunks_exact_mut(4).enumerate() {
        let (x, y) = (i % fw, i / fw);
        match word {
            "center" => {
                let (sx, sy) = (x as i64 - dx, y as i64 - dy);
                if (0..w as i64).contains(&sx) && (0..h as i64).contains(&sy) {
                    px.copy_from_slice(at(sx as usize, sy as usize));
                }
            }
            "tile" => {
                let sx = (x as i64 - dx).rem_euclid(w as i64) as usize;
                let sy = (y as i64 - dy).rem_euclid(h as i64) as usize;
                px.copy_from_slice(at(sx, sy));
            }
            _ => {
                // Stretch: the pixel's centre read at the same fraction of the picture, held
                // within its outer pixels, bilinear in premultiplied light.
                let u = ((x as f64 + 0.5) * w as f64 / fw as f64 - 0.5).clamp(0.0, (w - 1) as f64);
                let v = ((y as f64 + 0.5) * h as f64 / fh as f64 - 0.5).clamp(0.0, (h - 1) as f64);
                let (i0, j0) = (u.floor() as usize, v.floor() as usize);
                let (i1, j1) = ((i0 + 1).min(w - 1), (j0 + 1).min(h - 1));
                let (fu, fv) = (u - i0 as f64, v - j0 as f64);
                for (c, out) in px.iter_mut().enumerate() {
                    let top = at(i0, j0)[c] as f64 * (1.0 - fu) + at(i1, j0)[c] as f64 * fu;
                    let bottom = at(i0, j1)[c] as f64 * (1.0 - fu) + at(i1, j1)[c] as f64 * fu;
                    *out = (top * (1.0 - fv) + bottom * fv) as f32;
                }
            }
        }
    }
    Some(out)
}
