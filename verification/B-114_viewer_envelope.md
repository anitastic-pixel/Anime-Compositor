# B-114: the envelope, measured the way the viewer runs

Written by `tests/b114_viewer_envelope.rs` (`cargo test --release --test b114_viewer_envelope -- --ignored`), for D-174. Timings are this machine's on the day; run it again and they will move a little.

## Machine, build and configuration

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- Memory: 66.1 GB
- Frames drawn on the card, NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory
- Build: release
- Preview: draft, frames 0 to 239, one frame at a time
- Cache: built as the window builds it, `CelCache::viewer_sized`: nine sixteenths for drawings, seven for effect results

The older pages, `verification/T-06_performance_envelope.md` and `verification/T-06_declared_fixture.md`, measured a cache the window no longer builds: 1 GiB, drawings only, composited on the processor. They stay as the record of that. This page is what the envelope now rests on.

## What came back

Times in ms. A 24 fps clock allows 41.7 ms a frame; a frame over it is one the viewer drops rather than play slowly (D-32). **Cold** is the whole first time round the shot, reading every drawing from disk. **Warm** is the tenth time round the whole shot. **Seek** is a jump to a frame far from the last, after those ten loops. **Again** is asking for the frame just shown.

| Shot | Cache | Cold, first loop in all | Warm median | Warm p95 | Warm frames over 41.7 ms | Seek p95 | Again p95 | Drawings read from disk, tenth loop | Effect results reused, tenth loop | Cache holds (MiB) | Working set, loop 2 → 10 (MiB) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| The reference shot: four layers, no mattes, no effects | 15756 MiB (Automatic on this machine) | 696 | 0.50 | 0.96 | 0 of 240 | 1.26 | 0.88 | 0 | 0 | 1772 | 2069 → 2069 |
| The reference shot: four layers, no mattes, no effects | 1 GiB (the least Automatic gives) | 3449 | 14.13 | 20.49 | 0 of 240 | 19.73 | 3.00 | 430 | 0 | 570 | 870 → 806 |
| Document 08's shot: ten layers, two mattes, three effects | 15756 MiB (Automatic on this machine) | 2289 | 4.82 | 7.28 | 0 of 240 | 7.18 | 7.33 | 0 | 680 | 5343 | 5711 → 5583 |
| Document 08's shot: ten layers, two mattes, three effects | 1 GiB (the least Automatic gives) | 8779 | 36.35 | 51.70 | 63 of 240 | 54.58 | 11.38 | 1290 | 680 | 660 | 900 → 901 |

Peak working set of the whole run: 5735 MiB. That is the largest cache above plus the program, not something the window would add to it.

**Document 08's target** is a p95 seek at or below 100 ms and playback at 24 fps. Read them off the Seek p95 and the frames-over column. Peak memory on the card is not measured.
