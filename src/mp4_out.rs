//! D-72's MP4: H.264 video in an MP4, written by the encoder Windows carries (Media Foundation),
//! which is the road D-30 names. This build ships no codec of its own. On a system that is not
//! Windows the frames go to an `ffmpeg` the user has installed instead (D-30's second road,
//! B-21e), which ships no codec either.
//!
//! An MP4 has no alpha, so a frame is laid over black. It has no sound (D-71's ceiling).

use std::path::Path;

use crate::time::FrameRate;

/// Why this job cannot be an MP4, or `None`. Asked before any file is made. H.264 needs an even
/// width and height; nothing is padded or cropped to get one (D-72).
pub fn refusal(width: usize, height: usize) -> Option<String> {
    let odd = match (width % 2 == 1, height % 2 == 1) {
        (true, true) => format!("its width, {width}, and its height, {height}, are odd"),
        (true, false) => format!("its width, {width}, is odd"),
        (false, true) => format!("its height, {height}, is odd"),
        (false, false) => return None,
    };
    Some(format!(
        "An MP4 needs an even width and an even height, and this composition is {width}x{height}: \
         {odd}. Nothing is padded or cropped for you; change the composition's size, or export a \
         PNG sequence."
    ))
}

/// When frame `index` starts, in the 100-nanosecond units Media Foundation counts in. This is
/// only how the frames are handed over; `set_clock` writes the file's real clock afterwards.
#[cfg_attr(not(windows), allow(dead_code))]
fn starts_at(index: u64, rate: FrameRate) -> i64 {
    (index * 10_000_000 * rate.denominator() as u64 / rate.numerator() as u64) as i64
}

/// Give a finished file the clock FX-FMT-030 asks for: time counted in the frame rate's
/// numerator, every frame lasting its denominator, so 24000/1001 is exact.
///
/// Media Foundation counts in its own units and writes 23976 ticks a second with frames of 1000,
/// which is close to 24000/1001 and is not it. The pictures are untouched: only the numbers in
/// the file's index (`moov`) change, each the same size as the one it replaces. Anything in the
/// index this does not expect is an error rather than a guess.
pub fn set_clock(path: &Path, rate: FrameRate, frames: u64) -> Result<(), String> {
    use std::io::{Read, Seek, SeekFrom, Write};
    let (num, den) = (rate.numerator(), rate.denominator());
    let length = u32::try_from(frames * den as u64)
        .map_err(|_| "the film is too long for an MP4's 32-bit clock".to_string())?;
    let io = |e: std::io::Error| e.to_string();
    let mut file = std::fs::OpenOptions::new().read(true).write(true).open(path).map_err(io)?;

    // The top of the file: boxes of (size, four letters). `moov` is the index.
    let mut at = 0u64;
    let moov_len = loop {
        let mut head = [0u8; 16];
        file.seek(SeekFrom::Start(at)).map_err(io)?;
        file.read_exact(&mut head[..8]).map_err(|_| "the MP4 has no index (moov)".to_string())?;
        let mut size = u32::from_be_bytes(head[..4].try_into().unwrap()) as u64;
        if size == 1 {
            file.read_exact(&mut head[8..]).map_err(io)?;
            size = u64::from_be_bytes(head[8..].try_into().unwrap());
        }
        if &head[4..8] == b"moov" {
            break size;
        }
        if size < 8 {
            return Err("the MP4 has a box with no size".into());
        }
        at += size;
    };
    let mut moov = vec![0u8; moov_len as usize];
    file.seek(SeekFrom::Start(at)).map_err(io)?;
    file.read_exact(&mut moov).map_err(io)?;

    let mut seen = Vec::new();
    patch(&mut moov, 8, moov_len as usize, num, den, length, frames, &mut seen)?;
    for needed in ["mvhd", "tkhd", "mdhd", "stts"] {
        if seen.iter().filter(|s| *s == needed).count() != 1 {
            return Err(format!("the MP4's index does not hold exactly one {needed}"));
        }
    }
    file.seek(SeekFrom::Start(at)).map_err(io)?;
    file.write_all(&moov).map_err(io)
}

