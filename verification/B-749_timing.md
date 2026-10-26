# B-749 (D-869) and B-750 (D-870): timings, 8 bpc on the card

Measured with `tests/w8_arch_audit.rs`'s `w8_arch_timing` (wave 8's measurement, `W8_NO_EXPORT=1`), three release test binaries run one after another in each round: before (B-748, 8b76a930), B-749, and B-749 with B-750.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- Build: release, `CARGO_INCREMENTAL=0`
- Load: a shared machine; other lanes were building and running their own tests throughout.
- Shots: wave 8's 4-layer reference shot (`ref_L4`, 1920 by 1080, `Fixtures/reference_shot`) set to 8 bpc (After Effects), frames 0, 30, 60, 90 and 120, median of the five frames; then the median of 5 rounds. ms a frame.
  - `ref8_none`: nothing added (the `(none)` row).
  - `ref8_adj`: an adjustment layer on top with Gaussian Blur (6 px) and Levels.
  - `ref8_fx`: Drop Shadow (softness 20) on layer 2, Bloom (radius 20) on layer 3, Exposure (+0.5) and Solid Composite (30%) on layer 4.
  - `ref8_wrap`: a Light Wrap (width 10) on layer 3.
  The projects are in the lane's scratchpad (`wave9/w9c/ref8_*.json`, rounds in `wave9/w9c/timing_c2`).

## Medians of 5 rounds: before / B-749 / B-750

| Shot | CPU Draft | CPU Full | Card Draft | Card Full | Card fell back |
|---|---:|---:|---:|---:|---|
| ref8_none | 33.5 / 35.8 / 34.0 | 37.9 / 38.1 / 38.2 | 15.1 / 14.8 / 14.5 | 12.5 / 12.3 / 12.6 | no / no / no |
| ref8_adj | 55.4 / 54.8 / 53.1 | 89.8 / 88.6 / 87.5 | 44.5 / 21.4 / 21.3 | 94.0 / 23.8 / 24.1 | yes / no / no |
| ref8_fx | 35.2 / 38.5 / 36.3 | 150.3 / 149.1 / 142.0 | 33.5 / 21.6 / 10.7 | 156.4 / 162.4 / 31.0 | no / no / no |
| ref8_wrap | 36.8 / 37.8 / 36.9 | 73.3 / 76.8 / 77.5 | 26.5 / 28.6 / 9.8 | 82.8 / 18.7 / 21.3 | yes / no / no |

The CPU columns run the same code in all three builds; they show the machine's noise.

## Card per round

| Shot, build | Card Draft | Card Full |
|---|---|---|
| ref8_adj, before | 44.5, 44.0, 46.7, 46.7, 42.4 | 96.0, 101.3, 93.1, 91.4, 94.0 |
| ref8_adj, B-749 | 20.4, 21.4, 21.4, 23.0, 20.1 | 23.8, 31.5, 22.2, 27.8, 23.7 |
| ref8_adj, B-750 | 21.7, 20.7, 21.3, 21.3, 23.8 | 28.7, 22.5, 24.1, 23.4, 27.2 |
| ref8_fx, before | 34.6, 33.5, 24.6, 26.7, 33.7 | 181.9, 154.2, 155.8, 156.8, 156.4 |
| ref8_fx, B-749 | 20.5, 20.8, 21.9, 25.8, 21.6 | 158.0, 162.4, 165.6, 177.4, 155.6 |
| ref8_fx, B-750 | 16.7, 10.7, 10.6, 12.3, 7.7 | 20.4, 31.0, 32.1, 29.4, 32.2 |
| ref8_wrap, before | 28.8, 26.6, 26.5, 26.2, 25.9 | 76.4, 85.2, 82.8, 83.6, 75.7 |
| ref8_wrap, B-749 | 12.8, 28.6, 45.9, 13.4, 41.3 | 18.5, 18.7, 19.0, 20.4, 18.2 |
| ref8_wrap, B-750 | 9.8, 9.7, 11.3, 10.7, 9.7 | 20.0, 21.1, 21.3, 22.6, 21.4 |

## What this says

- **B-749:** an 8 bpc frame with an adjustment layer is no longer drawn on the processor: on the card at Full 94.0 to 23.8 (3.9 times), Draft 44.5 to 21.4; with a Light Wrap at Full 82.8 to 18.7 (4.4 times). The Light Wrap shot's Draft at B-749 varied 12.8 to 45.9 from round to round (median 28.6 against 26.5 before), so no Draft gain is claimed for it from that build; the B-750 build's five rounds measured it at 9.7 to 11.3.
- **B-750:** with Drop Shadow, Bloom, Exposure and Solid Composite left on the card, ref8_fx at Full 156.4 to 31.0 (5 times), Draft 33.5 to 10.7. Before, these four ran on the processor in the middle of the card's frame (the card did not fall back, but sent the layer to the processor and back).
- **The `(none)` row** is not slower: Full 12.5 / 12.3 / 12.6, Draft 15.1 / 14.8 / 14.5.
