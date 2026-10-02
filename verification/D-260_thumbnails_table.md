# D-260: the Project panel's small pictures

Written by `d260_a_small_picture_is_its_full_picture_shrunk` in `app/src/main.rs`. `thumbnail` is the answer to the page's `/thumb`; the expected pictures are shrunk here by the test's own loop, each 12 by 12 block of pixels averaged.

**5 of 5 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| layer2's small picture is its first drawing, layer2_000.png, a twelfth each way | 160 by 90, every byte the same | 160 by 90, every byte the same | PASS |
| the reference shot's small picture at frame 10 is its full frame 10, a twelfth each way | 160 by 90, every byte the same | 160 by 90, every byte the same | PASS |
| and it is a picture, not an empty one | some pixels drawn | some pixels drawn | PASS |
| a name that is no item is refused | refused | refused | PASS |
| asking changes nothing: the same composition on screen, no undo step, the same frames ready | unchanged | unchanged | PASS |
