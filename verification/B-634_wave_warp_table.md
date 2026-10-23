# B-634: Wave Warp gains After Effects' remaining wave types

D-755, accepted on 2026-10-23 by the owner's message "Proceed, if an effect is more like a inspiration/modified version of a AE effect, only improve it." Wave Warp is inspired by After Effects' (D-149), so nothing it drew before changes. Shape gains square, sawtooth, semicircle and uncircle, placed against the sine as After Effects' stills show them, and noise and smooth noise, ours in spirit: fixed, the same at every frame. Every expected pixel is `Fixtures/wave_warp/expected_wave_warp.json`, written by `tools/wave_warp_reference.py` before this code and printed in document 25 as FX-WAVE-001 to 047. The build's frame is compared sample by sample against the catalogue's tolerance of 2e-5.

## FX-WAVE-001 to 047 (document 25)

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
| FX-WAVE-023 frame 0: Shape "hexagon", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-023 frame 4: Shape "hexagon", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-024 frame 0: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-024 frame 4: Edges "Repeat": the word is exact, so a capital is not the choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-025 frame 0: FX-WAVE-003 with shape circle: each half wave is half a circle, so the push rises steeply from nothing where the sine crosses the middle and stays near its full 2 pixels across most of each half wave. | largest difference 2.1e-7 | yes |
| FX-WAVE-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-026 frame 0: FX-WAVE-025 at direction 0: the half circles run up the drawing and push each row left or right. | largest difference 1.9e-7 | yes |
| FX-WAVE-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-027 frame 0: FX-WAVE-003 with pinning all_edges: the push fades to nothing at every edge of the drawing, reaching its whole size 4 pixels in from the left and right and 2.5 from the top and bottom, so the drawing's outline stays put and only its middle waves. | largest difference 2.5e-7 | yes |
| FX-WAVE-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-028 frame 0: Pinning left_edge: column 0 is pushed an eighth as far as in FX-WAVE-003, column 1 three eighths, column 2 five, column 3 seven, and columns 4 on are FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-029 frame 0: Pinning top_edge: row 0 is pushed a fifth as far as in FX-WAVE-003, row 1 three fifths, and rows 2 on are FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-030 frame 0: Pinning right_edge: the drawing's right edge, past column 15, holds still; columns 12 to 15 are pushed less, and columns 0 to 11 are FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-031 frame 0: Pinning bottom_edge: rows 8 and 9 are pushed less, and rows 0 to 7 are FX-WAVE-003's. | largest difference 1.9e-7 | yes |
| FX-WAVE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-032 frame 0: Pinning horizontal_edges, the top and the bottom: FX-WAVE-029 and FX-WAVE-031 together. | largest difference 1.9e-7 | yes |
| FX-WAVE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-033 frame 0: Pinning vertical_edges, the left and the right: FX-WAVE-028 and FX-WAVE-030 together. | largest difference 1.9e-7 | yes |
| FX-WAVE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-034 frame 0: Pinning none written in the file: FX-WAVE-003 exactly. | largest difference 1.9e-7 | yes |
| FX-WAVE-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-035 frame 0: FX-WAVE-014 with pinning all_edges: the drawing moved three pixels right, its edges still held where the drawing is, so the grown columns left of it stay empty. | largest difference 2.5e-7 | yes |
| FX-WAVE-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-036 frame 0: FX-WAVE-025 with pinning all_edges and speed 45: the half circles travel while the edges stay put; frame 2 is a quarter wave on. | largest difference 2.5e-7 | yes |
| FX-WAVE-036 frame 2: FX-WAVE-025 with pinning all_edges and speed 45: the half circles travel while the edges stay put; frame 2 is a quarter wave on. | largest difference 2.5e-7 | yes |
| FX-WAVE-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-037 frame 0: Pinning "center", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-037 frame 4: Pinning "center", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-038 frame 0: Pinning "All Edges", the name on the panel, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-038 frame 4: Pinning "All Edges", the name on the panel, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WAVE-039 frame 0: FX-WAVE-003 with shape square: columns 0 to 3 are pushed down the whole 2 pixels and columns 4 to 7 up, and again from column 8, so the band steps down and up in blocks four pixels wide. | largest difference 1.9e-7 | yes |
| FX-WAVE-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-040 frame 0: FX-WAVE-003 with shape sawtooth: the push climbs evenly across each wave of eight columns, from 1.75 pixels one way at column 0 to 1.75 the other at column 7, then drops back at column 8. | largest difference 1.9e-7 | yes |
| FX-WAVE-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-041 frame 0: FX-WAVE-003 with shape semicircle: one round arch a wave, pushing up most at columns 3 and 4 and 11 and 12, and the arches meeting in points at columns 0, 7, 8 and 15, all but unmoved. | largest difference 2.1e-7 | yes |
| FX-WAVE-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-042 frame 0: FX-WAVE-041 with shape uncircle: every push the other way. | largest difference 2.1e-7 | yes |
| FX-WAVE-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-043 frame 0: Shape noise, height 2, width 40: a fixed uneven push, a new level every two and a half pixels with straight lines between. | largest difference 2.5e-7 | yes |
| FX-WAVE-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-044 frame 0: Shape smooth_noise, height 2, width 8: a fixed uneven push, a new level every four pixels joined smoothly. | largest difference 2.5e-7 | yes |
| FX-WAVE-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-045 frame 0: FX-WAVE-043 with speed 45: the noise travels, the same at every frame for the same settings; frame 2 is phase 90, the pattern a quarter of 40 pixels on. | largest difference 2.5e-7 | yes |
| FX-WAVE-045 frame 2: FX-WAVE-043 with speed 45: the noise travels, the same at every frame for the same settings; frame 2 is phase 90, the pattern a quarter of 40 pixels on. | largest difference 2.5e-7 | yes |
| FX-WAVE-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-046 frame 0: FX-WAVE-041 with pinning all_edges and direction 0: the arches run up the drawing, pushing each row left or right, held still at its edges. | largest difference 2.5e-7 | yes |
| FX-WAVE-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WAVE-047 frame 0: Shape "Smooth Noise", the name on the panel, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-047 frame 4: Shape "Smooth Noise", the name on the panel, not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-WAVE-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A project saved before the new shapes

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_003.json, written before D-755, opens with shape sine as it always did | EffectInstance { instance_id: Id("fx-0-0"), enabled: true, effect: WaveWarp { shape: "sine", height: 2.0, width: 8.0, direction: 90.0, speed: 0.0, phase: 0.0, edges: "transparent", pinning: "none", frame: 0 }, tracks: {}, mix: 100.0, record: FileRecord(Some(Object {"enabled": Bool(true), "instance_id": String("fx-0-0"), "parameters": Object {"direction": Number(90), "edges": String("transparent"), "height": Number(2), "phase": Number(0), "shape": String("sine"), "speed": Number(0), "width": Number(8)}, "type_id": String("core.wave_warp")})) } | yes |
| a circle saved before D-755 is saved again as circle | {"direction":90,"edges":"transparent","height":2,"phase":0,"shape":"circle","speed":0,"width":8} | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_wave_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shape "Smooth Noise" is refused with a sentence, and nothing changes | Wave Warp's shape is one of sine, triangle, circle, square, sawtooth, semicircle, uncircle, noise, smooth_noise, and this is "Smooth Noise". | yes |
| shape "Square" is refused with a sentence, and nothing changes | Wave Warp's shape is one of sine, triangle, circle, square, sawtooth, semicircle, uncircle, noise, smooth_noise, and this is "Square". | yes |
| shape "hexagon" is refused with a sentence, and nothing changes | Wave Warp's shape is one of sine, triangle, circle, square, sawtooth, semicircle, uncircle, noise, smooth_noise, and this is "hexagon". | yes |
| shape square, is taken | taken | yes |
| shape sawtooth, is taken | taken | yes |
| shape semicircle, is taken | taken | yes |
| shape uncircle, is taken | taken | yes |
| shape noise, is taken | taken | yes |
| shape smooth_noise, is taken | taken | yes |
| shape sine, is taken | taken | yes |
| undo 7 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_wave_039.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_041.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_044.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_045.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_wave_046.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Result

148 of 148 checks pass.
