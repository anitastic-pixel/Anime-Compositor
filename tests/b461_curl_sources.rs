//! B-461: D-583, Curl Noise's Source This Layer and Other Layer, after After Effects' Curl Noise
//! (26.3 beta) Source menu with its Source Layer, Softness, Source Mapping and Mirror at Edge:
//! the noise read from a picture's lightness instead of made.
//!
//! Writes `verification/D-583_curl_sources_table.md` and draws pictures into
//! `verification/D-583 pictures/`.
//!
//! Every expected pixel is `Fixtures/curl_noise/expected_curl_noise.json`, written by
//! `tools/curl_noise_reference.py` before this code existed and printed in document 25 as
//! FX-CURL-033, 034 and 045 to 060 (033 regenerated and 034 reworded under D-566). Tolerance
//! 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{
    both, changed, distance, median, on_card, reference, repo, set, town, Table, MAIN, TOWN,
};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Curl Noise with the settings `p`, every one `p` leaves out as it is added.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.curl_noise" => json!({"source": "internal", "speed": 10, "direction": 0, "size": 100, "offset": [0, 0], "evolution": 0,
            "turbulence_speed": 20, "swirl": 45, "density": 0, "smoothness": 50, "vertical_bias": 50, "sample_count": 12, "sample_radius": 30,
            "flow_softness": 20, "edge_definition": 50, "flow_falloff": 0, "view": "final_render", "contrast": 100, "brightness": 0,
            "clip_hdr_results": "on", "channel": "rgb"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Soft grey rings round the street's middle, 480 by 270, opaque: a picture whose lightness
/// turns the flow in circles.
fn rings() -> Vec<u8> {
    let (w, h) = TOWN;
    let mut out = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let r = (x as f64 - w as f64 / 2.0).hypot(y as f64 - h as f64 / 2.0);
            let v = (127.5 + 127.5 * (r / 14.0).sin()).round() as u8;
            out.extend([v, v, v, 255]);
        }
    }
    out
}

