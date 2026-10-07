# D-334 / B-215: Lightning Bolt's soft core

From P-26's tutorial 2 (Advanced Electric), 2026-10-07.

## What was wrong

After Effects' Advanced Lightning draws the bolt's core white in the middle, fading into the glow at its edge. Our Lightning Bolt drew the core as one flat ribbon of colour with a hard edge, so tutorial 2's bolt looked like a flat band rather than a line of light.

## What changed

Lightning Bolt has a new setting, **Core Edge**:

- **Hard** (the default): the core as before. Files saved before this open, draw and save exactly as before.
- **Soft**: the core is brightest in the middle of the bolt and fades to nothing at its edge. The glow is unchanged.

How quickly it fades is this program's own rule (brightness falls in a straight line from the middle to the edge). After Effects does not publish its own.

## Checks

`tests/b215_lightning_core.rs` holds the app to `Fixtures/lightning_core/expected_lightning_core.json`, written by `tools/lightning_core_reference.py` before the code. Results in `verification/D-334_lightning_core_table.md`: **35 of 35 pass**.

| Check | Result |
| --- | --- |
| FX-LCORE-001: a straight soft line 6 pixels wide: brightest on its middle row (0.917 of white), then 0.667, 0.333, 0.042, then nothing | matches, largest difference 2.5e-8 |
| FX-LCORE-002: the same line set to Hard: exactly the old core | matches |
| FX-LCORE-003: the starting bolt set to Soft: same shape and glow, the core dimmer toward its edge, never brighter | matches |
| FX-LCORE-004 and 005: a 1-pixel soft core is half lit; a soft core of no width leaves only the glow | match |
| FX-LCORE-006 and 007: the words "Soft" (capital) and "fuzzy" are refused with a warning, and the file still opens | match |
| A file from before D-334 is saved without the new word | yes |
| Undo, tiles, and the window's commands | all pass |

## Pictures (`verification/D-334 pictures/`)

The same bolt, width 4, on a night sky.

| Hard (as before) | Soft |
| --- | --- |
| ![Hard core](D-334%20pictures/1_hard.png) | ![Soft core](D-334%20pictures/2_soft.png) |

**What to look for:** on the left the bolt is a flat white ribbon with a crisp edge (1,171 pure white pixels). On the right it is white only along its middle and melts into the blue glow at its edge (no pure white pixels). The soft one is never brighter than the hard one anywhere.

## How to try it in the app

Add a Lightning Bolt, set Width to about 4, and switch **Core Edge** from Hard to Soft in Effect Controls. The bolt's edge should soften while its shape and glow stay the same.
