//! B-617: the boxes of a stretched layer read its keys at its key time, as the picture does,
//! against D-742.
//!
//! Writes `verification/B-617_stretched_values_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/stretched_values/expected_stretched_values.json`, written by
//! `tools/stretched_values_reference.py` before any code: for each case a frame, its key time,
//! and what each box shows there.
use super::*;
use serde_json::Value as J;

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join(rel)
}

/// The same, numbers compared as numbers within 1e-9, so 0 and 0.0 are one.
fn same(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() < 1e-9,
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| same(a, b))
        }
        _ => a == b,
    }
}

#[test]
fn the_boxes_of_a_stretched_layer_read_its_keys_at_its_key_time() {
    let expected: J = serde_json::from_str(
        &std::fs::read_to_string(repo(
            "Fixtures/stretched_values/expected_stretched_values.json",
        ))
        .unwrap(),
    )
    .unwrap();
    let file = repo("Fixtures").join(expected["host"].as_str().unwrap());
    let viewer = Mutex::new(open(&file).unwrap_or_else(|d| panic!("open: {}", d.message)));
    viewer.lock().unwrap().composition = Id::new(expected["composition"].as_str().unwrap());
    let lid = expected["layer"].as_str().unwrap();
    let mut out = String::from(
        "# B-617: the boxes of a stretched layer\n\n\
         Written by `app/src/tests/stretched_values.rs` from `Fixtures/stretched_values/`, against \
         D-742. The layer starts at frame 4 and is stretched to 200 percent. Each line is one box \
         on one frame: what `tools/stretched_values_reference.py` says it shows, read at the \
         layer's key time, and what the build's boxes show.\n\n\
         | Case | Frame | Key time | Box | Should show | The build shows | Matches |\n\
         | --- | --- | --- | --- | --- | --- | --- |\n",
    );
    let (mut checks, mut passed) = (0, 0);
    for case in expected["cases"].as_array().unwrap() {
        let frame = case["frame"].as_i64().unwrap() as i32;
        let answer: J = serde_json::from_slice(&boxes(&viewer, frame, None).into_body())
            .expect("the boxes answer is JSON");
        for (name, want) in case["values"].as_object().unwrap() {
            let got = &answer["values"][lid][name];
            let ok = same(got, want);
            checks += 1;
            passed += ok as usize;
            out.push_str(&format!(
                "| {} | {frame} | {} | {name} | {want} | {got} | {} |\n",
                case["id"].as_str().unwrap(),
                case["key_time"],
                if ok { "yes" } else { "**NO**" }
            ));
        }
    }
    out.push_str(&format!("\nB-617: {passed} of {checks} checks pass.\n"));
    std::fs::write(repo("verification/B-617_stretched_values_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-617_stretched_values_table.md"
    );
}
