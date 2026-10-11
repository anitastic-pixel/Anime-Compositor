# D-549: Reduce Interlace Flicker

B-429: `core.reduce_interlace_flicker` takes After Effects' Reduce Interlace Flicker (Stylize, Obsolete) control, Softness, as a second name over Gaussian Blur's one-way blur (D-303, as D-383 and D-394): Softness is the sigma in pixels of a blur up and down only, its edges repeated, so a thin line across no longer sits on one field. Gaussian Blur leaves a one-way blur to the processor, so this does too. Every expected pixel is `Fixtures/reduce_interlace_flicker/expected_reduce_interlace_flicker.json`, written by `tools/reduce_interlace_flicker_reference.py` before this code existed and printed in document 25 as FX-RIF-001 to 009; Gaussian Blur's FX-BLURRY-001 to 009 rerun unchanged. Tolerance 2e-5.

## FX-RIF-001 to 009 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RIF-001 frame 0: The setting as it starts: Softness 1, a Gaussian of sigma 1 down each column reaching 3 rows. Each row takes a little of the rows above and below it, so the darkening down each column is smoothed; the colours of neighbouring columns never mix; the top and bottom rows read themselves past the edge, so they do not fade; every covering is kept. | largest difference 1.7e-7 | yes |
| FX-RIF-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-002 frame 0: Softness 0: the drawing exactly as it is. | largest difference 1.9e-7 | yes |
| FX-RIF-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-003 frame 0: Softness 2.5: reaching 8 rows, every column far smoother than FX-RIF-001. | largest difference 1.7e-7 | yes |
| FX-RIF-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-004 frame 0: Softness 0.3: reaching one row, a slight softening. | largest difference 2.4e-7 | yes |
| FX-RIF-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-005 frame 0: Softness keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-RIF-001. | largest difference 1.9e-7 | yes |
| FX-RIF-005 frame 2: Softness keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-RIF-001. | largest difference 1.7e-7 | yes |
| FX-RIF-005 frame 4: Softness keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-RIF-001. | largest difference 1.8e-7 | yes |
| FX-RIF-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-006 frame 0: FX-RIF-001 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-RIF-006 frame 3: FX-RIF-001 moved three pixels right: the same, moved. | largest difference 1.5e-7 | yes |
| FX-RIF-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RIF-007 frame 0: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-007 frame 4: Softness 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-007: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIF-008 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-008 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-008: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RIF-009 frame 0: Softness keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-009 frame 4: Softness keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RIF-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Gaussian Blur, the engine under the second name: its fixtures, unchanged

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BLURRY-001 frame 0: Units blurriness, Blurriness 10: a Gaussian of sigma 3 reaching 20 pixels, so the square's orange spreads softly well past nine pixels out, where document 21's kernel of sigma 3 stops. | largest difference 2.9e-8 | yes |
| FX-BLURRY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-002 frame 0: The same number in a file without units: document 21's sigma 10, exactly as before D-321, far softer than FX-BLURRY-001. | largest difference 4.4e-9 | yes |
| FX-BLURRY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-003 frame 0: Units written "sigma": FX-BLURRY-002 exactly. | largest difference 4.4e-9 | yes |
| FX-BLURRY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-004 frame 0: Units blurriness, Blurriness 10, edges repeat: the layer does not grow, and past its left edge the square's own orange is read, so the left edge stays strong. | largest difference 3.8e-8 | yes |
| FX-BLURRY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-005 frame 0: Units blurriness, Blurriness 10, Blur Dimensions horizontal: spread across only, the rows above and below the square still clear. | largest difference 2.9e-8 | yes |
| FX-BLURRY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-006 frame 0: Units blurriness, Blurriness 0: the drawing untouched. | largest difference 3.1e-8 | yes |
| FX-BLURRY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-007 frame 0: A Float composition: units blurriness, Blurriness 4 (sigma 1.2, reaching 8 pixels), then Exposure +6. The light falls away smoothly for eight pixels right of the square; document 21's kernel of sigma 1.2 would stop dead four pixels out, the straight edge P-26 saw. | largest difference 4.7e-6 | yes |
| FX-BLURRY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BLURRY-008 frame 0: Units "Blurriness": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-BLURRY-008 frame 4: Units "Blurriness": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-BLURRY-008: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BLURRY-009 frame 0: Units "pixels", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-BLURRY-009 frame 4: Units "pixels", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-BLURRY-009: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rif_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rif_003.json is saved with its one setting, softness 2.5 | {"softness":2.5} | yes |
| fx_rif_007.json is refused in a sentence naming softness | Reduce Interlace Flicker's softness runs from 0 to 500, and this is 501. | yes |
| fx_rif_008.json is refused in a sentence naming softness | Reduce Interlace Flicker's softness runs from 0 to 500, and this is -1. | yes |
| a file with a Reduce Interlace Flicker whose softness is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Reduce Interlace Flicker without its softness is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| softness 501 is refused with a sentence, and nothing changes | Reduce Interlace Flicker's softness runs from 0 to 500, and this is 501. | yes |
| softness keyed to 600 is refused with a sentence, and nothing changes | Reduce Interlace Flicker's softness runs from 0 to 500, and this is 600. | yes |
| softness 2.5 is taken | taken | yes |
| softness keyed from 0 to 2 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rif_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rif_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rif_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rif_005.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_rif_006.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255, the blur left to the processor

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rif_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_rif_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 841237 pixels changed | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 0, Full | largest difference 1 of 255, 25686 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 100, Full | largest difference 1 of 255, 24818 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 239, Full | largest difference 1 of 255, 25461 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 0, Draft | largest difference 1 of 255, 15620 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 100, Draft | largest difference 1 of 255, 16122 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker as added (softness 1), left to the processor on three layers, frame 239, Draft | largest difference 1 of 255, 15802 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1305184 pixels changed | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 0, Full | largest difference 1 of 255, 48849 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 100, Full | largest difference 1 of 255, 48823 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 239, Full | largest difference 1 of 255, 48568 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 0, Draft | largest difference 1 of 255, 3890 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 100, Draft | largest difference 1 of 255, 3919 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reduce Interlace Flicker softness 2.5, left to the processor on three layers, frame 239, Draft | largest difference 1 of 255, 3824 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-549 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, thin lines with no effect: on the left white and black rows in turn, on the right white and black columns in turn; draws cleanly | [] | yes |
| softness 0 changes nothing: the lines byte for byte; draws cleanly | [], 0 pixels changed | yes |
| 2_as_added.png, as added (softness 1): the rows across on the left melt into one grey (a white row 189 and a black row 186 by hand, each within 1); the columns up and down on the right untouched byte for byte, since the blur never reaches sideways; draws cleanly | [], a white row [189, 189, 189, 255], a black row [186, 186, 186, 255], the right half kept: true | yes |
| 3_softness_0_3.png, softness 0.3: the rows across only softened, white rows still lighter than black ones by far more than in 2; draws cleanly | [], a white row 254, a black row 21 | yes |
| Reduce Interlace Flicker at softness 0.5 against Gaussian Blur sigma 0.5 pixels, vertical, edges repeated: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Reduce Interlace Flicker at softness 1 against Gaussian Blur sigma 1 pixels, vertical, edges repeated: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |
| Reduce Interlace Flicker at softness 2.5 against Gaussian Blur sigma 2.5 pixels, vertical, edges repeated: the same street byte for byte; both draw cleanly | [] [], 0 pixels differ | yes |

## Result

95 of 95 checks pass.
