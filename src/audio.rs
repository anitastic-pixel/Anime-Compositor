//! D-71 and ADR-018: reference audio. Whole frames become whole samples, and a WAV header is
//! read by hand. D-639 reads FLAC, MP3, Ogg Vorbis and AAC as well, and turns every sound into
//! the working sound (48,000 a second, two channels). D-640 mixes a composition's sound from
//! it; the window plays what it is handed.
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
    /// D-640: the working sound, worked out the first time it is mixed and kept with the sound.
    kept: std::sync::OnceLock<std::sync::Arc<[Vec<f64>; 2]>>,
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
                kept: Default::default(),
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

/// D-420: the sound at `path` (D-639: in any format `read_file` reads), read once and kept while the file's length and time stay the same.
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
    let sound = Arc::new(read_file(path)?);
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

/// D-639: what the core works in. Every sound is turned into this on reading.
pub const WORKING_RATE: u32 = 48000;

impl Sound {
    /// D-639: decoded samples, interleaved, kept as 32-bit floats so `at` reads them as a
    /// floating-point WAV's.
    fn floats(rate: u32, channels: usize, x: &[f32]) -> Sound {
        Sound {
            raw: x.iter().flat_map(|v| v.to_le_bytes()).collect(),
            data: 0,
            encoding: Encoding::Float,
            channels,
            bits: 32,
            rate,
            samples: (x.len() / channels) as u64,
            kept: Default::default(),
        }
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    /// D-639, document 21: the working sound, 48,000 samples a second in two channels.
    ///
    /// Channels first: one plays in both; two stay; of more, 0 is left and 1 right, 2 (the
    /// centre) goes to both at 0.7071, 3 (low frequencies) is left out, and 4 on go left (even)
    /// and right (odd) at 0.7071. Then the rate: output j sits at t = j rate / 48000 input
    /// samples and is the sum over k of x[k] fc sinc(fc (t - k)) w((t - k) / W), with
    /// fc = min(1, 48000 / rate), W = 32 / fc and the Blackman window
    /// w(u) = 0.42 + 0.5 cos(pi u) + 0.08 cos(2 pi u) for |u| < 1 (0 outside); samples outside the
    /// file are 0. There are ceil(n 48000 / rate) outputs. At 48,000 nothing changes.
    pub fn working(&self) -> [Vec<f64>; 2] {
        use rayon::prelude::*;
        let h = std::f64::consts::FRAC_1_SQRT_2;
        let n = self.samples as usize;
        let pair = |i: i64| match self.channels {
            1 => (self.at(0, i), self.at(0, i)),
            2 => (self.at(0, i), self.at(1, i)),
            c => {
                let centre = h * self.at(2, i);
                let (mut l, mut r) = (self.at(0, i) + centre, self.at(1, i) + centre);
                for ch in 4..c {
                    if ch % 2 == 0 {
                        l += h * self.at(ch, i);
                    } else {
                        r += h * self.at(ch, i);
                    }
                }
                (l, r)
            }
        };
        let (l, r): (Vec<f64>, Vec<f64>) = (0..n as i64).map(pair).unzip();
        if self.rate == WORKING_RATE {
            return [l, r];
        }
        let (rate, work) = (self.rate as u64, WORKING_RATE as u64);
        let out = (n as u64 * work).div_ceil(rate) as usize;
        let fc = (work as f64 / rate as f64).min(1.0);
        let w = 32.0 / fc;
        let reach = w as i64 + 1;
        let taps = |j: u64| -> Vec<f64> {
            let frac = (j * rate % work) as f64 / work as f64;
            (-reach..=reach)
                .map(|m| {
                    let d = frac - m as f64;
                    let u = d / w;
                    if u.abs() >= 1.0 {
                        return 0.0;
                    }
                    let x = std::f64::consts::PI * fc * d;
                    let sinc = if x == 0.0 { 1.0 } else { x.sin() / x };
                    let pu = std::f64::consts::PI * u;
                    fc * sinc * (0.42 + 0.5 * pu.cos() + 0.08 * (2.0 * pu).cos())
                })
                .collect()
        };
        // The weights repeat every `period` outputs; worked out once when that is short.
        let period = work / gcd(work, rate);
        let table: Vec<Vec<f64>> = if period <= 4096 { (0..period).map(taps).collect() } else { Vec::new() };
        let one = |x: &[f64], j: usize| -> f64 {
            let k0 = (j as u64 * rate / work) as i64;
            let own;
            let g = if table.is_empty() {
                own = taps(j as u64);
                &own
            } else {
                &table[(j as u64 % period) as usize]
            };
            g.iter()
                .zip(-reach..=reach)
                .map(|(g, m)| {
                    let k = k0 + m;
                    if k < 0 || k as usize >= n { 0.0 } else { g * x[k as usize] }
                })
                .sum()
        };
        let resample = |x: &[f64]| (0..out).into_par_iter().map(|j| one(x, j)).collect();
        [resample(&l), resample(&r)]
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

/// D-639: the warning an Opus file carries.
pub fn preview_only(name: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaAudioPreviewOnly,
        Severity::Warning,
        format!("The sound file \"{name}\" plays in the window, but exports and sound effects leave it out."),
        "D-639: this program reads WAV, FLAC, MP3, Ogg Vorbis and AAC. Opus is played only by the \
         window's own decoder. The layer and its file reference are kept.",
    )
    .with_remediation("Convert the file to WAV or FLAC to hear it in exports and sound effects.")
}

/// D-638 1 (a), D-639: a sound file, by the road its name picks. WAV as D-71 reads it; FLAC, MP3
/// and Ogg Vorbis by `symphonia`; AAC (`.m4a`, `.aac`, and the first sound track of `.mp4` and
/// `.mov`) by the decoder Windows carries, or elsewhere an `ffmpeg` the user installed; Opus by
/// neither, MEDIA_AUDIO_PREVIEW_ONLY. Any other name is read as a WAV, as before.
pub fn read_file(path: &std::path::Path) -> Result<Sound, Diagnostic> {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_ascii_lowercase();
    let name = path.file_name().map_or_else(|| path.display().to_string(), |n| n.to_string_lossy().into_owned());
    let raw = || std::fs::read(path).map_err(|e| unreadable(&format!("the file could not be read ({e})")));
    match ext.as_str() {
        "opus" => Err(preview_only(&name)),
        "flac" | "mp3" | "ogg" => by_symphonia(raw()?, &ext, &name),
        "m4a" | "aac" | "mp4" | "mov" => aac(path),
        _ => Sound::read(raw()?),
    }
}

fn by_symphonia(raw: Vec<u8>, ext: &str, name: &str) -> Result<Sound, Diagnostic> {
    use symphonia::core::codecs::audio::{well_known::CODEC_ID_OPUS, AudioDecoderOptions};
    use symphonia::core::formats::{probe::Hint, FormatOptions, TrackType};
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::meta::MetadataOptions;
    let bad = |e: &dyn std::fmt::Display| unreadable(&format!("the {ext} decoder answered: {e}"));
    let source = MediaSourceStream::new(Box::new(std::io::Cursor::new(raw)), Default::default());
    let mut hint = Hint::new();
    hint.with_extension(ext);
    let mut format = symphonia::default::get_probe()
        .probe(&hint, source, FormatOptions::default(), MetadataOptions::default())
        .map_err(|e| bad(&e))?;
    let track = format.default_track(TrackType::Audio).ok_or_else(|| unreadable("no sound track"))?;
    let params = track.codec_params.as_ref().and_then(|p| p.audio()).ok_or_else(|| unreadable("no sound track"))?.clone();
    if params.codec == CODEC_ID_OPUS {
        return Err(preview_only(name));
    }
    let id = track.id;
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&params, &AudioDecoderOptions::default())
        .map_err(|e| bad(&e))?;
    let (mut x, mut part, mut spec) = (Vec::new(), Vec::new(), None);
    while let Some(packet) = format.next_packet().map_err(|e| bad(&e))? {
        if packet.track_id != id {
            continue;
        }
        let got = decoder.decode(&packet).map_err(|e| bad(&e))?;
        spec.get_or_insert((got.spec().rate(), got.spec().channels().count()));
        part.resize(got.samples_interleaved(), 0f32);
        got.copy_to_slice_interleaved(&mut part);
        x.extend_from_slice(&part);
    }
    match spec {
        Some((rate, channels)) if rate > 0 && channels > 0 => Ok(Sound::floats(rate, channels, &x)),
        _ => Err(unreadable("no sound in the file")),
    }
}

/// D-639: where an MP4 or MOV says its sound starts and how long it lasts, from the first sound
/// track's edit list: (silence first, samples to skip, samples to keep), at the track's own
/// rate, and that rate. The encoder's start delay is told this way, and Windows' decoder does not skip it.
/// `None` for a file with no edit list, or that is not an MP4 at all (a raw `.aac`).
// ponytail: the first edit only, after any empty ones. The iTunes gapless note some encoders
// write instead is not read; add it if such a file is met.
#[cfg_attr(not(windows), allow(dead_code))]
fn edit_list(b: &[u8]) -> Option<(u64, u64, Option<u64>, u64)> {
    let u32_be = |at: usize| Some(u32::from_be_bytes(b.get(at..at + 4)?.try_into().ok()?) as u64);
    let u64_be = |at: usize| Some(u64::from_be_bytes(b.get(at..at + 8)?.try_into().ok()?));
    // The boxes inside `a..e`, as (four letters, start of the body, end).
    let boxes = |mut a: usize, e: usize| {
        let mut out = Vec::new();
        while a + 8 <= e {
            let size = match u32_be(a)? {
                1 => u64_be(a + 8)? as usize,
                0 => e - a,
                n => n as usize,
            };
            let head = if u32_be(a)? == 1 { 16 } else { 8 };
            if size < head || a + size > e {
                break;
            }
            out.push((b.get(a + 4..a + 8)?, a + head, a + size));
            a += size;
        }
        Some(out)
    };
    let find = |list: &[(&[u8], usize, usize)], name: &[u8]| list.iter().find(|x| x.0 == name).map(|x| (x.1, x.2));
    // A full box's timescale: after version and flags, and two dates of 4 or 8 bytes.
    let timescale = |at: usize| u32_be(at + if b.get(at)? == &1 { 20 } else { 12 });
    let (moov, end) = find(&boxes(0, b.len())?, b"moov")?;
    let inside = boxes(moov, end)?;
    let movie = timescale(find(&inside, b"mvhd")?.0)?;
    for (_, a, e) in inside.iter().filter(|x| x.0 == b"trak") {
        let trak = boxes(*a, *e)?;
        let (ma, me) = find(&trak, b"mdia")?;
        let mdia = boxes(ma, me)?;
        if b.get(find(&mdia, b"hdlr")?.0 + 8..)?.get(..4)? != b"soun" {
            continue;
        }
        let rate = timescale(find(&mdia, b"mdhd")?.0)?;
        let Some((ea, ee)) = find(&trak, b"edts") else { return None };
        let (la, _) = find(&boxes(ea, ee)?, b"elst")?;
        let wide = b.get(la)? == &1;
        let (step, count) = (if wide { 20 } else { 12 }, u32_be(la + 4)?);
        let mut silence = 0;
        for i in 0..count as usize {
            let at = la + 8 + i * step;
            let (length, start) = if wide {
                (u64_be(at)?, u64_be(at + 8)? as i64)
            } else {
                (u32_be(at)?, u32_be(at + 4)? as u32 as i32 as i64)
            };
            let length = (length as u128 * rate as u128 / movie.max(1) as u128) as u64;
            if start < 0 {
                silence += length;
                continue;
            }
            return Some((silence, start as u64, (length > 0).then_some(length), rate));
        }
        return None;
    }
    None
}

/// D-639: AAC through Media Foundation's source reader, asked for 32-bit floats. Each piece is
/// placed by its time stamp, then the edit list (`edit_list`) says where the sound starts and
/// ends, so the encoder's start delay is skipped.
#[cfg(windows)]
fn aac(path: &std::path::Path) -> Result<Sound, Diagnostic> {
    use windows::core::HSTRING;
    use windows::Win32::Media::MediaFoundation::*;
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};
    let said = |e: windows::core::Error| unreadable(&format!("Windows' sound decoder answered: {}", e.message()));
    unsafe {
        // Already initialised on this thread is an answer too, and not a failure.
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(said)?;
        let read = || -> windows::core::Result<(u32, usize, Vec<f32>)> {
            let reader = MFCreateSourceReaderFromURL(&HSTRING::from(path.as_os_str()), None)?;
            let first = MF_SOURCE_READER_FIRST_AUDIO_STREAM.0 as u32;
            reader.SetStreamSelection(MF_SOURCE_READER_ALL_STREAMS.0 as u32, false)?;
            reader.SetStreamSelection(first, true)?;
            let want = MFCreateMediaType()?;
            want.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Audio)?;
            want.SetGUID(&MF_MT_SUBTYPE, &MFAudioFormat_Float)?;
            reader.SetCurrentMediaType(first, None, &want)?;
            let got = reader.GetCurrentMediaType(first)?;
            let rate = got.GetUINT32(&MF_MT_AUDIO_SAMPLES_PER_SECOND)?;
            let channels = (got.GetUINT32(&MF_MT_AUDIO_NUM_CHANNELS)? as usize).max(1);
            let mut x: Vec<f32> = Vec::new();
            loop {
                let (mut flags, mut time, mut sample) = (0u32, 0i64, None);
                reader.ReadSample(first, 0, None, Some(&mut flags), Some(&mut time), Some(&mut sample))?;
                if let Some(sample) = sample {
                    let buffer = sample.ConvertToContiguousBuffer()?;
                    let (mut data, mut len) = (std::ptr::null_mut(), 0u32);
                    buffer.Lock(&mut data, None, Some(&mut len))?;
                    let piece = std::slice::from_raw_parts(data as *const f32, len as usize / 4);
                    // Where the piece starts, to the nearest sample: before 0 is the start delay.
                    let at = (time as i128 * rate as i128 + 5_000_000).div_euclid(10_000_000) as i64;
                    let skip = ((-at).max(0) as usize * channels).min(piece.len());
                    let start = at.max(0) as usize * channels;
                    if start > x.len() {
                        x.resize(start, 0.0);
                    }
                    x.truncate(start);
                    x.extend_from_slice(&piece[skip..]);
                    buffer.Unlock()?;
                }
                if flags & MF_SOURCE_READERF_ENDOFSTREAM.0 as u32 != 0 {
                    break;
                }
            }
            Ok((rate, channels, x))
        };
        let result = read();
        let _ = MFShutdown();
        let (rate, channels, mut x) = result.map_err(said)?;
        if let Some((silence, skip, keep, track)) = std::fs::read(path).ok().as_deref().and_then(edit_list) {
            let at = |n: u64| (n as u128 * rate as u128 / track.max(1) as u128) as usize * channels;
            x.drain(..at(skip).min(x.len()));
            if let Some(keep) = keep {
                x.truncate(at(keep));
            }
            x.splice(0..0, std::iter::repeat_n(0.0, at(silence)));
        }
        if rate == 0 || x.is_empty() {
            return Err(unreadable("no sound in the file"));
        }
        Ok(Sound::floats(rate, channels, &x))
    }
}

