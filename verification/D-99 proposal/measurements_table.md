The same frame with no effect at all takes 42.7 ms.

| Layer | Effect | Largest | Pixels moving more than 3 | Before, ms | After, ms | Times faster |
|---|---|---|---|---|---|---|
| background | Gaussian Blur, sigma 8 | 8 | 0.3% | 53.1 | 41.9 | 1.3x |
| background | Glow, bright parts over 40, radius 40, intensity 2 | 7 | 0.2% | 66.1 | 42.3 | 1.6x |
| background | Directional Blur, direction 30, length 40 | 34 | 14.8% | 56.2 | 40.9 | 1.4x |
| background | Bloom, radius 30, star streaks of 60 at angle 20 | 32 | 6.0% | 187.4 | 57.0 | 3.3x |
| background | Radial Blur, zoom 20 from the centre | 41 | 11.3% | 215.1 | 43.3 | 5.0x |
| background | Line Width, 4 thicker, black lines | 0 | 0.0% | 54.5 | 41.5 | 1.3x |
| ball | Gaussian Blur, sigma 8 | 1 | 0.0% | 48.3 | 42.1 | 1.1x |
| ball | Glow, bright parts over 40, radius 40, intensity 2 | 18 | 0.0% | 55.1 | 44.5 | 1.2x |
| ball | Directional Blur, direction 30, length 40 | 9 | 0.5% | 47.6 | 42.4 | 1.1x |
| ball | Bloom, radius 30, star streaks of 60 at angle 20 | 23 | 0.1% | 155.9 | 53.4 | 2.9x |
| ball | Radial Blur, zoom 20 from the centre | 6 | 0.2% | 55.9 | 40.7 | 1.4x |
| ball | Line Width, 4 thicker, black lines | 16 | 0.1% | 51.3 | 40.8 | 1.3x |
