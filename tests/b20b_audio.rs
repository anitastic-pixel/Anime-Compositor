//! B-20b: reference audio in the core, against D-71 and ADR-018.
//!
//! Writes `verification/B-20b_audio_table.md`.
//!
//! Every expected value is `Fixtures/audio/expected_audio.json`, written by
//! `tools/audio_reference.py` before this code existed and printed in document 25 as FX-AUD.
//! The numbers are whole and the match is exact. Nothing here is a snapshot of a run.
//!
//! Not here: hearing anything. Playback, the waveform and the bar styles are B-20c.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::audio::{self, Encoding, Wav};
use anime_compositor::command::{Command, Document, Target};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{BlendMode, Id, Prop, Value};
use anime_compositor::persist;
use anime_compositor::time::LayerTiming;

const COMP: &str = "comp-main";

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

struct Table {
    out: String,
    checks: usize,
    passed: usize,
}

impl Table {
    fn row(&mut self, what: &str, built: &str, ok: bool) {
        self.checks += 1;
        self.passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        self.out
            .push_str(&format!("| {what} | {built} | {verdict} |\n"));
    }

    fn heading(&mut self, text: &str) {
        self.out.push_str(&format!(
            "\n## {text}\n\n| Check | The build's answer | Matches |\n| --- | --- | --- |\n"
        ));
    }
}

fn u(v: &J) -> u32 {
    v.as_u64().unwrap() as u32
}

/// A WAV header as the fixture writes one.
fn wav_json(raw: &[u8], warnings: &mut Vec<String>) -> J {
    match audio::read_wav(raw) {
        Err(d) => {
            warnings.push(d.id.as_str().to_string());
            json!({ "refused": d.detail })
        }
        Ok(Wav::Other {
            channels,
            sample_rate,
        }) => json!({ "encoding": "other", "channels": channels, "sample_rate": sample_rate }),
        Ok(Wav::Read {
            encoding,
            channels,
            sample_rate,
            bits,
            samples,
            cut_short,
        }) => {
            let mut got = json!({
                "encoding": if encoding == Encoding::Pcm { "pcm" } else { "float" },
                "channels": channels,
                "sample_rate": sample_rate,
                "bits": bits,
                "samples": samples,
                "frames_at_24": audio::length_frames(samples, sample_rate, 24, 1),
            });
            if cut_short {
                got["warning"] = json!(audio::cut_short("").id.as_str());
            }
            got
        }
    }
}

fn refusal(result: Result<(), anime_compositor::diagnostics::Diagnostic>) -> String {
    match result {
        Ok(()) => "accepted".to_string(),
        Err(d) => format!("{}: {}", d.id.as_str(), d.message),
    }
}

