//! W7 (wave 7, lane d): the time every effect takes at 1920 by 1080, on the processor and on
//! the card, in Draft and Full, in Display and in 8 bpc (After Effects), at the setting it is
//! added with and at one heavy but usual setting. A measurement, run deliberately with
//! `cargo test --release -p anime_compositor_app perf_audit -- --ignored --nocapture`; it writes
//! `verification/W7_effect_performance_raw.md`.
use super::*;

fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = what
        .split_once('?')
        .map_or((what, None), |(i, q)| (i, Some(q)));
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

/// Every effect the core names, read from its own list, as keyed_settings reads it.
fn ids() -> Vec<&'static str> {
    let ids: Vec<&str> = include_str!("../../../src/effects.rs")
        .lines()
        .filter_map(|l| {
            l.strip_prefix("pub const ")?
                .split_once(": &str = \"")?
                .1
                .strip_suffix("\";")
        })
        .filter(|id| id.starts_with("core.") && new_effect(id).is_some_and(|e| !e.is_sound()))
        .collect();
    assert!(ids.len() > 90, "the effect list was not found: {ids:?}");
    ids
}

/// The reference shot with `id` added to layer-2, whose drawing changes every frame, so nothing
/// of the effect's is kept from one frame to the next.
fn with_effect(id: Option<&str>) -> (Viewer, Option<String>) {
    let viewer = Mutex::new(demo());
    let fx = id.map(|id| {
        run(&viewer, &format!("effect.add?layer=layer-2&type={id}"));
        let held = viewer.lock().unwrap();
        let comp = held.document.project().composition(&held.composition).unwrap();
        let e = comp.layer(&Id::new("layer-2")).unwrap().effects.last().unwrap();
        e.instance_id.as_str().to_string()
    });
    (viewer.into_inner().unwrap(), fx)
}

#[test]
#[ignore = "W7: a listing, run deliberately"]
fn perf_audit_dump() {
    for id in ids() {
        let e = new_effect(id).unwrap();
        let names: Vec<String> = e
            .names()
            .into_iter()
            .map(|n| format!("{n}={:?}", e.get(n).unwrap_or_default()))
            .collect();
        println!("{id} | {}", names.join(" "));
    }
}

/// One heavy but usual setting: a reach, blur or softness of at least 40 pixels, and twice the
/// samples, detail or iterations. What changed, for the table; empty when nothing did.
fn heavy(viewer: &Mutex<Viewer>, fx: &str) -> String {
    let current = |name: &str| {
        let held = viewer.lock().unwrap();
        let comp = held.document.project().composition(&held.composition).unwrap();
        let layer = comp.layer(&Id::new("layer-2")).unwrap();
        let e = &layer.effects.iter().find(|e| e.instance_id.as_str() == fx).unwrap().effect;
        (e.names(), e.get(name))
    };
    let reach = ["radius", "blur", "sigma", "softness", "feather", "length", "spread", "reach"];
    let more = ["samples", "complexity", "iterations", "passes", "detail", "quality", "steps"];
    let mut said = Vec::new();
    for name in current("").0 {
        let Some(v) = current(name).1 else { continue };
        if v.len() != 1 {
            continue;
        }
        let n = name.to_ascii_lowercase();
        let to = if reach.iter().any(|w| n.contains(w)) && v[0] < 40.0 {
            40.0
        } else if more.iter().any(|w| n.contains(w)) && v[0] >= 1.0 {
            (v[0] * 2.0).round()
        } else {
            continue;
        };
        let answer = edit_command(
            viewer,
            "property.set_base",
            Some(&format!("layer=layer-2&prop=fx:{fx}:{name}&frame=0&value={to}")),
        );
        if answer.is_some() && current(name).1 == Some(vec![to]) {
            said.push(format!("{name} {to}"));
        }
    }
    said.join(", ")
}

