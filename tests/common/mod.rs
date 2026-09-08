//! What two timing measurements both need and neither owns: this process's memory, and the
//! repository root.
//!
//! Shared by `tests/t06_envelope.rs`, which measures the reference shot, and
//! `tests/b12b_declared_fixture.rs`, which measures the fixture document 08 line 41 declares.
//! Extracted when the second one arrived rather than written ahead of it.

use std::path::PathBuf;

/// A path inside this repository, whatever directory cargo ran the test from.
pub fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
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
