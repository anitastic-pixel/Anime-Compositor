# B-812: Lens Blur's Diffraction Fringe and Highlight Saturation

D-932, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/lens_blur/expected_lens_blur_parity.json`, written by `tools/lens_blur_parity_reference.py` before this code existed and printed in document 25 as FX-LENS-070 to 085. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-LENS-001 to 069 are B-59's, B-64's and B-238's tables, run again unchanged.

## FX-LENS-070 to 085 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LENS-070 frame 0: Radius 4 with Diffraction Fringe and Highlight Saturation written at 0: FX-LENS-007 exactly. | largest difference 8.3e-8 | yes |
| FX-LENS-070: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-071 frame 0: Radius 4, Diffraction Fringe 50: the rim of the iris, the steps more than three pixels out, counts three times, so the red pixel's patch is a ring, brighter at its edge than in its middle. | largest difference 6.3e-8 | yes |
| FX-LENS-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-072 frame 0: Iris hexagon, radius 4, Diffraction Fringe 100: the hexagon's rim counts five times. | largest difference 6.1e-8 | yes |
| FX-LENS-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-073 frame 0: Radius 4, highlight gain 3 at threshold 80, Highlight Saturation 100: the lit skin twice as far from its grey before the average, so its haze is pinker than FX-LENS-029's. | largest difference 9.3e-8 | yes |
| FX-LENS-073: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-074 frame 0: Radius 4, highlight gain 0.5 at threshold 50, Highlight Saturation 50: the red is lit and made redder, its green and blue held at 0. | largest difference 7.6e-8 | yes |
| FX-LENS-074: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-075 frame 0: Radius 4, Highlight Saturation 100 with gain 0: nothing is lit, so nothing is saturated: FX-LENS-007. | largest difference 8.3e-8 | yes |
| FX-LENS-075: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-076 frame 0: Radius 4, Diffraction Fringe keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LENS-007, frame 2 the rim three times, frame 4 five. | largest difference 8.3e-8 | yes |
| FX-LENS-076 frame 2: Radius 4, Diffraction Fringe keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LENS-007, frame 2 the rim three times, frame 4 five. | largest difference 6.3e-8 | yes |
| FX-LENS-076 frame 4: Radius 4, Diffraction Fringe keyed from 0 at frame 0 to 100 at frame 4: frame 0 is FX-LENS-007, frame 2 the rim three times, frame 4 five. | largest difference 6.3e-8 | yes |
| FX-LENS-076: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-077 frame 0: Radius 0.9, Diffraction Fringe 100: the pixel alone has no rim, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-LENS-077: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-078 frame 0: Radius 2.5, edges repeat, Diffraction Fringe 100, on the picture that fills the layer: every pixel stays fully covered. | largest difference 1.3e-7 | yes |
| FX-LENS-078: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-079 frame 0: The ramp as the blur map, Focal Distance 0, Diffraction Fringe 50: each level's iris has its own rim, and the white right edge is FX-LENS-071's. | largest difference 1.5e-7 | yes |
| FX-LENS-079: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-080 frame 0: A white solid read by its Alpha as the blur map, Diffraction Fringe 50: every pixel at the full radius, FX-LENS-071. | largest difference 6.3e-8 | yes |
| FX-LENS-080: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-LENS-081 frame 0: Diffraction Fringe 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-081 frame 4: Diffraction Fringe 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-081: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-082 frame 0: Diffraction Fringe -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-082 frame 4: Diffraction Fringe -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-082: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-083 frame 0: Highlight Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-083 frame 4: Highlight Saturation 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-083: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-084 frame 0: Highlight Saturation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-084 frame 4: Highlight Saturation -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-084: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LENS-085 frame 0: Highlight Saturation keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-085 frame 4: Highlight Saturation keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-LENS-085: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Out-of-range settings are refused (what each warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_081.json: the warning names diffraction fringe | Lens Blur's diffraction fringe runs from 0 to 100, and this is 101. | yes |
| fx_lens_082.json: the warning names diffraction fringe | Lens Blur's diffraction fringe runs from 0 to 100, and this is -1. | yes |
| fx_lens_083.json: the warning names highlight saturation | Lens Blur's highlight saturation runs from 0 to 100, and this is 101. | yes |
| fx_lens_084.json: the warning names highlight saturation | Lens Blur's highlight saturation runs from 0 to 100, and this is -1. | yes |
| fx_lens_085.json: the warning names highlight saturation | Lens Blur's highlight saturation runs from 0 to 100, and this is 150. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_075.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_076.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_077.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_078.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_079.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_080.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_081.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_082.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_083.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_084.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_085.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_lens_072.json is saved with its fringe at 100 and its saturation at 0, as written | {"aspect":1,"diffraction_fringe":100,"edges":"transparent","highlight_gain":0,"highlight_saturation":0,"highlight_threshold":100,"iris":"hexagon","radius":4,"rotation":0,"roundness":0} | yes |
| fx_lens_007.json, from before them, is saved without them | {"edges":"transparent","radius":4} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_lens_071.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_073.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_076.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_lens_079.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

66 of 66 checks pass.
