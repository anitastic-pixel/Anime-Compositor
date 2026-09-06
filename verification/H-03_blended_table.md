# H-03 — the whole picture again, with the layers set to multiply, screen and add

**10 of 10 checks passed.**

Produced by `tests/h03_blended_picture.rs`.

## Why this exists

H-01 composites your shot twice and compares every pixel; H-02 does it again with the layers moved and scaled. Every layer in both is set to **normal**, which is the one mode that takes a different route through the renderer. The arithmetic that multiply, screen and add all share had never been run on a real frame by anything in this project - only on eighteen single pixels of made-up colour in `B-05c_blend_table.md`, which says so itself.

So the shot is composited twice again, with the second layer set to multiply, the third to screen at half opacity and the fourth to add. Once by the real renderer, once by a second compositor written inside the test from document 21's four equations.

## What to look at

- **`H-03_renderer_frame.png`** and **`H-03_independent_frame.png`** — frame 100, produced by the two compositors. The picture should look wrong in an obvious, deliberate way: parts of your shot darkened where they overlap, others brightened or blown out. The two files should look the same as each other. `H-01_renderer_frame.png` is the same frame with every layer left on normal, for comparison.

## The rows that are not pixel comparisons

Three of these rows are here because the pixel comparisons above them, on their own, cannot fail for certain faults. Each was written after breaking the code on purpose and watching the break get through; the mutation report has the detail.

**The modes did something.** The same four layers, same opacities, set back to normal must give a picture far away from this one, not one that differs in the last bits. Without it, a build that stored your choice of mode and then ignored it would pass every other row, because the second compositor would be told the same modes and a mode nobody applies changes nothing on either side.

**A layer over nothing keeps its own colour.** The blended term is weighted by the alphas of *both* the layer and what is under it, so over an empty background it vanishes and a screen layer at the bottom of a stack does not blow out against the emptiness. The reference shot's own bottom layer is normal, so every other row in this table blends onto something opaque, where dropping that weight changes nothing at all.

**Soft edges meeting soft edges.** One more picture, at frame 110, with the opaque background removed so the modes land on half-transparent pixels rather than solid ones. The rule for how two transparencies combine is only distinguishable from the simpler wrong answers when *both* are partly transparent, which never happens once the background has filled the frame. The row after it counts how many times that actually occurred and fails if it is not thousands — at frame 100 it happens **zero** times, which is how the frame was chosen.

## The tolerance

A difference of **0.000001** per channel is allowed, and the largest difference actually found is reported so the allowance can be judged rather than taken on trust. The reason is arithmetic: these modes divide colour by alpha, and this compositor works to about seventeen digits where the renderer works to about seven. Demanding identical bits would demand the two do the same operations in the same order, which would make them one compositor wearing two hats. A real fault — a blend applied to the wrong kind of colour, a missing weight, opacity applied after the blend instead of before — moves pixels by thousands of times more than the allowance.

## What is deliberately not here

**Transforms.** Every layer sits where it was drawn, so nothing resamples. H-02 owns moving and scaling; mixing the two would mean a disagreement could have come from either and neither could be blamed for it.

As in H-01 and H-02, both compositors were written from the same document by the same agent: this catches an implementation slip in the renderer, not a misreading of the specification.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| frame 0, layers set to multiply, screen and add: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 0: the largest disagreement anywhere was 1.9860273214877822e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| frame 14, layers set to multiply, screen and add: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 14: the largest disagreement anywhere was 2.8052271294765063e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| frame 100, layers set to multiply, screen and add: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 100: the largest disagreement anywhere was 2.1070894662411632e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| the modes changed the picture: the same frame with every layer set to normal is 0.7908 away at its furthest pixel, not a rounding difference | `further apart than 0.01` | `further apart than 0.01` | pass |
| a screen layer over nothing at all is its own colour, exactly as the same layer set to normal would be | `identical` | `identical` | pass |
| frame 110 with the opaque background layer removed: the modes still agree with the second compositor where soft edges meet soft edges | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| and that stack really does put partly-transparent pixels over partly-transparent ones - it happened 22502 times - so the row above is testing something | `more than 1000 such pixels` | `more than 1000 such pixels` | pass |