/// D-30's second road: an `ffmpeg` the user installed decodes the first sound track to a
/// floating-point WAV, which D-71's reader then reads.
// ponytail: not run by any test on the Windows build; the same road B-21e's export takes.
#[cfg(not(windows))]
fn aac(path: &std::path::Path) -> Result<Sound, Diagnostic> {
    let out = std::process::Command::new("ffmpeg")
        .args(["-v", "error", "-i"])
        .arg(path)
        .args(["-map", "0:a:0", "-c:a", "pcm_f32le", "-f", "wav", "-"])
        .output()
        .map_err(|e| unreadable(&format!("AAC is read through ffmpeg, which could not be started ({e})")))?;
    if !out.status.success() {
        return Err(unreadable("ffmpeg could not read the sound track"));
    }
    Sound::read(out.stdout)
}

impl Sound {
    /// D-640: `working`, worked out once and kept with the sound, so each stretch mixed reads
    /// it rather than working it out again.
    // ponytail: kept for as long as the sound is (the session, by `load`); 64-bit, so three
    // minutes is about 140 MB. Keep 32-bit or drop unused sounds if long recordings pile up.
    pub fn working_kept(&self) -> std::sync::Arc<[Vec<f64>; 2]> {
        self.kept.get_or_init(|| std::sync::Arc::new(self.working())).clone()
    }
}

