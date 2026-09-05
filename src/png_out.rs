//! Writing an RGBA PNG, in one place.
//!
//! Two things in this build write PNGs: the render trace (ADR-012, diagnostic, never part of an
//! export) and the export writer itself. They write different tags and different depths, and
//! they must not write different *pixels*, so the encoder call lives here and both go through
//! it. A second spelling of "write these samples as a PNG" would be a second place for the
//! colour type, the depth or the chunk encoding to drift.

use std::fs;
use std::io;
use std::path::Path;

use crate::OutputDepth;

/// Write RGBA samples as a PNG.
///
/// `samples` is what [`crate::WorkingBuffer::encode`] produced at this depth: one byte per
/// channel at eight bits, two big-endian bytes at sixteen.
///
/// Tags are written as iTXt, not tEXt: a layer ID or a file name is UTF-8, and the reference
/// shot already contains one — `layer2_桜_013.png` — that Latin-1 cannot hold.
pub fn write_rgba(
    path: &Path,
    width: usize,
    height: usize,
    depth: OutputDepth,
    tags: &[(&str, String)],
    samples: &[u8],
) -> io::Result<()> {
    let file = io::BufWriter::new(fs::File::create(path)?);
    let mut encoder = png::Encoder::new(file, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(match depth {
        OutputDepth::Eight => png::BitDepth::Eight,
        OutputDepth::Sixteen => png::BitDepth::Sixteen,
    });
    for (key, value) in tags {
        encoder
            .add_itxt_chunk((*key).to_string(), value.clone())
            .map_err(io::Error::other)?;
    }
    encoder
        .write_header()
        .map_err(io::Error::other)?
        .write_image_data(samples)
        .map_err(io::Error::other)?;
    Ok(())
}
