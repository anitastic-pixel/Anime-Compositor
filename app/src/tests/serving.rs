use super::*;
use anime_compositor::command::Command;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// One header as text, or the empty string if it is not there at all. A header the page cannot
/// read is the same to the page as a header that was never sent, so both read as absent here.
fn header(response: &Response<Vec<u8>>, name: &str) -> String {
    response.headers().get(name).map_or_else(String::new, |v| {
        v.to_str().unwrap_or("<not readable as text>").to_string()
    })
}

/// Percent-decoding, written out here rather than borrowed from the window.
///
/// The window encodes; this decodes. Using the window's own `from_a_query` would check one
/// function against itself and would pass just as happily if both halves were wrong together.
fn decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut at = 0;
    while at < bytes.len() {
        if bytes[at] == b'%' && at + 3 <= bytes.len() {
            let hex = std::str::from_utf8(&bytes[at + 1..at + 3]).expect("ascii");
            out.push(u8::from_str_radix(hex, 16).expect("two hex digits"));
            at += 3;
        } else {
            out.push(bytes[at]);
            at += 1;
        }
    }
    String::from_utf8(out).expect("what went in was utf-8")
}

/// A project of sixteen pixels whose answer is known before anything renders it.
///
/// Every pixel in the reference shot is either opaque or empty, and those two look the same
/// whichever way alpha is carried — so the shot cannot tell straight alpha and premultiplied
/// apart, and the page draws straight. This cel is four pixels of half-transparent red, which
/// can: carried straight it stays 255 red, carried premultiplied it comes back at about 188.
fn a_half_transparent_red_project() -> PathBuf {
    let directory = crate::test_temp().join("anime_compositor_b08_shell");
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(directory.join("media")).expect("make the scratch directory");
    let samples: Vec<u8> = std::iter::repeat_n([255u8, 0, 0, 128], 16)
        .flatten()
        .collect();
    anime_compositor::png_out::write_rgba(
        &directory.join("media/cel_0001.png"),
        4,
        4,
        OutputDepth::Eight,
        &[],
        &samples,
    )
    .expect("write the cel");
    let project = directory.join("shot.json");
    std::fs::write(
        &project,
        r#"{
  "schema_version": 0,
  "project_id": "proj-one-pixel",
  "color_settings": { "working_space": "linear-srgb", "alpha_mode": "premultiplied" },
  "assets": [
    {
      "id": "asset-cel",
      "kind": "image_sequence",
      "name": "Cel",
      "pattern": "cel_####.png",
      "frames": { "1": "media/cel_0001.png" },
      "interpretation": { "color_space": "srgb", "alpha": "straight" }
    }
  ],
  "compositions": [
    {
      "id": "comp-main",
      "name": "Main",
      "width": 4,
      "height": 4,
      "pixel_aspect_ratio": 1,
      "frame_rate": { "numerator": 24, "denominator": 1 },
      "start_frame": 0,
      "duration_frames": 1,
      "work_area": { "start_frame": 0, "end_frame_exclusive": 1 },
      "layer_order": ["layer-cel"],
      "layers": [
        {
          "id": "layer-cel",
          "kind": "raster",
          "name": "Cel",
          "asset_id": "asset-cel",
          "enabled": true,
          "locked": false,
          "in_frame": 0,
          "out_frame": 1,
          "source_offset_frames": 0,
          "transform": {
            "anchor": { "base": [0, 0], "keyframes": [] },
            "position": { "base": [0, 0], "keyframes": [] },
            "scale": { "base": [100, 100], "keyframes": [] },
            "rotation": { "base": 0, "keyframes": [] },
            "opacity": { "base": 1, "keyframes": [] }
          },
          "exposure_spans": [
            { "start_frame": 0, "end_frame_exclusive": 1, "drawing_number": 1 }
          ],
          "mask": null,
          "matte": null,
          "blend_mode": "normal",
          "effects": []
        }
      ]
    }
  ]
}
"#,
    )
    .expect("write the project");
    project
}

/// Ask the transport for something, the way the request handler does.
fn ask(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    path: &str,
    query: Option<&str>,
) -> Response<Vec<u8>> {
    let (ask, quality) = parse(path, query).expect("the test asks for something readable");
    serve(viewer, export, ask, quality)
}

/// One document 24 command, by the identifier the page would send.
fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = match what.split_once('?') {
        Some((id, query)) => (id, Some(query)),
        None => (what, None),
    };
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
    viewer.lock().expect("the viewer lock was poisoned")
}

/// A directory of this test's own, emptied first so a previous run cannot make a later one
/// pass.
fn scratch(name: &str) -> PathBuf {
    let directory = crate::test_temp().join(name);
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the scratch directory");
    directory
}

