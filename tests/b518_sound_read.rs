//! B-518, D-639 (S-1 of D-638): reading sound. Every test file in `Fixtures/sound_read/media`
//! is read by the road its name picks and held to `tools/sound_read_reference.py`'s numbers:
//! its rate, channels, length, where its two clicks land, ffmpeg's decode at named places, and,
//! for files not at 48,000 a second in two channels, the working sound. Opus is refused with
//! MEDIA_AUDIO_PREVIEW_ONLY. A waveform picture of each file goes to `verification/D-639 pictures/`.
//!
//! Writes `verification/D-639_sound_read_table.md`.

mod effect_table;

use anime_compositor::audio;
use effect_table::{repo, Table};
use serde_json::Value as J;

const FPS: u64 = 24;

fn first_sample(n: u64, rate: u64) -> usize {
    (n * rate / FPS) as usize
}

/// The reference tool's rule: the loudest sample of the channels' mean before frame 7, and
/// between frames 7 and 15.
fn clicks(mean: &dyn Fn(usize) -> f64, len: usize, rate: u64) -> [usize; 2] {
    let loudest = |a: usize, b: usize| {
        (a..b.min(len)).fold(a, |best, i| {
            if mean(i).abs() > mean(best).abs() {
                i
            } else {
                best
            }
        })
    };
    let (a, b) = (first_sample(7, rate), first_sample(15, rate));
    [loudest(0, a), loudest(a, b)]
}

fn picture(name: &str, sound: &audio::Sound) {
    let (w, h) = (900usize, 160usize);
    let spans = sound.spans(0, sound.samples as usize, w, 1.0, None);
    let mut px = vec![255u8; w * h * 3];
    for x in 0..w {
        let (lo, hi) = (spans[2 * x], spans[2 * x + 1]);
        let row = |v: f64| (((1.0 - v) / 2.0) * (h - 1) as f64).round() as usize;
        for y in row(hi)..=row(lo) {
            px[(y * w + x) * 3..][..3].copy_from_slice(&[30, 60, 160]);
        }
        px[(h / 2 * w + x) * 3..][..3].copy_from_slice(&[180, 180, 180]);
    }
    let dir = repo("verification/D-639 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    let file = std::fs::File::create(dir.join(format!("{}.png", name.replace('.', "_")))).unwrap();
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), w as u32, h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().unwrap().write_image_data(&px).unwrap();
}