/// D-640, document 21: the sound of composition `comp` for its frames `from .. from + frames`,
/// samples `first_sample(from) .. first_sample(from + frames)` at 48,000 a second (ADR-018), as
/// two channels. Each audio layer that is on is heard on the frames its span holds: frame f
/// reads its working sound from local frame n's first sample on, f's samples in turn, silence
/// before the sound's first sample and past its last, times its level; the layers are added
/// and nothing is clipped. A sound left out is told with its layer's id: an Opus one
/// (`MEDIA_AUDIO_PREVIEW_ONLY`, which the window plays itself) or one that cannot be read.
// ponytail: the composition's own audio layers only. Sound through composition layers, keyed
// level and pan are S-7 of D-638. A layer's stretch and time remap are not applied to its
// sound, as D-71 never did.
/// D-641: how many values of `sound` are past full level (above 1 or below -1), which an export
/// writes at full level.
pub fn clipped(sound: &[Vec<f64>; 2]) -> usize {
    sound.iter().flatten().filter(|v| v.abs() > 1.0).count()
}

/// D-641: `sound` as a WAV, 48,000 a second, two channels, 24 bits: each value kept between -1
/// and 1, times 8,388,607, rounded half away from nought.
pub fn wav24(sound: &[Vec<f64>; 2]) -> Vec<u8> {
    let n = sound[0].len();
    let data = (n * 6) as u32;
    let mut out = Vec::with_capacity(44 + n * 6);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    for v in [16u32, 1 | 2 << 16, WORKING_RATE, WORKING_RATE * 6, 6 | 24 << 16] {
        out.extend_from_slice(&v.to_le_bytes());
    }
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data.to_le_bytes());
    for i in 0..n {
        for c in sound {
            let v = (c[i].clamp(-1.0, 1.0) * 8_388_607.0).round() as i32;
            out.extend_from_slice(&v.to_le_bytes()[..3]);
        }
    }
    out
}

