# D-425: Threads

B-304, after CycoreFX's CC Threads: the layer woven into a cloth of threads in its own colours. Warp threads Width apart and weft threads Height apart, laid about Center and turned by Direction, each Coverage of its spacing wide; each passes over and under Overlaps of the others (taken whole); the thread beneath is darkened by Shadowing beside the one above, and Texture shades each thread round. Where neither thread lies the layer is clear. The manual gives no formula, ranges or defaults, so those are ours. Every expected pixel is `Fixtures/threads/expected_threads.json`, written by `tools/threads_reference.py` before this code existed and printed in document 25 as FX-THREADS-001 to 028. Tolerance 2e-5.

## FX-THREADS-001 to 028 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-THREADS-001 frame 0: As added: threads 50 pixels apart, coverage 90, shadowing 50, centred on the drawing, so its middle falls between threads: the gap between two warp threads and the gap between two weft threads cross in the middle, columns 6 to 9 and rows 3 to 6 clear, half clear beside them; elsewhere the photo, the thread above hiding the one beneath, which shows shadowed only at the upper thread's soft edge. | largest difference 1.5e-7 | yes |
| FX-THREADS-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-002 frame 0: Threads 4 pixels apart, unshadowed: a plain weave, each thread 3.6 pixels wide in the photo's colours, the edges of each pixel-wide gap between them soft. | largest difference 1.5e-7 | yes |
| FX-THREADS-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-003 frame 0: FX-THREADS-002 at coverage 100: the threads meet edge to edge, their seams on pixel edges, so the photo shows whole, untouched. | largest difference 1.5e-7 | yes |
| FX-THREADS-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-004 frame 0: FX-THREADS-002 at coverage 50: threads 2 pixels wide with 2-pixel gaps; where the gaps cross, clear. | largest difference 1.5e-7 | yes |
| FX-THREADS-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-005 frame 0: Coverage 0: no threads, everything clear. | largest difference 0.0e0 | yes |
| FX-THREADS-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-006 frame 0: FX-THREADS-002 with Overlaps 2: each thread over two and under two, a diagonal twill. | largest difference 1.5e-7 | yes |
| FX-THREADS-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-007 frame 0: FX-THREADS-006 with Overlaps 3. | largest difference 1.5e-7 | yes |
| FX-THREADS-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-008 frame 0: FX-THREADS-002 turned 30 degrees clockwise about the middle. | largest difference 1.7e-7 | yes |
| FX-THREADS-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-009 frame 0: Shadowing 100: where the threads cross, the one beneath darkened, fully beside the one above, half where it is further off; the one above as in FX-THREADS-002. | largest difference 1.5e-7 | yes |
| FX-THREADS-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-010 frame 0: Texture 100: each thread full in its middle and black at its edges, round. | largest difference 1.3e-7 | yes |
| FX-THREADS-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-011 frame 0: Width 6, height 3, coverage 80, shadowing 60, texture 40: wide warp threads crossing narrow weft threads. | largest difference 1.5e-7 | yes |
| FX-THREADS-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-012 frame 0: Centre 25, 50, the point (4, 5): the cloth slides a whole thread left, so each crossing swaps which thread is on top. | largest difference 1.5e-7 | yes |
| FX-THREADS-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-013 frame 0: On the shapes drawing, clear between its blocks: the threads take the blocks' colours, and are clear where the drawing is. | largest difference 1.7e-7 | yes |
| FX-THREADS-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-014 frame 0: FX-THREADS-009 on the holder moved 2 right and 1 down: the same, moved; the cloth is woven on the drawing before it moves. | largest difference 1.5e-7 | yes |
| FX-THREADS-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-015 frame 0: Direction keyed from 0 at frame 0 to 60 at frame 4, linear: frame 2 is FX-THREADS-008, frame 0 FX-THREADS-002. | largest difference 1.5e-7 | yes |
| FX-THREADS-015 frame 2: Direction keyed from 0 at frame 0 to 60 at frame 4, linear: frame 2 is FX-THREADS-008, frame 0 FX-THREADS-002. | largest difference 1.7e-7 | yes |
| FX-THREADS-015 frame 4: Direction keyed from 0 at frame 0 to 60 at frame 4, linear: frame 2 is FX-THREADS-008, frame 0 FX-THREADS-002. | largest difference 1.7e-7 | yes |
| FX-THREADS-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-016 frame 1: Overlaps keyed from 1 at frame 0 to 3 at frame 4, linear: read whole, frame 1 is plain still, frame 2 is FX-THREADS-006. | largest difference 1.5e-7 | yes |
| FX-THREADS-016 frame 2: Overlaps keyed from 1 at frame 0 to 3 at frame 4, linear: read whole, frame 1 is plain still, frame 2 is FX-THREADS-006. | largest difference 1.5e-7 | yes |
| FX-THREADS-016 frame 4: Overlaps keyed from 1 at frame 0 to 3 at frame 4, linear: read whole, frame 1 is plain still, frame 2 is FX-THREADS-006. | largest difference 1.5e-7 | yes |
| FX-THREADS-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-017 frame 0: Width and height 1, coverage 50: threads half a pixel wide, every pixel half warp and half weft. | largest difference 1.0e-7 | yes |
| FX-THREADS-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-018 frame 0: Coverage eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as frame 4 is. | largest difference 0.0e0 | yes |
| FX-THREADS-018 frame 2: Coverage eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-THREADS-018 frame 4: Coverage eased from 0 at frame 0 to 100 at frame 4 on a curve that overshoots: at frame 2 it would pass 100, is held at 100, as frame 4 is. | largest difference 1.5e-7 | yes |
| FX-THREADS-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-019 frame 0: Width 5, height 3, Overlaps 2, direction -20, coverage 85, shadowing 70, texture 60, centre 40, 60, on the ramp: the controls together. | largest difference 1.4e-7 | yes |
| FX-THREADS-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-THREADS-020 frame 0: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-020 frame 4: Width 0.5, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-021 frame 0: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-021 frame 4: Height 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-022 frame 0: Overlaps 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-022 frame 4: Overlaps 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-023 frame 0: Overlaps 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-023 frame 4: Overlaps 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-024 frame 0: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-024 frame 4: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-025 frame 0: A centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-025 frame 4: A centre 50, -1001, past ten heights. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-026 frame 0: Coverage 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-026 frame 4: Coverage 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-027 frame 0: Shadowing -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-027 frame 4: Shadowing -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-THREADS-028 frame 0: Texture 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-028 frame 4: Texture 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-THREADS-028: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing: the cloth is woven on the layer as it is | 0 | yes |
| a half-size draft preview halves the thread spacing; the centre is a share of the drawing | Threads { width: 4.0, height: 3.0, overlaps: 1.0, direction: 0.0, center: [50.0, 50.0], coverage: 90.0, shadowing: 50.0, texture: 0.0 } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threads_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_threads_019.json is saved with all 8 settings | {"center":[40,60],"coverage":85,"direction":-20,"height":3,"overlaps":2,"shadowing":70,"texture":60,"width":5} | yes |
| fx_threads_015.json is saved with the direction's two keys | {"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":60}]} | yes |
| fx_threads_020.json is refused in a sentence | Threads's width runs from 1 to 1000, and this is 0.5. | yes |
| fx_threads_021.json is refused in a sentence | Threads's height runs from 1 to 1000, and this is 1001. | yes |
| fx_threads_022.json is refused in a sentence | Threads's overlaps runs from 1 to 10, and this is 0. | yes |
| fx_threads_023.json is refused in a sentence | Threads's overlaps runs from 1 to 10, and this is 11. | yes |
| fx_threads_024.json is refused in a sentence | Threads's direction runs from -3600 to 3600, and this is 3601. | yes |
| fx_threads_025.json is refused in a sentence | Threads's center runs from -1000 to 1000, and this is -1001. | yes |
| fx_threads_026.json is refused in a sentence | Threads's coverage runs from 0 to 100, and this is 101. | yes |
| fx_threads_027.json is refused in a sentence | Threads's shadowing runs from 0 to 100, and this is -1. | yes |
| fx_threads_028.json is refused in a sentence | Threads's texture runs from 0 to 100, and this is 101. | yes |
| a file with a Threads with no `width` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Threads whose width is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Threads whose center is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Threads with no `texture` is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| width 0.5 is refused with a sentence, and nothing changes | Threads's width runs from 1 to 1000, and this is 0.5. | yes |
| height 1000.5 is refused with a sentence, and nothing changes | Threads's height runs from 1 to 1000, and this is 1000.5. | yes |
| overlaps 10.5 is refused with a sentence, and nothing changes | Threads's overlaps runs from 1 to 10, and this is 10.5. | yes |
| coverage -0.5 is refused with a sentence, and nothing changes | Threads's coverage runs from 0 to 100, and this is -0.5. | yes |
| center 1001, 50 is refused with a sentence, and nothing changes | Threads's center runs from -1000 to 1000, and this is 1001. | yes |
| direction keyed to 3601 is refused with a sentence, and nothing changes | Threads's direction runs from -3600 to 3600, and this is 3601. | yes |
| texture keyed to 150 is refused with a sentence, and nothing changes | Threads's texture runs from 0 to 100, and this is 150. | yes |
| the tops: width and height 1000, overlaps 10, direction 3600, coverage, shadowing and texture 100 is taken | taken | yes |
| the bottoms: width and height 1, overlaps 1, direction -3600, coverage, shadowing and texture 0 is taken | taken | yes |
| overlaps 2.5, read whole is taken | taken | yes |
| direction keyed from 0 to 90 is taken | taken | yes |
| center keyed from 0, 0 to 100, 100 is taken | taken | yes |
| undo 5 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threads_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_008.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_014.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_016.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_threads_019.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_threads_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_threads_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_threads_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; the same warnings: true | yes |
| fx_threads_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; the same warnings: true | yes |
| fx_threads_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| fx_threads_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; the same warnings: true | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 247320 pixels changed | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 0, Full | largest difference 1 of 255, 4482 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 100, Full | largest difference 1 of 255, 2417 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 239, Full | largest difference 1 of 255, 2692 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 0, Draft | largest difference 1 of 255, 109 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 100, Draft | largest difference 1 of 255, 31 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads as it starts (50 apart, coverage 90, shadowing 50) on three layers, frame 239, Draft | largest difference 1 of 255, 49 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 1891679 pixels changed | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 0, Full | largest difference 1 of 255, 1196 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 100, Full | largest difference 1 of 255, 1193 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 239, Full | largest difference 1 of 255, 1088 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 0, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 100, Draft | largest difference 1 of 255, 77 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads 12 by 8, overlaps 3, turned 30 degrees, shadowing 100, texture 60 on three layers, frame 239, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 931490 pixels changed | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 0, Full | largest difference 1 of 255, 2845 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 100, Full | largest difference 1 of 255, 1166 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 239, Full | largest difference 1 of 255, 1529 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 0, Draft | largest difference 1 of 255, 97 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 100, Draft | largest difference 1 of 255, 94 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Threads the direction keyed 0 to 90, width 20, coverage 60, centre 30, 70 on three layers, frame 239, Draft | largest difference 1 of 255, 47 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures: the street at frame 0, in `verification/D-425 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_before.png, the street with no effect; draws cleanly | [] | yes |
| 2_as_added.png, as it starts: threads 50 pixels apart crossing over the street, thin clear gaps between them; draws cleanly | [], 14490 pixels changed | yes |
| 3_fine_weave.png, 8 pixels apart, shadowing 80: a fine plain weave, the threads beneath darker; draws cleanly | [], 32040 pixels changed | yes |
| 4_twill_turned.png, 10 apart, overlaps 3, turned 45 degrees, texture 50: a diagonal twill of rounded threads; draws cleanly | [], 115548 pixels changed | yes |
| 5_open_cloth.png, 16 apart, coverage 50, texture 100: an open basket of round threads with big clear holes; draws cleanly | [], 129600 pixels changed | yes |
| 6_ribbons.png, width 40, height 12, coverage 80: broad ribbons across narrow ones; draws cleanly | [], 35040 pixels changed | yes |

## Result

191 of 191 checks pass.