#[test]
fn b518_sound_read() {
    let mut t = Table::new(
        "sound_read",
        "# D-639: reading sound (B-518)\n\nEach test file is read by the road its name picks \
         (WAV by hand as D-71 reads it; FLAC, MP3 and Ogg Vorbis by symphonia; AAC by Windows' own \
         decoder) and held to the numbers `tools/sound_read_reference.py` wrote before the code: \
         ffmpeg 8.1's decode of the same file. Every test sound has a click on the first sample \
         of frames 2 and 12 at 24 frames a second (B-112's method), so a click on the right sample \
         means the sound is in step with the picture.\n",
    );
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(t.root.join("expected_sound_read.json")).unwrap(),
    )
    .unwrap();
    let bound = expected["lossy_bound"].as_f64().unwrap();
    let media = t.root.join("media");
    let mut measured: Vec<(String, f64)> = Vec::new();

    t.heading("Each file against ffmpeg's decode");
    for f in expected["files"].as_array().unwrap() {
        let name = f["file"].as_str().unwrap();
        let sound = match audio::read_file(&media.join(name)) {
            Ok(s) => s,
            Err(d) => {
                t.row(
                    &format!("`{name}` is read"),
                    &format!("refused: {}", d.id.as_str()),
                    false,
                );
                continue;
            }
        };
        let (rate, ch) = (
            f["rate"].as_u64().unwrap(),
            f["channels"].as_u64().unwrap() as usize,
        );
        let lossless = f["lossless"].as_bool().unwrap();
        let tol = if lossless { 2e-5 } else { bound };
        t.row(
            &format!(
                "`{name}`: {rate} a second, {ch} channel{}",
                if ch == 1 { "" } else { "s" }
            ),
            &format!(
                "{} a second, {} channel{}",
                sound.rate,
                sound.channels(),
                if sound.channels() == 1 { "" } else { "s" }
            ),
            sound.rate as u64 == rate && sound.channels() == ch,
        );
        let len = sound.samples as usize;
        let (lo, hi) = (
            f["shortest"].as_u64().unwrap() as usize,
            f["samples"].as_u64().unwrap() as usize,
        );
        let want_len = if lo == hi {
            format!("{hi}")
        } else {
            format!("{lo} to {hi}")
        };
        t.row(
            &format!("`{name}`: length {want_len} samples"),
            &format!("{len}"),
            (lo..=hi).contains(&len),
        );
        let want_clicks: Vec<usize> = f["clicks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as usize)
            .collect();
        let got = clicks(&|i| sound.mean(i as i64), len, rate);
        t.row(
            &format!(
                "`{name}`: clicks on samples {} and {}",
                want_clicks[0], want_clicks[1]
            ),
            &format!("{} and {}", got[0], got[1]),
            got[..] == want_clicks[..],
        );
        let mut worst = 0f64;
        let mut compared = 0;
        for (i, v) in f["places"]
            .as_array()
            .unwrap()
            .iter()
            .zip(f["values"].as_array().unwrap())
        {
            let i = i.as_u64().unwrap() as usize;
            if i >= len {
                continue;
            }
            compared += 1;
            for (c, v) in v.as_array().unwrap().iter().enumerate() {
                worst = worst.max((sound.at(c, i as i64) - v.as_f64().unwrap()).abs());
            }
        }
        t.row(
            &format!(
                "`{name}`: {compared} named places within {}",
                if lossless {
                    "2e-5".to_string()
                } else {
                    format!("{bound} (lossy, before measuring)")
                }
            ),
            &format!("largest difference {worst:.2e}"),
            worst <= tol,
        );
        if !lossless {
            measured.push((name.to_string(), worst));
        }
        if let Some(w) = f.get("working") {
            let [l, r] = sound.working();
            let n = w["samples"].as_u64().unwrap() as usize;
            t.row(
                &format!("`{name}` working sound: {n} samples at 48,000"),
                &format!("{} and {}", l.len(), r.len()),
                l.len() == n && r.len() == n,
            );
            let wc: Vec<usize> = w["clicks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_u64().unwrap() as usize)
                .collect();
            let got = clicks(&|i| (l[i] + r[i]) / 2.0, l.len(), 48000);
            t.row(
                &format!("`{name}` working sound: clicks on {} and {}", wc[0], wc[1]),
                &format!("{} and {}", got[0], got[1]),
                got[..] == wc[..],
            );
            let mut worst = 0f64;
            for (i, v) in w["places"]
                .as_array()
                .unwrap()
                .iter()
                .zip(w["values"].as_array().unwrap())
            {
                let i = i.as_u64().unwrap() as usize;
                let v = v.as_array().unwrap();
                worst = worst
                    .max((l[i] - v[0].as_f64().unwrap()).abs())
                    .max((r[i] - v[1].as_f64().unwrap()).abs());
            }
            t.row(
                &format!("`{name}` working sound: named places within 2e-5"),
                &format!("largest difference {worst:.2e}"),
                worst <= 2e-5,
            );
        }
        picture(name, &sound);
    }

    t.heading("Refused, and what an older project already had");
    for f in expected["refused"].as_array().unwrap() {
        let name = f["file"].as_str().unwrap();
        let want = f["diagnostic"].as_str().unwrap();
        let got = match audio::read_file(&media.join(name)) {
            Ok(_) => "read".to_string(),
            Err(d) => format!("{}: {}", d.id.as_str(), d.message),
        };
        t.row(
            &format!("`{name}` refused with {want}, the window still plays it"),
            &got,
            got.starts_with(want),
        );
    }
    let wav = std::fs::read(media.join("stereo_48k.wav")).unwrap();
    let by_hand = audio::Sound::read(wav).unwrap();
    let by_name = audio::read_file(&media.join("stereo_48k.wav")).unwrap();
    let same = by_hand.samples == by_name.samples
        && (0..by_hand.samples as i64)
            .all(|i| by_hand.at(0, i) == by_name.at(0, i) && by_hand.at(1, i) == by_name.at(1, i));
    t.row(
        "a WAV reads exactly as D-71's reader reads it",
        if same {
            "every sample the same"
        } else {
            "different"
        },
        same,
    );
    let working = by_name.working();
    let same = (0..by_name.samples as usize).all(|i| {
        working[0][i] == by_name.at(0, i as i64) && working[1][i] == by_name.at(1, i as i64)
    });
    t.row(
        "a 48,000 stereo file's working sound is the file",
        if same {
            "every sample the same"
        } else {
            "different"
        },
        same,
    );
    let not = audio::read_file(&repo("Fixtures/audio/media/not_a_wav.wav"))
        .err()
        .map(|d| d.id.as_str());
    t.row(
        "a file that is not a WAV inside is still MEDIA_AUDIO_UNREADABLE",
        &format!("{not:?}"),
        not == Some("MEDIA_AUDIO_UNREADABLE"),
    );

    // B-518's measurement: the lossy files land as close to ffmpeg's decode as the lossless
    // ones, so the usual 2e-5 is proposed for them too, and checked here.
    t.heading("Measured: the lossy files against ffmpeg's decode, held to 2e-5");
    for (name, worst) in &measured {
        t.row(
            &format!("`{name}`: largest difference at the named places"),
            &format!("{worst:.2e}"),
            *worst <= 2e-5,
        );
    }
    t.finish("D-639_sound_read_table.md");
}

/// How long reading and the working sound take: a three-minute 44,100 stereo WAV, and the
/// same as MP3 (made by ffmpeg). Run with `--ignored`.
#[test]
#[ignore]
fn b518_sound_read_timing() {
    let dir = std::env::temp_dir().join("b518_timing");
    std::fs::create_dir_all(&dir).unwrap();
    let wav = dir.join("long.wav");
    let ok = std::process::Command::new("ffmpeg")
        .args([
            "-v",
            "error",
            "-y",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:sample_rate=44100:duration=180",
            "-ac",
            "2",
            "-c:a",
            "pcm_s16le",
        ])
        .arg(&wav)
        .status()
        .unwrap()
        .success();
    assert!(ok);
    for (ext, codec) in [("mp3", "libmp3lame"), ("flac", "flac"), ("m4a", "aac")] {
        let to = dir.join(format!("long.{ext}"));
        assert!(std::process::Command::new("ffmpeg")
            .args(["-v", "error", "-y", "-i"])
            .arg(&wav)
            .args(["-c:a", codec])
            .arg(&to)
            .status()
            .unwrap()
            .success());
    }
    let mut out = String::new();
    for ext in ["wav", "flac", "mp3", "m4a"] {
        let path = dir.join(format!("long.{ext}"));
        let start = std::time::Instant::now();
        let s = audio::read_file(&path).unwrap();
        let read = start.elapsed().as_secs_f64() * 1000.0;
        let start = std::time::Instant::now();
        let w = s.working();
        let work = start.elapsed().as_secs_f64() * 1000.0;
        out.push_str(&format!(
            "| {ext} | {:.0} | {:.0} | {} |\n",
            read,
            work,
            w[0].len()
        ));
    }
    println!(
        "| File | Read (ms) | Working sound (ms) | Working samples |\n|---|---:|---:|---:|\n{out}"
    );
}
