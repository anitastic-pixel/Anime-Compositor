# Test fixture catalog and tolerances

Version 0.2 | 2026-09-04 | Proposed baseline

## Fixture authority

This catalog binds test IDs to independent inputs, expected behavior and tolerances. The machine-readable subset is `Fixtures/fixture_manifest.json`. A production implementation may generate additional cases, but passing self-generated snapshots alone is insufficient.

## Numeric fixtures

| Fixture ID | Purpose | Input | Expected result / rule | Tolerance |
|---|---|---|---|---|
| FX-A-001 | transparent source-over | S=(0,0,0,0), D=(0.2,0.4,0.6,1) | D unchanged | 1e-7 CPU |
| FX-A-002 | opaque source-over | S=(0.8,0.1,0.2,1), any D | S | 1e-7 CPU |
| FX-A-003 | partial alpha over opaque | straight S=(1,0,0), As=.5; D=(0,0,1), Ad=1 | premul output=(.5,0,.5,1) | 1e-6 CPU |
| FX-A-004 | zero-alpha unpremultiply | C=(0,0,0), A=0 | straight RGB=(0,0,0), finite | exact |
| FX-B-001 | multiply opaque | red over 50% gray | B=(.5,0,0), A=1 | 1e-6 |
| FX-B-002 | screen opaque | .5 gray over .5 gray | .75 gray, A=1 | 1e-6 |
| FX-B-003 | add opaque | .7 + .6 gray | 1.0 clamped, A=1 | 1e-6 |
| FX-E-001 | exposure identity | e=0 | unchanged | 1e-7 |
| FX-E-002 | exposure +1 | RGB=.25, e=1 | RGB=.5; alpha unchanged | 1e-6 |
| FX-T-001 | tint zero | amount=0 | unchanged | 1e-7 |
| FX-T-002 | tint full | amount=1, alpha=.5 | straight RGB=tint; premultiplied by .5 | 1e-6 |

## Time fixtures

FX-TIME-001: exposure spans map composition frames 0..4 to drawing numbers `[1,1,2,2,2]` exactly through save, preview and export.

FX-TIME-002: sequence numbers `[1001,1002,1004]` with a requested 1003 produce `MEDIA_SEQUENCE_GAP`/missing drawing, not substitution.

FX-TIME-003: 24000/1001 remains the exact stored rate after round-trip; no replacement with decimal 23.976 authority.

FX-TIME-004: composition `start_frame=-12`, `duration=24` has valid frames -12 through 11 and exactly 24 export frames.

## Ease fixtures

Added on 2026-09-12 by D-52. Each case is one segment: two keyframes, the four numbers of the curve on the first of them, and the value expected at each of a handful of frames. The expected values are written out here rather than in the machine-readable manifest, because a number a person can read beside the case it belongs to is the point of this document, and because the build's own table prints the same rows for comparison.

**Every number below is produced by `tools/ease_reference.py`**, which solves document 20's equation by bisection, in another language, written from document 20 rather than from the build. Running that file prints this table. The build solves the same equation by Newton's method with a bisection fallback, so an agreement between the two is two independent answers and not one answer twice.

FX-EASE-001: the curve `[1/3, 1/3, 2/3, 2/3]` evaluates as linear, to 1e-9. An implementation that rounds, clamps or shortcuts the solve fails this one first. A scalar from 0 at frame 0 to 120 at frame 24.

| frame | 0 | 1 | 6 | 12 | 18 | 23 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 4.999999999999999 | 30 | 60 | 90 | 115 | 120 |

Frame 1 is not exactly 5, and that is the tolerance earning its place: the true answer is 5, the bisection lands one unit in the last place below it, and 1e-9 is far wider than the gap. A build that printed exactly 5 would also pass.

FX-EASE-002: easy ease, `[1/3, 0, 2/3, 1]`, on a scalar from 0 at frame 0 to 100 at frame 24. Tolerance 1e-6. The midpoint is exactly 50, and the whole row can be checked by hand against `100 * (3u^2 - 2u^3)`, which is what document 20 says this curve reduces to.

