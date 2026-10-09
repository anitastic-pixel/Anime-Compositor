# D-365: Broadcast Safe

B-244, after After Effects' Broadcast Colors: a pixel whose brightness and colour together make a video signal above the limit, in IRE, is darkened, greyed, or keyed out, or only those pixels are kept. Every expected pixel is `Fixtures/broadcast_safe/expected_broadcast_safe.json`, written by `tools/broadcast_safe_reference.py` before this code existed and printed in document 25 as FX-BCAST-001 to 015. Tolerance 2e-5.

## FX-BCAST-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BCAST-001 frame 0: NTSC, reduce luminance, 110 IRE, the settings as they start: the three brightest rows of yellow (pure yellow is about 131 IRE) and cyan and the top row of green are darkened until their signal is 110 IRE, keeping their colour; everything else is safe and stays as it is. | largest difference 1.9e-7 | yes |
| FX-BCAST-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-002 frame 0: Reduce saturation: the same unsafe pixels keep their brightness and are made greyer until their signal is 110 IRE. | largest difference 1.9e-7 | yes |
| FX-BCAST-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-003 frame 0: Key out unsafe: the unsafe pixels are cleared, showing what is behind; everything else stays exactly as it is. | largest difference 1.9e-7 | yes |
| FX-BCAST-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-004 frame 0: Key out safe: the other way round, only the unsafe pixels are left, to show where they are. | largest difference 7.9e-8 | yes |
| FX-BCAST-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-005 frame 0: PAL: no set-up, so black is 0 IRE rather than 7.5 and the limit allows a little less colour signal; the same pixels are unsafe as on NTSC, each brought down a little further. | largest difference 1.9e-7 | yes |
| FX-BCAST-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-006 frame 0: NTSC at 90 IRE: even the white and the light greys are above the limit, so they are darkened too. | largest difference 1.2e-7 | yes |
| FX-BCAST-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-007 frame 0: NTSC at 120 IRE, the loosest limit: only the two brightest rows of yellow and cyan are brought down. | largest difference 1.9e-7 | yes |
| FX-BCAST-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-008 frame 0: PAL at 95 IRE, reduce saturation: the white's brightness alone is past the limit and it has no colour to take away, so it becomes the grey at the limit; the bright colours lose colour, keeping their brightness. | largest difference 2.7e-7 | yes |
| FX-BCAST-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-009 frame 0: The limit keyed from 90 IRE at frame 0 to 120 at frame 4, linear: frame 0 is FX-BCAST-006, frame 2 is 105 IRE, frame 4 is FX-BCAST-007. | largest difference 1.2e-7 | yes |
| FX-BCAST-009 frame 2: The limit keyed from 90 IRE at frame 0 to 120 at frame 4, linear: frame 0 is FX-BCAST-006, frame 2 is 105 IRE, frame 4 is FX-BCAST-007. | largest difference 1.9e-7 | yes |
| FX-BCAST-009 frame 4: The limit keyed from 90 IRE at frame 0 to 120 at frame 4, linear: frame 0 is FX-BCAST-006, frame 2 is 105 IRE, frame 4 is FX-BCAST-007. | largest difference 1.9e-7 | yes |
| FX-BCAST-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-010 frame 0: FX-BCAST-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-BCAST-010 frame 3: FX-BCAST-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-BCAST-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BCAST-011 frame 0: Maximum signal amplitude 121, above 120. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-011 frame 4: Maximum signal amplitude 121, above 120. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-011: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BCAST-012 frame 0: Maximum signal amplitude 89, below 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-012 frame 4: Maximum signal amplitude 89, below 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BCAST-013 frame 0: Locale "secam", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-013 frame 4: Locale "secam", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BCAST-014 frame 0: Locale "NTSC": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-014 frame 4: Locale "NTSC": the word is exact, so capitals are not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BCAST-015 frame 0: Method "reduce", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-015 frame 4: Method "reduce", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BCAST-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bcast_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_bcast_008.json is saved with its locale, method and limit | {"locale":"pal","max_amplitude":95,"method":"reduce_saturation"} | yes |
| fx_bcast_013.json is refused in a sentence | Broadcast Safe's locale is "ntsc" or "pal", and this is "secam". | yes |
| fx_bcast_015.json is refused in a sentence | Broadcast Safe's method is "reduce_luminance", "reduce_saturation", "key_out_unsafe" or "key_out_safe", and this is "reduce". | yes |
| a file with a Broadcast Safe with no `method` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Broadcast Safe whose locale is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| limit 120.5 is refused with a sentence, and nothing changes | Broadcast Safe's max amplitude runs from 90 to 120, and this is 120.5. | yes |
| limit 89.5 is refused with a sentence, and nothing changes | Broadcast Safe's max amplitude runs from 90 to 120, and this is 89.5. | yes |
| locale "Pal", written with a capital is refused with a sentence, and nothing changes | Broadcast Safe's locale is "ntsc" or "pal", and this is "Pal". | yes |
| method "darken" is refused with a sentence, and nothing changes | Broadcast Safe's method is "reduce_luminance", "reduce_saturation", "key_out_unsafe" or "key_out_safe", and this is "darken". | yes |
| limit keyed to 130 is refused with a sentence, and nothing changes | Broadcast Safe's max amplitude runs from 90 to 120, and this is 130. | yes |
| PAL, key out safe, 100 IRE, is taken | taken | yes |
| limit keyed from 90 to 120 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bcast_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bcast_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bcast_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bcast_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_bcast_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_bcast_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_bcast_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bcast_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bcast_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bcast_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_bcast_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 649804 pixels changed | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 0, Full | largest difference 1 of 255, 4327 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 100, Full | largest difference 1 of 255, 1924 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 239, Full | largest difference 1 of 255, 2323 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 0, Draft | largest difference 1 of 255, 163 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 100, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, reduce luminance, 90 IRE on three layers, frame 239, Draft | largest difference 1 of 255, 81 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 386794 pixels changed | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 0, Full | largest difference 1 of 255, 3949 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 239, Full | largest difference 1 of 255, 1936 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 0, Draft | largest difference 1 of 255, 133 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe PAL, reduce saturation, 95 IRE on three layers, frame 239, Draft | largest difference 1 of 255, 57 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 658055 pixels changed | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 0, Full | largest difference 1 of 255, 3779 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 100, Full | largest difference 1 of 255, 1418 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 239, Full | largest difference 1 of 255, 1770 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 0, Draft | largest difference 1 of 255, 127 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 100, Draft | largest difference 1 of 255, 32 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out unsafe, 90 IRE on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1876428 pixels changed | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 100, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 239, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Broadcast Safe NTSC, key out safe, 100 IRE on three layers, frame 239, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 0, Full | largest difference 1 of 255, 335 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 100, Full | largest difference 1 of 255, 495 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 239, Full | largest difference 1 of 255, 340 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 0, Draft | largest difference 1 of 255, 42 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 100, Draft | largest difference 1 of 255, 28 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Color Neutralizer, Levels, Color Offset (polarize) and Broadcast Safe (100 IRE) in one colour run on three layers, frame 239, Draft | largest difference 1 of 255, 32 pixels differ; 9 of 9 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-365 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts (NTSC, reduce luminance, 110 IRE): every colour in the street is safe, so nothing changes; draws cleanly | [], 0 pixels changed | yes |
| 3_reduce_luminance_100.png, NTSC at 100 IRE: the lit windows (about 107 IRE) are darkened, the white road markings (about 98) are left; draws cleanly | [], a window [238, 224, 159] from [255, 240, 170], a marking [250, 250, 250] from [250, 250, 250] | yes |
| 4_reduce_saturation_100.png, the same made greyer instead: the windows lose colour; draws cleanly | [], a window [246, 238, 203] from [255, 240, 170] | yes |
| 5_key_out_safe_100.png, key out safe at 100 IRE: only the unsafe pixels are left, the lit windows; draws cleanly | [], a window's covering 255, a marking's 0, 2880 pixels left | yes |

## Result

125 of 125 checks pass.
