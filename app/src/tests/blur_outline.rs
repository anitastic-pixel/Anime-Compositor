use super::*;

fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = what
        .split_once('?')
        .map_or((what, None), |(i, q)| (i, Some(q)));
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

fn corners(viewer: &Mutex<Viewer>, id: &str, q: PreviewQuality) -> Vec<f64> {
    let answer: serde_json::Value =
        serde_json::from_slice(boxes(viewer, 0, Some(q)).body()).expect("boxes is JSON");
    answer["layers"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|b| b["layer"] == id)
        .and_then(|b| b["corners"].as_array().cloned())
        .unwrap_or_default()
        .iter()
        .map(|v| v.as_f64().unwrap_or(f64::NAN))
        .collect()
}

fn said(c: &[f64]) -> String {
    c.chunks(2)
        .map(|p| format!("{:.0},{:.0}", p[0], p[1]))
        .collect::<Vec<_>>()
        .join(" ")
}

#[test]
fn d325_a_blurred_layer_keeps_its_own_outline() {
    use PreviewQuality::{Draft, Full};
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let source = repo.join("Fixtures/projects/cel_holds_project.json");
    let viewer = Mutex::new(
        open(&source).unwrap_or_else(|d| panic!("open {}: {}", source.display(), d.message)),
    );
    let export = Mutex::new(Export::default());
    run(&viewer, "layer.add_solid");
    let answer: serde_json::Value = serde_json::from_str(&state(&viewer)).expect("state is JSON");
    let solid = answer["project"]["compositions"][0]["layers"]
        .as_array()
        .expect("layers")
        .iter()
        .find(|l| l["name"] == "Solid 1")
        .expect("Solid 1")["id"]
        .as_str()
        .expect("an id")
        .to_string();
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.into(), expected.into(), actual))
    };
    // The owner's case at 1920 by 1080: a white solid cut to a rectangle by a mask.
    run(&viewer, &format!("solid.set?layer={solid}&color=1,1,1"));
    run(&viewer, &format!("mask.add?layer={solid}&points=744,369,0,0,0,0;1176,369,0,0,0,0;1176,702,0,0,0,0;744,702,0,0,0,0"));
    let whole = "0,0 1920,0 1920,1080 0,1080";
    row(
        "the masked solid's outline, no effect, Full",
        whole,
        said(&corners(&viewer, &solid, Full)),
    );
    run(
        &viewer,
        &format!("effect.add?layer={solid}&type=core.gaussian_blur"),
    );
    let fx = {
        let answer: serde_json::Value =
            serde_json::from_str(&state(&viewer)).expect("state is JSON");
        answer["project"]["compositions"][0]["layers"]
            .as_array()
            .expect("layers")
            .iter()
            .find(|l| l["id"] == solid.as_str())
            .expect("the solid")["effects"][0]["instance_id"]
            .as_str()
            .expect("an id")
            .to_string()
    };
    run(
        &viewer,
        &format!("effect.set_parameters?layer={solid}&effect={fx}&sigma_px=20"),
    );
    let full = corners(&viewer, &solid, Full);
    row(
        "with Gaussian Blur 20, Full (was 60 wider on every side: -60,-60 1980,-60 ...)",
        whole,
        said(&full),
    );
    row(
        "and Draft, a quarter of it",
        "0,0 480,0 480,270 0,270",
        said(&corners(&viewer, &solid, Draft)),
    );
    // The page's `layerSpace`: a layer point lands at the outline's first corner plus the
    // point scaled by the outline's size over the layer's.
    let on_screen = |x: f64, y: f64| {
        if full.len() == 8 {
            format!(
                "{:.0},{:.0}",
                full[0] + x * (full[2] - full[0]) / 1920.0,
                full[1] + y * (full[7] - full[1]) / 1080.0
            )
        } else {
            "no outline".into()
        }
    };
    row(
        "the mask's top-left corner, carried to the screen as the page carries it",
        "744,369",
        on_screen(744.0, 369.0),
    );
    row(
        "and its bottom-right corner",
        "1176,702",
        on_screen(1176.0, 702.0),
    );
    // The picture itself: the middle of what is more than half covered is the mask's middle.
    let frame = serve_logged(&viewer, &export, None, None, Ask::Frame(0), Some(Full));
    let width = frame
        .headers()
        .get("x-width")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(0);
    let px = frame.body();
    let (mut lo, mut hi) = ((usize::MAX, usize::MAX), (0, 0));
    for (i, _) in px.chunks(4).enumerate().filter(|(_, p)| p[3] > 128) {
        let (x, y) = (i % width.max(1), i / width.max(1));
        lo = (lo.0.min(x), lo.1.min(y));
        hi = (hi.0.max(x), hi.1.max(y));
    }
    row(
        "the blurred picture's middle (over half covered)",
        "960,535",
        format!(
            "{},{}",
            (lo.0 + hi.0).div_ceil(2),
            (lo.1 + hi.1).div_ceil(2)
        ),
    );
    row(
        "the mask's middle",
        "960,535",
        format!("{},{}", (744 + 1176) / 2, (369 + 702) / 2),
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-325: a blurred layer keeps its own outline\n\nWritten by `d325_a_blurred_layer_keeps_its_own_outline` \
         in `app/src/main.rs`. The outline is the window's `/boxes` answer, which the page draws the layer's box from \
         and carries every mask point through. The layer is a white 1920 by 1080 solid cut by a mask from 744,369 to \
         1176,702, the owner's case of 4 October at this size.\n\n**{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo.join("verification/D-325_blur_outline_table.md"), text)
        .expect("write the table");
    assert_eq!(
        passed,
        rows.len(),
        "see verification/D-325_blur_outline_table.md"
    );
}
