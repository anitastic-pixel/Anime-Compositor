# B-91: diffusion

D-148, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the fifteenth of the third batch. Every expected pixel is `Fixtures/diffusion/expected_diffusion.json`, written by `tools/diffusion_reference.py` before this code existed and printed in document 25 as FX-DIFFUSE-001 to 018. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-DIFFUSE-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-DIFFUSE-001 frame 0: The settings as they start, radius 10, amount 50, screen: every pixel that shows is lightened, never darkened, by half its screen with the glow; every pixel of the dark shadow and the line lifts more than any of the pale skin or the light, whose red, already full, stays at 1; the coverings are kept, and the empty pixels, the hole in the skin among them, stay empty. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-002 frame 0: Amount 100: the whole screen laid on, every pixel lifted further than in FX-DIFFUSE-001, which is exactly halfway between the drawing and this. | largest difference 1.6e-7 | yes |
| FX-DIFFUSE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-003 frame 0: Amount 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-004 frame 0: Radius 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-005 frame 0: Radius 3, sigma 1: a tight glow. The shadow just left of the light, at (2, 4), lifts more than at radius 10, and the shadow in the far corner, at (1, 1), less than a fifth as much. | largest difference 1.6e-7 | yes |
| FX-DIFFUSE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-006 frame 0: Radius 20, sigma 6.67: a glow as wide as the cel, nearly the same colour everywhere, so the most-lifted shadow pixel lifts less than 1.3 times the least (at radius 10, more than twice). | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-007 frame 0: Blend lighten, amount 100: each channel takes the larger of its own and the glow's, so the shadow and the line lift, and the light, brighter in every channel than the glow round it, stays exactly as it is. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-008 frame 0: Blend normal, amount 100: every pixel that shows takes the glow's own colour at its own covering, the picture blurred. The soft edge, where the blur covers less than 0.4 as nothing lies to its right, takes the colour of what is near it, divided by that covering and so not darkened by the emptiness, and the hole stays empty. | largest difference 2.0e-7 | yes |
| FX-DIFFUSE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-009 frame 0: Blend normal, amount 50: halfway between the drawing and FX-DIFFUSE-008, so the light darkens as the shadow lightens. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-010 frame 0: Radius keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-DIFFUSE-001 and frame 4 is FX-DIFFUSE-006. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-010 frame 2: Radius keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-DIFFUSE-001 and frame 4 is FX-DIFFUSE-006. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-010 frame 4: Radius keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-DIFFUSE-001 and frame 4 is FX-DIFFUSE-006. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-011 frame 0: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing, and at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-DIFFUSE-002. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-011 frame 2: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing, and at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-DIFFUSE-002. | largest difference 1.6e-7 | yes |
| FX-DIFFUSE-011 frame 4: Amount keyed from 0 at frame 0 to 100 at frame 4, eased past its end: frame 0 is the drawing, and at frame 2 it has gone past 100 and is held there, so frames 2 and 4 are both FX-DIFFUSE-002. | largest difference 1.6e-7 | yes |
| FX-DIFFUSE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-012 frame 0: FX-DIFFUSE-001 moved three pixels right: the same, moved, the glow worked on the drawing before it is placed. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-012 frame 3: FX-DIFFUSE-001 moved three pixels right: the same, moved, the glow worked on the drawing before it is placed. | largest difference 1.8e-7 | yes |
| FX-DIFFUSE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-DIFFUSE-013 frame 0: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-013 frame 4: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-014 frame 0: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-014 frame 4: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-015 frame 0: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-015 frame 4: Amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-016 frame 0: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-016 frame 4: Amount -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-017 frame 0: Blend "add", which is not screen, lighten or normal. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-017 frame 4: Blend "add", which is not screen, lighten or normal. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-DIFFUSE-018 frame 0: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-018 frame 4: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-DIFFUSE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview halves the radius, 10 to 5, and nothing else | Diffusion { radius: 5.0, amount: 50.0, blend: "screen" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_diffuse_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_diffuse_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `blend` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a radius that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| radius 501 is refused with a sentence, and nothing changes | Diffusion's radius runs from 0 to 500, and this is 501. | yes |
| radius -1 is refused with a sentence, and nothing changes | Diffusion's radius runs from 0 to 500, and this is -1. | yes |
| amount 101 is refused with a sentence, and nothing changes | Diffusion's amount runs from 0 to 100, and this is 101. | yes |
| amount -1 is refused with a sentence, and nothing changes | Diffusion's amount runs from 0 to 100, and this is -1. | yes |
| blend "add" is refused with a sentence, and nothing changes | Diffusion's blend is "screen", "lighten" or "normal", and this is "add". | yes |
| radius keyed to 600 is refused with a sentence, and nothing changes | Diffusion's radius runs from 0 to 500, and this is 600. | yes |
| radius 0 and amount 0, the bottoms, is taken | taken | yes |
| radius 500 and amount 100, the tops, is taken | taken | yes |
| radius keyed from 0 to 20 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_diffuse_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_diffuse_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_diffuse_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

75 of 75 checks pass.
