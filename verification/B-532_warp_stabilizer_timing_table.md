# B-532: frame times with Warp Stabilizer (D-653)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b532_warp_stabilizer --no-run` and run with `--ignored b532_warp_stabilizer_timing` once, on its own, on the card (the effect itself runs on the processor). Before the run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. Before the run the card was 0% busy and the processor 3%. The run took 48 minutes (2858 s). The load, read once a minute (44 readings), showed the processor 43% busy (the middle reading; never below 15%) and the card 0% (the middle reading). Warp Stabilizer itself works on the processor, so these readings are mostly the test's own work; they cannot tell the owner's browsing apart from it. After the run the card was 6% busy and the processor 5%. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, from ff1cbcf6, PROVISIONAL: other lanes' builds and B-533's timing overlapped; that run took about 3 hours 25 minutes), Again at Full: Noise alone 18.5; as added 10326.8; Detailed Analysis 19050.9 ms. Inside the effect, afresh: as added 6258.3, Detailed Analysis 12666.7, Half 1116.0 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | First | Again | Afresh | Warp Stabilizer stage, afresh |
|---|---|---:|---:|---:|---:|
| Noise alone | Full | 16.1 | 12.3 | 40.6 | 0.0 |
| Warp Stabilizer as added, then Noise | Full | 3442.7 | 3593.4 | 3683.1 | 3224.4 |
| Warp Stabilizer, Stabilize Only, Detailed Analysis on, then Noise | Full | 5958.4 | 5937.8 | 6030.7 | 5611.9 |
| Noise alone | Half | 31.6 | 18.1 | 51.9 | 0.0 |
| Warp Stabilizer as added, then Noise | Half | 1158.3 | 1188.3 | 1269.2 | 1018.7 |

Warp Stabilizer is new, so there is no "before" build. The "Noise alone" rows are the comparison.

## What is timed

This follows B-227's, B-530's and B-531's method:
- The reference shot: 1920 by 1080, 24 frames a second, 3 layers.
- On each layer, Warp Stabilizer and then a Noise that changes every frame, so no finished frame is kept.
- Every eighth frame of 240, each asked for whole as the viewer asks. Draw on: GPU (Warp Stabilizer itself runs on the processor).
- **First** is the median of the first loop's 30 frames, started with empty caches. **Again** is the median of the seven loops after it.
- **Afresh** is the median of the same 30 frames, each with empty caches, so every stabilization is worked. **Warp Stabilizer stage, afresh** is the time spent inside the effect on those frames.
- Milliseconds a frame.

## Reading it

- At Smoothness 50 each frame works the motion between 12 pairs of neighbouring frames a layer, and none is kept for the next frame, so the cache does not help.
- **As added** costs about 3.2 s a frame inside the effect at Full and about 1.0 s at Half.
- **Detailed Analysis on** costs about 5.6 s a frame inside the effect at Full, about 1.7 times as added.
- First and Again now agree within 5% on every row with the effect. The earlier odd columns (Again far above First, and the reverse) were the other timing running at the same time.
- Every row with the effect is far above the Target P4 (100 ms a frame on the processor) and far from real time (41.7 ms at 24 frames a second). Keeping each pair's motion from frame to frame, a card version and a faster estimate would each be separate pieces of work.
