//! B-609: parametric shapes, against D-738 (a).
//!
//! Writes `verification/B-609_parametric_shapes_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/parametric_shapes/`, written by `tools/parametric_shape_reference.py` before any
//! code: `host.json`, five parametric shapes; `expected_parametric_shapes.json`, each shape's
//! outline at a frame, what saving must write, and the refused files.
//!
//! # What is checked
//!
//! Each case's outline is the shape's path at that frame, every number within 1e-9, and the
//! shape is drawn exactly as a plain shape of that outline is. Saved, every shape keeps its type,
//! its path is its outline at its settings' bases with no keys, and what the oval keeps is kept.
//! Each refused file is refused with the diagnostic, naming the word.

use std::fs;

use serde_json::Value as J;

use anime_compositor::mask::MaskPoint;
use anime_compositor::persist;
use anime_compositor::shape::{self, Shape};

mod common;
use common::repo;

fn written(points: &[MaskPoint]) -> J {
    J::Array(
        points
            .iter()
            .map(|p| {
                serde_json::json!({
                    "point": [p.point.0, p.point.1],
                    "in": [p.in_handle.0, p.in_handle.1],
                    "out": [p.out_handle.0, p.out_handle.1],
                })
            })
            .collect(),
    )
}

/// The same JSON, numbers within 1e-9.
fn close(a: &J, b: &J) -> bool {
    match (a, b) {
        (J::Number(x), J::Number(y)) => (x.as_f64().unwrap() - y.as_f64().unwrap()).abs() <= 1e-9,
        (J::Array(x), J::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(a, b)| close(a, b))
        }
        (J::Object(x), J::Object(y)) => {
            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| close(v, w)))
        }
        _ => a == b,
    }
}

fn read_points(v: &J) -> Vec<MaskPoint> {
    let two = |v: &J| (v[0].as_f64().unwrap(), v[1].as_f64().unwrap());
    v.as_array()
        .unwrap()
        .iter()
        .map(|p| MaskPoint {
            point: two(&p["point"]),
            in_handle: two(&p["in"]),
            out_handle: two(&p["out"]),
        })
        .collect()
}

#[test]
fn b609_parametric_shapes() {
    let root = repo("Fixtures/parametric_shapes");
    let expected: J = serde_json::from_str(
        &fs::read_to_string(root.join("expected_parametric_shapes.json")).unwrap(),
    )
    .unwrap();
    let host = fs::read_to_string(root.join("host.json")).unwrap();
    let loaded = persist::load_str(&host).expect("the host reads");
    let project = loaded.document.project();
    let layer = project.compositions[0].layers_in_order().next().unwrap();
    let (width, height) = (400, 300);
    let mut out = String::from(
        "# B-609: parametric shapes\n\n\
         Written by `tests/b609_parametric_shapes.rs` from `Fixtures/parametric_shapes/`, against \
         D-738 (a). Each line is one check: what `tools/parametric_shape_reference.py` says, and what \
         the build gives.\n\n\
         | Case | Check | Should | The build | Matches |\n| --- | --- | --- | --- | --- |\n",
    );
    let (mut checks, mut passed) = (0, 0);
    let mut row = |who: &str, what: &str, ok: bool, should: String, built: String| {
        checks += 1;
        passed += ok as usize;
        let verdict = if ok { "yes" } else { "**NO**" };
        out.push_str(&format!(
            "| {who} | {what} | {should} | {built} | {verdict} |\n"
        ));
    };

    for (cid, c) in expected["cases"].as_object().unwrap() {
        if let Some(outline) = c.get("outline") {
            let at = c["shape"].as_u64().unwrap() as usize;
            let frame = c["frame"].as_i64().unwrap() as i32;
            let s = &layer.shapes[at];
            let built = written(&s.points_at(frame));
            let ok = close(outline, &built);
            row(
                cid,
                &format!("\"{}\" at frame {frame}: the outline", s.name),
                ok,
                format!(
                    "{} points, first {}",
                    outline.as_array().unwrap().len(),
                    outline[0]["point"]
                ),
                if ok {
                    format!(
                        "{} points, first {}",
                        built.as_array().unwrap().len(),
                        built[0]["point"]
                    )
                } else {
                    built.to_string()
                },
            );
            // Drawn as the plain shape of that outline is: every channel of every pixel.
            let plain = Shape {
                points: read_points(outline),
                parametric: None,
                ..s.at(frame)
            };
            let same = shape::draw(&[s.at(frame)], width, height).data()
                == shape::draw(&[plain], width, height).data();
            row(
                cid,
                "drawn as a plain shape of that outline",
                same,
                "the same pixels".into(),
                if same {
                    "the same pixels"
                } else {
                    "different pixels"
                }
                .into(),
            );
        } else if let Some(file) = c["file"].as_str() {
            let id = c["refused"].as_str().unwrap();
            let word = c["names"].as_str().unwrap();
            let read = persist::load_str(&fs::read_to_string(root.join(file)).unwrap());
            let ok = read
                .as_ref()
                .err()
                .is_some_and(|d| d.id.as_str() == id && d.detail.contains(word));
            row(
                cid,
                file,
                ok,
                format!("refused, {id}, naming {word}"),
                match &read {
                    Ok(_) => "read".to_string(),
                    Err(d) => format!("refused, {}: {}", d.id.as_str(), d.detail),
                },
            );
        } else {
            let saved: J =
                serde_json::from_str(&persist::to_json(project, &loaded.preserved)).unwrap();
            let shapes = &saved["compositions"][0]["layers"][0]["shapes"];
            for want in c["saved"].as_array().unwrap() {
                let at = want["shape"].as_u64().unwrap() as usize;
                let s = &shapes[at];
                row(
                    cid,
                    &format!("shape {at}: type"),
                    s["parametric"]["type"] == want["type"],
                    want["type"].to_string(),
                    s["parametric"]["type"].to_string(),
                );
                let keys = s["path"]["keyframes"].as_array().map_or(99, Vec::len);
                let ok = close(&s["path"]["base"]["points"], &want["path_base"]) && keys == 0;
                row(
                    cid,
                    &format!("shape {at}: path, the base outline, no keys"),
                    ok,
                    "as the reference".into(),
                    if ok {
                        "as the reference".into()
                    } else {
                        format!("{} keys, {}", keys, s["path"])
                    },
                );
            }
            let kept = &c["kept"];
            let p = &shapes[kept["shape"].as_u64().unwrap() as usize]["parametric"];
            for key in ["roundness", "bulge"] {
                row(
                    cid,
                    &format!("the oval keeps {key}"),
                    p[key] == kept[key],
                    kept[key].to_string(),
                    p[key].to_string(),
                );
            }
            // And the saved project reads back as the same shapes.
            let back = persist::load_str(&saved.to_string()).map(|l| {
                l.document.project().compositions[0]
                    .layers_in_order()
                    .next()
                    .unwrap()
                    .shapes
                    .iter()
                    .map(|s| s.parametric.clone())
                    .collect::<Vec<_>>()
            });
            let same = back.as_ref().is_ok_and(|b| {
                *b == layer
                    .shapes
                    .iter()
                    .map(|s| s.parametric.clone())
                    .collect::<Vec<_>>()
            });
            row(
                cid,
                "saved, then read again",
                same,
                "the same settings".into(),
                if same {
                    "the same settings"
                } else {
                    "different"
                }
                .into(),
            );
        }
    }

    out.push_str(&format!("\nB-609: {passed} of {checks} checks pass.\n"));
    fs::write(repo("verification/B-609_parametric_shapes_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-609_parametric_shapes_table.md"
    );
}
