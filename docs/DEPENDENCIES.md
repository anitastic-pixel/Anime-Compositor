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

Every crate the build resolves for `x86_64-pc-windows-msvc`, at the version it resolved.

The platform matters to what this list means. Asked about every platform at once, cargo answers
with 435 crates, and 171 of those are the macOS and Linux windowing stacks that this build never
compiles. Naming them here would describe a program nobody has. ADR-001 makes Windows the only
supported platform and CI runs `windows-latest`, so the record is filtered to match.

`miniz_oxide` appears twice because two majors of it are in the graph at once, reached by different
dependants; that is not an error, and the check compares whole version sets per crate so that it
stays visible. Several other crates now do the same for the same reason.

| Crate | Version | Declared licence | Role | Form | Upstream | crates.io SHA-256 |
|---|---|---|---|---|---|---|
| `adler2` | 2.0.1 | 0BSD OR MIT OR Apache-2.0 | transitive | linked | https://github.com/oyvindln/adler2 | `320119579fcad9c2…` |
| `aho-corasick` | 1.1.5 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/aho-corasick | `c982642fa9e86060…` |
| `alloc-no-stdlib` | 2.0.4 | BSD-3-Clause | transitive | linked | https://github.com/dropbox/rust-alloc-no-stdlib | `cc7bb162ec39d46a…` |
| `alloc-stdlib` | 0.2.4 | BSD-3-Clause | transitive | linked | https://github.com/dropbox/rust-alloc-no-stdlib | `0e76a019e91224d2…` |
| `anyhow` | 1.0.104 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/anyhow | `330a5ed07fa54e47…` |
| `autocfg` | 1.5.1 | Apache-2.0 OR MIT | transitive | build-time only | https://github.com/cuviper/autocfg | `f2032f911046de80…` |
| `base64` | 0.22.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/marshallpierce/rust-base64 | `72b3254f16251a83…` |
| `bit-set` | 0.8.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/contain-rs/bit-set | `08807e080ed7f9d5…` |
| `bit-vec` | 0.8.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/contain-rs/bit-vec | `5e764a1d40d510da…` |
| `bitflags` | 1.3.2 | MIT/Apache-2.0 | transitive | linked | https://github.com/bitflags/bitflags | `bef38d45163c2f1d…` |
| `bitflags` | 2.13.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/bitflags/bitflags | `b588b76d00fde796…` |
| `block-buffer` | 0.10.4 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/RustCrypto/utils | `3078c7629b62d3f0…` |
| `brotli` | 8.0.4 | BSD-3-Clause AND MIT | transitive | linked | https://github.com/dropbox/rust-brotli | `5cc91aac060a7a1e…` |
| `brotli-decompressor` | 5.0.3 | BSD-3-Clause/MIT | transitive | linked | https://github.com/dropbox/rust-brotli-decompressor | `3a32acac15fe1967…` |
| `bs58` | 0.5.1 | MIT/Apache-2.0 | transitive | linked | https://github.com/Nullus157/bs58-rs | `bf88ba1141d185c3…` |
| `byteorder` | 1.5.0 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/byteorder | `1fd0f2584146f6f2…` |
| `bytes` | 1.12.1 | MIT | transitive | linked | https://github.com/tokio-rs/bytes | `fc652a48c352aef3…` |
| `camino` | 1.2.5 | MIT OR Apache-2.0 | transitive | linked | https://github.com/camino-rs/camino | `bb1307f12aa967b5…` |
| `cargo-platform` | 0.1.9 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/cargo | `e35af189006b9c0f…` |
| `cargo_metadata` | 0.19.2 | MIT | transitive | linked | https://github.com/oli-obk/cargo_metadata | `dd5eb614ed4c27c5…` |
| `cargo_toml` | 0.22.3 | Apache-2.0 OR MIT | transitive | build-time only | https://gitlab.com/lib.rs/cargo_toml | `374b7c592d9c00c1…` |
| `cc` | 1.4.5 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/rust-lang/cc-rs | `005ec2760ca554fa…` |
| `cfb` | 0.7.3 | MIT | transitive | linked | https://github.com/mdsteele/rust-cfb | `d38f2da7a0a2c4cc…` |
| `cfg-if` | 1.0.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/cfg-if | `9330f8b2ff13f345…` |
| `chrono` | 0.4.45 | MIT OR Apache-2.0 | transitive | linked | https://github.com/chronotope/chrono | `1aa79e62e7697b8e…` |
| `cookie` | 0.18.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/SergioBenitez/cookie-rs | `1a373e3602691c3c…` |
| `cpufeatures` | 0.2.17 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/RustCrypto/utils | `59ed5838eebb26a2…` |
| `crc32fast` | 1.5.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/srijs/rust-crc32fast | `8498c871161e1742…` |
| `crossbeam-channel` | 0.5.17 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `98b0cc327b5bc766…` |
| `crossbeam-deque` | 0.8.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `5181e0de7b61eb03…` |
| `crossbeam-epoch` | 0.9.20 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `2d6914041f254d6e…` |
| `crossbeam-utils` | 0.8.22 | MIT OR Apache-2.0 | transitive | linked | https://github.com/crossbeam-rs/crossbeam | `61803da095bee82a…` |
| `crypto-common` | 0.1.7 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/RustCrypto/traits | `78c8292055d1c1df…` |
| `cssparser` | 0.36.0 | MPL-2.0 | transitive | linked | https://github.com/servo/rust-cssparser | `dae61cf9c0abb83b…` |
| `cssparser-macros` | 0.6.1 | MPL-2.0 | transitive | linked | https://github.com/servo/rust-cssparser | `13b588ba4ac1a99f…` |
| `ctor` | 0.8.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/mmastrac/rust-ctor | `352d39c2f7bef1d6…` |
| `ctor-proc-macro` | 0.0.7 | Apache-2.0 OR MIT | transitive | linked | https://github.com/mmastrac/rust-ctor | `52560adf09603e58…` |
| `darling` | 0.23.0 | MIT | transitive | build-time only | https://github.com/TedDriggs/darling | `25ae13da2f202d56…` |
| `darling_core` | 0.23.0 | MIT | transitive | build-time only | https://github.com/TedDriggs/darling | `9865a50f7c335f53…` |
| `darling_macro` | 0.23.0 | MIT | transitive | build-time only | https://github.com/TedDriggs/darling | `ac3984ec7bd6cfa7…` |
| `defmt` | 1.1.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/knurling-rs/defmt | `e2953bfe4f93bbd2…` |
| `defmt-macros` | 1.1.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/knurling-rs/defmt | `bad9c72e7ca2137e…` |
| `defmt-parser` | 1.0.0 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/knurling-rs/defmt | `10d60334b3b2e7c9…` |
| `deranged` | 0.5.8 | MIT OR Apache-2.0 | transitive | linked | https://github.com/jhpratt/deranged | `7cd812cc2bc1d69d…` |
| `derive_more` | 2.1.1 | MIT | transitive | linked | https://github.com/JelteF/derive_more | `d751e9e49156b02b…` |
| `derive_more-impl` | 2.1.1 | MIT | transitive | linked | https://github.com/JelteF/derive_more | `799a97264921d862…` |
| `digest` | 0.10.7 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/RustCrypto/traits | `9ed9a281f7bc9b75…` |
| `dirs` | 6.0.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/soc/dirs-rs | `c3e8aa94d7514122…` |
| `dirs-sys` | 0.5.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dirs-dev/dirs-sys-rs | `e01a3366d27ee989…` |
| `displaydoc` | 0.2.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/yaahc/displaydoc | `c6232dd377dcc647…` |
| `dom_query` | 0.27.0 | MIT | transitive | linked | https://github.com/niklak/dom_query | `521e380c0c8afb8d…` |
| `dpi` | 0.1.2 | Apache-2.0 AND MIT | transitive | linked | https://github.com/rust-windowing/winit | `d8b14ccef22fc6f5…` |
| `dtoa` | 1.0.11 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/dtoa | `4c3cf4824e2d5f02…` |
| `dtoa-short` | 0.3.5 | MPL-2.0 | transitive | linked | https://github.com/upsuper/dtoa-short | `cd1511a7b6a56299…` |
| `dtor` | 0.3.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/mmastrac/rust-ctor | `f1057d6c64987086…` |
| `dtor-proc-macro` | 0.0.6 | Apache-2.0 OR MIT | transitive | linked | https://github.com/mmastrac/rust-ctor | `f678cf4a922c215c…` |
| `dunce` | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | transitive | linked | https://gitlab.com/kornelski/dunce | `92773504d58c093f…` |
| `dyn-clone` | 1.0.20 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/dyn-clone | `d0881ea181b1df73…` |
| `either` | 1.18.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rayon-rs/either | `252afb9ae5eaa683…` |
| `embed-resource` | 3.0.11 | MIT | transitive | build-time only | https://github.com/nabijaczleweli/rust-embed-resource | `fbfdaacccebec3b2…` |
| `equivalent` | 1.0.2 | Apache-2.0 OR MIT | transitive | linked | https://github.com/indexmap-rs/equivalent | `877a4ace8713b0bc…` |
| `erased-serde` | 0.4.10 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/erased-serde | `d2add8a07dd6a8d9…` |
| `fastrand` | 2.5.0 | Apache-2.0 OR MIT | transitive | build-time only | https://github.com/smol-rs/fastrand | `da7c62ceae207dd3…` |
| `fdeflate` | 0.3.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/image-rs/fdeflate | `1e6853b52649d4ac…` |
| `find-msvc-tools` | 0.1.12 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/rust-lang/cc-rs | `3e0f1c7c3a72c66f…` |
| `flate2` | 1.1.10 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/flate2-rs | `6e634e2e0ebac1ee…` |
| `fnv` | 1.0.7 | Apache-2.0 / MIT | transitive | linked | https://github.com/servo/rust-fnv | `3f9eec918d3f2406…` |
| `foldhash` | 0.2.0 | Zlib | transitive | linked | https://github.com/orlp/foldhash | `77ce24cb58228fbb…` |
| `form_urlencoded` | 1.2.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/rust-url | `cb4cb245038516f5…` |
| `generic-array` | 0.14.7 | MIT | transitive | build-time only | https://github.com/fizyk20/generic-array.git | `85649ca51fd72272…` |
| `getrandom` | 0.3.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-random/getrandom | `899def5c37c4fd7b…` |
| `getrandom` | 0.4.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-random/getrandom | `300e883d756b2e4e…` |
| `glob` | 0.3.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/glob | `e4eba85ea1d0a966…` |
| `hashbrown` | 0.12.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/hashbrown | `8a9ee70c43aaf417…` |
| `hashbrown` | 0.17.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/hashbrown | `ed5909b6e89a2db4…` |
| `heck` | 0.5.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/withoutboats/heck | `2304e00983f87ffb…` |
| `hex` | 0.4.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/KokaKiwi/rust-hex | `7f24254aa9a54b5c…` |
| `html5ever` | 0.38.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/html5ever | `1054432bae2f14e0…` |
| `http` | 1.5.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/hyperium/http | `918d3568bebf3527…` |
| `ico` | 0.5.0 | MIT | transitive | build-time only | https://github.com/mdsteele/rust-ico | `3e795dff5605e0f0…` |
| `icu_collections` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `fa68d21081c4a05d…` |
| `icu_locale_core` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `d56e28588da92eee…` |
| `icu_normalizer` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `12f9cf5f235641ed…` |
| `icu_normalizer_data` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `1563da1ed3e0b3bf…` |
| `icu_properties` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `7e7ca276ad314566…` |
| `icu_properties_data` | 2.3.0 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `e590f038c1464a96…` |
| `icu_provider` | 2.3.1 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `d27bbb9d3abbefac…` |
| `ident_case` | 1.0.1 | MIT/Apache-2.0 | transitive | build-time only | https://github.com/TedDriggs/ident_case | `b9e0384b61958566…` |
| `idna` | 1.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/rust-url/ | `3b0875f23caa0389…` |
| `idna_adapter` | 1.2.2 | Apache-2.0 OR MIT | transitive | linked | https://github.com/hsivonen/idna_adapter | `cb68373c0d6620ef…` |
| `indexmap` | 1.9.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/bluss/indexmap | `bd070e393353796e…` |
| `indexmap` | 2.14.2 | Apache-2.0 OR MIT | transitive | linked | https://github.com/indexmap-rs/indexmap | `cc4e190f5d26ca70…` |
| `infer` | 0.19.0 | MIT | transitive | linked | https://github.com/bojand/infer | `a588916bfdfd92e7…` |
| `itoa` | 1.0.18 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/itoa | `8f42a60cbdf9a97f…` |
| `jiff` | 0.2.35 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/jiff | `668b7183bd07af9a…` |
| `jiff-core` | 0.1.0 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/jiff | `7feca88439efe53d…` |
| `jiff-tzdb` | 0.1.8 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/jiff | `142bd39932ad231f…` |
| `jiff-tzdb-platform` | 0.1.3 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/jiff | `875a5a69ac2bab1a…` |
| `json-patch` | 3.0.1 | MIT/Apache-2.0 | transitive | linked | https://github.com/idubrov/json-patch | `863726d7afb6bc25…` |
| `jsonptr` | 0.6.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/chanced/jsonptr | `5dea2b27dd239b25…` |
| `keyboard-types` | 0.7.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/pyfisch/keyboard-types | `b750dcadc39a09db…` |
| `libc` | 0.2.189 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/libc | `3eaf3ede3fee6db1…` |
| `litemap` | 0.8.3 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `47d9d19d1d6efa01…` |
| `lock_api` | 0.4.14 | MIT OR Apache-2.0 | transitive | linked | https://github.com/Amanieu/parking_lot | `224399e74b87b5f3…` |
| `log` | 0.4.34 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/log | `f9f8bd3e56ce4dfc…` |
| `markup5ever` | 0.38.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/html5ever | `8983d30f2915feea…` |
| `memchr` | 2.8.3 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/memchr | `cf8baf1c55e62ffc…` |
| `mime` | 0.3.17 | MIT OR Apache-2.0 | transitive | linked | https://github.com/hyperium/mime | `6877bb514081ee2a…` |
| `miniz_oxide` | 0.8.9 | MIT OR Zlib OR Apache-2.0 | transitive | linked | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide | `1fa76a2c86f704bd…` |
| `miniz_oxide` | 0.9.1 | MIT OR Zlib OR Apache-2.0 | transitive | linked | https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide | `b63fbc4a50860e98…` |
| `mio` | 1.2.3 | MIT | transitive | linked | https://github.com/tokio-rs/mio | `4b18443e9c262bfe…` |
| `muda` | 0.19.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/muda | `1dd04e60bc0b0743…` |
| `new_debug_unreachable` | 1.0.6 | MIT | transitive | linked | https://github.com/mbrubeck/rust-debug-unreachable | `650eef8c711430f1…` |
| `num-conv` | 0.2.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/jhpratt/num-conv | `521739c6d2bac4aa…` |
| `num-traits` | 0.2.19 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-num/num-traits | `071dfc062690e90b…` |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/matklad/once_cell | `9f7c3e4beb33f85d…` |
| `option-ext` | 0.2.0 | MPL-2.0 | transitive | linked | https://github.com/soc/option-ext.git | `04744f49eae99ab7…` |
| `parking_lot` | 0.12.5 | MIT OR Apache-2.0 | transitive | linked | https://github.com/Amanieu/parking_lot | `93857453250e3077…` |
| `parking_lot_core` | 0.9.12 | MIT OR Apache-2.0 | transitive | linked | https://github.com/Amanieu/parking_lot | `2621685985a2ebf1…` |
| `percent-encoding` | 2.3.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/rust-url/ | `9b4f627cb1b25917…` |
| `phf` | 0.13.1 | MIT | transitive | linked | https://github.com/rust-phf/rust-phf | `c1562dc717473dba…` |
| `phf_codegen` | 0.13.1 | MIT | transitive | build-time only | https://github.com/rust-phf/rust-phf | `49aa7f9d80421bca…` |
| `phf_generator` | 0.13.1 | MIT | transitive | build-time only | https://github.com/rust-phf/rust-phf | `135ace3a761e564e…` |
| `phf_macros` | 0.13.1 | MIT | transitive | linked | https://github.com/rust-phf/rust-phf | `812f032b54b1e759…` |
| `phf_shared` | 0.13.1 | MIT | transitive | linked | https://github.com/rust-phf/rust-phf | `e57fef6bc5981e38…` |
| `pin-project-lite` | 0.2.17 | Apache-2.0 OR MIT | transitive | linked | https://github.com/taiki-e/pin-project-lite | `a89322df9ebe1c15…` |
| `plist` | 1.10.0 | MIT | transitive | linked | https://github.com/ebarnard/rust-plist/ | `7da1d65da6dd5d1e…` |
| `png` | 0.17.16 | MIT OR Apache-2.0 | direct | build-time only | https://github.com/image-rs/image-png | `82151a2fc869e011…` |
| `png` | 0.18.1 | MIT OR Apache-2.0 | direct | linked | https://github.com/image-rs/image-png | `60769b8b31b2a9f2…` |
| `potential_utf` | 0.1.6 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `d83eb9bc6d8e5cf5…` |
| `powerfmt` | 0.2.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/jhpratt/powerfmt | `439ee305def115ba…` |
| `precomputed-hash` | 0.1.1 | MIT | transitive | linked | https://github.com/emilio/precomputed-hash | `925383efa3467304…` |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/proc-macro2 | `985e7ec9bb745e6c…` |
| `quick-xml` | 0.41.0 | MIT | transitive | linked | https://github.com/tafia/quick-xml | `e660451e55124f79…` |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/quote | `1fbf4db142a473a8…` |
| `raw-window-handle` | 0.6.2 | MIT OR Apache-2.0 OR Zlib | transitive | linked | https://github.com/rust-windowing/raw-window-handle | `20675572f6f24e9e…` |
| `rayon` | 1.12.0 | MIT OR Apache-2.0 | direct | linked | https://github.com/rayon-rs/rayon | `fb39b166781f92d4…` |
| `rayon-core` | 1.13.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rayon-rs/rayon | `22e18b0f0062d30d…` |
| `ref-cast` | 1.0.27 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/ref-cast | `7e440fb4e4b41472…` |
| `ref-cast-impl` | 1.0.27 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/ref-cast | `92ecd8964f845372…` |
| `regex` | 1.13.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/regex | `f020237b6c8eed93…` |
| `regex-automata` | 0.4.18 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/regex | `ad8553b9b2641325…` |
| `regex-syntax` | 0.8.11 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/regex | `d6f6ff9a378485b2…` |
| `rfd` | 0.16.0 | MIT | transitive | linked | https://github.com/PolyMeilex/rfd | `a15ad77d9e70a924…` |
| `rustc-hash` | 2.1.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/rust-lang/rustc-hash | `6b1e7f9a428571be…` |
| `rustc_version` | 0.4.1 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/djc/rustc-version-rs | `cfcb3a22ef46e85b…` |
| `same-file` | 1.0.6 | Unlicense/MIT | transitive | linked | https://github.com/BurntSushi/same-file | `93fc1dc3aaa9bfed…` |
| `schemars` | 0.8.22 | MIT | transitive | linked | https://github.com/GREsau/schemars | `3fbf2ae1b8bc8e02…` |
| `schemars` | 0.9.0 | MIT | transitive | linked | https://github.com/GREsau/schemars | `4cd191f9397d57d5…` |
| `schemars` | 1.2.2 | MIT | transitive | linked | https://github.com/GREsau/schemars | `687274d293b6cdc6…` |
| `schemars_derive` | 0.8.22 | MIT | transitive | linked | https://github.com/GREsau/schemars | `32e265784ad61888…` |
| `scopeguard` | 1.2.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/bluss/scopeguard | `94143f37725109f9…` |
| `selectors` | 0.36.1 | MPL-2.0 | transitive | linked | https://github.com/servo/stylo | `c5d9c0c92a92d33f…` |
| `semver` | 1.0.28 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/semver | `8a7852d02fc84898…` |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | transitive | linked | https://github.com/serde-rs/serde | `4148590afebada38…` |
| `serde-untagged` | 0.1.9 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/serde-untagged | `f9faf48a4a2d2693…` |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | transitive | linked | https://github.com/serde-rs/serde | `67dca2c9c51e58a4…` |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | transitive | linked | https://github.com/serde-rs/serde | `e7a5d71263a5a7d4…` |
| `serde_derive_internals` | 0.29.1 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/serde-rs/serde | `18d26a20a969b9e3…` |
| `serde_json` | 1.0.151 | MIT OR Apache-2.0 | direct | linked | https://github.com/serde-rs/json | `c841b55ecdae098c…` |
| `serde_repr` | 0.1.21 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/serde-repr | `8d3b1629de253c70…` |
| `serde_spanned` | 1.1.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/toml-rs/toml | `6662b5879511e06e…` |
| `serde_with` | 3.22.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/jonasbb/serde_with/ | `ee78f1fbe43ac4a0…` |
| `serde_with_macros` | 3.22.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/jonasbb/serde_with/ | `8705578779c2b6bd…` |
| `serialize-to-javascript` | 0.1.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/chippers/serialize-to-javascript | `04f3666a07a197cd…` |
| `serialize-to-javascript-impl` | 0.1.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/chippers/serialize-to-javascript | `772ee033c0916d67…` |
| `servo_arc` | 0.4.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/stylo | `170fb83ab34de17d…` |
| `sha2` | 0.10.9 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/RustCrypto/hashes | `a7507d819769d01a…` |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/comex/rust-shlex | `f8fadd59c855ef20…` |
| `simd-adler32` | 0.3.10 | MIT | transitive | linked | https://github.com/mcountryman/simd-adler32 | `3a219298ac11a56e…` |
| `siphasher` | 1.0.3 | MIT/Apache-2.0 | transitive | linked | https://github.com/jedisct1/rust-siphash | `8ee5873ec9cce019…` |
| `smallvec` | 1.16.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/rust-smallvec | `b9be42f50aa861c5…` |
| `socket2` | 0.6.5 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-lang/socket2 | `c3d1e2c7f27f8d4c…` |
| `softbuffer` | 0.4.8 | MIT OR Apache-2.0 | transitive | linked | https://github.com/rust-windowing/softbuffer | `aac18da81ebbf051…` |
| `stable_deref_trait` | 1.2.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/storyyeller/stable_deref_trait | `6ce2be8dc25455e1…` |
| `string_cache` | 0.9.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/string-cache | `a18596f8c785a729…` |
| `string_cache_codegen` | 0.6.1 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/servo/string-cache | `585635e46db23105…` |
| `strsim` | 0.11.1 | MIT | transitive | build-time only | https://github.com/rapidfuzz/strsim-rs | `7da8b5736845d9f2…` |
| `syn` | 2.0.119 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/dtolnay/syn | `872831b642d1a079…` |
| `syn` | 3.0.5 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/dtolnay/syn | `12df2e0110f65b77…` |
| `synstructure` | 0.13.2 | MIT | transitive | build-time only | https://github.com/mystor/synstructure | `728a70f3dbaf5bab…` |
| `tao` | 0.35.3 | Apache-2.0 | transitive | linked | https://github.com/tauri-apps/tao | `d1c93047acf68669…` |
| `tauri` | 2.11.5 | Apache-2.0 OR MIT | direct | linked | https://github.com/tauri-apps/tauri | `667b20e2726d572d…` |
| `tauri-build` | 2.6.3 | Apache-2.0 OR MIT | direct | build-time only | https://github.com/tauri-apps/tauri | `bc9ce40b16101cb6…` |
| `tauri-codegen` | 2.6.3 | Apache-2.0 OR MIT | transitive | build-time only | https://github.com/tauri-apps/tauri | `08279169ff42f8fc…` |
| `tauri-macros` | 2.6.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/tauri | `e8b394794f399a42…` |
| `tauri-plugin` | 2.6.3 | Apache-2.0 OR MIT | transitive | build-time only | https://github.com/tauri-apps/tauri | `74be5dd4bed9afbd…` |
| `tauri-plugin-dialog` | 2.7.3 | Apache-2.0 OR MIT | direct | linked | https://github.com/tauri-apps/plugins-workspace | `61854a36651aa483…` |
| `tauri-plugin-fs` | 2.5.2 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/plugins-workspace | `de22eef34fd78c0d…` |
| `tauri-runtime` | 2.11.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/tauri | `b0b4bc95aed361b0…` |
| `tauri-runtime-wry` | 2.11.4 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/tauri | `4e6fac707727b7a2…` |
| `tauri-utils` | 2.9.3 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/tauri | `3e176a18e6776492…` |
| `tauri-winres` | 0.3.6 | MIT | transitive | build-time only | https://github.com/tauri-apps/winres | `cc65d45c68858bfe…` |
| `tendril` | 0.5.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/html5ever | `5fed54709c5b3a53…` |
| `thiserror` | 1.0.69 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/thiserror | `b6aaf5339b578ea8…` |
| `thiserror` | 2.0.20 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/thiserror | `ec86235f5fcc2a73…` |
| `thiserror-impl` | 1.0.69 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/thiserror | `4fee6c4efc90059e…` |
| `thiserror-impl` | 2.0.20 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/thiserror | `bc04cd3e1236dd4a…` |
| `time` | 0.3.55 | MIT OR Apache-2.0 | transitive | linked | https://github.com/time-rs/time | `cdb87b95ec50ddfa…` |
| `time-core` | 0.1.9 | MIT OR Apache-2.0 | transitive | linked | https://github.com/time-rs/time | `9e1c906769ad99c8…` |
| `time-macros` | 0.2.32 | MIT OR Apache-2.0 | transitive | linked | https://github.com/time-rs/time | `7e689342a48d2ea9…` |
| `tinystr` | 0.8.4 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `b1e27c91459209c2…` |
| `tinyvec` | 1.13.2 | Zlib OR Apache-2.0 OR MIT | transitive | linked | https://github.com/Lokathor/tinyvec | `4cf0ded5c4e56918…` |
| `tinyvec_macros` | 0.1.1 | MIT OR Apache-2.0 OR Zlib | transitive | linked | https://github.com/Soveu/tinyvec_macros | `1f3ccbac311fea05…` |
| `tokio` | 1.53.1 | MIT | transitive | linked | https://github.com/tokio-rs/tokio | `202caea871b69668…` |
| `toml` | 0.9.12+spec-1.1.0 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/toml-rs/toml | `cf92845e79fc2e2d…` |
| `toml` | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/toml-rs/toml | `12c0ba9680044b4c…` |
| `toml_datetime` | 0.7.5+spec-1.1.0 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/toml-rs/toml | `92e1cfed4a3038bc…` |
| `toml_datetime` | 1.1.1+spec-1.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/toml-rs/toml | `3165f65f62e28e01…` |
| `toml_parser` | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/toml-rs/toml | `1d38ac1cf9b95fac…` |
| `toml_writer` | 1.1.2+spec-1.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/toml-rs/toml | `7d56353a2a665ad0…` |
| `tracing` | 0.1.44 | MIT | transitive | linked | https://github.com/tokio-rs/tracing | `63e71662fa4b2a2c…` |
| `tracing-core` | 0.1.36 | MIT | transitive | linked | https://github.com/tokio-rs/tracing | `db97caf9d906fbde…` |
| `tray-icon` | 0.24.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/tauri-apps/tray-icon | `045979e3f037cd18…` |
| `typeid` | 1.0.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/dtolnay/typeid | `bc7d623258602320…` |
| `typenum` | 1.20.1 | MIT OR Apache-2.0 | transitive | build-time only | https://github.com/paholg/typenum | `b6f5e870be6c3b37…` |
| `unic-char-property` | 0.9.0 | MIT/Apache-2.0 | transitive | linked | https://github.com/open-i18n/rust-unic/ | `a8c57a407d9b6fa0…` |
| `unic-char-range` | 0.9.0 | MIT/Apache-2.0 | transitive | linked | https://github.com/open-i18n/rust-unic/ | `0398022d5f700414…` |
| `unic-common` | 0.9.0 | MIT/Apache-2.0 | transitive | linked | https://github.com/open-i18n/rust-unic/ | `80d7ff825a6a654e…` |
| `unic-ucd-ident` | 0.9.0 | MIT/Apache-2.0 | transitive | linked | https://github.com/open-i18n/rust-unic/ | `e230a37c0381caa9…` |
| `unic-ucd-version` | 0.9.0 | MIT/Apache-2.0 | transitive | linked | https://github.com/open-i18n/rust-unic/ | `96bd2f2237fe450f…` |
| `unicode-ident` | 1.0.24 | (MIT OR Apache-2.0) AND Unicode-3.0 | transitive | linked | https://github.com/dtolnay/unicode-ident | `e6e4313cd5fcd3da…` |
| `unicode-segmentation` | 1.13.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/unicode-rs/unicode-segmentation | `c6f5d3c3b1bf0902…` |
| `url` | 2.5.8 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/rust-url | `ff67a8a4397373c3…` |
| `urlpattern` | 0.3.0 | MIT | transitive | linked | https://github.com/denoland/rust-urlpattern | `70acd30e3aa1450b…` |
| `utf8_iter` | 1.0.4 | Apache-2.0 OR MIT | transitive | linked | https://github.com/hsivonen/utf8_iter | `b6c140620e7ffbb2…` |
| `uuid` | 1.26.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/uuid-rs/uuid | `b5772d71c9be8a8a…` |
| `version_check` | 0.9.5 | MIT/Apache-2.0 | transitive | build-time only | https://github.com/SergioBenitez/version_check | `0b928f33d975fc6a…` |
| `vswhom` | 0.1.0 | MIT | transitive | build-time only | https://github.com/nabijaczleweli/vswhom.rs | `be979b7f07507105…` |
| `vswhom-sys` | 0.1.3 | MIT | transitive | build-time only | https://github.com/nabijaczleweli/vswhom-sys.rs | `fb067e4cbd1ff067…` |
| `walkdir` | 2.5.0 | Unlicense/MIT | transitive | linked | https://github.com/BurntSushi/walkdir | `29790946404f91d9…` |
| `web_atoms` | 0.2.6 | MIT OR Apache-2.0 | transitive | linked | https://github.com/servo/html5ever | `ba8b815c1b593dc0…` |
| `webview2-com` | 0.38.2 | MIT | transitive | linked | https://github.com/wravery/webview2-rs | `7130243a7a5b33c5…` |
| `webview2-com-macros` | 0.8.1 | MIT | transitive | linked | https://github.com/wravery/webview2-rs | `67a921c1b6914c36…` |
| `webview2-com-sys` | 0.38.2 | MIT | transitive | linked | https://github.com/wravery/webview2-rs | `381336cfffd77237…` |
| `winapi-util` | 0.1.11 | Unlicense OR MIT | transitive | linked | https://github.com/BurntSushi/winapi-util | `c2a7b1c03c876122…` |
| `window-vibrancy` | 0.6.0 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/tauri-plugin-vibrancy | `d9bec5a31f3f9362…` |
| `windows` | 0.61.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `9babd3a767a4c1ae…` |
| `windows-collections` | 0.2.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `3beeceb5e5cfd9eb…` |
| `windows-core` | 0.61.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `c0fdd3ddb90610c7…` |
| `windows-future` | 0.2.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `fc6a41e98427b19f…` |
| `windows-implement` | 0.60.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `053e2e040ab57b9d…` |
| `windows-interface` | 0.59.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `3f316c4a2570ba26…` |
| `windows-link` | 0.1.3 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `5e6ad25900d524ea…` |
| `windows-link` | 0.2.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `f0805222e57f7521…` |
| `windows-numerics` | 0.2.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `9150af68066c4c5c…` |
| `windows-result` | 0.3.4 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `56f42bd332cc6c8e…` |
| `windows-strings` | 0.4.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `56e6c93f3a0c3b36…` |
| `windows-sys` | 0.59.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `1e38bc4d79ed67fd…` |
| `windows-sys` | 0.60.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `f2f500e4d28234f7…` |
| `windows-sys` | 0.61.2 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `ae137229bcbd6cdf…` |
| `windows-targets` | 0.52.6 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `9b724f72796e036a…` |
| `windows-targets` | 0.53.5 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `4945f9f551b88e0d…` |
| `windows-threading` | 0.1.0 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `b66463ad2e0ea3bb…` |
| `windows-version` | 0.1.7 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `e4060a1da109b9d0…` |
| `windows_x86_64_msvc` | 0.52.6 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `589f6da84c646204…` |
| `windows_x86_64_msvc` | 0.53.1 | MIT OR Apache-2.0 | transitive | linked | https://github.com/microsoft/windows-rs | `d6bbff5f0aada427…` |
| `winnow` | 0.7.15 | MIT | transitive | build-time only | https://github.com/winnow-rs/winnow | `df79d97927682d2f…` |
| `winnow` | 1.0.4 | MIT | transitive | linked | https://github.com/winnow-rs/winnow | `23b97319f7b8343d…` |
| `winreg` | 0.55.0 | MIT | transitive | build-time only | https://github.com/gentoo90/winreg-rs | `cb5a765337c50e9e…` |
| `writeable` | 0.6.4 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `3ad82d2a33cdc967…` |
| `wry` | 0.55.1 | Apache-2.0 OR MIT | transitive | linked | https://github.com/tauri-apps/wry | `186f9871daa55fd9…` |
| `yoke` | 0.8.3 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `709fe23a0424b6a4…` |
| `yoke-derive` | 0.8.2 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `de844c262c884881…` |
| `zerofrom` | 0.1.8 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `0ec05a11813ea801…` |
| `zerofrom-derive` | 0.1.7 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `11532158c46691ca…` |
| `zerotrie` | 0.2.5 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `4ea269c3bd32f0a3…` |
| `zerovec` | 0.11.8 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `bb0464e17806c1d9…` |
| `zerovec-derive` | 0.11.6 | Unicode-3.0 | transitive | linked | https://github.com/unicode-org/icu4x | `34df6fc39dbd26dd…` |
| `zlib-rs` | 0.6.7 | Zlib | transitive | linked | https://github.com/trifectatechfoundation/zlib-rs | `34b31d188d9d685a…` |
| `zmij` | 1.0.23 | MIT | transitive | linked | https://github.com/dtolnay/zmij | `29666d0abbfad1e3…` |

