# D-730: Turbulent Noise, six more blending modes

B-600: `core.turbulent_noise` takes Saturation, Color, Luminosity, None, Stencil Alpha and Stencil Luma. Saturation, Color and Luminosity follow the W3C Compositing and Blending Level 1 non-separable rules (Lum = 0.3 R + 0.59 G + 0.11 B, SetLum with ClipColor, SetSat): this is our basis, not a match to After Effects, whose own rule is unpublished. Turbulent Noise's colour is a grey, so Saturation and Color both give the layer the noise's grey at the layer's own lightness, and Luminosity gives the layer's colour the noise's lightness; each mixes by Opacity as the other modes do. Adobe's help says Fractal Noise's modes are the Timeline's and None draws the noise only, so None replaces the layer with the noise at Opacity (D-413), and the stencils keep the layer where the noise's alpha (Stencil Alpha) or lightness (Stencil Luma) is, as the Timeline's stencils do (D-301). Silhouette Alpha and Silhouette Luma are refused: the program has no Timeline rule for them to follow. Hue (FX-TURBNOISE-060) and Overlay (FX-TURBNOISE-032) were refused here; D-731, by the owner's decision of 2026-10-13, regenerated both and builds them. Every expected pixel is `Fixtures/turbulent_noise/expected_turbulent_blends.json`, written by `tools/turbulent_blends_reference.py` before this code existed and printed in document 25 as FX-TURBNOISE-061 to 074. Tolerance 2e-5.

