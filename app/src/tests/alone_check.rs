use super::*;

fn sent(response: &Response<Vec<u8>>) -> (usize, usize, Vec<u8>) {
    let number = |name: &str| {
        response
            .headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0)
    };
    (
        number("x-width"),
        number("x-height"),
        response.body().clone(),
    )
}

fn same(a: &(usize, usize, Vec<u8>), b: &(usize, usize, Vec<u8>)) -> String {
    if a == b {
        format!("{} by {}, every byte the same", a.0, a.1)
    } else {
        let differ = a.2.iter().zip(&b.2).filter(|(x, y)| x != y).count();
        format!(
            "{} by {} against {} by {}, {differ} bytes differ",
            a.0, a.1, b.0, b.1
        )
    }
}

#[test]
fn d258_one_layer_alone_is_that_layer_soloed() {
    use PreviewQuality::{Draft, Full};
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let export = Mutex::new(Export::default());
    let viewer = Mutex::new(demo());
    let ask = |q: PreviewQuality| {
        sent(&serve_logged(
            &viewer,
            &export,
            None,
            None,
            Ask::Frame(10),
            Some(q),
        ))
    };
    let layer = Id::new("layer-2");
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut row = |what: &str, expected: &str, actual: String| {
        rows.push((what.into(), expected.into(), actual))
    };

    let ordinary = ask(Full);
    let undo_before = viewer.lock().unwrap().document.undo_labels().len();
    let alone_full = alone(&viewer, &layer, 10, Full);
    let alone_draft = alone(&viewer, &layer, 10, Draft);
    row(
        "asking for layer2 alone solos nothing in the window",
        "no layer soloed, no undo step",
        if viewer.lock().unwrap().solo.is_empty()
            && viewer.lock().unwrap().document.undo_labels().len() == undo_before
        {
            "no layer soloed, no undo step".into()
        } else {
            "changed".into()
        },
    );
    row(
        "the ordinary frame 10 afterwards is the ordinary frame 10 before",
        "1920 by 1080, every byte the same",
        same(&ask(Full), &ordinary),
    );

    viewer.lock().unwrap().solo = vec![layer.clone()];
    let soloed_full = ask(Full);
    let soloed_draft = ask(Draft);
    viewer.lock().unwrap().solo.clear();
    row(
        "layer2 alone at frame 10, Full, is the frame sent with layer2 soloed",
        "1920 by 1080, every byte the same",
        alone_full
            .as_ref()
            .map_or_else(|e| e.clone(), |a| same(a, &soloed_full)),
    );
    row(
        "and at Draft too",
        &format!(
            "{} by {}, every byte the same",
            soloed_draft.0, soloed_draft.1
        ),
        alone_draft
            .as_ref()
            .map_or_else(|e| e.clone(), |a| same(a, &soloed_draft)),
    );
    row(
        "and it is not the whole picture",
        "different from the ordinary frame",
        alone_full.as_ref().map_or_else(
            |e| e.clone(),
            |a| {
                if a.2 == ordinary.2 {
                    "the same as the ordinary frame".into()
                } else {
                    "different from the ordinary frame".into()
                }
            },
        ),
    );
    row(
        "a layer that is not in the composition is refused",
        "refused",
        match alone(&viewer, &Id::new("layer-nobody"), 10, Full) {
            Ok(_) => "a picture".into(),
            Err(_) => "refused".into(),
        },
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# D-258: one layer alone, for the onion skin\n\nWritten by `d258_one_layer_alone_is_that_layer_soloed` \
         in `app/src/main.rs`. `alone` is the answer to the page's `/alone`; the soloed frames are the window's own \
         frame answers with the layer really soloed.\n\n**{passed} of {} checks pass.**\n\n\
         | Check | Expected | Actual | Result |\n|---|---|---|---|\n",
        rows.len()
    );
    for (what, expected, actual) in &rows {
        text.push_str(&format!(
            "| {what} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    std::fs::write(repo.join("verification/D-258_alone_table.md"), text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/D-258_alone_table.md");
}