## Purpose — why each direct dependency is here

The workspace names four dependencies by hand. Everything else in the table above arrived
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
- **`tauri`** is the window, chosen in ADR-003 and ADR-006. It is a dependency of `app/` only, not
  of the rendering core: document 06 requires that "the rendering core stays independent of the
  interface", and keeping the shell in its own crate is what makes cargo enforce that rather than
  leaving it to good intentions. `tauri-build` is its build-time half.

## What the shell cost, in crates

This is the honest number and it is worth stating plainly. Before the window, this record held
**28** crates. With it, **271**. One dependency brought in roughly two hundred and thirty
others, which is what a browser engine, an async runtime, a CSS selector engine and a bundler
amount to once they are counted rather than assumed.

That was known when ADR-003 chose Tauri, and the alternative it was weighed against was writing a
window and a rendering surface by hand. The number is recorded here so the trade is visible rather
than implied. **41** of them are build-time only — compiled, run during the build, and
absent from anything this project would ship.

An archive this size is not maintainable by hand, which is why it is not maintained by hand any
more. `tools/archive_licenses.py` copies every licence text out of the crate sources cargo
unpacked, `verification/B-11_license_archive.md` is the artifact it writes, and `--check` runs in
CI.

## What a reviewer has to look at

Named, not decided. Document 10 reserves that judgement: "legal conclusions requiring professional
judgment should be recorded by the appropriate reviewer." The first entry is the one that changed
with the shell and it should be read first.

