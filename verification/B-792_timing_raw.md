- Processor: AMD64 Family 26 Model 68 Stepping 0, AuthenticAMD, 24 threads
- System: windows
- Build: release
- Shot: the reference shot, 1920 by 1080, Color Key, Noise (animated) and then the effect on layer a, frames 0 to 210 by 30 on the processor at Full, median ms of the whole frame; 5 rounds, Exact then Fast in each

| Shot | Round | Exact | Fast |
|---|---:|---:|---:|
| Bilateral Blur as added (radius 5, threshold 20) | 1 | 183.6 | 169.7 |
| Bilateral Blur, radius 40 | 1 | 6838.5 | 134.5 |
| Cartoon as added (detail radius 8, threshold 10) | 1 | 409.7 | 221.5 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 1 | 6913.0 | 195.5 |
| Bilateral Blur as added (radius 5, threshold 20) | 2 | 187.7 | 176.1 |
| Bilateral Blur, radius 40 | 2 | 6839.5 | 128.7 |
| Cartoon as added (detail radius 8, threshold 10) | 2 | 420.8 | 216.0 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 2 | 6899.8 | 200.6 |
| Bilateral Blur as added (radius 5, threshold 20) | 3 | 190.2 | 175.2 |
| Bilateral Blur, radius 40 | 3 | 6160.1 | 118.0 |
| Cartoon as added (detail radius 8, threshold 10) | 3 | 376.6 | 200.0 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 3 | 6519.9 | 176.5 |
| Bilateral Blur as added (radius 5, threshold 20) | 4 | 172.4 | 150.0 |
| Bilateral Blur, radius 40 | 4 | 6201.9 | 121.0 |
| Cartoon as added (detail radius 8, threshold 10) | 4 | 377.2 | 208.8 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 4 | 6483.4 | 181.1 |
| Bilateral Blur as added (radius 5, threshold 20) | 5 | 173.3 | 152.6 |
| Bilateral Blur, radius 40 | 5 | 6195.5 | 118.4 |
| Cartoon as added (detail radius 8, threshold 10) | 5 | 374.8 | 204.8 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 5 | 6335.0 | 202.1 |

| Shot | Exact, median of rounds | Fast, median of rounds | Fast / Exact |
|---|---:|---:|---:|
| Bilateral Blur as added (radius 5, threshold 20) | 183.6 | 169.7 | 0.92 |
| Bilateral Blur, radius 40 | 6201.9 | 121.0 | 0.02 |
| Cartoon as added (detail radius 8, threshold 10) | 377.2 | 208.8 | 0.55 |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 6519.9 | 195.5 | 0.03 |

## Median of the 5 rounds

| Shot | Exact ms | Fast ms | Fast is |
|---|---:|---:|---:|
| Bilateral Blur as added (radius 5, threshold 20) | 183.6 | 169.7 | 1.1x faster |
| Bilateral Blur, radius 40 | 6201.9 | 121.0 | 51.3x faster |
| Cartoon as added (detail radius 8, threshold 10) | 377.2 | 208.8 | 1.8x faster |
| Cartoon heavy (detail radius 40, threshold 20, 16 steps) | 6519.9 | 195.5 | 33.3x faster |
