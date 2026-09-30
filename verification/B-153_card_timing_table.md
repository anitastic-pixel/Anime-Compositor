# B-153: frame times on three builds

Measured on 2026-09-29 between 22:51 and 23:07 by `b152_card_whole_frame_timing` in
`tests/b152_card_whole_frame.rs`, run with
`cargo test --release --test b152_card_whole_frame -- --ignored b152_card_whole_frame_timing`
on three builds taken turn about, four rounds:

- **before B-152**: the checks-first commit 5b28b19, the card still handing a blurred or mixed frame
  to the CPU whole, in a separate folder with its own build;
- **B-152**: e00e4e3, the card laying such layers, each sent through a new transfer buffer;
- **B-153**: this build, the card keeping its sending memory, its drawings' textures and its
  working textures from frame to frame.

The same test file was used on all three. Two other cargo processes (another agent's) were
running at the start of rounds 1 and 2 and none at the start of rounds 3 and 4. Whether the
owner's app was open was not checked.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

Each number is the median of 210 frames, in milliseconds: every eighth frame of 240, asked for as
the viewer asks, seven times after one loop that fills the caches. The frame is read back from the
card each time, as the test's "eight-bit picture" asks; the app's viewer shows it without reading
it back.

| Shot | Quality | Before B-152, each round | B-152, each round | B-153, each round | Before B-152, middle | B-152, middle | B-153, middle |
|---|---|---|---|---|---:|---:|---:|
| the reference shot | Draft | 7.7, 7.7, 7.7, 7.4 | 8.3, 9.9, 7.4, 7.4 | 7.8, 7.6, 7.2, 7.8 | 7.7 | 7.9 | 7.7 |
| the reference shot | Full | 10.6, 10.5, 10.2, 9.8 | 11.2, 13.1, 9.9, 9.7 | 10.1, 10.1, 9.3, 10.1 | 10.3 | 10.6 | 10.1 |
| the reference shot with motion blur | Draft | 16.0, 15.9, 16.5, 15.6 | 20.1, 20.5, 19.3, 17.1 | 15.8, 15.8, 15.1, 15.4 | 15.9 | 19.7 | 15.6 |
| the reference shot with motion blur | Full | 54.0, 50.0, 52.5, 47.9 | 55.8, 69.2, 69.5, 68.4 | 64.1, 66.5, 60.3, 42.9 | 51.2 | 68.8 | 62.2 |
| the reference shot with frame mix and dissolve | Draft | 50.5, 51.6, 47.0, 46.2 | 64.8, 66.7, 57.9, 57.8 | 58.8, 64.7, 54.6, 53.1 | 48.8 | 61.3 | 56.7 |
| the reference shot with frame mix and dissolve | Full | 63.6, 67.6, 54.2, 60.4 | 74.9, 63.9, 67.2, 66.7 | 68.9, 69.4, 59.3, 61.5 | 62.0 | 67.0 | 65.2 |
| the reference shot with motion blur and Roughen Edges | Draft | 19.3, 19.9, 18.3, 21.0 | 25.1, 26.0, 23.7, 23.8 | 22.5, 22.3, 21.9, 23.5 | 19.6 | 24.5 | 22.4 |
| the reference shot with motion blur and Roughen Edges | Full | 93.0, 94.9, 87.2, 87.7 | 76.7, 77.7, 74.8, 78.2 | 57.4, 57.6, 53.2, 58.1 | 90.3 | 77.2 | 57.5 |

**What it shows.** B-153 takes back most of what B-152 cost, and every B-153 middle is at or below
B-152's. With a card effect beside the blur at Full, the frame now takes 57.5 ms, against 90.3
before B-152. Motion blur at Draft is back to where it was, 15.6 ms against 15.9. But a frame
whose only unusual layers are blurred or mixed is still slower on the card than when the CPU drew
it whole: motion blur at Full 62.2 against 51.2, frame mix at Draft 56.7 against 48.8, and frame
mix at Full 65.2 against 62.0. The CPU's work of preparing the pictures now costs about a third
of what it did (measured separately while building, `Gpu` upload 10.85 to about 3.9 ms at Full).
What is left is the card's own copy of each new full-size picture, about 16 ms at Full for this
shot.

**An observation, not a measurement of this build.** While these runs went, `nvidia-smi` showed
the card in its low-power state P5 most of the time, graphics clock 210 to 480 MHz, and its link
at PCIe generation 2 with 8 lanes; the card and the board can do generation 4 with 16. The
viewer's short bursts of work do not wake the card fully, which is likely why its copies are
slow. That is the driver's power management, which this program does not set.

As D-218 said, such a frame now goes back to the CPU whole when no layer in it has an effect the
card draws. That is B-153b, next.