pub fn mix(
    project: &crate::model::Project,
    root: &std::path::Path,
    comp: &crate::model::Composition,
    from: i32,
    frames: u32,
) -> ([Vec<f64>; 2], Vec<(String, Diagnostic)>) {
    mix_in(project, root, comp, from as i64, frames as i64, &mut Vec::new())
}

/// D-645: whether the composition has anything to hear: an audio layer switched on, here or
/// inside a composition layer switched on.
pub fn has_sound(project: &crate::model::Project, comp: &crate::model::Composition) -> bool {
    fn inside(project: &crate::model::Project, comp: &crate::model::Composition, above: &mut Vec<crate::model::Id>) -> bool {
        if above.contains(&comp.id) {
            return false;
        }
        above.push(comp.id.clone());
        let heard = comp.layers_in_order().filter(|l| l.enabled).any(|l| {
            l.kind == crate::model::LayerKind::Audio
                || l.composition_id.as_ref().and_then(|id| project.composition(id)).is_some_and(|c| inside(project, c, above))
        });
        above.pop();
        heard
    }
    inside(project, comp, &mut Vec::new())
}

/// D-645: a layer's level in dB and pan at composition frame `f`, each held to its range.
fn level_at(layer: &crate::model::Layer, f: i64) -> (f64, f64) {
    let read = |p: &Option<crate::model::Property>, plain: f64| {
        p.as_ref().and_then(|p| p.value_at(f as i32).as_scalar()).unwrap_or(plain)
    };
    (read(&layer.gain_keys, layer.gain_db).clamp(-96.0, 12.0), read(&layer.pan, 0.0).clamp(-100.0, 100.0))
}

