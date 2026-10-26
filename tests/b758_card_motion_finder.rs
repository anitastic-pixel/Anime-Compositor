//! B-758 (D-878): the motion finder on the card, held to the processor's.
//!
//! The finder behind Pixel Motion Blur, Timewarp, Warp Stabilizer and Rolling Shutter Repair
//! runs on the card for a frame the viewer draws there, and on the processor everywhere else.
//! Two bars, D-804's and the pixel-cost bar:
//! - motion: on the reference shot, every found motion within 1e-4 px of the processor's, none
//!   0.05 px or more;
//! - pictures: every frame of the four effects' fixtures, at Full and Draft, and the reference
//!   shot with each effect, drawn with the card's finder and with the processor's: at 8 bpc at
//!   most 1 level on 99.9% of pixels and at most 2 anywhere, in float at most 1e-4 on solid areas.
//!
//! Writes `verification/B-758_card_motion_finder_table.md`. A machine with no card the finder
//! can use, or whose card fails the finder's own check, says so and passes: the processor then
//! finds every motion, which is what the frames were before.

use std::fmt::Write as _;
use std::path::Path;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::{self, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::Project;
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::render;
use anime_compositor::{found_motion, motion_card, motion_card_off, motion_card_uses};
use anime_compositor::{png_out, OutputDepth};
use serde_json::{json, Value};

mod common;
use common::repo;

#[derive(Default)]
struct Cost {
    pixels: usize,
    within1: usize,
    max8: u8,
    float_solid: f32,
    float_any: f32,
    frames: usize,
    card_finds: u64,
}

impl Cost {
    fn row(&self, name: &str) -> String {
        format!(
            "| {name} | {} | {} | {:.4}% | {} | {:.2e} | {:.2e} | {} |\n",
            self.frames,
            self.pixels,
            100.0 * self.within1 as f64 / self.pixels.max(1) as f64,
            self.max8,
            self.float_solid,
            self.float_any,
            self.card_finds
        )
    }
    fn holds(&self) -> bool {
        self.within1 as f64 >= 0.999 * self.pixels as f64
            && self.max8 <= 2
            && self.float_solid <= 1e-4
    }
}

/// One frame planned for the card, as the viewer plans it, with the processor's finder and with
/// the card's, each drawn by the processor so that only the finder differs.
fn compare(cost: &mut Cost, p: &Project, root: &Path, frame: i32, q: PreviewQuality) {
    let comp = p.compositions[0].id.clone();
    let draw = |off: bool| {
        motion_card_off(off);
        let mut log = FrameLog::new(3);
        let plan = compose::plan_preview(
            p,
            &comp,
            frame,
            root,
            q,
            &mut log,
            &mut CelCache::viewer(),
            true,
        );
        motion_card_off(false);
        plan.ok()
            .map(|plan| render::render(&plan, DEFAULT_TILE_SIZE))
    };
    let Some(a) = draw(true) else { return };
    let before = motion_card_uses();
    let Some(b) = draw(false) else { return };
    cost.card_finds += motion_card_uses() - before;
    cost.frames += 1;
    let (a8, b8) = (a.to_srgb8_straight(), b.to_srgb8_straight());
    for (i, (p, q)) in a8.chunks_exact(4).zip(b8.chunks_exact(4)).enumerate() {
        // A pixel whose alpha is 0 of 255 in both is not seen; its colour bytes are not counted.
        let d = if p[3] == 0 && q[3] == 0 {
            0
        } else {
            p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap()
        };
        cost.pixels += 1;
        cost.within1 += (d <= 1) as usize;
        cost.max8 = cost.max8.max(d);
        let (fa, fb) = (&a.data()[i * 4..i * 4 + 4], &b.data()[i * 4..i * 4 + 4]);
        let e = fa
            .iter()
            .zip(fb)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0f32, f32::max);
        cost.float_any = cost.float_any.max(e);
        if fa[3] == 1.0 && fb[3] == 1.0 {
            cost.float_solid = cost.float_solid.max(e);
        }
    }
}