## FX-TURBNOISE-061 to 074 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURBNOISE-061 frame 0: Size 4, blending mode saturation: the noise is grey, so the card takes its saturation, none, and goes to its own grey. | largest difference 1.2e-7 | yes |
| FX-TURBNOISE-061: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-062 frame 0: Size 4, blending mode color: the card's lightness in the noise's colour, grey, so the same as FX-TURBNOISE-061. | largest difference 1.2e-7 | yes |
| FX-TURBNOISE-062: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-063 frame 0: Size 4, blending mode luminosity: the card's colours at the clouds' lightness. | largest difference 1.0e-7 | yes |
| FX-TURBNOISE-063: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-064 frame 0: Size 4, blending mode luminosity, opacity 50: halfway from the card to FX-TURBNOISE-063. | largest difference 1.7e-7 | yes |
| FX-TURBNOISE-064: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-065 frame 0: Size 4, blending mode none: the clouds alone, the card gone, filling the clear corner too. | largest difference 2.2e-8 | yes |
| FX-TURBNOISE-065: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-066 frame 0: Size 4, blending mode none, opacity 60: the clouds alone at 60% everywhere. | largest difference 2.4e-8 | yes |
| FX-TURBNOISE-066: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-067 frame 0: Size 4, blending mode stencil_alpha: the noise is opaque, so the card is kept whole, the plain card. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-067: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-068 frame 0: Size 4, blending mode stencil_alpha, opacity 40: the card kept at 40%. | largest difference 8.1e-8 | yes |
| FX-TURBNOISE-068: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-069 frame 0: Size 4, blending mode stencil_luma: the card kept where the clouds are light and cut away where they are dark. | largest difference 1.3e-7 | yes |
| FX-TURBNOISE-069: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-070 frame 0: Size 4, blending mode stencil_luma, opacity 50: the clouds at half, so the card is kept less. | largest difference 1.0e-7 | yes |
| FX-TURBNOISE-070: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-071 frame 0: Size 4, blending mode none, opacity 0: nothing drawn and the card gone, a clear frame, not the card left as it was. | largest difference 0.0e0 | yes |
| FX-TURBNOISE-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-072 frame 0: Size 4, blending mode stencil_luma, opacity keyed from 100 at frame 0 to 0 at frame 4, linear: FX-TURBNOISE-069 fading to a clear frame. | largest difference 1.3e-7 | yes |
| FX-TURBNOISE-072 frame 2: Size 4, blending mode stencil_luma, opacity keyed from 100 at frame 0 to 0 at frame 4, linear: FX-TURBNOISE-069 fading to a clear frame. | largest difference 1.0e-7 | yes |
| FX-TURBNOISE-072 frame 4: Size 4, blending mode stencil_luma, opacity keyed from 100 at frame 0 to 0 at frame 4, linear: FX-TURBNOISE-069 fading to a clear frame. | largest difference 0.0e0 | yes |
| FX-TURBNOISE-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURBNOISE-073 frame 0: Blending mode "silhouette_alpha", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-073 frame 4: Blending mode "silhouette_alpha", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-073: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURBNOISE-074 frame 0: Blending mode "silhouette_luma", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-074 frame 4: Blending mode "silhouette_luma", which is not built. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURBNOISE-074: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

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
| FX-TURBNOISE-032 frame 0: Size 4, blend overlay: the card's light parts lightened and its dark parts darkened by the clouds, the black and white kept (the Timeline's Overlay, D-301, on the linear colour). | largest difference 3.8e-7 | yes |
| FX-TURBNOISE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
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
| FX-TURBNOISE-060 frame 0: Size 4, blending mode hue, opacity 60: the noise is grey, so the card is drained towards its own grey at its own lightness, 60 per cent of the way (the W3C's Hue, our basis). | largest difference 1.4e-7 | yes |
| FX-TURBNOISE-060: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
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
| fx_turbnoise_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turbnoise_061.json is saved with blending mode saturation | "saturation" | yes |
| fx_turbnoise_062.json is saved with blending mode color | "color" | yes |
| fx_turbnoise_063.json is saved with blending mode luminosity | "luminosity" | yes |
| fx_turbnoise_065.json is saved with blending mode none | "none" | yes |
| fx_turbnoise_067.json is saved with blending mode stencil_alpha | "stencil_alpha" | yes |
| fx_turbnoise_069.json is saved with blending mode stencil_luma | "stencil_luma" | yes |
| fx_turbnoise_073.json is refused in a sentence naming Turbulent Noise, blending mode and "silhouette_alpha" | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", "none", "stencil_alpha", "stencil_luma", "saturation", "color", "luminosity", "overlay", "hue", and this is "silhouette_alpha". | yes |
| fx_turbnoise_074.json is refused in a sentence naming Turbulent Noise, blending mode and "silhouette_luma" | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", "none", "stencil_alpha", "stencil_luma", "saturation", "color", "luminosity", "overlay", "hue", and this is "silhouette_luma". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| blending mode "silhouette_alpha" is refused with a sentence, and nothing changes | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", "none", "stencil_alpha", "stencil_luma", "saturation", "color", "luminosity", "overlay", "hue", and this is "silhouette_alpha". | yes |
| blending mode "silhouette_luma" is refused with a sentence, and nothing changes | Turbulent Noise's blending mode is one of "normal", "multiply", "screen", "add", "soft_light", "hard_light", "darken", "lighten", "difference", "none", "stencil_alpha", "stencil_luma", "saturation", "color", "luminosity", "overlay", "hue", and this is "silhouette_luma". | yes |
| blending mode luminosity is taken | taken | yes |
| blending mode stencil luma is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_061.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_063.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_066.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_070.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turbnoise_072.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turbnoise_061.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_062.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_063.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_064.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_065.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_066.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_067.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_068.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_069.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_070.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_071.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_072.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_073.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_turbnoise_074.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Noise Saturation: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-730 | 1932027 pixels changed, 2061296 differ from as added | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 0, Full | largest difference 1 of 255, 3275 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 100, Full | largest difference 1 of 255, 1045 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 239, Full | largest difference 1 of 255, 1585 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 0, Draft | largest difference 1 of 255, 161 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 100, Draft | largest difference 1 of 255, 88 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Saturation on three layers, frame 239, Draft | largest difference 1 of 255, 97 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-730 | 2066822 pixels changed, 2071431 differ from as added | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 0, Full | largest difference 1 of 255, 1043 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 100, Full | largest difference 1 of 255, 1018 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 239, Full | largest difference 1 of 255, 992 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Luminosity at opacity 70 on three layers, frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-730 | 2073600 pixels changed, 2073600 differ from as added | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 0, Full | largest difference 1 of 255, 1248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 100, Full | largest difference 1 of 255, 1313 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 239, Full | largest difference 1 of 255, 1197 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 0, Draft | largest difference 1 of 255, 83 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise None at opacity 60 on three layers, frame 239, Draft | largest difference 1 of 255, 82 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200: the processor's frame 100 differs from the shot without it and from Turbulent Noise as added, so the comparisons below test D-730 | 2031279 pixels changed, 2073589 differ from as added | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 0, Full | largest difference 1 of 255, 1427 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 100, Full | largest difference 1 of 255, 1500 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 239, Full | largest difference 1 of 255, 1336 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 0, Draft | largest difference 1 of 255, 332 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 100, Draft | largest difference 1 of 255, 282 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Noise Stencil Luma, contrast 200 on three layers, frame 239, Draft | largest difference 1 of 255, 324 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-730 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_normal.png, Size 40, blending mode Normal: the clouds as before D-730; draws cleanly | [] | yes |
| 4_none.png, None: the clouds alone, the street gone; the street is solid everywhere, so this is the same picture as 1_normal.png (Normal at full opacity also covers it); draws cleanly | [], the same as 1_normal.png: true | yes |
| 2_saturation.png, Saturation: the street's colours drained to grey, its light and dark kept; unlike every picture before; draws cleanly | [], its own: true | yes |
| 3_luminosity.png, Luminosity: the street's colours kept, lit and darkened by the clouds; unlike every picture before; draws cleanly | [], its own: true | yes |
| 5_none_60.png, None at opacity 60: the clouds alone, see-through, the street gone; unlike every picture before; draws cleanly | [], its own: true | yes |
| 6_stencil_luma.png, Stencil Luma: the street shows only where the clouds are light, clear where they are dark; unlike every picture before; draws cleanly | [], its own: true | yes |
| 7_stencil_luma_50.png, Stencil Luma at opacity 50: the street fainter, holes where the clouds are dark; unlike every picture before; draws cleanly | [], its own: true | yes |

## Result

309 of 309 checks pass.
