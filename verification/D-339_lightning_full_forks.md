# D-339 / B-219: Lightning Bolt's full-width long forks

From P-26's tutorial 2 (Advanced Electric), 2026-10-08.

## What was wrong

D-338's long forks reach the ground, but each one starts at half the main bolt's width, so the strands look thin. In the tutorial's finished shot the strands are nearly as thick as the main bolt.

## What changed

Lightning Bolt's **Forks** setting gains a third choice, **Long, full width** (saved as `full`). Its strands are exactly Long's: the same paths and lengths. The difference is that each one starts as wide as the main bolt where it leaves it, and thins by Decay as Long's do. Short and Long are unchanged, and so are files saved with them. For Breaking, whose forks already start at the main bolt's width, the new choice draws the same as Long.

## Checks

`tests/b219_lightning_full_forks.rs` holds the app to `Fixtures/lightning_full_forks/expected_lightning_full_forks.json`, written by `tools/lightning_full_forks_reference.py` before the code (89032b6). Results are in `verification/D-339_lightning_full_forks_table.md`: **37 of 37 pass**.

| Check | Result |
| --- | --- |
| FX-LFULL-001: the down bolt at width 2, full width: the same seven strands, each starting as wide as the main bolt | matches |
| FX-LFULL-002: the same with Long: half as wide | matches |
| FX-LFULL-003: over the ground, Alpha Obstacle 50: the strands stop at the ground, and the ground below is untouched | matches |
| FX-LFULL-004: the starting bolt's settings: the main bolt unchanged | matches |
| FX-LFULL-005: Breaking: full width draws exactly what Long draws | matches |
| FX-LFULL-006: the word "Full" (capital) is refused with a warning, and the file still opens | matches |
| Undo, tiles, and the window's commands | all pass |

D-338's own checks (B-218) still pass unchanged, with its refusal sentence now naming all three words. So do Lightning's older checks (B-126, B-203, B-211, B-215) and the app's 92 tests.

## Pictures (`verification/D-339 pictures/`)

These use tutorial 2's settings, as D-338's pictures do.

| | Long (D-338) | Long, full width |
| --- | --- | --- |
| Frame 7 | ![Long, frame 7](D-339%20pictures/f7_long.png) | ![Full width, frame 7](D-339%20pictures/f7_full.png) |
| Frame 0 | ![Long, frame 0](D-339%20pictures/f0_long.png) | ![Full width, frame 0](D-339%20pictures/f0_full.png) |

**What to look for:** the strands are in the same places in both columns. On the right they are as thick as the main bolt where they leave it. On frame 7, the left strand goes from a thin line to a full-width bolt. The white area in the sky grows from 1094 to 1457 pixels on frame 7, and from 940 to 1062 on frame 0.

## How to try it in the app

Set up the bolt as for D-338: end point in the ground, Alpha Obstacle 50, Decay 0, Branches about 40. In Effect Controls, switch **Forks** between Long and **Long, full width**. The strands should stay where they are and get thicker.