| frame | 0 | 3 | 6 | 12 | 18 | 21 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 4.296875 | 15.625 | 50 | 84.375 | 95.703125 | 100 |

The row is symmetric about its middle - 4.296875 from the start and 95.703125 from the end are the same distance - which is the shape of an ease that starts and stops at rest.

FX-EASE-003: the same curve on a pair, position from (0, 200) at frame 0 to (300, -40) at frame 12. Tolerance 1e-6. One timing curve drives both components; this is an ease in time, not a path in space.

| frame | 0 | 3 | 6 | 9 | 12 |
| --- | --- | --- | --- | --- | --- |
| x | 0 | 46.875 | 150 | 253.125 | 300 |
| y | 200 | 162.5 | 80 | -2.5 | -40 |

Both rows are the same fractions of their own journey, which is the check that matters here: at frame 3 the layer is 15.625% of the way along in x and 15.625% of the way along in y, so it is still on the straight line between the two keys and only its speed along that line has changed.

FX-EASE-004: `[0, 0, 0.58, 1]`, which is not symmetric, so a solver that assumes symmetry has somewhere to break. A scalar from 0 at frame 10 to 1 at frame 34, which also checks that a segment need not start at frame 0. Tolerance 1e-6.

| frame | 10 | 16 | 22 | 28 | 34 |
| --- | --- | --- | --- | --- | --- |
| value | 0 | 0.37813813082510966 | 0.6846431874274606 | 0.9065353492811752 | 1 |

This one leaves its first key fast and arrives at its second slowly, so the value at the middle frame is above half rather than at it. A solver that mirrored the curve would put it below.

FX-EASE-005: `[0.42, 0, 0.58, 1]`, whose `x` handles are not a third and two thirds, so the solve is real work rather than free. A scalar from 0 at frame 0 to 1 at frame 20. Tolerance 1e-6. Its presence is deliberate: the first two cases are curves where `x(t) = t` exactly, and a fixture set made only of those would not notice a solver that never worked.

| frame | 0 | 4 | 8 | 10 | 12 | 16 | 20 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| value | 0 | 0.08165985626589747 | 0.3318838700976461 | 0.5 | 0.6681161299023539 | 0.9183401437341024 | 1 |

The curve is symmetric, so the row is too, and the exact 0.5 at the middle frame is the one number in it that can be checked without a calculator.

## Motion path fixtures

Added on 2026-09-12 by D-53. Each case is one segment of `position`: two keyframes, the outgoing handle of the first and the incoming handle of the second as offsets in composition pixels from their own key, and the point expected at each of a handful of frames. The ease fixtures above test *when*; these test *where*, and FX-PATH-003 tests both at once.

**Every number below is produced by `tools/path_reference.py`**, which evaluates document 20's cubic by de Casteljau - repeated linear interpolation - in another language, written from document 20 rather than from the build. Running that file prints these tables. The build expands the polynomial, so an agreement between the two is two answers and not one answer twice. Unlike the ease there is nothing to solve: the parameter is the fraction of the way through the segment, so every point is a cubic in that fraction, and most of the numbers below are exact.

FX-PATH-001: the handles that make a straight line, written out. Position from (0, 0) at frame 0 to (240, 120) at frame 24, outgoing handle `[80, 40]` and incoming handle `[-80, -40]`, which are one third of the way to the other key in each direction. Tolerance 1e-9. This must agree with the linear segment that already exists to that tolerance, and it must also agree with the same two keys carrying *no* handles at all, which is what document 19 says an absent `spatial` means.

| frame | 0 | 1 | 6 | 12 | 18 | 23 | 24 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| x | 0 | 9.999999999999998 | 60 | 120 | 180 | 230.00000000000006 | 240 |
| y | 0 | 4.999999999999999 | 30 | 60 | 90 | 115.00000000000003 | 120 |

