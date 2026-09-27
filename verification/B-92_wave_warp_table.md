# B-92: wave warp

D-149, accepted on 2026-09-26 by the owner's message asking for thirty more effects, the sixteenth of the third batch. Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by `tools/wave_warp_reference.py` before this code existed and printed in document 25 as FX-WAVE-001 to 024. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-WAVE-001 to 024 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WAVE-001 frame 0: The settings as they start: shape sine, height 10, width 40, direction 90, speed 0, phase 0, edges transparent. The wave runs to the right and pushes each column down by 10 sin(2 pi X / 40), more than the drawing's height across its middle: column 0 is pushed down by 0.8 pixels, column 3 by 5.2, columns 7 to 12 by more than 9, so their rows 0 to 8 are empty and row 9 keeps only part of the drawing's top row, and column 15 by 6.5; the frames are the same, the wave standing still at speed 0. | largest difference 1.9e-7 | yes |
| FX-WAVE-001 frame 4: The settings as they start: shape sine, height 10, width 40, direction 90, speed 0, phase 0, edges transparent. The wave runs to the right and pushes each column down by 10 sin(2 pi X / 40), more than the drawing's height across its middle: column 0 is pushed down by 0.8 pixels, column 3 by 5.2, columns 7 to 12 by more than 9, so their rows 0 to 8 are empty and row 9 keeps only part of the drawing's top row, and column 15 by 6.5; the frames are the same, the wave standing still at speed 0. | largest difference 1.9e-7 | yes |
| FX-WAVE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-002 frame 0: Height 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-WAVE-002 frame 2: Height 0: the drawing, untouched, and nothing grows. | largest difference 1.9e-7 | yes |
| FX-WAVE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-003 frame 0: Height 2, width 8: a wave every eight pixels, each column pushed down or up by 2 sin(2 pi X / 8), so the blue band waves: columns 1 and 2 carry it down into rows 6 and 7 and columns 5 and 6 up into rows 2 and 3. Where a column is pushed down its top pixel reads past the drawing's top edge and loses covering, and row 9 takes the drawing's bottom row. The frames are the same. | largest difference 1.9e-7 | yes |
| FX-WAVE-003 frame 2: Height 2, width 8: a wave every eight pixels, each column pushed down or up by 2 sin(2 pi X / 8), so the blue band waves: columns 1 and 2 carry it down into rows 6 and 7 and columns 5 and 6 up into rows 2 and 3. Where a column is pushed down its top pixel reads past the drawing's top edge and loses covering, and row 9 takes the drawing's bottom row. The frames are the same. | largest difference 1.9e-7 | yes |
| FX-WAVE-003 frame 4: Height 2, width 8: a wave every eight pixels, each column pushed down or up by 2 sin(2 pi X / 8), so the blue band waves: columns 1 and 2 carry it down into rows 6 and 7 and columns 5 and 6 up into rows 2 and 3. Where a column is pushed down its top pixel reads past the drawing's top edge and loses covering, and row 9 takes the drawing's bottom row. The frames are the same. | largest difference 1.9e-7 | yes |
| FX-WAVE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-004 frame 0: FX-WAVE-003 with shape triangle: the zigzag pushes each pixel no further than the sine does, so the band's waves are flatter. | largest difference 1.9e-7 | yes |
| FX-WAVE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-005 frame 0: Direction 0: the wave runs up the drawing and pushes each row left or right, so the upright stripes wave while the band, pushed along itself, stays blue from column 3 to column 11. | largest difference 1.9e-7 | yes |
| FX-WAVE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-006 frame 0: Direction 45: the wave runs up and to the right and pushes along the other diagonal, so both the stripes and the band bend. | largest difference 2.5e-7 | yes |
| FX-WAVE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-007 frame 0: Direction 450, a whole turn past 90: FX-WAVE-003 exactly. | largest difference 1.9e-7 | yes |
| FX-WAVE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-008 frame 0: Direction 270 at phase 0: the wave runs left and pushes the other way, and a sine run backwards is the same sine turned over, so it is FX-WAVE-003. | largest difference 1.9e-7 | yes |
| FX-WAVE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-009 frame 0: Phase 90: the wave slid a quarter of the way along, so each column is pushed by 2 cos(2 pi X / 8) and the band waves two pixels further left than FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-010 frame 0: Speed 45: the wave travels. Frame 0 is FX-WAVE-003, frame 2 is phase 90, FX-WAVE-009, and frame 4 is phase 180, where every push is FX-WAVE-003's the other way. | largest difference 1.9e-7 | yes |
| FX-WAVE-010 frame 2: Speed 45: the wave travels. Frame 0 is FX-WAVE-003, frame 2 is phase 90, FX-WAVE-009, and frame 4 is phase 180, where every push is FX-WAVE-003's the other way. | largest difference 1.9e-7 | yes |
| FX-WAVE-010 frame 4: Speed 45: the wave travels. Frame 0 is FX-WAVE-003, frame 2 is phase 90, FX-WAVE-009, and frame 4 is phase 180, where every push is FX-WAVE-003's the other way. | largest difference 1.9e-7 | yes |
| FX-WAVE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-011 frame 0: FX-WAVE-003 with edges repeat: the layer does not grow, and a push past the drawing's top edge reads the drawing's top row, so where FX-WAVE-003's top pixels lose covering these keep it; every other pixel is FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-012 frame 0: Width 8, height keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is height 2, FX-WAVE-003, and frame 4 height 4. | largest difference 1.9e-7 | yes |
| FX-WAVE-012 frame 2: Width 8, height keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is height 2, FX-WAVE-003, and frame 4 height 4. | largest difference 1.9e-7 | yes |
| FX-WAVE-012 frame 4: Width 8, height keyed from 0 at frame 0 to 4 at frame 4, linear: frame 0 is the drawing, frame 2 is height 2, FX-WAVE-003, and frame 4 height 4. | largest difference 1.9e-7 | yes |
| FX-WAVE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-013 frame 0: Height 2, width 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-WAVE-003, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 1.9e-7 | yes |
| FX-WAVE-013 frame 2: Height 2, width 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-WAVE-003, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 1.9e-7 | yes |
| FX-WAVE-013 frame 4: Height 2, width 8, speed keyed from 0 at frame 0 to 360 at frame 4, eased past its end (477 at frame 2): frame 0 is FX-WAVE-003, and frame 2 is held at 360, so it is speed 360's frame 2. | largest difference 1.9e-7 | yes |
| FX-WAVE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-014 frame 0: FX-WAVE-005 moved three pixels right: the wave is worked in the drawing's own space, so it moves with it; the layer grew by 2, and the two columns left of the drawing show the grown pixels, into which the stripes are pushed, while the column left of those stays empty. | largest difference 1.9e-7 | yes |
| FX-WAVE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-015 frame 0: FX-WAVE-014 with edges repeat: nothing grows, so the three columns left of the drawing stay empty. | largest difference 1.9e-7 | yes |
| FX-WAVE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-016 frame 0: Height 2.5, width 8, direction 0, moved three pixels right: the layer grows by 3, the height rounded up, and the stripes pushed furthest left reach into the third column left of the drawing. | largest difference 2.5e-7 | yes |
| FX-WAVE-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-017 frame 0: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-017 frame 4: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-018 frame 0: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-018 frame 4: Width 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-019 frame 0: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-019 frame 4: Direction 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-020 frame 0: Speed -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-020 frame 4: Speed -361, below -360. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-021 frame 0: Phase 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-021 frame 4: Phase 100001, above 100000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-022 frame 0: Height keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-022 frame 4: Height keyed to 1500 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-023 frame 0: Shape "square", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-023 frame 4: Shape "square", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-024 frame 0: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-024 frame 4: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| with transparent edges, height 2.5 grows the drawing's bounds by 3 pixels | 3 | yes |
| with repeat edges it grows them by nothing | 0 | yes |
| at height 0 it grows them by nothing | 0 | yes |
| a half-size draft preview halves the height and the width | WaveWarp { shape: "sine", height: 5.0, width: 20.0, direction: 90.0, speed: 0.0, phase: 0.0, edges: "transparent", frame: 0 } | yes |
| a half-size draft of width 1 holds the width at 1, its range's bottom, rather than leaving the effect out | WaveWarp { shape: "sine", height: 5.0, width: 1.0, direction: 90.0, speed: 0.0, phase: 0.0, edges: "transparent", frame: 0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `edges` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a height that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| height 1001 is refused with a sentence, and nothing changes | Wave Warp's height runs from 0 to 1000, and this is 1001. | yes |
| width 0 is refused with a sentence, and nothing changes | Wave Warp's width runs from 1 to 10000, and this is 0. | yes |
| direction 3601 is refused with a sentence, and nothing changes | Wave Warp's direction runs from -3600 to 3600, and this is 3601. | yes |
| speed -361 is refused with a sentence, and nothing changes | Wave Warp's speed runs from -360 to 360, and this is -361. | yes |
| phase 100001 is refused with a sentence, and nothing changes | Wave Warp's phase runs from -100000 to 100000, and this is 100001. | yes |
| shape "square" is refused with a sentence, and nothing changes | Wave Warp's shape is "sine" or "triangle", and this is "square". | yes |
| edges "Repeat" is refused with a sentence, and nothing changes | Wave Warp's edges are "transparent" or "repeat", and this is "Repeat". | yes |
| height keyed to 1500 is refused with a sentence, and nothing changes | Wave Warp's height runs from 0 to 1000, and this is 1500. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| phase keyed from 0 to 360 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_006.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

103 of 103 checks pass.