/// The graph editor asks for the camera by the same word the commands use, and a lens comes
/// back in the millimetres the inspector shows it in.
#[test]
fn the_graph_editor_can_ask_for_the_cameras_curve() {
    let viewer = Mutex::new(demo());
    let samples = |query: &str| -> Vec<f64> {
        let body = curve(&viewer, Some(query)).into_body();
        let said: serde_json::Value = serde_json::from_slice(&body).expect("json");
        said["samples"]
            .as_array()
            .expect("samples")
            .iter()
            .map(|s| s[0].as_f64().expect("a number"))
            .collect()
    };
    let lens = samples("target=camera&prop=zoom&from=0&to=24");
    assert_eq!(lens.len(), 25);
    assert!(
        (lens[0] - 50.0).abs() < 1e-9,
        "an unmoved camera is a 50 mm lens: {}",
        lens[0]
    );

    let width = {
        let viewer = held(&viewer);
        viewer
            .document
            .project()
            .composition(&viewer.composition)
            .expect("a composition")
            .width
    };
    run(
        &viewer,
        "keyframe.add_remove?target=camera&prop=zoom&frame=0",
    );
    run(
        &viewer,
        &format!("property.set_base?target=camera&prop=zoom&value={width}&frame=24"),
    );
    let lens = samples("target=camera&prop=zoom&from=0&to=24");
    assert!(
        (lens[0] - 50.0).abs() < 1e-9 && (lens[24] - 36.0).abs() < 1e-9,
        "{lens:?}"
    );
    assert!(
        lens[12] < 50.0 && lens[12] > 36.0,
        "the curve goes between its keys: {lens:?}"
    );

    assert!(samples("target=camera&prop=opacity&from=0&to=24").is_empty());
}