/// D-645: the two channels' gains a share `t` of the way from `a` to `b` (each a level in dB
/// and a pan): the level moves in a straight line in dB, the pan by the balance law, which
/// keeps the centre at unity.
fn channel_gains(a: (f64, f64), b: (f64, f64), t: f64) -> [f64; 2] {
    let (db, p) = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
    let g = 10f64.powf(db / 20.0);
    [g * (1.0 - p / 100.0).min(1.0), g * (1.0 + p / 100.0).min(1.0)]
}

fn mix_in(
    project: &crate::model::Project,
    root: &std::path::Path,
    comp: &crate::model::Composition,
    from: i64,
    frames: i64,
    above: &mut Vec<crate::model::Id>,
) -> ([Vec<f64>; 2], Vec<(String, Diagnostic)>) {
    let (num, den) = (comp.frame_rate.numerator() as i128, comp.frame_rate.denominator() as i128);
    let first = |f: i64| (f as i128 * WORKING_RATE as i128 * den).div_euclid(num) as i64;
    let to = from + frames;
    let s0 = first(from);
    let n = (first(to) - s0) as usize;
    let mut out = [vec![0.0; n], vec![0.0; n]];
    let mut left_out = Vec::new();
    for layer in comp.layers_in_order() {
        if !layer.enabled {
            continue;
        }
        if let Some(inner_id) = &layer.composition_id {
            // D-645: a composition layer is heard. A missing composition was said on opening
            // (COMPOSITION_REFERENCE_MISSING); a cycle cannot open, and is silent here if an edit
            // makes one.
            let Some(inner) = project.composition(inner_id) else { continue };
            if above.contains(inner_id) || &comp.id == inner_id {
                continue;
            }
            let (ni, di) = (inner.frame_rate.numerator() as i128, inner.frame_rate.denominator() as i128);
            let first_in = |f: i64| (f as i128 * WORKING_RATE as i128 * di).div_euclid(ni) as i64;
            let (start, end) = (inner.start_frame as i64, inner.start_frame as i64 + inner.duration_frames as i64);
            // Composition frame f plays inner frame g = floor(source time) from its first sample.
            let reads: Vec<(i64, i64)> = (from.max(layer.in_frame as i64)..to.min(layer.out_frame as i64))
                .filter_map(|f| Some((f, layer.source_time(f as i32)?.floor() as i64)))
                .filter(|&(_, g)| (start..end).contains(&g))
                .collect();
            let (Some(lo), Some(need)) = (
                reads.iter().map(|r| r.1).min(),
                reads.iter().map(|&(f, g)| first_in(g) + first(f + 1) - first(f)).max(),
            ) else {
                continue;
            };
            let mut hi = lo + 1;
            while hi < end && first_in(hi) < need {
                hi += 1;
            }
            above.push(comp.id.clone());
            let (heard, said) = mix_in(project, root, inner, lo, hi - lo, above);
            above.pop();
            left_out.extend(said);
            let base = first_in(lo);
            for (f, g) in reads {
                let (a, src) = (first(f), first_in(g) - base);
                for s in a..first(f + 1) {
                    let i = src + (s - a);
                    if (i as usize) < heard[0].len() {
                        for c in 0..2 {
                            out[c][(s - s0) as usize] += heard[c][i as usize];
                        }
                    }
                }
            }
            continue;
        }
        if layer.kind != crate::model::LayerKind::Audio {
            continue;
        }
        let Some(path) = project.assets.iter().find(|a| a.id == layer.asset_id).and_then(|a| a.path.as_ref()) else {
            continue;
        };
        let sound = match load(&root.join(path)) {
            Ok(sound) => sound,
            Err(d) => {
                left_out.push((layer.id.as_str().to_string(), d));
                continue;
            }
        };
        let work = sound.working_kept();
        let timing = layer.timing();
        // D-645: the level and pan, after the effects, moving straight across each frame.
        let plain = layer.gain_keys.is_none() && layer.pan.is_none();
        let fixed = channel_gains(level_at(layer, 0), level_at(layer, 0), 0.0);
        let gains = |f: i64| {
            let (a, b) = if plain { ((0.0, 0.0), (0.0, 0.0)) } else { (level_at(layer, f), level_at(layer, f + 1)) };
            let (s_a, s_b) = (first(f), first(f + 1));
            move |s: i64| if plain { fixed } else { channel_gains(a, b, (s - s_a) as f64 / (s_b - s_a) as f64) }
        };
        if !layer.effects.is_empty() {
            // D-642: a sound effect needs what came before, so the layer is made from its
            // first sample, run through its effects in order, then given its level.
            // ponytail: each call starts again from the layer's first sample, so the window's
            // stretches cost more the later they are; a kept state per layer if that is felt.
            let (lo, hi) = (layer.in_frame as i64, to.min(layer.out_frame as i64));
            if hi <= lo {
                continue;
            }
            let start = first(lo);
            let len = (first(hi) - start) as usize;
            let mut y = [vec![0.0; len], vec![0.0; len]];
            for f in lo..hi {
                let Some(local) = timing.local_frame(f as i32) else { continue };
                let (a, b, src) = (first(f), first(f + 1), first(local as i64));
                for s in a..b {
                    let i = src + (s - a);
                    if i >= 0 && (i as usize) < work[0].len() {
                        for c in 0..2 {
                            y[c][(s - start) as usize] = work[c][i as usize];
                        }
                    }
                }
            }
            for fx in layer.effects.iter().filter(|e| e.enabled) {
                let id = layer.id.as_str().to_string();
                let name = fx.effect.name();
                if let crate::effects::Effect::Unsupported { type_id } = &fx.effect {
                    left_out.push((id, Diagnostic::new(
                        DiagnosticId::EffectUnsupported,
                        Severity::Warning,
                        format!("The layer \"{}\" uses the effect \"{type_id}\", which this build does not have.", layer.name),
                        "The effect is kept in the project and its sound passes through unchanged.",
                    )));
                } else if let Some(why) = fx.fault() {
                    left_out.push((id, Diagnostic::new(
                        DiagnosticId::EffectParameterInvalid,
                        Severity::Warning,
                        format!("The layer \"{}\" has a {name} whose settings this build cannot use.", layer.name),
                        format!("{why} The effect is kept and its sound passes through unchanged."),
                    )));
                } else if matches!(fx.effect, crate::effects::Effect::Compressor { .. }) {
                    compressor(&mut y, lo, hi, &first, fx);
                } else if matches!(fx.effect, crate::effects::Effect::Distortion { .. }) {
                    distortion(&mut y, lo, hi, &first, fx);
                } else {
                    noise_gate(&mut y, lo, hi, &first, fx);
                }
            }
            for f in from.max(lo)..hi {
                let gain = gains(f);
                for s in first(f)..first(f + 1) {
                    let g = gain(s);
                    for c in 0..2 {
                        out[c][(s - s0) as usize] += g[c] * y[c][(s - start) as usize];
                    }
                }
            }
            continue;
        }
        for f in from.max(layer.in_frame as i64)..to.min(layer.out_frame as i64) {
            let Some(local) = timing.local_frame(f as i32) else { continue };
            let (a, b, start) = (first(f), first(f + 1), first(local as i64));
            let gain = gains(f);
            for s in a..b {
                let i = start + (s - a);
                if i < 0 || i as usize >= work[0].len() {
                    continue;
                }
                let g = gain(s);
                for c in 0..2 {
                    out[c][(s - s0) as usize] += g[c] * work[c][i as usize];
                }
            }
        }
    }
    (out, left_out)
}

