//! B-391: D-511, after CycoreFX's CC Jaws (Transition): the layer split in two along a toothed
//! cut through the Center, the two parts moving apart. The CycoreFX manual names the controls
//! but publishes no formula and no settings when added: the rule and those settings are ours,
//! stated in `tools/teeth_wipe_reference.py`.
//!
//! Writes `verification/D-511_teeth_wipe_table.md` and draws pictures into
//! `verification/D-511 pictures/`.
//!
//! Every expected pixel is `Fixtures/teeth_wipe/expected_teeth_wipe.json`, written by the tool
//! before this code existed and printed in document 25 as FX-TEETH-001 to 027. Tolerance 2e-5.
//! Nothing here is a snapshot of a run.

mod effect_table;

use std::fs;
use std::path::Path;

use effect_table::{both, changed, distance, median, on_card, reference, repo, time_passes, Table, MAIN, TOWN};
use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::effects::Effect;
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::{persist, png_out, OutputDepth};

/// The reference shot (1920 by 1080) with `stack` on its first three layers.


/// Every fixture file on the card against the processor, frames 0 to 4 at Full and Draft. The
/// files numbered in `none` are left out with a warning.
fn card_fixtures(t: &mut Table, gpu: &mut Gpu, none: &[u32]) {
    let comp = Id::new(MAIN);
    for n in 1..=27 {
        let file = format!("fx_teeth_{n:03}.json");
        let project = persist::load(&t.root.join(&file)).unwrap().document.project().clone();
        let (mut largest, mut refused, mut agree, mut card) = (0, false, true, 0);
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in 0..5 {
                let (d, r, a, b) = both(gpu, &project, &comp, &t.root, frame, quality);
                largest = largest.max(d.0);
                refused |= r;
                agree &= a == b;
                card += on_card(&project, &comp, &t.root, frame, quality, |e| matches!(e, Effect::TeethWipe { .. }));
            }
        }
        t.row(
            &format!("{file}, frames 0 to 4 at Full and Draft"),
            &format!("largest difference {largest} of 255; on the card in {card} of 10 frames; refused by the card: {refused}; the same warnings: {agree}"),
            largest <= 1 && !refused && agree && (card == 0) == none.contains(&n),
        );
    }
}

/// The reference shot with each setting on its first three layers, frames 0, 100 and 239 at Full
/// and Draft, on the card against the processor; `want` of 3 on the card each frame.
fn card_reference(t: &mut Table, gpu: &mut Gpu, settings: &[(&str, J, usize)]) {
    let ref_comp = Id::new("comp-reference-shot");
    let ref_root = repo("Fixtures/reference_shot");
    let cpu = |project: &Project| {
        let mut log = FrameLog::new(3);
        preview::preview_frame_cached(project, &ref_comp, 100, &ref_root, PreviewQuality::Full, DEFAULT_TILE_SIZE, &mut log, &mut CelCache::viewer())
            .expect("the reference shot draws")
            .to_srgb8_straight()
    };
    let plain = cpu(&reference(|_| json!([])));
    for (what, p, want) in settings {
        let project = reference(|id| json!([fx(&format!("b391-{id}"), p)]));
        let changed = distance(&cpu(&project), &plain).1;
        t.row(
            &format!("the reference shot, Teeth Wipe {what}: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect"),
            &format!("{changed} pixels changed"),
            changed > 0,
        );
        for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
            for frame in [0, 100, 239] {
                let (d, r, a, b) = both(gpu, &project, &ref_comp, &ref_root, frame, quality);
                let card = on_card(&project, &ref_comp, &ref_root, frame, quality, |e| matches!(e, Effect::TeethWipe { .. }));
                t.row(
                    &format!("the reference shot, Teeth Wipe {what} on three layers, frame {frame}, {}", quality.label()),
                    &format!("largest difference {} of 255, {} pixels differ; {card} of 3 on the card; warnings CPU [{a}] GPU [{b}]", d.0, d.1),
                    d.0 <= 1 && !r && a == b && card == *want,
                );
            }
        }
    }
}

/// `effects` on the still `asset` (in `dir`, `size` pixels), drawn on the processor, straight
/// 8-bit, with what it warned of.
fn picture(dir: &Path, asset: &str, size: (usize, usize), effects: J) -> (Vec<u8>, Vec<String>) {
    let mut project: J = serde_json::from_str(&fs::read_to_string(repo("Fixtures/kernel/fx_kernel_001.json")).unwrap()).unwrap();
    project["assets"][0]["path"] = J::from(asset);
    let comp = &mut project["compositions"][0];
    comp["width"] = J::from(size.0);
    comp["height"] = J::from(size.1);
    let layer = &mut comp["layers"][0];
    let middle = json!([size.0 as f64 / 2.0, size.1 as f64 / 2.0]);
    layer["transform"]["anchor"]["base"] = middle.clone();
    layer["transform"]["position"]["base"] = middle;
    layer["effects"] = effects;
    let loaded = persist::load_str(&project.to_string()).expect("the picture's project reads");
    let mut log = FrameLog::new(3);
    let frame = render_frame(loaded.document.project(), &Id::new(MAIN), 0, dir, 64, &mut log).expect("the picture draws");
    let said = log.finish().iter().map(|d| format!("{} {}", d.id.as_str(), d.message)).collect();
    (frame.to_srgb8_straight(), said)
}


