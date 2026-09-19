//! D-72's one-file exports: a GIF, an animated PNG, and an MP4 (written by `mp4_out`). One file
//! holds every frame of the job.
//!
//! A GIF is a preview format: 256 colours a frame, alpha that is on or off, time in hundredths
//! of a second. An animated PNG loses nothing: it is written from the same samples as an exported
//! PNG frame, and its frame delay is a fraction, so it is exact.

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::mp4_out::Mp4;
use crate::time::FrameRate;
use crate::OutputDepth;

/// How long frame `index` of a GIF is shown, in hundredths of a second (FX-FMT-020 to 023).
///
/// Frame `n` is shown from `floor(n * 100 * den / num)` to `floor((n + 1) * 100 * den / num)`,
/// so the delays differ by one and the total never drifts, as ADR-018 did for samples. `index`
/// counts from the first frame of the file, not from the composition's frame 0.
pub fn gif_delay(index: u64, rate: FrameRate) -> u16 {
    let (num, den) = (rate.numerator() as u64, rate.denominator() as u64);
    let at = |n: u64| n * 100 * den / num;
    (at(index + 1) - at(index)).min(u16::MAX as u64) as u16
}

/// An animated file that is open and taking frames.
pub enum Film {
    Gif {
        encoder: gif::Encoder<BufWriter<File>>,
        rate: FrameRate,
        frames: u64,
    },
    Apng(png::Writer<BufWriter<File>>),
    Mp4(Mp4),
}

impl Film {
    /// Why this job cannot be a GIF, or `None`. Asked before any file is made.
    pub fn gif_refusal(width: usize, height: usize) -> Option<String> {
        (width > u16::MAX as usize || height > u16::MAX as usize).then(|| {
            format!("A GIF cannot be wider or taller than 65535 pixels, and this is {width}x{height}.")
        })
    }

    /// Why this job cannot be an animated PNG, or `None`. Asked before any file is made.
    pub fn apng_refusal(rate: FrameRate) -> Option<String> {
        (rate.numerator() > u16::MAX as u32 || rate.denominator() > u16::MAX as u32).then(|| {
            format!(
                "An animated PNG holds its frame delay as two numbers no larger than 65535, and \
                 the frame rate {} does not fit.",
                rate.label()
            )
        })
    }

    pub fn gif(path: &Path, width: usize, height: usize, rate: FrameRate) -> Result<Self, String> {
        let file = BufWriter::new(File::create(path).map_err(|e| e.to_string())?);
        let mut encoder =
            gif::Encoder::new(file, width as u16, height as u16, &[]).map_err(|e| e.to_string())?;
        encoder.set_repeat(gif::Repeat::Infinite).map_err(|e| e.to_string())?;
        Ok(Film::Gif { encoder, rate, frames: 0 })
    }

    pub fn apng(
        path: &Path,
        width: usize,
        height: usize,
        depth: OutputDepth,
        rate: FrameRate,
        frames: u32,
        tags: &[(&str, String)],
    ) -> Result<Self, String> {
        let file = BufWriter::new(File::create(path).map_err(|e| e.to_string())?);
        let mut encoder = png::Encoder::new(file, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(match depth {
            OutputDepth::Eight => png::BitDepth::Eight,
            OutputDepth::Sixteen => png::BitDepth::Sixteen,
        });
        encoder.set_animated(frames, 0).map_err(|e| e.to_string())?;
        // One frame lasts the rate's denominator over its numerator, exactly.
        encoder
            .set_frame_delay(rate.denominator() as u16, rate.numerator() as u16)
            .map_err(|e| e.to_string())?;
        for (key, value) in tags {
            encoder
                .add_itxt_chunk((*key).to_string(), value.clone())
                .map_err(|e| e.to_string())?;
        }
        Ok(Film::Apng(encoder.write_header().map_err(|e| e.to_string())?))
    }

    /// Add one frame. `srgb8` is 8-bit straight RGBA and is what a GIF is made from; `samples`
    /// is what an exported PNG frame would hold and is what an animated PNG is made from.
    pub fn push(&mut self, width: usize, height: usize, srgb8: &[u8], samples: &[u8]) -> Result<(), String> {
        match self {
            Film::Gif { encoder, rate, frames } => {
                // Alpha is on or off: half or more is on. The colour under a pixel that is off
                // is dropped, so it cannot take a place in the 256-colour palette.
                let mut rgba: Vec<u8> = srgb8
                    .chunks_exact(4)
                    .flat_map(|p| if p[3] >= 128 { [p[0], p[1], p[2], 255] } else { [0; 4] })
                    .collect();
                let mut frame = gif::Frame::from_rgba_speed(width as u16, height as u16, &mut rgba, 10);
                frame.delay = gif_delay(*frames, *rate);
                // Without this a see-through pixel would show the frame before it.
                frame.dispose = gif::DisposalMethod::Background;
                *frames += 1;
                encoder.write_frame(&frame).map_err(|e| e.to_string())
            }
            Film::Apng(writer) => writer.write_image_data(samples).map_err(|e| e.to_string()),
            Film::Mp4(mp4) => mp4.push(srgb8),
        }
    }

    pub fn finish(self) -> Result<(), String> {
        match self {
            Film::Gif { encoder, .. } => {
                use std::io::Write;
                let mut file = encoder.into_inner().map_err(|e| e.to_string())?;
                file.flush().map_err(|e| e.to_string())?;
            }
            Film::Apng(writer) => writer.finish().map_err(|e| e.to_string())?,
            Film::Mp4(mp4) => mp4.finish()?,
        }
        Ok(())
    }
}
