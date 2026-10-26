use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = what.split_once('?').map_or((what, None), |(i, q)| (i, Some(q)));
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers()
        .get(name)
        .map_or_else(String::new, |v| v.to_str().unwrap_or("").to_string())
}

/// Duplicate the composition on screen, open the copy and move its second layer, so the picture
/// and the boxes are of a composition the page has not seen.
pub(super) fn switch(viewer: &Mutex<Viewer>) {
    let from = held_comp(viewer);
    run(viewer, &format!("composition.duplicate?composition={from}"));
    let copy = {
        let v = viewer.lock().expect("the viewer lock was poisoned");
        let project = v.document.project();
        project
            .compositions
            .iter()
            .map(|c| c.id.as_str().to_string())
            .find(|id| *id != from)
            .expect("the copy is in the project")
    };
    run(viewer, &format!("composition.open?composition={copy}"));
    let layer = {
        let v = viewer.lock().expect("the viewer lock was poisoned");
        let project = v.document.project();
        let comp = project.composition(&v.composition).expect("open");
        let id = comp.layers_in_order().nth(1).expect("a second layer").id.as_str().to_string();
        id
    };
    run(viewer, &format!("property.set_base?layer={layer}&prop=position&value=500,300&frame=0"));
    assert_eq!(held_comp(viewer), copy, "the copy is on screen");
}

fn held_comp(viewer: &Mutex<Viewer>) -> String {
    viewer.lock().expect("the viewer lock was poisoned").composition.as_str().to_string()
}

/// A frame asked with `b=1`, split into its pixels and its boxes.
fn with_boxes(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    n: i32,
    q: PreviewQuality,
) -> (Response<Vec<u8>>, Vec<u8>, Option<serde_json::Value>) {
    let r = serve_frame(viewer, export, None, None, Ask::Frame(n), Some(q), None, true);
    let len: usize = header(&r, "x-boxes").parse().unwrap_or(0);
    let body = r.body().clone();
    let (pixels, outline) = body.split_at(body.len() - len);
    let outline = (len > 0).then(|| serde_json::from_slice(outline).expect("the boxes are JSON"));
    (r, pixels.to_vec(), outline)
}

/// D-885: a still frame asked with `b=1` brings the `/boxes` answer for the same snapshot after
/// its pixels. Checked after every kind of change the page sees: frames, qualities, an edit, a
/// drag under way, its end, an undo and a switch of composition. At each, the pixels must be
/// the frame asked without boxes, byte for byte, and the boxes the `/boxes` answer, value for
/// value.
///
/// Writes `verification/B-765_frame_boxes_table.md`.
#[test]
fn b765_the_frame_brings_its_own_boxes() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    let viewer = Mutex::new(demo());
    let steps: [(&str, &dyn Fn(&Mutex<Viewer>)); 6] = [
        ("as opened", &|_| {}),
        ("after an edit (layer-2 moved)", &|v| {
            run(v, "property.set_base?layer=layer-2&prop=position&value=700,400&frame=0");
        }),
        ("during a drag (layer-3, not let go)", &|v| {
            run(v, "property.drag_update?layer=layer-3&prop=position&value=300,200");
        }),
        ("after the drag ends", &|v| {
            run(v, "property.drag_end");
        }),
        ("after an undo", &|v| {
            undo(v);
        }),
        ("in another composition (a copy, one layer moved)", &|v| switch(v)),
    ];
    let mut rows = String::new();
    let (mut checks, mut passed) = (0, 0);
    for (what, step) in steps {
        step(&viewer);
        for q in [Draft, Full] {
            for n in [0, 120] {
                let plain = serve_seen(&viewer, &export, None, None, Ask::Frame(n), Some(q), None);
                let (r, pixels, outline) = with_boxes(&viewer, &export, n, q);
                let asked = serde_json::from_slice::<serde_json::Value>(
                    &boxes(&viewer, n, Some(q)).into_body(),
                )
                .expect("the boxes are JSON");
                let same_pixels = pixels == *plain.body();
                let same_boxes = outline.as_ref() == Some(&asked);
                let frame = outline.as_ref().map_or(-1, |o| o["frame"].as_i64().unwrap_or(-1));
                let ok = same_pixels
                    && same_boxes
                    && frame == i64::from(n)
                    && header(&r, "x-frame") == n.to_string()
                    && !asked["layers"].as_array().is_none_or(Vec::is_empty);
                checks += 1;
                passed += usize::from(ok);
                rows.push_str(&format!(
                    "| {what} | {} | {n} | {} | {} bytes, {} | {} layers, {} | {} |\n",
                    q.label(),
                    header(&r, "x-quality"),
                    pixels.len(),
                    if same_pixels { "same" } else { "DIFFER" },
                    asked["layers"].as_array().map_or(0, Vec::len),
                    if same_boxes { "same" } else { "DIFFER" },
                    if ok { "PASS" } else { "FAIL" },
                ));
            }
        }
    }
    // Asked without `b=1`, nothing is added: playback and every other caller are as before.
    let plain = serve_seen(&viewer, &export, None, None, Ask::Frame(0), Some(Full), None);
    let none = header(&plain, "x-boxes").is_empty();
    checks += 1;
    passed += usize::from(none);
    let text = format!(
        "# B-765: the frame brings its own boxes (D-885)\n\n\
         Written by `b765_the_frame_brings_its_own_boxes` in `app/src/tests/frame_boxes.rs`.\n\n\
         A still frame asked with `b=1` carries the `/boxes` answer after its pixels, from the same \
         snapshot, so the page asks no second question per frame and an outline can never be another \
         frame's. Each row asks the frame plainly, then with `b=1`, then `/boxes` on its own, on the \
         reference shot's viewer, on the CPU.\n\n\
         **{passed} of {checks} checks pass.**\n\n\
         | Viewer state | Quality asked | Frame | Quality said | Pixels with the boxes vs the plain frame | Boxes vs `/boxes` | Result |\n\
         |---|---|---|---|---|---|---|\n{rows}\n\
         A frame asked without `b=1` carries no boxes: {}.\n",
        if none { "PASS" } else { "FAIL" }
    );
    std::fs::write(repo("verification/B-765_frame_boxes_table.md"), text).expect("write the table");
    assert_eq!(passed, checks, "see verification/B-765_frame_boxes_table.md");
}