/// The milliseconds of each frame drawn, as the viewer draws it: by the processor when `cpu`,
/// otherwise by the card (which in 8 bpc hands the frame to the processor, D-763). One frame is
/// drawn first and not counted, so the card's passes are made and the cels read.
#[allow(clippy::too_many_arguments)]
fn times(
    gpu: &mut Gpu,
    cache: &mut CelCache,
    cpu: bool,
    project: &Project,
    comp: &Id,
    root: &Path,
    quality: PreviewQuality,
    frames: &[i32],
) -> Vec<f64> {
    gpu.forget();
    let mut out = Vec::new();
    for (i, &frame) in std::iter::once(&239).chain(frames).enumerate() {
        let mut log = FrameLog::new(3);
        let t = Instant::now();
        if cpu {
            drop(
                preview::preview_frame_cached(
                    project,
                    comp,
                    frame,
                    root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    cache,
                )
                .expect("CPU frame")
                .to_srgb8_straight(),
            );
        } else {
            drop(
                preview::preview_frame_srgb8(
                    project,
                    comp,
                    frame,
                    root,
                    quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    cache,
                    gpu,
                )
                .expect("card frame"),
            );
        }
        let ms = t.elapsed().as_secs_f64() * 1000.0;
        if i > 0 {
            out.push(ms);
            // A slow one is measured on three frames, not eight.
            if ms > 3000.0 && out.len() >= 3 {
                break;
            }
        }
    }
    out
}

fn med(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// Whether the card is handed layer-2's effect, at `quality`.
fn on_card(project: &Project, comp: &Id, root: &Path, quality: PreviewQuality) -> bool {
    let mut log = FrameLog::new(3);
    let plan = anime_compositor::compose::plan_frame_for_card(
        project,
        comp,
        30,
        root,
        quality,
        &mut log,
        &mut CelCache::viewer(),
    )
    .expect("plan the frame");
    plan.layers
        .iter()
        .filter(|l| l.id.as_str() == "layer-2")
        .any(|l| !l.on_card.is_empty())
}

fn eight_bpc(project: &Project) -> Project {
    let mut j: serde_json::Value =
        serde_json::from_str(&persist::to_json(project, &Preserved::none())).unwrap();
    for c in j["compositions"].as_array_mut().unwrap() {
        c["eight_bpc"] = serde_json::Value::from(true);
    }
    persist::load_str(&j.to_string())
        .expect("8 bpc")
        .document
        .project()
        .clone()
}

#[test]
#[ignore = "W7: a measurement, run deliberately with --release --ignored"]
fn perf_audit_timing() {
    let out = std::env::var("W7_OUT").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../verification/W7_effect_performance_raw.md")
    });
    let only: Option<Vec<String>> = std::env::var("W7_ONLY")
        .ok()
        .map(|s| s.split(',').map(str::to_string).collect());
    let frames: Vec<i32> = (0..8).map(|i| i * 30).collect();
    let mut gpu = Gpu::new().expect("a usable card");
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- Build: {}\n- Shot: the reference shot, 1920 by \
         1080, the effect on layer-2 (a new drawing every frame), frames 0 to 210 by 30, median ms \
         of the whole frame as the viewer asks for it (sRGB bytes)\n\n\
         | Effect | Setting | Card takes it (Draft/Full) | CPU Draft | CPU Full | Card Draft | \
         Card Full | 8 bpc Draft | 8 bpc Full |\n|---|---|---|---:|---:|---:|---:|---:|---:|\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        if cfg!(debug_assertions) { "debug" } else { "release" },
    );
    std::fs::write(&out, &s).unwrap();
    let mut cache = CelCache::with_budget(8 << 30);
    // The plain shot twice, the first time unrecorded, so every cel is read before a row is.
    let mut list: Vec<Option<&str>> = vec![None, None];
    list.extend(ids().into_iter().map(Some));
    for (at, id) in list.into_iter().enumerate() {
        let name = id.unwrap_or("(none)");
        if only.as_ref().is_some_and(|o| !o.iter().any(|x| x == name)) {
            continue;
        }
        let (viewer, fx) = with_effect(id);
        let viewer = Mutex::new(viewer);
        for pass in 0..2 {
            let setting = if pass == 0 {
                "as added".to_string()
            } else {
                let Some(fx) = &fx else { break };
                let h = heavy(&viewer, fx);
                if h.is_empty() {
                    break;
                }
                h
            };
            let (project, comp, root) = {
                let held = viewer.lock().unwrap();
                (
                    held.document.project().clone(),
                    held.composition.clone(),
                    held.root.clone(),
                )
            };
            let eight = eight_bpc(&project);
            let card = |q| {
                if on_card(&project, &comp, &root, q) {
                    "card"
                } else {
                    "CPU"
                }
            };
            let taken = format!(
                "{}/{}",
                card(PreviewQuality::Draft),
                card(PreviewQuality::Full)
            );
            let mut row = format!("| {name} | {setting} | {taken} |");
            for (cpu, p) in [(true, &project), (false, &project), (false, &eight)] {
                for q in [PreviewQuality::Draft, PreviewQuality::Full] {
                    let t = times(&mut gpu, &mut cache, cpu, p, &comp, &root, q, &frames);
                    row.push_str(&format!(" {:.1} |", med(t)));
                }
            }
            row.push('\n');
            if at == 0 {
                break;
            }
            s.push_str(&row);
            std::fs::write(&out, &s).unwrap();
            eprint!("{row}");
        }
    }
}

