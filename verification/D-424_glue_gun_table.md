# D-424: Glue Gun

B-303, after CycoreFX's CC Glue Gun: a glossy stroke squeezed out along the path the brush takes. Each frame the brush's place now and at each frame of the Time Span before (with Time Span 0, back to the layer's in point) is read as its keys are; Density blobs a frame are laid along that path, wobbling with Wobbly; the blobs run into one another by Strength, and the paint mirrors the layer by Reflection and is lit as Blobbylize is. The manual gives no formula, ranges or defaults, so those are ours. Every expected pixel is `Fixtures/glue_gun/expected_glue_gun.json`, written by `tools/glue_gun_reference.py` before this code existed and printed in document 25 as FX-GLUE-001 to 027. Tolerance 2e-5.

## FX-GLUE-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLUE-001 frame 0: As added, the brush never keyed at the middle: Stroke Width 20, Density 5, Time Span 1, Reflection 50, Strength 50, lit as Blobbylize is: one round glossy blob over the whole drawing, the same at frame 4 (more blobs on the same spot pile up but reach no further), on the grey ramp. | largest difference 9.7e-8 | yes |
| FX-GLUE-001 frame 4: As added, the brush never keyed at the middle: Stroke Width 20, Density 5, Time Span 1, Reflection 50, Strength 50, lit as Blobbylize is: one round glossy blob over the whole drawing, the same at frame 4 (more blobs on the same spot pile up but reach no further), on the grey ramp. | largest difference 9.7e-8 | yes |
| FX-GLUE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-002 frame 0: The sweep, unlit, Reflection 0: at frame 0 a single blob at the left, at frame 2 a stroke half way across, at frame 4 the whole stroke, 4 pixels wide, the photo beneath it at half brightness. | largest difference 1.7e-7 | yes |
| FX-GLUE-002 frame 2: The sweep, unlit, Reflection 0: at frame 0 a single blob at the left, at frame 2 a stroke half way across, at frame 4 the whole stroke, 4 pixels wide, the photo beneath it at half brightness. | largest difference 1.6e-7 | yes |
| FX-GLUE-002 frame 4: The sweep, unlit, Reflection 0: at frame 0 a single blob at the left, at frame 2 a stroke half way across, at frame 4 the whole stroke, 4 pixels wide, the photo beneath it at half brightness. | largest difference 1.6e-7 | yes |
| FX-GLUE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-003 frame 2: The sweep lit as added: the stroke stands up from the photo, lighter on its upper edge facing the top-left light, on the grey ramp. | largest difference 1.1e-7 | yes |
| FX-GLUE-003 frame 4: The sweep lit as added: the stroke stands up from the photo, lighter on its upper edge facing the top-left light, on the grey ramp. | largest difference 1.1e-7 | yes |
| FX-GLUE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-004 frame 4: The sweep with Time Span 0.1 seconds, 2 frames at 24 a second: at frame 4 only the last two frames of the stroke are left, the tail gone. | largest difference 1.6e-7 | yes |
| FX-GLUE-004 frame 6: The sweep with Time Span 0.1 seconds, 2 frames at 24 a second: at frame 4 only the last two frames of the stroke are left, the tail gone. | largest difference 1.6e-7 | yes |
| FX-GLUE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-005 frame 4: The sweep with Density 0.5, one blob every second frame, and Strength 0: separate round discs 2 pixels across, not touching. | largest difference 1.5e-7 | yes |
| FX-GLUE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-006 frame 4: FX-GLUE-005 with Strength 100: the same blobs swell into one another and join. | largest difference 1.5e-7 | yes |
| FX-GLUE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-007 frame 4: The sweep, Reflection 100, Stroke Width 6, unlit: the stroke's edges show the photo from across the stroke, mirrored, its middle the photo beneath. | largest difference 1.5e-7 | yes |
| FX-GLUE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-008 frame 4: The sweep, Wobbly, Wobble Width and Height 1, Speed 2: the blobs swing a pixel out of place, out of step with one another, and on at frame 6 though the brush has stopped. | largest difference 2.3e-7 | yes |
| FX-GLUE-008 frame 6: The sweep, Wobbly, Wobble Width and Height 1, Speed 2: the blobs swing a pixel out of place, out of step with one another, and on at frame 6 though the brush has stopped. | largest difference 1.8e-7 | yes |
| FX-GLUE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-009 frame 4: The sweep lit by a point light at 50, 20 per cent, 6 pixels up: brightest near the stroke's middle top. | largest difference 1.6e-7 | yes |
| FX-GLUE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-010 frame 4: The sweep, an orange light, Specular 100, Roughness 0.5, Metal 0, Diffuse 0, Stroke Width 6: a broad highlight in the light's own orange. | largest difference 1.5e-7 | yes |
| FX-GLUE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-011 frame 4: FX-GLUE-003 on the holder moved 2 right and 1 down: the same, moved. | largest difference 1.1e-7 | yes |
| FX-GLUE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-012 frame 4: Stroke Width 0: no stroke, the photo untouched. | largest difference 1.5e-7 | yes |
| FX-GLUE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-013 frame 4: Density 0: no blobs, the photo untouched. | largest difference 1.5e-7 | yes |
| FX-GLUE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-014 frame 4: The sweep over the shapes drawing, clear between its blocks, Stroke Width 6: over the blocks the stroke takes their colour; over the clear gap there is nothing to reflect, so the paint is black, lit only by its highlight. | largest difference 2.2e-7 | yes |
| FX-GLUE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-015 frame 2: The brush held at 20 per cent until frame 2, then at 80 from frame 3: at frame 4 a pile of blobs at the left, then a line of them across the jump, laid between frames 2 and 3, unlit. | largest difference 1.5e-7 | yes |
| FX-GLUE-015 frame 4: The brush held at 20 per cent until frame 2, then at 80 from frame 3: at frame 4 a pile of blobs at the left, then a line of them across the jump, laid between frames 2 and 3, unlit. | largest difference 1.7e-7 | yes |
| FX-GLUE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-016 frame 4: The sweep with the light's Direction 135, from the bottom right: the lower edge lights instead. | largest difference 1.3e-7 | yes |
| FX-GLUE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-017 frame 0: The sweep with Stroke Width keyed from 2 at frame 0 to 6 at frame 4: every blob takes the width as it is now, so the whole stroke thickens. | largest difference 1.5e-7 | yes |
| FX-GLUE-017 frame 4: The sweep with Stroke Width keyed from 2 at frame 0 to 6 at frame 4: every blob takes the width as it is now, so the whole stroke thickens. | largest difference 1.5e-7 | yes |
| FX-GLUE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-018 frame 2: The brush eased from 20 to 80 per cent across on a curve that overshoots, Stroke Width 4, unlit: the history is read as the keys are, past the last key's place and back. | largest difference 1.5e-7 | yes |
| FX-GLUE-018 frame 4: The brush eased from 20 to 80 per cent across on a curve that overshoots, Stroke Width 4, unlit: the history is read as the keys are, past the last key's place and back. | largest difference 1.5e-7 | yes |
| FX-GLUE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLUE-019 frame 0: Stroke Width 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-019 frame 4: Stroke Width 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-020 frame 0: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-020 frame 4: Density -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-021 frame 0: Time Span 101, above 100 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-021 frame 4: Time Span 101, above 100 seconds. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-022 frame 0: A paint style written "drippy". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-022 frame 4: A paint style written "drippy". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-023 frame 0: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-023 frame 4: A light type written "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-024 frame 0: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-024 frame 4: Roughness 0, below 0.001. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-025 frame 0: A brush position 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-025 frame 4: A brush position 50, 1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-026 frame 0: A light colour written "orange". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-026 frame 4: A light colour written "orange". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLUE-027 frame 0: Wobble Speed keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-027 frame 4: Wobble Speed keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLUE-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: paint past the layer's edge is cut | 0 | yes |
| a half-size draft preview halves the stroke width, the wobble and a point light's height; the brush and the light's place are shares of the drawing | GlueGun { brush_position: [50.0, 50.0], stroke_width: 10.0, density: 5.0, time_span: 1.0, reflection: 50.0, strength: 50.0, paint_style: "plain", wobble_width: 5.0, wobble_height: 5.0, wobble_speed: 1.0, light_intensity: 100.0, light_color: "#ffffff", light_type: "point", light_height: 20.0, light_position: [30.0, 30.0], light_direction: -45.0, ambient: 25.0, diffuse: 75.0, specular: 50.0, roughness: 0.05, metal: 100.0, trail: [], clock: [0.0, 0.0] } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glue_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glue_010.json is saved with all 21 settings, the brush's keys with them | {"ambient":25,"brush_position":{"base":[20,50],"keyframes":[{"frame":0,"interp":"linear","value":[20,50]},{"frame":4,"interp":"linear","value":[80,50]}]},"density":2,"diffuse":0,"light_color":"#ff8000","light_direction":-45,"light_height":40,"light_intensity":100,"light_position":[30,30],"light_type":"distant","metal":0,"paint_style":"plain","reflection":0,"roughness":0.5,"specular":100,"strength":50,"stroke_width":6,"time_span":0,"wobble_height":10,"wobble_speed":1,"wobble_width":10} | yes |
| fx_glue_019.json is refused in a sentence | Glue Gun's stroke width runs from 0 to 500, and this is 501. | yes |
| fx_glue_020.json is refused in a sentence | Glue Gun's density runs from 0 to 100, and this is -1. | yes |
| fx_glue_021.json is refused in a sentence | Glue Gun's time span runs from 0 to 100, and this is 101. | yes |
| fx_glue_022.json is refused in a sentence | Glue Gun's paint style is "plain" or "wobbly", and this is "drippy". | yes |
| fx_glue_023.json is refused in a sentence | Glue Gun's light type is distant or point, and this is "spot". | yes |
| fx_glue_024.json is refused in a sentence | Glue Gun's roughness runs from 0.001 to 1, and this is 0. | yes |
| fx_glue_025.json is refused in a sentence | Glue Gun's brush position runs from -1000 to 1000, and this is 1001. | yes |
| fx_glue_026.json is refused in a sentence | Glue Gun's light colour is written #rrggbb, and this is "orange". | yes |
| a file with a Glue Gun with no `brush_position` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glue Gun whose brush position is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glue Gun whose stroke width is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Glue Gun with no `metal` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| stroke width 500.5 is refused with a sentence, and nothing changes | Glue Gun's stroke width runs from 0 to 500, and this is 500.5. | yes |
| density -0.5 is refused with a sentence, and nothing changes | Glue Gun's density runs from 0 to 100, and this is -0.5. | yes |
| time span 100.5 is refused with a sentence, and nothing changes | Glue Gun's time span runs from 0 to 100, and this is 100.5. | yes |
| roughness 0 is refused with a sentence, and nothing changes | Glue Gun's roughness runs from 0.001 to 1, and this is 0. | yes |
| paint style "drippy" is refused with a sentence, and nothing changes | Glue Gun's paint style is "plain" or "wobbly", and this is "drippy". | yes |
| light type "spot" is refused with a sentence, and nothing changes | Glue Gun's light type is distant or point, and this is "spot". | yes |
| light colour "#fff" is refused with a sentence, and nothing changes | Glue Gun's light colour is written #rrggbb, and this is "#fff". | yes |
| brush position keyed to 50, 1001 is refused with a sentence, and nothing changes | Glue Gun's brush position runs from -1000 to 1000, and this is 1001. | yes |
| wobble speed keyed to 150 is refused with a sentence, and nothing changes | Glue Gun's wobble speed runs from 0 to 100, and this is 150. | yes |
| the tops: width 500, density 100, time span 100, wobbly 1000 by 1000 at 100, a point light is taken | taken | yes |
| the bottoms: width 0, density 0, time span 0, roughness 0.001 is taken | taken | yes |
| brush position keyed from 20, 50 to 80, 50 is taken | taken | yes |
| stroke width keyed from 2 to 6 is taken | taken | yes |
| light position keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 5 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glue_001.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_002.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_007.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_008.json frame 6 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_009.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_011.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_014.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glue_018.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glue_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_glue_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_glue_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 692 pixels changed | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 0, Full | largest difference 1 of 255, 3772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 239, Full | largest difference 1 of 255, 1763 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun as it starts (width 20, density 5, time span 1, the brush at the middle) on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 31484 pixels changed | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 100, Full | largest difference 1 of 255, 1117 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 100, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, width 60, density 10, time span 2 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 20057 pixels changed | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 100, Full | largest difference 1 of 255, 1199 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, wobbly 30 by 20, a warm point light, reflection 100 on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 9674 pixels changed | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 100, Full | largest difference 1 of 255, 1357 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 100, Draft | largest difference 1 of 255, 37 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Glue Gun the brush keyed across, strength 0, density 1, rough and dull, light from below on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street at frame 4, in `verification/D-424 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts, the brush never keyed: one round glossy blob in the middle; draws cleanly | [], 692 pixels changed | yes |
| 3_sweep.png, the brush keyed from bottom left to top right over frames 0 to 4, kept for ever, density 20: a shiny tube of paint across the street; draws cleanly | [], 9038 pixels changed | yes |
| 4_wobbly_point.png, the same across the middle, wobbly 8 by 12, a point light above: a lumpy wobbling stroke lit from above; draws cleanly | [], 13789 pixels changed | yes |
| 5_mirror_orange.png, width 40, reflection 100, an orange shine: the street mirrored in a thick glassy stroke with an orange highlight; draws cleanly | [], 19878 pixels changed | yes |
| 6_blobs.png, density 1, strength 0, width 30: five separate glossy beads, one a frame; draws cleanly | [], 3564 pixels changed | yes |

## Result

198 of 198 checks pass.
