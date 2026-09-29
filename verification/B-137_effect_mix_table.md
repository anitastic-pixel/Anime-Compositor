# B-137: Mix on every effect

D-202, accepted on 2026-09-28 with the After Effects picks (A12). Every expected pixel is `Fixtures/effect_mix/expected_effect_mix.json`, written by `tools/effect_mix_reference.py` before this code existed and printed in document 25 as FX-MIX-001 to 017. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MIX-001 to 017 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIX-001 frame 0: Invert, channel rgb, amount 100, at mix 50: every sample of the drawing half way between its own value and its negative's, in linear light, so black and white both land on the same light grey, 0.5 in linear light, #bcbcbc but for rounding (187.5), and the skin #f6d6be on #b59f91; each pixel's covering is kept. It is not Invert at amount 50, FX-INVERT-003, which meets in the encoded middle, #808080. | largest difference 9.2e-8 | yes |
| FX-MIX-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-002 frame 0: The same Invert with no mix written, as every file before D-202: read as 100, so the frame is FX-INVERT-001 exactly. | largest difference 4.8e-8 | yes |
| FX-MIX-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-003 frame 0: Mix 100 written: the same as no mix, FX-MIX-002. | largest difference 4.8e-8 | yes |
| FX-MIX-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-004 frame 0: Mix 0: the drawing, exactly, as if the Invert were switched off. | largest difference 1.9e-7 | yes |
| FX-MIX-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-005 frame 0: Mix 25: a quarter of the way from the drawing to its negative. | largest difference 1.7e-7 | yes |
| FX-MIX-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-006 frame 0: Gaussian Blur, sigma 1, edges transparent, at mix 50: the layer grows by 3 pixels on each side, and the sharp drawing is laid back in where it was, not at the grown layer's corner: each pixel half the blurred drawing and half the sharp one, so the empty rows and column round the drawing take half the blur's spill. | largest difference 1.8e-7 | yes |
| FX-MIX-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-007 frame 0: Mix keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 FX-MIX-001 and frame 4 FX-MIX-002. | largest difference 1.9e-7 | yes |
| FX-MIX-007 frame 2: Mix keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 FX-MIX-001 and frame 4 FX-MIX-002. | largest difference 9.2e-8 | yes |
| FX-MIX-007 frame 4: Mix keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 FX-MIX-001 and frame 4 FX-MIX-002. | largest difference 4.8e-8 | yes |
| FX-MIX-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-008 frame 0: Mix eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-MIX-002, as is frame 4; frame 0 is the drawing. | largest difference 1.9e-7 | yes |
| FX-MIX-008 frame 2: Mix eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-MIX-002, as is frame 4; frame 0 is the drawing. | largest difference 4.8e-8 | yes |
| FX-MIX-008 frame 4: Mix eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, and is FX-MIX-002, as is frame 4; frame 0 is the drawing. | largest difference 4.8e-8 | yes |
| FX-MIX-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-009 frame 0: Invert at mix 50, then Exposure -1 at no mix: the Exposure takes the mixed picture, so the frame is FX-MIX-001 at half its light. | largest difference 4.6e-8 | yes |
| FX-MIX-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-010 frame 0: Exposure -1, then Exposure +1 at mix 50: the second is mixed with what it was given, the darkened drawing, not with the drawing, so each colour is three quarters of the drawing's, not all of it. | largest difference 1.6e-7 | yes |
| FX-MIX-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-011 frame 0: Invert at mix 50, switched off: the drawing. | largest difference 1.9e-7 | yes |
| FX-MIX-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-012 frame 0: Invert at mix 50 on an adjustment layer above the drawing: the frame beneath is the drawing alone, so this is FX-MIX-001. | largest difference 9.2e-8 | yes |
| FX-MIX-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-013 frame 0: Light Wrap, as it starts, at mix 50, on Light Wrap's box over its bright bands: the box takes half the light FX-WRAP-001 lays on it, and the bands round it are untouched. | largest difference 1.9e-7 | yes |
| FX-MIX-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MIX-014 frame 0: Invert at mix 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-014 frame 4: Invert at mix 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIX-015 frame 0: Invert at mix -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-015 frame 4: Invert at mix -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIX-016 frame 0: Invert with its mix keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-016 frame 4: Invert with its mix keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-MIX-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-MIX-017 frame 1: Posterize Time, 12 a second, at mix 50: it holds the layer in time and has no picture to mix, so its mix must be 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-MIX-017 frame 3: Posterize Time, 12 a second, at mix 50: it holds the layer in time and has no picture to mix, so its mix must be 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-MIX-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mix_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_mix_003.json, Mix 100 written, is saved as FX-MIX-002 is, with no Mix: a plain 100 is never written | its layers the same as fx_mix_002.json's saved: true | yes |
| a file with a Mix written as a word is refused as a fault in its shape | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mix that is true is refused as a fault in its shape | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mix with an expression is refused as a fault in its shape | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Mix whose base is a word is refused as a fault in its shape | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| an effect this build does not have, with a Mix of 30, opened and saved holds what it held, the Mix included | the same | yes |
| an effect this build does not have, with a Mix keyed from 30 to 150, opened and saved holds what it held, the Mix included | the same | yes |
| every other project in Fixtures/ that opens, opened and saved, writes a Mix on no effect that did not have one: every file before D-202 is saved as it was | 2101 files, 1996 effects, 0 gained a Mix [] | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Mix 101 is refused with a sentence, and nothing changes | Mix is 0 to 100 per cent, and this is 101. | yes |
| Mix -1 is refused with a sentence, and nothing changes | Mix is 0 to 100 per cent, and this is -1. | yes |
| Mix not a number is refused with a sentence, and nothing changes | Mix is 0 to 100 per cent, and this is NaN. | yes |
| Mix keyed from 0 to 150 is refused with a sentence, and nothing changes | Mix is 0 to 100 per cent, and this is 150. | yes |
| Mix keyed with two numbers a key is refused with a sentence, and nothing changes | The keys of mix cannot be used. | yes |
| Mix on an effect the layer does not have is refused with a sentence, and nothing changes | There is no effect called fx-9-9 on this layer. | yes |
| fx_mix_002.json, the Invert at 100, set to Mix 50 draws FX-MIX-001, and the step to undo says what it was | largest difference 9.2e-8; "Set Invert's Mix to 50%" | yes |
| a drag of the Mix from 100 through 80 and 60 to 40 is one step to undo, and undoing it gives back the frame at 100 | steps added 1; frame back: true | yes |
| Mix 50, is taken | taken | yes |
| Mix keyed from 0 to 100, is taken | taken | yes |
| Mix 0, with keys on it, is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |
| Mix 100 on Posterize Time is refused with a sentence: it has no Mix | A core.posterize_time has no Mix. | yes |
| Mix 50 on Posterize Time is refused with a sentence: it has no Mix | A core.posterize_time has no Mix. | yes |
| keys on Posterize Time's Mix is refused with a sentence: it has no Mix | A core.posterize_time has no setting called mix. | yes |
| Mix 50 on an effect this build does not have is refused with a sentence: it is kept as it was written | A vendor.soft_focus has no Mix. | yes |

