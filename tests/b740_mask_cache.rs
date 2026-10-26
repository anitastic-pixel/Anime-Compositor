//! B-740 (D-860): the viewer keeps each layer's finished mask coverage (`CelCache::mask_coverage`)
//! instead of working it out again every frame.
//!
//! Writes `verification/B-740_mask_cache_table.md`.
//!
//! What it proves, against renders with no cache at all (`CelCache::none`, what an export uses):
//! - every project in the fixture folders that hold masks draws bit for bit the same through the
//!   viewer's cache, at Full and Draft, on a first pass and on a second one that finds what the
//!   first kept;
//! - on the reference shot, a mask that holds still over a drawing that changes every frame is
//!   found kept, and the frames are bit for bit the uncached ones;
//! - after each edit to anything the coverage depends on (points, a handle, feather, expansion,
//!   mode, invert, opacity, on/off, a mask added, the order, the layer's transform, the
//!   composition's size, Draft/Half/Full, time on a moving mask), the warm viewer cache draws the
//!   uncached picture of the edited project, the edit shows, and undoing it gives back the old
//!   picture.
//!
//! The timing is `b740_timing`, run deliberately with `--release --ignored` (see its comment).

use std::path::{Path, PathBuf};
use std::time::Instant;

use serde_json::{json, Value as J};

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::mask::{self, Mask, MaskMode, MaskPoint};
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// One frame's pixels as bits (or `None` when it could not be drawn) and its diagnostics.
type Frame = (Option<Vec<u32>>, Vec<String>);

fn draw(
    p: &Project,
    comp: &Id,
    f: i32,
    root: &Path,
    q: PreviewQuality,
    cache: &mut CelCache,
) -> Frame {
    let mut log = FrameLog::new(8);
    let r = preview::preview_frame_cached(p, comp, f, root, q, DEFAULT_TILE_SIZE, &mut log, cache);
    let bits = r
        .ok()
        .map(|b| b.as_image().data().iter().map(|v| v.to_bits()).collect());
    let notes = log
        .finish()
        .iter()
        .map(|d| format!("{:?} {}", d.id, d.message))
        .collect();
    (bits, notes)
}

/// With `B740_PICTURES` set to a folder, the warm cache's picture is also written there as a PNG,
/// for the playtest's proof pictures.
fn picture(p: &Project, comp: &Id, f: i32, root: &Path, cache: &mut CelCache, name: &str) {
    let Ok(dir) = std::env::var("B740_PICTURES") else {
        return;
    };
    let mut log = FrameLog::new(8);
    let b = preview::preview_frame_cached(
        p,
        comp,
        f,
        root,
        PreviewQuality::Draft,
        DEFAULT_TILE_SIZE,
        &mut log,
        cache,
    )
    .unwrap();
    anime_compositor::trace::write_png(&Path::new(&dir).join(name), &b, &[]).unwrap();
}

fn fresh(p: &Project, comp: &Id, f: i32, root: &Path, q: PreviewQuality) -> Frame {
    draw(p, comp, f, root, q, &mut CelCache::none())
}

const FOLDERS: [&str; 24] = [
    "audio_spectrum",
    "audio_waveform",
    "display_mix",
    "energy_stroke",
    "energy_stroke_extras",
    "fill",
    "fill_adjustment",
    "inner_outer_key",
    "key_requests",
    "liquify_brushes",
    "mask_smear",
    "masks",
    "open_masks",
    "particle_mapper",
    "particle_mapper_mass",
    "particle_mapper_physics",
    "particle_playground",
    "projects",
    "reshape",
    "reshape_points",
    "scribble",
    "stroke",
    "stretched_values",
    "vegas",
];