Frames 1 and 23 are not exactly 10 and 230, and that is the tolerance earning its place, as it does in FX-EASE-001: de Casteljau reaches the point by five roundings, the answer is a few units in the last place off, and 1e-9 is far wider than the gap. A build that printed exactly 10 would also pass. The other five columns are exact because a quarter, a half and three quarters are.

FX-PATH-002: one curved segment under linear timing. Position from (0, 0) at frame 0 to (240, 240) at frame 24, outgoing handle `[160, 0]` and incoming handle `[0, -160]`: the layer leaves the first key heading right and arrives at the second heading down, so it rounds the corner instead of cutting across it. Tolerance 1e-6.

| frame | 0 | 6 | 12 | 18 | 24 |
| --- | --- | --- | --- | --- | --- |
| x | 0 | 105 | 180 | 225 | 240 |
| y | 0 | 15 | 60 | 135 | 240 |

The middle frame can be checked by hand: the midpoint of a cubic Bezier is `(P0 + 3 P1 + 3 P2 + P3) / 8`, which with control points (0,0), (160,0), (240,80), (240,240) is (180, 60). The straight line would have been at (120, 120), and the difference between those two points is the whole of what this decision adds.

FX-PATH-003: the same segment as FX-PATH-002 with easy ease, `[1/3, 0, 2/3, 1]`, on its first keyframe. Tolerance 1e-6. This is the case that shows the two curves are independent and compose in one order: the ease turns frame 6 into the fraction 0.15625, exactly the 15.625% FX-EASE-002 has at its own frame 6, and the path then says where 0.15625 of the way along the curve is.

| frame | 0 | 6 | 12 | 18 | 24 |
| --- | --- | --- | --- | --- | --- |
| x | 0 | 69.140625 | 180 | 234.140625 | 240 |
| y | 0 | 5.859375 | 60 | 170.859375 | 240 |

Frame 12 is (180, 60), the same point as FX-PATH-002's frame 12, because easy ease is exactly half way at half the segment and the ease never moves the path - only when the layer is at each point on it. Frames 6 and 18 are closer to the ends than FX-PATH-002's, which is the ease doing what it does.

FX-PATH-004: a handle longer than its own segment. Position from (0, 0) at frame 0 to (200, 0) at frame 20, outgoing handle `[-150, 90]` and incoming handle `[150, 90]`: the layer sets off *away* from its destination, swings down and round, and overshoots past it before coming back. Tolerance 1e-6. This exists because the same thing in time is forbidden - an `x` handle outside its segment gives one frame two values and is diagnosed - and a fixture set that never left the segment would not show that space is different.

| frame | 0 | 5 | 10 | 15 | 20 |
| --- | --- | --- | --- | --- | --- |
| x | 0 | -10.9375 | 100 | 210.9375 | 200 |
| y | 0 | 50.625 | 67.5 | 50.625 | 0 |

The `x` at frame 5 is negative, which is the layer to the left of where it started, and the `x` at frame 15 is past 200, which is the layer beyond where it will finish; both are meant. The row of `y` is symmetric because the handles are mirror images, and its middle value is `(0 + 3*90 + 3*90 + 0) / 8`.

## Transform fixtures

FX-XF-001 identity preserves pixels and bounds. FX-XF-002 integer translation moves a 1x1 impulse exactly one pixel. FX-XF-003 half-pixel translation verifies bilinear weights. FX-XF-004 rotates around a nonzero anchor using the matrix order in 21.

## Parenting fixtures

Proposed on 2026-09-15 by D-57, awaiting the owner. Each case is a small chain of layers, every one of them a transform with no keys unless the case says otherwise, and the composition point expected for a handful of points in the child's own layer space. Anchor, position and scale default to (0, 0), (0, 0) and (100, 100), rotation to 0.

**Every number below is produced by `tools/parent_reference.py`**, which carries a point through document 21's four steps one at a time - subtract the anchor, scale, rotate clockwise, add the position - and then hands it to the parent. It never builds a matrix; the build multiplies them. Tolerance 1e-9 throughout, because a rotation of 90 degrees is not exactly a quarter turn in floating point.

