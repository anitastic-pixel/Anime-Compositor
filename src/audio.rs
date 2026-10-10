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
    layout(raw).map(|(wav, _)| wav)
}

/// `read_wav`, and where the data chunk's samples start in `raw`.
fn layout(raw: &[u8]) -> Result<(Wav, usize), Diagnostic> {
    if raw.len() < 12 || &raw[..4] != b"RIFF" || &raw[8..12] != b"WAVE" {
        return Err(unreadable("not a RIFF WAVE file"));
    }
    let (mut at, mut form, mut sound) = (12usize, None, None);
    while at + 8 <= raw.len() {
        let size = u32_at(raw, at + 4) as usize;
        let body = &raw[at + 8..(at + 8).saturating_add(size).min(raw.len())];
        match &raw[at..at + 4] {
            b"fmt " if form.is_none() => form = Some(body),
            b"data" if sound.is_none() => sound = Some((size, body.len(), at + 8)),
            _ => {}
        }
        at = at.saturating_add(8 + size + size % 2);
    }
    let form = match form {
        Some(f) if f.len() >= 16 => f,
        _ => return Err(unreadable("no format chunk")),
    };
    let Some((said, there, data)) = sound else {
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
            return Ok((
                Wav::Other {
                    channels,
                    sample_rate,
                },
                data,
            ))
        }
    };
    if align as u32 != channels as u32 * bits as u32 / 8 {
        return Err(unreadable("a sample size this build does not read"));
    }
    Ok((
        Wav::Read {
            encoding,
            channels,
            sample_rate,
            bits,
            samples: (there / align as usize) as u64,
            cut_short: there < said,
        },
        data,
    ))
}

/// D-420: a WAV's samples, read for an effect that listens to a sound layer.
pub struct Sound {
    raw: Vec<u8>,
    data: usize,
    encoding: Encoding,
    channels: usize,
    bits: u16,
    pub rate: u32,
    pub samples: u64,
}

impl Sound {
    /// The file's samples, or MEDIA_AUDIO_UNREADABLE for a file `read_wav` refuses or only
    /// measures (ADPCM and the like).
    pub fn read(raw: Vec<u8>) -> Result<Sound, Diagnostic> {
        match layout(&raw)? {
            (Wav::Read { encoding, channels, sample_rate, bits, samples, .. }, data) => Ok(Sound {
                raw,
                data,
                encoding,
                channels: channels as usize,
                bits,
                rate: sample_rate,
                samples,
            }),
            (Wav::Other { .. }, _) => Err(unreadable("an encoding this build plays but does not read")),
        }
    }

    /// Sample `i` of channel `ch`, -1 to 1 (8-bit `(v - 128) / 128`, 16-bit `v / 32768` and so
    /// on, floating point as written); 0 before the first sample and past the last.
    pub fn at(&self, ch: usize, i: i64) -> f64 {
        if i < 0 || i as u64 >= self.samples {
            return 0.0;
        }
        let size = self.bits as usize / 8;
        let b = &self.raw[self.data + (i as usize * self.channels + ch) * size..][..size];
        match (self.encoding, self.bits) {
            (Encoding::Pcm, 8) => (b[0] as f64 - 128.0) / 128.0,
            (Encoding::Pcm, 16) => i16::from_le_bytes([b[0], b[1]]) as f64 / 32768.0,
            (Encoding::Pcm, 24) => (i32::from_le_bytes([0, b[0], b[1], b[2]]) >> 8) as f64 / 8388608.0,
            (Encoding::Pcm, _) => i32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64 / 2147483648.0,
            (Encoding::Float, 32) => f32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f64,
            (Encoding::Float, _) => f64::from_le_bytes(b.try_into().unwrap_or([0; 8])),
        }
    }

    /// Sample `i`, its channels' mean.
    pub fn mean(&self, i: i64) -> f64 {
        (0..self.channels).map(|ch| self.at(ch, i)).sum::<f64>() / self.channels as f64
    }

