# B-601: Turbulent Noise Overlay and Hue, timing (D-731)

**PROVISIONAL: the machine was busy.** Another lane's test (b155_gpu_chain) and cargo ran on the card and processor throughout, so these numbers are not a speed claim; re-measure on a quiet machine.

The reference shot (1920 by 1080, 24 a second) with a Noise that changes every frame on its first three layers, then Turbulent Noise as added, in Overlay and in Hue; every eighth frame of 240 drawn whole at Full, as the viewer asks. Median milliseconds a frame: the first pass, then the later passes. Produced by `tests/b601_turbulent_overlay_hue.rs`, `b601_turbulent_overlay_hue_timing` (`--release --ignored`, `B601_CPU` set for the processor).

## On the card

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the card
- Loops: 8

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 45.5 | 43.3 |
| Noise, then Turbulent Noise as added | Full | 52.2 | 55.0 |
| the same in Overlay | Full | 56.1 | 53.4 |
| the same in Hue | Full | 28.9 | 48.0 |

## On the processor

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory
- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Drawn by: the processor
- Loops: 3

| Shot | Quality | First | Again |
|---|---|---:|---:|
| Noise alone | Full | 106.9 | 115.0 |
| Noise, then Turbulent Noise as added | Full | 214.6 | 220.9 |
| the same in Overlay | Full | 105.5 | 111.4 |
| the same in Hue | Full | 113.6 | 157.6 |

## What else was running

```
== before_card
6953 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "cargo.exe" 
== between
6757 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "cargo.exe" 
== after
6756 MiB
"b155_gpu_chain-bee64f3aecf235d4.exe" "cargo.exe" 
```
