# B-321: frame times with Write-on (D-441)

**Measured again on a quiet machine** on 2026-10-13 at 90a33e9a, built with `cargo test --release --test b321_write_on --no-run` and run with `--ignored b321_write_on_timing`: once on the card, then once with `B321_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the card was idle (0 and 5% busy; 4.1 GB of its memory held by the desktop and open programs); the same after each. The processor was 4% busy or less before each run. The owner's app was not open. One run each, single figures, not medians of rounds.

Earlier provisional figures (2026-10-10, on a machine that was not quiet), again: Noise alone card 36.4, processor 78.6; Noise, then Write-on as added card 48.6, processor 86.8 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-253's: the reference shot (1920 by 1080, 24 a second, 240 frames) with
a Noise that changes every frame (amount 12, colour, seed 7) on each of its first three layers,
then a Write-on, every eighth frame asked for whole at Full. The keyed brush goes from 10, 20 at
frame 0 to 90, 80 at frame 239, so by the end each layer holds about 1000 marks. **Again** is the
median of the loops after the first (seven on the card, two on the processor), milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.6 | 42.4 |
| Noise, then Write-on as added | 18.5 | 44.7 |
| Noise, then Write-on keyed across, size 40 (up to 1000 marks) | 24.3 | 47.7 |
| Noise, then Write-on keyed across, size keyed 10 to 80 per mark, stroke length 2 | 19.5 | 45.9 |

First loops (empty caches): card 16.5, 24.3, 31.5, 23.4; processor 41.9, 44.5, 48.1, 45.2.

**Reading it.** On the card three Write-ons add about 6 to 12 ms a frame over the Noise alone, about
2 to 4 ms a 1080p layer (18.5 to 24.3 against 12.6); the most marks (size 40, up to 1000) cost
the most. On the processor they add only 2 to 5 ms a frame, about 1 to 2 ms a layer (44.7 to 47.7
against 42.4), so here the processor is the quicker of the two. The busy run could not say this.
