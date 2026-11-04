//! B-966: D-1086, After Effects' Create Shapes from Text and Create Masks from Text: a text
//! layer's outlines on the frame, as a shape layer with one group per character or as a solid
//! with one mask per contour, in front of the text layer, which goes off.
//!
//! Every expected number is `Fixtures/text_outlines/expected_text_outlines.json`, written by
//! `tools/text_outlines_reference.py` (fontTools) before this code existed. The table is
//! `verification/B-966_text_outlines_table.md`.

mod effect_table;

use std::fs;

use effect_table::{repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::render_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::mask::{MaskMode, MaskPoint};
use anime_compositor::model::{Id, Layer, Project, Value};
use anime_compositor::{persist, text, WorkingBuffer};

fn expected() -> J {
    serde_json::from_str(&fs::read_to_string(repo("Fixtures/text_outlines/expected_text_outlines.json")).unwrap())
        .unwrap()
}

/// A 1280 by 720 composition of one text layer, `art`, with the case's animators.
fn project(case: &J) -> Project {
    let t = |v: J| json!({"base": v, "keyframes": []});
    let effects: Vec<J> = case["animators"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
        .map(|(i, a)| json!({"instance_id": format!("fx-{i}"), "type_id": "core.text_animator", "enabled": true, "parameters": a}))
        .collect();
    let layer = json!({"id": "art", "kind": "text", "name": "art", "enabled": true, "locked": false, "in_frame": 0,
        "out_frame": 4,
        "transform": {"anchor": t(json!([640, 360])), "position": t(json!([640, 360])), "scale": t(json!([100, 100])),
                      "rotation": t(json!(0)), "opacity": t(json!(1))},
        "matte": null, "blend_mode": "normal", "effects": effects, "masks": [], "source_text": case["text"]});
    let p = json!({
        "schema_version": 0, "project_id": "proj-b966",
        "color_settings": {"working_space": "linear-srgb", "alpha_mode": "premultiplied"}, "assets": [],
        "compositions": [{"id": MAIN, "name": MAIN, "width": 1280, "height": 720, "pixel_aspect_ratio": 1,
            "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 4,
            "work_area": {"start_frame": 0, "end_frame_exclusive": 4}, "layer_order": ["art"], "layers": [layer]}]
    });
    persist::load_str(&p.to_string())
        .unwrap_or_else(|d| panic!("the project reads: {}", d.message))
        .document
        .project()
        .clone()
}

fn node(m: &MaskPoint) -> [f64; 6] {
    [m.point.0, m.point.1, m.in_handle.0, m.in_handle.1, m.out_handle.0, m.out_handle.1]
}

/// The largest miss between a contour and the expected nodes, compared as closed loops from
/// whichever node; infinite when the counts differ.
fn loop_miss(built: &[MaskPoint], want: &J) -> f64 {
    let want: Vec<Vec<f64>> = want
        .as_array()
        .unwrap()
        .iter()
        .map(|n| n.as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect())
        .collect();
    if want.len() != built.len() {
        return f64::INFINITY;
    }
    (0..built.len())
        .map(|s| {
            built
                .iter()
                .enumerate()
                .map(|(i, b)| {
                    let w = &want[(i + s) % want.len()];
                    node(b).iter().zip(w).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max)
                })
                .fold(0.0, f64::max)
        })
        .fold(f64::INFINITY, f64::min)
}

fn render(project: &Project, frame: i32) -> WorkingBuffer {
    let mut log = FrameLog::new(8);
    render_frame(project, &Id::new(MAIN), frame, &repo("Fixtures/text_outlines"), 64, &mut log)
        .unwrap_or_else(|d| panic!("frame {frame} renders: {}", d.message))
}

/// The share of pixels whose coverage differs by more than a half, and the mean difference.
/// Each picture must have ink, so two empty frames never pass.
fn apart(a: &WorkingBuffer, b: &WorkingBuffer) -> (f64, f64) {
    let ink = |w: &WorkingBuffer| w.data().chunks_exact(4).filter(|p| p[3] > 0.5).count();
    if ink(a) < 500 || ink(b) < 500 {
        return (1.0, 1.0);
    }
    let (mut far, mut sum) = (0usize, 0.0);
    let n = a.data().len() / 4;
    for (p, q) in a.data().chunks_exact(4).zip(b.data().chunks_exact(4)) {
        let d = (p[3] - q[3]).abs() as f64;
        sum += d;
        if d > 0.5 {
            far += 1;
        }
    }
    (far as f64 / n as f64, sum / n as f64)
}

/// `project` with the command made on layer `art` at `frame`, as the app sends it: the new layer
/// in front, the text layer off, as one entry to undo.
fn made(project: &Project, frame: i32, masks: bool) -> Result<Document, String> {
    let comp = project.composition(&Id::new(MAIN)).unwrap();
    let layer = comp.layer(&Id::new("art")).unwrap();
    let new = text::outlines_layer(comp, layer, frame, Id::new("outlines"), masks)?;
    let at = comp.index_of(&layer.id).unwrap();
    let mut doc = Document::new(project.clone());
    doc.apply_all(vec![
        Command::AddLayer { composition: Id::new(MAIN), layer: Box::new(new), index: at + 1 },
        Command::SetLayerEnabled { composition: Id::new(MAIN), layer_id: Id::new("art"), value: false },
    ])
    .map_err(|d| d.message)?;
    Ok(doc)
}

fn outlines(doc: &Document) -> Layer {
    doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("outlines")).unwrap().clone()
}