FX-PARENT-001: one parent, nothing animated. Parent P: anchor (50, 50), position (400, 300), rotation 90. Child C: position (100, 0), parent P.

| child point | (0, 0) | (10, 0) | (0, 10) |
| --- | --- | --- | --- |
| x | 450 | 450 | 440 |
| y | 350 | 360 | 350 |

By hand: the child's origin is (100, 0) in P's space, which is (50, -50) from P's anchor; a quarter turn clockwise makes that (50, 50), and P's position adds (400, 300). A step to the right along the child is a step down the screen, because the parent is turned.

FX-PARENT-002: a chain of three, with the middle one animated. Grandparent G: position (960, 540), scale (50, 50). Parent P: position (200, 0), rotation keyed linearly from 0 at frame 0 to 90 at frame 24, parent G. Child C: position (100, 0), parent P.

| frame | 0 | 6 | 12 | 24 |
| --- | --- | --- | --- | --- |
| origin x | 1110 | 1106.1939766255643 | 1095.3553390593274 | 1060 |
| origin y | 540 | 559.1341716182545 | 575.3553390593274 | 590 |
| (20, 0) x | 1120 | 1115.4327719506773 | 1102.4264068711927 | 1060 |
| (20, 0) y | 540 | 562.9610059419053 | 582.4264068711929 | 600 |

The grandparent's half scale halves everything below it, including the child's 20-pixel step, which is 10 pixels on screen at every frame. The child has no keys of its own and still moves, which is the point of the case.

FX-PARENT-003: what is not inherited. FX-PARENT-001's two layers, with P's opacity 0.5, P switched off, P's out point at frame 24, and the composition read at frame 30. C is a 1x1 opaque white drawing with opacity 1. Expected: C's points land exactly as in FX-PARENT-001; pixel (449, 350), whose centre is where C's one pixel centre lands, reads white at alpha 1; P draws nothing. A build that multiplied C by P's opacity, or dropped P's transform because P is off or out of range, fails this.

FX-PARENT-004: loops. Layers A and B, both unparented. Setting A's parent to B succeeds. Setting B's parent to A is refused with `PARENT_CYCLE`, and so is setting A's parent to A; neither refusal changes the document revision, dirty state or undo stack. A project file in which A and B name each other is reported with `PARENT_CYCLE`, treated as document 28 treats `MATTE_CYCLE`.

FX-PARENT-005: keeping place, in the exact case. Parent P: anchor (50, 50), position (400, 300), scale (200, 200), rotation 90. Child C, unparented: anchor (10, 20), position (500, 400), rotation 30. Setting C's parent to P at frame 0 must give C these values:

| anchor | position | scale | rotation |
| --- | --- | --- | --- |
| (10, 20) | (100, 0) | (50, 50) | -60 |

and every corner of C's 100-by-100 layer must land where it was:

| corner | (0, 0) | (100, 0) | (0, 100) | (100, 100) |
| --- | --- | --- | --- | --- |
| x | 501.33974596215563 | 587.9422863405995 | 451.33974596215563 | 537.9422863405995 |
| y | 377.6794919243112 | 427.6794919243112 | 464.2820323027551 | 514.282032302755 |

The reference carries the corners through before and after and finds them equal to within 1e-13.

FX-PARENT-006: keeping place, in the case that cannot be exact. Parent P: position (300, 200), scale (200, 100). Child C, unparented: position (500, 400), rotation 45. Setting C's parent to P at frame 0 gives position (100, 200), scale (50, 100), rotation 45, and the command reports that the layer could not keep its shape exactly. C's anchor corner lands where it was; the others do not:

