# B-95: bulge

D-152, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the nineteenth of the third batch. Every expected pixel is `Fixtures/bulge/expected_bulge.json`, written by `tools/bulge_reference.py` before this code existed and printed in document 25 as FX-BULGE-001 to 023. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BULGE-001 to 023 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BULGE-001 frame 0: The settings as they start: centre 50, 50, radius 50, height 1. The circle is far larger than the drawing, so every pixel is pulled toward the middle, by about half at the middle and a third at the corners: the whole drawing is magnified about the middle, the blue band spreading into rows 3 and 6, and the empty top and bottom rows and right-hand column filled from inside. | largest difference 2.5e-7 | yes |
| FX-BULGE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-002 frame 0: Height 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BULGE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-003 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BULGE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-004 frame 0: Radius 6, height 1: only the pixels whose centres lie within 6 pixels of the middle change, the band and the stripes swelling there; every pixel farther out, the drawing's two ends among them, is kept exactly. | largest difference 2.5e-7 | yes |
| FX-BULGE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-005 frame 0: Radius 6, height -1: a pinch. Inside the circle every pixel reads from farther out than itself, so the middle shrinks and the stripes there narrow; outside it nothing changes. | largest difference 2.5e-7 | yes |
| FX-BULGE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-006 frame 0: Radius 6, height 4, the most: every pixel within 1.76 pixels of the middle, twelve of them, reads the middle point itself, the corner of four band pixels, so they are all the band's blue, a flat disc. | largest difference 2.5e-7 | yes |
| FX-BULGE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-007 frame 0: Radius 6, height -4, the least: a hard pinch, the pixel next to the middle reading from two and a half times as far out. | largest difference 3.1e-7 | yes |
| FX-BULGE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-008 frame 0: Centre 25, 50, radius 6, height 1: the swell moves to (4, 5), and every pixel 6 or more from there, every column from 10 on among them, is kept exactly. | largest difference 2.5e-7 | yes |
| FX-BULGE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-009 frame 0: Centre 53.125, 55, radius 6, height 1: the middle is the centre of pixel (8, 5) itself, 0 from it, so that pixel reads itself and is kept exactly; the pixels round it swell. | largest difference 2.5e-7 | yes |
| FX-BULGE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-010 frame 0: Centre -100, 50, radius 6: the circle lies wholly left of the drawing, so nothing is in reach: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BULGE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-011 frame 0: Radius 6, height keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-BULGE-004, and frame 4 is height 2. | largest difference 1.9e-7 | yes |
| FX-BULGE-011 frame 2: Radius 6, height keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-BULGE-004, and frame 4 is height 2. | largest difference 2.5e-7 | yes |
| FX-BULGE-011 frame 4: Radius 6, height keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-BULGE-004, and frame 4 is height 2. | largest difference 2.5e-7 | yes |
| FX-BULGE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-012 frame 0: Height 1, radius keyed from 0 at frame 0 to 12 at frame 4, linear: the swell opens out, frame 0 the drawing, frame 2 FX-BULGE-004 and frame 4 radius 12. | largest difference 1.9e-7 | yes |
| FX-BULGE-012 frame 2: Height 1, radius keyed from 0 at frame 0 to 12 at frame 4, linear: the swell opens out, frame 0 the drawing, frame 2 FX-BULGE-004 and frame 4 radius 12. | largest difference 2.5e-7 | yes |
| FX-BULGE-012 frame 4: Height 1, radius keyed from 0 at frame 0 to 12 at frame 4, linear: the swell opens out, frame 0 the drawing, frame 2 FX-BULGE-004 and frame 4 radius 12. | largest difference 2.5e-7 | yes |
| FX-BULGE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-013 frame 0: Radius 6, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-BULGE-004, frame 2 is centre 37.5, 50, and frame 4 is FX-BULGE-008. | largest difference 2.5e-7 | yes |
| FX-BULGE-013 frame 2: Radius 6, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-BULGE-004, frame 2 is centre 37.5, 50, and frame 4 is FX-BULGE-008. | largest difference 2.5e-7 | yes |
| FX-BULGE-013 frame 4: Radius 6, centre keyed from 50, 50 at frame 0 to 25, 50 at frame 4, linear: frame 0 is FX-BULGE-004, frame 2 is centre 37.5, 50, and frame 4 is FX-BULGE-008. | largest difference 2.5e-7 | yes |
| FX-BULGE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-014 frame 0: Radius 6, height eased from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frame 2 it would pass 4, is held at 4, and is FX-BULGE-006, as frame 4 is; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-BULGE-014 frame 2: Radius 6, height eased from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frame 2 it would pass 4, is held at 4, and is FX-BULGE-006, as frame 4 is; frame 0 is the drawing. | largest difference 2.5e-7 | yes |
| FX-BULGE-014 frame 4: Radius 6, height eased from 0 at frame 0 to 4 at frame 4 on a curve that overshoots: at frame 2 it would pass 4, is held at 4, and is FX-BULGE-006, as frame 4 is; frame 0 is the drawing. | largest difference 2.5e-7 | yes |
| FX-BULGE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-015 frame 0: FX-BULGE-004 moved three pixels right: the same, moved; the swell is worked in the drawing's own space and moves with it, nothing grows, and the three columns left of the drawing stay empty. | largest difference 2.5e-7 | yes |
| FX-BULGE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-016 frame 0: Centre 75, 50, radius 5, height 1: the swell sits on (12, 5), near the soft edge, so the soft column is spread outward and the empty column 15 takes part of its covering where the circle reaches it: a warp moves covering, and an empty pixel inside the circle need not stay empty. | largest difference 2.5e-7 | yes |
| FX-BULGE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-017 frame 0: Radius 10000, the most, height 1: the circle is so wide that every pixel is pulled almost exactly half way to the middle, within a hundredth of a pixel: the drawing doubled in size about its middle. | largest difference 2.5e-7 | yes |
| FX-BULGE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-018 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-018 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-019 frame 0: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-019 frame 4: Radius 10001, above 10000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-020 frame 0: Height 4.5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-020 frame 4: Height 4.5, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-021 frame 0: Height -4.5, below -4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-021 frame 4: Height -4.5, below -4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-022 frame 0: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-022 frame 4: Centre 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-023 frame 0: Height keyed to 5 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-023 frame 4: Height keyed to 5 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the radius, 50 to 25, and nothing else | Bulge { center: [50.0, 50.0], radius: 25.0, height: 1.0, vertical_radius: 0.0, taper_radius: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bulge_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `height` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a height that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius -1 is refused with a sentence, and nothing changes | Bulge's radius runs from 0 to 10000, and this is -1. | yes |
| radius 10001 is refused with a sentence, and nothing changes | Bulge's radius runs from 0 to 10000, and this is 10001. | yes |
| height 4.5 is refused with a sentence, and nothing changes | Bulge's height runs from -4 to 4, and this is 4.5. | yes |
| height -4.5 is refused with a sentence, and nothing changes | Bulge's height runs from -4 to 4, and this is -4.5. | yes |
| centre 50, 1001 is refused with a sentence, and nothing changes | Bulge's center runs from -1000 to 1000, and this is 1001. | yes |
| height keyed to 5 is refused with a sentence, and nothing changes | Bulge's height runs from -4 to 4, and this is 5. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| height keyed from 0 to 2 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bulge_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bulge_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bulge_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

88 of 88 checks pass.
