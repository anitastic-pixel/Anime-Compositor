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
    fs::write(path, encode_rgba(width, height, depth, tags, None, samples)?)
}

/// D-917: a colour chunk an exported file carries with colour management on: its four letters
/// and its contents, as the export chooses them.
pub type ColourChunk = ([u8; 4], &'static [u8]);

/// `sRGB`, rendering intent 0 (perceptual).
pub const SRGB_CHUNK: ColourChunk = (*b"sRGB", &[0]);
/// `cICP` 1, 1, 0, 1: BT.709 primaries, BT.709 transfer (H.273 gives BT.1886 material the same
/// code point), RGB, full range.
pub const CICP_REC709_CHUNK: ColourChunk = (*b"cICP", &[1, 1, 0, 1]);

/// Write `chunk` after the header, before the pictures, where both chunks belong.
pub fn write_colour<W: io::Write>(
    writer: &mut png::Writer<W>,
    chunk: Option<ColourChunk>,
) -> io::Result<()> {
    match chunk {
        Some((name, data)) => writer
            .write_chunk(png::chunk::ChunkType(name), data)
            .map_err(io::Error::other),
        None => Ok(()),
    }
}

/// [`write_rgba`]'s file, as bytes, for D-231's export, which encodes frames side by side and
/// writes them in order.
pub fn encode_rgba(
    width: usize,
    height: usize,
    depth: OutputDepth,
    tags: &[(&str, String)],
    colour: Option<ColourChunk>,
    samples: &[u8],
) -> io::Result<Vec<u8>> {
    let mut file = Vec::new();
    let mut encoder = png::Encoder::new(&mut file, width as u32, height as u32);
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
    let mut writer = encoder.write_header().map_err(io::Error::other)?;
    write_colour(&mut writer, colour)?;
    writer.write_image_data(samples).map_err(io::Error::other)?;
    writer.finish().map_err(io::Error::other)?;
    Ok(file)
}
