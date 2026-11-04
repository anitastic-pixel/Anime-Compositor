# B-985: Card Wipe's Position Jitter and Rotation Jitter (D-1105)

After Effects' Card Wipe shakes its cards with Position Jitter (X, Y and Z Jitter Amount and Speed) and Rotation Jitter (X, Y and Z Rot Jitter Amount and Speed). Each card is now slid and turned by a smooth random amount that takes a new value Speed times a second; every amount 0, as added, draws as before. Adobe's help publishes no formula, so the rule is ours, stated in `tools/card_wipe_reference.py` (step 2b). Every expected pixel was written by the tool before this code existed. Tolerance 2e-5.

## Card Wipe: FX-CW-070 to 081 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CW-070 frame 0: All six jitter amounts written at 0 with every speed 3: FX-CW-004 exactly, on every frame. | largest difference 1.5e-7 | yes |
| FX-CW-070 frame 4: All six jitter amounts written at 0 with every speed 3: FX-CW-004 exactly, on every frame. | largest difference 1.5e-7 | yes |
| FX-CW-070: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-071 frame 0: X Jitter Amount 0.3 at speed 6, not begun: each card slid sideways by up to 0.3 of a card, by a different amount on each frame. | largest difference 1.5e-7 | yes |
| FX-CW-071 frame 2: X Jitter Amount 0.3 at speed 6, not begun: each card slid sideways by up to 0.3 of a card, by a different amount on each frame. | largest difference 2.1e-7 | yes |
| FX-CW-071 frame 4: X Jitter Amount 0.3 at speed 6, not begun: each card slid sideways by up to 0.3 of a card, by a different amount on each frame. | largest difference 2.1e-7 | yes |
| FX-CW-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-072 frame 0: Y Jitter Amount 0.25 at speed 6 on FX-CW-004, with Z Jitter Amount 0.05 so no two cards lie in one plane. | largest difference 2.1e-7 | yes |
| FX-CW-072 frame 4: Y Jitter Amount 0.25 at speed 6 on FX-CW-004, with Z Jitter Amount 0.05 so no two cards lie in one plane. | largest difference 2.1e-7 | yes |
| FX-CW-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-073 frame 0: Z Jitter Amount 0.5 at speed 6 on FX-CW-004: the cards nearer and farther, so larger and smaller. | largest difference 2.1e-7 | yes |
| FX-CW-073 frame 4: Z Jitter Amount 0.5 at speed 6 on FX-CW-004: the cards nearer and farther, so larger and smaller. | largest difference 2.1e-7 | yes |
| FX-CW-073: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-074 frame 0: X Rot Jitter Amount 20 at speed 6 on FX-CW-004: each card tipped about its x axis. | largest difference 2.2e-7 | yes |
| FX-CW-074 frame 4: X Rot Jitter Amount 20 at speed 6 on FX-CW-004: each card tipped about its x axis. | largest difference 2.0e-7 | yes |
| FX-CW-074: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-075 frame 0: Y Rot Jitter 20 and Z Rot Jitter 15 at speed 6, specular 1, sharpness 20, the light at [20, 30]: the turned cards lit by their own turn. | largest difference 2.5e-7 | yes |
| FX-CW-075 frame 4: Y Rot Jitter 20 and Z Rot Jitter 15 at speed 6, specular 1, sharpness 20, the light at [20, 30]: the turned cards lit by their own turn. | largest difference 1.9e-7 | yes |
| FX-CW-075: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-076 frame 0: All six at once, seed 5: X 0.2, Y 0.2, Z 0.3, the turns 10, 10 and 10, every speed 6. | largest difference 2.2e-7 | yes |
| FX-CW-076 frame 2: All six at once, seed 5: X 0.2, Y 0.2, Z 0.3, the turns 10, 10 and 10, every speed 6. | largest difference 2.0e-7 | yes |
| FX-CW-076 frame 4: All six at once, seed 5: X 0.2, Y 0.2, Z 0.3, the turns 10, 10 and 10, every speed 6. | largest difference 2.1e-7 | yes |
| FX-CW-076: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-077 frame 0: FX-CW-071 at speed 0: the cards slid but still, frame 4 the same as frame 0. | largest difference 1.5e-7 | yes |
| FX-CW-077 frame 4: FX-CW-071 at speed 0: the cards slid but still, frame 4 the same as frame 0. | largest difference 1.5e-7 | yes |
| FX-CW-077: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-078 frame 0: X Jitter Amount keyed 0 at frame 0 to 0.4 at frame 4, speed 1: frame 0 the picture, frame 4 slid. | largest difference 1.5e-7 | yes |
| FX-CW-078 frame 4: X Jitter Amount keyed 0 at frame 0 to 0.4 at frame 4, speed 1: frame 0 the picture, frame 4 slid. | largest difference 1.6e-7 | yes |
| FX-CW-078: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-079 frame 0: X jitter amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-079 frame 4: X jitter amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-079: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-080 frame 0: Z jitter speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-080 frame 4: Z jitter speed 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-080: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-081 frame 0: Y rot jitter amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-081 frame 4: Y rot jitter amount 361, above 360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-081: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_cw_079.json is refused in a sentence naming x jitter amount | Card Wipe's x jitter amount runs from 0 to 100, and this is -1. | yes |
| fx_cw_080.json is refused in a sentence naming z jitter speed | Card Wipe's z jitter speed runs from 0 to 10, and this is 11. | yes |
| fx_cw_081.json is refused in a sentence naming y rot jitter amount | Card Wipe's y rot jitter amount runs from 0 to 360, and this is 361. | yes |
| fx_cw_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_075.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_076.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_077.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_078.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_079.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_080.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_081.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_004.json, from before D-1105, opens with no jitter and is saved without the twelve settings | [] | yes |
| fx_cw_071.json is saved with the X jitter it moved and nothing else of the twelve | x_jitter_amount 0.3; x_jitter_speed 6; y_jitter_amount -; y_jitter_speed -; z_jitter_amount -; z_jitter_speed -; x_rot_jitter_amount -; x_rot_jitter_speed -; y_rot_jitter_amount -; y_rot_jitter_speed -; z_rot_jitter_amount -; z_rot_jitter_speed - | yes |

## Card Wipe on the card: FX-CW-070 to 078, frames 0 to 4 at Full and Draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cw_070.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_071.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_072.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_073.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_074.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_075.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_076.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_077.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_078.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |

## Pictures (verification/B-985 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_position_jitter.png (frame 4): drawn without a warning, different from no jitter and from its own frame 0 | 60769 pixels differ from no jitter, 66967 from frame 0; warnings [] | yes |
| 2_depth_jitter.png (frame 4): drawn without a warning, different from no jitter and from its own frame 0 | 44296 pixels differ from no jitter, 50154 from frame 0; warnings [] | yes |
| 3_rotation_jitter.png (frame 4): drawn without a warning, different from no jitter and from its own frame 0 | 89003 pixels differ from no jitter, 87870 from frame 0; warnings [] | yes |

## Result

67 of 67 checks pass.
