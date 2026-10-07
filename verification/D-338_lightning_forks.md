# D-338 / B-218: Lightning Bolt's long forks

From P-26's tutorial 2 (Advanced Electric), 2026-10-07.

## What was wrong

In the tutorial's finished shot the bolt splits into two or three long strands, each running down to the ground. The author turns Advanced Lightning's Decay down "for them to touch down on the ground". Our Lightning Bolt's forks were always short twigs, a third to three fifths of the piece they leave, so they never reached the ground.

## What changed

Lightning Bolt has a new setting, **Forks**:

- **Short** (the default): the forks as before. Files saved before this open, draw and save exactly as before.
- **Long**: the forks that leave the main bolt in its first three splits run on down beside it, 10 to 30 degrees off its way, until they have covered (1 - Decay / 100) of what is left of the way. At Decay 0 they go all the way; at Decay 50, half way. Alpha Obstacle then stops each strand where it meets the ground, as it already stops the main bolt.

After Effects does not publish how its forks are made, so this rule is this program's own, fitted to what the tutorial shows. Where each strand goes is random, as in After Effects, so our strands will not land in the tutorial's exact places.

## Checks

`tests/b218_lightning_forks.rs` holds the app to `Fixtures/lightning_forks/expected_lightning_forks.json`, written by `tools/lightning_forks_reference.py` before the code (4183d56). Results in `verification/D-338_lightning_forks_table.md`: **45 of 45 pass**.

| Check | Result |
| --- | --- |
| FX-LFORK-001: a bolt straight down the picture, forks Long: seven forks, each reaching the bottom edge | matches, largest difference below 1e-7 |
| FX-LFORK-002: the same set to Short: exactly the old forks | matches |
| FX-LFORK-003: Decay 50: the forks go half the way down and thin to half | matches |
| FX-LFORK-004: over a ground, Alpha Obstacle 50: every strand stops on the ground's top row, the ground below untouched | matches |
| FX-LFORK-005 and 006: Strike, and the starting bolt with Long: the main bolt unchanged, its first forks longer | match |
| FX-LFORK-007 and 008: the words "Long" (capital) and "many" are refused with a warning, and the file still opens | match |
| A file from before D-338 is saved without the new word | yes |
| Undo, tiles, and the window's commands | all pass |

Lightning's older checks (B-126, B-203, B-211, B-215) still pass unchanged, and so do the app's 92 tests.

## Pictures (`verification/D-338 pictures/`)

Tutorial 2's settings: Direction, Decay 0, Branches 37, Alpha Obstacle 50, the bolt's end inside the ground, width 3, on a night sky over brown ground.

| | Short (as before) | Long |
| --- | --- | --- |
| Frame 7 | ![Short, frame 7](D-338%20pictures/f7_short.png) | ![Long, frame 7](D-338%20pictures/f7_long.png) |
| Frame 0 | ![Short, frame 0](D-338%20pictures/f0_short.png) | ![Long, frame 0](D-338%20pictures/f0_long.png) |

**What to look for:** on frame 7, with Short, one fork dangles off to the left and stops in mid-air; with Long, that fork and a second one run down to the ground, and four strands touch it instead of two. Frame 0 has fewer forks by chance; there, Long turns one mid-air twig into a strand that reaches the ground. Nothing ever goes down into the ground.

## How to try it in the app

Add a Lightning Bolt to a layer with ground at the bottom and empty sky above. Set the end point inside the ground, Alpha Obstacle to 50, Decay to 0 and Branches to about 40. Switch **Forks** from Short to Long in Effect Controls and play: some frames should show two or three strands reaching the ground. Raise Decay and they should get shorter.
