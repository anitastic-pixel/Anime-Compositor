# D-702: Reshape's Correspondence Points and Interpolation

B-572: `core.reshape` gains `correspondence_points` (pairs of per cent places along the source and destination outlines that answer each other; none is D-606's matching) and `interpolation` (Discrete, Linear or Smooth: Percent between two of its keys worked as a blend of the fields at the keys' Percents). Every expected pixel is `Fixtures/reshape_points/expected_reshape_points.json`, written by `tools/reshape_points_reference.py` before this code existed and printed in document 25 as FX-RESHP-001 to 015. Tolerance 2e-5.

## FX-RESHP-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RESHP-001 frame 0: D-606's FX-RESHAPE-002 (the square pulled three pixels right) with one correspondence pair, (0, 0): the first points answer each other, so FX-RESHAPE-002's frame. | largest difference 2.5e-7 | yes |
| FX-RESHP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-002 frame 0: One pair, (0, 25): the source's first corner answers the destination's second, so the square is pulled right and turned a quarter round. | largest difference 2.3e-7 | yes |
| FX-RESHP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-003 frame 0: Two pairs, (0, 0) and (50, 25): the source's first half is squeezed into the destination's first quarter. | largest difference 3.1e-7 | yes |
| FX-RESHP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-004 frame 0: The destination drawn the other way round with pairs (0, 0) and (25, 75): 75 per cent along it as drawn is 25 per cent along it turned, so FX-RESHAPE-002's frame. | largest difference 2.5e-7 | yes |
| FX-RESHP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-005 frame 0: Percent keyed 0 at frame 0 to 100 at frame 4, Interpolation Linear: frame 2 is half of frame 4's field, not D-606's Percent 50. | largest difference 1.9e-7 | yes |
| FX-RESHP-005 frame 1: Percent keyed 0 at frame 0 to 100 at frame 4, Interpolation Linear: frame 2 is half of frame 4's field, not D-606's Percent 50. | largest difference 2.5e-7 | yes |
| FX-RESHP-005 frame 2: Percent keyed 0 at frame 0 to 100 at frame 4, Interpolation Linear: frame 2 is half of frame 4's field, not D-606's Percent 50. | largest difference 2.5e-7 | yes |
| FX-RESHP-005 frame 4: Percent keyed 0 at frame 0 to 100 at frame 4, Interpolation Linear: frame 2 is half of frame 4's field, not D-606's Percent 50. | largest difference 2.5e-7 | yes |
| FX-RESHP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-006 frame 0: The same keys eased past their end, Linear: the ease is not read, so FX-RESHP-005's frames. | largest difference 1.9e-7 | yes |
| FX-RESHP-006 frame 1: The same keys eased past their end, Linear: the ease is not read, so FX-RESHP-005's frames. | largest difference 2.5e-7 | yes |
| FX-RESHP-006 frame 2: The same keys eased past their end, Linear: the ease is not read, so FX-RESHP-005's frames. | largest difference 2.5e-7 | yes |
| FX-RESHP-006 frame 4: The same keys eased past their end, Linear: the ease is not read, so FX-RESHP-005's frames. | largest difference 2.5e-7 | yes |
| FX-RESHP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-007 frame 0: Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames 1 and 3 blended through all three keys. | largest difference 1.9e-7 | yes |
| FX-RESHP-007 frame 1: Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames 1 and 3 blended through all three keys. | largest difference 2.5e-7 | yes |
| FX-RESHP-007 frame 2: Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames 1 and 3 blended through all three keys. | largest difference 2.5e-7 | yes |
| FX-RESHP-007 frame 3: Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames 1 and 3 blended through all three keys. | largest difference 2.5e-7 | yes |
| FX-RESHP-007 frame 4: Percent keyed 0, 100, 40 at frames 0, 2, 4, Interpolation Smooth: frames 1 and 3 blended through all three keys. | largest difference 2.5e-7 | yes |
| FX-RESHP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-008 frame 1: The same keys, Linear: frames 1 and 3 differ from Smooth's. | largest difference 2.5e-7 | yes |
| FX-RESHP-008 frame 3: The same keys, Linear: frames 1 and 3 differ from Smooth's. | largest difference 2.5e-7 | yes |
| FX-RESHP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-009 frame 0: Discrete written out, Percent keyed 0 to 100: D-606's FX-RESHAPE-010. | largest difference 1.9e-7 | yes |
| FX-RESHP-009 frame 2: Discrete written out, Percent keyed 0 to 100: D-606's FX-RESHAPE-010. | largest difference 2.5e-7 | yes |
| FX-RESHP-009 frame 4: Discrete written out, Percent keyed 0 to 100: D-606's FX-RESHAPE-010. | largest difference 2.5e-7 | yes |
| FX-RESHP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-010 frame 0: Smooth with Percent not keyed (100): FX-RESHAPE-002's frame. | largest difference 2.5e-7 | yes |
| FX-RESHP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-011 frame 0: The pairs keyed from (0, 0) at frame 0 to (0, 25) at frame 4: frame 0 is FX-RESHAPE-002's, frame 4 FX-RESHP-002's. | largest difference 2.5e-7 | yes |
| FX-RESHP-011 frame 4: The pairs keyed from (0, 0) at frame 0 to (0, 25) at frame 4: frame 0 is FX-RESHAPE-002's, frame 4 FX-RESHP-002's. | largest difference 2.3e-7 | yes |
| FX-RESHP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RESHP-012 frame 0: Correspondence points 0, 0, 50: three numbers, not pairs. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-012 frame 4: Correspondence points 0, 0, 50: three numbers, not pairs. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHP-013 frame 0: A correspondence point at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-013 frame 4: A correspondence point at 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHP-014 frame 0: 33 pairs, more than 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-014 frame 4: 33 pairs, more than 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RESHP-015 frame 0: Interpolation "cubic", not one of the three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-015 frame 4: Interpolation "cubic", not one of the three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-RESHP-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_reshp_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_reshp_003.json is saved with its seven settings, the pairs one flat list, the interpolation a word | {"boundary_mask":0,"correspondence_points":[0,0,50,25],"destination_mask":2,"elasticity":"normal","interpolation":"discrete","percent":100,"source_mask":1} | yes |
| fx_reshp_012.json is refused in a sentence naming Reshape and its correspondence points are none, or pairs | Reshape's correspondence points are none, or pairs of numbers, at most 32 pairs, and this has 3 numbers. | yes |
| fx_reshp_013.json is refused in a sentence naming Reshape and its correspondence points | Reshape's correspondence points runs from 0 to 100, and this is 101. | yes |
| fx_reshp_014.json is refused in a sentence naming Reshape and its at most 32 pairs | Reshape's correspondence points are none, or pairs of numbers, at most 32 pairs, and this has 66 numbers. | yes |
| fx_reshp_015.json is refused in a sentence naming Reshape and its interpolation is discrete, linear or smooth | Reshape's interpolation is discrete, linear or smooth, and this is "cubic". | yes |
| a file with a Reshape whose interpolation is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Reshape whose correspondence points are one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| three numbers for the correspondence points is refused with a sentence, and nothing changes | Reshape's correspondence points are none, or pairs of numbers, at most 32 pairs, and this has 3 numbers. | yes |
| interpolation cubic is refused with a sentence, and nothing changes | Reshape's interpolation is discrete, linear or smooth, and this is "cubic". | yes |
| the pairs keyed to end at 101 is refused with a sentence, and nothing changes | Reshape's correspondence points runs from 0 to 100, and this is 101. | yes |
| interpolation smooth is taken | taken | yes |
| the pairs keyed from (0, 0), (50, 25) to (0, 25), (50, 50) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_reshp_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshp_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshp_007.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |
| fx_reshp_007.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RESHP-001, fx_reshp_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-002, fx_reshp_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-003, fx_reshp_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-004, fx_reshp_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-005, fx_reshp_005.json, frames [0, 1, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 6 of 8 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-006, fx_reshp_006.json, frames [0, 1, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 6 of 8 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-007, fx_reshp_007.json, frames [0, 1, 2, 3, 4] at Full and Draft | largest difference 1 of 255; on the card in 8 of 10 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-008, fx_reshp_008.json, frames [1, 3] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-009, fx_reshp_009.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 4 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-010, fx_reshp_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-011, fx_reshp_011.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-012, fx_reshp_012.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-013, fx_reshp_013.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-014, fx_reshp_014.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-RESHP-015, fx_reshp_015.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25): the processor's frame 100 differs from the shot without it | 1881377 pixels changed | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 0, Full | largest difference 1 of 255, 744 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 100, Full | largest difference 1 of 255, 933 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 239, Full | largest difference 1 of 255, 663 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 0, Draft | largest difference 1 of 255, 46 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, the square to the square moved right, pairs (0, 0) and (50, 25), frame 239, Draft | largest difference 1 of 255, 35 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth: the processor's frame 100 differs from the shot without it | 1818476 pixels changed | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 0, Full | largest difference 1 of 255, 2110 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 100, Full | largest difference 1 of 255, 812 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 239, Full | largest difference 1 of 255, 803 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 0, Draft | largest difference 1 of 255, 90 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 100, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Reshape, Percent keyed 20 to 100 and back to 40, Smooth, frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/D-702 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect (its two masks cut nothing); draws cleanly | [] | yes |
| 2_matched_from_the_start.png, the square pulled 60 right with no pairs; one pair (0, 0) draws it byte for byte; both draw cleanly | [] [], 31846 pixels differ from the street, 0 between the two | yes |
| 3_turned_a_quarter.png, the same with the pair (0, 25): the source's top left corner answers the destination's top right, so the middle swirls a quarter turn on its way; differs from 2; draws cleanly | [], 84708 pixels differ from 2 | yes |
| 4_discrete_frame_3.png, 5_linear_frame_3.png and 6_smooth_frame_3.png: Percent keyed 0, 100, 40 at frames 0, 2, 4, drawn at frame 3 Discrete (one pull of 70 per cent), Linear (the pulls of 100 and 40 per cent, half each) and Smooth (the curve through the keys): Linear differs from Discrete and Smooth from Linear; all draw cleanly | [] [] []; Linear against Discrete 5174 pixels, Smooth against Linear 9360 | yes |

## Result

113 of 113 checks pass.
