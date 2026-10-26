use super::*;

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

fn page() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ui/index.html");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn source() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/main.rs");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Everything that follows `needle`, up to the first character a route cannot contain.
///
/// A route is written in the page in one of two shapes, and this reads both by being given
/// each opening in turn: a quoted string beginning with a slash, and the identifier handed to
/// `send`.
fn after(text: &str, needle: &str) -> Vec<String> {
    text.match_indices(needle)
        .map(|(at, _)| {
            let rest = &text[at + needle.len()..];
            let end = rest
                .find(|c: char| !(c.is_ascii_lowercase() || c == '.' || c == '_' || c == '-'))
                .unwrap_or(rest.len());
            rest[..end].to_string()
        })
        .filter(|found| !found.is_empty())
        .collect()
}

fn sorted(mut list: Vec<String>) -> Vec<String> {
    list.sort();
    list.dedup();
    list
}

/// Every route the page can ask the window for.
fn asked_for(page: &str) -> Vec<String> {
    let mut found = after(page, "'/");
    found.extend(after(page, "send('"));
    sorted(found)
}

/// The global accelerator handler, which is the last `keydown` listener in the page.
fn accelerators(page: &str) -> String {
    let at = page
        .find("addEventListener('keydown', (e) => {")
        .expect("the page has a global keydown handler");
    page[at..].to_string()
}

// ---- the page ----------------------------------------------------------------------------

/// Every command identifier written into the page, in the order a sorted list puts them.
///
/// Pinned rather than counted: a new `send` of an identifier nobody listed is a change to
/// what the window can do, and it should have to be written down here as well as there.
///
/// B-13d took `layer.set_depth` out of this list on 2026-09-16, and a removal is as much a
/// change to what the window can do as an addition. A depth became one of document 19's
/// properties in the sense document 24 line 91 already described, so the Depth row sends
/// `property.set_base` and `keyframe.add_remove` like every other number in the window, and
/// nothing in the page sends the older command any more. It is still in document 24 and the
/// window still answers it - `REACHED` below checks that it does - because it is the command
/// that will carry the keep-place conversion when a layer gains or loses a parent. Until
/// that is built, no control in the window reaches it, and that is the owner's to keep or
/// cut rather than a thing to quietly delete from the table.
///
/// B-13e took `camera.set_property` out on the same day, for the same reason and with the
/// same care. Once the camera could be keyed, a box that set a base was a trap rather than a
/// control: on a camera with keys the base is not what is drawn on any frame, so the box
/// would have moved a number nobody could see and looked broken. The Camera block's three
/// rows send `property.set_base` like every other number in the window, and no control sends
/// the older command any more. It too stays in document 24 and `REACHED` still checks the
/// window answers it.
const SENT: &[&str] = &[
    "asset.set_redistribute",
    "composition.create",
    "composition.delete",
    "composition.duplicate",
    "composition.open",
    "composition.set_frame_blending",
    "composition.set_motion_blur",
    "composition.set_settings",
    "edit.redo",
    "edit.undo",
    "effect.add",
    "effect.delete",
    "effect.fit_scale",
    "effect.move",
    "effect.move_down",
    "effect.move_up",
    "effect.paste",
    "effect.set_parameters",
    "effect.toggle_bypass",
    "exposure.set_span",
    "exposure.toggle_key",
    "exposure.write",
    "item.set_label",
    "keyframe.add_remove",
    "keyframe.move",
    "keyframe.reverse",
    "keyframe.set_interp",
    "keyframe.set_kind",
    "keyframe.set_path",
    "keyframe.set_roving",
    "layer.add_adjustment",
    "layer.add_composition",
    "layer.add_null",
    "layer.add_shape",
    "layer.add_solid",
    "layer.add_text",
    "layer.copy",
    "layer.create",
    "layer.delete",
    "layer.duplicate",
    "layer.freeze_frame",
    "layer.move",
    "layer.move_down",
    "layer.move_up",
    "layer.paste",
    "layer.precompose",
    "layer.rename",
    "layer.replace_source",
    "layer.set_blend_mode",
    "layer.set_drawing_dissolve",
    "layer.set_gain",
    "layer.set_label",
    "layer.set_markers",
    "layer.set_matte",
    "layer.set_matte_mode",
    "layer.set_parent",
    "layer.set_time_remap",
    "layer.set_time_stretch",
    "layer.shift",
    "layer.split",
    "layer.toggle_frame_blend",
    "layer.toggle_lock",
    "layer.toggle_motion_blur",
    "layer.toggle_preserve_transparency",
    "layer.toggle_shy",
    "layer.toggle_solo",
    "layer.toggle_visibility",
    "layer.trim",
    "mask.add",
    "mask.add_remove_key",
    "mask.delete",
    "mask.set",
    "mask.set_path",
    "media.import",
    "media.relink",
    "path.add_point",
    "path.remove_point",
    "preset.apply",
    "property.drag_cancel",
    "property.drag_end",
    "property.drag_update",
    "property.link",
    "property.separate",
    "property.set_base",
    "property.set_expression",
    "shape.add",
    "shape.add_remove_key",
    "shape.delete",
    "shape.group",
    "shape.operation",
    "shape.repeater",
    "shape.set",
    "shape.set_path",
    "shape.ungroup",
    "sheet.write_action",
    "sketch.add_stroke",
    "sketch.clear",
    "sketch.move_layer",
    "sketch.move_strokes",
    "sketch.remove_layer",
    "sketch.remove_strokes",
    "sketch.set_layer",
    "sketch.set_look",
    "solid.set",
    "text.set",
    "timeline.set_markers",
    "timeline.set_work_end",
    "timeline.set_work_start",
    "timesheet.import",
    "viewer.toggle_alpha",
    "viewer.toggle_checkerboard",
];

/// The routes that are not commands: the shell's own, and the two the transport uses.
const ROUTES: &[&str] = &[
    // The frame scheme's four, which are not commands and are not answered by the shell:
    // `frame` is a numbered frame, `at` is the frame playback has reached by a given number
    // of milliseconds, `play` starts the clock over from the playhead (W-09), and `boxes`
    // is where the selected layers landed on that frame, which only the renderer knows
    // because an asset records no pixel size. `curve` is the fifth, added by D-52: one
    // property evaluated across a range, so the graph editor draws what the renderer will
    // do rather than its own idea of it. The clock is checked in
    // `verification/B-08_preview_table.md`.
    // D-258: one layer by itself, for the onion skin.
    "alone",
    "at",
    "boxes",
    "cancel-export",
    "check-package",
    "collect",
    "curve",
    "export",
    // D-252: the job being written, and its frames, for the Render panel.
    "export-frame",
    "export-watch",
    // D-257: the files gone from the disk, for the health chip.
    "files-gone",
    // D-264: the fonts for the Character section, and one font's bytes for the typing box.
    "font",
    "fonts",
    "frame",
    // B-44: the CPU / GPU switch. A route rather than a `viewer.` command because the card
    // belongs to the window, not to the viewer `edit_command` is handed.
    "gpu-switch",
    // D-182: a Color Lookup's .cube file.
    "lut-choose",
    // B-48: the memory setting, which is the window's for the same reason.
    "memory",
    // D-640: on the frame scheme, the composition's sound as the core mixes it.
    "mix",
    "new",
    "open",
    "pause-export",
    // B-45: where the viewer's picture lies in the window, for the card to paint it there.
    "place",
    "play",
    // D-181: the starter presets. D-180: Export presets and Import presets.
    "presets-builtin",
    "presets-export",
    "presets-import",
    // D-251: the render queue.
    "queue",
    // W-40c (D-249): the frames the memory holds, for the green line under the ruler.
    "ready",
    "recent",
    "recover",
    "save",
    "save-as",
    // D-732: Increment and Save.
    "save-increment",
    // P-19: the session log panel's question, switch, Clear and Save a copy.
    "session-log",
    // D-84a: the seventh on the frame scheme, the Sheet tab's grid.
    "sheet",
    // D-289: a sketch saved as a picture file, its PNG bytes sent by the page.
    "sketch-png",
    // D-71: the sixth on the frame scheme, one sound file's bytes for the page's decoder.
    "sound",
    "state",
    // D-893: the viewer stops a frame being drawn once it is stale.
    "stop",
    // D-260: the Project panel's small pictures.
    "thumb",
];

/// A control, the identifier it must send, and the text in the page that says it does.
///
/// The third column is what makes this more than a list: it anchors the identifier to the
/// handler that sends it, so moving `layer.delete` onto the button that moves a layer
/// forward fails here rather than passing because the string is still somewhere in the file.
const WIRING: &[(&str, &str, &str)] = &[
    (
        "Level",
        "layer.set_gain",
        "level.onchange = () => command('/layer.set_gain?layer=' + encodeURIComponent(layer.id)",
    ),
    (
        "New solid",
        "layer.add_solid",
        "command('/layer.add_solid?' + (at < 0 ? '' : 'to=' + (at + 1) + '&') + 'name=' + encodeURIComponent(name)",
    ),
    (
        "New shape layer",
        "layer.add_shape",
        "await command('/layer.add_shape' + (at < 0 ? '' : '?to=' + (at + 1)));",
    ),
    (
        "New null",
        "layer.add_null",
        "command('/layer.add_null' + (at < 0 ? '' : '?to=' + (at + 1)))",
    ),
    (
        "the pen, the rectangle and the ellipse on a shape layer",
        "shape.add",
        "command((drawsShapes(layer) ? '/shape.add?' : '/mask.add?') + 'closed=' + (closed ? 1 : 0) + '&'",
    ),
    (
        "a shape's fill and stroke in the layer's panel",
        "shape.set",
        "const set = (what) => command('/shape.set?layer=' + encodeURIComponent(layer.id)",
    ),
    (
        "Width and Height of a solid, dragged or typed",
        "solid.set",
        "commit: (v) => set(side + '=' + encodeURIComponent(v)),",
    ),
    (
        "Colour, Width and Height of a solid",
        "solid.set",
        "command('/solid.set?layer=' + encodeURIComponent(layer.id) + '&' + what)",
    ),
    (
        "Can be passed on",
        "asset.set_redistribute",
        "command('/asset.set_redistribute?asset=' + encodeURIComponent(asset.id)",
    ),
    (
        "Delete layer",
        "layer.delete",
        "command('/layer.delete?layer=' + encodeURIComponent(selectedLayer) \
         + '&frame=' + frame)",
    ),
    (
        "Forward",
        "layer.move_up",
        "$('up').onclick = () => moveLayers(true)",
    ),
    (
        "Back",
        "layer.move_down",
        "$('down').onclick = () => moveLayers(false)",
    ),
    (
        "Undo",
        "edit.undo",
        "$('undo').onclick = () => command('/edit.undo')",
    ),
    (
        "Redo",
        "edit.redo",
        "$('redo').onclick = () => command('/edit.redo')",
    ),
    (
        "Alpha only",
        "viewer.toggle_alpha",
        "$('alpha').onclick = () => command('/viewer.toggle_alpha')",
    ),
    (
        "the transparency grid",
        "viewer.toggle_checkerboard",
        "$('checker').onclick = () => command('/viewer.toggle_checkerboard')",
    ),
    (
        "Import drawings",
        "media.import",
        "$('import').onclick = () => command('/media.import')",
    ),
    (
        "Add an exposure",
        "exposure.set_span",
        "command('/exposure.set_span?layer='",
    ),
    ("Add effect", "effect.add", "command('/effect.add?layer='"),
    (
        "Add layer",
        "layer.create",
        "command('/layer.create?asset='",
    ),
    (
        "Relink drawings",
        "media.relink",
        "command('/media.relink?asset=' + encodeURIComponent(selectedAsset))",
    ),
    (
        "Import cut",
        "timesheet.import",
        "$('importcut').onclick = () => command('/timesheet.import')",
    ),
    (
        "Apply the relink",
        "media.relink",
        "encodeURIComponent(relinking) + '&apply=1'",
    ),
    (
        "Leave it as it is",
        "media.relink",
        "encodeURIComponent(relinking) + '&cancel=1'",
    ),
    // B-27c: D-83's two whips.
    (
        "Value pick whip",
        "property.link",
        "'/property.link?' + whose(layer) + '&prop=' + prop + '&from_' + whoseId(id) + '&from_prop=' + to",
    ),
    (
        "Parent pick whip",
        "layer.set_parent",
        "return ['Parent: ' + parent.name, '/layer.set_parent?layer=' + encodeURIComponent(layer.id)",
    ),
];

/// D-261: the window's security policy drops a style written into the markup, so a colour or
/// size put there never shows; the page's look lives in its stylesheet.
#[test]
fn the_page_writes_no_style_into_its_markup() {
    assert_eq!(
        page().matches(" style=\"").count(),
        0,
        "a style=\"...\" in the page is refused by the window"
    );
}

