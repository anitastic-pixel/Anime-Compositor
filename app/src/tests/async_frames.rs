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

/// The page's canvas: the picture and the `x-sent` it holds, drawn as `show()` draws.
#[derive(Default)]
struct Page {
    pixels: Vec<u8>,
    width: usize,
    held: u64,
}

impl Page {
    fn draw(&mut self, r: &Response<Vec<u8>>) -> String {
        let width: usize = header(r, "x-width").parse().expect("x-width");
        let height: usize = header(r, "x-height").parse().expect("x-height");
        if self.pixels.len() != width * height * 4 || self.width != width {
            self.width = width;
            self.pixels = vec![0; width * height * 4];
        }
        let changed = header(r, "x-changed");
        let body = r.body();
        if changed.is_empty() {
            self.pixels.copy_from_slice(body);
        } else {
            let c: Vec<usize> = changed.split(',').map(|v| v.parse().expect("a number")).collect();
            for row in 0..c[3] {
                let at = ((c[1] + row) * width + c[0]) * 4;
                self.pixels[at..at + c[2] * 4]
                    .copy_from_slice(&body[row * c[2] * 4..(row + 1) * c[2] * 4]);
            }
        }
        self.held = header(r, "x-sent").parse().unwrap_or(0);
        if changed.is_empty() { "whole".into() } else { format!("changed {changed}") }
    }
}

type Shared = (&'static Mutex<Viewer>, &'static Mutex<Export>);

/// Ask frame `n` the way the window now does: queued on the frame thread, answered there.
fn queue(at: Shared, n: i32, q: PreviewQuality, have: u64) -> std::sync::mpsc::Receiver<Response<Vec<u8>>> {
    let (tx, rx) = std::sync::mpsc::channel();
    off_the_window_thread(Box::new(move || {
        let r = serve_frame(at.0, at.1, None, None, Ask::Frame(n), Some(q), None, false, Some(have));
        let _ = tx.send(r);
    }));
    rx
}

/// D-890: the `frame` scheme's answers are made on their own thread while commands run on the
/// window's. Each read sent after a command shows that command, byte for byte the whole frame
/// (with B-766's `have=` / `x-changed`), after edits, a drag, frame and quality changes, an
/// undo and a composition switch; a read queued before a command, while the frame thread was
/// busy, comes back with the newer picture, never an older one; and answers leave in the order
/// they were asked.
///
/// Writes `verification/B-770_async_frames_table.md`.
#[test]
fn b770_frames_off_the_window_thread_stay_ordered_and_current() {
    use PreviewQuality::{Draft, Full};
    let viewer: &'static Mutex<Viewer> = Box::leak(Box::new(Mutex::new(demo())));
    let export: &'static Mutex<Export> = Box::leak(Box::new(Mutex::new(Export::default())));
    let at = (viewer, export);
    let run = |what: &str| {
        let (id, query) = what.split_once('?').map_or((what, None), |(i, q)| (i, Some(q)));
        edit_command(viewer, id, query).expect("a command document 24 lists");
    };
    let mut page = Page::default();
    let (mut rows, mut checks, mut passed) = (String::new(), 0, 0);
    let mut check = |page: &mut Page, what: &str, r: &Response<Vec<u8>>, n: i32, q: PreviewQuality| {
        let how = page.draw(r);
        let whole = serve_seen(viewer, export, None, None, Ask::Frame(n), Some(q), None);
        let ok = page.pixels == *whole.body();
        checks += 1;
        passed += usize::from(ok);
        rows.push_str(&format!(
            "| {what} | {} | {n} | {how} | {} |\n",
            q.label(),
            if ok { "PASS" } else { "FAIL" }
        ));
        how
    };
    let mut step = |page: &mut Page, what: &str, n: i32, q: PreviewQuality| {
        let r = queue(at, n, q, page.held).recv().expect("an answer");
        check(page, what, &r, n, q);
    };
    step(&mut page, "first ask", 0, Full);
    run("property.set_base?layer=layer-2&prop=position&value=700,400&frame=0");
    step(&mut page, "after an edit (layer-2 moved)", 0, Full);
    for x in [710, 730, 760] {
        run(&format!("property.drag_update?layer=layer-3&prop=position&value={x},200"));
        step(&mut page, &format!("during a drag (layer-3 to {x},200)"), 0, Full);
    }
    run("property.drag_end");
    step(&mut page, "after the drag ends", 0, Full);
    step(&mut page, "another frame", 60, Full);
    step(&mut page, "switched to Draft", 60, Draft);
    step(&mut page, "switched back to Full", 60, Full);
    undo(viewer);
    step(&mut page, "after an undo", 60, Full);
    frame_boxes::switch(viewer);
    step(&mut page, "in another composition", 60, Full);

    // The frame thread is busy (held here), a read is queued behind it, and then a command runs
    // on this thread, as a command does on the window's while a slow frame renders.
    let (hold, wait) = std::sync::mpsc::channel::<()>();
    off_the_window_thread(Box::new(move || {
        let _ = wait.recv();
    }));
    let queued = queue(at, 60, Full, page.held);
    run("viewer.toggle_alpha");
    hold.send(()).expect("release the frame thread");
    let r = queued.recv().expect("an answer");
    let late = check(&mut page, "read queued before the alpha view was switched on, answered after it (newer, not staler)", &r, 60, Full);

    // Answers leave in the order they were asked.
    let order = Arc::new(Mutex::new(Vec::new()));
    let (done, finished) = std::sync::mpsc::channel();
    for i in 0..64 {
        let order = Arc::clone(&order);
        let done = done.clone();
        off_the_window_thread(Box::new(move || {
            order.lock().expect("order").push(i);
            let _ = done.send(());
        }));
    }
    for _ in 0..64 {
        finished.recv().expect("each job ran");
    }
    assert_ne!(late, "changed 0,0,0,0", "the queued read brought the newer picture");
    let in_order = *order.lock().expect("order") == (0..64).collect::<Vec<_>>();

    let text = format!(
        "# B-770: frames answered off the window's thread (D-890)\n\n\
         Written by `b770_frames_off_the_window_thread_stay_ordered_and_current` in \
         `app/src/tests/async_frames.rs`.\n\n\
         Each row asks a frame on the frame thread (`off_the_window_thread`, the one the window \
         now uses) with `have=` the picture the page holds, after the command named ran on the \
         test's own thread, draws the answer as the page does, and compares it with the whole \
         frame asked plainly, byte for byte. Reference shot, CPU.\n\n\
         **{passed} of {checks} pictures are the current whole frame, byte for byte.**\n\n\
         | Viewer state | Quality | Frame | Sent as | Result |\n|---|---|---|---|---|\n{rows}\n\
         64 jobs queued one after another ran in the order queued: {}.\n",
        if in_order { "PASS" } else { "FAIL" }
    );
    std::fs::write(repo("verification/B-770_async_frames_table.md"), text).expect("write the table");
    assert!(in_order, "answers in the order asked");
    assert_eq!(passed, checks, "see verification/B-770_async_frames_table.md");
}
