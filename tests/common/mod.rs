//! What the timing measurements need and none of them owns: this process's memory, the
//! repository root, and the fixture document 08 line 41 declares.
//!
//! Shared by `tests/t06_envelope.rs`, which measures the reference shot,
//! `tests/b12b_declared_fixture.rs`, which measures the fixture document 08 line 41 declares,
//! and `tests/p01_frame_trace.rs`, which measures where one frame of each of them goes.
//! Nothing here was written ahead of a second caller: the memory reader moved here when the
//! second timing test arrived, and [`build_fixture`] moved here when the third did.
//!
//! Every item is `#[allow(dead_code)]`, because a `mod common` is compiled separately into each
//! test binary that includes it and no one of them uses all of it. The alternative is a warning
//! in two binaries for every function the third one needs, which trains the reader to ignore
//! warnings — and `cargo clippy -- -D warnings` is a gate here.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value as J};

use anime_compositor::model::Project;
use anime_compositor::persist;

/// A path inside this repository, whatever directory cargo ran the test from.
pub fn repo(rel: &str) -> PathBuf {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // P-04 made the window's own tests a fourth caller, and the shell is a crate of its own, so
    // this is compiled with `app` as the manifest directory as well as with the root.
    let root = match here.ends_with("app") {
        true => here
            .parent()
            .expect("the app crate has a parent")
            .to_path_buf(),
        false => here,
    };
    root.join(rel)
}

/// Working set and peak working set of this process, in bytes.
///
/// `K32GetProcessMemoryInfo` is declared here rather than pulled in through a crate deliberately.
/// Adding `windows-sys` as a dev-dependency would change what the build resolves, and
/// `docs/DEPENDENCIES.md`, `Licenses/` and `tools/archive_licenses.py --check` are all keyed to
/// that resolution — a licence archive rewritten to take one measurement is a poor trade for ten
/// lines. The function has lived in kernel32 since Windows 7, which is below the floor
/// `docs/SUPPORTED_ENVELOPE.md` declares.
#[cfg(windows)]
mod process_memory {
    /// Windows writes all ten of these; this file reads two. The rest are named rather than padded
    /// over because the struct's layout is the contract, and `cb` is checked against its size.
    #[repr(C)]
    #[derive(Default)]
    #[allow(dead_code)]
    struct Counters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set_size: usize,
        working_set_size: usize,
        quota_peak_paged_pool_usage: usize,
        quota_paged_pool_usage: usize,
        quota_peak_non_paged_pool_usage: usize,
        quota_non_paged_pool_usage: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[allow(non_snake_case)]
    extern "system" {
        fn GetCurrentProcess() -> isize;
        fn K32GetProcessMemoryInfo(process: isize, counters: *mut Counters, cb: u32) -> i32;
    }

    /// `(working set, peak working set)` in bytes, or `None` if the call refused.
    pub fn read() -> Option<(usize, usize)> {
        let mut counters = Counters {
            cb: std::mem::size_of::<Counters>() as u32,
            ..Default::default()
        };
        // Read `cb` out before the call: passing `&mut counters` and `counters.cb` as two
        // arguments of one call is a borrow of the whole struct alongside a read of part of it.
        let cb = counters.cb;
        // Safety: `counters` is a live, correctly sized `PROCESS_MEMORY_COUNTERS` and `cb` is its
        // size; the pseudo-handle from `GetCurrentProcess` needs no closing.
        let ok = unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut counters, cb) };
        (ok != 0).then_some((counters.working_set_size, counters.peak_working_set_size))
    }
}

#[cfg(not(windows))]
mod process_memory {
    pub fn read() -> Option<(usize, usize)> {
        None
    }
}

/// The working set, or 0 if this platform will not say. A zero here is visible in the artifact as a
/// zero rather than as a plausible number.
pub fn working_set() -> usize {
    process_memory::read().map_or(0, |(now, _)| now)
}

pub fn peak_working_set() -> usize {
    process_memory::read().map_or(0, |(_, peak)| peak)
}

/// Which of the reference shot's four sequences each of the ten layers reads a copy of. One
/// background and nine cel layers, which is the shape of the shot the reference already is.
pub const SOURCES: [usize; 10] = [1, 2, 3, 4, 2, 3, 4, 2, 3, 4];
/// Layer 5 is matted by layer 4, layer 8 by layer 7. Two alpha mattes, no cycle.
pub const MATTES: [(usize, usize); 2] = [(5, 4), (8, 7)];

// ---------------------------------------------------------------------------------------
// Building the fixture
// ---------------------------------------------------------------------------------------

/// Each test that builds the fixture gets a root of its own, because cargo runs the tests in one
/// binary in parallel and two of them laying down the same copies would race.
pub fn workload_root(name: &str) -> PathBuf {
    repo("target/b12b_declared_fixture").join(name)
}