#[test]
fn what_the_page_gets_is_the_frame_it_asked_for() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
        rows.push((what.to_string(), expected.to_string(), actual.to_string()));
    };

    let viewer = Mutex::new(demo());
    let export = Mutex::new(Export::default());

    // ---- one frame, asked for by number ------------------------------------------------------
    // The reference shot is 240 frames of 1920 by 1080 at 24 frames a second, starting at 0,
    // and D-33 makes the preview open at draft resolution, which DRAFT_DIVISOR puts at a
    // quarter of each side: 480 by 270.
    let got = ask(&viewer, &export, "/frame/0", None);
    check(
        "asking for frame 0 is answered",
        &200,
        &got.status().as_u16(),
    );
    check(
        "with raw pixels rather than an image file, which is what the page draws",
        &"application/octet-stream",
        &header(&got, "content-type"),
    );
    check(
        "and the answer says which frame it is",
        &"0",
        &header(&got, "x-frame"),
    );
    check(
        "at draft resolution, which is what the preview opens at",
        &"Draft",
        &header(&got, "x-quality"),
    );
    check(
        "and says so, so nobody mistakes it for what an export would write",
        &"true",
        &header(&got, "x-differs"),
    );
    check("480 wide", &"480", &header(&got, "x-width"));
    check("270 high", &"270", &header(&got, "x-height"));
    check(
        "and the picture is exactly that many pixels, four bytes each",
        &(480 * 270 * 4),
        &got.body().len(),
    );
    // Without both of these the page fetches the answer successfully and is then refused it:
    // the picture fails and every header reads as absent. Either one alone is silence.
    check(
        "the page is allowed to read the answer",
        &"*",
        &header(&got, "access-control-allow-origin"),
    );
    check(
        "and allowed to read the headers beside it, not only receive them",
        &"*",
        &header(&got, "access-control-expose-headers"),
    );
    check(
        "a frame asked for by number did not come from the clock, so there is no playback \
         report beside it",
        &"",
        &header(&got, "x-report"),
    );

    // ---- stepping stops at the ends of the work area ------------------------------------------
    let got = ask(&viewer, &export, "/frame/999", None);
    check(
        "stepping past the end of the shot stops at the last frame",
        &"239",
        &header(&got, "x-frame"),
    );
    let got = ask(&viewer, &export, "/frame/-5", None);
    check(
        "and stepping before the beginning stops at the first",
        &"0",
        &header(&got, "x-frame"),
    );

    // ---- and asked for by the clock ----------------------------------------------------------
    let got = ask(&viewer, &export, "/at/0", None);
    check(
        "the first frame of a playback is the one at the start of the shot",
        &"0",
        &header(&got, "x-frame"),
    );
    check(
        "and this time the playback report is beside it",
        &"Played 1 frames in real time. No frames were dropped.",
        &header(&got, "x-report"),
    );
    // One second at 24 frames a second is frame 24. The 23 in between were never asked for,
    // and D-32 requires that to be said rather than hidden.
    let got = ask(&viewer, &export, "/at/1000", None);
    check(
        "a second into the shot the clock asks for frame 24, not the next one along",
        &"24",
        &header(&got, "x-frame"),
    );
    check(
        "and says the 23 frames in between were passed over",
        &"23",
        &header(&got, "x-skipped"),
    );
    // A page whose clock jumps backwards is a page with a bug, and the answer to that is to
    // hold still, not to invent a negative skip count and put it in front of the owner.
    let got = ask(&viewer, &export, "/at/500", None);
    check(
        "time running backwards holds the frame rather than rewinding",
        &"24",
        &header(&got, "x-frame"),
    );
    check(
        "and counts nothing as skipped",
        &"0",
        &header(&got, "x-skipped"),
    );

    // ---- the resolution the page asks for -----------------------------------------------------
    let got = ask(&viewer, &export, "/frame/0", Some("q=full"));
    check(
        "asking for full resolution gets it",
        &"Full",
        &header(&got, "x-quality"),
    );
    check("at the shot's own size", &"1920", &header(&got, "x-width"));
    check(
        "and full resolution is what an export would write, so the warning goes away",
        &"false",
        &header(&got, "x-differs"),
    );
    // A typo in a query string must not quietly change what the person is looking at.
    let got = ask(&viewer, &export, "/frame/0", Some("q=sideways"));
    check(
        "a resolution nobody recognises changes nothing rather than guessing",
        &"Full",
        &header(&got, "x-quality"),
    );
    let got = ask(&viewer, &export, "/frame/0", Some("q=draft"));
    check(
        "and asking for draft again gets draft",
        &"Draft",
        &header(&got, "x-quality"),
    );

    // ---- what the window has to say about the project ------------------------------------------
    {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        held.name = "背景_日本語.json".to_string();
        held.notes = vec!["One note.".to_string(), "And another.".to_string()];
        held.status = "Saved to 背景_日本語.json".to_string();
    }
    let got = ask(&viewer, &export, "/frame/0", None);
    check(
        "a Japanese project name arrives at the page as the name it started as",
        &"背景_日本語.json",
        &decode(&header(&got, "x-project")),
    );
    check(
        "and so does a sentence with Japanese in it",
        &"Saved to 背景_日本語.json",
        &decode(&header(&got, "x-status")),
    );
    check(
        "the notes travel whole, one to a line",
        &"One note.\nAnd another.",
        &decode(&header(&got, "x-notes")),
    );
    check(
        "nothing is being exported, and the page is told so",
        &"false",
        &header(&got, "x-exporting"),
    );
    check(
        "there is nothing outstanding in the project yet",
        &"false",
        &header(&got, "x-dirty"),
    );
    {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        let composition = held.composition.clone();
        let layer_id = held.document.project().compositions[0].layer_order()[0].clone();
        held.document
            .apply(Command::RenameLayer {
                composition,
                layer_id,
                name: "Renamed while the page watched".to_string(),
            })
            .expect("rename the layer");
    }
    let got = ask(&viewer, &export, "/frame/0", None);
    check(
        "and once something is changed the page is told that too",
        &"true",
        &header(&got, "x-dirty"),
    );

    // ---- a request the window does not understand ------------------------------------------------
    check(
        "a path the window does not understand is not answered with a guess",
        &"nothing",
        &parse("/frames/3", None).map_or("nothing", |_| "something"),
    );
    check(
        "and neither is a time that is not a number",
        &"nothing",
        &parse("/at/soon", None).map_or("nothing", |_| "something"),
    );

    // ---- a frame that cannot be made --------------------------------------------------------------
    // Document 28: a frame that cannot be drawn is reported in words, never replaced by
    // something that looks like a frame.
    let broken = Mutex::new(
        open(&repo("Fixtures/projects/missing_media_project.json"))
            .expect("open the project whose drawing is missing"),
    );
    let got = ask(&broken, &export, "/frame/0", None);
    check(
        "a missing drawing does not take the rest of the shot down with it: the frame still \
         comes back",
        &200,
        &got.status().as_u16(),
    );
    check(
        "and it is honestly empty rather than filled in with a guess",
        &"every pixel transparent",
        &if got.body().iter().all(|byte| *byte == 0) {
            "every pixel transparent"
        } else {
            "something was drawn"
        },
    );
    check(
        "and the page is carrying the sentence that explains why it is empty",
        &"2 of the files for \"Cel\" are not where the project expects them. The reference is \
          kept as it is. Relink the asset to point it at the files, or put them back. Frames \
          that cannot be found render as nothing rather than as a guess.",
        &decode(&header(&got, "x-notes"))
            .split('\t')
            .next()
            .unwrap_or(""),
    );
    check(
        "which the page can only read because the refusal carries the same permission the \
         picture does",
        &"*",
        &header(&got, "access-control-expose-headers"),
    );

    // ---- the pixels themselves ------------------------------------------------------------------
    // Everything above counts the bytes without reading one. The page draws these straight
    // into an ImageData, which is straight alpha and nothing else, so a picture handed over
    // premultiplied would be the right size, the right frame and the wrong colour.
    let known =
        Mutex::new(open(&a_half_transparent_red_project()).expect("open the four-pixel project"));
    let got = ask(&known, &export, "/frame/0", Some("q=full"));
    check(
        "a four by four picture is sixty-four bytes",
        &64,
        &got.body().len(),
    );
    check(
        "and a half-transparent red one comes back as the red it was drawn as, not the \
         darker red premultiplying it would give",
        &"255, 0, 0, 128",
        &got.body()[..4]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    check(
        "and every pixel of it is that same red",
        &true,
        &got.body().chunks_exact(4).all(|px| *px == got.body()[..4]),
    );

    write_artifact(&rows);
    let failed: Vec<&(String, String, String)> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, see verification/B-08_shell_table.md: {:#?}",
        failed.len(),
        rows.len(),
        failed
    );
}

