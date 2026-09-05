# H-02 — the whole picture again, with the layers moved and scaled

**7 of 7 checks passed.**

Produced by `tests/h02_transformed_picture.rs`.

## Why this exists

H-01 composites your shot twice and compares every pixel, which is the strongest check in this build — but every layer of the shot sits exactly where it was drawn. Nothing there is moved, scaled or faded, so nothing there resamples, and the code that decides *where a moved layer's pixels land* is checked only by a table of named pixels on small made-up images.

So the same shot is composited twice again, with the layers pushed around: layer 2 moved 320 pixels right and 180 up, layer 3 blown up to twice its size at half opacity, layer 4 shrunk to half. Once by the real renderer, once by a second compositor written inside the test from document 21, which works out where each pixel comes from with ordinary arithmetic instead of the renderer's matrices.

## What to look at

- **`H-02_renderer_frame.png`** and **`H-02_independent_frame.png`** — frame 100, produced by the two compositors. The picture should look wrong in an obvious, deliberate way: pieces of your shot shifted and resized. The two files should look the same as each other. Compare them against `H-01_renderer_frame.png`, which is the same frame untouched.

## The tolerance

H-01 demands the two compositors agree *exactly*. This one allows a difference of **0.000001** per channel, and reports the largest difference it actually found so the allowance can be judged rather than taken on trust. The reason is arithmetic, not laxity: blending four neighbouring pixels means adding four numbers, and adding the same four numbers in a different order gives answers that differ in the last bit or two. Demanding identical bits would demand the two compositors do the arithmetic in the same order, which would make them the same compositor. A real fault — a wrong weight, an inverted transform, opacity applied at the wrong step — moves pixels by thousands of times more than the allowance.

## What is deliberately not here

**Rotation.** `cos(90°)` in floating point is not zero but 0.000000000000000061, so a rotated layer's samples land a hair off the pixel centres and two correct implementations disagree in the last bits for reasons that have nothing to do with either being wrong. Rotation is checked in `B-05a_transform_table.md`, pixel by named pixel, where that is not a problem. The moves and scales chosen here are exact in binary on purpose, which is what makes comparing whole frames fair.

As in H-01, both compositors were written from the same document by the same agent: this catches an implementation slip in the renderer, not a misreading of the specification.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| frame 0, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 0: the largest disagreement anywhere was 2.2475834271507011e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| frame 14, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 14: the largest disagreement anywhere was 2.3278194227760451e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| frame 100, layers moved and scaled: every one of the 2073600 pixels agrees with a second compositor to within 0.000001 | `0 pixels differ by more than the bound` | `0 pixels differ by more than the bound` | pass |
| frame 100: the largest disagreement anywhere was 2.2475834271507011e-7, float rounding rather than a fault | `no more than 0.000001` | `no more than 0.000001` | pass |
| the transforms changed the picture: the same frame without them does not match | `they differ` | `they differ` | pass |
