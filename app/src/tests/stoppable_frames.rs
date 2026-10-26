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

/// The page's canvas as `show()` keeps it, and its `x-sent` (D-886).
#[derive(Default)]
struct Page {
    pixels: Vec<u8>,
    held: Option<u64>,
}

impl Page {
    /// Asks for frame `n` as `show()` does, on a frame that arrived with the count at `since`,
    /// and draws the answer as `show()` does: a stopped answer draws nothing.
    fn ask(&mut self, viewer: &Mutex<Viewer>, export: &Mutex<Export>, n: i32, q: PreviewQuality) -> Response<Vec<u8>> {
        let since = anime_compositor::stop::now();
        let r = anime_compositor::stop::watching(since, || {
            serve_frame(viewer, export, None, None, Ask::Frame(n), Some(q), None, false, Some(self.held.unwrap_or(0)))
        });
        if header(&r, "x-stopped") == "1" {
            return r;
        }
        let width: usize = header(&r, "x-width").parse().expect("x-width");
        let height: usize = header(&r, "x-height").parse().expect("x-height");
        if self.pixels.len() != width * height * 4 {
            self.pixels = vec![0; width * height * 4];
        }
        let changed = header(&r, "x-changed");
        if changed.is_empty() {
            self.pixels.copy_from_slice(r.body());
        } else {
            let c: Vec<usize> = changed.split(',').map(|v| v.parse().expect("a number")).collect();
            for row in 0..c[3] {
                let at = ((c[1] + row) * width + c[0]) * 4;
                self.pixels[at..at + c[2] * 4].copy_from_slice(&r.body()[row * c[2] * 4..(row + 1) * c[2] * 4]);
            }
        }
        self.held = header(&r, "x-sent").parse().ok();
        r
    }
}