/// The pixel at (x, y) of a picture at the street's size.
fn at(bytes: &[u8], p: (usize, usize)) -> [u8; 4] {
    let i = (p.1 * TOWN.0 + p.0) * 4;
    [bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]
}


/// A Teeth Wipe with the settings `p`, every one `p` leaves out as it is added.
fn fx(id: &str, p: &J) -> J {
    let mut all = json!({"completion": 0, "center": [50, 50], "direction": 0, "height": 40, "width": 40,
        "shape": "spikes"});
    for (k, v) in p.as_object().unwrap() {
        all[k] = v.clone();
    }
    json!({"instance_id": id, "type_id": "core.teeth_wipe", "enabled": true, "parameters": all})
}

#[test]
fn b391_teeth_wipe() {
    let mut t = Table::new(
        "teeth_wipe",
        "# D-511: Teeth Wipe\n\nB-391, after CycoreFX's CC Jaws (Transition): the layer split in \
         two along a toothed cut, the two parts moving apart. The CycoreFX manual names the \
         controls but publishes no formula and no settings when added, so the rule and those \
         settings are ours, stated in `tools/teeth_wipe_reference.py`: a cut through the Center at \
         the Direction, its teeth Width apart and Height tall (spikes, trapezoids or blocks); the \
         part before the cut moves back across it and the part after moves on, each read between \
         pixels, until at completion 100 both have left the drawing. Every expected pixel is \
         `Fixtures/teeth_wipe/expected_teeth_wipe.json`, written by the tool before this code \
         existed and printed in document 25 as FX-TEETH-001 to 027. Tolerance 2e-5.\n",
    );

    t.heading("FX-TEETH-001 to 027 (document 25)");
    t.fixtures_numbered("expected_teeth_wipe.json", 1..=27);

    t.heading("The file");
    let files: Vec<String> = (1..=27).map(|n| format!("fx_teeth_{n:03}.json")).collect();
    t.round_trips(&files.iter().map(String::as_str).collect::<Vec<_>>());
    let saved = t.saved_parameters("fx_teeth_016.json");
    t.row(
        "fx_teeth_016.json is saved with its six settings, trapezoids, direction -135, 2 high and 3 wide, Center [60, 40]",
        &saved.to_string(),
        saved.as_object().unwrap().len() == 6
            && saved["shape"] == "trapezoids"
            && saved["direction"] == json!(-135)
            && saved["height"] == json!(2)
            && saved["width"] == json!(3)
            && saved["center"] == json!([60, 40]),
    );
    for (file, want) in [
        ("fx_teeth_017.json", "completion"),
        ("fx_teeth_018.json", "completion"),
        ("fx_teeth_019.json", "center"),
        ("fx_teeth_020.json", "direction"),
        ("fx_teeth_021.json", "height"),
        ("fx_teeth_022.json", "height"),
        ("fx_teeth_023.json", "width"),
        ("fx_teeth_024.json", "width"),
        ("fx_teeth_025.json", "shape"),
        ("fx_teeth_026.json", "shape"),
    ] {
        let why = changed(&t, file, |_| {}).why_invalid();
        t.row(&format!("{file} is refused in a sentence naming {want}"), &why, why.contains(want));
    }
    t.shape_refused(
        "fx_teeth_001.json",
        "a Teeth Wipe whose height is a word",
        r##"{"completion": 0, "center": [50, 50], "direction": 0, "height": "tall", "width": 40, "shape": "spikes"}"##,
    );
    t.shape_refused(
        "fx_teeth_001.json",
        "a Teeth Wipe whose Center is one number",
        r##"{"completion": 0, "center": 50, "direction": 0, "height": 40, "width": 40, "shape": "spikes"}"##,
    );

    t.heading("On the card (D-511)");
    let mut gpu = Gpu::new().expect("a graphics card");
    card_fixtures(&mut t, &mut gpu, &[1, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]);
    card_reference(
        &mut t,
        &mut gpu,
        &[
            ("completion 50 (as added otherwise)", json!({"completion": 50}), 3),
            ("completion 40, blocks, direction 30, Center [20, 70]", json!({"completion": 40, "shape": "blocks", "direction": 30, "center": [20, 70]}), 3),
            ("completion 30, trapezoids 80 high and 25 wide", json!({"completion": 30, "shape": "trapezoids", "height": 80, "width": 25}), 3),
            ("completion keyed 10 to 90", json!({"completion": {"base": 10, "keyframes": [{"frame": 0, "value": 10, "interp": "linear"}, {"frame": 239, "value": 90, "interp": "linear"}]}}), 3),
        ],
    );

    t.heading("Pictures (verification/D-511 pictures)");
    let dir = repo("verification/D-506 pictures");
    let out = repo("verification/D-511 pictures");
    fs::create_dir_all(&out).unwrap();
    let (street, _) = picture(&dir, "1_left.png", (480, 270), json!([]));
    png_out::write_rgba(&out.join("0_the_street.png"), 480, 270, OutputDepth::Eight, &[], &street).unwrap();
    // name, settings, a pixel that must be gone, a pixel that must be kept and the street's pixel
    // it must show (each part moved a whole number of pixels: 62 = 0.4 (20 + 135), 104 =
    // 0.4 (20 + 240), 70 = 0.4 (40 + 135)).
    for (name, p, gone, kept, from) in [
        ("1_spikes_parted", json!({"completion": 40}), (240, 135), (2, 2), (2, 64)),
        ("2_trapezoids_parted", json!({"completion": 40, "shape": "trapezoids"}), (240, 135), (2, 267), (2, 205)),
        ("3_blocks_upright", json!({"completion": 40, "shape": "blocks", "direction": 90}), (240, 135), (2, 135), (106, 135)),
        ("4_tall_narrow_spikes", json!({"completion": 40, "height": 80, "width": 20}), (240, 135), (2, 2), (2, 72)),
        ("5_done", json!({"completion": 100}), (2, 2), (240, 135), (240, 135)),
    ] {
        let (bytes, said) = picture(&dir, "1_left.png", (480, 270), json!([fx("pic", &p)]));
        png_out::write_rgba(&out.join(format!("{name}.png")), 480, 270, OutputDepth::Eight, &[], &bytes).unwrap();
        let (g, k) = (at(&bytes, gone), at(&bytes, kept));
        let done = name == "5_done";
        t.row(
            &format!("{name}.png: drawn without a warning, the pixel at {gone:?} gone, {}", if done { "every pixel gone".to_string() } else { format!("the one at {kept:?} as the street's at {from:?}") }),
            &format!("{} pixels changed; at {gone:?} {g:?}; at {kept:?} {k:?}; warnings {said:?}", distance(&bytes, &street).1),
            said.is_empty() && g[3] == 0 && if done { bytes.chunks_exact(4).all(|p| p[3] == 0) } else { k == at(&street, from) },
        );
    }
    t.finish("D-511_teeth_wipe_table.md");
}


