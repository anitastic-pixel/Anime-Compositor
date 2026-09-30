//! B-171: a composition shown inside another is kept once drawn, in memory and on disk, so the
//! viewer does not draw it again, even after the program is closed and opened again (GPU plan
//! G12, D-243). The reference shot, with two blurs to make it heavy, shown whole in an
//! outer composition; a copy of its drawings in `target/b171_root`, and the viewer's disk setting
//! pointed at `target/b171_disk`, emptied first. At Full and Draft, frames 0, 1 and 100:
//!
//! 1. the viewer's frame is, byte for byte, the frame a cache that keeps nothing draws (the
//!    export's), and so is every frame below;
//! 2. drawn again, the inner composition is not drawn again: no drawing is asked for;
//! 3. the disk folder holds a copy of the inner frame;
//! 4. a new viewer (the program opened again) with the same folder asks for no drawing either;
//! 5. an edit to the outer layer alone still asks for no drawing;
//! 6. an edit inside the inner composition is drawn, and matches;
//! 7. a drawing file touched on disk makes the inner composition be drawn again.
//!
//! Writes `verification/B-171_precomp_table.md`.

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::cache::{CelCache, DiskCache};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

const OUTER: &str = "comp-b171-outer";

/// The reference shot made heavy, with an outer composition showing it; `inner` and `outer` are
/// the opacities of an inner layer and of the outer composition layer.
fn nested(inner: f64, outer: f64) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    let comp = &mut j["compositions"][0];
    comp["layers"][0]["effects"] = json!([{"instance_id": "b171-b", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": 24.0}}]);
    comp["layers"][2]["effects"] = json!([{"instance_id": "b171-g", "type_id": "core.gaussian_blur", "enabled": true,
        "parameters": {"sigma_px": 16.0}}]);
    comp["layers"][3]["transform"]["opacity"] = json!({"base": inner, "keyframes": []});
    let v = |x: Value| json!({"base": x, "keyframes": []});
    let outer_comp = json!({
        "id": OUTER, "name": "outer", "width": 1920, "height": 1080, "pixel_aspect_ratio": 1,
        "frame_rate": {"numerator": 24, "denominator": 1}, "start_frame": 0, "duration_frames": 240,
        "layer_order": ["pre"],
        "layers": [{"id": "pre", "kind": "composition", "name": "pre", "composition_id": "comp-reference-shot",
            "enabled": true, "locked": false, "in_frame": 0, "out_frame": 240, "source_offset_frames": 0,
            "transform": {"anchor": v(json!([960.0, 540.0])), "position": v(json!([960.0, 540.0])),
                "scale": v(json!([100, 100])), "rotation": v(json!(0)), "opacity": v(json!(outer))},
            "mask": null, "matte": null, "blend_mode": "normal", "effects": []}]
    });
    j["compositions"].as_array_mut().expect("compositions").push(outer_comp);
    persist::load_str(&j.to_string()).unwrap_or_else(|d| panic!("{}", d.message)).document.project().clone()
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("make the copy's folder");
    for entry in fs::read_dir(from).expect("list the fixture") {
        let entry = entry.expect("a fixture entry");
        let to = to.join(entry.file_name());
        if entry.file_type().expect("a file type").is_dir() {
            copy_dir(&entry.path(), &to);
        } else {
            fs::copy(entry.path(), to).expect("copy a fixture file");
        }
    }
}

fn files_in(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        if entry.file_type().is_ok_and(|t| t.is_dir()) {
            out.extend(files_in(&entry.path()));
        } else {
            out.push(entry.path());
        }
    }
    out
}

fn draw(project: &Project, frame: i32, root: &Path, quality: PreviewQuality, cache: &mut CelCache) -> (Vec<u8>, u64) {
    let asked = cache.hits() + cache.misses();
    let mut log = FrameLog::new(3);
    let picture = preview::preview_frame_cached(project, &Id::new(OUTER), frame, root, quality, DEFAULT_TILE_SIZE, &mut log, cache)
        .expect("the frame draws");
    assert!(log.finish().is_empty(), "frame {frame} said something");
    (picture.to_srgb8_straight(), cache.hits() + cache.misses() - asked)
}

fn viewer(disk: &Path) -> CelCache {
    let mut cache = CelCache::viewer();
    cache.set_disk(Some(DiskCache { folder: disk.to_path_buf(), cap: 20_000_000_000 }));
    cache
}

#[test]
fn a_composition_inside_another_is_kept() {
    let root = repo("target/b171_root");
    let disk = repo("target/b171_disk");
    let _ = fs::remove_dir_all(&root);
    let _ = fs::remove_dir_all(&disk);
    copy_dir(&repo("Fixtures/reference_shot"), &root);
    let (base, outer_edit, inner_edit) = (nested(1.0, 1.0), nested(1.0, 0.8), nested(0.5, 1.0));
    let export = |project: &Project, frame: i32, quality: PreviewQuality| draw(project, frame, &root, quality, &mut CelCache::none()).0;

    let mut rows = String::new();
    let (mut checks, mut passed) = (0, 0);
    let mut row = |quality: PreviewQuality, frame: i32, what: &str, asked: u64, same: bool, ok: bool| {
        checks += 1;
        passed += ok as usize;
        writeln!(
            rows,
            "| {} | {frame} | {what} | {asked} | {} | {} |",
            quality.label(),
            if same { "yes" } else { "NO" },
            if ok { "pass" } else { "FAIL" }
        )
        .unwrap();
    };
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        for frame in [0, 1, 100] {
            let want = export(&base, frame, quality);
            let mut first = viewer(&disk);
            let (got, asked) = draw(&base, frame, &root, quality, &mut first);
            row(quality, frame, "drawn the first time", asked, got == want, got == want && asked > 0);
            let (got, asked) = draw(&base, frame, &root, quality, &mut first);
            row(quality, frame, "drawn again", asked, got == want, got == want && asked == 0);
            // B-171b: the copy is written by a worker; wait for it.
            anime_compositor::cache::disk_writes_done();
            let copies = files_in(&disk).iter().filter(|p| p.extension().is_some_and(|e| e == "frame")).count();
            row(quality, frame, &format!("copies of composition frames on disk: {copies}"), 0, true, copies > 0);
            let mut reopened = viewer(&disk);
            let (got, asked) = draw(&base, frame, &root, quality, &mut reopened);
            row(quality, frame, "drawn by a new viewer, the program opened again", asked, got == want, got == want && asked == 0);
            let want_outer = export(&outer_edit, frame, quality);
            let (got, asked) = draw(&outer_edit, frame, &root, quality, &mut reopened);
            row(quality, frame, "the outer layer's opacity edited", asked, got == want_outer, got == want_outer && asked == 0);
            let want_inner = export(&inner_edit, frame, quality);
            let (got, asked) = draw(&inner_edit, frame, &root, quality, &mut reopened);
            row(quality, frame, "an inner layer's opacity edited", asked, got == want_inner, got == want_inner && asked > 0);
            // Every drawing file dated a minute later: the same pixels, but not the same files.
            for file in files_in(&root.join("layer1")) {
                let f = fs::File::options().write(true).open(&file).expect("open a drawing");
                let then = f.metadata().and_then(|m| m.modified()).expect("a modified time");
                f.set_modified(then + std::time::Duration::from_secs(60)).expect("date a drawing");
            }
            let mut touched = viewer(&disk);
            let (got, asked) = draw(&base, frame, &root, quality, &mut touched);
            row(quality, frame, "drawn after the drawings were dated anew", asked, got == want, got == want && asked > 0);
        }
    }
    let table = format!(
        "# B-171: a composition inside another kept\n\n\
         The reference shot, with a Gaussian Blur (24 px) on its first layer and another (16 px) on its third \
         to make it heavy, shown whole in an outer composition. The viewer keeps what it draws of \
         the inner composition in memory and in its disk folder. Each row draws one frame of the \
         outer composition. \"Drawings asked for\" counts the drawings the frame asked the cache \
         for: none means the inner composition was not drawn at all. A row passes when the frame \
         is, byte for byte, the frame a cache that keeps nothing (the export's) draws, and the \
         inner composition was drawn, or not, as the row says it should be.\n\n\
         **{passed} of {checks} pass.**\n\n\
         | Quality | Frame | What | Drawings asked for | Same bytes | Result |\n\
         |---|---:|---|---:|---|---|\n{rows}"
    );
    fs::write(repo("verification/B-171_precomp_table.md"), table).expect("write the B-171 table");
    assert_eq!(passed, checks, "see verification/B-171_precomp_table.md");
}