## The preview's memory and the graphics card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mix_006.json, the Gaussian Blur at Mix 50, drawn by the preview, then set to 100 and drawn again with what the preview remembers: the second is the blur at 100, not the remembered 50 | same as a fresh 100: true; same as the 50: false | yes |
| the Gaussian Blur at Mix 50 is drawn by the CPU, not left to the card, which does not mix yet; at 100 it is left to the card as before | left to the card at 50: 0; at 100: 1 | yes |
| fx_mix_006.json, the Gaussian Blur at Mix 50, through the card: the CPU blurs and mixes, the card lays it, no message, within 1 level of the CPU's frame | []; largest difference 1 of 255 | yes |
| fx_mix_013.json, the Light Wrap at Mix 50, through the card: the CPU draws the whole frame, says so in the warning panel, and it is the CPU's frame exactly | ["GPU_PREVIEW_ON_CPU The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet."]; largest difference 0 of 255 | yes |
| a Gaussian Blur with no Mix written, through the card: drawn by the card as before, no message, within 1 level of the CPU's frame | []; largest difference 0 of 255 | yes |

## Pictures: a street, in `verification/B-137 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| invert_mix.png, an Invert at Mix 0, 25, 50, 75 and 100, left to right: the street, fading to its negative; 0 is the street exactly and 100 the Invert with no Mix exactly; every sample of each on the straight line between the two, in linear light, within 1e-6 | 0 is the street: true; 100 is the Invert: true; furthest off the line 3.0e-8 | yes |
| blur_mix.png, a Gaussian Blur of 6 pixels at Mix 0, 50 and 100: the sharp street, a soft glow of the blur laid half over the sharp drawing, the blur; every sample of the middle half-way between the other two, within 1e-6 | 0 is the street: true; 100 is the blur: true; furthest off 0.0e0 | yes |
| invert_fade_frames.png, frames 0 to 4 of an Invert whose Mix is keyed from 0 at frame 0 to 100 at frame 4: the effect fading in, each frame exactly the panel of invert_mix.png with the Mix it has there, 0, 25, 50, 75 and 100 | every frame its panel: true | yes |
| every picture draws cleanly | [] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mix_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix_007.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

93 of 93 checks pass.
