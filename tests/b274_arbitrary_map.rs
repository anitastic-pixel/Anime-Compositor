//! B-274: D-395 Arbitrary Map, after After Effects' PS Arbitrary Map ("Color Correction" in
//! `docs/effects/EFFECTS.md`): a Photoshop arbitrary map (.amp) read as Adobe's file format
//! specification gives it, its colour tables applied by Color Lookup's 1D rule, Phase cycling
//! every table, and the alpha table, when asked, as Curves' alpha curve (D-302).
//!
//! Writes `verification/D-395_arbitrary_map_table.md`.
//!
//! Every expected pixel and reason is `Fixtures/arbitrary_map/expected_arbitrary_map.json`,
//! written by `tools/arbitrary_map_reference.py` before this code existed and printed in
//! document 25 as FX-AMAP-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws the street into `verification/D-395 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, largest_difference, repo, saved, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::Command;
use anime_compositor::compose::{self, plan_frame, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Asset, AssetKind, Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{lut, package, persist, png_out, render, OutputDepth};

fn map(map: &str, phase: f64, apply_to_alpha: &str) -> Effect {
    Effect::ArbitraryMap { map: map.to_string(), phase, apply_to_alpha: apply_to_alpha.to_string(), table: None }
}

fn is_map(e: &Effect) -> bool {
    matches!(e, Effect::ArbitraryMap { .. })
}

fn distance(a: &[u8], b: &[u8]) -> (u8, usize) {
    assert_eq!(a.len(), b.len(), "the two pictures are different sizes");
    let (mut largest, mut pixels) = (0, 0);
    for (p, q) in a.chunks_exact(4).zip(b.chunks_exact(4)) {
        if p[3] == 0 && q[3] == 0 {
            continue;
        }
        let d = p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
        largest = largest.max(d);
        pixels += (d > 0) as usize;
    }
    (largest, pixels)
}

fn said(log: FrameLog) -> String {
    let mut ids: Vec<&str> = log.finish().iter().map(|d| d.id.as_str()).collect();
    ids.sort();
    ids.dedup();
    ids.join(", ")
}

/// What planning `frame` of `document` says, by id.
fn at_frame(t: &Table, document: &anime_compositor::command::Document, frame: i32) -> Vec<String> {
    let mut log = FrameLog::new(8);
    let _ = plan_frame(document.project(), &Id::new(MAIN), frame, &t.root, &mut log);
    log.finish().iter().map(|d| d.id.as_str().to_string()).collect()
}

/// The effects `pick` takes that the card's plan leaves to the card, over every layer.
fn on_card(project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality, pick: fn(&Effect) -> bool) -> usize {
    let mut log = FrameLog::new(3);
    let plan = compose::plan_frame_for_card(project, comp, frame, root, quality, &mut log, &mut CelCache::viewer()).expect("plan the frame");
    plan.layers
        .iter()
        .flat_map(|l| &l.on_card)
        .filter(|c| matches!(c.unmixed(), render::OnCard::Fx(f) if pick(&f.instance.effect)))
        .count()
}

/// The processor and the card, each drawing the frame the page receives: the largest
/// difference, the pixels differing, whether the card refused the frame, and each one's warnings.
fn both(gpu: &mut Gpu, project: &Project, comp: &Id, root: &Path, frame: i32, quality: PreviewQuality) -> ((u8, usize), bool, String, String) {
    let mut cache = CelCache::viewer();
    let mut log = FrameLog::new(3);
    let c = preview::preview_frame_cached(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache)
        .unwrap_or_else(|d| panic!("frame {frame} on the CPU: {}", d.message));
    let said_cpu = said(log);
    let mut log = FrameLog::new(3);
    let (g, ..) = preview::preview_frame_srgb8(project, comp, frame, root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu)
        .unwrap_or_else(|d| panic!("frame {frame} on the GPU: {}", d.message));
    let said_gpu = said(log);
    let refused = said_gpu.contains(DiagnosticId::GpuPreviewOnCpu.as_str());
    (distance(&c.to_srgb8_straight(), &g), refused, said_cpu, said_gpu)
}

/// An Arbitrary Map with the settings `p`, every one left out as a new one takes it.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.arbitrary_map" => json!({"map": "asset-map", "phase": 0, "apply_to_alpha": "off"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// The map file `name` of the fixtures as an asset `asset-map`, its path seen from `from`.
fn map_asset(name: &str, from: &str) -> J {
    json!({"id": "asset-map", "kind": "lut", "name": name, "path": format!("{from}Fixtures/arbitrary_map/maps/{name}")})
}

/// The reference shot (1920 by 1080) with `stack` on its first three layers and the map `file`.
fn reference(file: &str, stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    j["assets"].as_array_mut().unwrap().push(map_asset(file, "../../"));
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing, are left out with a warning, or are the processor's.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, stem: &str, count: u32, none: &[u32], pick: fn(&Effect) -> bool) {
    let comp = Id::new(MAIN);
    for n in 1..=count {
        let file = format!("{stem}_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, pick);
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each map and setting on its first three layers, frames 0, 100 and
/// 239 at Full and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, &str, J)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference("straight_256.amp", |_| json!([])));
    for (what, file, p) in settings {
        let project = reference(file, |id| json!([fx("core.arbitrary_map", &format!("b274-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, is_map);
                t.row(
                    &format!("the reference shot, {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// `effects` on the street with the map `file`, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, file: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    project["assets"].as_array_mut().unwrap().push(map_asset(file, "../../"));
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

fn why(t: &mut Table, cases: &[(&str, &str)]) {
    for (file, want) in cases {
        let doc = t.load(file).document;
        let why = doc.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == *want);
    }
}

/// A package's manifest, the statuses of its files by path.
fn manifest(dest: &Path) -> (J, Vec<(String, String)>) {
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
fn b274_arbitrary_map() {
    let mut t = Table::new(
        "arbitrary_map",
        "# D-395: Arbitrary Map\n\nB-274, after After Effects' PS Arbitrary Map: a Photoshop \
         arbitrary map (.amp) read as Adobe's file format specification gives it, the colour \
         tables applied by Color Lookup's 1D rule (each channel's table, then the master's), Phase \
         cycling every table, and the alpha table, with Apply Phase Map To Alpha on, as Curves' \
         alpha curve (D-302). Every expected pixel and reason is \
         `Fixtures/arbitrary_map/expected_arbitrary_map.json`, written by \
         `tools/arbitrary_map_reference.py` before this code existed and printed in document 25 \
         as FX-AMAP-001 to 022. Tolerance 2e-5.\n",
    );
    let expected: J = serde_json::from_str(&fs::read_to_string(t.root.join("expected_arbitrary_map.json")).unwrap()).unwrap();
    let tolerance = expected["tolerance"].as_f64().unwrap();

    t.heading("FX-AMAP-001 to 021 (document 25)");
    t.fixtures("expected_arbitrary_map.json");

    t.heading("FX-AMAP-022: a map file the reading rule refuses");
    let u = &expected["unreadable"];
    let loaded = t.load(u["project"].as_str().unwrap());
    let opened: Vec<&str> = loaded.warnings.iter().map(|d| d.id.as_str()).collect();
    t.row("opening it says nothing: the file is read at the frame, not on opening", &format!("{opened:?}"), opened.is_empty());
    for (frame, pixels) in u["frames"].as_object().unwrap() {
        let frame: i32 = frame.parse().unwrap();
        let d = largest_difference(&t.render(&loaded.document, frame, 64), pixels);
        t.row(&format!("frame {frame}: the drawing, untouched"), &format!("largest difference {d:.1e}"), d <= tolerance);
        let said = at_frame(&t, &loaded.document, frame);
        let want = u["at_each_frame"].as_str().unwrap();
        t.row(&format!("frame {frame} says {want}, once"), &format!("{said:?}"), said == [want]);
    }
    let mut log = FrameLog::new(8);
    let _ = plan_frame(loaded.document.project(), &Id::new(MAIN), 0, &t.root, &mut log);
    let words: Vec<String> = log.finish().iter().map(|d| format!("{} {}", d.message, d.remediation.clone().unwrap_or_default())).collect();
    t.row(
        "it names the file and why, and asks for a .amp file",
        &format!("{words:?}"),
        words.len() == 1 && words[0].contains("odd_300") && words[0].contains(".amp file"),
    );
    let kept = saved(&loaded);
    let effect = &kept["compositions"][0]["layers"][0]["effects"][0]["parameters"];
    t.row(
        "the asset and the setting are kept as written",
        &format!("{} and {}", kept["assets"][1]["path"], effect["map"]),
        kept["assets"][1]["path"] == "maps/refused/odd_300.amp" && effect["map"] == "asset-map",
    );

    t.heading("The reading rule: each refused file, and its reason word for word");
    for (name, reason) in expected["refused"].as_object().unwrap() {
        let bytes = fs::read(t.root.join("maps").join(name)).unwrap();
        let got = lut::parse_amp(&bytes).err();
        t.row(&format!("{name} is refused: {}", reason.as_str().unwrap()), got.as_deref().unwrap_or("read"), got.as_deref() == reason.as_str());
    }
    let changing = std::env::temp_dir().join(format!("b274_changing_{}.amp", std::process::id()));
    fs::write(&changing, [7u8; 256]).unwrap();
    let first = lut::read_map(&changing);
    fs::write(&changing, [7u8; 300]).unwrap();
    let second = lut::read_map(&changing);
    t.row(
        "a map file changed on disk is read afresh, not kept from before",
        &format!("{:?}, then {:?}", first.as_ref().map(|_| "read"), second.as_ref().map(|_| "read")),
        first.is_ok() && second.is_err(),
    );
    let _ = fs::remove_file(&changing);
    let cube = fs::read(repo("Fixtures/cube_lut/luts/tint_1d.cube")).unwrap();
    t.row(
        "a .cube file is not read as a map, nor a map as a .cube file",
        "",
        lut::parse_amp(&cube).is_err() && lut::parse(&fs::read(t.root.join("maps/master_256.amp")).unwrap()).is_err(),
    );

    t.heading("How far it reaches");
    let mut draft = map("asset-map", 64.0, "on");
    let reach = draft.bounds_expansion();
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "it grows the drawing's bounds by nothing, and a half-size draft leaves it as it is",
        &format!("{reach}, {draft:?}"),
        reach == 0 && draft == map("asset-map", 64.0, "on"),
    );

    t.heading("The file");
    let files: Vec<String> = (1..=22).map(|n| format!("fx_amap_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let p = t.saved_parameters("fx_amap_012.json");
    t.row(
        "fx_amap_012.json is saved with its map, phase and alpha setting",
        &p.to_string(),
        p["map"] == "asset-map" && p["phase"] == 100 && p["apply_to_alpha"] == "on",
    );
    why(
        &mut t,
        &[
            ("fx_amap_018.json", "Arbitrary Map's phase runs from -255 to 255, and this is 256."),
            ("fx_amap_019.json", "Arbitrary Map's phase runs from -255 to 255, and this is -256."),
            ("fx_amap_021.json", "Arbitrary Map's apply phase map to alpha is \"off\" or \"on\", and this is \"yes\"."),
        ],
    );
    t.shape_refused("fx_amap_001.json", "an Arbitrary Map with no `apply_to_alpha`", r#"{"map": "asset-map", "phase": 0}"#);
    t.shape_refused("fx_amap_001.json", "an Arbitrary Map with no `map`", r#"{"phase": 0, "apply_to_alpha": "off"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_amap_002.json").document;
    t.refused(
        &mut document,
        vec![
            ("naming asset-nothing, which the project does not have,", set(map("asset-nothing", 0.0, "off"))),
            ("naming asset-bands, a drawing and not a map file,", set(map("asset-bands", 0.0, "off"))),
            ("phase 255.5", set(map("asset-map", 255.5, "off"))),
            ("apply phase map to alpha \"yes\"", set(map("asset-map", 0.0, "yes"))),
            ("phase keyed to -300", keys("phase", &[(0, &[0.0]), (4, &[-300.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_amap_002.json",
        vec![
            ("no file chosen,", set(map("", 0.0, "off"))),
            ("phase -255, alpha on", set(map("asset-map", -255.0, "on"))),
            ("phase keyed from 0 to 255", keys("phase", &[(0, &[0.0]), (4, &[255.0])])),
        ],
    );

    t.heading("Choosing a file on the card");
    let cases = expected["cases"].as_object().unwrap();
    let mut document = t.load("fx_amap_013.json").document;
    let untouched = t.render(&document, 0, 64);
    document.apply(set(map("asset-map", 0.0, "off"))).unwrap();
    let d = largest_difference(&t.render(&document, 0, 64), &cases["FX-AMAP-002"]["frames"]["0"]);
    t.row("FX-AMAP-013 with master_256.amp chosen is FX-AMAP-002's frame", &format!("largest difference {d:.1e}"), d <= tolerance);
    document.undo();
    let mut rgb = Asset::still(Id::new("asset-rgb"), "rgb_768", "maps/rgb_768.amp");
    rgb.kind = AssetKind::Lut;
    let took = document.apply_all(vec![Command::AddAsset { asset: rgb }, set(map("asset-rgb", 0.0, "off"))]).is_ok();
    let d = largest_difference(&t.render(&document, 0, 64), &cases["FX-AMAP-003"]["frames"]["0"]);
    t.row(
        "choosing rgb_768.amp, a file the project does not have yet, brings it in and uses it: FX-AMAP-003's frame",
        &format!("{}, largest difference {d:.1e}", if took { "taken" } else { "refused" }),
        took && d <= tolerance,
    );
    document.undo();
    let back = t.render(&document, 0, 64).data() == untouched.data() && document.project().assets.len() == 2;
    t.row("one undo takes back both the new file and the setting", if back { "the frame and the asset list as they were" } else { "differ" }, back);

    t.heading("Collect Files");
    let dest = std::env::temp_dir().join(format!("b274_collect_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dest);
    let loaded = t.load("fx_amap_002.json");
    let done = package::collect(loaded.document.project(), &loaded.preserved, Some(&t.root.join("fx_amap_002.json")), &dest);
    let (m, files) = manifest(&dest);
    let asset = m["assets"].as_array().unwrap().iter().find(|a| a["id"] == "asset-map").cloned();
    t.row(
        "FX-AMAP-002 collected: the map file is copied, listed as kind lut and used by the layer whose effect names it",
        &format!("{files:?}, {:?}", asset.as_ref().map(|a| (&a["kind"], &a["used_by"]))),
        done.is_ok()
            && files.iter().any(|(p, s)| p.ends_with("master_256.amp") && s == "copied")
            && asset.is_some_and(|a| a["kind"] == "lut" && a["used_by"].as_array().is_some_and(|u| u.len() == 1)),
    );
    let collected = persist::load(&dest.join("fx_amap_002.json"));
    let same = collected.as_ref().is_ok_and(|l| {
        let mut log = FrameLog::new(8);
        render_frame(l.document.project(), &Id::new(MAIN), 0, &dest, 64, &mut log).is_ok_and(|f| f.data() == t.render(&loaded.document, 0, 64).data())
    });
    t.row("the collected project draws the same frame from its copy", if same { "the same frame" } else { "differs" }, same);
    let _ = fs::remove_dir_all(&dest);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_amap_004.json", 0), ("fx_amap_006.json", 0), ("fx_amap_009.json", 0), ("fx_amap_012.json", 0), ("fx_amap_014.json", 3)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // 006 and 012 take the covering through the alpha table, which the card does not; 013 and
    // 015 to 022 have no table to draw.
    card_fixtures(&mut t, &mut gpu, "fx_amap", 22, &[6, 12, 13, 15, 16, 17, 18, 19, 20, 21, 22], is_map);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("master_rgb_1024.amp as it starts", "master_rgb_1024.amp", json!({})),
            ("rgb_768.amp, phase -100.25", "rgb_768.amp", json!({"phase": -100.25})),
            ("alpha_1280.amp, phase 64, alpha off", "alpha_1280.amp", json!({"phase": 64})),
        ],
    );

    t.heading("Pictures: the street, in `verification/D-395 pictures/`");
    let dir = repo("verification/D-395 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    let (before, said0) = picture(&dir, "straight_256.amp", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said0:?}"), said0.is_empty());
    let shots: [(&str, &str, J, &str); 6] = [
        ("brighter", "master_256.amp", json!({}), "master_256.amp, the master raised (t to the 0.6): the street brighter, the colours kept"),
        ("rgb", "rgb_768.amp", json!({}), "rgb_768.amp, red inverted, green halved, blue on a sine: strange colours"),
        ("four_tables", "master_rgb_1024.amp", json!({}), "master_rgb_1024.amp, an S-curve master over three channel tables"),
        ("phase_64", "master_rgb_1024.amp", json!({"phase": 64}), "the same, phase 64: every table cycled a quarter, the colours shifted again"),
        ("phase_minus_128", "master_rgb_1024.amp", json!({"phase": -128}), "the same, phase -128: half way round"),
        ("alpha_on", "alpha_1280.amp", json!({"apply_to_alpha": "on"}), "alpha_1280.amp with Apply Phase Map To Alpha on: the square-root alpha table leaves the street's covering whole"),
    ];
    let mut drawn = Vec::new();
    for (i, (name, file, p, what)) in shots.iter().enumerate() {
        let out = format!("{}_{name}.png", i + 2);
        let (bytes, said) = picture(&dir, file, json!([fx("core.arbitrary_map", "fx-0-0", p)]));
        write(&out, &bytes);
        let changed = distance(&bytes, &before).1;
        t.row(&format!("{out}, {what}; draws cleanly and changes the picture"), &format!("{said:?}, {changed} pixels changed"), said.is_empty() && changed > 0);
        drawn.push(bytes);
    }
    t.row("4_four_tables.png and 5_phase_64.png differ, so Phase changes the picture", "", drawn[2] != drawn[3]);

    t.finish("D-395_arbitrary_map_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B274_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-274: a measurement, run deliberately with --release --ignored"]
fn b274_arbitrary_map_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B274_CPU").is_ok();
    let passes = if cpu { 3 } else { 8 };
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n- Drawn by: {}\n- Loops: {passes}\n\n\
         | Shot | Quality | First | Again |\n|---|---|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        if cpu { "the processor" } else { "the card" },
    );
    let noise = |id: &str| json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
        "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}});
    let shots: [(&str, Option<J>); 2] = [
        ("Noise alone", None),
        ("Noise, then Arbitrary Map, master_rgb_1024.amp, phase 40", Some(json!({"phase": 40}))),
    ];
    for (name, e) in shots {
        let project = reference("master_rgb_1024.amp", |id| {
            let mut v = vec![noise(id)];
            if let Some(p) = &e {
                v.push(fx("core.arbitrary_map", &format!("{id}c"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let mut cache = CelCache::viewer();
        gpu.forget();
        let (mut first, mut times) = (Vec::new(), Vec::new());
        for pass in 0..passes {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                let t = std::time::Instant::now();
                if cpu {
                    drop(preview::preview_frame_cached(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache).expect("CPU frame"));
                } else {
                    drop(preview::preview_frame_srgb8(&project, &comp, frame, &root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu).expect("GPU frame"));
                }
                let ms = t.elapsed().as_secs_f64() * 1000.0;
                if pass > 0 { times.push(ms) } else { first.push(ms) }
            }
        }
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B274_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-274_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
