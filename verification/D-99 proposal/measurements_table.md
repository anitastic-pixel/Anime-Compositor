The same frame with no effect at all takes 47.8 ms.

| Layer | Effect | Largest | Pixels moving more than 3 | Before, ms | After, ms | Times faster |
|---|---|---|---|---|---|---|
| background | Gaussian Blur, sigma 8 | 8 | 0.3% | 55.3 | 45.5 | 1.2x |
| background | Glow, bright parts over 40, radius 40, intensity 2 | 7 | 0.2% | 66.9 | 43.8 | 1.5x |
| background | Directional Blur, direction 30, length 40 | 34 | 14.8% | 55.0 | 44.1 | 1.2x |
| background | Bloom, radius 30, star streaks of 60 at angle 20 | 32 | 6.0% | 188.5 | 62.4 | 3.0x |
| background | Radial Blur, zoom 20 from the centre | 51 | 9.7% | 231.3 | 47.4 | 4.9x |
| background | Line Width, 4 thicker, black lines | 0 | 0.0% | 57.8 | 46.0 | 1.3x |
| ball | Gaussian Blur, sigma 8 | 1 | 0.0% | 50.9 | 45.8 | 1.1x |
| ball | Glow, bright parts over 40, radius 40, intensity 2 | 18 | 0.0% | 58.7 | 45.8 | 1.3x |
| ball | Directional Blur, direction 30, length 40 | 9 | 0.5% | 51.5 | 46.7 | 1.1x |
| ball | Bloom, radius 30, star streaks of 60 at angle 20 | 23 | 0.1% | 163.2 | 60.5 | 2.7x |
| ball | Radial Blur, zoom 20 from the centre | 6 | 0.3% | 56.3 | 44.3 | 1.3x |
| ball | Line Width, 4 thicker, black lines | 16 | 0.1% | 52.9 | 44.4 | 1.2x |
