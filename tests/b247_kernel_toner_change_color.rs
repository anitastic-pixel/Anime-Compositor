//! B-247 to B-249: D-368 Kernel, after CycoreFX's CC Kernel; D-369 Toner, after CycoreFX's CC
//! Toner; D-370 Change Color, after After Effects' Change Color ("Color Correction" in
//! `docs/effects/EFFECTS.md`).
//!
//! Writes `verification/D-368_kernel_table.md`, `verification/D-369_toner_table.md` and
//! `verification/D-370_change_color_table.md`.
//!
//! Every expected pixel is `Fixtures/kernel/expected_kernel.json`,
//! `Fixtures/toner/expected_toner.json` or `Fixtures/change_color/expected_change_color.json`,
//! written by the matching `tools/*_reference.py` before this code existed and printed in
//! document 25 as FX-KERNEL-001 to 018, FX-TONER-001 to 013 and FX-CHCOLOR-001 to 024.
//! Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Each also draws the street into `verification/D-368 pictures/` (and D-369, D-370).

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{keys, repo, set, town, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, render, OutputDepth};

fn kernel(lines: [[f64; 3]; 3], divider: f64, absolute_values: &str) -> Effect {
    Effect::Kernel {
        line_1: lines[0].to_vec(),
        line_2: lines[1].to_vec(),
        line_3: lines[2].to_vec(),
        divider,
        absolute_values: absolute_values.into(),
    }
}

const IDENTITY: [[f64; 3]; 3] = [[0.0; 3], [0.0, 1.0, 0.0], [0.0; 3]];
const SHARPEN: [[f64; 3]; 3] = [[0.0, -1.0, 0.0], [-1.0, 5.0, -1.0], [0.0, -1.0, 0.0]];

fn toner(tones: &str, c: [&str; 5]) -> Effect {
    Effect::Toner {
        tones: tones.into(),
        highlights: c[0].into(),
        brights: c[1].into(),
        midtones: c[2].into(),
        darktones: c[3].into(),
        shadows: c[4].into(),
    }
}

const SEPIA: [&str; 5] = ["#ffffff", "#e0cfb0", "#8c7355", "#46382a", "#000000"];

/// Change Color: the view, the hue, lightness and saturation transforms, the colour, tolerance and
/// softness, the match and invert.
fn change(view: &str, hls: [f64; 3], color: &str, tolerance: f64, softness: f64, match_colors: &str, invert_mask: &str) -> Effect {
    Effect::ChangeColor {
        view: view.into(),
        hue_transform: hls[0],
        lightness_transform: hls[1],
        saturation_transform: hls[2],
        color_to_change: color.into(),
        tolerance,
        softness,
        match_colors: match_colors.into(),
        invert_mask: invert_mask.into(),
    }
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

/// The reference shot (1920 by 1080) with `stack` on its first three layers.
fn reference(stack: impl Fn(&str) -> J) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: J = serde_json::from_str(&text).expect("the reference shot is JSON");
    let layers = &mut j["compositions"][0]["layers"];
    for (i, id) in ["a", "b", "c"].iter().enumerate() {
        layers[i]["effects"] = stack(id);
    }
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("the reference shot: {}", d.message)).document.project().clone()
}

