# B-155: frame times before the build

`b155_gpu_chain_timing` in `tests/b155_gpu_chain.rs`, run three times on the checks-first build
(76394c2, when the card drew only the last effect of each layer), alternating with the build's
own runs. No other cargo process was running.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release

| Shot | Quality | Left to the card, first three layers | Run 1 ms | Run 2 ms | Run 3 ms |
|---|---|---|---:|---:|---:|
| the reference shot, runs of four, three and three | Draft | 1 / 1 / 1 | 10.2 | 10.8 | 10.5 |
| the reference shot, runs of four, three and three | Full | 1 / 1 / 1 | 10.9 | 10.6 | 10.1 |
| the same, the first two runs ending in a moving Noise | Draft | 1 / 1 / 1 | 13.8 | 13.5 | 13.9 |
| the same, the first two runs ending in a moving Noise | Full | 1 / 1 / 1 | 13.7 | 13.8 | 14.7 |
| the same, the first two runs beginning with a moving Noise | Draft | 1 / 1 / 1 | 10.4 | 10.6 | 10.7 |
| the same, the first two runs beginning with a moving Noise | Full | 1 / 1 / 1 | 96.7 | 95.7 | 96.4 |
