# B-557: frame times with Foam (D-678)

**Measured again on a quiet machine** on 2026-10-14 at 3f506d38, built with `cargo test --release --test b557_foam --no-run` and run with `--ignored b557_foam_timing` on its own: once on the card, then once with `B557_CPU` set on the processor. Before each run, `tasklist` showed no cargo, rustc or other test program and the owner's app was not open. Before the card run the check waited 84 s for the card to settle (it read 18 to 19% busy meanwhile) and started with the card 8% busy and the processor 2%; before the processor run the card was 0% busy and the processor 2%. Both runs finished between the once-a-minute load readings. After the runs the card was 5% and 0% busy and the processor 5% and 1%. One run each. The machine: NVIDIA GeForce RTX 4070 Ti SUPER (driver 610.88, Vulkan), AMD Ryzen 9 9900X (24 threads), Windows 11, release build. The owner was using the computer lightly (web browsing) during these runs, as on a normal day.

Earlier figures (2026-10-13, from dd9c6fce, PROVISIONAL on a machine that was not quiet), Again, card / processor: Noise alone 34.1 / 70.8; as added (Draft) 102.8 / 169.1; Rendered, Layer texture 66.2 / 193.3; production rate keyed 64.5 / 151.6 ms.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

**What is timed.** The reference shot (1920 by 1080, 24 a second, 240 frames) with a Noise that
changes every frame (amount 12, colour, seed 7) on each of its first three layers, then
Foam, every eighth frame asked for whole at Full.
**Again** is the median of the loops after the first (seven on the card, two on the processor),
milliseconds a frame.

| Shot | Card | Processor |
|---|---:|---:|
| Noise alone | 12.9 | 41.3 |
| Noise, then Foam as added (the Draft view) | 32.9 | 72.4 |
| Noise, then Foam, Rendered, bigger bubbles, the Layer texture with a reflection | 33.4 | 93.4 |
| Noise, then Foam, Rendered, production rate keyed 0 to 2 (the settings read back to the in point) | 33.0 | 71.8 |

First loops (empty caches): card 17.2, 35.5, 37.0, 33.1; processor 40.9, 78.4, 93.0, 74.2.

**Reading it.** On the card, Foam costs about 20 ms a frame over the Noise alone as added
(12.9 to 32.9, three layers, the Draft view); Rendered with bigger bubbles and the Layer texture with a
reflection costs about the same (33.4), and keying the production rate adds nothing measurable (33.0).
On the processor Foam costs about 31 ms a frame as added (41.3 to 72.4) and about 52 ms Rendered with the
Layer texture (93.4). The card is about 2 to 3 times quicker. The earlier figures, two to three times
higher, were from the busy machine.
