//! D-71 and ADR-018: reference audio. Whole frames become whole samples, and a WAV header is
//! read by hand. D-639 reads FLAC, MP3, Ogg Vorbis and AAC as well, and turns every sound into
//! the working sound (48,000 a second, two channels). Nothing here plays sound; the window does.
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