/// The frame times, B-253's way: the reference shot (1920 by 1080, 24 a second) with a Noise
/// that changes every frame on its first three layers, so nothing is kept, then the effect,
/// every eighth frame asked for as the viewer asks, whole. The first loop starts with empty
/// caches and its 30 frames' median is "first"; the median of the loops after it is "again".
/// With `B391_CPU` set, the processor draws instead.
#[test]
#[ignore = "B-391: a measurement, run deliberately with --release --ignored"]
fn b391_teeth_wipe_timing() {
    use std::fmt::Write as _;

    let cpu = std::env::var("B391_CPU").is_ok();
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
    let shots: [(&str, Option<J>); 4] = [
        ("Noise alone", None),
        ("Noise, then Teeth Wipe, completion 50 (as added otherwise)", Some(json!({"completion": 50}))),
        ("Noise, then Teeth Wipe, completion 50, blocks, direction 30", Some(json!({"completion": 50, "shape": "blocks", "direction": 30}))),
        ("Noise, then Teeth Wipe, completion 50, trapezoids 80 high and 20 wide", Some(json!({"completion": 50, "shape": "trapezoids", "height": 80, "width": 20}))),
    ];
    for (name, e) in shots {
        let project = reference(|id| {
            let mut v = vec![json!({"instance_id": format!("{id}n"), "type_id": "core.noise", "enabled": true,
                "parameters": {"amount": 12, "mode": "color", "seed": 7, "animate": "on"}})];
            if let Some(p) = &e {
                v.push(fx(&format!("{id}u"), p));
            }
            J::Array(v)
        });
        let (comp, root) = (Id::new("comp-reference-shot"), repo("Fixtures/reference_shot"));
        let (first, times) = time_passes(&mut gpu, cpu, passes, &project, &comp, &root, PreviewQuality::Full);
        let _ = writeln!(s, "| {name} | Full | {:.1} | {:.1} |", median(first), median(times));
    }
    let out = std::env::var("B391_OUT").map(std::path::PathBuf::from).unwrap_or_else(|_| repo("verification/B-391_timing_raw.md"));
    fs::write(out, s).expect("write the timing table");
}