- **`MPL-2.0` — five crates.** `cssparser`, `cssparser-macros`, `dtoa-short`, `option-ext` and
  `selectors`, all linked, all arriving under `tauri`. **This graph is no longer entirely
  permissive.** MPL-2.0 is file-scope copyleft: it attaches to the MPL-licensed files themselves
  rather than to a larger work that links them, so it does not reach this project's own code the
  way a GPL would. It does carry an obligation to make the source of those files available on
  distribution. This project is open source and modifies no dependency, which is the easiest
  possible position to be in, but easy is not the same as reviewed — and the previous version of
  this record said "nothing in the graph is copyleft". That sentence was true of twenty-eight
  crates and is not true of two hundred and sixty-four.
- **Eleven crates ship no licence text at all.** `alloc-stdlib`, `defmt-parser`, `selectors`, the
  five `unic-*` crates and the three `webview2-com*` crates declare terms in their manifests and
  publish no file carrying them. `selectors` is the one to look at twice: it is the MPL-2.0 crate,
  and MPL-2.0 is the licence in this graph with the most to say about notices. Their archive
  directories hold a generated `NO-LICENCE-TEXT-SHIPPED.md` recording the absence, because an empty
  directory cannot state a fact and git does not keep one anyway.
- **`Unicode-3.0` alone — eighteen crates.** The ICU family: `icu_collections`, `icu_locale_core`,
  `icu_normalizer`, `icu_normalizer_data`, `icu_properties`, `icu_properties_data`, `icu_provider`,
  `litemap`, `potential_utf`, `tinystr`, `writeable`, `yoke`, `yoke-derive`, `zerofrom`,
  `zerofrom-derive`, `zerotrie`, `zerovec`, `zerovec-derive`. Not a choice of MIT or Apache; the
  Unicode licence is the only terms offered. It is permissive and carries an attribution
  requirement, and it is in the graph because URL and text handling need Unicode tables.
