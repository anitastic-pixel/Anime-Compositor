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

## Quiet re-measure, 2026-09-30

Measured again with no other cargo process or build running, to settle the frame mix question
above. Same test, three builds taken turn about, the order turned each round, three rounds; the
figure is the median of the three:

- **before B-152**: 5b28b19, with B-153b's test file;
- **B-153b**: 6353cd0;
- **today**: c9bf93e, the program as it stands after the whole GPU plan.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads, 64 GB
- System: Windows 11
- Build: release, each build in its own folder with its own build (target)

| Shot | Quality | Before B-152 | B-153b | Today | Before B-152, runs | B-153b, runs | Today, runs |
|---|---|---:|---:|---:|---|---|---|
| the reference shot | Draft | 7.4 | 7.3 | 7.4 | 8.0, 7.4, 7.4 | 7.3, 7.6, 7.2 | 7.4, 7.4, 8.1 |
| the reference shot | Full | 9.6 | 9.3 | 9.8 | 10.4, 9.6, 9.5 | 9.3, 9.4, 9.3 | 9.8, 9.6, 11.0 |
| the reference shot with motion blur | Draft | 15.3 | 15.2 | 9.3 | 15.3, 15.5, 15.3 | 15.1, 15.2, 16.3 | 9.3, 9.0, 10.3 |
| the reference shot with motion blur | Full | 47.1 | 46.1 | 13.5 | 47.3, 47.1, 46.7 | 46.1, 46.1, 51.9 | 13.4, 13.5, 15.1 |
| the reference shot with frame mix and dissolve | Draft | 40.6 | 41.1 | 28.5 | 40.0, 40.6, 40.7 | 41.1, 40.7, 44.7 | 28.1, 28.5, 29.9 |
| the reference shot with frame mix and dissolve | Full | 53.6 | 52.6 | 41.0 | 60.5, 53.6, 52.3 | 53.7, 52.6, 52.1 | 41.0, 41.6, 40.9 |
| the reference shot with motion blur and Roughen Edges | Draft | 18.5 | 22.8 | 9.5 | 21.6, 18.5, 18.4 | 22.9, 22.6, 22.8 | 9.5, 13.0, 9.5 |
| the reference shot with motion blur and Roughen Edges | Full | 86.5 | 51.5 | 16.6 | 89.2, 86.3, 86.5 | 51.8, 51.5, 51.0 | 17.6, 16.6, 16.6 |

**What it answers.** Frame mix at Draft is 41.1 ms on B-153b against 40.6 before B-152, inside
the spread of the rounds (40.0 to 40.7 against 40.7 to 44.7): the 44.5 against 40.8 above was the
busy machine, and no real difference is left. Today, after the rest of the GPU plan, the same shot
takes 28.5 ms at Draft and 41.0 at Full. The shot with motion blur and Roughen Edges was really
slower at Draft on B-153b (22.8 against 18.5, every round), as the table above said; today it
takes 9.5 ms. Motion blur alone is 9.3 ms at Draft and 13.5 at Full today, against 15.3 and 47.1
before B-152.
