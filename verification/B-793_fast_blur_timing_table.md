- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Shot: the reference shot, 1920 by 1080, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the processor at Full, median ms of the whole frame; 5 rounds, Exact then Fast in each

| Shot | Round | Exact | Fast |
|---|---:|---:|---:|
| Glow, radius 30 (sigma 10) | 1 | 73.9 | 67.9 |
| Glow, radius 60 (sigma 20) | 1 | 79.6 | 70.8 |
| Bloom, radius 30, star | 1 | 204.6 | 196.0 |
| Bloom, radius 100, star | 1 | 236.0 | 176.5 |
| Deband, blur 30 (sigma 10) | 1 | 131.5 | 156.8 |
| Deband, blur 60 (sigma 20) | 1 | 184.4 | 158.3 |
| Halation as added (radius 25) | 1 | 143.8 | 123.8 |
| Halation, radius 100 | 1 | 160.3 | 130.6 |
| Glow, radius 30 (sigma 10) | 2 | 71.9 | 74.8 |
| Glow, radius 60 (sigma 20) | 2 | 87.7 | 70.3 |
| Bloom, radius 30, star | 2 | 215.1 | 199.7 |
| Bloom, radius 100, star | 2 | 226.0 | 189.0 |
| Deband, blur 30 (sigma 10) | 2 | 135.6 | 139.0 |
| Deband, blur 60 (sigma 20) | 2 | 184.5 | 152.7 |
| Halation as added (radius 25) | 2 | 146.6 | 136.9 |
| Halation, radius 100 | 2 | 164.2 | 130.3 |
| Glow, radius 30 (sigma 10) | 3 | 72.3 | 75.8 |
| Glow, radius 60 (sigma 20) | 3 | 80.5 | 66.8 |
| Bloom, radius 30, star | 3 | 203.6 | 203.6 |
| Bloom, radius 100, star | 3 | 233.7 | 187.6 |
| Deband, blur 30 (sigma 10) | 3 | 135.7 | 137.5 |
| Deband, blur 60 (sigma 20) | 3 | 185.1 | 147.7 |
| Halation as added (radius 25) | 3 | 154.1 | 120.5 |
| Halation, radius 100 | 3 | 155.6 | 130.8 |
| Glow, radius 30 (sigma 10) | 4 | 76.2 | 68.0 |
| Glow, radius 60 (sigma 20) | 4 | 78.7 | 69.6 |
| Bloom, radius 30, star | 4 | 205.7 | 193.1 |
| Bloom, radius 100, star | 4 | 233.4 | 186.5 |
| Deband, blur 30 (sigma 10) | 4 | 131.2 | 149.1 |
| Deband, blur 60 (sigma 20) | 4 | 187.8 | 146.7 |
| Halation as added (radius 25) | 4 | 147.4 | 124.8 |
| Halation, radius 100 | 4 | 160.6 | 129.2 |
| Glow, radius 30 (sigma 10) | 5 | 71.7 | 73.7 |
| Glow, radius 60 (sigma 20) | 5 | 81.1 | 66.4 |
| Bloom, radius 30, star | 5 | 201.7 | 200.5 |
| Bloom, radius 100, star | 5 | 233.7 | 193.2 |
| Deband, blur 30 (sigma 10) | 5 | 140.2 | 137.0 |
| Deband, blur 60 (sigma 20) | 5 | 185.4 | 155.9 |
| Halation as added (radius 25) | 5 | 148.9 | 122.1 |
| Halation, radius 100 | 5 | 152.3 | 125.0 |

| Shot | Exact, median of rounds | Fast, median of rounds | Fast / Exact |
|---|---:|---:|---:|
| Glow, radius 30 (sigma 10) | 72.3 | 73.7 | 1.02 |
| Glow, radius 60 (sigma 20) | 80.5 | 69.6 | 0.86 |
| Bloom, radius 30, star | 204.6 | 199.7 | 0.98 |
| Bloom, radius 100, star | 233.7 | 187.6 | 0.80 |
| Deband, blur 30 (sigma 10) | 135.6 | 139.0 | 1.03 |
| Deband, blur 60 (sigma 20) | 185.1 | 152.7 | 0.82 |
| Halation as added (radius 25) | 147.4 | 123.8 | 0.84 |
| Halation, radius 100 | 160.3 | 130.3 | 0.81 |