- **`unicode-ident` 1.0.24** — `(MIT OR Apache-2.0) AND Unicode-3.0`. The `AND` is the point: this
  is not a choice of one licence, both sets of terms apply. It is build-time only, which likely
  changes the answer, but "likely" is not a review.
- **Conjunctive terms elsewhere.** `brotli` is `BSD-3-Clause AND MIT` and `ryu` is
  `Apache-2.0 AND MIT`. Both mean both, not either.
- **Crates offering no MIT or Apache option at all.** `tao` 0.35.3 is `Apache-2.0` only and is the
  windowing layer, so it is unavoidable for as long as there is a window. `zlib-rs` 0.6.7 and
  `foldhash` 0.2.0 are `Zlib`. `alloc-no-stdlib` and `alloc-stdlib` are `BSD-3-Clause`. All
  permissive, none a choice.
- **`memchr` 2.8.3** — `Unlicense OR MIT`, and `same-file` and `walkdir` alongside it. The
  Unlicense is a public-domain dedication whose standing differs by jurisdiction, and document 10
  records that distribution jurisdictions are still open. `memchr`'s `UNLICENSE` file was absent
  from `Licenses/` until 2026-09-06, when `tools/archive_licenses.py` was written and found it: the
  record discussed a licence whose text the archive did not hold. That was the one disagreement in
  twenty-eight hand-made directories, and it is the reason the archive is no longer made by hand.

