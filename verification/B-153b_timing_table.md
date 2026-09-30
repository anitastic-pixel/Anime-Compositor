# B-153b: frame times on three builds

Measured on 2026-09-29 between 23:21 and 23:35 by `b152_card_whole_frame_timing` in
`tests/b152_card_whole_frame.rs`, run with
`cargo test --release --test b152_card_whole_frame -- --ignored b152_card_whole_frame_timing`
on three builds taken turn about, four rounds:

- **before B-152**: the checks-first commit 5b28b19, the card handing every blurred or mixed frame
  to the CPU whole, in a separate folder with its own build;
- **B-153**: 311dc82, the card laying such layers and keeping its memory from frame to frame;
- **B-153b**: this build, the card handing a blurred or mixed frame to the CPU whole when no layer
  has an effect the card draws, and its sending memory no longer capped by the frame's size.

Each build used its own copy of the test file; the four shots and the way they are timed are the
same in all three. No other cargo process was running at the start of any round. Whether the
owner's app was open was not checked.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each number is the median of 210 frames, in milliseconds: every eighth frame of 240, asked for as
the viewer asks, seven times after one loop that fills the caches. The frame is read back from the
card each time, as the test's "eight-bit picture" asks; the app's viewer shows it without reading
it back.

| Shot | Quality | Before B-152, each round | B-153, each round | B-153b, each round | Before B-152, middle | B-153, middle | B-153b, middle |
|---|---|---|---|---|---:|---:|---:|
| the reference shot | Draft | 7.4, 7.5, 7.5, 7.5 | 7.4, 7.2, 7.4, 7.4 | 7.3, 7.4, 7.1, 7.5 | 7.5 | 7.4 | 7.3 |
| the reference shot | Full | 9.9, 9.9, 9.6, 9.7 | 9.3, 9.2, 9.3, 9.5 | 10.4, 9.2, 9.2, 9.4 | 9.8 | 9.3 | 9.3 |
| the reference shot with motion blur | Draft | 15.5, 15.5, 15.1, 15.4 | 14.7, 15.0, 14.6, 14.8 | 17.0, 14.8, 14.9, 15.0 | 15.4 | 14.8 | 14.9 |
| the reference shot with motion blur | Full | 48.1, 47.3, 46.9, 47.6 | 65.7, 59.5, 57.5, 47.8 | 50.5, 45.1, 45.9, 49.4 | 47.5 | 58.5 | 47.6 |
| the reference shot with frame mix and dissolve | Draft | 40.7, 41.3, 40.4, 40.9 | 55.9, 53.1, 63.3, 62.0 | 42.1, 39.3, 46.9, 47.6 | 40.8 | 59.0 | 44.5 |
| the reference shot with frame mix and dissolve | Full | 53.8, 52.7, 61.8, 52.4 | 60.8, 69.1, 47.2, 61.6 | 53.2, 61.4, 59.8, 52.6 | 53.2 | 61.2 | 56.5 |
| the reference shot with motion blur and Roughen Edges | Draft | 18.6, 18.7, 19.4, 18.3 | 21.5, 18.4, 21.4, 21.8 | 21.5, 22.9, 21.5, 21.5 | 18.6 | 21.4 | 21.5 |
| the reference shot with motion blur and Roughen Edges | Full | 87.4, 86.7, 85.9, 86.3 | 53.6, 55.8, 52.4, 55.0 | 53.6, 54.9, 54.4, 55.1 | 86.5 | 54.3 | 54.6 |

**What it shows.** Motion blur at Full is back to before B-152, 47.6 ms against 47.5 (B-153
58.5). Frame mix is faster than B-153 (Draft 44.5 against 59.0, Full 56.5 against 61.2) but its
middle is still 3 to 4 ms above before B-152's. For these two shots B-153b runs the same code as
before B-152: the card looks at the frame, hands it back, and the CPU draws it as it did then. B-153b's
frame mix rounds range from 39.3 to 47.6 ms at Draft, from below to above before B-152's four
(40.4 to 41.3), so this measurement cannot say whether a real difference is left. The shot with
a Roughen Edges stays on the card and keeps B-153's gain at Full, 54.6 against 86.5 before B-152;
at Draft it stays 3 ms slower than before B-152, as in B-153.

The earlier measurement (`verification/B-153_card_timing_table.md`) gave somewhat different
numbers for the same builds. Each table compares only the builds within it, measured turn about.
