# B-159: the layers below an edit, kept

Written by `tests/b159_below.rs`. Each layer of the reference shot, in five forms, is edited in turn three ways, and each edit is drawn by one viewer that keeps what it drew and compared, every working-space value by its bits, with a viewer that keeps nothing. "Started from kept" is whether the viewer began that frame from the layers below the edited one as it last drew them; the second and third edits of every layer but the bottom one must, and the next frame after the edits must not. Layers count from the bottom; the adjustment layer is layer 3 of its shot.

**288 of 544 pass.**

| Shot | Quality | Frame | Drawn | Edit | Same bytes | Started from kept | Result |
|---|---|---:|---|---|---|---|---|
| as it is | Full | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| as it is | Full | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| as it is | Full | 0 | whole | layer 1 moved | yes | no | pass |
| as it is | Full | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| as it is | Full | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| as it is | Full | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| as it is | Full | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| as it is | Full | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| as it is | Full | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| as it is | Full | 0 | whole | next frame, 1, as it is | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 1 moved | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| as it is | Full | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| as it is | Full | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| as it is | Full | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| as it is | Full | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| as it is | Full | 100 | whole | layer 1 moved | yes | no | pass |
| as it is | Full | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| as it is | Full | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| as it is | Full | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| as it is | Full | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| as it is | Full | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| as it is | Full | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| as it is | Full | 100 | whole | next frame, 101, as it is | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 1 moved | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| as it is | Full | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| as it is | Full | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| as it is | Draft | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| as it is | Draft | 0 | whole | layer 1 moved | yes | no | pass |
| as it is | Draft | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | whole | next frame, 1, as it is | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 1 moved | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| as it is | Draft | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| as it is | Draft | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| as it is | Draft | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| as it is | Draft | 100 | whole | layer 1 moved | yes | no | pass |
| as it is | Draft | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | whole | next frame, 101, as it is | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 1 moved | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| as it is | Draft | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| as it is | Draft | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 1 moved | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | whole | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | whole | next frame, 1, as it is | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 1 moved | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Full | 0 | middle quarter | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Full | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 1 moved | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | whole | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | whole | next frame, 101, as it is | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 1 moved | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Full | 100 | middle quarter | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Full | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 1 moved | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | whole | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | whole | next frame, 1, as it is | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 1 moved | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 0 | middle quarter | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 1 moved | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | whole | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | whole | next frame, 101, as it is | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 1 moved | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 5 opacity 80% | yes | no | pass |
| adjustment layer | Draft | 100 | middle quarter | layer 5 opacity 60% | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | layer 5 moved | yes | no | **FAIL** |
| adjustment layer | Draft | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 1 moved | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | whole | next frame, 1, as it is | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 1 moved | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Full | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Full | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 1 moved | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | whole | next frame, 101, as it is | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 1 moved | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Full | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Full | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 1 moved | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | whole | next frame, 1, as it is | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 1 moved | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 1 moved | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | whole | next frame, 101, as it is | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 1 moved | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| Light Wrap | Draft | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| Light Wrap | Draft | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| motion blur | Full | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| motion blur | Full | 0 | whole | layer 1 moved | yes | no | pass |
| motion blur | Full | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | whole | next frame, 1, as it is | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 1 moved | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| motion blur | Full | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| motion blur | Full | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| motion blur | Full | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| motion blur | Full | 100 | whole | layer 1 moved | yes | no | pass |
| motion blur | Full | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | whole | next frame, 101, as it is | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 1 moved | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| motion blur | Full | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| motion blur | Full | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 1 moved | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | whole | next frame, 1, as it is | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 1 moved | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| motion blur | Draft | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| motion blur | Draft | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 1 moved | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | whole | next frame, 101, as it is | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 1 moved | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| motion blur | Draft | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| motion blur | Draft | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| effects | Full | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| effects | Full | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| effects | Full | 0 | whole | layer 1 moved | yes | no | pass |
| effects | Full | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| effects | Full | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| effects | Full | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| effects | Full | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| effects | Full | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| effects | Full | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| effects | Full | 0 | whole | next frame, 1, as it is | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 1 moved | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| effects | Full | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| effects | Full | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| effects | Full | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| effects | Full | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| effects | Full | 100 | whole | layer 1 moved | yes | no | pass |
| effects | Full | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| effects | Full | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| effects | Full | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| effects | Full | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| effects | Full | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| effects | Full | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| effects | Full | 100 | whole | next frame, 101, as it is | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 1 moved | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| effects | Full | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| effects | Full | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
| effects | Draft | 0 | whole | layer 1 opacity 80% | yes | no | pass |
| effects | Draft | 0 | whole | layer 1 opacity 60% | yes | no | pass |
| effects | Draft | 0 | whole | layer 1 moved | yes | no | pass |
| effects | Draft | 0 | whole | layer 2 opacity 80% | yes | no | pass |
| effects | Draft | 0 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | whole | layer 2 moved | yes | no | **FAIL** |
| effects | Draft | 0 | whole | layer 3 opacity 80% | yes | no | pass |
| effects | Draft | 0 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | whole | layer 3 moved | yes | no | **FAIL** |
| effects | Draft | 0 | whole | layer 4 opacity 80% | yes | no | pass |
| effects | Draft | 0 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | whole | layer 4 moved | yes | no | **FAIL** |
| effects | Draft | 0 | whole | next frame, 1, as it is | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 1 moved | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| effects | Draft | 0 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| effects | Draft | 0 | middle quarter | next frame, 1, as it is | yes | no | pass |
| effects | Draft | 100 | whole | layer 1 opacity 80% | yes | no | pass |
| effects | Draft | 100 | whole | layer 1 opacity 60% | yes | no | pass |
| effects | Draft | 100 | whole | layer 1 moved | yes | no | pass |
| effects | Draft | 100 | whole | layer 2 opacity 80% | yes | no | pass |
| effects | Draft | 100 | whole | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | whole | layer 2 moved | yes | no | **FAIL** |
| effects | Draft | 100 | whole | layer 3 opacity 80% | yes | no | pass |
| effects | Draft | 100 | whole | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | whole | layer 3 moved | yes | no | **FAIL** |
| effects | Draft | 100 | whole | layer 4 opacity 80% | yes | no | pass |
| effects | Draft | 100 | whole | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | whole | layer 4 moved | yes | no | **FAIL** |
| effects | Draft | 100 | whole | next frame, 101, as it is | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 1 opacity 80% | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 1 opacity 60% | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 1 moved | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 2 opacity 80% | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 2 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | layer 2 moved | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | layer 3 opacity 80% | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 3 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | layer 3 moved | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | layer 4 opacity 80% | yes | no | pass |
| effects | Draft | 100 | middle quarter | layer 4 opacity 60% | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | layer 4 moved | yes | no | **FAIL** |
| effects | Draft | 100 | middle quarter | next frame, 101, as it is | yes | no | pass |
