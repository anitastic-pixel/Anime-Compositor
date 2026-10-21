//! B-610: shape groups, against D-738 (b).
//!
//! Writes `verification/B-610_shape_groups_table.md`.
//!
//! # Where the expected values come from
//!
//! `Fixtures/shape_groups/`, written by `tools/shape_group_reference.py` before any code:
//! `host.json`, four shapes in three groups, one inside another; `expected_shape_groups.json`,
//! each case's shape as placed at a frame, what saving must write, and the refused files.
//!
//! # What is checked
//!
//! Each case's shape, placed by its groups at its frame, has the points, opacities, gradient
//! points and stroke width given, every number within 1e-9, and is drawn exactly as a plain
//! shape of that outline and paint is. Saved, every group, each shape's group and every path are
//! as read. Each refused file is refused with the diagnostic, naming the word.

use std::fs;

use serde_json::Value as J;

use anime_compositor::mask::MaskPoint;
use anime_compositor::model::{Property, Value};
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

fn pair(p: &Property) -> J {
    let (x, y) = p.base().as_vec2().unwrap();
    serde_json::json!([x, y])
}

/// What the build gives for each number the case names.
fn built(s: &Shape, wanted: &J) -> J {
    let mut out = serde_json::Map::new();
    for key in wanted.as_object().unwrap().keys() {
        let fill = s.fill.as_ref();
        let stroke = s.stroke.as_ref();
        let v = match key.as_str() {
            "points" => written(&s.points),
            "fill_opacity" => fill.map_or(J::Null, |f| f.opacity.into()),
            "fill_start" => fill
                .and_then(|f| f.gradient.as_ref())
                .map_or(J::Null, |g| pair(&g.start)),
            "fill_end" => fill
                .and_then(|f| f.gradient.as_ref())
                .map_or(J::Null, |g| pair(&g.end)),
            "stroke_opacity" => stroke.map_or(J::Null, |s| s.opacity.into()),
            "stroke_width" => stroke.map_or(J::Null, |s| s.width_px.into()),
            other => panic!("the fixture names {other}, which this test does not read"),
        };
        out.insert(key.clone(), v);
    }
    J::Object(out)
}

/// A shape in no group with the outline and paint the reference gives.
fn plain(s: &Shape, wanted: &J) -> Shape {
    let mut p = Shape {
        points: read_points(&wanted["points"]),
        group: None,
        ..s.clone()
    };
    let point =
        |v: &J| Property::constant(Value::Vec2(v[0].as_f64().unwrap(), v[1].as_f64().unwrap()));
    if let Some(f) = p.fill.as_mut() {
        f.opacity = wanted["fill_opacity"].as_f64().unwrap_or(f.opacity);
        if let Some(g) = f.gradient.as_mut() {
            g.start = point(&wanted["fill_start"]);
            g.end = point(&wanted["fill_end"]);
        }
    }
    if let Some(st) = p.stroke.as_mut() {
        st.opacity = wanted["stroke_opacity"].as_f64().unwrap_or(st.opacity);
        st.width_px = wanted["stroke_width"].as_f64().unwrap_or(st.width_px);
    }
    p
}

#[test]
fn b610_shape_groups() {
    let root = repo("Fixtures/shape_groups");
    let expected: J =
        serde_json::from_str(&fs::read_to_string(root.join("expected_shape_groups.json")).unwrap())
            .unwrap();
    let host = fs::read_to_string(root.join("host.json")).unwrap();
    let loaded = persist::load_str(&host).expect("the host reads");
    let project = loaded.document.project();
    let layer = project.compositions[0].layers_in_order().next().unwrap();
    let (width, height) = (400, 300);
    let mut out = String::from(
        "# B-610: shape groups\n\n\
         Written by `tests/b610_shape_groups.rs` from `Fixtures/shape_groups/`, against D-738 (b). \
         Each line is one check: what `tools/shape_group_reference.py` says, and what the build \
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
        let mut v = v.clone();
        if let Some(points) = v.get_mut("points") {
            let first = points[0]["point"].clone();
            *points = J::from(format!(
                "{} points, first {first}",
                points.as_array().unwrap().len()
            ));
        }
        v.to_string()
    };

    for (cid, c) in expected["cases"].as_object().unwrap() {
        if let Some(wanted) = c.get("placed") {
            let at = c["shape"].as_u64().unwrap() as usize;
            let frame = c["frame"].as_i64().unwrap() as i32;
            let s = &layer.shapes[at];
            let placed = s.placed(&layer.shape_groups, frame);
            let got = built(&placed, wanted);
            let ok = close(wanted, &got);
            row(
                cid,
                &format!("\"{}\" at frame {frame}: placed", s.name),
                ok,
                brief(wanted),
                if ok { brief(&got) } else { got.to_string() },
            );
            // Drawn as the plain shape of that outline and paint is: every channel of every pixel.
            let same = shape::draw(&[placed], width, height).data()
                == shape::draw(&[plain(&s.at(frame), wanted)], width, height).data();
            row(
                cid,
                "drawn as a plain shape of that outline and paint",
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
            let want = &c["saved"];
            let saved: J =
                serde_json::from_str(&persist::to_json(project, &loaded.preserved)).unwrap();
            let l = &saved["compositions"][0]["layers"][0];
            let ok = close(&l["groups"], &want["groups"]);
            row(
                cid,
                "the groups, blend included",
                ok,
                "as the reference".into(),
                if ok {
                    "as the reference".into()
                } else {
                    l["groups"].to_string()
                },
            );
            let shapes = l["shapes"].as_array().unwrap();
            let groups: Vec<J> = shapes
                .iter()
                .map(|s| s.get("group").cloned().unwrap_or(J::Null))
                .collect();
            let ok = J::Array(groups.clone()) == want["shape_groups"];
            row(
                cid,
                "each shape's group",
                ok,
                want["shape_groups"].to_string(),
                J::Array(groups).to_string(),
            );
            let paths: Vec<J> = shapes.iter().map(|s| s["path"].clone()).collect();
            let ok = close(&J::Array(paths), &want["paths"]);
            row(
                cid,
                "every path as read, not placed",
                ok,
                "as the reference".into(),
                if ok {
                    "as the reference".into()
                } else {
                    "different".into()
                },
            );
            // And the saved project reads back as the same groups.
            let back = persist::load_str(&saved.to_string()).map(|l| {
                let layer = l.document.project().compositions[0]
                    .layers_in_order()
                    .next()
                    .unwrap()
                    .clone();
                let groups: Vec<_> = layer
                    .shape_groups
                    .iter()
                    .map(|g| (g.name.clone(), g.parent, g.transform.clone()))
                    .collect();
                let shapes: Vec<_> = layer.shapes.iter().map(|s| s.group).collect();
                (groups, shapes)
            });
            let same = back.as_ref().is_ok_and(|(g, s)| {
                *g == layer
                    .shape_groups
                    .iter()
                    .map(|g| (g.name.clone(), g.parent, g.transform.clone()))
                    .collect::<Vec<_>>()
                    && *s == layer.shapes.iter().map(|s| s.group).collect::<Vec<_>>()
            });
            row(
                cid,
                "saved, then read again",
                same,
                "the same groups".into(),
                if same { "the same groups" } else { "different" }.into(),
            );
        }
    }

    out.push_str(&format!("\nB-610: {passed} of {checks} checks pass.\n"));
    fs::write(repo("verification/B-610_shape_groups_table.md"), &out).unwrap();
    assert_eq!(
        passed, checks,
        "see verification/B-610_shape_groups_table.md"
    );
}
