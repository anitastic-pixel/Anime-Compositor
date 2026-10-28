# B-811: Set Matte

D-931, accepted on 2026-10-27 under the owner's request for the tutorial follow-alongs. Every expected pixel is `Fixtures/set_matte/expected_set_matte.json`, written by `tools/set_matte_reference.py` before this code existed and printed in document 25 as FX-SETMATTE-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-SETMATTE-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SETMATTE-001 frame 0: As added: no layer named, so nothing changes. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-002 frame 0: The key's alpha, composited with the original: solid left of x = 12, part covering to 14, clear after; the shot's clear corner stays clear. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-003 frame 0: FX-SETMATTE-002 without compositing: the key's alpha replaces the shot's, so the shot's clear corner shows black where the key covers. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-004 frame 0: The key's red, falling left to right. | largest difference 2.1e-7 | yes |
| FX-SETMATTE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-005 frame 0: The key's green, rising top to bottom. | largest difference 2.0e-7 | yes |
| FX-SETMATTE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-006 frame 0: The key's blue. | largest difference 2.0e-7 | yes |
| FX-SETMATTE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-007 frame 0: The key's luminance. | largest difference 1.8e-7 | yes |
| FX-SETMATTE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-008 frame 0: The key's hue. | largest difference 1.8e-7 | yes |
| FX-SETMATTE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-009 frame 0: The key's lightness. | largest difference 1.8e-7 | yes |
| FX-SETMATTE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-010 frame 0: The key's saturation. | largest difference 5.5e-7 | yes |
| FX-SETMATTE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-011 frame 0: The key's luminance not premultiplied: the part-covering columns read their full colour, so they keep more than in FX-SETMATTE-007. | largest difference 1.8e-7 | yes |
| FX-SETMATTE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-012 frame 0: The key's alpha inverted: only what the key leaves clear is kept. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-013 frame 0: The key's middle 8 by 6, not stretched: centred, it lies where it was cut, and outside it everything is clear. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-014 frame 0: The middle 8 by 6's red stretched to 16 by 10: nothing the shot covers turns clear, and the ramp is spread twice as wide. | largest difference 1.7e-7 | yes |
| FX-SETMATTE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-015 frame 0: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-015 frame 4: A layer that is not in the composition, `gone`: nothing changes, and the warning every frame. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-SETMATTE-016 frame 0: The key with its in point at frame 4: before it the key is empty, so at frame 0 everything is clear; at frame 4 it is FX-SETMATTE-002. | largest difference 0.0e0 | yes |
| FX-SETMATTE-016 frame 4: The key with its in point at frame 4: before it the key is empty, so at frame 0 everything is clear; at frame 4 it is FX-SETMATTE-002. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-017 frame 0: FX-SETMATTE-004 on the holder moved 2 right and 1 down: the same picture moved, since the key lies on the layer. | largest difference 2.1e-7 | yes |
| FX-SETMATTE-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-018 frame 0: The effect on an adjustment layer above the holder, the key's alpha: the frame below is the holder's own rectangle, so this is FX-SETMATTE-002. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SETMATTE-019 frame 0: A matte channel written "chroma". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-019 frame 4: A matte channel written "chroma". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SETMATTE-020 frame 0: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-020 frame 4: Invert written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SETMATTE-021 frame 0: Stretch written "tile". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-021 frame 4: Stretch written "tile". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SETMATTE-022 frame 0: Composite written "1". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-022 frame 4: Composite written "1". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SETMATTE-023 frame 0: Premultiply written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-023 frame 4: Premultiply written "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SETMATTE-024 frame 0: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-024 frame 4: A layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SETMATTE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Files whose layers read each other (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| cycle_two.json: `a` and `b`, each the shot, each with a Set Matte whose matte layer is the other: refused, `EFFECT_LAYER_CYCLE`. | EFFECT_LAYER_CYCLE This project cannot be opened, because layers' effects read each other in a circle. | yes |

## Wrong words are refused (what each warns)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_setmatte_019.json: the warning names the use for matte | Set Matte's use for matte is one of "alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", and this is "chroma". | yes |
| fx_setmatte_020.json: the warning names the invert matte | Set Matte's invert matte is "off" or "on", and this is "yes". | yes |
| fx_setmatte_021.json: the warning names the stretch matte to fit | Set Matte's stretch matte to fit is "off" or "on", and this is "tile". | yes |
| fx_setmatte_022.json: the warning names the composite matte with original | Set Matte's composite matte with original is "off" or "on", and this is "1". | yes |
| fx_setmatte_023.json: the warning names the premultiply matte layer | Set Matte's premultiply matte layer is "off" or "on", and this is "maybe". | yes |
| fx_setmatte_024.json: the warning names the matte layer | Set Matte's matte layer is the name of a layer of this composition, and this is 3. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_setmatte_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_setmatte_013.json is saved with its six settings, the layer as written | {"composite_matte_with_original":"on","invert_matte":"off","layer":"patch","premultiply_matte_layer":"on","stretch_matte_to_fit":"off","use_for_matte":"alpha"} | yes |
| it grows the drawing's bounds by nothing: the layer keeps its size | 0 | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_setmatte_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_016.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_017.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_setmatte_018.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

96 of 96 checks pass.