/// D-642, document 21: Noise Gate over `y`, the layer's frames `lo` to `hi` from its first
/// sample. A gain from 0 (shut) rises by 1/attack while the louder channel is at or above the
/// threshold or the hold is counting down, and falls by 1/release after; the times are whole
/// samples. A keyed setting is read straight between its values on this frame and the next.
fn noise_gate(
    y: &mut [Vec<f64>; 2],
    lo: i64,
    hi: i64,
    first: &dyn Fn(i64) -> i64,
    fx: &crate::effects::EffectInstance,
) {
    let read = |f: i64| match fx.at(f as i32).effect {
        crate::effects::Effect::NoiseGate { threshold, attack, hold, release } => [threshold, attack, hold, release],
        _ => unreachable!("called for Noise Gate only"),
    };
    let whole = |ms: f64| (ms * 48.0 + 0.5).floor() as i64;
    let start = first(lo);
    let (mut g, mut held) = (0.0f64, 0i64);
    for f in lo..hi {
        let (s0, s1) = (first(f), first(f + 1));
        let (a, b) = (read(f), read(f + 1));
        for s in s0..s1 {
            let v = |k: usize| a[k] + (b[k] - a[k]) * (s - s0) as f64 / (s1 - s0) as f64;
            let (t, att, h, r) = (10f64.powf(v(0) / 20.0), whole(v(1)), whole(v(2)), whole(v(3)));
            let i = (s - start) as usize;
            let rise = |g: f64| if att == 0 { 1.0 } else { g + 1.0 / att as f64 };
            if y[0][i].abs().max(y[1][i].abs()) >= t {
                held = h;
                g = rise(g);
            } else if held > 0 {
                held -= 1;
                g = rise(g);
            } else {
                g -= 1.0 / r as f64;
            }
            if g >= 1.0 - 1e-12 {
                g = 1.0;
            }
            if g <= 1e-12 {
                g = 0.0;
            }
            y[0][i] *= g;
            y[1][i] *= g;
        }
    }
}

