# B-420: frame times with Screen Key (D-540)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b420_screen_key --no-run` and run with `--ignored b420_screen_key_timing`: once on the card, then once with `B420_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0% busy; 3.6 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-11, on a machine that was not quiet), again: Noise alone card 44.9, processor 45.3; Noise, then Screen Key, screen colour orange, gain 120, clip 10 to 90 (on the card) card 157.5, processor 73.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame on each of its first three layers (so nothing is kept), then the Screen Key,
every eighth frame asked for whole at Full. **Again** is the median of the loops after the first
(seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.2 | 40.9 |
| Noise, then Screen Key, screen colour orange, gain 120, clip 10 to 90 (on the card) | 19.6 | 64.0 |
| Noise, then Screen Key, the same as the screen matte (on the card) | 19.2 | 55.3 |
| Noise, then Screen Key, the same with shrink 2, softness 1.5 and edge refine 3 (the processor) | 775.7 | 754.0 |

First loops (empty caches): card 15.9, 24.4, 22.6, 804.0; processor 40.7, 64.5, 54.6, 750.1.

**Reading it.** On the card, once warm, Screen Key adds about 7 ms a frame over Noise alone as a plain key (19.6 against 12.2, three layers, about 2.5 ms a 1080p layer) and about the same as the screen matte (19.2). On the processor it adds about 23 ms a frame as a plain key (64.0 against 40.9, about 7.7 ms a layer) and about 14 ms as the screen matte (55.3, about 4.8 ms a layer), so the card is about three times quicker there. With shrink, softness and edge refine it costs about 238 ms a layer on the processor and about 254 on the card (754.0 and 775.7), most of it Key Cleaner's guided fit (Key Cleaner alone is about 209 ms a layer, B-383), far over the row's Target P2 (4 ms). The busy run put that at about 344 ms a layer; quiet, it is about 238, still far over, and the card is no quicker for it. The card's figures, unreadable on the busy machine, can now be read.
