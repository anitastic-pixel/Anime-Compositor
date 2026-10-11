# D-507: CC Spotlight

B-387, after CycoreFX's CC Spotlight (Perspective): a spotlight cone shining down on the layer. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/cc_spotlight_reference.py`: a lamp at From, Height above the drawing, pointed at To; a pixel's share of the light is 1 inside the inner angle, 0 outside half the Cone Angle, smoothstep between; Light Only, Light Add, Light Add+ and Light Shadow use it. The four Gel renders are D-519's. Every expected pixel is `Fixtures/cc_spotlight/expected_cc_spotlight.json`, written by the tool before this code existed and printed in document 25 as FX-SPOT-001 to 027. Tolerance 2e-5.

## FX-SPOT-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPOT-001 frame 0: The settings as they start: a white light at a quarter across and down, half the width above the drawing, pointed at its middle, opening 40 degrees, edge softness 20, intensity 100, render light shadow. Inside the cone the drawing is as it was, outside it is black, between it fades. | largest difference 1.2e-7 | yes |
| FX-SPOT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-002 frame 0: Render light only: the drawing is gone and the light alone is left, white and whole inside the cone, fading to transparent at its edge. | largest difference 2.8e-8 | yes |
| FX-SPOT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-003 frame 0: Render light add: the light added to the drawing where it is covered; the transparent columns stay transparent. | largest difference 1.9e-7 | yes |
| FX-SPOT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-004 frame 0: Render light add plus: the light added everywhere, so inside the cone the transparent first column, the light aimed at it, becomes white light. | largest difference 1.9e-7 | yes |
| FX-SPOT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-005 frame 0: Edge softness 0: the cone's edge is a hard step, every pixel whole or black. | largest difference 1.2e-7 | yes |
| FX-SPOT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-006 frame 0: Edge softness 100: the light fades from the axis itself to the edge. | largest difference 7.2e-8 | yes |
| FX-SPOT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-007 frame 0: Cone angle 90: a wider pool of light. | largest difference 1.9e-7 | yes |
| FX-SPOT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-008 frame 0: Height 10: the light low and to one side, so its pool is a long oval reaching away from it. | largest difference 8.4e-8 | yes |
| FX-SPOT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-009 frame 0: An orange light, render light add: orange added inside the cone. | largest difference 1.9e-7 | yes |
| FX-SPOT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-010 frame 0: Intensity 250, render light add: two and a half times the light, so the lit pixels run past 1. | largest difference 2.4e-7 | yes |
| FX-SPOT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-011 frame 0: Intensity 50, render light shadow: outside the cone half as dark. | largest difference 1.2e-7 | yes |
| FX-SPOT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-012 frame 0: Intensity 0, render light shadow, white: no shadow and no tint, so the drawing is exactly as it was. | largest difference 1.9e-7 | yes |
| FX-SPOT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-013 frame 0: From at the middle, straight above the target: a round pool in the middle. | largest difference 1.9e-7 | yes |
| FX-SPOT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-014 frame 0: Height 0 with From on To: the light has no direction, so nothing is lit and light shadow leaves the drawing black. | largest difference 3.0e-8 | yes |
| FX-SPOT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-015 frame 0: From keyed from [0, 0] at frame 0 to [100, 0] at frame 4, linear, render light add: the light swings across the top. | largest difference 1.9e-7 | yes |
| FX-SPOT-015 frame 2: From keyed from [0, 0] at frame 0 to [100, 0] at frame 4, linear, render light add: the light swings across the top. | largest difference 1.9e-7 | yes |
| FX-SPOT-015 frame 4: From keyed from [0, 0] at frame 0 to [100, 0] at frame 4, linear, render light add: the light swings across the top. | largest difference 1.9e-7 | yes |
| FX-SPOT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-016 frame 0: Cone angle keyed from 10 at frame 0 to 70 at frame 4, linear. | largest difference 8.4e-8 | yes |
| FX-SPOT-016 frame 2: Cone angle keyed from 10 at frame 0 to 70 at frame 4, linear. | largest difference 1.2e-7 | yes |
| FX-SPOT-016 frame 4: Cone angle keyed from 10 at frame 0 to 70 at frame 4, linear. | largest difference 1.6e-7 | yes |
| FX-SPOT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-017 frame 0: FX-SPOT-003 moved three pixels right: the same, moved, light and all. | largest difference 1.9e-7 | yes |
| FX-SPOT-017 frame 3: FX-SPOT-003 moved three pixels right: the same, moved, light and all. | largest difference 1.9e-7 | yes |
| FX-SPOT-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-018 frame 0: To at [150, 50], past the right edge: only the right side is lit, faintly at the rim. | largest difference 1.9e-7 | yes |
| FX-SPOT-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOT-019 frame 0: From x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-019 frame 4: From x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-020 frame 0: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-020 frame 4: Height -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-021 frame 0: Cone angle 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-021 frame 4: Cone angle 181, above 180. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-022 frame 0: Edge softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-022 frame 4: Edge softness 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-023 frame 0: Intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-023 frame 4: Intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-024 frame 0: Render "gel": not one of the eight renders (D-519 changed this from "gel_add", refused while the Gel renders were not built). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-024 frame 4: Render "gel": not one of the eight renders (D-519 changed this from "gel_add", refused while the Gel renders were not built). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-025 frame 0: Render "Light_Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-025 frame 4: Render "Light_Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-026 frame 0: Color "#ff80": not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-026 frame 4: Color "#ff80": not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOT-027 frame 0: Color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-027 frame 4: Color "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SPOT-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spot_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spot_009.json is saved with its eight settings, an orange light, render light add, From [25, 25] | {"color":"#ff8000","cone_angle":40,"edge_softness":20,"from":[25,25],"height":50,"intensity":100,"render":"light_add","to":[50,50]} | yes |
| fx_spot_019.json is refused in a sentence naming from | CC Spotlight's from runs from -1000 to 1000, and this is 1001. | yes |
| fx_spot_020.json is refused in a sentence naming height | CC Spotlight's height runs from 0 to 1000, and this is -1. | yes |
| fx_spot_021.json is refused in a sentence naming cone angle | CC Spotlight's cone angle runs from 0 to 180, and this is 181. | yes |
| fx_spot_022.json is refused in a sentence naming edge softness | CC Spotlight's edge softness runs from 0 to 100, and this is 101. | yes |
| fx_spot_023.json is refused in a sentence naming intensity | CC Spotlight's intensity runs from 0 to 1000, and this is 1001. | yes |
| fx_spot_024.json is refused in a sentence naming render | CC Spotlight's render is one of "light_only", "light_add", "light_add_plus", "light_shadow", "gel_only", "gel_add", "gel_add_plus", "gel_shadow", and this is "gel". | yes |
| fx_spot_025.json is refused in a sentence naming render | CC Spotlight's render is one of "light_only", "light_add", "light_add_plus", "light_shadow", "gel_only", "gel_add", "gel_add_plus", "gel_shadow", and this is "Light_Add". | yes |
| fx_spot_026.json is refused in a sentence naming colour | CC Spotlight's colour is written #rrggbb, and this is "#ff80". | yes |
| fx_spot_027.json is refused in a sentence naming colour | CC Spotlight's colour is written #rrggbb, and this is "white". | yes |
| a file with a CC Spotlight whose height is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a CC Spotlight whose From is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-507)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spot_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spot_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Spotlight as added (light shadow): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1684304 pixels changed | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 0, Full | largest difference 1 of 255, 3879 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 100, Full | largest difference 1 of 255, 1222 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 239, Full | largest difference 1 of 255, 1898 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 0, Draft | largest difference 1 of 255, 136 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 100, Draft | largest difference 1 of 255, 33 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight as added (light shadow) on three layers, frame 239, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 628339 pixels changed | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 0, Full | largest difference 1 of 255, 2625 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 100, Full | largest difference 1 of 255, 333 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 239, Full | largest difference 1 of 255, 662 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 0, Draft | largest difference 1 of 255, 112 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 100, Draft | largest difference 1 of 255, 26 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add, orange, intensity 250, edge softness 0 on three layers, frame 239, Draft | largest difference 1 of 255, 34 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1351767 pixels changed | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 0, Full | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 100, Full | largest difference 1 of 255, 218 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 239, Full | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 100, Draft | largest difference 1 of 255, 11 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light add plus, low and wide on three layers, frame 239, Draft | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 0, Full | largest difference 1 of 255, 14 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 100, Full | largest difference 1 of 255, 208 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 239, Full | largest difference 1 of 255, 14 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 100, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight light only, From keyed on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-507 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed, the middle lit | 105187 pixels changed; middle [90, 140, 170, 255]; warnings [] | yes |
| 2_light_only.png: drawn without a warning, the street changed, the middle lit | 129600 pixels changed; middle [255, 255, 255, 255]; warnings [] | yes |
| 3_light_add_orange.png: drawn without a warning, the street changed, the middle lit | 38928 pixels changed; middle [255, 230, 184, 255]; warnings [] | yes |
| 4_low_and_wide.png: drawn without a warning, the street changed, the middle lit | 71952 pixels changed; middle [90, 140, 170, 255]; warnings [] | yes |
| 5_hard_edge.png: drawn without a warning, the street changed, the middle lit | 105640 pixels changed; middle [90, 140, 170, 255]; warnings [] | yes |

## Result

167 of 167 checks pass.