/// D-893: a viewer frame stopped part way (at each point it looks, as a `/stop` arriving then
/// would) answers `x-stopped` with no picture; nothing of it is kept or counted as sent; and the
/// frame asked next is made afresh and is byte for byte the frame a viewer that was never
/// stopped makes, after an edit, an undo, a frame change, a quality change and a switch of
/// composition. The page's rule for when to stop is checked under Node.
///
/// Writes `verification/B-773_stoppable_frames_table.md`.
#[test]
fn b773_a_stopped_frame_is_never_shown_or_kept() {
    use PreviewQuality::{Draft, Full};
    let export = Mutex::new(Export::default());
    // `stopped` is the viewer whose frames are stopped part way; `plain` is never stopped.
    let stopped = Mutex::new(demo());
    let plain = Mutex::new(demo());
    let run = |viewer: &Mutex<Viewer>, what: &str| {
        let (id, query) = what.split_once('?').map_or((what, None), |(i, q)| (i, Some(q)));
        edit_command(viewer, id, query).expect("a command document 24 lists");
    };
    let mut page = Page::default();
    let mut rows = String::new();
    let (mut checks, mut passed) = (0, 0);
    let mut step = |page: &mut Page, what: &str, n: i32, q: PreviewQuality, viewer: &Mutex<Viewer>, other: &Mutex<Viewer>| {
        // Stopped at every point it looks, one after another (between layers, before each tile,
        // after the draw), until a frame comes back whole.
        let (mut stops, mut was_stopped) = (0, true);
        let next = loop {
            let held = page.held;
            anime_compositor::stop::stop_at_check(stops + 1);
            let r = page.ask(viewer, &export, n, q);
            anime_compositor::stop::stop_at_check(0);
            if header(&r, "x-stopped") != "1" || stops == 500 {
                break r;
            }
            stops += 1;
            was_stopped &= r.status() == 204 && r.body().is_empty() && header(&r, "x-sent").is_empty() && page.held == held;
        };
        let was_stopped = was_stopped && stops > 1;
        let fresh = header(&next, "x-cached") == "0";
        let whole = serve_seen(other, &export, None, None, Ask::Frame(n), Some(q), None);
        let same = page.pixels == *whole.body();
        let ok = was_stopped && fresh && same;
        checks += 1;
        passed += usize::from(ok);
        rows.push_str(&format!(
            "| {what} | {} | {n} | {} | {} | {} | {} |\n",
            q.label(),
            if was_stopped { format!("stopped at each of {stops} points, no picture, nothing sent") } else { format!("**{stops} stops, not all clean**") },
            if fresh { "made afresh" } else { "**from memory**" },
            if same { "same" } else { "**differs**" },
            if ok { "PASS" } else { "FAIL" }
        ));
    };
    step(&mut page, "first frame", 0, Full, &stopped, &plain);
    for v in [&stopped, &plain] {
        run(v, "property.set_base?layer=layer-2&prop=position&value=700,400&frame=0");
    }
    step(&mut page, "after an edit (layer-2 moved)", 0, Full, &stopped, &plain);
    for x in [710, 760] {
        for v in [&stopped, &plain] {
            run(v, &format!("property.drag_update?layer=layer-3&prop=position&value={x},200"));
        }
        step(&mut page, &format!("during a drag (layer-3 to {x},200)"), 0, Full, &stopped, &plain);
    }
    // Undone to a state never drawn, so its frame is not in memory (one in memory is answered
    // without drawing, and there is nothing to stop).
    for v in [&stopped, &plain] {
        run(v, "property.drag_end");
        run(v, "property.set_base?layer=layer-2&prop=position&value=640,360&frame=0");
        run(v, "property.set_base?layer=layer-2&prop=position&value=600,300&frame=0");
    }
    step(&mut page, "before the undo (layer-2 to 600,300)", 0, Full, &stopped, &plain);
    for v in [&stopped, &plain] {
        undo(v);
    }
    step(&mut page, "after an undo (layer-2 back to 640,360)", 0, Full, &stopped, &plain);
    step(&mut page, "a frame change", 60, Full, &stopped, &plain);
    step(&mut page, "a quality change (Draft)", 60, Draft, &stopped, &plain);
    step(&mut page, "Draft, a frame change", 61, Draft, &stopped, &plain);
    step(&mut page, "a quality change (back to Full)", 61, Full, &stopped, &plain);
    for v in [&stopped, &plain] {
        frame_boxes::switch(v);
    }
    step(&mut page, "a switch of composition", 61, Full, &stopped, &plain);
    let (checks, passed) = (checks, passed);

    // A frame not being watched (an export, a stored check) is never stopped.
    anime_compositor::stop::stop();
    let unwatched = serve_seen(&stopped, &export, None, None, Ask::Frame(62), Some(Full), None);
    let unwatched_ok = unwatched.status() == 200 && header(&unwatched, "x-stopped").is_empty();

    // The page's rule, run from its own lines under Node.
    let script = std::env::temp_dir().join("b773_stops_in_flight.js");
    std::fs::write(&script, POLICY).expect("write the policy check");
    let out = std::process::Command::new("node")
        .arg(&script)
        .arg(repo("app/ui/index.html"))
        .output()
        .expect("the page's rule runs under Node, which must be installed");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let said: serde_json::Value = serde_json::from_slice(&out.stdout).expect("JSON");
    let mut rule_rows = String::new();
    let mut rule_ok = true;
    for case in said.as_array().expect("a list") {
        let (want, got) = (case["want"].as_bool().expect("want"), case["got"].as_bool().expect("got"));
        rule_ok &= want == got;
        rule_rows.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            case["what"].as_str().expect("what"),
            if want { "stop" } else { "let it finish" },
            if got { "stop" } else { "let it finish" },
            if want == got { "PASS" } else { "FAIL" }
        ));
    }

    let page_text = std::fs::read_to_string(repo("app/ui/index.html")).expect("read the page");
    let wired = [
        "      fetch(FRAMES + '/stop').catch(() => {});",
        "    stoppedLast = response.headers.get('x-stopped') === '1';",
        "      if (!pending) pending = chosen;",
    ]
    .iter()
    .all(|line| page_text.contains(line));

    let text = format!(
        "# B-773: stoppable frames (D-893)\n\n\
         Written by `b773_a_stopped_frame_is_never_shown_or_kept` in `app/src/tests/stoppable_frames.rs`.\n\n\
         Each row stops a viewer frame part way, as a `/stop` from the page arriving then would: \
         first at the first point it looks, then asked again and stopped at the second, and so on \
         (between the top composition's layers, before each tile, after the draw), until it comes \
         back whole. Every stopped answer must be `PREVIEW_FRAME_STOPPED`'s: status 204, `x-stopped: 1`, \
         no picture, no `x-sent`, and the page's picture left as it was. The answer that comes back \
         whole must be made afresh (`x-cached: 0`, so nothing of the stopped \
         frame was kept as a frame) and the page's picture must be byte for byte the frame a second \
         viewer, given the same changes and never stopped, makes. On the reference shot's viewer, on \
         the CPU.\n\n\
         **{passed} of {checks} stopped frames were never shown or kept, and the frame after each is \
         byte for byte the uninterrupted one.**\n\n\
         | Viewer state | Quality | Frame | The stopped answer | The next answer | Pixels against the never-stopped viewer | Result |\n\
         |---|---|---|---|---|---|---|\n{rows}\n\
         A frame that is not the viewer's (an export or a stored check asks this way) is never stopped, \
         even right after a stop: {}.\n\n\
         ## When the page stops a frame\n\n\
         The page's own `stopsInFlight`, run under Node. A drag's own frames are never stopped, or \
         the viewer would draw nothing until the hand let go.\n\n\
         | Frame in flight, and what is waiting | Expected | The page's rule | Result |\n|---|---|---|---|\n{rule_rows}\n\
         The page sends `/stop` and handles `x-stopped` (asking again when nothing newer waits): {}.\n",
        if unwatched_ok { "PASS" } else { "FAIL" },
        if wired { "PASS" } else { "FAIL" },
    );
    std::fs::write(repo("verification/B-773_stoppable_frames_table.md"), text).expect("write the table");
    assert_eq!(passed, checks, "see verification/B-773_stoppable_frames_table.md");
    assert!(unwatched_ok && rule_ok && wired, "see verification/B-773_stoppable_frames_table.md");
}

