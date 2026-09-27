# B-66: distance gradation

D-123, accepted by the owner on 2026-09-26, the first of the second batch of ten. Every expected pixel is `Fixtures/distance_gradation/expected_distance_gradation.json`, written by `tools/distance_gradation_reference.py` before this code existed and printed in document 25 as FX-DISTGRAD-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DISTGRAD-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DISTGRAD-001 frame 0: The settings as they start: #6450a0, width 10, opacity 50, invert off, multiply. Every pixel that shows is darkened toward violet, the most along the block's edge, the dot and the quarter-covered column, and less at each pixel further in, and the least at (7, 5), sqrt 10 from the hole, the furthest from any out pixel; the empty pixels stay empty. | largest difference 1.2e-7 | yes |
| FX-DISTGRAD-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-002 frame 0: Width 2, opacity 100, normal: the violet at full strength on the quarter-covered column, fading in over two pixels; column 0, on the drawing's left edge, is three quarters violet, as the top and bottom rows are; the pixels round the hole are shaded; and every pixel 2.5 or more from the nearest out pixel is left as it is: (7, 3) to (8, 6) in the shadow, and (2, 6) and (6, 6). | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-003 frame 0: Width 1, opacity 100, normal: a pixel beside an out pixel is half violet, one out on a diagonal only is shaded by 1 - (sqrt 2 - 0.5), about 0.086, and every pixel two or more from the edge is untouched. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-004 frame 0: Width 0.5, opacity 100, normal: only the pixels that are themselves out and show, the quarter-covered column, are shaded, wholly violet at their own covering; every pixel that is in is untouched. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-005 frame 0: Width 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-006 frame 0: Width 0 with invert on: still the drawing, untouched; width 0 comes first. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-007 frame 0: Opacity 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-008 frame 0: Width 2, opacity 100, normal, invert on: FX-DISTGRAD-002 turned round, the quarter-covered column untouched, (7, 3) to (8, 6) wholly violet, and each pixel shaded by one less FX-DISTGRAD-002's strength. | largest difference 1.4e-7 | yes |
| FX-DISTGRAD-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-009 frame 0: Invert on, the rest as they start: the middle darkened the most and the edge the least; the quarter-covered column is untouched. | largest difference 2.0e-7 | yes |
| FX-DISTGRAD-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-010 frame 0: Blend normal, opacity 50: the violet laid over, at half the strength at most. | largest difference 1.3e-7 | yes |
| FX-DISTGRAD-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-011 frame 0: Blend screen: every pixel that shows is lightened toward violet and none darkened; no channel passes the pixel's covering. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-012 frame 0: Blend add: the violet added, past what a screen gives, the most at the edge. | largest difference 2.2e-7 | yes |
| FX-DISTGRAD-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-013 frame 0: Colour #ffffff, multiply: multiplying by white changes nothing, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-014 frame 0: FX-DISTGRAD-001 with the colour written in capitals, #6450A0: the same. | largest difference 1.2e-7 | yes |
| FX-DISTGRAD-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-015 frame 0: Width keyed from 0 at frame 0 to 4 at frame 4, linear, opacity 100, normal: frame 0 is the drawing, frame 2 is width 2, FX-DISTGRAD-002, and frame 4 width 4; the shading reaches further in as the width grows. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-015 frame 2: Width keyed from 0 at frame 0 to 4 at frame 4, linear, opacity 100, normal: frame 0 is the drawing, frame 2 is width 2, FX-DISTGRAD-002, and frame 4 width 4; the shading reaches further in as the width grows. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-015 frame 4: Width keyed from 0 at frame 0 to 4 at frame 4, linear, opacity 100, normal: frame 0 is the drawing, frame 2 is width 2, FX-DISTGRAD-002, and frame 4 width 4; the shading reaches further in as the width grows. | largest difference 1.4e-7 | yes |
| FX-DISTGRAD-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-016 frame 0: Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, width 2, normal: at frame 2 it would pass 100, is held at 100, and is FX-DISTGRAD-002; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-016 frame 2: Opacity eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots, width 2, normal: at frame 2 it would pass 100, is held at 100, and is FX-DISTGRAD-002; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-017 frame 0: FX-DISTGRAD-001 moved three pixels right: the same, moved; the shading is worked on the drawing before it moves, so the drawing's left edge stays an edge. | largest difference 1.2e-7 | yes |
| FX-DISTGRAD-017 frame 3: FX-DISTGRAD-001 moved three pixels right: the same, moved; the shading is worked on the drawing before it moves, so the drawing's left edge stays an edge. | largest difference 1.2e-7 | yes |
| FX-DISTGRAD-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DISTGRAD-018 frame 0: Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-018 frame 4: Width 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-019 frame 0: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-019 frame 4: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-020 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-020 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-021 frame 0: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-021 frame 4: Opacity keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-022 frame 0: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-022 frame 4: Invert "yes", which is not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-023 frame 0: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-023 frame 4: Blend "overlay", which is not a blend. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DISTGRAD-024 frame 0: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-024 frame 4: Colour "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DISTGRAD-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the shading stays on the covering | 0 | yes |
| a half-size draft preview halves the width, a distance, and leaves the opacity | DistanceGradation { color: "#6450a0", width: 6.0, opacity: 50.0, invert: "off", blend: "multiply" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_distgrad_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_distgrad_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `width` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with invert written as true rather than a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 1001 is refused with a sentence, and nothing changes | Distance Gradation's width runs from 0 to 1000, and this is 1001. | yes |
| width -1 is refused with a sentence, and nothing changes | Distance Gradation's width runs from 0 to 1000, and this is -1. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Distance Gradation's opacity runs from 0 to 100, and this is 101. | yes |
| invert "yes" is refused with a sentence, and nothing changes | Distance Gradation's invert is "off" or "on", and this is "yes". | yes |
| blend "overlay" is refused with a sentence, and nothing changes | Distance Gradation's blend is "normal", "multiply", "screen" or "add", and this is "overlay". | yes |
| colour "#12345" is refused with a sentence, and nothing changes | Distance Gradation's colour is written #rrggbb, and this is "#12345". | yes |
| opacity keyed to 150 is refused with a sentence, and nothing changes | Distance Gradation's opacity runs from 0 to 100, and this is 150. | yes |
| width 1000 and opacity 100, the tops, is taken | taken | yes |
| width 0, opacity 0, invert on and screen is taken | taken | yes |
| width keyed from 0 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_distgrad_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_distgrad_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

88 of 88 checks pass.
