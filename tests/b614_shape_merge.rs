//! B-614: Merge Paths, against D-738 (d).
//!
//! Writes `verification/B-614_shape_merge_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/shape_merge/`, written by `tools/shape_merge_reference.py` before any code:
//! `host.json`, sixteen shapes in nine groups merging by each of the five modes, one group's
//! repeated copies merged by the group outside it; `expected_shape_merge.json`, every shape drawn
//! at each case's frame in drawing order, what saving must write, and the refused files.
//!
//! # What is checked
//!
//! At each case's frame the layer draws the shapes given, in the order given, each with the
//! outlines, opacities and stroke width given, every number within 1e-9, and draws them exactly
//! as shapes of those outlines and paint are drawn. Saved, every group and its operations are as
//! read, the fields this build does not know included. Each refused file is refused with the
//! diagnostic, naming the word.

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

/// What the build drew, by the names the fixture gives, `copies` taken from the fixture since a
/// drawn shape does not carry it.
fn built(s: &Shape, wanted: &J) -> J {
    let mut out = serde_json::Map::new();
    out.insert("shape".into(), J::from(s.name.as_str()));
    out.insert("copies".into(), wanted["copies"].clone());
    let paths: Vec<(&Vec<MaskPoint>, bool)> = match &s.outlines {
        Some(o) => o.iter().map(|(p, closed)| (p, *closed)).collect(),
        None => vec![(&s.points, s.closed)],
    };
    let paths = paths
        .into_iter()
        .map(|(p, closed)| serde_json::json!({"closed": closed, "points": written(p)}));
    out.insert("paths".into(), J::Array(paths.collect()));
    if let Some(f) = &s.fill {
        out.insert("fill_opacity".into(), f.opacity.into());
    }
    if let Some(st) = &s.stroke {
        out.insert("stroke_opacity".into(), st.opacity.into());
        out.insert("stroke_width".into(), st.width_px.into());
    }
    J::Object(out)
}

/// The shape of those outlines and paint, in no group: a merged one drawn from its outlines.
fn plain(s: &Shape, wanted: &J) -> Shape {
    let paths: Vec<(Vec<MaskPoint>, bool)> = wanted["paths"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| (read_points(&p["points"]), p["closed"].as_bool().unwrap()))
        .collect();
    let mut p = Shape {
        group: None,
        ..s.clone()
    };
    if s.outlines.is_some() {
        p.outlines = Some(paths);
    } else {
        (p.points, p.closed) = paths.into_iter().next().unwrap();
    }
    if let Some(f) = p.fill.as_mut() {
        f.opacity = wanted["fill_opacity"].as_f64().unwrap();
    }
    if let Some(st) = p.stroke.as_mut() {
        st.opacity = wanted["stroke_opacity"].as_f64().unwrap();
        st.width_px = wanted["stroke_width"].as_f64().unwrap();
    }
    p
}

#[test]
fn b614_shape_merge() {
    let root = repo("Fixtures/shape_merge");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_shape_merge.json")).unwrap())
            .unwrap();
    let host = fs::read_to_string(root.join("host.json")).unwrap();
    let loaded = persist::load_str(&host).expect("the host reads");
    let project = loaded.document.project();
    let layer = project.compositions[0].layers_in_order().next().unwrap();
    let (width, height) = (600, 400);
    let mut out = String::from(
        "# B-614: Merge Paths\n\n\
         Written by `tests/b614_shape_merge.rs` from `Fixtures/shape_merge/`, against D-738 (d). \
         Each line is one check: what `tools/shape_merge_reference.py` says, and what the build \
         gives.\n\n\
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
    let brief = |v: &J| {
        let outlines = v["paths"].as_array().map_or(0, Vec::len);
        let first = &v["paths"][0]["points"][0]["point"];
        let fill = v.get("fill_opacity").map_or(String::new(), |o| {
            format!(
                ", fill opacity {}",
                (o.as_f64().unwrap() * 1e6).round() / 1e6
            )
        });
        format!(
            "{} {}: {outlines} outlines, first point {first}{fill}",
            v["shape"], v["copies"]
        )
    };

    for (cid, c) in expected["cases"].as_object().unwrap() {
        if let Some(wanted) = c.get("drawn").and_then(J::as_array) {
            let frame = c["frame"].as_i64().unwrap() as i32;
            let drawn = shape::drawn(&layer.shapes, &layer.shape_groups, frame, 24.0);
            row(
                cid,
                &format!("frame {frame}: how many shapes are drawn"),
                drawn.len() == wanted.len(),
                wanted.len().to_string(),
                drawn.len().to_string(),
            );
            for (n, (s, w)) in drawn.iter().zip(wanted).enumerate() {
                let got = built(s, w);
                let ok = close(w, &got);
                row(
                    cid,
                    &format!("drawn {}", n + 1),
                    ok,
                    brief(w),
                    if ok { brief(&got) } else { got.to_string() },
                );
            }
            // Drawn as shapes of those outlines and paint are: every channel of every pixel.
            let plains: Vec<Shape> = drawn.iter().zip(wanted).map(|(s, w)| plain(s, w)).collect();
            let same = shape::draw(&drawn, width, height).data()
                == shape::draw(&plains, width, height).data();
            row(
                cid,
                "drawn as shapes of those outlines and paint",
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
            let groups = &saved["compositions"][0]["layers"][0]["groups"];
            let ok = close(groups, &c["saved"]["groups"]);
            row(
                cid,
                "the groups and their merges, note and blend included",
                ok,
                "as the reference".into(),
                if ok {
                    "as the reference".into()
                } else {
                    groups.to_string()
                },
            );
            // And the saved project reads back as the same operations.
            let back = persist::load_str(&saved.to_string()).map(|l| {
                l.document.project().compositions[0]
                    .layers_in_order()
                    .next()
                    .unwrap()
                    .shape_groups
                    .iter()
                    .map(|g| g.operations.clone())
                    .collect::<Vec<_>>()
            });
            let now: Vec<_> = layer
                .shape_groups
                .iter()
                .map(|g| g.operations.clone())
                .collect();
            let same = back.as_ref().is_ok_and(|b| *b == now);
            row(
                cid,
                "saved, then read again",
                same,
                "the same operations".into(),
                if same {
                    "the same operations"
                } else {
                    "different"
                }
                .into(),
            );
        }
    }

    out.push_str(&format!("\nB-614: {passed} of {checks} checks pass.\n"));
    fs::write(repo("verification/B-614_shape_merge_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-614_shape_merge_table.md"
    );
}
