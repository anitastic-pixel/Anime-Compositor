# B-748 (D-868): timings, every blend mode on the card

Measured with `tests/w8_arch_audit.rs`'s `w8_arch_timing` (wave 8's measurement, `W8_NO_EXPORT=1`), the before build being main at 7cc14eb3 and the after build B-748, both release test binaries.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- Build: release, `CARGO_INCREMENTAL=0`
- Load: a shared machine. Four other lanes were building and running their own card and export tests throughout (`b754_float_card`, `b222_gpu_colour`, `b740_mask_cache`, `b160_faster_export`, several `cargo` and `rustc`). The same unchanged CPU column moves by up to 2 times from round to round, so only large and consistent differences mean anything.
- Shots: wave 8's reference shots `ref_L4`, `ref_L10`, `ref_L25` and `ref_L50` (4, 10, 25 and 50 layers over `Fixtures/reference_shot`), frames 0, 30, 60, 90 and 120, median of the five frames; then the median of 5 rounds, before and after alternating. ms a frame.
- `ref_L4` has only Normal layers (the `(none)` row: nothing in it changes). `ref_L10` has Overlay and Lighten among its layers, `ref_L25` and `ref_L50` Overlay and Lighten several times, so before B-748 the card refused them and the CPU drew the whole frame ("fell back yes").

## The quieter set of 5 rounds (timing_p2d)

| Shot | CPU Draft before / after | CPU Full before / after | Card Draft before / after | Card Full before / after | Card fell back before / after |
|---|---:|---:|---:|---:|---|
| ref_L4 | 21.3 / 13.9 | 18.7 / 15.2 | 15.2 / 9.3 | 19.3 / 13.4 | no / no |
| ref_L10 | 87.1 / 61.1 | 138.9 / 113.1 | 66.9 / 76.7 | 209.3 / 79.6 | yes / no |
| ref_L25 | 272.2 / 275.6 | 385.3 / 468.9 | 243.6 / 312.1 | 394.6 / 372.4 | yes / no |
| ref_L50 | 608.3 / 923.9 | 995.0 / 1212.4 | 656.0 / 605.4 | 1063.8 / 644.7 | yes / no |

Card Draft per round, ref_L25: before 243.6, 256.4, 182.8, 260.1, 201.2; after 299.6, 339.6, 286.1, 346.0, 312.1. ref_L10: before 75.6, 58.0, 66.9, 57.0, 200.6; after 78.6, 117.8, 76.7, 59.5, 73.1.

## The loaded set of 5 rounds (timing_p2, while the other lanes' tests ran)

| Shot | Card Draft before / after | Card Full before / after |
|---|---:|---:|
| ref_L4 | 37.4 / 15.5 | 17.0 / 16.4 |
| ref_L10 | 84.6 / 211.8 | 163.0 / 166.0 |
| ref_L25 | 408.1 / 400.3 | 493.6 / 493.8 |
| ref_L50 | 734.5 / 653.1 | 1051.3 / 681.8 |

## ref_L10 alone, 7 and 5 rounds

| Run | Card Draft before / after | Card Full before / after |
|---|---:|---:|
| 7 rounds (timing_p2b) | 72.5 / 127.6 | 174.1 / 155.6 |
| 5 rounds, with ref_L10 all Normal beside it (timing_p2c) | 48.4 / 55.4 | 101.8 / 57.9 |
| the same run, ref_L10 with every layer Normal | 103.4 / 81.1 | 93.3 / 100.1 |

## What this says

- **Full:** the frames that now stay on the card are faster than the CPU drawing them, in every set: ref_L50 1063.8 to 644.7 and 1051.3 to 681.8; ref_L10 209.3 to 79.6 and 101.8 to 57.9. ref_L25 is about the same (394.6 to 372.4; 493.6 to 493.8).
- **Draft is slower on ref_L10 and ref_L25**, by about 10 to 70 ms a frame (ref_L25 312.1 against 243.6, slower in every one of the 5 rounds). That is not the new modes: the card was already slower than the CPU at Draft on these shots with every layer Normal. The before build (no B-748 code at all), ref_L25 with every layer Normal, 5 rounds: CPU Draft 257.6, card Draft 533.4 (`timing_p2e`); ref_L10 all Normal above, 103.4 on the card against 57.6 on the CPU. B-748 moves the frames with other modes onto the card, so they take the card's Draft cost too. ref_L50's Draft is faster (656.0 to 605.4).
- **The `(none)` row**, ref_L4, is not slower: 15.2 to 9.3 at Draft, 19.3 to 13.4 at Full.
- **No 10 times.** The proposal guessed that 10 to 50 layer shots would be about 10 times faster. Measured, the best is 2.6 times at Full (ref_L10, 209.3 to 79.6). The likely reason, not measured here: most of the frame time on these shots goes into getting each layer's picture ready before anything is composited, and the CPU and the card share that part.

The rounds are kept in the lane's scratchpad (`wave9/w9c/timing_p2`, `timing_p2b` to `timing_p2e`).
