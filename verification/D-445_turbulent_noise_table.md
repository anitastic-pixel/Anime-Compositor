# D-445: Turbulent Noise

B-325: `core.turbulent_noise` takes After Effects' Turbulent Noise controls (Fractal Type, Noise Type, Invert, Contrast, Brightness, the size with Scale Width and Height, Offset Turbulence, Complexity, Evolution, Random Seed, Opacity, Blending Mode), a second name over Fractal Noise's engine (as D-383 and D-394): Fractal Noise with no speed, black to white, never cycling. Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_noise.json`, written by `tools/turbulent_noise_reference.py` before this code existed and printed in document 25 as FX-TURBNOISE-001 to 032; Fractal Noise's FX-FRACTAL-001 to 028 rerun unchanged. Tolerance 2e-5.

## FX-TURBNOISE-001 to 032 (document 25)

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
| FX-TURBNOISE-029 frame 0: Fractal type "dynamic", one of After Effects' that is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-029 frame 4: Fractal type "dynamic", one of After Effects' that is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-030 frame 0: Noise type "spline", one of After Effects' that is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-030 frame 4: Noise type "spline", one of After Effects' that is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-031 frame 0: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-031 frame 4: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-032 frame 0: Blend "overlay", which is not a blend here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-032 frame 4: Blend "overlay", which is not a blend here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Fractal Noise, the engine under the second name: its fixtures, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FRACTAL-001 frame 0: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001 frame 4: The settings as they start: size 100, complexity 4, contrast 100, brightness 0, evolution 0, speed 0, seed 0, black to white, opacity 100, normal. The card is covered by grey clouds so large that the drawing sees a small part of one cell: close greys, near the middle. Every pixel keeps its covering, the empty ones stay empty, and with speed 0 frame 4 is frame 0. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-002 frame 0: Size 4: four pixels a cell, so the greys vary across the card, lighter and darker patches. | largest difference 3.0e-8 | yes |
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
| FX-FRACTAL-016 frame 0: Size 4, blend multiply: the card darkened by the clouds, its own colours showing through; the black patch stays black. | largest difference 5.3e-8 | yes |
| FX-FRACTAL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-017 frame 0: Size 4, blend screen, opacity 50: the card lightened by the clouds, none darkened; the white patch stays white. | largest difference 2.1e-7 | yes |
| FX-FRACTAL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-018 frame 0: Size 4, blend add, opacity 50: the clouds added at half strength, not held, so the white patch goes past its covering. | largest difference 2.3e-7 | yes |
| FX-FRACTAL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-019 frame 0: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 1.9e-7 | yes |
| FX-FRACTAL-019 frame 2: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019 frame 4: Size 4, opacity keyed from 0 at frame 0 to 100 at frame 4, eased past its end (132.5 at frame 2): frame 0 is the drawing; frame 2 is held at 100 and is frame 4, FX-FRACTAL-002. | largest difference 3.0e-8 | yes |
| FX-FRACTAL-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FRACTAL-020 frame 0: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 1.5e-8 | yes |
| FX-FRACTAL-020 frame 3: FX-FRACTAL-002 moved three pixels right: the clouds are worked in the drawing's own space, so they move with it. | largest difference 1.5e-8 | yes |
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
| fx_turbnoise_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_017.json is saved with its fourteen settings and no speed, colours or cycle | {"blend":"normal","brightness":0,"complexity":6,"contrast":100,"evolution":0,"fractal_type":"basic","invert":"off","noise_type":"smooth","offset":[2.5,-3],"opacity":100,"scale_height":100,"scale_width":100,"seed":0,"size":4} | yes |
| fx_turbnoise_023.json is refused in a sentence naming Turbulent Noise and its size | Turbulent Noise's size runs from 1 to 1000, and this is 0. | yes |
| fx_turbnoise_024.json is refused in a sentence naming Turbulent Noise and its complexity | Turbulent Noise's complexity runs from 1 to 20, and this is 21. | yes |
| fx_turbnoise_025.json is refused in a sentence naming Turbulent Noise and its contrast | Turbulent Noise's contrast runs from 0 to 1000, and this is 1001. | yes |
| fx_turbnoise_027.json is refused in a sentence naming Turbulent Noise and its seed | Turbulent Noise's seed runs from 0 to 100000, and this is -1. | yes |
| fx_turbnoise_028.json is refused in a sentence naming Turbulent Noise and its scale width | Turbulent Noise's scale width runs from 1 to 10000, and this is 0. | yes |
| fx_turbnoise_029.json is refused in a sentence naming Turbulent Noise and its fractal type | Turbulent Noise's fractal type is "basic" or "turbulent", and this is "dynamic". | yes |
| fx_turbnoise_030.json is refused in a sentence naming Turbulent Noise and its noise type | Turbulent Noise's noise type is "smooth" or "block", and this is "spline". | yes |
| fx_turbnoise_031.json is refused in a sentence naming Turbulent Noise and its invert | Turbulent Noise's invert is "off" or "on", and this is "yes". | yes |
| fx_turbnoise_032.json is refused in a sentence naming Turbulent Noise and its blending mode | Turbulent Noise's blending mode is "normal", "multiply", "screen" or "add", and this is "overlay". | yes |
| a file with a Turbulent Noise whose contrast is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Turbulent Noise without its evolution is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Turbulent Noise whose offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| complexity 21 is refused with a sentence, and nothing changes | Turbulent Noise's complexity runs from 1 to 20, and this is 21. | yes |
| noise type spline is refused with a sentence, and nothing changes | Turbulent Noise's noise type is "smooth" or "block", and this is "spline". | yes |
| opacity keyed to 150 is refused with a sentence, and nothing changes | Turbulent Noise's opacity runs from 0 to 100, and this is 150. | yes |
| turbulent at size 4 is taken | taken | yes |
| evolution keyed from 0 to 720 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_022.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Noise as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072800 pixels changed | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 0, Full | largest difference 1 of 255, 1251 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 100, Full | largest difference 1 of 255, 1278 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 239, Full | largest difference 1 of 255, 1306 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 100, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise as added on three layers, frame 239, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072704 pixels changed | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 0, Full | largest difference 1 of 255, 1455 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 100, Full | largest difference 1 of 255, 1423 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 239, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 0, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 100, Draft | largest difference 1 of 255, 94 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise turbulent, size 40, complexity 20 on three layers, frame 239, Draft | largest difference 1 of 255, 87 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 0, Full | largest difference 1 of 255, 1248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 100, Full | largest difference 1 of 255, 1228 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 239, Full | largest difference 1 of 255, 1223 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 0, Draft | largest difference 1 of 255, 74 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise block, inverted, multiply at opacity 60 on three layers, frame 239, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-445 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as added: the street covered by large soft grey clouds, every pixel a grey; frame 4 the same, as Turbulent Noise has no speed of its own; draws cleanly | [], all grey: true, frame 4 the same: true | yes |
| 4_turbulent.png, fractal type turbulent at size 30: the clouds creased into dark veins, more very dark pixels than 3_size_30.png's basic clouds; both draw cleanly | [] [], pixels darker than 40: basic 285, turbulent 5168 | yes |
| 5_block.png, noise type block at size 30: square steps, more sharp edges between neighbours than 3_size_30.png; draws cleanly | [], neighbours more than 8 levels apart: smooth 12078, block 34521 | yes |
| 6_inverted.png, invert on at size 30: 3_size_30.png's clouds turned over, light where they were dark; draws cleanly | [], turned over: true | yes |
| 7_multiply_over_the_street.png, multiply at opacity 70, size 60: the street darkened by the clouds, no pixel lighter; draws cleanly | [], none lighter: true | yes |
| Turbulent Noise as added against Fractal Noise with the same settings, speed 0, black to white, cycle 0: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Turbulent Noise turbulent at size 30 against Fractal Noise with the same settings, speed 0, black to white, cycle 0: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Turbulent Noise block, inverted, seed 9, offset 20 by -7 against Fractal Noise with the same settings, speed 0, black to white, cycle 0: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Turbulent Noise scale 250 by 40, evolution 200, screen at 50 against Fractal Noise with the same settings, speed 0, black to white, cycle 0: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |

## Result

265 of 265 checks pass.