/// B-44: the CPU / GPU switch, through the same function every frame on screen comes from.
///
/// Writes `verification/B-44_gpu_window_table.md`, or says NOT RUN when there is no card.
#[test]
fn the_gpu_switch_draws_the_same_picture_and_says_where() {
    let out = repo("verification/B-44_gpu_window_table.md");
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let viewer = Mutex::new(demo());
    let export = Mutex::new(Export::default());
    let card = Mutex::new(Card::default());
    let frame =
        |ask: Ask, quality| serve_logged(&viewer, &export, None, Some(&card), ask, Some(quality));

    let before = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "the switch starts on the CPU".into(),
        "CPU".into(),
        header(&before, "x-drawn-on"),
    ));
    let said = card.lock().unwrap().set(DrawOn::Gpu);
    if !card.lock().unwrap().on {
        std::fs::write(
            &out,
            format!(
                "# B-44: the switch in the window

**NOT RUN.** {said}
"
            ),
        )
        .expect("write the table");
        return;
    }
    rows.push((
        "pressing it says which card".into(),
        "names the card".into(),
        if said.contains(&card.lock().unwrap().about()) {
            "names the card".into()
        } else {
            said.clone()
        },
    ));
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        card.lock().unwrap().on = false;
        let cpu = frame(Ask::Frame(100), quality);
        card.lock().unwrap().on = true;
        let gpu = frame(Ask::Frame(100), quality);
        rows.push((
            format!(
                "frame 100 at {} says it was drawn on the GPU",
                quality.label()
            ),
            "GPU".into(),
            header(&gpu, "x-drawn-on"),
        ));
        let largest = cpu
            .body()
            .iter()
            .zip(gpu.body())
            .map(|(a, b)| a.abs_diff(*b))
            .max();
        rows.push((
            "and is within 1 level of the CPU's picture, at the same size".to_string(),
            "at most 1, 1920×1080 or its draft".into(),
            match (cpu.body().len() == gpu.body().len(), largest) {
                (true, Some(l)) if l <= 1 => "at most 1, 1920×1080 or its draft".into(),
                _ => format!(
                    "largest {largest:?}, {} against {} bytes",
                    gpu.body().len(),
                    cpu.body().len()
                ),
            },
        ));
    }
    // Audit W8: an effect the card does not draw runs on the processor, and so does every
    // effect before it; the viewer's note names them. A stack the card draws whole has none.
    let add = |id: &str, effect: Effect, index: Option<usize>| Command::AddEffect {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-2"),
        effect: EffectInstance::new(Id::new(id), effect),
        index,
    };
    let note = |r: &Response<Vec<u8>>| match header(r, "x-partly") {
        n if n.is_empty() => "no note".to_string(),
        n => format!("partly on the processor ({n})"),
    };
    edit(
        &viewer,
        add(
            "w8-blur",
            Effect::GaussianBlur {
                sigma_px: 4.0,
                edges: "transparent".into(),
                dimensions: "both".into(),
                units: "sigma".into(),
            },
            None,
        ),
    );
    let blurred = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "a Gaussian Blur on layer 2, which the card draws: frame 100 is drawn on the GPU".into(),
        "GPU".into(),
        header(&blurred, "x-drawn-on"),
    ));
    rows.push((
        "and the viewer has no processor note beside it".into(),
        "no note".into(),
        note(&blurred),
    ));
    edit(
        &viewer,
        add("w8-echo", new_effect(ECHO).expect("an Echo"), Some(0)),
    );
    let echoed = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "an Echo put before the blur, which only the processor draws: still drawn on the GPU"
            .into(),
        "GPU".into(),
        header(&echoed, "x-drawn-on"),
    ));
    rows.push((
        "and the viewer's note names it".into(),
        "partly on the processor (Echo)".into(),
        note(&echoed),
    ));
    undo(&viewer);
    // Audit W9: in 8 bpc the card runs none of a layer's effects; the note names them.
    let comp = viewer
        .lock()
        .unwrap()
        .document
        .project()
        .composition(&Id::new("comp-reference-shot"))
        .unwrap()
        .clone();
    edit(
        &viewer,
        Command::SetCompositionSettings {
            composition: comp.id.clone(),
            name: comp.name.clone(),
            width: comp.width,
            height: comp.height,
            frame_rate: comp.frame_rate,
            duration_frames: comp.duration_frames,
            sheet_details: comp.sheet_details.clone(),
            background_color: comp.background_color,
            float_depth: false,
            eight_bpc: true,
            ae_32bpc: false,
        },
    );
    let eight = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "the blur in an 8 bpc composition, whose layers D-763 lays together on the CPU only: the CPU draws it and says so"
            .into(),
        "CPU, not GPU".into(),
        header(&eight, "x-drawn-on"),
    ));
    undo(&viewer);
    undo(&viewer);
    let said = card.lock().unwrap().set(DrawOn::Cpu);
    let after = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "choosing CPU goes back to the CPU".into(),
        "CPU".into(),
        header(&after, "x-drawn-on"),
    ));
    rows.push((
        "and says so".into(),
        "The preview is drawn on the CPU.".into(),
        said,
    ));
    // B-48, D-105: Auto draws on the card, and gives it up once, and says so, when it fails.
    let said = card.lock().unwrap().set(DrawOn::Auto);
    rows.push((
        "choosing Auto says the card is used whenever it can be".into(),
        "yes".into(),
        if said.contains("whenever it can be") {
            "yes".into()
        } else {
            said.clone()
        },
    ));
    let auto = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "and the next frame is drawn on the GPU".into(),
        "GPU".into(),
        header(&auto, "x-drawn-on"),
    ));
    let failures = card.lock().unwrap().failures();
    rows.push((
        "a frame the card did not fail leaves Auto on the card".into(),
        "no sentence".into(),
        card.lock()
            .unwrap()
            .give_up_if_failed(failures)
            .unwrap_or("no sentence".into()),
    ));
    let said = card
        .lock()
        .unwrap()
        .give_up_if_failed(failures.wrapping_sub(1));
    rows.push((
        "a frame the card failed makes Auto give it up, and say so".into(),
        "says so".into(),
        match said {
            Some(s) if s.starts_with("The graphics card failed, so Auto") => "says so".into(),
            other => format!("{other:?}"),
        },
    ));
    let given_up = frame(Ask::Frame(100), PreviewQuality::Full);
    rows.push((
        "and the next frame is drawn on the CPU".into(),
        "CPU".into(),
        header(&given_up, "x-drawn-on"),
    ));
    rows.push((
        "the tooltip says Auto gave the card up".into(),
        "yes".into(),
        if card
            .lock()
            .unwrap()
            .about()
            .ends_with("Auto gave it up after it failed")
        {
            "yes".into()
        } else {
            card.lock().unwrap().about()
        },
    ));
    rows.push((
        "once, not on every frame".into(),
        "no sentence".into(),
        card.lock()
            .unwrap()
            .give_up_if_failed(failures.wrapping_sub(1))
            .unwrap_or("no sentence".into()),
    ));
    card.lock().unwrap().set(DrawOn::Cpu);
    rows.push((
        "the CPU's picture after the GPU is byte for byte the one before it".into(),
        "identical".into(),
        if after.body() == before.body() {
            "identical".into()
        } else {
            "different".into()
        },
    ));

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-44: the switch in the window

