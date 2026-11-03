# B-815: Bulge's Antialiasing and Pin All Edges

D-935, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/bulge/expected_bulge_quality.json`, written by `tools/bulge_quality_reference.py` before this code existed and printed in document 25 as FX-BULGE-024 to 034. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance.

## FX-BULGE-024 to 034 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BULGE-024 frame 0: Antialiasing High on FX-BULGE-004 (radius 6, height 1): each pixel the mean of four reads a quarter pixel either way, so the swollen stripes' edges come out softer; every pixel whose four points are all past the circle, the drawing's two ends among them, is kept exactly. | largest difference 2.2e-7 | yes |
| FX-BULGE-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-025 frame 0: Antialiasing High on FX-BULGE-007 (radius 6, height -4), the hard pinch where one read per pixel jumps furthest: the four reads smooth it. | largest difference 2.4e-7 | yes |
| FX-BULGE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-026 frame 0: Antialiasing written Low: FX-BULGE-004 exactly. | largest difference 2.5e-7 | yes |
| FX-BULGE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-027 frame 0: Antialiasing High at height 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BULGE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-028 frame 0: Pin All Edges on FX-BULGE-001 (radius 50, height 1, every pixel in reach): the border pixels, rows 0 and 9 and columns 0 and 15, are kept exactly, and the pull fades in over the 2.5 pixels inside them, so the empty top and bottom rows and the right-hand column stay empty. | largest difference 2.5e-7 | yes |
| FX-BULGE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-029 frame 0: Pin All Edges on FX-BULGE-004 (radius 6): the circle reaches the top and bottom rows, which are kept; rows 1 and 8 swell less; columns 0, 1, 14 and 15, out of reach, are kept as before. | largest difference 2.3e-7 | yes |
| FX-BULGE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-030 frame 0: Pin All Edges and Antialiasing High together on FX-BULGE-001: the border kept, the inside swollen and smoothed. | largest difference 2.4e-7 | yes |
| FX-BULGE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-031 frame 0: FX-BULGE-028 moved three pixels right: the same, moved; the pinned edges are the drawing's own, and the three columns left of it stay empty. | largest difference 2.5e-7 | yes |
| FX-BULGE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-032 frame 0: Pin All Edges written off: FX-BULGE-001 exactly. | largest difference 2.5e-7 | yes |
| FX-BULGE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BULGE-033 frame 0: Antialiasing written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-033 frame 4: Antialiasing written "medium". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BULGE-034 frame 0: Pin All Edges written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-034 frame 4: Pin All Edges written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BULGE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A wrong word is refused (what it warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bulge_033.json: the warning names antialiasing | Bulge's antialiasing is "low" or "high", and this is "medium". | yes |
| fx_bulge_034.json: the warning names pin all edges | Bulge's pin all edges is "off" or "on", and this is "yes". | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bulge_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bulge_030.json is saved with antialiasing high and pin all edges on | {"antialiasing":"high","center":[50,50],"height":1,"pin_all_edges":"on","radius":50} | yes |
| fx_bulge_001.json, from before them, is saved without either word | {"center":[50,50],"height":1,"radius":50} | yes |
| fx_bulge_026.json, which writes Low and Off, keeps them as written | {"antialiasing":"low","center":[50,50],"height":1,"pin_all_edges":"off","radius":6} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bulge_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bulge_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bulge_030.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

36 of 36 checks pass.
