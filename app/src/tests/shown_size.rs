use super::*;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers().get(name).map_or_else(String::new, |v| {
        v.to_str().unwrap_or("<not readable as text>").to_string()
    })
}

/// The page's own `sizedAs`, run under Node: what each request becomes at each zoom.
const RULE: &str = r#"
const fs = require('fs');
const page = fs.readFileSync(process.argv[2], 'utf8');
const from = page.indexOf('// ---- B-162 / B-163 begin');
const to = page.indexOf('// ---- B-162 / B-163 end');
if (from < 0 || to < from) throw new Error('the markers are not in the page');
const { sizedAs } = new Function(page.slice(from, to) + '\nreturn { sizedAs };')();
const cases = [
  ['/frame/7?q=full', 1], ['/frame/7?q=full', 2], ['/frame/7?q=full', 0.6], ['/frame/7?q=full', 0.5],
  ['/frame/7?q=full', 0.3], ['/frame/7?q=full', 0.25], ['/frame/7?q=full', 0.1], ['/frame/7?q=half', 0.5],
  ['/frame/7?q=half', 0.25], ['/frame/7?q=draft', 0.25], ['/frame/7?q=full', null],
  ['/at/400?q=full', 0.25], ['/play/7?q=full', 0.25],
];
console.log(JSON.stringify(cases.map(([url, shown]) => ({ url, shown, got: sizedAs(url, shown) }))));
"#;

/// D-899 / B-779: below 100% zoom the viewer asks for the coarsest quality that still has a
/// picture pixel for every screen pixel (After Effects' Auto resolution), never finer than the one
/// chosen, unless Full pixels is ticked. The window draws that quality exactly as it always has,
/// so nothing new is drawn and exports are untouched.
///
/// Writes `verification/B-779_shown_size_table.md`.
#[test]
fn b779_below_100_percent_the_viewer_draws_at_the_size_shown() {
    let mut rows: Vec<(String, String, String)> = Vec::new();

    let script = std::env::temp_dir().join("b779_sized_as.js");
    std::fs::write(&script, RULE).expect("write the rule check");
    let out = std::process::Command::new("node")
        .arg(&script)
        .arg(repo("app/ui/index.html"))
        .output()
        .expect("the page's rule runs under Node, which must be installed");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let said: serde_json::Value = serde_json::from_slice(&out.stdout).expect("JSON");
    let want = [
        "/frame/7?q=full", "/frame/7?q=full", "/frame/7?q=full", "/frame/7?q=half",
        "/frame/7?q=half", "/frame/7?q=draft", "/frame/7?q=draft", "/frame/7?q=half",
        "/frame/7?q=draft", "/frame/7?q=draft", "/frame/7?q=full",
        "/at/400?q=full", "/play/7?q=full",
    ];
    for (case, want) in said.as_array().expect("a list").iter().zip(want) {
        let shown = match case["shown"].as_f64() {
            Some(s) => format!("{}%", s * 100.0),
            None => "any zoom, Full pixels ticked".into(),
        };
        rows.push((
            format!("`{}` shown at {shown}", case["url"].as_str().expect("url")),
            want.to_string(),
            case["got"].as_str().expect("got").to_string(),
        ));
    }

    // The window: the Half frame asked at 50% is a quarter of Full's pixels, labelled, and the
    // Full frame is untouched.
    let export = Mutex::new(Export::default());
    let viewer = Mutex::new(demo());
    let ask = |q| serve_logged(&viewer, &export, None, None, Ask::Frame(30), Some(q));
    let full = ask(PreviewQuality::Full);
    let half = ask(PreviewQuality::Half);
    rows.push((
        "frame 30 at 50%, Full chosen: drawn at Half, a quarter of the pixels, and says it is not final".into(),
        "960 540 Half true, 2073600 of 8294400 bytes".into(),
        format!(
            "{} {} {} {}, {} of {} bytes",
            header(&half, "x-width"),
            header(&half, "x-height"),
            header(&half, "x-quality"),
            header(&half, "x-differs"),
            half.body().len(),
            full.body().len()
        ),
    ));
    rows.push((
        "the quality chosen stays Full for the window, the read-ahead and exports".into(),
        "Full false".into(),
        format!("{} {}", header(&full, "x-quality"), header(&full, "x-differs")),
    ));
    let alone = Mutex::new(demo());
    rows.push((
        "and Full asked after it is byte for byte Full asked for alone".into(),
        "true".into(),
        (ask(PreviewQuality::Full).body()
            == serve_logged(&alone, &export, None, None, Ask::Frame(30), Some(PreviewQuality::Full)).body())
        .to_string(),
    ));

    let page = std::fs::read_to_string(repo("app/ui/index.html")).expect("read the page");
    for (what, line) in [
        ("the page asks for the size shown", "  url = sizedAs(url, shownNow()); // D-899"),
        ("which a picture of that size does not make the quality chosen", "    if (!rough && !sized) quality = drawnAt;"),
        ("the label says so", "(sized && !rough ? ' (size shown)' : '')"),
        ("a zoom asks again when the picture drawn is the wrong size", "  sizeStillRight();\n"),
        (
            "but never for a stand-in, the card's Draft for In motion included, which is sharpened anyway",
            "  if (!doc || playing || inFlight || drawnRough) return;",
        ),
        (
            "only on the processor: a frame the card drew keeps the quality chosen, as Half there is no quicker",
            "  return prefs.fullPixels || onCard ? null : shownScale() * devicePixelRatio;",
        ),
        ("which each frame's x-drawn-on says", "    onCard = !drawnOn.startsWith('CPU'); // D-899"),
        ("Full pixels is a tick in the quality menu, off unless ticked", "<input type=\"checkbox\" id=\"fullpixels\"> Full pixels</label>"),
        ("and remembered", "refine: true, fullPixels: false };"),
    ] {
        rows.push((what.to_string(), "true".into(), page.contains(line).to_string()));
    }

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-779: the viewer draws at the size it is shown (D-899)\n\n\
         Written by `b779_below_100_percent_the_viewer_draws_at_the_size_shown` in \
         `app/src/tests/shown_size.rs`.\n\n\
         Below 100% zoom a still frame, or a drag's, is asked for at the coarsest quality that \
         still has a picture pixel for every screen pixel, never finer than the one chosen; \
         \"shown at\" is screen pixels per composition pixel. Ticking Full pixels keeps the \
         quality chosen at any zoom. The first rows run the page's own `sizedAs` under Node; the \
         others ask the real request handler on the reference shot.\n\n\
         **{passed} of {} checks pass.**\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (check, expected, actual) in &rows {
        text.push_str(&format!(
            "| {check} | {expected} | {actual} | {} |\n",
            if expected == actual { "pass" } else { "FAIL" }
        ));
    }
    text.push_str(
        "\n## What this does not cover\n\nHow quick it is, which the lag harness measures \
         (D-899's evidence), and how it looks, which is `verification/B-779_playtest.md`.\n",
    );
    std::fs::write(repo("verification/B-779_shown_size_table.md"), text).expect("write the table");
    assert!(rows.iter().all(|(_, e, a)| e == a), "{rows:#?}");
}