Written by `cargo test -p anime_compositor_app`. \
The reference shot, through `serve_logged`, the function every frame on screen comes \
from. Card: {}.

**{passed} of {} checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
",
        card.lock().unwrap().about(),
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text += &format!(
            "| {what} | {expected} | {actual} | {} |
",
            if expected == actual { "PASS" } else { "FAIL" }
        );
    }
    std::fs::write(&out, text).expect("write the table");
    assert_eq!(passed, rows.len(), "see {}", out.display());
}

/// B-12a item 7: the two ways of looking at a frame, and the promise that neither changes it.
///
/// Writes `verification/B-12a_inspect_table.md`.
#[test]
fn looking_at_the_alpha_channel_changes_nothing_but_the_looking() {
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: &dyn ToString, actual: &dyn ToString| {
        rows.push((what.to_string(), expected.to_string(), actual.to_string()));
    };
    let pixel = |body: &[u8]| {
        body[..4]
            .iter()
            .map(|b| b.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let project = a_half_transparent_red_project();
    let viewer = Mutex::new(open(&project).expect("open the four-pixel project"));
    let export = Mutex::new(Export::default());

    let colour = ask(&viewer, &export, "/frame/0", Some("q=full"));
    check(
        "the window opens looking at the picture itself",
        &"false",
        &header(&colour, "x-alpha"),
    );
    check(
        "half-transparent red arrives as the red it was drawn as",
        &"255, 0, 0, 128",
        &pixel(colour.body()),
    );
    check(
        "and the transparency grid starts on, because a transparent frame that reads as \
         black is a frame somebody misjudges",
        &"true",
        &header(&colour, "x-checkerboard"),
    );

    // ---- the alpha-only view -------------------------------------------------------------
    check(
        "turning alpha inspection on says what will be on screen and what will not change",
        &"Alpha-only inspection is on. The picture is the alpha channel, white where the \
          frame is opaque and black where it is empty; what is exported is unchanged.",
        &run(&viewer, "viewer.toggle_alpha"),
    );
    let alpha = ask(&viewer, &export, "/frame/0", Some("q=full"));
    check(
        "and the frame that comes back says it is being looked at that way",
        &"true",
        &header(&alpha, "x-alpha"),
    );
    check(
        "a pixel that is half transparent is drawn as the grey half way up",
        &"128, 128, 128, 255",
        &pixel(alpha.body()),
    );
    check(
        "the alpha view is opaque, so what is being measured cannot itself be see-through",
        &true,
        &alpha.body().chunks_exact(4).all(|px| px[3] == 255),
    );
    check(
        "it is the same picture, at the same size",
        &format!(
            "{}x{}",
            header(&colour, "x-width"),
            header(&colour, "x-height")
        ),
        &format!(
            "{}x{}",
            header(&alpha, "x-width"),
            header(&alpha, "x-height")
        ),
    );

    // Document 21 line 97: presentation must not alter cached final pixels. The frame was
    // just rendered and cached while the alpha view was on; asking for it again with the
    // view off is what proves the cache was given the picture and not the grey.
    check(
        "turning it off says so",
        &"Alpha-only inspection is off.",
        &run(&viewer, "viewer.toggle_alpha"),
    );
    let again = ask(&viewer, &export, "/frame/0", Some("q=full"));
    check(
        "and the frame comes back byte for byte as it was, so what the cache kept was the \
         picture and never the view",
        &true,
        &(again.body() == colour.body()),
    );

    // ---- neither is a change to the project -----------------------------------------------
    check(
        "looking at the alpha channel is not unsaved work",
        &"false",
        &header(&again, "x-dirty"),
    );
    check(
        "and it is not in the undo history either, which is what document 24's own table \
         says: undoable, no",
        &0,
        &held(&viewer).document.undo_depth(),
    );

    // ---- the grid ---------------------------------------------------------------------------
    check(
        "turning the grid off says what it was",
        &"The transparency grid is off.",
        &run(&viewer, "viewer.toggle_checkerboard"),
    );
    let plain = ask(&viewer, &export, "/frame/0", Some("q=full"));
    check(
        "the frame says the grid is off",
        &"false",
        &header(&plain, "x-checkerboard"),
    );
    check(
        "and not one byte of the frame is different, because the grid was never in it",
        &true,
        &(plain.body() == colour.body()),
    );
    check(
        "turning it back on says so too",
        &"The transparency grid is on. It is drawn behind the frame and is not part of it.",
        &run(&viewer, "viewer.toggle_checkerboard"),
    );

    // ---- R-10: inspection must not alter export ---------------------------------------------
    // The comparison is two exports of the same frame, one taken while the alpha view is on,
    // rather than a claim about what an export ought to contain. Byte for byte, or the view
    // reached the file.
    let ordinary = scratch("anime_compositor_b12a_export_plain");
    let (p, root, request) = export_job(
        &held(&viewer),
        &ordinary,
        MissingSource::Block,
        OutputFormat::Png,
    );
    run_export(&p, &root, &request, &AtomicBool::new(false));
    run(&viewer, "viewer.toggle_alpha");
    let inspecting = scratch("anime_compositor_b12a_export_alpha");
    let (p, root, request) = export_job(
        &held(&viewer),
        &inspecting,
        MissingSource::Block,
        OutputFormat::Png,
    );
    run_export(&p, &root, &request, &AtomicBool::new(false));
    let read = |dir: &Path| {
        let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
            .expect("read the export directory")
            .filter_map(|e| e.ok().map(|e| e.path()))
            .collect();
        files.sort();
        files
            .iter()
            .map(|f| std::fs::read(f).expect("read an exported frame"))
            .collect::<Vec<_>>()
    };
    let written = read(&ordinary);
    check(
        "an export writes the frames of the work area",
        &1,
        &written.len(),
    );
    check(
        "and exporting while looking at the alpha channel writes the same file, byte for \
         byte, which is what R-10 asks for",
        &true,
        &(read(&inspecting) == written),
    );
    run(&viewer, "viewer.toggle_alpha");

    write_inspect_artifact(&rows);
    let failed: Vec<&(String, String, String)> = rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, see verification/B-12a_inspect_table.md: {:#?}",
        failed.len(),
        rows.len(),
        failed
    );
}

fn write_inspect_artifact(rows: &[(String, String, String)]) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::new();
    out.push_str("# B-12a: looking at the alpha channel, and the grid behind the frame\n\n");
    out.push_str(
        "W-01 ends with the artist inspecting alpha, and document 05 line 31 lists the two \
         views this is about: the transparency grid, and the alpha-only display. Both are in \
         the viewer now, as document 24's `viewer.toggle_checkerboard` and \
         `viewer.toggle_alpha`. Produced by `cargo test -p anime_compositor_app`, from \
         `app/src/main.rs`.\n\n",
    );
    out.push_str(
        "The promise these rows are about is document 21 line 97's, which R-10 states as a \
         requirement: **checkerboard and alpha-only inspection must not alter export.** A \
         view that quietly leaked into the file, or into the frames the preview keeps, would \
         be found by whoever opened the exported sequence, long after the person who turned \
         it on had forgotten it was on. So the grid is drawn by the page behind a canvas that \
         already carries transparency and touches no pixel at all, and the alpha view is \
         applied to the copy of the frame that is on its way to the screen, after the cache \
         has been handed the picture it keeps.\n\n",
    );
    out.push_str(
        "The fixture is four pixels of half-transparent red, drawn by this test. The \
         reference shot cannot be used for this: every pixel in it is either opaque or empty, \
         so an alpha view of it that was subtly wrong would still look right.\n\n",
    );
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            expected.replace('|', r"\|"),
            actual.replace('|', r"\|"),
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        rows.len()
    ));
    out.push_str(
        "\n## What this does not cover\n\nWhat the grid looks like. It is eight-pixel squares \
         of two greys in the page's stylesheet, fixed to the screen rather than to the \
         picture so that zooming does not stretch them, and no test can see it. The \
         photographs of the window are where it is judged.\n\nZoom and pan, which document 05 \
         lists in the same line: since W-06 the page zooms with the wheel and scrolls the \
         stage around it, and the grid, fixed to the screen, is what says the picture and \
         not the window was zoomed. No test can see that either.\n",
    );
    std::fs::write(repo("verification/B-12a_inspect_table.md"), out).expect("write it");
}

