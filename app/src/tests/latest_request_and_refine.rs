use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn header(response: &Response<Vec<u8>>, name: &str) -> String {
    response.headers().get(name).map_or_else(String::new, |v| {
        v.to_str().unwrap_or("<not readable as text>").to_string()
    })
}

fn page() -> String {
    std::fs::read_to_string(repo("app/ui/index.html")).expect("read the page")
}

/// The page's mailbox and a pretend window, on a pretend clock. A Draft frame takes 40 ms
/// and a Full one 300 ms: round numbers of the size the tables in `verification/` report
/// for the reference shot, chosen so that the order of events is easy to follow, and not a
/// measurement of anything.
const SIMULATION: &str = r#"
const fs = require('fs');
const page = fs.readFileSync(process.argv[2], 'utf8');
const from = page.indexOf('// ---- B-162 / B-163 begin');
const to = page.indexOf('// ---- B-162 / B-163 end');
if (from < 0 || to < from) throw new Error('the markers are not in the page');
const { SETTLE_MS, askedAs, drawnWhenBack } = new Function(page.slice(from, to)
  + '\nreturn { SETTLE_MS, askedAs, drawnWhenBack };')();

// `moves` is when the hand puts the playhead on which frame, at the quality chosen.
function run(refine, quality, moves) {
  const RENDER = { draft: 40, full: 300 };
  let now = 0, stirred = -Infinity, inFlight = null, pending = null, settleAt = null, frame = 0;
  const asked = [], sent = [], drawn = [], dropped = [];
  const restless = () => refine && now - stirred < SETTLE_MS;
  const show = (url) => {
    if (inFlight) { pending = url; return; }
    const out = askedAs(url, restless());
    inFlight = { url: out, rough: out !== url, done: now + RENDER[out.endsWith('draft') ? 'draft' : 'full'] };
    sent.push({ t: now, url: out, newest: asked.length ? asked[asked.length - 1].url : null });
  };
  const back = () => {
    const f = inFlight;
    inFlight = null;
    if (drawnWhenBack(f.url, pending, restless())) {
      drawn.push({ t: now, url: f.url });
      frame = +f.url.split('/frame/')[1].split('?')[0];
      settleAt = f.rough ? Math.max(now, stirred + SETTLE_MS) : null;
    } else {
      dropped.push({ t: now, url: f.url });
    }
    const next = pending;
    pending = null;
    if (next) show(next);
  };
  let i = 0;
  for (;;) {
    const times = [inFlight ? inFlight.done : Infinity, i < moves.length ? moves[i][0] : Infinity,
      settleAt === null ? Infinity : settleAt];
    const t = Math.min(...times);
    if (t === Infinity) break;
    now = t;
    if (t === times[0]) back();
    else if (t === times[1]) {
      const [, at] = moves[i++];
      stirred = now;
      settleAt = null;
      const url = '/frame/' + at + '?q=' + quality;
      asked.push({ t: now, url });
      show(url);
    } else {
      settleAt = null;
      show('/frame/' + frame + '?q=' + quality);
    }
  }
  return { asked, sent, drawn, dropped };
}

// A fast scrub: one frame further every 16 ms, frames 0 to 30.
const scrub = (first, last, start) => Array.from({ length: last - first + 1 },
  (_, k) => [start + 16 * k, first + k]);
console.log(JSON.stringify({
  settle: SETTLE_MS,
  scrubFull: run(true, 'full', scrub(0, 30, 0)),
  scrubFullNoRefine: run(false, 'full', scrub(0, 30, 0)),
  scrubDraft: run(true, 'draft', scrub(0, 30, 0)),
  // Frames 0 to 10, still, then frames 11 to 20 while the sharp picture of 10 is being made.
  again: run(true, 'full', scrub(0, 10, 0).concat(scrub(11, 20, 400))),
}));
"#;

fn simulate() -> serde_json::Value {
    let script = std::env::temp_dir().join("b162_latest_request_simulation.js");
    std::fs::write(&script, SIMULATION).expect("write the simulation");
    let out = std::process::Command::new("node")
        .arg(&script)
        .arg(repo("app/ui/index.html"))
        .output()
        .expect("these checks run the page's own lines under Node, which must be installed");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("the simulation prints JSON")
}

