# B-321: frame times with Write-on (D-441)

**PROVISIONAL.** Measured on 2026-10-10, built with `cargo test --release --test
b321_write_on` from 731260b7 (B-321's code), and run with `--ignored
b321_write_on_timing`: once on the card, then once with `B321_CPU` set on the processor.
The machine was not quiet: other lanes' cargo processes were running. To be measured again on a quiet machine.

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
| Noise alone | 36.4 | 78.6 |
| Noise, then Write-on as added | 48.6 | 86.8 |
| Noise, then Write-on keyed across, size 40 (up to 1000 marks) | 42.8 | 99.8 |
| Noise, then Write-on keyed across, size keyed 10 to 80 per mark, stroke length 2 | 40.4 | 99.8 |

First loops (empty caches): card 37.9, 51.2, 54.5, 42.2; processor 85.5, 79.7, 89.5, 87.8.

**Reading it.** On the card three Write-ons add about 4 to 12 ms a frame over the Noise alone
(provisional). The processor's rows move about from run to run on the busy machine (its Noise
alone row is slower than on other days), so they say little until measured again.
