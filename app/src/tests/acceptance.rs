use super::*;
use anime_compositor::time::ExposureMap;

struct Report {
    rows: Vec<(String, String, String)>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows
            .push((check.to_string(), expected.to_string(), actual.to_string()));
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// The walkthrough's own directory under `target/`, emptied first so that a previous run's
/// frames cannot make a later one pass. Under `target/` rather than the system temporary
/// directory because the frames are half the artifact: they are meant to be opened.
fn workspace(name: &str) -> PathBuf {
    let directory = repo("target/b12b_w01").join(name);
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).expect("make the walkthrough directory");
    directory
}

/// One command, by the document 24 identifier the page would send.
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

/// The phrase asked for, when it was in what the window said, and everything the window did
/// say when it was not. The exact wording of a diagnostic belongs to the core and is pinned
/// there; what matters here is that the person was told the thing at all.
fn says(phrase: &str, said: &str) -> String {
    match said.contains(phrase) {
        true => phrase.to_string(),
        false => format!("the window said: {said}"),
    }
}

/// The files in a directory, in name order.
fn written(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    names.sort();
    names
}

fn layer<T>(viewer: &Mutex<Viewer>, id: &str, read: impl FnOnce(&Layer) -> T) -> Option<T> {
    let held = held(viewer);
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layer(&Id::new(id)).map(read)
}

/// The layers of the composition, back to front, as `name(identifier)`.
fn stack(viewer: &Mutex<Viewer>) -> String {
    let held = held(viewer);
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layers_in_order()
        .map(|l| format!("{}({})", l.name, l.id))
        .collect::<Vec<_>>()
        .join(", ")
}

