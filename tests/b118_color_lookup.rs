//! B-118: Color Lookup, against D-182.
//!
//! Writes `verification/B-118_color_lookup_table.md`.
//!
//! Every expected pixel, reason and manifest line is `Fixtures/cube_lut/expected_color_lookup.json`,
//! written by `tools/cube_lut_reference.py` before this code existed and printed in document 25 as
//! FX-LUT-001 to 013. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! It also draws frame 100 of the reference shot through three of the fixture's files, into
//! `verification/B-118 pictures/`, for the owner to see the effect on a real drawing.

mod effect_table;

use std::fs;

use effect_table::{largest_difference, saved, set, Table, MAIN};
use serde_json::Value as J;

use anime_compositor::command::Command;
use anime_compositor::compose::plan_frame;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::model::{Asset, AssetKind, Id};
use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{lut, package, persist, png_out, OutputDepth};

fn lookup(lut: &str) -> Effect {
    Effect::ColorLookup { lut: lut.to_string(), table: None }
}

/// What planning `frame` of `document` says, by id.
fn at_frame(t: &Table, document: &anime_compositor::command::Document, frame: i32) -> Vec<String> {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(document.project(), &Id::new(MAIN), frame, &t.root, &mut log);
    log.finish().iter().map(|d| d.id.as_str().to_string()).collect()
}