/// Every project with a mask in it, drawn through the viewer's cache twice and with none.
fn fixtures(out: &mut String) -> bool {
    out.push_str(
        "\n## Every mask fixture, viewer cache against no cache\n\n\
         Frames: the first three of each composition (fewer if shorter), at Full and Draft, \
         drawn twice through one viewer cache per project. \"Same\" is every pixel's every \
         channel equal to the bit and the same diagnostics.\n\n\
         | Folder | Projects with masks | Frames compared | Coverages found kept | Same |\n| --- | ---: | ---: | ---: | --- |\n",
    );
    let mut all = true;
    for folder in FOLDERS {
        let dir = repo(&format!("Fixtures/{folder}"));
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|e| e == "json"))
            .collect();
        files.sort();
        let (mut projects, mut frames, mut hits, mut same) = (0, 0, 0, true);
        for file in files {
            let text = std::fs::read_to_string(&file).unwrap();
            if !text.contains("\"masks\"") {
                continue;
            }
            let Ok(loaded) = persist::load(&file) else {
                continue;
            };
            let project = loaded.document.project().clone();
            if !project
                .compositions
                .iter()
                .any(|c| c.layers_in_order().any(|l| !l.masks.is_empty()))
            {
                continue;
            }
            projects += 1;
            let mut cache = CelCache::viewer();
            for _pass in 0..2 {
                for comp in &project.compositions {
                    let n = comp.duration_frames.clamp(1, 3) as i32;
                    for f in comp.start_frame..comp.start_frame + n {
                        for q in [PreviewQuality::Full, PreviewQuality::Draft] {
                            let kept = draw(&project, &comp.id, f, &dir, q, &mut cache);
                            let none = fresh(&project, &comp.id, f, &dir, q);
                            frames += 1;
                            if kept != none {
                                same = false;
                                eprintln!("{} {} frame {f} {q:?} differs", file.display(), comp.id);
                            }
                        }
                    }
                }
            }
            hits += cache.mask_coverage_hits();
        }
        all &= same;
        out.push_str(&format!(
            "| {folder} | {projects} | {frames} | {hits} | {} |\n",
            if same { "yes" } else { "**NO**" }
        ));
    }
    all
}

fn mask_json(name: &str, mode: &str, feather: f64, expansion: f64, pts: &[[f64; 2]]) -> J {
    json!({
        "name": name, "enabled": true, "inverted": false, "mode": mode, "opacity": 1.0,
        "feather_px": feather, "expansion_px": expansion,
        "path": {"base": {"points": pts.iter().map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]})).collect::<Vec<_>>()}, "keyframes": []}
    })
}

/// The reference shot with two masks on layer 2, whose drawing changes every frame.
fn reference() -> J {
    let text = std::fs::read_to_string(repo("verification/B-08a_project.json")).unwrap();
    let mut v: J = serde_json::from_str(&text).unwrap();
    v["compositions"][0]["layers"][1]["masks"] = json!([
        mask_json(
            "A",
            "add",
            20.0,
            0.0,
            &[
                [1000.0, 600.0],
                [1300.0, 600.0],
                [1300.0, 1000.0],
                [1000.0, 1000.0]
            ]
        ),
        mask_json(
            "B",
            "subtract",
            6.0,
            10.0,
            &[[1100.0, 700.0], [1350.0, 680.0], [1200.0, 850.0]]
        ),
    ]);
    v
}

fn project(v: &J) -> Project {
    persist::load_str(&v.to_string())
        .unwrap_or_else(|d| panic!("{} {}", d.message, d.detail))
        .document
        .project()
        .clone()
}

