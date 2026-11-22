# D-1139: Chromatic Aberration at the drawing's edge

D-1139, accepted by the owner on 2026-11-21. A sample past the edge of the effect's input reads the nearest edge pixel, on the processor and on the card. Every expected pixel is `Fixtures/chromatic_edge/expected_chromatic_edge.json`, written by `tools/chromatic_edge_reference.py` before the fix and printed in document 25 as FX-CHROMAEDGE-001 to 003. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CHROMAEDGE-001 to 003 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHROMAEDGE-001 frame 0: A flat #f6d6be filling the frame, under an adjustment layer holding Chromatic Aberration amount 12 about the middle: every sample past the frame's edge reads the edge's own colour, so every pixel is unchanged and no band round the frame loses its blue. | largest difference 2.5e-7 | yes |
| FX-CHROMAEDGE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMAEDGE-002 frame 0: A 6 by 6 opaque square of different colours in the middle of the frame, the effect on it at amount 3: blue sampled past the square's edge reads the nearest edge pixel, so the square keeps blue to its edge, keeps its full covering, and nothing is drawn outside it. | largest difference 1.2e-7 | yes |
| FX-CHROMAEDGE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CHROMAEDGE-003 frame 0: FX-CHROMAEDGE-002 at amount 12, past the square's half diagonal 4.24: the red picture is turned over through the centre and both samples fall mostly past the edge, each reading the nearest edge pixel. | largest difference 1.5e-7 | yes |
| FX-CHROMAEDGE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_chromaedge_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_chromaedge_003.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CHROMAEDGE-001, fx_chromaedge_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CHROMAEDGE-002, fx_chromaedge_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-CHROMAEDGE-003, fx_chromaedge_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |

## Result

11 of 11 checks pass.