/// An effect of `type_id` with the settings `p`; for the three here, every setting `p` leaves
/// out is the one a new one takes, since the file holds them all.
fn fx(type_id: &str, id: &str, p: &J) -> J {
    let mut all = match type_id {
        "core.kernel" => json!({"line_1": [0, 0, 0], "line_2": [0, 1, 0], "line_3": [0, 0, 0], "divider": 1, "absolute_values": "off"}),
        "core.toner" => json!({
            "tones": "tritone", "highlights": "#ffffff", "brights": "#e0cfb0", "midtones": "#8c7355",
            "darktones": "#46382a", "shadows": "#000000"}),
        "core.change_color" => json!({
            "view": "corrected", "hue_transform": 0, "lightness_transform": 0, "saturation_transform": 0,
            "color_to_change": "#ff0000", "tolerance": 15, "softness": 0, "match_colors": "hue", "invert_mask": "off"}),
        _ => json!({}),
    };
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": type_id, "enabled": true, "parameters": all})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are left out with a warning, so the card is not asked.
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

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, name: &str, type_id: &str, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p) in settings {
        let project = reference(|id| json!([fx(type_id, &format!("b247-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, {name} {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, {name} {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

fn colour_run(e: &Effect) -> bool {
    matches!(e, Effect::Toner { .. } | Effect::ChangeColor { .. })
}

/// Toner and Change Color in one run with Levels between, on the reference shot: the card fuses
/// the colour run into one pass, and it still agrees with the processor.
fn card_fused(t: &mut Table, gpu: &mut Gpu) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let project = reference(|id| {
        json!([
            fx("core.toner", &format!("{id}t"), &json!({"tones": "pentone", "highlights": "#fff3b0", "brights": "#ff9e00", "midtones": "#9d4edd", "darktones": "#3c096c", "shadows": "#10002b"})),
            fx("core.levels", &format!("{id}l"), &json!({"input_black": 20, "input_white": 230, "gamma": 1.2, "output_black": 10, "output_white": 245})),
            fx("core.change_color", &format!("{id}c"), &json!({"hue_transform": 150, "saturation_transform": 30, "color_to_change": "#9d4edd", "tolerance": 20, "softness": 10})),
        ])
    });
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [0, 100, 239] {
            let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
            let card = on_card(&project, &ref_comp, &ref_root, frame, quality, colour_run);
            t.row(
                &format!("the reference shot, Toner (pentone), Levels and Change Color (purple turned 150) in one colour run on three layers, frame {frame}, {}", quality.label()),
                &format!("largest difference {} of 255, {} pixels differ; {card} of 6 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                d.0 <= 1 && !r && a == b && card == 6,
            );
        }
    }
}

/// `effects` on the picture `asset` in `dir`, drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
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

/// The pictures' folder, with the street written into it, and a writer for more.
fn pictures(t: &mut Table, d: &str) -> (std::path::PathBuf, impl Fn(&str, &[u8])) {
    t.heading(&format!("Pictures: the street, in `verification/{d} pictures/`"));
    let dir = repo(&format!("verification/{d} pictures"));
    fs::create_dir_all(&dir).unwrap();
    let out = dir.clone();
    let write = move |name: &str, bytes: &[u8]| png_out::write_rgba(&out.join(name), TOWN.0, TOWN.1, OutputDepth::Eight, &[], bytes).unwrap();
    write("town.png", &town());
    (dir, write)
}

/// Where the street first has the colour `c`.
fn first(bytes: &[u8], c: [u8; 3]) -> usize {
    bytes.chunks_exact(4).position(|p| p[..3] == c).expect("the street has that colour")
}

fn px(b: &[u8], i: usize) -> [u8; 3] {
    [b[i * 4], b[i * 4 + 1], b[i * 4 + 2]]
}

fn effect_of(d: &anime_compositor::command::Document) -> Effect {
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("art")).unwrap().effects[0].effect.clone()
}

// --- Kernel ----------------------------------------------------------------------------------

fn is_kernel(e: &Effect) -> bool {
    matches!(e, Effect::Kernel { .. })
}

#[test]
fn b247_kernel() {
    let mut t = Table::new(
        "kernel",
        "# D-368: Kernel\n\nB-247, after CycoreFX's CC Kernel: each pixel rebuilt from itself and \
         its eight neighbours, each weighed by a number in a three by three grid the user types, \
         the sum divided by the divider, and made positive if asked. Every expected pixel is \
         `Fixtures/kernel/expected_kernel.json`, written by `tools/kernel_reference.py` before \
         this code existed and printed in document 25 as FX-KERNEL-001 to 018. Tolerance 2e-5.\n",
    );

    t.heading("FX-KERNEL-001 to 018 (document 25)");
    t.fixtures("expected_kernel.json");

    t.heading("The file");
    let files: Vec<String> = (1..=18).map(|n| format!("fx_kernel_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_kernel_006.json");
    t.row(
        "fx_kernel_006.json is saved with its three lines, divider and absolute values",
        &saved.to_string(),
        saved["line_1"] == json!([-2, -1, 0]) && saved["line_3"] == json!([0, 1, 2]) && saved["divider"] == 1 && saved["absolute_values"] == "off",
    );
    for (file, want) in [
        ("fx_kernel_014.json", "Kernel's line 2 is three numbers, left, middle and right, and this has 2."),
        ("fx_kernel_017.json", "Kernel's absolute values is \"off\" or \"on\", and this is \"yes\"."),
        ("fx_kernel_018.json", "Kernel's absolute values is \"off\" or \"on\", and this is \"ON\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }
    t.shape_refused("fx_kernel_001.json", "a Kernel with no `divider`", r#"{"line_1": [0, 0, 0], "line_2": [0, 1, 0], "line_3": [0, 0, 0], "absolute_values": "off"}"#);
    t.shape_refused("fx_kernel_001.json", "a Kernel whose line 1 is a word", r#"{"line_1": "0, 0, 0", "line_2": [0, 1, 0], "line_3": [0, 0, 0], "divider": 1, "absolute_values": "off"}"#);

    t.heading("Commands");
    let mut document = t.load("fx_kernel_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("line 1 left 1000.5", set(kernel([[1000.5, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0; 3]], 1.0, "off"))),
            ("divider 0", set(kernel(IDENTITY, 0.0, "off"))),
            ("divider 1000.5", set(kernel(IDENTITY, 1000.5, "off"))),
            ("absolute values \"On\", written with a capital", set(kernel(IDENTITY, 1.0, "On"))),
            ("divider keyed to 0", keys("divider", &[(0, &[1.0]), (4, &[0.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_kernel_001.json",
        vec![
            ("the sharpen grid, absolute values on,", set(kernel(SHARPEN, 1.0, "on"))),
            ("divider keyed from 1 to 4", keys("divider", &[(0, &[1.0]), (4, &[4.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_kernel_002.json", 0), ("fx_kernel_003.json", 0), ("fx_kernel_005.json", 0), ("fx_kernel_006.json", 0), ("fx_kernel_011.json", 2), ("fx_kernel_012.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_kernel", 18, &[1, 13, 14, 15, 16, 17, 18], is_kernel);
    card_reference(
        &mut t,
        &mut gpu,
        "Kernel",
        "core.kernel",
        &[
            ("sharpen (0 -1 0, -1 5 -1, 0 -1 0)", json!({"line_1": [0, -1, 0], "line_2": [-1, 5, -1], "line_3": [0, -1, 0]})),
            ("box blur (all ones, divided by 9)", json!({"line_1": [1, 1, 1], "line_2": [1, 1, 1], "line_3": [1, 1, 1], "divider": 9})),
            ("edges, absolute values on", json!({"line_1": [-1, -1, -1], "line_2": [-1, 8, -1], "line_3": [-1, -1, -1], "absolute_values": "on"})),
            ("emboss (-2 -1 0, -1 1 1, 0 1 2)", json!({"line_1": [-2, -1, 0], "line_2": [-1, 1, 1], "line_3": [0, 1, 2]})),
        ],
        is_kernel,
    );

    let (dir, write) = pictures(&mut t, "D-368");
    // A road pixel with road all round it, below the markings.
    let road = 250 * TOWN.0 + 3;
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.kernel", "fx-0-0", &p)]);
    let (as_added, said) = picture(&dir, "town.png", one(json!({})));
    write("2_as_added.png", &as_added);
    t.row(
        "2_as_added.png, as it starts (the middle pixel alone, divider 1): nothing changes; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&as_added, &before).1),
        said.is_empty() && distance(&as_added, &before).0 == 0,
    );
    for (name, p, what) in [
        ("3_box_blur.png", json!({"line_1": [1, 1, 1], "line_2": [1, 1, 1], "line_3": [1, 1, 1], "divider": 9}), "all ones divided by 9: every edge softened by one pixel"),
        ("4_sharpen.png", json!({"line_1": [0, -1, 0], "line_2": [-1, 5, -1], "line_3": [0, -1, 0]}), "sharpen: every edge crisper, a light rim on the light side"),
        ("6_emboss.png", json!({"line_1": [-2, -1, 0], "line_2": [-1, 1, 1], "line_3": [0, 1, 2]}), "emboss: edges lit from the lower right, shadowed at the upper left"),
    ] {
        let (bytes, said) = picture(&dir, "town.png", one(p));
        write(name, &bytes);
        t.row(
            &format!("{name}, {what}; the flat road keeps its colour; draws cleanly"),
            &format!("{said:?}, {} pixels changed, the road {:?} from {:?}", distance(&bytes, &before).1, px(&bytes, road), px(&before, road)),
            said.is_empty() && distance(&bytes, &before).1 > 0 && px(&bytes, road) == px(&before, road),
        );
    }
    let (edges, said) = picture(&dir, "town.png", one(json!({"line_1": [-1, -1, -1], "line_2": [-1, 8, -1], "line_3": [-1, -1, -1], "absolute_values": "on"})));
    write("5_edges.png", &edges);
    let lit = edges.chunks_exact(4).filter(|p| p[..3] != [0, 0, 0]).count();
    t.row(
        "5_edges.png, eight round the middle, absolute values on: flat areas go black, only the outlines of houses, windows and markings are left; draws cleanly",
        &format!("{said:?}, the road {:?}, {lit} of {} pixels not black", px(&edges, road), TOWN.0 * TOWN.1),
        said.is_empty() && px(&edges, road) == [0, 0, 0] && lit > 0 && lit < TOWN.0 * TOWN.1 / 2,
    );

    t.finish("D-368_kernel_table.md");
}

// --- Toner -----------------------------------------------------------------------------------

fn is_toner(e: &Effect) -> bool {
    matches!(e, Effect::Toner { .. })
}

#[test]
fn b248_toner() {
    let mut t = Table::new(
        "toner",
        "# D-369: Toner\n\nB-248, after CycoreFX's CC Toner: the picture coloured by its lightness \
         along two, three or five tones from shadows to highlights. Every expected pixel is \
         `Fixtures/toner/expected_toner.json`, written by `tools/toner_reference.py` before this \
         code existed and printed in document 25 as FX-TONER-001 to 013. Tolerance 2e-5.\n",
    );

    t.heading("FX-TONER-001 to 013 (document 25)");
    t.fixtures("expected_toner.json");

    t.heading("The file");
    let files: Vec<String> = (1..=13).map(|n| format!("fx_toner_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_toner_006.json");
    t.row(
        "fx_toner_006.json is saved with its tones and all five colours",
        &saved.to_string(),
        saved["tones"] == "pentone" && saved["brights"] == "#ff9e00" && saved["shadows"] == "#10002b",
    );
    for (file, want) in [
        ("fx_toner_010.json", "Toner's tones are \"duotone\", \"tritone\" or \"pentone\", and this is \"quadtone\"."),
        ("fx_toner_011.json", "Toner's tones are \"duotone\", \"tritone\" or \"pentone\", and this is \"Tritone\"."),
        ("fx_toner_012.json", "Toner's highlights is written #rrggbb, and this is \"#fff\"."),
        ("fx_toner_013.json", "Toner's midtones is written #rrggbb, and this is \"brown\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }
    t.shape_refused("fx_toner_001.json", "a Toner with no `tones`", r##"{"highlights": "#ffffff", "brights": "#e0cfb0", "midtones": "#8c7355", "darktones": "#46382a", "shadows": "#000000"}"##);
    t.shape_refused("fx_toner_001.json", "a Toner whose shadows are a number", r##"{"tones": "tritone", "highlights": "#ffffff", "brights": "#e0cfb0", "midtones": "#8c7355", "darktones": "#46382a", "shadows": 0}"##);

    t.heading("Commands");
    let mut document = t.load("fx_toner_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("tones \"monotone\"", set(toner("monotone", SEPIA))),
            ("midtones \"#8C7355\" written with capitals and a missing digit, \"#8C735\"", set(toner("tritone", ["#ffffff", "#e0cfb0", "#8C735", "#46382a", "#000000"]))),
            ("shadows \"black\"", set(toner("tritone", ["#ffffff", "#e0cfb0", "#8c7355", "#46382a", "black"]))),
        ],
    );
    t.taken(
        &mut document,
        "fx_toner_001.json",
        vec![("pentone, the purple and orange colours,", set(toner("pentone", ["#fff3b0", "#ff9e00", "#9d4edd", "#3c096c", "#10002b"])))],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_toner_001.json", 0), ("fx_toner_003.json", 0), ("fx_toner_006.json", 0), ("fx_toner_009.json", 0)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_toner", 13, &[10, 11, 12, 13], is_toner);
    card_reference(
        &mut t,
        &mut gpu,
        "Toner",
        "core.toner",
        &[
            ("as it starts (sepia tritone)", json!({})),
            ("duotone, deep purple to gold", json!({"tones": "duotone", "highlights": "#ffd166", "shadows": "#2b0a3d"})),
            ("pentone, purple and orange", json!({"tones": "pentone", "highlights": "#fff3b0", "brights": "#ff9e00", "midtones": "#9d4edd", "darktones": "#3c096c", "shadows": "#10002b"})),
        ],
        is_toner,
    );
    card_fused(&mut t, &mut gpu);

    let (dir, write) = pictures(&mut t, "D-369");
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.toner", "fx-0-0", &p)]);
    let (sepia, said) = picture(&dir, "town.png", one(json!({})));
    write("2_sepia_tritone.png", &sepia);
    let warm = sepia.chunks_exact(4).all(|p| p[0] >= p[1] && p[1] >= p[2]);
    t.row(
        "2_sepia_tritone.png, as it starts (black, #8c7355, white): an old brown photograph, every pixel's red at least its green and green at least its blue; draws cleanly",
        &format!("{said:?}, every pixel warm: {warm}"),
        said.is_empty() && warm,
    );
    let (grey, said) = picture(&dir, "town.png", one(json!({"tones": "duotone"})));
    write("3_duotone_black_white.png", &grey);
    let greyed = grey.chunks_exact(4).all(|p| p[0] == p[1] && p[1] == p[2]);
    t.row(
        "3_duotone_black_white.png, duotone from black to white: the street in plain greys; draws cleanly",
        &format!("{said:?}, every pixel grey: {greyed}"),
        said.is_empty() && greyed,
    );
    let (gold, said) = picture(&dir, "town.png", one(json!({"tones": "duotone", "highlights": "#ffd166", "shadows": "#2b0a3d"})));
    write("4_duotone_purple_gold.png", &gold);
    t.row(
        "4_duotone_purple_gold.png, duotone from deep purple to gold: a poster-like two-colour street; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&gold, &before).1),
        said.is_empty() && distance(&gold, &before).1 > 0,
    );
    let (five, said) = picture(&dir, "town.png", one(json!({"tones": "pentone", "highlights": "#fff3b0", "brights": "#ff9e00", "midtones": "#9d4edd", "darktones": "#3c096c", "shadows": "#10002b"})));
    write("5_pentone_purple_orange.png", &five);
    t.row(
        "5_pentone_purple_orange.png, pentone from near-black purple through violet and orange to pale yellow: a sunset-coloured street; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&five, &before).1),
        said.is_empty() && distance(&five, &before).1 > 0,
    );

    t.finish("D-369_toner_table.md");
}

// --- Change Color ----------------------------------------------------------------------------

fn is_change(e: &Effect) -> bool {
    matches!(e, Effect::ChangeColor { .. })
}

#[test]
fn b249_change_color() {
    let mut t = Table::new(
        "change_color",
        "# D-370: Change Color\n\nB-249, after After Effects' Change Color: pixels near one colour, \
         by RGB, hue or chroma, within a tolerance and a soft edge, turned round the colour wheel \
         and made lighter or darker, stronger or weaker; the mask it made can be shown or turned \
         over. Every expected pixel is `Fixtures/change_color/expected_change_color.json`, written \
         by `tools/change_color_reference.py` before this code existed and printed in document 25 \
         as FX-CHCOLOR-001 to 024. Tolerance 2e-5.\n",
    );

    t.heading("FX-CHCOLOR-001 to 024 (document 25)");
    t.fixtures("expected_change_color.json");

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_chcolor_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_chcolor_011.json");
    t.row(
        "fx_chcolor_011.json is saved with its view, transforms, colour, tolerance, softness, match and invert",
        &saved.to_string(),
        saved["hue_transform"] == 180
            && saved["lightness_transform"] == -30
            && saved["color_to_change"] == "#f6d6be"
            && saved["tolerance"] == 20
            && saved["match_colors"] == "rgb"
            && saved["invert_mask"] == "off",
    );
    for (file, want) in [
        ("fx_chcolor_020.json", "Change Color's colour to change is written #rrggbb, and this is \"#ff00\"."),
        ("fx_chcolor_021.json", "Change Color's match colors is \"rgb\", \"hue\" or \"chroma\", and this is \"lab\"."),
        ("fx_chcolor_022.json", "Change Color's match colors is \"rgb\", \"hue\" or \"chroma\", and this is \"Hue\"."),
        ("fx_chcolor_023.json", "Change Color's view is \"corrected\" or \"mask\", and this is \"matte\"."),
        ("fx_chcolor_024.json", "Change Color's invert mask is \"off\" or \"on\", and this is \"yes\"."),
    ] {
        let why = effect_of(&t.load(file).document).why_invalid();
        t.row(&format!("{file} is refused in a sentence"), &why, why == want);
    }
    t.shape_refused(
        "fx_chcolor_001.json",
        "a Change Color with no `color_to_change`",
        r#"{"view": "corrected", "hue_transform": 0, "lightness_transform": 0, "saturation_transform": 0, "tolerance": 15, "softness": 0, "match_colors": "hue", "invert_mask": "off"}"#,
    );
    t.shape_refused(
        "fx_chcolor_001.json",
        "a Change Color whose tolerance is a word",
        r##"{"view": "corrected", "hue_transform": 0, "lightness_transform": 0, "saturation_transform": 0, "color_to_change": "#ff0000", "tolerance": "15", "softness": 0, "match_colors": "hue", "invert_mask": "off"}"##,
    );

    t.heading("Commands");
    let mut document = t.load("fx_chcolor_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("hue transform 3600.5", set(change("corrected", [3600.5, 0.0, 0.0], "#ff0000", 15.0, 0.0, "hue", "off"))),
            ("lightness transform -100.5", set(change("corrected", [0.0, -100.5, 0.0], "#ff0000", 15.0, 0.0, "hue", "off"))),
            ("softness 100.5", set(change("corrected", [30.0, 0.0, 0.0], "#ff0000", 15.0, 100.5, "hue", "off"))),
            ("match colors \"luma\"", set(change("corrected", [30.0, 0.0, 0.0], "#ff0000", 15.0, 0.0, "luma", "off"))),
            ("tolerance keyed to 120", keys("tolerance", &[(0, &[15.0]), (4, &[120.0])])),
        ],
    );
    t.taken(
        &mut document,
        "fx_chcolor_001.json",
        vec![
            ("the mask view, green by chroma at 30 and 15, inverted,", set(change("mask", [120.0, -30.0, 20.0], "#00ff00", 30.0, 15.0, "chroma", "on"))),
            ("hue transform keyed from 0 to 240", keys("hue_transform", &[(0, &[0.0]), (4, &[240.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_chcolor_003.json", 0), ("fx_chcolor_004.json", 0), ("fx_chcolor_011.json", 0), ("fx_chcolor_012.json", 0), ("fx_chcolor_013.json", 2)]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    card_fixtures(&mut t, &mut gpu, "fx_chcolor", 24, &[1, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24], is_change);
    card_reference(
        &mut t,
        &mut gpu,
        "Change Color",
        "core.change_color",
        &[
            ("red by hue turned 120, softness 8", json!({"hue_transform": 120, "softness": 8})),
            ("the mask view, inverted, softness 8", json!({"view": "mask", "invert_mask": "on", "softness": 8})),
            ("skin by RGB at 20 and 10, turned 180 and darker", json!({"hue_transform": 180, "lightness_transform": -30, "color_to_change": "#f6d6be", "tolerance": 20, "softness": 10, "match_colors": "rgb"})),
            ("green by chroma at 30 and 15, stronger by 60", json!({"saturation_transform": 60, "color_to_change": "#00ff00", "tolerance": 30, "softness": 15, "match_colors": "chroma"})),
        ],
        is_change,
    );

    let (dir, write) = pictures(&mut t, "D-370");
    let street = town();
    let red = first(&street, [180, 90, 70]);
    let blue = first(&street, [90, 140, 170]);
    let window = first(&street, [255, 240, 170]);
    let (before, said) = picture(&dir, "town.png", json!([]));
    write("1_before.png", &before);
    t.row("1_before.png, the street with no effect; draws cleanly", &format!("{said:?}"), said.is_empty());
    let one = |p: J| json!([fx("core.change_color", "fx-0-0", &p)]);
    let (as_added, said) = picture(&dir, "town.png", one(json!({})));
    write("2_as_added.png", &as_added);
    t.row(
        "2_as_added.png, as it starts (red by hue at 15, every transform 0): nothing changes; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&as_added, &before).1),
        said.is_empty() && distance(&as_added, &before).0 == 0,
    );
    let (green, said) = picture(&dir, "town.png", one(json!({"hue_transform": 120, "softness": 8})));
    write("3_red_walls_to_green.png", &green);
    t.row(
        "3_red_walls_to_green.png, red by hue turned 120: the red-brown houses turn green, the lit windows and blue houses stay as they were; draws cleanly",
        &format!("{said:?}, a red wall {:?} from {:?}, a window {:?}, a blue wall {:?}", px(&green, red), px(&before, red), px(&green, window), px(&green, blue)),
        said.is_empty() && green[red * 4 + 1] > green[red * 4] && px(&green, window) == px(&before, window) && px(&green, blue) == px(&before, blue),
    );
    let (mask, said) = picture(&dir, "town.png", one(json!({"view": "mask", "softness": 8})));
    write("4_mask_view.png", &mask);
    let greyed = mask.chunks_exact(4).all(|p| p[0] == p[1] && p[1] == p[2]);
    t.row(
        "4_mask_view.png, the mask view: white where red was picked, black elsewhere; draws cleanly",
        &format!("{said:?}, a red wall {:?}, a blue wall {:?}, every pixel grey: {greyed}", px(&mask, red), px(&mask, blue)),
        said.is_empty() && px(&mask, red) == [255; 3] && px(&mask, blue) == [0; 3] && greyed,
    );
    let (kept, said) = picture(&dir, "town.png", one(json!({"saturation_transform": -100, "softness": 8, "invert_mask": "on"})));
    write("5_only_red_kept.png", &kept);
    let spread = |b: &[u8], i: usize| b[i * 4..i * 4 + 3].iter().max().unwrap() - b[i * 4..i * 4 + 3].iter().min().unwrap();
    t.row(
        "5_only_red_kept.png, the mask turned over and saturation -100: everything goes grey but the red-brown houses; draws cleanly",
        &format!("{said:?}, a red wall {:?}, a blue wall {:?} from {:?}", px(&kept, red), px(&kept, blue), px(&before, blue)),
        said.is_empty() && px(&kept, red) == px(&before, red) && spread(&kept, blue) <= 1 && spread(&before, blue) > 40,
    );

    t.finish("D-370_change_color_table.md");
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect, every
/// eighth frame asked for as the viewer asks, whole. The first loop starts with empty caches and
/// its 30 frames' median is "first"; the median of the loops after it is "again". With
/// `B247_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-247: a measurement, run deliberately with --release --ignored"]
fn b247_colour_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B247_CPU").is_ok();
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
    let shots: [(&str, Option<(&str, J)>); 4] = [
        ("Noise alone", None),
        ("Noise, then Kernel, sharpen (0 -1 0, -1 5 -1, 0 -1 0)", Some(("core.kernel", json!({"line_1": [0, -1, 0], "line_2": [-1, 5, -1], "line_3": [0, -1, 0]})))),
        (
            "Noise, then Toner, pentone purple and orange",
            Some(("core.toner", json!({"tones": "pentone", "highlights": "#fff3b0", "brights": "#ff9e00", "midtones": "#9d4edd", "darktones": "#3c096c", "shadows": "#10002b"}))),
        ),
        ("Noise, then Change Color, red by hue turned 120, softness 8", Some(("core.change_color", json!({"hue_transform": 120, "softness": 8})))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![fx("core.noise", &format!("{id}n"), &json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"}))];
            if let Some((type_id, p)) = &e {
                v.push(fx(type_id, &format!("{id}c"), p));
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
    let out = std::env::var("B247_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-247_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
