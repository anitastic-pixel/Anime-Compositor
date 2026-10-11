# D-519: CC Spotlight's Gel renders

B-399: the four Gel renders (Gel Only, Gel Add, Gel Add+, Gel Shadow) and the Gel Layer of `core.cc_spotlight` (D-507), after CycoreFX's CC Spotlight (Perspective): the light projects a layer through its cone like a slide. CycoreFX describes the renders but publishes no formula, so the rule is ours, stated in `tools/cc_spotlight_gel_reference.py`: the gel layer (D-189's layer setting, centred on the drawing) is carried across the cone, its longer side spanning the opening; the light let through is the gel over white, (1 - gel alpha) + gel colour; each Gel render is its Light render with the light's colour multiplied by that. With no gel layer a Gel render is its Light render, and a Light render never reads the gel layer. Every expected pixel is `Fixtures/cc_spotlight_gel/expected_cc_spotlight_gel.json`, written by the tool before this code existed and printed in document 25 as FX-SPOTGEL-001 to 021. Tolerance 2e-5.

## FX-SPOTGEL-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SPOTGEL-001 frame 0: Render gel only, the paint as the gel, the light straight down on the middle, opening 90 degrees: the drawing is gone and the paint is projected in a round pool, its long side across the pool, fading at the edge. | largest difference 1.4e-7 | yes |
| FX-SPOTGEL-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-002 frame 0: Render gel add: the paint's light added to the drawing where it is covered; the transparent corner at the bottom right stays transparent. | largest difference 3.0e-7 | yes |
| FX-SPOTGEL-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-003 frame 0: Render gel add plus, the light straight down on the transparent corner at the bottom right: the paint's light falls on the transparent part too. | largest difference 2.9e-7 | yes |
| FX-SPOTGEL-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-004 frame 0: Render gel shadow, the slide as the gel: outside the cone black, inside the drawing as it was where the slide is clear and tinted blue under its cross. | largest difference 1.6e-7 | yes |
| FX-SPOTGEL-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-005 frame 0: Render gel add with no gel layer: exactly light add. | largest difference 2.1e-7 | yes |
| FX-SPOTGEL-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-006 frame 0: Render light add with the paint named as the gel: a Light render ignores the gel, so exactly light add. | largest difference 2.1e-7 | yes |
| FX-SPOTGEL-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-007 frame 0: Gel add, the small paint (8 by 6) as the gel: centred on the drawing, so its light is the paint's in the middle and white around it. | largest difference 2.9e-7 | yes |
| FX-SPOTGEL-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-008 frame 0: Gel add, the gel a layer that starts at frame 4: plain white light before, the slide from frame 4. | largest difference 2.1e-7 | yes |
| FX-SPOTGEL-008 frame 4: Gel add, the gel a layer that starts at frame 4: plain white light before, the slide from frame 4. | largest difference 3.1e-7 | yes |
| FX-SPOTGEL-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-009 frame 0: Gel add, the gel layer "gone" not in the composition: the drawing unchanged, warned every frame. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-009 frame 2: Gel add, the gel layer "gone" not in the composition: the drawing unchanged, warned every frame. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-SPOTGEL-010 frame 0: Gel only, the light low at the top left, height 20, opening 60: the paint is drawn out into a long oval. | largest difference 1.8e-7 | yes |
| FX-SPOTGEL-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-011 frame 0: Gel add, an orange light at intensity 250 through the slide: orange light past 1 through the clear parts, darker under the cross. | largest difference 2.9e-7 | yes |
| FX-SPOTGEL-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-012 frame 0: Gel shadow at intensity 50 through the paint: outside the cone half as dark, inside tinted by the paint. | largest difference 1.8e-7 | yes |
| FX-SPOTGEL-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-013 frame 0: Gel only, the holder itself as the gel: its own drawing projected onto the cleared layer. | largest difference 1.6e-7 | yes |
| FX-SPOTGEL-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-014 frame 0: Gel add, opening 180: the gel's spread is held at 89 degrees, so its middle colours the whole lit drawing. | largest difference 2.6e-7 | yes |
| FX-SPOTGEL-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-015 frame 0: Gel add with From keyed from [30, 50] at frame 0 to [70, 50] at frame 4, linear: the gel slides and stretches as the light swings. | largest difference 2.7e-7 | yes |
| FX-SPOTGEL-015 frame 2: Gel add with From keyed from [30, 50] at frame 0 to [70, 50] at frame 4, linear: the gel slides and stretches as the light swings. | largest difference 3.0e-7 | yes |
| FX-SPOTGEL-015 frame 4: Gel add with From keyed from [30, 50] at frame 0 to [70, 50] at frame 4, linear: the gel slides and stretches as the light swings. | largest difference 3.1e-7 | yes |
| FX-SPOTGEL-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-016 frame 0: FX-SPOTGEL-004 on an adjustment layer above the drawing: the same. | largest difference 1.6e-7 | yes |
| FX-SPOTGEL-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-017 frame 0: FX-SPOTGEL-002 moved three pixels right: the same, moved, light and all. | largest difference 3.0e-7 | yes |
| FX-SPOTGEL-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-018 frame 0: Gel add, the light along the drawing (height 0) from the left middle to the right middle: the axis lies along x, so the gel's across is the drawing's y. | largest difference 3.7e-7 | yes |
| FX-SPOTGEL-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SPOTGEL-019 frame 0: A gel layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-019 frame 4: A gel layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOTGEL-020 frame 0: Render "Gel_Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-020 frame 4: Render "Gel_Add": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SPOTGEL-021 frame 0: Render "gel_glow": not one of CC Spotlight's renders. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-021 frame 4: Render "gel_glow": not one of CC Spotlight's renders. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SPOTGEL-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the holder's drawing, each with a CC Spotlight rendering gel add whose gel layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |
| light_names_other.json: The same two layers with Light renders: a Light render never reads its gel layer, so there is no cycle and the file opens. | opened | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spotgel_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_spotgel_011.json is saved with its nine settings, render gel add, the slide as the gel | {"color":"#ff8000","cone_angle":90,"edge_softness":20,"from":[50,50],"gel_layer":"slide","height":50,"intensity":250,"render":"gel_add","to":[50,50]} | yes |
| D-507's fx_spot_009.json, written before the Gel Layer: opens with no gel layer and is saved with its eight settings, as it was | CcSpotlight { from: [25.0, 25.0], to: [50.0, 50.0], height: 50.0, cone_angle: 40.0, edge_softness: 20.0, color: "#ff8000", intensity: 100.0, render: "light_add", gel_layer: String(""), gel_map: None }; saved {"color":"#ff8000","cone_angle":40,"edge_softness":20,"from":[25,25],"height":50,"intensity":100,"render":"light_add","to":[50,50]} | yes |
| fx_spotgel_019.json is refused in a sentence naming gel layer | CC Spotlight's gel layer is the name of a layer of this composition, and this is 3. | yes |
| fx_spotgel_020.json is refused in a sentence naming render | CC Spotlight's render is one of "light_only", "light_add", "light_add_plus", "light_shadow", "gel_only", "gel_add", "gel_add_plus", "gel_shadow", and this is "Gel_Add". | yes |
| fx_spotgel_021.json is refused in a sentence naming render | CC Spotlight's render is one of "light_only", "light_add", "light_add_plus", "light_shadow", "gel_only", "gel_add", "gel_add_plus", "gel_shadow", and this is "gel_glow". | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| render "gel_glow" is refused with a sentence, and nothing changes | CC Spotlight's render is one of "light_only", "light_add", "light_add_plus", "light_shadow", "gel_only", "gel_add", "gel_add_plus", "gel_shadow", and this is "gel_glow". | yes |
| a gel layer written as the number 3 is refused with a sentence, and nothing changes | CC Spotlight's gel layer is the name of a layer of this composition, and this is 3. | yes |
| render gel only, the small paint as the gel is taken | taken | yes |
| intensity keyed from 0 to 250 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |
| a half-size draft preview changes no setting (none is a distance) | CcSpotlight { from: [50.0, 50.0], to: [50.0, 50.0], height: 50.0, cone_angle: 90.0, edge_softness: 20.0, color: "#ffffff", intensity: 100.0, render: "gel_add", gel_layer: String("paint"), gel_map: None } | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spotgel_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_008.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_015.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_spotgel_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_spotgel_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_spotgel_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1877214 pixels changed | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 0, Full | largest difference 1 of 255, 983 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 100, Full | largest difference 1 of 255, 1931 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 239, Full | largest difference 1 of 255, 1318 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 0, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 100, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add, layer 4 as the gel, opening 90 on three layers, frame 239, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 0, Full | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 100, Full | largest difference 1 of 255, 164 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 239, Full | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 100, Draft | largest difference 1 of 255, 9 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel only, layer 4 as the gel, low and wide on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 0, Full | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 100, Full | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 239, Full | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 0, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 100, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel shadow, layer 4 as the gel, From [50, 50], orange on three layers, frame 239, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 625052 pixels changed | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 0, Full | largest difference 1 of 255, 73 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 100, Full | largest difference 1 of 255, 383 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 239, Full | largest difference 1 of 255, 140 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 0, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 100, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, CC Spotlight gel add plus, no gel layer on three layers, frame 239, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-519 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 3_gel_only.png, gel only, the four panes as the gel, the light straight down on the middle: the street gone, the panes projected in a round pool of light on black (the clear outside laid on black in the picture), red top left, green top right, blue bottom left, yellow bottom right, white light round them; draws cleanly | [], 32400 pixels differ from light only | yes |
| 4_gel_add.png, gel add: the street with the coloured panes' light added in the pool; draws cleanly | [], 121444 pixels changed | yes |
| 5_gel_shadow.png, gel shadow: black outside the pool, the street inside tinted by each pane; draws cleanly | [], 76556 pixels changed | yes |
| 6_low_light.png, gel only, the light low at the top left: the panes and the pool drawn out into a long slanted shape, on black; draws cleanly | [], 86602 pixels differ from 3 | yes |
| gel add with no gel layer is byte for byte light add with the gel named (a Light render ignores it) | [], 0 pixels differ | yes |
| a gel layer not in the composition: the street exactly as it was, with the warning | ["EFFECT_LAYER_MISSING An effect on layer art reads layer gone, which is not in this composition."], 0 pixels changed | yes |

## Result

148 of 148 checks pass.
