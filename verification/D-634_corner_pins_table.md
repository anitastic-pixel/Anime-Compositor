# D-634: the Corner Pins camera

B-514, After Effects' Corner Pins camera system for Card Wipe, Card Dance and Shatter, under the coordinator's approval of the D-634 design for the owner's /loop request. The four pins say where the layer's corners land; the camera that puts them there is worked out from them, with Focal Length as the lens or, with Auto Focal Length on, the lens the pins themselves imply where it lies within 1 to 1000 mm. The light stays where it is on the layer. Adobe's help publishes no formula, so the rule is ours, stated in `tools/card_dance_reference.py` (step 4). Every expected pixel was written by the tools before this code existed. Tolerance 2e-5.

## Card Wipe: FX-CW-057 to 069 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CW-057 frame 0: Camera system "corner_pins" with the pins as added: FX-CW-004 exactly. | largest difference 1.5e-7 | yes |
| FX-CW-057: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-058 frame 0: Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the lower right up to [100, 95], Auto Focal Length off: FX-CW-004 seen at a slant. | largest difference 2.1e-7 | yes |
| FX-CW-058: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-059 frame 0: FX-CW-058 with Auto Focal Length on: the lens worked out from the pins. | largest difference 2.1e-7 | yes |
| FX-CW-059: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-060 frame 0: Corner Pins on a parallelogram, Auto Focal Length on: no perspective to work a lens from, so the focal length is used. | largest difference 2.1e-7 | yes |
| FX-CW-060: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-061 frame 0: FX-CW-060 with Auto Focal Length off: the same picture. | largest difference 2.1e-7 | yes |
| FX-CW-061: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-062 frame 0: Corner Pins on a general four-sided shape, Auto Focal Length on, a point light at [20, 30] depth 0.3, specular 1, sharpness 20: the light stays on the layer. | largest difference 1.2e-7 | yes |
| FX-CW-062: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-063 frame 0: FX-CW-059 with the camera turned 30 about y, at x 40, y 50, Z position 1.7: Corner Pins does not use them, so FX-CW-059. | largest difference 2.1e-7 | yes |
| FX-CW-063: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-064 frame 0: Camera system "camera_position" written out: FX-CW-004. | largest difference 1.5e-7 | yes |
| FX-CW-064: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-065 frame 0: Corner Pins with the upper right corner keyed [100, 0] at frame 0 to [80, 20] at frame 4: frame 0 FX-CW-004, frame 4 pulled in. | largest difference 1.5e-7 | yes |
| FX-CW-065 frame 4: Corner Pins with the upper right corner keyed [100, 0] at frame 0 to [80, 20] at frame 4: frame 0 FX-CW-004, frame 4 pulled in. | largest difference 1.8e-7 | yes |
| FX-CW-065: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CW-066 frame 0: Camera system "comp_camera", not built for Card Wipe (a gap). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-066 frame 4: Camera system "comp_camera", not built for Card Wipe (a gap). The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-066: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-067 frame 0: Camera system "orbit". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-067 frame 4: Camera system "orbit". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-067: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-068 frame 0: Upper left corner [-1001, 0], below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-068 frame 4: Upper left corner [-1001, 0], below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-068: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CW-069 frame 0: Auto focal length "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-069 frame 4: Auto focal length "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CW-069: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_cw_066.json is refused in a sentence naming camera system | Card Wipe's camera system is one of "camera_position", "corner_pins", and this is "comp_camera". | yes |
| fx_cw_067.json is refused in a sentence naming camera system | Card Wipe's camera system is one of "camera_position", "corner_pins", and this is "orbit". | yes |
| fx_cw_068.json is refused in a sentence naming upper left corner | Card Wipe's upper left corner runs from -1000 to 1000, and this is -1001. | yes |
| fx_cw_069.json is refused in a sentence naming auto focal length | Card Wipe's auto focal length is one of "off", "on", and this is "maybe". | yes |
| fx_cw_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cw_004.json, from before D-634, opens with the pins as added and is saved without them | [] | yes |
| fx_cw_058.json is saved with Corner Pins and the three pins it moved, the lower left pin it left alone not written | camera_system "corner_pins"; upper_left_corner [20,10]; upper_right_corner [85,0]; lower_left_corner -; lower_right_corner [100,95]; auto_focal_length - | yes |

## Card Wipe on the card: FX-CW-057 to 065, frames 0 to 4 at Full and Draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cw_057.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_058.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_059.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_060.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_061.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_062.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_063.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_064.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_cw_065.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |

