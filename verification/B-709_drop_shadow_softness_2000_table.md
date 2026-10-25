# B-709: Drop Shadow's softness up to 2000

D-829. After Effects' Drop Shadow panel takes Softness 2000 (Smertimba, https://www.youtube.com/watch?v=gsihkt5sO2w, 2:03); ours stopped at 500 and refused more. The rule is unchanged, sigma softness / 5 (D-774). The fixtures' numbers are `tools/drop_shadow_reference.py`'s, worked before this code; every FX-SHADOW case is also run by B-58's table.

## FX-SHADOW-020 and 031 to 033 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHADOW-031 frame 0: Softness 600, above the old top of 500, Shadow Only "on", opacity 100, distance 0: blurred at sigma 120, the card's shadow is spread so wide that every pixel of the frame takes a faint, almost even shadow, darkest where the card was. | largest difference 3.7e-11 | yes |
| FX-SHADOW-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-032 frame 0: Softness 1000, as FX-SHADOW-031: sigma 200, so about (120 / 200)^2, 0.36, of its covering at every pixel. | largest difference 1.2e-11 | yes |
| FX-SHADOW-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-033 frame 0: Softness 2000, the new top, as FX-SHADOW-031: sigma 400, about a quarter of FX-SHADOW-032's covering at every pixel. On this small frame it is near the tolerance, so B-709 also holds the build to it within a thousandth of each pixel's own value. | largest difference 4.3e-12 | yes |
| FX-SHADOW-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-020 frame 0: Softness 2001, above 2000 (501, above 500, before D-829). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020 frame 4: Softness 2001, above 2000 (501, above 500, before D-829). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-031: every pixel's covering within a thousandth of its own value | largest share off 1.6e-7 | yes |
| FX-SHADOW-032: every pixel's covering within a thousandth of its own value | largest share off 1.5e-7 | yes |
| FX-SHADOW-033: every pixel's covering within a thousandth of its own value | largest share off 2.1e-7 | yes |

## The panel

| Check | The build's answer | Matches |
| --- | --- | --- |
| the Effect Controls hint reads 0 to 2000 | core.drop_shadow's Softness row | yes |

## What it costs: a 960 by 200 title in a 1920 by 1080 frame, distance 20

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 100: the card draws it, within 1 level of the CPU | 1 levels at most, 2054 pixels differ;  | yes |
| softness 100 (sigma 20): time to draw the frame, median of 3 | CPU 11 ms, card 10 ms | yes |
| softness 500: the card draws it, within 1 level of the CPU | 1 levels at most, 4411 pixels differ;  | yes |
| softness 500 (sigma 100): time to draw the frame, median of 3 | CPU 31 ms, card 28 ms | yes |
| softness 1000: the card draws it, within 1 level of the CPU | 1 levels at most, 5910 pixels differ;  | yes |
| softness 1000 (sigma 200): time to draw the frame, median of 3 | CPU 49 ms, card 58 ms | yes |
| softness 2000: the card draws it, within 1 level of the CPU | 1 levels at most, 5224 pixels differ;  | yes |
| softness 2000 (sigma 400): time to draw the frame, median of 3 | CPU 106 ms, card 139 ms | yes |

## Result

21 of 21 checks pass.
