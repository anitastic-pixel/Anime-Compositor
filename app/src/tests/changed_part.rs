use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers()
        .get(name)
        .map_or_else(String::new, |v| v.to_str().unwrap_or("").to_string())
}

/// The page's canvas, as `show()` keeps it: the picture, its size and the `x-sent` it holds.
#[derive(Default)]
struct Page {
    pixels: Vec<u8>,
    width: usize,
    height: usize,
    held: Option<u64>,
}

impl Page {
    /// Ask for frame `n` as `show()` does and draw the answer as `show()` does. Returns the
    /// bytes the answer carried and whether it was only the changed rectangle.
    fn ask(&mut self, viewer: &Mutex<Viewer>, export: &Mutex<Export>, n: i32, q: PreviewQuality) -> (usize, String) {
        let r = serve_frame(viewer, export, None, None, Ask::Frame(n), Some(q), None, false, Some(self.held.unwrap_or(0)));
        let width: usize = header(&r, "x-width").parse().expect("x-width");
        let height: usize = header(&r, "x-height").parse().expect("x-height");
        let body = r.body();
        // Resizing a canvas empties it, as in `show()`.
        if (width, height) != (self.width, self.height) {
            (self.width, self.height) = (width, height);
            self.pixels = vec![0; width * height * 4];
        }
        let changed = header(&r, "x-changed");
        if changed.is_empty() {
            assert_eq!(body.len(), width * height * 4, "a whole picture");
            self.pixels.copy_from_slice(body);
        } else {
            let c: Vec<usize> = changed.split(',').map(|v| v.parse().expect("a number")).collect();
            assert_eq!(body.len(), c[2] * c[3] * 4, "the rectangle's bytes");
            for row in 0..c[3] {
                let at = ((c[1] + row) * width + c[0]) * 4;
                self.pixels[at..at + c[2] * 4].copy_from_slice(&body[row * c[2] * 4..(row + 1) * c[2] * 4]);
            }
        }
        self.held = header(&r, "x-sent").parse().ok();
        (body.len(), if changed.is_empty() { "whole".into() } else { format!("changed {changed}") })
    }
}

/// D-886: a still frame asked with `have=` the picture the page holds is sent only the
/// rectangle that changed, and the page's picture put together from it is the whole frame,
/// byte for byte, after edits, frame changes, quality switches, the alpha view, an undo, an
/// answer the page threw away and a switch of composition.
///
/// Writes `verification/B-766_changed_part_table.md`.
#[test]
fn b766_the_changed_part_makes_the_whole_frame() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    let viewer = Mutex::new(demo());
    let run = |what: &str| {
        let (id, query) = what.split_once('?').map_or((what, None), |(i, q)| (i, Some(q)));
        edit_command(&viewer, id, query).expect("a command document 24 lists");
    };
    let mut page = Page::default();
    let mut rows = String::new();
    let (mut checks, mut passed, mut deltas) = (0, 0, 0);
    let mut step = |page: &mut Page, what: &str, n: i32, q: PreviewQuality, viewer: &Mutex<Viewer>| {
        let (bytes, how) = page.ask(viewer, &export, n, q);
        let whole = serve_seen(viewer, &export, None, None, Ask::Frame(n), Some(q), None);
        let ok = page.pixels == *whole.body();
        checks += 1;
        passed += usize::from(ok);
        deltas += usize::from(how != "whole");
        rows.push_str(&format!(
            "| {what} | {} | {n} | {how} | {bytes} of {} | {} |\n",
            q.label(),
            whole.body().len(),
            if ok { "PASS" } else { "FAIL" }
        ));
    };
    step(&mut page, "first ask, nothing held", 0, Full, &viewer);
    step(&mut page, "same frame again", 0, Full, &viewer);
    run("property.set_base?layer=layer-2&prop=position&value=700,400&frame=0");
    step(&mut page, "after an edit (layer-2 moved)", 0, Full, &viewer);
    for x in [710, 730, 760] {
        run(&format!("property.drag_update?layer=layer-3&prop=position&value={x},200"));
        step(&mut page, &format!("during a drag (layer-3 to {x},200)"), 0, Full, &viewer);
    }
    run("property.drag_end");
    step(&mut page, "after the drag ends", 0, Full, &viewer);
    for n in [1, 2, 60, 120] {
        step(&mut page, "another frame", n, Full, &viewer);
    }
    step(&mut page, "switched to Draft", 120, Draft, &viewer);
    step(&mut page, "Draft, next frame", 121, Draft, &viewer);
    step(&mut page, "switched back to Full", 121, Full, &viewer);
    undo(&viewer);
    step(&mut page, "after an undo", 121, Full, &viewer);
    run("viewer.toggle_alpha");
    step(&mut page, "alpha view on", 121, Full, &viewer);
    step(&mut page, "alpha view, next frame", 122, Full, &viewer);
    run("viewer.toggle_alpha");
    step(&mut page, "alpha view off", 122, Full, &viewer);
    // An answer the page threw away (a newer frame was asked for first): the core moved on, the
    // page did not, so the next answer must be whole.
    let _ = serve_frame(&viewer, &export, None, None, Ask::Frame(5), Some(Full), None, false, Some(page.held.unwrap_or(0)));
    step(&mut page, "after an answer the page threw away", 6, Full, &viewer);
    frame_boxes::switch(&viewer);
    step(&mut page, "in another composition (a copy, one layer moved)", 6, Full, &viewer);
    step(&mut page, "that composition, next frame", 7, Full, &viewer);
    let (checks, passed, deltas) = (checks, passed, deltas);
    // Asked without `have=`, as the snapshot tool and playback ask, the answer is as before.
    let plain = serve_seen(&viewer, &export, None, None, Ask::Frame(7), Some(Full), None);
    let none = header(&plain, "x-sent").is_empty() && header(&plain, "x-changed").is_empty();
    let text = format!(
        "# B-766: only the part that changed is sent (D-886)\n\n\
         Written by `b766_the_changed_part_makes_the_whole_frame` in `app/src/tests/changed_part.rs`.\n\n\
         Each row asks a frame as the page does, with `have=` the picture it holds, draws the answer \
         onto the picture it holds as the page does, then compares that picture with the whole frame \
         asked plainly, byte for byte. On the reference shot's viewer, on the CPU.\n\n\
         **{passed} of {checks} pictures are the whole frame, byte for byte; {deltas} came as only \
         the part that changed.**\n\n\
         | Viewer state | Quality | Frame | Sent as | Bytes sent | Result |\n\
         |---|---|---|---|---|---|\n{rows}\n\
         A frame asked without `have=` is sent whole, with no `x-sent` or `x-changed`: {}.\n",
        if none { "PASS" } else { "FAIL" }
    );
    std::fs::write(repo("verification/B-766_changed_part_table.md"), text).expect("write the table");
    assert!(none, "a plain ask is unchanged");
    assert!(deltas > 0, "some answers were only the changed part");
    assert_eq!(passed, checks, "see verification/B-766_changed_part_table.md");
}
