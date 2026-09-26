# P-20: the final performance audit of every effect and the app

**Every one of the thirteen effects and every timing table in the program was measured again on 2026-09-26. Two slow spots were found and fixed without moving a single pixel: Gaussian Blur with "Repeat Edge Pixels" is 3.3x faster (39.3 ms to 11.9 ms) and Radial Blur's zoom with "Repeat Edge Pixels" is 3.7x faster on a character (196.7 ms to 53.0 ms). All 58 effect results have the same fingerprint before and after.** At the app level, the numbers are far better than the tables last said. The declared ten-layer fixture now renders at 26.7 frames a second cold and was at 4.1 when its table was last written, on 2026-09-09. The reference shot plays every one of its ten loops inside the 24 fps deadline, where it had none.

Asked for by the owner on 2026-09-26: "now let's do a final performance audit of all the effects/app".

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads; 61.5 GB of memory
- Graphics card: NVIDIA GeForce RTX 4070 Ti SUPER, driver 32.0.16.1074 (NVIDIA 610.74), through Vulkan
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- **The owner's copy of the app was open throughout**, on the same machine. Every number here was taken with it running, so it is a working machine and not an empty one.
- Effects: `tests/p16_effect_cost.rs`, 7 runs a case, median reported, run with `cargo test --release --test p16_effect_cost -- --ignored --nocapture`. This pass adds three cases, one for each blur that has the "Repeat Edge Pixels" setting. **Before**: commit 1f69b17 (B-53's playtest), run twice. **After**: the commit that carries this page, run once. Both on the same day.
- The app: every timer the program has (the list is at the end), run once each in a release build, between 15:27 and 16:00. Each rewrote its own table in `verification/`, and those rewritten tables are committed with this page.

## The effects, one at a time

Each effect runs alone on a 1920x1080 cel. **Character** is a figure in flat colours on nothing, with about a fifth of the frame showing. **Background** is opaque everywhere. "Before" is the second of the two runs before the change; the first run agreed with it to within a few percent. "Picture" is the first 16 hex digits of the SHA-256 of the effect's result, every float by its bits.

| Effect | Cel | Before, median ms | After, median ms | Picture (the same before and after) |
|---|---|---:|---:|---|
| Exposure +1 | character | 1.1 | 1.0 | `6b129da00b1903e8` |
| Tint 50% | character | 1.2 | 1.1 | `860bcdce1b5b3414` |
| Gaussian Blur 4 | character | 8.5 | 8.1 | `3a38b6e49182b72e` |
| Gaussian Blur 10 | character | 9.2 | 8.8 | `d8193d52de8a0442` |
| Gaussian Blur 40 | character | 15.1 | 14.5 | `2ec3c48d6304282f` |
| Line Smooth | character | 10.7 | 10.0 | `84e209be436f8f63` |
| Selective Colour Blur 12 | character | 50.8 | 50.9 | `b3d27ec2b08546f2` |
| Selective Colour Blur 100 | character | 72.8 | 74.6 | `979e1692ec91b410` |
| Glow 10 | character | 16.5 | 16.7 | `d219ed90e9d00faa` |
| Glow 50 | character | 18.4 | 18.2 | `78347795f2a92ccb` |
| Glow 250 | character | 33.0 | 33.1 | `122eed6712e6a299` |
| Glow 50, chosen colour | character | 16.9 | 17.2 | `3bf7c29067da916f` |
| Line Recolour | character | 1.5 | 1.4 | `1b70622b9245cca3` |
| Directional Blur 10 | character | 8.7 | 8.4 | `78834984dbca176f` |
| Directional Blur 100 | character | 9.4 | 8.5 | `8186cb202b3e57f0` |
| Select Colour | character | 2.1 | 2.0 | `f5427894f63f71da` |
| Line Width 3 | character | 7.2 | 6.8 | `8cfe0e562aee5ba3` |
| Line Width 10 | character | 59.3 | 59.6 | `8a64ae5d2c9abb3d` |
| Line Width -3 | character | 3.8 | 3.9 | `0277a4a7a1259955` |
| Line Width 3, chosen colour | character | 8.5 | 8.3 | `0ab101492a528da7` |
| Radial Blur spin 10 | character | 47.9 | 47.0 | `eb0dfda51c730752` |
| Radial Blur zoom 20 | character | 53.6 | 52.8 | `e39ae3d56d2c7d00` |
| Bloom 20 | character | 77.3 | 77.6 | `41488b6a90ea2c0e` |
| Bloom 20, star 60 | character | 134.5 | 140.4 | `46e0584d3cda9db2` |
| Colour Key rgb | character | 1.8 | 1.7 | `72cac54dd11e4165` |
| Colour Key hue | character | 2.0 | 1.8 | `72cac54dd11e4165` |
| **Gaussian Blur 10, edges repeat** | character | **39.3** | **11.9** | `674263539fdbd09c` |
| Directional Blur 100, edges repeat | character | 9.6 | 9.4 | `95c2b81d444d9f69` |
| **Radial Blur zoom 20, edges repeat** | character | **196.7** | **53.0** | `e39ae3d56d2c7d00` |
| Exposure +1 | background | 1.1 | 1.3 | `fbcaff39348a1d89` |
| Tint 50% | background | 2.4 | 2.7 | `3d8acba2eefefb3f` |
| Gaussian Blur 4 | background | 8.1 | 9.2 | `fb3d6ed63475ccfe` |
| Gaussian Blur 10 | background | 11.9 | 12.3 | `b4e4bd4c90226236` |
| Gaussian Blur 40 | background | 34.3 | 34.8 | `3648ac2d47125b65` |
| Line Smooth | background | 11.6 | 11.6 | `5e8c26a9f8d4132a` |
| Selective Colour Blur 12 | background | 42.5 | 46.8 | `5e8c26a9f8d4132a` |
| Selective Colour Blur 100 | background | 85.4 | 90.9 | `5e8c26a9f8d4132a` |
| Glow 10 | background | 16.8 | 15.5 | `936a584d0df11e0f` |
| Glow 50 | background | 23.7 | 23.8 | `9fe672a899030e5c` |
| Glow 250 | background | 91.4 | 95.8 | `913aee6751793c2e` |
| Glow 50, chosen colour | background | 9.2 | 10.1 | `5e8c26a9f8d4132a` |
| Line Recolour | background | 4.2 | 4.0 | `5e8c26a9f8d4132a` |
| Directional Blur 10 | background | 11.9 | 11.7 | `46ef07085f0255f0` |
| Directional Blur 100 | background | 12.4 | 11.6 | `27ecc05ab95f3a94` |
| Select Colour | background | 5.0 | 5.3 | `33c9aff6d2302613` |
| Line Width 3 | background | 3.0 | 3.2 | `f01bafc9cf2cdad3` |
| Line Width 10 | background | 4.0 | 4.2 | `cb821ab5988fcafd` |
| Line Width -3 | background | 8.3 | 7.9 | `a0aef07880dc1963` |
| Line Width 3, chosen colour | background | 12.1 | 11.0 | `3a9a284f9d11ca23` |
| Radial Blur spin 10 | background | 162.1 | 157.9 | `276676bae9a45ed1` |
| Radial Blur zoom 20 | background | 174.0 | 170.1 | `35623b383d2a3e23` |
| Bloom 20 | background | 80.7 | 82.4 | `d6d44644194e2bce` |
| Bloom 20, star 60 | background | 164.6 | 151.5 | `5b67730c430e0ebc` |
| Colour Key rgb | background | 4.8 | 4.4 | `46c8c1224f54e229` |
| Colour Key hue | background | 5.8 | 5.4 | `ecabdfcc87fe897d` |
| **Gaussian Blur 10, edges repeat** | background | **42.1** | **11.7** | `55dee55accbf4d13` |
| Directional Blur 100, edges repeat | background | 11.9 | 12.6 | `6616bf9b579b82ef` |
| **Radial Blur zoom 20, edges repeat** | background | **219.1** | **180.1** | `35623b383d2a3e23` |

Rows other than the three in bold moved only within run-to-run spread, up or down by a few percent; the two runs before the change differed from each other by as much. Several pairs share a picture for a plain reason. The plate has none of the chosen colours, so the colour effects leave it alone. Radial zoom's picture is the same with either edge setting, because a zoom that smears outward only reads from inside the picture (D-110).

Since P-17 on 2026-09-25, Directional Blur 100 went from 37.0 ms to 8.5 ms on the character, and Bloom star 60 from 228.3 ms to 134.5 ms. That is B-42's running sum, already committed; it is not this pass.

## What changed

Two effects, and in each one every pixel still adds exactly the same samples, in exactly the same order, starting from zero. That is why no bit moved.

- **Gaussian Blur with "Repeat Edge Pixels"** (`src/effects.rs`). The repeat version, added on 2026-09-26 by B-52, worked one pixel at a time and reached for each of its taps. The ordinary version has always worked one tap at a time along a whole row, which reads memory in order and is several times faster. The repeat version now does the same. The pixels past the ends of a row read the end pixel, and a whole held row is added at once going down. A new check, `the_row_held_blur_is_the_pixel_held_blur`, keeps the old pixel-at-a-time version as the reference. It compares every value to the bit, on pictures with odd sizes, narrow strips and a blur wider than the picture. It passes.
- **Radial Blur's zoom with "Repeat Edge Pixels"** (`src/blurs.rs`). The ordinary version skips a pixel whose smear only crosses empty space; the repeat version could not skip anything, because every sample was held at the edge first. Now a sample is held only when its path can actually leave the picture. A path that stays inside is walked, or skipped, exactly as the ordinary version would. On a character with empty space around it, most paths stay inside, which is the 3.7x. On an opaque plate almost every path touches the drawing, so the gain is small (1.2x).

## The graphics card against the processor

Taken from the rewritten card tables, at Full size on a 1920x1080 plate, in ms a frame. The card path covers a last Radial Blur, Bloom, Directional Blur, Gaussian Blur or Glow; every other effect runs on the processor both ways.

| Table | Processor | Card |
|---|---:|---:|
| B-46 Radial Blur | 160.9 / 160.5 | 25.2 / 24.8 |
| B-47 Bloom | 168.8 / 178.6 | 29.8 / 27.8 |
| B-49 Directional Blur | 57.1 / 56.2 | 27.2 / 26.7 |
| B-50 Gaussian Blur | 58.8 / 58.4 | 30.4 / 29.3 |
| B-51 Glow | 51.6 / 50.9 | 17.6 / 17.8 |

Each pair is the table's first loop and second loop. At Draft size the two are within a millisecond of each other, about 17 ms, as before. Every card table agrees with its previous run to within about 10%. B-48's memory table shows the same shape as before: with Auto drawing, Full size is 16.0 ms on the processor and holds 3.3 GB.

## The app, before and now

"Before" is the table as it was last written, on the date shown. "Now" is this pass. **This pass did not make these faster.** The work that did landed between those dates (P-13's shared pool, P-14, P-03(f)'s draft tiles, and the colour-conversion lookup, among others). The tables had simply not been run again since, and now they are.

| Table (last written) | What | Before | Now |
|---|---|---:|---:|
| T-06 declared fixture (2026-09-09) | ten layers, cold render, frames a second | 4.1 | 26.7 |
| T-06 declared fixture (2026-09-09) | warm loops inside the 24 fps deadline | 0 of 10 | **10 of 10** |
| T-06 declared fixture (2026-09-09) | tenth loop, median ms a frame | 264.17 | 38.26 |
| T-06 envelope (2026-09-09) | reference shot, cold render, frames a second | 23.1 | 67.8 |
| T-06 envelope (2026-09-09) | warm loops inside the 24 fps deadline | 0 of 10 | **10 of 10** |
| T-06 envelope (2026-09-09) | frames of the tenth loop over 41.7 ms | 148 of 240 | 0 of 240 |
| T-06 envelope (2026-09-09) | worst seek, p95 ms | 85.37 | 24.20 |
| P-01 frame trace (2026-09-11) | "transfer function and premultiply", ms a frame | 42.9 | 4.0 |
| P-01 frame trace (2026-09-11) | reference shot, Draft, cold, p50 ms | 83.6 | 43.3 |
| P-01 frame trace (2026-09-11) | ten layers, Draft, cold, p50 ms | 250.1 | 111.5 |
| P-03(b) first playthrough (2026-09-11) | ten layers, Full, p50 ms | 257.3 | 64.2 |
| P-14 blur drag (2026-09-12) | Draft, p50 ms / worst frame | 34.8 / 40.0 | 8.9 / 10.5 |
| P-14 blur drag (2026-09-12) | Full, p50 ms / worst frame | 52.8 / 59.7 | 38.7 / 45.2 |
| B-08b cache (2026-09-09) | 1 GiB budget, frames a second | 21.9 | 79.5 |
| B-08 preview latency (2026-09-06) | Draft, p50 ms | 81.69 | 43.11 |
| B-08 preview latency (2026-09-06) | Full, p50 ms | 99.90 | 52.78 |
| D-37 decode cost (2026-09-09) | a small cel, median ms | 8.25 | 6.23 |

The declared ten-layer fixture now plays each loop inside the 10 seconds a 24 fps clock allows, in 8.3 to 8.7 s. But its frames sit close to the line, 38 ms at the median against 41.7 allowed, and **42 of the tenth loop's 240 frames** still cost more than that, so the viewer would drop those. This is the one scene in the tables that is not yet comfortably real time on the processor alone. The card is the way to fix it, as B-44 showed.

**B-08 and D-37 are quoted here but not rewritten.** Their generators still carry prose from before later corrections. Rewriting them would bring back sentences the project has since corrected, such as D-37's "473 MB" that D-40 put right as 1.89 GB. Their committed text stands, and their numbers are in the table above.

## What is left, and not done here

- **The slowest effects that run only on the processor**: Selective Colour Blur 100 (74.6 ms on the character, 90.9 on the plate), Line Width 10 on a character (59.6 ms), Bloom with streaks (140 to 150 ms) and Radial Blur on a plate (158 to 180 ms against about 25 on the card). Each would be a card job like B-46 to B-51. None was started, because each one needs its own decision.
- **The T-06 envelope's headline sentence is out of date.** It still says warm playback "sits on the deadline, and which side of it is not settled", while the same page now counts 10 of 10 loops at 64% under it. The numbers are right and the sentence is the generator's fixed wording. Changing it is a one-line change to `tests/t06_envelope.rs` if the owner wants it.
- **D-99's two draft pictures** (`verification/D-99 proposal/`) were drawn again by their timer. They now show B-53's outward zoom, which is what the program does. They are committed with this page, and they still await the owner as before.
- The B-21d film is also rewritten by its timer, but it is not committed: it is a large video file and its content did not change.

## Every timer run

`b44_gpu_preview`, `b46_gpu_radial`, `b47_gpu_bloom`, `b49_gpu_directional`, `b50_gpu_gaussian`, `b51_gpu_glow`, `b48_memory`, `t06_envelope`, `b08_preview`, `b08b_cache`, `p01_frame_trace` (with P-03(b) and P-14), `p05_culling`, `p06_draft_cels`, `p15_mask_cost`, `p18_read_ahead`, `p19_session_log`, `d99_draft_effects`, `p03f_draft_tile_size`, `b05a_transform`, `b10_full_shot` and `b12b_declared_fixture`: all passed, 15:27 to 16:00. Then `p16_effect_cost` twice before the change and once after.

## How to check this

1. Open the app on a project with a character layer, add a Gaussian Blur of 10 and set Edges to Repeat Edge Pixels. Dragging the radius should feel as quick as it does with Edges on Transparent. Before this change it was noticeably slower.
2. Do the same with a Radial Blur, type Zoom, amount 20, Repeat Edge Pixels.
3. In both, the picture should look exactly as it did before this change. Nothing about how the effects look was touched.