fn edits() -> Vec<(&'static str, Box<dyn Fn(&mut J)>)> {
    fn m(v: &mut J, i: usize) -> &mut J {
        &mut v["compositions"][0]["layers"][1]["masks"][i]
    }
    vec![
        (
            "a point moved",
            Box::new(|v| m(v, 0)["path"]["base"]["points"][1]["point"] = json!([1400.0, 700.0])),
        ),
        (
            "a handle pulled",
            Box::new(|v| m(v, 0)["path"]["base"]["points"][1]["out"] = json!([150.0, 100.0])),
        ),
        ("feather", Box::new(|v| m(v, 0)["feather_px"] = json!(60.0))),
        (
            "expansion",
            Box::new(|v| m(v, 1)["expansion_px"] = json!(-30.0)),
        ),
        (
            "mode to Intersect",
            Box::new(|v| m(v, 1)["mode"] = json!("intersect")),
        ),
        (
            "mode to Difference",
            Box::new(|v| m(v, 1)["mode"] = json!("difference")),
        ),
        ("inverted", Box::new(|v| m(v, 0)["inverted"] = json!(true))),
        ("opacity", Box::new(|v| m(v, 0)["opacity"] = json!(0.4))),
        (
            "switched off",
            Box::new(|v| m(v, 1)["enabled"] = json!(false)),
        ),
        (
            "a mask added",
            Box::new(|v| {
                let extra = mask_json(
                    "C",
                    "add",
                    0.0,
                    0.0,
                    &[[1250.0, 850.0], [1450.0, 850.0], [1450.0, 1000.0]],
                );
                v["compositions"][0]["layers"][1]["masks"]
                    .as_array_mut()
                    .unwrap()
                    .push(extra);
            }),
        ),
        (
            "the order swapped",
            Box::new(|v| {
                v["compositions"][0]["layers"][1]["masks"]
                    .as_array_mut()
                    .unwrap()
                    .swap(0, 1)
            }),
        ),
        (
            "the layer moved",
            Box::new(|v| {
                v["compositions"][0]["layers"][1]["transform"]["position"]["base"] =
                    json!([1000.0, 600.0])
            }),
        ),
        (
            "the layer scaled",
            Box::new(|v| {
                v["compositions"][0]["layers"][1]["transform"]["scale"]["base"] =
                    json!([70.0, 70.0])
            }),
        ),
        (
            "the composition's size",
            Box::new(|v| {
                v["compositions"][0]["width"] = json!(1280);
                v["compositions"][0]["height"] = json!(720);
            }),
        ),
    ]
}

/// The reference shot: held masks found kept, then every edit drawn fresh, then undone.
fn reference_edits(out: &mut String) -> bool {
    let root = repo("Fixtures/reference_shot");
    let comp = project(&reference()).compositions[0].id.clone();
    let base = project(&reference());
    let mut cache = CelCache::viewer();
    let mut all = true;
    out.push_str(
        "\n## The reference shot: a mask holding still over a drawing that changes\n\n\
         | Frames | Quality | Coverages found kept | Same as no cache |\n| --- | --- | ---: | --- |\n",
    );
    for q in [PreviewQuality::Draft, PreviewQuality::Full] {
        // A cache of its own for each, or Full would find Draft's masked drawings kept (B-170).
        let mut cache = CelCache::viewer();
        let before = cache.mask_coverage_hits();
        let mut same = true;
        for f in 0..6 {
            same &= draw(&base, &comp, f, &root, q, &mut cache) == fresh(&base, &comp, f, &root, q);
        }
        let hits = cache.mask_coverage_hits() - before;
        all &= same && hits >= 5;
        out.push_str(&format!(
            "| 0 to 5 | {q:?} | {hits} | {} |\n",
            if same { "yes" } else { "**NO**" }
        ));
    }

    out.push_str(
        "\n## Each edit, drawn by the warm viewer cache\n\n\
         Frame 4 at Draft (the last row of each kind also at Full). \"Fresh\": the warm cache's \
         picture of the edited project is the uncached one to the bit. \"Shows\": it differs from \
         the picture before the edit. \"Undone\": the project put back draws its old picture \
         again, from what was kept.\n\n\
         | Edit | Quality | Fresh | Shows | Undone |\n| --- | --- | --- | --- | --- |\n",
    );
    let f = 4;
    for q in [PreviewQuality::Draft, PreviewQuality::Full] {
        let was = fresh(&base, &comp, f, &root, q);
        assert_eq!(draw(&base, &comp, f, &root, q, &mut cache), was);
        for (what, edit) in edits() {
            if q == PreviewQuality::Full
                && !matches!(what, "feather" | "a point moved" | "mode to Intersect")
            {
                continue;
            }
            let mut v = reference();
            edit(&mut v);
            let edited = project(&v);
            let now = draw(&edited, &comp, f, &root, q, &mut cache);
            let ok = now == fresh(&edited, &comp, f, &root, q);
            let shows = now.0 != was.0;
            if q == PreviewQuality::Draft
                && matches!(what, "feather" | "a point moved" | "mode to Intersect")
            {
                picture(
                    &edited,
                    &comp,
                    f,
                    &root,
                    &mut cache,
                    &format!("frame4_{}.png", what.replace(' ', "_")),
                );
            }
            let undone = draw(&base, &comp, f, &root, q, &mut cache) == was;
            if q == PreviewQuality::Draft && what == "feather" {
                picture(
                    &base,
                    &comp,
                    f,
                    &root,
                    &mut cache,
                    "frame4_before_any_edit.png",
                );
            }
            all &= ok && shows && undone;
            let yes = |b: bool| if b { "yes" } else { "**NO**" };
            out.push_str(&format!(
                "| {what} | {q:?} | {} | {} | {} |\n",
                yes(ok),
                yes(shows),
                yes(undone)
            ));
        }
    }

    // Draft, Half and Full in turn on one cache: the same masks at three layer sizes.
    for q in [
        PreviewQuality::Half,
        PreviewQuality::Draft,
        PreviewQuality::Full,
        PreviewQuality::Half,
    ] {
        let ok = draw(&base, &comp, f, &root, q, &mut cache) == fresh(&base, &comp, f, &root, q);
        all &= ok;
        out.push_str(&format!(
            "| quality switched | {q:?} | {} | - | - |\n",
            if ok { "yes" } else { "**NO**" }
        ));
    }

    // Time: mask A keyed to move between frames 0 and 8; frames drawn out of order.
    let mut v = reference();
    let at = |dx: f64| {
        let pts = [
            [1000.0 + dx, 600.0],
            [1300.0 + dx, 600.0],
            [1300.0 + dx, 1000.0],
            [1000.0 + dx, 1000.0],
        ];
        let pts: Vec<J> = pts
            .iter()
            .map(|p| json!({"point": p, "in": [0, 0], "out": [0, 0]}))
            .collect();
        json!({ "points": pts })
    };
    v["compositions"][0]["layers"][1]["masks"][0]["path"]["keyframes"] = json!([
        {"frame": 0, "value": at(0.0), "interp": "linear"},
        {"frame": 8, "value": at(160.0), "interp": "linear"},
    ]);
    let moving = project(&v);
    let mut previous: Option<Frame> = None;
    let mut ok = true;
    let mut shows = true;
    for f in [3, 4, 3, 6, 7, 4] {
        let now = draw(&moving, &comp, f, &root, PreviewQuality::Draft, &mut cache);
        ok &= now == fresh(&moving, &comp, f, &root, PreviewQuality::Draft);
        if let Some(p) = &previous {
            shows &= p.0 != now.0;
        }
        previous = Some(now);
    }
    all &= ok && shows;
    out.push_str(&format!(
        "| time on a moving mask (frames 3, 4, 3, 6, 7, 4) | Draft | {} | {} | - |\n",
        if ok { "yes" } else { "**NO**" },
        if shows { "yes" } else { "**NO**" }
    ));
    all
}