/// D-643, document 21: Compressor over `y`, the layer's frames `lo` to `hi` from its first
/// sample. A gain change g in dB, 0 at the first sample, eases toward the knee curve's change for
/// the louder channel's level: over attack when it must fall, over release when it may rise (a
/// tenth of release while Auto Release is on and g is more than 3 dB below). Then Makeup, and an
/// instant Output Limit on the louder channel that keeps the balance.
fn compressor(
    y: &mut [Vec<f64>; 2],
    lo: i64,
    hi: i64,
    first: &dyn Fn(i64) -> i64,
    fx: &crate::effects::EffectInstance,
) {
    let read = |f: i64| match fx.at(f as i32).effect {
        crate::effects::Effect::Compressor { threshold, ratio, knee, attack, release, makeup, output_limit, .. } => {
            [threshold, ratio, knee, attack, release, makeup, output_limit]
        }
        _ => unreachable!("called for Compressor only"),
    };
    let auto = matches!(&fx.effect, crate::effects::Effect::Compressor { auto_release, .. } if auto_release == "on");
    let start = first(lo);
    let mut g = 0.0f64;
    for f in lo..hi {
        let (s0, s1) = (first(f), first(f + 1));
        let (a, b) = (read(f), read(f + 1));
        for s in s0..s1 {
            let v = |k: usize| a[k] + (b[k] - a[k]) * (s - s0) as f64 / (s1 - s0) as f64;
            let (t, r, w) = (v(0), v(1), v(2));
            let i = (s - start) as usize;
            let x = 20.0 * y[0][i].abs().max(y[1][i].abs()).max(1e-10).log10();
            let d = x - t;
            let c = if 2.0 * d < -w {
                0.0
            } else if w > 0.0 && 2.0 * d.abs() <= w {
                (1.0 / r - 1.0) * (d + w / 2.0).powi(2) / (2.0 * w)
            } else {
                (1.0 / r - 1.0) * d
            };
            let time = if c < g {
                v(3) * 48.0
            } else if auto && c - g > 3.0 {
                v(4) * 48.0 / 10.0
            } else {
                v(4) * 48.0
            };
            let k = if time > 0.0 { (-1.0 / time).exp() } else { 0.0 };
            g = k * g + (1.0 - k) * c;
            let gain = 10f64.powf((g + v(5)) / 20.0);
            let limit = 10f64.powf(v(6) / 20.0);
            let (l, rr) = (y[0][i] * gain, y[1][i] * gain);
            let q = l.abs().max(rr.abs());
            let scale = if q > limit { limit / q } else { 1.0 };
            y[0][i] = l * scale;
            y[1][i] = rr * scale;
        }
    }
}

