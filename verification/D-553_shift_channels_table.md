# D-553: Shift Channels, its fixture folder

B-433: `core.shift_channels` (D-305, built in B-190 from P-26's Colorful Glitch tutorial) takes each of a drawing's alpha, red, green and blue from a channel or a measure of its colour, after After Effects' Shift Channels (Channel). B-190 pinned it on hand-worked solids; this unit gives it a fixture folder. Every expected pixel is `Fixtures/shift_channels/expected_shift_channels.json`, written by `tools/shift_channels_reference.py` before this code existed and printed in document 25 as FX-SHIFT-001 to 020. No code changed. Tolerance 2e-5.

## FX-SHIFT-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHIFT-001 frame 0: The settings as they start, alpha from alpha, red from red, green from green, blue from blue: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-SHIFT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-002 frame 0: Red from blue and blue from red, the two swapped: the red swatch turns blue, #2828c8, the blue turns #ff6040, the skin a pale blue, #bed6f6; every grey stays as it is, and every covering is kept. | largest difference 1.9e-7 | yes |
| FX-SHIFT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-003 frame 0: Red, green and blue all from green: each pixel turns the grey of its own green: the red #282828, the green #b4b4b4, the skin #d6d6d6. | largest difference 5.5e-8 | yes |
| FX-SHIFT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-004 frame 0: Red, green and blue from luminance: each pixel turns the grey of its encoded luma, 0.2126 red + 0.7152 green + 0.0722 blue: the red 74.016 of 255, the green 147.268; white, grey and black stay as they are. | largest difference 9.1e-8 | yes |
| FX-SHIFT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-005 frame 0: Red, green and blue from hue: each pixel turns the grey of its HSV hue, a full turn the whole scale: the red, at 0 degrees, turns black, the green (130 degrees) 92.08 of 255, the blue (229.95 degrees) 162.88 and the skin (25.71 degrees) 18.21; a grey has no hue and turns black. | largest difference 4.1e-8 | yes |
| FX-SHIFT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-006 frame 0: Red, green and blue from lightness, the middle of the largest and the smallest channel: the red 120 of 255, the skin 218, the blue 159.5; every grey stays as it is. | largest difference 1.3e-7 | yes |
| FX-SHIFT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-007 frame 0: Red, green and blue from saturation, HSL's: the red two thirds of the scale (170), the green half (127.5), the blue the whole; every grey has none and turns black. | largest difference 6.9e-7 | yes |
| FX-SHIFT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-008 frame 0: Red full, green half and blue off: every pixel that shows turns orange, full red and green at half of the encoded scale (127.5), its covering kept, so the soft edges are orange at half covering. | largest difference 3.0e-8 | yes |
| FX-SHIFT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-009 frame 0: Alpha from luminance, the Colorful Glitch tutorial's use: the dark parts drop out: the black is gone, the white stays, the grey shows at 128 of 255, the red at 74.016, the line at 27.57; a soft edge's covering is its colour's luma, so the soft line is as faint as the line and the soft skin as strong as the skin. | largest difference 2.1e-7 | yes |
| FX-SHIFT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-010 frame 0: Alpha full: everything shows, at full covering: the empty border turns black, and the soft edges turn solid line and solid skin. | largest difference 1.9e-7 | yes |
| FX-SHIFT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-011 frame 0: Alpha off: nothing shows. | largest difference 0.0e0 | yes |
| FX-SHIFT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-012 frame 0: Alpha half: everything shows at half covering, the empty border black at half, the soft edges no fainter than the rest. | largest difference 9.3e-8 | yes |
| FX-SHIFT-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-013 frame 0: Red, green and blue from alpha: the covering as grey: the drawing white, its soft edges a half grey (128 of 255) at half covering, the border still empty. | largest difference 3.0e-8 | yes |
| FX-SHIFT-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-014 frame 0: Alpha from saturation: every grey drops out, the blue stays whole, the red shows at two thirds and the green at half. | largest difference 6.2e-7 | yes |
| FX-SHIFT-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-015 frame 0: All four at once: red from hue, green from saturation, blue from lightness and alpha from luminance, the swatches seen as HSL over their brightness. | largest difference 7.1e-7 | yes |
| FX-SHIFT-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-016 frame 0: FX-SHIFT-009 moved three pixels right: the same, moved. | largest difference 2.1e-7 | yes |
| FX-SHIFT-016 frame 3: FX-SHIFT-009 moved three pixels right: the same, moved. | largest difference 2.1e-7 | yes |
| FX-SHIFT-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHIFT-017 frame 0: Red from "Full": the words are lower case. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-017 frame 4: Red from "Full": the words are lower case. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIFT-018 frame 0: Alpha from "yes", not a word Shift Channels takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-018 frame 4: Alpha from "yes", not a word Shift Channels takes. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIFT-019 frame 0: Green from "brightness": this program's word is "luminance". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-019 frame 4: Green from "brightness": this program's word is "luminance". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHIFT-020 frame 0: Blue from "", no word at all. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-020 frame 4: Blue from "", no word at all. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHIFT-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shift_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shift_015.json is saved with its four settings | {"take_alpha":"luminance","take_blue":"lightness","take_green":"saturation","take_red":"hue"} | yes |
| fx_shift_017.json is refused in a sentence saying Shift Channels takes red from ... | Shift Channels takes red from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "Full". | yes |
| fx_shift_018.json is refused in a sentence saying Shift Channels takes alpha from ... | Shift Channels takes alpha from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "yes". | yes |
| fx_shift_019.json is refused in a sentence saying Shift Channels takes green from ... | Shift Channels takes green from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "brightness". | yes |
| fx_shift_020.json is refused in a sentence saying Shift Channels takes blue from ... | Shift Channels takes blue from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "". | yes |
| a file with a Shift Channels whose take red is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shift Channels without its take blue is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red from Full is refused with a sentence, and nothing changes | Shift Channels takes red from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "Full". | yes |
| alpha from yes is refused with a sentence, and nothing changes | Shift Channels takes alpha from "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half" or "off", and this is "yes". | yes |
| alpha from luminance is taken | taken | yes |
| blue from hue is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shift_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shift_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shift_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shift_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shift_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_shift_016.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shift_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_019.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_shift_020.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Shift Channels alpha from luminance: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 0, Full | largest difference 1 of 255, 861 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 100, Full | largest difference 1 of 255, 858 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 239, Full | largest difference 1 of 255, 947 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 0, Draft | largest difference 1 of 255, 360 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 100, Draft | largest difference 1 of 255, 295 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels alpha from luminance on three layers, frame 239, Draft | largest difference 1 of 255, 347 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1910331 pixels changed | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 0, Full | largest difference 1 of 255, 4922 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 100, Full | largest difference 1 of 255, 3122 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 239, Full | largest difference 1 of 255, 3895 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 0, Draft | largest difference 1 of 255, 146 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red and blue swapped on three layers, frame 239, Draft | largest difference 1 of 255, 107 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Shift Channels red from hue, green from saturation, blue from lightness, alpha full on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-553 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| the settings as they start change nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_alpha_from_luminance.png, alpha from luminance (the Colorful Glitch use): only white (or within 5 levels of it) stays solid, and the darkest parts (every channel 8 or below) keep at most 8 of 255; draws cleanly | [], only white solid: true, the darkest gone: true | yes |
| 3_red_blue_swapped.png, red from blue and blue from red: every solid pixel's red and blue traded, its green kept, within 1 level; draws cleanly | [], traded: true | yes |
| 4_hue_as_grey.png, red, green and blue from hue: the street in greys, light where its colour is far round the wheel, and every grey of the street black; draws cleanly | [], all grey: true, the street's greys black: true | yes |
| 5_alpha_full.png, alpha full: every pixel solid, the street's solid pixels as they were; draws cleanly | [], every pixel solid: true, solid pixels kept: true | yes |

## Result

130 of 130 checks pass.
