# B-684: a drawing too faint for half precision goes to the card in 32-bit floats

Written by `tests/b684_faint_upload.rs`. b222's three failed cases, Hue/Saturation, Exposure and Levels after Pixel Motion Blur or Timewarp in one run, drawn by the CPU and by the card. Rule: no channel of any pixel more than 1 level of 255 apart (ADR-006, D-100). Before B-684 these were 10, 10 and 6 levels apart.

| Case | Pixels with alpha above 0 but under half a level | Pixels differing | Largest (levels) | Result |
|---|---:|---:|---:|---|
| fx_pmb_013 frame 1, Full | 12 | 0 | 0 | PASS |
| fx_pmb_013 frame 2, Draft | 6 | 0 | 0 | PASS |
| fx_tw_020 frame 5, Draft | 2 | 0 | 0 | PASS |

3 of 3 cases pass.

Pictures in `verification/B-684 pictures/`: the CPU's frame, the card's, and the difference (25 times brighter per level; black where they agree).
