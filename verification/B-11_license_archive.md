# B-11 licence archive, checked against the crates the build resolved

The artifact is the directory tree under `Licenses/`. This is the check that keeps it true, and it answers a narrower question than `verification/B-11_record_table.md` does. That one asks whether every crate has an archived directory. This one asks whether what is inside each directory is what the crate actually ships. A hand-maintained archive passes the first and quietly fails the second.

Produced by `tools/archive_licenses.py --report`, from `cargo metadata` and the crate sources cargo unpacked to build with. Nothing is downloaded and no file list is typed. `tools/archive_licenses.py --check` runs in CI and fails the build if the two ever part company.

**271 crates resolved. 0 disagree with the archive. 12 ship no licence text at all.**

## Crates shipping no licence text

These declare a licence in their manifest, which `docs/DEPENDENCIES.md` records, but ship no file for this to copy. Nothing is missing from the archive; there was nothing to archive. Listing them is the point, because a notice that has to be reproduced at distribution cannot be reproduced from a file that does not exist. Each one has a `NO-LICENCE-TEXT-SHIPPED.md` in its archive directory saying so, because an empty directory cannot, and git does not keep empty directories at all.

- `alloc-stdlib-0.2.4`
- `defmt-parser-1.0.0`
- `selectors-0.36.1`
- `tauri-plugin-2.6.3`
- `unic-char-property-0.9.0`
- `unic-char-range-0.9.0`
- `unic-common-0.9.0`
- `unic-ucd-ident-0.9.0`
- `unic-ucd-version-0.9.0`
- `webview2-com-0.38.2`
- `webview2-com-macros-0.8.1`
- `webview2-com-sys-0.38.2`

## What is archived

