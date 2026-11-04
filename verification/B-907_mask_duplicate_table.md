# B-907: Ctrl+D with a mask chosen copies the mask (D-1028)

Written by `b907_mask_duplicate` in `app/src/tests/w12v.rs`.

After Effects: with a mask selected, Edit > Duplicate (Ctrl+D) copies it, everything included, just below it. The first rows run the window's own command on `Fixtures/projects/cel_holds_project.json`; the last rows pin how the page sends it.

**14 of 14 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Two masks to start: Mask 1 subtract, feathered, inverted, its path keyed at 0 and 10 | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | pass |
| Ctrl+D on Mask 1: the copy sits just below it, named Mask 3 (the next free), mode, feather, inverted and keys kept | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 3 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 3 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | pass |
| The copy's path is the original's, key for key | true | true | pass |
| One entry to undo, named for the new mask | Draw Mask 3 | Draw Mask 3 | pass |
| Undo takes the copy back | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | pass |
| A mask that is not there is refused, with how many there are | Which mask? "Cel" has 2. | Which mask? "Cel" has 2. | pass |
| and nothing changed | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | Mask 1 subtract 4 points 2 keys, feather 8, inverted; Mask 2 add 4 points 0 keys, feather 0 | pass |
| The page holds: Ctrl+D (and the finder line that presses it) copies the chosen mask first | true | true | pass |
| The page holds: only with a mask chosen, one layer chosen, and a layer of masks rather than shapes | true | true | pass |
| The page holds: the copy becomes the mask chosen | true | true | pass |
| The page holds: a mask's name in the timeline chooses it (a shape's does not) | true | true | pass |
| The page holds: a double-click on its outline chooses it | true | true | pass |
| The page holds: choosing a layer lets the mask go | true | true | pass |
| The page holds: and so does choosing several | true | true | pass |