#[test]
fn the_page_asks_for_nothing_the_window_cannot_answer() {
    let mut report = Report { rows: Vec::new() };
    let page = page();
    let source = source();
    let asked = asked_for(&page);

    let (commands, routes): (Vec<String>, Vec<String>) =
        asked.into_iter().partition(|found| found.contains('.'));

    report.check(
        "the identifiers the page sends are the ones written down here",
        SENT.join(", "),
        commands.join(", "),
    );
    report.check(
        "the routes the page asks for that are not commands are the ones written down here",
        ROUTES.join(", "),
        routes.join(", "),
    );

    // Every command identifier, answered. `None` is the window saying it has never heard of
    // the identifier, which is what a page sending `layer.remove` for `layer.delete` would
    // look like: a button that does nothing at all, silently.
    let viewer = Mutex::new(demo());
    for id in &commands {
        report.check(
            &format!("the window answers `{id}`"),
            "a sentence",
            match edit_command(&viewer, id, None) {
                Some(said) if said.is_empty() => "an empty answer".to_string(),
                Some(_) => "a sentence".to_string(),
                None => "nothing - the window has never heard of it".to_string(),
            },
        );
    }
    // The two inspection toggles were flipped by the loop above, because answering them is
    // what they do. Put them back, so that nothing after this reads a viewer this test left
    // looking at the alpha channel.
    for id in ["viewer.toggle_alpha", "viewer.toggle_checkerboard"] {
        edit_command(&viewer, id, None);
    }

    // The routes are answered by `fn command` rather than by `edit_command`, so what is
    // checked is that the shell has an arm of that name. `frame` and `state` are not in
    // that match: one is the other scheme, one is answered before it.
    for route in &routes {
        // `frame`, `at`, `play`, `boxes`, `curve`, `sheet`, `sound`, `mix` and `ready` belong to the
        // other scheme and are served beside `fn frame`, not by the command shell, so there is
        // no arm of that name to look for.
        if matches!(
            route.as_str(),
            "frame" | "at" | "play" | "boxes" | "curve" | "sheet" | "sound" | "mix" | "ready" | "stop"
        ) {
            continue;
        }
        let arm = format!("\"{route}\"");
        report.check(
            &format!("the window's shell answers `/{route}`"),
            true,
            source.contains(&arm),
        );
    }

    for (control, id, wiring) in WIRING {
        report.check(
            &format!("the {control} control sends `{id}`"),
            true,
            page.contains(wiring),
        );
    }

    // Every check above this line reads the script and none of them reads the markup, which
    // is how renaming `id="up"` to `id="upward"` and leaving the handler saying `$('up')`
    // got through the whole table once. A handler hung on a name nothing is called is a
    // button that does nothing, silently, exactly like an identifier nobody answers.
    let defined: Vec<&str> = page
        .match_indices("id=\"")
        .map(|(at, _)| {
            let rest = &page[at + 4..];
            &rest[..rest.find('"').unwrap_or(0)]
        })
        .collect();
    let missing: Vec<String> = sorted(after(&page, "$('"))
        .into_iter()
        .filter(|name| !defined.contains(&name.as_str()))
        .collect();
    report.check(
        "every control the script reaches for is one the markup defines",
        "none missing",
        match missing.is_empty() {
            true => "none missing".to_string(),
            false => missing.join(", "),
        },
    );

    write_artifact(
        &report,
        "verification/B-12b_page_table.md",
        "B-12b: what the page asks the window for",
        PAGE_INTRO,
        PAGE_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const PAGE_INTRO: &[&str] = &[
    "`app/ui/index.html` is the one file in this project that no test has ever read. Every \
     B-12a table calls the same function the window's URL scheme calls, which checks \
     everything from the request inwards and nothing outwards of it; what the page does with \
     a click has been checked only by looking at photographs. This table is the outward half, \
     as far as reading a file can take it.",
    "It reads the page and the window's own source, and asks three questions. Which \
     identifiers does the page send? Does anything answer them? And is each named control \
     wired to the identifier document 24 gives it?",
];

const PAGE_NOTES: &[&str] = &[
    "## What to look at\n\n- **The first two rows are a list, not a count.** They name every \
     identifier and every route the page can ask for, in one cell each. A button added \
     tomorrow that sends something new fails this row until the new identifier is written \
     down beside the others, which is the point: what the window can be asked to do should \
     be a thing somebody wrote down.\n- **`nothing - the window has never heard of it`** is \
     the answer this table exists to catch. A page sending an identifier the window does not \
     answer is a button that does nothing, with no error, no status line and nothing in the \
     log.\n- **The wiring rows anchor an identifier to its handler.** Checking only that \
     `layer.delete` appears somewhere in the page would pass if Delete layer and Forward \
     swapped commands.\n- **The last row reads the markup rather than the script.** Every \
     other row here reads the handlers; that one checks that each control a handler reaches \
     for is a control this page actually contains, because renaming a button and forgetting \
     its handler leaves the handler attached to nothing.",
    "## What this cannot cover\n\nThis is string matching over JavaScript, not a browser. It \
     can see that a handler names an identifier. It cannot see that the handler is attached \
     to a button, that the button is on the screen, that it is enabled, or that clicking it \
     reaches the handler. `verification/B-12a_window_and_keyboard.md` and its photographs are \
     what say those, and the owner's run under B-12 is what says it for real.\n\nIt also \
     cannot see the shell routes actually working. `/save` is checked here only as far as \
     `fn command` having an arm of that name; what a save writes is \
     `verification/B-09_save_table.md`.",
];

// ---- document 24 -------------------------------------------------------------------------

/// How each of document 24's command identifiers is reached in this build.
///
/// Written down rather than derived, because the interesting half is the identifiers nothing
/// reaches: an identifier that is simply absent from a build looks exactly like one nobody
/// has noticed is absent. The test measures which of the four this build actually does and
/// compares; a row that says `nothing yet` is checked to be true as hard as the others.
const REACHED: &[(&str, &str)] = &[
    ("project.new", "a route the shell answers"),
    ("project.open", "a route the shell answers"),
    ("project.save", "a route the shell answers"),
    ("project.save_as", "a route the shell answers"),
    ("project.save_increment", "a route the shell answers"),
    ("composition.create", "a command the window answers"),
    ("composition.open", "a command the window answers"),
    ("composition.set_settings", "a command the window answers"),
    // D-188: the composition's shutter, built by B-124b.
    (
        "composition.set_motion_blur",
        "a command the window answers",
    ),
    // D-216, accepted on 2026-09-29 and built by B-150b.
    (
        "composition.set_frame_blending",
        "a command the window answers",
    ),
    ("composition.duplicate", "a command the window answers"),
    ("composition.delete", "a command the window answers"),
    ("edit.undo", "a command the window answers"),
    ("edit.redo", "a command the window answers"),
    ("media.import", "a command the window answers"),
    ("media.relink", "a command the window answers"),
    // D-61, B-15c: the tick on a drawing, and File > Collect Files and Check Package.
    ("asset.set_redistribute", "a command the window answers"),
    // D-253 and D-254, accepted on 2026-10-02.
    ("item.set_label", "a command the window answers"),
    ("project.collect", "a route the shell answers"),
    ("package.check", "a route the shell answers"),
    ("layer.create", "a command the window answers"),
    ("layer.delete", "a command the window answers"),
    ("layer.rename", "a command the window answers"),
    ("layer.move_up", "a command the window answers"),
    ("layer.move_down", "a command the window answers"),
    ("layer.toggle_visibility", "a command the window answers"),
    ("layer.toggle_lock", "a command the window answers"),
    ("layer.set_matte", "a command the window answers"),
    ("layer.set_parent", "a command the window answers"),
    // D-58, accepted by the owner on 2026-09-15 and built by B-13c. The inspector's
    // depth row sends one and its camera block sends the other, so both are in SENT as
    // well; this column measures the window answering them, which it did first.
    ("layer.set_depth", "a command the window answers"),
    ("camera.set_property", "a command the window answers"),
    ("layer.shift", "a command the window answers"),
    ("layer.trim", "a command the window answers"),
    ("layer.move", "a command the window answers"),
    ("layer.duplicate", "a command the window answers"),
    ("layer.split", "a command the window answers"),
    ("layer.toggle_solo", "a command the window answers"),
    ("layer.set_label", "a command the window answers"),
    ("layer.replace_source", "a command the window answers"),
    ("layer.set_blend_mode", "a command the window answers"),
    ("layer.set_gain", "a command the window answers"),
    // D-66, accepted on 2026-09-17; B-17c put the button in the window the same day.
    ("layer.add_adjustment", "a command the window answers"),
    // D-67, accepted on 2026-09-18 and built in the core by B-18b; B-18c put both in the
    // window.
    ("layer.add_composition", "a command the window answers"),
    ("layer.precompose", "a command the window answers"),
    ("layer.copy", "a command the window answers"),
    ("layer.paste", "a command the window answers"),
    ("layer.toggle_shy", "a command the window answers"),
    // D-188, accepted on 2026-09-28 and built by B-124b.
    ("layer.toggle_motion_blur", "a command the window answers"),
    // D-828, built by B-708: the T switch.
    ("layer.toggle_preserve_transparency", "a command the window answers"),
    // D-216, accepted on 2026-09-29 and built by B-150b.
    ("layer.toggle_frame_blend", "a command the window answers"),
    ("layer.set_time_stretch", "a command the window answers"),
    // D-323, from D-308: Enable Time Remapping.
    ("layer.set_time_remap", "a command the window answers"),
    // D-314, from D-309: Freeze Frame.
    ("layer.freeze_frame", "a command the window answers"),
    ("layer.set_drawing_dissolve", "a command the window answers"),
    // D-74, accepted on 2026-09-19 and built in the core by B-23b; B-23c put both in the
    // window.
    ("layer.add_solid", "a command the window answers"),
    ("solid.set", "a command the window answers"),
    // D-77, accepted on 2026-09-19 and built in the core by B-24b; B-24c put four of them
    // in the window and B-24d added the fifth, the stopwatch on a path. Each one reads the
    // layer's list of masks, changes one thing in it and sends the whole list back, because
    // SET_MASKS is what the core takes.
    ("mask.add", "a command the window answers"),
    ("mask.add_remove_key", "a command the window answers"),
    ("mask.set_path", "a command the window answers"),
    ("mask.set", "a command the window answers"),
    ("mask.delete", "a command the window answers"),
    // D-78, accepted on 2026-09-21 and built in the core by B-25b; B-25c put all six in the
    // window, each the twin of its mask command.
    ("layer.add_shape", "a command the window answers"),
    // D-263, accepted on 2026-10-02: text layers.
    ("layer.add_text", "a command the window answers"),
    ("text.set", "a command the window answers"),
    ("shape.add", "a command the window answers"),
    ("shape.add_remove_key", "a command the window answers"),
    ("shape.set_path", "a command the window answers"),
    ("shape.set", "a command the window answers"),
    ("shape.delete", "a command the window answers"),
    // D-738 (b), accepted on 2026-10-21: shape groups.
    ("shape.group", "a command the window answers"),
    ("shape.ungroup", "a command the window answers"),
    // D-738 (d), accepted on 2026-10-21: path operations.
    ("shape.operation", "a command the window answers"),
    // D-738 (c), accepted on 2026-10-21: repeaters.
    ("shape.repeater", "a command the window answers"),
    // D-79, accepted on 2026-09-22: one pair for both kinds of path, since a mask's points
    // and a shape's are the same points.
    ("path.add_point", "a command the window answers"),
    ("path.remove_point", "a command the window answers"),
    // D-82, accepted on 2026-09-23 and built in the core by B-26b; B-26c put it in the window.
    ("layer.add_null", "a command the window answers"),
    // D-83, accepted on 2026-09-23: the value whip; the parent whip sends `layer.set_parent`.
    ("property.link", "a command the window answers"),
    // D-84, accepted on 2026-09-24 and built in the core by B-28b; B-28c put it in the window.
    ("timesheet.import", "a command the window answers"),
    // D-84d, accepted on 2026-09-24: B-28g. The page asks for `/sheet/print` and prints it.
    ("sheet.print", "the page, with no request"),
    ("timeline.previous_frame", "the page, with no request"),
    ("timeline.next_frame", "the page, with no request"),
    ("timeline.previous_drawing", "the page, with no request"),
    ("timeline.next_drawing", "the page, with no request"),
    ("timeline.play_pause", "the page, with no request"),
    ("timeline.set_work_start", "a command the window answers"),
    ("timeline.set_work_end", "a command the window answers"),
    ("timeline.set_markers", "a command the window answers"),
    // D-765, accepted on 2026-10-23; B-645 built it.
    ("layer.set_markers", "a command the window answers"),
    ("exposure.set_span", "a command the window answers"),
    // D-84b, accepted on 2026-09-24; B-28e built it, from the Sheet.
    ("exposure.write", "a command the window answers"),
    // D-84g, accepted on 2026-09-24; B-28j built both, from the Sheet.
    ("exposure.toggle_key", "a command the window answers"),
    ("sheet.write_action", "a command the window answers"),
    // D-261, accepted on 2026-10-02: the Sketch workspace.
    ("sketch.set_layer", "a command the window answers"),
    ("sketch.remove_layer", "a command the window answers"),
    ("sketch.add_stroke", "a command the window answers"),
    ("sketch.clear", "a command the window answers"),
    // D-282 to D-284, accepted on 2026-10-03.
    ("sketch.set_look", "a command the window answers"),
    ("sketch.move_layer", "a command the window answers"),
    ("sketch.remove_strokes", "a command the window answers"),
    ("sketch.move_strokes", "a command the window answers"),
    ("property.set_base", "a command the window answers"),
    ("keyframe.add_remove", "a command the window answers"),
    ("keyframe.move", "a command the window answers"),
    // D-314, from D-309: Time-Reverse Keyframes.
    ("keyframe.reverse", "a command the window answers"),
    ("keyframe.set_interp", "a command the window answers"),
    ("keyframe.set_path", "a command the window answers"),
    // D-69, accepted by the owner on 2026-09-18; B-19g carries it out.
    ("keyframe.set_kind", "a command the window answers"),
    ("keyframe.set_roving", "a command the window answers"),
    ("property.separate", "a command the window answers"),
    // D-59, accepted by the owner on 2026-09-16; B-14c carries it out.
    ("property.set_expression", "a command the window answers"),
    ("effect.add", "a command the window answers"),
    ("effect.delete", "a command the window answers"),
    ("effect.toggle_bypass", "a command the window answers"),
    ("effect.set_parameters", "a command the window answers"),
    ("effect.move_up", "a command the window answers"),
    ("effect.move_down", "a command the window answers"),
    ("effect.move", "a command the window answers"),
    ("effect.paste", "a command the window answers"),
    ("effect.fit_scale", "a command the window answers"),
    ("viewer.fit", "the page, with no request"),
    ("viewer.zoom_100", "the page, with no request"),
    ("viewer.toggle_checkerboard", "a command the window answers"),
    ("viewer.toggle_alpha", "a command the window answers"),
    ("render.preview_current", "the page, with no request"),
    ("export.sequence", "a route the shell answers"),
    ("app.command_palette", "the page, with no request"),
    ("app.effects_console", "the page, with no request"),
];

/// Document 24's identifier, and the text in the page that binds the shortcut it promises.
///
/// Only the identifiers document 24 gives a G1 shortcut appear here. `none` in that column
/// is not a gap and is not listed.
const SHORTCUTS: &[(&str, &str, &str)] = &[
    (
        "project.new",
        "Ctrl+Alt+N",
        "e.preventDefault(); newProject();",
    ),
    ("project.open", "Ctrl+O", "e.ctrlKey && (e.key === 'o'"),
    ("project.save", "Ctrl+S", "e.ctrlKey && (e.key === 's'"),
    ("project.save_as", "Ctrl+Shift+S", "e.shiftKey ? '/save-as'"),
    (
        "project.save_increment",
        "Ctrl+Alt+Shift+S",
        "e.altKey && e.shiftKey ? '/save-increment'",
    ),
    (
        "composition.create",
        "Ctrl+N",
        "!e.shiftKey && (e.key === 'n'",
    ),
    ("edit.undo", "Ctrl+Z", "e.key === 'z'"),
    ("edit.redo", "Ctrl+Shift+Z", "e.shiftKey ? $('redo')"),
    ("media.import", "Ctrl+I", "e.key === 'i'"),
    ("layer.create", "Ctrl+Alt+L", "e.altKey && (e.key === 'l'"),
    (
        "layer.add_adjustment",
        "Ctrl+Alt+Y",
        "e.altKey && !e.shiftKey && (e.key === 'y'",
    ),
    ("layer.add_solid", "Ctrl+Y", "$('addsolid').click();"),
    (
        "layer.add_null",
        "Ctrl+Alt+Shift+Y",
        "e.shiftKey && (e.key === 'Y'",
    ),
    (
        "layer.precompose",
        "Ctrl+Shift+C",
        "e.shiftKey && !typing && e.code === 'KeyC'",
    ),
    ("layer.delete", "Delete", "e.key === 'Delete'"),
    ("layer.rename", "F2", "e.key === 'F2'"),
    ("layer.move_up", "Ctrl+]", "e.key === ']'"),
    ("layer.move_down", "Ctrl+[", "e.key === '['"),
    ("layer.shift", "[ or ]", "e.key === '[' || e.key === ']'"),
    (
        "layer.trim",
        "Alt+[ or Alt+]",
        "e.key === '[' || e.key === ']'",
    ),
    (
        "layer.move",
        "Ctrl+Shift+] or Ctrl+Shift+[",
        "e.code === 'BracketRight' || e.code === 'BracketLeft'",
    ),
    ("layer.duplicate", "Ctrl+D", "e.code === 'KeyD'"),
    ("layer.split", "Ctrl+Shift+D", "e.shiftKey ? splitLayers()"),
    ("timeline.previous_frame", "Left", "e.key === 'ArrowLeft'"),
    ("timeline.next_frame", "Right", "e.key === 'ArrowRight'"),
    ("timeline.previous_drawing", "Shift+,", "e.code === 'Comma'"),
    ("timeline.next_drawing", "Shift+.", "e.code === 'Period'"),
    ("timeline.play_pause", "Space", "e.key === ' '"),
    ("keyframe.set_interp", "F9", "e.key === 'F9'"),
    ("timeline.set_work_start", "B", "setWork('start')"),
    ("timeline.set_work_end", "N", "setWork('end')"),
    ("timeline.set_markers", "*", "e.key === '*'"),
    ("layer.set_markers", "* with a layer selected", "addMarkerHere()"),
    ("viewer.fit", "Shift+/", "e.key === '?'"),
    ("viewer.zoom_100", "Ctrl+1", "e.key === '1'"),
    ("export.sequence", "Ctrl+M", "e.key === 'm'"),
    ("app.command_palette", "Ctrl+Shift+P", "e.code === 'KeyP'"),
    (
        "sheet.print",
        "Ctrl+P",
        "e.ctrlKey && !e.altKey && (e.key === 'p'",
    ),
    ("composition.set_settings", "Ctrl+K", "e.code === 'KeyK'"),
    ("layer.copy", "Ctrl+C", "copyLayers()"),
    ("layer.paste", "Ctrl+V", "pasteLayers()"),
    ("effect.paste", "Ctrl+V", "pasteCopiedEffects()"),
    ("app.effects_console", "Ctrl+Space", "fxConsole()"),
    ("layer.toggle_lock", "Ctrl+L", "toggleLayers('lock')"),
    ("layer.replace_source", "Ctrl+Alt+/", "e.code === 'Slash'"),
    (
        "layer.toggle_visibility",
        "Ctrl+Alt+V",
        "toggleLayers('visibility')",
    ),
    // B-24c: the tools that draw a mask. The key does not send the command - it takes up a
    // tool, and the mask is sent when the shape closes - so what is checked here is that the
    // key document 15 promises the pen really reaches for it.
    (
        "mask.add",
        "G for the pen, Q for the rectangle and the ellipse",
        "useTool('pen')",
    ),
];

/// The accelerators that work by pressing a button, and the button each one presses.
///
/// The table above only sees that a key is tested for somewhere in the handler. Moving
/// `Delete` onto the Forward button went straight through it, because the key test was still
/// there and only what it did had changed. This reads the arm each key opens - the text from
/// its test to the next `else if` - and says which control that arm reaches for.
const PRESSES: &[(&str, &str, &str)] = &[
    ("Ctrl+M", "e.key === 'm'", "$('export')"),
    ("Ctrl+I", "e.key === 'i'", "$('import')"),
    ("Ctrl+N", "e.key === 'n'", "$('newcomp')"),
    ("Ctrl+Alt+L", "e.key === 'l'", "$('addlayer')"),
    ("Ctrl+Alt+Y", "e.key === 'y'", "$('addadjust')"),
    (
        "Ctrl+Alt+Shift+Y",
        "e.shiftKey && (e.key === 'Y'",
        "$('addnull')",
    ),
    ("Ctrl+]", "e.key === ']'", "$('up')"),
    ("Ctrl+[", "e.key === '['", "$('down')"),
    ("Delete", "e.key === 'Delete'", "$('dellayer')"),
    ("Space", "e.key === ' ') {", "$('play')"),
    ("D", "e.key === 'd'", "$('toggle')"),
    ("Alt+A", "e.key === 'a'", "$('alpha')"),
    // B-24c: the checkerboard moved to Alt+G when document 15 gave the pen G.
    ("Alt+G", "e.altKey && (e.key === 'g'", "$('checker')"),
    ("Shift+/", "e.key === '?'", "$('fit')"),
];

/// Document 24's command table, read out of the document itself.
fn document_24() -> Vec<(String, String)> {
    let text = std::fs::read_to_string(repo("Markdown/24_UI_Command_and_Interaction_Map.md"))
        .expect("read document 24");
    text.lines()
        .filter(|line| line.starts_with('|'))
        .map(|line| {
            line.trim_matches('|')
                .split('|')
                .map(str::trim)
                .collect::<Vec<_>>()
        })
        .filter(|cells| cells.len() == 4 && cells[0].contains('.'))
        .map(|cells| (cells[0].to_string(), cells[2].to_string()))
        .collect()
}

#[test]
fn every_command_document_24_names_is_accounted_for() {
    let mut report = Report { rows: Vec::new() };
    let page = page();
    let keys = accelerators(&page);
    let listed = document_24();
    let viewer = Mutex::new(demo());
    let source = source();
    let asked = asked_for(&page);

    report.check(
        "document 24 names the identifiers this table walks",
        REACHED.len(),
        listed.len(),
    );
    report.check(
        "and they are the same identifiers, in the same order",
        REACHED
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>()
            .join(", "),
        listed
            .iter()
            .map(|(id, _)| id.as_str())
            .collect::<Vec<_>>()
            .join(", "),
    );

    for (id, expected) in REACHED {
        // Measured four ways, in the order a request would find them: the shell answers a
        // route before `edit_command` sees it, `edit_command` answers a command, the page
        // may do something with no request at all, and otherwise nothing does.
        let route = match *id {
            "project.new" => "new",
            "project.open" => "open",
            "project.save" => "save",
            "project.save_as" => "save-as",
            "project.save_increment" => "save-increment",
            "export.sequence" => "export",
            "project.collect" => "collect",
            "package.check" => "check-package",
            _ => "",
        };
        let reached = if !route.is_empty() && source.contains(&format!("\"{route}\"")) {
            "a route the shell answers"
        } else if edit_command(&viewer, id, None).is_some() {
            "a command the window answers"
        } else if asked.iter().any(|sent| sent == id) {
            "the page sends it and nothing answers"
        } else if PAGE_SIDE.contains(id) {
            "the page, with no request"
        } else {
            "nothing yet"
        };
        report.check(&format!("`{id}` is reached by"), expected, reached);
    }
    for id in ["viewer.toggle_alpha", "viewer.toggle_checkerboard"] {
        edit_command(&viewer, id, None);
    }

    for (id, shortcut, binding) in SHORTCUTS {
        let promised = listed
            .iter()
            .find(|(listed_id, _)| listed_id == id)
            .map(|(_, shortcut)| shortcut.clone())
            .unwrap_or_else(|| "(not in document 24)".to_string());
        report.check(
            &format!("document 24 gives `{id}` the shortcut"),
            shortcut,
            promised,
        );
        let reached = REACHED
            .iter()
            .find(|(reached_id, _)| reached_id == id)
            .map(|(_, how)| *how)
            .unwrap_or("nothing yet");
        report.check(
            &format!("and {shortcut} is bound"),
            match reached {
                "nothing yet" => "no, and the command is not built either",
                _ => "yes",
            },
            match (binding.is_empty(), keys.contains(binding)) {
                (true, _) => "no, and the command is not built either",
                (false, true) => "yes",
                (false, false) => "no, though the command is built",
            },
        );
    }

    for (shortcut, key, control) in PRESSES {
        let arm = match keys.find(key) {
            Some(at) => {
                let rest = &keys[at..];
                // The next key's arm starts at the handler's own indent; an arm may nest
                // an else if of its own, as Delete does for chosen mask points (B-24i).
                &rest[..rest.find("\n  else if").unwrap_or(rest.len())]
            }
            None => "",
        };
        report.check(
            &format!("and {shortcut} presses"),
            *control,
            match arm.split_once("$('") {
                Some((_, rest)) => format!("$('{}", &rest[..rest.find(')').unwrap_or(0) + 1]),
                None => format!("nothing - no arm tests for {key}"),
            },
        );
    }

    write_artifact(
        &report,
        "verification/B-12b_command_map_table.md",
        "B-12b: document 24's command map against the window",
        MAP_INTRO,
        MAP_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// The three document 24 identifiers the page carries out itself.
///
/// Stepping and playback change nothing in the project - document 24 marks all three not
/// undoable - so they are a number the page holds and a frame it asks for, and there is no
/// command for them to send. `render.preview_current` is the same thing said differently:
/// every frame this window shows is rendered when it is asked for.
const PAGE_SIDE: &[&str] = &[
    "timeline.previous_frame",
    "timeline.next_frame",
    // D-733: the previous and next drawing, read off the Sheet the page already asks for.
    "timeline.previous_drawing",
    "timeline.next_drawing",
    "timeline.play_pause",
    "render.preview_current",
    "viewer.fit",
    "viewer.zoom_100",
    "app.command_palette",
    "app.effects_console",
    "sheet.print",
];

const MAP_INTRO: &[&str] = &[
    "Document 24 names thirty-six command identifiers. This walks all thirty-six against \
     the build and says, for each, what actually reaches it - and then walks the twenty-one \
     it gives a keyboard shortcut and says whether that shortcut is bound. The identifiers \
     and the shortcuts are read out of `Markdown/24_UI_Command_and_Interaction_Map.md` \
     itself, so an edit to the document that nothing implements fails here.",
    "There are four ways a command is reached in this build, and the second column of every \
     row is one of them. **A route the shell answers** is one that needs the operating system \
     - opening, saving, exporting - and is answered before the command layer sees it. **A \
     command the window answers** goes through `edit_command`, which is the command layer \
     document 24's first paragraph describes. **The page, with no request** is the transport: \
     stepping and playing change nothing in the project, so there is nothing to send. \
     **Nothing yet** is a command this build does not have.",
];

const MAP_NOTES: &[&str] = &[
    "## What to look at\n\n- **`nothing yet` is checked as hard as the rest.** A row that \
     says a command is not built is a claim, and the test confirms nothing answers that \
     identifier. This is what stops the list quietly going stale after somebody builds \
     one.\n- **`the page sends it and nothing answers`** is a value no row expects, and \
     seeing it in the Actual column would mean a button that does nothing.\n- **The shortcut \
     rows come in pairs.** The first says what document 24 promises, read from the document. \
     The second says whether the page binds it.\n- **The `presses` rows say what the key \
     does, not only that it is bound.** A shortcut moved onto the wrong button keeps its key \
     test and stops doing its job; those rows are the ones that would say so.",
    "## The commands built last\n\n- **`project.new`, `timeline.set_work_start`, \
     `timeline.set_work_end` and `app.command_palette`** were built by W-24, with \
     `timeline.set_markers`, `layer.set_label` and `layer.toggle_solo` beside them, so no \
     row says `nothing yet` any more. Ctrl+N asks twice when the project has unsaved \
     work. The palette is the page's own: each line presses the key it names.\n- \
     **`viewer.fit` and `viewer.zoom_100`** are the page's own since W-06: a zoom is a \
     size the page gives the canvas and a scroll of the stage around it, and nothing in \
     the project changes, so neither sends a request.",
    "## What this cannot cover\n\nThat a bound shortcut reaches the command. The binding is \
     read as text in the page's accelerator handler; that pressing the key really runs it is \
     `verification/B-12a_window_and_keyboard.md`, where a Q-03 photograph found exactly that \
     defect - rows that Tab reached and no key could operate.\n\nWhether document 24's \
     shortcuts conflict with Windows or with the web view, which document 24 itself asks for \
     and which needs a person at the keyboard.",
];

/// One table with its own prose around it.
fn write_artifact(report: &Report, file: &str, title: &str, intro: &[&str], notes: &[&str]) {
    let passed = report.rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = format!("# {title}\n\n");
    out.push_str(
        "Generated by `app/src/main.rs`, module `contract`. Re-run with `cargo test \
         --workspace`.\n\n",
    );
    for paragraph in intro.iter().chain(notes) {
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
    std::fs::write(repo(file), out).expect("write the artifact");
}

// ---- document 26, the text half of coalescing ---------------------------------------------

/// The three places in the page where somebody types into a field and the window is sent a
/// command: the control, the text that wires it, and what commits it.
///
/// The layer's name uses `onblur`, and since W-14 both numbers use the one rule every
/// number in this window now keeps: what is typed is sent when it is committed, by Enter or
/// by leaving the field, and never per keystroke - "a half-typed `-` is not a value anybody
/// meant", which is what the transform fields have said since W-04. That is document 26's
/// sentence about text, "committing/focus exit ends the transaction", and it is what After
/// Effects does, which is what the fifth sitting asked these numbers to be like.
///
/// Until W-14 an effect's setting was the one that went per keystroke, inside the drag
/// transaction the picture's drags use. The second sitting's finding 10 was that the effect
/// "did not show" until a click elsewhere took the focus; Enter now commits it where before
/// only a click elsewhere did, so the complaint that bought that liveness is answered
/// without it. Swapping a commit for a keystroke is a one-word edit, which is why the line
/// that commits is pinned rather than trusted.
const TYPED_FIELDS: &[(&str, &str, &str)] = &[
    (
        "a layer's name is committed by losing focus",
        "layer.rename",
        "box.onblur = () => finish(true);",
    ),
    (
        "an effect's settings are sent when the number is committed, by Enter or by leaving \
         the field",
        "effect.set_parameters",
        "commit: () => sendParameters('', name),",
    ),
    (
        "an exposure's frames are committed by Enter or by leaving the field",
        "exposure.set_span",
        "commit: sendSpan,",
    ),
];

#[test]
fn typing_into_a_field_is_one_history_entry_when_it_is_committed() {
    let mut report = Report { rows: Vec::new() };
    let page = page();

    // The page half. Nothing here can press a key -- what it can say is that no handler in
    // the page attached to the event that fires per keystroke sends a command. Since W-06
    // the zoom slider has such a handler, and a zoom asks the window for nothing, so the
    // statement each handler is in is read for a request rather than the handler counted.
    let per_keystroke: Vec<&str> = page
        .match_indices("oninput")
        .chain(page.match_indices("addEventListener('input'"))
        .map(|(at, _)| {
            let rest = &page[at..];
            let end = [rest.find(";\n"), rest.find(";\r")]
                .into_iter()
                .flatten()
                .min();
            &rest[..end.map_or(rest.len(), |end| end + 1)]
        })
        .filter(|statement| statement.contains("command("))
        .collect();
    report.check(
        "no field in the page sends anything while a key is being pressed",
        "no input event handler sends a command",
        match per_keystroke.is_empty() {
            true => "no input event handler sends a command".to_string(),
            false => format!("{} do: {}", per_keystroke.len(), per_keystroke.join(" / ")),
        },
    );
    for (control, id, wiring) in TYPED_FIELDS {
        report.check(
            &format!("{control}, and sends `{id}`"),
            true,
            page.contains(wiring),
        );
    }
    report.check(
        "what an effect's field sends per keystroke is inside a drag",
        true,
        page.contains(
            "const live = (changed) => { open = true; dragging = true; sendParameters('&drag=1', changed); };",
        ),
    );
    report.check(
        "and a name committed unchanged sends nothing at all",
        true,
        page.contains("if (commit && wanted !== layer.name) {"),
    );

    // The window half. One committed field, one entry in the undo list, named in the words
    // the person will read on the Undo button.
    let viewer = Mutex::new(demo());
    let before = held(&viewer).document.undo_depth();
    run(&viewer, "layer.rename?layer=layer-1&name=the background");
    report.check(
        "committing a new name is one entry in the undo list",
        format!("undo list {}", before + 1),
        format!("undo list {}", held(&viewer).document.undo_depth()),
    );
    report.check(
        "and the entry says what it will undo",
        "Rename layer to the background",
        held(&viewer)
            .document
            .undo_labels()
            .last()
            .unwrap_or(&"(nothing)")
            .to_string(),
    );

    run(&viewer, "effect.add?layer=layer-1&type=core.gaussian_blur");
    let before = held(&viewer).document.undo_depth();
    for sigma in [2, 6, 9] {
        run(
            &viewer,
            &format!("effect.set_parameters?layer=layer-1&effect=fx-1&sigma_px={sigma}"),
        );
    }
    report.check(
        "three settings committed one after another are three entries, not one and not thirty",
        format!("undo list {}", before + 3),
        format!("undo list {}", held(&viewer).document.undo_depth()),
    );
    run(&viewer, "edit.undo");
    report.check(
        "so undoing once goes back one commit, not back to before the field was touched",
        "GaussianBlur { sigma_px: 6.0, edges: \"transparent\", dimensions: \"both\", units: \"sigma\" }",
        layer(&viewer, "layer-1", |l| {
            l.effects
                .iter()
                .map(|e| format!("{:?}", e.effect))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "(no such layer)".to_string()),
    );

    // W-07: the same field typed live. Three keystrokes travel inside one drag and losing
    // the focus closes it, so the three are one entry and the entry holds the last of them.
    let before = held(&viewer).document.undo_depth();
    for sigma in ["3", "3.", "3.5"] {
        run(
            &viewer,
            &format!("effect.set_parameters?layer=layer-1&effect=fx-1&sigma_px={sigma}&drag=1"),
        );
    }
    run(&viewer, "property.drag_end");
    report.check(
        "a setting typed in three keystrokes, each sent as it lands, is one entry",
        format!("undo list {}", before + 1),
        format!("undo list {}", held(&viewer).document.undo_depth()),
    );
    report.check(
        "and the entry holds the last keystroke, not the first",
        "GaussianBlur { sigma_px: 3.5, edges: \"transparent\", dimensions: \"both\", units: \"sigma\" }",
        layer(&viewer, "layer-1", |l| {
            l.effects
                .iter()
                .map(|e| format!("{:?}", e.effect))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "(no such layer)".to_string()),
    );

    // The numeric half of the same rule, for contrast: a drag is many requests and one
    // entry, because document 26 makes the release the commit.
    let before = held(&viewer).document.undo_depth();
    for x in [100.0_f64, 140.0, 180.0] {
        run(
            &viewer,
            &format!("property.drag_update?layer=layer-2&prop=position&value={x}, 0"),
        );
    }
    run(&viewer, "property.drag_end?layer=layer-2&prop=position");
    report.check(
        "and a drag of three steps is one entry, which is the same rule for a number",
        format!("undo list {}", before + 1),
        format!("undo list {}", held(&viewer).document.undo_depth()),
    );

    // Two layers selected on the picture and dragged together. Still one press, still one
    // release, so still one entry - and both layers have to be in it, or undoing would put
    // one of them back and leave the other where the pointer left it.
    let before = held(&viewer).document.undo_depth();
    for x in [200.0_f64, 240.0, 300.0] {
        for layer in ["layer-1", "layer-2"] {
            run(
                &viewer,
                &format!("property.drag_update?layer={layer}&prop=position&value={x}, 0"),
            );
        }
    }
    run(&viewer, "property.drag_end");
    // Each of these takes the lock and gives it straight back. Two `held(&viewer)` calls
    // inside one `format!` would both still be alive when the second one asked for it, and
    // the second would wait for the first for ever.
    let depth = held(&viewer).document.undo_depth();
    let entry = held(&viewer)
        .document
        .undo_labels()
        .last()
        .unwrap_or(&"(nothing)")
        .to_string();
    report.check(
        "a drag holding two layers is one entry that says it moved more than one",
        format!(
            "undo list {}, Set position to (300, 0) and 1 more",
            before + 1
        ),
        format!("undo list {depth}, {entry}"),
    );
    report.check(
        "and undoing that one entry puts both layers back, not just the first",
        "layer-1 (0, 0), layer-2 (180, 0)",
        {
            run(&viewer, "edit.undo");
            ["layer-1", "layer-2"]
                .iter()
                .map(|id| {
                    format!(
                        "{id} {}",
                        layer(&viewer, id, |l| l.transform.position.base().to_string())
                            .unwrap_or_else(|| "(no such layer)".to_string())
                    )
                })
                .collect::<Vec<_>>()
                .join(", ")
        },
    );

    // W-05: a layer's bar dragged along the timeline. The same transaction with a different
    // command inside it, and the flag in the query is what puts it there.
    let before = held(&viewer).document.undo_depth();
    let (in_0, out_0) =
        layer(&viewer, "layer-2", |l| (l.in_frame, l.out_frame)).expect("layer-2 is in the demo");
    for at in [4, 9, 12] {
        run(
            &viewer,
            &format!("layer.shift?layer=layer-2&in={at}&drag=1"),
        );
    }
    run(&viewer, "property.drag_end");
    let depth = held(&viewer).document.undo_depth();
    let frames = |viewer: &Mutex<Viewer>| {
        layer(viewer, "layer-2", |l| {
            format!("{} to {}", l.in_frame, l.out_frame)
        })
        .unwrap_or_else(|| "(no such layer)".to_string())
    };
    report.check(
        "a layer's bar dragged three steps along the timeline is one entry",
        format!("undo list {}, 12 to {}", before + 1, 12 + out_0 - in_0),
        format!("undo list {depth}, {}", frames(&viewer)),
    );
    run(&viewer, "edit.undo");
    report.check(
        "and undoing it puts the bar back where it was",
        format!("{in_0} to {out_0}"),
        frames(&viewer),
    );

    write_artifact(
        &report,
        "verification/B-12b_text_coalescing_table.md",
        "B-12b: one entry in the undo list per thing typed",
        TEXT_INTRO,
        TEXT_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

fn held(viewer: &Mutex<Viewer>) -> std::sync::MutexGuard<'_, Viewer> {
    viewer.lock().expect("the viewer lock was poisoned")
}

fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = match what.split_once('?') {
        Some((id, query)) => (id, Some(query)),
        None => (what, None),
    };
    edit_command(viewer, id, query).expect("a command document 24 lists")
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

const TEXT_INTRO: &[&str] = &[
    "Document 26: \"Text edits may coalesce while one field has focus; committing or focus \
     exit ends the transaction.\" In plain terms, the promise is that typing `1`, `2`, `0` \
     into a field is one thing to undo and not three, and that leaving a field you did not \
     change is not a thing to undo at all.",
    "There are three fields in this window a person types into: a layer's name, an effect's \
     settings, and the frames of an exposure. All three keep that promise the same way, and \
     it is worth saying plainly because it is not code anybody in this project wrote: they \
     are wired to the browser's `change` event, which fires when a field is committed **and** \
     its value is not the one it had when it took focus. The alternative event, `input`, \
     fires on every keystroke, and a field wired to it would put one entry in the undo list \
     per letter. The first row below is the check that no field in this page is.",
];

const TEXT_NOTES: &[&str] = &[
    "## What this cannot cover\n\nNo test in this project presses a key. The rows about the \
     page read the file and say which event each field is attached to; what the browser then \
     does with that event is the browser's, and is documented behaviour rather than \
     something measured here. The rows about the window send the command a committed field \
     would send and count what lands in the undo list, which is the half that is this \
     project's own.\n\nThe last three rows are not about text at all. They drag a position \
     through several values and end the drag, and are here because they are the same \
     sentence of document 26 read the other way: many requests, one entry. Without them a \
     reader has no way to see that three entries for three committed settings is the \
     intended answer rather than the same defect in the other direction. The last two of \
     them have two layers under the pointer at once, which is that promise again with more \
     than one thing being moved: one entry, both layers named in it, and undoing brings both \
     of them back rather than the first.",
];

// ---- what the panels draw ------------------------------------------------------------------

/// The variables the page reads project data out of, and where each one comes from in the
/// answer `/state` gives.
///
/// The fields themselves are not written down here. They are read out of the page, so a panel
/// that starts reading something new is checked from the moment it does rather than when
/// somebody remembers to add a row.
const READ_FROM: &[(&str, &str)] = &[
    ("doc", "the whole answer"),
    ("comp", "the composition on screen"),
    ("layer", "one of its layers"),
    ("asset", "one sequence in the media bin"),
    ("span", "one exposure on a layer"),
    ("fx", "one effect in a layer's stack"),
];

/// Words that follow a dot in this page and are not fields of the project.
///
/// Every one of them is JavaScript's own - a method or a property of the language rather than
/// of the document. Kept short deliberately: anything not on this list is treated as a field
/// the panels read, so the mistake this list can make is letting a check through, never
/// inventing one.
const NOT_OURS: &[&str] = &[
    "concat", "every", "filter", "find", "forEach", "includes", "indexOf", "join", "length", "map",
    "push", "slice", "some", "sort", "split", "has", "trim", "toFixed", "value",
];

/// Every `<var>.<field>` the page reads, for one variable name.
///
/// The character in front of the name is what separates a field read from a command
/// identifier: `layer.move_up` inside `send('layer.move_up')` follows a quote, and
/// `'/layer.create?asset='` follows a slash. Neither is a field.
fn fields_read(page: &str, var: &str) -> Vec<String> {
    let needle = format!("{var}.");
    let mut found = Vec::new();
    for (at, _) in page.match_indices(&needle) {
        let before = page[..at].chars().next_back().unwrap_or(' ');
        if before == '/'
            || before == '\''
            || before == '.'
            || before == '_'
            || before.is_alphanumeric()
        {
            continue;
        }
        let rest = &page[at + needle.len()..];
        let end = rest
            .find(|c: char| !(c.is_ascii_alphanumeric() || c == '_'))
            .unwrap_or(rest.len());
        let field = &rest[..end];
        if field.is_empty() || NOT_OURS.contains(&field) {
            continue;
        }
        found.push(field.to_string());
    }
    sorted(found)
}

/// A layer of the answer, by identifier.
fn layer_of<'a>(answer: &'a serde_json::Value, id: &str) -> &'a serde_json::Value {
    let comp = answer["project"]["compositions"]
        .as_array()
        .expect("the answer holds a list of compositions")
        .iter()
        .find(|c| c["id"] == answer["composition"])
        .expect("the composition the viewer says is on screen");
    comp["layers"]
        .as_array()
        .expect("the composition holds a list of layers")
        .iter()
        .find(|l| l["id"] == id)
        .unwrap_or_else(|| panic!("no layer {id} in the composition"))
}

/// B-13f: every request built from a chosen key's own row names its target, not a layer.
///
/// The camera's keys ride the same `layer|prop|frame` ids a layer's keys ride, and `keyParts`
/// hands back the camera's row for the one id that names it. A send built from that row with
/// `layer=` therefore puts the window's own `::camera` name where a layer id belongs, and the
/// window refuses it by name. Three functions did exactly that - F9, the hold and the paste -
/// and B-13e's notes, document 24, the backlog and a playtest step all said they did not.
///
/// The page is not run by this suite, so this is a text pin like `fields_read` beside it: the
/// body of each function that acts on chosen keys must not spell `layer=` anywhere. `whose`
/// and `whoseId` are the only two spellings allowed, and they are the two that know a camera
/// is named rather than identified.
#[test]
fn the_gestures_that_act_on_chosen_keys_name_their_target() {
    let page = page();
    let body = |name: &str| -> String {
        let at = page
            .find(&format!("function {name}("))
            .unwrap_or_else(|| panic!("the page has no function called {name}"));
        let rest = &page[at..];
        let end = rest
            .find("\n}\n")
            .unwrap_or_else(|| panic!("{name} does not end at the margin"));
        rest[..end].to_string()
    };
    for name in [
        "easeKeys",
        "holdKeys",
        "pasteKeys",
        "copyKeys",
        "moveKeys",
        "removeKeys",
    ] {
        assert!(
            !body(name).contains("layer="),
            "{name} spells a layer into its request, so a camera key cannot travel through \
             it: everything built from a chosen key's row goes through `whose` or `whoseId`"
        );
    }
    // The one place that turns a row's id into the word a request carries. If this moves,
    // the pin above is checking the absence of something nothing produces any more.
    assert!(
        page.contains("const whoseId = (id) => (id === CAMERA_ROW"),
        "the single place that decides how a row is named to the window has moved"
    );
}

#[test]
fn every_field_the_panels_read_is_in_the_answer_the_window_gives() {
    let mut report = Report { rows: Vec::new() };
    let page = page();
    let viewer = Mutex::new(demo());
    // The reference shot carries no effect and no matte, because nothing in it needs one, and
    // those are the two panels most likely to be reading a name that has moved. They are put
    // there with the same commands a person would use.
    for kind in ["core.gaussian_blur", "core.exposure", "core.tint"] {
        run(&viewer, &format!("effect.add?layer=layer-3&type={kind}"));
    }
    run(
        &viewer,
        "layer.set_matte?layer=layer-3&matte=layer-2&only=false",
    );
    // W-24: a work area, a marker and a label are written only when there is one, so the
    // ruler and the label colours are given one each the same way. W-26: and a shy layer.
    // B-13b: and a parent, which D-57 writes only when the layer rides on something.
    // B-13c: and a depth and a camera, which D-58 writes only where one has been set. The
    // inspector reads both through a default, so their absence is not what this test is
    // about; that the two names are spelled the way the answer spells them is.
    // B-15c: and D-61's tick, written only when unticked.
    let first_asset = held(&viewer).document.project().assets[0].id.clone();
    run(
        &viewer,
        &format!("asset.set_redistribute?asset={first_asset}&redistribute=false"),
    );
    for edit in [
        "timeline.set_work_start?frame=1",
        "timeline.set_markers?marker=2|hit",
        // D-765: and a layer marker with a length, written only when the layer has one.
        "layer.set_markers?layer=layer-3&marker=2%2B3|beat",
        "layer.set_label?layer=layer-3&label=2",
        "layer.toggle_shy?layer=layer-3",
        // B-124b: and D-188's two switches, each written only when on.
        "layer.toggle_motion_blur?layer=layer-3",
        // B-708: and D-828's T switch, written only when on.
        "layer.toggle_preserve_transparency?layer=layer-3",
        "composition.set_motion_blur?enabled=true",
        // B-150b: and D-216's four, each written only when not the default.
        "composition.set_frame_blending?enabled=true",
        "layer.toggle_frame_blend?layer=layer-3",
        "layer.set_time_stretch?layer=layer-3&value=200",
        // D-323: and a Time Remap, written only when on.
        "layer.set_time_remap?layer=layer-3&on=1",
        "layer.set_drawing_dissolve?layer=layer-3&frames=2",
        "layer.set_parent?layer=layer-3&parent=layer-2&frame=0",
        "layer.set_depth?layer=layer-3&depth=640",
        "camera.set_property?property=zoom&value=50",
        // B-18c: and a composition layer, the only kind that carries `composition_id`.
        "layer.precompose?layer=layer-1",
        // B-28i: and D-84f's title block, written only when one of its four is.
        "composition.set_settings?episode=3",
        // D-311: and a background colour, written only when there is one.
        "composition.set_settings?background_color=0.5,0.25,1",
        // D-319: and Float working depth, written only when on.
        "composition.set_settings?float_depth=true",
        // D-261: and a sketch layer with a stroke, written only when there is one. D-271:
        // drawn with a pen, since its pressure is written only then.
        "sketch.set_layer?sketch=sketch-a&name=Notes",
        "sketch.add_stroke?sketch=sketch-a&frame=0&tool=brush&size=4&colour=%23c8302c&points=1,2;3,4&pressure=0.25;1",
    ] {
        run(&viewer, edit);
    }
    // B-20c: and an audio layer with a level, the only kind that carries `gain_db`.
    import(
        &viewer,
        &[
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../Fixtures/audio/media/pcm16_mono_48k.wav"),
        ],
    );
    let sound_asset = held(&viewer)
        .document
        .project()
        .assets
        .iter()
        .find(|a| a.kind == AssetKind::Audio)
        .map(|a| a.id.as_str().to_string())
        .unwrap_or_default();
    run(
        &viewer,
        &format!("layer.create?asset={sound_asset}&name=Sound"),
    );
    let answer: serde_json::Value =
        serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
    let sound_id = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "audio"))
        .and_then(|l| l["id"].as_str())
        .unwrap_or_default()
        .to_string();
    run(
        &viewer,
        &format!("layer.set_gain?layer={sound_id}&value=-6"),
    );
    // B-23c: and a solid, the only kind that carries `solid`.
    run(&viewer, "layer.add_solid");
    // B-28c: and a cut imported from its timesheet, whose layers are the only ones that
    // carry `timesheet`. The import puts the cut on screen, so the one before goes back.
    let before_cut = held(&viewer).composition.clone();
    import_cut(&viewer, &repo("Fixtures/xdts/fx_xdts_040"));
    show(&viewer, &before_cut);
    // D-263: and a text layer, the only kind that carries `source_text`.
    run(&viewer, "layer.add_text");
    // B-25c: and a shape layer with a shape, the only kind that carries `shapes`.
    run(&viewer, "layer.add_shape");
    let shape_id = held(&viewer).document.project().compositions[0]
        .layers_in_order()
        .find(|l| l.kind == anime_compositor::model::LayerKind::Shape)
        .map(|l| l.id.as_str().to_string())
        .unwrap_or_default();
    run(
        &viewer,
        &format!("shape.add?layer={shape_id}&points=0,0,0,0,0,0;9,9,0,0,0,0;0,9,0,0,0,0"),
    );
    // D-738 (b): and that shape in a group, because a layer with none writes no `groups`.
    run(&viewer, &format!("shape.group?layer={shape_id}&shapes=0"));
    // B-24b: and a mask on layer-3, because a layer with none writes no `masks` key at all.
    // It goes in through the core rather than through a URL: the window has no mask command
    // until B-24c, and this check is about the panel reading a field that is there.
    {
        let mut held = held(&viewer);
        let composition = held.composition.clone();
        let _ = held
            .document
            .apply(anime_compositor::command::Command::SetMasks {
                composition,
                layer_id: anime_compositor::model::Id::new("layer-3"),
                masks: vec![anime_compositor::mask::Mask::polygon(vec![
                    (0.0, 0.0),
                    (4.0, 0.0),
                    (4.0, 4.0),
                ])],
            });
    }
    let answer: serde_json::Value =
        serde_json::from_str(&state(&viewer)).expect("the state answer is JSON");
    let sound = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "audio"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let solid = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "solid"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let layer = layer_of(&answer, "layer-3");
    let shape = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "shape"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let words = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "text"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let precomp = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .and_then(|all| all.iter().find(|l| l["kind"] == "composition"))
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    let from_sheet = answer["project"]["compositions"]
        .as_array()
        .into_iter()
        .flatten()
        .flat_map(|c| c["layers"].as_array().into_iter().flatten())
        .find(|l| l.get("timesheet").is_some())
        .cloned()
        .unwrap_or(serde_json::Value::Null);
    // D-202: an effect's Mix is written only when it is not 100, so it is read off a fixture
    // effect mixed to 50.
    let mixed = persist::load(&repo("Fixtures/effect_mix/fx_mix_001.json"))
        .map(|l| persist::to_json(l.document.project(), &l.preserved))
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .map(|p| p["compositions"][0]["layers"][0]["effects"][0].clone())
        .unwrap_or(serde_json::Value::Null);
    // D-330: 8 bpc is written only when on and never with Float, so it is read off a fixture;
    // D-333: 32 bpc (After Effects) too.
    let comp_of = |rel: &str| {
        persist::load(&repo(rel))
            .map(|l| persist::to_json(l.document.project(), &l.preserved))
            .ok()
            .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
            .map(|p| p["compositions"][0].clone())
            .unwrap_or(serde_json::Value::Null)
    };
    let eight = comp_of("Fixtures/eight_bpc/fx_8bpc_001.json");
    // D-645: a pan is written only when set, and only a file sets one, so it is read off one.
    let panned = comp_of("Fixtures/sound_layer/fx_slay_006.json")["layers"][0].clone();
    let ae32 = comp_of("Fixtures/ae_32bpc/fx_ae32_001.json");

    let node = |var: &str| -> serde_json::Value {
        match var {
            "doc" => answer.clone(),
            "comp" => {
                let mut comp = answer["project"]["compositions"]
                    .as_array()
                    .expect("compositions")
                    .iter()
                    .find(|c| c["id"] == answer["composition"])
                    .expect("the composition on screen")
                    .clone();
                // The page reads `comp.layers`, and since W-24 the work area and markers.
                comp["layers"] = comp["layers"].clone();
                comp
            }
            "layer" => layer.clone(),
            "asset" => answer["project"]["assets"][0].clone(),
            "span" => layer["exposure_spans"][0].clone(),
            "fx" => layer["effects"][0].clone(),
            other => panic!("no node for {other}"),
        }
    };

    for (var, where_from) in READ_FROM {
        let holds = node(var);
        let fields = fields_read(&page, var);
        report.check(
            &format!(
                "the panels read {} field{} out of {where_from}",
                fields.len(),
                match fields.len() {
                    1 => "",
                    _ => "s",
                }
            ),
            true,
            !fields.is_empty(),
        );
        for field in fields {
            report.check(
                &format!("`{var}.{field}` is in the answer"),
                "present",
                match holds.get(&field).or_else(|| match field.as_str() {
                    "composition_id" => precomp.get(&field),
                    "gain_db" => sound.get(&field),
                    "pan" => panned.get(&field),
                    "solid" => solid.get(&field),
                    "shapes" | "groups" => shape.get(&field),
                    "source_text" => words.get(&field),
                    "timesheet" => from_sheet.get(&field),
                    "mix" => mixed.get(&field),
                    "eight_bpc" => eight.get(&field),
                    "ae_32bpc" => ae32.get(&field),
                    _ => None,
                }) {
                    Some(_) => "present".to_string(),
                    None => format!("missing - the panel would draw nothing for {field}"),
                },
            );
        }
    }

    // Three reads the loop above cannot see, because they are one level further in and the
    // page reaches them through a local name rather than through `layer`.
    for prop in ["anchor", "position", "scale", "rotation", "opacity"] {
        report.check(
            &format!("`layer.transform.{prop}.base` is in the answer"),
            "present",
            match layer["transform"][prop].get("base") {
                Some(_) => "present",
                None => "missing - the inspector would show an empty field",
            },
        );
    }
    for field in ["layer_id", "matte_only"] {
        report.check(
            &format!("`layer.matte.{field}` is in the answer"),
            "present",
            match layer["matte"].get(field) {
                Some(_) => "present",
                None => "missing - the matte row would forget which layer it is",
            },
        );
    }

    // The masks, which no command in this build can make: the panel says how many points a
    // layer's mask has, and the only way to have one is to open a project that carries one.
    // A wrong name here would not be a blank field, it would be `undefined points`. D-77
    // moved the points two levels in, to `masks[0].path.base.points`, and this is the row
    // that would have gone on reading the old place without saying so.
    let with_a_mask = std::fs::read_to_string(repo("Fixtures/projects/cel_holds_project.json"))
        .expect("read the cel-holds fixture")
        .replace(
            "\"mask\": null",
            "\"mask\": { \"vertices\": [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]], \
             \"enabled\": true, \"inverted\": false }",
        );
    let loaded = persist::load_str(&with_a_mask).expect("a project carrying a mask opens");
    let written: serde_json::Value = serde_json::from_str(&persist::to_json(
        loaded.document.project(),
        &loaded.preserved,
    ))
    .expect("what a save would write is JSON");
    report.check(
        "`layer.masks[0].path.base.points` is in a saved project that has a mask",
        "present",
        match written["compositions"][0]["layers"][0]["masks"][0]["path"]["base"].get("points") {
            Some(_) => "present",
            None => "missing - the inspector would say `undefined points`",
        },
    );

    // The effect panel's own vocabulary. These are not fields of the answer, they are the
    // names the panel puts in a request, and a name the command does not read is a field a
    // person can type into that changes nothing.
    for (type_id, params) in EFFECT_KINDS {
        // D-642: a sound effect goes on the sound layer and only there.
        let on = match *type_id {
            "core.noise_gate" | "core.compressor" | "core.distortion" => sound_id.as_str(),
            _ => "layer-4",
        };
        report.check(
            &format!("the panel's `{type_id}` is an effect this build has"),
            "added",
            match run(&viewer, &format!("effect.add?layer={on}&type={type_id}")) {
                said if said.starts_with("Add ") => "added".to_string(),
                said => said,
            },
        );
        let effects = layer_of(
            &serde_json::from_str::<serde_json::Value>(&state(&viewer)).expect("JSON"),
            on,
        )["effects"]
            .clone();
        // Found by its type: line smoothing and selective colour blur go to the top of the
        // stack (D-86, D-87), not the end.
        let instance = effects
            .as_array()
            .and_then(|e| e.iter().rev().find(|e| e["type_id"] == *type_id))
            .and_then(|e| e["instance_id"].as_str())
            .expect("the effect just added has an identifier")
            .to_string();
        let sent = params
            .iter()
            .map(|(name, value)| format!("&{name}={value}"))
            .collect::<String>();
        report.check(
            &format!(
                "and the settings it sends for it - {} - are the ones the command reads",
                params
                    .iter()
                    .map(|(name, _)| *name)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "accepted",
            match run(
                &viewer,
                &format!("effect.set_parameters?layer={on}&effect={instance}{sent}"),
            ) {
                said if said.starts_with("Set ") || said.starts_with("Change ") => {
                    "accepted".to_string()
                }
                said => said,
            },
        );
    }

    write_artifact(
        &report,
        "verification/B-12b_state_fields_table.md",
        "B-12b: what the panels read, against what the window sends them",
        FIELDS_INTRO,
        FIELDS_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

/// The effect identifiers the page's own tables name, and one full set of settings for each.
///
/// Read off `EFFECT_PARAMS` and `EFFECT_NAMES` in the page by eye rather than by parser: the
/// list began as five effects, when a parser for a JavaScript object literal would have been
/// longer than it, and has been added to by hand with every effect since. The check is that the command reads these names, so an effect renamed on one
/// side and not the other fails here.
const EFFECT_KINDS: &[(&str, &[(&str, &str)])] = &[
    ("core.gaussian_blur", &[("sigma_px", "4")]),
    // D-335: the offset, the gamma and the bypass.
    ("core.exposure", &[("stops", "0.5"), ("offset", "0.1"), ("gamma", "1.5"), ("bypass", "on")]),
    (
        "core.tint",
        &[("color", "0.9, 0.7, 0.5"), ("amount", "0.3")],
    ),
    ("core.line_smooth", &[("softness", "60"), ("threshold", "12")]),
    // The colours are drawn apart from EFFECT_PARAMS and sent as one list (D-87).
    (
        "core.selective_color_blur",
        &[
            ("blur", "12"),
            ("colors", "%23f6d6be,%23dba08e"),
            ("tolerance", "0"),
        ],
    ),
    // D-89: the switches, the colours and the tint are drawn apart and sent with them.
    (
        "core.glow",
        &[
            ("threshold", "60"),
            ("radius", "10"),
            ("intensity", "1"),
            ("tolerance", "0"),
            ("colors", "%233c286e"),
            ("based_on", "bright"),
            ("operation", "add"),
            ("tint", "%23ff4000"),
            // D-747.
            ("composite", "on_top"),
            ("dimensions", "horizontal"),
            ("glow_colors", "a_and_b"),
            ("color_looping", "sawtooth_ab"),
            ("color_loops", "2"),
            ("color_phase", "45"),
            ("midpoint", "40"),
            ("color_a", "%23ff0000"),
            ("color_b", "%230000ff"),
        ],
    ),
    // D-91: the colours and the new colour are drawn apart and sent with the tolerance.
    (
        "core.line_recolor",
        &[
            ("tolerance", "0"),
            ("colors", "%231e1a24"),
            ("new_color", "%236b3a1e"),
        ],
    ),
    (
        "core.directional_blur",
        &[("direction", "45"), ("length", "10")],
    ),
    // D-93: the colours and the keep word are sent with the tolerance.
    (
        "core.select_color",
        &[
            ("tolerance", "0"),
            ("colors", "%231e1a24"),
            ("keep", "others"),
        ],
    ),
    // D-94: the width, the based-on word, the colours and the tolerance.
    (
        "core.line_width",
        &[
            ("width", "-2"),
            ("tolerance", "0"),
            ("colors", "%231e1a24"),
            ("based_on", "colors"),
        ],
    ),
    // D-95: the amount, the centre as two numbers, and the type.
    (
        "core.radial_blur",
        &[("amount", "20"), ("center", "25,75"), ("type", "zoom")],
    ),
    // D-96: the five numbers and the streaks word.
    (
        "core.bloom",
        &[
            ("threshold", "70"),
            ("radius", "12"),
            ("intensity", "1.5"),
            ("length", "30"),
            ("angle", "15"),
            ("streaks", "star"),
        ],
    ),
    // D-97: the colours and the match word are sent with the two numbers.
    (
        "core.color_key",
        &[
            ("tolerance", "30"),
            ("softness", "10"),
            ("colors", "%2300b140"),
            ("match", "hue"),
        ],
    ),
    // D-111: each curve's points, in then out, a comma between points.
    (
        "core.curves",
        &[
            ("master", "0%200,128%20180,255%20255"),
            ("red", "0%200,255%20255"),
            ("green", "0%200,255%20128"),
            ("blue", "0%2064,255%20255"),
            ("alpha", "0%200,128%2064,255%20255"),
        ],
    ),
    // D-112: the five numbers.
    (
        "core.levels",
        &[
            ("input_black", "32"),
            ("input_white", "224"),
            ("gamma", "1.5"),
            ("output_black", "16"),
            ("output_white", "240"),
        ],
    ),
    // D-383: the Channel menu and a set for each channel, under both names.
    ("core.levels", &[("channel", "red"), ("input_black", "8"), ("input_white", "250"), ("gamma", "1"), ("output_black", "0"), ("output_white", "255"), ("red_input_white", "192"), ("alpha_output_white", "200")]),
    ("core.levels_individual", &[("input_black", "8"), ("input_white", "250"), ("gamma", "1"), ("output_black", "0"), ("output_white", "255"), ("green_gamma", "1.5"), ("blue_output_black", "40")]),
    // D-113: the three numbers.
    (
        "core.hue_saturation",
        &[("hue", "60"), ("saturation", "-50"), ("lightness", "20"), ("reds_hsl", "14, 0, 0"), ("blues_hsl", "0, -30, 10")],
    ),
    // D-776: Colorize and its three numbers.
    ("core.hue_saturation", &[("hue", "0"), ("saturation", "0"), ("lightness", "0"), ("colorize", "on"), ("colorize_hue", "180"), ("colorize_saturation", "70"), ("colorize_lightness", "-4")]),
    // D-114: the two points as two numbers each, the colours, the opacities and the words.
    (
        "core.gradient",
        &[
            ("start", "0,50"),
            ("end", "100,50"),
            ("start_opacity", "100"),
            ("end_opacity", "20"),
            ("start_color", "%23ff8000"),
            ("end_color", "%236450a0"),
            ("shape", "radial"),
            ("blend", "screen"),
        ],
    ),
    // D-115: the colour and the four numbers.
    (
        "core.drop_shadow",
        &[
            ("color", "%232040a0"),
            ("opacity", "75"),
            ("direction", "45"),
            ("distance", "8"),
            ("softness", "4"),
            // D-760.
            ("shadow_only", "on"),
        ],
    ),
    // D-116: the radius and the edges; D-121: the iris, its four numbers and the highlights.
    (
        "core.lens_blur",
        &[
            ("radius", "6"),
            ("edges", "repeat"),
            ("iris", "hexagon"),
            ("roundness", "20"),
            ("rotation", "15"),
            ("aspect", "1.5"),
            ("highlight_gain", "3"),
            ("highlight_threshold", "90"),
            // D-359: the blur map's five.
            ("blur_layer", "layer-4"),
            ("fit", "stretch"),
            ("channel", "alpha"),
            ("focal_distance", "128"),
            ("invert", "on"),
        ],
    ),
    // D-117: the colour, the four numbers and the blend.
    (
        "core.rim_light",
        &[
            ("color", "%23ffb040"),
            ("direction", "300"),
            ("width", "5"),
            ("softness", "2"),
            ("intensity", "80"),
            ("blend", "screen"),
        ],
    ),
    // D-118: the colour and the three numbers.
    (
        "core.outline",
        &[
            ("color", "%23c82828"),
            ("width", "4"),
            ("softness", "2"),
            ("opacity", "80"),
        ],
    ),
    // D-119: the two numbers and the two words.
    (
        "core.noise",
        &[
            ("amount", "25"),
            ("mode", "color"),
            ("seed", "42"),
            ("animate", "off"),
        ],
    ),
    // D-120: the amount and the centre.
    (
        "core.chromatic_aberration",
        &[("amount", "7"), ("center", "30,70")],
    ),
    // D-409: the new form's mode, numbers and scales.
    (
        "core.chromatic_aberration",
        &[
            ("mode", "offset"),
            ("amount", "7"),
            ("center", "30,70"),
            ("angle", "45"),
            ("falloff", "50"),
            ("red_scale", "150"),
            ("green_scale", "10"),
            ("blue_scale", "-80"),
            ("fringe_blur", "40"),
        ],
    ),
    // D-123: the two numbers, the colour and the two words.
    (
        "core.distance_gradation",
        &[
            ("width", "7"),
            ("opacity", "80"),
            ("color", "%23ff8000"),
            ("invert", "on"),
            ("blend", "screen"),
        ],
    ),
    // D-124: the centre, the three numbers and the colour.
    (
        "core.light_rays",
        &[
            ("center", "30,70"),
            ("length", "80"),
            ("threshold", "60"),
            ("intensity", "2.5"),
            ("color", "%23ffc070"),
        ],
    ),
    // D-423: CC Light Rays' numbers, centre, words and colour.
    (
        "core.light_rays",
        &[
            ("intensity", "250"),
            ("center", "30,70"),
            ("radius", "80"),
            ("warp_softness", "120"),
            ("shape", "square"),
            ("direction", "30"),
            ("color_from_source", "off"),
            ("allow_brightening", "off"),
            ("color", "%23ffc070"),
            ("transfer_mode", "screen"),
        ],
    ),
    // D-125: the three numbers.
    (
        "core.exposure_flicker",
        &[("amount", "1.5"), ("hold", "3"), ("seed", "42")],
    ),
    // D-126: the numbers, the colour and the centre.
    (
        "core.vignette",
        &[
            ("amount", "70"),
            ("color", "%236450a0"),
            ("size", "80"),
            ("roundness", "40"),
            ("softness", "30"),
            ("center", "40,60"),
        ],
    ),
    // D-127: the six numbers and the edges.
    (
        "core.turbulent_displace",
        &[
            ("amount", "25"),
            ("size", "40"),
            ("complexity", "4"),
            ("evolution", "90"),
            ("speed", "-30"),
            ("seed", "12"),
            ("edges", "repeat"),
            ("displacement", "horizontal"),
            ("pinning", "all"),
            ("new_seed_every", "3"),
            ("drift_direction", "0"),
            ("drift_speed", "2"),
            ("offset", "3,-2"),
            ("units", "after_effects"),
        ],
    ),
    // D-128: the eight numbers, the two colours and the blend. D-299: and the look.
    (
        "core.fractal_noise",
        &[
            ("fractal_type", "turbulent"),
            ("noise_type", "block"),
            ("invert", "on"),
            ("offset", "15, -8"),
            ("scale_width", "150"),
            ("scale_height", "60"),
            ("cycle", "4"),
            ("size", "40"),
            ("complexity", "6"),
            ("contrast", "170"),
            ("brightness", "-10"),
            ("evolution", "45"),
            ("speed", "12"),
            ("seed", "3"),
            ("dark_color", "%231e1a24"),
            ("light_color", "%23fff0b0"),
            ("opacity", "60"),
            ("blend", "screen"),
            // D-743.
            ("rotation", "30"),
            ("sub_influence", "80"),
            ("sub_scaling", "65"),
            ("sub_rotation", "20"),
            ("sub_offset", "3, -2"),
            ("overflow", "soft_clamp"),
            ("perspective_offset", "on"),
            ("center_subscale", "on"),
        ],
    ),
    // D-129: the three colours and the two numbers.
    (
        "core.gradient_map",
        &[
            ("shadow_color", "%232a1650"),
            ("midtone_color", "%23c85a50"),
            ("highlight_color", "%23ffe6b4"),
            ("midpoint", "40"),
            ("amount", "80"),
        ],
    ),
    // D-130: the three tones. D-295: and Preserve Luminosity.
    (
        "core.color_balance",
        &[
            ("shadows", "0, 0, 40"),
            ("midtones", "-10, 5, 0"),
            ("highlights", "30, 10, -20"),
            ("preserve_luminosity", "on"),
        ],
    ),
    // D-131: the shift.
    ("core.offset", &[("shift", "40, -12")]),
    // D-132: the two numbers and the blend.
    (
        "core.light_wrap",
        &[("width", "24"), ("intensity", "150"), ("blend", "add")],
    ),
    // D-134: the channel and the amount.
    ("core.invert", &[("channel", "alpha"), ("amount", "60")]),
    // D-135: the brightness and the contrast.
    ("core.brightness_contrast", &[("brightness", "30"), ("contrast", "40")]),
    // D-136: the six ranges.
    ("core.black_white", &[("reds", "120"), ("yellows", "110"), ("greens", "-10"), ("cyans", "-50"), ("blues", "-50"), ("magentas", "120")]),
    // D-137: the levels.
    ("core.posterize", &[("levels", "16")]),
    // D-138: the level.
    ("core.threshold", &[("level", "172")]),
    // D-139: the three rows and monochrome.
    (
        "core.channel_mixer",
        &[
            ("red", "30, 59, 11, 0"),
            ("green", "0, 100, 0, 20"),
            ("blue", "100, 0, 0, 0"),
            ("monochrome", "on"),
        ],
    ),
    // D-140: the vibrance and the saturation.
    ("core.vibrance", &[("vibrance", "40"), ("saturation", "20")]),
    // D-141: the colour and the three numbers.
    (
        "core.leave_color",
        &[
            ("color", "%234060ff"),
            ("tolerance", "20"),
            ("softness", "30"),
            ("amount", "80"),
        ],
    ),
    // D-142: the threshold.
    ("core.solarize", &[("threshold", "200")]),
    // D-143: the three numbers and the two colours.
    (
        "core.halftone",
        &[
            ("size", "6"),
            ("angle", "30"),
            ("ink", "%231e1a24"),
            ("paper", "%23f6d6be"),
            ("amount", "90"),
        ],
    ),
    // D-144: the size.
    // D-754: the counts and Sharp Colors too.
    (
        "core.mosaic",
        &[("size", "24"), ("mode", "count"), ("horizontal", "7"), ("vertical", "3"), ("sharp_colors", "on")],
    ),
    // D-145: the three numbers and the mode.
    (
        "core.emboss",
        &[
            ("direction", "90"),
            ("relief", "3"),
            ("contrast", "250"),
            ("mode", "color"),
        ],
    ),
    // D-146: the choice and the amount.
    ("core.find_edges", &[("invert", "on"), ("amount", "40")]),
    // D-147: both numbers.
    ("core.sharpen", &[("amount", "250"), ("radius", "2"), ("threshold", "12")]),
    // D-148: both numbers and the blend.
    ("core.diffusion", &[("radius", "25"), ("amount", "70"), ("blend", "lighten"), ("second_blend", "overlay"), ("second_amount", "40")]),
    // D-149: the five numbers and both words.
    (
        "core.wave_warp",
        &[
            ("shape", "triangle"),
            ("height", "6"),
            ("width", "30"),
            ("direction", "45"),
            ("speed", "12"),
            ("phase", "90"),
            ("edges", "repeat"),
            // D-745.
            ("pinning", "left_edge"),
        ],
    ),
    // D-150: the centre and the five numbers.
    (
        "core.ripple",
        &[
            ("center", "30,70"),
            ("amplitude", "8"),
            ("wavelength", "45"),
            ("speed", "-15"),
            ("phase", "90"),
            ("fade", "200"),
        ],
    ),
    // D-151: the two numbers and the centre.
    ("core.twirl", &[("angle", "-270"), ("radius", "120"), ("center", "40,60"), ("twirl_radius", "45")]),
    // D-152: the centre and the two numbers.
    ("core.bulge", &[("center", "40,60"), ("radius", "120"), ("height", "-2"), ("vertical_radius", "80"), ("taper_radius", "30")]),
    // D-153: the centre and the angle.
    ("core.mirror", &[("center", "40,60"), ("angle", "90")]),
    // D-154: the two sizes and the choice.
    ("core.motion_tile", &[("output_width", "300"), ("output_height", "200"), ("mirror", "on"), ("tile_center", "40, 60"), ("tile_width", "28"), ("tile_height", "50")]),
    // D-155: the three numbers.
    ("core.linear_wipe", &[("completion", "40"), ("angle", "135"), ("feather", "12")]),
    // D-156: the numbers, the centre and the choice.
    ("core.radial_wipe", &[("completion", "40"), ("start_angle", "90"), ("center", "40,60"), ("wipe", "both"), ("feather", "30")]),
    // D-157: the four numbers.
    ("core.venetian_blinds", &[("completion", "40"), ("angle", "90"), ("width", "12"), ("feather", "3")]),
    // D-158: the numbers, the centre and the choice.
    ("core.iris_wipe", &[("completion", "40"), ("center", "40,60"), ("feather", "12"), ("invert", "on")]),
    // D-159: the one number.
    ("core.simple_choker", &[("choke", "-3")]),
    // D-160: the centre, the colour and the numbers.
    (
        "core.speed_lines",
        &[
            ("center", "40,60"),
            ("color", "%233a6fd8"),
            ("count", "60"),
            ("thickness", "3"),
            ("inner", "20"),
            ("inner_jitter", "10"),
            ("angle_jitter", "30"),
            ("seed", "7"),
            ("hold", "3"),
            ("opacity", "80"),
        ],
    ),
    // D-161: the numbers and the colour.
    (
        "core.cross_glare",
        &[
            ("threshold", "70"),
            ("length", "25"),
            ("points", "6"),
            ("angle", "10"),
            ("intensity", "2"),
            ("color", "%23ffd080"),
        ],
    ),
    // D-162: the numbers.
    (
        "core.camera_shake",
        &[("amount", "6"), ("rotation", "3"), ("hold", "2"), ("seed", "7")],
    ),
    // D-163: the colour and the numbers.
    (
        "core.rain",
        &[
            ("color", "%23a0b0e0"),
            ("density", "45"),
            ("spacing", "16"),
            ("length", "30"),
            ("width", "2"),
            ("direction", "190"),
            ("speed", "40"),
            ("seed", "3"),
            ("opacity", "70"),
        ],
    ),
    // D-182: the lookup file, none.
    ("core.color_lookup", &[("lut", "")]),
    // D-183.
    (
        "core.line_blur",
        &[("length", "6"), ("strength", "80"), ("lines_only", "on")],
    ),
    // D-184.
    (
        "core.hsv_key",
        &[
            ("hue", "142"),
            ("saturation", "90"),
            ("value", "50"),
            ("hue_range", "10"),
            ("saturation_range", "20"),
            ("value_range", "45"),
            ("invert", "on"),
        ],
    ),
    // D-185: the three numbers, the colour and the blend.
    (
        "core.paraffin",
        &[
            ("direction", "45"),
            ("spread", "40"),
            ("opacity", "90"),
            ("color", "%23ff8000"),
            ("blend", "soft_light"),
        ],
    ),
    // D-186: the nine numbers, the shape and the colour.
    (
        "core.kira_kira",
        &[
            ("threshold", "90"),
            ("spacing", "32"),
            ("density", "80"),
            ("size", "20"),
            ("shape", "cross"),
            ("angle", "30"),
            ("twinkle", "50"),
            ("period", "12"),
            ("seed", "5"),
            ("opacity", "80"),
            ("color", "%23ffe0a0"),
        ],
    ),
    // D-190: the two points, the eight numbers and the two colours.
    (
        "core.lightning_bolt",
        &[
            ("start", "20,10"),
            ("end", "70,90"),
            ("jagged", "60"),
            ("detail", "5"),
            ("branches", "50"),
            ("width", "4"),
            ("glow", "30"),
            ("opacity", "90"),
            ("hold", "3"),
            ("seed", "8"),
            ("color", "%23fff0c0"),
            ("glow_color", "%23ff4a1a"),
            ("composite", "off"),
            // D-324: Advanced Lightning's five.
            ("kind", "strike"),
            ("turbulence", "40"),
            ("decay", "30"),
            ("conductivity", "2.5"),
            ("obstacle", "50"),
            // D-329: going round.
            ("path", "around"),
            // D-334: the soft core.
            ("core", "soft"),
            // D-338: the long forks.
            ("forks", "long"),
        ],
    ),
    (
        "core.compound_blur",
        &[
            ("blur_layer", "layer-4"),
            ("fit", "tile"),
            ("max_blur", "12"),
            ("invert", "on"),
            ("edges", "repeat"),
        ],
    ),
    (
        "core.displacement_map",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("horizontal", "luminance"),
            ("max_horizontal", "-12"),
            ("vertical", "alpha"),
            ("max_vertical", "30"),
            ("wrap", "on"),
            ("red_amount", "50"),
            ("green_amount", "100"),
            ("blue_amount", "150"),
            ("spectrum", "9"),
        ],
    ),
    (
        "core.gradient_wipe",
        &[
            ("map_layer", "layer-4"),
            ("fit", "center"),
            ("completion", "40"),
            ("softness", "25"),
            ("invert", "on"),
        ],
    ),
    (
        "core.echo",
        &[
            ("echo_time", "-3"),
            ("echoes", "4"),
            ("intensity", "0.8"),
            ("decay", "0.6"),
            ("operator", "composite_in_back"),
        ],
    ),
    ("core.posterize_time", &[("frame_rate", "8")]),
    // D-766: After Effects' Expression Controls, one setting each.
    ("core.slider_control", &[("slider", "42")]),
    ("core.checkbox_control", &[("checkbox", "1")]),
    ("core.angle_control", &[("angle", "90")]),
    ("core.point_control", &[("point", "100, 200")]),
    ("core.color_control", &[("color", "0.25, 0.5, 0.75")]),
    ("core.layer_control", &[("control_layer", "layer-4")]),
    // D-520: the two step counts and the word.
    (
        "core.cc_wide_time",
        &[("forward_steps", "5"), ("backward_steps", "2"), ("native_motion_blur", "on")],
    ),
    // D-581: the two numbers and the two words.
    (
        "core.force_motion_blur",
        &[("motion_blur_samples", "4"), ("override_native_shutter", "on"), ("shutter_angle", "360"), ("native_motion_blur", "off")],
    ),
    // D-650: the word and the three numbers.
    (
        "core.pixel_motion_blur",
        &[("shutter_control", "manual"), ("shutter_angle", "360"), ("shutter_samples", "8"), ("vector_detail", "50")],
    ),
    // D-652: the seven words and the thirteen numbers.
    (
        "core.timewarp",
        &[
            ("method", "frame_mix"),
            ("adjust_time_by", "source_frame"),
            ("speed", "200"),
            ("source_frame", "12"),
            ("vector_detail", "50"),
            ("global_smoothness", "0.25"),
            ("local_smoothness", "0.75"),
            ("smoothing_iterations", "8"),
            ("build_from_one_image", "on"),
            ("correct_luminance_changes", "on"),
            ("filtering", "extreme"),
            ("error_threshold", "2"),
            ("block_size", "8"),
            ("red_weight", "0.5"),
            ("green_weight", "0.25"),
            ("blue_weight", "0.25"),
            ("enable_motion_blur", "on"),
            ("shutter_control", "manual"),
            ("shutter_angle", "360"),
            ("shutter_samples", "8"),
        ],
    ),
    // D-653: the five words and the four numbers.
    (
        "core.warp_stabilizer",
        &[
            ("result", "no_motion"),
            ("smoothness", "100"),
            ("method", "perspective"),
            ("preserve_scale", "on"),
            ("framing", "stabilize_crop"),
            ("maximum_scale", "200"),
            ("action_safe_margin", "10"),
            ("additional_scale", "120"),
            ("detailed_analysis", "on"),
        ],
    ),
    // D-654: the three words and the two numbers.
    (
        "core.rolling_shutter_repair",
        &[
            ("rolling_shutter_rate", "80"),
            ("scan_direction", "left_to_right"),
            ("method", "pixel_motion"),
            ("detailed_analysis", "on"),
            ("pixel_motion_detail", "60"),
        ],
    ),
    // D-582: the ten numbers, the colour and the two words.
    (
        "core.cc_rainfall",
        &[
            ("drops", "40"),
            ("size", "2"),
            ("scene_depth", "96"),
            ("speed", "240"),
            ("wind", "-60"),
            ("wind_variation", "80"),
            ("spread", "30"),
            ("color", "%23c8d8ff"),
            ("opacity", "70"),
            ("transfer_mode", "composite"),
            ("composite_with_original", "off"),
            ("ground_level", "90"),
            ("random_seed", "3"),
        ],
    ),
    // D-630: a ground layer, the numbers, the two positions and the five words.
    (
        "core.wave_world",
        &[
            ("brightness", "0.4"),
            ("contrast", "2"),
            ("gamma", "1.5"),
            ("render_dry_areas", "transparent"),
            ("transparency", "0.3"),
            ("grid_resolution", "40"),
            ("grid_res_downsamples", "on"),
            ("wave_speed", "0.5"),
            ("damping", "0.2"),
            ("reflect_edges", "all"),
            ("pre_roll", "0.5"),
            ("map_layer", "layer-4"),
            ("steepness", "0.5"),
            ("height", "0.3"),
            ("wave_strength", "1"),
            ("producer_1_type", "line"),
            ("producer_1_position", "40,60"),
            ("producer_1_height", "0.2"),
            ("producer_1_width", "0.05"),
            ("producer_1_angle", "30"),
            ("producer_1_amplitude", "0.8"),
            ("producer_1_frequency", "2"),
            ("producer_1_phase", "90"),
            ("producer_2_type", "ring"),
            ("producer_2_position", "70,30"),
            ("producer_2_height", "0.1"),
            ("producer_2_width", "0.1"),
            ("producer_2_angle", "0"),
            ("producer_2_amplitude", "0.3"),
            ("producer_2_frequency", "3"),
            ("producer_2_phase", "0"),
        ],
    ),
    // D-633: five layers, the patterns and modes, two forces, the physics, the camera and light.
    (
        "core.shatter",
        &[
            ("view", "rendered"),
            ("render", "pieces"),
            ("pattern", "hexagons"),
            ("custom_shatter_map", "layer-4"),
            ("white_tiles_fixed", "on"),
            ("repetitions", "12"),
            ("direction", "30"),
            ("origin", "40,60"),
            ("extrusion_depth", "0.3"),
            ("force_1_position", "30,40"),
            ("force_1_depth", "0.2"),
            ("force_1_radius", "0.5"),
            ("force_1_strength", "-2"),
            ("force_2_position", "70,60"),
            ("force_2_depth", "-0.1"),
            ("force_2_radius", "0.25"),
            ("force_2_strength", "8"),
            ("gradient_layer", "layer-4"),
            ("shatter_threshold", "40"),
            ("invert_gradient", "on"),
            ("rotation_speed", "0.5"),
            ("tumble_axis", "xz"),
            ("randomness", "0.3"),
            ("viscosity", "0.5"),
            ("mass_variance", "60"),
            ("gravity", "6"),
            ("gravity_direction", "90"),
            ("gravity_inclination", "20"),
            ("color", "#ffeecc"),
            ("opacity", "0.75"),
            ("front_mode", "color"),
            ("side_mode", "layer_plus_opacity"),
            ("back_mode", "tinted_layer_plus_opacity"),
            ("front_layer", "layer-4"),
            ("side_layer", "layer-4"),
            ("back_layer", "layer-4"),
            ("camera_system", "comp_camera"),
            ("camera_x_rotation", "15"),
            ("camera_y_rotation", "-30"),
            ("camera_z_rotation", "5"),
            ("camera_xy_position", "40,60"),
            ("camera_z_position", "2"),
            ("focal_length", "35"),
            ("light_type", "point"),
            ("light_intensity", "1.5"),
            ("light_color", "#ffeecc"),
            ("light_position", "20,30"),
            ("light_depth", "0.8"),
            ("ambient_light", "0.2"),
            ("diffuse_reflection", "1"),
            ("specular_reflection", "0.5"),
            ("highlight_sharpness", "20"),
        ],
    ),
    // D-632: three layers, the eight sources, multipliers and offsets, the camera and light.
    (
        "core.card_dance",
        &[
            ("rows_columns", "columns_follow_rows"),
            ("rows", "12"),
            ("columns", "6"),
            ("back_layer", "layer-4"),
            ("gradient_layer_1", "layer-4"),
            ("gradient_layer_2", "layer-4"),
            ("rotation_order", "zyx"),
            ("transformation_order", "position_rotate_scale"),
            ("x_position_source", "red_1"),
            ("y_position_source", "green_2"),
            ("z_position_source", "intensity_2"),
            ("x_rotation_source", "blue_1"),
            ("y_rotation_source", "alpha_1"),
            ("z_rotation_source", "alpha_2"),
            ("x_scale_source", "intensity_1"),
            ("y_scale_source", "none"),
            ("x_position_multiplier", "2"),
            ("x_position_offset", "0.5"),
            ("y_position_multiplier", "-1"),
            ("y_position_offset", "0.25"),
            ("z_position_multiplier", "3"),
            ("z_position_offset", "-1"),
            ("x_rotation_multiplier", "45"),
            ("x_rotation_offset", "10"),
            ("y_rotation_multiplier", "180"),
            ("y_rotation_offset", "-20"),
            ("z_rotation_multiplier", "360"),
            ("z_rotation_offset", "30"),
            ("x_scale_multiplier", "0.25"),
            ("x_scale_offset", "1.5"),
            ("y_scale_multiplier", "0.75"),
            ("y_scale_offset", "0.5"),
            ("camera_system", "comp_camera"),
            ("camera_x_rotation", "15"),
            ("camera_y_rotation", "-30"),
            ("camera_z_rotation", "5"),
            ("camera_xy_position", "40,60"),
            ("camera_z_position", "2"),
            ("focal_length", "35"),
            ("light_type", "point"),
            ("light_intensity", "1.5"),
            ("light_color", "#ffeecc"),
            ("light_position", "20,30"),
            ("light_depth", "0.8"),
            ("ambient_light", "0.2"),
            ("diffuse_reflection", "1"),
            ("specular_reflection", "0.5"),
            ("highlight_sharpness", "20"),
        ],
    ),
    // D-631: three layers, the numbers, the light's position and the seven words.
    (
        "core.caustics",
        &[
            ("bottom_layer", "layer-4"),
            ("bottom_scaling", "0.5"),
            ("bottom_repeat_mode", "reflected"),
            ("bottom_fit", "center"),
            ("bottom_blur", "10"),
            ("water_surface", "layer-4"),
            ("wave_height", "0.4"),
            ("smoothing", "8"),
            ("water_depth", "0.2"),
            ("refractive_index", "1.33"),
            ("surface_color", "#336699"),
            ("surface_opacity", "0.5"),
            ("caustics_strength", "0.7"),
            ("sky_layer", "layer-4"),
            ("sky_scaling", "2"),
            ("sky_repeat_mode", "once"),
            ("sky_fit", "center"),
            ("sky_intensity", "0.6"),
            ("sky_convergence", "0.4"),
            ("light_type", "point"),
            ("light_intensity", "1.5"),
            ("light_color", "#ffeecc"),
            ("light_position", "20,30"),
            ("light_height", "0.8"),
            ("ambient_light", "0.2"),
            ("diffuse_reflection", "1"),
            ("specular_reflection", "0.5"),
            ("highlight_sharpness", "20"),
        ],
    ),
    // D-521: a target, the two numbers and the two words.
    (
        "core.time_difference",
        &[
            ("map_layer", "layer-4"),
            ("time_offset", "-0.5"),
            ("contrast", "20"),
            ("absolute_difference", "off"),
            ("alpha_channel", "max_of_result"),
        ],
    ),
    // D-197: the colours, the words and the four numbers.
    (
        "core.change_to_color",
        &[
            ("from", "%23c82828"),
            ("to", "%234060ff"),
            ("change", "hue_saturation"),
            ("change_by", "transforming"),
            ("hue_tolerance", "12"),
            ("lightness_tolerance", "40"),
            ("saturation_tolerance", "60"),
            ("softness", "30"),
            ("view_matte", "on"),
        ],
    ),
    (
        "core.corner_pin",
        &[
            ("upper_left", "10,5"),
            ("upper_right", "90,-5"),
            ("lower_left", "-20,110"),
            ("lower_right", "120,95"),
        ],
    ),
    // D-199: the centre, the numbers, the words and the colour.
    (
        "core.light_sweep",
        &[
            ("center", "30,60"),
            ("direction", "20"),
            ("shape", "sharp"),
            ("width", "24"),
            ("sweep_intensity", "70"),
            ("edge_intensity", "40"),
            ("edge_thickness", "3"),
            ("light_color", "%23ffb040"),
            ("light_reception", "composite"),
        ],
    ),
    // D-200: the producer, the numbers, the profile and the colour.
    (
        "core.radio_waves",
        &[
            ("producer_point", "30,60"),
            ("sides", "6"),
            ("interval", "12"),
            ("expansion", "3"),
            ("orientation", "15"),
            ("direction", "45"),
            ("velocity", "2"),
            ("spin", "4"),
            ("lifespan", "40"),
            ("opacity", "80"),
            ("fade_in_time", "2"),
            ("fade_out_time", "20"),
            ("start_width", "8"),
            ("end_width", "2"),
            ("profile", "sine"),
            ("color", "%23ffb040"),
        ],
    ),
    // D-201: the number and the word.
    ("core.polar_coordinates", &[("interpolation", "60"), ("conversion", "polar_to_rect")]),
    // D-203.
    ("core.median", &[("radius", "4"), ("operate_on_alpha", "on")]),
    // D-453.
    ("core.dust_scratches", &[("radius", "3"), ("threshold", "24"), ("operate_on_alpha", "on")]),
    // D-455.
    (
        "core.remove_grain",
        &[
            ("noise_reduction", "2"),
            ("passes", "3"),
            ("mode", "single_channel"),
            ("unsharp_amount", "120"),
            ("unsharp_radius", "2"),
            ("unsharp_threshold", "10"),
        ],
    ),
    // D-457.
    (
        "core.advanced_spill_suppressor",
        &[
            ("method", "ultra"),
            ("suppression", "80"),
            ("key_color", "#1040c0"),
            ("tolerance", "40"),
            ("desaturate", "30"),
            ("spill_range", "60"),
            ("spill_color_correction", "20"),
            ("luma_correction", "70"),
        ],
    ),
    // D-459.
    (
        "core.color_difference_key",
        &[
            ("view", "partial_b"),
            ("key_color", "#28b43c"),
            ("partial_a_in_black", "20"),
            ("partial_a_in_white", "230"),
            ("partial_a_gamma", "1.2"),
            ("partial_a_out_black", "5"),
            ("partial_a_out_white", "250"),
            ("partial_b_in_black", "30"),
            ("partial_b_in_white", "180"),
            ("partial_b_gamma", "0.8"),
            ("partial_b_out_black", "0"),
            ("partial_b_out_white", "240"),
            ("matte_in_black", "40"),
            ("matte_in_white", "210"),
            ("matte_gamma", "1.5"),
        ],
    ),
    // D-500.
    (
        "core.color_range",
        &[
            ("color_space", "yuv"),
            ("fuzziness", "25"),
            ("min_1", "20"),
            ("max_1", "120"),
            ("min_2", "170"),
            ("max_2", "230"),
            ("min_3", "80"),
            ("max_3", "125"),
        ],
    ),
    // D-501: the words, a plate and the numbers.
    (
        "core.difference_matte",
        &[
            ("view", "matte"),
            ("map_layer", "layer-4"),
            ("fit", "stretch"),
            ("matching_tolerance", "8"),
            ("matching_softness", "12"),
            ("blur_before_difference", "1.5"),
        ],
    ),
    // D-502: the masks, the numbers and the word.
    (
        "core.inner_outer_key",
        &[
            ("foreground", "1"),
            ("background", "2"),
            ("single_mask_highlight_radius", "4"),
            ("edge_thin", "-1.5"),
            ("edge_feather", "3"),
            ("edge_threshold", "20"),
            ("invert_extraction", "on"),
            ("blend_with_original", "25"),
        ],
    ),
    // D-503.
    (
        "core.key_cleaner",
        &[("additional_edge_radius", "4"), ("alpha_contrast", "30"), ("strength", "75")],
    ),
    // D-504.
    (
        "core.linear_color_key",
        &[
            ("view", "matte"),
            ("key_color", "#28b43c"),
            ("match_colors", "hue"),
            ("key_operation", "keep"),
            ("matching_tolerance", "20"),
            ("matching_softness", "15"),
        ],
    ),
    // D-540.
    (
        "core.screen_key",
        &[
            ("view", "screen_matte"),
            ("screen_color", "#1e46c8"),
            ("screen_gain", "120"),
            ("screen_balance", "95"),
            ("clip_black", "10"),
            ("clip_white", "90"),
            ("despill_bias", "30"),
            ("despill", "80"),
            ("shrink_grow", "-2"),
            ("softness", "1.5"),
            ("edge_refine", "3"),
        ],
    ),
    // D-548.
    ("core.spill_suppressor", &[("color_to_suppress", "#28b43c"), ("suppression", "60")]),
    // D-549.
    ("core.reduce_interlace_flicker", &[("softness", "2.5")]),
    // D-505.
    (
        "core.unmult",
        &[
            ("background_color", "white"),
            ("black_level", "5"),
            ("white_level", "85"),
            ("softness", "12"),
            ("remove_color_matting", "off"),
            ("clip_hdr_results", "on"),
        ],
    ),
    // D-508: the points, the numbers and the words.
    (
        "core.energy_stroke",
        &[
            ("source", "masks"),
            ("core_start", "10,20"),
            ("core_end", "90,80"),
            ("evolution", "45"),
            ("start_offset", "10"),
            ("end_offset", "90"),
            ("start_size", "80"),
            ("end_size", "20"),
            ("core_size", "12"),
            ("core_softness", "50"),
            ("glow_color", "#ff8000"),
            ("glow_intensity", "150"),
            ("glow_spread", "25"),
            ("flicker_amount", "30"),
            ("flicker_speed", "40"),
            ("distortion_amount", "5"),
            ("distortion_scale", "30"),
            ("distortion_speed", "10"),
            ("random_seed", "7"),
            ("composite", "transparent"),
            ("start_roundness", "40"),
            ("end_roundness", "60"),
            ("noise_along_path", "off"),
            ("alpha_mode", "core"),
            ("invert_masks", "on"),
        ],
    ),
    // D-580: the two layers, the numbers and the words.
    (
        "core.card_wipe",
        &[
            ("transition_completion", "40"),
            ("transition_width", "30"),
            ("back_layer", "layer-4"),
            ("rows_columns", "columns_follow_rows"),
            ("rows", "6"),
            ("columns", "5"),
            ("card_scale", "0.8"),
            ("flip_axis", "random"),
            ("flip_direction", "negative"),
            ("flip_order", "gradient"),
            ("gradient_layer", "layer-4"),
            ("timing_randomness", "0.5"),
            ("random_seed", "12"),
            ("x_rotation", "-20"),
            ("y_rotation", "30"),
            ("z_rotation", "10"),
            ("xy_position", "40,60"),
            ("z_position", "1.5"),
            ("focal_length", "35"),
            ("light_type", "point"),
            ("light_intensity", "1.5"),
            ("light_color", "#ffcc88"),
            ("light_position", "20,30"),
            ("light_depth", "0.5"),
            ("ambient_light", "0.4"),
            ("diffuse_reflection", "0.6"),
            ("specular_reflection", "1"),
            ("highlight_sharpness", "20"),
        ],
    ),
    // D-532: the point, the numbers and the words.
    (
        "core.cc_sphere",
        &[
            ("radius", "80"),
            ("offset", "40,60"),
            ("rotation_x", "15"),
            ("rotation_y", "30"),
            ("rotation_z", "-10"),
            ("render", "outside"),
            ("light_intensity", "150"),
            ("light_color", "#ff8000"),
            ("light_height", "-20"),
            ("light_direction", "120"),
            ("ambient", "30"),
            ("diffuse", "70"),
            ("specular", "80"),
            ("roughness", "40"),
            ("metal", "25"),
        ],
    ),
    // D-531: an environment layer, the numbers and the words.
    (
        "core.cc_environment",
        &[
            ("map_layer", "layer-4"),
            ("mapping", "probe"),
            ("horizontal_pan", "30"),
            ("tilt", "-15"),
            ("roll", "10"),
            ("field_of_view", "60"),
            ("filter_environment", "off"),
        ],
    ),
    // D-530: the point, the numbers and the words.
    (
        "core.cc_cylinder",
        &[
            ("radius", "80"),
            ("position", "40,60"),
            ("position_z", "20"),
            ("rotation_x", "15"),
            ("rotation_y", "30"),
            ("rotation_z", "-10"),
            ("render", "outside"),
            ("light_intensity", "150"),
            ("light_color", "#ff8000"),
            ("light_height", "-20"),
            ("light_direction", "120"),
            ("ambient", "30"),
            ("diffuse", "70"),
            ("specular", "80"),
            ("roughness", "40"),
            ("metal", "25"),
        ],
    ),
    // D-507: the points, the numbers and the words; D-519: a Gel render and its gel.
    (
        "core.cc_spotlight",
        &[
            ("from", "10,20"),
            ("to", "60,40"),
            ("height", "30"),
            ("cone_angle", "50"),
            ("edge_softness", "10"),
            ("color", "#ff8000"),
            ("intensity", "150"),
            ("render", "gel_add"),
            ("gel_layer", "layer-4"),
        ],
    ),
    // D-551: the numbers and the two words.
    (
        "core.cc_ball_action",
        &[
            ("scatter", "20"),
            ("rotation_axis", "xy15z"),
            ("rotation", "45"),
            ("twist_property", "fast_top"),
            ("twist_angle", "90"),
            ("grid_spacing", "12"),
            ("ball_size", "150"),
            ("instability_state", "30"),
        ],
    ),
    // D-610: every setting, the words and the colours.
    (
        "core.particle_space",
        &[
            ("birth_rate", "3"),
            ("longevity", "1"),
            ("producer_x", "0"),
            ("producer_y", "0"),
            ("producer_z", "0"),
            ("radius_x", "0.025"),
            ("radius_y", "0.025"),
            ("radius_z", "0.025"),
            ("velocity", "1"),
            ("inherit_velocity", "0"),
            ("gravity", "0.5"),
            ("resistance", "0"),
            ("axis_x", "0"),
            ("axis_y", "-1"),
            ("axis_z", "0"),
            ("extra", "0.5"),
            ("extra_angle", "360"),
            ("gravity_x", "0"),
            ("gravity_y", "1"),
            ("gravity_z", "0"),
            ("floor_position", "0.5"),
            ("bounciness", "50"),
            ("random_bounciness", "0"),
            ("bounce_spread", "0"),
            ("wind_x", "0"),
            ("wind_y", "0"),
            ("wind_z", "0"),
            ("turbulence", "0"),
            ("turbulence_scale", "0.25"),
            ("turbulence_evolution", "0"),
            ("birth_size", "0.15"),
            ("death_size", "0.15"),
            ("size_variation", "0"),
            ("max_opacity", "75"),
            ("volume_shade", "0"),
            ("camera_distance", "2"),
            ("camera_rotation_x", "0"),
            ("camera_rotation_y", "0"),
            ("camera_rotation_z", "0"),
            ("camera_fov", "50"),
            ("depth_cue_distance", "4"),
            ("light_direction", "45"),
            ("light_height", "45"),
            ("random_seed", "0"),
            ("rotation_speed", "90"),
            ("initial_rotation", "180"),
            ("map_layer", "layer-4"),
            ("animation", "vortex"),
            ("floor_action", "bounce"),
            ("particle_type", "shaded_sphere"),
            ("color_map", "custom"),
            ("transfer_mode", "add"),
            ("depth_cue", "fog"),
            ("composite_on_original", "on"),
            ("opacity_map", "constant"),
            ("birth_color", "#ffd933"),
            ("death_color", "#cc1a00"),
            ("custom_color_1", "#ffd933"),
            ("custom_color_2", "#f2a926"),
            ("custom_color_3", "#e67a1a"),
            ("custom_color_4", "#d94a0d"),
            ("custom_color_5", "#cc1a00"),
            ("depth_cue_color", "#000000"),
        ],
    ),
    // D-677: the numbers.
    (
        "core.star_burst",
        &[("scatter", "60"), ("speed", "-2"), ("phase", "0.25"), ("grid_spacing", "8"), ("size", "150"), ("blend_with_original", "20")],
    ),
    // D-676: the numbers and the word.
    (
        "core.scatterize",
        &[("amount", "120"), ("right_twist", "45"), ("left_twist", "-30"), ("transfer_mode", "add")],
    ),
    // D-678: the numbers and the words.
    (
        "core.foam",
        &[
            ("producer_x", "40"),
            ("producer_y", "60"),
            ("producer_x_size", "0.2"),
            ("producer_y_size", "0.1"),
            ("producer_orientation", "15"),
            ("production_rate", "2"),
            ("size", "1.5"),
            ("size_variance", "0.3"),
            ("lifespan", "120"),
            ("growth_speed", "0.5"),
            ("strength", "5"),
            ("initial_speed", "1"),
            ("initial_direction", "45"),
            ("wind_speed", "1"),
            ("wind_direction", "120"),
            ("turbulence", "2"),
            ("wobble", "0.2"),
            ("repulsion", "3"),
            ("pop_velocity", "5"),
            ("viscosity", "0.3"),
            ("stickiness", "0.4"),
            ("zoom", "1.5"),
            ("universe_size", "2"),
            ("reflection_strength", "0.3"),
            ("reflection_convergence", "0.5"),
            ("random_seed", "7"),
            ("view", "rendered"),
            ("zoom_producer_point", "on"),
            ("blend_mode", "solid_new_on_top"),
            ("bubble_texture", "spit"),
            ("bubble_orientation", "velocity"),
            ("quality", "high"),
        ],
    ),
    // D-675: the numbers and the words.
    (
        "core.polygon_shards",
        &[
            ("force", "450"),
            ("gravity", "-20"),
            ("spinning", "-200"),
            ("force_center_x", "30"),
            ("force_center_y", "70"),
            ("direction_randomness", "60"),
            ("speed_randomness", "40"),
            ("grid_spacing", "12"),
            ("start_time", "0.5"),
            ("object", "square"),
            ("depth_sort", "off"),
        ],
    ),
    // D-674: the numbers, the words and the colour.
    (
        "core.mercury",
        &[
            ("birth_rate", "35"),
            ("longevity", "3"),
            ("producer_x", "40"),
            ("producer_y", "30"),
            ("radius_x", "25"),
            ("radius_y", "5"),
            ("direction", "120"),
            ("velocity", "80"),
            ("extra", "20"),
            ("gravity", "-30"),
            ("resistance", "150"),
            ("blob_influence", "220"),
            ("birth_size", "60"),
            ("death_size", "300"),
            ("light_intensity", "150"),
            ("light_height", "40"),
            ("light_position_x", "60"),
            ("light_position_y", "20"),
            ("light_direction", "30"),
            ("ambient", "10"),
            ("diffuse", "40"),
            ("specular", "90"),
            ("roughness", "0.2"),
            ("metal", "40"),
            ("animation", "vortex"),
            ("influence_map", "out"),
            ("light_type", "point"),
            ("light_color", "%23ff8000"),
        ],
    ),
    // D-673: the numbers, the word and the colour.
    (
        "core.hair",
        &[
            ("length", "250"),
            ("thickness", "12"),
            ("weight", "-40"),
            ("density", "30"),
            ("color_inheritance", "20"),
            ("opacity", "80"),
            ("light_intensity", "150"),
            ("light_direction", "60"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("specular", "90"),
            ("roughness", "0.3"),
            ("metal", "40"),
            ("constant_mass", "on"),
            ("hair_color", "%232050ff"),
        ],
    ),
    // D-672: the numbers, the word and the colour.
    (
        "core.drizzle",
        &[
            ("drip_rate", "25"),
            ("longevity", "2"),
            ("rippling", "720"),
            ("displacement", "80"),
            ("ripple_height", "-60"),
            ("spreading", "120"),
            ("light_intensity", "150"),
            ("light_height", "40"),
            ("light_position_x", "60"),
            ("light_position_y", "20"),
            ("light_direction", "30"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("specular", "90"),
            ("roughness", "0.2"),
            ("metal", "40"),
            ("light_type", "point"),
            ("light_color", "%23ff8000"),
        ],
    ),
    // D-671: the numbers and the words.
    (
        "core.bubbles",
        &[
            ("amount", "40"),
            ("speed", "-1.5"),
            ("wobble_amplitude", "12"),
            ("wobble_frequency", "2"),
            ("size", "3"),
            ("random_seed", "7"),
            ("reflection", "world"),
            ("shading", "darken"),
        ],
    ),
    // D-612: every setting, the words and the colours. D-710: the turn, the texture layer
    // and Composite With Original.
    (
        "core.particle_plane",
        &[
            ("birth_rate", "3"),
            ("longevity", "1.5"),
            ("producer_x", "0.1"),
            ("producer_y", "-0.1"),
            ("radius_x", "0.05"),
            ("radius_y", "0.02"),
            ("velocity", "1.2"),
            ("inherit_velocity", "0"),
            ("gravity", "0.4"),
            ("resistance", "10"),
            ("direction", "30"),
            ("extra", "0.5"),
            ("extra_angle", "90"),
            ("birth_size", "0.5"),
            ("death_size", "0.2"),
            ("size_variation", "20"),
            ("max_opacity", "80"),
            ("custom_opacity_1", "0"),
            ("custom_opacity_2", "100"),
            ("custom_opacity_3", "60"),
            ("custom_opacity_4", "30"),
            ("custom_opacity_5", "0"),
            ("random_seed", "7"),
            ("rotation_speed", "90"),
            ("initial_rotation", "180"),
            ("map_layer", "layer-4"),
            ("animation", "vortex"),
            ("particle_type", "star"),
            ("opacity_map", "custom"),
            ("color_map", "custom"),
            ("transfer_mode", "screen"),
            ("composite_on_original", "on"),
            ("birth_color", "#ffd933"),
            ("death_color", "#cc1a00"),
            ("custom_color_1", "#ffd933"),
            ("custom_color_2", "#f2a926"),
            ("custom_color_3", "#e67a1a"),
            ("custom_color_4", "#d94a0d"),
            ("custom_color_5", "#cc1a00"),
        ],
    ),
    // D-613: every setting, the two points, the words, the six layers and the colours.
    (
        "core.particle_playground",
        &[
            ("barrel_radius", "-3"),
            ("particles_per_second", "40"),
            ("cannon_direction", "30"),
            ("direction_random_spread", "45"),
            ("velocity", "60"),
            ("velocity_random_spread", "10"),
            ("cannon_particle_radius", "1.5"),
            ("grid_width", "80"),
            ("grid_height", "60"),
            ("particles_across", "4"),
            ("particles_down", "3"),
            ("grid_particle_radius", "1"),
            ("explode_radius", "3"),
            ("explode_dispersion", "25"),
            ("particle_exploder_radius", "0.5"),
            ("particle_exploder_dispersion", "15"),
            ("gravity_force", "50"),
            ("gravity_random_spread", "5"),
            ("gravity_direction", "170"),
            ("wall_mask", "0"),
            ("particle_exploder_older_younger_than", "0.5"),
            ("particle_exploder_age_feather", "0.2"),
            ("layer_map_older_younger_than", "-1"),
            ("layer_map_age_feather", "0"),
            ("gravity_older_younger_than", "0"),
            ("gravity_age_feather", "0"),
            ("wall_older_younger_than", "0"),
            ("wall_age_feather", "0"),
            ("random_seed", "7"),
            ("cannon_position", "40,60"),
            ("grid_position", "55,45"),
            ("particle_exploder_particles_from", "cannon"),
            ("layer_map_particles_from", "all_but_grid"),
            ("gravity_particles_from", "grid_layer_exploder"),
            ("wall_particles_from", "all"),
            ("auto_orient", "on"),
            ("explode_layer", "layer-4"),
            ("layer_map", "layer-4"),
            ("particle_exploder_selection_map", "layer-4"),
            ("layer_map_selection_map", "layer-4"),
            ("gravity_selection_map", "layer-4"),
            ("wall_selection_map", "layer-4"),
            ("cannon_color", "#00ff80"),
            ("grid_color", "#3366ff"),
            ("persistent_older_younger_than", "0.5"),
            ("persistent_age_feather", "0.1"),
            ("ephemeral_older_younger_than", "-2"),
            ("ephemeral_age_feather", "0"),
            ("persistent_min_red", "0.2"),
            ("persistent_max_red", "0.9"),
            ("persistent_min_green", "-5"),
            ("persistent_max_green", "5"),
            ("persistent_min_blue", "0"),
            ("persistent_max_blue", "1"),
            ("ephemeral_min_red", "1"),
            ("ephemeral_max_red", "2"),
            ("ephemeral_min_green", "0"),
            ("ephemeral_max_green", "30"),
            ("ephemeral_min_blue", "-1"),
            ("ephemeral_max_blue", "1"),
            ("persistent_particles_from", "grid"),
            ("ephemeral_particles_from", "all_but_cannon"),
            ("persistent_map_red_to", "opacity"),
            ("persistent_map_green_to", "x_force"),
            ("persistent_map_blue_to", "none"),
            ("ephemeral_map_red_to", "scale"),
            ("ephemeral_map_green_to", "y_velocity"),
            ("ephemeral_map_blue_to", "angle"),
            ("ephemeral_operator", "multiply"),
            ("persistent_map_layer", "layer-4"),
            ("ephemeral_map_layer", "layer-4"),
            ("persistent_selection_map", "layer-4"),
            ("ephemeral_selection_map", "layer-4"),
        ],
    ),
    // D-615: every setting, the position, the words and the colours. D-711: the second
    // system's.
    (
        "core.particle_emitter",
        &[
            ("particles_per_second", "40"),
            ("emitter_z", "-20"),
            ("emitter_size_x", "30"),
            ("emitter_size_y", "20"),
            ("emitter_size_z", "10"),
            ("emitter_rotation_x", "15"),
            ("emitter_rotation_y", "-30"),
            ("emitter_rotation_z", "45"),
            ("grid_x", "3"),
            ("grid_y", "2"),
            ("grid_z", "1"),
            ("velocity", "60"),
            ("velocity_random", "10"),
            ("velocity_from_motion", "50"),
            ("direction_spread", "30"),
            ("life", "1.5"),
            ("life_random", "25"),
            ("size", "6"),
            ("size_random", "20"),
            ("size_over_life_1", "0"),
            ("size_over_life_2", "150"),
            ("size_over_life_3", "100"),
            ("size_over_life_4", "80"),
            ("size_over_life_5", "20"),
            ("opacity", "90"),
            ("opacity_random", "10"),
            ("opacity_over_life_1", "0"),
            ("opacity_over_life_2", "100"),
            ("opacity_over_life_3", "100"),
            ("opacity_over_life_4", "60"),
            ("opacity_over_life_5", "0"),
            ("color_random", "15"),
            ("sphere_feather", "70"),
            ("gravity", "40"),
            ("air_resistance", "5"),
            ("wind_x", "10"),
            ("wind_y", "-5"),
            ("wind_z", "2"),
            ("turbulence", "20"),
            ("turbulence_scale", "60"),
            ("turbulence_evolution", "0.5"),
            ("floor_position", "90"),
            ("bounce", "40"),
            ("bounce_random", "10"),
            ("random_seed", "7"),
            ("aux_particles_per_second", "12"),
            ("aux_particles_per_bounce", "4"),
            ("aux_life", "0.5"),
            ("aux_velocity", "25"),
            ("aux_inherit_velocity", "50"),
            ("aux_size", "3"),
            ("aux_size_at_death", "20"),
            ("aux_opacity", "80"),
            ("aux_opacity_at_death", "10"),
            ("emitter_position", "40,60"),
            ("emitter_type", "box"),
            ("direction", "directional"),
            ("particle_type", "glow_sphere"),
            ("set_color", "over_life"),
            ("transfer_mode", "add"),
            ("floor_action", "bounce"),
            ("aux_emit", "continuously"),
            ("aux_particle_type", "streak"),
            ("aux_set_color", "own"),
            ("color", "#00ff80"),
            ("gradient_color_1", "#ffffff"),
            ("gradient_color_2", "#ffcc00"),
            ("gradient_color_3", "#ff6600"),
            ("gradient_color_4", "#cc0000"),
            ("gradient_color_5", "#330000"),
            ("aux_color", "#ff0040"),
        ],
    ),
    // D-616: every setting, the centre, the words, the layers and the colour.
    (
        "core.particle_grid",
        &[
            ("form_size_x", "300"),
            ("form_size_y", "200"),
            ("form_size_z", "50"),
            ("particles_in_x", "12"),
            ("particles_in_y", "8"),
            ("particles_in_z", "2"),
            ("form_center_z", "-20"),
            ("form_rotation_x", "15"),
            ("form_rotation_y", "-30"),
            ("form_rotation_z", "45"),
            ("size", "6"),
            ("size_random", "20"),
            ("opacity", "90"),
            ("opacity_random", "10"),
            ("sphere_feather", "70"),
            ("disperse", "5"),
            ("twist", "90"),
            ("displace", "12"),
            ("fractal_scale", "60"),
            ("flow_x", "10"),
            ("flow_y", "-5"),
            ("flow_z", "2"),
            ("flow_evolution", "0.5"),
            ("evolution", "3"),
            ("affect_size", "50"),
            ("affect_opacity", "40"),
            ("map_displace", "-30"),
            ("random_seed", "7"),
            ("form_center", "40,60"),
            ("base_form", "sphere_layered"),
            ("particle_type", "glow_sphere"),
            ("transfer_mode", "add"),
            ("color_map", "layer-4"),
            ("size_map", "layer-4"),
            ("displace_map", "layer-4"),
            ("base_layer", "layer-4"),
            ("color", "#00ff80"),
        ],
    ),
    // D-518: the numbers, the layer and the two words.
    (
        "core.warp_blend",
        &[
            ("completion", "40"),
            ("layer_to_reveal", "layer-4"),
            ("reactor", "local_differences"),
            ("smoothness", "5"),
            ("warp_amount", "-20"),
            ("warp_direction", "twisting"),
            ("blend_span", "30"),
        ],
    ),
    // D-517: the numbers, the back, the shading and the centre.
    (
        "core.twister",
        &[("completion", "40"), ("backside", "layer-4"), ("shading", "off"), ("center", "30,60"), ("axis", "45")],
    ),
    // D-516: the numbers and the centre.
    ("core.stretch_wipe", &[("scale", "250"), ("center", "40,55"), ("direction", "20")]),
    // D-515: the numbers, the centre and the reverse.
    ("core.push_iris", &[("completion", "40"), ("center", "30,60"), ("reverse", "on")]),
    // D-514: the numbers and the flip.
    (
        "core.stripe_sweep",
        &[("completion", "40"), ("direction", "30"), ("thickness", "12"), ("slant", "20"), ("flip", "on")],
    ),
    // D-513: the numbers, the centre and the words.
    (
        "core.glow_wipe",
        &[
            ("completion", "40"),
            ("center", "30,60"),
            ("intensity", "150"),
            ("direction", "15"),
            ("shape", "doors"),
            ("color_from_source", "on"),
            ("color", "#ff8000"),
            ("reverse", "on"),
        ],
    ),
    // D-544: the numbers and the colour.
    (
        "core.halation",
        &[
            ("threshold", "40"),
            ("threshold_smooth", "20"),
            ("radius", "60"),
            ("amount", "150"),
            ("color", "%23ffb040"),
        ],
    ),
    // D-511: the numbers, the centre and the shape.
    // D-512: the layer, the numbers and the words.
    (
        "core.channel_wipe",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("completion", "40"),
            ("softness", "25"),
            ("property", "hue"),
            ("blur", "6"),
            ("invert", "on"),
        ],
    ),
    (
        "core.teeth_wipe",
        &[
            ("completion", "40"),
            ("center", "30,60"),
            ("direction", "15"),
            ("height", "12"),
            ("width", "20"),
            ("shape", "blocks"),
        ],
    ),
    // D-510: the numbers, the centre and the words.
    (
        "core.tile_wipe",
        &[
            ("completion", "40"),
            ("center", "30,60"),
            ("rotation", "15"),
            ("border", "3"),
            ("tiles", "6"),
            ("shape", "rectangle"),
            ("reverse", "on"),
        ],
    ),
    // D-509: the numbers and the two layers.
    (
        "core.glass_melt",
        &[
            ("completion", "40"),
            ("layer_to_reveal", "layer-4"),
            ("gradient_layer", "layer-4"),
            ("softness", "5"),
            ("displacement_amount", "-20"),
        ],
    ),
    // D-506: the two views, the numbers and the words.
    (
        "core.three_d_glasses",
        &[
            ("left_view", "layer-4"),
            ("right_view", "layer-4"),
            ("scene_convergence", "3"),
            ("vertical_alignment", "-2"),
            ("units", "percent"),
            ("swap_left_right", "on"),
            ("three_d_view", "red_green"),
            ("balance", "1.5"),
        ],
    ),
    // D-458.
    (
        "core.simple_wire_removal",
        &[
            ("point_a", "20,30"),
            ("point_b", "80,70"),
            ("removal_style", "displace_horizontal"),
            ("thickness", "7"),
            ("slope", "30"),
            ("mirror_blend", "60"),
            ("frame_offset", "2"),
        ],
    ),
    // D-456.
    (
        "core.deband",
        &[
            ("view", "selected_edge"),
            ("edge_threshold", "6"),
            ("edge_softness", "3"),
            ("blur_amount", "12"),
            ("edge_blur_reduction", "80"),
            ("diffusion_amount", "2"),
            ("edge_diffusion_reduction", "90"),
            ("diffusion_reseed_interval", "2"),
            ("noise_amount", "3"),
            ("edge_noise_reduction", "70"),
            ("noise_reseed_interval", "4"),
        ],
    ),
    // D-454.
    (
        "core.match_grain",
        &[
            ("map_layer", "layer-4"),
            ("intensity", "2"),
            ("size", "2"),
            ("softness", "0.3"),
            ("aspect_ratio", "1.5"),
            ("red_intensity", "1.5"),
            ("green_intensity", "1"),
            ("blue_intensity", "0.5"),
            ("monochromatic", "off"),
            ("saturation", "0.5"),
            ("blending_mode", "add"),
            ("shadows", "1"),
            ("midtones", "2"),
            ("highlights", "0.5"),
            ("midpoint", "0.4"),
            ("animation_speed", "1"),
            ("animate_smoothly", "on"),
            ("random_seed", "5"),
        ],
    ),
    // D-447.
    (
        "core.brush_strokes",
        &[
            ("stroke_angle", "200"),
            ("brush_size", "1.5"),
            ("stroke_length", "5.5"),
            ("stroke_density", "1.8"),
            ("stroke_randomness", "0.6"),
            ("paint_surface", "black"),
            ("blend_with_original", "20"),
            ("random_seed", "31"),
            ("animate", "off"),
        ],
    ),
    // D-448.
    (
        "core.cartoon",
        &[
            ("render", "fill_and_edges"),
            ("detail_radius", "3"),
            ("detail_threshold", "30"),
            ("shading_steps", "5"),
            ("shading_smoothness", "20"),
            ("edge_threshold", "2.5"),
            ("edge_width", "2.5"),
            ("edge_softness", "30"),
            ("edge_opacity", "80"),
            ("edge_enhancement", "40"),
            ("edge_black_level", "0.2"),
            ("edge_contrast", "0.7"),
        ],
    ),
    // D-449.
    ("core.block_load", &[("completion", "40"), ("scans", "3"), ("start_cleared", "off"), ("bilinear", "on")]),
    // D-460.
    ("core.burn_film", &[("burn", "40"), ("center", "30,60"), ("seed", "3")]),
    // D-461.
    ("core.hex_tile", &[("render", "fold_seamless"), ("radius", "40"), ("center", "30,60"), ("lock_center", "on"), ("rotate", "15"), ("smearing", "25")]),
    // D-462.
    (
        "core.mr_smoothie",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("property", "hue"),
            ("smoothness", "12"),
            ("sample_a", "10,20"),
            ("sample_b", "90,80"),
            ("phase", "45"),
            ("color_loop", "bab"),
        ],
    ),
    // D-463.
    (
        "core.plastic",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("property", "luminance"),
            ("softness", "6"),
            ("height", "-40"),
            ("cut_min", "20"),
            ("cut_max", "80"),
            ("light_intensity", "150"),
            ("light_color", "%23ffeecc"),
            ("light_type", "point"),
            ("light_height", "40"),
            ("light_position", "25,75"),
            ("light_direction", "30"),
            ("ambient_light_color", "%23ccddff"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("dust", "30"),
            ("specular", "80"),
            ("roughness", "0.2"),
            ("metal", "40"),
        ],
    ),
    // D-464.
    (
        "core.repeat_tile",
        &[("expand_right", "4"), ("expand_left", "2"), ("expand_down", "3"), ("expand_up", "1"), ("tiling", "brick"), ("blend_borders", "30")],
    ),
    // D-465.
    ("core.scatter", &[("scatter_amount", "6"), ("grain", "horizontal"), ("randomize", "on")]),
    // D-466.
    (
        "core.strobe_light",
        &[
            ("strobe_color", "%23ff8800"),
            ("blend_with_original", "20"),
            ("strobe_duration", "0.2"),
            ("strobe_period", "0.5"),
            ("random_strobe_probability", "10"),
            ("strobe", "transparent"),
            ("strobe_operator", "xor"),
            ("random_seed", "7"),
        ],
    ),
    // D-469.
    (
        "core.threshold_rgb",
        &[
            ("red_threshold", "200"),
            ("green_threshold", "90"),
            ("blue_threshold", "40"),
            ("invert_red", "off"),
            ("invert_green", "on"),
            ("invert_blue", "off"),
            ("blend_with_original", "10"),
        ],
    ),
    // D-470.
    (
        "core.contour_paint",
        &[
            ("map_layer", "layer-4"),
            ("view", "final_result"),
            ("stylization", "70"),
            ("cleanliness", "30"),
            ("smear_iterations", "4"),
            ("apply_to_alpha", "on"),
            ("draw_color_fills", "over_smeared_image"),
            ("reduction_method", "median"),
            ("posterization_level", "6"),
            ("low_strength", "10"),
            ("high_strength", "90"),
            ("offset_flow_angles", "45"),
            ("quantize_flow_angles", "15"),
        ],
    ),
    // D-562.
    (
        "core.cc_vignette",
        &[("amount", "-60"), ("angle_of_view", "120"), ("center", "30,70"), ("pin_highlights", "40")],
    ),
    // D-468.
    (
        "core.channel_threshold",
        &[("threshold", "100"), ("channel", "saturation"), ("invert", "on"), ("blend_with_original", "20")],
    ),
    // D-467.
    (
        "core.texturize",
        &[("map_layer", "layer-4"), ("fit", "center"), ("light_direction", "45"), ("texture_contrast", "1.5")],
    ),
    ("core.smart_blur", &[("radius", "5"), ("threshold", "20")]),
    // D-358.
    ("core.bilateral_blur", &[("radius", "8"), ("threshold", "30"), ("colorize", "off")]),
    // D-360..D-362.
    ("core.cross_blur", &[("radius_x", "20"), ("radius_y", "6"), ("mode", "add"), ("edges", "repeat")]),
    ("core.spin_zoom_blur", &[("type", "rotate"), ("amount", "45"), ("quality", "80"), ("center", "40,60")]),
    ("core.fast_zoom_blur", &[("amount", "70"), ("center", "30,40"), ("zoom", "brightest")]),
    // D-365..D-367.
    ("core.broadcast_safe", &[("locale", "pal"), ("method", "key_out_unsafe"), ("max_amplitude", "100")]),
    (
        "core.color_neutralizer",
        &[
            ("shadows_unbalance", "#3c2d1e"),
            ("midtones_unbalance", "#968064"),
            ("highlights_unbalance", "#f0e6c8"),
            ("shadows", "20, 0, -20"),
            ("midtones", "0, 10, 0"),
            ("highlights", "-30, 0, 30"),
            ("pinning", "50"),
            ("black_point", "20"),
            ("white_point", "230"),
        ],
    ),
    ("core.color_offset", &[("red_phase", "90"), ("green_phase", "-45"), ("blue_phase", "400"), ("overflow", "polarize")]),
    // D-368..D-370: lines, numbers, colours and words.
    (
        "core.kernel",
        &[("line_1", "0, -1, 0"), ("line_2", "-1, 5, -1"), ("line_3", "0, -1, 0"), ("divider", "1"), ("absolute_values", "on")],
    ),
    (
        "core.toner",
        &[
            ("tones", "pentone"),
            ("highlights", "#fff3b0"),
            ("brights", "#ff9e00"),
            ("midtones", "#9d4edd"),
            ("darktones", "#3c096c"),
            ("shadows", "#10002b"),
        ],
    ),
    (
        "core.change_color",
        &[
            ("view", "mask"),
            ("hue_transform", "120"),
            ("lightness_transform", "-30"),
            ("saturation_transform", "20"),
            ("color_to_change", "#00ff00"),
            ("tolerance", "30"),
            ("softness", "15"),
            ("match_colors", "chroma"),
            ("invert_mask", "on"),
        ],
    ),
    // D-374..D-376.
    ("core.color_balance_hls", &[("hue", "120"), ("lightness", "-20"), ("saturation", "30")]),
    // D-382.
    (
        "core.gamma_pedestal_gain",
        &[
            ("black_stretch", "1.5"),
            ("red_gamma", "1"),
            ("red_pedestal", "0"),
            ("red_gain", "1.2"),
            ("green_gamma", "1"),
            ("green_pedestal", "0.1"),
            ("green_gain", "1"),
            ("blue_gamma", "1.5"),
            ("blue_pedestal", "0"),
            ("blue_gain", "1"),
        ],
    ),
    // D-384: a word, a colour, a number and a switch.
    ("core.photo_filter", &[("filter", "custom"), ("color", "#3366cc"), ("density", "60"), ("preserve_luminosity", "off")]),
    // D-397: the look and the 22 numbers.
    (
        "core.color_grade",
        &[
            ("look", ""),
            ("temperature", "10"),
            ("tint", "-5"),
            ("exposure", "0.5"),
            ("contrast", "20"),
            ("highlights", "-10"),
            ("shadows", "10"),
            ("whites", "5"),
            ("blacks", "-5"),
            ("saturation", "110"),
            ("look_intensity", "100"),
            ("faded_film", "10"),
            ("vibrance", "15"),
            ("creative_saturation", "100"),
            ("shadow_tint_hue", "200"),
            ("shadow_tint_amount", "20"),
            ("highlight_tint_hue", "40"),
            ("highlight_tint_amount", "20"),
            ("tint_balance", "0"),
            ("vignette_amount", "-1"),
            ("vignette_midpoint", "50"),
            ("vignette_roundness", "0"),
            ("vignette_feather", "50"),
        ],
    ),
    (
        "core.color_link",
        &[
            ("map_layer", "layer-4"),
            ("sample", "median"),
            ("clip", "10"),
            ("stencil", "on"),
            ("opacity", "60"),
            ("blending_mode", "overlay"),
        ],
    ),
    (
        "core.color_stabilizer",
        &[
            ("stabilize", "curves"),
            ("reference_frame", "3"),
            ("black_point", "10, 20"),
            ("mid_point", "50, 40"),
            ("white_point", "90, 60"),
            ("sample_size", "8"),
        ],
    ),
    // D-205: the numbers, the point and the word.
    (
        "core.kaleidoscope",
        &[
            ("segments", "8"),
            ("rotation", "15"),
            ("size", "150"),
            ("center", "40,60"),
            ("mode", "repeat"),
        ],
    ),
    // D-206: the two words and the numbers.
    (
        "core.roughen_edges",
        &[
            ("edge_type", "roughen_color"),
            ("edge_color", "%23204080"),
            ("border", "12"),
            ("size", "20"),
            ("complexity", "4"),
            ("evolution", "90"),
            ("speed", "5"),
            ("seed", "3"),
        ],
    ),
    // D-207: the two points, the five numbers, the two colours and the word.
    (
        "core.beam",
        &[
            ("start", "20,30"),
            ("end", "80,70"),
            ("length", "40"),
            ("time", "25"),
            ("start_thickness", "6"),
            ("end_thickness", "16"),
            ("softness", "30"),
            ("inside_color", "%23fff0c0"),
            ("outside_color", "%23ff4a1a"),
            ("composite", "off"),
        ],
    ),
    // D-208: the four points, the four colours, the two numbers and the word.
    (
        "core.four_color_gradient",
        &[
            ("point_1", "20,30"),
            ("point_2", "80,20"),
            ("point_3", "15,85"),
            ("point_4", "90,75"),
            ("color_1", "%23ff8000"),
            ("color_2", "%2300c0ff"),
            ("color_3", "%23ffffff"),
            ("color_4", "%23202080"),
            ("blend", "60"),
            ("opacity", "80"),
            ("blending_mode", "multiply"),
            ("jitter", "40"),
        ],
    ),
    // D-209: the two words, the six numbers, the two colours and the blend.
    (
        "core.cell_pattern",
        &[
            ("pattern", "crystals"),
            ("invert", "on"),
            ("contrast", "250"),
            ("disperse", "0.5"),
            ("size", "24"),
            ("evolution", "45"),
            ("seed", "7"),
            ("dark_color", "%23203070"),
            ("light_color", "%23ffd060"),
            ("opacity", "80"),
            ("blend", "screen"),
            // D-762.
            ("offset", "3,-2"),
            ("overflow", "soft_clamp"),
            ("cycle_evolution", "on"),
            ("cycle", "2"),
            ("tiling", "on"),
            ("tiles_horizontal", "4"),
            ("tiles_vertical", "3"),
        ],
    ),
    // D-210: the number, the two words and the point.
    (
        "core.optics_compensation",
        &[
            ("field_of_view", "120"),
            ("reverse", "on"),
            ("orientation", "diagonal"),
            ("center", "40,60"),
        ],
    ),
    // D-211: the colour, the numbers, the point and the two words.
    (
        "core.radial_shadow",
        &[
            ("color", "%232040a0"),
            ("opacity", "75"),
            ("light", "20,30"),
            ("distance", "25"),
            ("softness", "4"),
            ("render", "glass_edge"),
            ("color_influence", "60"),
            ("shadow_only", "on"),
        ],
    ),
    // D-212: the channel, the four numbers and invert.
    (
        "core.extract",
        &[
            ("channel", "green"),
            ("black_point", "30"),
            ("white_point", "220"),
            ("black_softness", "10"),
            ("white_softness", "20"),
            ("invert", "on"),
        ],
    ),
    // D-213: the four settings of each.
    (
        "core.bevel_alpha",
        &[
            ("edge_thickness", "5"),
            ("light_angle", "45"),
            ("light_color", "%232040a0"),
            ("light_intensity", "0.7"),
        ],
    ),
    (
        "core.bevel_edges",
        &[
            ("edge_thickness", "0.25"),
            ("light_angle", "135"),
            ("light_color", "%23ffd070"),
            ("light_intensity", "0.9"),
        ],
    ),
    // D-214.
    (
        "core.block_dissolve",
        &[("completion", "40"), ("block_width", "12"), ("block_height", "6"), ("feather", "3")],
    ),
    // D-305.
    (
        "core.shift_channels",
        &[("take_alpha", "luminance"), ("take_red", "full"), ("take_green", "hue"), ("take_blue", "off")],
    ),
    // D-312, D-313.
    (
        "core.solid_composite",
        &[("source_opacity", "80"), ("color", "%23203040"), ("opacity", "60"), ("blend", "screen")],
    ),
    (
        "core.channel_blur",
        &[
            ("red_blurriness", "3"),
            ("green_blurriness", "0"),
            ("blue_blurriness", "8"),
            ("alpha_blurriness", "2"),
            ("edges", "repeat"),
            ("dimensions", "horizontal"),
        ],
    ),
    // D-327.
    (
        "core.fast_box_blur",
        &[("radius", "6.5"), ("iterations", "2"), ("edges", "repeat"), ("dimensions", "vertical")],
    ),
    // D-471.
    ("core.fast_blur_legacy", &[("blurriness", "20"), ("edges", "repeat"), ("dimensions", "horizontal")]),
    // D-472.
    ("core.gaussian_blur_legacy", &[("blurriness", "15"), ("dimensions", "vertical")]),
    // D-316.
    (
        "core.colorama",
        &[
            ("get_phase", "luminance"),
            ("map_layer", "layer-4"),
            ("fit", "center"),
            ("phase_shift", "90"),
            ("cycle_repetitions", "2"),
            ("stops", "3"),
            ("color_1", "%23102030"),
            ("color_2", "%23405060"),
            ("color_3", "%23708090"),
            ("color_4", "%23a0b0c0"),
            ("color_5", "%23d0e0f0"),
            ("blend_with_original", "25"),
            // D-381.
            ("add_phase_from", "hue"),
            ("add_mode", "screen"),
            ("interpolate", "off"),
            ("opacity_1", "90"),
            ("opacity_2", "80"),
            ("opacity_3", "70"),
            ("opacity_4", "60"),
            ("opacity_5", "50"),
            ("modify", "lightness"),
            ("modify_alpha", "on"),
            ("change_empty", "on"),
            ("matching_mode", "chroma"),
            ("matching_color", "%23336699"),
            ("matching_tolerance", "20"),
            ("matching_softness", "10"),
            ("mask_layer", "layer-4"),
            ("masking_mode", "inverse_alpha"),
            ("composite_over", "off"),
        ],
    ),
    // D-377..D-379.
    ("core.bend_it", &[("bend", "-90"), ("start", "20,80"), ("end", "70,10"), ("render_prestart", "mirror"), ("distort", "extended")]),
    ("core.bender", &[("amount", "35"), ("style", "boxer"), ("adjust_to_distance", "on"), ("top", "40,10"), ("base", "60,90")]),
    // D-385..D-387.
    (
        "core.flow_motion",
        &[
            ("knot_1", "20,30"),
            ("amount_1", "40"),
            ("knot_2", "70,60"),
            ("amount_2", "-25"),
            ("falloff", "3"),
            ("tile_edges", "off"),
            ("finer_controls", "on"),
            ("antialiasing", "high"),
        ],
    ),
    ("core.griddler", &[("horizontal_scale", "60"), ("vertical_scale", "-120"), ("tile_size", "20"), ("rotation", "30"), ("cut_tiles", "off")]),
    ("core.fisheye", &[("center", "40,60"), ("size", "80"), ("convergence", "-70")]),
    // D-388..D-390.
    (
        "core.page_turn",
        &[
            ("controls", "classic"),
            ("fold_position", "40,60"),
            ("fold_direction", "-30"),
            ("fold_radius", "12"),
            ("light_direction", "60"),
            ("render", "front"),
            ("map_layer", "layer-4"),
            ("back_opacity", "70"),
            ("paper_color", "%23eeddcc"),
        ],
    ),
    (
        "core.power_pin",
        &[
            ("top_left", "10,5"),
            ("top_right", "90,0"),
            ("bottom_left", "0,100"),
            ("bottom_right", "100,90"),
            ("perspective", "50"),
            ("unstretch", "on"),
            ("expansion_top", "10"),
            ("expansion_left", "-5"),
            ("expansion_right", "20"),
            ("expansion_bottom", "0"),
        ],
    ),
    // D-607.
    (
        "core.mask_smear",
        &[("source_mask", "1"), ("boundary_mask", "2"), ("mask_offset", "3,0"), ("mask_rotation", "45"), ("mask_scale", "150"), ("percent", "60"), ("elasticity", "liquid")],
    ),
    // D-606.
    (
        "core.reshape",
        &[
            ("source_mask", "1"),
            ("destination_mask", "2"),
            ("boundary_mask", "3"),
            ("percent", "60"),
            ("elasticity", "liquid"),
            ("correspondence_points", "0,0,50,25"),
            ("interpolation", "smooth"),
        ],
    ),
    // D-605, D-700.
    (
        "core.liquify",
        &[
            ("tool", "pucker"),
            ("brush_size", "40"),
            ("brush_pressure", "70"),
            ("stroke", "50,50"),
            ("distortion_percentage", "120"),
            ("distortion_mesh_offset", "2,3"),
            ("turbulent_jitter", "40"),
            ("clone_offset", "5,-5"),
            ("reconstruction_mode", "displace"),
            ("freeze_area_mask", "1"),
            ("strokes", "10%2030%2050%2070%200%200%200%2020%2020"),
        ],
    ),
    // D-604.
    (
        "core.warp",
        &[("warp_style", "twist"), ("warp_axis", "vertical"), ("bend", "40"), ("horizontal_distortion", "10"), ("vertical_distortion", "-10")],
    ),
    // D-603.
    (
        "core.mesh_warp",
        &[
            ("rows", "2"),
            ("columns", "2"),
            ("quality", "6"),
            ("distortion_mesh", "0,0,50,0,100,0,0,50,70,50,100,50,0,100,50,100,100,100"),
            ("vertex_handles", "0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,-20,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0"),
        ],
    ),
    // D-602.
    (
        "core.bezier_warp",
        &[
            ("top_left_vertex", "10,5"),
            ("top_left_tangent", "30,20"),
            ("top_right_tangent", "70,-10"),
            ("right_top_vertex", "100,0"),
            ("right_top_tangent", "90,30"),
            ("right_bottom_tangent", "110,60"),
            ("bottom_right_vertex", "95,100"),
            ("bottom_right_tangent", "60,90"),
            ("bottom_left_tangent", "30,110"),
            ("left_bottom_vertex", "0,100"),
            ("left_bottom_tangent", "-5,60"),
            ("left_top_tangent", "5,30"),
            ("quality", "6"),
        ],
    ),
    ("core.ripple_pulse", &[("center", "40,60"), ("pulse_level", "30"), ("time_span", "0.5"), ("amplitude", "25"), ("render_bump_map", "on")]),
    // D-391..D-393.
    (
        "core.slant",
        &[("slant", "-25"), ("stretching", "on"), ("height", "60"), ("floor", "30,80"), ("set_color", "on"), ("color", "%23112233")],
    ),
    ("core.smear", &[("from", "30,40"), ("to", "70,55"), ("reach", "150"), ("radius", "40")]),
    ("core.split", &[("point_a", "20,30"), ("point_b", "80,70"), ("split", "35")]),
    ("core.split_2", &[("point_a", "20,30"), ("point_b", "80,70"), ("split_1", "35"), ("split_2", "10")]),
    ("core.tiles", &[("scale", "25"), ("center", "30,40"), ("blend", "20")]),
    (
        "core.magnify",
        &[
            ("shape", "square"),
            ("center", "30,40"),
            ("magnification", "300"),
            ("link", "size"),
            ("size", "20"),
            ("feather", "5"),
            ("opacity", "80"),
            ("scaling", "soft"),
            ("blending_mode", "screen"),
            ("resize_layer", "on"),
        ],
    ),
    ("core.spherize", &[("radius", "40"), ("center", "30,60")]),
    // D-413.
    (
        "core.checkerboard",
        &[
            ("anchor", "30,40"),
            ("size_from", "corner_point"),
            ("corner", "45,60"),
            ("width", "20"),
            ("height", "10"),
            ("feather_width", "2"),
            ("feather_height", "3"),
            ("color", "%23ff8000"),
            ("opacity", "80"),
            ("blending_mode", "stencil_alpha"),
        ],
    ),
    // D-414.
    (
        "core.circle",
        &[
            ("center", "30,60"),
            ("radius", "40"),
            ("edge", "thickness"),
            ("edge_thickness", "12"),
            ("feather_outer", "2"),
            ("feather_inner", "3"),
            ("invert", "on"),
            ("color", "%23ff8000"),
            ("opacity", "80"),
            ("blending_mode", "screen"),
        ],
    ),
    // D-415.
    (
        "core.ellipse",
        &[
            ("center", "30,60"),
            ("width", "120"),
            ("height", "80"),
            ("thickness", "6"),
            ("softness", "40"),
            ("inside_color", "%23fff0c0"),
            ("outside_color", "%23ff8000"),
            ("composite", "off"),
        ],
    ),
    ("core.detail_upscale", &[("scale", "250"), ("reduce_noise", "30"), ("detail", "40")]),
    // D-420.
    (
        "core.audio_spectrum",
        &[
            ("map_layer", "layer-4"),
            ("start_point", "5,70"),
            ("end_point", "95,30"),
            ("path", "0"),
            ("use_polar_path", "on"),
            ("start_frequency", "40"),
            ("end_frequency", "4000"),
            ("frequency_bands", "32"),
            ("maximum_height", "120"),
            ("audio_duration", "60"),
            ("audio_offset", "-20"),
            ("thickness", "5"),
            ("softness", "30"),
            ("inside_color", "%23fff0c0"),
            ("outside_color", "%23ff8000"),
            ("blend_overlapping_colors", "on"),
            ("hue_interpolation", "90"),
            ("dynamic_hue_phase", "on"),
            ("color_symmetry", "on"),
            ("display_options", "analog_lines"),
            ("side_options", "side_a"),
            ("duration_averaging", "on"),
            ("composite", "on"),
        ],
    ),
    // D-421.
    (
        "core.audio_waveform",
        &[
            ("map_layer", "layer-4"),
            ("start_point", "5,70"),
            ("end_point", "95,30"),
            ("path", "0"),
            ("displayed_samples", "48"),
            ("maximum_height", "120"),
            ("audio_duration", "60"),
            ("audio_offset", "-20"),
            ("thickness", "5"),
            ("softness", "30"),
            ("random_seed", "7"),
            ("inside_color", "%23fff0c0"),
            ("outside_color", "%23ff8000"),
            ("waveform_options", "left"),
            ("display_options", "digital"),
            ("composite", "on"),
        ],
    ),
    // D-416.
    (
        "core.fractal",
        &[
            ("set_choice", "julia"),
            ("equation", "z3"),
            ("mandelbrot_center", "-0.5,0.25"),
            ("mandelbrot_magnification", "2"),
            ("mandelbrot_escape_limit", "300"),
            ("julia_center", "0.1,-0.2"),
            ("julia_magnification", "1"),
            ("julia_escape_limit", "200"),
            ("overlay", "on"),
            ("transparency", "on"),
            ("palette", "hue_wheel"),
            ("hue", "120"),
            ("cycle_steps", "12"),
            ("cycle_offset", "3"),
            ("edge_highlight", "on"),
            ("oversample_method", "brute_force"),
            ("oversample_factor", "3"),
        ],
    ),
    // D-419.
    (
        "core.eyedropper_fill",
        &[
            ("sample_point", "25,60"),
            ("sample_radius", "4"),
            ("average_pixel_colors", "including_alpha"),
            ("maintain_original_alpha", "on"),
            ("blend_with_original", "30"),
        ],
    ),
    // D-440.
    (
        "core.paint_bucket",
        &[
            ("fill_point", "25,60"),
            ("fill_selector", "straight_color"),
            ("tolerance", "30"),
            ("view_threshold", "on"),
            ("stroke", "feather"),
            ("invert_fill", "on"),
            ("spread_radius", "5"),
            ("stroke_width", "6"),
            ("feather_softness", "7"),
            ("color", "#3080ff"),
            ("opacity", "40"),
            ("blending_mode", "fill_only"),
        ],
    ),
    // D-450.
    (
        "core.noise_alpha",
        &[
            ("noise", "squared_animation"),
            ("amount", "45"),
            ("original_alpha", "edges"),
            ("overflow", "wrap_back"),
            ("random_seed", "9"),
            ("noise_phase", "120"),
            ("cycle_noise", "on"),
            ("cycle", "3"),
        ],
    ),
    // D-451.
    (
        "core.noise_hls",
        &[
            ("noise", "grain"),
            ("hue", "35"),
            ("lightness", "20"),
            ("saturation", "55"),
            ("grain_size", "2.5"),
            ("noise_phase", "240"),
        ],
    ),
    // D-452.
    (
        "core.noise_hls_auto",
        &[
            ("noise", "squared"),
            ("hue", "25"),
            ("lightness", "15"),
            ("saturation", "45"),
            ("grain_size", "3"),
            ("animation_speed", "0.75"),
        ],
    ),
    // D-443.
    (
        "core.add_grain",
        &[
            ("intensity", "2.5"),
            ("size", "3"),
            ("softness", "0.4"),
            ("aspect_ratio", "1.5"),
            ("red_intensity", "2"),
            ("green_intensity", "0.5"),
            ("blue_intensity", "3"),
            ("monochromatic", "on"),
            ("saturation", "0.25"),
            ("blending_mode", "overlay"),
            ("shadows", "2"),
            ("midtones", "0.5"),
            ("highlights", "4"),
            ("midpoint", "0.3"),
            ("animation_speed", "2"),
            ("animate_smoothly", "off"),
            ("random_seed", "9"),
        ],
    ),
    // D-442.
    (
        "core.scribble",
        &[
            ("scribble", "all_masks_using_modes"),
            ("mask", "2"),
            ("fill_type", "outside_edge"),
            ("edge_width", "6"),
            ("color", "#3080ff"),
            ("opacity", "70"),
            ("angle", "-30"),
            ("stroke_width", "3"),
            ("curviness", "40"),
            ("curviness_variation", "10"),
            ("spacing", "7"),
            ("spacing_variation", "2"),
            ("path_overlap", "-1"),
            ("path_overlap_variation", "3"),
            ("start", "10"),
            ("end", "90"),
            ("fill_paths_sequentially", "off"),
            ("wiggle_type", "jumpy"),
            ("wiggles_per_second", "12"),
            ("random_seed", "7"),
            ("composite", "reveal"),
        ],
    ),
    // D-441.
    (
        "core.write_on",
        &[
            ("brush_position", "25,60"),
            ("color", "#3080ff"),
            ("brush_size", "12"),
            ("brush_hardness", "40"),
            ("brush_opacity", "70"),
            ("stroke_length", "2"),
            ("brush_spacing", "0.05"),
            ("paint_time_properties", "opacity"),
            ("brush_time_properties", "size_and_hardness"),
            ("paint_style", "reveal"),
        ],
    ),
    // D-418.
    (
        "core.fill",
        &[
            ("mask", "2"),
            ("all_masks", "on"),
            ("color", "%233080ff"),
            ("invert", "on"),
            ("horizontal_feather", "3"),
            ("vertical_feather", "6"),
            ("opacity", "70"),
        ],
    ),
    // D-417.
    (
        "core.grid",
        &[
            ("anchor", "30,60"),
            ("size_from", "width_and_height_sliders"),
            ("corner", "70,80"),
            ("width", "40"),
            ("height", "30"),
            ("border", "3"),
            ("feather_width", "2"),
            ("feather_height", "1"),
            ("invert", "on"),
            ("color", "%23ff8000"),
            ("opacity", "80"),
            ("blending_mode", "screen"),
        ],
    ),
    // D-426.
    (
        "core.lens_flare",
        &[("flare_center", "70,20"), ("flare_brightness", "150"), ("lens_type", "105mm"), ("blend_with_original", "25")],
    ),
    // D-445.
    (
        "core.turbulent_noise",
        &[
            ("fractal_type", "turbulent"),
            ("noise_type", "block"),
            ("invert", "on"),
            ("contrast", "170"),
            ("brightness", "-10"),
            ("size", "40"),
            ("scale_width", "150"),
            ("scale_height", "60"),
            ("offset", "15, -8"),
            ("complexity", "9"),
            ("evolution", "45"),
            ("seed", "3"),
            ("opacity", "60"),
            ("blend", "screen"),
        ],
    ),
    // D-446.
    (
        "core.curl_noise",
        &[
            ("source", "internal"),
            ("speed", "35"),
            ("direction", "200"),
            ("size", "6"),
            ("offset", "2, -1"),
            // D-750.
            ("uniform_scaling", "off"),
            ("scale_width", "150"),
            ("scale_height", "60"),
            ("evolution", "45"),
            ("turbulence_speed", "60"),
            ("swirl", "-200"),
            ("density", "-30"),
            ("smoothness", "20"),
            ("vertical_bias", "70"),
            ("sample_count", "9"),
            ("sample_radius", "6.5"),
            ("flow_softness", "10"),
            ("edge_definition", "30"),
            ("flow_falloff", "40"),
            ("view", "curl_generation"),
            ("contrast", "150"),
            ("brightness", "-5"),
            ("clip_hdr_results", "off"),
            ("channel", "green"),
        ],
    ),
    // D-444.
    (
        "core.vegas",
        &[
            ("stroke", "shapes"),
            ("mask", "2"),
            ("all_masks", "on"),
            ("segments", "8"),
            ("length", "0.5"),
            ("segment_distribution", "even"),
            ("rotation", "90"),
            ("random_phase", "on"),
            ("random_seed", "7"),
            ("blend_mode", "under"),
            ("color", "%23ff8000"),
            ("width", "6"),
            ("hardness", "0.5"),
            ("start_opacity", "0.8"),
            ("mid_point_opacity", "0.25"),
            ("mid_point_position", "0.4"),
            ("end_opacity", "0.1"),
        ],
    ),
    // D-425.
    (
        "core.threads",
        &[
            ("width", "12"),
            ("height", "8"),
            ("overlaps", "2"),
            ("direction", "30"),
            ("center", "40,60"),
            ("coverage", "80"),
            ("shadowing", "70"),
            ("texture", "25"),
        ],
    ),
    // D-424.
    (
        "core.glue_gun",
        &[
            ("brush_position", "20,70"),
            ("stroke_width", "12"),
            ("density", "3"),
            ("time_span", "2"),
            ("reflection", "80"),
            ("strength", "30"),
            ("paint_style", "wobbly"),
            ("wobble_width", "4"),
            ("wobble_height", "6"),
            ("wobble_speed", "2"),
            ("light_intensity", "150"),
            ("light_color", "%23ff8000"),
            ("light_type", "point"),
            ("light_height", "40"),
            ("light_position", "60,20"),
            ("light_direction", "30"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("specular", "90"),
            ("roughness", "0.2"),
            ("metal", "40"),
        ],
    ),
    // D-422.
    (
        "core.light_burst",
        &[
            ("center", "30,70"),
            ("intensity", "250"),
            ("ray_length", "80"),
            ("burst", "fade"),
            ("set_color", "on"),
            ("color", "%23ff8000"),
        ],
    ),
    ("core.arbitrary_map", &[("map", ""), ("phase", "20"), ("apply_to_alpha", "on")]),
    // D-396: a word and nine families of four numbers.
    (
        "core.selective_color",
        &[
            ("method", "absolute"),
            ("reds", "10, -20, 30, 5"),
            ("yellows", "0, 0, -40, 0"),
            ("greens", "0, 25, 0, 0"),
            ("cyans", "-30, 0, 0, 0"),
            ("blues", "0, 0, 15, -5"),
            ("magentas", "0, -10, 0, 10"),
            ("whites", "0, 0, 0, 20"),
            ("neutrals", "-5, 0, 10, 0"),
            ("blacks", "0, 0, 0, -15"),
        ],
    ),
    // D-400.
    (
        "core.shadow_highlight",
        &[
            ("shadow_amount", "70"),
            ("highlight_amount", "40"),
            ("shadow_tonal_width", "60"),
            ("shadow_radius", "12"),
            ("highlight_tonal_width", "35"),
            ("highlight_radius", "45"),
            ("color_correction", "80"),
            ("midtone_contrast", "25"),
            ("black_clip", "2"),
            ("white_clip", "3"),
            // D-564.
            ("auto_amounts", "on"),
            ("temporal_smoothing", "0.5"),
            ("scene_detect", "on"),
        ],
    ),
    // D-402: the three colours and the blend.
    (
        "core.tritone",
        &[
            ("highlights", "%23fdf3a7"),
            ("midtones", "%23c0392b"),
            ("shadows", "%231a2a6c"),
            ("blend_with_original", "30"),
        ],
    ),
    // D-403: the colour, the amount and a matte.
    (
        "core.aerial_haze",
        &[
            ("haze_color", "%23a0b8d0"),
            ("amount", "45"),
            ("map_layer", "layer-4"),
            ("fit", "center"),
        ],
    ),
    // D-408: the points, the numbers and the words.
    (
        "core.transform",
        &[
            ("anchor_point", "40,60"),
            ("position", "55,45"),
            ("uniform_scale", "off"),
            ("scale_height", "80"),
            ("scale_width", "120"),
            ("skew", "10"),
            ("skew_axis", "30"),
            ("rotation", "15"),
            ("opacity", "90"),
            ("use_composition_shutter_angle", "off"),
            ("shutter_angle", "270"),
            ("sampling", "bicubic"),
        ],
    ),
    (
        "core.blobbylize",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("property", "luminance"),
            ("softness", "6"),
            ("cut_away", "20"),
            ("light_intensity", "150"),
            ("light_color", "%23ffeecc"),
            ("light_type", "point"),
            ("light_height", "40"),
            ("light_position", "25,75"),
            ("light_direction", "30"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("specular", "80"),
            ("roughness", "0.2"),
            ("metal", "40"),
        ],
    ),
    // D-317.
    (
        "core.glass",
        &[
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("property", "luminance"),
            ("softness", "6"),
            ("height", "-40"),
            ("displacement", "75"),
            ("light_angle", "30"),
            ("light_color", "%23ffeecc"),
            ("light_intensity", "60"),
            // D-561.
            ("light_type", "point"),
            ("light_height", "40"),
            ("light_position", "25,75"),
            ("ambient", "10"),
            ("diffuse", "60"),
            ("specular", "80"),
            ("roughness", "0.2"),
            ("metal", "40"),
        ],
    ),
    // D-336.
    (
        "core.vector_blur",
        &[
            ("type", "perpendicular"),
            ("amount", "4"),
            ("angle_offset", "30"),
            ("ridge_smoothness", "20"),
            ("map_layer", "layer-4"),
            ("fit", "tile"),
            ("property", "hue"),
            ("map_softness", "12"),
        ],
    ),
    // D-347.
    (
        "core.moment_map",
        &[("max_time", "-2"), ("resolution", "24"), ("map_layer", "layer-4"), ("fit", "tile")],
    ),
    // D-348.
    (
        "core.pass_extract",
        &[("pass", "normals"), ("black_point", "-1"), ("white_point", "1"), ("invert", "on"), ("clamp", "off"), ("unmult", "on")],
    ),
    ("core.depth_key", &[("depth", "6"), ("feather", "4"), ("invert", "on")]),
    // D-349.
    ("core.id_key", &[("aux_channel", "material_id"), ("id", "2"), ("feather", "1.5"), ("invert", "on")]),
    // D-527: every setting.
    (
        "core.timecode",
        &[
            ("display_format", "feet_frames_16mm"),
            ("time_source", "custom"),
            ("time_units", "60"),
            ("drop_frame", "on"),
            ("starting_frame", "1000"),
            ("text_position", "10,20"),
            ("text_size", "48"),
            ("text_color", "#ff0000"),
            ("show_box", "off"),
            ("box_color", "#00ff00"),
            ("opacity", "40"),
            ("composite_on_original", "off"),
        ],
    ),
    // D-526: every setting, the type as number_type.
    (
        "core.numbers",
        &[
            ("number_type", "long_date"),
            ("random_values", "on"),
            ("value", "365"),
            ("decimal_places", "2"),
            ("position", "10,20"),
            ("display", "stroke_over_fill"),
            ("fill_color", "#ff0000"),
            ("stroke_color", "#00ff00"),
            ("stroke_width", "5"),
            ("size", "48"),
            ("tracking", "20"),
            ("proportional_spacing", "off"),
            ("composite_on_original", "off"),
            ("font", "MPLUSRounded1c-Regular.ttf"),
            ("alignment", "right"),
        ],
    ),
    // D-473: every setting.
    (
        "core.basic_text",
        &[
            ("text", "Hello"),
            ("position", "10,20"),
            ("display", "stroke_over_fill"),
            ("fill_color", "#ff0000"),
            ("stroke_color", "#00ff00"),
            ("stroke_width", "5"),
            ("size", "48"),
            ("tracking", "20"),
            ("composite_on_original", "off"),
            ("font", "MPLUSRounded1c-Regular.ttf"),
            ("alignment", "right"),
        ],
    ),
    // D-525: the channel, the id and the output.
    ("core.identifier", &[("aux_channel", "material_id"), ("id", "3"), ("output", "luma_matte")]),
    // D-522: the two texts and the word.
    ("core.cryptomatte", &[("crypto_layer", "CryptoMaterial"), ("selection", "steel, rubber"), ("output", "keyed")]),
    ("core.depth_of_field", &[("focal_plane", "6"), ("maximum_radius", "12"), ("focal_plane_thickness", "2"), ("focal_bias", "80")]),
    // D-524: the colour, the numbers, the word and a gradient.
    (
        "core.fog_3d",
        &[
            ("fog_color", "%234060a0"),
            ("fog_start_depth", "2"),
            ("fog_end_depth", "12"),
            ("fog_opacity", "80"),
            ("scattering_density", "30"),
            ("foggy_background", "off"),
            ("map_layer", "layer-4"),
            ("layer_contribution", "60"),
        ],
    ),
    (
        "core.text_animator",
        &[
            ("position", "0, -60"),
            ("scale", "150, 120"),
            ("rotation", "15"),
            ("opacity", "40"),
            ("fill", "on"),
            ("color", "0, 1, 0.5"),
            ("tracking", "50"),
            ("start", "20"),
            ("end", "80"),
            ("offset", "10"),
            ("amount", "75"),
            ("based_on", "words"),
            ("shape", "triangle"),
            ("smoothness", "50"),
            ("ease_high", "25"),
            ("ease_low", "75"),
        ],
    ),
    // D-351.
    (
        "core.stretch_levels",
        &[("temporal_smoothing", "0.5"), ("scene_detect", "on"), ("black_clip", "2"), ("white_clip", "3")],
    ),
    (
        "core.stretch_contrast",
        &[("temporal_smoothing", "1"), ("scene_detect", "off"), ("black_clip", "0.5"), ("white_clip", "0")],
    ),
    (
        "core.stretch_color",
        &[
            ("temporal_smoothing", "0.25"),
            ("scene_detect", "on"),
            ("black_clip", "1"),
            ("white_clip", "1.5"),
            ("snap_neutral_midtones", "on"),
        ],
    ),
    ("core.spread_tones", &[("equalize", "photoshop"), ("amount", "60")]),
    // D-352.
    (
        "core.matte_choker",
        &[
            ("geometric_softness_1", "3"),
            ("choke_1", "-20"),
            ("gray_level_softness_1", "40"),
            ("geometric_softness_2", "1.5"),
            ("choke_2", "30"),
            ("gray_level_softness_2", "60"),
            ("iterations", "2"),
        ],
    ),
    (
        "core.refine_hard_matte",
        &[
            ("feather", "4"),
            ("contrast", "20"),
            ("shift_edge", "-10"),
            ("decontaminate", "on"),
            ("decontamination_amount", "80"),
            ("decontamination_radius", "2"),
            ("view_decontamination_map", "off"),
        ],
    ),
    (
        "core.refine_soft_matte",
        &[
            ("edge_radius", "6"),
            ("view_edge_region", "on"),
            ("feather", "1"),
            ("contrast", "10"),
            ("shift_edge", "5"),
            ("decontaminate", "off"),
            ("decontamination_amount", "50"),
            ("decontamination_radius", "3"),
            ("view_decontamination_map", "on"),
        ],
    ),
    (
        "core.stroke",
        &[
            ("mask", "2"),
            ("all_masks", "on"),
            ("stroke_sequentially", "on"),
            ("color", "%23ff8800"),
            ("brush_size", "6"),
            ("brush_hardness", "40"),
            ("opacity", "80"),
            ("start", "10"),
            ("end", "90"),
            ("spacing", "25"),
            ("paint_style", "reveal"),
            ("source", "shapes"),
        ],
    ),
    // D-204: the colour and the numbers.
    (
        "core.snowfall",
        &[
            ("color", "%23e0f0ff"),
            ("density", "60"),
            ("spacing", "20"),
            ("size", "4"),
            ("depth", "80"),
            ("speed", "3"),
            ("wind", "-1"),
            ("wiggle", "5"),
            ("period", "30"),
            ("seed", "2"),
            ("opacity", "90"),
        ],
    ),
    // D-642..D-644: the sound effects, whose settings the panel reads off the same table.
    (
        "core.noise_gate",
        &[("threshold", "-40"), ("attack", "10"), ("hold", "50"), ("release", "100")],
    ),
    (
        "core.distortion",
        &[
            ("drive", "50"),
            ("gain", "25"),
            ("wet", "100"),
            ("volume", "11"),
            ("resolution", "24"),
            ("downsample", "1"),
            ("type", "tube"),
        ],
    ),
    (
        "core.compressor",
        &[
            ("threshold", "-20"),
            ("ratio", "3"),
            ("knee", "6"),
            ("attack", "10"),
            ("release", "100"),
            ("makeup", "0"),
            ("output_limit", "-1"),
            ("auto_release", "off"),
        ],
    ),
];

const FIELDS_INTRO: &[&str] = &[
    "The panels draw themselves out of one JSON answer, `/state`, whose project half is \
     the same text a save writes. Nothing has ever checked that the names the panels read are \
     the names the window sends. A field renamed on one side is a panel that draws a blank \
     space: no error, no status line, nothing in the log, and a person who assumes the layer \
     simply has no name.",
    "This is that check, and the names are read out of `app/ui/index.html` itself rather than \
     written down here. Every `doc.`, `comp.`, `layer.`, `asset.`, `span.` and `fx.` the page \
     reads is looked for in a real answer from a real project, with an effect stack and a \
     matte added by command first, because the reference shot has neither.",
];

const FIELDS_NOTES: &[&str] = &[
    "## What to look at\n\n- **`missing`** in the Actual column is the failure this table \
     exists to catch, and it names the field. Every other row is a name the panels read and \
     the window sends.\n- **The transform, matte and mask rows are one level further in.** \
     The page reaches those through a local name rather than through `layer`, so the scan \
     cannot see them and they are asked for by name.\n- **The rows from the first `the panel's` \
     on are the other direction**: not what the panel reads, but what it sends. The effect panel builds a \
     request out of the parameter names in its own table, so a setting renamed in the core \
     and not in the page is a field a person can type into that changes nothing.",
    "## What this cannot cover\n\nThat the panel draws the value correctly once it has it. \
     This says the name resolves, not that the number is put in the right box, formatted the \
     right way, or updated when it changes. `verification/B-12a_window_and_keyboard.md` and \
     the photographs are what say that, and the owner's run under B-12 is what says it for \
     real.\n\nIt also cannot see a field that is present and always null. `mask` is null on \
     every layer of the reference shot, which is why the mask row opens a project that \
     carries one.",
];

// ---- the routes the shell owns ---------------------------------------------------------

#[test]
fn the_windows_own_routes_are_not_swallowed_on_the_way_past() {
    let mut report = Report { rows: Vec::new() };
    let source = source();
    let viewer = Mutex::new(demo());

    // The defect this test exists for. `fn command` offers every request to `edit_command`
    // first and reads `None` as permission to try its own routes, so an `edit_command` that
    // answers something it has never heard of takes Open, Save, Save As, Export, the recent
    // list and recovery with it. That is what happened, it reached a photograph, and every
    // test that saves calls `save` directly - only the window goes through this path.
    for route in ROUTES {
        if *route == "frame" || *route == "at" {
            continue;
        }
        report.check(
            &format!("`/{route}` is left alone by the command layer"),
            "not mine - the shell answers it",
            match edit_command(&viewer, route, None) {
                None => "not mine - the shell answers it".to_string(),
                Some(said) => format!("swallowed, and answered: {said}"),
            },
        );
    }

    // A word that is neither. Nothing may claim it, or the shell's 404 - the only thing that
    // tells a page it asked for something that does not exist - never happens.
    for nonsense in ["layer.remove", "save-it", "", "state/../save"] {
        report.check(
            &format!("`/{nonsense}`, which is nothing this window has, is refused"),
            "not mine",
            match edit_command(&viewer, nonsense, None) {
                None => "not mine".to_string(),
                Some(said) => format!("answered: {said}"),
            },
        );
    }

    // The sentence a 404 carries is the only list of routes a person or a page ever sees, and
    // it is written by hand. This is the one check that it still names what the shell has.
    let listed: Vec<String> = {
        let at = source
            .find("ask for /state,")
            .expect("the not-found answer is in this file");
        let rest = &source[at..];
        sorted(after(
            &rest[..rest.find("command IDs").unwrap_or(rest.len())],
            "/",
        ))
    };
    let shell: Vec<String> = ROUTES
        .iter()
        .filter(|route| {
            !matches!(
                **route,
                "frame"
                    | "at"
                    | "play"
                    | "boxes"
                    | "curve"
                    | "sheet"
                    | "sound"
                    | "mix"
                    | "place"
                    | "ready"
                    | "stop"
            )
        })
        .map(|route| route.to_string())
        .collect();
    report.check(
        "the answer to a route that does not exist names the routes that do",
        shell.join(", "),
        listed.join(", "),
    );

    // Query strings, which every command in this window arrives with. The page builds them
    // with `encodeURIComponent`, so what comes back has to be what was typed.
    let cases: &[(&str, &str, &str)] = &[
        ("layer=layer%201", "layer", "layer 1"),
        ("name=a%26b", "name", "a&b"),
        ("name=%E7%8C%AB", "name", "猫"),
        ("layers=1&layer=2", "layer", "2"),
        // A name that ends with the name being asked for. `matte_layer` is a real one, and
        // a search that looks for `layer=` anywhere in a pair rather than at its start
        // answers this with the wrong sequence's identifier.
        ("matte_layer=layer-9&layer=layer-2", "layer", "layer-2"),
        ("start=12&drawing=", "drawing", ""),
    ];
    for (query, name, expected) in cases {
        report.check(
            &format!("`?{query}` gives `{name}`"),
            *expected,
            parameter(Some(query), name).unwrap_or_else(|| "(nothing)".to_string()),
        );
    }
    report.check(
        "and a name that is not in the query gives nothing at all",
        "(nothing)",
        parameter(Some("layer=layer-1"), "name").unwrap_or_else(|| "(nothing)".to_string()),
    );

    // The one query string in this file that decides what reaches the disk.
    for (query, expected) in [
        (Some("missing=write"), "RenderTransparent"),
        (Some("missing=Write"), "Block"),
        (Some("missing=1"), "Block"),
        (Some("missing="), "Block"),
        (None, "Block"),
    ] {
        report.check(
            &format!(
                "what an export asked for with `{}` does with a missing drawing",
                query.unwrap_or("no query at all")
            ),
            expected,
            format!("{:?}", missing_source(query)),
        );
    }

    write_artifact(
        &report,
        "verification/B-12b_routes_table.md",
        "B-12b: the routes the window answers itself",
        ROUTES_INTRO,
        ROUTES_NOTES,
    );
    let failed: Vec<&String> = report
        .rows
        .iter()
        .filter(|(_, e, a)| e != a)
        .map(|(c, _, _)| c)
        .collect();
    assert!(failed.is_empty(), "these checks failed: {failed:#?}");
}

const ROUTES_INTRO: &[&str] = &[
    "Every request from the page arrives at one function, which offers it to the command \
     layer first and answers it itself if the command layer has never heard of it. That \
     hand-off is the whole of the window's routing, and on 2026-09-07 it was broken: the \
     command layer answered everything, including identifiers it did not have, so Open, \
     Save, Save As, Export, the recent list and recovery were all swallowed on the way past. \
     The editing window worked perfectly and the project could not be written.",
    "No test saw it, because every test that saves calls the save function directly and only \
     the window goes through the hand-off. This table is that path: for each route the shell \
     owns, that the command layer leaves it alone; that a word neither of them has is refused \
     by both; that the sentence a person gets when they ask for something that does not exist \
     still names what does; and that a query string is read back as what was typed into it.",
];

const ROUTES_NOTES: &[&str] = &[
    "## What to look at\n\n- **`swallowed, and answered:`** in the Actual column is the \
     defect this table was written for, and the sentence beside it is what the person would \
     have got instead of their project being saved.\n- **The empty route** is in the list on \
     purpose. A page can ask for `/` and something has to refuse it.\n- **`missing=Write` \
     blocks.** The override that writes frames with drawings missing is exactly the word \
     `write`; everything else takes document 07's default and refuses the job. A comparison \
     that ignored case would let a typo become an export nobody asked for.",
    "## What this cannot cover\n\nThe three routes that open a dialog. Import, Save As and \
     Export hand the request to Windows before anything of ours runs, and no test in this \
     project has hands to answer a file dialog. What is checked here is that the request \
     reaches the shell at all; what happens after the dialog is \
     `verification/B-09_save_table.md` and `verification/B-10_export_table.md`.\n\nAnd the \
     function itself. It needs a running application to be called, so what is checked is the \
     decision it makes rather than the response it builds: the hand-off, the list, the query \
     reading and the export override are each called directly.",
];

// ---- document 05 line 69, the keyboard claim ----------------------------------------------

/// The name a `$('id')` in the page reaches for, where that lookup is followed by something
/// that makes it a control: a handler, a synthetic click, or a checkbox being read.
fn controls(page: &str) -> Vec<String> {
    page.match_indices("$('")
        .filter_map(|(at, _)| {
            let rest = &page[at + 3..];
            let end = rest.find('\'')?;
            let after = &rest[end + 2..];
            let is_a_control = after.starts_with(".on")
                || after.starts_with(".click")
                || after.starts_with(".checked");
            is_a_control.then(|| rest[..end].to_string())
        })
        .collect()
}

/// The tag the markup gives an element, read backwards from its identifier.
fn tag_of(page: &str, id: &str) -> String {
    let needle = format!("id=\"{id}\"");
    match page.find(&needle) {
        None => "not in the markup".to_string(),
        Some(at) => {
            let before = &page[..at];
            let open = before.rfind('<').map(|o| &before[o + 1..]).unwrap_or("");
            open.split(|c: char| c.is_whitespace() || c == '>')
                .next()
                .unwrap_or("")
                .to_string()
        }
    }
}

/// The page with every mouse-only handler cut out of it.
///
/// A double click and a pointer drag are the two gestures a keyboard cannot make. Whatever
/// a handler for one of them sends is unreachable without a mouse unless the same request is
/// sent from somewhere else as well, so the question "can this be done without a mouse" is
/// the question "does this identifier still appear once those handlers are gone".
///
/// A handler is cut from its name to the end of its statement, which is the first semicolon
/// at depth zero - handlers here are written both as a braced body and as a one-line arrow.
fn without_the_mouse(page: &str) -> String {
    const MOUSE: [&str; 4] = [
        "ondblclick",
        "onpointerdown",
        "onpointermove",
        "onpointerup",
    ];
    let mut out = String::with_capacity(page.len());
    let bytes: Vec<char> = page.chars().collect();
    let mut i = 0;
    while i < bytes.len() {
        let rest: String = bytes[i..].iter().take(20).collect();
        match MOUSE.iter().find(|m| rest.starts_with(**m)) {
            None => {
                out.push(bytes[i]);
                i += 1;
            }
            Some(_) => {
                let mut depth = 0i32;
                while i < bytes.len() {
                    match bytes[i] {
                        '(' | '{' | '[' => depth += 1,
                        ')' | '}' | ']' => depth -= 1,
                        ';' if depth <= 0 => {
                            i += 1;
                            break;
                        }
                        _ => {}
                    }
                    i += 1;
                }
            }
        }
    }
    out
}

/// The keys the window answers when nothing in particular is focused, as the page writes
/// them: every `e.key === '...'` in the one global handler.
fn key_names(page: &str) -> Vec<String> {
    let block = accelerators(page);
    let block = block.as_str();
    let mut keys: Vec<String> = block
        .match_indices("e.key === '")
        .filter_map(|(at, _)| {
            let rest = &block[at + 11..];
            let end = rest.find('\'')?;
            Some(rest[..end].to_string())
        })
        // A letter is written in the page in both cases and is one key; anything longer
        // is already the name of a key and is left as it is.
        .map(|key| match key.as_str() {
            " " => "Space".to_string(),
            one if one.chars().count() == 1 => one.to_uppercase(),
            other => other.to_string(),
        })
        .collect();
    keys.sort();
    keys.dedup();
    keys
}

/// Document 05 line 69: "complete W-01 without assistance, with all required controls
/// reachable by keyboard."
///
/// Every other artifact in B-12 checks what a request does. This one checks that a person
/// with no mouse can make the request at all, by reading the page the way the claim is
/// worded: which controls exist, whether each is a thing the Tab key stops at, which keys
/// the window answers on its own, and - the row this table is really for - whether every
/// command the page can send survives having the mouse-only handlers cut out of it.
///
/// Writes `verification/B-12c_keyboard_table.md`.
#[test]
fn every_command_the_page_sends_can_be_asked_for_without_a_mouse() {
    let page = page();
    let mut report = Report { rows: Vec::new() };

    let mut named = controls(&page);
    named.sort();
    named.dedup();
    report.check(
        "the controls the page wires are the ones written down here",
        CONTROLS.join(", "),
        named.join(", "),
    );
    // A button, a select and a checkbox are stops on the Tab order because the browser makes
    // them so. A span or a div with a click handler is not, and that is the shape this row
    // exists to catch, unless its markup puts it in the Tab order by hand, as D-84b's Sheet
    // does so that its cells can be written from the keyboard.
    let not_focusable: Vec<String> = named
        .iter()
        .map(|id| (id, tag_of(&page, id)))
        .filter(|(_, tag)| !matches!(tag.as_str(), "button" | "select" | "input"))
        .filter(|(id, _)| !page.contains(&format!("id=\"{id}\" tabindex=\"0\"")))
        .map(|(id, tag)| format!("{id} is a {tag}"))
        .collect();
    report.check(
        "and every one of them is a control the Tab key stops at on its own",
        "none of them is anything else",
        match not_focusable.is_empty() {
            true => "none of them is anything else".to_string(),
            false => not_focusable.join(", "),
        },
    );

    // The two lists are built out of `li`, which nothing focuses by itself.
    for (what, marker) in [
        (
            "a row in the media bin or the layer list",
            "  li.tabIndex = 0;",
        ),
        ("the number a drag changes", "  handle.tabIndex = 0;"),
        (
            "the Sheet, whose cells take typing (D-84b)",
            "<div id=\"xsheet\" tabindex=\"0\">",
        ),
    ] {
        report.check(
            &format!("{what} is put into the Tab order by hand"),
            true,
            page.contains(marker),
        );
    }
    report.check(
        "and a focused row is chosen with Enter or Space, which is what a click does",
        true,
        page.contains(
            "li.dispatchEvent(new MouseEvent('click', { bubbles: true, shiftKey: e.shiftKey }));",
        ),
    );
    report.check(
        "and a focused drag handle is moved with the arrow keys, which is what a drag does",
        true,
        page.contains("const by = { ArrowLeft: -1, ArrowRight: 1, ArrowDown: -1, ArrowUp: 1 }"),
    );
    // W-39: a menu line or a top bar button written as data-run runs the command palette's
    // line of that name, so a name the palette does not have would be a dead button.
    let lost: Vec<&str> = page
        .split("data-run=\"")
        .skip(1)
        .filter_map(|rest| rest.split('"').next())
        .filter(|name| !page.contains(&format!("['{name}', ")))
        .collect();
    report.check(
        "every menu line and top bar button the page runs by name is a line of the command palette",
        "all of them are",
        match lost.is_empty() {
            true => "all of them are".to_string(),
            false => lost.join(", "),
        },
    );

    report.check(
        "the keys the window answers with nothing focused are the ones written down here",
        KEYS.join(", "),
        key_names(&page).join(", "),
    );

    // The row this table is for. Everything the page can ask the window to do, asked of a
    // copy of the page with the double clicks and the pointer drags cut out.
    let without = asked_for(&without_the_mouse(&page));
    for id in asked_for(&page) {
        if matches!(id.as_str(), "frame" | "at" | "state") {
            continue; // Not a control: the page asks for these itself, to draw with.
        }
        let reachable = without.contains(&id);
        report.check(
            &format!("`{id}` can be asked for without a mouse"),
            match MOUSE_ONLY.contains(&id.as_str()) {
                true => "no - it is a drag, and a drag is a mouse",
                false => "yes",
            },
            match reachable {
                true => "yes",
                false => "no - it is a drag, and a drag is a mouse",
            },
        );
    }

    // Every gesture that needs a mouse, each with the thing that does the same job without one.
    for (gesture, does, instead) in MOUSE_GESTURES {
        report.check(
            &format!("{gesture} is not the only way to {does}"),
            true,
            page.contains(instead),
        );
    }

    write_artifact(
        &report,
        "verification/B-12c_keyboard_table.md",
        "B-12c: the keyboard claim, and what is mouse-only",
        KEYBOARD_INTRO,
        KEYBOARD_NOTES,
    );
    assert!(
        report.rows.iter().all(|(_, e, a)| e == a),
        "the keyboard claim does not hold: {:?}",
        report
            .rows
            .iter()
            .filter(|(_, e, a)| e != a)
            .collect::<Vec<_>>()
    );
}

/// Every control the page wires a handler to, or clicks for the person, or reads.
const CONTROLS: [&str; 134] = [
    "addadjust",
    "addeffect",
    "addexposure",
    "addlayer",
    "addnote",
    "addnull",
    "addshape",
    "addsolid",
    "addtoqueue",
    "alpha",
    "anyway",
    "applyrelink",
    "back",
    "cancelcomp",
    "cancelexport",
    "cancelprecomp",
    "cancelprint",
    "cancelrelink",
    "cancelsolid",
    "checker",
    "checkpackage",
    "closecomp",
    "closemap",
    "closeprefs",
    "collect",
    "compadvtab",
    "comparebtn",
    "compbasictab",
    "compbgon",
    "compfps",
    "compframes",
    "compguides",
    "compheight",
    "complock",
    "comppreset",
    "compwidth",
    "delcomppreset",
    "dellayer",
    "down",
    "emptyimport",
    "emptynew",
    "export",
    "exportformat",
    "fbswitch",
    "filmquality",
    "findcommand",
    "findlayer",
    "fit",
    "fit100",
    "fwd",
    "fxsearch",
    "gifdither",
    "gpu",
    "graphall",
    "graphfit",
    "graphmode",
    "gridbtn",
    "guidebtn",
    "halfq",
    "health",
    "import",
    "importcut",
    "keys",
    "makecomp",
    "makeprecomp",
    "makesolid",
    "maptab",
    "mbswitch",
    "mirror",
    "newcomp",
    "newlayer",
    "notedetails",
    "onionbtn",
    "open",
    "play",
    "precompmap",
    "precompname",
    "precompopen",
    "prefcard",
    "prefdisk",
    "prefdiskfolder",
    "preferences",
    "prefmemory",
    "prefram",
    "printnow",
    "printsheet",
    "profileragain",
    "profilerbtn",
    "profilerclose",
    "recent",
    "recovery",
    "redo",
    "refine",
    "regionbtn",
    "relink",
    "renderadd",
    "renderpause",
    "renderstart",
    "renderstop",
    "resetworkspace",
    "save",
    "saveas",
    "savecomppreset",
    "savepreset",
    "saveworkspace",
    "sessionlogbutton",
    "sessionlogclear",
    "sessionlogclose",
    "sessionlogon",
    "sessionlogsave",
    "sheetclose",
    "sheetcolumn",
    "shyswitch",
    "skover",
    "slatebtn",
    "snapshot",
    "solidcompsize",
    "soundwav",
    "stripbtn",
    "tabboth",
    "tabgraph",
    "tabsheet",
    "timescroll",
    "timezoom",
    "toggle",
    "turnleft",
    "turnright",
    "undo",
    "unturn",
    "up",
    "workspace",
    "workspacename",
    "xsheet",
    "zoomer",
];

/// Document 24's shortcuts, as keys rather than as chords: the modifiers live in the same
/// branch as the key and `verification/B-12b_command_map_table.md` is what checks the pair.
const KEYS: [&str; 51] = [
    "*",
    ",",
    "-",
    ".",
    "1",
    "2",
    "3",
    "4",
    "=",
    "?",
    "A",
    "ArrowDown",
    "ArrowLeft",
    "ArrowRight",
    "ArrowUp",
    "B",
    "C",
    "D",
    "Delete",
    "E",
    "End",
    "Enter",
    "Escape",
    "F",
    "F2",
    "F3",
    "F9",
    "G",
    "H",
    "Home",
    "I",
    "J",
    "K",
    "L",
    "M",
    "N",
    "O",
    "P",
    "PageDown",
    "PageUp",
    "Q",
    "R",
    "S",
    "Space",
    "T",
    "U",
    "V",
    "Y",
    "Z",
    "[",
    "]",
];

/// The identifiers that are a drag and cannot be anything else.
///
/// It was the three `property.drag_*` identifiers until W-04. A nudge with the arrow keys
/// now moves every selected layer, and one press moving three layers has to be one thing
/// to undo, so the nudge opens and commits the same transaction a drag does:
/// `property.drag_update` once per layer and then `property.drag_end`. The cancel was the
/// last of them, and W-07 took it too: an effect's setting is sent as it is typed inside
/// that same transaction, and Escape in the field abandons it.
///
/// `effect.move` is what is left, since W-07. It is the drop at the end of a card dragged
/// up or down the stack, and the two arrows on the card send `effect.move_up` and
/// `effect.move_down` to the same end a step at a time, which is the pairing the rows
/// below are for.
///
/// `layer.move` joined it in W-22, the same drop for a layer's row, and left again in W-23
/// when Ctrl+Shift+] and Ctrl+Shift+[ sent it to take a layer to the very front or back.
// B-27c: the value whip is a drag. What it writes can be typed into the expression box.
// D-261: a stroke is drawn with a mouse or a pen; there is nothing to draw it with otherwise.
const MOUSE_ONLY: [&str; 3] = ["effect.move", "property.link", "sketch.add_stroke"];

/// A mouse gesture, what it does, and the text in the page that does the same job without one.
const MOUSE_GESTURES: [(&str, &str, &str); 36] = [
    (
        "dragging the border between two panels",
        "give one of them more of the window",
        "split.onkeydown = (e) => {",
    ),
    (
        "double clicking a drawing sequence in the media bin",
        "make a layer out of it",
        "$('addlayer').click();",
    ),
    (
        "double clicking a layer",
        "rename it",
        "else if (e.key === 'F2') { e.preventDefault(); beginRename(); }",
    ),
    (
        "dragging a transform value",
        "change it",
        "held = fit(round(held + by * step * (whole && !e.shiftKey ? 1 : pace(e))));",
    ),
    (
        "dragging a layer on the picture",
        "move it",
        "const by = { ArrowLeft: [-1, 0], ArrowRight: [1, 0],",
    ),
    (
        "pulling a corner or the rotation arm on the picture",
        "scale or turn the layer",
        "held = fit(round(held + by * step * (whole && !e.shiftKey ? 1 : pace(e))));",
    ),
    (
        "dragging along the ruler",
        "go to a frame",
        "else if (e.key === 'ArrowRight') step(e.shiftKey ? 10 : 1);",
    ),
    (
        "dragging a layer's bar along the exposure sheet",
        "move the layer in time",
        "e.key === '[' || e.key === ']'",
    ),
    (
        "pulling either end of a layer's bar",
        "trim it",
        "e.key === '[' || e.key === ']'",
    ),
    (
        "dragging the seam between two exposure blocks",
        "retime the exposures on either side of it",
        "commit: sendSpan,",
    ),
    (
        "dragging a box across the picture",
        "select several layers",
        "shiftKey: e.shiftKey",
    ),
    (
        "dragging the anchor mark",
        "move the anchor",
        // D-280: typed in the timeline's Transform group, since the effect controls lost it.
        "group('transform', 'Transform', () => tracks(propsOf(layer).filter((prop) => PROPS.includes(prop)",
    ),
    (
        "turning the wheel over the picture",
        "zoom",
        "e.key === '1'",
    ),
    (
        "dragging the handle beside an effect's setting",
        "change the setting",
        "held = fit(round(held + by * step * (whole && !e.shiftKey ? 1 : pace(e))));",
    ),
    (
        "moving over the tint's colour picker",
        "choose the colour",
        "fields[name].push(input);",
    ),
    (
        "dragging an effect's bar up or down the stack",
        "reorder the effects",
        "command('/effect.move_up' + where)",
    ),
    (
        "dragging a layer up or down the list",
        "reorder the layers",
        "$('up').onclick = () => moveLayers(true);",
    ),
    (
        "dragging a drawing from the media bin onto the layer list",
        "make a layer out of it at that place",
        "$('addlayer').click();",
    ),
    (
        "right clicking a layer",
        "open its menu",
        "else if (e.key === 'F2') { e.preventDefault(); beginRename(); }",
    ),
    (
        "dragging a layer on the picture with Shift held",
        "move it along one line",
        "const by = { ArrowLeft: [-1, 0], ArrowRight: [1, 0],",
    ),
    (
        "dragging the picture with the middle button",
        "look at another part of a zoomed picture",
        "e.key === '?'",
    ),
    (
        "Alt-clicking a property's diamond",
        "add an expression to the property or remove it",
        "e.altKey && e.shiftKey && (e.key === '=' || e.key === '+')",
    ),
    (
        "dragging either end of the work area on the ruler",
        "set the work area",
        "setWork('start')",
    ),
    (
        "double clicking the work area",
        "put it back to the whole composition",
        "['Reset the work area', '', () => setWork('reset')]",
    ),
    (
        "dragging a marker along the ruler, or Ctrl-clicking it",
        "move or remove it",
        "['Remove the marker at the playhead', '', () => removeMarkerAt(frame)]",
    ),
    (
        "double clicking a marker",
        "name it",
        "['Name the marker at the playhead', '', () => nameMarker(frame)]",
    ),
    (
        "right clicking a property's name",
        "put it back to its default",
        "['Reset ' + prop, '', () => resetProp(prop)]",
    ),
    (
        "right clicking a chosen key",
        "open its menu of key speed, ease, hold, copy, paste and delete",
        "['Key speed and influence', '', velocityDialog]",
    ),
    (
        "right clicking an empty place on the timeline",
        "paste keys or layers there, add a key, a layer or a marker, or open the settings",
        "['Add a marker', '*', { key: '*' }]",
    ),
    // B-24c: the two mask gestures. A mask is drawn with a hand, so the menu carries the
    // one shape a person without a mouse cannot draw - the whole layer - and a point that
    // has been tabbed to moves under the arrow keys, ten pixels with Shift.
    (
        "drawing a mask with the pen, the rectangle or the ellipse",
        "put a mask on the layer",
        "['New mask', 'G for the pen', newMask,",
    ),
    (
        "dragging a mask point or one of its handles",
        "move it",
        "'Point ' + (i + 1) + ' of ' + mask.name",
    ),
    // B-24h: and the third mask gesture. Plus and minus on a point that has been tabbed to
    // add a point after it or take it off, which is what the pen does with one.
    (
        "the pen over a mask's path, or over one of its points",
        "add a point there, or take that point off",
        "command('/path.add_point' + which + '&at=' + i + '&t=0.5');",
    ),
    // B-24i, D-80: choosing several points at once. A box drawn round four of them, or Shift
    // and a click on each, is a hand; the keyboard chooses the point it has tabbed to, and
    // every key below acts on the chosen points and on that one alike, so the same work is
    // done a point at a time.
    (
        "dragging a box across a mask's points, or Shift-clicking them one by one",
        "choose several of them and move or delete them together",
        "const moving = [...new Set([...chosenPoints(shaper, at), i])].sort((a, b) => a - b);",
    ),
    // B-24j, D-80: curving the path. Pulling a handle is a drag and Alt and a drag out of a
    // corner is a drag, so the brackets on a point that has been tabbed to lengthen and
    // shorten its pair of handles along the line between its neighbours, which is the
    // direction a smooth point's handles take anyway.
    (
        "pulling a mask point's handle, or dragging out of a point with Alt held",
        "curve the path through that point, or straighten it again",
        "if (k.key === ']' || k.key === '[') {",
    ),
    // B-27c, D-83: the pick whips. A link is an expression, and the box takes it typed; a
    // parent is the Parent list's.
    (
        "dragging a property's pick whip onto another property",
        "link one to the other",
        "text.title = 'The ' + named + ' expression. Click away or press Ctrl+Enter to apply it, '",
    ),
    (
        "dragging a layer's pick whip onto another layer",
        "make that layer its parent",
        "chooser.setAttribute('aria-label', 'Parent for ' + layer.name);",
    ),
];

const KEYBOARD_INTRO: &[&str] = &[
    "Document 05 line 69 sets B-12's bar: \"complete W-01 without assistance, with all \
     required controls reachable by keyboard.\" `verification/B-11_display_and_keyboard.md` \
     is a photograph of the Tab key reaching the Open button, which says the order exists \
     and that the control holding it is visible. This is the other half, and it is the half \
     a photograph cannot take: whether there is any command in this window that a person \
     without a mouse cannot ask for.",
    "It reads `app/ui/index.html`. The question it asks is not \"is there a keyboard \
     shortcut\" but \"is this request sent from anywhere that is not a mouse gesture\", and \
     it asks it by cutting every double-click and pointer-drag handler out of a copy of the \
     page and looking for the request in what is left.",
];

const KEYBOARD_NOTES: &[&str] = &[
    "## What to look at\n\nThe one row that says **no**. `effect.move` is the drop of a \
     card dragged up or down the effect stack, and the arrows on the card do the same job a \
     step at a time. It used to be `property.drag_cancel`, Escape during a drag, and before \
     W-04 three: `property.drag_update` and `property.drag_end` are the running transaction \
     and the coalescing document 26 asks for, and until W-04 only a pointer opened one. \
     Nudging the picture with the arrow keys now moves every selected layer, and one press \
     moving three layers has to be one thing to undo, so it opens and commits that same \
     transaction; and since W-07 an effect's setting is sent as it is typed inside that same \
     transaction, so Escape in the field abandons it, which is the cancel without a pointer. \
     The rest of the keyboard changes numbers rather than dragging them, and the last sixteen rows \
     are the mouse gestures in this \r
         window each paired with the thing that does the same job without one: the arrow keys on a \r
         focused handle send `property.set_base`, which is one undo step per press rather than one per \r
         drag. The picture itself works the same way: a layer is dragged, and it is nudged by the \r
         arrow keys while the canvas holds the focus; a corner handle scales and the rotation arm \r
         turns, and both numbers are typed or stepped in the inspector; the playhead is dragged along \r
         the exposure sheet, and it is stepped by the arrow keys everywhere the canvas does not hold \r
         the focus. Since W-06 a box dragged across the picture selects the layers inside it, and \
     Shift with Space or Enter on a row in the layer list adds that row the way Shift with a \
     click does; the anchor mark is dragged, and its two numbers are in the inspector like \
     the rest; and the wheel zooms, where Ctrl+1 and Shift+/ set the two zooms document 24 \
     names. Since W-07 every value box in the effects panel has the handle the transform rows \
     have, and its arrow keys step the value the same way; the tint's colour picker sets the \
     three numbers beside it, which are typed like any other; and an effect card is dragged \
     up or down the stack, where the arrows on it move it a step at a time. Since W-12 the \
     After Effects property keys A, P, S, R, T and U open a selected layer's properties under \
     it on the timeline, Shift with a key adds one, and the arrow beside the layer's name is \
     a button that opens all five; alpha-only inspection moved from A to Alt+A to make room.          Since W-13 the borders between the panels are dragged, and each border is a stop on the          Tab order that the arrow keys move and Home puts back.",
    "The two lists are the part that had to be built rather than inherited. Every button \
     here is a `button` and every chooser a `select`, so the Tab order is the browser's and \
     nothing had to be arranged; the rows of the media bin and the layer list are `li` \
     elements, which nothing focuses, and they carry a tab stop and an Enter/Space handler \
     put there by hand. The row that checks the list of controls is a list rather than a \
     count for the same reason as the one in `verification/B-12b_page_table.md`: a control \
     added tomorrow fails this table until somebody writes it down beside the others, and \
     the row underneath then asks what kind of element it is.",
    "## What this cannot cover\n\nThat the Tab order is a sensible one. It says every \
     control is in it; it does not say the order walks the window in the order a person \
     reads it, and nothing but a person pressing Tab can say that. The photograph in \
     `verification/B-11_display_and_keyboard.md` counts six presses to Open, which is one \
     sample of it.",
    "Whether a focused control can be seen. The focus ring is a colour in a stylesheet, and \
     this reads the page as text. That is the photograph's job as well.",
    "And the file dialogs. Import, Open, Save As and Export hand over to Windows, which \
     brings its own keyboard handling and is not this project's to check.",
];

fn cell(text: &str) -> String {
    text.replace('|', r"\|")
}

/// A setting added after projects were already saved is read as its as-added value when a
/// project leaves it out, and the panel must show and send that same value. A panel row with
/// no start value shows 0 and sends 0: Curl Noise's Scale (1 to 10000) then refused every
/// edit to every other Curl Noise setting in an older project, because the panel sends all of
/// them at once.
///
/// Reads the window's `if parameter(query, "x").is_some() { number("x")? } else { N }` form and
/// the project reader's `effect_number_or(params, "x", at, N)`; the `num("x", start.x)` form
/// takes its value from a struct and is not read here. The settings tables in
/// src/effects.rs are not read either; the 2026-10-15 run of every effect through the real
/// panel found Color Grade's and Particle Playground's starts there.
#[test]
fn a_setting_an_older_project_leaves_out_shows_its_as_added_value() {
    let page = page();
    let effects = std::fs::read_to_string(repo("src/effects.rs")).expect("read src/effects.rs");
    let persist = std::fs::read_to_string(repo("src/persist.rs")).expect("read src/persist.rs");
    let (mut checked, mut wrong) = (0, Vec::new());
    // Where the setting's name starts after `opening`, the text after its as-added value, and
    // the text before its effect's constant.
    let window = source();
    let reads = [
        (
            window.as_str(),
            "if parameter(query, \"",
            "\").is_some() { number(\"{}\")? } else { ",
            " => Ok(Effect::",
        ),
        (
            persist.as_str(),
            "effect_number_or(params, \"",
            "\", at, ",
            " => Some(crate::effects::Effect::",
        ),
    ];
    for (source, opening, between, arm_mark) in reads {
        for (at, _) in source.match_indices(opening) {
            let name = source[at + opening.len()..]
                .split('"')
                .next()
                .unwrap_or_default();
            let shape = format!("{opening}{name}{}", between.replace("{}", name));
            let Some(tail) = source[at..].strip_prefix(shape.as_str()) else {
                continue;
            };
            let value: String = tail
                .chars()
                .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                .collect();
            let Ok(value) = value.parse::<f64>() else {
                continue;
            };
            // The effect is the nearest `NAME => Ok(Effect::` above, and NAME its identifier's constant.
            let Some(arm_end) = source[..at].rfind(arm_mark) else {
                continue;
            };
            let arm = source[..arm_end]
                .rsplit(char::is_whitespace)
                .next()
                .unwrap_or_default();
            let arm = arm.trim_start_matches("crate::effects::");
            let Some(type_id) = effects.split(&format!("pub const {arm}: &str = \"")).nth(1) else {
                continue;
            };
            let type_id = type_id.split('"').next().unwrap_or_default();
            let Some(block) = page.split(&format!("'{type_id}': [")).nth(1) else {
                continue;
            };
            let block = block.split("]],").next().unwrap_or_default();
            let Some(row) = block.split(&format!("['{name}', '")).nth(1) else {
                continue;
            };
            let row = row.split(']').next().unwrap_or_default();
            // What follows the row's last quoted text: nothing, or `, 100`, or `, null, 100`.
            let start = row[row.rfind('\'').unwrap_or(0) + 1..]
                .rsplit(',')
                .next()
                .unwrap_or_default()
                .trim();
            let start = if start.is_empty() {
                0.0
            } else {
                start.parse::<f64>().unwrap_or(f64::NAN)
            };
            checked += 1;
            if start != value {
                wrong.push(format!(
                    "{type_id} {name}: a project without it reads {value}, the panel shows {start}"
                ));
            }
        }
    }
    assert!(
        checked >= 60,
        "only {checked} settings were found to compare; the reading has drifted"
    );
    assert!(
        wrong.is_empty(),
        "the panel starts these at a value the app does not: {wrong:#?}"
    );
}

/// Every effect the page names has a numbers entry, even an empty one: without it the card
/// says this build does not have the effect and offers no settings, as Toner's did.
#[test]
fn every_effect_the_page_names_has_a_settings_entry() {
    let page = page();
    let table = |name: &str| {
        let from = page
            .find(&format!("\nconst {name} = {{"))
            .unwrap_or_else(|| panic!("no {name} in the page"));
        let to = from + page[from..].find("\n};").expect("the table ends");
        page[from..to].to_string()
    };
    let (names, params) = (table("EFFECT_NAMES"), table("EFFECT_PARAMS"));
    let types: Vec<&str> = names
        .lines()
        .filter_map(|line| line.trim().strip_prefix('\'')?.split('\'').next())
        .collect();
    assert!(
        types.len() > 200,
        "only {} effects were read from EFFECT_NAMES",
        types.len()
    );
    let missing: Vec<&&str> = types
        .iter()
        .filter(|t| !params.contains(&format!("\n  '{t}': ")))
        .collect();
    assert!(
        missing.is_empty(),
        "these effects have no EFFECT_PARAMS entry: {missing:#?}"
    );
}
