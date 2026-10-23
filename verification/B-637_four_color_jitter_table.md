# B-637: 4-Color Gradient's jitter (D-758)

D-758 adds After Effects' Jitter as `jitter` (0 to 100, keyable, absent 0): each pixel's four weights are nudged by up to jitter / 1000 of themselves with a seeded number fixed to the drawing (document 21). A file without it draws as before. Every expected pixel is `Fixtures/four_color_gradient/expected_four_color_gradient.json`, written by `tools/four_color_gradient_reference.py` before this code existed and printed in document 25 as FX-4CG-027 to 037. Tolerance 2e-5.

## FX-4CG-027 to 037 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-4CG-027 frame 0: Jitter 50, the rest as they start: FX-4CG-001 with a fine grain through the colours, each pixel's four weights nudged by up to a twentieth of themselves. | largest difference 3.9e-8 | yes |
| FX-4CG-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-028 frame 0: Jitter 0 written: FX-4CG-001 exactly. | largest difference 4.1e-8 | yes |
| FX-4CG-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-029 frame 0: Jitter 100: a stronger grain, each weight nudged by up to a tenth of itself. | largest difference 4.1e-8 | yes |
| FX-4CG-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-030 frame 0: Jitter keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-4CG-001, frame 2 jitter 50, FX-4CG-027, and frame 4 FX-4CG-029. | largest difference 4.1e-8 | yes |
| FX-4CG-030 frame 2: Jitter keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-4CG-001, frame 2 jitter 50, FX-4CG-027, and frame 4 FX-4CG-029. | largest difference 3.9e-8 | yes |
| FX-4CG-030 frame 4: Jitter keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-4CG-001, frame 2 jitter 50, FX-4CG-027, and frame 4 FX-4CG-029. | largest difference 4.1e-8 | yes |
| FX-4CG-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-031 frame 0: FX-4CG-027 moved three pixels right: the grain is fixed to the drawing, so it moves with it. | largest difference 3.9e-8 | yes |
| FX-4CG-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-032 frame 0: FX-4CG-027 after a Motion Tile that grows the layer: the grain is the drawing's own, so the frame is FX-4CG-027's. | largest difference 3.9e-8 | yes |
| FX-4CG-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-033 frame 0: Jitter 100 with point 1 on the centre of the pixel in column 4 and row 2, FX-4CG-004's: that pixel is still yellow exactly. | largest difference 4.2e-8 | yes |
| FX-4CG-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-034 frame 0: Jitter 100 with all four colours #6450a0: nudged weights mix one colour, so this is FX-4CG-011. | largest difference 3.0e-8 | yes |
| FX-4CG-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-4CG-035 frame 0: Jitter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-035 frame 4: Jitter 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-036 frame 0: Jitter -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-036 frame 4: Jitter -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-4CG-037 frame 0: Jitter keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-037 frame 4: Jitter keyed to 150 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-4CG-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: no jitter, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_4cg_001.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_002.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_003.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_004.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_005.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_006.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_007.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_008.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_009.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_010.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_011.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_012.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_013.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_014.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_015.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_016.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_017.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_018.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_019.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_020.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_021.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_022.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_023.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_024.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_025.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |
| fx_4cg_026.json (no jitter) is saved as it was written (letters' case aside: 010's capital colours are read lowercase, as before), and draws frames 0 and 4 bit for bit as its twin with Jitter 0 | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_4cg_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_4cg_027.json is saved with Jitter 50 | {"blend":100,"blending_mode":"normal","color_1":"#ffff00","color_2":"#00ff00","color_3":"#ff00ff","color_4":"#0000ff","jitter":50,"opacity":100,"point_1":[10,10],"point_2":[90,10],"point_3":[10,90],"point_4":[90,90]} | yes |
| fx_4cg_001.json (no jitter) is saved with no jitter written | {"blend":100,"blending_mode":"normal","color_1":"#ffff00","color_2":"#00ff00","color_3":"#ff00ff","color_4":"#0000ff","opacity":100,"point_1":[10,10],"point_2":[90,10],"point_3":[10,90],"point_4":[90,90]} | yes |
| fx_4cg_035.json (Jitter 101) is refused in a sentence naming jitter | 4-Color Gradient's jitter runs from 0 to 100, and this is 101. | yes |
| fx_4cg_036.json (Jitter -1) is refused in a sentence naming jitter | 4-Color Gradient's jitter runs from 0 to 100, and this is -1. | yes |
| a file with a 4-Color Gradient whose Jitter is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview keeps Jitter 50, not a distance | FourColorGradient { point_1: [10.0, 10.0], point_2: [90.0, 10.0], point_3: [10.0, 90.0], point_4: [90.0, 90.0], color_1: "#ffff00", color_2: "#00ff00", color_3: "#ff00ff", color_4: "#0000ff", blend: 100.0, opacity: 100.0, blending_mode: "normal", jitter: 50.0 } | yes |
| Jitter 100 never grows the layer | 0 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| jitter 100.5 is refused with a sentence, and nothing changes | 4-Color Gradient's jitter runs from 0 to 100, and this is 100.5. | yes |
| jitter keyed to 150 is refused with a sentence, and nothing changes | 4-Color Gradient's jitter runs from 0 to 100, and this is 150. | yes |
| jitter 40 is taken | taken | yes |
| jitter keyed from 0 to 100 is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_4cg_027.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_030.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_031.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_032.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_4cg_033.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| the card draws a 4-Color Gradient without jitter as before; one with jitter above 0 stays on the processor, so those frames are the processor's | see the rows below | yes |
| FX-4CG-001, fx_4cg_001.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-002, fx_4cg_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-003, fx_4cg_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-004, fx_4cg_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-005, fx_4cg_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-006, fx_4cg_006.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-007, fx_4cg_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-008, fx_4cg_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-009, fx_4cg_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-010, fx_4cg_010.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-011, fx_4cg_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-012, fx_4cg_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-013, fx_4cg_013.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-014, fx_4cg_014.json, frames [0, 2] at Full and Draft | largest difference 1 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-015, fx_4cg_015.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-016, fx_4cg_016.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-017, fx_4cg_017.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-018, fx_4cg_018.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-019, fx_4cg_019.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-020, fx_4cg_020.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-021, fx_4cg_021.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-022, fx_4cg_022.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-023, fx_4cg_023.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-024, fx_4cg_024.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-025, fx_4cg_025.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-026, fx_4cg_026.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-027, fx_4cg_027.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-028, fx_4cg_028.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-029, fx_4cg_029.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-030, fx_4cg_030.json, frames [0, 2, 4] at Full and Draft | largest difference 1 of 255; on the card in 2 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-031, fx_4cg_031.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-032, fx_4cg_032.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-033, fx_4cg_033.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-034, fx_4cg_034.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-035, fx_4cg_035.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-036, fx_4cg_036.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-4CG-037, fx_4cg_037.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card): the processor's frame 100 differs from the shot without it | 2073600 pixels changed | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 0, Full | largest difference 1 of 255, 1146 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 100, Full | largest difference 1 of 255, 1152 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 239, Full | largest difference 1 of 255, 1153 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 100, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 0 written (on the card), frame 239, Draft | largest difference 1 of 255, 80 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's): the processor's frame 100 differs from the shot without it | 2073600 pixels changed | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 0, Full | largest difference 1 of 255, 72662 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 100, Full | largest difference 1 of 255, 72832 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 239, Full | largest difference 1 of 255, 72776 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 0, Draft | largest difference 1 of 255, 4593 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 100, Draft | largest difference 1 of 255, 4572 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, 4-Color Gradient, opacity 60, Jitter 70 (the processor's), frame 239, Draft | largest difference 1 of 255, 4549 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/B-637 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_jitter_0.png and its four-times corner 1_jitter_0_zoom.png: the street under the gradient at Jitter 0; draws cleanly | []; 0 pixels differ from Jitter 0, by up to 0 levels | yes |
| 2_jitter_50.png and its four-times corner 2_jitter_50_zoom.png: the street under the gradient at Jitter 50; draws cleanly, and differs from Jitter 0 finely (few levels, many pixels) | []; 100611 pixels differ from Jitter 0, by up to 6 levels | yes |
| 3_jitter_100.png and its four-times corner 3_jitter_100_zoom.png: the street under the gradient at Jitter 100; draws cleanly, and differs from Jitter 0 finely (few levels, many pixels) | []; 112931 pixels differ from Jitter 0, by up to 12 levels | yes |

## Result

133 of 133 checks pass.
