# B-718: Radial Blur's zoom smears both ways about each pixel (D-838)

Seven creators' After Effects frames fit a zoom centred on each pixel and half its amount long 3.8 to 4.6 times better than D-110's, which smeared the whole amount from the pixel back toward the centre. A zoom now samples from 1 - Amount / 400 to 1 + Amount / 400 of the pixel's distance from the centre, on the processor and the card. Spin is unchanged, and Light Rays, Spin & Zoom Blur and Fast Zoom Blur keep the outward zoom.

## Radial Blur's fixtures, at the reference's numbers (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RADIAL-001 frame 0: Spin 30 about the middle: every edge smears round the centre, the further out the longer, and the middle of the block stays solid. | largest difference 2.1e-7 | yes |
| FX-RADIAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-002 frame 0: Zoom 30 about the middle: every edge smears both ways along the line from the centre, inward and outward alike, the further out the longer. | largest difference 2.1e-7 | yes |
| FX-RADIAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-RADIAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-004 frame 0: Spin 30 about the top left corner, centre 0, 0: the smears are arcs about that corner, so the far corner smears most. | largest difference 1.7e-7 | yes |
| FX-RADIAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-005 frame 0: Zoom 30 about centre 25, 50, the point (4, 5): everything smears along the line from it, the block, right of it, onto column 10, and the line, left of it, onto column 1 and off the drawing, so columns 2 and 3 stay empty. | largest difference 2.5e-7 | yes |
| FX-RADIAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-006 frame 0: Spin 100, the most: longer arcs than FX-RADIAL-001. | largest difference 2.5e-7 | yes |
| FX-RADIAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-007 frame 0: Zoom 100, the most: each pixel samples a quarter of its distance from the centre either side of itself. | largest difference 2.5e-7 | yes |
| FX-RADIAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-008 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 1.9e-7 | yes |
| FX-RADIAL-008 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 2.5e-7 | yes |
| FX-RADIAL-008 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, spin, linear: frame 0 untouched, frame 2 spins 20, frame 4 spins 40. | largest difference 2.5e-7 | yes |
| FX-RADIAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-009 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 2.1e-7 | yes |
| FX-RADIAL-009 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 1.9e-7 | yes |
| FX-RADIAL-009 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, spin 30: frame 0 is FX-RADIAL-001, frame 2 turns about 25, 25, frame 4 is FX-RADIAL-004. | largest difference 1.7e-7 | yes |
| FX-RADIAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-010 frame 0: Spin 30, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.1e-7 | yes |
| FX-RADIAL-010 frame 3: Spin 30, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of the drawing's edge. | largest difference 2.1e-7 | yes |
| FX-RADIAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-011 frame 0: Spin 100 about centre -1000, -1000, far off the top left: every pixel's arc is longer than 255 pixels, so each takes the most samples, 256. | largest difference 3.0e-9 | yes |
| FX-RADIAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-012 frame 0: A directional blur, direction 90 and length 4, then spin 30 about centre 0, 0: the directional blur grew the layer two pixels on every side, and the spin still turns about the drawing's own top left corner. | largest difference 1.7e-7 | yes |
| FX-RADIAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RADIAL-013 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-013 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-014 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-014 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-015 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-015 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-016 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-016 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-017 frame 0: Type "twist", which is not a type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-017 frame 4: Type "twist", which is not a type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RADIAL-018 frame 0: Type "Spin": the word is exact, so a capital is not the type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-018 frame 4: Type "Spin": the word is exact, so a capital is not the type. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RADIAL-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## FX-EDGES-008, a zoom with Edges Repeat (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EDGES-008 frame 0: Radial Blur, zoom 40 about the top left corner, edges repeat: every pixel still fully covered. | largest difference 1.3e-7 | yes |
| FX-EDGES-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## The zoom smears both ways now: FX-RADIAL-002 against D-110's outward zoom

| Check | The build's answer | Matches |
| --- | --- | --- |
| Pixels the centred zoom 30 covers that D-110's outward zoom 30 (FX-SPINZOOM-001, the same drawing) leaves empty: the smear reaching inward, toward the centre | 6 pixels | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_radial_002.json, the same bytes as at c4b0696c (before D-838; no project file changed), opens with no warning | [] | yes |
| fx_radial_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_radial_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| and draws the centred zoom (FX-RADIAL-002), not D-110's outward one it drew before | largest difference from FX-RADIAL-002 2.1e-7, from D-110's 0.38 | yes |

