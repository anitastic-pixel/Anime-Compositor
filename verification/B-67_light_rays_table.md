# B-67: light rays

D-124, accepted by the owner on 2026-09-26, the second of the second batch of ten. Every expected pixel is `Fixtures/light_rays/expected_light_rays.json`, written by `tools/light_rays_reference.py` before this code existed and printed in document 25 as FX-RAYS-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RAYS-001 to 024 (document 25)

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

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: rays past the layer's edge are cut | 0 | yes |
| a half-size draft preview changes nothing: the length and the centre are shares of the drawing | LightRays { center: [30.0, 70.0], length: 60.0, threshold: 70.0, intensity: 1.0, color: "#ffffff" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rays_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rays_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `center` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| length 101 is refused with a sentence, and nothing changes | Light Rays's length runs from 0 to 100, and this is 101. | yes |
| threshold -1 is refused with a sentence, and nothing changes | Light Rays's threshold runs from 0 to 100, and this is -1. | yes |
| intensity 11 is refused with a sentence, and nothing changes | Light Rays's intensity runs from 0 to 10, and this is 11. | yes |
| centre 1001, 50 is refused with a sentence, and nothing changes | Light Rays's center runs from -1000 to 1000, and this is 1001. | yes |
| colour "orange" is refused with a sentence, and nothing changes | Light Rays's colour is written #rrggbb, and this is "orange". | yes |
| intensity keyed to 20 is refused with a sentence, and nothing changes | Light Rays's intensity runs from 0 to 10, and this is 20. | yes |
| length 100, threshold 100, intensity 10 and centre 1000, 1000, the tops, is taken | taken | yes |
| length 0, threshold 0, intensity 0 and centre -1000, -1000, the bottoms, is taken | taken | yes |
| centre keyed from 50, 50 to 0, 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rays_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rays_017.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

92 of 92 checks pass.
