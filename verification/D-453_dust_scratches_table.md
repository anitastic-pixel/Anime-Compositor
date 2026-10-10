# D-453: Dust & Scratches

B-333, after After Effects' Dust & Scratches: D-203's Median with a threshold. Each channel of a pixel's straight colour takes the median of the taps within Radius that show when their 8-bit values (D-88's, through the sRGB curve) are more than Threshold apart, and keeps its own otherwise; with Operate on Alpha on the covering is worked the same way against the median of every tap's. So specks go and grain within Threshold stays. Every expected pixel is `Fixtures/dust_scratches/expected_dust_scratches.json`, written by `tools/dust_scratches_reference.py` before this code existed and printed in document 25 as FX-DUST-001 to 018. Tolerance 2e-5.

## FX-DUST-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DUST-001 frame 0: Dust & Scratches as it starts, Radius 1, Threshold 0 and Operate on Alpha off: a disc of five, the pixel and its four neighbours, so the three specks are gone into the skin and the line, two pixels wide, is kept; the hole stays a hole and the half-covered column stays at half. | largest difference 1.9e-7 | yes |
| FX-DUST-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-002 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DUST-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-003 frame 0: Radius 2, Threshold 0: D-203's Median at Radius 2, FX-MEDIAN-001, the specks gone and the grain mostly skin. | largest difference 1.9e-7 | yes |
| FX-DUST-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-004 frame 0: Radius 2, Threshold 16: the dark specks, far more than 16 from the skin, are gone as in FX-DUST-003, and the white speck, 9, 41 and 65 above the skin in red, green and blue, keeps its red and takes the skin's green and blue; the grain, 8 from the skin, is kept just as drawn. | largest difference 1.9e-7 | yes |
| FX-DUST-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-005 frame 0: Radius 2, Threshold 64: the dark specks are gone; the white speck, 9, 41 and 65 above the skin in red, green and blue, keeps its red and green and takes the skin's blue, the one channel more than 64 apart, so it turns yellow. The grain is kept. | largest difference 1.9e-7 | yes |
| FX-DUST-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-006 frame 0: Radius 2, Threshold 255: nothing is ever more than 255 apart, so the drawing is unchanged. | largest difference 1.9e-7 | yes |
| FX-DUST-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-007 frame 0: Radius 2, Threshold 16, Operate on Alpha on: the specks go as in FX-DUST-004 and the hole, 255 from the skin round it in its covering, is filled with skin; the drawing's top-left corner, with more of its disc outside the drawing than in, is cut away, and the grain is kept. | largest difference 1.9e-7 | yes |
| FX-DUST-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-008 frame 0: Radius 2, Threshold 0, Operate on Alpha off: the hole stays a hole, for a pixel keeps its own covering. | largest difference 1.9e-7 | yes |
| FX-DUST-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-009 frame 0: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear, at Radius 2: frame 0 is FX-DUST-003 and frame 4 is FX-DUST-004; at frame 2, Threshold 8, the grain 8 from the median is kept. | largest difference 1.9e-7 | yes |
| FX-DUST-009 frame 2: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear, at Radius 2: frame 0 is FX-DUST-003 and frame 4 is FX-DUST-004; at frame 2, Threshold 8, the grain 8 from the median is kept. | largest difference 1.9e-7 | yes |
| FX-DUST-009 frame 4: Threshold keyed from 0 at frame 0 to 16 at frame 4, linear, at Radius 2: frame 0 is FX-DUST-003 and frame 4 is FX-DUST-004; at frame 2, Threshold 8, the grain 8 from the median is kept. | largest difference 1.9e-7 | yes |
| FX-DUST-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-010 frame 0: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2), at Threshold 0: frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering. | largest difference 1.9e-7 | yes |
| FX-DUST-010 frame 2: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2), at Threshold 0: frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering. | largest difference 1.9e-7 | yes |
| FX-DUST-010 frame 4: Radius keyed from 0 at frame 0 to 10 at frame 4, eased past its end (about 13 at frame 2), at Threshold 0: frame 2 is held at 10, the same as frame 4, where every pixel that shows is the skin at its own covering. | largest difference 1.9e-7 | yes |
| FX-DUST-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-011 frame 0: Dust & Scratches as it starts, the layer moved three pixels right: FX-DUST-001 moved with it. | largest difference 1.9e-7 | yes |
| FX-DUST-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DUST-012 frame 0: Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-012 frame 4: Radius 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-013 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-013 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-014 frame 0: Radius keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-014 frame 4: Radius keyed to 20 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-015 frame 0: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-015 frame 4: Threshold 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-016 frame 0: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-016 frame 4: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-017 frame 0: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-017 frame 4: Threshold keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DUST-018 frame 0: Operate on Alpha "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-018 frame 4: Operate on Alpha "sometimes", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DUST-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dust_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_dust_009.json is saved with its word and numbers as written, and the threshold's keys kept | {"operate_on_alpha":"off","radius":2,"threshold":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":16}]}} | yes |
| fx_dust_007.json is saved with its word and numbers as written | {"operate_on_alpha":"on","radius":2,"threshold":16} | yes |
| fx_dust_012.json is refused in a sentence | Dust & Scratches's radius runs from 0 to 10, and this is 11. | yes |
| fx_dust_013.json is refused in a sentence | Dust & Scratches's radius runs from 0 to 10, and this is -1. | yes |
| fx_dust_015.json is refused in a sentence | Dust & Scratches's threshold runs from 0 to 255, and this is 256. | yes |
| fx_dust_016.json is refused in a sentence | Dust & Scratches's threshold runs from 0 to 255, and this is -1. | yes |
| fx_dust_018.json is refused in a sentence | Dust & Scratches's operate on alpha is "off" or "on", and this is "sometimes". | yes |
| a file with a Dust & Scratches with no `operate_on_alpha` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Dust & Scratches whose operate on alpha is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Dust & Scratches with no `threshold` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 10.5 is refused with a sentence, and nothing changes | Dust & Scratches's radius runs from 0 to 10, and this is 10.5. | yes |
| radius -0.5 is refused with a sentence, and nothing changes | Dust & Scratches's radius runs from 0 to 10, and this is -0.5. | yes |
| threshold 255.5 is refused with a sentence, and nothing changes | Dust & Scratches's threshold runs from 0 to 255, and this is 255.5. | yes |
| threshold -2 is refused with a sentence, and nothing changes | Dust & Scratches's threshold runs from 0 to 255, and this is -2. | yes |
| operate on alpha "On" is refused with a sentence, and nothing changes | Dust & Scratches's operate on alpha is "off" or "on", and this is "On". | yes |
| threshold keyed to 400 is refused with a sentence, and nothing changes | Dust & Scratches's threshold runs from 0 to 255, and this is 400. | yes |
| radius 4, threshold 30, operate on alpha on is taken | taken | yes |
| radius keyed from 0 to 10 is taken | taken | yes |
| threshold keyed from 0 to 255 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dust_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_009.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_010.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_dust_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_dust_001.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_002.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_003.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_004.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_005.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_006.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_007.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_008.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_009.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_010.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; the same warnings: true | yes |
| fx_dust_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_dust_012.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_013.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_014.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_015.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_016.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_017.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_dust_018.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1171701 pixels changed | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 0, Full | largest difference 1 of 255, 3814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 100, Full | largest difference 1 of 255, 1731 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 239, Full | largest difference 1 of 255, 1824 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 0, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 100, Draft | largest difference 1 of 255, 24 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 0 on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 145378 pixels changed | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 0, Full | largest difference 1 of 255, 3786 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 100, Full | largest difference 1 of 255, 1637 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 239, Full | largest difference 1 of 255, 1781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 0, Draft | largest difference 1 of 255, 133 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 100, Draft | largest difference 1 of 255, 29 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 6, Threshold 20 on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 226711 pixels changed | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 0, Full | largest difference 1 of 255, 3909 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 100, Full | largest difference 1 of 255, 1685 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 239, Full | largest difference 1 of 255, 1905 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 0, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 100, Draft | largest difference 1 of 255, 20 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold 8, Operate on Alpha on on three layers, frame 239, Draft | largest difference 1 of 255, 48 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 75895 pixels changed | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 0, Full | largest difference 1 of 255, 3814 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 100, Full | largest difference 1 of 255, 1477 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 239, Full | largest difference 1 of 255, 1772 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 0, Draft | largest difference 1 of 255, 131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 100, Draft | largest difference 1 of 255, 27 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Dust & Scratches Radius 4, Threshold keyed 0 to 60 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street, in `verification/D-453 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_radius_2.png, frame 0, Radius 2, Threshold 0: fine detail evened with its neighbours, the covering kept; draws cleanly | [], 1390 pixels changed of 129600, the largest by 190 of 255 | yes |
| 3_radius_3.png, frame 0, Radius 3, Threshold 0: more evened, as Median at 3, the covering kept; draws cleanly | [], 3793 pixels changed of 129600, the largest by 190 of 255 | yes |
| 4_radius_3_threshold_40.png, frame 0, Radius 3, Threshold 40: only what stands out by more than 40 is changed, so far fewer pixels than at 0; draws cleanly | [], 1330 pixels changed of 129600, the largest by 190 of 255 | yes |
| 5_threshold_255.png, frame 0, Radius 3, Threshold 255: the street untouched; draws cleanly | [], 0 pixels changed of 129600, the largest by 0 of 255 | yes |
| 3_radius_3.png changes more pixels than 4_radius_3_threshold_40.png: the threshold keeps what is close to its neighbours | 3793 pixels changed at Threshold 0, 1330 at 40 | yes |

## Result

144 of 144 checks pass.
