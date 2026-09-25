# B-38: line width

D-94, accepted by the owner on 2026-09-25. Every expected pixel is `Fixtures/line_width/expected_line_width.json`, written by `tools/line_width_reference.py` before this code existed and printed in document 25 as FX-WIDTH-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-WIDTH-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WIDTH-001 frame 0: Shape, width 1: the drawing grows one pixel up, down, left and right. The half-covering edge in column 1 becomes solid line, and a new half-covering edge appears in column 0; the corners stay square-cut. | largest difference 1.9e-7 | yes |
| FX-WIDTH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-002 frame 0: Shape, width 1.5: the diagonal neighbours count too, so the corners fill out as well. | largest difference 1.9e-7 | yes |
| FX-WIDTH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-003 frame 0: Shape, width -1: the drawing shrinks one pixel. The box's outer line touches the transparent outside and goes; the line in column 2 touches the half-covering edge and becomes half covering, still the line's colour. | largest difference 1.9e-7 | yes |
| FX-WIDTH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-004 frame 0: Width 0: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-WIDTH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-005 frame 0: Colours, the line chosen, width 1: the line spreads one pixel into the skin inside and the transparent outside, filling the skin between the red trace line's ends and the box; the trace line itself is untouched. | largest difference 1.9e-7 | yes |
| FX-WIDTH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-006 frame 0: Colours, the trace line chosen, width -1: a one-pixel line thinned by one is gone; each of its pixels takes the skin above it, which wins the tie with the skin below. | largest difference 1.9e-7 | yes |
| FX-WIDTH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-007 frame 0: Colours, the trace line chosen, width 2: the red line grows two pixels up and down and sideways, painting over the skin and the box's line alike. | largest difference 1.9e-7 | yes |
| FX-WIDTH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-008 frame 0: Shape, width keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 1 is FX-WIDTH-001, and frames 2 and 4 grow two and four pixels. | largest difference 1.9e-7 | yes |
| FX-WIDTH-008 frame 1: Shape, width keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 1 is FX-WIDTH-001, and frames 2 and 4 grow two and four pixels. | largest difference 1.9e-7 | yes |
| FX-WIDTH-008 frame 2: Shape, width keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 1 is FX-WIDTH-001, and frames 2 and 4 grow two and four pixels. | largest difference 1.9e-7 | yes |
| FX-WIDTH-008 frame 4: Shape, width keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 1 is FX-WIDTH-001, and frames 2 and 4 grow two and four pixels. | largest difference 1.9e-7 | yes |
| FX-WIDTH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-009 frame 0: FX-WIDTH-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-WIDTH-009 frame 3: FX-WIDTH-001 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-WIDTH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-010 frame 0: Colours with no colour chosen, width 3: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-WIDTH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-011 frame 0: Shape with the line listed as a colour: the colours are kept but not used, and the frame is FX-WIDTH-001. | largest difference 1.9e-7 | yes |
| FX-WIDTH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-012 frame 0: Shape, width -20: the drawing, six pixels tall, is gone. | largest difference 0.0e0 | yes |
| FX-WIDTH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WIDTH-013 frame 0: Width 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-013 frame 4: Width 21, above 20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-013: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-014 frame 0: Width -21, below -20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-014 frame 4: Width -21, below -20. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-015 frame 0: Width keyed to 25 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-015 frame 4: Width keyed to 25 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-016 frame 0: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-016 frame 4: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-017 frame 0: Based on "line", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-017 frame 4: Based on "line", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-018 frame 0: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-018 frame 4: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WIDTH-019 frame 0: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-019 frame 4: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WIDTH-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 0 grows the drawing's bounds by 0 | 0 | yes |
| width 1 grows the drawing's bounds by 1 | 1 | yes |
| width 1.5 grows the drawing's bounds by 2 | 2 | yes |
| width 20 grows the drawing's bounds by 20 | 20 | yes |
| width -5 grows the drawing's bounds by 0 | 0 | yes |
| a half-size draft preview halves the width, thinning included, and keeps the rest | LineWidth { width: -3.0, based_on: "colors", colors: ["#1e1a24"], tolerance: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_width_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_width_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `based_on` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a width that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 21 is refused with a sentence, and nothing changes | Line Width's width runs from -20 to 20, and this is 21. | yes |
| width -21 is refused with a sentence, and nothing changes | Line Width's width runs from -20 to 20, and this is -21. | yes |
| based on "line" is refused with a sentence, and nothing changes | Line Width is based on "shape" or "colors", and this is "line". | yes |
| based on "Shape" is refused with a sentence, and nothing changes | Line Width is based on "shape" or "colors", and this is "Shape". | yes |
| nine colours is refused with a sentence, and nothing changes | Line Width takes up to eight colours, and this has 9. | yes |
| the colour "#12345" is refused with a sentence, and nothing changes | A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this is "#12345". | yes |
| width keyed to 25 is refused with a sentence, and nothing changes | Line Width's width runs from -20 to 20, and this is 25. | yes |
| tolerance keyed to 300 is refused with a sentence, and nothing changes | Line Width's tolerance runs from 0 to 255, and this is 300. | yes |
| width 20 with eight colours, the top of the ranges, is taken | taken | yes |
| width -20 with no colour, the bottom, is taken | taken | yes |
| width keyed from -4 to 4 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_width_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_width_009.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

81 of 81 checks pass.
