# B-156: frame times, before and after

`b156_gpu_adjust_timing` in `tests/b156_gpu_adjust.rs`: the reference shot, 1920 by 1080, with
an adjustment layer on top, every eighth frame asked for as the viewer asks, one loop to fill
the caches and seven timed, 210 frames; the figure is their median in ms. Run three times on
each build; the column is the median of the three runs. The runs before are in
`verification/B-156_timing_before.md`.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: 6a24bc5 (the checks first). After: this build. No other cargo
  process was running. The three runs before were made first and the three after later the
  same day, not turn about.

| Shot | Quality | Drawn on (before → after) | Before ms | After ms | After, runs 1 / 2 / 3 |
|---|---|---|---:|---:|---|
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame | Draft | CPU → GPU | 18.1 | 13.8 | 13.8 / 14.8 / 13.8 |
| Levels, Gaussian Blur and Hue/Saturation, over the whole frame | Full | CPU → GPU | 61.4 | 23.2 | 22.6 / 24.1 / 23.2 |
| the same, over part of it | Draft | CPU → GPU | 20.3 | 15.0 | 14.8 / 15.8 / 15.0 |
| the same, over part of it | Full | CPU → GPU | 62.1 | 23.2 | 22.1 / 23.2 / 23.3 |
| a moving Noise, then the same, over the whole frame | Draft | CPU → GPU | 19.1 | 15.5 | 15.2 / 15.5 / 15.5 |
| a moving Noise, then the same, over the whole frame | Full | CPU → GPU | 75.5 | 23.6 | 25.4 / 23.4 / 23.6 |

What it says:

- **Full: about two and a half to three times quicker** (61 to 76 ms down to 23 to 24 ms). Before,
  any adjustment layer sent the whole frame to the processor. Now the card draws the layers
  beneath, runs the adjustment layer's effects on what it drew and mixes the result back.
- **Draft: about a quarter quicker** (18 to 20 ms down to 14 to 16 ms). The small draft frame
  was already quicker on the processor, so there is less to win.
- Under an adjustment layer the card now keeps every drawing at full precision (twice the
  memory of the usual half precision; D-225 says why). That cost is inside these figures.
