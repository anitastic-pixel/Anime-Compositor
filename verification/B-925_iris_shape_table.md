# B-925: Iris Wipe's shape

D-1045, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/iris_wipe/expected_iris_shape.json`, written by `tools/iris_shape_reference.py` before this code existed and printed in document 25 as FX-IRIS-028 to 045. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-IRIS-028 to 045 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-IRIS-028 frame 0: Iris Points 6, completion 50: a triangle standing on its base, its top point straight up from the middle, closing on (8, 5); inside it is kept and outside it gone. | largest difference 1.9e-7 | yes |
| FX-IRIS-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-029 frame 0: Iris Points 8, completion 50: a diamond, four corners up, right, down and left, closing on the middle. | largest difference 1.9e-7 | yes |
| FX-IRIS-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-030 frame 0: Iris Points 8, Outer Radius 4.3: After Effects' hole, a diamond reaching 4.3 pixels up, down, left and right from the middle, transparent; every pixel outside it is kept exactly. Completion is not read. | largest difference 1.9e-7 | yes |
| FX-IRIS-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-031 frame 0: Iris Points 10, Outer Radius 4.6, Use Inner Radius on, Inner Radius 2.1: a five-pointed star hole, its top point straight up. | largest difference 1.9e-7 | yes |
| FX-IRIS-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-032 frame 0: FX-IRIS-030 turned 45 degrees: the diamond becomes a square standing level, its corners 4.3 from the middle. | largest difference 1.9e-7 | yes |
| FX-IRIS-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-033 frame 0: FX-IRIS-030 with feather 2: the hole's edge soft over 2 units of the shape's own size, each pixel faded as a whole. | largest difference 1.9e-7 | yes |
| FX-IRIS-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-034 frame 0: FX-IRIS-030 with invert on: only the diamond is kept, everything outside it gone, so the two add up to the drawing. | largest difference 1.9e-7 | yes |
| FX-IRIS-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-035 frame 0: Iris Points 0, the circle, Outer Radius 4.3: a round hole of radius 4.3. | largest difference 1.9e-7 | yes |
| FX-IRIS-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-036 frame 0: Iris Points 8, Outer Radius keyed from 0 at frame 0 to 8.6 at frame 4, linear, as the tutorial keys it: frame 0 is the drawing (Outer Radius 0 reads Completion, 0), frame 2 is FX-IRIS-030, and frame 4 a hole twice as big. | largest difference 1.9e-7 | yes |
| FX-IRIS-036 frame 2: Iris Points 8, Outer Radius keyed from 0 at frame 0 to 8.6 at frame 4, linear, as the tutorial keys it: frame 0 is the drawing (Outer Radius 0 reads Completion, 0), frame 2 is FX-IRIS-030, and frame 4 a hole twice as big. | largest difference 1.9e-7 | yes |
| FX-IRIS-036 frame 4: Iris Points 8, Outer Radius keyed from 0 at frame 0 to 8.6 at frame 4, linear, as the tutorial keys it: frame 0 is the drawing (Outer Radius 0 reads Completion, 0), frame 2 is FX-IRIS-030, and frame 4 a hole twice as big. | largest difference 1.9e-7 | yes |
| FX-IRIS-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-037 frame 0: Iris Points 4, below 6: the circle, FX-IRIS-003 exactly. | largest difference 1.9e-7 | yes |
| FX-IRIS-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-038 frame 0: Iris Points 6, completion 50, Use Inner Radius on with Inner Radius 1: with Outer Radius 0 the inner radius is not read, so FX-IRIS-028 exactly. | largest difference 1.9e-7 | yes |
| FX-IRIS-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-039 frame 0: FX-IRIS-029 moved three pixels right: the same, moved, and the three columns left of the drawing stay empty. | largest difference 1.9e-7 | yes |
| FX-IRIS-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-IRIS-040 frame 0: Iris Points 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-040 frame 4: Iris Points 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-041 frame 0: Iris Points -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-041 frame 4: Iris Points -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-042 frame 0: Outer Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-042 frame 4: Outer Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-043 frame 0: Inner Radius 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-043 frame 4: Inner Radius 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-044 frame 0: Use Inner Radius "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-044 frame 4: Use Inner Radius "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-IRIS-045 frame 0: Rotation 100001, past the most. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-045 frame 4: Rotation 100001, past the most. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-IRIS-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A wrong word is refused (what it warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iris_044.json: the warning names use inner radius | Iris Wipe's use inner radius is "off" or "on", and this is "yes". | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| halves the outer and inner radius, and leaves the points and the rotation | IrisWipe { completion: 0.0, center: [50.0, 50.0], feather: 0.0, invert: "off", points: 8.0, outer_radius: 200.0, use_inner: "on", inner_radius: 50.0, rotation: 30.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iris_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_iris_001.json, from before them, is saved without the five settings | {"center":[50,50],"completion":0,"feather":0,"invert":"off"} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| points 33 is refused with a sentence, and nothing changes | Iris Wipe's points runs from 0 to 32, and this is 33. | yes |
| outer radius -1 is refused with a sentence, and nothing changes | Iris Wipe's outer radius runs from 0 to 100000, and this is -1. | yes |
| inner radius 100001 is refused with a sentence, and nothing changes | Iris Wipe's inner radius runs from 0 to 100000, and this is 100001. | yes |
| use inner radius "yes" is refused with a sentence, and nothing changes | Iris Wipe's use inner radius is "off" or "on", and this is "yes". | yes |
| rotation keyed to 100001 is refused with a sentence, and nothing changes | Iris Wipe's rotation runs from -100000 to 100000, and this is 100001. | yes |
| every new number at its bottom, is taken | taken | yes |
| every new number at its top, is taken | taken | yes |
| outer radius keyed from 0 to 1800 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_iris_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iris_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_iris_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

63 of 63 checks pass.
