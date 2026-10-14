# B-556: frame times with Star Burst (D-677)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b556_star_burst --no-run` and run with `--ignored b556_star_burst_timing` on its own: once on the card, then once with `B556_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. Before the card run the check waited 406 s for the card to settle (it read 10 to 27% busy from desktop programs meanwhile) and started with the card 4% busy and the processor 4%; before the processor run the card was 7% busy and the processor 5%. During the card run one reading showed the processor 17% busy and the card 3%. After the runs the card was 1% and 10% busy and the processor 4% and 2%. One run each. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, from fa7a59f8, PROVISIONAL on a machine that was not quiet), Again, card / processor: Noise alone 14.1 / 61.5; as added 25.8 / 97.0; Grid Spacing 0.5 320.3 / 516.0; Speed keyed, Blend 30 91.6 / 102.5 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Star Burst, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.8 | 43.1 |
| Noise, then Star Burst as added | 23.7 | 69.5 |
| Noise, then Star Burst, Grid Spacing 0.5 (about half a million stars) | 283.7 | 453.7 |
| Noise, then Star Burst, Speed keyed 0 to 5, Blend w. Original 30 | 24.7 | 70.3 |

First loops (empty caches): card 16.5, 25.5, 295.9, 25.8; processor 42.1, 70.1, 454.4, 70.0.

**Reading it.** On the card, Star Burst costs about 11 ms a frame over the Noise alone as added
(12.8 to 23.7, three layers), and keying Speed with Blend w. Original 30 adds about 1 ms (24.7). The cost
grows with the grid's star count: at Grid Spacing 0.5 (about half a million stars) it is about 271 ms a
frame (283.7). On the processor Star Burst costs about 26 ms a frame as added (43.1 to 69.5) and about
411 ms at Grid Spacing 0.5 (453.7). The card is about 3 times quicker as added and about 1.6 times quicker
with half a million stars. The earlier keyed figure on the card (91.6) was from the busy machine.