fn urls(list: &serde_json::Value) -> Vec<String> {
    list.as_array()
        .expect("a list")
        .iter()
        .map(|r| r["url"].as_str().expect("a url").to_string())
        .collect()
}

fn times(list: &serde_json::Value) -> Vec<f64> {
    list.as_array()
        .expect("a list")
        .iter()
        .map(|r| r["t"].as_f64().expect("a time"))
        .collect()
}

/// The longest the picture stood still while the hand was moving, in pretend milliseconds.
fn longest_wait(run: &serde_json::Value) -> f64 {
    let last_move = *times(&run["asked"]).last().expect("the hand moved");
    let mut at = 0.0;
    let mut most: f64 = 0.0;
    for t in times(&run["drawn"]).into_iter().filter(|&t| t <= last_move) {
        most = most.max(t - at);
        at = t;
    }
    most.max(last_move - at)
}

/// Ask the real request handler for each of `asked`, in order, on one viewer.
fn replay(asked: &[String]) -> Vec<Response<Vec<u8>>> {
    let viewer = Mutex::new(demo());
    let export = Mutex::new(Export::default());
    asked
        .iter()
        .map(|url| {
            let (path, query) = url.split_once('?').expect("a quality");
            let (ask, quality) = parse(path, Some(query)).expect("a request the page sends");
            serve(&viewer, &export, ask, quality)
        })
        .collect()
}

fn write_table(
    rows: &[(String, String, String)],
    file: &str,
    title: &str,
    intro: &str,
    notes: &str,
) {
    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut out = format!("# {title}\n\n{intro}\n\nProduced by `cargo test -p anime_compositor_app latest_request_and_refine`, from `app/src/main.rs`.\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
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
        "\n**{passed} of {} checks pass.**\n\n{notes}\n",
        rows.len()
    ));
    std::fs::write(repo(file), out).expect("write the table");
}

