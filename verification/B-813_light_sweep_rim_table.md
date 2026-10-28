# B-813: Light Sweep's rim

D-933, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/light_sweep/expected_light_sweep_rim.json`, written by `tools/light_sweep_rim_reference.py` before this code existed and printed in document 25 as FX-SWEEP-027 to 036. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-SWEEP-001 to 026 are B-134's table, run again unchanged.

## FX-SWEEP-027 to 036 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SWEEP-027 frame 0: Edge look rim, the edges alone as FX-SWEEP-009 has them (width 10000, sweep intensity 0, edge intensity 100, thickness 1, direction -30): the block's border is lit all round, softly, the side facing up and left most. | largest difference 2.3e-7 | yes |
| FX-SWEEP-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-028 frame 0: As FX-SWEEP-027, width 0: no band, and the rim is lit just the same. | largest difference 2.3e-7 | yes |
| FX-SWEEP-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-029 frame 0: As FX-SWEEP-027, direction 0, thickness 3: the rim three pixels deep, fading inward; the top border brighter than the foot. | largest difference 2.5e-7 | yes |
| FX-SWEEP-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-030 frame 0: As FX-SWEEP-029, direction 180: the foot brighter than the top, the mirror of FX-SWEEP-029's. | largest difference 2.5e-7 | yes |
| FX-SWEEP-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-031 frame 0: The tutorial's settings: smooth, width 100, sweep intensity 0, edge intensity 60, thickness 3, cutout, white, direction -30: only a soft rim of white light is left, faint in the middle of a block too small to have one clear. | largest difference 3.1e-8 | yes |
| FX-SWEEP-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-032 frame 0: Edge look band, written: FX-SWEEP-009 exactly. | largest difference 2.4e-7 | yes |
| FX-SWEEP-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-033 frame 0: Edge look rim with edge intensity 0: the band alone, FX-SWEEP-004 exactly. | largest difference 2.2e-7 | yes |
| FX-SWEEP-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-034 frame 0: As FX-SWEEP-031, the direction keyed from -30 at frame 0 to 330 at frame 4, a turn: frames 0 and 4 the same, frame 2 lit most at the foot. | largest difference 3.1e-8 | yes |
| FX-SWEEP-034 frame 2: As FX-SWEEP-031, the direction keyed from -30 at frame 0 to 330 at frame 4, a turn: frames 0 and 4 the same, frame 2 lit most at the foot. | largest difference 3.1e-8 | yes |
| FX-SWEEP-034 frame 4: As FX-SWEEP-031, the direction keyed from -30 at frame 0 to 330 at frame 4, a turn: frames 0 and 4 the same, frame 2 lit most at the foot. | largest difference 3.1e-8 | yes |
| FX-SWEEP-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-035 frame 0: As FX-SWEEP-031, the layer moved three pixels right: the rim moves with it. | largest difference 3.1e-8 | yes |
| FX-SWEEP-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SWEEP-036 frame 0: Edge look "glow", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-036 frame 4: Edge look "glow", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SWEEP-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A wrong word is refused (what it warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sweep_036.json: the warning names the edge look | Light Sweep's edge look is "band" or "rim", and this is "glow". | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sweep_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sweep_031.json is saved with its edge look rim | {"center":[50,50],"direction":-30,"edge_intensity":60,"edge_look":"rim","edge_thickness":3,"light_color":"#ffffff","light_reception":"cutout","shape":"smooth","sweep_intensity":0,"width":100} | yes |
| fx_sweep_001.json, from before it, is saved without it | {"center":[50,50],"direction":-30,"edge_intensity":100,"edge_thickness":1,"light_color":"#ffffff","light_reception":"add","shape":"smooth","sweep_intensity":50,"width":50} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sweep_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sweep_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_sweep_035.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

33 of 33 checks pass.
