# D-408: Transform

B-287: `core.transform`, After Effects' Transform effect: the drawing moved, scaled, skewed, turned and faded inside its own layer (the layer never grows), bilinear or bicubic, with its own motion blur when the layer's switch is on and the composition's blur enabled. Every expected pixel is `Fixtures/transform/expected_transform.json`, written by `tools/transform_reference.py` before this code existed and printed in document 25 as FX-XFORM-001 to 032. Tolerance 2e-5.

## FX-XFORM-001 to 032 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-XFORM-001 frame 0: As added: the anchor point and position at the middle, scale 100, no skew or rotation, opacity 100: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-XFORM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-002 frame 0: Position (75, 50): the drawing four pixels right inside its layer; its left four columns clear and its right four cut off, the layer never growing. | largest difference 1.9e-7 | yes |
| FX-XFORM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-003 frame 0: Position (53.125, 50): half a pixel right, each pixel the even mix of itself and the one to its left. | largest difference 1.9e-7 | yes |
| FX-XFORM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-004 frame 0: Scale Height 50 with Uniform Scale on, Scale Width 200 not read: the drawing at half size round the middle, in columns 4 to 11 and rows 2 to 7. | largest difference 1.9e-7 | yes |
| FX-XFORM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-005 frame 0: Uniform Scale off, Scale Width 50, Scale Height 100: squeezed to half its width round the middle, its full height kept. | largest difference 1.9e-7 | yes |
| FX-XFORM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-006 frame 0: Rotation 90: a quarter turn clockwise round the middle; column x of the frame is row 12 - x of the drawing, cut to the layer. | largest difference 1.9e-7 | yes |
| FX-XFORM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-007 frame 0: Rotation 30, bilinear sampling: turned a twelfth, soft at the stripes' edges. | largest difference 2.5e-7 | yes |
| FX-XFORM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-008 frame 0: Rotation 30, bicubic sampling: the same turn, the stripes' edges crisper than FX-XFORM-007. | largest difference 3.2e-7 | yes |
| FX-XFORM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-009 frame 0: Skew 30 along axis 0: the rows slide sideways, the top ones right and the lower ones left, round the middle row; the stripes lean. | largest difference 2.5e-7 | yes |
| FX-XFORM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-010 frame 0: Skew 30 along axis 90: the columns slide up and down instead; the blue band leans. | largest difference 2.5e-7 | yes |
| FX-XFORM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-011 frame 0: Anchor point and position (0, 0), rotation 45: an eighth turn round the drawing's top left corner, half of it swung off the layer. | largest difference 2.5e-7 | yes |
| FX-XFORM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-012 frame 0: Opacity 50: every pixel at half its covering. | largest difference 9.3e-8 | yes |
| FX-XFORM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-013 frame 0: Scale Height -100, uniform: turned over both ways, the same as rotation 180. | largest difference 1.9e-7 | yes |
| FX-XFORM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-014 frame 0: Scale Height 0: nothing left, the frame clear. | largest difference 0.0e0 | yes |
| FX-XFORM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-015 frame 0: Position keyed from (50, 50) at frame 0 to (100, 50) at frame 4, linear, no motion blur: two pixels further right each frame. | largest difference 1.9e-7 | yes |
| FX-XFORM-015 frame 2: Position keyed from (50, 50) at frame 0 to (100, 50) at frame 4, linear, no motion blur: two pixels further right each frame. | largest difference 1.9e-7 | yes |
| FX-XFORM-015 frame 4: Position keyed from (50, 50) at frame 0 to (100, 50) at frame 4, linear, no motion blur: two pixels further right each frame. | largest difference 1.9e-7 | yes |
| FX-XFORM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-016 frame 0: FX-XFORM-015 with the layer's motion blur switch on and the composition's blur enabled at 180 degrees, phase -90, 4 samples: frame 2 is the mean of the slide at 1.8125, 1.9375, 2.0625 and 2.1875, a streak a pixel long. | largest difference 1.9e-7 | yes |
| FX-XFORM-016 frame 2: FX-XFORM-015 with the layer's motion blur switch on and the composition's blur enabled at 180 degrees, phase -90, 4 samples: frame 2 is the mean of the slide at 1.8125, 1.9375, 2.0625 and 2.1875, a streak a pixel long. | largest difference 1.4e-7 | yes |
| FX-XFORM-016 frame 4: FX-XFORM-015 with the layer's motion blur switch on and the composition's blur enabled at 180 degrees, phase -90, 4 samples: frame 2 is the mean of the slide at 1.8125, 1.9375, 2.0625 and 2.1875, a streak a pixel long. | largest difference 1.9e-7 | yes |
| FX-XFORM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-017 frame 2: FX-XFORM-016 with the layer's switch off: no blur, as FX-XFORM-015. | largest difference 1.9e-7 | yes |
| FX-XFORM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-018 frame 2: FX-XFORM-016 with the composition's blur not enabled: no blur. | largest difference 1.9e-7 | yes |
| FX-XFORM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-019 frame 2: FX-XFORM-016 with Use Composition's Shutter Angle off and Shutter Angle 360, centred on the frame: the moments 1.625, 1.875, 2.125 and 2.375, at twice the spacing round 2, a streak two pixels long. | largest difference 1.7e-7 | yes |
| FX-XFORM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-020 frame 2: FX-XFORM-019 with Shutter Angle 0: no blur, as FX-XFORM-015. | largest difference 1.9e-7 | yes |
| FX-XFORM-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-021 frame 0: Rotation 30, not keyed, with the layer's and composition's blur on: nothing moves through the shutter, so no blur, as FX-XFORM-007. | largest difference 2.5e-7 | yes |
| FX-XFORM-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-022 frame 2: Rotation keyed from 0 at frame 0 to 90 at frame 4 with the blur on, bicubic: frame 2 is the mean of four turns round 45 degrees, a spin blur. | largest difference 2.9e-7 | yes |
| FX-XFORM-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-023 frame 0: Position (75, 50) on the layer moved three pixels right: the effect works inside the layer, so FX-XFORM-002 moved three more. | largest difference 1.9e-7 | yes |
| FX-XFORM-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-024 frame 0: Motion Tile at 300% by 300% first, then rotation 90: the points are the drawing's own, so the tiles turn round the drawing's middle and fill the frame. | largest difference 1.9e-7 | yes |
| FX-XFORM-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-025 frame 0: Bicubic sampling with the settings as added: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-XFORM-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-XFORM-026 frame 0: Skew 86, above 85. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-026 frame 4: Skew 86, above 85. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-027 frame 0: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-027 frame 4: Opacity 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-028 frame 0: Position keyed to (1200, 50) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-028 frame 4: Position keyed to (1200, 50) at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-029 frame 0: Shutter Angle 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-029 frame 4: Shutter Angle 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-030 frame 0: Uniform Scale written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-030 frame 4: Uniform Scale written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-031 frame 0: Sampling written "nearest". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-031 frame 4: Sampling written "nearest". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-XFORM-032 frame 0: Use Composition's Shutter Angle written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-032 frame 4: Use Composition's Shutter Angle written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-XFORM-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_xform_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_xform_019.json is saved with its twelve settings, its own shutter as written | {"anchor_point":[50,50],"opacity":100,"position":{"base":[50,50],"keyframes":[{"frame":0,"interp":"linear","value":[50,50]},{"frame":4,"interp":"linear","value":[100,50]}]},"rotation":0,"sampling":"bilinear","scale_height":100,"scale_width":100,"shutter_angle":360,"skew":0,"skew_axis":0,"uniform_scale":"on","use_composition_shutter_angle":"off"} | yes |
| fx_xform_026.json is refused in a sentence naming skew | Transform's skew runs from -85 to 85, and this is 86. | yes |
| fx_xform_027.json is refused in a sentence naming opacity | Transform's opacity runs from 0 to 100, and this is 101. | yes |
| fx_xform_029.json is refused in a sentence naming shutter_angle | Transform's shutter angle runs from 0 to 360, and this is 361. | yes |
| fx_xform_030.json is refused in a sentence naming uniform scale | Transform's uniform scale is "on" or "off", and this is "yes". | yes |
| fx_xform_031.json is refused in a sentence naming sampling | Transform's sampling is "bilinear" or "bicubic", and this is "nearest". | yes |
| fx_xform_032.json is refused in a sentence naming shutter angle | Transform's use composition's shutter angle is "on" or "off", and this is "maybe". | yes |
| a file with a Transform whose rotation is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Transform without its position is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Transform whose anchor point is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| skew 86 is refused with a sentence, and nothing changes | Transform's skew runs from -85 to 85, and this is 86. | yes |
| opacity 101 is refused with a sentence, and nothing changes | Transform's opacity runs from 0 to 100, and this is 101. | yes |
| sampling nearest is refused with a sentence, and nothing changes | Transform's sampling is "bilinear" or "bicubic", and this is "nearest". | yes |
| rotation keyed to 4000 is refused with a sentence, and nothing changes | Transform's rotation runs from -3600 to 3600, and this is 4000. | yes |
| rotation 30, bicubic is taken | taken | yes |
| position keyed from (50, 50) to (100, 50) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_xform_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_022.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_xform_024.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_xform_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_021.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_022.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_025.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_026.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_027.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_028.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_029.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_030.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_031.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_xform_032.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Transform turned 15 degrees: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2016958 pixels changed | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 0, Full | largest difference 1 of 255, 1843 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 100, Full | largest difference 1 of 255, 857 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 239, Full | largest difference 1 of 255, 866 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 100, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform turned 15 degrees on three layers, frame 239, Draft | largest difference 1 of 255, 53 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2031322 pixels changed | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 0, Full | largest difference 1 of 255, 885 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 100, Full | largest difference 1 of 255, 917 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 239, Full | largest difference 1 of 255, 722 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 0, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 100, Draft | largest difference 1 of 255, 75 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform skewed 20 along 30, scaled 80 by 120, bicubic on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 0, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 100, Full | largest difference 1 of 255, 317 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 0, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 100, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Transform moved to (60, 40) round (45, 55) at opacity 70 on three layers, frame 239, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-408 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| as added changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_turned_small.png, rotation 20 at scale 80: the street smaller and tilted clockwise inside the frame, clear corners round it, the frame no bigger; draws cleanly | [], 49496 clear pixels against 0 | yes |
| 3_skew_25_bicubic.png, skew 25, bicubic: the houses lean, the top of the street slid right and the bottom left; draws cleanly | [], 8366 clear pixels | yes |
| 4_opacity_50.png, opacity 50: the street see-through, no pixel more than half covered; draws cleanly | [], the most covered pixel 128 of 255 | yes |
| 5_sliding_sharp.png and 6_sliding_blurred.png, the street sliding right through frame 2, without and with motion blur: the blurred one smeared sideways, its edges softer; both draw cleanly | [] [], sideways contrast 182514 against 287246 | yes |

## Result

184 of 184 checks pass.
