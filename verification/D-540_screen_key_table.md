# D-540: Screen Key

B-420, this program's own green and blue screen keyer (roadmap item L-03; not a copy of Keylight, whose code and text are licensed). A colour difference matte against a picked screen colour, with gain and balance; clip levels; despill by holding the screen channel to a blend of the other two; shrink or grow on Simple Choker's disc, softness on document 21's Gaussian, and an edge refine by Key Cleaner's fit; views Final Result, Screen Matte and Status. The rule is stated in `tools/screen_key_reference.py`. Every expected pixel is `Fixtures/screen_key/expected_screen_key.json`, written by the tool before this code existed and printed in document 25 as FX-SKEY-001 to 024. Tolerance 2e-5.

## FX-SKEY-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SKEY-001 frame 0: The settings as they start: screen colour pure green, gain 100, balance 50, no clip, despill bias 50 at full strength. Pure green's top row is keyed out, its darker rows keep a share in step with their darkness; the green screen, less saturated than pure green, keeps about half its covering and loses its green spill; skin, the greys and the reds keep all of theirs. | largest difference 1.2e-7 | yes |
| FX-SKEY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-002 frame 0: Screen colour picked from the green screen (40, 180, 60): its top row is keyed out, its darker rows keep a little more each (their green stands less above the rest); pure green, more saturated than the screen, is gone in its top five rows; skin, greys, reds and blues are untouched apart from the despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-003 frame 0: Screen gain 150: more is keyed; the green screen's darker rows fall further and cyan and yellow, which stand half as far above, lose more. | largest difference 1.2e-7 | yes |
| FX-SKEY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-004 frame 0: Screen balance 0: the screen channel is measured against the larger of the other two, so yellow and cyan no longer read as screen. | largest difference 2.1e-7 | yes |
| FX-SKEY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-005 frame 0: Screen balance 100: against the smaller of the other two, so yellow and cyan read as screen as strongly as green. | largest difference 1.8e-7 | yes |
| FX-SKEY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-006 frame 0: Clip black 30, clip white 70: the matte below 0.3 is screen, above 0.7 foreground, stretched between; the green screen's darker rows clean up. | largest difference 1.7e-7 | yes |
| FX-SKEY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-007 frame 0: Clip black 53 and clip white 53: a hard matte, each pixel in or out. | largest difference 1.2e-7 | yes |
| FX-SKEY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-008 frame 0: Despill bias 0: the green is held to the larger of red and blue, the gentlest despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-009 frame 0: Despill bias 100: the green is held to the smaller of red and blue, the strongest despill. | largest difference 1.8e-7 | yes |
| FX-SKEY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-010 frame 0: Despill 0: the matte as FX-SKEY-002, the colours not despilled. | largest difference 1.8e-7 | yes |
| FX-SKEY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-011 frame 0: Blue screen (30, 70, 200) at balance 95, the advice for blue screens: the blue screen and its half-covered copy are keyed, pure blue is gone, and so are cyan, magenta and violet, whose blue stands as far above; pale blue keeps a share; green, yellow and red keep all their covering. | largest difference 1.4e-7 | yes |
| FX-SKEY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-012 frame 0: View screen matte, FX-SKEY-006's settings: the matte as grey, covering kept. | largest difference 1.9e-7 | yes |
| FX-SKEY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-013 frame 0: View status, FX-SKEY-006's settings: white where all foreground, black where all screen, one grey for every blend. | largest difference 3.0e-8 | yes |
| FX-SKEY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-014 frame 0: Shrink/grow -1: the foreground matte shrinks a pixel, so the columns beside the keyed green screen and pure green lose their covering too; the frame's own edge is not eaten. | largest difference 1.7e-7 | yes |
| FX-SKEY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-015 frame 0: Shrink/grow 1.5: the foreground matte grows a pixel and a half, filling the keyed green screen from its neighbours; pure green, between cyan and yellow that are keyed too at the top, stays keyed there. | largest difference 1.7e-7 | yes |
| FX-SKEY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-016 frame 0: Softness 1: the matte blurred at sigma 1, the edge between the screen and the foreground soft. | largest difference 2.0e-7 | yes |
| FX-SKEY-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-017 frame 0: Edge refine 2: Refine Soft Matte's guided fit at radius 2 on the keyed result's edges. | largest difference 1.3e-7 | yes |
| FX-SKEY-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-018 frame 0: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.2e-7 | yes |
| FX-SKEY-018 frame 2: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.8e-7 | yes |
| FX-SKEY-018 frame 4: Screen gain keyed from 0 at frame 0 to 200 at frame 4, linear: nothing keyed at frame 0, twice FX-SKEY-002's at frame 4. | largest difference 1.2e-7 | yes |
| FX-SKEY-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-019 frame 0: FX-SKEY-014 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-SKEY-019 frame 3: FX-SKEY-014 moved three pixels right: the same, moved. | largest difference 1.7e-7 | yes |
| FX-SKEY-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SKEY-020 frame 0: Screen gain 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-020 frame 4: Screen gain 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-021 frame 0: Clip black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-021 frame 4: Clip black -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-022 frame 0: Shrink/grow 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-022 frame 4: Shrink/grow 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-023 frame 0: View "Status": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-023 frame 4: View "Status": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SKEY-024 frame 0: Screen colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-024 frame 4: Screen colour "green", not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.2e-7 | yes |
| FX-SKEY-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_skey_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_skey_004.json is saved with its eleven settings, view final, screen colour #28b43c and screen balance 0 | {"clip_black":0,"clip_white":100,"despill":100,"despill_bias":50,"edge_refine":0,"screen_balance":0,"screen_color":"#28b43c","screen_gain":100,"shrink_grow":0,"softness":0,"view":"final"} | yes |
| fx_skey_020.json is refused in a sentence naming screen gain | Screen Key's screen gain runs from 0 to 200, and this is 201. | yes |
| fx_skey_021.json is refused in a sentence naming clip black | Screen Key's clip black runs from 0 to 100, and this is -1. | yes |
| fx_skey_022.json is refused in a sentence naming shrink grow | Screen Key's shrink grow runs from -100 to 100, and this is 101. | yes |
| fx_skey_023.json is refused in a sentence naming view | Screen Key's view is "final", "screen_matte" or "status", and this is "Status". | yes |
| fx_skey_024.json is refused in a sentence naming screen colour | Screen Key's screen colour is written #rrggbb, and this is "green". | yes |
| a file with a Screen Key whose gain is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Screen Key without its screen colour is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Screen Key without its edge refine is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| screen gain 201 is refused with a sentence, and nothing changes | Screen Key's screen gain runs from 0 to 200, and this is 201. | yes |
| view "Status" is refused with a sentence, and nothing changes | Screen Key's view is "final", "screen_matte" or "status", and this is "Status". | yes |
| softness keyed to 300 is refused with a sentence, and nothing changes | Screen Key's softness runs from 0 to 100, and this is 300. | yes |
| the green screen's colour picked, clip white 70 is taken | taken | yes |
| clip black keyed from 0 to 40 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_skey_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_015.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_016.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_018.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_skey_019.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_skey_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_011.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_skey_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 201383 pixels changed | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 0, Full | largest difference 1 of 255, 4789 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 100, Full | largest difference 1 of 255, 3262 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 239, Full | largest difference 1 of 255, 2879 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 0, Draft | largest difference 1 of 255, 138 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 100, Draft | largest difference 1 of 255, 43 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, gain 120, clip 10 to 90 on three layers, frame 239, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 0, Full | largest difference 1 of 255, 839 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 100, Full | largest difference 1 of 255, 1755 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 239, Full | largest difference 1 of 255, 1065 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 0, Draft | largest difference 1 of 255, 56 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 100, Draft | largest difference 1 of 255, 68 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour sky blue, balance 95, the screen matte on three layers, frame 239, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 268449 pixels changed | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 0, Full | largest difference 1 of 255, 36148 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 100, Full | largest difference 1 of 255, 37764 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 239, Full | largest difference 1 of 255, 35282 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 0, Draft | largest difference 1 of 255, 17556 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 100, Draft | largest difference 1 of 255, 17553 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Screen Key screen colour orange, shrink 2 and softness 2, left to the processor on three layers, frame 239, Draft | largest difference 1 of 255, 17691 pixels differ; 0 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-540 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the green-screen shot with no effect (a face whose edge is spilled green, a blue shirt); draws cleanly | [] | yes |
| 2_as_added.png, as added (screen colour pure green): the screen, less saturated than pure green, half see-through (covering 125 by hand); the face and the shirt whole; draws cleanly | [], screen 125, face 255, shirt 255 | yes |
| 3_picked.png, the screen colour picked from the screen: the screen see-through; the face's middle and the shirt untouched; the spilled edge pixel partly see-through (covering 175 by hand) with its green spill taken out (green 134 or 135 by hand, red 156 and blue 113 kept); draws cleanly | [], screen 0, face [214, 172, 140, 255], shirt [40, 60, 160, 255], spilled [156, 135, 113, 176] | yes |
| 4_clipped.png, clip black 30 and clip white 70: the screen see-through, the spilled edge pixel nearly solid (covering 247 by hand); draws cleanly | [], screen 0, spilled 248 | yes |
| 5_screen_matte.png, the same as View Screen Matte: the screen black, the face and the shirt white, all fully covered; draws cleanly | [], screen [0, 0, 0, 255], face [255, 255, 255, 255], shirt [255, 255, 255, 255] | yes |
| 6_status.png, the same as View Status: the screen black, the face white, the spilled edge pixel (matte 0.97) the one grey, encoded 128; draws cleanly | [], screen [0, 0, 0, 255], face [255, 255, 255, 255], spilled [128, 128, 128, 255] | yes |
| 7_edge.png, then shrink 2 pixels, softness 1.5 and edge refine 3: the screen still see-through, the face's middle and the shirt still whole, the face's edge pulled in and softened; draws cleanly | [], screen 0, face [214, 172, 140, 255], shirt 255 | yes |

## Result

156 of 156 checks pass.