#[test]
fn b966_text_outlines() {
    let e = expected();
    let tol = e["tolerance"].as_f64().unwrap();
    let mut t = Table::new(
        "text_outlines",
        "# B-966: Create Shapes from Text and Create Masks from Text (D-1086)\n\nWritten by \
         `tests/b966_text_outlines.rs`. Each case against `Fixtures/text_outlines/expected_text_outlines.json` \
         from `tools/text_outlines_reference.py`: every character's contours, point by point and \
         handle by handle, which are holes, and the shapes and masks the two commands make.\n",
    );
    t.heading("The outlines");
    for (case, c) in e["cases"].as_object().unwrap() {
        let p = project(c);
        let comp = p.composition(&Id::new(MAIN)).unwrap();
        let layer = comp.layer(&Id::new("art")).unwrap();
        let animators: Vec<_> = layer
            .effects
            .iter()
            .map(|i| anime_compositor::expr::effect_at(comp, &layer.id, i, 0, 0.0).0.effect)
            .collect();
        let built = text::char_outlines(layer.text.as_ref().unwrap(), &animators).expect("the bundled font");
        let want = c["chars"].as_array().unwrap();
        let says = c["says"].as_str().unwrap();
        let names = built.iter().map(|b| b.ch).collect::<String>();
        let same_chars = built.len() == want.len()
            && built.iter().zip(want).all(|(b, w)| {
                b.ch.to_string() == w["char"].as_str().unwrap() && b.index as u64 == w["index"].as_u64().unwrap()
            });
        t.row(&format!("{case}: {says} Characters with an outline"), &names, same_chars);
        let mut miss: f64 = 0.0;
        let mut holes_ok = true;
        let mut colour: f64 = 0.0;
        for (b, w) in built.iter().zip(want) {
            let wc = w["contours"].as_array().unwrap();
            if wc.len() != b.contours.len() {
                miss = f64::INFINITY;
                continue;
            }
            for ((hole, nodes), wn) in b.contours.iter().zip(wc) {
                miss = miss.max(loop_miss(nodes, &wn["points"]));
                holes_ok &= *hole == wn["hole"].as_bool().unwrap();
            }
            for k in 0..3 {
                colour = colour.max((b.color[k] - w["color"][k].as_f64().unwrap()).abs());
            }
        }
        let holes: usize = built.iter().map(|b| b.contours.iter().filter(|c| c.0).count()).sum();
        t.row(
            &format!("{case}: every contour's points and handles"),
            &format!("at most {miss:.2e} px out"),
            miss <= tol,
        );
        t.row(&format!("{case}: which contours are holes"), &format!("{holes} holes"), holes_ok);
        t.row(&format!("{case}: each character's colour"), &format!("at most {colour:.2e} out"), colour <= tol);

        // The masks Create Masks from Text makes, in order.
        let doc = made(&p, 0, true).unwrap();
        let m = outlines(&doc).masks;
        let wm = c["masks"].as_array().unwrap();
        let order = m.len() == wm.len()
            && m.iter().zip(wm).all(|(a, w)| {
                a.name == w["name"].as_str().unwrap()
                    && (a.mode == MaskMode::Subtract) == (w["mode"] == "subtract")
                    && a.mode != MaskMode::None
                    && loop_miss(&a.points, &w["points"]) <= tol
            });
        let list: Vec<String> = m
            .iter()
            .map(|a| format!("{} {}", a.name, if a.mode == MaskMode::Subtract { "subtract" } else { "add" }))
            .collect();
        t.row(&format!("{case}: Create Masks from Text's masks, names, modes and points in order"), &list.join(", "), order);

        // Create Shapes from Text: one group per character, its paths, a Merge Paths in Merge mode.
        let doc = made(&p, 0, false).unwrap();
        let s = outlines(&doc);
        let groups: Vec<&str> = s.shape_groups.iter().map(|g| g.name.as_str()).collect();
        let per: Vec<usize> =
            (0..s.shape_groups.len()).map(|g| s.shapes.iter().filter(|x| x.group == Some(g)).count()).collect();
        let want_per: Vec<usize> = want.iter().map(|w| w["contours"].as_array().unwrap().len()).collect();
        let merged = s.shape_groups.iter().all(|g| {
            g.operations.len() == 1 && g.operations[0].kind == "merge" && g.operations[0].choice.as_deref() == Some("merge")
        });
        let named = s.shapes.iter().all(|x| x.name.starts_with("Path ") && x.fill.is_some());
        let ok = groups == want.iter().map(|w| w["char"].as_str().unwrap()).collect::<Vec<_>>()
            && per == want_per
            && merged
            && named;
        t.row(
            &format!("{case}: Create Shapes from Text's groups and paths (each group a Merge Paths, Merge)"),
            &format!("{} groups, paths {:?}", groups.len(), per),
            ok,
        );
        // For the playtest's pictures: the three projects, when asked for.
        if let Ok(dir) = std::env::var("B966_SHEET_DIR") {
            let out = std::path::Path::new(&dir);
            let none = persist::Preserved::default();
            fs::write(out.join(format!("{case}_text.json")), persist::to_json(&p, &none)).unwrap();
            fs::write(out.join(format!("{case}_shapes.json")), persist::to_json(doc.project(), &none)).unwrap();
            let solid = made(&p, 0, true).unwrap();
            fs::write(out.join(format!("{case}_masks.json")), persist::to_json(solid.project(), &none)).unwrap();
        }
        // Drawn: the shapes and the masked solid cover what the words covered.
        let words = render(&p, 0);
        let (far, mean) = apart(&words, &render(doc.project(), 0));
        t.row(
            &format!("{case}: the shape layer draws where the words were drawn"),
            &format!("{:.3}% of pixels differ by more than half, mean {mean:.6}", far * 100.0),
            far < 0.001 && mean < 0.002,
        );
        let solid = made(&p, 0, true).unwrap();
        let (far, mean) = apart(&words, &render(solid.project(), 0));
        t.row(
            &format!("{case}: the masked solid draws where the words were drawn"),
            &format!("{:.3}% of pixels differ by more than half, mean {mean:.6}", far * 100.0),
            far < 0.001 && mean < 0.002,
        );
    }

    t.heading("The project");
    let x = &e["projects"]["FX-TXO-020"];
    let loaded = t.load("fx_txo_020.json");
    let p = loaded.document.project().clone();
    for masks in [false, true] {
        let what = if masks { "Create Masks from Text" } else { "Create Shapes from Text" };
        let doc = made(&p, 2, masks).unwrap();
        let comp = doc.project().composition(&Id::new(MAIN)).unwrap();
        let o = outlines(&doc);
        let order: Vec<&str> = comp.layer_order().iter().map(|i| i.as_str()).collect();
        let pos = o.transform.position.base();
        let rot = o.transform.rotation.base();
        let placed = o.name == x["name"].as_str().unwrap()
            && o.in_frame == 2
            && o.out_frame == 20
            && pos == Value::Vec2(700.0, 380.0)
            && rot == Value::Scalar(10.0)
            && order == ["art", "outlines"]
            && !comp.layer(&Id::new("art")).unwrap().enabled;
        t.row(
            &format!("fx_txo_020.json, {what}: \"{}\" in front, frames {} to {}, position {:?}, rotation {:?}; Title off", o.name, o.in_frame, o.out_frame, pos, rot),
            &format!("order (back to front) {order:?}"),
            placed,
        );
        if masks {
            let got: Vec<J> = o
                .masks
                .iter()
                .map(|m| json!({"name": m.name, "mode": if m.mode == MaskMode::Subtract { "subtract" } else { "add" }}))
                .collect();
            let colour = o.solid.map(|s| s.color.to_vec());
            let want_colour: Vec<f64> = x["color"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
            t.row(
                "fx_txo_020.json: the masks H add, i add, i add, O add, O subtract; the solid in the text's colour, the composition's size",
                &J::Array(got.clone()).to_string(),
                J::Array(got) == x["masks"] && colour == Some(want_colour) && o.solid.map(|s| (s.width, s.height)) == Some((1280, 720)),
            );
        } else {
            let groups: Vec<&str> = o.shape_groups.iter().map(|g| g.name.as_str()).collect();
            let per: Vec<u64> =
                (0..o.shape_groups.len()).map(|g| o.shapes.iter().filter(|s| s.group == Some(g)).count() as u64).collect();
            let fills = o.shapes.iter().all(|s| {
                s.fill.as_ref().map(|f| f.color.to_vec())
                    == Some(x["color"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect())
            });
            t.row(
                "fx_txo_020.json: groups H, i, O with 1, 2 and 2 paths, every one filled light blue",
                &format!("{groups:?} {per:?}"),
                J::from(groups.clone()) == x["groups"] && J::from(per) == x["paths"] && fills,
            );
        }
        // The frame looks the same, the text drawn by its outlines.
        let (far, mean) = apart(&render(&p, 2), &render(doc.project(), 2));
        t.row(
            &format!("fx_txo_020.json, {what}: frame 2 looks as it did"),
            &format!("{:.3}% of pixels differ by more than half, mean {mean:.6}", far * 100.0),
            far < 0.001 && mean < 0.002,
        );
        // One undo takes both back.
        let mut doc = doc;
        doc.undo();
        let back = doc.project() == &p;
        t.row(&format!("fx_txo_020.json, {what}: one undo puts it back as it was"), if back { "as it was" } else { "differs" }, back);
        // Saved and opened, the new layer holds what it held.
        let doc = made(&p, 2, masks).unwrap();
        let reopened = persist::load_str(&persist::to_json(doc.project(), &loaded.preserved))
            .unwrap_or_else(|d| panic!("the saved project opens: {}", d.message));
        let same = render(reopened.document.project(), 2).data() == render(doc.project(), 2).data();
        t.row(&format!("fx_txo_020.json, {what}: saved and opened again, it draws the same"), if same { "the same" } else { "differs" }, same);
    }

    t.heading("Refused");
    let spaces = t.load("fx_txo_021.json").document.project().clone();
    for masks in [false, true] {
        let what = if masks { "Create Masks from Text" } else { "Create Shapes from Text" };
        let said = made(&spaces, 2, masks).err().unwrap_or_default();
        t.row(&format!("fx_txo_021.json, {what}: words that are only spaces are refused"), &said, said.contains("no outline"));
    }
    let comp = spaces.composition(&Id::new(MAIN)).unwrap();
    let solid = Layer::solid(Id::new("s"), "Red Solid", anime_compositor::model::Solid { color: [1.0, 0.0, 0.0], width: 100, height: 100 }, 1280, 720, 0, 10);
    let said = text::outlines_layer(comp, &solid, 0, Id::new("x"), false).err().unwrap_or_default();
    t.row("A layer that is not text is refused", &said, said.contains("not a text layer"));
    t.finish("B-966_text_outlines_table.md");
}
