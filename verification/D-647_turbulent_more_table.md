# D-647: Turbulent Noise, Overflow, Transform and Sub Settings

B-526: `core.turbulent_noise` takes After Effects' Overflow (Allow HDR Results, Clip, Soft Clamp, Wrap Back), Rotation, Uniform Scaling, Perspective Offset, Sub Settings (Sub Influence, Sub Scaling, Sub Rotation, Sub Offset, Center Subscale) and five more blending modes (Soft Light, Hard Light, Darken, Lighten, Difference). With c the drawing's middle, octave o of the clouds is read at c + R(-t_o) (p - w_o Offset - o Sub Offset - c), t_o = Rotation + o Sub Rotation, w_o = 0.5^o with Perspective Offset on, else 1; each octave Sub Influence per cent of the last's strength (cloudy 0.35 scaled by Sub Influence / 50) and 100 / Sub Scaling times as fine, round c with Center Subscale on. Soft Clamp is 0.5 + 0.5 tanh(2 (v - 0.5)); Wrap Back folds at black and white. Every setting as added (Allow HDR Results, 0, off, off, 50, 50, 0, (0, 0), off) draws every file before D-647 as before; After Effects' own Sub Influence 70 and Sub Scaling 56 are not used for that reason. The rule is this program's own; Adobe publishes none. Overlay and the hue, saturation, color, luminosity, none and stencil modes are not built; adding Overlay needs the owner's fixture decision (FX-TURBNOISE-032). Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_more.json`, written by `tools/turbulent_more_reference.py` before this code existed and printed in document 25 as FX-TURBNOISE-033 to 060. Tolerance 2e-5.

## FX-TURBNOISE-033 to 060 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURBNOISE-033 frame 0: Size 4, contrast 300, Overflow clip: in an 8-bit composition the same as FX-TURBNOISE-009, allow_hdr holding to 0..1 there too. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-034 frame 0: Size 4, contrast 300, Overflow soft_clamp: the greys eased toward black and white, none reaching them. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-035 frame 0: Size 4, contrast 300, Overflow wrap_back: the greys past white and black fold back, light bands inside the dark. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-036 frame 0: Size 4, Rotation 90: FX-TURBNOISE-002's clouds turned a quarter clockwise round the card's middle. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-037 frame 0: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037 frame 2: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037 frame 4: Size 4, Rotation keyed from 0 at frame 0 to 90 at frame 4, linear: frame 0 is FX-TURBNOISE-002, frame 4 FX-TURBNOISE-036. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-038 frame 0: Size 4, Scale Width 200, Scale Height 50, Uniform Scaling on: the two scales left out, so it is FX-TURBNOISE-002. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-039 frame 0: Size 4, Offset (6, -4), Perspective Offset on: the coarse clouds slide 6 right and 4 up, each finer octave half as far as the last. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-040 frame 0: Size 4, Perspective Offset on with no offset: nothing slides, so it is FX-TURBNOISE-002 within the tolerance. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-041 frame 0: Size 4, Sub Influence 100: every octave as strong as the first, busier fine detail. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-042 frame 0: Size 4, Sub Influence 0: only the first octave counts, so it is FX-TURBNOISE-007 (complexity 1) within the tolerance. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-043 frame 0: Size 4, Sub Scaling 70: each octave only a little finer than the last, softer clouds. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-044 frame 0: Size 4, Sub Rotation 45: each finer octave turned an eighth more. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-045 frame 0: Size 4, Sub Offset (5, 3): each finer octave slid further. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-046 frame 0: Size 4, Center Subscale on: the octaves grow finer round the card's middle, not its corner. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-047 frame 0: Size 4, dynamic on spline, Rotation 30, Offset (3, 2) with Perspective Offset, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (1, -2), Center Subscale on: everything together. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-048 frame 0: Size 4, cloudy, Sub Influence 80: cloudy's 0.35 a step scaled to 0.56. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-049 frame 0: Size 4, blending mode soft_light: the card's shading lifted and lowered gently by the clouds. | largest difference 3.7e-7 | yes |
| FX-TURBNOISE-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-050 frame 0: Size 4, blending mode hard_light, opacity 70: as soft_light but harder. | largest difference 1.3e-7 | yes |
| FX-TURBNOISE-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-051 frame 0: Size 4, blending mode darken: each channel the darker of the card and the clouds. | largest difference 3.1e-8 | yes |
| FX-TURBNOISE-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-052 frame 0: Size 4, blending mode lighten: each channel the lighter. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-053 frame 0: Size 4, blending mode difference: each channel how far apart; the black patch shows the clouds themselves. | largest difference 2.1e-7 | yes |
| FX-TURBNOISE-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-054 frame 0: FX-TURBNOISE-036 moved three pixels right: turned round the drawing's own middle, so the turned clouds move with it. | largest difference 2.2e-8 | yes |
| FX-TURBNOISE-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-055 frame 0: FX-TURBNOISE-002 with every D-647 setting written as added: FX-TURBNOISE-002 to the bit. | largest difference 3.0e-8 | yes |
| FX-TURBNOISE-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-056 frame 0: Overflow "hdr", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-056 frame 4: Overflow "hdr", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-057 frame 0: Sub Scaling 5, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-057 frame 4: Sub Scaling 5, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-058 frame 0: Sub Influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-058 frame 4: Sub Influence 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-059 frame 0: Uniform Scaling "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-059 frame 4: Uniform Scaling "yes", not off or on. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-060 frame 0: Blending mode "hue", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-060 frame 4: Blending mode "hue", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Turbulent Noise's earlier fixtures, unchanged

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
| FX-TURBNOISE-032 frame 0: Blend "overlay", which is not a blend here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-032 frame 4: Blend "overlay", which is not a blend here. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
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

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_001.json, from before D-647, opens as added and saves none of the nine new settings | {"blend":"normal","brightness":0,"complexity":6,"contrast":100,"evolution":0,"fractal_type":"basic","invert":"off","noise_type":"smooth","offset":[0,0],"opacity":100,"scale_height":100,"scale_width":100,"seed":0,"size":100} | yes |
| fx_turbnoise_055.json, every new setting written as added, keeps all nine when saved | {"blend":"normal","brightness":0,"center_subscale":"off","complexity":6,"contrast":100,"evolution":0,"fractal_type":"basic","invert":"off","noise_type":"smooth","offset":[0,0],"opacity":100,"overflow":"allow_hdr","perspective_offset":"off","rotation":0,"scale_height":100,"scale_width":100,"seed":0,"size":4,"sub_influence":50,"sub_offset":[0,0],"sub_rotation":0,"sub_scaling":50,"uniform_scaling":"off"} | yes |
| fx_turbnoise_047.json is saved with Rotation 30, Perspective Offset on, Sub Influence 70, Sub Scaling 60, Sub Rotation 20, Sub Offset (1, -2), Center Subscale on | {"blend":"normal","brightness":0,"center_subscale":"on","complexity":6,"contrast":100,"evolution":0,"fractal_type":"dynamic","invert":"off","noise_type":"spline","offset":[3,2],"opacity":100,"overflow":"allow_hdr","perspective_offset":"on","rotation":30,"scale_height":100,"scale_width":100,"seed":0,"size":4,"sub_influence":70,"sub_offset":[1,-2],"sub_rotation":20,"sub_scaling":60,"uniform_scaling":"off"} | yes |
| fx_turbnoise_037.json is saved with Rotation's keys kept | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":90}]} | yes |
| fx_turbnoise_056.json is refused in a sentence naming Turbulent Noise, overflow and "hdr" | Turbulent Noise's overflow is one of "allow_hdr", "clip", "soft_clamp", "wrap_back", and this is "hdr". | yes |
| fx_turbnoise_057.json is refused in a sentence naming Turbulent Noise, sub_scaling and 5 | Turbulent Noise's sub scaling runs from 10 to 100, and this is 5. | yes |
| fx_turbnoise_058.json is refused in a sentence naming Turbulent Noise, sub_influence and 101 | Turbulent Noise's sub influence runs from 0 to 100, and this is 101. | yes |
| fx_turbnoise_059.json is refused in a sentence naming Turbulent Noise, uniform scaling and "yes" | Turbulent Noise's uniform scaling is "off" or "on", and this is "yes". | yes |
| fx_turbnoise_060.json is refused in a sentence naming Turbulent Noise, blending mode and "hue" | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", and this is "hue". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| overflow "hdr" is refused with a sentence, and nothing changes | Turbulent Noise's overflow is one of "allow_hdr", "clip", "soft_clamp", "wrap_back", and this is "hdr". | yes |
| sub scaling 5 is refused with a sentence, and nothing changes | Turbulent Noise's sub scaling runs from 10 to 100, and this is 5. | yes |
| blending mode "overlay" is refused with a sentence, and nothing changes | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", and this is "overlay". | yes |
| rotation -45, sub rotation 30, center subscale on, wrap back, difference is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_036.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_037.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_046.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_047.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_054.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_034.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_035.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_049.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_052.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_054.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_055.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_056.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_057.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_058.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_059.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_060.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-647 | 2073137 pixels changed, 2055758 differ from as added | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 0, Full | largest difference 1 of 255, 1225 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 100, Full | largest difference 1 of 255, 1241 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 239, Full | largest difference 1 of 255, 1226 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 0, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Rotation 30, Offset (40, -25) with Perspective Offset on three layers, frame 239, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-647 | 2072802 pixels changed, 2054107 differ from as added | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 0, Full | largest difference 1 of 255, 1241 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 100, Full | largest difference 1 of 255, 1264 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 239, Full | largest difference 1 of 255, 1346 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 0, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 100, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Sub Influence 80, Sub Scaling 65, Sub Rotation keyed 0 to 90, Sub Offset (7, 3), Center Subscale on, contrast 200, Soft Clamp on three layers, frame 239, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-647 | 2064484 pixels changed, 2073133 differ from as added | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 0, Full | largest difference 1 of 255, 1180 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 100, Full | largest difference 1 of 255, 1160 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 239, Full | largest difference 1 of 255, 1099 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 0, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 100, Draft | largest difference 1 of 255, 67 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise contrast 250, Wrap Back, Hard Light at opacity 70 on three layers, frame 239, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-647 | 2072818 pixels changed, 2073376 differ from as added | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 0, Full | largest difference 1 of 255, 1190 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 100, Full | largest difference 1 of 255, 1247 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 239, Full | largest difference 1 of 255, 1116 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 0, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Difference on three layers, frame 239, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-647 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png, Size 40, every setting as added: the clouds as before D-647; draws cleanly | [] | yes |
| 2_rotation_45.png, Rotation 45: the same clouds turned an eighth round the middle; unlike every picture before; draws cleanly | [], its own: true | yes |
| 3_sub_settings.png, Sub Influence 80, Sub Scaling 70, Sub Rotation 45: busier, softer fine detail, each layer turned further; unlike every picture before; draws cleanly | [], its own: true | yes |
| 4_center_subscale.png, Center Subscale on, Sub Scaling 70: the fine detail grows from the middle; unlike every picture before; draws cleanly | [], its own: true | yes |
| 5_wrap_back.png, contrast 300, Wrap Back: greys past white fold back to dark, banded; unlike every picture before; draws cleanly | [], its own: true | yes |
| 6_soft_clamp.png, contrast 300, Soft Clamp: the same greys eased, no flat white or black patches; unlike every picture before; draws cleanly | [], its own: true | yes |
| 7_difference.png, blending mode Difference: the street's colours flipped where the clouds are light; unlike every picture before; draws cleanly | [], its own: true | yes |
| 8_hard_light.png, blending mode Hard Light: the street darkened and lightened hard by the clouds; unlike every picture before; draws cleanly | [], its own: true | yes |

## Result

309 of 309 checks pass.
