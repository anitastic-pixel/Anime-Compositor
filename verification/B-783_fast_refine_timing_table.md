# B-783: Refine Soft and Hard Matte, Exact against Fast (D-903)

Written by `b783_fast_refine_timing` in `tests/b783_fast_refine.rs` (`--release --ignored`). Other wave 11 lanes were building on the machine at the same time, so each figure carries that load; Exact and Fast were taken in turn within each round so both carry it alike.

- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Shot: the reference shot, 1920 by 1080, Color Key, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the processor at Full, median ms of the whole frame; 5 rounds, Exact then Fast in each

| Shot | Round | Exact | Fast |
|---|---:|---:|---:|
| Refine Soft Matte as added (edge radius 10) | 1 | 294.1 | 239.3 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 1 | 420.4 | 252.1 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 1 | 235.7 | 159.5 |
| Refine Hard Matte, feather 16 | 1 | 247.8 | 162.9 |
| Refine Soft Matte as added (edge radius 10) | 2 | 330.8 | 274.4 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 2 | 459.9 | 285.3 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 2 | 252.7 | 158.5 |
| Refine Hard Matte, feather 16 | 2 | 236.1 | 149.1 |
| Refine Soft Matte as added (edge radius 10) | 3 | 322.6 | 259.6 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 3 | 431.9 | 259.0 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 3 | 234.0 | 152.9 |
| Refine Hard Matte, feather 16 | 3 | 234.4 | 152.6 |
| Refine Soft Matte as added (edge radius 10) | 4 | 312.2 | 245.8 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 4 | 449.6 | 266.9 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 4 | 239.8 | 151.3 |
| Refine Hard Matte, feather 16 | 4 | 230.8 | 153.8 |
| Refine Soft Matte as added (edge radius 10) | 5 | 306.0 | 244.5 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 5 | 414.6 | 266.9 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 5 | 236.5 | 159.5 |
| Refine Hard Matte, feather 16 | 5 | 238.2 | 149.5 |

| Shot | Exact, median of rounds | Fast, median of rounds | Fast / Exact |
|---|---:|---:|---:|
| Refine Soft Matte as added (edge radius 10) | 312.2 | 245.8 | 0.79 |
| Refine Soft Matte heavy (edge radius, feather, decontamination radius 40) | 431.9 | 266.9 | 0.62 |
| Refine Hard Matte heavy (feather, decontamination radius 40) | 236.5 | 158.5 | 0.67 |
| Refine Hard Matte, feather 16 | 236.1 | 152.6 | 0.65 |