/// Frame 120's middle at Full (Pixel Motion Blur at vector detail 100, where Full uses the card),
/// with the processor's finder and the card's, and their difference times 64 (a 1-level difference shows as a grey of 64), in `verification/B-758 pictures/`.
fn pictures(p: &Project, root: &Path) {
    let comp = p.compositions[0].id.clone();
    let draw = |off: bool| {
        motion_card_off(off);
        let plan = compose::plan_preview(
            p,
            &comp,
            120,
            root,
            PreviewQuality::Full,
            &mut FrameLog::new(3),
            &mut CelCache::viewer(),
            true,
        )
        .unwrap();
        motion_card_off(false);
        let f = render::render(&plan, DEFAULT_TILE_SIZE);
        (f.width(), f.to_srgb8_straight())
    };
    let (w, a) = draw(true);
    let before = motion_card_uses();
    let (_, b) = draw(false);
    assert!(
        motion_card_uses() > before,
        "the card found frame 120's motion"
    );
    let h = a.len() / 4 / w;
    let (cw, ch) = (480, 270);
    let crop = |s: &[u8]| -> Vec<u8> {
        let (x0, y0) = (w / 2 - cw / 2, h / 2 - ch / 2);
        (y0..y0 + ch)
            .flat_map(|y| s[(y * w + x0) * 4..(y * w + x0 + cw) * 4].to_vec())
            .collect()
    };
    let diff: Vec<u8> = crop(&a)
        .chunks_exact(4)
        .zip(crop(&b).chunks_exact(4))
        .flat_map(|(p, q)| {
            let v = (p.iter().zip(q).map(|(x, y)| x.abs_diff(*y)).max().unwrap() as usize * 64)
                .min(255) as u8;
            [v, v, v, 255]
        })
        .collect();
    let dir = repo("verification/B-758 pictures");
    std::fs::create_dir_all(&dir).unwrap();
    for (what, bytes) in [
        ("processor_finder", crop(&a)),
        ("card_finder", crop(&b)),
        ("difference_x64", diff),
    ] {
        png_out::write_rgba(
            &dir.join(format!("pmb_f120_{what}.png")),
            cw,
            ch,
            OutputDepth::Eight,
            &[],
            &bytes,
        )
        .unwrap();
    }
}

/// The reference shot with `effect` on its second layer.
fn reference_with(effect: Value) -> Project {
    let text = std::fs::read_to_string(repo("verification/B-08a_project.json")).unwrap();
    let mut j: Value = serde_json::from_str(&text).unwrap();
    j["compositions"][0]["layers"][1]["effects"] = json!([effect]);
    persist::load_str(&j.to_string())
        .unwrap()
        .document
        .project()
        .clone()
}

/// The first effect of a fixture, to put on the reference shot.
fn fixture_effect(folder: &str, file: &str) -> Value {
    let fx: Value = serde_json::from_str(
        &std::fs::read_to_string(repo(&format!("Fixtures/{folder}/{file}"))).unwrap(),
    )
    .unwrap();
    fx["compositions"][0]["layers"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|l| l["effects"].as_array().and_then(|e| e.first().cloned()))
        .unwrap()
}

fn pmb_added() -> Value {
    json!({"instance_id": "b758", "type_id": "core.pixel_motion_blur", "enabled": true, "parameters":
        {"shutter_control": "automatic", "shutter_angle": 180, "shutter_samples": 16, "vector_detail": 20}})
}

fn timewarp_50() -> Value {
    json!({"instance_id": "b758", "type_id": "core.timewarp", "enabled": true, "parameters":
        {"method": "pixel_motion", "adjust_time_by": "speed", "speed": 50, "source_frame": 0, "vector_detail": 20,
         "global_smoothness": 0.5, "local_smoothness": 0.5, "smoothing_iterations": 4, "build_from_one_image": "off",
         "correct_luminance_changes": "off", "filtering": "normal", "error_threshold": 1, "block_size": 6,
         "red_weight": 0.3, "green_weight": 0.6, "blue_weight": 0.1, "enable_motion_blur": "off",
         "shutter_control": "automatic", "shutter_angle": 180, "shutter_samples": 5}})
}

