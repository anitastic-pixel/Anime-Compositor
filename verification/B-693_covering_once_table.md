# B-693: Drop Shadow and Rim Light blur the covering's alpha alone, with no pixel changed

D-813. The before build is c4b0696c; the after build is B-693 (with B-692 in). Both are
`cargo test --release`, run alternately.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER, driver NVIDIA 610.88, Vulkan
- Processor: AMD64 Family 26 Model 68 (Ryzen 9 9900X), 24 threads; Windows 11
- Build: release, `CARGO_INCREMENTAL=0`
- Load: not quiet. Other lanes were building and testing: 2 to 15 cargo/rustc processes were
  counted at the start of each round. Medians are given, with the number of rounds.

## Speed: one effect

`b693_timing` (`tests/b693_covering_once.rs`): a 1920 by 1080 soft disc, the median of 5 runs a
round, in ms; the median of 15 alternating rounds.

| Effect | Before (c4b0696c) | After (B-693) | Change |
|---|---:|---:|---:|
| Drop Shadow, softness 40, distance 12 | 25.2 | 11.7 | 2.2 times as fast |
| Rim Light, width 4, softness 3 | 17.1 | 7.6 | 2.3 times as fast |

## Speed: whole shots

`w8_arch_timing` (`tests/w8_arch_audit.rs`), in ms a frame, the median of the shot's frames, then
the median of the alternating rounds. "CPU" is the viewer's processor path
(`preview_frame_cached`), "Card" the card's (these shots fall back to the processor: 8 bpc and
32 bpc frames are not drawn on the card yet), "Export frame" `compose::render_frame`.

The seven Drop Shadows shot is wave 7's tutorial 4 project (seven Drop Shadows on a title,
softness 10 to 500, so sigma 2 to 100; in the 8 bpc copy the two set to 1000 and 2000 are above
the 500 the panel allows, so they are refused and not drawn, which is why it is quicker), 5
rounds:

| Shot | Column | Before | After |
|---|---|---:|---:|
| 7 Drop Shadows, 32 bpc | CPU Draft | 1486.5 | 253.6 |
| | CPU Full | 1013.6 | 274.0 |
| | Card Full (fell back) | 1053.2 | 336.1 |
| | Export frame | 934.0 | 304.6 |
| 7 Drop Shadows, 8 bpc | CPU Draft | 512.3 | 268.5 |
| | CPU Full | 555.3 | 173.3 |
| | Card Full (fell back) | 398.9 | 173.5 |
| | Export frame | 303.6 | 211.6 |

The `(none)` rows, shots with no Drop Shadow or Rim Light (the blur's code is shared), 7 rounds:

| Shot | Column | Before | After |
|---|---|---:|---:|
| reference shot, 4 layers | CPU Draft | 25.4 | 21.5 |
| | CPU Full | 19.9 | 22.1 |
| | Export frame | 104.4 | 99.8 |
| three nested compositions | CPU Draft | 30.3 | 32.8 |
| | CPU Full | 68.0 | 65.1 |
| | Export frame | 156.4 | 138.8 |

The same within the machine's spread.

## No pixel changed

`b693_hashes` applies 96 Drop Shadows (two colours and opacities, four directions, three
distances, two softnesses, Shadow Only off and on) and 72 Rim Lights (three directions, three
widths, two softnesses, four intensity and blend pairs) to three drawings (a soft disc on a
gradient, a ragged noise field with holes, a one-pixel line) and takes the SHA-256 of every
float of each result, with its size and offset.

| | Before | After |
|---|---:|---:|
| results | 504 | 504 |
| results that differ | | 0 |

The old build's list is committed as `verification/B-693_old_build_hashes.txt`, and the named
test checks the new build against it. B-692's sweep (8678 exports of every fixture composition,
every file's SHA-256) was run with this change in, and is the same.

## Named tests

| Test | Result |
|---|---|
| `b693_a_every_shadow_and_rim_is_the_old_builds` | pass |
| `b58_drop_shadow`, `b60_rim_light`, `b639_drop_shadow_only`, `b654_drop_shadow_softness` | pass; their tables are written again byte for byte |

## Not reached

Most of the seven Drop Shadows shot's time is now the Gaussian itself: at sigma 100 the kernel
is 601 taps a pixel a pass, over a layer grown by 300 pixels a side. A faster blur of the same
Gaussian (a running-sum box approximation, or a Fourier transform as Lens Blur's D-222 offered)
moves pixels, so it is not built here; wave 8's report gives it as a proposal.