fn write_artifact(rows: &[(String, String, String)]) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let checkout = repo("").display().to_string();
    let mut out = String::new();
    out.push_str("# B-08, what the page is handed when it asks for a frame\n\n");
    out.push_str(
        "Between the renderer and the picture on screen there is a short journey: the page \
         asks for a frame over a local address, and gets back raw pixels with everything the \
         window has to say about them in the headers beside. Nothing else in this project \
         looks at that journey. Produced by `cargo test -p anime_compositor_app`, from \
         `app/src/main.rs`.\n\n",
    );
    out.push_str(
        "The promise is that **the picture and the words about it always came from the same \
         render**. The frame number, the resolution, the project's name, the notes and the \
         unsaved-work flag are attached to the picture they describe, so the number on screen \
         can never belong to a different drawing than the one under it.\n\n",
    );
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in rows {
        let short = |text: &str| {
            text.replace(&checkout, "<the checkout>")
                .replace('|', r"\|")
                .replace('\n', "⏎")
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            short(expected),
            short(actual),
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        rows.len()
    ));
    out.push_str(
        "\n## What this does not cover\n\nThe window itself. These rows call the same function \
         the request handler calls, with the same arguments, but no window is opened and no \
         page is loaded, so a build that never registered the handler at all would pass this \
         table. The photographs in `verification/B-08_window_shell.md` are what show that the \
         window exists and draws; this table is what shows that what it draws is described \
         correctly.\n\n\
         Nor does it cover whether the *picture* is right. Whether frame 24 looks like the \
         shot is H-01's question and B-08a's; the question here is only whether the frame \
         that comes back is the frame that was asked for, whether it is the size it says it \
         is, and whether its colours arrive the way the page draws them. The last of those \
         needs a picture the shot cannot supply, so the four-pixel rows draw their own.\n\n\
         Where a row says *the checkout*, the real value was this machine's copy of the \
         repository, whose path differs on every machine. A line break inside a value is \
         shown as ⏎ so that one row stays one row.\n",
    );
    std::fs::write(repo("verification/B-08_shell_table.md"), out).expect("write the artifact");
}