## Card Dance: FX-CD-065 to 076 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CD-065 frame 0: Corner Pins with the pins as added, three turns with the back layer: FX-CD-014 exactly. | largest difference 1.4e-7 | yes |
| FX-CD-065: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-066 frame 0: Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the lower right up to [100, 95], Auto Focal Length off: the layer seen leaning back and turned, the nearer cards larger. | largest difference 2.7e-7 | yes |
| FX-CD-066: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-067 frame 0: FX-CD-066 with Auto Focal Length on: the focal length worked out from the pins. | largest difference 3.1e-7 | yes |
| FX-CD-067: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-068 frame 0: Corner Pins turned and slanted with no perspective (a parallelogram), Auto Focal Length on: no focal length can be worked out, so Focal Length's. | largest difference 2.0e-7 | yes |
| FX-CD-068: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-069 frame 0: FX-CD-068 with Auto Focal Length off: the same. | largest difference 2.0e-7 | yes |
| FX-CD-069: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-070 frame 0: Corner Pins on a general four-sided shape, Auto Focal Length on, three turns with the back layer. | largest difference 1.4e-7 | yes |
| FX-CD-070: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-071 frame 0: FX-CD-067 with a point light at the top left, depth 0.3, specular 1, sharpness 20: the light stays on the layer. | largest difference 6.8e-8 | yes |
| FX-CD-071: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-072 frame 0: FX-CD-067 with the camera turned 30 about y, at x 40 per cent and Z position 3: Corner Pins does not use them, so FX-CD-067. | largest difference 3.1e-7 | yes |
| FX-CD-072: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-073 frame 0: The upper right pin keyed from [100, 0] at frame 0 to [80, 20] at frame 4: frame 0 FX-CD-003. | largest difference 2.1e-7 | yes |
| FX-CD-073 frame 4: The upper right pin keyed from [100, 0] at frame 0 to [80, 20] at frame 4: frame 0 FX-CD-003. | largest difference 2.1e-7 | yes |
| FX-CD-073: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-074 frame 0: Upper left corner [-1001, 0], below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-074 frame 4: Upper left corner [-1001, 0], below -1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-074: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-075 frame 0: Auto focal length "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-075 frame 4: Auto focal length "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-075: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-076 frame 0: Lower right corner keyed to [100, 1200] at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-076 frame 4: Lower right corner keyed to [100, 1200] at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-076: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_cd_074.json is refused in a sentence naming upper left corner | Card Dance's upper left corner runs from -1000 to 1000, and this is -1001. | yes |
| fx_cd_075.json is refused in a sentence naming auto focal length | Card Dance's auto focal length is one of "off", "on", and this is "maybe". | yes |
| fx_cd_076.json is refused in a sentence naming lower right corner | The layer "holder" has a core.card_dance whose settings this build cannot use. Card Dance's lower right corner runs from -1000 to 1000, and this is 1200. The effect is kept and bypassed. | yes |
| fx_cd_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_075.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_076.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_040.json, from before D-634, opens with the pins as added and is saved without them | [] | yes |

## Shatter: FX-SH-079 to 083 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SH-079 frame 3: Corner Pins with the pins as added, thrown from the middle: FX-SH-052 exactly. | largest difference 1.8e-7 | yes |
| FX-SH-079: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-080 frame 3: Corner Pins, the upper corners pulled in to [20, 10] and [85, 0] and the lower right up to [100, 95], Auto Focal Length on, thrown from the middle. | largest difference 1.9e-7 | yes |
| FX-SH-080: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-081 frame 3: Corner Pins on a general four-sided shape, Auto Focal Length off, the pieces tumbling about y, with a point light at the top left, specular 1: the light stays on the layer. | largest difference 1.8e-7 | yes |
| FX-SH-081: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-082 frame 0: Lower right corner [50, 1001], above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-082 frame 4: Lower right corner [50, 1001], above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-082: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-083 frame 0: Auto focal length "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-083 frame 4: Auto focal length "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-083: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_sh_082.json is refused in a sentence naming lower right corner | Shatter's lower right corner runs from -1000 to 1000, and this is 1001. | yes |
| fx_sh_083.json is refused in a sentence naming auto focal length | Shatter's auto focal length is one of "off", "on", and this is "yes". | yes |
| fx_sh_079.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_080.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_081.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_082.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_083.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_001.json, from before D-634, opens with the pins as added and is saved without them | [] | yes |

## Pictures (verification/D-634 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_card_wipe_pulled_in.png: drawn without a warning, different from the camera as added | 80529 pixels differ from the same frame seen by the camera as added; warnings [] | yes |
| 2_card_wipe_pulled_in_auto_lens.png: drawn without a warning, different from the camera as added | 79047 pixels differ from the same frame seen by the camera as added; warnings [] | yes |
| 3_card_wipe_four_sided_point_light.png: drawn without a warning, different from the camera as added | 101754 pixels differ from the same frame seen by the camera as added; warnings [] | yes |
| 4_card_dance_pulled_in_auto_lens.png: drawn without a warning, different from the camera as added | 100485 pixels differ from the same frame seen by the camera as added; warnings [] | yes |
| 5_card_dance_four_sided.png: drawn without a warning, different from the camera as added | 96693 pixels differ from the same frame seen by the camera as added; warnings [] | yes |
| 6_shatter_pulled_in_auto_lens.png: drawn without a warning, different from the camera as added | 117941 pixels differ from the same frame seen by the camera as added; warnings [] | yes |

## Result

129 of 129 checks pass.
