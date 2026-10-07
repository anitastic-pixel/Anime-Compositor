# B-217: Exposure's Offset, Gamma Correction and Bypass

D-335, from P-26's tutorial 2. Every expected pixel is `Fixtures/exposure_ae/expected_exposure_ae.json`, written by `tools/exposure_ae_reference.py` before this code existed and printed in document 25 as FX-EXPAE-001 to 016. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-EXPAE-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-EXPAE-001 frame 0: Float, gamma 1.69 alone: each straight colour to the power 1 / 1.69, brighter in the middle, black and white held. | largest difference 1.3e-7 | yes |
| FX-EXPAE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-002 frame 0: Float, offset 0.1 alone: 0.1 added to each straight colour, so the patches are lifted and nothing is black, though the empty space stays empty. | largest difference 1.8e-7 | yes |
| FX-EXPAE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-003 frame 0: Float, Exposure +1, offset -0.2, gamma 2: in that order, doubled, lowered and then the square root; where the offset takes a colour below 0 it stays below 0. | largest difference 1.7e-7 | yes |
| FX-EXPAE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-004 frame 0: Float, the tutorial's ground texture, Exposure 2.47, gamma 1.69, on the half-covering patches. | largest difference 2.4e-7 | yes |
| FX-EXPAE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-005 frame 0: 32 bpc (After Effects), the tutorial's ground texture, Exposure 2.47, gamma 1.69: through D-333's 2.2 curve. | largest difference 4.3e-7 | yes |
| FX-EXPAE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-006 frame 0: 32 bpc (After Effects) with bypass on: Exposure +1, offset 0.05, gamma 1.2 on the display values themselves. | largest difference 5.3e-7 | yes |
| FX-EXPAE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-007 frame 0: Float with bypass on: Float is linear light already, so this is FX-EXPAE-003 exactly. | largest difference 1.7e-7 | yes |
| FX-EXPAE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-008 frame 0: 8 bpc, Exposure +1, gamma 1.69: linear light as D-330's Exposure, then held to 8 bits. | largest difference 1.4e-7 | yes |
| FX-EXPAE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-009 frame 0: 8 bpc with bypass on, Exposure +1, gamma 1.69: on the display values, then held to 8 bits. | largest difference 1.9e-7 | yes |
| FX-EXPAE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-010 frame 0: Float, gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2. | largest difference 1.5e-7 | yes |
| FX-EXPAE-010 frame 2: Float, gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2. | largest difference 8.1e-8 | yes |
| FX-EXPAE-010 frame 4: Float, gamma keyed from 1 at frame 0 to 3 at frame 4, linear: frame 0 is the drawing, frame 2 is gamma 2. | largest difference 7.5e-8 | yes |
| FX-EXPAE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-011 frame 0: 32 bpc (After Effects), bypass written "off" and offset 0, gamma 1 written: Exposure +1 as D-333's. | largest difference 1.1e-7 | yes |
| FX-EXPAE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-EXPAE-012 frame 0: Gamma 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-012 frame 4: Gamma 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-012: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXPAE-013 frame 0: Gamma 10, above 9.99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-013 frame 4: Gamma 10, above 9.99. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXPAE-014 frame 0: Offset 0.6, above 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-014 frame 4: Offset 0.6, above 0.5. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXPAE-015 frame 0: Offset keyed to -1 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-015 frame 4: Offset keyed to -1 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-EXPAE-016 frame 0: Bypass "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-016 frame 4: Bypass "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-EXPAE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_expae_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_expae_001.json, which writes no offset and no bypass, is saved without them | {"gamma":1.69,"stops":0} | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_expae_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_expae_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: the town at 32 bpc (After Effects), in `verification/D-335 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_none.png, no Exposure; draws cleanly | [] | yes |
| 2_exposure.png, Exposure 2.47 alone, as before D-335; draws cleanly | [] | yes |
| 3_gamma.png, Exposure 2.47 and Gamma Correction 1.69, tutorial 2's ground; draws cleanly | [] | yes |
| 4_offset.png, Offset -0.1 alone: the darks go black; draws cleanly | [] | yes |
| On the road the gamma lifts the grey above Exposure alone, and Offset -0.1 darkens it | red: none 60, Exposure 131, with gamma 172, offset 0 | yes |

## Result

53 of 53 checks pass.