/// `effects` on the street (in `dir`), with `rings.png` (in `dir`) beneath it as the hidden layer
/// `rings`, drawn on the processor at `frame`, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("town.png");
    let mut asset = project["assets"][0].clone();
    asset["id"] = J::from("asset-rings");
    asset["name"] = J::from("rings");
    asset["path"] = J::from("rings.png");
    project["assets"].as_array_mut().unwrap().push(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(TOWN.0);
    comp["height"] = J::from(TOWN.1);
    comp["duration_frames"] = J::from(48);
    comp["work_area"]["end_frame_exclusive"] = J::from(48);
    let layer = &mut comp["layers"][0];
    layer["out_frame"] = J::from(48);
    let middle = json!([TOWN.0 as f64 / 2.0, TOWN.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let mut hidden = layer.clone();
    hidden["id"] = J::from("rings");
    hidden["name"] = J::from("rings");
    hidden["asset_id"] = J::from("asset-rings");
    hidden["enabled"] = J::from(false);
    hidden["effects"] = json!([]);
    comp["layers"].as_array_mut().unwrap().push(hidden);
    comp["layer_order"].as_array_mut().unwrap().push(J::from("rings"));
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}

/// Most of the street changed, and every pixel a grey: its three channels equal.
fn greys(a: &[u8], b: &[u8]) -> bool {
    distance(a, b).1 > a.len() / 4 / 2 && a.chunks_exact(4).all(|p| p[0] == p[1] && p[1] == p[2])
}

#[test]
fn b461_curl_sources() {
    let mut t = Table::new(
        "curl_noise",
        "# D-583: Curl Noise, Source This Layer and Other Layer\n\nB-461: `core.curl_noise`'s Source takes After \
         Effects' This Layer and Other Layer. The noise is then the picture's lightness (each pixel's luminance times \
         its covering, read between pixels), blurred first by Softness (a Gaussian, sigma Softness / 10 pixels); its \
         slope, the lightness one pixel on less one pixel back, steers the flow lines as the made noise's does. This \
         Layer reads the layer as it reaches the effect, so it moves with the drawing; Other Layer reads D-189's \
         layer setting Source Layer, fitted to the drawing by Source Mapping (Fill stretches it, Center keeps its own \
         size). Outside the picture is black, or with Mirror at Edge on the picture mirrored back. The rule is this \
         program's own; Adobe publishes none. Every expected pixel is `Fixtures/curl_noise/expected_curl_noise.json`, \
         written by `tools/curl_noise_reference.py` before this code existed and printed in document 25 as FX-CURL-033, \
         034 and 045 to 060 (033 regenerated as a working This Layer case and 034 reworded, under D-566). Tolerance \
         2e-5.\n",
    );

    t.heading("FX-CURL-033, 034 and 045 to 060 (document 25)");
    t.fixtures_numbered("expected_curl_noise.json", 33..=34);
    t.fixtures_numbered("expected_curl_noise.json", 45..=60);

    t.heading("The file");
    let mut all = vec!["fx_curl_033.json".to_string(), "fx_curl_034.json".to_string()];
    all.extend((45..=60).map(|n| format!("fx_curl_{n:03}.json")));
    t.round_trips(&all.iter().map(String::as_str).collect::<Vec<_>>());
    let old = t.saved_parameters("fx_curl_001.json");
    t.row(
        "fx_curl_001.json, from before the sources, opens as Internal and saves without the four new settings",
        &old.to_string(),
        old["source"] == "internal" && ["source_layer", "softness", "source_mapping", "mirror_at_edge"].iter().all(|k| old.get(k).is_none()),
    );
    let saved = t.saved_parameters("fx_curl_048.json");
    t.row(
        "fx_curl_048.json is saved with Source Other Layer, its layer and Center",
        &saved.to_string(),
        saved["source"] == "other_layer" && saved["source_layer"] == "ramp" && saved["source_mapping"] == "center",
    );
    let saved = t.saved_parameters("fx_curl_051.json");
    t.row("fx_curl_051.json is saved with Mirror at Edge on", &saved.to_string(), saved["mirror_at_edge"] == "on");
    let saved = t.saved_parameters("fx_curl_056.json");
    t.row("fx_curl_056.json is saved with Softness's keys kept", &saved["softness"].to_string(), saved["softness"]["keyframes"][1]["value"] == json!(30));
    for (file, want) in [
        ("fx_curl_034.json", "Curl Noise's Source is Other Layer, but Source Layer names no layer: choose one, or set Source to Internal or This Layer."),
        ("fx_curl_057.json", "Curl Noise's softness runs from 0 to 100, and this is 101."),
        ("fx_curl_058.json", "Curl Noise's source mapping is \"fill\" or \"center\", and this is \"stretch\"."),
        ("fx_curl_059.json", "Curl Noise's mirror at edge is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_curl_060.json", "Curl Noise's source layer is the name of a layer of this composition, and this is 3."),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }

    t.heading("Commands");
    let mut document = t.load("fx_curl_047.json").document;
    let with = |f: fn(&mut Effect)| changed(&t, "fx_curl_047.json", f);
    let refused = vec![
        ("softness 101", set(with(|e| if let Effect::CurlNoise { softness, .. } = e { *softness = 101.0 }))),
        ("source mapping \"stretch\"", set(with(|e| if let Effect::CurlNoise { source_mapping, .. } = e { *source_mapping = "stretch".into() }))),
        ("mirror at edge \"yes\"", set(with(|e| if let Effect::CurlNoise { mirror_at_edge, .. } = e { *mirror_at_edge = "yes".into() }))),
        ("source layer 3", set(with(|e| if let Effect::CurlNoise { source_layer, .. } = e { *source_layer = json!(3) }))),
        ("Other Layer naming no layer", set(with(|e| if let Effect::CurlNoise { source_layer, .. } = e { *source_layer = json!("") }))),
    ];
    let taken = vec![
        (
            "This Layer, softness 20, mirror at edge on",
            set(with(|e| {
                if let Effect::CurlNoise { source, softness, mirror_at_edge, .. } = e {
                    *source = "this_layer".into();
                    *softness = 20.0;
                    *mirror_at_edge = "on".into();
                }
            })),
        ),
        ("Other Layer \"ramp\", Center", set(with(|e| if let Effect::CurlNoise { source_mapping, .. } = e { *source_mapping = "center".into() }))),
    ];
    t.refused(&mut document, refused);
    t.taken(&mut document, "fx_curl_047.json", taken);

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_curl_033.json", 4),
        ("fx_curl_046.json", 0),
        ("fx_curl_049.json", 0),
        ("fx_curl_050.json", 0),
        ("fx_curl_051.json", 0),
        ("fx_curl_055.json", 0),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new(MAIN);
    for n in [33u32, 34].into_iter().chain(45..=60) {
        let file = format!("fx_curl_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(&mut gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::CurlNoise { .. }));
            }
        }
        // The refused ones (034, 057 to 060) and the one whose layer is gone (053) leave nothing
        // for the card.
        let none = n == 34 || n == 53 || n >= 57;
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none,
        );
    }
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in [
        ("This Layer as added", json!({"source": "this_layer"})),
        (
            "This Layer, Softness 20, Curl Generation, Mirror at Edge on, offset 30 and -20",
            json!({"source": "this_layer", "softness": 20, "view": "curl_generation", "mirror_at_edge": "on", "offset": [30, -20]}),
        ),
        ("Other Layer layer4, Fill, radius 12", json!({"source": "other_layer", "source_layer": "layer-4", "sample_radius": 12})),
        (
            "Other Layer layer4, Center, Softness keyed 0 to 40, channel Alpha",
            json!({"source": "other_layer", "source_layer": "layer-4", "source_mapping": "center", "softness": keyed(&[(0, json!(0)), (239, json!(40))]), "channel": "alpha"}),
        ),
    ] {
        let project = reference(|id| json!([fx("core.curl_noise", &format!("b461-{id}"), &p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Curl Noise {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(&mut gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::CurlNoise { .. }));
                t.row(
                    &format!("the reference shot, Curl Noise {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }

    t.heading("Pictures: the street, in `verification/D-583 pictures/`");
    let dir = repo("verification/D-583 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    write("rings.png", &rings());
    let street = |p: J| picture(&dir, json!([fx("core.curl_noise", "fx-0-0", &p)]), 0);
    let (before, said) = picture(&dir, json!([]), 0);
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());

    let (internal, _) = street(json!({}));
    let (own, said) = street(json!({"source": "this_layer"}));
    write("2_this_layer.png", &own);
    t.row(
        "2_this_layer.png, Source This Layer as added: grey flow lines that run along the street's own edges (buildings, windows, road), even where it is flat; draws cleanly",
        &format!("{said:?}, differs from the made noise {}", own != internal),
        said.is_empty() && greys(&own, &before) && own != internal,
    );

    let (input, said) = street(json!({"source": "this_layer", "view": "input_noise"}));
    write("3_this_layer_input_noise.png", &input);
    let luminance = |p: &[u8]| 0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64;
    let darker = input.chunks_exact(4).zip(before.chunks_exact(4)).filter(|(a, b)| (a[0] as f64 - luminance(b)).abs() > 40.0).count();
    t.row(
        "3_this_layer_input_noise.png, This Layer, View Input Noise: the street in grey, its own lightness; draws cleanly",
        &format!("{said:?}, {darker} pixels far from the street's lightness"),
        said.is_empty() && greys(&input, &before) && darker < input.len() / 4 / 50,
    );

    let (soft, said) = street(json!({"source": "this_layer", "softness": 40}));
    write("4_this_layer_softness_40.png", &soft);
    t.row(
        "4_this_layer_softness_40.png, This Layer, Softness 40: the picture blurred first, so the lines follow only the street's large shapes; draws cleanly",
        &format!("{said:?}, differs from 2_this_layer.png {}", soft != own),
        said.is_empty() && greys(&soft, &before) && soft != own,
    );

    let (ringed, said) = street(json!({"source": "other_layer", "source_layer": "rings", "view": "curl_generation"}));
    write("5_rings_curl_generation.png", &ringed);
    let coloured = ringed.chunks_exact(4).filter(|p| p[0].abs_diff(p[1]) > 8).count();
    t.row(
        "5_rings_curl_generation.png, Source Other Layer, the hidden layer rings (rings.png), View Curl Generation: the flow as red and green running round the rings; draws cleanly",
        &format!("{said:?}, {coloured} pixels where red and green differ"),
        said.is_empty() && coloured > ringed.len() / 4 / 2,
    );

    let (swirls, said) = street(json!({"source": "other_layer", "source_layer": "rings", "sample_radius": 40}));
    write("6_rings_flow.png", &swirls);
    t.row(
        "6_rings_flow.png, Source Other Layer rings, radius 40: grey flow lines in circles round the middle; draws cleanly",
        &format!("{said:?}, differs from 2_this_layer.png {}", swirls != own),
        said.is_empty() && greys(&swirls, &before) && swirls != own,
    );

    let (gone, said) = street(json!({"source": "other_layer", "source_layer": "gone"}));
    t.row(
        "the street with Source Other Layer naming gone, not a layer: drawn without the effect, with D-189's warning",
        &format!("{said:?}"),
        gone == before && said.iter().any(|s| s.starts_with(DiagnosticId::EffectLayerMissing.as_str())),
    );

    t.finish("D-583_curl_sources_table.md");
}

/// The frame times, B-326's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B461_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-461: a measurement, run deliberately with --release --ignored"]
fn b461_curl_sources_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B461_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 3] = [
        ("Noise alone", None),
        ("Noise, then Curl Noise This Layer as added", Some(json!({"source": "this_layer"}))),
        ("the same with Softness 50", Some(json!({"source": "this_layer", "softness": 50}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx("core.curl_noise", &format!("{id}c"), p));
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
    let out = std::env::var("B461_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-461_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
