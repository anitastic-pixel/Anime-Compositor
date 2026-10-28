# B-830: the Align panel (D-950)

Written by `b830_align_panel` in `app/src/tests/w12r.rs`.

The first rows run the page's own `alignMoves` under Node on three made-up boxes in a 1920 by 1080 composition (A: 0 to 100 across, 0 to 50 down; B: 200 to 260, 100 to 200; C: 500 to 540, 20 to 60), each answer the distance each layer moves; one row asks the real window; the rest pin the wiring. The private-app check is `verification/B-830_private_app_table.md`.

**16 of 16 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| left edges to the selection's left | A 0,0; B -200,0; C -500,0 | A 0,0; B -200,0; C -500,0 | pass |
| right edges to the composition's right (1920) | A 1820,0; B 1660,0; C 1380,0 | A 1820,0; B 1660,0; C 1380,0 | pass |
| centres to the selection's centre (270) | A 220,0; B 40,0; C -250,0 | A 220,0; B 40,0; C -250,0 | pass |
| middles to the composition's middle (540) | A 0,515; B 0,390; C 0,500 | A 0,515; B 0,390; C 0,500 | pass |
| bottoms to the selection's bottom (200) | A 0,150; B 0,0; C 0,140 | A 0,150; B 0,0; C 0,140 | pass |
| tops to the composition's top | A 0,0; B 0,-100; C 0,-20 | A 0,0; B 0,-100; C 0,-20 | pass |
| distribute left edges: the outer two stay, the middle one goes halfway | A 0,0; B 50,0; C 0,0 | A 0,0; B 50,0; C 0,0 | pass |
| distribute centres (50, 230, 520 become 50, 285, 520) | A 0,0; B 55,0; C 0,0 | A 0,0; B 55,0; C 0,0 | pass |
| distribute middles (25, 40, 150 become 25, 87.5, 150) | A 0,0; C 0,47.5; B 0,0 | A 0,0; C 0,47.5; B 0,0 | pass |
| distribute with two layers does nothing |  |  | pass |
| one layer and Selection aligns to the composition | B -200,0 | B -200,0 | pass |
| the window: 3 layers moved as the panel sends, then one Ctrl+Z | moved, 1 entry, all back | moved, 1 entry, all back | pass |
| the panel sits in the Effect controls column | true | true | pass |
| to the selection or the composition | true | true | pass |
| a parented layer is left out (its Position is in its parent's space) | true | true | pass |
| each button moves through alignPanel | true | true | pass |
