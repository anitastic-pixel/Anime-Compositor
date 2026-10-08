# D-336 / B-220: CC Vector Blur

From P-26's tutorial 2 (Advanced Electric), 2026-10-08, at the owner's word of 2026-10-08 ("then implement CC Vector Blur").

## What it does

CC Vector Blur smears each pixel along a direction read from a map: a layer's brightness (or one of its channels, its alpha, hue or saturation), the layer itself when none is named. Where the map slopes, the picture is smeared; where it is flat, it is left alone.

| Setting | What it does |
| --- | --- |
| Type | **Natural**: along the slope, longer where it is steeper, fading towards the ends. **Constant Length**: along the slope, always Amount long, evenly. **Perpendicular**: Natural turned a quarter, so round the slope instead of down it. **Direction Center**: the way turns with the map's height, the same length everywhere, both ways. **Direction Fading**: the same, forwards only, fading. |
| Amount | how far the smear reaches, 0 to 500 pixels |
| Angle Offset | turns the smear away from the slope |
| Ridge Smoothness | for Natural and Perpendicular, how much gentle slopes are shortened; for the Direction types, how many turns the way makes from the map's black to its white |
| Vector Map, Placement | which layer is the map, and how it is fitted |
| Property | what is read from the map |
| Map Softness | how much the map is blurred before its slopes are read |

The exact smearing is not published by CycoreFX. This is **this program's own reading** of their manual, written down as a rule in document 21 and as fixtures before the code; it is not a copy of After Effects' result. The starting settings (Natural, Amount 10, Ridge Smoothness 10, its own lightness, Map Softness 30) are this program's choice.

## Checks

`tests/b220_vector_blur.rs` holds the app to `Fixtures/vector_blur/expected_vector_blur.json`, written by `tools/vector_blur_reference.py` before the code (8994d21). Results are in `verification/D-336_vector_blur_table.md`: **111 of 111 pass**.

| Check | Result |
| --- | --- |
| FX-VBLUR-001 to 018: each type, Angle Offset, Ridge Smoothness, Map Softness, a ramp, a disc and a black solid as maps, alpha, hue and saturation, a keyed Amount, a moved map, a missing map layer (warned of, and the layer used instead) | all match, largest difference 1.0e-5 (saturation, 015) against the tolerance of 2e-5; every other case is 3.8e-7 or less |
| FX-VBLUR-019 to 027: a wrong type word, amounts past 0 to 500, ridge past 100, a negative keyed softness, a wrong property, a wrong placement, a number for a layer, a too-large angle | the file still opens; the effect is kept as written, left out of every frame, and warned of |
| The window's commands, undo, and tiles of 1 and 64 pixels | all pass |

CC Glass (B-198), Compound Blur (B-127) and Colorama (B-197), which read maps the same way, still pass, and so do the app's 92 tests.

## Pictures (`verification/D-336 pictures/`)

| | |
| --- | --- |
| ![Before](D-336%20pictures/before.png) the street, before | ![As added](D-336%20pictures/as_added.png) as added: smeared along its own lightness |
| ![Natural over the disc](D-336%20pictures/natural_disc.png) Natural, a soft disc as the map: smeared out from the middle | ![Perpendicular over the disc](D-336%20pictures/perpendicular_disc.png) Perpendicular, the same disc: smeared round the middle |
| ![Direction Center](D-336%20pictures/twist_disc.png) Direction Center: the way turns once from the edge to the middle | ![Tutorial setting](D-336%20pictures/tutorial.png) tutorial 2's setting, Natural, Amount 4 |

The disc map is `verification/D-336 pictures/disc.png`, white in the middle, black past 110 pixels out. **What to look for:** with Natural, the windows streak outwards like spokes; with Perpendicular, in rings; outside the disc, where the map is flat black, nothing moves (0 pixels changed past 125 from the middle, in both).

## How to try it in the app

Effects, Blur & Sharpen, **CC Vector Blur**. With no map named it smears the layer along its own lightness. Put a soft white-on-black layer below, switch it off, and choose it as **Vector Map**; change **Type** between Natural and Perpendicular to see spokes become rings.

## Tutorial 2

The tutorial adds CC Vector Blur at Amount 3 to 4 at 25:38 and switches it off again by 27:16, so its finished shot does not use it. The P-26 replay leaves it out.
