# D-321: Gaussian Blur in After Effects' Blurriness

From D-308, held back for want of an After Effects frame, then settled on 2026-10-04 from two players of After Effects' own files: lottie-web ("Empirical value, matching AE's blur appearance", 0.3) and Skia's Skottie ("Close-enough to AE", 0.3). Gaussian Blur gains Units: Blurriness, which a new one takes, is After Effects' number, sigma 0.3 times it, its kernel reaching 6.5 sigmas so a strong Exposure after it finds no box; Sigma, what a file without units means, is document 21's rule as before. Every expected pixel is `Fixtures/blurriness/expected_blurriness.json`, written by `tools/blurriness_reference.py` before the build had Blurriness, printed in document 25 as FX-BLURRY-001 to 009. Tolerance 2e-5.

## FX-BLURRY-001 to 009 (document 25)

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
| fx_blurry_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blurry_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blurry_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blurry_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blurry_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_blurry_002.json, a blur with no units as before D-321, is saved without units | {"sigma_px":10} | yes |
| A blur in Blurriness is saved as units: blurriness | {"sigma_px":10,"units":"blurriness"} | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| units "pixels" is refused with a sentence, and nothing changes | Gaussian Blur's units are "sigma" or "blurriness", and this is "pixels". | yes |
| units "Blurriness", written with a capital is refused with a sentence, and nothing changes | Gaussian Blur's units are "sigma" or "blurriness", and this is "Blurriness". | yes |
| FX-BLURRY-002 set to Blurriness, is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| FX-BLURRY-002 set to Blurriness by the command draws FX-BLURRY-001's frame | largest difference 2.9e-8 | yes |

## The preview card

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blurry_001.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_blurry_002.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_blurry_004.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |
| fx_blurry_006.json: drawn on the card, within 1 level of 255 of the CPU | card, largest difference 0 of 255 | yes |

## Pictures: a white square, in `verification/D-321 pictures/`, over black

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_square.png, the square with no effect; draws cleanly | [], the middle 255 of 255, 14 pixels left of the square 0, the last lit pixel right of it 255 | yes |
| 2_old_blur_20.png, Gaussian Blur 20 in an old file (sigma 20): very soft, the square's middle well below white; draws cleanly | [], the middle 125 of 255, 14 pixels left of the square 89, the last lit pixel right of it 1 | yes |
| 3_blurriness_20.png, Gaussian Blur at Blurriness 20, as After Effects reads it (sigma 6): the square kept, its edges softened; draws cleanly | [], the middle 245 of 255, 14 pixels left of the square 28, the last lit pixel right of it 1 | yes |
| 4_old_cut_exposure_20.png, the same sigma 6 cut at three sigmas, then Exposure +20 in Float: the hard-edged box P-26 saw; draws cleanly | [], the middle 255 of 255, 14 pixels left of the square 255, the last lit pixel right of it 255 | yes |
| 5_blurriness_20_exposure_20.png, Blurriness 20 then Exposure +20 in Float: the glow fades out with no box; draws cleanly | [], the middle 255 of 255, 14 pixels left of the square 255, the last lit pixel right of it 1 | yes |
| Blurriness 20 keeps the square brighter than the old Blur 20 (After Effects' 20 is a third as soft) | 245 against 125 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_blurry_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blurry_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_blurry_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

45 of 45 checks pass.
