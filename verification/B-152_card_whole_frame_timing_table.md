# B-152: frame times before and after

Measured on 2026-09-29 by `b152_card_whole_frame_timing` in `tests/b152_card_whole_frame.rs`, run with
`cargo test --release --test b152_card_whole_frame -- --ignored b152_card_whole_frame_timing`: "before" on the
checks-first commit 5b28b19 (the card still handing such frames to the CPU whole), in a separate folder with its
own build, "after" on the B-152 build, the two taken turn about. No other cargo process was running at the start
of any run; whether the owner's app was open was not checked.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each number is the median of 210 frames, in milliseconds: every eighth frame of 240, asked for as the viewer asks,
seven times after one loop that fills the caches. The first three shots were run four times on each build; the
last, the motion blur shot with a Roughen Edges (an effect the card draws, changing every frame) on the first
layer, whose own motion blur switch is off, once on each. An earlier version of that shot left the first
layer's switch on, which keeps its effects on the processor; its runs are not used.

| Shot | Quality | Before, each run | After, each run | Before, middle | After, middle |
|---|---|---|---|---:|---:|
| the reference shot | Draft | 8.0, 7.6, 7.6, 7.5 | 7.5, 8.1, 7.4, 7.4 | 7.6 | 7.5 |
| the reference shot | Full | 10.2, 9.4, 10.9, 9.5 | 9.5, 11.0, 9.6, 9.4 | 9.8 | 9.6 |
| the reference shot with motion blur | Draft | 16.6, 15.2, 17.7, 15.3 | 16.3, 19.2, 16.6, 17.2 | 16.0 | 16.9 |
| the reference shot with motion blur | Full | 55.5, 46.8, 53.4, 48.4 | 47.6, 55.6, 60.2, 68.8 | 50.9 | 57.9 |
| the reference shot with frame mix and dissolve | Draft | 52.4, 43.0, 47.8, 52.6 | 65.6, 61.9, 56.6, 64.3 | 50.1 | 63.1 |
| the reference shot with frame mix and dissolve | Full | 62.8, 61.8, 59.1, 61.6 | 68.6, 66.1, 66.1, 58.8 | 61.7 | 66.1 |
| the reference shot with motion blur and Roughen Edges | Draft | 19.1 | 24.3 | 19.1 | 24.3 |
| the reference shot with motion blur and Roughen Edges | Full | 86.9 | 72.8 | 86.9 | 72.8 |

**What it shows.** B-152 alone does not make these frames faster, except the one with a card effect beside the
blur at Full (86.9 to 72.8 ms). A frame whose only unusual layers are blurred or mixed is slower on the card by
about 5 to 15 ms, at Draft by more with frame mix: the CPU builds a new full-size picture for each such layer every
frame either way, and sending it to the card, a new transfer buffer each time, costs more than the CPU laying it.
That sending cost is what the plan's next item, G2 (B-153: persistent resources, one staging buffer), removes.