#[allow(clippy::too_many_arguments)]
fn patch(
    b: &mut [u8],
    mut at: usize,
    end: usize,
    num: u32,
    den: u32,
    length: u32,
    frames: u64,
    seen: &mut Vec<String>,
) -> Result<(), String> {
    let get = |b: &[u8], at: usize| u32::from_be_bytes(b[at..at + 4].try_into().unwrap());
    let put = |b: &mut [u8], at: usize, v: u32| b[at..at + 4].copy_from_slice(&v.to_be_bytes());
    while at + 8 <= end {
        let size = get(b, at) as usize;
        let name = String::from_utf8_lossy(&b[at + 4..at + 8]).into_owned();
        if size < 8 || at + size > end {
            return Err(format!("the MP4's {name} box has a size that does not fit"));
        }
        let body = at + 8;
        match name.as_str() {
            "trak" | "mdia" | "minf" | "stbl" => patch(b, body, at + size, num, den, length, frames, seen)?,
            // version and flags, two dates, then the timescale and the duration.
            "mvhd" | "mdhd" => {
                if b[body] != 0 {
                    return Err(format!("the MP4's {name} is a version this build does not rewrite"));
                }
                put(b, body + 12, num);
                put(b, body + 16, length);
            }
            // version and flags, two dates, the track's number, four spare bytes, the duration.
            "tkhd" => {
                if b[body] != 0 {
                    return Err("the MP4's tkhd is a version this build does not rewrite".into());
                }
                put(b, body + 20, length);
            }
            // Runs of (how many frames, how long each lasts).
            "stts" => {
                let runs = get(b, body + 4) as usize;
                let mut counted = 0u64;
                for run in 0..runs {
                    counted += get(b, body + 8 + run * 8) as u64;
                    put(b, body + 12 + run * 8, den);
                }
                if counted != frames {
                    return Err(format!(
                        "the MP4 holds {counted} frames and {frames} were written into it"
                    ));
                }
            }
            // Either would hold times in the old clock, and the encoder writes neither today.
            "edts" | "ctts" => {
                return Err(format!("the MP4 holds a {name} box, which this build does not rewrite"))
            }
            _ => {}
        }
        seen.push(name);
        at += size;
    }
    Ok(())
}

/// 8-bit straight RGBA, laid over black, as the NV12 picture an H.264 encoder takes: a plane of
/// brightness, then a half-size plane of the two colour differences side by side.
///
/// Done here and not left to Windows, whose own conversion uses the standard-definition numbers
/// (BT.601) and does not say so in the file, so an HD player, which assumes BT.709, shows the
/// greens wrong. These are BT.709's numbers, in the 16 to 235 range video uses. `width` and
/// `height` are even: `refusal` saw to it.
pub fn nv12_bt709(srgb8: &[u8], width: usize, height: usize) -> Vec<u8> {
    let at = |x: usize, y: usize| {
        let p = &srgb8[(y * width + x) * 4..][..4];
        let a = p[3] as f32 / 255.0;
        [p[0] as f32 * a / 255.0, p[1] as f32 * a / 255.0, p[2] as f32 * a / 255.0]
    };
    let luma = |c: [f32; 3]| 0.2126 * c[0] + 0.7152 * c[1] + 0.0722 * c[2];
    let mut out = vec![0u8; width * height * 3 / 2];
    for y in 0..height {
        for x in 0..width {
            out[y * width + x] = (16.0 + 219.0 * luma(at(x, y))).round() as u8;
        }
    }
    let chroma = width * height;
    for y in (0..height).step_by(2) {
        for x in (0..width).step_by(2) {
            // One colour for each block of four: their average.
            let four = [at(x, y), at(x + 1, y), at(x, y + 1), at(x + 1, y + 1)];
            let mean = [0, 1, 2].map(|i| four.iter().map(|c| c[i]).sum::<f32>() / 4.0);
            let l = luma(mean);
            let to = chroma + y / 2 * width + x;
            out[to] = (128.0 + 224.0 * (mean[2] - l) / 1.8556).round() as u8;
            out[to + 1] = (128.0 + 224.0 * (mean[0] - l) / 1.5748).round() as u8;
        }
    }
    out
}