/// Every effect's frame 30 at Full, as added and heavy, hashed: the processor's floats, the
/// card's bytes and 8 bpc's bytes. Written to `W7_OUT` before and after a speed change, the two
/// files are the same when no pixel moved.
#[test]
#[ignore = "W7: a check run deliberately with --release --ignored"]
fn perf_audit_hashes() {
    let s: String = hashes().concat();
    std::fs::write(std::env::var("W7_OUT").expect("W7_OUT names the file"), s).unwrap();
}

/// B-660 to B-662: each `W7_ONLY` row's three hashes against `verification/W7_pixel_hashes.txt`,
/// taken at 440cecf0 before any of W7's speed changes, written as a table to `W7_TABLE`. Every
/// row must be the same: a speed change that moves one byte or one float bit fails here.
#[test]
#[ignore = "W7: a check run deliberately with --release --ignored"]
fn perf_audit_unchanged() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../verification");
    let before = std::fs::read_to_string(dir.join("W7_pixel_hashes.txt")).unwrap();
    let before: Vec<&str> = before.lines().collect();
    let out = std::env::var("W7_TABLE").expect("W7_TABLE names the table");
    let mut t = String::from(
        "| Effect | Setting | Processor floats | Card bytes | 8 bpc bytes | Same as 440cecf0 |\n\
         |---|---|---|---|---|---|\n",
    );
    let (mut same, mut all) = (0, 0);
    for row in hashes() {
        let row = row.trim_end();
        let p: Vec<&str> = row.split(" | ").collect();
        let h: Vec<&str> = p[2].split(' ').collect();
        let was = before.iter().find(|b| b.starts_with(&format!("{} | {} | ", p[0], p[1])));
        let ok = was == Some(&row);
        (same, all) = (same + ok as usize, all + 1);
        t.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            p[0],
            p[1],
            h[0],
            h[1],
            h[2],
            if ok { "yes" } else { "NO" }
        ));
    }
    t.push_str(&format!("\n**{same} of {all} rows the same, bit for bit.**\n"));
    std::fs::write(&out, &t).unwrap();
    assert_eq!(same, all, "a speed change moved a pixel: {out}");
}

/// Frame 30 at Full of `(none)` and each effect (or the `W7_ONLY` ones), as added and heavy,
/// one line each: name, setting, then the hashes of the processor's floats, the card's bytes and
/// 8 bpc's bytes.
fn hashes() -> Vec<String> {
    use std::hash::{Hash, Hasher};
    let hash = |bytes: &[u8]| {
        let mut h = std::collections::hash_map::DefaultHasher::new();
        bytes.hash(&mut h);
        h.finish()
    };
    let mut s = Vec::new();
    each_frame(|name, setting, cpu, card, eight| {
        let cpu: Vec<u8> = cpu.data().iter().flat_map(|v| v.to_bits().to_le_bytes()).collect();
        let row = format!(
            "{name} | {setting} | {:016x} {:016x} {:016x}\n",
            hash(&cpu),
            hash(card),
            hash(eight)
        );
        eprint!("{row}");
        s.push(row);
    });
    s
}

