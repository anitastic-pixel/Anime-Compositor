# B-156c: frame times, before and after

`b152_card_whole_frame_timing` in `tests/b152_card_whole_frame.rs`, as in
`verification/B-156b_timing_table.md`: every eighth frame asked for as the viewer asks, one loop
to fill the caches and seven timed, 210 frames; the figure is their median in ms. Run three times
on each build; the column is the median of the three runs.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads, 64 GB
- System: Windows 11
- Build: release, both. Before: 32237bc (B-156b; its three runs are the "after" of
  `verification/B-156b_timing_table.md`). After: this build. No other cargo process was running.

| Shot | Quality | Before ms | After ms | After, runs 1 / 2 / 3 |
|---|---|---:|---:|---|
| the reference shot | Draft | 7.2 | 7.5 | 7.3 / 7.5 / 8.0 |
| the reference shot | Full | 9.4 | 9.8 | 9.8 / 9.4 / 11.0 |
| with motion blur | Draft | 8.6 | 8.8 | 8.8 / 8.8 / 10.7 |
| with motion blur | Full | 15.2 | 15.1 | 13.3 / 15.1 / 16.7 |
| **with frame mix and dissolve** | Draft | 46.0 | **31.3** | 31.2 / 31.3 / 32.5 |
| **with frame mix and dissolve** | Full | 61.8 | **44.8** | 44.8 / 45.3 / 44.8 |
| with motion blur and Roughen Edges | Draft | 12.5 | 12.7 | 12.7 / 12.6 / 12.9 |
| with motion blur and Roughen Edges | Full | 16.3 | 16.6 | 16.5 / 16.9 / 16.6 |

What it says:

- **Frame mix and dissolve: about a third quicker** (46.0 to 31.3 ms at Draft, 61.8 to 44.8 ms
  at Full). The mix is now shared among the processor's threads, and a drawing that is only read
  is no longer copied first.
- The other shots do not change beyond run-to-run noise: this build does not touch them.
- Such a frame is still about four times slower than the plain shot. The rest is the processor
  fetching the drawings and the card receiving a new mixed picture every frame; mixing on the
  card was not built (D-227 says why).