#[test]
fn b162_only_the_newest_frame_is_made_and_a_stale_one_is_not_drawn() {
    let said = simulate();
    let page = page();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: String, actual: String| {
        rows.push((what.to_string(), expected, actual));
    };

    for (name, run) in [
        ("at Full with Refine when idle", &said["scrubFull"]),
        ("at Full without it", &said["scrubFullNoRefine"]),
        ("at Draft", &said["scrubDraft"]),
    ] {
        let asked = run["asked"].as_array().expect("asked").len();
        let sent = run["sent"].as_array().expect("sent");
        check(
            &format!("a fast scrub {name}: 31 frames are asked for and fewer are made"),
            "true".into(),
            (sent.len() < asked).to_string(),
        );
        let older = sent
            .iter()
            .filter(|s| {
                let url = s["url"].as_str().expect("url");
                let newest = s["newest"].as_str().unwrap_or(url);
                let frame_of = |u: &str| u.split('?').next().unwrap_or(u).to_string();
                frame_of(url) != frame_of(newest)
            })
            .count();
        check(
            &format!("{name}: every frame made was the newest one asked for when it began"),
            "0 older frames made".into(),
            format!("{older} older frames made"),
        );
        let drawn = urls(&run["drawn"]);
        check(
            &format!("{name}: the last picture drawn is frame 30"),
            "/frame/30".into(),
            drawn
                .last()
                .and_then(|u| u.split('?').next())
                .unwrap_or("")
                .to_string(),
        );
    }
    let today = &said["scrubFullNoRefine"];
    check(
        "without Refine when idle every answer is drawn, which is how the viewer has always been",
        urls(&today["sent"]).join(" "),
        urls(&today["drawn"]).join(" "),
    );
    let again = &said["again"];
    check(
        "the hand moves again while the sharp picture of frame 10 is being made: it is not drawn",
        "/frame/10?q=full".into(),
        urls(&again["dropped"]).join(" "),
    );
    check(
        "and the picture ends on frame 20 at Full",
        "/frame/20?q=full".into(),
        urls(&again["drawn"]).last().cloned().unwrap_or_default(),
    );

    // The same requests, asked of the window: the answer to each is the picture that request
    // has always made.
    let sent = urls(&said["scrubFullNoRefine"]["sent"]);
    let made = replay(&sent);
    let alone = replay(&sent[sent.len() - 1..]);
    check(
        "the window, asked for the frames the scrub sent, ends on frame 30",
        "30".into(),
        header(made.last().expect("an answer"), "x-frame"),
    );
    check(
        "and its pixels are byte for byte the ones frame 30 at Full makes when asked for alone",
        "true".into(),
        (made.last().expect("an answer").body() == alone[0].body()).to_string(),
    );

    for (what, marker) in [
        (
            "the page asks for a frame at the quality `askedAs` gives",
            "  url = askedAs(url, restless());",
        ),
        (
            "and draws an answer only when `drawnWhenBack` says so",
            "    if (!drawnWhenBack(url, pending, restless())) return;",
        ),
        (
            "one request out and the newest waiting (P-04, unchanged)",
            "    if (keep) pending = url;",
        ),
    ] {
        check(what, "true".into(), page.contains(marker).to_string());
    }

    write_table(
        &rows,
        "verification/B-162_latest_request_table.md",
        "B-162: only the newest frame is made, and a stale one is not drawn",
        "When the playhead is dragged fast, the viewer should not spend its time finishing \
         frames the hand has already left. The page sends one request at a time and keeps only \
         the newest one waiting (P-04 built that); this unit adds that a sharp picture of a \
         place the hand has left is not drawn when it arrives late, and checks the whole \
         bargain. The scrub rows run the page's own decision lines on a pretend clock (a Draft \
         frame 40 ms, a Full one 300 ms); the window rows ask the real request handler.",
        "## What this does not cover\n\nStopping a frame half way. The window answers the \
         viewer's requests one after another on its own thread, so word that a newer frame is \
         wanted cannot reach a render until it has finished, and the renderer has no point at \
         which to stop. The frame in flight is always finished; at most one more waits behind \
         it. Nor does it cover how it feels in the window: that is the playtest sheet, \
         `verification/B-162_B-163_playtest.md`.",
    );
    assert!(rows.iter().all(|(_, e, a)| e == a), "{rows:#?}");
}