/// Frame 30 at Full of `(none)` and each effect (or the `W7_ONLY` ones), as added and heavy,
/// handed to `f`: name, setting, the processor's frame, the card's bytes and 8 bpc's bytes.
fn each_frame(mut f: impl FnMut(&str, &str, &anime_compositor::WorkingBuffer, &[u8], &[u8])) {
    let only: Option<Vec<String>> = std::env::var("W7_ONLY")
        .ok()
        .map(|s| s.split(',').map(str::to_string).collect());
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cache = CelCache::with_budget(8 << 30);
    let mut list: Vec<Option<&str>> = vec![None];
    list.extend(ids().into_iter().map(Some));
    for id in list {
        let name = id.unwrap_or("(none)");
        if only.as_ref().is_some_and(|o| !o.iter().any(|x| x == name)) {
            continue;
        }
        let (viewer, fx) = with_effect(id);
        let viewer = Mutex::new(viewer);
        for pass in 0..2 {
            let setting = if pass == 0 {
                "as added".to_string()
            } else {
                let Some(fx) = &fx else { break };
                let h = heavy(&viewer, fx);
                if h.is_empty() {
                    break;
                }
                h
            };
            let (project, comp, root) = {
                let held = viewer.lock().unwrap();
                (held.document.project().clone(), held.composition.clone(), held.root.clone())
            };
            let q = PreviewQuality::Full;
            let mut log = FrameLog::new(3);
            let cpu = preview::preview_frame_cached(
                &project, &comp, 30, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache,
            )
            .expect("CPU frame");
            let mut on = |p: &Project| {
                gpu.forget();
                preview::preview_frame_srgb8(
                    p, &comp, 30, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache, &mut gpu,
                )
                .expect("card frame")
                .0
            };
            let card = on(&project);
            let eight = on(&eight_bpc(&project));
            f(name, &setting, &cpu, &card, &eight);
        }
    }
}

/// W8 (D-800 to D-802): the pixels of each `W7_ONLY` row's frame 30 at Full, written into the
/// folder `W8_PIXELS` (the processor's floats and its, the card's and 8 bpc's bytes); with
/// `W8_BEFORE` naming a folder an older build wrote, each is measured against it by the pixel-cost
/// bar of the owner's 2026-10-25 request, as a table into `W8_TABLE`: the largest 8-bit
/// difference, the share of pixels within 1 level, the largest float error anywhere and on solid
/// areas, and the largest edge shift (a float difference over the slope beside it, where the
/// slope is at least 0.01 a pixel).
#[test]
#[ignore = "W8: a check run deliberately with --release --ignored"]
fn perf_audit_pixels() {
    let dir = PathBuf::from(std::env::var("W8_PIXELS").expect("W8_PIXELS names a folder"));
    std::fs::create_dir_all(&dir).unwrap();
    let before = std::env::var("W8_BEFORE").ok().map(PathBuf::from);
    let mut t = String::from(
        "| Effect | Setting | Picture | Pixels changed | Largest 8-bit difference | Pixels within 1 level | \
         Pixels over 2 levels | Largest float error | On solid areas | Largest edge shift (px) | \
         Within the bar |\n|---|---|---|---:|---:|---:|---:|---:|---:|---:|---|\n",
    );
    let mut all_ok = true;
    each_frame(|name, setting, cpu, card, eight| {
        let stem: String = format!("{name} {setting}")
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() || c == '.' { c } else { '_' })
            .collect();
        let floats: Vec<u8> = cpu.data().iter().flat_map(|v| v.to_le_bytes()).collect();
        let cpu8 = cpu.to_srgb8_straight();
        let files = [
            ("cpu.f32", floats.as_slice()),
            ("cpu.u8", cpu8.as_slice()),
            ("card.u8", card),
            ("8bpc.u8", eight),
        ];
        for (ext, bytes) in files {
            std::fs::write(dir.join(format!("{stem}.{ext}")), bytes).unwrap();
        }
        let Some(before) = &before else { return };
        let (w, h) = (cpu.width(), cpu.height());
        let read = |ext: &str| std::fs::read(before.join(format!("{stem}.{ext}"))).unwrap();
        for (what, ext, now) in [
            ("processor", "cpu.u8", cpu8.as_slice()),
            ("card", "card.u8", card),
            ("8 bpc", "8bpc.u8", eight),
        ] {
            let was = read(ext);
            let (mut most, mut within, mut over, mut changed) = (0u8, 0usize, 0usize, 0usize);
            for (a, b) in was.chunks_exact(4).zip(now.chunks_exact(4)) {
                let d = a.iter().zip(b).map(|(a, b)| a.abs_diff(*b)).max().unwrap();
                most = most.max(d);
                within += (d <= 1) as usize;
                over += (d > 2) as usize;
                changed += (d > 0) as usize;
            }
            let share = within as f64 / (w * h) as f64;
            let mut row = format!(
                "| {name} | {setting} | {what} | {changed} | {most} | {:.4}% | {over} |",
                100.0 * share
            );
            let mut ok = most <= 2 && share >= 0.999;
            if what == "processor" {
                let was: Vec<f32> = read("cpu.f32")
                    .chunks_exact(4)
                    .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                    .collect();
                let now = cpu.data();
                let (mut err, mut solid, mut shift) = (0f64, 0f64, 0f64);
                for y in 1..h - 1 {
                    for x in 1..w - 1 {
                        for c in 0..4 {
                            let at = |x: usize, y: usize| was[(y * w + x) * 4 + c] as f64;
                            let i = (y * w + x) * 4 + c;
                            let e = (now[i] as f64 - was[i] as f64).abs();
                            let slope = ((at(x + 1, y) - at(x - 1, y)) / 2.0)
                                .hypot((at(x, y + 1) - at(x, y - 1)) / 2.0);
                            err = err.max(e);
                            if slope < 1e-3 {
                                solid = solid.max(e);
                            } else if slope >= 0.01 {
                                shift = shift.max(e / slope);
                            }
                        }
                    }
                }
                ok &= solid <= 1e-4 && shift <= 0.05;
                row.push_str(&format!(" {err:.2e} | {solid:.2e} | {shift:.2e} |"));
            } else {
                row.push_str(" (bytes) | (bytes) | (bytes) |");
            }
            all_ok &= ok;
            row.push_str(if ok { " yes |\n" } else { " NO |\n" });
            eprint!("{row}");
            t.push_str(&row);
        }
    });
    if before.is_some() {
        let out = std::env::var("W8_TABLE").expect("W8_TABLE names the table");
        t.push_str(&format!(
            "\n**{}**\n",
            if all_ok { "Every row within the bar." } else { "A row is outside the bar." }
        ));
        std::fs::write(&out, &t).unwrap();
    }
}

