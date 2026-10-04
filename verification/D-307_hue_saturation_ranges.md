# D-307 / B-192: Hue/Saturation's colour ranges

Found by P-26, tutorial 3 (the colourful glitch). In After Effects, Hue/Saturation's Channel Control lets you shift only the reds, or only the blues, and so on. Here the effect could only change every colour at once.

## What changed

Under Hue, Saturation and Lightness, Hue/Saturation now has six rows: **Reds, Yellows, Greens, Cyans, Blues, Magentas**. Each row has three boxes:

- **H**: hue, -180 to 180
- **S**: saturation, -100 to 100
- **L**: lightness, -100 to 100

Each row is added on top of the master three, but only for colours in that range. A colour within 15 degrees of the range's centre gets the full amount; from there it fades to nothing at 45 degrees. So orange, halfway between red and yellow, gets half of each. Greys have no hue and are left alone. Every box can be keyed.

The coloured strip at the top of the effect now shows what the ranges do, as it already did for the master.

With every range at 0 the effect draws exactly as before. Its 23 fixtures (FX-HUESAT-001 to 023) still pass. Older files read and save the same: a range is written only once it is changed.

Not built: After Effects' movable range sliders (dragging where "Reds" starts and stops) and its Colorize option. Each range here has fixed edges, 60 degrees apart.

The graphics card draws only the master three. When a range is moved, the CPU draws it, so the preview and the export match.

## Checks (cargo test)

`tests/b192_hue_saturation_ranges.rs`, 2 of 2 pass:

| Check | Expected | Got |
|---|---|---|
| Red with Reds hue +120 | turns green, exactly as the master hue +120 turns it | so |
| Orange (hue 30) with Reds hue +120 | turns half as far, as the master hue +60 | so |
| Yellow, grey and blue with Reds hue +120 | unchanged | so |
| Red with Reds saturation -60 and lightness +20, master saturation -20 | the same as the master at saturation -80, lightness +20 | so |
| Yellow with Yellows hue -60 (and Reds +90) | turns pure red; the Reds row does not touch it | so |
| Saving: an old file / one range set / a range with two numbers | nothing new written / only that range kept / refused with a message | so |

Unchanged and still passing: FX-HUESAT-001 to 023 (`tests/b56_hue_saturation.rs`), the effect-cost table, the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

Seven strips, red, orange, yellow, green, cyan, blue and magenta, each with its own Hue/Saturation.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-307 pictures/1_as_added_nothing_changes.png` | as it starts | the seven strips as drawn; the panel lists Reds to Magentas with H, S and L boxes | pass |
| `D-307 pictures/2_reds_hue_plus_120.png` | Reds H 120 | red is green, orange is yellow-green, the other five unchanged; the panel's lower strip shows only the red end shifted | pass |
| `D-307 pictures/3_blues_saturation_minus_100.png` | Blues S -100 | only the blue strip is grey | pass |

## For the owner to try

1. Put Hue/Saturation on a colourful picture.
2. Set **Reds** H to 120: only the red parts turn green.
3. Set it back to 0, then set **Blues** S to -100: only the blue parts go grey.