/// D-644, document 21: Distortion over `y`, the layer's frames `lo` to `hi` from its first
/// sample. Each channel's sample x goes through the type's curve at a pre-gain of
/// 10^(drive / 50), then Resolution's steps and Downsample's hold (counted from the layer's first
/// sample); the result times gain / 25 is mixed with x by Mix, then scaled by volume / 11.
/// ponytail: no oversampling, so a high Drive folds harmonics above 24 kHz back down (aliasing),
/// a ceiling logged in D-644; add a 4x oversampled path if a playtest hears it.
fn distortion(
    y: &mut [Vec<f64>; 2],
    lo: i64,
    hi: i64,
    first: &dyn Fn(i64) -> i64,
    fx: &crate::effects::EffectInstance,
) {
    let read = |f: i64| match fx.at(f as i32).effect {
        crate::effects::Effect::Distortion { drive, gain, wet, volume, resolution, downsample, .. } => {
            [drive, gain, wet, volume, resolution, downsample]
        }
        _ => unreachable!("called for Distortion only"),
    };
    let kind = match &fx.effect {
        crate::effects::Effect::Distortion { kind, .. } => kind.as_str(),
        _ => unreachable!("called for Distortion only"),
    };
    let t4 = 0.25f64.tanh();
    let curve = |u: f64| match kind {
        "hard_clip" => u.clamp(-1.0, 1.0),
        "saturation_1" => u / (1.0 + u.abs()),
        "saturation_2" => 2.0 / std::f64::consts::PI * (std::f64::consts::PI * u / 2.0).atan(),
        "tube" => ((u + 0.25).tanh() - t4) / (1.0 - t4 * t4),
        "fuzz" => u.abs().sqrt().min(1.0).copysign(u),
        _ => u.tanh(),
    };
    let start = first(lo);
    let (mut count, mut held) = (0i64, [0.0f64; 2]);
    for f in lo..hi {
        let (s0, s1) = (first(f), first(f + 1));
        let (a, b) = (read(f), read(f + 1));
        for s in s0..s1 {
            let v = |k: usize| a[k] + (b[k] - a[k]) * (s - s0) as f64 / (s1 - s0) as f64;
            let i = (s - start) as usize;
            let k = 10f64.powf(v(0) / 50.0);
            let bits = v(4).floor() as i32;
            let mut w = [curve(k * y[0][i]), curve(k * y[1][i])];
            if bits < 24 {
                let q = 2f64.powi(bits - 1);
                w = w.map(|x| (x * q + 0.5).floor() / q);
            }
            if count <= 0 {
                held = w;
                count = v(5).floor() as i64;
            }
            count -= 1;
            let (m, gain, volume) = (v(2) / 100.0, v(1) / 25.0, v(3) / 11.0);
            for c in 0..2 {
                y[c][i] = volume * ((1.0 - m) * y[c][i] + m * held[c] * gain);
            }
        }
    }
}
