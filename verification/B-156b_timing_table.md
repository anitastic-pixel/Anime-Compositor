# B-156b: frame times, before and after

`b152_card_whole_frame_timing` in `tests/b152_card_whole_frame.rs`: the reference shot, 1920 by
1080, plain, with motion blur on every layer, with frame mix and dissolve, and with motion blur
and a Roughen Edges; every eighth frame asked for as the viewer asks, one loop to fill the caches
and seven timed, 210 frames; the figure is their median in ms. Run three times on each build; the
column is the median of the three runs. The runs before are in `verification/B-156b_timing_before.md`.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: e6d5f7d (the checks first). After: this build. No other cargo
  process was running. The three runs before were made first and the three after later the
  same day, not turn about.

| Shot | Quality | Moments added up on (before → after) | Before ms | After ms | After, runs 1 / 2 / 3 |
|---|---|---|---:|---:|---|
| the reference shot | Draft | none | 7.5 | 7.2 | 7.5 / 7.2 / 7.2 |
| the reference shot | Full | none | 9.6 | 9.4 | 9.6 / 9.4 / 9.3 |
| with motion blur | Draft | CPU → GPU | 15.4 | 8.6 | 8.7 / 8.6 / 8.6 |
| with motion blur | Full | CPU → GPU | 47.4 | 15.2 | 13.1 / 15.2 / 15.3 |
| with frame mix and dissolve | Draft | none (mixed on the CPU) | 45.4 | 46.0 | 44.1 / 46.6 / 46.0 |
| with frame mix and dissolve | Full | none (mixed on the CPU) | 61.1 | 61.8 | 54.1 / 61.8 / 65.8 |
| with motion blur and Roughen Edges | Draft | CPU → GPU | 25.8 | 12.5 | 12.5 / 12.4 / 13.7 |
| with motion blur and Roughen Edges | Full | CPU → GPU | 53.2 | 16.3 | 16.2 / 16.3 / 17.1 |

What it says:

- **Motion blur at Full: about three times quicker** (47.4 to 15.2 ms; with a Roughen Edges
  53.2 to 16.3 ms). Before, the processor added up every moment of every blurred layer and sent
  the result to the card; now the card adds them up itself.
- **Motion blur at Draft: about twice as quick** (15.4 to 8.6 ms; with a Roughen Edges 25.8 to
  12.5 ms).
- The plain shot and the frame-mix shot do not change beyond run-to-run noise: this build does
  not touch them. The frame mix is still done on the processor, which is why that shot is the
  slowest here.
