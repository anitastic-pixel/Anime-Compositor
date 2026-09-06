# B-11 licence archive, checked against the crates the build resolved

The artifact is the directory tree under `Licenses/`. This is the check that keeps it true, and it answers a narrower question than `verification/B-11_record_table.md` does. That one asks whether every crate has an archived directory. This one asks whether what is inside each directory is what the crate actually ships. A hand-maintained archive passes the first and quietly fails the second.

Produced by `tools/archive_licenses.py --report`, from `cargo metadata` and the crate sources cargo unpacked to build with. Nothing is downloaded and no file list is typed. `tools/archive_licenses.py --check` runs in CI and fails the build if the two ever part company.

**28 crates resolved. 0 disagree with the archive. 0 ship no licence text at all.**

## What is archived

| Crate | Archived files |
|---|---|
| `adler2-2.0.1` | LICENSE-0BSD, LICENSE-APACHE, LICENSE-MIT |
| `bitflags-2.13.1` | LICENSE-APACHE, LICENSE-MIT |
| `cfg-if-1.0.4` | LICENSE-APACHE, LICENSE-MIT |
| `crc32fast-1.5.1` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-deque-0.8.7` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-epoch-0.9.20` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-utils-0.8.22` | LICENSE-APACHE, LICENSE-MIT |
| `either-1.18.0` | LICENSE-APACHE, LICENSE-MIT |
| `fdeflate-0.3.7` | LICENSE-APACHE, LICENSE-MIT |
| `flate2-1.1.10` | LICENSE-APACHE, LICENSE-MIT |
| `itoa-1.0.18` | LICENSE-APACHE, LICENSE-MIT |
| `memchr-2.8.3` | COPYING, LICENSE-MIT, UNLICENSE |
| `miniz_oxide-0.8.9` | LICENSE, LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `miniz_oxide-0.9.1` | LICENSE, LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `png-0.18.1` | LICENSE-APACHE, LICENSE-MIT |
| `proc-macro2-1.0.107` | LICENSE-APACHE, LICENSE-MIT |
| `quote-1.0.47` | LICENSE-APACHE, LICENSE-MIT |
| `rayon-1.12.0` | LICENSE-APACHE, LICENSE-MIT |
| `rayon-core-1.13.0` | LICENSE-APACHE, LICENSE-MIT |
| `serde-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde_core-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde_derive-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde_json-1.0.151` | LICENSE-APACHE, LICENSE-MIT |
| `simd-adler32-0.3.10` | LICENSE.md |
| `syn-3.0.5` | LICENSE-APACHE, LICENSE-MIT |
| `unicode-ident-1.0.24` | LICENSE-APACHE, LICENSE-MIT, LICENSE-UNICODE |
| `zlib-rs-0.6.7` | LICENSE |
| `zmij-1.0.23` | LICENSE-MIT |

This check reads no licence and decides nothing about one. Document 10 reserves that for a reviewer, and there has not been one.