    /// D-420, Audio Spectrum's levels: for each of `freqs`, 2 |sum x_k w_k e^(-2 pi i f k / rate)|
    /// / sum w_k over the `count` samples from `start`, w the Hann window
    /// 1/2 - 1/2 cos(2 pi (k + 1/2) / count), x the channels' mean times `gain`; a sine of
    /// amplitude A at f reads A. With `averaging`, the mean over windows starting half a window
    /// before and after as well.
    // ponytail: one sum per band, count x bands sines a frame. A transform would be quicker for
    // thousands of bands over long windows, but changes the rule.
    pub fn levels(&self, start: i64, count: usize, freqs: &[f64], gain: f64, averaging: bool) -> Vec<f64> {
        use rayon::prelude::*;
        let w: Vec<f64> = (0..count)
            .map(|k| 0.5 - 0.5 * (2.0 * std::f64::consts::PI * (k as f64 + 0.5) / count as f64).cos())
            .collect();
        let sw: f64 = w.iter().sum();
        let half = (count / 2) as i64;
        let starts = if averaging { vec![start - half, start, start + half] } else { vec![start] };
        let windows: Vec<Vec<f64>> = starts
            .iter()
            .map(|s| (0..count).map(|k| self.mean(s + k as i64) * gain * w[k]).collect())
            .collect();
        let rate = self.rate as f64;
        freqs
            .par_iter()
            .map(|f| {
                let total: f64 = windows
                    .iter()
                    .map(|x| {
                        let (mut re, mut im) = (0.0, 0.0);
                        for (k, v) in x.iter().enumerate() {
                            let a = 2.0 * std::f64::consts::PI * f * k as f64 / rate;
                            re += v * a.cos();
                            im -= v * a.sin();
                        }
                        2.0 * re.hypot(im) / sw
                    })
                    .sum();
                total / windows.len() as f64
            })
            .collect()
    }
}

impl Sound {
    /// D-421, Audio Waveform's: for each of `shown` displayed samples, the least and greatest of
    /// its share of the `count` samples from `start` (samples floor(j count / shown) up to
    /// floor((j + 1) count / shown), or the first alone), each the channels' mean or channel
    /// `channel` (the last when the file has fewer) times `gain`, held to -1..1; least and
    /// greatest in turn.
    pub fn spans(&self, start: i64, count: usize, shown: usize, gain: f64, channel: Option<usize>) -> Vec<f64> {
        let x = |i: i64| {
            let v = match channel {
                None => self.mean(i),
                Some(ch) => self.at(ch.min(self.channels - 1), i),
            };
            (v * gain).clamp(-1.0, 1.0)
        };
        (0..shown)
            .flat_map(|j| {
                let (a, b) = (j * count / shown, (j + 1) * count / shown);
                let (lo, hi) = (a..b.max(a + 1)).map(|k| x(start + k as i64)).fold((f64::INFINITY, f64::NEG_INFINITY), |(lo, hi), v| (lo.min(v), hi.max(v)));
                [lo, hi]
            })
            .collect()
    }
}

/// D-420: the sound at `path`, read once and kept while the file's length and time stay the same.
// ponytail: every sound read stays for the session. Bound it if long recordings pile up.
pub fn load(path: &std::path::Path) -> Result<std::sync::Arc<Sound>, Diagnostic> {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex, OnceLock};
    type Kept = HashMap<std::path::PathBuf, (u64, Option<std::time::SystemTime>, Arc<Sound>)>;
    static KEPT: OnceLock<Mutex<Kept>> = OnceLock::new();
    let meta = std::fs::metadata(path).map_err(|e| unreadable(&format!("the file could not be opened ({e})")))?;
    let stamp = (meta.len(), meta.modified().ok());
    let kept = KEPT.get_or_init(Default::default);
    if let Some((len, time, sound)) = kept.lock().unwrap_or_else(|e| e.into_inner()).get(path) {
        if (*len, *time) == stamp {
            return Ok(sound.clone());
        }
    }
    let raw = std::fs::read(path).map_err(|e| unreadable(&format!("the file could not be read ({e})")))?;
    let sound = Arc::new(Sound::read(raw)?);
    kept.lock()
        .unwrap_or_else(|e| e.into_inner())
        .insert(path.to_path_buf(), (stamp.0, stamp.1, sound.clone()));
    Ok(sound)
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