| corner | (0, 0) | (100, 0) | (0, 100) | (100, 100) |
| --- | --- | --- | --- | --- |
| before x | 500 | 570.7106781186548 | 429.28932188134524 | 500 |
| before y | 400 | 470.71067811865476 | 470.71067811865476 | 541.4213562373095 |
| after x | 500 | 570.7106781186548 | 358.5786437626905 | 429.28932188134524 |
| after y | 400 | 435.3553390593274 | 470.71067811865476 | 506.06601717798213 |

This case pins what the build does when it cannot do the right thing, so that it does the same wrong thing every time and says so.

FX-PARENT-007: the file. `Fixtures/projects/parenting_project.json` holds FX-PARENT-002's chain as `layer-grand`, `layer-parent` and `layer-child`, and a fourth layer, `layer-orphan`, at position (10, 10) whose parent is `layer-gone`, which does not exist. Loading it reports one `PARENT_REFERENCE_MISSING` naming `layer-orphan` and `layer-gone`, beside the `MEDIA_MISSING` its absent drawings already give; `layer-child`'s origin matches FX-PARENT-002 at frames 0, 6, 12 and 24; `layer-orphan`'s origin lands at (10, 10). Saving it again writes `"parent": "layer-gone"` unchanged, and no layer without a parent gains a `parent` field.

FX-PARENT-008: deleting a parent. From FX-PARENT-005's result, deleting P leaves C with no parent and with its FX-PARENT-005 starting values, anchor (10, 20), position (500, 400), scale (100, 100), rotation 30, to within 1e-9, as one entry to undo. Undo restores P, and C's parent and values from FX-PARENT-005's table.

## Persistence fixtures

`Fixtures/projects/minimal_project.json`: smallest valid project. `cel_holds_project.json`: explicit exposure spans. `unicode_paths_project.json`: non-ASCII display/path fields. `missing_media_project.json`: valid project with intentionally unavailable asset. `unknown_effect_project.json`: structurally valid unknown effect that must survive load/save with a warning.

## Failure fixtures

FX-IO-001 interrupted replacement retains last valid project. FX-IO-002 disk-full/write failure reports `PROJECT_SAVE_FAILED` and does not truncate the previous valid save. FX-MATTE-001 creates A->B and B->A matte references and must be rejected with `MATTE_CYCLE`. FX-PARENT-004, above, does the same for a parent loop with `PARENT_CYCLE`.

## Image/filter fixtures

Gaussian blur uses a synthetic single-pixel impulse in a transparent image. Expected weights are independently generated from the normalized sigma/kernel definition in 21. Test symmetry, normalization, alpha behavior and expanded bounds.

Mask rasterization fixtures must record the exact rasterizer/reference tool once selected in B-06; until then, polygon interior/exterior topology tests are authoritative but subpixel edge goldens are OPEN.

**Selected in B-06 on 2026-09-06 and recorded in ADR-016.** The rasterizer is a 4x4 ordered grid of sample points per pixel, coverage being the count of samples inside the polygon over sixteen, with insideness by the even-odd rule; the sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`. The reference tool is a second implementation written from document 19 by a different method - summing the signed angle each edge subtends at the sample point - living in `tests/b06_mask.rs` rather than a drawing library, on the same discipline H-03 and H-04 use. Coverage is therefore an exact rational and the edge quantum is one sixteenth, which this sentence's OPEN now closes: subpixel edge goldens are written, in `verification/B-06_mask_table.md`. Raising the grid changes every one of them and is an amendment to ADR-016.

## CPU/GPU comparison

Simple arithmetic: absolute per-channel error <= 1e-5. Filter operations: tolerance is declared per fixture after the independent CPU reference is implemented; default target <= 2e-5 for float output. Exported integer PNG tests compare decoded integer samples exactly where deterministic quantization is specified.

A backend that exceeds tolerance requires diagnosis. Do not loosen tolerances globally to hide a backend-specific error.

## Workflow fixtures

RSH-01 and RSH-02 are specified in 22. They are manually reviewed in addition to automated tests. Their visual acceptance cannot replace the numeric fixtures above.

Related documents: 11, 20, 21, 22 and 28.