/// Frame 100 of the reference shot at Draft, as 8-bit bytes, with what its log said.
fn shot(project: &J) -> (usize, usize, Vec<u8>, Vec<String>) {
    let loaded = persist::load_str(&project.to_string()).expect("the reference shot reads");
    let mut log = FrameLog::new(3);
    let frame = preview::preview_frame_cached(
        loaded.document.project(),
        &Id::new("comp-reference-shot"),
        100,
        &effect_table::repo("Fixtures/reference_shot"),
        PreviewQuality::Draft,
        DEFAULT_TILE_SIZE,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("frame 100 draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.width(), frame.height(), frame.to_srgb8_straight(), said)
}

/// A package's manifest, the statuses of its files by path, and the asset with `id`.
fn manifest(dest: &std::path::Path) -> (J, Vec<(String, String)>) {
    let m: J = serde_json::from_str(&fs::read_to_string(dest.join(package::MANIFEST)).unwrap()).unwrap();
    let files = m["assets"]
        .as_array()
        .unwrap()
        .iter()
        .flat_map(|a| a["files"].as_array().unwrap().iter())
        .map(|f| (f["path"].as_str().unwrap().to_string(), f["status"].as_str().unwrap().to_string()))
        .collect();
    (m, files)
}

#[test]
fn b118_color_lookup() {
    let mut t = Table::new(
        "cube_lut",
        "# B-118: Color Lookup\n\nD-182, accepted on 2026-09-28 by the owner's words \"3D \
         trilinear, 1D if cheap. Collect/package copies the file; a missing .cube is diagnosed \
         and kept.\" Every expected pixel, refusal and manifest line is \
         `Fixtures/cube_lut/expected_color_lookup.json`, written by \
         `tools/cube_lut_reference.py` before this code existed and printed in document 25 as \
         FX-LUT-001 to 013. The build's frame is compared sample by sample; the answer is the \
         largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );
    let expected: J = serde_json::from_str(
        &fs::read_to_string(t.root.join("expected_color_lookup.json")).unwrap(),
    )
    .unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    t.heading("FX-LUT-001 to 012 (document 25)");
    t.fixtures("expected_color_lookup.json");

    t.heading("FX-LUT-013: a lookup file the reading rule refuses");
    let u = &expected["unreadable"];
    let loaded = t.load(u["project"].as_str().unwrap());
    let said: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
    t.row(
        "opening it says nothing: the file is read at the frame, not on opening",
        &format!("{said:?}"),
        said.is_empty(),
    );
    for (frame, pixels) in u["frames"].as_object().unwrap() {
        let frame: i32 = frame.parse().unwrap();
        let d = largest_difference(&t.render(&loaded.document, frame, 64), pixels);
        t.row(
            &format!("frame {frame}: the drawing, untouched"),
            &format!("largest difference {d:.1e}"),
            d <= tolerance,
        );
        let said = at_frame(&t, &loaded.document, frame);
        t.row(
            &format!("frame {frame} says {}, once", u["at_each_frame"].as_str().unwrap()),
            &format!("{said:?}"),
            said == [u["at_each_frame"].as_str().unwrap()],
        );
    }
    let kept = saved(&loaded);
    t.row(
        "the asset and the setting are kept as written",
        &format!("{} and {}", kept["assets"][1]["path"], kept["compositions"][0]["layers"][0]["effects"][0]["parameters"]["lut"]),
        kept["assets"][1]["path"] == "luts/refused/count.cube"
            && kept["compositions"][0]["layers"][0]["effects"][0]["parameters"]["lut"] == "asset-lut",
    );

    t.heading("The reading rule: each refused file, and its reason word for word");
    for (name, reason) in expected["refused"].as_object().unwrap() {
        let bytes = fs::read(t.root.join("luts").join(name)).unwrap();
        let got = lut::parse(&bytes).err();
        t.row(
            &format!("{name} is refused: {}", reason.as_str().unwrap()),
            got.as_deref().unwrap_or("read"),
            got.as_deref() == reason.as_str(),
        );
    }
    let changing = std::env::temp_dir().join("b118_changing.cube");
    fs::write(&changing, "LUT_1D_SIZE 2\n0 0 0\n1 1 1\n").unwrap();
    let first = lut::read(&changing);
    fs::write(&changing, "LUT_1D_SIZE 2\n0 0 0\n1 1 1\n1 1 1\n").unwrap();
    let second = lut::read(&changing);
    t.row(
        "a lookup file changed on disk is read afresh, not kept from before",
        &format!("{:?}, then {:?}", first.as_ref().map(|_| "read"), second.as_ref().map(|_| "read")),
        first.is_ok() && second.is_err(),
    );
    let _ = fs::remove_file(&changing);

    t.heading("How far it reaches");
    let mut draft = lookup("asset-lut");
    let reach = draft.bounds_expansion();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "it grows the drawing's bounds by nothing, and a half-size draft leaves it as it is",
        &format!("{reach}, {draft:?}"),
        reach == 0 && draft == lookup("asset-lut"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=13).map(|n| format!("fx_lut_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let mut json: J =
        serde_json::from_str(&fs::read_to_string(t.root.join("fx_lut_002.json")).unwrap()).unwrap();
    json["compositions"][0]["layers"][0]["asset_id"] = J::from("asset-lut");
    let path = std::env::temp_dir().join("b118_layer_shows_lut.json");
    fs::write(&path, json.to_string()).unwrap();
    let refused = persist::load(&path).err();
    t.row(
        "a file whose layer shows the lookup file as its drawing is refused as a fault in its shape",
        &refused.as_ref().map_or("opened".to_string(), |d| d.message.clone()),
        refused.is_some_and(|d| d.id.as_str() == "PROJECT_SCHEMA_INVALID"),
    );
    let _ = fs::remove_file(&path);

    t.heading("Commands");
    let mut document = t.load("fx_lut_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("naming asset-nothing, which the project does not have,", set(lookup("asset-nothing"))),
            ("naming asset-bands, a drawing and not a lookup file,", set(lookup("asset-bands"))),
        ],
    );
    let comp = document.project().composition(&Id::new(MAIN)).unwrap();
    let mut layer = comp.layer(&Id::new("art")).unwrap().clone();
    layer.id = Id::new("shows-lut");
    layer.asset_id = Id::new("asset-lut");
    layer.effects.clear();
    let said = document
        .apply(Command::AddLayer { composition: Id::new(MAIN), layer: Box::new(layer), index: 0 })
        .err();
    t.row(
        "a layer showing the lookup file as its drawing is refused with a sentence",
        &said.as_ref().map_or("taken".to_string(), |d| d.message.clone()),
        said.is_some(),
    );
    t.taken(
        &mut document,
        "fx_lut_002.json",
        vec![("no file chosen,", set(lookup("")))],
    );

    t.heading("Choosing a file on the card");
    let cases = expected["cases"].as_object().unwrap();
    let mut document = t.load("fx_lut_007.json").document;
    let untouched = t.render(&document, 0, 64);
    document.apply(set(lookup("asset-lut"))).unwrap();
    let d = largest_difference(&t.render(&document, 0, 64), &cases["FX-LUT-002"]["frames"]["0"]);
    t.row(
        "FX-LUT-007 with warm_17.cube chosen is FX-LUT-002's frame",
        &format!("largest difference {d:.1e}"),
        d <= tolerance,
    );
    document.undo();
    let mut cool = Asset::still(Id::new("asset-cool"), "cool_3", "luts/cool_3.cube");
    cool.kind = AssetKind::Lut;
    let took = document.apply_all(vec![
        Command::AddAsset { asset: cool },
        set(lookup("asset-cool")),
    ])
    .is_ok();
    let d = largest_difference(&t.render(&document, 0, 64), &cases["FX-LUT-003"]["frames"]["0"]);
    t.row(
        "choosing cool_3.cube, a file the project does not have yet, brings it in and uses it: \
         FX-LUT-003's frame",
        &format!("{}, largest difference {d:.1e}", if took { "taken" } else { "refused" }),
        took && d <= tolerance,
    );
    document.undo();
    let back = t.render(&document, 0, 64).data() == untouched.data()
        && document.project().assets.len() == 2;
    t.row(
        "one undo takes back both the new file and the setting",
        if back { "the frame and the asset list as they were" } else { "differ" },
        back,
    );

    t.heading("Collect Files and Check Package");
    let c = &expected["collect"];
    let dest = std::env::temp_dir().join("b118_collect");
    let _ = fs::remove_dir_all(&dest);
    let loaded = t.load(c["project"].as_str().unwrap());
    let done = package::collect(
        loaded.document.project(),
        &loaded.preserved,
        Some(&t.root.join(c["project"].as_str().unwrap())),
        &dest,
    );
    let (m, files) = manifest(&dest);
    let want: Vec<(String, String)> = c["copied"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(p, s)| (p.clone(), s.as_str().unwrap().to_string()))
        .collect();
    let mut got = files.clone();
    got.sort();
    t.row(
        "FX-LUT-002 collected: the drawing and the lookup file are both copied",
        &format!("{got:?}"),
        done.is_ok() && got == want,
    );
    let lut_asset = m["assets"].as_array().unwrap().iter().find(|a| a["id"] == "asset-lut");
    t.row(
        "the manifest lists the lookup file as kind lut, used by the layer whose effect names it",
        &format!("{:?}", lut_asset.map(|a| (&a["kind"], &a["used_by"]))),
        lut_asset.is_some_and(|a| a["kind"] == c["kind"] && a["used_by"] == c["used_by"]),
    );
    let collected = persist::load(&dest.join("fx_lut_002.json"));
    let names = collected.as_ref().ok().map(|l| saved(l)["assets"][1]["path"].clone());
    let same = collected.as_ref().is_ok_and(|l| {
        let mut log = FrameLog::new(8);
        anime_compositor::compose::render_frame(l.document.project(), &Id::new(MAIN), 0, &dest, 64, &mut log)
            .is_ok_and(|f| f.data() == t.render(&loaded.document, 0, 64).data())
    });
    t.row(
        "the collected project names the copy, and draws the same frame from it",
        &format!("{names:?}, {}", if same { "the same frame" } else { "differs" }),
        names == Some(J::from("media/asset-lut/warm_17.cube")) && same,
    );
    let rows = package::check(&dest.join("fx_lut_002.json"));
    let answers: Vec<String> = rows
        .as_ref()
        .map(|r| r.iter().map(|r| format!("{} {}", r.path, r.answer.as_str())).collect())
        .unwrap_or_default();
    t.row(
        "Check Package finds the lookup file and the drawing whole",
        &format!("{answers:?}"),
        rows.is_ok_and(|r| r.len() == 2 && r.iter().all(|r| r.answer == package::Answer::Ok)),
    );
    let _ = fs::remove_dir_all(&dest);
    let loaded = t.load(c["missing_project"].as_str().unwrap());
    let done = package::collect(
        loaded.document.project(),
        &loaded.preserved,
        Some(&t.root.join(c["missing_project"].as_str().unwrap())),
        &dest,
    );
    let (m, files) = manifest(&dest);
    let (path, status) = c["missing"].as_object().unwrap().iter().next().unwrap();
    t.row(
        &format!("FX-LUT-009 collected: {path} is listed as {}, and the asset is kept", status.as_str().unwrap()),
        &format!("{files:?}"),
        done.is_ok()
            && files.contains(&(path.clone(), status.as_str().unwrap().to_string()))
            && m["assets"].as_array().unwrap().iter().any(|a| a["id"] == "asset-lut"),
    );
    let _ = fs::remove_dir_all(&dest);

    t.heading("Pictures: frame 100 of the reference shot, in `verification/B-118 pictures/`");
    let pictures = effect_table::repo("verification/B-118 pictures");
    fs::create_dir_all(&pictures).unwrap();
    let base: J = serde_json::from_str(
        &fs::read_to_string(effect_table::repo("verification/B-08a_project.json")).unwrap(),
    )
    .unwrap();
    let (w, h, before, said) = shot(&base);
    png_out::write_rgba(&pictures.join("before.png"), w, h, OutputDepth::Eight, &[], &before).unwrap();
    t.row("before.png, the shot with no lookup, draws cleanly", &format!("{said:?}"), said.is_empty());
    for name in ["warm_17", "cool_3", "tint_1d"] {
        // On an adjustment layer above all four, as a grade is used; layer-4 made into one.
        let mut project = base.clone();
        project["assets"].as_array_mut().unwrap().push(serde_json::json!({
            "id": "asset-look", "kind": "lut", "name": name,
            "path": format!("../cube_lut/luts/{name}.cube"),
        }));
        let comp = &mut project["compositions"][0];
        let mut grade =
            comp["layers"].as_array().unwrap().iter().find(|l| l["id"] == "layer-4").unwrap().clone();
        let fields = grade.as_object_mut().unwrap();
        for k in ["asset_id", "exposure_spans", "source_offset_frames"] {
            fields.remove(k);
        }
        fields.insert("id".into(), J::from("layer-grade"));
        fields.insert("name".into(), J::from("grade"));
        fields.insert("kind".into(), J::from("adjustment"));
        fields.insert(
            "effects".into(),
            serde_json::json!([{
                "instance_id": "fx-grade", "type_id": "core.color_lookup", "enabled": true,
                "parameters": { "lut": "asset-look" },
            }]),
        );
        comp["layers"].as_array_mut().unwrap().push(grade);
        comp["layer_order"].as_array_mut().unwrap().push(J::from("layer-grade"));
        let (w, h, bytes, said) = shot(&project);
        png_out::write_rgba(&pictures.join(format!("{name}.png")), w, h, OutputDepth::Eight, &[], &bytes)
            .unwrap();
        let changed = bytes.chunks(4).zip(before.chunks(4)).filter(|(a, b)| a != b).count();
        t.row(
            &format!(
                "{name}.png: {name}.cube on an adjustment layer above the shot draws cleanly and \
                 changes the picture"
            ),
            &format!("{said:?}, {changed} of {} pixels changed", w * h),
            said.is_empty() && changed > 0,
        );
    }

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_lut_002.json", 0), ("fx_lut_004.json", 0), ("fx_lut_012.json", 0)]);

    t.finish("B-118_color_lookup_table.md");
}
