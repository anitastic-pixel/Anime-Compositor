//! D-71 and ADR-018: reference audio. Whole frames become whole samples, and a WAV header is
//! read by hand. Nothing here decodes or plays sound; the window's own decoder does that.
//!
//! Pinned by FX-AUD-001 to 010, whose numbers come from `tools/audio_reference.py`.

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::time::LayerTiming;

/// ADR-018 decision 2: frame `n` of a file begins at `floor(n * rate * den / num)`.
pub fn first_sample(n: u64, rate: u32, num: u32, den: u32) -> u64 {
    (n as u128 * rate as u128 * den as u128 / num as u128) as u64
}

/// How many frames a file of `samples` covers, the last one counted when it is partly filled.
pub fn length_frames(samples: u64, rate: u32, num: u32, den: u32) -> u64 {
    (samples as u128 * num as u128).div_ceil(rate as u128 * den as u128) as u64
}

/// The samples of the file heard on a composition frame, first and one past the last, or `None`
/// for silence: outside the layer, before the file's first sample, or after its last.
pub fn heard(
    timing: &LayerTiming,
    samples: u64,
    frame: i32,
    rate: u32,
    num: u32,
    den: u32,
) -> Option<(u64, u64)> {
    let n = u64::try_from(timing.local_frame(frame)?).ok()?;
    let a = first_sample(n, rate, num, den);
    let b = first_sample(n + 1, rate, num, den).min(samples);
    (a < b).then_some((a, b))
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Encoding {
    Pcm,
    Float,
}

/// What a WAV header says.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Wav {
    Read {
        encoding: Encoding,
        channels: u16,
        sample_rate: u32,
        bits: u16,
        /// Samples per channel that are really in the file.
        samples: u64,
        /// `MEDIA_AUDIO_CUT_SHORT`: the data chunk promised more than the file holds.
        cut_short: bool,
    },
    /// A WAV in an encoding the core does not measure, ADPCM for one. D-71 treats it as the
    /// other formats: the window plays it and reports its length.
    Other { channels: u16, sample_rate: u32 },
}

fn unreadable(why: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaAudioUnreadable,
        Severity::Warning,
        "This sound file could not be read, so its layer is silent.",
        format!("D-71: {why}. The layer and its file reference are kept."),
    )
    .with_remediation("Relink the layer to a WAV file, or convert this one to WAV.")
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// D-71's WAV rules, FX-AUD-010. `raw` is the whole file.
// ponytail: the whole file is read to measure it. Read the chunk headers with seeks if a long
// recording makes import slow.
pub fn read_wav(raw: &[u8]) -> Result<Wav, Diagnostic> {
    if raw.len() < 12 || &raw[..4] != b"RIFF" || &raw[8..12] != b"WAVE" {
        return Err(unreadable("not a RIFF WAVE file"));
    }
    let (mut at, mut form, mut sound) = (12usize, None, None);
    while at + 8 <= raw.len() {
        let size = u32_at(raw, at + 4) as usize;
        let body = &raw[at + 8..(at + 8).saturating_add(size).min(raw.len())];
        match &raw[at..at + 4] {
            b"fmt " if form.is_none() => form = Some(body),
            b"data" if sound.is_none() => sound = Some((size, body.len())),
            _ => {}
        }
        at = at.saturating_add(8 + size + size % 2);
    }
    let form = match form {
        Some(f) if f.len() >= 16 => f,
        _ => return Err(unreadable("no format chunk")),
    };
    let Some((said, there)) = sound else {
        return Err(unreadable("no data chunk"));
    };
    let (mut tag, channels, sample_rate) = (u16_at(form, 0), u16_at(form, 2), u32_at(form, 4));
    let (align, bits) = (u16_at(form, 12), u16_at(form, 14));
    if tag == 0xFFFE && form.len() >= 26 {
        tag = u16_at(form, 24);
    }
    if sample_rate == 0 || channels == 0 {
        return Err(unreadable("no sample rate or no channels"));
    }
    let encoding = match (tag, bits) {
        (1, 8 | 16 | 24 | 32) => Encoding::Pcm,
        (3, 32 | 64) => Encoding::Float,
        (1 | 3, _) => return Err(unreadable("a sample size this build does not read")),
        _ => {
            return Ok(Wav::Other {
                channels,
                sample_rate,
            })
        }
    };
    if align as u32 != channels as u32 * bits as u32 / 8 {
        return Err(unreadable("a sample size this build does not read"));
    }
    Ok(Wav::Read {
        encoding,
        channels,
        sample_rate,
        bits,
        samples: (there / align as usize) as u64,
        cut_short: there < said,
    })
}

/// The warning a file cut short carries. It is still read as far as it goes.
pub fn cut_short(name: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaAudioCutShort,
        Severity::Warning,
        format!("The sound file \"{name}\" ends before its header says it does."),
        "D-71: the data chunk promises more samples than the file holds. What is there plays.",
    )
}