#[test]
fn b758_card_motion_finder() {
    let mut table = String::from(
        "# B-758: the card motion finder against the processor's (D-878)\n\n\
         Written by `tests/b758_card_motion_finder.rs`. Change nothing here by hand.\n\n",
    );
    if !motion_card() {
        table.push_str("This machine has no card the finder can use (or its card failed the finder's own check), so the processor found every motion and nothing was compared.\n");
        std::fs::write(
            repo("verification/B-758_card_motion_finder_table.md"),
            table,
        )
        .unwrap();
        println!("B-758: no card for the motion finder on this machine; the processor finds every motion");
        return;
    }

    // Old projects: a fixture of each effect loads and saves as it did; nothing new is saved.
    for (folder, file) in [
        ("pixel_motion_blur", "fx_pmb_001.json"),
        ("timewarp", "fx_tw_001.json"),
        ("warp_stabilizer", "fx_ws_001.json"),
        ("rolling_shutter_repair", "fx_rsr_001.json"),
    ] {
        let loaded = persist::load(&repo(&format!("Fixtures/{folder}/{file}"))).unwrap();
        let saved = persist::to_json(loaded.document.project(), &loaded.preserved);
        let again = persist::load_str(&saved).unwrap();
        assert_eq!(
            saved,
            persist::to_json(again.document.project(), &again.preserved),
            "{file} saves as it loads"
        );
        assert!(
            !saved.contains("\"card\""),
            "{file}: the finder's place is not saved"
        );
    }

    // Where the card is used: a Full frame at vector detail 20 (as added) finds its motion on the
    // processor, and the card is not opened for it; Full at 40 and Draft at 20 use the card.
    let root = repo("Fixtures/reference_shot");
    for (vd, q, card) in [
        (20, PreviewQuality::Full, false),
        (40, PreviewQuality::Full, true),
        (20, PreviewQuality::Draft, true),
    ] {
        for mut effect in [pmb_added(), timewarp_50()] {
            effect["parameters"]["vector_detail"] = json!(vd);
            let p = reference_with(effect);
            let before = motion_card_uses();
            compose::plan_preview(
                &p,
                &p.compositions[0].id,
                121,
                &root,
                q,
                &mut FrameLog::new(3),
                &mut CelCache::viewer(),
                true,
            )
            .unwrap();
            assert_eq!(
                motion_card_uses() > before,
                card,
                "{q:?} at vector detail {vd}: the card finds the motion: {card}"
            );
        }
    }
    table.push_str("A frame planned for the card finds its motion on the card at Draft, and at Full from vector detail 40; at Full below 40 (vector detail 20 as added) the processor finds it (checked above this table on Pixel Motion Blur and Timewarp).\n\n");

    // The motion bar, on the reference shot.
    let p = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap()
        .document
        .project()
        .clone();
    let comp = p.compositions[0].id.clone();
    table.push_str("## Found motion, card against processor\n\nThe reference shot's frames 0 to 210, every 30th, each with the next, as the layer's picture the finder is given.\n\n| quality | vector detail | vectors | largest difference (px) | over 1e-4 px | over 0.05 px |\n|---|---:|---:|---:|---:|---:|\n");
    let mut worst_all = 0.0f64;
    for q in [PreviewQuality::Full, PreviewQuality::Draft] {
        let draw = |f: i32| {
            preview::preview_frame_cached(
                &p,
                &comp,
                f,
                &root,
                q,
                DEFAULT_TILE_SIZE,
                &mut FrameLog::new(3),
                &mut CelCache::viewer(),
            )
            .unwrap()
        };
        let pairs: Vec<_> = (0..=210)
            .step_by(30)
            .map(|f| (draw(f), draw(f + 1)))
            .collect();
        for vd in [20.0, 40.0, 100.0] {
            let (mut worst, mut over4, mut over05, mut n) = (0.0f64, 0, 0, 0);
            for (a, b) in &pairs {
                let before = motion_card_uses();
                let c = found_motion(a, b, vd, false);
                let g = found_motion(a, b, vd, true);
                assert_eq!(motion_card_uses(), before + 1, "the card found the motion");
                for (u, v) in c.0.iter().zip(&g.0).chain(c.1.iter().zip(&g.1)) {
                    let d = (u - v).abs();
                    worst = worst.max(d);
                    over4 += (d > 1e-4) as usize;
                    over05 += (d >= 0.05) as usize;
                    n += 1;
                }
            }
            writeln!(
                table,
                "| {q:?} | {vd} | {n} | {worst:.3e} | {over4} | {over05} |"
            )
            .unwrap();
            worst_all = worst_all.max(worst);
        }
    }

    // The picture bar.
    table.push_str("\n## Pictures, the card's finder against the processor's\n\nEach frame planned for the card as the viewer plans it, once with each finder, both drawn by the processor so only the finder differs. A pixel transparent in both is not counted.\n\n| set | frames | pixels | within 1 level | largest (levels) | float, solid | float, any | motions the card found |\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    let mut all_hold = true;
    let mut found_any = 0;
    for folder in [
        "pixel_motion_blur",
        "timewarp",
        "warp_stabilizer",
        "rolling_shutter_repair",
    ] {
        let mut cost = Cost::default();
        let dir = repo(&format!("Fixtures/{folder}"));
        let mut files: Vec<_> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        files.sort();
        for f in files {
            let name = f.file_name().unwrap().to_string_lossy().into_owned();
            if !name.ends_with(".json") || name.starts_with("expected") {
                continue;
            }
            let Ok(l) = persist::load(&f) else { continue };
            let p = l.document.project().clone();
            let c = &p.compositions[0];
            for frame in c.start_frame..c.start_frame + c.duration_frames as i32 {
                for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                    compare(&mut cost, &p, &dir, frame, q);
                }
            }
        }
        table.push_str(&cost.row(&format!("Fixtures/{folder}, every frame, Full and Draft")));
        all_hold &= cost.holds();
        found_any += cost.card_finds;
    }
    let mut rsr = fixture_effect("rolling_shutter_repair", "fx_rsr_001.json");
    rsr["parameters"]["method"] = json!("pixel_motion");
    for (label, effect) in [
        ("Pixel Motion Blur as added", pmb_added()),
        ("Timewarp speed 50", timewarp_50()),
        (
            "Warp Stabilizer with fx_ws_001's settings",
            fixture_effect("warp_stabilizer", "fx_ws_001.json"),
        ),
        (
            "Rolling Shutter Repair, pixel motion, fx_rsr_001's settings",
            rsr,
        ),
    ] {
        let p = reference_with(effect);
        let mut cost = Cost::default();
        for frame in (0..=210).step_by(30) {
            for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                compare(&mut cost, &p, &root, frame, q);
            }
        }
        table.push_str(&cost.row(&format!(
            "reference shot, {label}, frames 0 to 210 every 30th, Full and Draft"
        )));
        all_hold &= cost.holds();
        assert!(cost.card_finds > 0, "{label}: the card found its motion");
    }
    let mut e = pmb_added();
    e["parameters"]["vector_detail"] = json!(100);
    pictures(&reference_with(e), &root);
    writeln!(table, "\nBars: motion within 1e-4 px (largest here {worst_all:.3e}); pictures at most 1 level on 99.9% of pixels, at most 2 anywhere, float on solid areas at most 1e-4. All hold: {}.", worst_all <= 1e-4 && all_hold).unwrap();
    std::fs::write(
        repo("verification/B-758_card_motion_finder_table.md"),
        &table,
    )
    .unwrap();
    print!("{table}");
    assert!(
        worst_all <= 1e-4,
        "found motion within 1e-4 px of the processor's"
    );
    assert!(all_hold, "pictures within the pixel bar");
    println!("B-758: the card's finder ran {found_any} times in the fixtures");
}

