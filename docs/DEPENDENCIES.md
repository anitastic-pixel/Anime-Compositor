# Dependency record

ADR-005: "Every dependency records purpose, version and license before a distributable build."
Document 10 additionally asks for upstream URL, license text, NOTICE files, modifications, build
flags, distribution form and reviewer/date, and says the list must be generated from the build
inputs rather than guessed.

The table below is generated from `cargo metadata` against the committed `Cargo.lock`, so it is
the resolved graph rather than a hand-kept list. It is **not** the software bill of materials
document 10 requires at distribution: license *texts* and NOTICE files are not vendored here, and
no legal reviewer has signed anything. Both arrive with B-11 (packaging) and T-16.

Distribution form: a single self-contained Windows executable, statically linked, open source
(ADR-010). No dependency is modified. No non-default build flags are set.

Generated 2026-09-04 from `Cargo.lock`. 11 crates.

| Crate | Version | License | Kind | Upstream |
|---|---|---|---|---|
| `adler2` | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | transitive | https://github.com/oyvindln/adler2 |
| `bitflags` | 2.13.1 | MIT OR Apache-2.0 | transitive | https://github.com/bitflags/bitflags |
| `cfg-if` | 1.0.4 | MIT OR Apache-2.0 | transitive | https://github.com/rust-lang/cfg-if |
| `crc32fast` | 1.5.1 | MIT OR Apache-2.0 | transitive | https://github.com/srijs/rust-crc32fast |
| `fdeflate` | 0.3.7 | MIT OR Apache-2.0 | transitive | https://github.com/image-rs/fdeflate |
| `flate2` | 1.1.10 | MIT OR Apache-2.0 | transitive | https://github.com/rust-lang/flate2-rs |
| `miniz_oxide` | 0.9.1 | MIT OR Zlib OR Apache-2.0 | transitive | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `miniz_oxide` | 0.8.9 | MIT OR Zlib OR Apache-2.0 | transitive | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide |
| `png` | 0.18.1 | MIT OR Apache-2.0 | direct | https://github.com/image-rs/image-png |
| `simd-adler32` | 0.3.10 | MIT | transitive | https://github.com/mcountryman/simd-adler32 |
| `zlib-rs` | 0.6.7 | Zlib | transitive | https://github.com/trifectatechfoundation/zlib-rs |

## Purpose

`png` is the only dependency this project chose. It decodes PNG for B-03 import and will encode
it for B-08 export. Everything else in the table is something `png` pulled in: the DEFLATE
implementations (`flate2`, `miniz_oxide`, `zlib-rs`, `fdeflate`), the checksum crates
(`crc32fast`, `adler2`, `simd-adler32`) a PNG codec needs, and `bitflags` and `cfg-if`.

Writing a PNG decoder by hand was considered and rejected. PNG is a format with real security
surface (risk K-07, unsafe input), and a widely used decoder has had far more malformed files
thrown at it than this project ever will. ADR-005 prefers owned code to a dependency, but its
stated reason is the cost of understanding, updating and license-reviewing one forever; a
hand-written codec for a compressed binary format inverts that trade.

## License compatibility

Every license above is permissive and compatible with open-source distribution under ADR-010.
No copyleft and no source-availability obligation beyond attribution. This is an engineering read
of the license identifiers, not a legal opinion; document 10 is explicit that qualified review is
still required before distribution.
