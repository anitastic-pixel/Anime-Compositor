# B-232: frame times with Matte Choker, Refine Hard Matte and Refine Soft Matte

Measured on 2026-10-08, after the code commit (8a05033), on the code that commit holds. No other
cargo or rustc process was running and the owner's app was not open: checked before every round
and after the last, none. The processor's load from background programs was not recorded. The
timing test is `b232_matte_refine_timing` in `tests/b232_matte_refine.rs`, built with
`cargo test --release --test b232_matte_refine` and run three rounds in a row with
`--ignored b232_matte_refine_timing --exact`, each round timing the five shots in the order below
(about 610 seconds a round).

The three effects are new, so there is no "before" build: the row with Noise alone is the
comparison. Matte Choker is drawn by the graphics card; Refine Hard Matte and Refine Soft Matte
by the processor, the viewer's frame then finished by the card.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (`verification/B-08a_project.json`, 1920 by 1080, 24 a
second, 240 frames) with a Noise that changes every frame on each of its first three layers and
the effect after it, so the effect has new input every frame and is worked every time. Every
eighth frame is asked for whole, as the viewer asks for it (`preview_frame_srgb8`), at Full with
Draw on: GPU. **First** is the median of the first loop of 30 frames, starting with empty
caches; **Again** the median of the next seven loops, 210 frames. Each figure is the median of
the three rounds, in milliseconds a frame.

| Shot | First | Again |
|---|---:|---:|
| Noise alone (before) | 16.9 | 12.6 |
| Noise, then Matte Choker as added (card) | 42.0 | 39.7 |
| Noise, then Refine Hard Matte as added (processor) | 671.6 | 655.8 |
| Noise, then Refine Hard Matte, decontaminate on (processor) | 917.9 | 844.7 |
| Noise, then Refine Soft Matte as added, edge radius 10, decontaminate on (processor) | 928.1 | 910.2 |

The rounds, first / again:

| Shot | Round 1 | Round 2 | Round 3 |
|---|---|---|---|
| Noise alone | 16.1 / 12.0 | 16.9 / 13.1 | 16.9 / 12.6 |
| Matte Choker | 40.9 / 38.7 | 42.0 / 40.2 | 42.5 / 39.7 |
| Refine Hard Matte | 671.7 / 659.7 | 671.6 / 655.8 | 669.7 / 653.5 |
| Refine Hard Matte, decontaminate | 955.0 / 850.9 | 867.4 / 844.7 | 917.9 / 841.9 |
| Refine Soft Matte | 953.8 / 910.2 | 926.7 / 900.9 | 928.1 / 913.4 |

**Against the target.** EFFECTS.md gives Matte Choker Target P2 (4 ms or less added) and the two
Refine effects Target P3 (8 ms or less). None meets it.

- **Matte Choker** adds about 27 ms a frame on three 1080p layers, about 9 ms a layer: above P2,
  though the frame (39.7 ms) is still inside a 24 frames a second budget (41.7 ms). Its running
  totals and disc sums are worked in double precision, which this card does slowly; that keeps it
  within 1 level of the processor.
- **Refine Hard Matte and Refine Soft Matte** take about 0.65 to 0.9 seconds a frame on three
  1080p layers, about 0.2 to 0.3 seconds a layer, so the viewer plays at one or two frames a
  second with them; export is unaffected in result, only slower. Each frame works several
  thirteen-channel box sums and a 3 by 3 solve per pixel in double precision on the processor.
  The way to speed them up, not done here, is a card pass (the box sums are the same running
  totals Matte Choker's pass already uses); the processor's box sums could also be made to add
  their columns in parallel (`src/matte_refine.rs`, `box_sum`).
