# B-692: an export that blocks on a missing drawing draws each frame once, with no byte changed

D-812. The before build is c4b0696c (the commit before D-812); the after build is B-692. Both are
`cargo test --release`, run one after the other, alternating, from `tests/b692_export_once.rs`
(`b692_timing` and `b692_hashes`, `#[ignore]`, run by name; the before build is that file cut at
its "own checks" line and compiled against c4b0696c with `B692_LABEL=before`).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER, driver NVIDIA 610.88, Vulkan (not used: an export is
  drawn by the processor)
- Processor: AMD64 Family 26 Model 68 (Ryzen 9 9900X), 24 threads; Windows 11
- Build: release, `CARGO_INCREMENTAL=0`
- Load: not quiet. Other lanes were building and testing: 7 to 35 cargo/rustc processes were
  counted at the start of each round. Medians of 7 alternating rounds are given, and every round.

## Speed

The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 8-bit PNG), in ms.

| Export | Before (c4b0696c) | After (B-692) | Change |
|---|---:|---:|---:|
| frames 0 to 13, blocking (the default; none is missing) | 1312 | 899 | 32% less |
| frames 0 to 13, the missing written transparent (no check) | 1019 | 1087 | the same path; spread |
| frames 0 to 47, blocking, which stops (frames 14, 15, 38 and 39 miss a drawing) | 904 | 946 | the same; spread |

Every round (blocking 0-13, transparent 0-13, blocking 0-47):

| Round | Before | After |
|---:|---|---|
| 1 | 1178, 1364, 887 | 847, 871, 857 |
| 2 | 1381, 1012, 1095 | 915, 1191, 946 |
| 3 | 1609, 1469, 3449 | 3749, 1566, 1159 |
| 4 | 1250, 1019, 845 | 899, 1001, 864 |
| 5 | 1312, 850, 793 | 847, 1087, 2731 |
| 6 | 3268, 2213, 1572 | 1507, 1109, 1021 |
| 7 | 1203, 891, 904 | 876, 870, 887 |

A blocking export now takes what an export without the check takes. Wave 8's survey
(`tests/w8_arch_audit.rs`, one run, 8 frames each) found the check cost 1.0 to 2.2 times the
plain export on every shot measured (25 layers 6355 against 3446 ms, 4K 4895 against 2906,
Noise into Polar Coordinates 3491 against 1946), which is what this removes.

A first version drew each frame as soon as its own plan was whole; on the blocked range (0 to 47)
that drew frames which were then thrown away: 1718 ms against 1004 (medians of 7 alternating
rounds, the same machine and load). Now a chunk of frames is drawn
only when every plan in it is whole, so a blocked export costs what it did.

## No byte changed

`b692_hashes` exports, blocking (the default), and takes the SHA-256 of every file written and of
the report (status, files expected, files written, fidelity, every diagnostic):

- the reference shot, frames 0 to 47 as 8-bit PNG, 16-bit premultiplied PNG and EXR half, and
  frames 0 to 23 as a GIF and an animated PNG (all blocked: their reports are compared);
- every composition of every fixture project in `Fixtures/` that opens, every frame, 8-bit PNG.

| | Before | After |
|---|---:|---:|
| exports | 8678 | 8678 |
| lines (files and reports) | 83202 | 83202 |
| lines that differ | | 0 |

The two hash files are the same byte for byte (`cmp`). They are kept in the lane's scratch folder,
not committed (83202 lines).

## Named tests

| Test | Result |
|---|---|
| `b692_a_blocking_export_writes_what_drawing_once_writes` (12 frames blocking, 1 at a time and as many as the export picks, and a GIF, against the same export with no check) | pass |
| `t08_export`, `d48_export_threads`, `h04_exported_file`, `t07e_roundtrip_export`, `b520_sound_export`, `b160_faster_export` | pass |

`t08_export` rewrites its table each run; the one row that moves, "the frames it wrote are still
on disk after the stop", counts the frames a cancelled export had written when the other thread's
Cancel arrived, which changes run to run (4, 7 and 2 were seen); it is not committed.