/// Where one frame of each `W7_ONLY` effect goes, by the perf stages, on the processor, on the
/// card and in 8 bpc at Full, after one frame drawn and not counted.
#[test]
#[ignore = "W7: a measurement, run deliberately with --release --ignored"]
fn perf_audit_stages() {
    let only = std::env::var("W7_ONLY").expect("W7_ONLY names the effects");
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cache = CelCache::with_budget(8 << 30);
    anime_compositor::perf::enable();
    for name in only.split(',') {
        let (viewer, _) = with_effect((name != "(none)").then_some(name));
        let project = viewer.document.project().clone();
        let (comp, root) = (viewer.composition.clone(), viewer.root.clone());
        for (how, cpu, p) in [
            ("CPU", true, project.clone()),
            ("card", false, project.clone()),
            ("8bpc", false, eight_bpc(&project)),
        ] {
            let q = PreviewQuality::Full;
            times(&mut gpu, &mut cache, cpu, &p, &comp, &root, q, &[29]);
            anime_compositor::perf::reset();
            let ms = times(&mut gpu, &mut cache, cpu, &p, &comp, &root, q, &[30])[0];
            let mut line = format!("{name} {how} {ms:.1} ms:");
            for (s, nanos, calls) in anime_compositor::perf::snapshot() {
                if nanos > 500_000 {
                    line.push_str(&format!(" {}={:.1}x{calls}", s.label(), nanos as f64 / 1e6));
                }
            }
            eprintln!("{line}");
        }
    }
}