/// D-767: a frame on which an expression fails carries document 28's diagnostic beside the
/// window's notes (and so in the snapshot JSON), and the property is still drawn at its keyed
/// value. A working expression says nothing.
#[test]
fn a_failed_expression_says_so_beside_the_frame() {
    let directory = scratch("anime_compositor_b647_failed_expression");
    let shot = |text: &str| -> PathBuf {
        let project = directory.join("shot.json");
        let json = serde_json::json!({
          "schema_version": 0, "project_id": "proj-b647",
          "color_settings": { "working_space": "linear-srgb", "alpha_mode": "premultiplied" },
          "assets": [],
          "compositions": [{
            "id": "comp-main", "name": "Main", "width": 4, "height": 4, "pixel_aspect_ratio": 1,
            "frame_rate": { "numerator": 24, "denominator": 1 }, "start_frame": 0,
            "duration_frames": 1, "work_area": { "start_frame": 0, "end_frame_exclusive": 1 },
            "layer_order": ["layer-box"],
            "layers": [{
              "id": "layer-box", "kind": "solid", "name": "Box", "enabled": true, "locked": false,
              "in_frame": 0, "out_frame": 1,
              "transform": {
                "anchor": { "base": [2, 2], "keyframes": [] },
                "position": { "base": [2, 2], "keyframes": [] },
                "scale": { "base": [100, 100], "keyframes": [] },
                "rotation": { "base": 0, "keyframes": [] },
                "opacity": { "base": 1, "keyframes": [],
                             "expression": { "text": text, "enabled": true } }
              },
              "mask": null, "matte": null, "blend_mode": "normal", "effects": [],
              "solid": { "color": [1, 0, 0], "width": 4, "height": 4 }
            }]
          }]
        });
        std::fs::write(&project, json.to_string()).expect("write the project");
        project
    };
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut look = |about: &str, text: &str, note: &str, pixel: &str| {
        let viewer = Mutex::new(open(&shot(text)).expect("open the four-pixel project"));
        let got = ask(&viewer, &export, "/frame/0", Some("q=full"));
        let notes = decode(&header(&got, "x-frame-notes"));
        rows.push((
            format!("{about}: the note beside the frame"),
            note.to_string(),
            notes.split('\t').nth(1).unwrap_or("no note").to_string(),
        ));
        rows.push((
            format!("{about}: the first pixel"),
            pixel.to_string(),
            got.body()[..4]
                .iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(", "),
        ));
    };
    look(
        "Opacity typed `if (`, which cannot be read",
        "if (",
        "EXPRESSION_SYNTAX: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off.",
        "255, 0, 0, 255",
    );
    look(
        "Opacity typed `effect(\"Slider Control\")(\"Slider\")` with no Slider Control on the layer",
        "effect(\"Slider Control\")(\"Slider\")",
        "EXPRESSION_REFERENCE_MISSING: The expression on Box's opacity does not work at frame 0. Correct the expression, or switch it off.",
        "255, 0, 0, 255",
    );
    look(
        "Opacity typed `if (1 == 1) 50 else 0`, which works",
        "if (1 == 1) 50 else 0",
        "no note",
        "255, 0, 0, 128",
    );
    let failed = rows.iter().filter(|(_, e, a)| e != a).count();
    let mut out = String::from(
        "# B-647 table: a failed expression says so (D-767)\n\n\
         Made by `a_failed_expression_says_so_beside_the_frame` in `app/src/tests/serving.rs`. \
         A four-pixel red solid whose Opacity carries the expression; the window is asked for \
         frame 0 as the page asks for it. The note is the code part of `x-frame-notes`, which \
         the page lists under the error details and `tools/app_snapshot.py` reads as \
         `frame_notes`.\n\n| Check | Expected | Got | Result |\n|---|---|---|---|\n",
    );
    for (check, expected, actual) in &rows {
        let verdict = if expected == actual { "pass" } else { "FAIL" };
        out.push_str(&format!("| {check} | {expected} | {actual} | {verdict} |\n"));
    }
    out.push_str(&format!("\n{} of {} pass.\n", rows.len() - failed, rows.len()));
    std::fs::write(repo("verification/B-647_failed_expression_table.md"), out)
        .expect("write the artifact");
    assert_eq!(failed, 0, "{rows:#?}");
}

