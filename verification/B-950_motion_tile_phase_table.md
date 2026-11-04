# D-1070: Motion Tile, Phase and Horizontal Phase Shift

B-950. An 8 by 8 white solid, half cleared by Linear Wipe, tiled at 50 per cent across and down: each tile is 4 pixels, and a line through it reads 1 1 0 0 1 1 0 0 (1 is solid, 0 clear). The middle tile covers the drawing's columns and rows 2 to 5; 0, 1, 6 and 7 are in the tiles either side. Phase slides those by Phase / 360 of a tile, so at 180 by 2 pixels: 0 0 1 1 0 0 1 1.

## Phase slides every other column down (top half cleared, read down a column)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Phase 0, column 0: as before | 1 1 0 0 1 1 0 0 | yes |
| Phase 180, column 4 (the middle tile): not moved | 1 1 0 0 1 1 0 0 | yes |
| Phase 180, column 0: slid down half a tile | 0 0 1 1 0 0 1 1 | yes |
| Phase 180, column 7: slid down half a tile | 0 0 1 1 0 0 1 1 | yes |
| Phase -180, column 0: half a tile up, which looks the same | 0 0 1 1 0 0 1 1 | yes |
| Phase 360, column 0: a whole tile, as Phase 0 | 1 1 0 0 1 1 0 0 | yes |
| Phase 90, column 0: a quarter tile, 1 pixel down | 0 1 1 0 0 1 1 0 | yes |
| Phase 180, read across row 0 (left half cleared): rows are not slid sideways | 1 1 0 0 1 1 0 0 | yes |

## Horizontal Phase Shift on: every other row slides right (left half cleared, read across)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Phase 180, row 0: slid right half a tile | 0 0 1 1 0 0 1 1 | yes |
| Phase 180, row 3 (the middle tile): not moved | 1 1 0 0 1 1 0 0 | yes |
| Phase 180, row 6: slid right half a tile | 0 0 1 1 0 0 1 1 | yes |
| Phase 180, read down column 0 (top half cleared): columns are not slid | 1 1 0 0 1 1 0 0 | yes |
| with Mirror on, row 0 reads mirror alone's 0 0 0 0 1 1 1 1 with each tile turned over where it lands, 2 pixels on: 1 1 0 0 0 0 1 1 | 1 1 0 0 0 0 1 1 | yes |

## At the drawing's own size, grown, and on the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| the drawing's own tile, output 200, Phase 180 across: the middle row of tiles reads 1111 0000 1111 0000 (the drawing in the middle) | 1 1 1 1 0 0 0 0 1 1 1 1 0 0 0 0 | yes |
| and the row of tiles above is slid half a tile, 4 pixels: 0000 1111 0000 1111 | 0 0 0 0 1 1 1 1 0 0 0 0 1 1 1 1 | yes |
| the viewer with the card draws row 0 the same (it hands a Phase to the CPU) | [0, 0, 255, 255, 0, 0, 255, 255] | yes |

## Saved and refused

| Check | The build's answer | Matches |
| --- | --- | --- |
| a file without them saves without them | no phase words | yes |
| set ones are written | phase 180, horizontal "on" | yes |
| an unknown Horizontal Phase Shift word switches the effect off | runs: false | yes |
| bricks.png: a quarter-size tile of a half-cleared square, Phase 0, then 180 with Horizontal Phase Shift, then 90 down | drawn | yes |

## Result

20 of 20 checks pass.
