# D-382: Gamma/Pedestal/Gain

B-261, after After Effects' Gamma/Pedestal/Gain: a black stretch that lifts the dark values of every channel, then for red, green and blue apart a gamma for the middle, a pedestal for the lowest value and a gain for the highest. Every expected pixel is `Fixtures/gamma_pedestal_gain/expected_gamma_pedestal_gain.json`, written by `tools/gamma_pedestal_gain_reference.py` before this code existed and printed in document 25 as FX-GPG-001 to 021. Tolerance 2e-5.

## FX-GPG-001 to 021 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-GPG-001 frame 0: The settings as they start: black stretch 1, every gamma and gain 1, every pedestal 0, so the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-GPG-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-002 frame 0: Black stretch 2: every dark value lifted, the warm shadow and the dim rows most; black and full values stay. | largest difference 1.2e-7 | yes |
| FX-GPG-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-003 frame 0: Black stretch 4, as far as it goes: the darks lifted much more. | largest difference 8.6e-8 | yes |
| FX-GPG-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-004 frame 0: Red gamma 2: the middle of the red channel brighter, so the skin, grey and warm tones go redder; green and blue untouched, and 0 and full red stay. | largest difference 1.3e-7 | yes |
| FX-GPG-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-005 frame 0: Every gamma 0.5: the middle of every channel darker, the colours deeper. | largest difference 3.6e-7 | yes |
| FX-GPG-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-006 frame 0: Every pedestal 0.2: black becomes a grey of 0.2 (about 51 of 255), white stays white, everything between lifted a little less the brighter it is. | largest difference 1.7e-7 | yes |
| FX-GPG-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-007 frame 0: Every gain 0.5: white becomes a grey of 0.5, black stays black, everything halved in its encoded values. | largest difference 4.4e-8 | yes |
| FX-GPG-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-008 frame 0: Every pedestal and gain 0.5: every colour the same grey of 0.5, the covering kept. | largest difference 3.0e-8 | yes |
| FX-GPG-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-009 frame 0: Every pedestal 1 and gain 0: the channel turned over, so the drawing's negative: black white, red cyan, yellow blue. | largest difference 6.7e-8 | yes |
| FX-GPG-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-010 frame 0: Every pedestal -0.5: the lower half of each channel held at 0, the rest stretched down to it; white stays white. | largest difference 2.9e-7 | yes |
| FX-GPG-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-011 frame 0: Every gain 2: each channel doubled, everything above half held at full. | largest difference 1.5e-7 | yes |
| FX-GPG-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-012 frame 0: Black stretch 1.5, red gain 1.2, green pedestal 0.1 and blue gamma 1.5 together, each channel its own curve. | largest difference 1.7e-7 | yes |
| FX-GPG-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-013 frame 0: Red gain keyed from 1 at frame 0 to 0 at frame 4, linear: frame 0 untouched, frame 2 the red channel halved, frame 4 no red at all. | largest difference 1.9e-7 | yes |
| FX-GPG-013 frame 2: Red gain keyed from 1 at frame 0 to 0 at frame 4, linear: frame 0 untouched, frame 2 the red channel halved, frame 4 no red at all. | largest difference 1.3e-7 | yes |
| FX-GPG-013 frame 4: Red gain keyed from 1 at frame 0 to 0 at frame 4, linear: frame 0 untouched, frame 2 the red channel halved, frame 4 no red at all. | largest difference 1.3e-7 | yes |
| FX-GPG-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-014 frame 0: FX-GPG-012 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-GPG-014 frame 3: FX-GPG-012 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-GPG-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-GPG-015 frame 0: Black stretch 0.9, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-015 frame 4: Black stretch 0.9, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-016 frame 0: Black stretch 4.1, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-016 frame 4: Black stretch 4.1, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-017 frame 0: Green gamma 0.09, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-017 frame 4: Green gamma 0.09, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-018 frame 0: Blue gamma 10.1, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-018 frame 4: Blue gamma 10.1, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-019 frame 0: Red pedestal 1.1, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-019 frame 4: Red pedestal 1.1, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-020 frame 0: Green gain -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-020 frame 4: Green gain -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-GPG-021 frame 0: Blue gain 4.1, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-021 frame 4: Blue gain 4.1, above 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-GPG-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gpg_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_gpg_012.json is saved with its ten settings | {"black_stretch":1.5,"blue_gain":1,"blue_gamma":1.5,"blue_pedestal":0,"green_gain":1,"green_gamma":1,"green_pedestal":0.1,"red_gain":1.2,"red_gamma":1,"red_pedestal":0} | yes |
| fx_gpg_015.json is refused in a sentence naming black stretch | Gamma/Pedestal/Gain's black stretch runs from 1 to 4, and this is 0.9. | yes |
| fx_gpg_016.json is refused in a sentence naming black stretch | Gamma/Pedestal/Gain's black stretch runs from 1 to 4, and this is 4.1. | yes |
| fx_gpg_017.json is refused in a sentence naming green gamma | Gamma/Pedestal/Gain's green gamma runs from 0.1 to 10, and this is 0.09. | yes |
| fx_gpg_018.json is refused in a sentence naming blue gamma | Gamma/Pedestal/Gain's blue gamma runs from 0.1 to 10, and this is 10.1. | yes |
| fx_gpg_019.json is refused in a sentence naming red pedestal | Gamma/Pedestal/Gain's red pedestal runs from -1 to 1, and this is 1.1. | yes |
| fx_gpg_020.json is refused in a sentence naming green gain | Gamma/Pedestal/Gain's green gain runs from 0 to 4, and this is -0.1. | yes |
| fx_gpg_021.json is refused in a sentence naming blue gain | Gamma/Pedestal/Gain's blue gain runs from 0 to 4, and this is 4.1. | yes |
| a file with a Gamma/Pedestal/Gain whose red gain is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Gamma/Pedestal/Gain without its blue pedestal is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| red gamma 0.05 is refused with a sentence, and nothing changes | Gamma/Pedestal/Gain's red gamma runs from 0.1 to 10, and this is 0.05. | yes |
| blue gain 4.5 is refused with a sentence, and nothing changes | Gamma/Pedestal/Gain's blue gain runs from 0 to 4, and this is 4.5. | yes |
| black stretch keyed to 5 is refused with a sentence, and nothing changes | Gamma/Pedestal/Gain's black stretch runs from 1 to 4, and this is 5. | yes |
| black stretch 2, every pedestal 1 is taken | taken | yes |
| blue gain keyed from 1 to 0 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gpg_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gpg_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gpg_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gpg_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_gpg_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_gpg_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_gpg_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 0, Full | largest difference 1 of 255, 3121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 100, Full | largest difference 1 of 255, 316 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 239, Full | largest difference 1 of 255, 908 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 0, Draft | largest difference 1 of 255, 132 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 100, Draft | largest difference 1 of 255, 52 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain black stretch 2, gammas 1.4, 1, 0.7 on three layers, frame 239, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 0, Full | largest difference 1 of 255, 400860 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 100, Full | largest difference 1 of 255, 407804 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 239, Full | largest difference 1 of 255, 406825 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 0, Draft | largest difference 1 of 255, 11283 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 100, Draft | largest difference 1 of 255, 11592 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain pedestals 0.1, gains 0.9, 1.1, 1.3 on three layers, frame 239, Draft | largest difference 1 of 255, 11487 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073492 pixels changed | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 0, Full | largest difference 1 of 255, 1945 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 100, Full | largest difference 1 of 255, 2806 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 239, Full | largest difference 1 of 255, 2218 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 0, Draft | largest difference 1 of 255, 2658 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 100, Draft | largest difference 1 of 255, 2165 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Gamma/Pedestal/Gain every pedestal 1 and gain 0, the negative on three layers, frame 239, Draft | largest difference 1 of 255, 2700 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-382 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| the effect as it is added changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_black_stretch_3.png, black stretch 3: the shadows opened up, no pixel darker; draws cleanly | [], no channel darker: true | yes |
| 3_warm.png, red gamma and gain 1.2, blue gain 0.8 and pedestal 0.05: the street warmer, its blacks a little blue; draws cleanly | [], 129600 pixels changed | yes |
| 4_negative.png, every pedestal 1 and gain 0: the street's negative, each channel 255 less itself within 1 level; draws cleanly | [], every channel turned over: true | yes |

## Result

141 of 141 checks pass.