fn exposures(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    layer(viewer, layer_id, |l| {
        l.exposure_spans
            .iter()
            .map(|s| {
                format!(
                    "{}-{}:{}",
                    s.start_frame, s.end_frame_exclusive, s.drawing_number
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    })
    .unwrap_or_else(|| "(no such layer)".to_string())
}

/// Which drawing of its sequence a layer shows at every frame of the composition, read the
/// way the renderer reads it -- through `ExposureMap` and the layer's own timing -- rather
/// than by re-reading the spans that were typed in. This is W-01's acceptance line, "cel
/// identity per frame matches the exposure reference", and [`CEL_REFERENCE`] is the
/// reference.
fn cel_identity(viewer: &Mutex<Viewer>, layer_id: &str, frames: i32) -> String {
    let Some((spans, timing)) =
        layer(viewer, layer_id, |l| (l.exposure_spans.clone(), l.timing()))
    else {
        return "(no such layer)".to_string();
    };
    let map = match ExposureMap::new(spans) {
        Ok(map) => map,
        // Document 20's rules about spans are the core's, and a sheet that breaks one is a
        // failed row here rather than a panic: the table is the thing that has to say so.
        Err(error) => return format!("(not a legal sheet: {error:?})"),
    };
    (0..frames)
        .map(|frame| match timing.local_frame(frame) {
            None => "-".to_string(),
            Some(local) => match map.drawing_at(local) {
                None => "-".to_string(),
                Some(number) => number.to_string(),
            },
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// The effect stack of a layer, in evaluation order.
fn effects(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    layer(viewer, layer_id, |l| {
        l.effects
            .iter()
            .map(|e| e.type_id().to_string())
            .collect::<Vec<_>>()
            .join(" then ")
    })
    .unwrap_or_else(|| "(no such layer)".to_string())
}

/// One number of a transform, as a number rather than as a spelling of one. `110` and
/// `110.0` are the same value, and a table that failed on which of the two a serialiser
/// chose would be failing on something nobody can see in a panel.
fn number(value: &serde_json::Value) -> String {
    match value.as_f64() {
        Some(number) => format!("{number}"),
        None => value.to_string(),
    }
}

/// The transform of a layer, read out of the same JSON the inspector is given rather than
/// out of the model, so that what is checked is what a person would read in the panel --
/// including its units, which is where this walk found a defect.
fn transform(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    let answer: serde_json::Value =
        serde_json::from_str(&state(viewer)).expect("the state answer is JSON");
    // The composition named by `composition`, not the first one in the list. Since B-12d this
    // walk makes its own composition and the project holds two, and the page picks the same
    // way -- `doc.project.compositions.find((c) => c.id === doc.composition)`.
    let Some(layer) = answer["project"]["compositions"]
        .as_array()
        .expect("a project has compositions")
        .iter()
        .find(|c| c["id"] == answer["composition"])
        .expect("the composition on screen")["layers"]
        .as_array()
        .expect("a composition has layers")
        .iter()
        .find(|l| l["id"] == layer_id)
        .cloned()
    else {
        return "(no such layer)".to_string();
    };
    ["anchor", "position", "scale", "rotation", "opacity"]
        .iter()
        .map(|prop| {
            let base = &layer["transform"][prop]["base"];
            let numbers = match base.as_array() {
                Some(pair) => pair.iter().map(number).collect::<Vec<_>>(),
                None => vec![number(base)],
            };
            format!("{prop} {}", numbers.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn matte(viewer: &Mutex<Viewer>, layer_id: &str) -> String {
    layer(viewer, layer_id, |l| match &l.matte {
        None => "none".to_string(),
        Some(m) => format!("{} matte_only={}", m.layer_id, m.matte_only),
    })
    .unwrap_or_else(|| "(no such layer)".to_string())
}

/// The drawings of one folder of the reference shot, sorted, which is the selection a person
/// makes in the import dialog.
fn drawings_in(folder: &str) -> Vec<PathBuf> {
    let dir = repo(&format!("Fixtures/reference_shot/{folder}"));
    let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "png"))
        .collect();
    files.sort();
    files
}

/// The request the media bin sends once the file dialog has been answered.
fn import_of(files: &[PathBuf]) -> String {
    let parts: Vec<String> = files
        .iter()
        .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
        .collect();
    format!("media.import?{}", parts.join("&"))
}

fn header(response: &Response<Vec<u8>>, name: &str) -> String {
    response
        .headers()
        .get(name)
        .map(|v| v.to_str().unwrap_or("(not text)").to_string())
        .unwrap_or_else(|| "(no such header)".to_string())
}

/// The exposure sheet this walk assigns to the cel layer, as somebody would type it into the
/// exposure panel: start frame, end frame exclusive, drawing number. Threes and twos with
/// two single-frame accents at the end, which is what a sheet looks like and what the
/// on-twos fixtures in this repository do not exercise.
const SHEET: &[(i32, i32, u32)] = &[
    (0, 3, 0),
    (3, 6, 1),
    (6, 8, 2),
    (8, 10, 3),
    (10, 13, 4),
    (13, 16, 5),
    (16, 19, 6),
    (19, 22, 7),
    (22, 23, 8),
    (23, 24, 9),
];

/// The same sheet read out one composition frame at a time, written by hand from the spans
/// above rather than from a run of the code (ADR-009). This is the exposure reference W-01's
/// acceptance criterion compares cel identity against.
const CEL_REFERENCE: &str = "0,0,0,1,1,1,2,2,3,3,4,4,4,5,5,5,6,6,6,7,7,7,8,9";

/// The transform typed into the inspector at step 6, in the units the panel reads back.
/// `app/ui/index.html` shows the project file's own numbers and sends them back unconverted,
/// so scale is the percentage document 19 stores and opacity the nought-to-one it stores.
const TRANSFORM_TYPED_IN: &str =
    "anchor 960, 540; position 980, 520; scale 110, 110; rotation 6; opacity 0.8";

#[test]
// Slow: twenty-five full-resolution previews and a twenty-four frame export. Run by name,
// exactly as B-10's whole shot is, and its artifact is committed rather than made again on
// every build.
#[ignore = "renders and exports full-resolution frames; run by name"]
fn the_thirteen_steps_of_w01_end_in_a_sequence_on_the_disk() {
    let mut report = Report { rows: Vec::new() };
    let folder = workspace("project");
    let source = repo("Fixtures/projects/minimal_project.json");
    let project_file = folder.join("my_shot.json");
    std::fs::copy(&source, &project_file).expect("copy the empty project to work in");

    let viewer = Mutex::new(
        open(&project_file)
            .unwrap_or_else(|d| panic!("open {}: {}", project_file.display(), d.message)),
    );
    let export_state = Mutex::new(Export::default());

    // ---- step 3, taken first: the composition -------------------------------------------
    // W-01 lists "create a composition" third, and until B-12d nothing in this build could
    // take that step: the composition a person worked in was whichever one their project
    // file already held. `composition.create` is that step, and the twenty-four frames at
    // 1920x1080 and 24 fps are the reference shot's own shape, so everything below happens
    // in a composition this walkthrough made rather than one it was handed.
    let made = run(
        &viewer,
        "composition.create?name=My shot&width=1920&height=1080&fps=24&frames=24",
    );
    report.check(
        "step 3: a composition, made by the command document 24 names",
        "New composition My shot, 1920x1080 at 24 fps, 24 frames",
        says(
            "New composition My shot, 1920x1080 at 24 fps, 24 frames",
            &made,
        ),
    );
    report.check(
        "and the person is told it is empty, rather than left looking at a blank frame",
        "It is empty; import drawings and add layers to fill it.",
        says(
            "It is empty; import drawings and add layers to fill it.",
            &made,
        ),
    );
    report.check(
        "and the window is showing the one that was just made, not the one the file held",
        "comp-1 My shot 1920x1080 at 24 fps, 24 frames",
        {
            let held = held(&viewer);
            let comp = held
                .document
                .project()
                .composition(&held.composition)
                .expect("the composition on screen");
            format!(
                "{} {} {}x{} at {} fps, {} frames",
                comp.id,
                comp.name,
                comp.width,
                comp.height,
                comp.frame_rate.label(),
                comp.duration_frames
            )
        },
    );
    report.check(
        "and the one the file held is still there, because making one is not replacing one",
        "comp-main, comp-1",
        held(&viewer)
            .document
            .project()
            .compositions
            .iter()
            .map(|c| c.id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );

    // ---- step 1: import the drawings ----------------------------------------------------
    let background = run(&viewer, &import_of(&drawings_in("layer1")));
    let cel = run(&viewer, &import_of(&drawings_in("layer2")));
    let shape = run(&viewer, &import_of(&drawings_in("layer3")));
    let overlay = run(&viewer, &import_of(&drawings_in("layer4")));
    report.check(
        "step 1: four sequences imported into a project that had none",
        "asset-1, asset-2, asset-3, asset-4",
        held(&viewer)
            .document
            .project()
            .assets
            .iter()
            .map(|a| a.id.to_string())
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "and the person is told what each one holds",
        "24 drawings, numbered 0 to 23",
        says("24 drawings, numbered 0 to 23", &cel),
    );

    // ---- step 2: grouping, and the warning about the gap --------------------------------
    report.check(
        "step 2: twenty-four files chosen at once are one sequence, not twenty-four",
        "layer2_%03d.png",
        held(&viewer)
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == Id::new("asset-2"))
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "(no asset-2)".to_string()),
    );
    report.check(
        "and the sequence with a hole in it says so at import, not at export",
        "drawing 7 is missing",
        says("drawing 7 is missing", &shape),
    );
    report.check(
        "the one-drawing sequence and the twenty-drawing one are read as they are",
        "1 drawings, numbered 0 to 0 | 20 drawings, numbered 0 to 19",
        format!(
            "{} | {}",
            says("1 drawings, numbered 0 to 0", &background),
            says("20 drawings, numbered 0 to 19", &overlay)
        ),
    );

    // ---- step 5, taken before step 4: stack the layers -----------------------------------
    // The exposures of step 4 are assigned to a layer, so the layers have to exist first.
    // W-01's order is the order a person thinks in, not an order the build enforces.
    run(&viewer, "layer.create?asset=asset-1&name=background");
    run(&viewer, "layer.create?asset=asset-2&name=cel");
    run(&viewer, "layer.create?asset=asset-4&name=overlay");
    run(&viewer, "layer.create?asset=asset-3&name=shape");
    report.check(
        "step 5: four layers, the newest at the front",
        "background(layer-1), cel(layer-2), overlay(layer-3), shape(layer-4)",
        stack(&viewer),
    );
    let said = run(&viewer, "layer.move_down?layer=layer-4");
    report.check(
        "and one of them moved back a place, which is what the reorder button sends",
        "background(layer-1), cel(layer-2), shape(layer-4), overlay(layer-3)",
        stack(&viewer),
    );
    report.check(
        "the window says what moved, in the words undo will use",
        "Move layer to position 2",
        says("Move layer to position 2", &said),
    );

    // ---- step 4: assign exposures --------------------------------------------------------
    // A layer with no exposures renders transparent (document 20), so this step is not
    // decoration: without it the export below writes twenty-four empty frames.
    for (start, end, drawing) in SHEET {
        run(
            &viewer,
            &format!(
                "exposure.set_span?layer=layer-2&start={start}&end={end}&drawing={drawing}"
            ),
        );
    }
    report.check(
        "step 4: the cel layer holds the sheet that was typed into it",
        SHEET
            .iter()
            .map(|(s, e, d)| format!("{s}-{e}:{d}"))
            .collect::<Vec<_>>()
            .join(", "),
        exposures(&viewer, "layer-2"),
    );
    report.check(
        "and the drawing on screen at each of the twenty-four frames is the reference sheet",
        CEL_REFERENCE,
        cel_identity(&viewer, "layer-2", 24),
    );
    // The other three layers, so that every one of them draws something. The layer whose
    // sequence has a hole in it is deliberately never given drawing 7.
    run(
        &viewer,
        "exposure.set_span?layer=layer-1&start=0&end=24&drawing=0",
    );
    for (start, end, drawing) in [(0, 8, 0), (8, 16, 3), (16, 24, 9)] {
        run(
            &viewer,
            &format!(
                "exposure.set_span?layer=layer-4&start={start}&end={end}&drawing={drawing}"
            ),
        );
    }
    for (start, end, drawing) in [(0, 12, 0), (12, 24, 10)] {
        run(
            &viewer,
            &format!(
                "exposure.set_span?layer=layer-3&start={start}&end={end}&drawing={drawing}"
            ),
        );
    }
    report.check(
        "the sequence with the hole is given exposures that step around it",
        "0-8:0, 8-16:3, 16-24:9",
        exposures(&viewer, "layer-4"),
    );
    // And the panel says so when one does not. Undone straight away: what is being checked
    // is the warning, not a change to the shot.
    let onto_the_hole = run(
        &viewer,
        "exposure.set_span?layer=layer-4&start=8&end=16&drawing=7",
    );
    report.check(
        "and typing the missing drawing into the panel is answered there and then",
        "Drawing 7 is not in this sequence",
        says("Drawing 7 is not in this sequence", &onto_the_hole),
    );
    run(&viewer, "edit.undo");
    report.check(
        "undo puts the exposure back, so meeting the warning cost the shot nothing",
        "0-8:0, 8-16:3, 16-24:9",
        exposures(&viewer, "layer-4"),
    );

    // ---- step 6: anchors and transforms ---------------------------------------------------
    for (prop, value) in [
        ("anchor", "960, 540"),
        ("position", "980, 520"),
        ("scale", "110, 110"),
        ("rotation", "6"),
        ("opacity", "0.8"),
    ] {
        run(
            &viewer,
            &format!("property.set_base?layer=layer-3&prop={prop}&value={value}"),
        );
    }
    report.check(
        "step 6: the anchor and the transform, in the units the panel shows and sends",
        TRANSFORM_TYPED_IN,
        transform(&viewer, "layer-3"),
    );

    // ---- step 7: apply a matte ------------------------------------------------------------
    let said = run(
        &viewer,
        "layer.set_matte?layer=layer-3&matte=layer-4&only=true",
    );
    report.check(
        "step 7: the overlay is shaped by the layer under it",
        "layer-4 matte_only=true",
        matte(&viewer, "layer-3"),
    );
    report.check(
        "and the window says which layer is shaping it, and that it is a matte only",
        "Set matte to layer-4, matte only",
        says("Set matte to layer-4, matte only", &said),
    );

    // ---- steps 8 and 9: one blur and one colour operation ---------------------------------
    run(&viewer, "effect.add?layer=layer-3&type=core.gaussian_blur");
    run(
        &viewer,
        "effect.set_parameters?layer=layer-3&effect=fx-1&sigma_px=4",
    );
    run(&viewer, "effect.add?layer=layer-3&type=core.tint");
    run(
        &viewer,
        "effect.set_parameters?layer=layer-3&effect=fx-2&color=0.2,0.4,1&amount=0.5",
    );
    report.check(
        "steps 8 and 9: a blur, then a colour operation, in the order they were added",
        "core.gaussian_blur then core.tint",
        effects(&viewer, "layer-3"),
    );

    // ---- step 10: inspect the alpha -------------------------------------------------------
    let revision_before = held(&viewer).document.revision();
    let ordinary = serve(&viewer, &export_state, Ask::Frame(12), None);
    let said = run(&viewer, "viewer.toggle_alpha");
    let inspected = serve(&viewer, &export_state, Ask::Frame(12), None);
    report.check(
        "step 10: alpha-only inspection says what it is showing",
        "the picture is the alpha channel",
        says("the picture is the alpha channel", &said.to_lowercase()),
    );
    report.check(
        "and what comes back is grey everywhere, and the same size as the picture was",
        "a different picture, every pixel grey, same size",
        {
            let same_size = ordinary.body().len() == inspected.body().len();
            let changed = ordinary.body() != inspected.body();
            let grey = inspected
                .body()
                .chunks_exact(4)
                .all(|p| p[0] == p[1] && p[1] == p[2]);
            match (same_size, changed, grey) {
                (true, true, true) => {
                    "a different picture, every pixel grey, same size".to_string()
                }
                _ => format!("same size {same_size}, changed {changed}, all grey {grey}"),
            }
        },
    );
    report.check(
        "and looking at it changed nothing about the project, which is what makes it looking",
        format!("revision {revision_before}"),
        format!("revision {}", held(&viewer).document.revision()),
    );
    run(&viewer, "viewer.toggle_alpha");

    // ---- step 11: preview the work area ---------------------------------------------------
    let mut stepped = Vec::new();
    for frame in 0..24 {
        let response = serve(&viewer, &export_state, Ask::Frame(frame), None);
        stepped.push(format!(
            "{}:{}",
            response.status().as_u16(),
            header(&response, "x-frame")
        ));
    }
    report.check(
        "step 11: every frame of the work area answers, and answers with itself",
        (0..24)
            .map(|f| format!("200:{f}"))
            .collect::<Vec<_>>()
            .join(" "),
        stepped.join(" "),
    );
    let played = serve(&viewer, &export_state, Ask::At(0), None);
    report.check(
        "and playing rather than stepping asks the clock, which starts at the first frame",
        "200:0",
        format!(
            "{}:{}",
            played.status().as_u16(),
            header(&played, "x-frame")
        ),
    );

    // ---- step 12: save, close, reopen ------------------------------------------------------
    let said = save_as(&viewer, &project_file);
    report.check(
        "step 12: the shot is written to the file it was opened from",
        "Saved to",
        says("Saved to", &said),
    );
    report.check(
        "and the window no longer holds unsaved work",
        "false",
        held(&viewer).document.is_dirty().to_string(),
    );
    drop(viewer);
    let viewer = Mutex::new(
        open(&project_file)
            .unwrap_or_else(|d| panic!("reopen {}: {}", project_file.display(), d.message)),
    );
    report.check(
        "reopened, the layers are in the order they were left in",
        "background(layer-1), cel(layer-2), shape(layer-4), overlay(layer-3)",
        stack(&viewer),
    );
    report.check(
        "the effect order survives the reopening, which W-01 asks for by name",
        "core.gaussian_blur then core.tint",
        effects(&viewer, "layer-3"),
    );
    report.check(
        "so does the matte",
        "layer-4 matte_only=true",
        matte(&viewer, "layer-3"),
    );
    report.check(
        "so does the transform",
        TRANSFORM_TYPED_IN,
        transform(&viewer, "layer-3"),
    );
    report.check(
        "and the cel identity at every frame is still the reference sheet",
        CEL_REFERENCE,
        cel_identity(&viewer, "layer-2", 24),
    );
    report.check(
        "the reopened project had nothing to warn about",
        "[]",
        format!("{:?}", held(&viewer).notes),
    );

    // ---- step 13: export a PNG sequence -----------------------------------------------------
    let into = workspace("frames");
    let (snapshot, root, request) = {
        let held = held(&viewer);
        export_job(&held, &into, MissingSource::Block, OutputFormat::Png)
    };
    report.check(
        "step 13: the range offered is the whole composition, first to last inclusive",
        "0 to 23",
        format!("{} to {}", request.first_frame, request.last_frame),
    );
    let said = run_export(&snapshot, &root, &request, &AtomicBool::new(false));
    report.check(
        "the export finishes under the default missing-drawing policy and says where",
        format!("Exported 24 frames into {}.", into.display()),
        says(
            &format!("Exported 24 frames into {}.", into.display()),
            &said,
        ),
    );
    report.check(
        "twenty-four files are on the disk, named for the project and numbered from zero",
        "24 files, my_shot_0000.png to my_shot_0023.png",
        {
            let names = written(&into);
            format!(
                "{} files, {} to {}",
                names.len(),
                names.first().map(String::as_str).unwrap_or("(none)"),
                names.last().map(String::as_str).unwrap_or("(none)")
            )
        },
    );
    report.check(
        "and not one of them is empty",
        "true",
        written(&into)
            .iter()
            .all(|name| {
                std::fs::metadata(into.join(name))
                    .map(|m| m.len() > 0)
                    .unwrap_or(false)
            })
            .to_string(),
    );

    // One frame kept beside the table, so that the end of the walk is something to look at
    // rather than a count of files. The rest stay under `target/b12b_w01/frames`.
    std::fs::copy(
        into.join("my_shot_0012.png"),
        repo("verification/B-12b_w01_frame_12.png"),
    )
    .expect("keep one exported frame");

    write_artifact(&report);
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "W-01 did not walk end to end: {failed:#?}"
    );
}

fn write_artifact(report: &Report) {
    write_report(
        report,
        "verification/B-12b_w01_walkthrough.md",
        "# W-01 walked end to end, before the owner walks it\n\nGenerated by \
         `app/src/main.rs`, module `acceptance`. Slow, so it does not run with the rest: \
         `cargo test -p anime_compositor_app the_thirteen_steps -- --ignored`.\n\n",
        INTRO,
        NOTES,
    );
}

/// One walkthrough's table, with its own heading, opening paragraphs and closing notes.
fn write_report(report: &Report, path: &str, heading: &str, intro: &[&str], notes: &[&str]) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = String::from(heading);
    for paragraph in intro {
        out.push_str(paragraph);
        out.push_str("\n\n");
    }
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for (check, expected, actual) in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            check,
            cell(expected),
            cell(actual),
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    out.push_str(&format!(
        "\n**{} of {} checks pass.**\n",
        passed,
        report.rows.len()
    ));
    for paragraph in notes {
        out.push('\n');
        out.push_str(paragraph);
        out.push('\n');
    }
    std::fs::write(repo(path), out).expect("write the artifact");
}

const INTRO: &[&str] = &[
    "B-12 is the owner's acceptance run of W-01: thirteen steps by hand, from importing \
     drawings to an exported sequence. This table is the same thirteen steps driven by \
     script first. The reason for doing it twice is arithmetic -- a defect found here costs \
     a re-run of a test, and the same defect found on step twelve of the owner's run costs \
     the run. Two were found this way. One of them, a release-blocking one, is written up in \
     `verification/B-12b_save_works.md`; the other is below.",
    "The shot is built from nothing: a copy of `Fixtures/projects/minimal_project.json`, \
     which is one empty 1920 by 1080 composition of twenty-four frames with no drawings and \
     no layers in it. Everything after that goes through the same function the window's URL \
     scheme calls, with the same text the page would put in it.",
    "**Step 3 used to be the one step that could not be walked.** W-01 says \"create a \
     composition\", and until B-12d this build had no command that made one: document 24 had \
     no `composition.create` row, and the composition somebody worked in was the one their \
     project file already held. This table recorded that as missing rather than working \
     around it. B-12d built it, and the four rows below are that step actually taken -- the \
     composition everything after step 3 happens in is one this walk made, not one it was \
     handed. The composition the file came with is still in the project, untouched, because \
     making one is not replacing one.",
];

const NOTES: &[&str] = &[
    "## The defect this walk found\n\nTyping into the scale field made the layer a hundred \
     times too big. The panel is given the project file's own numbers and sends them back in \
     the same units; a scale in the file is a percentage -- `100` for full size -- while the \
     model holds the factor document 21 composes with. Nothing between the two divided by a \
     hundred. So somebody who typed the `100` the field was already showing, or who dragged \
     the label one step, got a layer a hundred times its size. It survived because every \
     earlier test of that function either set a position, where the two units are the same, \
     or checked a scale being refused. The division now happens once, where typing and \
     dragging both pass through, and the two transform rows above are what check it.\n\nOne \
     part of it is left for the owner rather than quietly reworded: the status line reports \
     what the *model* was set to, so typing 110 into scale is answered \"Set scale to (1.1, \
     1.1)\". That sentence belongs to the core, which is right about its own units, and \
     changing it is a decision rather than a fix.",
    "## What to look at\n\n- **The exposure sheet is threes and twos with two single-frame \
     accents**, not the on-twos cadence every fixture in this repository uses. The reference \
     it is checked against was written out by hand from the spans, not from a run of the \
     code, and it is compared twice: once after it is typed in, and once after the project \
     has been saved, closed and reopened. That is W-01's acceptance line.\n- **The layer \
     whose sequence has a hole in it is given exposures that step around the hole**, and the \
     row after that types the missing drawing in on purpose, to check the panel says so at \
     the moment it is typed rather than three hundred frames into an export. The undo after \
     it costs the shot nothing.\n- **The export runs under the default policy**, which \
     refuses a frame whose drawing is missing, and it finishes -- because nothing in this \
     shot exposes a drawing that is not there.",
    "## What this cannot cover\n\n- **The page.** Every row calls the request rather than \
     clicking the control that sends it. What is wired to what is \
     `verification/B-12b_page_table.md`; that it is on the screen and reachable by keyboard \
     is `verification/B-12a_window_and_keyboard.md`.\n- **The three dialogs.** Import, Save \
     As and Export each open a window Windows draws, and no script can answer one. What is \
     walked here is the request underneath.\n- **Whether it was pleasant.** A script cannot \
     say whether a sentence read well, whether the picture looked right, or whether a step \
     was hard to find. That is what B-12 is for, and it is why this is a rehearsal and not a \
     replacement.\n- **The 240-frame shot.** This composition is twenty-four frames. W-01's \
     \"exactly 240 correctly named files\" is `verification/B-10_full_shot_table.md`, which \
     exports the reference shot twice and compares the two byte for byte.",
    "## Where the frames are\n\nAll twenty-four are written to `target/b12b_w01/frames` and \
     are not cleaned between runs, so they can be opened or flipped through straight after \
     the test. One of them, frame 12, is kept beside this table as \
     `B-12b_w01_frame_12.png`: the background, the cel on its fifth drawing, and the overlay \
     blurred, tinted, moved off centre and shaped by the layer under it.",
];

// ---- W-02, walked the same way ---------------------------------------------------------

/// One sequence of the media bin, by identifier.
fn sequence(viewer: &Mutex<Viewer>, asset: &str) -> String {
    held(viewer)
        .document
        .project()
        .assets
        .iter()
        .find(|a| a.id == Id::new(asset))
        .map(|a| {
            let mut numbers: Vec<u32> = a.frames.keys().copied().collect();
            numbers.sort_unstable();
            format!(
                "{} {} drawings, numbered {}, drawing 0 at {}",
                a.pattern.clone().unwrap_or_default(),
                numbers.len(),
                runs(&numbers),
                a.frames
                    .get(&0)
                    .map(|f| f.replace('\\', "/"))
                    .unwrap_or_else(|| "(no drawing 0)".to_string())
            )
        })
        .unwrap_or_else(|| "(no such sequence)".to_string())
}

/// The drawing numbers a sequence holds, as the runs they fall into, so that a gap in the
/// middle of eleven drawings is something a person reads rather than counts.
fn runs(numbers: &[u32]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut at = 0;
    while at < numbers.len() {
        let mut end = at;
        while end + 1 < numbers.len() && numbers[end + 1] == numbers[end] + 1 {
            end += 1;
        }
        out.push(match end == at {
            true => numbers[at].to_string(),
            false => format!("{} to {}", numbers[at], numbers[end]),
        });
        at = end + 1;
    }
    out.join(", ")
}

/// A picture, short enough to put in a table cell and long enough that two different
/// pictures cannot share one. Not a cryptographic digest: this is a table a person reads,
/// and what it has to do is differ when the drawings differ and match when they are the
/// same file rendered twice.
fn picture(response: &Response<Vec<u8>>) -> String {
    let body = response.body();
    let mut sum: u64 = 1469598103934665603;
    for byte in body {
        sum ^= *byte as u64;
        sum = sum.wrapping_mul(1099511628211);
    }
    format!("{} bytes, {sum:016x}", body.len())
}

/// Copy one folder of drawings into the walkthrough's own directory, so that the project
/// being walked has its own media and nothing here can write into `Fixtures/`.
fn copy_drawings(from: &Path, into: &Path, rename: Option<&str>) {
    std::fs::create_dir_all(into).expect("make the drawings directory");
    for entry in std::fs::read_dir(from).expect("read the drawings folder") {
        let path = entry.expect("a directory entry").path();
        if path.extension().is_some_and(|x| x == "png") {
            let name = path
                .file_name()
                .expect("a file has a name")
                .to_string_lossy()
                .into_owned();
            let name = match rename {
                // `layer4_004.png` delivered as the revised `layer3_004.png`: the drawing
                // number is what a relink matches on, so the revised artwork has to arrive
                // under the numbers the exposure sheet already names.
                Some(stem) => match name.split_once('_') {
                    Some((_, number)) => format!("{stem}_{number}"),
                    None => name,
                },
                None => name,
            };
            std::fs::copy(&path, into.join(name)).expect("copy a drawing");
        }
    }
}

/// The five things W-02 asks for, walked in one go on the reference shot.
///
/// W-02 is a sentence and a half: relink "by explicit user choice", "present changed
/// dimensions, frame range or alpha interpretation before applying the change", "media
/// replacement invalidates affected cache entries and preserves intentional holds", "if
/// frames are absent, report them", "never silently slide subsequent drawings to fill a
/// gap", and "undo restores the prior reference".
///
/// `verification/B-12a_relink_table.md` checks the window's half of that on a small project
/// built for it: what is said, what is refused, and what has not moved yet. What it cannot
/// check is the picture, and the picture is where two of those clauses live. Invalidating a
/// cache entry is invisible unless the frame that comes back afterwards is a different one,
/// and not sliding drawings to fill a gap is invisible unless the frame at the gap is
/// looked at. This walk renders the same frame before the relink, after it and after the
/// undo, on the 240-frame reference shot with its holds, and compares the three.
///
/// Writes `verification/B-12c_w02_walkthrough.md`.
#[test]
// Slow: it copies the reference shot's drawings and renders full-resolution previews.
#[ignore = "copies the reference shot and renders previews; run by name"]
fn the_revised_drawings_replace_the_old_ones_and_the_shot_keeps_its_timing() {
    let mut report = Report { rows: Vec::new() };
    let folder = workspace("w02");
    // The shot as delivered: the reference project and its four folders of drawings, copied
    // so that the media sits beside the project file exactly as a person's would.
    let project_file = folder.join("revised_shot.json");
    std::fs::copy(repo("verification/B-08a_project.json"), &project_file)
        .expect("copy the reference shot project");
    for name in ["layer1", "layer2", "layer3", "layer4"] {
        copy_drawings(
            &repo(&format!("Fixtures/reference_shot/{name}")),
            &folder.join(name),
            None,
        );
    }
    // The revised artwork, delivered in a folder of its own the way a revision arrives:
    // layer 3 redrawn. It is layer 4's drawings under layer 3's numbers, so every drawing
    // is a different picture from the one it replaces, and drawing 7 is not delivered -
    // the hole in this sequence is deliberate and stays a hole.
    let revised = folder.join("revised");
    copy_drawings(
        &repo("Fixtures/reference_shot/layer4"),
        &revised,
        Some("layer3"),
    );
    for number in [7, 12, 13, 14, 15, 16, 17, 18, 19] {
        let _ = std::fs::remove_file(revised.join(format!("layer3_{number:03}.png")));
    }
    let mut delivered: Vec<PathBuf> = std::fs::read_dir(&revised)
        .expect("read the revised folder")
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();
    delivered.sort();

    let viewer = Mutex::new(
        open(&project_file)
            .unwrap_or_else(|d| panic!("open {}: {}", project_file.display(), d.message)),
    );
    let export_state = Mutex::new(Export::default());

    // ---- the shot as it stands ---------------------------------------------------------
    report.check(
        "the shot opens with the four sequences and the four layers it was left with",
        "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
        sequence(&viewer, "asset-layer3"),
    );
    report.check(
        "the layer that uses the sequence being revised holds its drawings on twos",
        "0-2:0, 2-4:1, 4-6:2",
        exposures(&viewer, "layer-3")
            .split(", ")
            .take(3)
            .collect::<Vec<_>>()
            .join(", "),
    );
    let before = picture(&serve(&viewer, &export_state, Ask::Frame(8), None));
    let at_the_gap_before = picture(&serve(&viewer, &export_state, Ask::Frame(14), None));
    report.check(
        "and a frame of it can be looked at, which is what a relink has to change",
        true,
        before != "0 bytes, 0000000000000000",
    );

    // ---- W-02 step 1: choose the revised drawings --------------------------------------
    let said = run(
        &viewer,
        &format!(
            "media.relink?asset=asset-layer3&{}",
            delivered
                .iter()
                .map(|p| format!("file={}", for_a_header(&p.display().to_string())))
                .collect::<Vec<_>>()
                .join("&")
        ),
    );
    report.check(
        "step 1: choosing the revised drawings says what the sequence would become",
        "would give it 11 drawings, numbered 0 to 11, and drawing 7 would be missing",
        says(
            "would give it 11 drawings, numbered 0 to 11, and drawing 7 would be missing",
            &said,
        ),
    );
    report.check(
        "and states the size, which W-02 asks for whether or not it changed",
        "The drawings are 1920x1080, the same size as the ones it points at now.",
        says(
            "The drawings are 1920x1080, the same size as the ones it points at now.",
            &said,
        ),
    );
    report.check(
        "and how the colour and the alpha will be read, which W-02 also asks for",
        "The colour is read as sRGB with straight alpha",
        says("The colour is read as sRGB with straight alpha", &said),
    );
    report.check(
        "and that the drawing that was not delivered is missing, at the moment of choosing",
        "One drawing is missing from layer3_%03d.png: 7.",
        says("One drawing is missing from layer3_%03d.png: 7.", &said),
    );

    // ---- step 2: nothing has moved yet -------------------------------------------------
    report.check(
        "step 2: the sequence still points at the drawings it did",
        "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
        sequence(&viewer, "asset-layer3"),
    );
    report.check(
        "and there is nothing to undo, because nothing has been done",
        0,
        held(&viewer).document.undo_depth(),
    );
    report.check(
        "and the frame that comes back is the frame that came back before",
        &before,
        picture(&serve(&viewer, &export_state, Ask::Frame(8), None)),
    );

    // ---- step 3: agree to it -----------------------------------------------------------
    let said = run(&viewer, "media.relink?asset=asset-layer3&apply=1");
    report.check(
        "step 3: agreeing names what was done, in the words undo will use",
        "Relink layer3",
        says("Relink layer3", &said),
    );
    report.check(
        "the sequence now points at the drawings that were chosen",
        "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at revised/layer3_000.png",
        sequence(&viewer, "asset-layer3"),
    );
    report.check(
        "the layer is the same layer, holding the same drawings for the same frames",
        "0-2:0, 2-4:1, 4-6:2",
        exposures(&viewer, "layer-3")
            .split(", ")
            .take(3)
            .collect::<Vec<_>>()
            .join(", "),
    );
    report.check(
        "and its whole exposure sheet is untouched, which is W-02's intentional holds",
        120,
        layer(&viewer, "layer-3", |l| l.exposure_spans.len()).unwrap_or(0),
    );
    report.check(
        "the other three sequences were not touched by a relink of this one",
        "layer2_%03d.png 24 drawings, numbered 0 to 23, drawing 0 at layer2/layer2_000.png",
        sequence(&viewer, "asset-layer2"),
    );

    // ---- step 4: the picture, which is where the cache lives ---------------------------
    let after = picture(&serve(&viewer, &export_state, Ask::Frame(8), None));
    report.check(
        "step 4: the frame that had been looked at before the relink comes back different",
        "a different picture",
        match after == before {
            true => "the same picture, so a cached frame outlived the drawings it was made \
                     from"
                .to_string(),
            false => "a different picture".to_string(),
        },
    );
    report.check(
        "and the frame at the gap is the picture it was, because nothing was slid \
         into it",
        &at_the_gap_before,
        picture(&serve(&viewer, &export_state, Ask::Frame(14), None)),
    );
    report.check(
        "and frames 14 to 17 still ask for drawings 7, 7, 8, 8 - the two frames over the \
         gap keep asking for the drawing nobody drew, rather than being given the next one",
        "7,7,8,8",
        cel_identity(&viewer, "layer-3", 20)
            .split(',')
            .skip(14)
            .take(4)
            .collect::<Vec<_>>()
            .join(","),
    );

    // ---- step 5: undo restores the prior reference -------------------------------------
    let said = run(&viewer, "edit.undo");
    report.check(
        "step 5: undo says which thing it undid",
        "Undone: Relink layer3",
        says("Undone: Relink layer3", &said),
    );
    report.check(
        "the sequence points at the drawings it pointed at before",
        "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at layer3/layer3_000.png",
        sequence(&viewer, "asset-layer3"),
    );
    report.check(
        "and the frame is the picture it was, byte for byte, not a cached revised one",
        &before,
        picture(&serve(&viewer, &export_state, Ask::Frame(8), None)),
    );

    // ---- and it survives being written down --------------------------------------------
    run(&viewer, "edit.redo");
    let said = save_as(&viewer, &project_file);
    report.check(
        "redone and saved, the file names the drawings that were chosen",
        "Saved to",
        says("Saved to", &said),
    );
    let reopened = Mutex::new(
        open(&project_file)
            .unwrap_or_else(|d| panic!("reopen {}: {}", project_file.display(), d.message)),
    );
    report.check(
        "and reopening it finds the revised sequence, with the gap still a gap",
        "layer3_%03d.png 11 drawings, numbered 0 to 6, 8 to 11, drawing 0 at revised/layer3_000.png",
        sequence(&reopened, "asset-layer3"),
    );
    report.check(
        "and the exposure sheet that was typed before the revision, unchanged",
        120,
        layer(&reopened, "layer-3", |l| l.exposure_spans.len()).unwrap_or(0),
    );

    write_report(
        &report,
        "verification/B-12c_w02_walkthrough.md",
        "# W-02 walked end to end, on the shot W-01 built\n\n\
         Generated by `app/src/main.rs`, module `acceptance`. Slow, so it does not run with \
         the rest: `cargo test -p anime_compositor_app the_revised_drawings -- --ignored`.\n\n",
        W02_INTRO,
        W02_NOTES,
    );
    let failed: Vec<&(String, String, String)> =
        report.rows.iter().filter(|(_, e, a)| e != a).collect();
    assert!(
        failed.is_empty(),
        "W-02 did not walk end to end: {failed:#?}"
    );
}

const W02_INTRO: &[&str] = &[
    "W-02 is the other acceptance run: an existing shot, a folder of redrawn artwork, and \
     the question of whether replacing one costs the other. The window's half of it - what \
     is said before anything changes, what is refused, what has not moved yet - is \
     `verification/B-12a_relink_table.md`, on a small project built for it. This is the same \
     workflow on the 240-frame reference shot, and it is here for the two clauses of W-02 \
     that table cannot reach, because both of them are about the picture.",
    "**\"Media replacement invalidates affected cache entries.\"** A frame is decoded once \
     and kept. The failure is invisible in the project file and visible only in the window: \
     the sequence says it points at the new drawings, and the frame that comes back is the \
     old ones. So the same frame is rendered before the relink, after it, and again after \
     the undo, and the three are compared. The last of those is the sharper check - after \
     undoing, the picture has to be the first one back byte for byte.",
    "**\"Never silently slide subsequent drawings to fill a gap.\"** The revised folder is \
     delivered one drawing short, which is what happens. Nothing is renumbered to close the \
     hole: the frames that exposed drawing 7 still ask for drawing 7, and document 28 draws \
     a drawing that is not there as nothing.",
];

const W02_NOTES: &[&str] = &[
    "## What to look at\n\n- **The two pictures that must differ and the two that must \
     match.** Frame 8 before the relink and after it are different pictures; frame 8 before \
     the relink and after the undo are the same picture, to the byte. A build that cached \
     the decoded drawing and never dropped it would fail the first; a build that dropped \
     everything and re-decoded from the wrong reference would fail the second.\n- **The \
     exposure sheet is not touched.** 120 spans before, 120 spans after - the 240 frames of \
     the shot held on twos - and the drawing \
     each frame asks for either side of the gap is what it was. Relinking replaces one \
     record in the media bin and cannot reach a layer, which is the design; these rows are \
     what says so on the shot rather than in the abstract.\n- **The revised artwork arrives \
     in a folder of its own.** It is layer4's drawings delivered under layer3's numbers, \
     which is what a revision looks like: same names, same numbers, different pictures, one \
     of them late.",
    "## What this cannot cover\n\n- **The file dialog.** As everywhere in this window, the \
     walk begins at the files a person chose.\n- **Whether the revised artwork is the right \
     artwork.** Nothing here can say the redraw was the one that was asked for; it can say \
     the shot now shows it and kept its timing.\n- **Masks.** W-02 names them among what a \
     relink preserves. It does preserve them, for the same reason it preserves exposures - \
     it replaces an asset record and cannot reach a layer - but this build has no gesture \
     for drawing a mask, so there is no mask in this shot to watch survive one.\n- **The \
     page.** Every row calls the request the panel sends rather than clicking the panel. \
     What is wired to what is `verification/B-12b_page_table.md`.",
];

fn cell(text: &str) -> String {
    text.replace('|', r"\|")
}