## Licence compatibility, as an engineering read

Not a legal opinion, and a less confident one than this section used to give. Every crate above
offers permissive terms. Five are weak copyleft at file scope (MPL-2.0) and carry a source
availability obligation on distribution rather than a licence on this project's own code; nothing
in the graph is copyleft in the sense that would constrain what licence this project may carry.
Roughly a dozen crates offer no MIT or Apache option, so a distribution has to satisfy their terms
as written rather than choosing the familiar ones.

The archived texts under `Licenses/` are what a distribution would have to carry with it. Two
checks keep them honest and neither reads a word of the terms: `tests/b11_dependency_record.rs`
confirms a directory exists for every crate at its resolved version, and
`tools/archive_licenses.py --check`, which runs in CI, confirms that what is inside each directory
is the set of files that crate actually ships. The second is the one a hand-maintained archive
fails quietly, because a missing file looks exactly like a crate that never had one.

## What this record does not yet contain

- **A reviewer and a date.** There has been no legal reviewer. Inventing a sign-off would be worse
  than leaving it blank. The MPL-2.0 entry above is the first thing in this project that genuinely
  wants one.
- **NOTICE files.** Document 10 lists them separately from licence texts. That is read off the
  crate sources themselves rather than off the archive — `tools/archive_licenses.py` looks for
  `NOTICE` alongside the licence names and reports every crate shipping no text at all — but it is
  still not verified by a reviewer.
- **A distribution.** T-16 stays NOT RUN because there is no distributable build to check. Nothing
  here has been shipped to anyone, so no obligation in it has come due. `bundle.active` is `false`
  in `app/tauri.conf.json` for that reason.
- **A signed-off review.** D-31 is now closed — the project is `MIT OR Apache-2.0`, `Cargo.toml`
  declares it and `LICENSE-MIT` and `LICENSE-APACHE` are in the repository root — but that is the
  owner choosing a licence, not a reviewer confirming that this graph may be redistributed under
  it. The copyright line in `LICENSE-MIT` names the GitHub identity that owns the repository; the
  owner should replace it with whatever name belongs on the notice.
