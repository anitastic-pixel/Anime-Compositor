# B-600: Turbulent Noise blending modes, timing (D-730)

**PROVISIONAL: the machine was busy.** Other lanes' tests (b155_gpu_chain, b557_foam, b570_liquify_brushes) and cargo builds ran on the card and processor throughout, so these numbers are not a speed claim; re-measure on a quiet machine.

The reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, then Turbulent Noise in each mode; every eighth frame of 240 drawn whole at Full, as the viewer asks. Median milliseconds a frame: the first pass, then the later passes. Produced by `tests/b600_turbulent_blends.rs`, `b600_turbulent_blends_timing` (`--release --ignored`, `B600_CPU` set for the processor).

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 49.6 | 45.2 |
| Noise, then Turbulent Noise as added | Full | 38.1 | 50.1 |
| the same in Luminosity | Full | 73.7 | 57.0 |
| the same in None at opacity 60 | Full | 97.4 | 69.0 |
| the same in Stencil Luma | Full | 61.7 | 53.8 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 108.1 | 107.3 |
| Noise, then Turbulent Noise as added | Full | 204.7 | 221.3 |
| the same in Luminosity | Full | 138.8 | 103.2 |
| the same in None at opacity 60 | Full | 205.5 | 204.0 |
| the same in Stencil Luma | Full | 157.5 | 198.4 |

## What else was running

```
== before_card
6132 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "b557_foam-7d46623bcf0d557f.exe" "cargo.exe" "rustc.exe" 
== between
9397 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "b557_foam-7d46623bcf0d557f.exe" "b570_liquify_brushes-957899983bb913a5.exe" "cargo.exe" 
== after
7800 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "b570_liquify_brushes-957899983bb913a5.exe" "cargo.exe" "rustc.exe" 
```