#[cfg(not(windows))]
pub use through_ffmpeg::Mp4;

/// D-30's second road, for systems that are not Windows: an `ffmpeg` the user has installed,
/// found on the PATH when the export starts. Compiled everywhere so that Windows can test it.
pub mod through_ffmpeg {
    use super::*;
    use std::io::{Read, Write};
    use std::process::{Child, Command, Stdio};

    pub struct Mp4 {
        ffmpeg: Child,
        width: usize,
        height: usize,
    }

    impl Mp4 {
        pub fn create(path: &Path, width: usize, height: usize, rate: FrameRate) -> Result<Self, String> {
            let (num, den) = (rate.numerator(), rate.denominator());
            // The same bitrate rule as the Windows road.
            let bits = ((width * height) as f64 * num as f64 / den as f64 * 0.2).clamp(1.0e6, 1.0e8) as u64;
            // ponytail: asks for libx264 by name; a build of ffmpeg without it fails with
            // ffmpeg's own sentence. Try the system's encoder (h264_videotoolbox on macOS) when
            // someone meets that.
            Command::new("ffmpeg")
                .args(["-loglevel", "error", "-y", "-f", "rawvideo", "-pix_fmt", "nv12"])
                .args(["-s", &format!("{width}x{height}"), "-r", &format!("{num}/{den}"), "-i", "-"])
                // No reordered frames, so the file's index is the plain one FX-FMT-030 reads.
                .args(["-c:v", "libx264", "-bf", "0", "-b:v", &bits.to_string(), "-pix_fmt", "yuv420p"])
                .args(["-colorspace", "bt709", "-color_primaries", "bt709", "-color_trc", "bt709"])
                .args(["-color_range", "tv", "-video_track_timescale", &num.to_string()])
                .arg(path)
                .stdin(Stdio::piped())
                .stdout(Stdio::null())
                .stderr(Stdio::piped())
                .spawn()
                .map(|ffmpeg| Mp4 { ffmpeg, width, height })
                .map_err(|_| {
                    "MP4 export on this system needs ffmpeg, and no program called ffmpeg was \
                     found. Install it, or export an animated PNG or a PNG sequence."
                        .to_string()
                })
        }

        pub fn push(&mut self, srgb8: &[u8]) -> Result<(), String> {
            let picture = nv12_bt709(srgb8, self.width, self.height);
            let sent = self.ffmpeg.stdin.as_mut().expect("opened piped").write_all(&picture);
            sent.map_err(|_| self.said())
        }

        pub fn finish(mut self) -> Result<(), String> {
            drop(self.ffmpeg.stdin.take());
            match self.ffmpeg.wait() {
                Ok(status) if status.success() => Ok(()),
                _ => Err(self.said()),
            }
        }

        /// ffmpeg's own words for what went wrong.
        fn said(&mut self) -> String {
            let mut words = String::new();
            if let Some(mut err) = self.ffmpeg.stderr.take() {
                let _ = err.read_to_string(&mut words);
            }
            format!("ffmpeg could not write the MP4: {}", words.trim())
        }
    }

    impl Drop for Mp4 {
        // A cancelled export drops the film without finishing it; do not leave ffmpeg waiting.
        fn drop(&mut self) {
            drop(self.ffmpeg.stdin.take());
            let _ = self.ffmpeg.wait();
        }
    }
}

#[cfg(windows)]
pub use on_windows::Mp4;

#[cfg(windows)]
mod on_windows {
    use super::*;
    use windows::core::HSTRING;
    use windows::Win32::Media::MediaFoundation::*;
    use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

    pub struct Mp4 {
        path: std::path::PathBuf,
        writer: IMFSinkWriter,
        stream: u32,
        width: usize,
        height: usize,
        rate: FrameRate,
        frames: u64,
    }

    fn said(e: windows::core::Error) -> String {
        format!("Windows' video encoder answered: {}", e.message())
    }

