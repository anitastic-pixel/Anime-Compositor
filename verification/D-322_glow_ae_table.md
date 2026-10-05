# D-322: Glow in After Effects' own numbers

From D-308, held back for want of an After Effects frame, then settled on 2026-10-04 from the Creative COW thread "Glow Effect and transparent background mechanics", where After Effects' Glow was measured on white shapes: the radius is a Gaussian Blur (here Blurriness, D-321), and the glow's brightness is GI*(GT/100) + GI*16*(1-GT/100), its covering untouched by intensity. Corrected by D-331 on 2026-10-05 from tutorials 2 and 3: that brightness is the colour read straight, so the colour is at the intensity and the covering divided by t + 16 (1 - t). Glow gains Units: After Effects, which a new one takes, and Classic, what a file without units means, D-89's rule as before. Every expected pixel is `Fixtures/glow_ae/expected_glow_ae.json`, written by `tools/glow_ae_reference.py` before the build had it, printed in document 25 as FX-GLOW-AE-001 to 010. Tolerance 2e-5.

## FX-GLOW-AE-001 to 010 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GLOW-AE-001 frame 0: Units after_effects with After Effects' defaults, threshold 60, radius 10, intensity 1, Add: spread as Gaussian Blur at Blurriness 10 (sigma 3), the glow's colour the blurred light, its covering a seventh of the blurred covering (0.6 + 16 x 0.4), so read straight it is seven times the light. | largest difference 2.6e-7 | yes |
| FX-GLOW-AE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-002 frame 0: The same settings in a file without units: D-89's rule as before, FX-GLOW-014 exactly. | largest difference 2.0e-7 | yes |
| FX-GLOW-AE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-003 frame 0: Units written "classic": FX-GLOW-AE-002 exactly. | largest difference 2.0e-7 | yes |
| FX-GLOW-AE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-004 frame 0: Tutorial 2's kind of setting, threshold 0, radius 4, intensity 0.1: every pixel that shows glows, its colour a tenth of the blurred light, its covering a sixteenth: read straight, 1.6 times the light. | largest difference 1.9e-7 | yes |
| FX-GLOW-AE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-005 frame 0: Intensity 0: the drawing, untouched. | largest difference 1.5e-7 | yes |
| FX-GLOW-AE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-006 frame 0: Radius 0: nothing spreads, and each glowing pixel's colour is added onto itself once. | largest difference 3.1e-7 | yes |
| FX-GLOW-AE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-007 frame 0: Operation Screen: the glow is held inside 0 to 1 and screened on. | largest difference 1.8e-7 | yes |
| FX-GLOW-AE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-008 frame 0: Tint #ff4000, threshold 0, intensity 0.2: the glow is orange, read straight 3.2 times the tint. | largest difference 1.8e-7 | yes |
| FX-GLOW-AE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GLOW-AE-009 frame 0: Units "After_Effects": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-AE-009 frame 4: Units "After_Effects": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-AE-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GLOW-AE-010 frame 0: Units "ae", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-AE-010 frame 4: Units "ae", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-GLOW-AE-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_ae_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_ae_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_ae_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_ae_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_ae_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_glow_ae_002.json, a glow with no units as before D-322, is saved without units | {"based_on":"bright","colors":[],"intensity":1,"operation":"add","radius":10,"threshold":60,"tint":"","tolerance":0} | yes |
| A glow in After Effects units is saved as units: after_effects | {"based_on":"bright","colors":[],"intensity":1,"operation":"add","radius":10,"threshold":60,"tint":"","tolerance":0,"units":"after_effects"} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| units "ae" is refused with a sentence, and nothing changes | Glow's units are "classic" or "after_effects", and this is "ae". | yes |
| units "After_Effects", written with capitals is refused with a sentence, and nothing changes | Glow's units are "classic" or "after_effects", and this is "After_Effects". | yes |
| FX-GLOW-AE-002 set to After Effects units, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-GLOW-AE-002 set to After Effects units by the command draws FX-GLOW-AE-001's frame | largest difference 2.6e-7 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_ae_001.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_glow_ae_002.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_glow_ae_004.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_glow_ae_006.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_glow_ae_007.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_glow_ae_008.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |

## Pictures: a white square, in `verification/D-322 pictures/`, over black

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_square.png, the square with no glow; draws cleanly | [], 6 pixels right of the square 0 of 255, 20 pixels right 0 | yes |
| 2_old_glow_defaults.png, Glow with After Effects' defaults (threshold 60, radius 10, intensity 1) in an older project (Classic); draws cleanly | [], 6 pixels right of the square 62 of 255, 20 pixels right 0 | yes |
| 3_ae_glow_defaults.png, the same in After Effects units: as bright over black, a little tighter (D-331); draws cleanly | [], 6 pixels right of the square 51 of 255, 20 pixels right 0 | yes |
| 4_old_tutorial_glow.png, tutorial 2's first Glow (threshold 0, radius 39, intensity 0.1) in Float, Classic: barely there; draws cleanly | [], 6 pixels right of the square 40 of 255, 20 pixels right 14 | yes |
| 5_ae_tutorial_glow.png, the same in After Effects units: as faint over black, as tutorial 2's After Effects frames show (D-331); draws cleanly | [], 6 pixels right of the square 41 of 255, 20 pixels right 11 | yes |
| Over black, After Effects units glow as bright as Classic near the square (within 12 of 255; D-322 had them far brighter) | defaults 51 against 62; tutorial 41 against 40 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_glow_ae_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glow_ae_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_glow_ae_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

49 of 49 checks pass.