#[test]
fn b740_mask_cache() {
    let mut out = String::from(
        "# B-740: the viewer keeps each mask's finished coverage (D-860)\n\n\
         Written by `tests/b740_mask_cache.rs`. Every comparison is against the same frame drawn \
         with no cache at all, which is how an export draws it.\n",
    );
    let a = reference_edits(&mut out);
    let b = fixtures(&mut out);
    out.push_str(&format!(
        "\n**Result: {}**\n",
        if a && b {
            "every check passed"
        } else {
            "a check FAILED"
        }
    ));
    std::fs::write(repo("verification/B-740_mask_cache_table.md"), &out).unwrap();
    assert!(a && b, "{out}");
}

fn square(x0: f64, mode: MaskMode, feather: f64) -> Mask {
    Mask {
        mode,
        feather_px: feather,
        points: vec![
            MaskPoint::corner(x0, 4.0),
            MaskPoint::corner(x0 + 20.0, 4.0),
            MaskPoint::corner(x0 + 20.0, 20.0),
            MaskPoint::corner(x0, 20.0),
        ],
        ..Mask::default()
    }
}

/// The same masks at two sizes are two coverages, each the function's own answer; and a cache
/// with no effect budget (an export's) keeps none.
#[test]
fn b740_kept_by_size_and_never_for_export() {
    let masks = vec![
        square(3.0, MaskMode::Add, 4.0),
        square(10.0, MaskMode::Subtract, 0.0),
    ];
    let mut cache = CelCache::viewer();
    for _ in 0..2 {
        for (w, h) in [(40, 30), (30, 40), (40, 31)] {
            let kept = cache.mask_coverage(&masks, w, h).unwrap();
            assert_eq!(kept.len(), w * h);
            let fresh = mask::coverage(&masks, w, h).unwrap();
            assert!(kept
                .iter()
                .zip(&fresh)
                .all(|(a, b)| a.to_bits() == b.to_bits()));
        }
    }
    assert_eq!(cache.mask_coverage_hits(), 3);
    let mut export = CelCache::none();
    export.mask_coverage(&masks, 40, 30);
    export.mask_coverage(&masks, 40, 30);
    assert_eq!(export.mask_coverage_hits(), 0);
    assert_eq!(export.effect_held_bytes(), 0);
}

