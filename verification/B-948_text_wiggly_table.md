# D-1068: Text Animator, the Wiggly Selector

B-948. The title is "Neon Sign" with the tutorial's animator: Opacity 25, the Range Selector picking nothing (End 0, as if deleted), and a Wiggly Selector with Mode Add, Max 100, Min -100, 3 wiggles a second, Correlation 50, unless a row says. Each number is how much the animator picks a letter (1 is fully, so 25% opacity; 0 or below is full brightness). Spaces are left out. FX-TXA-001 to 023 are unchanged and pass in B-230.

## Off is as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| a file with no Wiggly words and one with Wiggly off pick the same (End 50: the first half) | 1.00, 1.00, 1.00, 1.00, 0.00, 0.00, 0.00, 0.00 | yes |

## The wiggle

| Check | The build's answer | Matches |
| --- | --- | --- |
| at 0 seconds each letter has its own pick, from -1 to 1, not all the same | -0.34, -0.07, -0.92, -0.46, -0.47, -0.83, -0.13, -0.02 | yes |
| the same time picks the same again | -0.34, -0.07, -0.92, -0.46, -0.47, -0.83, -0.13, -0.02 | yes |
| half a second later the picks have changed | 0.23, -0.06, 0.40, 0.28, 0.07, 0.51, 0.04, 0.13 | yes |
| 3 wiggles a second at 0.5 s pick what 1 wiggle a second does at 1.5 s | 0.23, -0.06, 0.40, 0.28, 0.07, 0.51, 0.04, 0.13 | 0.23, -0.06, 0.40, 0.28, 0.07, 0.51, 0.04, 0.13 | yes |
| 0 wiggles a second holds still | -0.34, -0.07, -0.92, -0.46, -0.47, -0.83, -0.13, -0.02 | yes |
| Correlation 100: every letter picks the same | 0.67, 0.67, 0.67, 0.67, 0.67, 0.67, 0.67, 0.67 | yes |
| Max 50 and Min 50: every letter picks 0.5 | 0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50, 0.50 | yes |
| Random Seed 1 is another wiggle | 0.63, 0.18, -0.05, -0.06, 0.06, 0.30, -0.04, 0.19 | yes |
| Temporal Phase 180 at 0 s is the wiggle half a wiggle later | -0.04, 0.09, -0.10, -0.07, 0.04, -0.12, -0.13, 0.02 | yes |

## Modes, with the Range Selector picking every letter (End 100)

| Check | The build's answer | Matches |
| --- | --- | --- |
| intersect: 1 times the wiggle: the wiggle | 0.25, 0.25, 0.67, 0.30, 0.52, 0.55, -0.14, 0.06 | yes |
| add: 1 plus the wiggle, held to 1 | 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 0.86, 1.00 | yes |
| subtract: 1 less the wiggle, held to 1 | 0.75, 0.75, 0.33, 0.70, 0.48, 0.45, 1.00, 0.94 | yes |
| min: the smaller: the wiggle | 0.25, 0.25, 0.67, 0.30, 0.52, 0.55, -0.14, 0.06 | yes |
| max: the larger: 1 | 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00 | yes |
| difference: the gap from 1 | 0.75, 0.75, 0.33, 0.70, 0.48, 0.45, 1.00, 0.94 | yes |

## Through compose, saved and refused

| Check | The build's answer | Matches |
| --- | --- | --- |
| frame 8 at 3 wiggles a second draws what frame 24 at 1 does (24 frames a second) | compared | yes |
| frame 8 differs from frame 0 | compared | yes |
| with Wiggly off, frame 8 and frame 0 are the same | compared | yes |
| saved with Wiggly on, every Wiggly word is written and none of the time | wiggly "on", mode "add", rate 3, seed 0 | yes |
| a file from before it saves again with no Wiggly words | no wiggly_ words | yes |
| an unknown mode switches the animator off with a message | runs: false | yes |
| neon_frames.png: frames 0, 6, 12, 18 and 24 (a second) from the top; letters dim and brighten each on its own | drawn | yes |

## Result

23 of 23 checks pass.
