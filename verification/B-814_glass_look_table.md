# B-814: CC Glass's AE Lights and rim look

D-934, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/glass/expected_glass_look.json`, written by `tools/glass_look_reference.py` before this code existed and printed in document 25 as FX-GLASS-028 to 038. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-GLASS-001 to 027 are B-441's table, run again unchanged.

## FX-GLASS-028 to 038 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLASS-028 frame 0: As a CC Glass is now added: Edge Look Rim with the rest as FX-GLASS-001: the two blocks bent outward, their middles their own colour, their soft edges darker all round, a highlight toward the top left. | largest difference 6.1e-6 | yes |
| FX-GLASS-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-029 frame 0: Rim, Softness 4, Displacement 0: the light alone. The blocks keep about their colour, darker than D-561's light makes them, a little darker toward every edge, the bottom and right as well as the top and left, with a faint highlight at the top left. | largest difference 1.8e-7 | yes |
| FX-GLASS-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-030 frame 0: Rim with a point light at 25, 30 per cent, 10 pixels up: the darkening as FX-GLASS-029, the highlight moved toward the light. | largest difference 2.3e-7 | yes |
| FX-GLASS-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-031 frame 0: Rim at Height 0: a flat surface, so every pixel keeps its colour, plus the faintest highlight in its own colour. | largest difference 1.5e-7 | yes |
| FX-GLASS-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-032 frame 0: Rim with Light Intensity 0, Displacement 20: no darkening and no highlight, the blocks bent and their colours untouched. | largest difference 1.9e-6 | yes |
| FX-GLASS-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-033 frame 0: Rim with Ambient 0, Diffuse 100, Specular 0, Softness 4, Displacement 0: each pixel its own colour times Nz, never brighter, darkest where the surface is steepest. | largest difference 1.8e-7 | yes |
| FX-GLASS-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-034 frame 0: Light Type AE Lights, Displacement 20: the blocks bent and not lit, as After Effects shows them under one white ambient light. | largest difference 1.9e-6 | yes |
| FX-GLASS-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-035 frame 0: AE Lights with Displacement 0: nothing bent and nothing lit, the drawing comes back untouched. | largest difference 1.5e-7 | yes |
| FX-GLASS-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-036 frame 0: Rim with the classic light, Displacement 20, Light Intensity 70: the look is not read, so the same as FX-GLASS-003. | largest difference 1.9e-6 | yes |
| FX-GLASS-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-037 frame 0: Edge Look written `lit`: D-561's look, the same as FX-GLASS-001. | largest difference 6.1e-6 | yes |
| FX-GLASS-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLASS-038 frame 0: An edge look written "dark". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-038 frame 4: An edge look written "dark". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLASS-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A wrong word is refused (what it warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_038.json: the warning names the edge look | CC Glass's edge look is "lit" or "rim", and this is "dark". | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glass_028.json is saved with its edge look rim | {"ambient":75,"diffuse":90,"displacement":100,"fit":"stretch","height":50,"layer":"","light_angle":-45,"light_color":"#ffffff","light_height":25,"light_intensity":100,"light_look":"rim","light_position":[50,50],"light_type":"distant","metal":100,"property":"alpha","roughness":0.1,"softness":10,"specular":25} | yes |
| fx_glass_001.json, from before it, is saved without it | {"ambient":75,"diffuse":90,"displacement":100,"fit":"stretch","height":50,"layer":"","light_angle":-45,"light_color":"#ffffff","light_height":25,"light_intensity":100,"light_position":[50,50],"light_type":"distant","metal":100,"property":"alpha","roughness":0.1,"softness":10,"specular":25} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glass_028.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_030.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glass_034.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

33 of 33 checks pass.
