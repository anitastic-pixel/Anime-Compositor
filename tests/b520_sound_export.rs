//! B-520, D-641 (S-3 of D-638): a composition's sound in its export. Each project in
//! `Fixtures/sound_export` is exported and held to `tools/sound_export_reference.py`'s numbers.
//!
//! An MP4's sound is AAC, which changes every value, so it is never compared value for value:
//! it is decoded by ffmpeg 8.1 and by Windows' own decoder (D-639's reader), and the loudest
//! sample near each click must be exactly the click's frame's first sample (the click method,
//! ADR-018). A WAV beside a sequence is compared value for value.
//!
//! Writes `verification/D-641_sound_export_table.md` and, for the owner to play,
//! `verification/D-641 film/flash_and_click.mp4`.
#![cfg(windows)]

mod effect_table;

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;

use anime_compositor::audio;
use anime_compositor::export::{export_sequence, ExportChoices, ExportReport, ExportRequest, MissingSource, OutputFormat};
use anime_compositor::exr_io::ExrSamples;
use anime_compositor::model::Project;
use anime_compositor::{persist, OutputAlpha, OutputDepth};
use effect_table::{repo, Table};
use serde_json::Value as J;

const FULL: f64 = 8_388_607.0;

fn export(project: &Project, root: &Path, into: &Path, naming: &str, format: OutputFormat, (a, b): (i32, i32), wav: bool) -> ExportReport {
    let _ = std::fs::remove_dir_all(into);
    std::fs::create_dir_all(into).unwrap();
    let request = ExportRequest {
        composition: project.compositions[0].id.clone(),
        first_frame: a,
        last_frame: b,
        output_dir: into.to_path_buf(),
        naming: naming.to_string(),
        depth: OutputDepth::Eight,
        alpha: OutputAlpha::Straight,
        tile_size: anime_compositor::compose::DEFAULT_TILE_SIZE,
        missing: MissingSource::Block,
        format,
        choices: ExportChoices { wav, ..Default::default() },
    };
    export_sequence(project, root, &request, &AtomicBool::new(false))
}

/// The first sound track decoded by ffmpeg to 48,000 a second, left channel.
fn ffmpeg_left(file: &Path) -> Vec<f32> {
    let out = Command::new("ffmpeg")
        .args(["-v", "error", "-i"])
        .arg(file)
        .args(["-map", "0:a:0", "-f", "f32le", "-ac", "2", "-ar", "48000", "-"])
        .output()
        .expect("ffmpeg 8.1 on the PATH");
    out.stdout.chunks_exact(8).map(|c| f32::from_le_bytes(c[..4].try_into().unwrap())).collect()
}

fn probe(file: &Path) -> String {
    let out = Command::new("ffprobe")
        .args(["-v", "error", "-select_streams", "a", "-show_entries", "stream=codec_name,sample_rate,channels,bit_rate", "-of", "csv=p=0"])
        .arg(file)
        .output()
        .expect("ffprobe");
    String::from_utf8_lossy(&out.stdout).trim().to_string()
}

/// Where the loudest sample is within `reach` of `at`, and how loud.
fn loudest(x: &[f32], at: usize, reach: usize) -> (usize, f32) {
    (at.saturating_sub(reach)..(at + reach).min(x.len()))
        .map(|i| (i, x[i].abs()))
        .fold((0, -1.0), |m, v| if v.1 > m.1 { v } else { m })
}

fn ids(report: &ExportReport) -> Vec<&'static str> {
    report.diagnostics.iter().map(|d| d.id.as_str()).collect()
}

fn tracks(file: &Path) -> (usize, usize) {
    let bytes = std::fs::read(file).unwrap();
    let count = |kind: &[u8]| bytes.windows(4).enumerate().filter(|(i, w)| *w == b"hdlr" && bytes.get(i + 12..i + 16) == Some(kind)).count();
    (count(b"vide"), count(b"soun"))
}

