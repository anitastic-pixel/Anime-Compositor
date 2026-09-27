# B-75: light wrap

D-132, accepted by the owner on 2026-09-26, the last of the second batch of ten, with its light read from everything beneath the layer, the owner's answer the same day. Every expected pixel is `Fixtures/light_wrap/expected_light_wrap.json`, written by `tools/light_wrap_reference.py` before this code existed and printed in document 25 as FX-WRAP-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-WRAP-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WRAP-001 frame 0: The defaults, width 10, intensity 100, screen, on the box over the bands: every pixel of the box takes the bands' light, blurred at sigma 3.33, most at its edges and least in its middle, each colour from the bands nearest it; its covering is unchanged, and the bands around it are untouched. | largest difference 1.6e-7 | yes |
| FX-WRAP-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-002 frame 0: Width 3: the light reaches about a pixel in. The box's corner takes more than twenty times the light its middle does. | largest difference 2.1e-7 | yes |
| FX-WRAP-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-003 frame 0: Width 20, sigma 6.67: most of the blur falls past the 16 by 10 frame, where nothing is beneath, so less light arrives than at width 10 and the box takes it almost evenly. The frame's edge cuts the light off, as D-66 cuts an adjustment layer's blur. | largest difference 1.8e-7 | yes |
| FX-WRAP-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-004 frame 0: Intensity 400: four times the light, held at white before the screen; every pixel at least as bright as FX-WRAP-001's. | largest difference 1.4e-7 | yes |
| FX-WRAP-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-005 frame 0: Intensity 0: nothing changes. | largest difference 1.9e-7 | yes |
| FX-WRAP-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-006 frame 0: Width 0: the blur is left out, so the fully covered pixels are unchanged and only the half-covered edge down column 4 takes the light, through its uncovered half: its straight colour is screened by half the band behind it. | largest difference 1.9e-7 | yes |
| FX-WRAP-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-007 frame 0: Blend add: the light is added, not screened, so every pixel is at least as bright as FX-WRAP-001's, and the line's colour may pass its covering. | largest difference 2.3e-7 | yes |
| FX-WRAP-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-008 frame 0: Nothing beneath: the box alone, and nothing changes. | largest difference 1.9e-7 | yes |
| FX-WRAP-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-009 frame 0: The bands moved eight pixels right, so only columns 8 to 15 have anything behind: the box's left edge, far from the light, takes less than its right edge, and columns 0 to 7 around the box stay empty. | largest difference 1.7e-7 | yes |
| FX-WRAP-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-010 frame 0: The box moved three pixels right, the bands not: its covering and its wrap move with it, the light it takes is from the bands now behind it, and columns 0 to 6 are the bands, untouched. | largest difference 1.6e-7 | yes |
| FX-WRAP-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-011 frame 0: The box at opacity 50 per cent: the wrap is worked first, then the box is drawn at half its covering over the bands. | largest difference 9.9e-8 | yes |
| FX-WRAP-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-012 frame 0: The box in blend mode multiply: the wrap is worked first, then the box, lit, multiplies the bands. | largest difference 1.6e-7 | yes |
| FX-WRAP-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-013 frame 0: Exposure -1 before Light Wrap in the stack: the box is darkened in its own space, then placed and wrapped. | largest difference 8.8e-8 | yes |
| FX-WRAP-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-014 frame 0: Light Wrap before Exposure -1 in the stack: the other effects still run first, so this is FX-WRAP-013 exactly. | largest difference 8.8e-8 | yes |
| FX-WRAP-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-015 frame 0: Light Wrap on an adjustment layer above the bands and the box: it has no drawing of its own, so nothing changes. | largest difference 1.9e-7 | yes |
| FX-WRAP-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-016 frame 0: Light Wrap switched off: nothing changes. | largest difference 1.9e-7 | yes |
| FX-WRAP-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-017 frame 0: Width keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-WRAP-006, frame 2 is FX-WRAP-002, and at frame 4 the light reaches further in, so the box's middle takes more than at frame 2. | largest difference 1.9e-7 | yes |
| FX-WRAP-017 frame 2: Width keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-WRAP-006, frame 2 is FX-WRAP-002, and at frame 4 the light reaches further in, so the box's middle takes more than at frame 2. | largest difference 2.1e-7 | yes |
| FX-WRAP-017 frame 4: Width keyed from 0 at frame 0 to 6 at frame 4, linear: frame 0 is FX-WRAP-006, frame 2 is FX-WRAP-002, and at frame 4 the light reaches further in, so the box's middle takes more than at frame 2. | largest difference 1.8e-7 | yes |
| FX-WRAP-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-018 frame 0: Intensity keyed from 0 at frame 0 to 400 at frame 4, eased past its end (530 at frame 2): frame 0 changes nothing; frame 2 is held at 400 and is frame 4, FX-WRAP-004. | largest difference 1.9e-7 | yes |
| FX-WRAP-018 frame 2: Intensity keyed from 0 at frame 0 to 400 at frame 4, eased past its end (530 at frame 2): frame 0 changes nothing; frame 2 is held at 400 and is frame 4, FX-WRAP-004. | largest difference 1.4e-7 | yes |
| FX-WRAP-018 frame 4: Intensity keyed from 0 at frame 0 to 400 at frame 4, eased past its end (530 at frame 2): frame 0 changes nothing; frame 2 is held at 400 and is frame 4, FX-WRAP-004. | largest difference 1.4e-7 | yes |
| FX-WRAP-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WRAP-019 frame 0: Width 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-019 frame 4: Width 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRAP-020 frame 0: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-020 frame 4: Width -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRAP-021 frame 0: Intensity 401, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-021 frame 4: Intensity 401, above 400. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRAP-022 frame 0: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-022 frame 4: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRAP-023 frame 0: Blend "multiply", which is not screen or add. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-023 frame 4: Blend "multiply", which is not screen or add. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WRAP-024 frame 0: Width keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-024 frame 4: Width keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WRAP-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the light falls inside the layer's edge | 0 | yes |
| a half-size draft preview reaches half as far in: width 24 becomes 12 | LightWrap { width: 12.0, intensity: 150.0, blend: "add" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wrap_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wrap_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `blend` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 501 is refused with a sentence, and nothing changes | Light Wrap's width runs from 0 to 500, and this is 501. | yes |
| width -1 is refused with a sentence, and nothing changes | Light Wrap's width runs from 0 to 500, and this is -1. | yes |
| intensity 401 is refused with a sentence, and nothing changes | Light Wrap's intensity runs from 0 to 400, and this is 401. | yes |
| blend "multiply" is refused with a sentence, and nothing changes | Light Wrap's blend is "screen" or "add", and this is "multiply". | yes |
| width keyed to 600 is refused with a sentence, and nothing changes | Light Wrap's width runs from 0 to 500, and this is 600. | yes |
| width 500, intensity 400 and blend add, the tops, is taken | taken | yes |
| width 0 and intensity 0, the bottoms, is taken | taken | yes |
| intensity keyed from 0 to 400 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wrap_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wrap_010.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wrap_017.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

86 of 86 checks pass.
