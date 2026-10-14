# B-533: frame times with Rolling Shutter Repair (D-654)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b533_rolling_shutter_repair --no-run` and run with `--ignored b533_rolling_shutter_repair_timing` once, on its own, on the card (the effect itself runs on the processor). Before the run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. Before the run the card was 7% busy and the processor 9%. The run took 352 s. The load, read once a minute (5 readings), showed the processor 33 to 43% busy and the card 0 to 35%; Rolling Shutter Repair itself works on the processor and the Noise on the card, so these readings are mostly the test's own work. After the run the card was 0% busy and the processor 3%. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, from ef31a130, PROVISIONAL: other lanes' builds and B-532's timing ran at the same time), Again at Full: Noise alone 58.9; Warp 7260.4; Pixel Motion 1865.0 ms. Inside the effect, afresh: Warp 7257.8, Pixel Motion 1036.5, Warp at Half 387.2 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Rolling Shutter Repair stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 16.7 | 13.0 | 42.5 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Full | 742.5 | 714.0 | 765.2 | 609.9 |
| Rolling Shutter Repair, Pixel Motion, then Noise | Full | 259.5 | 251.3 | 273.2 | 145.4 |
| Noise alone | Half | 31.6 | 17.7 | 51.8 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Half | 225.1 | 214.9 | 257.5 | 164.0 |

Rolling Shutter Repair is new, so there is no "before" build. The "Noise alone" rows are the comparison.

## What is timed

This follows B-227's, B-530's and B-531's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Rolling Shutter Repair and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU (Rolling Shutter Repair itself runs on the processor).
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** is the median of the same 30 frames, each with empty caches, so every repair is worked. **Rolling Shutter Repair stage, afresh** is the time spent inside the effect on those frames.
- Milliseconds a frame.

## Reading it

- **Warp** (the setting when added) costs about 610 ms a frame inside the effect at Full and about 164 ms at Half.
- **Pixel Motion** costs about 145 ms a frame inside the effect at Full, when D-650's motion estimate has to run for both neighbouring frames, so here it is about 4 times quicker than Warp.
- First and Again now agree within 5% on every row with the effect. The earlier odd Warp columns (Again 7.3 s against First 2.7 s) were Warp Stabilizer's timing running at the same time.
- Every row with the effect is above the Target P4 (100 ms a frame on the processor) and slower than real time (41.7 ms at 24 frames a second). A card version and a faster estimate would each be separate pieces of work.

## After audit P5 (moves kept, both sides at once, 7152d077)

**Measured** on 2026-10-14 at 508a1d3d, built with `cargo test --release --test b533_rolling_shutter_repair --no-run` (own target folder) and run with `--ignored --exact b533_rolling_shutter_repair_timing`, one test at a time, nothing else of this work running. The run took about 2 minutes 20 seconds (352 s before). One reading of the card just before the run said 70% busy; it was taken the moment the previous timing test ended, and the Noise alone rows (12.3 and 18.0) match a quiet card. Same machine as above.

| Shot | Quality | First, before | First, after | Again, before | Again, after | Stage "afresh", before | Stage "afresh", after |
|---|---|---:|---:|---:|---:|---:|---:|
| Noise alone | Full | 16.7 | 16.4 | 13.0 | 12.3 | 0.0 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Full | 742.5 | 129.8 | 714.0 | 130.0 | 609.9 | 34.5 |
| Rolling Shutter Repair, Pixel Motion, then Noise | Full | 259.5 | 261.8 | 251.3 | 244.0 | 145.4 | 117.0 |
| Noise alone | Half | 31.6 | 27.1 | 17.7 | 18.0 | 0.0 | 0.0 |
| Rolling Shutter Repair as added (Warp), then Noise | Half | 225.1 | 77.0 | 214.9 | 57.5 | 164.0 | 11.3 |

Whole frames "afresh", after: 41.7, 207.7, 286.2, 49.5, 120.1.

**What changed in the columns.** "Afresh" empties the frame cache and the card before each frame, but the moves P5 keeps are not emptied, so after P5 the afresh frames are frames shown again, and the stage figure there is only the Warp's pixel work. **First** (the first loop, every eighth frame, nothing kept for those frames yet) is the cold figure to compare.

**Reading it.** Warp as added, cold, is about 5.7 times quicker at Full (742.5 to 129.8 ms a frame) and about 2.9 at Half (225.1 to 77.0). P5 measures the two neighbours at once and works the Warp's pixels on all threads; this run does not time those two apart. Shown again, its own stage drops from about 610 ms to about 35. Pixel Motion is about the same (its estimate is D-650's, not changed here). Every row with the effect is still slower than real time (41.7 ms at 24 a second); Warp at Half comes closest (57.5 again). Pictures unchanged: `D-654_rolling_shutter_repair_table.md` 111 of 111, including frames shown again byte-identical to cold ones.