/// The finder alone and the whole viewer frame, with the card's finder and the processor's, in
/// alternating rounds on the reference shot. Writes `verification/B-758_timing_table.md`.
#[test]
#[ignore = "a measurement; run alone on a quiet machine"]
fn b758_timing() {
    use anime_compositor::gpu::Gpu;
    assert!(motion_card(), "this measures the card's finder");
    let mut gpu = Gpu::new().expect("a card for the viewer");
    let root = repo("Fixtures/reference_shot");
    let med = |mut v: Vec<f64>| {
        v.sort_by(f64::total_cmp);
        v[v.len() / 2]
    };
    let rounds = 7;
    let mut table = format!(
        "# B-758: time of the card motion finder (D-878)\n\nWritten by `tests/b758_card_motion_finder.rs` (`b758_timing`, ignored; run alone). {rounds} rounds, the processor's finder and the card's alternating, medians of the rounds. Frames 0 to 210 every 30th of the reference shot.\n\n\
         ## The finder alone (ms per call)\n\n| quality | vector detail | processor | card | faster |\n|---|---:|---:|---:|---:|\n"
    );
    let p = persist::load(&repo("verification/B-08a_project.json"))
        .unwrap()
        .document
        .project()
        .clone();
    let comp = p.compositions[0].id.clone();
    for q in [PreviewQuality::Full, PreviewQuality::Draft] {
        let draw = |f: i32| {
            preview::preview_frame_cached(
                &p,
                &comp,
                f,
                &root,
                q,
                DEFAULT_TILE_SIZE,
                &mut FrameLog::new(3),
                &mut CelCache::viewer(),
            )
            .unwrap()
        };
        let pairs: Vec<_> = (0..=210)
            .step_by(30)
            .map(|f| (draw(f), draw(f + 1)))
            .collect();
        for vd in [20.0, 30.0, 40.0, 50.0, 100.0] {
            found_motion(&pairs[0].0, &pairs[0].1, vd, true);
            let mut t = [vec![], vec![]];
            for _ in 0..rounds {
                for (k, card) in [false, true].into_iter().enumerate() {
                    let s = std::time::Instant::now();
                    for (a, b) in &pairs {
                        found_motion(a, b, vd, card);
                    }
                    t[k].push(s.elapsed().as_secs_f64() * 1e3 / pairs.len() as f64);
                }
            }
            let [c, g] = t.map(med);
            writeln!(table, "| {q:?} | {vd} | {c:.1} | {g:.1} | {:.1}x |", c / g).unwrap();
        }
    }
    table.push_str("\n## The whole viewer frame on the card (ms per frame)\n\n| effect | quality | processor's finder | card's finder | faster |\n|---|---|---:|---:|---:|\n");
    let at = |mut e: Value, vd: i32| {
        e["parameters"]["vector_detail"] = json!(vd);
        Some(e)
    };
    for (label, effect) in [
        ("(none)", None),
        (
            "Pixel Motion Blur as added (vector detail 20)",
            Some(pmb_added()),
        ),
        ("Pixel Motion Blur, vector detail 40", at(pmb_added(), 40)),
        ("Pixel Motion Blur, vector detail 100", at(pmb_added(), 100)),
        ("Timewarp speed 50 (vector detail 20)", Some(timewarp_50())),
        ("Timewarp speed 50, vector detail 40", at(timewarp_50(), 40)),
    ] {
        let p = match effect {
            Some(e) => reference_with(e),
            None => persist::load(&repo("verification/B-08a_project.json"))
                .unwrap()
                .document
                .project()
                .clone(),
        };
        let comp = p.compositions[0].id.clone();
        for q in [PreviewQuality::Full, PreviewQuality::Draft] {
            let mut t = [vec![], vec![]];
            for _ in 0..rounds {
                for (k, off) in [true, false].into_iter().enumerate() {
                    motion_card_off(off);
                    let s = std::time::Instant::now();
                    for frame in (0..=210).step_by(30) {
                        preview::preview_frame_srgb8(
                            &p,
                            &comp,
                            frame,
                            &root,
                            q,
                            DEFAULT_TILE_SIZE,
                            &mut FrameLog::new(3),
                            &mut CelCache::viewer(),
                            &mut gpu,
                        )
                        .unwrap();
                    }
                    t[k].push(s.elapsed().as_secs_f64() * 1e3 / 8.0);
                    motion_card_off(false);
                }
            }
            let [c, g] = t.map(med);
            writeln!(
                table,
                "| {label} | {q:?} | {c:.1} | {g:.1} | {:.2}x |",
                c / g
            )
            .unwrap();
        }
    }
    table.push_str("\nThe finder alone is asked of the card at every setting; in the viewer the card finds motion only at Draft and at Full from vector detail 40 (D-878), so the whole-frame rows at Full below 40 are the processor's finder both times.\n");
    std::fs::write(repo("verification/B-758_timing_table.md"), &table).unwrap();
    print!("{table}");
}
