# D-514: Stripe Sweep

B-394, after CycoreFX's CC Line Sweep (Transition): the layer wiped away line by line. The CycoreFX manual names the controls but publishes no formula and no settings when added, so the rule and those settings are ours, stated in `tools/stripe_sweep_reference.py`: the drawing cut into lines Thickness pixels thick across the Direction, each wiped from one end to the other (the other, flipped), the lines starting one after another, the Slant how much they overlap (0 one at a time, 99 all together). Every expected pixel is `Fixtures/stripe_sweep/expected_stripe_sweep.json`, written by the tool before this code existed and printed in document 25 as FX-STRIPE-001 to 024. Tolerance 2e-5.

## FX-STRIPE-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-STRIPE-001 frame 0: The settings as they start: completion 0, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-STRIPE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-002 frame 0: Completion 100: every pixel gone. | largest difference 0.0e0 | yes |
| FX-STRIPE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-003 frame 0: Thickness 3, completion 40: four lines across the drawing, each wiped from the left, the top one furthest along. | largest difference 1.2e-7 | yes |
| FX-STRIPE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-004 frame 0: Thickness 3, slant 0, completion 40: the top line done, the second part way, the others not started. | largest difference 1.2e-7 | yes |
| FX-STRIPE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-005 frame 0: Thickness 3, slant 99, completion 40: all four lines together, a straight edge. | largest difference 1.9e-7 | yes |
| FX-STRIPE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-006 frame 0: FX-STRIPE-003 flipped: the lines wiped from the right. | largest difference 1.2e-7 | yes |
| FX-STRIPE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-007 frame 0: Direction 90, thickness 4, completion 40: the lines are columns, wiped downward. | largest difference 1.2e-7 | yes |
| FX-STRIPE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-008 frame 0: Direction 30, thickness 3, completion 50: the lines turned. | largest difference 8.4e-8 | yes |
| FX-STRIPE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-009 frame 0: Thickness 1, completion 50: ten one-pixel lines, a staircase edge. | largest difference 9.5e-8 | yes |
| FX-STRIPE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-010 frame 0: Thickness 20 (as added), completion 60: one line, a straight wipe. | largest difference 1.9e-7 | yes |
| FX-STRIPE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-011 frame 0: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, thickness 3: frame 2 is completion 50, frame 4 all clear. | largest difference 1.9e-7 | yes |
| FX-STRIPE-011 frame 2: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, thickness 3: frame 2 is completion 50, frame 4 all clear. | largest difference 9.5e-8 | yes |
| FX-STRIPE-011 frame 4: Completion keyed from 0 at frame 0 to 100 at frame 4, linear, thickness 3: frame 2 is completion 50, frame 4 all clear. | largest difference 0.0e0 | yes |
| FX-STRIPE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-012 frame 0: Thickness keyed from 2 at frame 0 to 6 at frame 4, linear, completion 50: frame 2 is thickness 4. | largest difference 9.5e-8 | yes |
| FX-STRIPE-012 frame 2: Thickness keyed from 2 at frame 0 to 6 at frame 4, linear, completion 50: frame 2 is thickness 4. | largest difference 9.5e-8 | yes |
| FX-STRIPE-012 frame 4: Thickness keyed from 2 at frame 0 to 6 at frame 4, linear, completion 50: frame 2 is thickness 4. | largest difference 1.1e-7 | yes |
| FX-STRIPE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-013 frame 0: FX-STRIPE-003 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-STRIPE-013 frame 3: FX-STRIPE-003 moved three pixels right: the same, moved. | largest difference 1.2e-7 | yes |
| FX-STRIPE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-014 frame 0: Direction -135, flipped, slant 20, thickness 2, completion 65. | largest difference 1.9e-7 | yes |
| FX-STRIPE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-STRIPE-015 frame 0: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-015 frame 4: Completion 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-016 frame 0: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-016 frame 4: Completion -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-017 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-017 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-018 frame 0: Thickness 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-018 frame 4: Thickness 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-019 frame 0: Thickness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-019 frame 4: Thickness 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-020 frame 0: Slant -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-020 frame 4: Slant -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-021 frame 0: Slant 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-021 frame 4: Slant 100, above 99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-022 frame 0: Flip "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-022 frame 4: Flip "yes": it is "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-023 frame 0: Flip "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-023 frame 4: Flip "On": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-STRIPE-024 frame 0: Slant keyed to 120 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-024 frame 4: Slant keyed to 120 at frame 4: a key outside the range. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-STRIPE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stripe_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_stripe_014.json is saved with its five settings, flipped, direction -135, slant 20, thickness 2, completion 65 | {"completion":65,"direction":-135,"flip":"on","slant":20,"thickness":2} | yes |
| fx_stripe_015.json is refused in a sentence naming completion | Stripe Sweep's completion runs from 0 to 100, and this is 101. | yes |
| fx_stripe_016.json is refused in a sentence naming completion | Stripe Sweep's completion runs from 0 to 100, and this is -1. | yes |
| fx_stripe_017.json is refused in a sentence naming direction | Stripe Sweep's direction runs from -3600 to 3600, and this is 3601. | yes |
| fx_stripe_018.json is refused in a sentence naming thickness | Stripe Sweep's thickness runs from 1 to 1000, and this is 0. | yes |
| fx_stripe_019.json is refused in a sentence naming thickness | Stripe Sweep's thickness runs from 1 to 1000, and this is 1001. | yes |
| fx_stripe_020.json is refused in a sentence naming slant | Stripe Sweep's slant runs from 0 to 99, and this is -1. | yes |
| fx_stripe_021.json is refused in a sentence naming slant | Stripe Sweep's slant runs from 0 to 99, and this is 100. | yes |
| fx_stripe_022.json is refused in a sentence naming flip | Stripe Sweep's flip is "off" or "on", and this is "yes". | yes |
| fx_stripe_023.json is refused in a sentence naming flip | Stripe Sweep's flip is "off" or "on", and this is "On". | yes |
| a file with a Stripe Sweep whose thickness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Stripe Sweep whose flip is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-514)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_stripe_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 4 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_stripe_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1036800 pixels changed | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 0, Full | largest difference 1 of 255, 752 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 100, Full | largest difference 1 of 255, 814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 239, Full | largest difference 1 of 255, 504 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 0, Draft | largest difference 1 of 255, 25 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 100, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 50 (as added otherwise) on three layers, frame 239, Draft | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 663150 pixels changed | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 0, Full | largest difference 1 of 255, 3725 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 0, Draft | largest difference 1 of 255, 124 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 40, thickness 6, slant 0, direction 30 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1244160 pixels changed | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 100, Full | largest difference 1 of 255, 408 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 0, Draft | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 100, Draft | largest difference 1 of 255, 12 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion 60, flipped, slant 99, direction -90, thickness 100 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 898740 pixels changed | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 1173 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 28 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Stripe Sweep completion keyed 10 to 90 on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-514 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_stripes.png: drawn without a warning, the pixel at (240, 2) gone, the one at (240, 268) as the street's | 52780 pixels changed; at (240, 2) [0, 0, 0, 0]; at (240, 268) [60, 60, 66, 255]; warnings [] | yes |
| 2_one_at_a_time.png: drawn without a warning, the pixel at (240, 2) gone, the one at (240, 268) as the street's | 53760 pixels changed; at (240, 2) [0, 0, 0, 0]; at (240, 268) [60, 60, 66, 255]; warnings [] | yes |
| 3_all_together.png: drawn without a warning, the pixel at (2, 135) gone, the one at (470, 135) as the street's | 51840 pixels changed; at (2, 135) [0, 0, 0, 0]; at (470, 135) [170, 195, 215, 255]; warnings [] | yes |
| 4_columns_flipped.png: drawn without a warning, the pixel at (470, 135) gone, the one at (2, 135) as the street's | 50760 pixels changed; at (470, 135) [0, 0, 0, 0]; at (2, 135) [180, 90, 70, 255]; warnings [] | yes |
| 5_turned.png: drawn without a warning, the pixel at (240, 135) gone, the one at (470, 268) as the street's | 66441 pixels changed; at (240, 135) [0, 0, 0, 0]; at (470, 268) [60, 60, 66, 255]; warnings [] | yes |
| 6_done.png: drawn without a warning, the pixel at (2, 2) gone, every pixel gone | 129600 pixels changed; at (2, 2) [0, 0, 0, 0]; at (240, 135) [0, 0, 0, 0]; warnings [] | yes |

## Result

157 of 157 checks pass.
