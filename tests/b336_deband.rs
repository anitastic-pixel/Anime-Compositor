//! B-336: D-456 Deband, this program's own effect on the control list of CelFX's Deband
//! (`docs/effects/EFFECTS.md` row 225): an edge weight from each pixel's largest step to its four
//! neighbours, an edge-stopping blur inside the flat areas, diffusion and triangular dither, each
//! reseeded every N frames, and each pulled back on edges.
//!
//! Writes `verification/D-456_deband_table.md`.
//!
//! Every expected pixel is `Fixtures/deband/expected_deband.json`, written by
//! `tools/deband_reference.py` before this code existed and printed in document 25 as
//! FX-DEBAND-001 to 024. Tolerance 2e-5. Nothing here is a snapshot of a run.
//!
//! Also draws a banded sky into `verification/D-456 pictures/`.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, distance, median, on_card, reference, repo, Table, MAIN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::{Effect, EffectKey};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Interp, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// A Deband with the starting settings but `view`, blur and diffusion.
fn debanding(view: &str, blur_amount: f64, diffusion_amount: f64) -> Effect {
    Effect::Deband {
        view: view.into(),
        edge_threshold: 4.0,
        edge_softness: 2.0,
        blur_amount,
        edge_blur_reduction: 100.0,
        diffusion_amount,
        edge_diffusion_reduction: 100.0,
        diffusion_reseed_interval: 1.0,
        noise_amount: 1.0,
        edge_noise_reduction: 100.0,
        noise_reseed_interval: 1.0,
        frame: 0,
    }
}

fn set_holder(effect: Effect) -> Command {
    Command::SetEffectParameters { composition: Id::new(MAIN), layer_id: Id::new("holder"), instance_id: Id::new("fx-0-0"), effect }
}

fn keys_holder(setting: &str, values: &[(i32, &[f64])]) -> Command {
    Command::SetEffectKeys {
        composition: Id::new(MAIN),
        layer_id: Id::new("holder"),
        instance_id: Id::new("fx-0-0"),
        setting: setting.to_string(),
        keys: values.iter().map(|&(frame, v)| EffectKey { frame, value: v.to_vec(), interp: Interp::Linear }).collect(),
    }
}

/// Why the `holder` layer's Deband is bypassed: a setting outside its range, the constant or a key.
fn holder_fault(d: &Document) -> String {
    d.project().composition(&Id::new(MAIN)).unwrap().layer(&Id::new("holder")).unwrap().effects[0].fault().unwrap_or_default()
}

/// Each command on the `holder` file is refused with a sentence and leaves it as it was; then
/// each taken one is taken, and as many undos give back `file`'s frame 0 exactly.
fn holder_commands(t: &mut Table, file: &str, refused: Vec<(&str, Command)>, taken: Vec<(&str, Command)>) {
    let mut document = t.load(file).document;
    let held = format!("{:?}", document.project());
    for (what, command) in refused {
        let why = document.apply(command).err();
        let untouched = format!("{:?}", document.project()) == held;
        t.row(&format!("{what} is refused with a sentence, and nothing changes"), &why.as_ref().map_or("taken".into(), |d| d.message.clone()), why.is_some() && untouched);
    }
    let n = taken.len();
    for (what, command) in taken {
        let ok = document.apply(command).is_ok();
        t.row(&format!("{what} is taken"), if ok { "taken" } else { "refused" }, ok);
    }
    let before = t.render(&t.load(file).document, 0, 64);
    for _ in 0..n {
        document.undo();
    }
    let same = t.render(&document, 0, 64).data() == before.data();
    t.row(&format!("undo {n} times: frame 0 is the frame it was"), if same { "byte-identical" } else { "differ" }, same);
}

/// Deband's settings as it is added.
const STARTING: &str = r#"{"view": "normal", "edge_threshold": 4, "edge_softness": 2, "blur_amount": 8,
    "edge_blur_reduction": 100, "diffusion_amount": 1, "edge_diffusion_reduction": 100,
    "diffusion_reseed_interval": 1, "noise_amount": 1, "edge_noise_reduction": 100,
    "noise_reseed_interval": 1}"#;