#[test]
fn b163_draft_while_moving_and_the_full_picture_once_still() {
    let said = simulate();
    let page = page();
    let settle = said["settle"].as_f64().expect("SETTLE_MS");
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |what: &str, expected: String, actual: String| {
        rows.push((what.to_string(), expected, actual));
    };

    let run = &said["scrubFull"];
    let sent = urls(&run["sent"]);
    let last_move = *times(&run["asked"]).last().expect("moves");
    let during: Vec<&String> = sent[..sent.len() - 1].iter().collect();
    check(
        "at Full with Refine when idle, every frame made while the hand moves is Draft",
        "true".into(),
        during.iter().all(|u| u.ends_with("?q=draft")).to_string(),
    );
    check(
        "and one more is asked for once it stops: frame 30 at Full",
        "/frame/30?q=full".into(),
        sent.last().cloned().unwrap_or_default(),
    );
    let refined_at = *times(&run["sent"]).last().expect("sent");
    check(
        "no sooner than 0.2 s after the last move",
        "true".into(),
        (refined_at - last_move >= settle).to_string(),
    );
    check(
        "and no later than 0.2 s after it, or after the frame then being made",
        "true".into(),
        (refined_at - last_move <= settle + 40.0).to_string(),
    );
    check(
        "the last picture drawn is that Full one",
        "/frame/30?q=full".into(),
        urls(&run["drawn"]).last().cloned().unwrap_or_default(),
    );
    let (on, off) = (longest_wait(run), longest_wait(&said["scrubFullNoRefine"]));
    check(
        &format!(
            "the picture keeps up: the longest it stands still during the scrub is {on} pretend ms, \
             against {off} without Refine when idle"
        ),
        "true".into(),
        (on < off).to_string(),
    );
    check(
        "at Draft nothing changes: every frame is asked for at Draft",
        "true".into(),
        urls(&said["scrubDraft"]["sent"])
            .iter()
            .all(|u| u.ends_with("?q=draft"))
            .to_string(),
    );
    check(
        "without Refine when idle nothing changes: every frame is asked for at Full",
        "true".into(),
        urls(&said["scrubFullNoRefine"]["sent"])
            .iter()
            .all(|u| u.ends_with("?q=full"))
            .to_string(),
    );

    // The requests the scrub sent, asked of the window, then the Full picture on its own.
    let made = replay(&sent);
    let sharp_alone = replay(&["/frame/30?q=full".to_string()]).remove(0);
    let rough_alone = replay(&["/frame/30?q=draft".to_string()]).remove(0);
    let at = sent
        .iter()
        .rposition(|u| u == "/frame/30?q=draft")
        .expect("a stand-in of frame 30");
    let rough = &made[at];
    let sharp = made.last().expect("an answer");
    check(
        "a stand-in is Draft and says so",
        "Draft true".into(),
        format!(
            "{} {}",
            header(rough, "x-quality"),
            header(rough, "x-differs")
        ),
    );
    check(
        "the picture that replaces it is Full, the same as export",
        "Full false".into(),
        format!(
            "{} {}",
            header(sharp, "x-quality"),
            header(sharp, "x-differs")
        ),
    );
    check(
        "at 1920 by 1080",
        "1920 1080".into(),
        format!("{} {}", header(sharp, "x-width"), header(sharp, "x-height")),
    );
    check(
        "and its pixels are byte for byte frame 30 at Full asked for alone, which is today's Full",
        "true".into(),
        (sharp.body() == sharp_alone.body()).to_string(),
    );
    check(
        "and the Draft stand-in of frame 30 is byte for byte frame 30 at Draft asked for alone",
        "true".into(),
        (rough.body() == rough_alone.body()).to_string(),
    );

    for (what, marker) in [
        ("Refine when idle is on unless it has been unticked", "memory: 'auto', refine: true };"),
        ("its tick is in the viewer's row beside the Full/Draft button", "<input type=\"checkbox\" id=\"refine\" checked> Refine when idle</label>"),
        ("and does nothing at Draft", "    $('refine').disabled = quality !== 'full';"),
        ("a stand-in leaves the Full/Draft choice as it was", "    if (!rough) quality = drawnAt;"),
        ("the Full/Draft button still names what pressing it does", "    $('toggle').textContent = quality === 'draft' ? 'Full resolution' : 'Draft resolution';"),
        ("\"sharpening…\" shows while a stand-in is on screen", "    $('refining').hidden = !rough;"),
        ("the sharp picture is asked for once the hand has been still for SETTLE_MS", "    if (rough) settling = setTimeout(sharpen, Math.max(0, stirred + SETTLE_MS - performance.now()));"),
        ("layer outlines are asked for at the quality of the picture they are drawn over", "'/boxes/' + asked + '?q=' + drawnAt"),
    ] {
        check(what, "true".into(), page.contains(marker).to_string());
    }

    let ms = |r: &Response<Vec<u8>>| header(r, "x-ms");
    write_table(
        &rows,
        "verification/B-163_refine_on_stop_table.md",
        "B-163: Draft while the hand moves, the Full picture once it stops",
        "At Full, with Refine when idle ticked (it is, unless unticked), a frame asked for \
         while the playhead is dragged or a value is scrubbed is made at Draft, and 0.2 s after \
         the hand stops the same frame is made at Full and replaces it. The scrub rows run the \
         page's own decision lines on a pretend clock (a Draft frame 40 ms, a Full one 300 ms); \
         the window rows ask the real request handler for the frames that scrub sent.",
        &format!(
            "## What this does not cover\n\nHow it feels in the window, which is the playtest \
             sheet, `verification/B-162_B-163_playtest.md`. For scale, this run's window took \
             {} ms for the Draft stand-in of frame 30 and {} ms for its Full picture, on the \
             CPU, in a test build sharing the machine with other builds: a note, not a \
             measurement.",
            ms(rough),
            ms(sharp)
        ),
    );
    assert!(rows.iter().all(|(_, e, a)| e == a), "{rows:#?}");
}