    fn video_type(
        format: &windows::core::GUID,
        width: usize,
        height: usize,
        rate: FrameRate,
    ) -> windows::core::Result<IMFMediaType> {
        unsafe {
            let t = MFCreateMediaType()?;
            t.SetGUID(&MF_MT_MAJOR_TYPE, &MFMediaType_Video)?;
            t.SetGUID(&MF_MT_SUBTYPE, format)?;
            t.SetUINT32(&MF_MT_INTERLACE_MODE, MFVideoInterlace_Progressive.0 as u32)?;
            t.SetUINT64(&MF_MT_FRAME_SIZE, (width as u64) << 32 | height as u64)?;
            t.SetUINT64(
                &MF_MT_FRAME_RATE,
                (rate.numerator() as u64) << 32 | rate.denominator() as u64,
            )?;
            t.SetUINT64(&MF_MT_PIXEL_ASPECT_RATIO, 1 << 32 | 1)?;
            Ok(t)
        }
    }

    impl Mp4 {
        pub fn create(path: &Path, width: usize, height: usize, rate: FrameRate) -> Result<Self, String> {
            unsafe {
                // Already initialised on this thread is an answer too, and not a failure.
                let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
                MFStartup(MF_VERSION, MFSTARTUP_FULL).map_err(said)?;
                let open = || -> windows::core::Result<(IMFSinkWriter, u32)> {
                    let writer =
                        MFCreateSinkWriterFromURL(&HSTRING::from(path.as_os_str()), None, None)?;
                    let out = video_type(&MFVideoFormat_H264, width, height, rate)?;
                    // ponytail: one bitrate rule, 0.2 bits a pixel a frame (about 10 Mbit/s at
                    // 1080p24), and no quality choice; add one when the owner asks for smaller
                    // or cleaner files.
                    let fps = rate.numerator() as f64 / rate.denominator() as f64;
                    let bits = (width * height) as f64 * fps * 0.2;
                    out.SetUINT32(&MF_MT_AVG_BITRATE, bits.clamp(1.0e6, 1.0e8) as u32)?;
                    // ponytail: the encoder's default profile, Baseline. High packs smaller but
                    // reorders frames, which puts a `ctts` box in the file that `set_clock`
                    // refuses; teach `set_clock` that box before asking for High.
                    let stream = writer.AddStream(&out)?;
                    let into = video_type(&MFVideoFormat_NV12, width, height, rate)?;
                    writer.SetInputMediaType(stream, &into, None)?;
                    writer.BeginWriting()?;
                    Ok((writer, stream))
                };
                match open() {
                    Ok((writer, stream)) => Ok(Mp4 { path: path.to_path_buf(), writer, stream, width, height, rate, frames: 0 }),
                    Err(e) => {
                        let _ = MFShutdown();
                        Err(said(e))
                    }
                }
            }
        }

        /// Add one frame of 8-bit straight RGBA, laid over black: an MP4 has no alpha.
        pub fn push(&mut self, srgb8: &[u8]) -> Result<(), String> {
            let (w, h) = (self.width, self.height);
            let (at, next) = (starts_at(self.frames, self.rate), starts_at(self.frames + 1, self.rate));
            self.frames += 1;
            let write = || -> windows::core::Result<()> {
                unsafe {
                    let picture = nv12_bt709(srgb8, w, h);
                    let buffer = MFCreateMemoryBuffer(picture.len() as u32)?;
                    let mut data = std::ptr::null_mut();
                    buffer.Lock(&mut data, None, None)?;
                    std::ptr::copy_nonoverlapping(picture.as_ptr(), data, picture.len());
                    buffer.Unlock()?;
                    buffer.SetCurrentLength(picture.len() as u32)?;
                    let sample = MFCreateSample()?;
                    sample.AddBuffer(&buffer)?;
                    sample.SetSampleTime(at)?;
                    sample.SetSampleDuration(next - at)?;
                    self.writer.WriteSample(self.stream, &sample)
                }
            };
            write().map_err(said)
        }

        pub fn finish(self) -> Result<(), String> {
            unsafe { self.writer.Finalize() }.map_err(said)?;
            set_clock(&self.path, self.rate, self.frames)
        }
    }

    impl Drop for Mp4 {
        fn drop(&mut self) {
            unsafe {
                let _ = MFShutdown();
            }
        }
    }
}