/// B-688 (D-808): every effect in 8 bpc (After Effects), as added and heavy, frames 30 and 120 at
/// Draft and Full, drawn by the processor and by the card, each pixel's largest channel
/// difference in levels of 255. D-808's bar: within 1 level on at least 99.9% of an effect's
/// pixels and within 2 everywhere. Run deliberately with
/// `cargo test --release -p anime_compositor_app perf_audit_eight -- --ignored --nocapture`; it
/// writes `verification/B-688_eight_card_settings_table.md` (or `B688_OUT`).
#[test]
#[ignore = "B-688: a measurement, run deliberately with --release --ignored"]
fn perf_audit_eight() {
    let out = std::env::var("B688_OUT").map(PathBuf::from).unwrap_or_else(|_| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../verification/B-688_eight_card_settings_table.md")
    });
    let only: Option<Vec<String>> = std::env::var("W7_ONLY")
        .ok()
        .map(|s| s.split(',').map(str::to_string).collect());
    let mut gpu = Gpu::new().expect("a usable card");
    let mut cache = CelCache::with_budget(8 << 30);
    let mut s = format!(
        "# B-688 (D-808): 8 bpc (After Effects) on the card against the processor, W7's settings\n\n\
         - Card: {}\n- Shot: W7's, the reference shot at 1920 by 1080 with the effect on layer-2, \
         set to 8 bpc; frames 30 and 120, Draft and Full\n- Bar (D-808): at most 1 level of 255 on \
         at least 99.9% of pixels, at most 2 anywhere\n\n\
         | Effect | Setting | On the card (Draft/Full) | Largest | Pixels over 1 | Pixels over 2 | \
         Within 1 | Bar |\n|---|---|---|---:|---:|---:|---:|---|\n",
        gpu.about(),
    );
    let (mut rows, mut met) = (0, 0);
    let mut list: Vec<Option<&str>> = vec![None];
    list.extend(ids().into_iter().map(Some));
    for id in list {
        let name = id.unwrap_or("(none)");
        if only.as_ref().is_some_and(|o| !o.iter().any(|x| x == name)) {
            continue;
        }
        let (viewer, fx) = with_effect(id);
        let viewer = Mutex::new(viewer);
        for pass in 0..2 {
            let setting = if pass == 0 {
                "as added".to_string()
            } else {
                let Some(fx) = &fx else { break };
                let h = heavy(&viewer, fx);
                if h.is_empty() {
                    break;
                }
                h
            };
            let (project, comp, root) = {
                let held = viewer.lock().unwrap();
                (held.document.project().clone(), held.composition.clone(), held.root.clone())
            };
            let eight = eight_bpc(&project);
            let card = |q| if on_card(&eight, &comp, &root, q) { "card" } else { "CPU" };
            let taken = format!("{}/{}", card(PreviewQuality::Draft), card(PreviewQuality::Full));
            let (mut largest, mut over1, mut over2, mut all) = (0u8, 0usize, 0usize, 0usize);
            for q in [PreviewQuality::Draft, PreviewQuality::Full] {
                for frame in [30, 120] {
                    let mut log = FrameLog::new(3);
                    let cpu = preview::preview_frame_cached(
                        &eight, &comp, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache,
                    )
                    .expect("CPU frame")
                    .to_srgb8_straight();
                    gpu.forget();
                    let mut log = FrameLog::new(3);
                    let (on, ..) = preview::preview_frame_srgb8(
                        &eight, &comp, frame, &root, q, DEFAULT_TILE_SIZE, &mut log, &mut cache,
                        &mut gpu,
                    )
                    .expect("card frame");
                    for (a, b) in cpu.chunks_exact(4).zip(on.chunks_exact(4)) {
                        let d = a.iter().zip(b).map(|(x, y)| x.abs_diff(*y)).max().unwrap_or(0);
                        largest = largest.max(d);
                        over1 += (d > 1) as usize;
                        over2 += (d > 2) as usize;
                        all += 1;
                    }
                }
            }
            let within = 100.0 * (all - over1) as f64 / all as f64;
            let ok = over2 == 0 && (all - over1) * 1000 >= all * 999;
            rows += 1;
            met += ok as usize;
            let row = format!(
                "| {name} | {setting} | {taken} | {largest} | {over1} | {over2} | {within:.4}% | {} |\n",
                if ok { "met" } else { "**NOT MET**" }
            );
            eprint!("{row}");
            s.push_str(&row);
            std::fs::write(&out, &s).unwrap();
        }
    }
    s.push_str(&format!("\n**{met} of {rows} rows meet the bar.**\n"));
    std::fs::write(&out, &s).unwrap();
}