#[test]
fn b520_sound_export() {
    let mut t = Table::new(
        "sound_export",
        "# D-641: a composition's sound in its export (B-520)\n\nEach project is exported and held \
         to the numbers `tools/sound_export_reference.py` wrote before the code. An MP4's sound is \
         AAC (48,000 a second, two channels, 192 kilobits a second, Windows' encoder), which changes \
         every value, so it is checked by the click method only: each case starts a layer holding \
         one click on chosen frames, and the loudest sample near each click, decoded by ffmpeg 8.1 \
         and by Windows' own decoder, must be exactly that frame's first sample, floor(f x 48000 x \
         den / num), counted from the first frame exported. A WAV beside a sequence is 24-bit and \
         compared value for value (tolerance 2e-5 of full level). The **click table** is at the \
         bottom.\n",
    );
    let expected: J = serde_json::from_str(&std::fs::read_to_string(t.root.join("expected_sound_export.json")).unwrap()).unwrap();
    let tol = expected["tolerance"].as_f64().unwrap();
    let tmp = std::env::temp_dir().join("anime_compositor_b520");
    let mut click_rows = String::new();

    for case in expected["films"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        t.heading(&format!("{name}: {}", case["what"].as_str().unwrap()));
        let loaded = t.load(file);
        let project = loaded.document.project();
        let range = (case["first_frame"].as_i64().unwrap() as i32, case["last_frame"].as_i64().unwrap() as i32);
        let into = tmp.join(name);
        let report = export(project, &t.root, &into, "film.mp4", OutputFormat::Mp4, range, false);
        let film = into.join("film.mp4");
        t.row("the MP4 is written", &format!("{:?}, {} file", report.status, report.written.len()), report.succeeded());
        let warned: Vec<&str> = case["warnings"].as_array().unwrap().iter().map(|w| w.as_str().unwrap()).collect();
        t.row("its warnings", &format!("{:?}", ids(&report)), ids(&report) == warned);
        t.row("one picture track and one sound track", &format!("{:?}", tracks(&film)), tracks(&film) == (1, 1));
        let probed = probe(&film);
        let parts: Vec<&str> = probed.split(',').collect();
        let bits: f64 = parts.get(3).and_then(|b| b.parse().ok()).unwrap_or(0.0);
        // The encoder is held to 192 kilobits; ffprobe reports what the file spends, which is that
        // number within a few per cent.
        t.row(
            "AAC, 48,000 a second, two channels, about 192 kilobits a second (ffprobe)",
            &probed,
            parts.first() == Some(&"aac") && parts.get(1) == Some(&"48000") && parts.get(2) == Some(&"2") && (bits - 192_000.0).abs() < 192_000.0 * 0.05,
        );
        let samples = case["samples"].as_u64().unwrap() as usize;
        let by_ffmpeg = ffmpeg_left(&film);
        let windows = audio::read_file(&film).map(|s| s.working()[0].iter().map(|v| *v as f32).collect::<Vec<f32>>());
        let windows = windows.unwrap_or_else(|d| panic!("{}: {}", d.message, d.detail));
        t.row(
            "length at 48,000 a second: Windows' decoder (by the file's edit list) / ffmpeg (at least)",
            &format!("{} / {}", windows.len(), by_ffmpeg.len()),
            windows.len() == samples && by_ffmpeg.len() >= samples,
        );
        for click in case["clicks"].as_array().unwrap() {
            let (f, at) = (click["frame"].as_i64().unwrap(), click["sample"].as_u64().unwrap() as usize);
            let (ff, ffv) = loudest(&by_ffmpeg, at, 1000);
            let (wi, wiv) = loudest(&windows, at, 1000);
            t.row(&format!("the click on frame {f} lands on sample {at} (ffmpeg / Windows)"), &format!("{ff} / {wi}"), ff == at && wi == at);
            click_rows.push_str(&format!(
                "| {name} | {f} | {at} | {ff} ({ffv:.2}) | {wi} ({wiv:.2}) | {} |\n",
                if ff == at && wi == at { "yes" } else { "**NO**" }
            ));
        }
    }

    // ---- no sound layer: the film is what it always was ---------------------------------------
    let case = &expected["silent"];
    t.heading(&format!("{}: {}", case["project"].as_str().unwrap().trim_end_matches(".json"), case["what"].as_str().unwrap()));
    let loaded = t.load(case["project"].as_str().unwrap());
    let into = tmp.join("silent");
    let report = export(loaded.document.project(), &t.root, &into, "film.mp4", OutputFormat::Mp4, (0, 11), false);
    t.row("written, with no warning", &format!("{:?}, {:?}", report.status, ids(&report)), report.succeeded() && report.diagnostics.is_empty());
    let film = into.join("film.mp4");
    t.row("picture tracks, sound tracks", &format!("{:?}", tracks(&film)), tracks(&film) == (1, 0));

    // ---- the WAV beside ------------------------------------------------------------------------
    for case in expected["wavs"].as_array().unwrap() {
        let file = case["project"].as_str().unwrap();
        let name = file.trim_end_matches(".json");
        t.heading(&format!("{name}: {}", case["what"].as_str().unwrap()));
        let loaded = t.load(file);
        let format = match case["format"].as_str().unwrap() {
            "png" => OutputFormat::Png,
            "apng" => OutputFormat::Apng,
            _ => OutputFormat::Exr(ExrSamples::Half),
        };
        let range = (case["first_frame"].as_i64().unwrap() as i32, case["last_frame"].as_i64().unwrap() as i32);
        let into = tmp.join(name);
        let report = export(loaded.document.project(), &t.root, &into, case["naming"].as_str().unwrap(), format, range, true);
        let wav = into.join(case["file"].as_str().unwrap());
        t.row(
            &format!("written, the WAV among the files, as {}", case["file"].as_str().unwrap()),
            &format!("{:?}, {} of {} files, WAV there: {}", report.status, report.written.len(), report.files_expected, report.written.contains(&wav)),
            report.succeeded() && report.written.contains(&wav),
        );
        let past = case["past_full"].as_u64().unwrap();
        let clipped: Vec<&str> = ids(&report).into_iter().filter(|i| *i == "EXPORT_SOUND_CLIPPED").collect();
        let said = report.diagnostics.iter().find(|d| d.id.as_str() == "EXPORT_SOUND_CLIPPED").map_or(String::new(), |d| d.message.clone());
        t.row(
            &format!("values past full level: {past}, and a warning only then"),
            &format!("{} EXPORT_SOUND_CLIPPED warning(s): {said}", clipped.len()),
            if past > 0 { clipped.len() == 1 && said.contains(&format!(" {past} places")) } else { clipped.is_empty() },
        );
        let bytes = std::fs::read(&wav).unwrap_or_default();
        let u16_at = |i: usize| u16::from_le_bytes(bytes[i..i + 2].try_into().unwrap());
        let u32_at = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
        let head = format!("{} {} {} {}", u16_at(20), u16_at(22), u32_at(24), u16_at(34));
        t.row("format, channels, rate, bits", &head, head == "1 2 48000 24" && &bytes[..4] == b"RIFF" && &bytes[36..40] == b"data");
        let n = u32_at(40) as usize / 6;
        t.row("length in samples", &n.to_string(), n == case["samples"].as_u64().unwrap() as usize);
        let value = |i: usize, c: usize| {
            let at = 44 + i * 6 + c * 3;
            (i32::from_le_bytes([0, bytes[at], bytes[at + 1], bytes[at + 2]]) >> 8) as f64
        };
        let (mut worst, mut exact) = (0f64, 0usize);
        let places = case["places"].as_array().unwrap();
        for (p, want) in places.iter().zip(case["values"].as_array().unwrap()) {
            for c in 0..2 {
                let (got, want) = (value(p.as_u64().unwrap() as usize, c), want[c].as_f64().unwrap());
                worst = worst.max((got - want).abs());
                exact += (got == want) as usize;
            }
        }
        t.row(
            &format!("the {} places in both channels, within {tol} of full level", places.len()),
            &format!("largest difference {worst} steps of 8,388,607; {exact} of {} exact", places.len() * 2),
            worst / FULL <= tol,
        );
    }

    // ---- a GIF writes no WAV -------------------------------------------------------------------
    let case = &expected["gif"];
    t.heading(&format!("{}: {}", case["project"].as_str().unwrap().trim_end_matches(".json"), case["what"].as_str().unwrap()));
    let loaded = t.load(case["project"].as_str().unwrap());
    let into = tmp.join("gif");
    let report = export(loaded.document.project(), &t.root, &into, "preview.gif", OutputFormat::Gif, (0, 5), true);
    let names: Vec<String> = std::fs::read_dir(&into).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    t.row("the folder holds the GIF and nothing else", &format!("{:?}, {names:?}", report.status), report.succeeded() && names == ["preview.gif"]);

    // ---- the owner's film: a white flash and a click on the same frames ------------------------
    t.heading("The owner's film: `verification/D-641 film/flash_and_click.mp4`");
    let (project, flashes) = flash_and_click();
    let into = repo("verification/D-641 film");
    let report = export(&project, &t.root, &into, "flash_and_click.mp4", OutputFormat::Mp4, (0, 71), false);
    let film = into.join("flash_and_click.mp4");
    t.row("72 frames at 24 a second, with its sound", &format!("{:?}, tracks {:?}", report.status, tracks(&film)), report.succeeded() && tracks(&film) == (1, 1));
    let by_ffmpeg = ffmpeg_left(&film);
    for f in flashes {
        let at = f as usize * 2000;
        let (ff, ffv) = loudest(&by_ffmpeg, at, 1000);
        t.row(&format!("the click with the flash on frame {f} lands on sample {at}"), &ff.to_string(), ff == at);
        click_rows.push_str(&format!("| flash_and_click | {f} | {at} | {ff} ({ffv:.2}) | | {} |\n", if ff == at { "yes" } else { "**NO**" }));
    }

    let mut out = String::new();
    out.push_str("\n## The click table\n\nEach click's frame, the sample it must land on, and the loudest sample \
                  found there (with its height) by ffmpeg 8.1 and by Windows' own decoder.\n\n\
                  | Case | Frame | Must land on | ffmpeg | Windows | Lands |\n| --- | --- | --- | --- | --- | --- |\n");
    out.push_str(&click_rows);
    t.heading("Every click");
    t.row("every click lands", "see below", !click_rows.contains("**NO**"));
    t.finish("D-641_sound_export_table.md");
    let file = repo("verification/D-641_sound_export_table.md");
    let mut table = std::fs::read_to_string(&file).unwrap();
    table.push_str(&out);
    std::fs::write(&file, table).unwrap();
}

