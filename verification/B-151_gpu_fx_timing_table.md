# B-151: frame times before and after

Measured again on 2026-09-30 with no other cargo process or build running (the quiet re-measure
the GPU plan ended with; the first measurement, on 2026-09-29, ran beside two other agents'
builds). `b151_gpu_fx_timing` in `tests/b151_gpu_fx.rs`, run with
`cargo test --release --test b151_gpu_fx -- --ignored b151_gpu_fx_timing`, on three builds, each
in its own folder with its own build, taken turn about, the order turned each round, three rounds:

- **before**: a763fc0, the commit before B-151, with B-151's test file;
- **B-151**: 9a55de2;
- **today**: c9bf93e, the program as it stands after the whole GPU plan.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads, 64 GB
- System: Windows 11
- Build: release

**What is timed.** The reference shot with the effect on three layers (one after a Drop Shadow),
every eighth frame of 240, each asked for as the viewer asks for a frame, whole, in milliseconds.
**First** is the median of the first loop's 30 frames, started with empty caches: what a frame
costs the first time it is seen. **Again** is the median of the next seven loops' 210 frames: the
drawings and any effect that does not change with time are already done, so an effect that holds
still costs little either way, and Roughen Edges and Snowfall, which change every frame, show the
card's work. "None" is the reference shot as it is, with no effect added: the floor. The CPU
column is the processor drawing the whole frame on the B-151 build, as the comparison. Each figure
is the median of the three rounds.

| Effect | Quality | GPU first, before | GPU first, B-151 | GPU first, today | GPU again, before | GPU again, B-151 | GPU again, today | CPU again, B-151 |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| None | Draft | 10.6 | 10.5 | 9.6 | 7.8 | 7.9 | 8.1 | 9.5 |
| None | Full | 13.1 | 12.9 | 11.3 | 9.5 | 9.6 | 9.6 | 22.0 |
| Median | Draft | 18.2 | 18.8 | 17.0 | 11.9 | 11.5 | 11.6 | 11.7 |
| Median | Full | 27.9 | 14.3 | 11.4 | 9.9 | 9.8 | 9.6 | 22.1 |
| Smart Blur | Draft | 24.9 | 15.8 | 17.0 | 12.3 | 11.5 | 11.8 | 11.6 |
| Smart Blur | Full | 28.3 | 14.1 | 11.7 | 11.4 | 9.6 | 9.7 | 22.0 |
| Roughen Edges | Draft | 35.1 | 24.1 | 23.0 | 13.8 | **16.1** | **16.5** | 11.3 |
| Roughen Edges | Full | 150.9 | 28.4 | 19.0 | 103.9 | 21.2 | 17.9 | 98.7 |
| Radial Shadow | Draft | 26.3 | 15.7 | 16.6 | 12.4 | 11.4 | 11.4 | 11.5 |
| Radial Shadow | Full | 25.4 | 16.5 | 11.3 | 10.3 | 9.8 | 9.5 | 21.8 |
| Bevel Alpha | Draft | 25.0 | 21.4 | 17.2 | 12.3 | 12.2 | 12.0 | 11.5 |
| Bevel Alpha | Full | 15.5 | 14.3 | 12.5 | 9.7 | 9.6 | 11.0 | 22.2 |
| Snowfall | Draft | 25.1 | 25.7 | 24.1 | 12.2 | **16.1** | **18.0** | 11.3 |
| Snowfall | Full | 96.8 | 27.6 | 19.1 | 73.1 | 21.6 | 18.1 | 68.4 |
| Cell Pattern | Draft | 27.3 | 19.9 | 16.3 | 12.4 | 11.5 | 11.4 | 11.6 |
| Cell Pattern | Full | 26.6 | 14.3 | 11.2 | 11.4 | 9.5 | 9.5 | 22.0 |
| Polar Coordinates | Draft | 26.7 | 20.2 | 17.0 | 13.6 | 12.1 | 11.9 | 11.4 |
| Polar Coordinates | Full | 16.2 | 14.5 | 11.5 | 10.3 | 9.5 | 9.6 | 21.6 |
| Optics Compensation | Draft | 22.0 | 20.7 | 19.0 | 12.2 | 12.2 | 13.7 | 11.4 |
| Optics Compensation | Full | 26.5 | 14.4 | 12.3 | 9.8 | 9.6 | 10.9 | 22.1 |
| Corner Pin | Draft | 25.9 | 20.1 | 17.7 | 12.2 | 12.4 | 12.9 | 11.5 |
| Corner Pin | Full | 26.8 | 15.1 | 12.3 | 9.9 | 10.4 | 9.6 | 22.1 |

The three rounds of every figure are in the job's raw files; the "first" figures of the before
build spread most (Radial Shadow at Full 14.8, 25.4, 28.4), the "again" figures within about 1 ms.

**What it shows.** At Full, the card draws Roughen Edges about 5 times as fast as before (again
103.9 to 21.2 ms, today 17.9) and Snowfall about 3.4 times (73.1 to 21.6, today 18.1); the first
sight of a frame with Median, Smart Blur, Cell Pattern or Corner Pin falls from about 27 to 14-15
ms (today 11-12). The others cost little on the reference shot's drawings either way.

**A real slowdown at Draft, still there today.** At Draft, where the processor works on a
sixteenth of the pixels, the card is slower than before B-151 for the two effects that change
every frame: Roughen Edges again 13.8 to 16.1 ms (today 16.5) and Snowfall 12.2 to 16.1 (today
18.0), in all three rounds of each. The processor draws both frames in 11.3 ms. The rest of the
GPU plan did not remove it. Handing these two to the processor at Draft is the obvious fix; it is
not built. Optics
Compensation at Draft (12.2 to 13.7 today) and Bevel Alpha at Full (9.7 to 11.0 today) are about
1.5 ms slower today than on B-151, beyond the spread of their rounds; which later unit did it was
not measured.
