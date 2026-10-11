# D-532: CC Sphere

B-412, after CycoreFX's CC Sphere (Perspective): the layer wrapped round a sphere as a map round a globe, turned and lit, in its own fake 3D. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/cc_sphere_reference.py`: a fixed eye (a 50 mm lens on a 36 mm frame) looks at the sphere, turned X then Y then Z; the outside is the nearer hit and the inside the farther; CC Cylinder's directional light (Light Height and Direction) gives ambient, diffuse and a Blinn-Phong highlight whose colour Metal mixes between the layer's and the light's. The composition camera and 3D layers are not used. Every expected pixel is `Fixtures/cc_sphere/expected_cc_sphere.json`, written by the tool before this code existed and printed in document 25 as FX-SPH-001 to 026. Tolerance 2e-5.

## FX-SPH-001 to 026 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPH-001 frame 0: The settings as they start: the drawing wrapped round a sphere half the drawing's half width in radius, in the middle, its middle column facing the viewer and its top row the top pole, lit from the upper left; it darkens round the curve, the corners round it are transparent, and the inside's far wall shows only where the drawing is not covered. | largest difference 1.2e-7 | yes |
| FX-SPH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-002 frame 0: Render outside: the outside alone. | largest difference 1.2e-7 | yes |
| FX-SPH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-003 frame 0: Render inside: the inside alone, the far wall, the drawing seen from behind and so mirrored. | largest difference 1.2e-7 | yes |
| FX-SPH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-004 frame 0: Rotation Y 90: the sphere turned a quarter about its upright axis, so the drawing three quarters of the way across faces the viewer. | largest difference 1.2e-7 | yes |
| FX-SPH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-005 frame 0: Rotation X 30: the sphere tipped, its top towards the viewer, so the top rows come round to the front. | largest difference 1.4e-7 | yes |
| FX-SPH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-006 frame 0: Rotation Z 90: the sphere turned on its side, its poles left and right. | largest difference 1.2e-7 | yes |
| FX-SPH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-007 frame 0: Radius 100: a sphere as wide as the drawing, nearer the viewer and so running past the drawing's top and bottom. | largest difference 1.4e-7 | yes |
| FX-SPH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-008 frame 0: Radius 0: no sphere, so nothing is drawn and the layer is transparent. | largest difference 0.0e0 | yes |
| FX-SPH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-009 frame 0: Offset [25, 50]: the sphere a quarter of the way across, seen a little from the side. | largest difference 1.4e-7 | yes |
| FX-SPH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-010 frame 0: Light intensity 0: the ambient light alone, every pixel the wrapped drawing at 40 per cent. | largest difference 1.2e-7 | yes |
| FX-SPH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-011 frame 0: Ambient 100, diffuse 0, specular 0: no shading, the wrapped drawing as it is. | largest difference 1.2e-7 | yes |
| FX-SPH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-012 frame 0: An orange light, metal 0: the lit side and the highlight orange; with no blue in the light, blue is the ambient's alone. | largest difference 1.2e-7 | yes |
| FX-SPH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-013 frame 0: Roughness 50, specular 100: a larger, duller highlight. | largest difference 1.8e-7 | yes |
| FX-SPH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-014 frame 0: Light height -50: the light from behind, so the side facing the viewer is darker. | largest difference 1.2e-7 | yes |
| FX-SPH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-015 frame 0: Light direction 90: the light from the right. | largest difference 1.2e-7 | yes |
| FX-SPH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-016 frame 0: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.2e-7 | yes |
| FX-SPH-016 frame 2: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.1e-7 | yes |
| FX-SPH-016 frame 4: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.2e-7 | yes |
| FX-SPH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-017 frame 0: FX-SPH-001 moved three pixels right: the same, moved, sphere and all. | largest difference 1.2e-7 | yes |
| FX-SPH-017 frame 3: FX-SPH-001 moved three pixels right: the same, moved, sphere and all. | largest difference 1.2e-7 | yes |
| FX-SPH-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPH-018 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-018 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-019 frame 0: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-019 frame 4: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-020 frame 0: Offset [1001, 50], above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-020 frame 4: Offset [1001, 50], above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-021 frame 0: Rotation X 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-021 frame 4: Rotation X 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-022 frame 0: Render "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-022 frame 4: Render "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-023 frame 0: Light color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-023 frame 4: Light color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-024 frame 0: Metal 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-024 frame 4: Metal 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-025 frame 0: Roughness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-025 frame 4: Roughness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPH-026 frame 0: Light intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-026 frame 4: Light intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPH-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sph_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sph_012.json is saved with its fifteen settings, an orange light, metal 0, Offset [50, 50] | {"ambient":40,"diffuse":60,"light_color":"#ff8000","light_direction":-45,"light_height":50,"light_intensity":100,"metal":0,"offset":[50,50],"radius":50,"render":"full","rotation_x":0,"rotation_y":0,"rotation_z":0,"roughness":10,"specular":50} | yes |
| fx_sph_018.json is refused in a sentence naming radius | CC Sphere's radius runs from 0 to 1000, and this is -1. | yes |
| fx_sph_019.json is refused in a sentence naming radius | CC Sphere's radius runs from 0 to 1000, and this is 1001. | yes |
| fx_sph_020.json is refused in a sentence naming offset | CC Sphere's offset runs from -1000 to 1000, and this is 1001. | yes |
| fx_sph_021.json is refused in a sentence naming rotation x | CC Sphere's rotation x runs from -3600 to 3600, and this is 3601. | yes |
| fx_sph_022.json is refused in a sentence naming render | CC Sphere's render is one of "full", "outside", "inside", and this is "Full". | yes |
| fx_sph_023.json is refused in a sentence naming light colour | CC Sphere's light colour is written #rrggbb, and this is "white". | yes |
| fx_sph_024.json is refused in a sentence naming metal | CC Sphere's metal runs from 0 to 100, and this is 101. | yes |
| fx_sph_025.json is refused in a sentence naming roughness | CC Sphere's roughness runs from 0 to 100, and this is -1. | yes |
| fx_sph_026.json is refused in a sentence naming light intensity | CC Sphere's light intensity runs from 0 to 1000, and this is 1001. | yes |
| a file with a CC Sphere whose radius is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Sphere whose Offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-532)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sph_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_sph_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Sphere as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073288 pixels changed | yes |
| the reference shot, CC Sphere as added on three layers, frame 0, Full | largest difference 1 of 255, 1001 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere as added on three layers, frame 100, Full | largest difference 1 of 255, 1042 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere as added on three layers, frame 239, Full | largest difference 1 of 255, 997 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere as added on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere as added on three layers, frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere as added on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 0, Full | largest difference 1 of 255, 1016 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 100, Full | largest difference 1 of 255, 1003 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 239, Full | largest difference 1 of 255, 851 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 0, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 100, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere render inside, tipped, an orange light, metal 0 on three layers, frame 239, Draft | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072095 pixels changed | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 0, Full | largest difference 1 of 255, 921 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 100, Full | largest difference 1 of 255, 934 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 239, Full | largest difference 1 of 255, 894 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 0, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere radius 100, turned on its side, rough on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 0, Full | largest difference 1 of 255, 1001 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 100, Full | largest difference 1 of 255, 1271 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 239, Full | largest difference 1 of 255, 997 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 0, Draft | largest difference 1 of 255, 60 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 100, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Sphere Rotation Y keyed one whole turn on three layers, frame 239, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-532 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [106, 142, 167, 255]; warnings [] | yes |
| 2_inside.png: drawn without a warning, the street changed, the middle drawn | 129495 pixels changed; middle [157, 171, 187, 255]; warnings [] | yes |
| 3_turned_a_quarter.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [155, 178, 197, 255]; warnings [] | yes |
| 4_tipped_towards_you.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [140, 170, 202, 255]; warnings [] | yes |
| 5_on_its_side_orange.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [105, 118, 123, 255]; warnings [] | yes |

## Result

161 of 161 checks pass.