/// 72 frames, 320 x 180 at 24 a second: black, a white frame every half second (frames 12, 24,
/// 36, 48, 60), and `click.wav` started on each of those frames, so each flash has its click.
fn flash_and_click() -> (Project, Vec<i32>) {
    let flashes = vec![12, 24, 36, 48, 60];
    let mut layers = Vec::new();
    for &f in &flashes {
        layers.push(serde_json::json!({"id": format!("flash{f}"), "kind": "solid", "name": format!("flash {f}"),
            "enabled": true, "locked": false, "in_frame": f, "out_frame": f + 1,
            "transform": {"anchor": {"base": [160, 90], "keyframes": []}, "position": {"base": [160, 90], "keyframes": []},
                "scale": {"base": [100, 100], "keyframes": []}, "rotation": {"base": 0, "keyframes": []},
                "opacity": {"base": 1, "keyframes": []}},
            "mask": null, "matte": null, "blend_mode": "normal", "effects": [],
            "solid": {"color": [1.0, 1.0, 1.0], "width": 320, "height": 180}}));
        layers.push(serde_json::json!({"id": format!("click{f}"), "kind": "audio", "name": format!("click {f}"),
            "asset_id": "click", "enabled": true, "locked": false, "in_frame": f, "out_frame": 72,
            "source_offset_frames": 0, "gain_db": 0}));
    }
    let doc = serde_json::json!({
        "schema_version": 0, "project_id": "proj-flash-and-click",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"},
        "assets": [{"id": "click", "kind": "audio", "name": "click", "path": "media/click.wav"}],
        "compositions": [{"id": "comp-main", "name": "Flash and click", "width": 320, "height": 180,
            "pixel_aspect_ratio": 1, "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0,
            "duration_frames": 72, "work_area": {"start_frame": 0, "end_frame_exclusive": 72},
            "layer_order": layers.iter().map(|l| l["id"].clone()).collect::<Vec<_>>(), "layers": layers}],
    });
    let file: PathBuf = std::env::temp_dir().join("anime_compositor_b520_flash.json");
    std::fs::write(&file, serde_json::to_string_pretty(&doc).unwrap()).unwrap();
    let loaded = persist::load(&file).unwrap_or_else(|d| panic!("the flash project opens: {}", d.message));
    (loaded.document.project().clone(), flashes)
}