/// Copy one of the reference shot's sequences to a directory of its own, so that the render path
/// treats it as a different sequence. Emptied first: a stale copy is a fixture nobody declared.
pub fn lay_down_copy(source: &Path, into: &Path) {
    if into.exists() {
        fs::remove_dir_all(into).unwrap_or_else(|e| panic!("empty {}: {e}", into.display()));
    }
    fs::create_dir_all(into).unwrap_or_else(|e| panic!("make {}: {e}", into.display()));
    for entry in fs::read_dir(source).unwrap_or_else(|e| panic!("read {}: {e}", source.display())) {
        let entry = entry.expect("a directory entry of the reference shot");
        let to = into.join(entry.file_name());
        fs::copy(entry.path(), &to).unwrap_or_else(|e| panic!("copy to {}: {e}", to.display()));
    }
}

/// The declared fixture, as a project file this build can open, plus the root its drawings are
/// under.
///
/// Built out of `verification/B-08a_project.json` rather than written from nothing, so that every
/// exposure sheet here is one this repository already renders — including layer 3's missing
/// drawing 7 and layer 4's out-of-order re-exposure, both of which the copies inherit.
pub fn build_fixture(name: &str) -> (Project, PathBuf, String) {
    let base_text = fs::read_to_string(repo("verification/B-08a_project.json"))
        .expect("read verification/B-08a_project.json");
    let base: J = serde_json::from_str(&base_text).expect("B-08a's artifact is JSON");
    let root = workload_root(name);
    fs::create_dir_all(&root).unwrap_or_else(|e| panic!("make {}: {e}", root.display()));

    let mut assets = Vec::new();
    let mut layers = Vec::new();
    let mut order = Vec::new();

    for (index, source) in SOURCES.iter().enumerate() {
        let n = index + 1;
        let from = format!("layer{source}");
        let to = format!("copy{n}");
        lay_down_copy(
            &repo("Fixtures/reference_shot").join(&from),
            &root.join(&to),
        );

        let mut asset = base["assets"][source - 1].clone();
        asset["id"] = json!(format!("asset-{n}"));
        asset["name"] = json!(to);
        asset["pattern"] = json!(asset["pattern"]
            .as_str()
            .expect("every asset of the reference shot is a sequence")
            .replace(&from, &to));
        let frames: BTreeMap<String, J> = asset["frames"]
            .as_object()
            .expect("an image sequence has an explicit frame list")
            .iter()
            .map(|(drawing, file)| {
                let file = file.as_str().expect("a frame maps a drawing to a file");
                (
                    drawing.clone(),
                    json!(file.replacen(&format!("{from}/"), &format!("{to}/"), 1)),
                )
            })
            .collect();
        asset["frames"] = json!(frames);
        assets.push(asset);

        let mut layer = base["compositions"][0]["layers"][source - 1].clone();
        layer["id"] = json!(format!("layer-{n}"));
        layer["name"] = json!(to);
        layer["asset_id"] = json!(format!("asset-{n}"));
        layer["matte"] = J::Null;
        layer["effects"] = json!([]);
        layers.push(layer);
        order.push(format!("layer-{n}"));
    }

    for (on, from) in MATTES {
        layers[on - 1]["matte"] = json!({
            // The only mode the format has: `persist` accepts "alpha" and nothing else.
            "mode": "alpha",
            "layer_id": format!("layer-{from}"),
            "matte_only": false,
        });
    }

    // Three simple effect instances, one of each kind this build has, on three different layers.
    layers[1]["effects"] = json!([{
        "instance_id": "fx-exposure",
        "type_id": "core.exposure",
        "enabled": true,
        "parameters": { "stops": 0.5 },
    }]);
    layers[5]["effects"] = json!([{
        "instance_id": "fx-blur",
        "type_id": "core.gaussian_blur",
        "enabled": true,
        "parameters": { "sigma_px": 4.0 },
    }]);
    layers[8]["effects"] = json!([{
        "instance_id": "fx-tint",
        "type_id": "core.tint",
        "enabled": true,
        "parameters": { "color": [0.9, 0.7, 0.5], "amount": 0.3 },
    }]);

    let mut project = base.clone();
    project["project_id"] = json!("proj-t06-declared-fixture");
    project["assets"] = json!(assets);
    project["compositions"][0]["layer_order"] = json!(order);
    project["compositions"][0]["layers"] = json!(layers);

    let text = serde_json::to_string_pretty(&project).expect("the fixture serialises");

    let loaded = persist::load_str(&text).unwrap_or_else(|d| {
        panic!(
            "the fixture this test wrote, this build cannot open: {} -- {}",
            d.message, d.detail
        )
    });
    (loaded.document.project().clone(), root, text)
}
