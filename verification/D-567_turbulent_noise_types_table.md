# D-567: Turbulent Noise's full type lists

B-446: Turbulent Noise (D-445) and Fractal Noise (D-128, D-299), one engine, take After Effects' whole Fractal Type list (Basic, Turbulent Basic, Turbulent Smooth, Turbulent Sharp, Dynamic, Dynamic Progressive, Dynamic Twist, Max, Smeary, Swirly, Rocky, Cloudy, Terrain, Subscale, Small Bumps, Strings, Threads) and Noise Type list (Block, Linear, Soft Linear, Spline, with this program's Smooth). Adobe publishes no formulas, so each type past the four already built is this program's own rule after its name (document 21). Every expected pixel is `Fixtures/turbulent_noise_types/expected_turbulent_noise_types.json`, written by `tools/turbulent_noise_types_reference.py` before this code existed and printed in document 25 as FX-TURBTYPE-001 to 025; Turbulent Noise's FX-TURBNOISE-001 to 032 (029 and 030 renamed under D-566) and Fractal Noise's FX-FRACTAL cases rerun unchanged. Tolerance 2e-5.

## FX-TURBTYPE-001 to 025 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURBTYPE-001 frame 0: Size 4, fractal type turbulent_smooth: rounded creases, softer than turbulent's. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-002 frame 0: Size 4, fractal type turbulent_sharp: thin, sharp creases. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-003 frame 0: Size 4, fractal type dynamic: each octave pushed by the clouds so far, the clouds drawn out. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-004 frame 0: Size 4, fractal type dynamic_progressive: each octave pushed further than the last. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-005 frame 0: Size 4, fractal type dynamic_twist: each octave pushed in a direction the clouds turn, like smoke. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-006 frame 0: Size 4, fractal type max: each octave only ever lightening the clouds, light clouds. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-007 frame 0: Size 4, fractal type smeary: the finer octaves streaked across. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-008 frame 0: Size 4, fractal type swirly: bands that follow the clouds, like agate. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-009 frame 0: Size 4, fractal type rocky: flat patches in five greys, like camouflage. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-010 frame 0: Size 4, fractal type cloudy: soft light puffs on dark. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-011 frame 0: Size 4, fractal type terrain: light ridges. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-012 frame 0: Size 4, fractal type subscale: the fine detail only where the coarser clouds are light. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-013 frame 0: Size 4, fractal type small_bumps: small bumps, starting two octaves finer. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-014 frame 0: Size 4, fractal type strings: thin light lines where the clouds cross the middle grey. | largest difference 4.1e-8 | yes |
| FX-TURBTYPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-015 frame 0: Size 4, fractal type threads: thin threads at every scale. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-016 frame 0: Size 4, noise type linear: straight lines between the points, a harder look than smooth. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-017 frame 0: Size 4, noise type soft_linear: softer at the points than linear. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-018 frame 0: Size 4, noise type spline: a smooth wave through every point. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-019 frame 0: Size 4, dynamic_twist on spline, inverted, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: the smoke moves as it evolves. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-019 frame 2: Size 4, dynamic_twist on spline, inverted, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: the smoke moves as it evolves. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-019 frame 4: Size 4, dynamic_twist on spline, inverted, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: the smoke moves as it evolves. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-020 frame 0: Size 4, rocky on block, contrast 200: square flat patches. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-021 frame 0: Size 4, terrain on linear at complexity 20, the most. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-022 frame 0: Size 4, swirly at complexity 1.9, which counts as 1: sin(3 pi v) of the one octave. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-023 frame 0: FX-TURBTYPE-006 (max) moved three pixels right: worked in the drawing's own space, so it moves with it. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-023 frame 3: FX-TURBTYPE-006 (max) moved three pixels right: worked in the drawing's own space, so it moves with it. | largest difference 3.0e-8 | yes |
| FX-TURBTYPE-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBTYPE-024 frame 0: Fractal type "Turbulent Basic", the name shown and not the word written. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBTYPE-024 frame 4: Fractal type "Turbulent Basic", the name shown and not the word written. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBTYPE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBTYPE-025 frame 0: Noise type "soft linear", with a space and not the underscore. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBTYPE-025 frame 4: Noise type "soft linear", with a space and not the underscore. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBTYPE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Turbulent Noise's and Fractal Noise's own fixtures, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURBNOISE-001 frame 0: The settings as they start: basic, smooth, not inverted, contrast 100, brightness 0, size 100 at scale 100 by 100, offset 0, complexity 6, evolution 0, seed 0, opacity 100, normal. Grey clouds so large that the card sees a small part of one: close greys near the middle. Every pixel keeps its covering, the empty ones stay empty, and with nothing keyed frame 4 is frame 0: Turbulent Noise has no speed of its own. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-001 frame 4: The settings as they start: basic, smooth, not inverted, contrast 100, brightness 0, size 100 at scale 100 by 100, offset 0, complexity 6, evolution 0, seed 0, opacity 100, normal. Grey clouds so large that the card sees a small part of one: close greys near the middle. Every pixel keeps its covering, the empty ones stay empty, and with nothing keyed frame 4 is frame 0: Turbulent Noise has no speed of its own. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-002 frame 0: Size 4: four pixels a cloud, so the greys vary across the card. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-003 frame 0: Size 4, fractal type turbulent: each octave's distance from the middle, so the clouds crease into dark veins. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-004 frame 0: Size 4, noise type block: each cell of each octave one grey, square steps. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-005 frame 0: Size 4, turbulent and block together. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-006 frame 0: Size 4, invert on: FX-TURBNOISE-002 turned over about the middle grey, light where it was dark. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-007 frame 0: Size 4, complexity 1.9, which counts as 1: one octave, the smooth value noise itself. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-008 frame 0: Size 4, complexity 20, the most: twenty octaves. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-009 frame 0: Size 4, contrast 300: FX-TURBNOISE-002's greys three times as far from the middle, held at black and white. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-010 frame 0: Size 4, brightness 30: FX-TURBNOISE-002 lifted by 0.3, held at white. | largest difference 3.2e-8 | yes |
| FX-TURBNOISE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-011 frame 0: Brightness -100: held at 0 everywhere, every shown pixel black. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-012 frame 0: Size 4, evolution 360: one full turn moves the field one cell through its third direction, clouds of their own. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-013 frame 0: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-TURBNOISE-002 and frame 2, at 360, is FX-TURBNOISE-012. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-013 frame 2: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-TURBNOISE-002 and frame 2, at 360, is FX-TURBNOISE-012. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-013 frame 4: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear: frame 0 is FX-TURBNOISE-002 and frame 2, at 360, is FX-TURBNOISE-012. | largest difference 3.1e-8 | yes |
| FX-TURBNOISE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-014 frame 0: Size 4, seed 7: clouds of their own. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-015 frame 0: Size 4, seed 7.9, which counts as 7: FX-TURBNOISE-014. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-016 frame 0: Size 4, scale width 200 and scale height 50: clouds twice as wide and half as tall. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-017 frame 0: Size 4, offset 2.5 right and 3 up: the clouds slide with it. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-018 frame 0: Size 4, blend multiply: the card darkened by the clouds; the black patch stays black. | largest difference 6.4e-8 | yes |
| FX-TURBNOISE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-019 frame 0: Size 4, blend screen, opacity 50: the card lightened, none darkened. | largest difference 2.1e-7 | yes |
| FX-TURBNOISE-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-020 frame 0: Size 4, blend add, opacity 50: added at half strength, not held, so the white patch goes past its covering. | largest difference 2.2e-7 | yes |
| FX-TURBNOISE-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-021 frame 0: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-TURBNOISE-002. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-021 frame 2: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-TURBNOISE-002. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-021 frame 4: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-TURBNOISE-002. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-022 frame 0: FX-TURBNOISE-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 2.2e-8 | yes |
| FX-TURBNOISE-022 frame 3: FX-TURBNOISE-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 2.2e-8 | yes |
| FX-TURBNOISE-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-023 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-023 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-024 frame 0: Complexity 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-024 frame 4: Complexity 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-025 frame 0: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-025 frame 4: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-026 frame 0: Brightness keyed to -1200 at frame 4, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-026 frame 4: Brightness keyed to -1200 at frame 4, below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-027 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-027 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-028 frame 0: Scale width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-028 frame 4: Scale width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-029 frame 0: Fractal type "plasma", which is not one of the seventeen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-029 frame 4: Fractal type "plasma", which is not one of the seventeen. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-030 frame 0: Noise type "cubic", which is not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-030 frame 4: Noise type "cubic", which is not one of the five. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-031 frame 0: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-031 frame 4: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-032 frame 0: Size 4, blend overlay: the card's light parts lightened and its dark parts darkened by the clouds, the black and white kept (the Timeline's Overlay, D-301, on the linear colour). | largest difference 3.8e-7 | yes |
| FX-TURBNOISE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-001 frame 0: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001 frame 4: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-002 frame 0: Size 4: a cell of 2.7 pixels (D-756), so the greys vary across the card, lighter and darker patches. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-003 frame 0: Size 4, complexity 1.9, which counts as 1: one octave, the smooth value noise itself, softer than FX-FRACTAL-002's four. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-004 frame 0: Size 4, complexity 8: eight octaves, finer detail on top of FX-FRACTAL-002's. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-005 frame 0: Size 4, contrast 0: every shown pixel is the middle of the ramp, encoded exactly 0.5, at its own covering. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-006 frame 0: Size 4, contrast 300: FX-FRACTAL-002's greys three times as far from the middle, held at black and white. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-007 frame 0: Size 4, brightness 30: FX-FRACTAL-002's value lifted by 0.3, held at white. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-008 frame 0: Brightness -100: the value is held at 0 everywhere, so every shown pixel is black at its own covering. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-009 frame 0: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009 frame 2: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009 frame 4: Size 4, speed 90 degrees a frame: the clouds change from frame to frame; frame 0 is FX-FRACTAL-002, frame 2 is evolution 180, and frame 4 is evolution 360, FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-010 frame 0: Size 4, evolution 360: one full turn moves the field one cell through its third direction, clouds of their own, not FX-FRACTAL-002's. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-011 frame 0: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011 frame 2: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011 frame 4: Size 4, evolution keyed from 0 at frame 0 to 720 at frame 4, linear, speed 0: frame 0 is FX-FRACTAL-002 and frame 2, at 360, is FX-FRACTAL-010. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-012 frame 0: Size 4, seed 7: clouds of their own. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-013 frame 0: Size 4, seed 7.9, which counts as 7: FX-FRACTAL-012. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-014 frame 0: Size 4, dark #1e1a24 and light #fff0b0: the clouds run from the dark violet to the cream, mixed in encoded values. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-015 frame 0: FX-FRACTAL-014 with its colours written in capitals: the same. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-016 frame 0: Size 4, blend multiply: the card darkened by the clouds, its own colours showing through; the black patch stays black. | largest difference 8.9e-8 | yes |
| FX-FRACTAL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-017 frame 0: Size 4, blend screen, opacity 50: the card lightened by the clouds, none darkened; the white patch stays white. | largest difference 2.0e-7 | yes |
| FX-FRACTAL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-018 frame 0: Size 4, blend add, opacity 50: the clouds added at half strength, not held, so the white patch goes past its covering. | largest difference 2.5e-7 | yes |
| FX-FRACTAL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-019 frame 0: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-019 frame 2: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019 frame 4: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-020 frame 0: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 2.6e-8 | yes |
| FX-FRACTAL-020 frame 3: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 2.6e-8 | yes |
| FX-FRACTAL-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-021 frame 0: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-021 frame 4: Size 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-023 frame 0: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-023 frame 4: Contrast 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-025 frame 0: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-025 frame 4: Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-026 frame 0: Speed keyed to 400 at frame 4, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-026 frame 4: Speed keyed to 400 at frame 4, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-027 frame 0: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-027 frame 4: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FRACTAL-028 frame 0: A dark colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-028 frame 4: A dark colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbtype_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbtype_001.json is saved with fractal type "turbulent_smooth" | "turbulent_smooth" | yes |
| fx_turbtype_002.json is saved with fractal type "turbulent_sharp" | "turbulent_sharp" | yes |
| fx_turbtype_003.json is saved with fractal type "dynamic" | "dynamic" | yes |
| fx_turbtype_004.json is saved with fractal type "dynamic_progressive" | "dynamic_progressive" | yes |
| fx_turbtype_005.json is saved with fractal type "dynamic_twist" | "dynamic_twist" | yes |
| fx_turbtype_006.json is saved with fractal type "max" | "max" | yes |
| fx_turbtype_007.json is saved with fractal type "smeary" | "smeary" | yes |
| fx_turbtype_008.json is saved with fractal type "swirly" | "swirly" | yes |
| fx_turbtype_009.json is saved with fractal type "rocky" | "rocky" | yes |
| fx_turbtype_010.json is saved with fractal type "cloudy" | "cloudy" | yes |
| fx_turbtype_011.json is saved with fractal type "terrain" | "terrain" | yes |
| fx_turbtype_012.json is saved with fractal type "subscale" | "subscale" | yes |
| fx_turbtype_013.json is saved with fractal type "small_bumps" | "small_bumps" | yes |
| fx_turbtype_014.json is saved with fractal type "strings" | "strings" | yes |
| fx_turbtype_015.json is saved with fractal type "threads" | "threads" | yes |
| fx_turbtype_016.json is saved with noise type "linear" | "linear" | yes |
| fx_turbtype_017.json is saved with noise type "soft_linear" | "soft_linear" | yes |
| fx_turbtype_018.json is saved with noise type "spline" | "spline" | yes |
| fx_turbtype_024.json is refused in a sentence naming Turbulent Noise and its fractal type | Turbulent Noise's fractal type is one of "basic", "turbulent", "turbulent_smooth", "turbulent_sharp", "dynamic", "dynamic_progressive", "dynamic_twist", "max", "smeary", "swirly", "rocky", "cloudy", "terrain", "subscale", "small_bumps", "strings", "threads", and this is "Turbulent Basic". | yes |
| fx_turbtype_025.json is refused in a sentence naming Turbulent Noise and its noise type | Turbulent Noise's noise type is one of "smooth", "block", "linear", "soft_linear", "spline", and this is "soft linear". | yes |
| the refusal lists all seventeen fractal types by the word written | Turbulent Noise's fractal type is one of "basic", "turbulent", "turbulent_smooth", "turbulent_sharp", "dynamic", "dynamic_progressive", "dynamic_twist", "max", "smeary", "swirly", "rocky", "cloudy", "terrain", "subscale", "small_bumps", "strings", "threads", and this is "Turbulent Basic". | yes |
| the refusal lists all five noise types by the word written | Turbulent Noise's noise type is one of "smooth", "block", "linear", "soft_linear", "spline", and this is "soft linear". | yes |
| fx_turbtype_019.json as Fractal Noise (dynamic_twist on spline, cycle 2) is taken and saved with its words | {"blend":"normal","brightness":0,"complexity":6,"contrast":100,"cycle":2,"dark_color":"#000000","evolution":0,"fractal_type":"dynamic_twist","invert":"on","light_color":"#ffffff","noise_type":"spline","offset":[0,0],"opacity":100,"scale_height":100,"scale_width":100,"seed":0,"size":6,"speed":0}; valid: true | yes |
| fx_turbtype_024.json as Fractal Noise is refused in Fractal Noise's name | Fractal Noise's fractal type is one of "basic", "turbulent", "turbulent_smooth", "turbulent_sharp", "dynamic", "dynamic_progressive", "dynamic_twist", "max", "smeary", "swirly", "rocky", "cloudy", "terrain", "subscale", "small_bumps", "strings", "threads", and this is "Turbulent Basic". | yes |
| a Fractal Noise whose file leaves out both words opens at basic and smooth and saves neither word, as before (D-299) | {"blend":"normal","brightness":0,"complexity":6,"contrast":100,"cycle":0,"dark_color":"#000000","evolution":0,"invert":"off","light_color":"#ffffff","offset":[0,0],"opacity":100,"scale_height":100,"scale_width":100,"seed":0,"size":6,"speed":0}; basic and smooth: true | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| fractal type plasma is refused with a sentence, and nothing changes | Turbulent Noise's fractal type is one of "basic", "turbulent", "turbulent_smooth", "turbulent_sharp", "dynamic", "dynamic_progressive", "dynamic_twist", "max", "smeary", "swirly", "rocky", "cloudy", "terrain", "subscale", "small_bumps", "strings", "threads", and this is "plasma". | yes |
| threads on spline is taken | taken | yes |
| evolution keyed from 0 to 720 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbtype_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbtype_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbtype_019.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbtype_023.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbtype_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbtype_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072987 pixels changed | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 0, Full | largest difference 1 of 255, 1256 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 100, Full | largest difference 1 of 255, 1291 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 239, Full | largest difference 1 of 255, 1242 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 0, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 100, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise dynamic_twist on spline, size 40, complexity 20 on three layers, frame 239, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2065062 pixels changed | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 0, Full | largest difference 1 of 255, 195 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 100, Full | largest difference 1 of 255, 122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 239, Full | largest difference 1 of 255, 178 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise rocky on block, multiply at opacity 60 on three layers, frame 239, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 0, Full | largest difference 1 of 255, 874 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 100, Full | largest difference 1 of 255, 997 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 239, Full | largest difference 1 of 255, 765 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 0, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Fractal Noise terrain on soft_linear, colours, speed 12, cycle 2, screen on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-567 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| t01_basic.png, fractal type basic at size 40, for comparing; draws cleanly | [] | yes |
| t02_turbulent.png, fractal type turbulent at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t03_turbulent_smooth.png, fractal type turbulent_smooth at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t04_turbulent_sharp.png, fractal type turbulent_sharp at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t05_dynamic.png, fractal type dynamic at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t06_dynamic_progressive.png, fractal type dynamic_progressive at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t07_dynamic_twist.png, fractal type dynamic_twist at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t08_max.png, fractal type max at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t09_smeary.png, fractal type smeary at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t10_swirly.png, fractal type swirly at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t11_rocky.png, fractal type rocky at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t12_cloudy.png, fractal type cloudy at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t13_terrain.png, fractal type terrain at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t14_subscale.png, fractal type subscale at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t15_small_bumps.png, fractal type small_bumps at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t16_strings.png, fractal type strings at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| t17_threads.png, fractal type threads at size 40: clouds of their own, unlike every picture before; draws cleanly | [], its own: true | yes |
| n02_block.png, noise type block at size 40, fractal type basic: unlike every picture before; draws cleanly | [], its own: true | yes |
| n03_linear.png, noise type linear at size 40, fractal type basic: unlike every picture before; draws cleanly | [], its own: true | yes |
| n04_soft_linear.png, noise type soft_linear at size 40, fractal type basic: unlike every picture before; draws cleanly | [], its own: true | yes |
| n05_spline.png, noise type spline at size 40, fractal type basic: unlike every picture before; draws cleanly | [], its own: true | yes |
| Turbulent Noise dynamic_twist on spline against Fractal Noise with the same settings: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Turbulent Noise rocky on block, inverted, seed 9 against Fractal Noise with the same settings: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Turbulent Noise strings on linear, evolution 200, screen at 50 against Fractal Noise with the same settings: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| c1_cycle_2_at_90.png, Fractal Noise dynamic_progressive on spline with cycle 2: evolution 810 draws the same street, 450 does not | 810 the same: true, 450 the same: false | yes |

## Result

329 of 329 checks pass.
