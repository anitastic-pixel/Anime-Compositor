# B-273: frame times with Split 2, and Split after its engine change

Measured on 2026-10-09 on the code commit (c849fdda). No cargo or rustc process was running
before or after any round and the owner's app was not running; the two card rounds agree within
0.7 ms on every shot, so the machine is taken as quiet. Another agent works in a separate
worktree and could have started a build mid-round; nothing in the figures suggests it did. The
timing test is `b273_split_2_timing` in `tests/b273_split_2.rs`, built with
`cargo test --release --test b273_split_2` and run with `--ignored b273_split_2_timing`
(`B273_CPU` set for the processor).

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD Ryzen 9 9900X (AMD64 Family 26 Model 68), 24 threads
- System: Windows 11
- Build: release

**What is timed.** As B-270: the reference shot `verification/B-08a_project.json`, 1920 by 1080,
each of its first three layers a Noise that changes every frame and then the effect, every eighth
frame asked for whole at Full, with Draw on: GPU (`preview_frame_srgb8`) or by the processor alone
(`preview_frame_cached`). **First** is the median of the first loop, from empty caches; **Again**
the median of the loops after it (8 loops on the card, 3 on the processor). Milliseconds a frame.

| Shot | Card first, round 1 | Card again, round 1 | Card first, round 2 | Card again, round 2 | Processor first | Processor again |
|---|---:|---:|---:|---:|---:|---:|
| Noise alone | 16.2 | 12.0 | 16.6 | 12.4 | 41.5 | 41.7 |
| Noise, then Split, a diagonal (10, 10 to 90, 90), split 200 | 20.9 | 16.3 | 21.2 | 16.4 | 69.0 | 71.4 |
| Noise, then Split 2, the same diagonal, split 1 200, split 2 40 | 20.5 | 16.5 | 19.8 | 16.2 | 71.2 | 72.0 |

**Reading it.** Per 1080p layer, from the "again" figures over three layers:

| Effect | Card | Card against its target | Processor |
|---|---|---|---|
| Split 2 | about 1.3 to 1.5 ms (16.5 against 12.0; 16.2 against 12.4) | within Target P2's 4 ms | about 10.1 ms (72.0 against 41.7) |
| Split, after the change | about 1.3 to 1.4 ms | within P2 | about 9.9 ms |

**Before and after for Split.** Split's engine now takes two amounts. Before, in
`verification/B-270_distort_timing_table.md` (provisional, a busier machine), Split cost about
11.1 ms a layer on the processor and too little to read on the card; now about 9.9 ms and
1.3 to 1.4 ms. The change adds no measurable cost; the difference is the machine.