/// B-701 (D-821): an effect on an adjustment layer that cannot use the masks it names says so
/// beside the frame, as a failed expression does (B-647).
#[test]
fn an_adjustment_layer_warning_says_so_beside_the_frame() {
    let export = Mutex::new(Export::default());
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut sent = Vec::new();
    for (about, file, note) in [
        (
            "Fill naming Fill Mask 2 with only one mask, on an adjustment layer (FX-FILLADJ-012)",
            "Fixtures/fill_adjustment/fx_filladj_012.json",
            "EFFECT_PATH_MISSING",
        ),
        (
            "Fill naming Fill Mask 1 on an adjustment layer, which D-830 draws (FX-FILLADJ-007)",
            "Fixtures/fill_adjustment/fx_filladj_007.json",
            "no note",
        ),
        (
            "Fill with All Masks on, on an adjustment layer, which D-830 draws (FX-FILLADJ-008)",
            "Fixtures/fill_adjustment/fx_filladj_008.json",
            "no note",
        ),
        (
            "Fill with Fill Mask 0 on an adjustment layer, which fills the frame (FX-FILLADJ-001)",
            "Fixtures/fill_adjustment/fx_filladj_001.json",
            "no note",
        ),
    ] {
        let viewer = Mutex::new(open(&repo(file)).expect("open the fixture"));
        let got = ask(&viewer, &export, "/frame/0", Some("q=full"));
        let notes = decode(&header(&got, "x-frame-notes"));
        let code = notes
            .split('\t')
            .nth(1)
            .map_or("no note", |w| w.split(':').next().unwrap_or(""));
        rows.push((
            format!("{about}: the note's code beside the frame"),
            note.to_string(),
            code.to_string(),
        ));
        rows.push((
            format!("{about}: the note's sentence"),
            if note == "no note" { "none" } else { "the sentence the frame said" }.to_string(),
            match notes.split_once('\t') {
                None => "none".to_string(),
                Some((said, _)) if !said.is_empty() => "the sentence the frame said".to_string(),
                Some(_) => "empty".to_string(),
            },
        ));
        if !notes.is_empty() {
            sent.push(format!("- {about}: {}\n", notes.replace('\t', " / ")));
        }
    }
    let failed = rows.iter().filter(|(_, e, a)| e != a).count();
    let mut out = String::from(
        "# B-701 table: an effect on an adjustment layer that cannot use its masks says so (D-821)\n\n\
         Made by `an_adjustment_layer_warning_says_so_beside_the_frame` in \
         `app/src/tests/serving.rs`. Each of D-786's fixtures is opened and the window is asked \
         for frame 0 as the page asks for it. The note is the code part of `x-frame-notes`, \
         which the page lists under the error details and `tools/app_snapshot.py` reads as \
         `frame_notes`. Before D-821 a missing mask said nothing beside the frame; since D-830 an adjustment layer's own masks are used, so only a mask it does not have warns.\n\n\
         | Check | Expected | Got | Result |\n|---|---|---|---|\n",
    );
    for (check, expected, actual) in &rows {
        let verdict = if expected == actual { "pass" } else { "FAIL" };
        out.push_str(&format!("| {check} | {expected} | {actual} | {verdict} |\n"));
    }
    out.push_str(&format!("\n{} of {} pass.\n", rows.len() - failed, rows.len()));
    out.push_str("\nThe notes as the window sent them (the sentence, then the code and sentence):\n\n");
    out.push_str(&sent.concat());
    std::fs::write(repo("verification/B-701_frame_notes_table.md"), out)
        .expect("write the artifact");
    assert_eq!(failed, 0, "{rows:#?}");
}
