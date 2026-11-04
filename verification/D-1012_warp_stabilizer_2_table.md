# B-892: Warp Stabilizer's Crop Less <-> Smooth More

D-1012, wave 12 (lane w12u, the Warp Stabilizer tutorial at 5:00). Every expected pixel is `Fixtures/warp_stabilizer_2/expected_warp_stabilizer_2.json`, written by `tools/warp_stabilizer_2_reference.py` before this code existed and printed in document 25 as FX-WS2-001 to 010; tolerance 2e-5.

## FX-WS2-001 to 010 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WS2-001 frame 2: As added, Crop Less <-> Smooth More 50: steadied half as far as D-653's, so cut and scaled up less (FX-WS-001 at 100). | largest difference 1.9e-7 | yes |
| FX-WS2-001 frame 3: As added, Crop Less <-> Smooth More 50: steadied half as far as D-653's, so cut and scaled up less (FX-WS-001 at 100). | largest difference 1.9e-7 | yes |
| FX-WS2-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-002 frame 2: At 100: D-653's own picture, FX-WS-001. | largest difference 2.4e-7 | yes |
| FX-WS2-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-003 frame 2: At 0: not moved, so nothing cut or scaled: the drawing as it is. | largest difference 1.6e-7 | yes |
| FX-WS2-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-004 frame 2: At 50, Stabilize, Crop: fewer pixels cut away than FX-WS-003's. | largest difference 2.1e-7 | yes |
| FX-WS2-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-005 frame 2: At 50 with Stabilize Only: the setting does nothing, FX-WS-002. | largest difference 2.0e-7 | yes |
| FX-WS2-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-006 frame 3: At 50 with No Motion: the setting does nothing, No Motion's own picture. | largest difference 2.5e-7 | yes |
| FX-WS2-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-007 frame 2: Keyed from 0 at frame 0 to 100 at frame 4: 50 at frame 2, FX-WS2-001's frame 2. | largest difference 1.9e-7 | yes |
| FX-WS2-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-008 frame 2: A D-653 file without the setting: read as 100, FX-WS-001's picture. | largest difference 2.4e-7 | yes |
| FX-WS2-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WS2-009 frame 0: Crop Less <-> Smooth More 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS2-009 frame 4: Crop Less <-> Smooth More 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS2-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WS2-010 frame 0: Crop Less <-> Smooth More -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.6e-7 | yes |
| FX-WS2-010 frame 4: Crop Less <-> Smooth More -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-WS2-010: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ws2_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ws2_008.json, a file from before D-1012, saves without the setting | None | yes |
| fx_ws2_009.json is refused in a sentence naming the setting | Warp Stabilizer's crop less smooth more runs from 0 to 100, and this is 101. | yes |
| fx_ws2_010.json is refused in a sentence naming the setting | Warp Stabilizer's crop less smooth more runs from 0 to 100, and this is -1. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Crop Less <-> Smooth More 101 is refused with a sentence, and nothing changes | Warp Stabilizer's crop less smooth more runs from 0 to 100, and this is 101. | yes |
| Crop Less <-> Smooth More 0 is taken | taken | yes |
| Crop Less <-> Smooth More keyed from 0 at frame 0 to 100 at frame 4 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: the shaken striped scene, Stabilize, Crop, in `verification/D-1012 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_smooth_more_100_frame_4.png, 100: draws cleanly; night-blue pixels at frame 4, and how much frames 3 and 4 differ | [] [], 4048 pixels, 1.9 levels | yes |
| 2_as_added_50_frame_4.png, 50: draws cleanly; night-blue pixels at frame 4, and how much frames 3 and 4 differ | [] [], 2032 pixels, 9.7 levels | yes |
| 3_crop_less_0_frame_4.png, 0: draws cleanly; night-blue pixels at frame 4, and how much frames 3 and 4 differ | [] [], 0 pixels, 17.6 levels | yes |
| lower cuts less: fewer night-blue pixels at 50 than at 100, none at 0 | 4048 > 2032 > 0 | yes |
| lower steadies less: frames 3 and 4 differ more at 50 than at 100, and most at 0 | 1.9 < 9.7 < 17.6 | yes |

## Result

43 of 43 checks pass.
