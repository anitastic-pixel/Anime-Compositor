# B-42: running sums for Directional Blur and Bloom's streaks

**Directional Blur 100 on an opaque plate is about 11x faster (140 ms to 13 ms), and Bloom's star streaks about 2.1x (330 ms to 155 ms). A long Directional Blur now costs about the same as a short one.** One case got slower: a short Directional Blur on a mostly empty cel, 5.6 ms to 9 ms (below). Every fixture passes against D-98's numbers, which were written before this code.

Built on 2026-09-25 after the owner accepted D-98 ("accept d98, proceed"). D-98's fixture values replaced D-92's and D-96's first, in commit 9566131; this is the code.

## The fixtures

| Table | Cases | Result | Largest difference | Tolerance |
|---|---|---|---|---|
| `verification/B-36_directional_blur_table.md` | FX-DIRBLUR-001 to 015 | 66 of 66 | 1.9e-7 | 2e-5 |
| `verification/B-39_radial_blur_table.md` | FX-RADIAL-001 to 018 | 76 of 76 | 2.5e-7 | 2e-5 |
| `verification/B-40_bloom_table.md` | FX-BLOOM-001 to 028 | 106 of 106 | 1.0e-6 | 2e-5 |

Radial Blur is on the list because FX-RADIAL-012 puts a directional blur ahead of it. The whole core suite passes (94 tests, 0 failures), as does the app's (63 passed).

## Machine, build and configuration

- CPU: AMD Ryzen 9 9900X, 12 cores, 24 hardware threads
- OS: Microsoft Windows 11 Education, 10.0.26200
- Toolchain: rustc 1.89.0, cargo release profile, `opt-level = 3`
- Pool: rayon's default, one thread per hardware thread
- Harness: P-17's `tests/p16_effect_cost.rs`, unchanged, run with `cargo test --release --test p16_effect_cost -- --ignored --nocapture`
- Before: commit 9566131, the old code, built from a separate checkout. After: the commit that carries this page. Same day, and the runs went before, after, before, after, one at a time.
- Workload: one effect alone on a 1920x1080 cel, 7 runs each, median reported. P-17's two cels: **character**, a figure on nothing with about a fifth of the frame showing, and **background**, opaque everywhere. Directional Blur runs at direction 30, and Bloom at threshold 80, radius 20, with a star at angle 0 and length 60.

## Before and after

Medians in ms, from both rounds.

| Effect | Cel | Before, run 1 | Before, run 2 | After, run 1 | After, run 2 | Times faster |
|---|---|---|---|---|---|---|
| Directional Blur 10 | character | 5.7 | 5.6 | 8.8 | 9.3 | 0.6x |
| Directional Blur 100 | character | 34.4 | 34.5 | 9.3 | 9.5 | 3.7x |
| Bloom 20, star 60 | character | 221.1 | 226.4 | 154.2 | 143.0 | 1.5x |
| Directional Blur 10 | background | 17.9 | 16.4 | 12.8 | 12.2 | 1.4x |
| Directional Blur 100 | background | 142.0 | 138.7 | 12.9 | 13.8 | 10.5x |
| Bloom 20, star 60 | background | 328.8 | 335.4 | 152.3 | 158.6 | 2.1x |
| Bloom 20 (no streaks) | character | 88.0 | 84.3 | 87.3 | 86.5 | 1.0x |
| Bloom 20 (no streaks) | background | 85.7 | 88.5 | 88.2 | 85.8 | 1.0x |
| Radial Blur spin 10 | background | 161.2 | 165.5 | 171.4 | 179.7 | 0.9x |
| Radial Blur zoom 20 | background | 176.1 | 185.2 | 184.3 | 189.5 | 1.0x |

**The pictures.** Directional Blur and Bloom's star give different results than before, as D-98 said they would: their SHA-256s changed. Each new one is held to D-98's fixtures, above. Bloom with no streaks, and every Radial Blur row, give the same SHA-256 as before, to the bit.

**Radial Blur**'s code did not change, and neither did its results. Its after column came out a little slower in both rounds, most on the plate. Each after run followed a before run straight away, so this is most likely the machine running warmer, not the change. The same goes for the other effects in the harness; they are not repeated here.

**The slower case.** A short Directional Blur on a mostly empty cel used to skip each empty pixel on its own. It now reads whole lines across the picture and skips a line only when it passes nowhere near the drawing. That costs about 3 ms at 1080p. On an opaque plate, or at any length above about 20, the new way is ahead.

## What changed

- **Directional Blur** (`src/blurs.rs`). The picture is read along straight lines in the blur's direction, one pixel apart. A running total along each line gives every point's average with a few subtractions, however long the blur. Each pixel then mixes the two lines nearest it. This is D-98's rule, as document 21 writes it.
- **Bloom's streaks** (`src/bloom.rs`) use the same lines, with the streak's tent worked from a second running total. The halo is unchanged.
- **Along the way.** Mostly-vertical directions (direction 30 is one) are read in place rather than by turning the picture on its side and back. Turning it cost more than the blur. Lines that hold nothing are skipped when the pixels are mixed. And each worker's scratch space is set up once per 16 lines, not once per line. Before that fix, setting it up took longer than the blur.
- **Memory.** Every line is held at once while the blur runs: about 45 MB for a 1080p layer at direction 30, up to about 100 MB near 45 degrees, freed straight after. Working in bands of lines would lower that if memory ever matters.

## Try it

1. Open any project, add a full-frame background plate, and press **Full resolution**.
2. Add **Directional Blur** to it and set Length to 100, then drag Direction. At that length each frame used to take about a seventh of a second on a plate. It should now respond about as quickly as it does at length 10.
3. Add **Bloom** to a bright drawing, pick the star streaks and set Length to 60. The streaks should look as they did, and the picture should update faster when you change a setting.

The streaks and blurs should look as they did at the end of the D-98 proposal: `verification/B-42a proposal/` has the before-and-after pictures, and the differences they show are all that should have changed.