## Light Rays, Spin & Zoom Blur and Fast Zoom Blur keep D-110's outward zoom (tolerance 2e-5)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RAYS-001 frame 0: The settings as they start: centre 50, 50, the point (8, 5), length 50, threshold 70, intensity 1, white. Only the yellow, 98 %, is bright enough; its light streaks outward, away from the centre: left across its soft edge into the empty columns 0 to 3, and up and down into the empty rows above and below it, fading as it goes, and it brightens the yellow itself. The brown, 60 %, and the purple give no light, and columns 9 to 15 are untouched; nothing is drawn past the layer's edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-002 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-RAYS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-003 frame 0: Length 0: no streaks, but the light is still added onto itself: each yellow pixel twice as bright, and the half-covering edge, whose light is its own, doubled to full covering. | largest difference 3.1e-7 | yes |
| FX-RAYS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-004 frame 0: Threshold 100: nothing is that bright, so the drawing is untouched. | largest difference 1.5e-7 | yes |
| FX-RAYS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-005 frame 0: Threshold 60: the brown, at exactly 60 %, is lit too, and its light streaks right, over the purple line and on to the drawing's right edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-006 frame 0: Length 100, the most: each pixel gathers the light all the way from the centre out to itself, so the streaks reach further than FX-RAYS-001's; and as the yellow touches the centre, every pixel of the drawing takes some of its light, right of the centre too, faintest at the right edge. | largest difference 3.1e-7 | yes |
| FX-RAYS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-007 frame 0: Intensity 2.5: two and a half times FX-RAYS-001's rays, and where they add past white they are not cut off; the covering stops at full. | largest difference 6.2e-7 | yes |
| FX-RAYS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-008 frame 0: Colour #ff8000, orange: FX-RAYS-001's rays tinted, their red as before, their green a fifth and their blue gone; the covering as FX-RAYS-001's. | largest difference 3.1e-7 | yes |
| FX-RAYS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-009 frame 0: Colour #000000, black: the rays add no colour but still add covering, so they fall as a dark shadow on the empty pixels; the solid yellow is unchanged, and its half-covering edge gains covering but no colour, so it darkens. | largest difference 1.5e-7 | yes |
| FX-RAYS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-010 frame 0: FX-RAYS-008 with its colour written in capitals, #FF8000: the same. | largest difference 3.1e-7 | yes |
| FX-RAYS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-011 frame 0: Centre 0, 0, the top left corner: the streaks run down and right, away from the corner, onto the empty pixels right of the yellow and below it; the corner itself stays empty. | largest difference 3.1e-7 | yes |
| FX-RAYS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-012 frame 0: Centre 50, -100, above the drawing: the streaks run straight down from the yellow to the bottom edge, and nothing lights the rows above it. | largest difference 2.9e-7 | yes |
| FX-RAYS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-013 frame 0: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013 frame 2: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013 frame 4: Length keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-RAYS-003, frame 2 is FX-RAYS-001, frame 4 is FX-RAYS-006. | largest difference 3.1e-7 | yes |
| FX-RAYS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-014 frame 0: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 1.5e-7 | yes |
| FX-RAYS-014 frame 2: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 3.1e-7 | yes |
| FX-RAYS-014 frame 4: Threshold keyed from 100 at frame 0 to 60 at frame 4, linear: frame 0 is the drawing, frame 2, at 80, lights the yellow alone and is FX-RAYS-001, frame 4 is FX-RAYS-005. | largest difference 3.1e-7 | yes |
| FX-RAYS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-015 frame 0: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.1e-7 | yes |
| FX-RAYS-015 frame 2: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.7e-7 | yes |
| FX-RAYS-015 frame 4: Centre keyed from 50, 50 at frame 0 to 0, 0 at frame 4, linear: frame 0 is FX-RAYS-001, frame 2 streams from 25, 25, frame 4 is FX-RAYS-011. | largest difference 3.1e-7 | yes |
| FX-RAYS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-016 frame 0: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-RAYS-016 frame 2: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 2.0e-6 | yes |
| FX-RAYS-016 frame 4: Intensity eased from 0 at frame 0 to 10 at frame 4 on a curve that overshoots: at frame 2 it would pass 10, is held at 10, and is intensity 10 plain, as frame 4 is. | largest difference 2.0e-6 | yes |
| FX-RAYS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-017 frame 0: FX-RAYS-001 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYS-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-RAYS-017 frame 3: FX-RAYS-001 moved three pixels right: the rays are drawn on the drawing before it is moved, so it is FX-RAYS-001 moved, and the three columns left of the drawing stay empty, as the layer does not grow. | largest difference 3.1e-7 | yes |
| FX-RAYS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RAYS-018 frame 0: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-018 frame 4: Length 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-019 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-019 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-020 frame 0: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-020 frame 4: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-021 frame 0: Intensity keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-021 frame 4: Intensity keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-022 frame 0: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-022 frame 4: Centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-023 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-023 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RAYS-024 frame 0: A colour written "orange", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-024 frame 4: A colour written "orange", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-RAYS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-001 frame 0: Straight zoom 30 about the middle: every edge streaks outward, evenly; Radial Blur's zoom 30 as it was before D-838 (D-110's outward zoom), exactly. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-002 frame 0: Fading zoom 30: the same streaks, fading as they run out, so the drawing itself stays stronger. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-003 frame 0: Centered zoom 30: streaks both outward and inward, half as long each way. | largest difference 2.4e-7 | yes |
| FX-SPINZOOM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-004 frame 0: Rotate 30: every edge streaks clockwise only, so the line down the left edge streaks upward past its top and not past its foot. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-005 frame 0: Rotate -30: anticlockwise, the line streaking past its foot instead. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-006 frame 0: Scratch 30: both ways evenly; Radial Blur's spin 30, FX-RADIAL-001, exactly. | largest difference 2.1e-7 | yes |
| FX-SPINZOOM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-007 frame 0: Rotate fading 30: clockwise, fading as it goes. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-008 frame 0: Straight zoom 60, quality 10: a fifth of the points, so the streaks break into separate copies. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-009 frame 0: Rotate 30, quality 100: twice the points, smoother than FX-SPINZOOM-004. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-010 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-011 frame 0: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-011 frame 2: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-011 frame 4: Amount keyed from 0 at frame 0 to 40 at frame 4, rotate, linear, as a wheel spinning up. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-012 frame 0: Rotate 30 about the top left corner, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of it. | largest difference 1.8e-7 | yes |
| FX-SPINZOOM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-013 frame 0: Straight zoom -30: the points run outward from the pixel, so the streaks run inward, toward the centre. | largest difference 2.5e-7 | yes |
| FX-SPINZOOM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-014 frame 0: A directional blur, direction 90 and length 4, then rotate fading 30 about 0, 0: the grown layer is read, the centre still the drawing's corner. | largest difference 1.7e-7 | yes |
| FX-SPINZOOM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-015 frame 0: Rotate 360 about -1000, -1000: every path is longer than 255 pixels, so each takes the most points, 256. | largest difference 1.5e-9 | yes |
| FX-SPINZOOM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPINZOOM-016 frame 0: Amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-016 frame 4: Amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-017 frame 0: Amount -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-017 frame 4: Amount -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-018 frame 0: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-018 frame 4: Quality 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-019 frame 0: Quality 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-019 frame 4: Quality 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-020 frame 0: Type "spin", which is Radial Blur's word, not one of these. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-020 frame 4: Type "spin", which is Radial Blur's word, not one of these. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-021 frame 0: Type "Rotate": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-021 frame 4: Type "Rotate": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPINZOOM-022 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-022 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPINZOOM-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-001 frame 0: Standard, amount 50 (the default), about the middle: every edge streaks outward, fading as it runs. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-002 frame 0: Brightest: only light streaks; the skin block streaks outward over the clear pixels round it, the dark line, with only clear pixels behind it, is kept, and nothing grows darker anywhere. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-003 frame 0: Darkest: only dark streaks; the line down the left edge, with clear pixels behind it, fades to a quarter, the skin block, whose streaks run back over itself, is kept, and nothing grows lighter. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-004 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-005 frame 0: Standard, amount 100, the most: each pixel reads all the way back to the centre. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-006 frame 0: Brightest about centre 25, 50, the point (4, 5): the block streaks right, away from it. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-007 frame 0: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-007 frame 2: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-007 frame 4: Amount keyed from 0 at frame 0 to 80 at frame 4, brightest, linear. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-008 frame 0: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008 frame 2: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008 frame 4: Centre keyed from 0, 50 at frame 0 to 100, 50 at frame 4, brightest amount 80, as a light passing along a line of text. | largest difference 2.5e-7 | yes |
| FX-FASTZOOM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-009 frame 0: Standard, moved three pixels right: the centre moves with the drawing, and nothing is drawn left of it. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FASTZOOM-010 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-010 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-011 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-011 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-012 frame 0: Zoom "bright", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-012 frame 4: Zoom "bright", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-013 frame 0: Zoom "Standard": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-013 frame 4: Zoom "Standard": the word is exact. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FASTZOOM-014 frame 0: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-014 frame 4: Centre 1001, 50, past ten widths. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FASTZOOM-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_radial_002.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |
| fx_radial_005.json, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_radial_007.json, frame 0 at Full and Draft | largest difference 1 of 255; refused by the card: false; the same warnings: true | yes |
| fx_edges_008.json, frame 0 at Full and Draft | largest difference 0 of 255; refused by the card: false; the same warnings: true | yes |

## Result

212 of 212 checks pass.