| Crate | Archived files |
|---|---|
| `adler2-2.0.1` | LICENSE-0BSD, LICENSE-APACHE, LICENSE-MIT |
| `aho-corasick-1.1.5` | COPYING, LICENSE-MIT, UNLICENSE |
| `alloc-no-stdlib-2.0.4` | LICENSE |
| `alloc-stdlib-0.2.4` | none shipped |
| `anyhow-1.0.104` | LICENSE-APACHE, LICENSE-MIT |
| `autocfg-1.5.1` | LICENSE-APACHE, LICENSE-MIT |
| `base64-0.22.1` | LICENSE-APACHE, LICENSE-MIT |
| `bit-set-0.8.0` | LICENSE-APACHE, LICENSE-MIT |
| `bit-vec-0.8.0` | LICENSE-APACHE, LICENSE-MIT |
| `bitflags-1.3.2` | LICENSE-APACHE, LICENSE-MIT |
| `bitflags-2.13.1` | LICENSE-APACHE, LICENSE-MIT |
| `block-buffer-0.10.4` | LICENSE-APACHE, LICENSE-MIT |
| `brotli-8.0.4` | LICENSE.BSD-3-Clause, LICENSE.MIT |
| `brotli-decompressor-5.0.3` | LICENSE |
| `bs58-0.5.1` | LICENSE-APACHE, LICENSE-MIT |
| `byteorder-1.5.0` | COPYING, LICENSE-MIT, UNLICENSE |
| `bytes-1.12.1` | LICENSE |
| `camino-1.2.5` | LICENSE-APACHE, LICENSE-MIT |
| `cargo-platform-0.1.9` | LICENSE-APACHE, LICENSE-MIT |
| `cargo_metadata-0.19.2` | LICENSE-MIT |
| `cargo_toml-0.22.3` | LICENSE |
| `cc-1.4.5` | LICENSE-APACHE, LICENSE-MIT |
| `cfb-0.7.3` | LICENSE |
| `cfg-if-1.0.4` | LICENSE-APACHE, LICENSE-MIT |
| `chrono-0.4.45` | LICENSE.txt |
| `cookie-0.18.2` | LICENSE-APACHE, LICENSE-MIT |
| `cpufeatures-0.2.17` | LICENSE-APACHE, LICENSE-MIT |
| `crc32fast-1.5.1` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-channel-0.5.17` | LICENSE-APACHE, LICENSE-MIT, LICENSE-THIRD-PARTY |
| `crossbeam-deque-0.8.7` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-epoch-0.9.20` | LICENSE-APACHE, LICENSE-MIT |
| `crossbeam-utils-0.8.22` | LICENSE-APACHE, LICENSE-MIT |
| `crypto-common-0.1.7` | LICENSE-APACHE, LICENSE-MIT |
| `cssparser-0.36.0` | LICENSE |
| `cssparser-macros-0.6.1` | LICENSE |
| `ctor-0.8.0` | LICENSE-APACHE, LICENSE-MIT |
| `ctor-proc-macro-0.0.7` | LICENSE-APACHE, LICENSE-MIT |
| `darling-0.23.0` | LICENSE |
| `darling_core-0.23.0` | LICENSE |
| `darling_macro-0.23.0` | LICENSE |
| `defmt-1.1.1` | LICENSE-APACHE, LICENSE-MIT |
| `defmt-macros-1.1.1` | LICENSE-APACHE, LICENSE-MIT |
| `defmt-parser-1.0.0` | none shipped |
| `deranged-0.5.8` | LICENSE-Apache, LICENSE-MIT |
| `derive_more-2.1.1` | LICENSE |
| `derive_more-impl-2.1.1` | LICENSE |
| `digest-0.10.7` | LICENSE-APACHE, LICENSE-MIT |
| `dirs-6.0.0` | LICENSE-APACHE, LICENSE-MIT |
| `dirs-sys-0.5.0` | LICENSE-APACHE, LICENSE-MIT |
| `displaydoc-0.2.7` | LICENSE-APACHE, LICENSE-MIT |
| `dom_query-0.27.0` | LICENSE |
| `dpi-0.1.2` | LICENSE, LICENSE-LIBM-MIT |
| `dtoa-1.0.11` | LICENSE-APACHE, LICENSE-MIT |
| `dtoa-short-0.3.5` | LICENSE |
| `dtor-0.3.0` | LICENSE-APACHE, LICENSE-MIT |
| `dtor-proc-macro-0.0.6` | LICENSE-APACHE, LICENSE-MIT |
| `dunce-1.0.5` | LICENSE |
| `dyn-clone-1.0.20` | LICENSE-APACHE, LICENSE-MIT |
| `either-1.18.0` | LICENSE-APACHE, LICENSE-MIT |
| `embed-resource-3.0.11` | LICENSE |
| `equivalent-1.0.2` | LICENSE-APACHE, LICENSE-MIT |
| `erased-serde-0.4.10` | LICENSE-APACHE, LICENSE-MIT |
| `fastrand-2.5.0` | LICENSE-APACHE, LICENSE-MIT |
| `fdeflate-0.3.7` | LICENSE-APACHE, LICENSE-MIT |
| `find-msvc-tools-0.1.12` | LICENSE-APACHE, LICENSE-MIT |
| `flate2-1.1.10` | LICENSE-APACHE, LICENSE-MIT |
| `fnv-1.0.7` | LICENSE-APACHE, LICENSE-MIT |
| `foldhash-0.2.0` | LICENSE |
| `form_urlencoded-1.2.2` | LICENSE-APACHE, LICENSE-MIT |
| `generic-array-0.14.7` | LICENSE |
| `getrandom-0.3.4` | LICENSE-APACHE, LICENSE-MIT |
| `getrandom-0.4.3` | LICENSE-APACHE, LICENSE-MIT |
| `glob-0.3.4` | LICENSE-APACHE, LICENSE-MIT |
| `hashbrown-0.12.3` | LICENSE-APACHE, LICENSE-MIT |
| `hashbrown-0.17.1` | LICENSE-APACHE, LICENSE-MIT |
| `heck-0.5.0` | LICENSE-APACHE, LICENSE-MIT |
| `hex-0.4.3` | LICENSE-APACHE, LICENSE-MIT |
| `html5ever-0.38.0` | LICENSE-APACHE, LICENSE-MIT |
| `http-1.5.0` | LICENSE-APACHE, LICENSE-MIT |
| `ico-0.5.0` | LICENSE |
| `icu_collections-2.3.0` | LICENSE |
| `icu_locale_core-2.3.0` | LICENSE |
| `icu_normalizer-2.3.0` | LICENSE |
| `icu_normalizer_data-2.3.0` | LICENSE |
| `icu_properties-2.3.0` | LICENSE |
| `icu_properties_data-2.3.0` | LICENSE |
| `icu_provider-2.3.1` | LICENSE |
| `ident_case-1.0.1` | LICENSE |
| `idna-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `idna_adapter-1.2.2` | LICENSE-APACHE, LICENSE-MIT |
| `indexmap-1.9.3` | LICENSE-APACHE, LICENSE-MIT |
| `indexmap-2.14.2` | LICENSE-APACHE, LICENSE-MIT |
| `infer-0.19.0` | LICENSE |
| `itoa-1.0.18` | LICENSE-APACHE, LICENSE-MIT |
| `jiff-0.2.35` | COPYING, LICENSE-MIT, UNLICENSE |
| `jiff-core-0.1.0` | COPYING, LICENSE-MIT, UNLICENSE |
| `jiff-tzdb-0.1.8` | COPYING, LICENSE-MIT, UNLICENSE |
| `jiff-tzdb-platform-0.1.3` | COPYING, LICENSE-MIT, UNLICENSE |
| `json-patch-3.0.1` | LICENSE-APACHE, LICENSE-MIT |
| `jsonptr-0.6.3` | LICENSE-APACHE, LICENSE-MIT |
| `keyboard-types-0.7.0` | LICENSE-APACHE, LICENSE-MIT |
| `libc-0.2.189` | LICENSE-APACHE, LICENSE-MIT |
| `litemap-0.8.3` | LICENSE |
| `lock_api-0.4.14` | LICENSE-APACHE, LICENSE-MIT |
| `log-0.4.34` | LICENSE-APACHE, LICENSE-MIT |
| `markup5ever-0.38.0` | LICENSE-APACHE, LICENSE-MIT |
| `memchr-2.8.3` | COPYING, LICENSE-MIT, UNLICENSE |
| `mime-0.3.17` | LICENSE-APACHE, LICENSE-MIT |
| `miniz_oxide-0.8.9` | LICENSE, LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `miniz_oxide-0.9.1` | LICENSE, LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `mio-1.2.3` | LICENSE |
| `muda-0.19.3` | LICENSE-APACHE, LICENSE-MIT, LICENSE.spdx |
| `new_debug_unreachable-1.0.6` | LICENSE-MIT |
| `num-conv-0.2.2` | LICENSE-Apache, LICENSE-MIT |
| `num-traits-0.2.19` | LICENSE-APACHE, LICENSE-MIT |
| `once_cell-1.21.4` | LICENSE-APACHE, LICENSE-MIT |
| `option-ext-0.2.0` | LICENSE.txt |
| `parking_lot-0.12.5` | LICENSE-APACHE, LICENSE-MIT |
| `parking_lot_core-0.9.12` | LICENSE-APACHE, LICENSE-MIT |
| `percent-encoding-2.3.2` | LICENSE-APACHE, LICENSE-MIT |
| `phf-0.13.1` | LICENSE |
| `phf_codegen-0.13.1` | LICENSE |
| `phf_generator-0.13.1` | LICENSE |
| `phf_macros-0.13.1` | LICENSE |
| `phf_shared-0.13.1` | LICENSE |
| `pin-project-lite-0.2.17` | LICENSE-APACHE, LICENSE-MIT |
| `plist-1.10.0` | LICENCE |
| `png-0.17.16` | LICENSE-APACHE, LICENSE-MIT |
| `png-0.18.1` | LICENSE-APACHE, LICENSE-MIT |
| `potential_utf-0.1.6` | LICENSE |
| `powerfmt-0.2.0` | LICENSE-Apache, LICENSE-MIT |
| `precomputed-hash-0.1.1` | LICENSE |
| `proc-macro2-1.0.107` | LICENSE-APACHE, LICENSE-MIT |
| `quick-xml-0.41.0` | LICENSE-MIT.md |
| `quote-1.0.47` | LICENSE-APACHE, LICENSE-MIT |
| `raw-window-handle-0.6.2` | LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `rayon-1.12.0` | LICENSE-APACHE, LICENSE-MIT |
| `rayon-core-1.13.0` | LICENSE-APACHE, LICENSE-MIT |
| `ref-cast-1.0.27` | LICENSE-APACHE, LICENSE-MIT |
| `ref-cast-impl-1.0.27` | LICENSE-APACHE, LICENSE-MIT |
| `regex-1.13.1` | LICENSE-APACHE, LICENSE-MIT |
| `regex-automata-0.4.18` | LICENSE-APACHE, LICENSE-MIT |
| `regex-syntax-0.8.11` | LICENSE-APACHE, LICENSE-MIT |
| `rfd-0.16.0` | LICENSE |
| `rustc-hash-2.1.3` | LICENSE-APACHE, LICENSE-MIT |
| `rustc_version-0.4.1` | LICENSE-APACHE, LICENSE-MIT |
| `same-file-1.0.6` | COPYING, LICENSE-MIT, UNLICENSE |
| `schemars-0.8.22` | LICENSE |
| `schemars-0.9.0` | LICENSE |
| `schemars-1.2.2` | LICENSE |
| `schemars_derive-0.8.22` | LICENSE |
| `scopeguard-1.2.0` | LICENSE-APACHE, LICENSE-MIT |
| `selectors-0.36.1` | none shipped |
| `semver-1.0.28` | LICENSE-APACHE, LICENSE-MIT |
| `serde-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde-untagged-0.1.9` | LICENSE-APACHE, LICENSE-MIT |
| `serde_core-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde_derive-1.0.229` | LICENSE-APACHE, LICENSE-MIT |
| `serde_derive_internals-0.29.1` | LICENSE-APACHE, LICENSE-MIT |
| `serde_json-1.0.151` | LICENSE-APACHE, LICENSE-MIT |
| `serde_repr-0.1.21` | LICENSE-APACHE, LICENSE-MIT |
| `serde_spanned-1.1.1` | LICENSE-APACHE, LICENSE-MIT |
| `serde_with-3.22.0` | LICENSE-APACHE, LICENSE-MIT |
| `serde_with_macros-3.22.0` | LICENSE-APACHE, LICENSE-MIT |
| `serialize-to-javascript-0.1.2` | LICENSE-APACHE, LICENSE-MIT |
| `serialize-to-javascript-impl-0.1.2` | LICENSE-APACHE, LICENSE-MIT |
| `servo_arc-0.4.3` | LICENSE-APACHE, LICENSE-MIT |
| `sha2-0.10.9` | LICENSE-APACHE, LICENSE-MIT |
| `shlex-2.0.1` | LICENSE-APACHE, LICENSE-MIT |
| `simd-adler32-0.3.10` | LICENSE.md |
| `siphasher-1.0.3` | COPYING |
| `smallvec-1.16.0` | LICENSE-APACHE, LICENSE-MIT |
| `socket2-0.6.5` | LICENSE-APACHE, LICENSE-MIT |
| `softbuffer-0.4.8` | LICENSE-APACHE, LICENSE-MIT |
| `stable_deref_trait-1.2.1` | LICENSE-APACHE, LICENSE-MIT |
| `string_cache-0.9.0` | LICENSE-APACHE, LICENSE-MIT |
| `string_cache_codegen-0.6.1` | LICENSE-APACHE, LICENSE-MIT |
| `strsim-0.11.1` | LICENSE |
| `syn-2.0.119` | LICENSE-APACHE, LICENSE-MIT |
| `syn-3.0.5` | LICENSE-APACHE, LICENSE-MIT |
| `synstructure-0.13.2` | LICENSE |
| `tao-0.35.3` | LICENSE, LICENSE.spdx |
| `tauri-2.11.5` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-build-2.6.3` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-codegen-2.6.3` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-macros-2.6.3` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-plugin-2.6.3` | none shipped |
| `tauri-plugin-dialog-2.7.3` | LICENSE.spdx, LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-plugin-fs-2.5.2` | LICENSE.spdx, LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-runtime-2.11.3` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-runtime-wry-2.11.4` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-utils-2.9.3` | LICENSE_APACHE-2.0, LICENSE_MIT |
| `tauri-winres-0.3.6` | LICENSE |
| `tendril-0.5.1` | LICENSE-APACHE, LICENSE-MIT |
| `thiserror-1.0.69` | LICENSE-APACHE, LICENSE-MIT |
| `thiserror-2.0.20` | LICENSE-APACHE, LICENSE-MIT |
| `thiserror-impl-1.0.69` | LICENSE-APACHE, LICENSE-MIT |
| `thiserror-impl-2.0.20` | LICENSE-APACHE, LICENSE-MIT |
| `time-0.3.55` | LICENSE-Apache, LICENSE-MIT |
| `time-core-0.1.9` | LICENSE-Apache, LICENSE-MIT |
| `time-macros-0.2.32` | LICENSE-Apache, LICENSE-MIT |
| `tinystr-0.8.4` | LICENSE |
| `tinyvec-1.13.2` | LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `tinyvec_macros-0.1.1` | LICENSE-APACHE.md, LICENSE-MIT.md, LICENSE-ZLIB.md |
| `tokio-1.53.1` | LICENSE |
| `toml-0.9.12+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `toml-1.1.5+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `toml_datetime-0.7.5+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `toml_datetime-1.1.1+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `toml_parser-1.1.3+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `toml_writer-1.1.2+spec-1.1.0` | LICENSE-APACHE, LICENSE-MIT |
| `tracing-0.1.44` | LICENSE |
| `tracing-core-0.1.36` | LICENSE |
| `tray-icon-0.24.2` | LICENSE-APACHE, LICENSE-MIT, LICENSE.spdx |
| `typeid-1.0.3` | LICENSE-APACHE, LICENSE-MIT |
| `typenum-1.20.1` | LICENSE, LICENSE-APACHE, LICENSE-MIT |
| `unic-char-property-0.9.0` | none shipped |
| `unic-char-range-0.9.0` | none shipped |
| `unic-common-0.9.0` | none shipped |
| `unic-ucd-ident-0.9.0` | none shipped |
| `unic-ucd-version-0.9.0` | none shipped |
| `unicode-ident-1.0.24` | LICENSE-APACHE, LICENSE-MIT, LICENSE-UNICODE |
| `unicode-segmentation-1.13.3` | COPYRIGHT, LICENSE-APACHE, LICENSE-MIT |
| `url-2.5.8` | LICENSE-APACHE, LICENSE-MIT |
| `urlpattern-0.3.0` | LICENSE |
| `utf8_iter-1.0.4` | COPYRIGHT, LICENSE-APACHE, LICENSE-MIT |
| `uuid-1.26.0` | LICENSE-APACHE, LICENSE-MIT |
| `version_check-0.9.5` | LICENSE-APACHE, LICENSE-MIT |
| `vswhom-0.1.0` | LICENSE |
| `vswhom-sys-0.1.3` | LICENSE |
| `walkdir-2.5.0` | COPYING, LICENSE-MIT, UNLICENSE |
| `web_atoms-0.2.6` | LICENSE-APACHE, LICENSE-MIT |
| `webview2-com-0.38.2` | none shipped |
| `webview2-com-macros-0.8.1` | none shipped |
| `webview2-com-sys-0.38.2` | none shipped |
| `winapi-util-0.1.11` | COPYING, LICENSE-MIT, UNLICENSE |
| `window-vibrancy-0.6.0` | LICENSE-APACHE, LICENSE-MIT, LICENSE.spdx |
| `windows-0.61.3` | license-apache-2.0, license-mit |
| `windows-collections-0.2.0` | license-apache-2.0, license-mit |
| `windows-core-0.61.2` | license-apache-2.0, license-mit |
| `windows-future-0.2.1` | license-apache-2.0, license-mit |
| `windows-implement-0.60.2` | license-apache-2.0, license-mit |
| `windows-interface-0.59.3` | license-apache-2.0, license-mit |
| `windows-link-0.1.3` | license-apache-2.0, license-mit |
| `windows-link-0.2.1` | license-apache-2.0, license-mit |
| `windows-numerics-0.2.0` | license-apache-2.0, license-mit |
| `windows-result-0.3.4` | license-apache-2.0, license-mit |
| `windows-strings-0.4.2` | license-apache-2.0, license-mit |
| `windows-sys-0.59.0` | license-apache-2.0, license-mit |
| `windows-sys-0.60.2` | license-apache-2.0, license-mit |
| `windows-sys-0.61.2` | license-apache-2.0, license-mit |
| `windows-targets-0.52.6` | license-apache-2.0, license-mit |
| `windows-targets-0.53.5` | license-apache-2.0, license-mit |
| `windows-threading-0.1.0` | license-apache-2.0, license-mit |
| `windows-version-0.1.7` | license-apache-2.0, license-mit |
| `windows_x86_64_msvc-0.52.6` | license-apache-2.0, license-mit |
| `windows_x86_64_msvc-0.53.1` | license-apache-2.0, license-mit |
| `winnow-0.7.15` | LICENSE-MIT |
| `winnow-1.0.4` | LICENSE-MIT |
| `winreg-0.55.0` | LICENSE |
| `writeable-0.6.4` | LICENSE |
| `wry-0.55.1` | LICENSE-APACHE, LICENSE-MIT, LICENSE.spdx |
| `yoke-0.8.3` | LICENSE |
| `yoke-derive-0.8.2` | LICENSE |
| `zerofrom-0.1.8` | LICENSE |
| `zerofrom-derive-0.1.7` | LICENSE |
| `zerotrie-0.2.5` | LICENSE |
| `zerovec-0.11.8` | LICENSE |
| `zerovec-derive-0.11.6` | LICENSE |
| `zlib-rs-0.6.7` | LICENSE |
| `zmij-1.0.23` | LICENSE-MIT |

This check reads no licence and decides nothing about one. Document 10 reserves that for a reviewer, and there has not been one.
