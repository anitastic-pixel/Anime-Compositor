# Research 33 evidence and verification

Research-only verification, September 9, 2026. This artifact checks arithmetic, citation numbering, and selected source anchors. It is not a rendering fixture or performance benchmark.

## Arithmetic and document checks

Expected numbers below were written as report arithmetic, independently of the compositor. No renderer output generated an expectation. Equality applies to the stated rounded values.

| Check | Expected | Actual | Result |
|---|---:|---:|---|
| 1080p RGBA32F bytes | 33177600 | 33177600 | PASS |
| 4K RGBA32F bytes | 132710400 | 132710400 | PASS |
| 8K RGBA32F MiB | 506.25 | 506.25 | PASS |
| 240 full-HD RGBA32F frames, GiB rounded | 7.42 | 7.42 | PASS |
| 240 UHD RGBA32F frames, GiB rounded | 29.66 | 29.66 | PASS |
| 240 UHD RGBA8 frames, GiB rounded | 7.42 | 7.42 | PASS |
| 166 full-HD RGBA32F cels, GiB rounded | 5.13 | 5.13 | PASS |
| 166 draft RGBA32F cels, GiB rounded | 0.321 | 0.321 | PASS |
| 4K60 RGBA8 payload, decimal GB/s rounded | 1.991 | 1.991 | PASS |
| 24 fps deadline ms rounded | 41.667 | 41.667 | PASS |
| Tenth-loop median/deadline rounded | 6.34 | 6.34 | PASS |
| First-loop aggregate fps rounded | 4.3 | 4.3 | PASS |
| First-loop reciprocal median fps rounded | 4.14 | 4.14 | PASS |
| Half of work accelerated tenfold, speedup rounded | 1.82 | 1.82 | PASS |
| Numbered external source definitions | 23 | 23 | PASS |
| Undefined numbered citations | 0 | 0 | PASS |

## Source anchors

These confirm cited text exists at the inspected checkout; they do not prove runtime behavior. The lock lifetime, data flow, and dependency observations were inspected separately.

| File | Expected anchor | Actual location | Result |
|---|---|---|---|
| `src/cache.rs` | `pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;` | 73 | PASS |
| `src/cache.rs` | `let buffer = entry.1.clone();` | 163 | PASS |
| `src/compose.rs` | `crate::mask::apply(&mut source, mask);` | 341 | PASS |
| `src/compose.rs` | `crate::effects::apply_stack(&mut source` | 352 | PASS |
| `src/render.rs` | `pub a: f64,` | 36 | PASS |
| `src/render.rs` | `.par_iter()` | 274 | PASS |
| `src/render.rs` | `for layer in &plan.layers {` | 293 | PASS |
| `app/src/main.rs` | `let mut pixels = buffer.to_srgb8_straight();` | 351 | PASS |
| `app/ui/index.html` | `if (inFlight) return;` | 476 | PASS |
| `verification/T-06_declared_fixture.md` | `&#124; 10 &#124; 60922.6 &#124; 264.17 &#124; 319.42 &#124;` | 53 | PASS |
| `verification/T-06_declared_fixture.md` | `&#124; The frame just shown, again &#124; 1 GiB` | 70 | PASS |

## Provenance

- Inspected source baseline: `d609310a003a13a9af5b947210388dd23230685d`.
- HEAD at verification: `4a8d5b963de2887311038038bc278bef143a0fdd`. HEAD changed during this research through other work; this research made no commits or branch changes.
- Relevant source/build/T-06 differences from baseline: NONE (PASS). This additional check guards against source drift during research.
- Research report SHA-256: `536984d98cf8de9153ae1a9df1d49cdef65b078a9ffae720f7c7baa64141dc81`.
- Regeneration: run `verification/derive_research33.py` with Python 3 from this repository; only this research evidence file is written.
- Vendor manuals were downloaded to ignored `target/performance-research/`; relevant text was extracted with bundled pypdf 6.10.0. No dependency was added to the product.
- `resolve.pdf` source snapshot SHA-256: `1e4b76b52f637bb3d704342f3788d4506d92055af8075a22ccc4460ba72ad457`; 209,018,074 bytes. Original URLs and versions are recorded in report sources 6 and 7.
- `fusion.pdf` source snapshot SHA-256: `afd2543e65c203e997a7f3f42c54a4ff856bb3a78b3e57a81ccfd8958a558e81`; 63,978,605 bytes. Original URLs and versions are recorded in report sources 6 and 7.

## What was and was not run

- Ran: 16 arithmetic/document checks and 11 source-anchor checks; 0 failed.
- Read: governing rendering/cache/architecture contracts, relevant ADRs, source paths, historical verification records, document 32, and cited external sources. Resolve chapter 8 and selected Fusion sections were read, not the entire manuals.
- Not run: Rust unit/integration fixtures, ignored T-06 performance tests, GPU experiments, window responsiveness benchmarks, export comparisons, AE/Resolve application benchmarks, or new screenshot/color validation. This request was research and production code was not changed.
- No new measured speedup, GPU utilization, memory peak, codec throughput, or 64 GB-machine performance result is claimed. Historical timings remain labeled historical.
- Hardware inventory attempt: Windows CIM/storage queries returned Access denied. The supplied CPU/GPU/64 GB specification is the planning input; storage and runtime headroom were not established. The queries were optional and no elevation was sought.
- PDF text extraction initially encountered a console encoding error after successfully saving the Resolve contents; subsequent extraction to UTF-8 files succeeded. PyMuPDF was unavailable; bundled pypdf was used. These are research-tool limitations, not product failures.
- No expected values in Fixtures or document 25 were edited; no production source, dependency, schema, accepted ADR, or document 32 was changed.
- The report includes a Mermaid architecture diagram. No new exported rendering images or screenshots are supplied or implied.

The report proposes future verification artifacts for implementation work. This research artifact does not close R-06, T-06, or any other implementation requirement.
