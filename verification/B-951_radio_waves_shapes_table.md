# D-1071: Radio Waves, Curve Size, Star, Star Depth and Frequency

B-951. One white wave on a black 200-pixel solid at frame 10: a square (Sides 4) turned so its sides stand straight, 60 pixels from the middle to a corner and 42.43 to a side's middle, 2 pixels wide. Each row reads five pixels' red, 0 black to 1 white; a check passes when a pixel meant to be lit is above 0.5 and one meant to be black is below 0.01: a side's middle (142, 100); the corner (142, 142); the diagonal 43.1 out (130, 130), where a circle touching the sides' middles passes; the diagonal 51.6 out (136, 136), where Curve Size 0.5's round corner passes (30 to the shrunk corner and 21.2 more); and (121, 100), 21.5 out towards a side, where a star of Star Depth -0.5 has its inner corner.

## The polygon, Curve Size and Star

| Check | The build's answer | Matches |
| --- | --- | --- |
| defaults: the sharp square, as before | 1 1 0 0 0 | yes |
| Curve Size 1: a circle through the sides' middles | 1 0 0.79 0 0 | yes |
| Curve Size 0.5: the corner rounded, the side's middle kept | 1 0 0 1 0 | yes |
| Star on, Star Depth -0.5: the corner kept, the side pulled in to 21.2 | 0 1 0 0 1 | yes |
| Star on, Star Depth 0: the square's own outline | 1 1 0 0 0 | yes |
| Star off: Star Depth does nothing | 1 1 0 0 0 | yes |

## Frequency, waves a second (the composition runs at 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Frequency 0: Interval 1000 sends only the first wave | 1 0 | yes |
| Frequency 2: a wave every 12 frames, so a second sent at frame 12 | 1 1 | yes |
| Frequency 0.5: every 48 frames, so still one | 1 0 | yes |
| Frequency 2 with Interval 5: the Frequency rules | 1 1 | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| the viewer with the card lights Curve Size 1, the circle's pixel (it hands the wave to the CPU) | red 230 of 255 | yes |
| the viewer with the card lights the star's inner corner (handed to the CPU) | red 255 of 255 | yes |
| the viewer with the card lights Frequency 2's second wave (drawn on the card) | red 255 of 255 | yes |

## Saved and refused

| Check | The build's answer | Matches |
| --- | --- | --- |
| a file without them saves without them | none of the four | yes |
| set ones are written | curve 0.78, star "on", depth -0.5, frequency 1.9 | yes |
| an unknown Star word switches the effect off | runs: false | yes |
| a Curve Size above 1 switches the effect off | runs: false | yes |
| a Frequency below 0 switches the effect off | runs: false | yes |
| shapes.png: the tutorial's look, white waves on orange, Frequency 4 at 24 frames a second: a sharp hexagon, Curve Size 0.78, and a five-point star at Star Depth -0.5 | drawn | yes |

## Result

19 of 19 checks pass.
