# B-783: Refine Soft and Hard Matte, Exact against Fast (D-903)

Written by `b783_fast_refine_timing` in `tests/b783_fast_refine.rs` (`--release --ignored`). Re-measured on 2026-10-27 in a quiet slot, nothing else building on the machine (the first run, under other lanes' builds, gave 0.62 to 0.79); Exact and Fast are taken in turn within each round.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Shot: the reference shot, 1920 by 1080, Color Key, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the processor at Full, median ms of the whole frame; 5 rounds, Exact then Fast in each

| Shot | Round | Exact | Fast |
|---|---:|---:|---:|
| Refine Soft Matte as added (edge radius 10) | 1 | 303.1 | 254.4 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 1 | 438.8 | 268.0 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 1 | 252.5 | 156.3 |
| Refine Hard Matte, feather 16 | 1 | 235.1 | 155.7 |
| Refine Soft Matte as added (edge radius 10) | 2 | 315.0 | 254.1 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 2 | 470.3 | 259.9 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 2 | 242.2 | 151.8 |
| Refine Hard Matte, feather 16 | 2 | 246.1 | 156.5 |
| Refine Soft Matte as added (edge radius 10) | 3 | 312.1 | 251.2 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 3 | 459.0 | 266.5 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 3 | 253.7 | 155.1 |
| Refine Hard Matte, feather 16 | 3 | 245.7 | 157.2 |
| Refine Soft Matte as added (edge radius 10) | 4 | 332.4 | 260.6 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 4 | 470.4 | 281.4 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 4 | 259.0 | 157.2 |
| Refine Hard Matte, feather 16 | 4 | 246.0 | 157.6 |
| Refine Soft Matte as added (edge radius 10) | 5 | 318.4 | 254.0 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 5 | 444.7 | 258.8 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 5 | 251.9 | 155.9 |
| Refine Hard Matte, feather 16 | 5 | 249.8 | 153.8 |

| Shot | Exact, median of rounds | Fast, median of rounds | Fast / Exact |
|---|---:|---:|---:|
| Refine Soft Matte as added (edge radius 10) | 315.0 | 254.1 | 0.81 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 459.0 | 266.5 | 0.58 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 252.5 | 155.9 | 0.62 |
| Refine Hard Matte, feather 16 | 246.0 | 156.5 | 0.64 |