#[test]
fn the_audio_fixtures() {
    let root = repo("Fixtures/audio");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_audio.json")).unwrap())
            .unwrap();
    let cases = &expected["cases"];
    let mut t = Table {
        out: String::new(),
        checks: 0,
        passed: 0,
    };

    t.heading("A frame becomes a sample (FX-AUD-001 to 004)");
    for fx in ["FX-AUD-001", "FX-AUD-002", "FX-AUD-003", "FX-AUD-004"] {
        let case = &cases[fx];
        let (rate, num, den) = (
            u(&case["sample_rate"]),
            u(&case["frame_rate"][0]),
            u(&case["frame_rate"][1]),
        );
        let want = case["first_sample"].as_object().unwrap();
        let mut frames: Vec<u64> = want.keys().map(|k| k.parse().unwrap()).collect();
        frames.sort();
        let got: Vec<u64> = frames
            .iter()
            .map(|n| audio::first_sample(*n, rate, num, den))
            .collect();
        let ok = frames
            .iter()
            .zip(&got)
            .all(|(n, s)| want[&n.to_string()].as_u64() == Some(*s));
        t.row(
            &format!("{fx}: {}", case["says"].as_str().unwrap()),
            &format!("frames {frames:?} begin at samples {got:?}"),
            ok,
        );
    }

    t.heading("What is heard on a layer (FX-AUD-005 to 008)");
    for fx in ["FX-AUD-005", "FX-AUD-006", "FX-AUD-007", "FX-AUD-008"] {
        let case = &cases[fx];
        let layer = &case["layer"];
        let timing = LayerTiming {
            in_frame: layer["in_frame"].as_i64().unwrap() as i32,
            out_frame: layer["out_frame"].as_i64().unwrap() as i32,
            source_offset_frames: layer["source_offset_frames"].as_i64().unwrap() as i32,
        };
        let samples = layer["samples"].as_u64().unwrap();
        let want = case["heard"].as_object().unwrap();
        let mut frames: Vec<i32> = want.keys().map(|k| k.parse().unwrap()).collect();
        frames.sort();
        let mut said = Vec::new();
        let mut ok = true;
        for f in frames {
            let got = audio::heard(&timing, samples, f, 48000, 24, 1);
            let got_json = got.map_or(J::Null, |(a, b)| json!([a, b]));
            ok &= got_json == want[&f.to_string()];
            said.push(match got {
                None => format!("{f}: silence"),
                Some((a, b)) => format!("{f}: {a} up to {b}"),
            });
        }
        t.row(
            &format!("{fx}: {}", case["says"].as_str().unwrap()),
            &said.join("; "),
            ok,
        );
    }

    t.heading("What each WAV file is read as (FX-AUD-010)");
    let files = cases["FX-AUD-010"]["files"].as_object().unwrap();
    for (name, want) in files {
        let raw = fs::read(root.join("media").join(name)).unwrap();
        let mut codes = Vec::new();
        let got = wav_json(&raw, &mut codes);
        // A refusal's words are the build's own; the fixture pins that it is refused, and the
        // code it is refused with is D-71's.
        let ok = if want.get("refused").is_some() {
            got.get("refused").is_some() && codes == ["MEDIA_AUDIO_UNREADABLE"]
        } else {
            &got == want
        };
        let shown = if codes.is_empty() {
            got.to_string()
        } else {
            format!("{} {got}", codes[0])
        };
        t.row(&format!("`{name}`"), &shown, ok);
    }

    t.heading("The picture does not change (FX-AUD-020)");
    {
        let case = &cases["FX-AUD-020"];
        let with = persist::load(&root.join(case["project"].as_str().unwrap())).unwrap();
        let without = persist::load(&root.join(case["same_picture_as"].as_str().unwrap())).unwrap();
        t.row(
            "Both projects open with no warning",
            &format!(
                "{} and {} warnings",
                with.warnings.len(),
                without.warnings.len()
            ),
            with.warnings.is_empty() && without.warnings.is_empty(),
        );
        for frame in case["frames"].as_array().unwrap() {
            let frame = frame.as_i64().unwrap() as i32;
            let mut logs = (FrameLog::new(8), FrameLog::new(8));
            let a = render_frame(
                with.document.project(),
                &Id::new(COMP),
                frame,
                &root,
                64,
                &mut logs.0,
            )
            .unwrap();
            let b = render_frame(
                without.document.project(),
                &Id::new(COMP),
                frame,
                &root,
                64,
                &mut logs.1,
            )
            .unwrap();
            let differing = a
                .data()
                .iter()
                .zip(b.data())
                .filter(|(x, y)| x.to_bits() != y.to_bits())
                .count();
            t.row(
                &format!("Frame {frame} with the audio layer is frame {frame} without it"),
                &format!("{} samples compared, {differing} differ", a.data().len()),
                differing == 0 && a.data().len() == b.data().len(),
            );
        }
        let text = persist::to_json(with.document.project(), &with.preserved);
        let back: J = serde_json::from_str(&text).unwrap();
        let file: J = serde_json::from_str(
            &fs::read_to_string(root.join(case["project"].as_str().unwrap())).unwrap(),
        )
        .unwrap();
        t.row(
            "Saved again, the audio asset and the audio layer are what the file held",
            &format!(
                "{} / {}",
                back["assets"][2], back["compositions"][0]["layers"][1]
            ),
            back["assets"][2] == file["assets"][2]
                && back["compositions"][0]["layers"][1] == file["compositions"][0]["layers"][1],
        );
    }

    t.heading("What a file may not say (FX-AUD-030 to 035)");
    for fx in [
        "FX-AUD-030",
        "FX-AUD-031",
        "FX-AUD-032",
        "FX-AUD-033",
        "FX-AUD-034",
        "FX-AUD-035",
    ] {
        let case = &cases[fx];
        let got = persist::load(&root.join(case["project"].as_str().unwrap()));
        let (code, words) = match &got {
            Ok(_) => ("opened".to_string(), String::new()),
            Err(d) => (d.id.as_str().to_string(), d.detail.clone()),
        };
        t.row(
            &format!("{fx}: {}", case["says"].as_str().unwrap()),
            &format!("{code}. {words}"),
            Some(code.as_str()) == case["refused"].as_str(),
        );
    }

    t.heading("Editing (D-71; no fixture, the rule is a sentence)");
    {
        let loaded = persist::load(&root.join("fx_aud_020.json")).unwrap();
        let mut document: Document = loaded.document;
        let comp = Id::new(COMP);
        let sound = Id::new("sound");
        let level = |d: &Document| {
            d.project()
                .composition(&Id::new(COMP))
                .unwrap()
                .layer(&Id::new("sound"))
                .unwrap()
                .gain_db
        };
        let depth = document.undo_depth();
        let set = document
            .apply(Command::SetAudioGain {
                composition: comp.clone(),
                layer_id: sound.clone(),
                value: -6.0,
            })
            .map(|_| ());
        t.row(
            "SET_AUDIO_GAIN to -6 sets the level, as one entry to undo",
            &format!(
                "{}; level {}; {} new entry",
                refusal(set),
                level(&document),
                document.undo_depth() - depth
            ),
            level(&document) == -6.0 && document.undo_depth() == depth + 1,
        );
        document.undo();
        t.row(
            "Undo puts the level back",
            &format!("level {}", level(&document)),
            level(&document) == 0.0,
        );
        let refused_with = |d: &mut Document, c: Command| {
            let got = refusal(d.apply(c).map(|_| ()));
            let ok = got.starts_with("COMMAND_");
            (got, ok)
        };
        let tries: Vec<(&str, Command)> = vec![
            (
                "A level of +12.5 is refused",
                Command::SetAudioGain {
                    composition: comp.clone(),
                    layer_id: sound.clone(),
                    value: 12.5,
                },
            ),
            (
                "A level on a picture layer is refused",
                Command::SetAudioGain {
                    composition: comp.clone(),
                    layer_id: Id::new("bg"),
                    value: -6.0,
                },
            ),
            (
                "A position on the audio layer is refused",
                Command::SetPropertyBase {
                    composition: comp.clone(),
                    target: Target::Layer(sound.clone()),
                    prop: Prop::Position,
                    value: Value::Vec2(1.0, 1.0),
                },
            ),
            (
                "A blend mode on the audio layer is refused",
                Command::SetBlendMode {
                    composition: comp.clone(),
                    layer_id: sound.clone(),
                    mode: BlendMode::Multiply,
                },
            ),
            (
                "The audio layer as a matte is refused",
                Command::SetMatte {
                    composition: comp.clone(),
                    layer_id: Id::new("half"),
                    matte: Some(sound.clone()),
                    matte_only: false,
                },
            ),
            (
                "The audio layer as a parent is refused",
                Command::SetParent {
                    composition: comp.clone(),
                    layer_id: Id::new("half"),
                    parent: Some(sound.clone()),
                    frame: 0,
                    keep_place: false,
                },
            ),
        ];
        for (what, command) in tries {
            let (got, ok) = refused_with(&mut document, command);
            t.row(what, &got, ok);
        }
        let moved = document
            .apply(Command::ShiftLayer {
                composition: comp.clone(),
                layer_id: sound.clone(),
                in_frame: 2,
            })
            .map(|_| ());
        let at = document
            .project()
            .composition(&comp)
            .unwrap()
            .layer(&sound)
            .unwrap()
            .in_frame;
        t.row(
            "Moving the audio layer two frames later is the command every layer has",
            &format!("{}; in_frame {at}", refusal(moved)),
            at == 2,
        );

        // Relink and collect, which B-20b's backlog line names.
        let mut other = document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new("asset-sound"))
            .unwrap()
            .clone();
        other.path = Some("media/pcm8_mono_8k.wav".to_string());
        let relinked = document
            .apply(Command::RelinkAsset {
                asset: Box::new(other),
            })
            .map(|_| ());
        let project = document.project();
        let now = project
            .assets
            .iter()
            .find(|a| a.id == Id::new("asset-sound"))
            .and_then(|a| a.path.clone())
            .unwrap_or_default();
        let still = project
            .composition(&comp)
            .unwrap()
            .layer(&sound)
            .map(|l| l.asset_id == Id::new("asset-sound"))
            .unwrap_or(false);
        t.row(
            "Relinking the sound to another file changes the record and leaves the layer on it",
            &format!(
                "{}; path {now}; the layer still names it: {still}",
                refusal(relinked)
            ),
            now == "media/pcm8_mono_8k.wav" && still,
        );

        let dest = std::env::temp_dir().join("anime_compositor_b20b_collect");
        let _ = fs::remove_dir_all(&dest);
        let said = anime_compositor::package::collect(
            document.project(),
            &loaded.preserved,
            Some(&root.join("fx_aud_020.json")),
            &dest,
        );
        let mut wavs = Vec::new();
        let mut stack = vec![dest.clone()];
        while let Some(dir) = stack.pop() {
            for entry in fs::read_dir(&dir).into_iter().flatten().flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.extension().is_some_and(|x| x == "wav") {
                    let same = fs::read(&path).ok() == fs::read(root.join(&now)).ok();
                    wavs.push(format!(
                        "{}, byte for byte the same: {same}",
                        path.file_name().unwrap().to_string_lossy()
                    ));
                }
            }
        }
        t.row(
            "Collect files copies the sound file into the package",
            &format!("{}; {}", refusal(said), wavs.join(", ")),
            wavs.len() == 1 && wavs[0].ends_with("true"),
        );
        let _ = fs::remove_dir_all(&dest);
    }

    let report = format!(
        "# B-20b: reference audio in the core (D-71, ADR-018)\n\n\
         Written by `cargo test --test b20b_audio`. Every expected value is from\n\
         `Fixtures/audio/expected_audio.json`, which `tools/audio_reference.py` wrote before this\n\
         code existed. The numbers are whole and the match is exact.\n\n\
         Nothing is heard yet: playback, the waveform and the bar styles are B-20c.\n\n\
         **{} of {} checks match.**\n{}",
        t.passed, t.checks, t.out
    );
    fs::write(repo("verification/B-20b_audio_table.md"), report).unwrap();
    assert_eq!(t.passed, t.checks, "see verification/B-20b_audio_table.md");
}