/// A Deband with the settings `p`; every setting `p` leaves out is the one a new one takes,
/// since the file holds them all.
fn fx(id: &str, p: &J) -> J {
    let mut all: J = serde_json::from_str(STARTING).unwrap();
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.deband", "enabled": true, "parameters": all})
}

/// A setting keyed linearly through `points`, each a frame and a value.
fn keyed(points: &[(i32, J)]) -> J {
    let frames: Vec<J> = points.iter().map(|(f, v)| json!({"frame": f, "value": v, "interp": "linear"})).collect();
    json!({"base": points[0].1, "keyframes": frames})
}

/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` change nothing or are refused, so the card is not asked.
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
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at
/// Full and Draft, on the card against the processor.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J)], pick: fn(&Effect) -> bool) {
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
        let project = reference(|id| json!([fx(&format!("b336-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot with Deband {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, pick);
                t.row(
                    &format!("the reference shot, Deband {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == 3,
                );
            }
        }
    }
}

/// The sky's size: a quarter of 1920 by 1080.
const SKY: (usize, usize) = (480, 270);

/// The smooth sky's colour at row `y`, before rounding: from (68.5, 88.5, 149) at the top, 3 levels
/// (2 in blue) brighter every 18 rows.
fn gradient(y: usize) -> [f64; 3] {
    let t = y as f64 * 15.0 / SKY.1 as f64 - 0.5;
    [70.0 + 3.0 * t, 90.0 + 3.0 * t, 150.0 + 2.0 * t]
}

/// A dusk sky, straight 8-bit: `banded` holds each colour for 18 rows and then steps 3 levels
/// (2 in blue), the nearest of those bands to the smooth gradient, as an 8-bit gradient bands;
/// otherwise the smooth gradient rounded. A dark hill stands in front of it, its edge a step of
/// more than 60 levels.
fn sky(banded: bool) -> Vec<u8> {
    let (w, h) = SKY;
    let mut bytes = Vec::with_capacity(w * h * 4);
    for y in 0..h {
        for x in 0..w {
            let band = (y * 15 / h) as f64;
            let g = gradient(y);
            let smooth = [g[0].round(), g[1].round(), g[2].round()];
            let mut c = if banded { [70.0 + 3.0 * band, 90.0 + 3.0 * band, 150.0 + 2.0 * band] } else { smooth }.map(|v| v as u8);
            let dx = x as f64 - 240.0;
            if y as f64 > 170.0 + dx * dx / 400.0 {
                c = [30, 25, 40];
            }
            bytes.extend([c[0], c[1], c[2], 255]);
        }
    }
    bytes
}

/// `effects` on the banded sky at `frame`. Drawn, straight 8-bit, with what it warned of.
fn picture(dir: &Path, effects: J, frame: i32) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from("sky.png");
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(SKY.0);
    comp["height"] = J::from(SKY.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([SKY.0 as f64 / 2.0, SKY.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let drawn = render_frame(loaded.document.project(), &Id::new(MAIN), frame, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (drawn.to_srgb8_straight(), said)
}

/// How far each row of sky (the hill left out) is from the smooth gradient, its pixels averaged
/// so that dither cancels out as the eye averages it: the mean over rows and channels, in 8-bit
/// levels. Bands are rows held at one colour and then a step, so they show here.
fn error(a: &[u8], banded: &[u8]) -> f64 {
    let (w, h) = SKY;
    let (mut sum, mut n) = (0.0, 0.0);
    for y in 0..h {
        let (mut row, mut count) = ([0.0; 3], 0.0);
        for x in 0..w {
            let i = 4 * (y * w + x);
            if banded[i] != 30 {
                (0..3).for_each(|c| row[c] += a[i + c] as f64);
                count += 1.0;
            }
        }
        if count > 0.0 {
            let g = gradient(y);
            (0..3).for_each(|c| sum += (row[c] / count - g[c]).abs());
            n += 3.0;
        }
    }
    sum / n
}

/// The largest step between one row and the next, each row's 80 pixels beside the hill (40 at
/// each side, sky from top to bottom) averaged so that dither cancels out, in 8-bit levels over
/// the three channels: how visible the bands are. As drawn it is the bands' 3 levels.
fn largest_step(a: &[u8]) -> f64 {
    let (w, h) = SKY;
    let row = |y: usize| -> [f64; 3] {
        let mut sum = [0.0; 3];
        for x in (0..40).chain(w - 40..w) {
            (0..3).for_each(|c| sum[c] += a[4 * (y * w + x) + c] as f64 / 80.0);
        }
        sum
    };
    (1..h).map(|y| (0..3).map(|c| (row(y)[c] - row(y - 1)[c]).abs()).fold(0.0, f64::max)).fold(0.0, f64::max)
}

/// The average step across the hill's edge (a step of 40 levels or more between side-by-side or
/// stacked pixels of the banded sky), in 8-bit levels: how hard the edge stays.
fn edges(a: &[u8], banded: &[u8]) -> f64 {
    let w = SKY.0;
    let (mut sum, mut n) = (0.0, 0.0);
    for i in 0..banded.len() / 4 {
        for j in [i + 1, i + w] {
            if j >= banded.len() / 4 || (j == i + 1 && (i + 1) % w == 0) {
                continue;
            }
            let step = |b: &[u8]| (0..3).map(|c| (b[4 * i + c] as f64 - b[4 * j + c] as f64).abs()).sum::<f64>();
            if step(banded) >= 40.0 {
                sum += step(a);
                n += 1.0;
            }
        }
    }
    sum / n
}

/// How many pixels have red at least 200 and green and blue at most 80: Selected Edge's red.
fn reds(a: &[u8]) -> usize {
    a.chunks_exact(4).filter(|p| p[0] >= 200 && p[1] <= 80 && p[2] <= 80).count()
}

fn is_deband(e: &Effect) -> bool {
    matches!(e, Effect::Deband { .. })
}

#[test]
fn b336_deband() {
    let mut t = Table::new(
        "deband",
        "# D-456: Deband\n\nB-336, this program's own Deband on the control list of CelFX's Deband \
         (P_Deband, whose page gives the names only). Each pixel's edge weight is its largest step \
         to its four neighbours, in levels, against Edge Threshold and Edge Softness. Inside the \
         flat areas it is blurred across then down by a normalised Gaussian that edges lend \
         nothing to and that stops at them (after Gastal and Oliveira's edge-stopping separable \
         filters, 2011); then each pixel reads its colour from up to Diffusion Amount away (D-119's \
         hash), and triangular dither of Noise Amount levels is added (Lipshitz, Wannamaker and \
         Vanderkooy, 1992), each reseeded every N frames by P0-23's held step and each pulled back \
         on edges by its reduction. Edge Only shows the edge weight as grey; Selected Edge lays it \
         over the result in red. The rule, ranges and starting values are ours. Every expected \
         pixel is `Fixtures/deband/expected_deband.json`, written by `tools/deband_reference.py` \
         before this code existed and printed in document 25 as FX-DEBAND-001 to 024. Tolerance \
         2e-5.\n",
    );

    t.heading("FX-DEBAND-001 to 024 (document 25)");
    t.fixtures("expected_deband.json");

    t.heading("The file");
    let files: Vec<String> = (1..=24).map(|n| format!("fx_deband_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_deband_018.json");
    t.row(
        "fx_deband_018.json is saved with its word and numbers as written, Blur Amount's keys kept, and no frame",
        &saved.to_string(),
        saved["view"] == "normal" && saved["blur_amount"]["keyframes"][1]["value"] == json!(16) && saved.get("frame").is_none(),
    );
    for (file, want) in [
        ("fx_deband_020.json", "\"edges\""),
        ("fx_deband_021.json", "300"),
        ("fx_deband_022.json", "-1"),
        ("fx_deband_023.json", "101"),
        ("fx_deband_024.json", "40"),
    ] {
        let why = holder_fault(&t.load(file).document);
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want) && why.starts_with("Deband"));
    }
    let mut no_view: J = serde_json::from_str(STARTING).unwrap();
    no_view.as_object_mut().unwrap().remove("view");
    t.shape_refused("fx_deband_001.json", "a Deband with no `view`", &no_view.to_string());

    t.heading("Commands");
    holder_commands(
        &mut t,
        "fx_deband_001.json",
        vec![
            ("view \"edges\"", set_holder(debanding("edges", 8.0, 1.0))),
            ("blur amount -1", set_holder(debanding("normal", -1.0, 1.0))),
            ("noise amount keyed to 40", keys_holder("noise_amount", &[(0, &[1.0]), (4, &[40.0])])),
        ],
        vec![
            ("view Edge Only, blur 12, diffusion 2", set_holder(debanding("edge_only", 12.0, 2.0))),
            ("blur amount keyed from 0 to 16", keys_holder("blur_amount", &[(0, &[0.0]), (4, &[16.0])])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[
        ("fx_deband_001.json", 0),
        ("fx_deband_001.json", 1),
        ("fx_deband_006.json", 0),
        ("fx_deband_009.json", 0),
        ("fx_deband_015.json", 0),
        ("fx_deband_016.json", 0),
        ("fx_deband_018.json", 2),
    ]);

    t.heading("On the card against the processor: within 1 level of 255");
    let mut gpu = Gpu::new().expect("a usable card");
    // FX-DEBAND-002 (blur, diffusion and noise all 0) changes nothing, and 020 to 024 are
    // refused, so the card is not asked. 018's frame 0 is blur 0, its other frames are not.
    let mut none: Vec<u32> = vec![2];
    none.extend(20..=24);
    card_fixtures(&mut t, &mut gpu, "fx_deband", 24, &none, is_deband);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("as added", json!({})),
            (
                "blur 30, diffusion 4, noise 4 reseeded every 3 frames, Selected Edge",
                json!({"blur_amount": 30, "diffusion_amount": 4, "noise_amount": 4, "noise_reseed_interval": 3, "view": "selected_edge"}),
            ),
            ("threshold keyed 2 to 20, Edge Blur Reduction 50", json!({"edge_threshold": keyed(&[(0, json!(2)), (239, json!(20))]), "edge_blur_reduction": 50})),
        ],
        is_deband,
    );

    t.heading("Pictures: a banded dusk sky, in `verification/D-456 pictures/`");
    let dir = repo("verification/D-456 pictures");
    fs::create_dir_all(&dir).unwrap();
    let write = |name: &str, bytes: &[u8]| png_out::write_rgba(&dir.join(name), SKY.0, SKY.1, OutputDepth::Eight, &[], bytes).unwrap();
    let smooth = sky(false);
    write("sky.png", &sky(true));
    write("0_smooth.png", &smooth);
    let deband = |p: J| json!([fx("fx-0-0", &p)]);
    let (banded, said) = picture(&dir, json!([]), 0);
    write("1_banded.png", &banded);
    let (e_banded, edge_banded, s_banded) = (error(&banded, &banded), edges(&banded, &banded), largest_step(&banded));
    t.row(
        "1_banded.png, the sky as drawn: bands 18 rows tall, each a step of 3 levels; draws cleanly",
        &format!("{said:?}, each row on average {e_banded:.2} levels from the smooth gradient (0_smooth.png); largest step between rows {s_banded:.2} levels"),
        said.is_empty() && e_banded > 0.5 && (s_banded - 3.0).abs() < 1e-9,
    );
    let (off, said) = picture(&dir, deband(json!({"blur_amount": 0, "diffusion_amount": 0, "noise_amount": 0})), 0);
    t.row(
        "blur, diffusion and noise 0: the sky untouched; draws cleanly",
        &format!("{said:?}, {} pixels changed", distance(&off, &banded).1),
        said.is_empty() && distance(&off, &banded).1 == 0,
    );
    let (added, said) = picture(&dir, deband(json!({})), 0);
    write("2_as_added.png", &added);
    let (e_added, edge_added, s_added) = (error(&added, &banded), edges(&added, &banded), largest_step(&added));
    t.row(
        "2_as_added.png, Deband as added: closer to the smooth gradient than 1, smaller steps between rows, and the hill's edge keeps at least nine tenths of its step; draws cleanly",
        &format!("{said:?}, each row on average {e_added:.2} levels from the smooth gradient; largest step between rows {s_added:.2} levels; edge {edge_added:.1} against {edge_banded:.1} as drawn"),
        said.is_empty() && e_added < e_banded && s_added < s_banded && edge_added >= 0.9 * edge_banded,
    );
    let (next, _) = picture(&dir, deband(json!({})), 1);
    t.row(
        "Deband as added, frame 1: fresh dither, so it differs from frame 0",
        &format!("{} pixels differ", distance(&next, &added).1),
        distance(&next, &added).1 > 0,
    );
    let (strong, said) = picture(&dir, deband(json!({"blur_amount": 40, "diffusion_amount": 3, "noise_amount": 2})), 0);
    write("3_strong.png", &strong);
    let (s_strong, edge_strong) = (largest_step(&strong), edges(&strong, &banded));
    t.row(
        "3_strong.png, blur 40, diffusion 3, noise 2: smaller steps between rows than 1, and the hill's edge still keeps nine tenths of its step; draws cleanly",
        &format!("{said:?}, largest step between rows {s_strong:.2} levels against {s_banded:.2} in 1; edge {edge_strong:.1} against {edge_banded:.1} as drawn"),
        said.is_empty() && s_strong < s_banded && edge_strong >= 0.9 * edge_banded,
    );
    let (edge_only, said) = picture(&dir, deband(json!({"view": "edge_only"})), 0);
    write("4_edge_only.png", &edge_only);
    let white = edge_only.chunks_exact(4).filter(|p| p[0] == 255 && p[1] == 255 && p[2] == 255).count();
    let black = edge_only.chunks_exact(4).filter(|p| p[0] == 0 && p[1] == 0 && p[2] == 0).count();
    t.row(
        "4_edge_only.png, View Edge Only: the hill's outline white, the sky's 3-level bands black (not edges); draws cleanly",
        &format!("{said:?}, {white} white pixels, {black} black of {}", SKY.0 * SKY.1),
        said.is_empty() && white > 0 && white < SKY.0 * SKY.1 / 20 && black > SKY.0 * SKY.1 * 9 / 10,
    );
    let (selected, said) = picture(&dir, deband(json!({"view": "selected_edge"})), 0);
    write("5_selected_edge.png", &selected);
    t.row(
        "5_selected_edge.png, View Selected Edge: 2 with the hill's outline in red, as many red pixels as 4 has white; draws cleanly",
        &format!("{said:?}, {} red pixels", reds(&selected)),
        said.is_empty() && reds(&selected) == white,
    );

    t.finish("D-456_deband_table.md");
}

/// The frame times, B-239's way: the reference shot (1920 by 1080, 24 a second) with the effect
/// on its first three layers, every eighth frame asked for as the viewer asks, whole. Deband
/// reseeding every frame changes every frame, so nothing of it is kept. The first loop starts
/// with empty caches and its 30 frames' median is "first"; the median of the loops after it is
/// "again". With `B336_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-336: a measurement, run deliberately with --release --ignored"]
fn b336_deband_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B336_CPU").is_ok();
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
        ("The shot alone", None),
        ("Deband as added", Some(json!({}))),
        ("Deband blur 30, diffusion 4, noise 4", Some(json!({"blur_amount": 30, "diffusion_amount": 4, "noise_amount": 4}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| match &e {
            Some(p) => json!([fx(&format!("{id}d"), p)]),
            None => json!([]),
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
    let out = std::env::var("B336_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-336_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