const POLICY: &str = r#"
const fs = require('fs');
const page = fs.readFileSync(process.argv[2], 'utf8');
const from = page.indexOf('// ---- B-162 / B-163 begin');
const to = page.indexOf('// ---- B-162 / B-163 end');
if (from < 0 || to < from) throw new Error('the markers are not in the page');
const { stopsInFlight } = new Function(page.slice(from, to) + '\nreturn { stopsInFlight };')();
const F = (n, q) => 'http://frame.localhost/frame/' + n + '?q=' + q;
const cases = [
  ['the sharp Full picture, and the drag has started again (B-162 would throw it away)', [F(4, 'full'), F(5, 'full'), true, false, false], true],
  ['a Draft frame of a drag, and the drag goes on', [F(4, 'draft'), F(5, 'draft'), true, false, false], false],
  ['a Full frame with Refine off, and the drag goes on', [F(4, 'full'), F(5, 'full'), false, false, false], false],
  ['any frame, and the hand is still (an undo, a click, a typed value)', [F(4, 'draft'), F(4, 'draft'), false, true, false], true],
  ['the same, but the last answer was a stopped one', [F(4, 'draft'), F(4, 'draft'), false, true, true], false],
  ['a frame, and nothing is waiting', [F(4, 'full'), null, true, true, false], false],
  ['playback', ['http://frame.localhost/at/400?q=draft', F(4, 'draft'), false, true, false], false],
];
console.log(JSON.stringify(cases.map(([what, args, want]) => ({ what, want, got: stopsInFlight(...args) }))));
"#;