/// The timing behind `verification/B-171_timing_table.md`: frames 0 to 23 of the outer
/// composition drawn as the window draws them, on the card, with the viewer's cache at the size
/// Automatic gives it on this machine. Median milliseconds of a frame, per pass.
#[test]
#[ignore]
fn b171_timing() {
    use anime_compositor::gpu::Gpu;
    let mut gpu = Gpu::new().expect("a usable card");
    let root = repo("target/b171_root");
    let disk = repo("target/b171_disk");
    let _ = fs::remove_dir_all(&root);
    copy_dir(&repo("Fixtures/reference_shot"), &root);
    let (base, outer_edit) = (nested(1.0, 1.0), nested(1.0, 0.8));
    let mut rows = String::new();
    for quality in [PreviewQuality::Full, PreviewQuality::Draft] {
        let _ = fs::remove_dir_all(&disk);
        let fresh = || {
            let mut cache = CelCache::viewer_sized(anime_compositor::cache::automatic_budget());
            cache.set_disk(Some(DiskCache { folder: disk.clone(), cap: 20_000_000_000 }));
            cache
        };
        let pass = |project: &Project, cache: &mut CelCache, gpu: &mut Gpu| {
            let mut ms: Vec<f64> = (0..24)
                .map(|frame| {
                    let mut log = FrameLog::new(3);
                    let at = std::time::Instant::now();
                    preview::preview_frame_srgb8(project, &Id::new(OUTER), frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, cache, gpu)
                        .expect("the frame draws");
                    at.elapsed().as_secs_f64() * 1000.0
                })
                .collect();
            ms.sort_by(f64::total_cmp);
            ms[ms.len() / 2]
        };
        gpu.forget();
        let mut viewer = fresh();
        let first = pass(&base, &mut viewer, &mut gpu);
        let again = pass(&base, &mut viewer, &mut gpu);
        let edited = pass(&outer_edit, &mut viewer, &mut gpu);
        gpu.forget();
        let reopened = pass(&base, &mut fresh(), &mut gpu);
        writeln!(rows, "| {} | {first:.1} | {again:.1} | {edited:.1} | {reopened:.1} |", quality.label()).unwrap();
    }
    fs::write(
        repo("verification/B-171_timing_raw.md"),
        format!("| Quality | First play (ms) | Played again (ms) | Outer layer edited (ms) | Program opened again (ms) |\n|---|---:|---:|---:|---:|\n{rows}"),
    )
    .expect("write the timing");
}