/// Document 27: kept coverages share the effect budget and are let go oldest first.
#[test]
fn b740_coverages_stay_inside_the_budget() {
    let budget = 3 * 4 * 50 * 50;
    let mut cache = CelCache::with_budgets(0, budget);
    for i in 0..10 {
        let masks = vec![square(i as f64, MaskMode::Add, 2.0)];
        cache.mask_coverage(&masks, 50, 50);
        assert!(cache.effect_held_bytes() <= budget);
    }
    // The newest is still kept, the oldest is not.
    cache.mask_coverage(&[square(9.0, MaskMode::Add, 2.0)], 50, 50);
    assert_eq!(cache.mask_coverage_hits(), 1);
    cache.mask_coverage(&[square(0.0, MaskMode::Add, 2.0)], 50, 50);
    assert_eq!(cache.mask_coverage_hits(), 1);
}

/// The measured claim. `W9A_CASES` lists `name|project|media root|frames` a line (W8's format);
/// each frame f is drawn after f+1 on one viewer cache, as `w8_stages` drew it, at Full and at
/// Draft, and the median of the frames is printed. Run the build before the change and after it
/// in alternation: `cargo test --release --test b740_mask_cache b740_timing -- --ignored --nocapture`.
#[test]
#[ignore = "B-740: a measurement, run deliberately with --release --ignored"]
fn b740_timing() {
    let list = std::fs::read_to_string(std::env::var("W9A_CASES").expect("W9A_CASES")).unwrap();
    for l in list
        .lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with('#'))
    {
        let p: Vec<&str> = l.split('|').collect();
        let project = persist::load_str(&std::fs::read_to_string(p[1]).unwrap())
            .unwrap()
            .document
            .project()
            .clone();
        let comp = project.compositions[0].id.clone();
        let root = PathBuf::from(p[2]);
        let frames: Vec<i32> = p[3].split(',').map(|f| f.parse().unwrap()).collect();
        anime_compositor::perf::enable();
        // The window's own cache (a quarter of the machine's memory, D-105) and the smallest
        // the window may have (D-40's gibibyte).
        let auto = anime_compositor::cache::automatic_budget();
        for (budget, q) in [
            (auto, PreviewQuality::Full),
            (auto, PreviewQuality::Draft),
            (1 << 30, PreviewQuality::Full),
            (1 << 30, PreviewQuality::Draft),
        ] {
            let mut cache = CelCache::viewer_sized(budget);
            let (mut took, mut masking) = (Vec::new(), Vec::new());
            for &f in &frames {
                draw(&project, &comp, f + 1, &root, q, &mut cache);
                anime_compositor::perf::reset();
                let t = Instant::now();
                draw(&project, &comp, f, &root, q, &mut cache);
                took.push(t.elapsed().as_secs_f64() * 1000.0);
                let stages = anime_compositor::perf::snapshot();
                let mask = stages.iter().find(|s| s.0.label() == "layer mask");
                masking.push(mask.map_or(0.0, |s| s.1 as f64 / 1e6));
            }
            let med = |mut v: Vec<f64>| {
                v.sort_by(f64::total_cmp);
                v[v.len() / 2]
            };
            println!(
                "TIMING {} {}MB {q:?} frame median {:.1} ms, layer mask median {:.1} ms",
                p[0],
                budget >> 20,
                med(took),
                med(masking)
            );
        }
    }
}
