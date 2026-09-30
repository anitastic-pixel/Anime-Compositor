# B-151: frame times before and after, PROVISIONAL

Measured on 2026-09-29 by `b151_gpu_fx_timing` in `tests/b151_gpu_fx.rs`, run with
`cargo test --release --test b151_gpu_fx -- --ignored b151_gpu_fx_timing`, once on the build before
B-151 (main at a763fc0, in a separate folder, its own build) and once on the B-151 build, one after
the other.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**PROVISIONAL: the machine was busy.** Two other cargo processes (other agents' builds and tests)
were running during both runs, and whether the owner's app was open was not checked. A quiet
re-measure comes at the end of the GPU plan.

**What is timed.** The reference shot with the effect on three layers (one after a Drop Shadow),
every eighth frame of 240, each asked for as the viewer asks for a frame, whole, in milliseconds.
**First** is the median of the first loop's 30 frames, started with empty caches: what a frame
costs the first time it is seen. **Again** is the median of the next seven loops' 210 frames: the
drawings and any effect that does not change with time are already done, so an effect that holds
still costs little either way, and Roughen Edges and Snowfall, which change every frame, show the
card's work. "None" is the reference shot as it is, with no effect added: the floor. The CPU
column is the processor drawing the whole frame, which B-151 does not change; it is here as the
comparison.

| Effect | Quality | GPU first, before | GPU first, after | GPU again, before | GPU again, after | CPU again, after |
|---|---|---:|---:|---:|---:|---:|
| None | Draft | 11.9 | 10.5 | 9.2 | 8.5 | 9.8 |
| None | Full | 25.8 | 28.3 | 10.5 | 11.9 | 24.3 |
| Median | Draft | 14.9 | 15.3 | 11.6 | 12.4 | 13.7 |
| Median | Full | 29.5 | 15.8 | 10.9 | 10.8 | 25.7 |
| Smart Blur | Draft | 21.3 | 16.3 | 12.6 | 12.2 | 12.6 |
| Smart Blur | Full | 28.9 | 15.5 | 10.8 | 10.3 | 23.2 |
| Roughen Edges | Draft | 29.3 | 21.5 | 13.1 | 17.4 | 12.0 |
| Roughen Edges | Full | 152.3 | 34.5 | 102.6 | 22.7 | 115.3 |
| Radial Shadow | Draft | 20.2 | 16.7 | 11.7 | 11.6 | 12.6 |
| Radial Shadow | Full | 28.4 | 17.4 | 10.8 | 12.5 | 24.9 |
| Bevel Alpha | Draft | 21.5 | 21.1 | 12.7 | 11.0 | 12.7 |
| Bevel Alpha | Full | 29.0 | 31.8 | 10.8 | 11.3 | 24.7 |
| Snowfall | Draft | 27.2 | 36.3 | 12.5 | 21.2 | 14.8 |
| Snowfall | Full | 102.5 | 33.9 | 67.0 | 22.9 | 82.3 |
| Cell Pattern | Draft | 22.0 | 22.6 | 12.5 | 13.1 | 12.6 |
| Cell Pattern | Full | 28.3 | 33.3 | 10.9 | 11.4 | 24.3 |
| Polar Coordinates | Draft | 20.8 | 22.8 | 12.6 | 11.7 | 12.7 |
| Polar Coordinates | Full | 28.4 | 25.7 | 10.8 | 9.6 | 22.0 |
| Optics Compensation | Draft | 20.7 | 20.2 | 12.6 | 11.8 | 11.3 |
| Optics Compensation | Full | 28.6 | 25.6 | 10.8 | 9.7 | 21.8 |
| Corner Pin | Draft | 21.6 | 20.1 | 12.6 | 11.6 | 11.5 |
| Corner Pin | Full | 27.9 | 25.6 | 10.8 | 9.7 | 21.8 |

**What it shows.** At Full, the card now draws Roughen Edges about 4.5 times as fast as before
(again 102.6 to 22.7 ms) and Snowfall about 3 times (67.0 to 22.9 ms); the first sight of a frame
with Median, Smart Blur or Radial Shadow falls from about 29 to 16-17 ms. The other five cost
little on the reference shot's drawings either way. At Draft, where the processor works on a
sixteenth of the pixels, the card is no faster, and Snowfall (again 12.5 to 21.2 ms) and Roughen
Edges (13.1 to 17.4 ms) are slower: sending a drawing and running a double-precision pass costs
more than the processor's small job. Whether Draft should keep these on the processor is for G6's
double-precision audit, after a quiet re-measure.
