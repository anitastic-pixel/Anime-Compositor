# D-530: CC Cylinder

B-410, after CycoreFX's CC Cylinder (Perspective): the layer wrapped round an open cylinder, turned and lit, in its own fake 3D. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/cc_cylinder_reference.py`: a fixed eye (a 50 mm lens on a 36 mm frame) looks at the cylinder, turned X then Y then Z; the outside is the nearer hit and the inside the farther; a directional light (Light Height and Direction) gives ambient, diffuse and a Blinn-Phong highlight whose colour Metal mixes between the layer's and the light's. The composition camera and 3D layers are not used. Every expected pixel is `Fixtures/cc_cylinder/expected_cc_cylinder.json`, written by the tool before this code existed and printed in document 25 as FX-CYL-001 to 027. Tolerance 2e-5.

## FX-CYL-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CYL-001 frame 0: The settings as they start: the drawing wrapped round an upright cylinder half the drawing's half width in radius, in the middle, its middle column facing the viewer, lit from the upper left; its sides fall away round the curve, the corners beside it are transparent, and the inside's far wall is hidden behind the front. | largest difference 1.2e-7 | yes |
| FX-CYL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-002 frame 0: Render outside: the outside alone. | largest difference 1.2e-7 | yes |
| FX-CYL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-003 frame 0: Render inside: the inside alone, the far wall, the drawing seen from behind and so mirrored, its middle at the drawing's two side edges. | largest difference 1.2e-7 | yes |
| FX-CYL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-004 frame 0: Rotation Y 90: the cylinder turned a quarter about its upright axis, so the drawing three quarters of the way across faces the viewer. | largest difference 2.2e-7 | yes |
| FX-CYL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-005 frame 0: Rotation X 30: the cylinder tipped, its top towards the viewer, so the open end shows the inside. | largest difference 1.0e-7 | yes |
| FX-CYL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-006 frame 0: Rotation Z 90: the cylinder lying on its side, the drawing's columns wrapped round it from top to bottom and its rows running along it. | largest difference 1.2e-7 | yes |
| FX-CYL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-007 frame 0: Radius 100: a cylinder as wide as the drawing, nearer the viewer and so running past the drawing's sides. | largest difference 1.4e-7 | yes |
| FX-CYL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-008 frame 0: Radius 0: no cylinder, so nothing is drawn and the layer is transparent. | largest difference 0.0e0 | yes |
| FX-CYL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-009 frame 0: Position Z 100: the cylinder a drawing's width farther away, smaller. | largest difference 1.1e-7 | yes |
| FX-CYL-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-010 frame 0: Position [25, 50]: the cylinder a quarter of the way across, seen a little from the side. | largest difference 1.2e-7 | yes |
| FX-CYL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-011 frame 0: Light intensity 0: the ambient light alone, every pixel the wrapped drawing at 40 per cent. | largest difference 6.0e-8 | yes |
| FX-CYL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-012 frame 0: Ambient 100, diffuse 0, specular 0: no shading, the wrapped drawing as it is. | largest difference 1.5e-7 | yes |
| FX-CYL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-013 frame 0: An orange light, metal 0: the lit side and the highlight orange; with no blue in the light, blue is the ambient's alone. | largest difference 1.2e-7 | yes |
| FX-CYL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-014 frame 0: Roughness 50, specular 100: a larger, duller highlight. | largest difference 1.9e-7 | yes |
| FX-CYL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-015 frame 0: Light height -50: the light from behind, so the side facing the viewer is darker. | largest difference 6.0e-8 | yes |
| FX-CYL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-016 frame 0: Light direction 90: the light from the right. | largest difference 1.3e-7 | yes |
| FX-CYL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-017 frame 0: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.2e-7 | yes |
| FX-CYL-017 frame 2: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.2e-7 | yes |
| FX-CYL-017 frame 4: Rotation Y keyed from 0 at frame 0 to 360 at frame 4, linear: one whole turn, half way at frame 2. | largest difference 1.2e-7 | yes |
| FX-CYL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-018 frame 0: FX-CYL-001 moved three pixels right: the same, moved, cylinder and all. | largest difference 1.2e-7 | yes |
| FX-CYL-018 frame 3: FX-CYL-001 moved three pixels right: the same, moved, cylinder and all. | largest difference 1.2e-7 | yes |
| FX-CYL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CYL-019 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-019 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-020 frame 0: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-020 frame 4: Radius 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-021 frame 0: Position Z 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-021 frame 4: Position Z 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-022 frame 0: Rotation Y 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-022 frame 4: Rotation Y 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-023 frame 0: Render "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-023 frame 4: Render "Full": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-024 frame 0: Light color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-024 frame 4: Light color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-025 frame 0: Ambient 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-025 frame 4: Ambient 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-026 frame 0: Roughness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-026 frame 4: Roughness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CYL-027 frame 0: Light height 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-027 frame 4: Light height 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CYL-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cyl_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cyl_013.json is saved with its sixteen settings, an orange light, metal 0, Position [50, 50] | {"ambient":40,"diffuse":60,"light_color":"#ff8000","light_direction":-45,"light_height":50,"light_intensity":100,"metal":0,"position":[50,50],"position_z":0,"radius":50,"render":"full","rotation_x":0,"rotation_y":0,"rotation_z":0,"roughness":10,"specular":50} | yes |
| fx_cyl_019.json is refused in a sentence naming radius | CC Cylinder's radius runs from 0 to 1000, and this is -1. | yes |
| fx_cyl_020.json is refused in a sentence naming radius | CC Cylinder's radius runs from 0 to 1000, and this is 1001. | yes |
| fx_cyl_021.json is refused in a sentence naming position z | CC Cylinder's position z runs from -1000 to 1000, and this is 1001. | yes |
| fx_cyl_022.json is refused in a sentence naming rotation y | CC Cylinder's rotation y runs from -3600 to 3600, and this is 3601. | yes |
| fx_cyl_023.json is refused in a sentence naming render | CC Cylinder's render is one of "full", "outside", "inside", and this is "Full". | yes |
| fx_cyl_024.json is refused in a sentence naming light colour | CC Cylinder's light colour is written #rrggbb, and this is "white". | yes |
| fx_cyl_025.json is refused in a sentence naming ambient | CC Cylinder's ambient runs from 0 to 100, and this is 101. | yes |
| fx_cyl_026.json is refused in a sentence naming roughness | CC Cylinder's roughness runs from 0 to 100, and this is -1. | yes |
| fx_cyl_027.json is refused in a sentence naming light height | CC Cylinder's light height runs from -100 to 100, and this is 101. | yes |
| a file with a CC Cylinder whose radius is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Cylinder whose Position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-530)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cyl_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cyl_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Cylinder as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073170 pixels changed | yes |
| the reference shot, CC Cylinder as added on three layers, frame 0, Full | largest difference 1 of 255, 866 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder as added on three layers, frame 100, Full | largest difference 1 of 255, 1168 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder as added on three layers, frame 239, Full | largest difference 1 of 255, 953 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder as added on three layers, frame 0, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder as added on three layers, frame 100, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder as added on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073599 pixels changed | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 0, Full | largest difference 1 of 255, 824 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 100, Full | largest difference 1 of 255, 750 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 239, Full | largest difference 1 of 255, 644 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 100, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder render inside, tipped, an orange light, metal 0 on three layers, frame 239, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2072296 pixels changed | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 0, Full | largest difference 1 of 255, 855 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 100, Full | largest difference 1 of 255, 1012 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 239, Full | largest difference 1 of 255, 928 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 100, Draft | largest difference 1 of 255, 79 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder radius 100, lying on its side, rough on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073551 pixels changed | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 0, Full | largest difference 1 of 255, 866 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 100, Full | largest difference 1 of 255, 1225 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 239, Full | largest difference 1 of 255, 953 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 0, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 100, Draft | largest difference 1 of 255, 78 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Cylinder Rotation Y keyed one whole turn on three layers, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-530 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [106, 142, 167, 255]; warnings [] | yes |
| 2_inside.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [157, 171, 187, 255]; warnings [] | yes |
| 3_turned_a_quarter.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [156, 179, 197, 255]; warnings [] | yes |
| 4_tipped_towards_you.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [115, 146, 179, 255]; warnings [] | yes |
| 5_on_its_side_orange.png: drawn without a warning, the street changed, the middle drawn | 129600 pixels changed; middle [105, 118, 123, 255]; warnings [] | yes |

## Result

165 of 165 checks pass.
