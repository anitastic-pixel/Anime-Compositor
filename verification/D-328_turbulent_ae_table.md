# D-328: Turbulent Displace in After Effects' units

From P-26's tutorial 2, where After Effects' Turbulent Displace at Amount 80, Size 2 leaves a glow smooth with a fine shimmer and D-127's rule breaks it into dust. No source found says how After Effects scales Amount with Size, so this program reads it as a push of amount x min(size, 100) / 100 pixels: at Size 100 and above it is D-127's, at Size 2 Amount 80 pushes 1.6. Turbulent Displace gains Units: After Effects, which a new one takes, and Classic, what a file without units means, D-127's rule as before. Every expected pixel is `Fixtures/turbulent_ae/expected_turbulent_ae.json`, written by `tools/turbulent_ae_reference.py` before the build had it, printed in document 25 as FX-TURB-AE-001 to 012. Tolerance 2e-5.

## FX-TURB-AE-001 to 012 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURB-AE-001 frame 0: Units after_effects, amount 80, size 2, speed 0, tutorial 2's setting: the push is 80 x 2 / 100 = 1.6 pixels at most, so every pixel reads from within two pixels of itself and the stripes stay stripes, shimmering. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-002 frame 0: The same settings in a file without units: D-127's rule as before, a push of up to 80 pixels at a wave every two, so each pixel reads from far off, mostly past the drawing: it breaks into dust, and in this small frame only two specks are left. | largest difference 1.7e-7 | yes |
| FX-TURB-AE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-003 frame 0: Units written "classic": FX-TURB-AE-002 exactly. | largest difference 1.7e-7 | yes |
| FX-TURB-AE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-004 frame 0: Units after_effects with the settings as they start, amount 10, size 60, speed 20: the push is 6 pixels at most, D-127's amount 6, size 60, on every frame. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-004 frame 2: Units after_effects with the settings as they start, amount 10, size 60, speed 20: the push is 6 pixels at most, D-127's amount 6, size 60, on every frame. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-004 frame 4: Units after_effects with the settings as they start, amount 10, size 60, speed 20: the push is 6 pixels at most, D-127's amount 6, size 60, on every frame. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-005 frame 0: Units after_effects, amount 3, size 1000, speed 0: past size 100 the push is the amount, so this is FX-TURB-009 exactly. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-006 frame 0: Units after_effects, amount 3, size 100, speed 0: at size 100 the push is the amount, D-127's amount 3, size 100. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-007 frame 0: Units after_effects, amount 30, size 8, speed 0: a push of 2.4 pixels at most, D-127's amount 2.4 at size 8; the layer grows by 3. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-008 frame 0: Units after_effects, size 8, speed 0, amount keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing, frame 2 amount 20 (a push of 1.6), frame 4 amount 40 (3.2). | largest difference 1.9e-7 | yes |
| FX-TURB-AE-008 frame 2: Units after_effects, size 8, speed 0, amount keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing, frame 2 amount 20 (a push of 1.6), frame 4 amount 40 (3.2). | largest difference 2.5e-7 | yes |
| FX-TURB-AE-008 frame 4: Units after_effects, size 8, speed 0, amount keyed from 0 at frame 0 to 40 at frame 4, linear: frame 0 is the drawing, frame 2 amount 20 (a push of 1.6), frame 4 amount 40 (3.2). | largest difference 2.5e-7 | yes |
| FX-TURB-AE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-009 frame 0: FX-TURB-AE-001 with edges repeat: nothing grows, and a push past the drawing's edge reads the nearest edge pixel. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-010 frame 0: FX-TURB-AE-007 with seed 8, moved three pixels right: the layer grows by ceil(2.4) = 3, so the three columns left of the drawing show the stripes pushed into them. | largest difference 2.5e-7 | yes |
| FX-TURB-AE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-AE-011 frame 0: Units "After_Effects": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-AE-011 frame 4: Units "After_Effects": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-AE-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-AE-012 frame 0: Units "ae", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-AE-012 frame 4: Units "ae", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-AE-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The rule

| Check | The build's answer | Matches |
| --- | --- | --- |
| Amount 80, size 2, after_effects: the push is 1.6 pixels | 1.6 | yes |
| Amount 80, size 2, classic: the push is 80 pixels | 80 | yes |
| Amount 3, size 100, after_effects: the push is 3 pixels | 3 | yes |
| Amount 3, size 1000, after_effects: the push is 3 pixels | 3 | yes |
| Amount 1000, size 1000, after_effects: the push is 1000 pixels | 1000 | yes |
| A half-size draft of amount 80, size 2, After Effects units, pushes half of 1.6 (its size held at 1) | amount 80, size 1, push 0.8 | yes |
| A half-size draft in Classic units pushes 40, as before D-328 | amount 40 | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_ae_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ae_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ae_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ae_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ae_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ae_002.json, a Turbulent Displace with no units as before D-328, is saved without units | {"amount":80,"complexity":2,"edges":"transparent","evolution":0,"seed":0,"size":2,"speed":0} | yes |
| One in After Effects units is saved as units: after_effects | {"amount":80,"complexity":2,"edges":"transparent","evolution":0,"seed":0,"size":2,"speed":0,"units":"after_effects"} | yes |
| One whose file says units: classic keeps saying it | {"amount":80,"complexity":2,"edges":"transparent","evolution":0,"seed":0,"size":2,"speed":0,"units":"classic"} | yes |
| FX-TURB-001's file, from before D-328, is saved without units | {"amount":10,"complexity":2,"edges":"transparent","evolution":0,"seed":0,"size":60,"speed":20} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| units "ae" is refused with a sentence, and nothing changes | Turbulent Displace's units are "classic" or "after_effects", and this is "ae". | yes |
| units "After_Effects", written with capitals is refused with a sentence, and nothing changes | Turbulent Displace's units are "classic" or "after_effects", and this is "After_Effects". | yes |
| FX-TURB-AE-002 set to After Effects units, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-TURB-AE-002 set to After Effects units by the command draws FX-TURB-AE-001's frame | largest difference 2.5e-7 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_ae_001.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_002.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_004.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_005.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_007.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_009.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_turb_ae_010.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |

## Pictures: a soft glow, in `verification/D-328 pictures/`, over black

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_glow.png, the glow with no warp; draws cleanly | [], grain 1.83 | yes |
| 2_classic_amount80_size2.png, Amount 80, Size 2 in an older project (Classic): broken into dust; draws cleanly | [], grain 34.79 | yes |
| 3_ae_amount80_size2.png, the same in After Effects units: the glow stays smooth, a fine shimmer at its edge; draws cleanly | [], grain 2.06 | yes |
| 4_ae_amount80_size100.png, Amount 80, Size 100 in After Effects units: Classic's rule, the whole glow carried and bent; draws cleanly | [], grain 1.52 | yes |
| After Effects units at Amount 80, Size 2 stay close to the plain glow; Classic does not | mean difference from the plain glow: After Effects 1.26, Classic 50.46 levels of 255 | yes |
| Classic is grainy (dust); After Effects units are about as smooth as the plain glow | grain: plain 1.83, After Effects 2.06, Classic 34.79 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_ae_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ae_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ae_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

67 of 67 checks pass.
