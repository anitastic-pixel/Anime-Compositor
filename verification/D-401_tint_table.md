# D-401: Tint with After Effects' settings

B-280: `core.tint` takes After Effects' Map Black To, Map White To and Amount to Tint; each pixel's lightness picks a colour between the two, by Gradient Map's rule with its two ends only. A Tint saved before D-401 (`color` and `amount`) is read and drawn by its own older rule, unchanged. Every expected pixel is `Fixtures/tint/expected_tint.json`, written by `tools/tint_reference.py` before this code existed and printed in document 25 as FX-TINT-001 to 016; the older Tint's are FX-ADJ-008 to 010 and FX-FXK-004, 005, 008 and 009, pinned long before D-401. Tolerance 2e-5.

## FX-TINT-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TINT-001 frame 0: The settings as they start: black to #000000, white to #ffffff, amount 100. Every pixel that shows turns grey by its lightness, red, green and blue alike; black stays black and white white; the yellow at half covering takes the same grey as the yellow, at its own covering. | largest difference 1.2e-7 | yes |
| FX-TINT-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-002 frame 0: Amount 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-TINT-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-003 frame 0: Amount 50: every pixel halfway, in linear light, between the drawing and FX-TINT-001. | largest difference 1.2e-7 | yes |
| FX-TINT-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-004 frame 0: Black to #1a2a6c, a navy, and white to #fdbb2d, a gold: a two-colour picture; the black pixels turn exactly #1a2a6c and the white #fdbb2d, the rest between by their lightness. | largest difference 1.4e-7 | yes |
| FX-TINT-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-005 frame 0: Black to #ffffff and white to #000000, the two swapped from as they start (Swap Colors): a negative in grey; black turns white and white black. | largest difference 8.0e-8 | yes |
| FX-TINT-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-006 frame 0: FX-TINT-004's colours swapped: black to the gold and white to the navy. | largest difference 8.8e-8 | yes |
| FX-TINT-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-007 frame 0: Both colours #6450a0: every pixel that shows is #6450a0 at its own covering, whatever its lightness. | largest difference 3.0e-8 | yes |
| FX-TINT-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-008 frame 0: FX-TINT-004 with its colours written in capitals: the same. | largest difference 1.4e-7 | yes |
| FX-TINT-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-009 frame 0: FX-TINT-004 at amount 30: each pixel 30 per cent of the way, in linear light, toward its colour. | largest difference 1.5e-7 | yes |
| FX-TINT-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-010 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TINT-003, frame 4 is FX-TINT-001. | largest difference 1.9e-7 | yes |
| FX-TINT-010 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TINT-003, frame 4 is FX-TINT-001. | largest difference 1.2e-7 | yes |
| FX-TINT-010 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-TINT-003, frame 4 is FX-TINT-001. | largest difference 1.2e-7 | yes |
| FX-TINT-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-011 frame 0: FX-TINT-004 moved three pixels right: the same, moved. | largest difference 1.4e-7 | yes |
| FX-TINT-011 frame 3: FX-TINT-004 moved three pixels right: the same, moved. | largest difference 1.4e-7 | yes |
| FX-TINT-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TINT-012 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-012 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TINT-013 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-013 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TINT-014 frame 0: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-014 frame 4: Amount keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TINT-015 frame 0: Map Black To written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-015 frame 4: Map Black To written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TINT-016 frame 0: Map White To written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-016 frame 4: Map White To written "white", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TINT-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A Tint saved before D-401: its fixtures from before, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADJ-008 frame 0: A full tint on a half-covered picture keeps its coverage. | largest difference 3.0e-8 | yes |
| FX-ADJ-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADJ-009 frame 0: Two adjustment layers, lower one first. | largest difference 3.1e-8 | yes |
| FX-ADJ-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ADJ-010 frame 0: The same two, the other way up. | largest difference 3.1e-8 | yes |
| FX-ADJ-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_adj_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_adj_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_adj_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_adj_008.json, a Tint with a colour and an amount, opens as the older Tint | ["the older Tint"] | yes |
| FX-FXK-004 frame 0: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.0e-8 | yes |
| FX-FXK-004 frame 1: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.7e-8 | yes |
| FX-FXK-004 frame 2: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.0e-8 | yes |
| FX-FXK-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FXK-005 frame 1: A tint's amount eased from 0 to 1 on a curve that overshoots: the amount stops at 1 and goes no further. | largest difference 1.7e-8 | yes |
| FX-FXK-005 frame 2: A tint's amount eased from 0 to 1 on a curve that overshoots: the amount stops at 1 and goes no further. | largest difference 0.0e0 | yes |
| FX-FXK-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FXK-008 frame 2: Two settings of one effect keyed at once, and a second effect left plain. | largest difference 3.1e-8 | yes |
| FX-FXK-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FXK-009 frame 0: A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FXK-009 frame 4: A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FXK-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_fxk_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fxk_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fxk_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_fxk_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tint_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_tint_008.json, its colours written in capitals, is saved in small letters, as Gradient Map's are | "#1a2a6c" "#fdbb2d" | yes |
| fx_tint_009.json is saved with its three settings, the colours in small letters | {"amount_to_tint":30,"map_black_to":"#1a2a6c","map_white_to":"#fdbb2d"} | yes |
| fx_tint_012.json is refused in a sentence naming amount_to_tint | Tint's amount to tint runs from 0 to 100, and this is 101. | yes |
| fx_tint_013.json is refused in a sentence naming amount_to_tint | Tint's amount to tint runs from 0 to 100, and this is -1. | yes |
| fx_tint_015.json is refused in a sentence naming Map Black To | Tint's Map Black To is written #rrggbb, and this is "#12345". | yes |
| fx_tint_016.json is refused in a sentence naming Map White To | Tint's Map White To is written #rrggbb, and this is "white". | yes |
| a file with a Tint whose amount to tint is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tint without its Map Black To is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Tint whose Map White To is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| amount to tint 101 is refused with a sentence, and nothing changes | Tint's amount to tint runs from 0 to 100, and this is 101. | yes |
| Map White To #fff is refused with a sentence, and nothing changes | Tint's Map White To is written #rrggbb, and this is "#fff". | yes |
| amount to tint keyed to 150 is refused with a sentence, and nothing changes | Tint's amount to tint runs from 0 to 100, and this is 150. | yes |
| navy to gold is taken | taken | yes |
| amount to tint keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tint_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tint_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tint_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tint_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tint_011.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tint_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_tint_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Tint as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1932066 pixels changed | yes |
| the reference shot, Tint as added on three layers, frame 0, Full | largest difference 1 of 255, 2972 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint as added on three layers, frame 100, Full | largest difference 1 of 255, 990 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint as added on three layers, frame 239, Full | largest difference 1 of 255, 1256 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint as added on three layers, frame 0, Draft | largest difference 1 of 255, 138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint as added on three layers, frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint as added on three layers, frame 239, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 0, Full | largest difference 1 of 255, 572 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 100, Full | largest difference 1 of 255, 1600 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 239, Full | largest difference 1 of 255, 711 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 100, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint navy to gold at 30 on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073490 pixels changed | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 0, Full | largest difference 1 of 255, 1175 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 100, Full | largest difference 1 of 255, 765 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 239, Full | largest difference 1 of 255, 1588 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 0, Draft | largest difference 1 of 255, 86 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 100, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Tint swapped, white to black on three layers, frame 239, Draft | largest difference 1 of 255, 118 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-401 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| amount to tint 0 changes nothing: the street byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (black to black, white to white, 100): the street in greys, no pixel coloured; draws cleanly | [], 0 coloured pixels | yes |
| 3_navy_to_gold.png, navy (#1a2a6c) to gold (#fdbb2d): every pixel's red, green and blue between the navy's and the gold's; draws cleanly | [], all between: true | yes |
| 4_swapped.png, Swap Colors (gold to navy): the dark parts gold and the light parts navy, the other way from 3; draws cleanly | [], 6246 of the 6246 darkest and lightest pixels turned over | yes |
| 5_amount_30.png, navy to gold at amount 30: the street with a light wash of the two colours; draws cleanly | [], 129600 pixels changed, nearer the street than 3 is | yes |
| 6_older_tint.png, an older Tint (colour #6450a0 in linear numbers, amount 0.3) against the new Tint with #6450a0 at both ends and amount 30: the same picture within 1 level; both draw cleanly | [] [], largest difference 0 of 255 | yes |

## Result

145 of 145 checks pass.
