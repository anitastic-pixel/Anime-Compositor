# Dependency record

B-11's dependency and licence record. This file supersedes the short generated table that stood
here before; it is the single dependency record ADR-005 asks for, and `.gitignore` names it as the
reason `Cargo.lock` is committed.

`tests/b11_dependency_record.rs` checks this file against `Cargo.lock` in both directions and
writes `verification/B-11_record_table.md`. The table is produced by `tools/gen_dependencies.py` from
`cargo metadata` and `Cargo.lock` rather than typed, because document 10 asks for a bill of
materials "generated from the final build inputs rather than a guessed list". The prose sections
are written by hand.

Distribution form: statically linked, open source (ADR-010, D-03), under `MIT OR Apache-2.0`
(D-31, decided 2026-09-05). No dependency is modified. No non-default build flags are set. Reviewer: none. Date reviewed: none. Both are blank on purpose;
see the last section.

## Bill of materials

Every crate the build resolves, at the version it resolved. `miniz_oxide` appears twice because
two majors of it are in the graph at once, reached by different dependants; that is not an error,
and the check compares whole version sets per crate so that it stays visible.

| Crate | Version | Declared licence | Role | Form | Upstream | crates.io SHA-256 |
|---|---|---|---|---|---|---|
| `adler2` | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | transitive | linked | https://github.com/oyvindln/adler2 | `320119579fcad9c2…` |
| `bitflags` | 2.13.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/bitflags/bitflags | `b588b76d00fde796…` |
| `cfg-if` | 1.0.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/cfg-if | `9330f8b2ff13f345…` |
| `crc32fast` | 1.5.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/srijs/rust-crc32fast | `8498c871161e1742…` |
| `crossbeam-deque` | 0.8.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `5181e0de7b61eb03…` |
| `crossbeam-epoch` | 0.9.20 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `2d6914041f254d6e…` |
| `crossbeam-utils` | 0.8.22 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `61803da095bee82a…` |
| `either` | 1.18.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rayon-rs/either | `252afb9ae5eaa683…` |
| `fdeflate` | 0.3.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/image-rs/fdeflate | `1e6853b52649d4ac…` |
| `flate2` | 1.1.10 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/flate2-rs | `6e634e2e0ebac1ee…` |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/itoa | `8f42a60cbdf9a97f…` |
| `memchr` | 2.8.3 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/memchr | `cf8baf1c55e62ffc…` |
| `miniz_oxide` | 0.8.9 | MIT OR Zlib OR Apache-2.0 | transitive | linked | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide | `1fa76a2c86f704bd…` |
| `miniz_oxide` | 0.9.1 | MIT OR Zlib OR Apache-2.0 | transitive | linked | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide | `b63fbc4a50860e98…` |
| `png` | 0.18.1 | MIT OR Apache-2.0 | direct | linked | https://github.com/image-rs/image-png | `60769b8b31b2a9f2…` |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/dtolnay/proc-macro2 | `985e7ec9bb745e6c…` |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/dtolnay/quote | `1fbf4db142a473a8…` |
| `rayon` | 1.12.0 | MIT OR Apache-2.0 | direct | linked | https://github.com/rayon-rs/rayon | `fb39b166781f92d4…` |
| `rayon-core` | 1.13.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rayon-rs/rayon | `22e18b0f0062d30d…` |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | transitive | linked | https://github.com/serde-rs/serde | `4148590afebada38…` |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | transitive | linked | https://github.com/serde-rs/serde | `67dca2c9c51e58a4…` |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/serde-rs/serde | `e7a5d71263a5a7d4…` |
| `serde_json` | 1.0.151 | MIT OR Apache-2.0 | direct | linked | https://github.com/serde-rs/json | `c841b55ecdae098c…` |
| `simd-adler32` | 0.3.10 | MIT | transitive | linked | https://github.com/mcountryman/simd-adler32 | `3a219298ac11a56e…` |
| `syn` | 3.0.5 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/dtolnay/syn | `12df2e0110f65b77…` |
| `unicode-ident` | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | transitive | build-time only | https://github.com/dtolnay/unicode-ident | `e6e4313cd5fcd3da…` |
| `zlib-rs` | 0.6.7 | Zlib | transitive | linked | https://github.com/trifectatechfoundation/zlib-rs | `34b31d188d9d685a…` |
| `zmij` | 1.0.23 | MIT | transitive | linked | https://github.com/dtolnay/zmij | `29666d0abbfad1e3…` |

## Purpose — why each of the three is here

`Cargo.toml` names three dependencies by hand. Everything else in the table above arrived
underneath one of them.

- **`png`** decodes the cel images the compositor reads and encodes the frames it exports. PNG is
  the format the reference shot is drawn in and the format document 21 names for export. Writing a
  PNG encoder that is correct about bit depth, alpha and interlacing is not work this project has
  any reason to do.
- **`rayon`** renders frames in parallel. A 240-frame export is 240 independent compositions, and
  the export path is the only place it is used.
- **`serde_json`** reads and writes the project file. The format is JSON by ADR-008; the
  alternative is a hand-written parser, which is a source of silent data loss and the one failure
  this project is least able to tolerate.

## The three that need a reviewer's eye

Named, not decided. Document 10 reserves that judgement: "legal conclusions requiring professional
judgment should be recorded by the appropriate reviewer."

- **`unicode-ident` 1.0.24** — `(MIT OR Apache-2.0) AND Unicode-3.0`. The `AND` is the point: this
  is not a choice of one licence, both sets of terms apply. It is build-time only, which likely
  changes the answer, but "likely" is not a review.
- **`zlib-rs` 0.6.7** — `Zlib`, with no MIT or Apache alternative offered. The only crate in the
  graph whose terms are not a choice.
- **`memchr` 2.8.3** — `Unlicense OR MIT`. The Unlicense is a public-domain dedication whose
  standing differs by jurisdiction, and document 10 records that distribution jurisdictions are
  still open.

## Licence compatibility, as an engineering read

Not a legal opinion. Every crate above offers permissive terms, and all but `zlib-rs` offer MIT or
Apache-2.0 among them. Nothing in the graph is copyleft, so nothing here constrains what licence
this project's own code may carry. The archived texts under `Licenses/` are what a distribution
would have to carry with it; a check confirms one exists for every crate at its resolved version,
and it checks only that, never what the text says.

## What this record does not yet contain

- **A reviewer and a date.** There has been no legal reviewer. Inventing a sign-off would be worse
  than leaving it blank.
- **NOTICE files.** Document 10 lists them separately from licence texts. No crate in this graph
  ships one, but that was read off the archived directories, not verified by a reviewer.
- **A distribution.** T-16 stays NOT RUN because there is no distributable build to check. Nothing
  here has been shipped to anyone, so no obligation in it has come due.
- **A signed-off review.** D-31 is now closed - the project is `MIT OR Apache-2.0`, `Cargo.toml`
  declares it and `LICENSE-MIT` and `LICENSE-APACHE` are in the repository root - but that is the
  owner choosing a licence, not a reviewer confirming that this graph may be redistributed under
  it. The copyright line in `LICENSE-MIT` names the GitHub identity that owns the repository; the
  owner should replace it with whatever name belongs on the notice.
