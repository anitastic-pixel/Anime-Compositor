# B-155: frame times, before and after

`b155_gpu_chain_timing` in `tests/b155_gpu_chain.rs`: the reference shot on the card, 1920 by
1080, every eighth frame asked for as the viewer asks, one loop to fill the caches and seven
timed, 210 frames; the figure is their median in ms. Run three times on each build, the two
builds alternating; the column is the median of the three runs. The runs themselves are in
`verification/B-155_timing_before.md` (before) and below (after).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: 76394c2 (the checks first). After: this build. No other cargo
  process was running.

| Shot | Quality | Effects the card draws, first three layers (before → after) | Before ms | After ms | After, runs 1 / 2 / 3 |
|---|---|---|---:|---:|---|
| the reference shot, runs of four, three and three | Draft | 1 / 1 / 1 → 4 / 3 / 3 | 10.5 | 10.1 | 10.3 / 10.1 / 10.0 |
| the reference shot, runs of four, three and three | Full | 1 / 1 / 1 → 4 / 3 / 3 | 10.6 | 9.7 | 9.7 / 10.3 / 9.5 |
| the same, the first two runs ending in a moving Noise | Draft | 1 / 1 / 1 → 5 / 4 / 3 | 13.8 | 12.2 | 14.2 / 12.0 / 12.2 |
| the same, the first two runs ending in a moving Noise | Full | 1 / 1 / 1 → 5 / 4 / 3 | 13.8 | 14.9 | 15.4 / 13.4 / 14.9 |
| the same, the first two runs beginning with a moving Noise | Draft | 1 / 1 / 1 → 5 / 4 / 3 | 10.6 | 14.2 | 14.6 / 12.4 / 14.2 |
| the same, the first two runs beginning with a moving Noise | Full | 1 / 1 / 1 → 5 / 4 / 3 | 96.4 | 24.7 | 24.9 / 23.2 / 24.7 |

What it says:

- **Still effects: about the same.** When nothing in the stack moves, the effect cache already
  keeps the processor's part from one frame to the next, so moving it to the card saves little.
- **A moving effect at the end: about the same.** The card redraws only the moving effect: it
  keeps its picture after each effect of the run and starts again from the one before the
  change. Draft is 1.6 ms quicker, Full 1.1 ms slower, both within the spread of the runs.
- **A moving effect at the start: Full is four times quicker, Draft 3.6 ms slower.** At Full,
  before, the processor redrew the whole stack every frame on a 1920 by 1080 layer; the card
  now does it (96.4 → 24.7 ms). At Draft, the processor's work on a quarter-size layer is
  already small, and setting up seven more effects on the card costs more than it saves (3.6 ms
  in all; how that splits between the effects was not measured). G12/G13 of the plan (fewer passes a frame) are
  where that set-up cost is meant to come down.
- The first measurement of this build, before the card kept its picture after each effect,
  was 20.3 ms at Full for "ending in a moving Noise": the whole run was redrawn each frame.
  That is fixed, and `b155_kept_run_is_fresh` checks the kept pictures give exactly the fresh
  card's bytes.
