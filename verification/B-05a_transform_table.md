# B-05a transform fixtures

Test T-03 (render half), requirement R-03, fixtures FX-XF-001 through FX-XF-004 of document 25. Produced by `tests/b05a_transform.rs`. **30 of 30 checks pass.**

## What to check by eye

Every expected value in the table is a literal taken from document 21's rules, worked out by hand before the renderer ran. The comments in the test say where each one comes from. Nothing here was captured from a run of the code it is testing.

The four fixtures document 25 names:

- **FX-XF-001**, identity preserves pixels and bounds. A 4x4 image where no two pixels and no two channels share a value goes in, and the same floats come out.
- **FX-XF-002**, integer translation moves a 1x1 impulse exactly one pixel. One pixel is touched, not two: a whole-pixel shift must not blur.
- **FX-XF-003**, half-pixel translation verifies bilinear weights. The impulse becomes four pixels of exactly one quarter each, because 0.5 across times 0.5 down is 0.25.
- **FX-XF-004**, rotation about a nonzero anchor. A pixel to the left of the anchor ends up above it, which is clockwise, which is what document 21 calls positive.

Beside them are rows for the edge rule, opacity and scale, because those are the parts of the same code path that the four fixtures do not exercise.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| FX-XF-001: identity output extent equals the composition extent | `4x4` | `4x4` | PASS |
| FX-XF-001: identity reproduces every source pixel exactly, bit for bit | `identical` | `identical` | PASS |
| FX-XF-002: the impulse lands whole on the pixel one to the right | `[0.25, 0.5, 0.75, 1]` | `[0.25, 0.5, 0.75, 1]` | PASS |
| FX-XF-002: exactly one pixel is touched, so nothing was smeared | `[(3, 2)]` | `[(3, 2)]` | PASS |
| FX-XF-002: the pixel the impulse left is exactly transparent black | `[0, 0, 0, 0]` | `[0, 0, 0, 0]` | PASS |
| FX-XF-003: pixel (2, 2) carries one quarter of the impulse | `[0.0625, 0.125, 0.1875, 0.25]` | `[0.0625, 0.125, 0.1875, 0.25]` | PASS |
| FX-XF-003: pixel (3, 2) carries one quarter of the impulse | `[0.0625, 0.125, 0.1875, 0.25]` | `[0.0625, 0.125, 0.1875, 0.25]` | PASS |
| FX-XF-003: pixel (2, 3) carries one quarter of the impulse | `[0.0625, 0.125, 0.1875, 0.25]` | `[0.0625, 0.125, 0.1875, 0.25]` | PASS |
| FX-XF-003: pixel (3, 3) carries one quarter of the impulse | `[0.0625, 0.125, 0.1875, 0.25]` | `[0.0625, 0.125, 0.1875, 0.25]` | PASS |
| FX-XF-003: exactly four pixels are touched | `[(2, 2), (3, 2), (2, 3), (3, 3)]` | `[(2, 2), (3, 2), (2, 3), (3, 3)]` | PASS |
| FX-XF-003: the four quarters sum to the whole impulse, so no energy was invented | `1` | `1` | PASS |
| FX-XF-003: the weights are exactly 0.25, not merely close | `0.25` | `0.25` | PASS |
| outside the source extent is transparent black, so edge weights are not renormalised | `0.25` | `0.25` | PASS |
| the surviving quarter is the top-left pixel | `[(0, 0)]` | `[(0, 0)]` | PASS |
| an opaque edge shifted half a pixel fades to 0.25 at its corner, it does not clamp | `0.25` | `0.25` | PASS |
| and to 0.5 along its side | `0.5` | `0.5` | PASS |
| the interior stays fully opaque | `1` | `1` | PASS |
| FX-XF-004: a 90 degree rotation about the anchor sends left-of-centre to above-centre | `[0.250000, 0.500000, 0.750000, 1.000000]` | `[0.250000, 0.500000, 0.750000, 1.000000]` | PASS |
| FX-XF-004: the pixel the impulse left is empty to six decimal places | `[0.000000, 0.000000, 0.000000, 0.000000]` | `[0.000000, 0.000000, 0.000000, 0.000000]` | PASS |
| FX-XF-004: total alpha is preserved, so the rotation neither lost nor invented cover | `1.000000` | `1.000000` | PASS |
| a non-uniform scale is applied in the layer's own axes, before the rotation turns them | `[0.250000, 0.500000, 0.750000, 1.000000]` | `[0.250000, 0.500000, 0.750000, 1.000000]` | PASS |
| the doubled axis spreads the impulse: the pixel below takes exactly half of it | `[0.125000, 0.250000, 0.375000, 0.500000]` | `[0.125000, 0.250000, 0.375000, 0.500000]` | PASS |
| a 360 degree rotation returns the impulse to six decimal places | `[0.250000, 0.500000, 0.750000, 1.000000]` | `[0.250000, 0.500000, 0.750000, 1.000000]` | PASS |
| scale 1.0 is identity: the opaque 4x4 source stays 4x4 of fully opaque output | `16` | `16` | PASS |
| scale 2.0 doubles it: 8x8 of coverage with a fully opaque 6x6 interior | `36` | `36` | PASS |
| a scale of zero renders nothing rather than dividing by zero | `0` | `0` | PASS |
| layer opacity 0.5 halves the premultiplied sample, RGB and alpha together | `[0.125, 0.25, 0.375, 0.5]` | `[0.125, 0.25, 0.375, 0.5]` | PASS |
| a layer at zero opacity contributes nothing at all, not a faint one | `0` | `0` | PASS |
| bilinear weight of the one nonzero tap at (0.9, 0.6) is 0.4 * 0.9 | `0.360000` | `0.360000` | PASS |
| and swapping the sample coordinates gives a different answer, 0.1 * 0.6 | `0.060000` | `0.060000` | PASS |

## Notes

- FX-XF-004 is checked to six decimal places rather than exactly. `cos(90 degrees)` in f64 is 6.1e-17 and not 0, so a right-angle rotation leaks about 1e-16 of the impulse into its neighbours. Document 21 asks for "exact/near-exact float comparisons appropriate to the operation"; that is what near-exact means for a trigonometric one. Every other fixture on this page is compared exactly.


## Not run by this test

- Masks, effects and alpha mattes, which are steps 2, 3 and 5 of document 21's layer render order. Masks are parked to G1-rest with R-04 under D-12; effects and mattes are B-06. A layer here is decoded, transformed, faded and composited, and nothing else.
- The multiply, screen and add blend modes, which are now implemented and are verified in `verification/B-05c_blend_table.md` against document 25's FX-B fixtures. Every layer in this table is `normal`.
- Tile margins for neighbourhood operations. Every operation in G1-core is per-pixel, so no margin is needed yet. Document 21 says the first one that needs it is the blur in R-05, which is parked.
- Sub-pixel-accurate polygon rasterisation, which document 21 requires be fixture-tested before subpixel equivalence is claimed. No polygon exists yet.
