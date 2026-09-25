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

Accepted on 2026-09-15 by D-57. Each case is a small chain of layers, every one of them a transform with no keys unless the case says otherwise, and the composition point expected for a handful of points in the child's own layer space. Anchor, position and scale default to (0, 0), (0, 0) and (100, 100), rotation to 0.

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

## Camera and depth fixtures

Specified on 2026-09-15 by D-58, accepted by the owner the same day and built by B-13c. Every case is a composition 1920 by 1080, so
the centre of the frame is (960, 540). Layer transforms are document 21's, with no keys unless
the case says otherwise, and no depth unless the case gives one.

Two cameras appear below and the difference matters. **The default camera** is the one a
composition has when its file says nothing: a 50 mm lens on a 36 mm film back, which is After
Effects' own default, so on a 1920-wide composition its zoom is 1920 x 50 / 36 and it sits that
far back - position (960, 540), depth and zoom 2666.6666666666665. FX-CAM-001 and FX-CAM-011 are
its cases. **Every other case names its own camera** at depth -1920 and zoom 1920, a 36 mm lens,
because a fixture that pins parallax is easier to read and to check by hand in whole pixels. A
plane at depth 0 is drawn at 1:1 under either of them, which is why the lens can be argued about
without touching a number in any fixture written before D-58.

**Every number below is produced by `tools/camera_reference.py`**, which reuses
`tools/parent_reference.py` to walk a point through document 21's four steps one at a time, and
then does D-58's two lines by hand - the scale `zoom / (world depth - camera depth)`, and the
point measured from the camera and scaled about the centre of the frame. It never builds a
matrix; the build folds the projection into the layer's transform and multiplies matrices.
Tolerance 1e-9.

FX-CAM-001: the camera nobody has touched changes nothing. One layer, anchor (50, 50), position
(400, 300), scale (150, 150), rotation 30, no depth, seen by the default camera.

| layer point | comp x | comp y | screen x | screen y |
| --- | --- | --- | --- | --- |
| (0, 0) | 372.5480947161671 | 197.5480947161671 | 372.54809471616704 | 197.5480947161671 |
| (10, 0) | 385.5384757729337 | 205.0480947161671 | 385.5384757729337 | 205.0480947161671 |
| (0, 10) | 365.0480947161671 | 210.53847577293368 | 365.04809471616704 | 210.53847577293368 |

**Look at the first row before reading on.** The two halves agree to 1e-13 and not bit for bit,
because carrying a point out to the camera and back is two roundings. That is why D-58 requires
a build to leave out a projection that is the identity rather than to apply one: a build that
applies it puts every transform fixture written before D-58 within a tolerance of its expected
value instead of exactly on it. This case exists to catch that, and it was found by running the
reference rather than by argument. It is also why the lens could be changed after the fact -
these three rows are the same under any zoom, because the camera always sits at `-zoom`.

FX-CAM-002: one plane, at four depths, seen by a 36 mm camera the file names for itself. The
layer is at position (960, 540), which is the centre of the frame, so its origin cannot move and
only its size answers.

| depth | drawn at | origin x | origin y | (100,0) x | (100,0) y |
| --- | --- | --- | --- | --- | --- |
| -960 | 2 | 960 | 540 | 1160 | 540 |
| 0 | 1 | 960 | 540 | 1060 | 540 |
| 960 | 0.6666666666666666 | 960 | 540 | 1026.6666666666667 | 540 |
| 1920 | 0.5 | 960 | 540 | 1010 | 540 |

Depth 1920 is twice as far from this camera as depth 0 and is drawn half the size, which is the
convention D-56 accepted. Depth -960 is half as far and is drawn at twice the size.

FX-CAM-003: the sideways track, which is what the whole entry is for. Three planes at depths 0,
640 and 1920, each at position (960, 540). The camera's position is keyed linearly from
(760, 540) at frame 0 to (1160, 540) at frame 48; its depth is -1920 and its zoom 1920. Where
each plane's origin lands:

| frame | near x | middle x | far x |
| --- | --- | --- | --- |
| 0 | 1160 | 1110 | 1060 |
| 16 | 1026.6666666666667 | 1010 | 993.3333333333334 |
| 32 | 893.3333333333335 | 910 | 926.6666666666667 |
| 48 | 760 | 810 | 860 |

| plane | depth | pixels travelled |
| --- | --- | --- |
| near | 0 | 400 |
| middle | 640 | 300 |
| far | 1920 | 200 |

The camera travelled 400 pixels. The near plane travelled all 400 of them, the middle plane 300
and the far plane 200. A build that moves all three the same distance has a camera that pans
rather than a camera that is somewhere, and fails here rather than in the owner's eye.

FX-CAM-004: a zoom and a dolly are different moves, which is why the camera has both numbers.
Two planes, at depths 0 and 1920, both at position (960, 540); the point measured is (100, 0).
One camera keys `zoom` from 1920 to 2880 over 48 frames; the other keys the camera's own `depth`
from -1920 to -960, which is the same camera moving 960 pixels closer.

| move | frame | near x | far x | near over far |
| --- | --- | --- | --- | --- |
| zoom to 2880 | 0 | 1060 | 1010 | 2 |
| zoom to 2880 | 48 | 1110 | 1035 | 2 |
| dolly in 960 | 0 | 1060 | 1010 | 2 |
| dolly in 960 | 48 | 1160 | 1026.6666666666667 | 3 |

The last column is how far the near plane's point sits from the centre divided by how far the
far plane's does. **A zoom leaves it at 2**: both planes grow together and the parallax between
them is unchanged. **A dolly moves it to 3**: the near plane grows faster than the far one, which
is the difference a person sees and cannot get from a zoom. A build with one number for both
cannot produce both pairs of rows.

FX-CAM-005: what is drawn first. Four layers in composition order A, B, C, D, at depths 0, 1920,
1920 and -960.

| layer stack | depths | drawn first to last |
| --- | --- | --- |
| A, B, C, D | 0, 1920, 1920, -960 | B, C, A, D |

Farthest first, so B and C are drawn before A, and D is drawn last and is nearest the viewer. B
and C are at the same depth and keep the order the composition gives them. Note what this costs:
D is last in the stack and ends up in front of everything, so **depth overrides the layer stack
whenever two layers are at different depths.**

FX-CAM-006: level with the camera, and behind it. The 36 mm camera, whose own depth is -1920.

| depth | in front of the camera by | drawn at |
| --- | --- | --- |
| -2000 | -80 | not drawn |
| -1920 | 0 | not drawn |
| -1919 | 1 | 1920 |
| -960 | 960 | 2 |

The first two rows report `CAMERA_PLANE_BEHIND` and draw nothing, and neither changes the
layer's record. The third is one pixel in front of the camera and is drawn at 1920 times its
size, which is not an error and must not be clamped into one.

FX-CAM-007: a layer rides on its parent's plane. FX-PARENT-001's two layers - P with anchor
(50, 50), position (400, 300), rotation 90, and C at position (100, 0) parented to P - with P
given depth 1920 and C given no depth at all.

| layer | own depth | world depth | drawn at |
| --- | --- | --- | --- |
| P | 1920 | 1920 | 0.5 |
| C | 0 | 1920 | 0.5 |

| child point | comp x | comp y | screen x | screen y |
| --- | --- | --- | --- | --- |
| (0, 0) | 450 | 350 | 705 | 445 |
| (10, 0) | 450 | 360 | 705 | 450 |
| (0, 10) | 440 | 350 | 700 | 445 |

The comp columns are FX-PARENT-001's numbers unchanged, which is the check that this entry adds
a step to document 21 rather than editing one. C has no depth of its own and is on P's plane, so
a mouth cel parented to a head plane is on the head's plane without anybody saying so twice. A
build in which C stays at depth 0 draws it at full size in front of the head and fails here.

FX-CAM-008: the file. `Fixtures/projects/camera_project.json` holds FX-CAM-003's three planes as
`layer-near`, `layer-middle` and `layer-far`, with FX-CAM-003's keyed camera. Loading it and
saving it again must reproduce it byte for byte. `layer-near` carries no `depth` field and must
not gain one; the other two carry a depth in the same shape as every other animatable property,
a base and an empty keyframe list. Its `layer_order` is near, middle, far - deliberately not the
order the planes are drawn in - so a build that draws by the stack fails. Each plane's origin
matches FX-CAM-003 at frames 0 and 48, beside the `MEDIA_MISSING` its absent drawings already
give.

FX-CAM-009: a matte is a plane too. Layer A at depth 0 and its matte M at depth 1920, both at
position (960, 540), seen by a 36 mm camera at (760, 540).

| layer | depth | origin x with the camera at 760 |
| --- | --- | --- |
| A | 0 | 1160 |
| M | 1920 | 1060 |

The matte is projected at its own depth before its alpha is sampled, so a matte on another plane
slides against the layer it shapes by 100 pixels here, and by more as the camera moves further.
This is correct and it looks like a defect, which is why it is written down: a matte is meant to
share its layer's depth, and nothing in this contract forces it to.

FX-CAM-010: a depth that is animated, and a draw order that changes because of it. Two planes at
position (960, 540): A at depth 0, and B whose depth is keyed from -960 at frame 0 to 1920 at
frame 48.

| frame | B depth | B drawn at | drawn first to last |
| --- | --- | --- | --- |
| 0 | -960 | 2 | A, B |
| 24 | 480 | 0.8 | B, A |
| 48 | 1920 | 0.5 | B, A |

At frame 0 B is nearer than A and is drawn last, in front. By frame 24 it has passed behind A
and is drawn first. **What is in front of what is a question with a different answer on
different frames**, so a build that sorts the layers once when a project is opened fails here. A
depth is an animatable property for this reason and because After Effects keyframes Z all day;
the cost of it is this case.

FX-CAM-011: what the default lens is, in the terms After Effects uses for it.

| composition width | lens in mm on a 36 mm back | zoom in pixels | horizontal angle of view | a plane one width back, drawn at |
| --- | --- | --- | --- | --- |
| 1920 | 50 | 2666.6666666666665 | 39.597752709049864 | 0.5813953488372093 |
| 1280 | 50 | 1777.7777777777778 | 39.597752709049864 | 0.5813953488372093 |

| lens in mm | zoom in pixels | horizontal angle of view | a plane one width back, drawn at |
| --- | --- | --- | --- |
| 36 | 1920 | 53.13010235415598 | 0.5 |

The second table is the lens D-58 proposed before the After Effects question was asked, kept
here because the difference is the answer to that question. Both are honest cameras and neither
is more correct; what the 50 mm one buys is that a depth copied out of an After Effects project,
or out of a tutorial written for one, parallaxes here by the amount it does there. The angle of
view does not depend on the composition's width, which is the check that the default is a lens
and not a number of pixels that happens to suit one size of picture.

## Expression fixtures

Specified on 2026-09-16 by D-59, accepted by the owner the same day, and built by B-14b. Every case is `Fixtures/projects/expression_project.json` or that file with one property's expression replaced, as the case says: one composition, 1920 by 1080 at 24 frames a second, 49 frames long, nine layers named after what they do. Their IDs and names are `layer-spin` Spin, `layer-follow` Follow, `layer-shake` Shake, `layer-shake-twos` Shake on threes, `layer-lead` Lead, `layer-trail` Trail, `layer-loop` Loop, `layer-fade` Fade and `layer-off` Switched off. Every layer is at (960, 540) with rotation 0 and opacity 1 unless the case says otherwise.

**Every number below is produced by `tools/expression_reference.py`**, which reads and evaluates document 09's language in Python and shares nothing with the build. Tolerance 1e-9, except that the noise behind `wiggle` and `random` is specified to the operation and a build must match FX-EXPR-002, 003, 004, 013 and 014 to 1e-12. A diagnostic column is the identifier the property must report at that frame, and `none` means it must report nothing.

`python tools/expression_reference.py --check` confirms that the project file is still what the generator writes. B-14b must open that file and save it byte for byte.

FX-EXPR-001: time, and one layer reading another. Spin's rotation is `time * 90`. Follow's is `thisComp.layer("layer-spin").transform.rotation * -1`, so it turns the other way at the same speed. Neither has keys.

| frame | Spin rotation | Follow rotation |
| --- | --- | --- |
| 0 | 0 | 0 |
| 6 | 22.5 | -22.5 |
| 12 | 45 | -45 |
| 24 | 90 | -90 |
| 36 | 135 | -135 |
| 48 | 180 | -180 |

FX-EXPR-002: `wiggle(2, 30)` on Shake's position, base (960, 540). Every number here is the noise of document 09 worked to the operation, so a build that differs in the last digit has a different noise, not a rounding difference. Frame 6 is exactly halfway between frames 0 and 12, and frame 18 between 12 and 24, because at 2 wiggles a second those frames fall on the half-way point between two lattice values, where the fade is exactly one half.

| frame | Shake position |
| --- | --- |
| 0 | (939.2579552997871, 548.2812330411965) |
| 6 | (941.545853571091, 544.3187500385676) |
| 12 | (943.8337518423949, 540.3562670359387) |
| 18 | (957.0113563546388, 535.4764003540593) |
| 24 | (970.1889608668827, 530.5965336721797) |
| 48 | (966.906188209151, 529.5843168112989) |

FX-EXPR-003: `posterizeTime(8)` then `wiggle(2, 30)`, on a layer with a different ID from Shake's, so a different wiggle. At 24 frames a second the value holds for three frames and then moves.

| frame | Shake on threes position |
| --- | --- |
| 0 | (954.7511202122558, 555.917263510788) |
| 1 | (954.7511202122558, 555.917263510788) |
| 2 | (954.7511202122558, 555.917263510788) |
| 3 | (955.3280386609665, 554.9464196445025) |
| 4 | (955.3280386609665, 554.9464196445025) |
| 5 | (955.3280386609665, 554.9464196445025) |
| 6 | (957.5377451720659, 551.2279044585414) |

FX-EXPR-004: the same expression is the same number every time, and a different seed or a different property is a different number. The first row equals FX-EXPR-002 at frame 12. The third row is the same text on a different property and does not move with Shake.

| case | value at frame 12 |
| --- | --- |
| Shake's own expression, evaluated a second time | (943.8337518423949, 540.3562670359387) |
| Shake with seedRandom(5) first | (958.3765471180127, 545.6703353848442) |
| wiggle(2, 30) on Spin's rotation | 24.35692893913322 |

FX-EXPR-005: `valueAtTime`. Lead's position is keyed linearly from (0, 540) at frame 0 to (1920, 540) at frame 48. Trail's expression, which starts with a comment line, is `thisComp.layer("layer-lead").position.valueAtTime(time - 0.25)`: six frames behind. Before frame 6 it reads the held first key.

| frame | Lead position | Trail position |
| --- | --- | --- |
| 0 | (0, 540) | (0, 540) |
| 6 | (240, 540) | (0, 540) |
| 12 | (480, 540) | (240, 540) |
| 24 | (960, 540) | (720, 540) |
| 48 | (1920, 540) | (1680, 540) |

FX-EXPR-006: the four loops. Loop's rotation is keyed linearly 0 at frame 0, 90 at frame 12 and 30 at frame 24, and its expression is `loopOut(kind)`. Up to frame 24 all four are the keys. Continue carries on at the slope of the final frame, which is -5 degrees a frame.

| frame | cycle | pingpong | offset | continue |
| --- | --- | --- | --- | --- |
| 0 | 0 | 0 | 0 | 0 |
| 12 | 90 | 90 | 90 | 90 |
| 24 | 30 | 30 | 30 | 30 |
| 30 | 45 | 60 | 75 | 0 |
| 36 | 90 | 90 | 120 | -30 |
| 42 | 60 | 45 | 90 | -60 |
| 48 | 0 | 0 | 60 | -90 |

FX-EXPR-007: opacity is a percentage inside an expression. Fade's expression is `linear(time, 0, 1, 0, 100)` and the second column replaces `linear` with `ease`. The numbers are what the file-units property holds, 0 to 1.

| frame | linear, in the file | ease, in the file |
| --- | --- | --- |
| 0 | 0 | 0 |
| 6 | 0.25 | 0.15625 |
| 12 | 0.5 | 0.5 |
| 24 | 1 | 1 |
| 48 | 1 | 1 |

FX-EXPR-008: wrong types and wrong shapes. Each row is one expression put on the named property of the fixture project, evaluated at frame 12. A failed expression leaves the property at its keyed value, which is the third column. Opacity 250 is not an error: it is limited to 100, which is 1 in the file. A camera zoom of -10 is an error and is not limited.

| property | expression | value at frame 12 | diagnostic |
| --- | --- | --- | --- |
| rotation | `[1, 2]` | 0 | EXPRESSION_TYPE |
| position | `5` | (960, 540) | EXPRESSION_TYPE |
| position | `value + 5` | (960, 540) | EXPRESSION_TYPE |
| rotation | `1 / 0` | 0 | EXPRESSION_TYPE |
| rotation | `"ninety"` | 0 | EXPRESSION_TYPE |
| rotation | `value[0]` | 0 | EXPRESSION_TYPE |
| position | `[1, 2] * [3, 4]` | (960, 540) | EXPRESSION_TYPE |
| rotation | `posterizeTime(8)` | 0 | EXPRESSION_TYPE |
| opacity | `250` | 1 | none |
| zoom | `-10` | 1920 | EXPRESSION_TYPE |
| rotation | `wiggle(2, 30, 40)` | 0 | EXPRESSION_TYPE |

FX-EXPR-009: references survive a rename and fail visibly on a delete. Follow reads Spin by ID.

| case | Follow rotation at frame 12 | diagnostic |
| --- | --- | --- |
| as written | -45 | none |
| Spin renamed | -45 | none |
| Spin deleted | 0 | EXPRESSION_REFERENCE_MISSING |

FX-EXPR-010: cycles. In the first two rows Spin's expression is replaced with `thisComp.layer("layer-follow").rotation`, while Follow still reads Spin. The third is `thisLayer.rotation + 1` on Spin, and the fourth `thisProperty.valueAtTime(time - 1) + value`, which reads Spin's keys and is not a cycle. After Effects would give the third row a number; this language gives a diagnostic.

| case | property | value at frame 12 | diagnostic |
| --- | --- | --- | --- |
| Spin follows Follow, which follows Spin | Spin rotation | 0 | EXPRESSION_CYCLE |
| the same | Follow rotation | 0 | EXPRESSION_CYCLE |
| Spin reads its own finished value | Spin rotation | 0 | EXPRESSION_CYCLE |
| Spin reads its own keys through thisProperty | Spin rotation | 0 | none |

FX-EXPR-011: runaway work stops, and stops the same way every time. In the first two rows Spin is `thisComp.layer("layer-follow").rotation.valueAtTime(time - 1/24) + 1` and Follow is `thisComp.layer("layer-spin").rotation.valueAtTime(time) + 1`, so each reads the other at an ever earlier frame and neither ever meets itself at the same frame. The depth limit of 16 stops it. The second row starts at frame 3 to show that passing frame 0 does not stop it, because the held first key goes on for ever. The third row is too long to read.

| case | value at frame 40 | diagnostic |
| --- | --- | --- |
| Spin and Follow each read the other a frame earlier, forever | 0 | EXPRESSION_TIMEOUT |
| the same at frame 3, which runs out of frames before it runs out of depth | 0 | EXPRESSION_TIMEOUT |
| `1 + 1 + ...`, 4401 bytes | 0 | EXPRESSION_SYNTAX |

FX-EXPR-012: there is no way out. Each is Spin's expression. None of them is evaluated: each is refused when the text is read, or on the first word the language does not have.

| expression | value at frame 12 | diagnostic |
| --- | --- | --- |
| `require("fs")` | 0 | EXPRESSION_SYNTAX |
| `eval("1")` | 0 | EXPRESSION_SYNTAX |
| `system.callSystem("calc")` | 0 | EXPRESSION_SYNTAX |
| `$.sleep(100000)` | 0 | EXPRESSION_SYNTAX |
| `fetch("http://example.com")` | 0 | EXPRESSION_SYNTAX |
| `while (1) {}` | 0 | EXPRESSION_SYNTAX |
| `function f() { return 1 }` | 0 | EXPRESSION_SYNTAX |
| `thisComp.layer("layer-spin").sourceText` | 0 | EXPRESSION_SYNTAX |

FX-EXPR-013: `random`. Each column is Spin's whole expression. The first three are the same call and differ only in range. The fourth is timeless and gives the same number on every frame.

| frame | random() | random(10) | random(5, 10) | seedRandom(3, 1) then random() |
| --- | --- | --- | --- | --- |
| 0 | 0.577284866174014 | 5.77284866174014 | 7.88642433087007 | 0.734884388919881 |
| 1 | 0.15815400784099987 | 1.5815400784099987 | 5.790770039204999 | 0.734884388919881 |
| 12 | 0.6487858604973431 | 6.487858604973431 | 8.243929302486716 | 0.734884388919881 |
| 48 | 0.1420645882663093 | 1.420645882663093 | 5.710322941331547 | 0.734884388919881 |

FX-EXPR-014: the camera. The fixture's camera position is `wiggle(1, 10)` on base (960, 540), with depth -1920 and zoom 1920. The third column is Spin with `thisComp.activeCamera.zoom / 100`.

| frame | camera position | Spin rotation from the camera's zoom |
| --- | --- | --- |
| 0 | (969.4336249008326, 547.6315259802685) | 19.2 |
| 12 | (966.7653177123977, 548.3560595722439) | 19.2 |
| 24 | (964.097010523963, 549.0805931642192) | 19.2 |
| 48 | (967.4231219378125, 530.3694238295922) | 19.2 |

FX-EXPR-015: local names and Math. Shake's expression replaced with `bob = Math.sin(time * Math.PI * 2) * 20` and, on the next line, `value + [0, bob]`: one bob a second, 20 pixels.

| frame | Shake position |
| --- | --- |
| 0 | (960, 540) |
| 6 | (960, 560) |
| 12 | (960, 540) |
| 18 | (960, 520) |
| 24 | (960, 540) |

FX-EXPR-016: a switched-off expression, and opacity read by another expression. Switched off's opacity is 1 in the file and carries the expression `50` with `enabled` false. The second and third rows are Spin with `thisComp.layer("layer-off").opacity` and `thisComp.layer("layer-fade").opacity`. Both are percentages, whether or not the opacity read has a working expression of its own.

| property | value at frame 12 | diagnostic |
| --- | --- | --- |
| Switched off opacity | 1 | none |
| Spin rotation reading Switched off's opacity | 100 | none |
| Spin rotation reading Fade's opacity | 50 | none |

## Packaging fixtures

Specified on 2026-09-16 by D-61, accepted by the owner the same day, for B-15b. Every case starts from `Fixtures/packaging/source/shot.json`: two compositions, 64 by 36, three frames, eight assets and seven small drawings, laid out to catch each of D-61's rules. **Every expected value is produced by `tools/package_reference.py`**, which applies D-61 in Python with Python's own SHA-256 and shares nothing with the build. The exact manifest is `Fixtures/packaging/expected_manifest.json`, and the places and check answers are `Fixtures/packaging/expected_package.json`. A build must match both exactly, byte for byte for the manifest.

FX-PACK-001: collecting the shot into an empty folder. Each asset's folder and files, and what happened to each file:

| Asset | Folder | Files, with the frame numbers they stand for | Status |
|---|---|---|---|
| `asset-cel` | `asset-cel` | `cel_0001.png` [1], `cel_0002.png` [2, 3] | copied, copied |
| `asset-bg` | `asset-bg` | `背景.png` | copied |
| `asset-mix` | `asset-mix` | `x.png` [1], `x-2.png` [2], `X-3.png` [3] | copied, copied, copied |
| `asset-licensed` | `asset-licensed` | `sheet.png` | excluded |
| `asset-gone` | `asset-gone` | `gone_0001.png` [1], `cel_0001.png` [2] | missing, copied |
| `shot 1/é` | `shot 1_é` | `背景.png` | copied |
| `shot 1?é` | `shot 1_é-2` | `cel_0001.png` | copied |
| `con` | `_con` | `cel_0002.png` | copied |

Frame 3 of `asset-cel` reuses frame 2's drawing, which is copied once. `asset-bg` is used by a layer in each composition and listed with both. `shot 1?é` is used by nothing and is still collected. Ten files are copied, and no file is written for the excluded or the missing drawing.

FX-PACK-002: the packaged project. It is the source project with each path replaced by its place above (the `places` of the expected file), and with nothing else changed. Opened from the package, it reports `MEDIA_MISSING` for exactly two drawings: the licensed sheet and `gone_0001.png`.

FX-PACK-003: moved. The whole package folder is moved to a folder with a different name and depth, with a space and Japanese in it, and opened there. The answer is the same as FX-PACK-002. Every frame of `comp-alt`, which uses neither the licensed sheet nor the missing drawing, is the same to the byte as the source's. So is every frame of `comp-main` once the licensed sheet is put in its place, except for the missing drawing, which the source cannot draw either: that layer shows drawing 2, which is present in both.

FX-PACK-004: checking, in six situations. Every file not named is `ok`.

| Situation | Files not `ok` |
|---|---|
| as collected | asset-licensed/sheet.png: excluded; asset-gone/gone_0001.png: missing |
| one byte of cel_0001.png changed | asset-cel/cel_0001.png: changed; asset-licensed/sheet.png: excluded; asset-gone/gone_0001.png: missing |
| cel_0001.png deleted | asset-cel/cel_0001.png: missing; asset-licensed/sheet.png: excluded; asset-gone/gone_0001.png: missing |
| the licensed sheet supplied by the recipient | asset-gone/gone_0001.png: missing |
| a different file put where the licensed sheet goes | asset-licensed/sheet.png: changed; asset-gone/gone_0001.png: missing |
| a file put where the missing drawing goes | asset-licensed/sheet.png: excluded; asset-gone/gone_0001.png: unverified |

FX-PACK-005: refusals. A folder holding any file is refused with `PACKAGE_DESTINATION_NOT_EMPTY` and is unchanged afterwards; so is an empty folder whose `.partial` sibling exists. A write that fails partway, made to fail by the test, gives `PACKAGE_WRITE_FAILED` and leaves neither the folder nor its `.partial` sibling behind. Checking a project with no manifest beside it gives `PACKAGE_MANIFEST_INVALID`.

FX-PACK-006: nothing in the window changes. After collecting, the open project, its file on disk, its undo history and whether it has unsaved changes are all as they were.

The build's SHA-256 must also give FIPS 180-4's published digests for the empty message, `abc`, the 448-bit message `abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq`, and a million `a`s.

## EXR fixtures

Specified on 2026-09-17 by D-62, accepted by the owner the same day, for B-16b. Every file is in `Fixtures/exr/` and was written by OpenEXR's own library, version 3.4.15. **Every expected value is produced by `tools/exr_reference.py`**, which reads each file back with that library and applies D-62 in Python, sharing nothing with the build. `Fixtures/exr/expected_exr.json` holds, for each file, the answer and, for a drawn file, its size, every pixel as it must reach the working buffer and every reason `MEDIA_EXR_ADJUSTED` must give. **A build must match every pixel exactly**; no tolerance applies (D-62 records the measurement behind that). A file with no reason listed below must be drawn with no report.

Most files hold the same test picture: 8 by 6, premultiplied, with colour below 0 and above 1 (up to 1000), alpha of 1, 0.75, 0.5, 0.25, 0.125 and 0, and one pixel that has colour but no alpha at all, which must come in unchanged.

FX-EXR-001: compression. The test picture in each of the ten compressions a build must read - none, RLE, ZIPS, ZIP, PIZ, PXR24, B44, B44A, DWAA and DWAB - once with half samples and once with float, twenty files in `compression/`. All are drawn, with no report. The lossy ones (PXR24 for float, B44, DWA) must give what OpenEXR's library gives, which is the expected value, not the picture that was written.

FX-EXR-002: sample types.

| File | Answer | Reported |
|---|---|---|
| `types/uint.exr`: whole-number samples, alpha 0, 1 and 2 | drawn, the numbers as floats | `alpha_clamped` 6 |
| `types/mixed.exr`: half and float channels in one file | drawn | none |

FX-EXR-003: channels, matched with capitals.

| File | Answer | Reported |
|---|---|---|
| `channels/rgb.exr`: no alpha | drawn, alpha 1 | none |
| `channels/y.exr`: luminance only | drawn, R = G = B = Y, alpha 1 | none |
| `channels/ya.exr`: luminance and alpha | drawn, R = G = B = Y | none |
| `channels/r_only.exr`: red only | drawn, G and B 0, alpha 1 | none |
| `channels/extra.exr`: RGBA with `Z`, `diffuse.G` and `diffuse.R` | drawn from RGBA | `channels_ignored` Z, diffuse.G, diffuse.R |
| `channels/rgb_and_y.exr`: RGBA and `Y` | drawn from RGBA | `channels_ignored` Y |
| `channels/lowercase.exr`: `r`, `g`, `b`, `a` | refused, `MEDIA_UNSUPPORTED_FORMAT`, `no_colour_channels` | |

FX-EXR-004: windows and layout. Every one is drawn 8 by 6.

| File | What it holds | Reported |
|---|---|---|
| `windows/inset.exr` | data window (2,1)-(5,3) inside display window (0,0)-(7,5): the rest is transparent black | none |
| `windows/overscan.exr` | data window two pixels wider and one taller on every side, filled with 9 there | `outside_display_window` 48 |
| `windows/offset.exr` | both windows at (10,20)-(17,25): drawn as if at the origin | none |
| `windows/disjoint.exr` | data window (20,20)-(21,21), wholly outside: all transparent black | `outside_display_window` 4 |
| `windows/decreasing_y.exr` | lines stored bottom to top: drawn the right way up | none |
| `windows/tiled.exr` | 3 by 3 tiles, one level, PIZ | none |
| `windows/aspect.exr` | pixel aspect ratio 2: drawn with square pixels | `pixel_aspect` 2.0 |

FX-EXR-005: values. `values/specials.exr`, float: a NaN in red, +infinity in green and -infinity in blue on three pixels, 1e30 in red and -70000 in green on two others, and alpha of 1.5, -0.25 and NaN on three more. Drawn with the four non-finite samples as 0, the two alphas clamped to 1 and 0 (the NaN alpha became 0 first and is not counted again), and 1e30 and -70000 kept. Reported: `non_finite` 4, `alpha_clamped` 2.

FX-EXR-006: primaries. `colour/rec709.exr` states Rec. 709 and is drawn with no report. `colour/acescg.exr` states ACEScg's primaries and white; it is drawn with the same numbers, and `primaries` is reported with the eight stated values.

FX-EXR-007: refused. Each keeps its asset and draws nothing.

| File | Identifier | Reason |
|---|---|---|
| `refused/multipart.exr`: two parts | `MEDIA_UNSUPPORTED_FORMAT` | `multipart` |
| `refused/deep.exr`: deep data | `MEDIA_UNSUPPORTED_FORMAT` | `deep` |
| `refused/htj2k.exr`: HTJ2K compression | `MEDIA_UNSUPPORTED_FORMAT` | `htj2k` |
| `refused/layers_only.exr`: colour only as `beauty.R` and so on | `MEDIA_UNSUPPORTED_FORMAT` | `no_colour_channels` |
| `refused/luminance_chroma.exr`: `Y`, `RY`, `BY` | `MEDIA_UNSUPPORTED_FORMAT` | `luminance_chroma` |
| `refused/truncated.exr`: the first half of a good file | `MEDIA_DECODE_FAILED` | `unreadable` |
| `refused/empty.exr`: no bytes | `MEDIA_DECODE_FAILED` | `unreadable` |
| `refused/not_exr.exr`: a line of text | `MEDIA_DECODE_FAILED` | `unreadable` |

`refused/htj2k.exr` is small enough that OpenEXR stored its pixels uncompressed, and the `exr` crate reads it; it must be refused anyway, from its header. No file tests `subsampled`: OpenEXR's Python library cannot write one, so that refusal is specified and not fixture-tested.

FX-EXR-008: a sequence. `sequence/render_0001.exr`, `_0002`, `_0003` and `_0005`, each a flat red of 0.1 times its number, imported as one sequence of pattern `render_%04d.exr`. Drawings 1, 2, 3 and 5 are there; drawing 4 is `MEDIA_SEQUENCE_GAP`, exactly as for PNG.

FX-EXR-009: export, the plain picture. A composition 8 by 6, one frame, numbered 0, holding `compression/none_float.exr` at identity over transparent black, so the working buffer is the drawing (D-58). Exported as `none_float_as_float` (float, 24 frames a second) and `none_float_as_half` (half, 24). `tools/exr_reference.py check <folder>` reads both with OpenEXR's library and checks one part, scanlines, ZIP, channels A, B, G, R, the sample type, both windows (0,0)-(7,5), increasing Y, pixel aspect 1, screen window (0,0) and 1, Rec. 709 primaries, the frame rate as a fraction, no other attribute but `chunkCount`, and every sample against `export` in the expected file. Float is the drawing exactly; half is each value rounded to the nearest half.

FX-EXR-010: export, the extreme picture. The same with `values/specials.exr`, at 24000/1001 frames a second, as `specials_as_float` and `specials_as_half`. The float file holds the working buffer, 1e30 included. In the half file, 1e30 is 65504 and -70000 is -65504.

The check prints a table with one row per item, 15 for each of the four files, 60 in all, and must say 60 of 60.

## Adjustment layer fixtures

D-66, accepted on 2026-09-17; B-17b's `verification/B-17b_adjust_table.md` walks every case. Every case is a composition 6 by 2 at 24 fps, three frames long, drawn from the projects and drawings in `Fixtures/adjust/`: `bg` is opaque, red on the top row and sRGB grey 128 on the bottom; `dot` is one opaque white pixel at (2, 0); `half` is green at alpha 128 everywhere; `matte` is white on the left three columns and empty on the right three. Layers are listed bottom first. Every layer's transform is the identity unless the case moves it. Each cell is R G B A of the finished frame, linear and premultiplied.

**Every number below is produced by `tools/adjust_reference.py`**, which renders each pixel from document 21 and blurs with the two-dimensional kernel summed directly; the build runs two one-dimensional passes. The same numbers are in `Fixtures/adjust/expected_adjust.json`. Tolerance 1e-6, because the build works in 32-bit floats.

FX-ADJ-001: One stop brighter on everything below.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 0, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |

FX-ADJ-002: The same at half opacity: half way between.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1.5 0 0 1 | 1.5 0 0 1 | 1.5 0 0 1 | 1.5 0 0 1 | 1.5 0 0 1 | 1.5 0 0 1 |
| 0, 1 | 0.32379075 0.32379075 0.32379075 1 | 0.32379075 0.32379075 0.32379075 1 | 0.32379075 0.32379075 0.32379075 1 | 0.32379075 0.32379075 0.32379075 1 | 0.32379075 0.32379075 0.32379075 1 | 0.32379075 0.32379075 0.32379075 1 |

FX-ADJ-003: A layer above the adjustment layer is not adjusted.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 |
| 0, 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 |

FX-ADJ-004: A mask on the adjustment layer: only the left three columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-ADJ-005: Moved four pixels right: only the right two columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |

FX-ADJ-006: A matte-only layer shapes the adjustment: only the left three columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-ADJ-007: A blur spreads one pixel over the frame and is cut off at its edge.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.0215509428 0.0215509428 0.0215509428 0.0215509428 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.159241126 0.159241126 0.159241126 0.159241126 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.0215509428 0.0215509428 0.0215509428 0.0215509428 | 0.00176900911 0.00176900911 0.00176900911 0.00176900911 |
| 0, 1 | 0.0130713076 0.0130713076 0.0130713076 0.0130713076 | 0.0585815363 0.0585815363 0.0585815363 0.0585815363 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.0585815363 0.0585815363 0.0585815363 0.0585815363 | 0.0130713076 0.0130713076 0.0130713076 0.0130713076 | 0.00107295826 0.00107295826 0.00107295826 0.00107295826 |

FX-ADJ-008: A full tint on a half-covered picture keeps its coverage.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 |
| 0, 1 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 | 0 0 0.501960784 0.501960784 |

FX-ADJ-009: Two adjustment layers, lower one first.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0.5 1 | 1 0 0.5 1 | 1 0 0.5 1 | 1 0 0.5 1 | 1 0 0.5 1 | 1 0 0.5 1 |
| 0, 1 | 0.2158605 0.2158605 0.7158605 1 | 0.2158605 0.2158605 0.7158605 1 | 0.2158605 0.2158605 0.7158605 1 | 0.2158605 0.2158605 0.7158605 1 | 0.2158605 0.2158605 0.7158605 1 | 0.2158605 0.2158605 0.7158605 1 |

FX-ADJ-010: The same two, the other way up.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 1 1 | 1 0 1 1 | 1 0 1 1 | 1 0 1 1 | 1 0 1 1 | 1 0 1 1 |
| 0, 1 | 0.2158605 0.2158605 1.2158605 1 | 0.2158605 0.2158605 1.2158605 1 | 0.2158605 0.2158605 1.2158605 1 | 0.2158605 0.2158605 1.2158605 1 | 0.2158605 0.2158605 1.2158605 1 | 0.2158605 0.2158605 1.2158605 1 |

FX-ADJ-011: No effects switched on: the frame is the frame without the layer.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-ADJ-012: Behind the picture in depth: drawn first, onto nothing, so nothing changes.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-ADJ-013: Only between its in and out frames (frame 1 only).

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 1, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 1, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |
| 2, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 2, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

## Precomposition fixtures

D-67, accepted on 2026-09-18; B-18b walks every case in `verification/B-18b_precomp_table.md`, 70 of 70. Every case is a project of compositions 6 by 2 at 24 fps, three frames long, drawn from the projects and drawings in `Fixtures/precomp/`; the frames shown are the composition Main's, which holds a layer of Inner, itself a composition of the project. The drawings are the adjustment fixtures' - `bg`, `dot`, `half`, `matte` - and `small`, opaque blue 2 by 2, for the case whose Inner is 2 by 2. Layers are listed bottom first. Every layer's transform is the identity unless the case moves it, and a composition layer's anchor is the centre of Inner and its position the centre of Main. Each cell is R G B A of the finished frame, linear and premultiplied.

**Every number below is produced by `tools/precomp_reference.py`**, which renders each pixel from document 21 and nests by plain recursion, reusing `tools/adjust_reference.py` for the drawings and effects. The same numbers are in `Fixtures/precomp/expected_precomp.json`. Tolerance 1e-6. FX-PRE-014 also expects the warning `COMPOSITION_REFERENCE_MISSING`, once; FX-PRE-015 has no frames and expects the file to be refused with `COMPOSITION_CYCLE`.

FX-PRE-001: A composition layer shows the inner composition's frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-PRE-002: A blur on the composition layer blurs the inner picture as one, not each drawing on its own.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.315693136 0.0580983763 0.0580983763 0.44845613 | 0.424923201 0.145777498 0.145777498 0.603622291 | 0.449295723 0.211255578 0.211255578 0.638244542 | 0.449295723 0.148599077 0.148599077 0.638244542 | 0.424923201 0.0707438161 0.0707438161 0.603622291 | 0.315693136 0.0383164426 0.0383164426 0.44845613 |
| 0, 1 | 0.229566958 0.0733278386 0.0733278386 0.44845613 | 0.3089973 0.139686873 0.139686873 0.603622291 | 0.326720605 0.182341959 0.182341959 0.638244542 | 0.326720605 0.14433887 0.14433887 0.638244542 | 0.3089973 0.094176644 0.094176644 0.603622291 | 0.229566958 0.0613294892 0.0613294892 0.44845613 |

FX-PRE-003: Half opacity on the composition layer: the inner picture at half.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.5 0 0 0.5 |
| 0, 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 |

FX-PRE-004: Moved two pixels right: the inner picture moves as one drawing.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 1 1 1 | 1 1 1 1 | 1 1 1 1 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 1 1 1 1 | 1 1 1 1 | 1 1 1 1 | 0 0 0 0 |

FX-PRE-005: A two-by-two inner composition lands centred, at its own size.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0 0 1 1 | 0 0 1 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 1 1 | 0 0 1 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-PRE-006: The inner composition's own time: a source offset of one frame, and nothing past its end.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 1, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-PRE-007: Only between the composition layer's own in and out frames (frame 1 only).

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 2, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-PRE-008: Two levels deep: each level's own opacity and effects.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 0.5 | 1 0 0 0.5 | 1 0 0 0.5 | 1 0 0 0.5 | 1 0 0 0.5 | 1 0 0 0.5 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 0.5 | 0.2158605 0.2158605 0.2158605 0.5 | 0.2158605 0.2158605 0.2158605 0.5 | 0.2158605 0.2158605 0.2158605 0.5 | 0.2158605 0.2158605 0.2158605 0.5 | 0.2158605 0.2158605 0.2158605 0.5 |

FX-PRE-009: An adjustment layer inside stays inside: the layer above the composition layer is not adjusted.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 | 0.996078431 0.501960784 0 1 |
| 0, 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 | 0.215013988 0.716974773 0.215013988 1 |

FX-PRE-010: An adjustment layer above a composition layer adjusts it like any layer.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 0, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |

FX-PRE-011: A mask on the composition layer: only the left three columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-PRE-012: A matte-only layer shapes the composition layer: only the left three columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-PRE-013: The same composition twice, the second moved three right: two layers.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0.501960784 0 0.501960784 | 0 0.501960784 0 0.501960784 | 0 0.501960784 0 0.501960784 | 0 0.75195694 0 0.75195694 | 0 0.75195694 0 0.75195694 | 0 0.75195694 0 0.75195694 |
| 0, 1 | 0 0.501960784 0 0.501960784 | 0 0.501960784 0 0.501960784 | 0 0.501960784 0 0.501960784 | 0 0.75195694 0 0.75195694 | 0 0.75195694 0 0.75195694 | 0 0.75195694 0 0.75195694 |

FX-PRE-014: A composition that does not exist: drawn as nothing, with COMPOSITION_REFERENCE_MISSING.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-PRE-015: Main holds a layer of Inner and Inner a layer of Main: the file is refused.

## Solid layer fixtures

**D-74, proposed and accepted on 2026-09-19; B-23b answers to these, `verification/B-23b_solid_table.md`.** Every case is a composition 6 by 2 at 24 fps, three frames long, from the projects in `Fixtures/solid/`. The one drawing, `bg`, is the adjustment fixtures' own: opaque, red on the top row and sRGB grey 128 on the bottom. The solid's colour is linear `0.2 0.5 0.8` unless a case says otherwise, and a solid is the size of the frame and centred unless a case says otherwise. Layers are listed bottom first. Each cell is R G B A of the finished frame, linear and premultiplied.

**Every number below is produced by `tools/solid_reference.py`**, which renders each pixel from document 21 and moves only by whole pixels, so nothing is resampled. The same numbers are in `Fixtures/solid/expected_solid.json`. Tolerance 1e-6. FX-SOL-020 to 029 have no frames: each is FX-SOL-001's file with one change, and a build must refuse it whole.

FX-SOL-001: A solid the size of the frame, on nothing: every pixel its colour, opaque.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |

FX-SOL-002: The same at half opacity over the red and grey drawing.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.6 0.25 0.4 1 | 0.6 0.25 0.4 1 | 0.6 0.25 0.4 1 | 0.6 0.25 0.4 1 | 0.6 0.25 0.4 1 | 0.6 0.25 0.4 1 |
| 0, 1 | 0.20793025 0.35793025 0.50793025 1 | 0.20793025 0.35793025 0.50793025 1 | 0.20793025 0.35793025 0.50793025 1 | 0.20793025 0.35793025 0.50793025 1 | 0.20793025 0.35793025 0.50793025 1 | 0.20793025 0.35793025 0.50793025 1 |

FX-SOL-003: Two pixels square, centred at (3, 1): only columns 2 and 3 are its colour.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-SOL-004: A mask keeping its left three columns.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-SOL-005: One stop of exposure on the solid doubles its colour, past 1 where it goes.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 |
| 0, 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 | 0.4 1 1.6 1 |

FX-SOL-006: The small solid as a matte-only layer for the drawing: the drawing shows in columns 2 and 3 only.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 |

FX-SOL-007: Multiplied onto the drawing: each colour times the solid's.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0 0 1 | 0.2 0 0 1 | 0.2 0 0 1 | 0.2 0 0 1 | 0.2 0 0 1 | 0.2 0 0 1 |
| 0, 1 | 0.0431721 0.10793025 0.1726884 1 | 0.0431721 0.10793025 0.1726884 1 | 0.0431721 0.10793025 0.1726884 1 | 0.0431721 0.10793025 0.1726884 1 | 0.0431721 0.10793025 0.1726884 1 | 0.0431721 0.10793025 0.1726884 1 |

FX-SOL-008: Only between its in and out frames (frame 1 only).

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 1, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |
| 1, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |
| 2, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 2, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SOL-001's file with one change:

- FX-SOL-020: A solid that names an asset.
- FX-SOL-021: A solid with exposures.
- FX-SOL-022: A solid with a source offset.
- FX-SOL-023: A solid with no solid record.
- FX-SOL-024: A colour of two numbers.
- FX-SOL-025: A colour above 1.
- FX-SOL-026: A colour below 0.
- FX-SOL-027: A width of 0.
- FX-SOL-028: A height that is not a whole number.
- FX-SOL-029: A raster layer carrying a solid record.

## Null layer fixtures

**D-82, accepted on 2026-09-23. B-26b answers to these, `verification/B-26b_null_table.md`.** Every case is a composition 6 by 2 at 24 fps, three frames long, from the projects in `Fixtures/null/`. The one drawing, `bg`, is the solid fixtures' own: opaque, red on the top row and sRGB grey 128 on the bottom, centred. A null's anchor is (50, 50), the middle of its 100 by 100 outline, and it stands at (52, 50) unless a case says otherwise, so a layer parented to it is carried two pixels right. Layers are listed bottom first. Each cell is R G B A of the finished frame, linear and premultiplied.

**Every number below is produced by `tools/null_reference.py`**, which carries the drawing's corner through the parent chain with `tools/parent_reference.py`'s four steps and lays it down by whole pixels, so nothing is resampled. The same numbers are in `Fixtures/null/expected_null.json`. Tolerance 1e-6. FX-NULL-020 to 029 have no frames: each is FX-NULL-002's file with one change, and a build must refuse it whole.

FX-NULL-001: A null on its own, switched on: every pixel transparent.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-NULL-002: A null switched on above the red and grey drawing: the drawing, untouched.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-NULL-003: The drawing parented to a null that stands two pixels right: the drawing moves two pixels right, and columns 0 and 1 are empty.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-NULL-004: The same with the null switched off: the same frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-NULL-005: The same with the null alive on frame 1 only: every frame is moved.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 1, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 1, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 2, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 2, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-NULL-006: The same with the null at a tenth opacity: the drawing stays opaque, as opacity does not pass to a child.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-NULL-007: A null parented to a null, each a pixel right: the drawing moves two, as in FX-NULL-003.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-NULL-002's file with one change:

- FX-NULL-020: A null that names an asset.
- FX-NULL-021: A null with exposures.
- FX-NULL-022: A null with a source offset.
- FX-NULL-023: A null carrying a solid record.
- FX-NULL-024: A null carrying shapes.
- FX-NULL-025: A null with a mask.
- FX-NULL-026: A null with an effect.
- FX-NULL-027: A null with a matte of its own.
- FX-NULL-028: A null with a blend mode other than normal.
- FX-NULL-029: A drawing whose matte is a null.

## Pick whip fixtures

**D-83, accepted on 2026-09-23. B-27b answers to these, `verification/B-27b_pickwhip_table.md`.** Each case is one or two drops of the value whip on `Fixtures/pickwhip/pickwhip_project.json`, which is D-59's expression project (`Fixtures/projects/expression_project.json`) with two layers more: **Null 1** (`null-1`), a null whose anchor, position, scale, rotation and opacity are each keyed from frame 0 to frame 48, and **Target** (`layer-target`), a drawing standing still at the centre. The camera's depth is keyed from -1920 to -960 over the same frames; its position still has D-59's wiggle.

**Every number below is produced by `tools/pickwhip_reference.py`**, which writes each drop's text by D-83's rule and evaluates it with `tools/expression_reference.py`, D-59's second implementation. The same text and numbers are in `Fixtures/pickwhip/expected_pickwhip.json`. Tolerance 1e-6. "source" is the property the drop reads and "linked" the property that now reads it, both in the file's own units, so an opacity reads 0 to 1 here although the expression sees 0 to 100. A build passes a case when it writes the text exactly and every linked value matches.

FX-WHIP-001 to 012, one drop each, in words:

- FX-WHIP-001: Target's Position whipped to Null 1's Position: two numbers onto two.
- FX-WHIP-002: Target's Rotation whipped to Null 1's Rotation: one number onto one.
- FX-WHIP-003: Target's Rotation whipped to Null 1's Position: two numbers onto one takes the first, x.
- FX-WHIP-004: Target's Scale whipped to Null 1's Rotation: one number onto two uses it for both.
- FX-WHIP-005: Target's Opacity whipped to Null 1's Opacity: the same percentage, though the file keeps 0 to 1 and an expression 0 to 100.
- FX-WHIP-006: Target's Position whipped to Null 1's Anchor Point: written anchorPoint, as After Effects writes it.
- FX-WHIP-007: Target's Scale whipped to Null 1's Scale, which is keyed on both numbers.
- FX-WHIP-008: Target's Position whipped to the camera's Position, which already has a wiggle: the link reads the wiggled value, not the keys.
- FX-WHIP-009: The camera's Depth whipped to Null 1's Rotation: a whip can start on the camera.
- FX-WHIP-010: Null 1's Position whipped to Trail's Position: a whip can start on a null, and reads a layer that is itself an expression.
- FX-WHIP-011: Shake's Position, which already has a wiggle, whipped to Null 1's Position: the drop replaces the old text whole.
- FX-WHIP-012: Target's Position whipped to the camera's Depth, which is keyed: one number onto two, from the camera.

FX-WHIP-013 is two drops, and shows a loop is written and then diagnosed rather than refused: Target's Position whipped to Null 1's, then Null 1's back to Target's: the second drop is written, both ask for each other, and each falls back to its keys with EXPRESSION_CYCLE, as any expression loop does under D-59.

| case | drop | text written | frame | source | linked | error |
| --- | --- | --- | --- | --- | --- | --- |
| FX-WHIP-001 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 0 | (960, 540) | (960, 540) | none |
| FX-WHIP-001 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 12 | (1080, 480) | (1080, 480) | none |
| FX-WHIP-001 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 24 | (1200, 420) | (1200, 420) | none |
| FX-WHIP-001 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 36 | (1320, 360) | (1320, 360) | none |
| FX-WHIP-001 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 48 | (1440, 300) | (1440, 300) | none |
| FX-WHIP-002 | layer-target rotation to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 0 | 0 | 0 | none |
| FX-WHIP-002 | layer-target rotation to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 12 | 45 | 45 | none |
| FX-WHIP-002 | layer-target rotation to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 24 | 90 | 90 | none |
| FX-WHIP-002 | layer-target rotation to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 36 | 135 | 135 | none |
| FX-WHIP-002 | layer-target rotation to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 48 | 180 | 180 | none |
| FX-WHIP-003 | layer-target rotation to null-1 position | `thisComp.layer("null-1").transform.position[0]` | 0 | (960, 540) | 960 | none |
| FX-WHIP-003 | layer-target rotation to null-1 position | `thisComp.layer("null-1").transform.position[0]` | 12 | (1080, 480) | 1080 | none |
| FX-WHIP-003 | layer-target rotation to null-1 position | `thisComp.layer("null-1").transform.position[0]` | 24 | (1200, 420) | 1200 | none |
| FX-WHIP-003 | layer-target rotation to null-1 position | `thisComp.layer("null-1").transform.position[0]` | 36 | (1320, 360) | 1320 | none |
| FX-WHIP-003 | layer-target rotation to null-1 position | `thisComp.layer("null-1").transform.position[0]` | 48 | (1440, 300) | 1440 | none |
| FX-WHIP-004 | layer-target scale to null-1 rotation | `temp = thisComp.layer("null-1").transform.rotation; [temp, temp]` | 0 | 0 | (0, 0) | none |
| FX-WHIP-004 | layer-target scale to null-1 rotation | `temp = thisComp.layer("null-1").transform.rotation; [temp, temp]` | 12 | 45 | (45, 45) | none |
| FX-WHIP-004 | layer-target scale to null-1 rotation | `temp = thisComp.layer("null-1").transform.rotation; [temp, temp]` | 24 | 90 | (90, 90) | none |
| FX-WHIP-004 | layer-target scale to null-1 rotation | `temp = thisComp.layer("null-1").transform.rotation; [temp, temp]` | 36 | 135 | (135, 135) | none |
| FX-WHIP-004 | layer-target scale to null-1 rotation | `temp = thisComp.layer("null-1").transform.rotation; [temp, temp]` | 48 | 180 | (180, 180) | none |
| FX-WHIP-005 | layer-target opacity to null-1 opacity | `thisComp.layer("null-1").transform.opacity` | 0 | 1 | 1 | none |
| FX-WHIP-005 | layer-target opacity to null-1 opacity | `thisComp.layer("null-1").transform.opacity` | 12 | 0.8125 | 0.8125 | none |
| FX-WHIP-005 | layer-target opacity to null-1 opacity | `thisComp.layer("null-1").transform.opacity` | 24 | 0.625 | 0.625 | none |
| FX-WHIP-005 | layer-target opacity to null-1 opacity | `thisComp.layer("null-1").transform.opacity` | 36 | 0.4375 | 0.4375 | none |
| FX-WHIP-005 | layer-target opacity to null-1 opacity | `thisComp.layer("null-1").transform.opacity` | 48 | 0.25 | 0.25 | none |
| FX-WHIP-006 | layer-target position to null-1 anchor | `thisComp.layer("null-1").transform.anchorPoint` | 0 | (50, 50) | (50, 50) | none |
| FX-WHIP-006 | layer-target position to null-1 anchor | `thisComp.layer("null-1").transform.anchorPoint` | 12 | (37.5, 62.5) | (37.5, 62.5) | none |
| FX-WHIP-006 | layer-target position to null-1 anchor | `thisComp.layer("null-1").transform.anchorPoint` | 24 | (25, 75) | (25, 75) | none |
| FX-WHIP-006 | layer-target position to null-1 anchor | `thisComp.layer("null-1").transform.anchorPoint` | 36 | (12.5, 87.5) | (12.5, 87.5) | none |
| FX-WHIP-006 | layer-target position to null-1 anchor | `thisComp.layer("null-1").transform.anchorPoint` | 48 | (0, 100) | (0, 100) | none |
| FX-WHIP-007 | layer-target scale to null-1 scale | `thisComp.layer("null-1").transform.scale` | 0 | (100, 100) | (100, 100) | none |
| FX-WHIP-007 | layer-target scale to null-1 scale | `thisComp.layer("null-1").transform.scale` | 12 | (87.5, 125) | (87.5, 125) | none |
| FX-WHIP-007 | layer-target scale to null-1 scale | `thisComp.layer("null-1").transform.scale` | 24 | (75, 150) | (75, 150) | none |
| FX-WHIP-007 | layer-target scale to null-1 scale | `thisComp.layer("null-1").transform.scale` | 36 | (62.5, 175) | (62.5, 175) | none |
| FX-WHIP-007 | layer-target scale to null-1 scale | `thisComp.layer("null-1").transform.scale` | 48 | (50, 200) | (50, 200) | none |
| FX-WHIP-008 | layer-target position to camera position | `thisComp.activeCamera.position` | 0 | (969.4336249008326, 547.6315259802685) | (969.4336249008326, 547.6315259802685) | none |
| FX-WHIP-008 | layer-target position to camera position | `thisComp.activeCamera.position` | 12 | (966.7653177123977, 548.3560595722439) | (966.7653177123977, 548.3560595722439) | none |
| FX-WHIP-008 | layer-target position to camera position | `thisComp.activeCamera.position` | 24 | (964.097010523963, 549.0805931642192) | (964.097010523963, 549.0805931642192) | none |
| FX-WHIP-008 | layer-target position to camera position | `thisComp.activeCamera.position` | 36 | (965.7600662308878, 539.7250084969057) | (965.7600662308878, 539.7250084969057) | none |
| FX-WHIP-008 | layer-target position to camera position | `thisComp.activeCamera.position` | 48 | (967.4231219378125, 530.3694238295922) | (967.4231219378125, 530.3694238295922) | none |
| FX-WHIP-009 | camera depth to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 0 | 0 | 0 | none |
| FX-WHIP-009 | camera depth to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 12 | 45 | 45 | none |
| FX-WHIP-009 | camera depth to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 24 | 90 | 90 | none |
| FX-WHIP-009 | camera depth to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 36 | 135 | 135 | none |
| FX-WHIP-009 | camera depth to null-1 rotation | `thisComp.layer("null-1").transform.rotation` | 48 | 180 | 180 | none |
| FX-WHIP-010 | null-1 position to layer-trail position | `thisComp.layer("layer-trail").transform.position` | 0 | (0, 540) | (0, 540) | none |
| FX-WHIP-010 | null-1 position to layer-trail position | `thisComp.layer("layer-trail").transform.position` | 12 | (240, 540) | (240, 540) | none |
| FX-WHIP-010 | null-1 position to layer-trail position | `thisComp.layer("layer-trail").transform.position` | 24 | (720, 540) | (720, 540) | none |
| FX-WHIP-010 | null-1 position to layer-trail position | `thisComp.layer("layer-trail").transform.position` | 36 | (1200, 540) | (1200, 540) | none |
| FX-WHIP-010 | null-1 position to layer-trail position | `thisComp.layer("layer-trail").transform.position` | 48 | (1680, 540) | (1680, 540) | none |
| FX-WHIP-011 | layer-shake position to null-1 position | `thisComp.layer("null-1").transform.position` | 0 | (960, 540) | (960, 540) | none |
| FX-WHIP-011 | layer-shake position to null-1 position | `thisComp.layer("null-1").transform.position` | 12 | (1080, 480) | (1080, 480) | none |
| FX-WHIP-011 | layer-shake position to null-1 position | `thisComp.layer("null-1").transform.position` | 24 | (1200, 420) | (1200, 420) | none |
| FX-WHIP-011 | layer-shake position to null-1 position | `thisComp.layer("null-1").transform.position` | 36 | (1320, 360) | (1320, 360) | none |
| FX-WHIP-011 | layer-shake position to null-1 position | `thisComp.layer("null-1").transform.position` | 48 | (1440, 300) | (1440, 300) | none |
| FX-WHIP-012 | layer-target position to camera depth | `temp = thisComp.activeCamera.depth; [temp, temp]` | 0 | -1920 | (-1920, -1920) | none |
| FX-WHIP-012 | layer-target position to camera depth | `temp = thisComp.activeCamera.depth; [temp, temp]` | 12 | -1680 | (-1680, -1680) | none |
| FX-WHIP-012 | layer-target position to camera depth | `temp = thisComp.activeCamera.depth; [temp, temp]` | 24 | -1440 | (-1440, -1440) | none |
| FX-WHIP-012 | layer-target position to camera depth | `temp = thisComp.activeCamera.depth; [temp, temp]` | 36 | -1200 | (-1200, -1200) | none |
| FX-WHIP-012 | layer-target position to camera depth | `temp = thisComp.activeCamera.depth; [temp, temp]` | 48 | -960 | (-960, -960) | none |
| FX-WHIP-013 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 0 | (960, 540) | (960, 540) | EXPRESSION_CYCLE |
| FX-WHIP-013 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 12 | (1080, 480) | (960, 540) | EXPRESSION_CYCLE |
| FX-WHIP-013 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 24 | (1200, 420) | (960, 540) | EXPRESSION_CYCLE |
| FX-WHIP-013 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 36 | (1320, 360) | (960, 540) | EXPRESSION_CYCLE |
| FX-WHIP-013 | layer-target position to null-1 position | `thisComp.layer("null-1").transform.position` | 48 | (1440, 300) | (960, 540) | EXPRESSION_CYCLE |

Where nothing is written:

- FX-WHIP-020: Target's Position dropped on its own Position. The window sends nothing, and `property.link` asked for it directly refuses it in a sentence and leaves the project as it was.

## Timesheet fixtures

**D-84, accepted on 2026-09-24 with FX-XDTS-028 added by its own check against OpenToonz.** Each case is a cut's folder under `Fixtures/xdts/`, named after the case (FX-XDTS-001 is `fx_xdts_001`), holding one XDTS timesheet and the drawings for its columns. **No real timesheet was available, so every sheet is synthetic**, written by `tools/xdts_reference.py` from CELSYS's public specification. The same tool reads each folder by D-84's rules, on its own and not from any other reader, and writes what it must become into `Fixtures/xdts/expected_xdts.json`: the composition (name, length, 24 frames a second, and size), each layer from the bottom up (name, track, exposures as start, end and drawing, the drawing files, and the `timesheet` record), and the notes. A build passes a case when all of that matches exactly, with the notes compared by ID and facts in any order.

Every drawing is 160 by 90 and blank but for one 16-pixel square in its column's colour (A red, B green, C blue, D yellow). The square's row says the column and its distance to the right says the drawing number, so a cut played in the window shows its timing.

**How to read the lines below.** Each column is printed as the frames it shows, from frame 0 onward: a number is the drawing on that frame, and `x` is nothing. A timesheet is read the same way, down the page. Notes are the IDs in the table at the end, with their facts.

- FX-XDTS-001: One column on ones: a new drawing every frame.
  - A: `1 2 3 4 5 6`
- FX-XDTS-002: On twos: each drawing written once and held for two frames. The sheet says nothing on the frames between, and a drawing holds until the next entry.
  - A: `1 1 2 2 3 3 4 4`
- FX-XDTS-003: FX-XDTS-002 with SYMBOL_HYPHEN written on every held frame, as the specification writes a held line of dialogue. The same timing.
  - A: `1 1 2 2 3 3 4 4`
- FX-XDTS-004: A long hold: the last drawing written holds to the end of the sheet.
  - A: `1 1 1 2 2 2 2 2 2 2 2 2`
- FX-XDTS-005: Blank cells (SYMBOL_NULL_CELL, the X on a paper sheet): the column shows nothing from there until the next drawing.
  - A: `1 1 1 x x 2 2 x x x`
- FX-XDTS-006: A column that starts late: nothing is shown before its first entry.
  - A: `x x x x 1 1 1 1`
- FX-XDTS-007: Drawings reused out of order, as a cycle goes 1, 2, 3, 2, 1.
  - A: `1 1 2 2 3 3 2 2 1 1`
- FX-XDTS-008: The same drawing written twice in a row is one exposure, not two.
  - A: `1 1 1 1 2 2`
- FX-XDTS-009: Three columns, written in the file in the order 2, 0, 1: they stack by track number, 0 at the bottom, whatever the file order. A is on threes, B on twos, C on ones.
  - C: `1 2 3 4 5 6`
  - B: `1 1 2 2 3 3`
  - A: `1 1 1 2 2 2`
- FX-XDTS-010: No folders: loose files beside the sheet named for their column, with each of the four separators D-84 accepts.
  - D: `1 1`
  - C: `1 1`
  - B: `1 1`
  - A: `1 2`
- FX-XDTS-011: Both a folder and loose files for column A: the folder is used, and the loose files are named as not used.
  - A: `1 2`
  - note `{"id": "TIMESHEET_NOT_USED", "names": ["A_0001.png", "A_0002.png"]}`
- FX-XDTS-012: A column named in lower case finds a folder named in upper case.
  - a: `1 1`
- FX-XDTS-013: The two tick marks change nothing that is shown: the drawing before each holds through it, and each is reported as a mark on its frames.
  - A: `1 1 2 2`
  - note `{"id": "TIMESHEET_MARK", "column": "A", "mark": "inbetween", "frames": [1]}`
  - note `{"id": "TIMESHEET_MARK", "column": "A", "mark": "reverse sheet", "frames": [3]}`
- FX-XDTS-014: A sheet with a dialogue column and a camerawork column: all three are read, the two text columns named Dialogue and Camera, since the headers name only the cells.
  - A: `1 1 1 1`
  - dialogue column Dialogue: frames 0 to 2 `["MIKA", "Wait!"]`
  - camera column Camera: frames 0 to 3 `["PAN"]`
- FX-XDTS-015: Two timetables in one file: the first is read, the second is named as not read.
  - A: `1 1`
  - note `{"id": "TIMESHEET_TABLE_NOT_READ", "tables": ["c015 retake"]}`
  - note `{"id": "TIMESHEET_DRAWING_UNUSED", "column": "A", "drawings": [2]}`
- FX-XDTS-016: A version other than 5 is read anyway, and reported.
  - A: `1 1`
  - note `{"id": "TIMESHEET_VERSION", "version": 4}`
- FX-XDTS-017: A sheet saved with a byte-order mark and Windows line endings reads as any other.
  - A: `1 1`
- FX-XDTS-018: A timetable with no name: the composition is named after the sheet's file.
  - A: `1 1`
- FX-XDTS-019: Text columns as a sheet can write them: a line held with hyphens, a line on one frame, a cross after it, a hyphen with nothing before it, a number where text belongs, an entry past the end, and a field this program does not know.
  - A: `1 1 1 1 1 1 1 1`
  - dialogue column S1: frames 0 to 2 `["MIKA", "Wait!"]`; frame 4 `["KAI", "No."]`
  - note `{"id": "TIMESHEET_FIELD_NOT_READ", "field": "7", "tracks": 1}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "S1", "frames": [9], "reason": "outside the sheet"}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "S1", "frames": [6], "reason": "a continuation with nothing before it"}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "S1", "frames": [7], "reason": "not text"}`
- FX-XDTS-020: The sheet calls for drawing 2, which is not in A's folder. The timing is kept, so those frames will say MEDIA_SEQUENCE_GAP when drawn, and the import names the drawing.
  - A: `1 1 2 2 3 3`
  - note `{"id": "TIMESHEET_DRAWING_MISSING", "column": "A", "drawings": [2]}`
- FX-XDTS-021: A's folder holds drawings 2 and 4, which the sheet never shows.
  - A: `1 1 3 3`
  - note `{"id": "TIMESHEET_DRAWING_UNUSED", "column": "A", "drawings": [2, 4]}`
- FX-XDTS-022: Column B has no folder and no loose files: A becomes a layer and B does not, and the import says so.
  - A: `1 1`
  - note `{"id": "TIMESHEET_COLUMN_NO_DRAWINGS", "column": "B"}`
- FX-XDTS-023: A background folder and a background still beside the sheet match no column, and are named as not used. A text file is not a drawing and is not mentioned.
  - A: `1 1`
  - note `{"id": "TIMESHEET_NOT_USED", "names": ["BG", "BG.png"]}`
- FX-XDTS-024: A cell that is not a whole number (`3a`): the column is blank from there to its next entry, and the value and frame are reported.
  - A: `1 1 x x 2 2`
  - note `{"id": "TIMESHEET_CELL_UNREADABLE", "column": "A", "frame": 2, "value": "3a"}`
  - note `{"id": "TIMESHEET_DRAWING_UNUSED", "column": "A", "drawings": [3]}`
- FX-XDTS-025: Entries the sheet cannot use: a second entry on frame 2, and one on frame 6 of a 4-frame sheet. Both are left out and reported.
  - A: `1 1 2 2`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "A", "frames": [6], "reason": "outside the sheet"}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "A", "frames": [2], "reason": "a second entry on the same frame"}`
  - note `{"id": "TIMESHEET_DRAWING_UNUSED", "column": "A", "drawings": [3, 4]}`
- FX-XDTS-026: Column B holds only blank cells: it makes no layer, and its folder is named as not used.
  - A: `1 1`
  - note `{"id": "TIMESHEET_COLUMN_EMPTY", "column": "B"}`
  - note `{"id": "TIMESHEET_NOT_USED", "names": ["B"]}`
- FX-XDTS-027: Track 1 has no name in the sheet, so no drawings can be found for it and it makes no layer.
  - A: `1 1`
  - note `{"id": "TIMESHEET_COLUMN_UNNAMED", "track": 1}`
- FX-XDTS-028: Clip Studio Paint can write a column's first drawing before frame 0, which the specification does not allow. As OpenToonz reads it, the last entry before frame 0 is shown from frame 0 until the column's next entry, and is reported; A's earlier one is left out. B has its own entry on frame 0, so its entry before it is left out.
  - B: `2 2 2 2`
  - A: `1 1 2 2`
  - note `{"id": "TIMESHEET_ENTRY_CARRIED_IN", "column": "A", "frame": -1}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "A", "frames": [-2], "reason": "outside the sheet"}`
  - note `{"id": "TIMESHEET_ENTRY_IGNORED", "column": "B", "frames": [-1], "reason": "outside the sheet"}`

Nothing is imported from these, and the ID is the refusal:

- FX-XDTS-030: The folder holds no timesheet. `TIMESHEET_NOT_FOUND`
- FX-XDTS-031: The folder holds two timesheets, and which one is meant is the person's to say. `TIMESHEET_NOT_FOUND`
- FX-XDTS-032: The first line is not the XDTS line: this is bare JSON. `TIMESHEET_UNREADABLE`
- FX-XDTS-033: The first line is right and what follows is not JSON. `TIMESHEET_UNREADABLE`
- FX-XDTS-034: The sheet has a dialogue column and no cell column. `TIMESHEET_NO_CELLS`
- FX-XDTS-035: The sheet's length is 0 frames. `TIMESHEET_UNREADABLE`
- FX-XDTS-036: No column has any drawings beside the sheet, so no layer can be made. `TIMESHEET_NO_CELLS`; note `{"id": "TIMESHEET_COLUMN_NO_DRAWINGS", "column": "A"}`

### FX-XDTS-040, the sample cut

Two seconds of a made-up cut, `Fixtures/xdts/fx_xdts_040`, made to be played and learned from. It is the one B-28c's playtest uses. A is a body on twos that stops, holds and steps back; B is a mouth that moves while a line of dialogue runs, blank from frame 16 to 29 while the body holds; C is an effect that comes in late, goes, and comes back at frame 40 with a tick mark on frame 41. The sheet also has one line of dialogue and one camera instruction, which D-84c reads as text columns (below the table), and a background still, `BG.png`, which is on no column. Below is what it must show, a frame to a row, as a paper timesheet is laid out (the top of the stack is the right-hand column):

| frame | A | B | C |
| --- | --- | --- | --- |
| 0 | 1 | 1 | x |
| 1 | 1 | 1 | x |
| 2 | 2 | 1 | x |
| 3 | 2 | 2 | x |
| 4 | 3 | 2 | x |
| 5 | 3 | 2 | x |
| 6 | 4 | 3 | x |
| 7 | 4 | 2 | x |
| 8 | 5 | 1 | x |
| 9 | 5 | 1 | x |
| 10 | 6 | 1 | x |
| 11 | 6 | 2 | x |
| 12 | 7 | 2 | x |
| 13 | 7 | 2 | x |
| 14 | 8 | 3 | x |
| 15 | 8 | 2 | x |
| 16 | 8 | x | x |
| 17 | 8 | x | x |
| 18 | 8 | x | x |
| 19 | 8 | x | x |
| 20 | 8 | x | 1 |
| 21 | 8 | x | 2 |
| 22 | 8 | x | 3 |
| 23 | 8 | x | 4 |
| 24 | 7 | x | 5 |
| 25 | 7 | x | 5 |
| 26 | 6 | x | 6 |
| 27 | 6 | x | 6 |
| 28 | 5 | x | x |
| 29 | 5 | x | x |
| 30 | 4 | 1 | x |
| 31 | 4 | 1 | x |
| 32 | 3 | 1 | x |
| 33 | 3 | 2 | x |
| 34 | 2 | 2 | x |
| 35 | 2 | 2 | x |
| 36 | 1 | 3 | x |
| 37 | 1 | 3 | x |
| 38 | 1 | 3 | x |
| 39 | 1 | 1 | x |
| 40 | 1 | 1 | 1 |
| 41 | 1 | 1 | 1 |
| 42 | 1 | 1 | 2 |
| 43 | 1 | 1 | 2 |
| 44 | 1 | 1 | 2 |
| 45 | 1 | 1 | 2 |
| 46 | 1 | 1 | 2 |
| 47 | 1 | 1 | 2 |

Its notes: `{"id": "TIMESHEET_MARK", "column": "C", "mark": "inbetween", "frames": [41]}`, `{"id": "TIMESHEET_NOT_USED", "names": ["BG.png"]}`.

Its text columns (D-84c): dialogue column Dialogue, frames 0 to 15 `["MIKA", "Over here!"]`; camera column Camera, frames 20 to 47 `["FOLLOW"]`. Its Sheet, left to right, is frame, Action, Dialogue, A, B, C, Camera (D-84g). Action is empty on every frame. Dialogue shows `MIKA Over here!` on frame 0 and a line on frames 1 to 15; Camera shows `FOLLOW` on frame 20 and a line on frames 21 to 47; both are empty on every other frame.

### The notes

| ID | Severity | Meaning | Required behavior |
| --- | --- | --- | --- |
| TIMESHEET_NOT_FOUND | ERROR | The chosen folder holds no `.xdts` file, or more than one | import nothing; name what was found |
| TIMESHEET_UNREADABLE | ERROR | Not XDTS: the first line is wrong, the rest is not JSON, there is no timetable, or its length is not a number of frames | import nothing; say which |
| TIMESHEET_NO_CELLS | ERROR | No drawing column, or none that can become a layer | import nothing; the other notes say why |
| TIMESHEET_VERSION | WARNING | The file's version is not 5 | read it anyway; name the version |
| TIMESHEET_TABLE_NOT_READ | INFO | The file has more than one timetable | read the first; name the others |
| TIMESHEET_FIELD_NOT_READ | INFO | A field other than drawings, dialogue and camerawork (D-84c) | read the drawing columns; name the field and its column count |
| TIMESHEET_COLUMN_UNNAMED | WARNING | A drawing column has no name, so its drawings cannot be found | make no layer for it; name its track number |
| TIMESHEET_ENTRY_IGNORED | WARNING | An entry past the end of the sheet, one before frame 0 that is not carried in, or a second entry on one frame; in a dialogue or camera column also one before frame 0, a hyphen with nothing before it, or one that is not text (D-84c) | leave it out; name the column, frames and reason |
| TIMESHEET_ENTRY_CARRIED_IN | INFO | An entry before frame 0, which Clip Studio Paint can write and the specification does not allow | the last one stands on frame 0 unless frame 0 has its own entry; name the column and its frame |
| TIMESHEET_CELL_UNREADABLE | WARNING | A cell that is not a whole number or a symbol | the column is blank from there to its next entry; name the column, frame and value |
| TIMESHEET_MARK | INFO | A tick mark | change nothing shown; name the column, mark and frames |
| TIMESHEET_COLUMN_EMPTY | INFO | A drawing column that never shows a drawing | make no layer for it |
| TIMESHEET_COLUMN_NO_DRAWINGS | WARNING | No folder and no loose files for a column | make no layer for it; name the column |
| TIMESHEET_DRAWING_MISSING | WARNING | The sheet calls for a drawing its column does not have | keep the timing; name the drawings |
| TIMESHEET_DRAWING_UNUSED | INFO | A column has drawings the sheet never shows | name the drawings |
| TIMESHEET_NOT_USED | INFO | A folder or drawing beside the sheet that no column used | name them |


## Sheet writing fixtures

**D-84b, proposed on 2026-09-24.** Each case is one layer showing an image sequence that has drawings 1 to 9, in a composition as long as the case's lines, at 24 frames a second and starting at frame 0. A line is what the layer shows from frame 0 onward, as in the timesheet fixtures above: a number is the drawing on that frame, `x` is nothing, and `-` is outside the layer. In a Before line, `/` between two frames of the same drawing means a second exposure of it starts there. "Write 5 on frame 2" is typing 5 in that layer's cell on frame 2 and pressing Enter; "write x" is the cross; "erase frame 2" is Delete on that cell. "Exposures" is how many the layer has after, where the case is about that. A case that is refused changes nothing and puts nothing in the history. Every other case is one entry in the history, and Undo puts Before back.

- FX-SHEET-001: A number written on a hold starts that drawing there. It lasts until the next thing written below it.
  - Before: `1 1 1 1 2 2 2 2`
  - Do: write 5 on frame 2
  - After: `1 1 5 5 2 2 2 2`
- FX-SHEET-002: A number written on a number replaces it, for as long as the old one lasted.
  - Before: `1 1 1 1 2 2 2 2`
  - Do: write 7 on frame 4
  - After: `1 1 1 1 7 7 7 7`
- FX-SHEET-003: A cross written on a hold: nothing is shown from there to the next thing written.
  - Before: `1 1 1 1 2 2 2 2`
  - Do: write x on frame 2
  - After: `1 1 x x 2 2 2 2`
- FX-SHEET-004: A number written inside a blank ends the blank there.
  - Before: `1 1 x x x x 2 2`
  - Do: write 3 on frame 3
  - After: `1 1 x 3 3 3 2 2`
- FX-SHEET-005: A number written on a cross replaces the blank it started.
  - Before: `1 1 x x 2 2`
  - Do: write 4 on frame 2
  - After: `1 1 4 4 2 2`
- FX-SHEET-006: Erasing a number: the drawing above runs on through its frames, as one longer exposure.
  - Before: `1 1 2 2 3 3`
  - Do: erase frame 2
  - After: `1 1 1 1 3 3`
  - Exposures: 2
- FX-SHEET-007: Erasing a cross: the drawing above runs on through the blank.
  - Before: `1 1 x x 2 2`
  - Do: erase frame 2
  - After: `1 1 1 1 2 2`
  - Exposures: 2
- FX-SHEET-008: Erasing the first thing written: with nothing above it, those frames go blank.
  - Before: `1 1 2 2`
  - Do: erase frame 0
  - After: `x x 2 2`
- FX-SHEET-009: A held frame has nothing written on it to erase. Refused.
  - Before: `1 1 2 2`
  - Do: erase frame 1
  - After: `1 1 2 2`
- FX-SHEET-010: A number written in a blank that runs to the end of the layer lasts to the end.
  - Before: `1 1 2 2 x x x x`
  - Do: write 3 on frame 5
  - After: `1 1 2 2 x 3 3 3`
- FX-SHEET-011: Writing the drawing a frame already shows changes nothing, and nothing goes in the history.
  - Before: `1 1 1 1`
  - Do: write 1 on frame 2
  - After: `1 1 1 1`
  - Exposures: 1
- FX-SHEET-012: The rest of the column is left as it was, even two exposures of one drawing side by side.
  - Before: `1 1 / 1 1 2 2`
  - Do: write 5 on frame 4
  - After: `1 1 1 1 5 5`
  - Exposures: 3
- FX-SHEET-013: A layer that starts at frame 2: the cell on frame 3 is the layer's second frame.
  - Before: `- - 1 1 2 2`
  - Do: write 5 on frame 3
  - After: `- - 1 5 2 2`
- FX-SHEET-014: The same layer: a frame before it starts cannot be written. Refused.
  - Before: `- - 1 1 2 2`
  - Do: write 5 on frame 0
  - After: `- - 1 1 2 2`
- FX-SHEET-015: A drawing the sequence does not have is written anyway, and the answer names it.
  - Before: `1 1 2 2`
  - Do: write 12 on frame 2
  - After: `1 1 12 12`
- FX-SHEET-016: Anything but a whole number, a cross or an erase is refused.
  - Before: `1 1 2 2`
  - Do: write a on frame 2
  - After: `1 1 2 2`

**D-84g, accepted by the owner on 2026-09-24 ("proceed").** These use the same layer and lines. "Action" is what the Action column shows, frame by frame, with `.` for nothing; "Keys" is the layer's key drawings, and "Circled" the frames whose written number is circled. "Write `raises` in Action on frame 1" is typing it in the Action cell on frame 1 and pressing Enter; "erase Action on frame 1" is Delete there; "press K on frame 2" is K with the layer's cell on frame 2 chosen. Several things done are separated by `;`.

- FX-SHEET-017: Words typed in an Action cell go on its one frame.
  - Before: `1 1 2 2`; Action `. . . .`
  - Do: write `raises` in Action on frame 1
  - After: `1 1 2 2`; Action `. raises . .`
- FX-SHEET-018: Writing on a frame with a note replaces it.
  - Before: Action `. raises . .`
  - Do: write `turns` in Action on frame 1
  - After: Action `. turns . .`
- FX-SHEET-019: Erasing a note.
  - Before: Action `. raises . .`
  - Do: erase Action on frame 1
  - After: Action `. . . .`, and the composition saves no Action column
- FX-SHEET-020: An Action cell with nothing in it has nothing to erase, and only spaces write nothing. Both refused.
  - Before: Action `. . . .`
  - Do: erase Action on frame 2; write `   ` in Action on frame 2
  - After: Action `. . . .`
- FX-SHEET-021: K on a number marks its drawing a key, circled everywhere its number is written.
  - Before: `1 1 2 2 1 1 2 2`; Keys: none
  - Do: press K on frame 2
  - After: Keys: 2; Circled: frames 2 and 6
- FX-SHEET-022: K on a key's number unmarks it.
  - Before: `1 1 2 2 1 1 2 2`; Keys: 2
  - Do: press K on frame 6
  - After: Keys: none
- FX-SHEET-023: A line, a cross or an empty cell cannot be marked. Refused.
  - Before: `1 1 x x`; Keys: none
  - Do: press K on frame 1; press K on frame 2; press K on frame 3
  - After: Keys: none
- FX-SHEET-024: A note and a key, saved and opened again, read back as they were; a layer with no keys saves no `key_drawings`.
  - Before: `1 1 2 2`; Action `. raises . .`; Keys: 2
  - Do: save, close and open
  - After: `1 1 2 2`; Action `. raises . .`; Keys: 2

## Mask fixtures

**D-77, accepted by the owner on 2026-09-19**, built in the core by B-24b, put in the window by B-24c and set moving by B-24d. Every case is a composition 6 by 2 at 24 fps: three frames long where the mask stands still, and five where its path moves (FX-MSK-031 to 035), from the projects in `Fixtures/masks/`. The one drawing, `bg`, is the adjustment fixtures' own: opaque, red on the top row and sRGB grey 128 on the bottom. One raster layer carries the masks; nothing else is on it, so every cell is the drawing multiplied by the coverage the masks work out to. Each cell is R G B A of the finished frame, linear and premultiplied.

**Every number below is produced by `tools/mask_reference.py`**, which samples each outline from ADR-016's 4x4 grid and renders each pixel from document 21, and which is written from the rule rather than copied from `src/mask.rs`. The same numbers are in `Fixtures/masks/expected_masks.json`, and every still case is drawn in `verification/B-24a proposal/mask_cases.png` and every moving one, frame by frame across the row, in `verification/B-24d keys/mask_key_cases.png`, in order, numbered. Tolerance 1e-6.

FX-MSK-001 exists to be compared with what the build already does: it is today's four-point rectangle written in D-77's shape, and every number in it must be the number the build gives now. FX-MSK-002 is the same mask written the old way, with one `mask` key, and must give the same frame after conversion. FX-MSK-018 and 019 have frames **and** a warning: the mask is kept in the file, takes no part in the picture, and raises `MASK_INVALID_OUTLINE`, which is what the build does today. FX-MSK-020 to 030 have no frames: each is FX-MSK-001's file with one change, and a build must refuse it whole.

FX-MSK-001: Today's rectangle written the new way: the left three columns, and every number the same as before D-77.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-002: The same file written the old way, with one `mask` key: read as one Add mask, the same frame as FX-MSK-001.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-003: A sloped edge: coverage in whole sixteenths.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0.75 0 0 0.75 | 0.25 0 0 0.25 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.161895375 0.161895375 0.161895375 0.75 | 0.053965125 0.053965125 0.053965125 0.25 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-004: Inverted: the three columns the mask keeps are the ones it now cuts.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-005: At half opacity: the kept columns are half there.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-006: Two masks, the second Add: both halves, so the whole frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-007: Two masks, the second Subtract: the left three columns with its middle column taken out.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-008: Two masks, the second Intersect: only where both are, column 2.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-009: Two masks, the second Difference: where one is but not both.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-010: A second mask in mode None takes no part: the frame of FX-MSK-001.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-011: A first mask in Subtract takes from the whole layer: a hole in columns 2 and 3.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-012: Expanded a quarter of a pixel: the rectangle grows on every side, its corners rounded.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0.75 0 0 0.75 | 1 0 0 1 | 1 0 0 1 | 0.75 0 0 0.75 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0.161895375 0.161895375 0.161895375 0.75 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.161895375 0.161895375 0.161895375 0.75 | 0 0 0 0 |

FX-MSK-013: Shrunk a quarter of a pixel: the same rectangle the other way.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0.125 0 0 0.125 | 0.5 0 0 0.5 | 0.5 0 0 0.5 | 0.125 0 0 0.125 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0.0269825625 0.0269825625 0.0269825625 0.125 | 0.10793025 0.10793025 0.10793025 0.5 | 0.10793025 0.10793025 0.10793025 0.5 | 0.0269825625 0.0269825625 0.0269825625 0.125 | 0 0 0 0 |

FX-MSK-014: Feathered two pixels: the hard edge at column 3 becomes a soft band.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.445614162 0 0 0.445614162 | 0.566158073 0 0 0.566158073 | 0.445614162 0 0 0.445614162 | 0.192630379 0 0 0.192630379 | 0.0374642178 0 0 0.0374642178 | 0.00284196738 0 0 0.00284196738 |
| 0, 1 | 0.096190496 0.096190496 0.096190496 0.445614162 | 0.122211165 0.122211165 0.122211165 0.566158073 | 0.096190496 0.096190496 0.096190496 0.445614162 | 0.04158129 0.04158129 0.04158129 0.192630379 | 0.00808704479 0.00808704479 0.00808704479 0.0374642178 | 0.0006134685 0.0006134685 0.0006134685 0.00284196738 |

FX-MSK-015: A circle of four points with handles, filling the frame's height.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0.8125 0 0 0.8125 | 0.8125 0 0 0.8125 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.175386656 0.175386656 0.175386656 0.8125 | 0.175386656 0.175386656 0.175386656 0.8125 | 0 0 0 0 | 0 0 0 0 |

FX-MSK-016: The same circle, expanded half a pixel.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0.375 0 0 0.375 | 1 0 0 1 | 1 0 0 1 | 0.375 0 0 0.375 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0.0809476875 0.0809476875 0.0809476875 0.375 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.0809476875 0.0809476875 0.0809476875 0.375 | 0 0 0 0 |

FX-MSK-017: A mask switched off takes no part: the whole drawing.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-018: A mask of two points is kept, diagnosed and takes no part: the whole drawing.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-019: A mask whose points cross is kept, diagnosed and takes no part.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-MSK-001's file with one change:

- FX-MSK-020: Both a `mask` and a `masks` key.
- FX-MSK-021: A mode this build does not know.
- FX-MSK-022: An opacity above 1.
- FX-MSK-023: A feather below 0.
- FX-MSK-024: An expansion past the 8192 pixel limit.
- FX-MSK-025: A point that is not two numbers.
- FX-MSK-026: A handle that is not two numbers.
- FX-MSK-027: A mask with no path.
- FX-MSK-028: A `masks` key that is not a list.
- FX-MSK-029: Masks on an audio layer.
- FX-MSK-030: A keyed path whose key holds a different number of points.

### A path that moves

B-24d, on 2026-09-20, from the rule D-77 already states: a path is a document 19 property, and its value at a frame is document 20's, worked point by point - the point and both its handles, all at the same fraction of the way. A case below is five frames long and every one of its five frames is pinned, because the claim being made is about movement and a single frame cannot carry it. The number of points never changes along a path: a key holding a different number from the base is FX-MSK-030, refused whole.

FX-MSK-031: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the rectangle slides three columns in four frames, three quarters of a column a frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 0.25 0 0 0.25 | 1 0 0 1 | 1 0 0 1 | 0.75 0 0 0.75 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.053965125 0.053965125 0.053965125 0.25 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.161895375 0.161895375 0.161895375 0.75 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0 0 0 0 | 0.5 0 0 0.5 | 1 0 0 1 | 1 0 0 1 | 0.5 0 0 0.5 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0.10793025 0.10793025 0.10793025 0.5 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0 0 0 0 |
| 3, 0 | 0 0 0 0 | 0 0 0 0 | 0.75 0 0 0.75 | 1 0 0 1 | 1 0 0 1 | 0.25 0 0 0.25 |
| 3, 1 | 0 0 0 0 | 0 0 0 0 | 0.161895375 0.161895375 0.161895375 0.75 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.053965125 0.053965125 0.053965125 0.25 |
| 4, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 4, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-032: The same two keys, held: the shape does not move until frame 4, when it is the right three columns at once.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 3, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 3, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 4, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 4, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-033: The same two keys, easy ease: the same two shapes and the same path, reached at a different time - a quarter of the way through, the shape has moved 0.15625 of the way.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 0.5 0 0 0.5 | 1 0 0 1 | 1 0 0 1 | 0.5 0 0 0.5 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0 0 0 0 | 0.5 0 0 0.5 | 1 0 0 1 | 1 0 0 1 | 0.5 0 0 0.5 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0.10793025 0.10793025 0.10793025 0.5 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0 0 0 0 |
| 3, 0 | 0 0 0 0 | 0 0 0 0 | 0.5 0 0 0.5 | 1 0 0 1 | 1 0 0 1 | 0.5 0 0 0.5 |
| 3, 1 | 0 0 0 0 | 0 0 0 0 | 0.10793025 0.10793025 0.10793025 0.5 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.10793025 0.10793025 0.10793025 0.5 |
| 4, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 4, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-034: Keys at frames 1 and 3 only: frame 0 is the first key's shape and frame 4 the last key's, because a path holds outside its keys as any property does.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0 0 0 0 | 0.5 0 0 0.5 | 1 0 0 1 | 1 0 0 1 | 0.5 0 0 0.5 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0.10793025 0.10793025 0.10793025 0.5 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.10793025 0.10793025 0.10793025 0.5 | 0 0 0 0 |
| 3, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 3, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 4, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 4, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-MSK-035: Handles move with their points: the four corners stay where they are and only the handles on the right edge grow, from nothing at frame 0 to two pixels at frame 4, so the edge bellies further out every frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0.25 0 0 0.25 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.053965125 0.053965125 0.053965125 0.25 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0.625 0 0 0.625 | 0 0 0 0 | 0 0 0 0 |
| 2, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.134912813 0.134912813 0.134912813 0.625 | 0 0 0 0 | 0 0 0 0 |
| 3, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0.875 0 0 0.875 | 0 0 0 0 | 0 0 0 0 |
| 3, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.188877938 0.188877938 0.188877938 0.875 | 0 0 0 0 | 0 0 0 0 |
| 4, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 0.9375 0 0 0.9375 | 0.3125 0 0 0.3125 | 0 0 0 0 |
| 4, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.202369219 0.202369219 0.202369219 0.9375 | 0.0674564063 0.0674564063 0.0674564063 0.3125 | 0 0 0 0 |

## Shape fixtures

**D-78, accepted by the owner on 2026-09-20 ("accept D-78, proceed with B-25b"), proposed the same day by the agent as B-25a.** `tests/b25b_shapes.rs` walks every case here and writes `verification/B-25b_shape_table.md`. Every case is a composition 6 by 2 at 24 fps: one frame long where the shape stands still, and five where its path moves (FX-SHP-017), from the projects in `Fixtures/shapes/`. One shape layer carries the shapes; it has no width or height of its own, because a shape layer's space is its composition's. Nothing else is on the frame except in FX-SHP-010, where the shape layer sits over the one drawing, `bg`, the adjustment fixtures' own: opaque, red on the top row and sRGB grey 128 on the bottom. Each cell is R G B A of the finished frame, linear and premultiplied. The fill is 0.2 0.5 0.8 and the stroke 0.8 0.2 0.1, both in the working space.

**Every number below is produced by `tools/shape_reference.py`**, which samples each outline from ADR-016's 4x4 grid and renders each pixel from document 21, and which is written from D-78's rule rather than from any source file - there is none yet to copy. The same numbers are in `Fixtures/shapes/expected_shapes.json`, and every still case is drawn in `verification/B-25a proposal/shape_cases.png` and the moving one, frame by frame across the row, in `verification/B-25a proposal/shape_moving_case.png`, in order, numbered. Tolerance 1e-6.

FX-SHP-016 has a frame **and** a warning: the shape is kept in the file, takes no part in the picture, and raises `SHAPE_INVALID_OUTLINE`, a new identifier D-78 asks document 28 for. FX-SHP-020 to 030 have no frames: each is FX-SHP-001's file with one change, and a build must refuse it whole.

FX-SHP-001: A filled rectangle on the left three columns, its edges on pixel boundaries: the same sixteen decisions a mask's edge is made of.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-002: The same rectangle ending half way through column 2: that column is exactly half covered, which is the edge quantum ADR-016 states.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.1 0.25 0.4 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.1 0.25 0.4 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-003: A sloped edge: coverage in whole sixteenths.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.15 0.375 0.6 0.75 | 0.05 0.125 0.2 0.25 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.15 0.375 0.6 0.75 | 0.05 0.125 0.2 0.25 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-004: An ellipse of four curved segments, filling the frame's height.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0.1625 0.40625 0.65 0.8125 | 0.1625 0.40625 0.65 0.8125 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0.1625 0.40625 0.65 0.8125 | 0.1625 0.40625 0.65 0.8125 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-005: The same rectangle at half fill opacity: half there, and nothing else changed.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.1 0.25 0.4 0.5 | 0.1 0.25 0.4 0.5 | 0.1 0.25 0.4 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.1 0.25 0.4 0.5 | 0.1 0.25 0.4 0.5 | 0.1 0.25 0.4 0.5 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-006: A stroke and no fill, one pixel wide, on a rectangle taller than the frame: a band half a pixel either side of each upright edge, and the middle of the rectangle empty, because a stroke is on the line and not within it.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-007: A stroke on an open path of two points, one pixel wide: the band runs the length of the line and ends in a half circle at each end, because a cap is round by definition.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.15 0.0375 0.01875 0.1875 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.15 0.0375 0.01875 0.1875 |
| 0, 1 | 0.15 0.0375 0.01875 0.1875 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.4 0.1 0.05 0.5 | 0.15 0.0375 0.01875 0.1875 |

FX-SHP-008: Fill and stroke together on that rectangle: the stroke is laid over its own fill, so column 1 is the fill's colour alone, column 3 the stroke's alone, and the columns they share carry the stroke over the fill.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.5 0.35 0.45 1 | 0.2 0.5 0.8 1 | 0.5 0.35 0.45 1 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.5 0.35 0.45 1 | 0.2 0.5 0.8 1 | 0.5 0.35 0.45 1 | 0.4 0.1 0.05 0.5 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-009: Two shapes, the second covering the first: the list is drawn first to last.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.6875 0.25625 0.23125 1 | 0.65 0.1625 0.08125 0.8125 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.6875 0.25625 0.23125 1 | 0.65 0.1625 0.08125 0.8125 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-010: A shape layer over the drawing: the drawing shows everywhere the shape does not, and the shape is opaque where it does.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |

FX-SHP-011: An open path with both a fill and a stroke: the fill closes it with a straight line from its last point to its first, while the stroke does not run along that line.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0.2 0.05 0.025 0.25 | 0.6125 0.18125 0.125 0.8125 | 0.6125 0.18125 0.125 0.8125 | 0.2 0.05 0.025 0.25 | 0 0 0 0 |
| 0, 1 | 0.3 0.075 0.0375 0.375 | 0.6125 0.18125 0.125 0.8125 | 0.275 0.2375 0.325 0.625 | 0.275 0.2375 0.325 0.625 | 0.6125 0.18125 0.125 0.8125 | 0.3 0.075 0.0375 0.375 |

FX-SHP-012: A path that crosses itself, filled: the even-odd rule says what is inside, and a shape is drawn rather than diagnosed for it, which is where a shape and a mask part company.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.1625 0.40625 0.65 0.8125 | 0.0875 0.21875 0.35 0.4375 | 0.025 0.0625 0.1 0.125 | 0.0375 0.09375 0.15 0.1875 | 0.1125 0.28125 0.45 0.5625 | 0.175 0.4375 0.7 0.875 |
| 0, 1 | 0.1625 0.40625 0.65 0.8125 | 0.0875 0.21875 0.35 0.4375 | 0.025 0.0625 0.1 0.125 | 0.0375 0.09375 0.15 0.1875 | 0.1125 0.28125 0.45 0.5625 | 0.175 0.4375 0.7 0.875 |

FX-SHP-013: A mask on the shape layer, keeping its left three columns: a shape layer is masked like any other, after its shapes are drawn - the frame of FX-SHP-001.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-014: A shape with neither a fill nor a stroke: nothing is drawn and nothing is said, because a path a person has not decided about yet is not a fault.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-015: A shape switched off takes no part: the empty frame.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-016: A shape of one point is kept, diagnosed and draws nothing: there is nothing to fill and nothing to stroke between.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-SHP-017: A path keyed from the left three columns at frame 0 to the right three at frame 4, linear: the fill slides three columns in four frames, three quarters of a column a frame, by D-77's rule and document 20's.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 1, 0 | 0.05 0.125 0.2 0.25 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.15 0.375 0.6 0.75 | 0 0 0 0 | 0 0 0 0 |
| 1, 1 | 0.05 0.125 0.2 0.25 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.15 0.375 0.6 0.75 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0 0 0 0 | 0.1 0.25 0.4 0.5 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.1 0.25 0.4 0.5 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0.1 0.25 0.4 0.5 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.1 0.25 0.4 0.5 | 0 0 0 0 |
| 3, 0 | 0 0 0 0 | 0 0 0 0 | 0.15 0.375 0.6 0.75 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.05 0.125 0.2 0.25 |
| 3, 1 | 0 0 0 0 | 0 0 0 0 | 0.15 0.375 0.6 0.75 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.05 0.125 0.2 0.25 |
| 4, 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |
| 4, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 | 0.2 0.5 0.8 1 |

Refused whole, each as `PROJECT_SCHEMA_INVALID`; each is FX-SHP-001's file with one change:

- FX-SHP-020: A shape layer that names an asset.
- FX-SHP-021: A shape layer with exposures.
- FX-SHP-022: A shape layer with a source offset.
- FX-SHP-023: A `shapes` key that is not a list.
- FX-SHP-024: A shape with no path.
- FX-SHP-025: A `closed` that is not true or false.
- FX-SHP-026: A fill colour above 1.
- FX-SHP-027: A fill opacity below 0.
- FX-SHP-028: A stroke width of 0.
- FX-SHP-029: A stroke width past the 8192 pixel limit.
- FX-SHP-030: A raster layer carrying a `shapes` key.

## Keyframed effect setting fixtures

D-68, accepted on 2026-09-18. Every case is a project of one composition 6 by 2 at 24 fps, five frames long, in `Fixtures/fxkey/`. The drawings are the adjustment fixtures' `bg`, `half` and `dot`. An effect's setting is written in the file as a property record with keys, where the adjustment fixtures write a plain number.

**Every number below is produced by `tools/fxkey_reference.py`**, which works each setting's value at the frame from document 20, holds it inside its range, and renders each pixel from document 21 through `tools/adjust_reference.py`; an ease is solved by `tools/ease_reference.py`'s bisection. The same numbers are in `Fixtures/fxkey/expected_fxkey.json`. Tolerance 1e-6. The overshooting curve is `[0.3, 1.6, 0.7, 1.6]`. FX-FXK-009 holds a key outside its setting's range: the file is read, the effect is kept as written, and every frame is the frame without it, with the warning `EFFECT_PARAMETER_INVALID` (D-46).

FX-FXK-001: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 0, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 2, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 2, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |
| 3, 0 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 |
| 3, 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 |
| 4, 0 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 | 4 0 0 1 |
| 4, 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 | 0.863442 0.863442 0.863442 1 |

FX-FXK-002: A hold key: 0 stops until the next key, then 1 stop.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 0 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 | 1 0 0 1 |
| 1, 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 | 0.2158605 0.2158605 0.2158605 1 |
| 2, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 2, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |

FX-FXK-003: An eased key, 0 to 2 stops over four frames on the ease-in-out curve: slow, then fast, then slow.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 0 | 1.19608827 0 0 1 | 1.19608827 0 0 1 | 1.19608827 0 0 1 | 1.19608827 0 0 1 | 1.19608827 0 0 1 | 1.19608827 0 0 1 |
| 1, 1 | 0.258188212 0.258188212 0.258188212 1 | 0.258188212 0.258188212 0.258188212 1 | 0.258188212 0.258188212 0.258188212 1 | 0.258188212 0.258188212 0.258188212 1 | 0.258188212 0.258188212 0.258188212 1 | 0.258188212 0.258188212 0.258188212 1 |
| 2, 0 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 | 2 0 0 1 |
| 2, 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 | 0.431721 0.431721 0.431721 1 |
| 3, 0 | 3.34423479 0 0 1 | 3.34423479 0 0 1 | 3.34423479 0 0 1 | 3.34423479 0 0 1 | 3.34423479 0 0 1 | 3.34423479 0 0 1 |
| 3, 1 | 0.721888194 0.721888194 0.721888194 1 | 0.721888194 0.721888194 0.721888194 1 | 0.721888194 0.721888194 0.721888194 1 | 0.721888194 0.721888194 0.721888194 1 | 0.721888194 0.721888194 0.721888194 1 | 0.721888194 0.721888194 0.721888194 1 |

FX-FXK-004: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 |
| 0, 1 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 | 0.501960784 0 0 0.501960784 |
| 1, 0 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 |
| 1, 1 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 | 0.376470588 0 0.125490196 0.501960784 |
| 2, 0 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 |
| 2, 1 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 | 0.250980392 0 0.250980392 0.501960784 |

FX-FXK-005: A tint's amount eased from 0 to 1 on a curve that overshoots: the amount stops at 1 and goes no further.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 0 | 0.0607945324 0 0.939205468 1 | 0.0607945324 0 0.939205468 1 | 0.0607945324 0 0.939205468 1 | 0.0607945324 0 0.939205468 1 | 0.0607945324 0 0.939205468 1 | 0.0607945324 0 0.939205468 1 |
| 1, 1 | 0.0131231382 0.0131231382 0.952328606 1 | 0.0131231382 0.0131231382 0.952328606 1 | 0.0131231382 0.0131231382 0.952328606 1 | 0.0131231382 0.0131231382 0.952328606 1 | 0.0131231382 0.0131231382 0.952328606 1 | 0.0131231382 0.0131231382 0.952328606 1 |
| 2, 0 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 |
| 2, 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 | 0 0 1 1 |

FX-FXK-006: A blur on a drawing's own layer, keyed from no blur to 2 pixels.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 0 0 0 0 | 0 0 0 0 | 1 1 1 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 0, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 0 | 0.0215509428 0.0215509428 0.0215509428 0.0215509428 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.159241126 0.159241126 0.159241126 0.159241126 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.0215509428 0.0215509428 0.0215509428 0.0215509428 | 0.00176900911 0.00176900911 0.00176900911 0.00176900911 |
| 2, 1 | 0.0130713076 0.0130713076 0.0130713076 0.0130713076 | 0.0585815363 0.0585815363 0.0585815363 0.0585815363 | 0.096584625 0.096584625 0.096584625 0.096584625 | 0.0585815363 0.0585815363 0.0585815363 0.0585815363 | 0.0130713076 0.0130713076 0.0130713076 0.0130713076 | 0.00107295826 0.00107295826 0.00107295826 0.00107295826 |

FX-FXK-007: A blur eased from 1 pixel to none on the overshooting curve: a blur of less than nothing is no blur.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 2, 0 | 0 0 0 0 | 0 0 0 0 | 1 1 1 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |
| 2, 1 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 | 0 0 0 0 |

FX-FXK-008: Two settings of one effect keyed at once, and a second effect left plain.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 2, 0 | 1 0.5 0.5 1 | 1 0.5 0.5 1 | 1 0.5 0.5 1 | 1 0.5 0.5 1 | 1 0.5 0.5 1 | 1 0.5 0.5 1 |
| 2, 1 | 0.2158605 0.7158605 0.7158605 1 | 0.2158605 0.7158605 0.7158605 1 | 0.2158605 0.7158605 0.7158605 1 | 0.2158605 0.7158605 0.7158605 1 | 0.2158605 0.7158605 0.7158605 1 | 0.2158605 0.7158605 0.7158605 1 |

FX-FXK-009: A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are FX-FXK-001's frame 0.

## Separate dimension and key kind fixtures

D-69, accepted on 2026-09-18; nothing is built against these yet. FX-SEP-001 and 002 are projects in `Fixtures/keykind/` whose layer's position is written as X and Y apart, and pin the position at each of five frames. Every other case is an edit: the keys before, what is done, and the keys that must be there after, since a key's kind and roving change what an edit leaves in the file and never how a frame is worked out from it.

**Every number below is produced by `tools/keykind_reference.py`** from D-69's formulas, each checked there against a value worked by hand; an ease is solved by `tools/ease_reference.py`'s bisection. The same cases are in `Fixtures/keykind/expected_keykind.json`. Tolerance 1e-9. "To the next key" is the key's `interp`, and for an ease its four numbers.

FX-SEP-001: X goes 0 to 8 at a steady speed while Y goes 0 to 4 on the ease-in-out curve: each has its own ease.

| frame | X | Y |
| --- | --- | --- |
| 0 | 0 | 0 |
| 1 | 2 | 0.516647724 |
| 2 | 4 | 2 |
| 3 | 6 | 3.48335228 |
| 4 | 8 | 4 |

FX-SEP-002: X and Y keyed on different frames, Y with a hold: neither needs a key where the other has one.

| frame | X | Y |
| --- | --- | --- |
| 0 | 0 | 2 |
| 1 | 2 | 2 |
| 2 | 4 | 2 |
| 3 | 6 | 6 |
| 4 | 8 | 6 |

FX-KIND-001: A key made auto: it passes through at the slope between its neighbours, 4 over 12 frames, with handles a third long.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.944444444] |
| 4 | 8 | linear |
| 12 | 4 | linear |

After: make the key at frame 4 auto:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.944444444] |
| 4 | 8 | ease [0.333333333, -0.222222222, 0.666666667, 0.666666667], auto |
| 12 | 4 | linear |

FX-KIND-002: Auto on the first and last keys: a straight line out and in.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | linear |
| 4 | 8 | linear |

After: make both keys auto:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.666666667], auto |
| 4 | 8 | linear, auto |

FX-KIND-003: An auto key on a position: 5 pixels in and 12 out over 8 frames, so 17/8 of a pixel a frame on both sides.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | ease [0.333333333, 0.333333333, 0.666666667, 0.433333333] |
| 4 | [3, 4] | linear |
| 8 | [3, 16] | linear |

After: make the key at frame 4 auto:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | ease [0.333333333, 0.333333333, 0.666666667, 0.433333333] |
| 4 | [3, 4] | ease [0.333333333, 0.236111111, 0.666666667, 0.666666667], auto |
| 8 | [3, 16] | linear |

FX-KIND-004: An auto key follows its neighbours: the next key's value goes from 4 to 12 and the slope through the auto key becomes 1.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.944444444] |
| 4 | 8 | ease [0.333333333, -0.222222222, 0.666666667, 0.666666667], auto |
| 12 | 4 | linear |

After: set the value of the key at frame 12 to 12:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.833333333] |
| 4 | 8 | ease [0.333333333, 0.666666667, 0.666666667, 0.666666667], auto |
| 12 | 12 | linear |

FX-KIND-005: A corner made continuous: 2 a frame in and 1 a frame out become 1.5 on both sides, the handles keeping their lengths.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.75] |
| 4 | 8 | linear |
| 8 | 12 | linear |

After: make the key at frame 4 continuous:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.75] |
| 4 | 8 | ease [0.333333333, 0.5, 0.666666667, 0.666666667], continuous |
| 8 | 12 | linear |

FX-KIND-006: One handle of a continuous key pulled by hand: the other side follows it to the same speed, 2 a frame.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.75] |
| 4 | 8 | ease [0.333333333, 0.5, 0.666666667, 0.666666667], continuous |
| 8 | 12 | linear |

After: set the ease of the key at frame 4 to [0.25, 0.5, 0.75, 1]:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.666666667] |
| 4 | 8 | ease [0.25, 0.5, 0.75, 1], continuous |
| 8 | 12 | linear |

FX-KIND-007: An auto key beside a segment that goes nowhere: that side has no speed to set and is left as it was.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.833333333] |
| 4 | 8 | linear |
| 8 | 8 | linear |

After: make the key at frame 4 auto:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.333333333, 0.333333333, 0.666666667, 0.833333333] |
| 4 | 8 | linear, auto |
| 8 | 8 | linear |

FX-ROVE-001: A roving key: 50 pixels then 100, so it sits a third of the way through the ten frames, on frame 3.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear |
| 2 | [30, 40] | linear |
| 10 | [30, 140] | linear |

After: make the key at frame 2 roving:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear |
| 3 | [30, 40] | linear, roving |
| 10 | [30, 140] | linear |

FX-ROVE-002: Two roving keys crowded at the start of a short run: each still gets a frame of its own.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear |
| 1 | [1, 0] | linear |
| 2 | [2, 0] | linear |
| 3 | [102, 0] | linear |

After: make the keys at frames 1 and 2 roving:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear |
| 1 | [1, 0] | linear, roving |
| 2 | [2, 0] | linear, roving |
| 3 | [102, 0] | linear |

FX-ROVE-003: A run with fewer frames than roving keys is refused with COMMAND_INVALID_VALUE and nothing changes.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear |
| 1 | [10, 0] | linear, roving |
| 3 | [20, 0] | linear, roving |
| 4 | [30, 0] | linear |

After: move the key at frame 4 to frame 2: refused.

FX-ROVE-004: A curved path counts at its own length, measured along 64 straight pieces: the arch is longer than the straight run after it.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear, path handles [0, 0, 0, 60] |
| 5 | [60, 0] | linear, path handles [0, 60, 0, 0] |
| 10 | [120, 0] | linear |

After: make the key at frame 5 roving:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | linear, path handles [0, 0, 0, 60] |
| 7 | [60, 0] | linear, roving, path handles [0, 60, 0, 0] |
| 10 | [120, 0] | linear |

FX-SEP-003: Separating a position: each key becomes a key of X and a key of Y with the same ease and kind; the path handles are dropped.

Before:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 0] | ease [0.42, 0, 0.58, 1], path handles [0, 0, 0, 20] |
| 4 | [8, 4] | linear, auto |

After: separate the position, X:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.42, 0, 0.58, 1] |
| 4 | 8 | linear, auto |

After: separate the position, Y:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | ease [0.42, 0, 0.58, 1] |
| 4 | 4 | linear, auto |

FX-SEP-004: Joining FX-SEP-002 again: a key wherever either had one, holding the position at that frame, with X's ease where X had a key and else Y's.

Before, X:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | 0 | linear |
| 4 | 8 | linear |

Before, Y:

| frame | value | to the next key |
| --- | --- | --- |
| 1 | 2 | hold |
| 3 | 6 | linear |

After: join the position:

| frame | value | to the next key |
| --- | --- | --- |
| 0 | [0, 2] | linear |
| 1 | [2.0, 2] | hold |
| 3 | [6.0, 6] | linear |
| 4 | [8, 6] | linear |

## Audio fixtures

D-71 and ADR-018, accepted by the owner on 2026-09-19. `verification/B-20b_audio_table.md` walks them against the build. An audio layer draws nothing, so these pin arithmetic and file reading: which samples of a file a frame is, what is heard on a layer, what a WAV header is read as, that a picture does not change, and what a file may not say.

**Every number below is produced by `tools/audio_reference.py`**, which reads WAV with `struct` and not with a sound library, and checks each claim against a value worked by hand. The same cases are in `Fixtures/audio/expected_audio.json`. The numbers are whole and the match is exact.

FX-AUD-001: 24 frames a second and 48000 samples a second: 2000 samples to every frame.

| frame | first sample |
| --- | --- |
| 0 | 0 |
| 1 | 2000 |
| 2 | 4000 |
| 3 | 6000 |
| 4 | 8000 |
| 23 | 46000 |
| 24 | 48000 |
| 1000 | 2000000 |
| 86400 | 172800000 |

FX-AUD-002: 24 frames a second and 44100 samples: 1837.5 to a frame, so frames take 1837 and 1838 in turn and no sample is played twice or dropped.

| frame | first sample |
| --- | --- |
| 0 | 0 |
| 1 | 1837 |
| 2 | 3675 |
| 3 | 5512 |
| 4 | 7350 |
| 23 | 42262 |
| 24 | 44100 |
| 1000 | 1837500 |
| 86400 | 158760000 |

FX-AUD-003: 24000/1001 frames a second and 48000 samples: 2002 to every frame, exactly.

| frame | first sample |
| --- | --- |
| 0 | 0 |
| 1 | 2002 |
| 2 | 4004 |
| 3 | 6006 |
| 4 | 8008 |
| 23 | 46046 |
| 24 | 48048 |
| 1000 | 2002000 |
| 86400 | 172972800 |

FX-AUD-004: 24000/1001 frames a second and 44100 samples: 1839.3375 to a frame.

| frame | first sample |
| --- | --- |
| 0 | 0 |
| 1 | 1839 |
| 2 | 3678 |
| 3 | 5518 |
| 4 | 7357 |
| 23 | 42304 |
| 24 | 44144 |
| 1000 | 1839337 |
| 86400 | 158918760 |

FX-AUD-005: A layer that starts at frame 10 plays the file's first sample on frame 10.

| composition frame | samples of the file heard |
| --- | --- |
| 0 | silence |
| 9 | silence |
| 10 | 0 up to 2000 |
| 11 | 2000 up to 4000 |
| 12 | 4000 up to 6000 |
| 13 | 6000 up to 8000 |
| 14 | 8000 up to 10000 |
| 23 | 26000 up to 28000 |
| 24 | 28000 up to 30000 |
| 33 | 46000 up to 48000 |
| 34 | silence |
| 39 | silence |
| 40 | silence |

FX-AUD-006: A source offset of 5 starts the layer five frames into the file.

| composition frame | samples of the file heard |
| --- | --- |
| 0 | silence |
| 9 | silence |
| 10 | 10000 up to 12000 |
| 11 | 12000 up to 14000 |
| 12 | 14000 up to 16000 |
| 13 | 16000 up to 18000 |
| 14 | 18000 up to 20000 |
| 23 | 36000 up to 38000 |
| 24 | 38000 up to 40000 |
| 33 | silence |
| 34 | silence |
| 39 | silence |
| 40 | silence |

FX-AUD-007: A negative source offset: the layer is silent until the file's first sample comes round, three frames after the layer's in frame.

| composition frame | samples of the file heard |
| --- | --- |
| 0 | silence |
| 9 | silence |
| 10 | silence |
| 11 | silence |
| 12 | silence |
| 13 | 0 up to 2000 |
| 14 | 2000 up to 4000 |
| 23 | 20000 up to 22000 |
| 24 | 22000 up to 24000 |
| 33 | 40000 up to 42000 |
| 34 | 42000 up to 44000 |
| 39 | silence |
| 40 | silence |

FX-AUD-008: A file shorter than its layer: silence from the frame after its last sample. 47000 samples end part of the way through the 24th frame, which plays what there is.

| composition frame | samples of the file heard |
| --- | --- |
| 0 | 0 up to 2000 |
| 9 | 18000 up to 20000 |
| 10 | 20000 up to 22000 |
| 11 | 22000 up to 24000 |
| 12 | 24000 up to 26000 |
| 13 | 26000 up to 28000 |
| 14 | 28000 up to 30000 |
| 23 | 46000 up to 47000 |
| 24 | silence |
| 33 | silence |
| 34 | silence |
| 39 | silence |
| 40 | silence |

FX-AUD-010: What each WAV file in `Fixtures/audio/media/` is read as. `frames_at_24` is how many frames of a 24 frame a second composition the file covers.

| file | read as |
| --- | --- |
| `pcm16_mono_48k.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 48000, "bits": 16, "samples": 4800, "frames_at_24": 3} |
| `pcm24_stereo_44k.wav` | {"encoding": "pcm", "channels": 2, "sample_rate": 44100, "bits": 24, "samples": 4410, "frames_at_24": 3} |
| `pcm8_mono_8k.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 8000, "bits": 8, "samples": 801, "frames_at_24": 3} |
| `pcm32_mono_48k.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 48000, "bits": 32, "samples": 480, "frames_at_24": 1} |
| `float32_stereo_48k.wav` | {"encoding": "float", "channels": 2, "sample_rate": 48000, "bits": 32, "samples": 480, "frames_at_24": 1} |
| `float64_mono_96k.wav` | {"encoding": "float", "channels": 1, "sample_rate": 96000, "bits": 64, "samples": 960, "frames_at_24": 1} |
| `extensible_6ch_48k.wav` | {"encoding": "pcm", "channels": 6, "sample_rate": 48000, "bits": 16, "samples": 480, "frames_at_24": 1} |
| `chunks_in_the_way.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 48000, "bits": 16, "samples": 480, "frames_at_24": 1} |
| `cut_short.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 48000, "bits": 16, "samples": 480, "warning": "MEDIA_AUDIO_CUT_SHORT", "frames_at_24": 1} |
| `no_sound.wav` | {"encoding": "pcm", "channels": 1, "sample_rate": 48000, "bits": 16, "samples": 0, "frames_at_24": 0} |
| `adpcm.wav` | {"encoding": "other", "channels": 1, "sample_rate": 22050} |
| `rf64.wav` | {"refused": "not a RIFF WAVE file"} |
| `no_fmt.wav` | {"refused": "no format chunk"} |
| `no_data.wav` | {"refused": "no data chunk"} |
| `not_a_wav.wav` | {"refused": "not a RIFF WAVE file"} |
| `zero_rate.wav` | {"refused": "no sample rate or no channels"} |

FX-AUD-020: An audio layer between two picture layers changes no pixel: every frame of `fx_aud_020.json` is the picture of `fx_aud_021.json`, the same project without it, sample for sample, and neither has a warning.

FX-AUD-030: An audio layer with a transform: it has no place to be. Refused, `PROJECT_SCHEMA_INVALID`.

FX-AUD-031: An audio layer with effects. Refused, `PROJECT_SCHEMA_INVALID`.

FX-AUD-032: An audio layer with a blend mode. Refused, `PROJECT_SCHEMA_INVALID`.

FX-AUD-033: An audio layer whose asset is a picture. Refused, `PROJECT_SCHEMA_INVALID`.

FX-AUD-034: A level above +12 dB. Refused, `PROJECT_SCHEMA_INVALID`.

FX-AUD-035: A level that is not a number. Refused, `PROJECT_SCHEMA_INVALID`.

## Format fixtures

D-72, proposed on 2026-09-19; nothing is built against these yet. `tools/formats_reference.py` writes `Fixtures/formats/` and prints what follows. The drawings are one 16 by 12 picture of smooth ramps, saved by Pillow in each format, and each twin is Pillow's own reading of that file.

| case | file | says | its PNG twin | match |
| --- | --- | --- | --- | --- |
| FX-FMT-001 | `bmp24.bmp` | A 24-bit BMP. | `bmp24.twin.png` | exact |
| FX-FMT-002 | `tga24.tga` | A 24-bit TGA, not compressed. | `tga24.twin.png` | exact |
| FX-FMT-003 | `tga32.tga` | A 32-bit TGA with its alpha, which is how cels leave most Japanese paint software. | `tga32.twin.png` | exact |
| FX-FMT-004 | `tga32_rle.tga` | The same, run-length compressed. | `tga32_rle.twin.png` | exact |
| FX-FMT-005 | `tiff_rgba.tif` | An 8-bit TIFF with alpha, not compressed. | `tiff_rgba.twin.png` | exact |
| FX-FMT-006 | `tiff_lzw.tif` | The same, LZW compressed. | `tiff_lzw.twin.png` | exact |
| FX-FMT-007 | `webp_lossless.webp` | A lossless WebP with alpha. | `webp_lossless.twin.png` | exact |
| FX-FMT-008 | `jpeg_q95.jpg` | A JPEG. It has no alpha, so it is opaque. | `jpeg_q95.twin.png` | within 3 of 255 |
| FX-FMT-009 | `jpeg_grey.jpg` | A greyscale JPEG: the one value is red, green and blue. | `jpeg_grey.twin.png` | within 3 of 255 |
| FX-FMT-010 | `webp_lossy.webp` | A lossy WebP. | `webp_lossy.twin.png` | within 3 of 255 |

FX-FMT-020: 24 frames a second: 4.1667 hundredths to a frame, so every sixth delay is 5.

| frames | delays, in hundredths of a second | total |
| --- | --- | --- |
| 24 | 4 4 4 4 4 5 4 4 4 4 4 5 4 4 4 4 4 5 4 4 4 4 4 5 | 100 |

FX-FMT-021: 12 frames a second: 8.33 hundredths.

| frames | delays, in hundredths of a second | total |
| --- | --- | --- |
| 12 | 8 8 9 8 8 9 8 8 9 8 8 9 | 100 |

FX-FMT-022: 24000/1001 frames a second.

| frames | delays, in hundredths of a second | total |
| --- | --- | --- |
| 24 | 4 4 4 4 4 5 4 4 4 4 4 5 4 4 4 4 4 5 4 4 4 4 4 5 | 100 |

FX-FMT-023: 25 frames a second divides exactly: every delay is 4.

| frames | delays, in hundredths of a second | total |
| --- | --- | --- |
| 25 | 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 4 | 100 |

FX-FMT-030: An MP4 counts time in the frame rate's numerator and every frame lasts the denominator, so 24000/1001 is exact and no frame is dropped or doubled.

| frame rate | timescale | each frame lasts | frames | the film lasts |
| --- | --- | --- | --- | --- |
| 24/1 | 24 | 1 | 48 | 48 |
| 24000/1001 | 24000 | 1001 | 48 | 48048 |
| 30/1 | 30 | 1 | 1 | 1 |

**D-73, proposed and accepted on 2026-09-19.** The same script prints what follows. FX-FMT-040 is the number asked of the encoder, not the size of the file: an encoder spends fewer bits on a simple picture. FX-FMT-050 to 052 are small enough to check by eye.

FX-FMT-040: An MP4's quality level is thousandths of a bit for every pixel of every frame: preview 100, standard 200, high 500. The bitrate asked of the encoder is floored to whole bits a second and kept between 1 and 100 megabits.

| size | frame rate | level | bits a second asked for |
| --- | --- | --- | --- |
| 1920x1080 | 24/1 | preview | 4976640 |
| 1920x1080 | 24/1 | standard | 9953280 |
| 1920x1080 | 24/1 | high | 24883200 |
| 1280x720 | 24000/1001 | preview | 2209630 |
| 1280x720 | 24000/1001 | standard | 4419260 |
| 1280x720 | 24000/1001 | high | 11048151 |
| 64x64 | 24/1 | preview | 1000000 |
| 64x64 | 24/1 | standard | 1000000 |
| 64x64 | 24/1 | high | 1000000 |
| 3840x2160 | 60/1 | preview | 49766400 |
| 3840x2160 | 60/1 | standard | 99532800 |
| 3840x2160 | 60/1 | high | 100000000 |

FX-FMT-050: A flat mid grey with only black and white to spend becomes a checker of the two.

Palette: 0 is (0, 0, 0), 1 is (255, 255, 255). A dot is see-through.

```
1010
0101
1010
0101
```

FX-FMT-051: A grey ramp, black to white, in black, mid grey and white.

Palette: 0 is (0, 0, 0), 1 is (128, 128, 128), 2 is (255, 255, 255). A dot is see-through.

```
00111122
00111212
```

FX-FMT-052: Orange with red and yellow to spend, and a see-through pixel in the way: it gets no colour and carries no error.

Palette: 0 is (255, 0, 0), 1 is (255, 255, 0). A dot is see-through.

```
1010
0.01
1010
```

**D-73a, proposed and accepted on 2026-09-19.** FX-FMT-050 to 052 above are retired by it, marked `retired_by` in the file with their values unchanged, and no build answers to them: they spend two or three far-apart colours, which is the case the limit gives up.

D-73a: the difference passed on is held within 16.

FX-FMT-053: A flat grey of 100 between greys of 96 and 104 becomes a mix of the two.

Palette: 0 is (96, 96, 96), 1 is (104, 104, 104). A dot is see-through.

```
0100
0001
1010
0000
```

FX-FMT-054: A soft grey ramp, 90 to 118, in greys of 88, 104 and 120, with a see-through pixel in the way: it gets no colour and carries no error.

Palette: 0 is (88, 88, 88), 1 is (104, 104, 104), 2 is (120, 120, 120). A dot is see-through.

```
00111122
001.1121
```

FX-FMT-055: The specks: a field of dull red the palette has nothing near, with two greys and a bright red to spend. It becomes the nearer grey, with no bright red dots.

Palette: 0 is (96, 96, 96), 1 is (104, 104, 104), 2 is (255, 40, 40). A dot is see-through.

```
111111
111111
111111
111111
```

The same picture under D-73 as accepted, for comparison, not a fixture:

```
111111
112112
111111
121211
```

FX-FMT-060: An MP4 says what its colour is: BT.709 primaries, transfer and matrix, video range. Read back from the file, from the H.264 header or the container's colour box, whichever the file carries.

| primaries | transfer | matrix | full range |
| --- | --- | --- | --- |
| 1 | 1 | 1 | no |

## Sheet printing fixtures

**D-84d, accepted by the owner on 2026-09-24 ("proceed"), and D-84e, D-84f and D-84g, accepted the same day ("proceed").** Each case is the printable page the window writes for the composition on screen, at 6 seconds a page and in black unless it says otherwise. At 6 seconds a page, a page holds two halves of three seconds, left then right; at 3, one column of three seconds. On paper, frames are counted from 1 (D-84e): "rows 1 to 72" are the cut's first 72 frames, and the Sheet on screen's frame 0 is row 1. Rows past the cut's end are empty. "Seconds" are the numbers in the sec column, each beside the row that ends its second. The length is seconds + frames. A header written as labels, NAME `s01 c012` and so on, names the title block's boxes (D-84f); one written as four values is NAME, SHEET, TIME and RATE, with the other boxes empty. Every printed cell is the cell the Sheet shows on screen for that frame and column, and each half has the Sheet's columns in its order.

- FX-PRINT-001: The sample cut, FX-XDTS-040, imported: one page.
  - Header: NAME `s01 c012`; EPISODE, SCENE, CUT and ANIMATOR empty; TIME `2 + 0`; RATE `24 fps`; SHEET `Sheet 1 of 1`; MEMO empty
  - Page 1: left half rows 1 to 72, right half rows 73 to 144
  - Columns: sec, frame, Action, Dialogue, A, B, C, three empty, Camera
  - End line under row 48
  - Seconds: 1 to 6, beside rows 24, 48, 72 and so on to 144
- FX-PRINT-002: The sample cut made 300 frames long (Composition Settings, length 300): three pages, the last mostly empty.
  - Header: `s01 c012`, `Sheet 1 of 3` to `Sheet 3 of 3`, `12 + 12`, `24 fps`
  - Page 1: rows 1 to 72 and 73 to 144; page 2: 145 to 216 and 217 to 288; page 3: 289 to 360 and 361 to 432
  - End line under row 300
  - Seconds: 1 to 18, beside rows 24, 48, 72 and so on to 432
- FX-PRINT-003: The sample cut at 30 frames a second (Composition Settings, frame rate 30): a page holds 180 frames.
  - Header: `s01 c012`, `Sheet 1 of 1`, `1 + 18`, `30 fps`
  - Page 1: left half rows 1 to 90, right half rows 91 to 180
  - End line under row 48
  - Seconds: 1 to 6, beside rows 30, 60, 90 and so on to 180
- FX-PRINT-004: A composition with only a solid layer: nothing to print.
  - Print... is not offered; Ctrl+P says `There is nothing to print: no layer here shows drawings.`
  - `/sheet/print` writes no pages
- FX-PRINT-005: The sample cut at 3 seconds a page: one page of one column.
  - Header: `s01 c012`, `Sheet 1 of 1`, `2 + 0`, `24 fps`
  - Page 1: rows 1 to 72
  - End line under row 48
  - Seconds: 1 to 3, beside rows 24, 48 and 72
- FX-PRINT-006: The sample cut made 300 frames long, at 3 seconds a page: five pages.
  - Header: `s01 c012`, `Sheet 1 of 5` to `Sheet 5 of 5`, `12 + 12`, `24 fps`
  - Pages 1 to 5: rows 1 to 72, 73 to 144, 145 to 216, 217 to 288 and 289 to 360
  - End line under row 300
  - Seconds: 1 to 15, beside rows 24, 48, 72 and so on to 360
- FX-PRINT-007: The sample cut in red.
  - Red: the rules, the headings, the frame and second numbers, and the title block's labels
  - Black: the numbers, lines, crosses and words in the cells, and the title block's entries
  - Everything else as FX-PRINT-001
- FX-PRINT-008: A choice the window does not offer, 4 seconds a page. Refused.
  - `/sheet/print` writes no pages and says `A page holds 6 or 3 seconds.`
- FX-PRINT-009: The sample cut with Episode `3`, Scene `1`, Cut `012` and Animator `K. Sato` written in Composition Settings.
  - Header: NAME `s01 c012`, EPISODE `3`, SCENE `1`, CUT `012`, ANIMATOR `K. Sato`
  - One Undo empties the four and leaves the rest of the composition's settings as they were
- FX-PRINT-010: The same project saved and opened again.
  - The four read back as written, and print as FX-PRINT-009
  - Saved with the four empty, the file has no `sheet_details`
- FX-PRINT-011: The sample cut with `jumps` written in Action on the Sheet's frame 20, and drawing 3 of A marked a key.
  - Action: `jumps` on row 21, and nothing else in the column
  - A: 3 circled on rows 5 and 33, where it is written; no other number circled

## Line smoothing fixtures

D-86, accepted on 2026-09-25. Every case is a project of one composition 12 by 8 at 24 fps, five frames long, in `Fixtures/smooth/`, holding one drawing the same size with `core.line_smooth` on it; the drawings are in `Fixtures/smooth/media/`. Softness is 50 and threshold 10 unless the case says. Values are linear premultiplied working values, as in every section above, and only the pixels that change are listed: every other pixel is the drawing's own, exactly.

**Every number below is produced by `tools/smooth_reference.py`**, which ports OpenToonz's method in double precision and adds D-86's changes, and is checked against a second port written for the proposal on 300 random drawings. The same numbers are in `Fixtures/smooth/expected_smooth.json`. Tolerance 2e-5, since the build works in single precision through the sRGB curve twice.

**Worked by hand.** In FX-SMOOTH-001, row 4 is white on its left six pixels and black on its right six, with white above and black below, so each half is a run of 6 that reaches the edge of the picture. The slope starts at a half at the step and falls by 1/12 a pixel, so the pixel `k` places from the step, on either side, is mixed with the other colour by `0.5 - (2k + 1)/24`: 0.4583, 0.375, 0.2917, 0.2083, 0.125 and 0.0417 of the way, in encoded values. At softness 100 (FX-SMOOTH-003) it falls half as fast and the mix is `0.5 - (2k + 1)/48`, so the pixels at the ends of the row are still mixed by 0.2708. The tool checks both against its own numbers. The other cases are checked by what they claim: nothing two rows from the line changes (004), the box is unchanged and would not be without the guard (005), red and blue mix to encoded values that add to one with green 0 (007), and every pixel that shows on nothing has the line's own straight colour (008, 009).

FX-SMOOTH-001: One step between white and black, softness 50: row 4 turns into an even slope across the whole row, and no other row changes.

Frame 0, the 12 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 4 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 1, 4 | 1 1 1 1 | 0.7388447 0.7388447 0.7388447 1 |
| 2, 4 | 1 1 1 1 | 0.589799 0.589799 0.589799 1 |
| 3, 4 | 1 1 1 1 | 0.4599475 0.4599475 0.4599475 1 |
| 4, 4 | 1 1 1 1 | 0.3485102 0.3485102 0.3485102 1 |
| 5, 4 | 1 1 1 1 | 0.2546539 0.2546539 0.2546539 1 |
| 6, 4 | 0 0 0 1 | 0.1774814 0.1774814 0.1774814 1 |
| 7, 4 | 0 0 0 1 | 0.1160161 0.1160161 0.1160161 1 |
| 8, 4 | 0 0 0 1 | 0.0691804 0.0691804 0.0691804 1 |
| 9, 4 | 0 0 0 1 | 0.03576087 0.03576087 0.03576087 1 |
| 10, 4 | 0 0 0 1 | 0.01434987 0.01434987 0.01434987 1 |
| 11, 4 | 0 0 0 1 | 0.003227441 0.003227441 0.003227441 1 |

FX-SMOOTH-002: The same at softness 0: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-SMOOTH-003: The same at softness 100: the slope falls half as fast, so it is still part of the way down at both ends of the row; row 4 only.

Frame 0, the 12 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 4 | 1 1 1 1 | 0.4906527 0.4906527 0.4906527 1 |
| 1, 4 | 1 1 1 1 | 0.4303934 0.4303934 0.4303934 1 |
| 2, 4 | 1 1 1 1 | 0.3746878 0.3746878 0.3746878 1 |
| 3, 4 | 1 1 1 1 | 0.3234318 0.3234318 0.3234318 1 |
| 4, 4 | 1 1 1 1 | 0.2765176 0.2765176 0.2765176 1 |
| 5, 4 | 1 1 1 1 | 0.2338333 0.2338333 0.2338333 1 |
| 6, 4 | 0 0 0 1 | 0.1952623 0.1952623 0.1952623 1 |
| 7, 4 | 0 0 0 1 | 0.1606827 0.1606827 0.1606827 1 |
| 8, 4 | 0 0 0 1 | 0.1299668 0.1299668 0.1299668 1 |
| 9, 4 | 0 0 0 1 | 0.1029804 0.1029804 0.1029804 1 |
| 10, 4 | 0 0 0 1 | 0.07958143 0.07958143 0.07958143 1 |
| 11, 4 | 0 0 0 1 | 0.05961881 0.05961881 0.05961881 1 |

FX-SMOOTH-004: A one-pixel black line stepping down every three pixels: each step is softened, and a pixel two rows from the line is untouched.

Frame 0, the 26 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 1 | 0 0 0 1 | 0.007628138 0.007628138 0.007628138 1 |
| 1, 1 | 0 0 0 1 | 0.05087609 0.05087609 0.05087609 1 |
| 2, 1 | 0 0 0 1 | 0.14485 0.14485 0.14485 1 |
| 3, 1 | 1 1 1 1 | 0.4019778 0.4019778 0.4019778 1 |
| 4, 1 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 0, 2 | 1 1 1 1 | 0.8207967 0.8207967 0.8207967 1 |
| 1, 2 | 1 1 1 1 | 0.5225216 0.5225216 0.5225216 1 |
| 2, 2 | 1 1 1 1 | 0.2994389 0.2994389 0.2994389 1 |
| 3, 2 | 0 0 0 1 | 0.09084171 0.09084171 0.09084171 1 |
| 4, 2 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 5, 2 | 0 0 0 1 | 0.09084171 0.09084171 0.09084171 1 |
| 6, 2 | 1 1 1 1 | 0.4019778 0.4019778 0.4019778 1 |
| 7, 2 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 4, 3 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 5, 3 | 1 1 1 1 | 0.4019778 0.4019778 0.4019778 1 |
| 6, 3 | 0 0 0 1 | 0.09084171 0.09084171 0.09084171 1 |
| 7, 3 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 8, 3 | 0 0 0 1 | 0.09084171 0.09084171 0.09084171 1 |
| 9, 3 | 1 1 1 1 | 0.2994389 0.2994389 0.2994389 1 |
| 10, 3 | 1 1 1 1 | 0.5225216 0.5225216 0.5225216 1 |
| 11, 3 | 1 1 1 1 | 0.8207967 0.8207967 0.8207967 1 |
| 7, 4 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 8, 4 | 1 1 1 1 | 0.4019778 0.4019778 0.4019778 1 |
| 9, 4 | 0 0 0 1 | 0.14485 0.14485 0.14485 1 |
| 10, 4 | 0 0 0 1 | 0.05087609 0.05087609 0.05087609 1 |
| 11, 4 | 0 0 0 1 | 0.007628138 0.007628138 0.007628138 1 |

FX-SMOOTH-005: A black box six by four: straight edges and corners of 4 or more, so nothing changes.

Frame 0: every pixel is the drawing's, unchanged.

FX-SMOOTH-006: A black box three by three: its corners are shorter than 4 and are rounded.

Frame 0, the 8 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 4, 2 | 0 0 0 1 | 0.2691129 0.2691129 0.2691129 1 |
| 5, 2 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 6, 2 | 0 0 0 1 | 0.2691129 0.2691129 0.2691129 1 |
| 4, 3 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 6, 3 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 4, 4 | 0 0 0 1 | 0.2691129 0.2691129 0.2691129 1 |
| 5, 4 | 0 0 0 1 | 0.00740039 0.00740039 0.00740039 1 |
| 6, 4 | 0 0 0 1 | 0.2691129 0.2691129 0.2691129 1 |

FX-SMOOTH-007: A red line on blue: every pixel is red, blue or a mix of the two, never darker.

Frame 0, the 26 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 1 | 1 0 0 1 | 0.8207967 0 0.007628138 1 |
| 1, 1 | 1 0 0 1 | 0.5225216 0 0.05087609 1 |
| 2, 1 | 1 0 0 1 | 0.2994389 0 0.14485 1 |
| 3, 1 | 0 0 1 1 | 0.09084171 0 0.4019778 1 |
| 4, 1 | 0 0 1 1 | 0.003227441 0 0.9078199 1 |
| 0, 2 | 0 0 1 1 | 0.007628138 0 0.8207967 1 |
| 1, 2 | 0 0 1 1 | 0.05087609 0 0.5225216 1 |
| 2, 2 | 0 0 1 1 | 0.14485 0 0.2994389 1 |
| 3, 2 | 1 0 0 1 | 0.4019778 0 0.09084171 1 |
| 4, 2 | 1 0 0 1 | 0.8243209 0 0.00740039 1 |
| 5, 2 | 1 0 0 1 | 0.4019778 0 0.09084171 1 |
| 6, 2 | 0 0 1 1 | 0.09084171 0 0.4019778 1 |
| 7, 2 | 0 0 1 1 | 0.003227441 0 0.9078199 1 |
| 4, 3 | 0 0 1 1 | 0.003227441 0 0.9078199 1 |
| 5, 3 | 0 0 1 1 | 0.09084171 0 0.4019778 1 |
| 6, 3 | 1 0 0 1 | 0.4019778 0 0.09084171 1 |
| 7, 3 | 1 0 0 1 | 0.8243209 0 0.00740039 1 |
| 8, 3 | 1 0 0 1 | 0.4019778 0 0.09084171 1 |
| 9, 3 | 0 0 1 1 | 0.14485 0 0.2994389 1 |
| 10, 3 | 0 0 1 1 | 0.05087609 0 0.5225216 1 |
| 11, 3 | 0 0 1 1 | 0.007628138 0 0.8207967 1 |
| 7, 4 | 0 0 1 1 | 0.003227441 0 0.9078199 1 |
| 8, 4 | 0 0 1 1 | 0.09084171 0 0.4019778 1 |
| 9, 4 | 1 0 0 1 | 0.2994389 0 0.14485 1 |
| 10, 4 | 1 0 0 1 | 0.5225216 0 0.05087609 1 |
| 11, 4 | 1 0 0 1 | 0.8207967 0 0.007628138 1 |

FX-SMOOTH-008: A black line on nothing: the softened pixels are black, partly covering, with no grey or white fringe.

Frame 0, the 26 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 1 | 0 0 0 1 | 0 0 0 0.9166667 |
| 1, 1 | 0 0 0 1 | 0 0 0 0.75 |
| 2, 1 | 0 0 0 1 | 0 0 0 0.5833333 |
| 3, 1 | 0 0 0 0 | 0 0 0 0.3333333 |
| 4, 1 | 0 0 0 0 | 0 0 0 0.04166667 |
| 0, 2 | 0 0 0 0 | 0 0 0 0.08333333 |
| 1, 2 | 0 0 0 0 | 0 0 0 0.25 |
| 2, 2 | 0 0 0 0 | 0 0 0 0.4166667 |
| 3, 2 | 0 0 0 1 | 0 0 0 0.6666667 |
| 4, 2 | 0 0 0 1 | 0 0 0 0.9184028 |
| 5, 2 | 0 0 0 1 | 0 0 0 0.6666667 |
| 6, 2 | 0 0 0 0 | 0 0 0 0.3333333 |
| 7, 2 | 0 0 0 0 | 0 0 0 0.04166667 |
| 4, 3 | 0 0 0 0 | 0 0 0 0.04166667 |
| 5, 3 | 0 0 0 0 | 0 0 0 0.3333333 |
| 6, 3 | 0 0 0 1 | 0 0 0 0.6666667 |
| 7, 3 | 0 0 0 1 | 0 0 0 0.9184028 |
| 8, 3 | 0 0 0 1 | 0 0 0 0.6666667 |
| 9, 3 | 0 0 0 0 | 0 0 0 0.4166667 |
| 10, 3 | 0 0 0 0 | 0 0 0 0.25 |
| 11, 3 | 0 0 0 0 | 0 0 0 0.08333333 |
| 7, 4 | 0 0 0 0 | 0 0 0 0.04166667 |
| 8, 4 | 0 0 0 0 | 0 0 0 0.3333333 |
| 9, 4 | 0 0 0 1 | 0 0 0 0.5833333 |
| 10, 4 | 0 0 0 1 | 0 0 0 0.75 |
| 11, 4 | 0 0 0 1 | 0 0 0 0.9166667 |

FX-SMOOTH-009: A red line on nothing: every pixel that shows is the line's red.

Frame 0, the 26 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 1 | 1 0 0 1 | 0.9166667 0 0 0.9166667 |
| 1, 1 | 1 0 0 1 | 0.75 0 0 0.75 |
| 2, 1 | 1 0 0 1 | 0.5833333 0 0 0.5833333 |
| 3, 1 | 0 0 0 0 | 0.3333333 0 0 0.3333333 |
| 4, 1 | 0 0 0 0 | 0.04166667 0 0 0.04166667 |
| 0, 2 | 0 0 0 0 | 0.08333333 0 0 0.08333333 |
| 1, 2 | 0 0 0 0 | 0.25 0 0 0.25 |
| 2, 2 | 0 0 0 0 | 0.4166667 0 0 0.4166667 |
| 3, 2 | 1 0 0 1 | 0.6666667 0 0 0.6666667 |
| 4, 2 | 1 0 0 1 | 0.9184028 0 0 0.9184028 |
| 5, 2 | 1 0 0 1 | 0.6666667 0 0 0.6666667 |
| 6, 2 | 0 0 0 0 | 0.3333333 0 0 0.3333333 |
| 7, 2 | 0 0 0 0 | 0.04166667 0 0 0.04166667 |
| 4, 3 | 0 0 0 0 | 0.04166667 0 0 0.04166667 |
| 5, 3 | 0 0 0 0 | 0.3333333 0 0 0.3333333 |
| 6, 3 | 1 0 0 1 | 0.6666667 0 0 0.6666667 |
| 7, 3 | 1 0 0 1 | 0.9184028 0 0 0.9184028 |
| 8, 3 | 1 0 0 1 | 0.6666667 0 0 0.6666667 |
| 9, 3 | 0 0 0 0 | 0.4166667 0 0 0.4166667 |
| 10, 3 | 0 0 0 0 | 0.25 0 0 0.25 |
| 11, 3 | 0 0 0 0 | 0.08333333 0 0 0.08333333 |
| 7, 4 | 0 0 0 0 | 0.04166667 0 0 0.04166667 |
| 8, 4 | 0 0 0 0 | 0.3333333 0 0 0.3333333 |
| 9, 4 | 1 0 0 1 | 0.5833333 0 0 0.5833333 |
| 10, 4 | 1 0 0 1 | 0.75 0 0 0.75 |
| 11, 4 | 1 0 0 1 | 0.9166667 0 0 0.9166667 |

FX-SMOOTH-010: Two skin colours six apart, threshold 10: one colour to the rule, so nothing changes.

Frame 0: every pixel is the drawing's, unchanged.

FX-SMOOTH-011: The same at threshold 0: two colours, and their step is softened.

Frame 0, the 12 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.9538023 0.6706751 0.4895493 1 |
| 1, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.9494689 0.6671472 0.4866138 1 |
| 2, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.945147 0.6636301 0.4836886 1 |
| 3, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.9408366 0.6601239 0.4807738 1 |
| 4, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.9365377 0.6566285 0.4778692 1 |
| 5, 4 | 0.9559734 0.6724432 0.4910208 1 | 0.9322503 0.6531439 0.4749748 1 |
| 6, 4 | 0.9046612 0.6307571 0.456411 1 | 0.9279743 0.6496701 0.4720907 1 |
| 7, 4 | 0.9046612 0.6307571 0.456411 1 | 0.9237098 0.6462071 0.4692169 1 |
| 8, 4 | 0.9046612 0.6307571 0.456411 1 | 0.9194568 0.6427549 0.4663533 1 |
| 9, 4 | 0.9046612 0.6307571 0.456411 1 | 0.9152152 0.6393135 0.4634999 1 |
| 10, 4 | 0.9046612 0.6307571 0.456411 1 | 0.910985 0.6358829 0.4606567 1 |
| 11, 4 | 0.9046612 0.6307571 0.456411 1 | 0.9067663 0.632463 0.4578237 1 |

FX-SMOOTH-012: Softness keyed from 0 at frame 0 to 100 at frame 4, linear: frame 0 is FX-SMOOTH-002, frame 2 is FX-SMOOTH-001, frame 4 is FX-SMOOTH-003.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2, the 12 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 4 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 1, 4 | 1 1 1 1 | 0.7388447 0.7388447 0.7388447 1 |
| 2, 4 | 1 1 1 1 | 0.589799 0.589799 0.589799 1 |
| 3, 4 | 1 1 1 1 | 0.4599475 0.4599475 0.4599475 1 |
| 4, 4 | 1 1 1 1 | 0.3485102 0.3485102 0.3485102 1 |
| 5, 4 | 1 1 1 1 | 0.2546539 0.2546539 0.2546539 1 |
| 6, 4 | 0 0 0 1 | 0.1774814 0.1774814 0.1774814 1 |
| 7, 4 | 0 0 0 1 | 0.1160161 0.1160161 0.1160161 1 |
| 8, 4 | 0 0 0 1 | 0.0691804 0.0691804 0.0691804 1 |
| 9, 4 | 0 0 0 1 | 0.03576087 0.03576087 0.03576087 1 |
| 10, 4 | 0 0 0 1 | 0.01434987 0.01434987 0.01434987 1 |
| 11, 4 | 0 0 0 1 | 0.003227441 0.003227441 0.003227441 1 |

Frame 4, the 12 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 0, 4 | 1 1 1 1 | 0.4906527 0.4906527 0.4906527 1 |
| 1, 4 | 1 1 1 1 | 0.4303934 0.4303934 0.4303934 1 |
| 2, 4 | 1 1 1 1 | 0.3746878 0.3746878 0.3746878 1 |
| 3, 4 | 1 1 1 1 | 0.3234318 0.3234318 0.3234318 1 |
| 4, 4 | 1 1 1 1 | 0.2765176 0.2765176 0.2765176 1 |
| 5, 4 | 1 1 1 1 | 0.2338333 0.2338333 0.2338333 1 |
| 6, 4 | 0 0 0 1 | 0.1952623 0.1952623 0.1952623 1 |
| 7, 4 | 0 0 0 1 | 0.1606827 0.1606827 0.1606827 1 |
| 8, 4 | 0 0 0 1 | 0.1299668 0.1299668 0.1299668 1 |
| 9, 4 | 0 0 0 1 | 0.1029804 0.1029804 0.1029804 1 |
| 10, 4 | 0 0 0 1 | 0.07958143 0.07958143 0.07958143 1 |
| 11, 4 | 0 0 0 1 | 0.05961881 0.05961881 0.05961881 1 |

FX-SMOOTH-013: FX-SMOOTH-001 moved two pixels right: the same smoothed drawing, moved; the smoothing is done on the drawing's own pixels before it is moved.

Frame 0, the 10 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 2, 4 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 3, 4 | 1 1 1 1 | 0.7388447 0.7388447 0.7388447 1 |
| 4, 4 | 1 1 1 1 | 0.589799 0.589799 0.589799 1 |
| 5, 4 | 1 1 1 1 | 0.4599475 0.4599475 0.4599475 1 |
| 6, 4 | 1 1 1 1 | 0.3485102 0.3485102 0.3485102 1 |
| 7, 4 | 1 1 1 1 | 0.2546539 0.2546539 0.2546539 1 |
| 8, 4 | 0 0 0 1 | 0.1774814 0.1774814 0.1774814 1 |
| 9, 4 | 0 0 0 1 | 0.1160161 0.1160161 0.1160161 1 |
| 10, 4 | 0 0 0 1 | 0.0691804 0.0691804 0.0691804 1 |
| 11, 4 | 0 0 0 1 | 0.03576087 0.03576087 0.03576087 1 |

Frame 3, the 10 pixels that change:

| x, y | drawing | smoothed |
| --- | --- | --- |
| 2, 4 | 1 1 1 1 | 0.9078199 0.9078199 0.9078199 1 |
| 3, 4 | 1 1 1 1 | 0.7388447 0.7388447 0.7388447 1 |
| 4, 4 | 1 1 1 1 | 0.589799 0.589799 0.589799 1 |
| 5, 4 | 1 1 1 1 | 0.4599475 0.4599475 0.4599475 1 |
| 6, 4 | 1 1 1 1 | 0.3485102 0.3485102 0.3485102 1 |
| 7, 4 | 1 1 1 1 | 0.2546539 0.2546539 0.2546539 1 |
| 8, 4 | 0 0 0 1 | 0.1774814 0.1774814 0.1774814 1 |
| 9, 4 | 0 0 0 1 | 0.1160161 0.1160161 0.1160161 1 |
| 10, 4 | 0 0 0 1 | 0.0691804 0.0691804 0.0691804 1 |
| 11, 4 | 0 0 0 1 | 0.03576087 0.03576087 0.03576087 1 |

FX-SMOOTH-020: Softness 150, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SMOOTH-021: Threshold -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SMOOTH-022: Softness keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

## Selective colour blur fixtures

D-87, accepted on 2026-09-25. Every case is a project of one composition 12 by 8 at 24 fps, five frames long, in `Fixtures/selblur/`, holding one drawing the same size with `core.selective_color_blur` on it; the drawings are in `Fixtures/selblur/media/`. The colours are skin `#f6d6be`, shadow `#dba08e`, highlight `#fff3e8` and line `#1e1a24`. Blur is 6 with skin and shadow chosen unless the case says. Values are linear premultiplied working values, as in every section above, and only the pixels that change are listed: every other pixel is the drawing's own, exactly. Where several rows change the same way they are listed once.

**Every number below is produced by `tools/selblur_reference.py`**, which ports F's Plugins' method in double precision and adds D-87's changes. The same numbers are in `Fixtures/selblur/expected_selblur.json`. Tolerance 2e-5, since the build works in single precision through the sRGB curve twice.

**Worked by what they claim.** In FX-SELBLUR-001 the whole row is chosen, so the softening runs across the edge from one side of the drawing to the other and the ramp is its own mirror image: the pixel `x` places from the left and the one `x` places from the right add up to skin plus shadow, channel by channel, in encoded values. In 8-bit terms the red of the row runs 245.77, 245.46, 244.54, 242.64, 239.41, 234.97 on the skin side and 230.03, 225.59, 222.36, 220.46, 219.54, 219.23 on the shadow side, falling at every step, and never reaching past either colour. The tool checks the mirror and the fall. The other cases are checked by what they claim: nothing changes with one colour chosen, a line between, a colour one step off, or none chosen (003, 004, 008, 014); only the skin and shadow change beside a highlight (005); nothing stays exactly nothing (006); every pixel keeps its covering and takes 001's colour (007); and the keyed, moved, 5.5 and capital-letter cases are 001's own numbers (009, 010, 011, 015).

FX-SELBLUR-001: Skin beside shadow with no line between, both chosen, blur 6: the edge turns into a soft ramp across the whole row, the same on every row.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-002: The same at blur 0: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-003: The same with only the shadow chosen: it has no chosen colour beside it to mix with, so nothing changes.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-004: Skin and shadow with a black line between, both chosen: the line is not chosen, so neither colour reaches the other and nothing changes.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-005: Skin, shadow and highlight, with skin and shadow chosen: the skin and shadow edge goes soft; the highlight stays exactly as drawn, and none of it mixes into the shadow.

Frame 0:

Rows 0 to 7, each the same, the 8 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8997358 0.6364137 0.4875062 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8881293 0.6175402 0.4731447 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8624382 0.5764433 0.4418666 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8289922 0.524391 0.4022373 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.793028 0.4703325 0.361063 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.7611926 0.4242174 0.3259223 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7377253 0.3913167 0.3008406 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7273915 0.377133 0.2900247 1 |

FX-SELBLUR-006: The halves as a block on nothing, with black chosen as well: nothing stays nothing, even though its colour is black, and the soft edge reaches no further than the block.

Frame 0:

Rows 0 to 1, each the same: unchanged.

Rows 2 to 5, each the same, the 8 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8997358 0.6364137 0.4875062 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8881293 0.6175402 0.4731447 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8624382 0.5764433 0.4418666 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8289922 0.524391 0.4022373 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.793028 0.4703325 0.361063 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7611926 0.4242174 0.3259223 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7377253 0.3913167 0.3008406 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7273915 0.377133 0.2900247 1 |

Rows 6 to 7, each the same: unchanged.

FX-SELBLUR-007: The halves with the top four rows half covering: every pixel keeps its own covering, and its colour is FX-SELBLUR-001's.

Frame 0:

Rows 0 to 3, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4616067 0.3358918 0.2572146 0.5019608 |
| 1 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4603156 0.333749 0.2555844 0.5019608 |
| 2 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4564011 0.3272797 0.2506626 0.5019608 |
| 3 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4483796 0.3141541 0.2406757 0.5019608 |
| 4 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4349642 0.2926058 0.2242764 0.5019608 |
| 5 | 0.462598 0.3375401 0.2584685 0.5019608 | 0.4168972 0.2644122 0.2028124 0.5019608 |
| 6 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.397314 0.2349761 0.180392 0.5019608 |
| 7 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.3801845 0.2102385 0.1615407 0.5019608 |
| 8 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.3680077 0.1932551 0.1485926 0.5019608 |
| 9 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.3609363 0.1836304 0.1412524 0.5019608 |
| 10 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.3575404 0.1790722 0.1377754 0.5019608 |
| 11 | 0.3555769 0.1764556 0.1357793 0.5019608 | 0.3564282 0.1775883 0.1366434 0.5019608 |

Rows 4 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-008: The halves with the skin chosen one step off in blue (#f6d6bf) beside the shadow: the skin is not chosen, so nothing changes.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-009: Blur keyed from 0 at frame 0 to 12 at frame 4, linear: frame 0 is FX-SELBLUR-002, frame 2 is FX-SELBLUR-001, frame 4 is the blur at 12.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

Frame 4:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8716715 0.5911041 0.4530257 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8649036 0.5803458 0.4448371 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8571703 0.5681343 0.4355417 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8483809 0.5543622 0.4250574 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8338143 0.5317919 0.4078729 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8185862 0.5085413 0.3901672 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.8032438 0.4854807 0.3726028 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7883451 0.4634448 0.3558154 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7743912 0.4431329 0.3403383 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7661085 0.4312284 0.3312659 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7589044 0.4209678 0.3234454 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7526628 0.4121492 0.3167233 1 |

FX-SELBLUR-010: FX-SELBLUR-001 moved two pixels right: the same softened drawing, moved; the blur is done on the drawing's own pixels before it is moved.

Frame 0:

Rows 0 to 7, each the same, the 10 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |

Frame 3:

Rows 0 to 7, each the same, the 10 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |

FX-SELBLUR-011: Blur 5.5: a blur is a whole number of pixels, and a half rounds up, so this is FX-SELBLUR-001.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-012: Skin above a sloping edge, shadow below, blur 12: the edge goes soft all along its length.

Frame 0:

Row 0, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8228974 0.5150876 0.3951526 1 |
| 1 | 0.7083758 0.3515326 0.2704978 1 | 0.8179912 0.5076402 0.389481 1 |
| 2 | 0.7083758 0.3515326 0.2704978 1 | 0.812627 0.4995404 0.383312 1 |
| 3 | 0.7083758 0.3515326 0.2704978 1 | 0.8067492 0.4907169 0.3765913 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.7969332 0.4761036 0.3654595 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.7871827 0.4617405 0.3545169 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7777369 0.4479738 0.3440272 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7688157 0.4351068 0.3342218 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7605975 0.4233714 0.3252774 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7557563 0.4165117 0.3200487 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7515831 0.4106305 0.3155656 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7479785 0.4055748 0.3117113 1 |

Row 1, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8308966 0.5273096 0.4044598 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8260273 0.5198581 0.3987855 1 |
| 2 | 0.7083758 0.3515326 0.2704978 1 | 0.8206681 0.5116989 0.392572 1 |
| 3 | 0.7083758 0.3515326 0.2704978 1 | 0.8147572 0.5027516 0.3857577 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8048819 0.4879253 0.3744649 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.7949712 0.4732011 0.3632484 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7852702 0.4589415 0.3523843 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7760116 0.4454752 0.3421232 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7673913 0.4330646 0.3326653 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.762308 0.4258046 0.327132 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7578939 0.4195357 0.3223538 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7540525 0.414107 0.3182157 1 |

Row 2, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.839066 0.5398923 0.4140407 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8342697 0.5324925 0.4084064 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.828954 0.5243324 0.4021927 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8230513 0.5153218 0.395331 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8131837 0.500379 0.3839507 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.8031755 0.4853789 0.3725252 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7932747 0.4706964 0.3613402 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7837241 0.456683 0.3506634 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7747359 0.4436308 0.3407177 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7694309 0.4359898 0.3348947 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7647906 0.4293449 0.3298304 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7607221 0.4235484 0.3254124 1 |

Row 3, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8472437 0.5525888 0.4237072 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8425571 0.5453001 0.4181581 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8373255 0.5372031 0.4119931 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8314751 0.5281972 0.4051357 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8216874 0.5132473 0.3937512 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.8116522 0.4980733 0.3821945 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.801617 0.4830574 0.3707568 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7918321 0.4685702 0.3597204 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7825234 0.4549316 0.349329 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7770247 0.4469418 0.3432408 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7721803 0.4399441 0.3379082 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7679016 0.4337959 0.3332227 1 |

Row 4, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8552583 0.5651287 0.4332537 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8507164 0.5580107 0.4278349 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8456088 0.5500425 0.4217687 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8398553 0.5411134 0.4149704 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8302206 0.5262728 0.4036703 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8202329 0.5110384 0.3920689 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8101359 0.4957942 0.3804586 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8001834 0.4809252 0.3691326 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7906128 0.4667758 0.3583532 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7849553 0.4584812 0.3520335 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7799358 0.4511655 0.3464593 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7754703 0.4446922 0.3415266 1 |

Row 5, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8629445 0.577244 0.4424761 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8585788 0.5703519 0.4372298 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8536317 0.5625759 0.4313104 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8480174 0.5537952 0.4246257 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8386051 0.5391798 0.4134982 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8287391 0.5240034 0.4019422 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8186552 0.5086459 0.3902469 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8086076 0.4935009 0.3787118 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7988417 0.4789328 0.3676148 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7930652 0.4703875 0.3611049 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7879046 0.4627986 0.3553231 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7832813 0.4560367 0.350171 1 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8701591 0.5886942 0.4511915 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.865995 0.5820761 0.4461541 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8612396 0.5745491 0.4404248 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8558019 0.5659828 0.4339039 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8466739 0.5517009 0.4230313 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8369986 0.5366985 0.4116089 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8270009 0.521345 0.3999178 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8169318 0.506037 0.3882599 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.8070411 0.4911539 0.3769241 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.8011878 0.4824187 0.3702703 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7959233 0.4746087 0.3643207 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7911747 0.4676025 0.3589831 1 |

Row 7, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8767926 0.5992886 0.4592548 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8728486 0.5929821 0.454455 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8683092 0.5857511 0.4489514 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8630787 0.5774562 0.4426376 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8542855 0.5636017 0.4320912 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.844861 0.5488791 0.420883 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8350164 0.5336424 0.4092819 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.824996 0.5182845 0.3975872 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.8150509 0.5031948 0.3860953 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.8091628 0.4943335 0.3793461 1 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.8038324 0.4863587 0.3732716 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.798993 0.4791572 0.3677858 1 |

FX-SELBLUR-013: The same at blur 64, where all five passes act.

Frame 0:

Row 0, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8104982 0.4963385 0.3808732 1 |
| 1 | 0.7083758 0.3515326 0.2704978 1 | 0.8104921 0.4963293 0.3808662 1 |
| 2 | 0.7083758 0.3515326 0.2704978 1 | 0.8104798 0.4963109 0.3808522 1 |
| 3 | 0.7083758 0.3515326 0.2704978 1 | 0.8104675 0.4962924 0.3808381 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8104553 0.496274 0.3808241 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.810443 0.4962556 0.38081 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.8104307 0.4962371 0.380796 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8104184 0.4962187 0.3807819 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.8104062 0.4962002 0.3807679 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8103939 0.4961818 0.3807539 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8103816 0.4961634 0.3807398 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8103755 0.4961542 0.3807328 1 |

Row 1, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8106494 0.4965657 0.3810462 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8106433 0.4965565 0.3810392 1 |
| 2 | 0.7083758 0.3515326 0.2704978 1 | 0.810631 0.496538 0.3810252 1 |
| 3 | 0.7083758 0.3515326 0.2704978 1 | 0.8106187 0.4965196 0.3810111 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8106064 0.4965011 0.3809971 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.8105942 0.4964827 0.380983 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.8105819 0.4964642 0.380969 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8105696 0.4964458 0.3809549 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.8105573 0.4964274 0.3809409 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8105451 0.4964089 0.3809268 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8105328 0.4963905 0.3809128 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8105267 0.4963813 0.3809058 1 |

Row 2, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8108011 0.4967936 0.3812199 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.810795 0.4967844 0.3812128 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8107827 0.496766 0.3811988 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8107704 0.4967475 0.3811847 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8107581 0.4967291 0.3811707 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.8107459 0.4967106 0.3811566 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.8107336 0.4966922 0.3811426 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8107213 0.4966737 0.3811285 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.810709 0.4966553 0.3811145 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8106967 0.4966368 0.3811004 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8106845 0.4966184 0.3810864 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8106783 0.4966092 0.3810794 1 |

Row 3, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8109535 0.4970227 0.3813943 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8109473 0.4970134 0.3813873 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.810935 0.496995 0.3813732 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8109228 0.4969765 0.3813592 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8109105 0.4969581 0.3813451 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.8108982 0.4969396 0.381331 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.8108859 0.4969212 0.381317 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8108737 0.4969027 0.3813029 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.8108614 0.4968842 0.3812889 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8108491 0.4968658 0.3812748 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8108368 0.4968473 0.3812608 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8108307 0.4968381 0.3812537 1 |

Row 4, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8109569 0.4970279 0.3813983 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8109508 0.4970187 0.3813913 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8109385 0.4970002 0.3813772 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8109262 0.4969818 0.3813631 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.810914 0.4969633 0.3813491 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8109017 0.4969448 0.381335 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8108894 0.4969264 0.381321 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8108771 0.4969079 0.3813069 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.8108649 0.4968895 0.3812929 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8108526 0.496871 0.3812788 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8108403 0.4968526 0.3812648 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8108342 0.4968433 0.3812577 1 |

Row 5, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8111093 0.497257 0.3815728 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8111032 0.4972477 0.3815657 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8110909 0.4972293 0.3815517 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8110786 0.4972108 0.3815376 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8110663 0.4971923 0.3815236 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8110541 0.4971739 0.3815095 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8110418 0.4971554 0.3814954 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8110295 0.497137 0.3814814 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.8110172 0.4971185 0.3814673 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.8110049 0.4971 0.3814533 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8109927 0.4970816 0.3814392 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8109865 0.4970724 0.3814322 1 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8112611 0.4974851 0.3817465 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8112549 0.4974759 0.3817395 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8112426 0.4974574 0.3817254 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8112304 0.4974389 0.3817114 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8112181 0.4974205 0.3816973 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8112058 0.497402 0.3816832 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8111935 0.4973835 0.3816692 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8111812 0.4973651 0.3816551 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.811169 0.4973466 0.381641 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.8111567 0.4973281 0.381627 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.8111444 0.4973097 0.3816129 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8111383 0.4973005 0.3816059 1 |

Row 7, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8114123 0.4977125 0.3819198 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8114062 0.4977033 0.3819127 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8113939 0.4976848 0.3818987 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8113816 0.4976664 0.3818846 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8113693 0.4976479 0.3818705 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8113571 0.4976294 0.3818565 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8113448 0.497611 0.3818424 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8113325 0.4975925 0.3818283 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.8113202 0.497574 0.3818143 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.8113079 0.4975556 0.3818002 1 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.8112956 0.4975371 0.3817861 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.8112895 0.4975279 0.3817791 1 |

FX-SELBLUR-014: No colours chosen: nothing changes. An effect just added has none.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-015: FX-SELBLUR-001 with its colours written in capitals: the same.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-020: Blur 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELBLUR-021: Blur -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELBLUR-022: Blur keyed to 250 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELBLUR-023: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELBLUR-024: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.


### The tolerance (D-88)

D-88, accepted on 2026-09-25. The same projects, drawings and colours, with one more drawing, `speckled`: the halves with the skin painted as a checkerboard of skin `#f6d6be` and skin one step off in blue `#f6d6bf`. Tolerance is 0 unless the case says, and a project whose tolerance is 0 does not write it. The same tool writes the numbers and checks the claims: that 026, 027 and 031's later frames are FX-SELBLUR-001's own numbers or change every pixel, that 028 and 031's frame 0 change nothing, that 029 is the three colours chosen at tolerance 0, and that 030 changes the line.

FX-SELBLUR-025: Skin painted unevenly, a checkerboard of skin and skin one step off in blue, beside shadow, tolerance 0: the off pixels are not chosen and stop the softening, so the skin stays as painted except the exact skin pixel touching the shadow on every other row; the shadow goes soft.

Frame 0:

Row 0, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7236504 0.372045 0.2861443 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7202881 0.3674935 0.2826729 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7148515 0.3601772 0.2770923 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7114183 0.3555845 0.2735889 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7096081 0.3531717 0.2717482 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7089731 0.3523268 0.2711036 1 |

Row 1, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.753505 0.4133353 0.3176274 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7409233 0.3957441 0.3042164 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7212821 0.368837 0.2836976 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7153906 0.3609003 0.2776438 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7116711 0.3559221 0.2738464 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7097104 0.353308 0.2718522 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7090227 0.3523927 0.271154 1 |

Row 2, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7258508 0.3750347 0.2884244 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7220015 0.3698104 0.28444 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7157806 0.3614237 0.2780431 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7118541 0.3561663 0.2740327 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7097844 0.3534065 0.2719274 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7090586 0.3524404 0.2711904 1 |

Row 3, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.7544219 0.4146279 0.3186128 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7419948 0.3972316 0.3053505 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7224922 0.3704749 0.2849468 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7160466 0.3617809 0.2783155 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7119788 0.3563329 0.2741598 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7098349 0.3534737 0.2719786 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.709083 0.352473 0.2712152 1 |

Row 4, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7269931 0.37659 0.2896106 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7228907 0.3710148 0.2853586 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7162626 0.3620709 0.2785368 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7120801 0.3564681 0.2742629 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7098759 0.3535283 0.2720203 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7091029 0.3524994 0.2712353 1 |

Row 5, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.7550958 0.4155789 0.3193377 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7427824 0.3983263 0.3061851 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7233818 0.3716806 0.2858664 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7165286 0.3624285 0.2788095 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7122048 0.3566348 0.2743901 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7099264 0.3535955 0.2720715 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7091273 0.3525319 0.2712602 1 |

Row 6, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.72855 0.3787137 0.2912302 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7241024 0.3726583 0.2866121 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.716919 0.3629532 0.2792098 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7123878 0.3568793 0.2745766 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7100004 0.3536941 0.2721468 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7091632 0.3525796 0.2712966 1 |

Row 7, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.7563967 0.4174168 0.3207387 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7443031 0.4004427 0.3077988 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7250995 0.3740128 0.2876451 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.717459 0.3636795 0.2797638 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7126409 0.3572176 0.2748347 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7101028 0.3538305 0.2722508 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7092128 0.3526456 0.2713469 1 |

FX-SELBLUR-026: The same at tolerance 1: every skin pixel is chosen, and the edge goes soft across the whole row, as in FX-SELBLUR-001.

Frame 0:

Row 0, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5153536 1 |
| 1 | 0.9215819 0.6724432 0.5209956 1 | 0.917035 0.6648906 0.5120792 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.5021708 1 |
| 3 | 0.9215819 0.6724432 0.5209956 1 | 0.8932562 0.6258539 0.4820298 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4489416 1 |
| 5 | 0.9215819 0.6724432 0.5209956 1 | 0.8305374 0.5267586 0.4056366 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3604163 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3224102 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2963152 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815253 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745201 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722394 1 |

Row 1, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5209956 1 | 0.9196071 0.6691595 0.5153806 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5120987 1 |
| 2 | 0.9215819 0.6724432 0.5209956 1 | 0.9092366 0.6520025 0.5021771 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4820252 1 |
| 4 | 0.9215819 0.6724432 0.5209956 1 | 0.8665303 0.5829255 0.448929 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4056203 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3604007 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223983 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2963078 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815215 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745184 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722385 1 |

Row 2, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5154001 1 |
| 1 | 0.9215819 0.6724432 0.5209956 1 | 0.917035 0.6648906 0.5121128 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.5021816 1 |
| 3 | 0.9215819 0.6724432 0.5209956 1 | 0.8932562 0.6258539 0.4820218 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.44892 1 |
| 5 | 0.9215819 0.6724432 0.5209956 1 | 0.8305374 0.5267586 0.4056085 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603893 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223897 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2963024 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815188 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745172 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722379 1 |

Row 3, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5209956 1 | 0.9196071 0.6691595 0.5154134 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5121224 1 |
| 2 | 0.9215819 0.6724432 0.5209956 1 | 0.9092366 0.6520025 0.5021847 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4820196 1 |
| 4 | 0.9215819 0.6724432 0.5209956 1 | 0.8665303 0.5829255 0.4489138 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4056004 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603816 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223838 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2962988 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815169 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745164 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722375 1 |

Row 4, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5154242 1 |
| 1 | 0.9215819 0.6724432 0.5209956 1 | 0.917035 0.6648906 0.5121302 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.5021872 1 |
| 3 | 0.9215819 0.6724432 0.5209956 1 | 0.8932562 0.6258539 0.4820177 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4489087 1 |
| 5 | 0.9215819 0.6724432 0.5209956 1 | 0.8305374 0.5267586 0.4055938 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603753 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223791 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2962958 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815154 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745157 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722372 1 |

Row 5, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5209956 1 | 0.9196071 0.6691595 0.5154375 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5121398 1 |
| 2 | 0.9215819 0.6724432 0.5209956 1 | 0.9092366 0.6520025 0.5021903 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4820154 1 |
| 4 | 0.9215819 0.6724432 0.5209956 1 | 0.8665303 0.5829255 0.4489026 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4055858 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603676 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223732 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2962922 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815135 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745149 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722367 1 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.515457 1 |
| 1 | 0.9215819 0.6724432 0.5209956 1 | 0.917035 0.6648906 0.5121538 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.5021948 1 |
| 3 | 0.9215819 0.6724432 0.5209956 1 | 0.8932562 0.6258539 0.4820121 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4488935 1 |
| 5 | 0.9215819 0.6724432 0.5209956 1 | 0.8305374 0.5267586 0.405574 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603563 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223646 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2962868 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2815108 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2745137 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722361 1 |

Row 7, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5209956 1 | 0.9196071 0.6691595 0.515484 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5121733 1 |
| 2 | 0.9215819 0.6724432 0.5209956 1 | 0.9092366 0.6520025 0.5022011 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4820075 1 |
| 4 | 0.9215819 0.6724432 0.5209956 1 | 0.8665303 0.5829255 0.4488809 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4055576 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3603406 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3223527 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2962794 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.281507 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.274512 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722353 1 |

FX-SELBLUR-027: FX-SELBLUR-008, the skin chosen one step off, at tolerance 1: the skin is chosen again, and this is FX-SELBLUR-001.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-028: The same at tolerance 0.5: one step is more than half a step, so the skin is not chosen and nothing changes. The tolerance is not rounded.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELBLUR-029: Skin, shadow and highlight with skin and shadow chosen, tolerance 255: every colour that shows is chosen, the highlight too, so this is the three colours chosen at tolerance 0.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.8997358 0.6364137 0.4875062 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.8882324 0.6177654 0.4733899 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8631515 0.577895 0.4433804 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8322051 0.5304827 0.4083498 1 |
| 4 | 0.7083758 0.3515326 0.2704978 1 | 0.8038567 0.4896523 0.3799183 1 |
| 5 | 0.7083758 0.3515326 0.2704978 1 | 0.7896343 0.4729398 0.3727787 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7979416 0.4929709 0.3982124 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.8287625 0.5513447 0.4586108 1 |
| 8 | 1 0.8962694 0.8069523 1 | 0.8731072 0.6359644 0.5438023 1 |
| 9 | 1 0.8962694 0.8069523 1 | 0.9179317 0.7246955 0.6330601 1 |
| 10 | 1 0.8962694 0.8069523 1 | 0.9533653 0.7973286 0.7064344 1 |
| 11 | 1 0.8962694 0.8069523 1 | 0.9695124 0.8311672 0.7407461 1 |

FX-SELBLUR-030: Skin and shadow with a black line between, tolerance 255: the line is chosen too, and goes soft. A large tolerance takes in the lines.

Frame 0:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9079868 0.6623106 0.5077813 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.893359 0.6509551 0.4997307 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.8528541 0.6189626 0.4769573 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.78222 0.5614443 0.4356943 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.6890011 0.481958 0.3779692 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6001159 0.3998165 0.3172171 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0.5454042 0.3380464 0.2702174 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.5378847 0.3076675 0.2456217 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.5688651 0.3045818 0.2410159 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.6160167 0.3164397 0.2477123 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.6577749 0.3308923 0.2568784 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.6772565 0.3382818 0.2616753 1 |

FX-SELBLUR-031: FX-SELBLUR-027 with the tolerance keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frames 2 and 4 are FX-SELBLUR-001.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

Frame 4:

Rows 0 to 7, each the same, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.9215819 0.6724432 0.5149177 1 | 0.9196071 0.6691595 0.5124197 1 |
| 1 | 0.9215819 0.6724432 0.5149177 1 | 0.917035 0.6648906 0.5091721 1 |
| 2 | 0.9215819 0.6724432 0.5149177 1 | 0.9092366 0.6520025 0.499367 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0.8932562 0.6258539 0.4794711 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0.8665303 0.5829255 0.4468007 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.8305374 0.5267586 0.4040402 1 |
| 6 | 0.7083758 0.3515326 0.2704978 1 | 0.7915239 0.4681165 0.3593747 1 |
| 7 | 0.7083758 0.3515326 0.2704978 1 | 0.7573988 0.4188346 0.3218194 1 |
| 8 | 0.7083758 0.3515326 0.2704978 1 | 0.7331404 0.3850004 0.2960243 1 |
| 9 | 0.7083758 0.3515326 0.2704978 1 | 0.7190527 0.3658263 0.2814012 1 |
| 10 | 0.7083758 0.3515326 0.2704978 1 | 0.7122876 0.3567454 0.2744744 1 |
| 11 | 0.7083758 0.3515326 0.2704978 1 | 0.7100718 0.3537893 0.2722194 1 |

FX-SELBLUR-032: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELBLUR-033: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

## Glow fixtures

D-89, accepted on 2026-09-25. Every case is a project of one composition 16 by 10 at 24 fps, five frames long, in `Fixtures/glow/`, holding one drawing the same size with `core.glow` on it; the drawings are in `Fixtures/glow/media/`. `patches` has three patches in rows 3 to 6 on nothing: yellow `#fadc78` in columns 2 to 4, whose brightest channel is 250; brown `#996633` in columns 7 and 8, whose brightest channel is 153, exactly 60 per cent; and purple `#3c286e` in columns 11 to 13, whose brightest channel is 110. `faint` is the same half covering. Unless the case says: bright parts, threshold 60, radius 4, intensity 1, Add, no tint, no colours, tolerance 0. Values are linear premultiplied working values, and only the pixels that change are listed: every other pixel is the drawing's own, exactly.

**Every number below is produced by `tools/glow_reference.py`**, which works D-89's rule in double precision, summing the blur in two dimensions at once. The same numbers are in `Fixtures/glow/expected_glow.json`. Tolerance 2e-5.

**Checked by what they claim.** The tool checks each case's claim on its numbers: the yellow, the brown and the purple all brighten in FX-GLOW-001, though only the first two glow, and the empty space round them lights up, while the corner, more than 4 pixels from anything glowing, stays empty; intensity 0, threshold 100, a colour one step off at tolerance 0, and no colours leave the drawing exactly (002, 004, 008, 010); radius 0 adds each glowing pixel to itself (003); tolerance 1 is the exact colour (009); Add passes white at intensity 2.5 (011), and Screen never does (012); a tint's light, where nothing else lies, is the tint's colour exactly (013); the keyed cases' middle frames are 001 (016, 017, 018); and in the moved case the light that spread past the drawing's left edge shows (019). In 001, row 4's red working value runs 0.101, 0.281, 1.461, 1.566, 1.468, 0.314, 0.188, 0.483, 0.463, 0.087, 0.033, 0.053, 0.046, 0.045, 0, 0: the yellow passes 1, which is white, as Add lets it, and a display cuts it at white.

FX-GLOW-001: Bright parts, threshold 60, radius 4: the yellow patch and the brown one, whose brightest channel is exactly 60 %, glow; the purple does not, though the light of the others reaches it. The glow spreads into the empty space round the patches, which shows it.

Frame 0:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 8 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 9 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 10 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 11 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 12 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 8 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 9 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 10 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 11 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 12 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 8 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 9 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 10 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 11 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 12 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4434865 0.1905424 0.04764125 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4287694 0.1795243 0.04474976 1 |
| 9 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 10 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05077497 0.02355013 0.1565073 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04587096 0.02150463 0.1559976 1 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4825636 0.208581 0.05218778 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4632433 0.1941168 0.04839193 1 |
| 9 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 10 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05252296 0.02427923 0.1566889 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04608513 0.02159396 0.1560199 1 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4825636 0.208581 0.05218778 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4632433 0.1941168 0.04839193 1 |
| 9 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 10 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05252296 0.02427923 0.1566889 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04608513 0.02159396 0.1560199 1 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4434865 0.1905424 0.04764125 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4287694 0.1795243 0.04474976 1 |
| 9 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 10 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05077497 0.02355013 0.1565073 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04587096 0.02150463 0.1559976 1 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 8 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 9 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 10 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 11 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 12 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 8 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 9 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 10 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 11 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 12 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 8 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 9 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 10 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 11 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 12 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

FX-GLOW-002: Intensity 0: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-GLOW-003: Radius 0: nothing spreads, and each glowing pixel is added onto itself, twice as bright.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2: unchanged.

Row 3, the 5 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |

Row 4, the 5 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |

Row 5, the 5 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |

Row 6, the 5 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.911947 1.431387 0.3756415 1 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6370936 0.2657366 0.06620953 1 |

Row 7: unchanged.

Row 8: unchanged.

Row 9: unchanged.

FX-GLOW-004: Threshold 100: nothing is that bright, so the drawing is untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-GLOW-005: Threshold 0: every pixel that shows glows, the purple too.

Frame 0:

Row 0, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005250874 0.002423916 0.0006245297 0.01508698 |
| 8 | 0 0 0 0 | 0.004662045 0.001974934 0.0006039099 0.01508698 |
| 9 | 0 0 0 0 | 0.002946798 0.001237136 0.0008166354 0.01214694 |
| 10 | 0 0 0 0 | 0.001471902 0.0006362777 0.001577334 0.01270317 |
| 11 | 0 0 0 0 | 0.0009982581 0.0004564549 0.002659248 0.01763485 |
| 12 | 0 0 0 0 | 0.0009502745 0.0004447307 0.003182922 0.02048408 |
| 13 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 14 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 15 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |

Row 1, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02405243 0.01110312 0.002860754 0.06910823 |
| 8 | 0 0 0 0 | 0.02135521 0.009046488 0.002766302 0.06910823 |
| 9 | 0 0 0 0 | 0.01349826 0.00566689 0.003740723 0.05564092 |
| 10 | 0 0 0 0 | 0.006742273 0.002914568 0.007225222 0.05818883 |
| 11 | 0 0 0 0 | 0.004572674 0.002090861 0.01218109 0.08077915 |
| 12 | 0 0 0 0 | 0.004352878 0.002037157 0.01457986 0.09383044 |
| 13 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 14 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 15 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |

Row 2, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06776737 0.03128286 0.008060131 0.1947114 |
| 8 | 0 0 0 0 | 0.06016799 0.02548834 0.007794013 0.1947114 |
| 9 | 0 0 0 0 | 0.03803114 0.01596638 0.01053943 0.1567674 |
| 10 | 0 0 0 0 | 0.01899625 0.00821175 0.02035695 0.1639461 |
| 11 | 0 0 0 0 | 0.01288344 0.005890971 0.03432005 0.227594 |
| 12 | 0 0 0 0 | 0.01226417 0.005739659 0.04107854 0.2643659 |
| 13 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 14 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 15 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |

Row 3, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4435837 0.190588 0.04797643 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4295621 0.1798966 0.04748542 1 |
| 9 | 0 0 0 0 | 0.07017087 0.0294594 0.0194462 0.2892501 |
| 10 | 0 0 0 0 | 0.0350498 0.01515142 0.03756041 0.3024954 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06895731 0.03208838 0.21925 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.0678147 0.0318092 0.2317201 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 14 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 15 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |

Row 4, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4826911 0.2086408 0.0526278 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4642841 0.1946055 0.05198322 1 |
| 9 | 0 0 0 0 | 0.09211801 0.03867333 0.02552833 0.3797179 |
| 10 | 0 0 0 0 | 0.04601222 0.01989028 0.04930807 0.397106 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07639213 0.03548796 0.2390556 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.07489214 0.03512146 0.2554258 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 14 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 15 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |

Row 5, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4826911 0.2086408 0.0526278 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4642841 0.1946055 0.05198322 1 |
| 9 | 0 0 0 0 | 0.09211801 0.03867333 0.02552833 0.3797179 |
| 10 | 0 0 0 0 | 0.04601222 0.01989028 0.04930807 0.397106 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07639213 0.03548796 0.2390556 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.07489214 0.03512146 0.2554258 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 14 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 15 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |

Row 6, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4435837 0.190588 0.04797643 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4295621 0.1798966 0.04748542 1 |
| 9 | 0 0 0 0 | 0.07017087 0.0294594 0.0194462 0.2892501 |
| 10 | 0 0 0 0 | 0.0350498 0.01515142 0.03756041 0.3024954 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06895731 0.03208838 0.21925 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.0678147 0.0318092 0.2317201 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 14 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 15 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |

Row 7, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06776737 0.03128286 0.008060131 0.1947114 |
| 8 | 0 0 0 0 | 0.06016799 0.02548834 0.007794013 0.1947114 |
| 9 | 0 0 0 0 | 0.03803114 0.01596638 0.01053943 0.1567674 |
| 10 | 0 0 0 0 | 0.01899625 0.00821175 0.02035695 0.1639461 |
| 11 | 0 0 0 0 | 0.01288344 0.005890971 0.03432005 0.227594 |
| 12 | 0 0 0 0 | 0.01226417 0.005739659 0.04107854 0.2643659 |
| 13 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 14 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 15 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |

Row 8, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02405243 0.01110312 0.002860754 0.06910823 |
| 8 | 0 0 0 0 | 0.02135521 0.009046488 0.002766302 0.06910823 |
| 9 | 0 0 0 0 | 0.01349826 0.00566689 0.003740723 0.05564092 |
| 10 | 0 0 0 0 | 0.006742273 0.002914568 0.007225222 0.05818883 |
| 11 | 0 0 0 0 | 0.004572674 0.002090861 0.01218109 0.08077915 |
| 12 | 0 0 0 0 | 0.004352878 0.002037157 0.01457986 0.09383044 |
| 13 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 14 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 15 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |

Row 9, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005250874 0.002423916 0.0006245297 0.01508698 |
| 8 | 0 0 0 0 | 0.004662045 0.001974934 0.0006039099 0.01508698 |
| 9 | 0 0 0 0 | 0.002946798 0.001237136 0.0008166354 0.01214694 |
| 10 | 0 0 0 0 | 0.001471902 0.0006362777 0.001577334 0.01270317 |
| 11 | 0 0 0 0 | 0.0009982581 0.0004564549 0.002659248 0.01763485 |
| 12 | 0 0 0 0 | 0.0009502745 0.0004447307 0.003182922 0.02048408 |
| 13 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 14 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 15 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |

FX-GLOW-006: Threshold 61: the brown, at exactly 60 %, no longer glows.

Frame 0:

Row 0, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01949593 0.01459571 0.00383038 0.0203938 |
| 4 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 5 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 6 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 7 | 0 0 0 0 | 0.0007043398 0.000527307 0.0001383822 0.0007367777 |
| 8 | 0 0 0 0 | 8.629798e-05 6.460735e-05 1.695503e-05 9.027237e-05 |

Row 1, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.0893041 0.0668579 0.01754564 0.09341694 |
| 4 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 5 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 6 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 7 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 8 | 0 0 0 0 | 0.0003953011 0.0002959439 7.766509e-05 0.0004135064 |

Row 2, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.251613 0.1883711 0.04943458 0.2632008 |
| 4 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 5 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 6 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 7 | 0 0 0 0 | 0.009090154 0.006805383 0.001785949 0.009508795 |
| 8 | 0 0 0 0 | 0.001113755 0.0008338173 0.0002188202 0.001165048 |

Row 3, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420222 1.063255 0.279032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 5 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 6 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3353189 0.1454249 0.0364 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3206018 0.1344068 0.03350851 1 |

Row 4, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.565424 1.171961 0.3075599 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 5 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 6 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3405647 0.1493521 0.03743064 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3212445 0.134888 0.03363479 1 |

Row 5, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.565424 1.171961 0.3075599 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 5 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 6 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3405647 0.1493521 0.03743064 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3212445 0.134888 0.03363479 1 |

Row 6, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.420222 1.063255 0.279032 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 5 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 6 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3353189 0.1454249 0.0364 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3206018 0.1344068 0.03350851 1 |

Row 7, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.251613 0.1883711 0.04943458 0.2632008 |
| 4 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 5 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 6 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 7 | 0 0 0 0 | 0.009090154 0.006805383 0.001785949 0.009508795 |
| 8 | 0 0 0 0 | 0.001113755 0.0008338173 0.0002188202 0.001165048 |

Row 8, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.0893041 0.0668579 0.01754564 0.09341694 |
| 4 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 5 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 6 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 7 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 8 | 0 0 0 0 | 0.0003953011 0.0002959439 7.766509e-05 0.0004135064 |

Row 9, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01949593 0.01459571 0.00383038 0.0203938 |
| 4 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 5 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 6 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 7 | 0 0 0 0 | 0.0007043398 0.000527307 0.0001383822 0.0007367777 |
| 8 | 0 0 0 0 | 8.629798e-05 6.460735e-05 1.695503e-05 9.027237e-05 |

FX-GLOW-007: Chosen colours, the purple chosen: only the purple glows.

Frame 0:

Row 0, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 4.079066e-06 1.91549e-06 1.407585e-05 9.027237e-05 |
| 8 | 0 0 0 0 | 3.329219e-05 1.563369e-05 0.0001148831 0.0007367777 |
| 9 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |
| 10 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 11 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 12 | 0 0 0 0 | 0.0009215186 0.0004327363 0.003179934 0.0203938 |
| 13 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 14 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 15 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |

Row 1, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 1.868479e-05 8.774197e-06 6.447659e-05 0.0004135064 |
| 8 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |
| 9 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |
| 10 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 11 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 12 | 0 0 0 0 | 0.004221157 0.001982215 0.01456617 0.09341694 |
| 13 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 14 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 15 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |

Row 2, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 5.26441e-05 2.472117e-05 0.0001816618 0.001165048 |
| 8 | 0 0 0 0 | 0.0004296663 0.0002017672 0.001482673 0.009508795 |
| 9 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |
| 10 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 11 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 12 | 0 0 0 0 | 0.01189305 0.005584861 0.04103997 0.2632008 |
| 13 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 14 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 15 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |

Row 3, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3186439 0.1329139 0.03343995 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3193396 0.1332406 0.03584043 1 |
| 9 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |
| 10 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06712994 0.03152358 0.2316489 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 14 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 15 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |

Row 4, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3186743 0.1329282 0.03354478 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3195875 0.133357 0.03669606 1 |
| 9 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |
| 10 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.07399322 0.03474651 0.2553324 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 14 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 15 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |

Row 5, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3186743 0.1329282 0.03354478 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3195875 0.133357 0.03669606 1 |
| 9 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |
| 10 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.07399322 0.03474651 0.2553324 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06905538 0.03242775 0.2382931 1 |
| 14 | 0 0 0 0 | 0.01330402 0.00624744 0.04590889 0.2944266 |
| 15 | 0 0 0 0 | 0.004767206 0.002238635 0.01645046 0.1055014 |

Row 6, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3186439 0.1329139 0.03343995 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3193396 0.1332406 0.03584043 1 |
| 9 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |
| 10 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06712994 0.03152358 0.2316489 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.06336854 0.02975726 0.2186692 1 |
| 14 | 0 0 0 0 | 0.01013433 0.004758986 0.03497109 0.2242794 |
| 15 | 0 0 0 0 | 0.003631418 0.001705279 0.01253113 0.08036564 |

Row 7, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 5.26441e-05 2.472117e-05 0.0001816618 0.001165048 |
| 8 | 0 0 0 0 | 0.0004296663 0.0002017672 0.001482673 0.009508795 |
| 9 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |
| 10 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 11 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 12 | 0 0 0 0 | 0.01189305 0.005584861 0.04103997 0.2632008 |
| 13 | 0 0 0 0 | 0.009854444 0.004627553 0.03400526 0.2180852 |
| 14 | 0 0 0 0 | 0.005492596 0.002579271 0.0189536 0.1215547 |
| 15 | 0 0 0 0 | 0.001968152 0.0009242256 0.006791609 0.04355649 |

Row 8, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 1.868479e-05 8.774197e-06 6.447659e-05 0.0004135064 |
| 8 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |
| 9 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |
| 10 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 11 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 12 | 0 0 0 0 | 0.004221157 0.001982215 0.01456617 0.09341694 |
| 13 | 0 0 0 0 | 0.003497603 0.001642441 0.01206937 0.07740423 |
| 14 | 0 0 0 0 | 0.001949468 0.0009154514 0.006727133 0.04314298 |
| 15 | 0 0 0 0 | 0.0006985494 0.0003280321 0.002410522 0.01545935 |

Row 9, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 4.079066e-06 1.91549e-06 1.407585e-05 9.027237e-05 |
| 8 | 0 0 0 0 | 3.329219e-05 1.563369e-05 0.0001148831 0.0007367777 |
| 9 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |
| 10 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 11 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 12 | 0 0 0 0 | 0.0009215186 0.0004327363 0.003179934 0.0203938 |
| 13 | 0 0 0 0 | 0.0007635599 0.0003585605 0.002634857 0.01689808 |
| 14 | 0 0 0 0 | 0.0004255873 0.0001998517 0.001468597 0.009418522 |
| 15 | 0 0 0 0 | 0.0001525 7.161253e-05 0.0005262398 0.003374923 |

FX-GLOW-008: The purple chosen one step off in blue, tolerance 0: nothing is chosen, and the drawing is untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-GLOW-009: The same at tolerance 1: the purple is chosen again, and this is FX-GLOW-007.

Frame 0: the same as FX-GLOW-007 frame 0.

FX-GLOW-010: Chosen colours with none chosen: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-GLOW-011: Intensity 2.5: the glow is two and a half times as strong, and where it adds past white it is not cut off.

Frame 0:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.008065841 0.006038526 0.001584702 0.008437308 |
| 1 | 0 0 0 0 | 0.02250964 0.01685194 0.004422485 0.02354631 |
| 2 | 0 0 0 0 | 0.04038528 0.03023461 0.007934525 0.04224519 |
| 3 | 0 0 0 0 | 0.04881172 0.03651927 0.009583421 0.05121019 |
| 4 | 0 0 0 0 | 0.04097202 0.03047935 0.007995502 0.04408714 |
| 5 | 0 0 0 0 | 0.02512543 0.017943 0.004694329 0.03175793 |
| 6 | 0 0 0 0 | 0.01505159 0.008952334 0.002310691 0.03036735 |
| 7 | 0 0 0 0 | 0.01311699 0.006055 0.001526135 0.03749177 |
| 8 | 0 0 0 0 | 0.01157188 0.004898251 0.001222567 0.03587551 |
| 9 | 0 0 0 0 | 0.006985744 0.002913808 0.0007259889 0.02193004 |
| 10 | 0 0 0 0 | 0.002615787 0.001091065 0.000271844 0.008211627 |
| 11 | 0 0 0 0 | 0.0005867454 0.000244736 6.097713e-05 0.001841944 |
| 12 | 0 0 0 0 | 7.188993e-05 2.998585e-05 7.471114e-06 0.0002256809 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03694682 0.02766039 0.007258968 0.03864838 |
| 1 | 0 0 0 0 | 0.1031089 0.07719288 0.02025787 0.1078575 |
| 2 | 0 0 0 0 | 0.1849909 0.1384943 0.0363453 0.1935106 |
| 3 | 0 0 0 0 | 0.2235896 0.1672821 0.04389833 0.2345761 |
| 4 | 0 0 0 0 | 0.1876786 0.1396153 0.03662462 0.2019479 |
| 5 | 0 0 0 0 | 0.1150909 0.08219067 0.02150309 0.1454721 |
| 6 | 0 0 0 0 | 0.06894609 0.04100754 0.01058447 0.1391023 |
| 7 | 0 0 0 0 | 0.06008437 0.02773586 0.006990693 0.1717368 |
| 8 | 0 0 0 0 | 0.05300678 0.02243719 0.005600155 0.1643333 |
| 9 | 0 0 0 0 | 0.03199927 0.01334714 0.003325504 0.1004539 |
| 10 | 0 0 0 0 | 0.01198201 0.004997791 0.001245223 0.03761461 |
| 11 | 0 0 0 0 | 0.002687677 0.001121051 0.0002793151 0.008437308 |
| 12 | 0 0 0 0 | 0.0003293028 0.0001373548 3.422258e-05 0.001033766 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1040971 0.07793274 0.02045203 0.1088912 |
| 1 | 0 0 0 0 | 0.2905077 0.2174898 0.05707625 0.3038868 |
| 2 | 0 0 0 0 | 0.5212092 0.3902055 0.1024023 0.5452131 |
| 3 | 0 0 0 0 | 0.6299602 0.4713148 0.1236829 0.6609147 |
| 4 | 0 0 0 0 | 0.5287817 0.393364 0.1031893 0.5689851 |
| 5 | 0 0 0 0 | 0.3242668 0.231571 0.06058464 0.4098654 |
| 6 | 0 0 0 0 | 0.1942546 0.1155381 0.02982159 0.3919186 |
| 7 | 0 0 0 0 | 0.1692868 0.07814536 0.01969617 0.4838659 |
| 8 | 0 0 0 0 | 0.1493458 0.06321644 0.01577835 0.4630065 |
| 9 | 0 0 0 0 | 0.09015747 0.03760538 0.009369556 0.2830274 |
| 10 | 0 0 0 0 | 0.03375914 0.0140812 0.003508397 0.1059786 |
| 11 | 0 0 0 0 | 0.00757249 0.003158544 0.0007869661 0.02377199 |
| 12 | 0 0 0 0 | 0.0009278058 0.000386995 9.642161e-05 0.00291262 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1920685 0.1437929 0.03773584 0.2009141 |
| 1 | 0 0 0 0 | 0.5360128 0.4012882 0.1053108 0.5606984 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.917651 1.435658 0.3767623 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.118307 1.585312 0.4160268 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.931623 1.441486 0.3782143 1 |
| 5 | 0 0 0 0 | 0.5983014 0.4272693 0.1117841 0.7562385 |
| 6 | 0 0 0 0 | 0.3584172 0.2131782 0.05502352 0.7231251 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6308962 0.2770535 0.06944598 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5941032 0.2495084 0.06221724 1 |
| 9 | 0 0 0 0 | 0.1663486 0.0693853 0.01728767 0.522211 |
| 10 | 0 0 0 0 | 0.06228866 0.02598108 0.006473308 0.1955401 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05915813 0.02704681 0.1573785 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04689809 0.02193305 0.1561044 1 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.2521412 0.1887666 0.04953837 0.2637534 |
| 1 | 0 0 0 0 | 0.7036599 0.5267979 0.1382486 0.7360664 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 2.218432 1.660839 0.435857 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.481846 1.8573 0.4874021 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 2.236774 1.668489 0.4377632 1 |
| 5 | 0 0 0 0 | 0.7854304 0.560905 0.1467465 0.9927649 |
| 6 | 0 0 0 0 | 0.4705182 0.2798533 0.07223306 0.9492948 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.7285888 0.3221499 0.08081231 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6802882 0.2859895 0.07132267 1 |
| 9 | 0 0 0 0 | 0.218377 0.09108674 0.02269469 0.6855414 |
| 10 | 0 0 0 0 | 0.08177049 0.0341071 0.008497945 0.2566985 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06352809 0.02886955 0.1578326 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04743351 0.02215638 0.15616 1 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.2521412 0.1887666 0.04953837 0.2637534 |
| 1 | 0 0 0 0 | 0.7036599 0.5267979 0.1382486 0.7360664 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 2.218432 1.660839 0.435857 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.481846 1.8573 0.4874021 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 2.236774 1.668489 0.4377632 1 |
| 5 | 0 0 0 0 | 0.7854304 0.560905 0.1467465 0.9927649 |
| 6 | 0 0 0 0 | 0.4705182 0.2798533 0.07223306 0.9492948 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.7285888 0.3221499 0.08081231 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6802882 0.2859895 0.07132267 1 |
| 9 | 0 0 0 0 | 0.218377 0.09108674 0.02269469 0.6855414 |
| 10 | 0 0 0 0 | 0.08177049 0.0341071 0.008497945 0.2566985 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06352809 0.02886955 0.1578326 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04743351 0.02215638 0.15616 1 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1920685 0.1437929 0.03773584 0.2009141 |
| 1 | 0 0 0 0 | 0.5360128 0.4012882 0.1053108 0.5606984 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.917651 1.435658 0.3767623 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.118307 1.585312 0.4160268 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.931623 1.441486 0.3782143 1 |
| 5 | 0 0 0 0 | 0.5983014 0.4272693 0.1117841 0.7562385 |
| 6 | 0 0 0 0 | 0.3584172 0.2131782 0.05502352 0.7231251 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6308962 0.2770535 0.06944598 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5941032 0.2495084 0.06221724 1 |
| 9 | 0 0 0 0 | 0.1663486 0.0693853 0.01728767 0.522211 |
| 10 | 0 0 0 0 | 0.06228866 0.02598108 0.006473308 0.1955401 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05915813 0.02704681 0.1573785 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04689809 0.02193305 0.1561044 1 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1040971 0.07793274 0.02045203 0.1088912 |
| 1 | 0 0 0 0 | 0.2905077 0.2174898 0.05707625 0.3038868 |
| 2 | 0 0 0 0 | 0.5212092 0.3902055 0.1024023 0.5452131 |
| 3 | 0 0 0 0 | 0.6299602 0.4713148 0.1236829 0.6609147 |
| 4 | 0 0 0 0 | 0.5287817 0.393364 0.1031893 0.5689851 |
| 5 | 0 0 0 0 | 0.3242668 0.231571 0.06058464 0.4098654 |
| 6 | 0 0 0 0 | 0.1942546 0.1155381 0.02982159 0.3919186 |
| 7 | 0 0 0 0 | 0.1692868 0.07814536 0.01969617 0.4838659 |
| 8 | 0 0 0 0 | 0.1493458 0.06321644 0.01577835 0.4630065 |
| 9 | 0 0 0 0 | 0.09015747 0.03760538 0.009369556 0.2830274 |
| 10 | 0 0 0 0 | 0.03375914 0.0140812 0.003508397 0.1059786 |
| 11 | 0 0 0 0 | 0.00757249 0.003158544 0.0007869661 0.02377199 |
| 12 | 0 0 0 0 | 0.0009278058 0.000386995 9.642161e-05 0.00291262 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03694682 0.02766039 0.007258968 0.03864838 |
| 1 | 0 0 0 0 | 0.1031089 0.07719288 0.02025787 0.1078575 |
| 2 | 0 0 0 0 | 0.1849909 0.1384943 0.0363453 0.1935106 |
| 3 | 0 0 0 0 | 0.2235896 0.1672821 0.04389833 0.2345761 |
| 4 | 0 0 0 0 | 0.1876786 0.1396153 0.03662462 0.2019479 |
| 5 | 0 0 0 0 | 0.1150909 0.08219067 0.02150309 0.1454721 |
| 6 | 0 0 0 0 | 0.06894609 0.04100754 0.01058447 0.1391023 |
| 7 | 0 0 0 0 | 0.06008437 0.02773586 0.006990693 0.1717368 |
| 8 | 0 0 0 0 | 0.05300678 0.02243719 0.005600155 0.1643333 |
| 9 | 0 0 0 0 | 0.03199927 0.01334714 0.003325504 0.1004539 |
| 10 | 0 0 0 0 | 0.01198201 0.004997791 0.001245223 0.03761461 |
| 11 | 0 0 0 0 | 0.002687677 0.001121051 0.0002793151 0.008437308 |
| 12 | 0 0 0 0 | 0.0003293028 0.0001373548 3.422258e-05 0.001033766 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.008065841 0.006038526 0.001584702 0.008437308 |
| 1 | 0 0 0 0 | 0.02250964 0.01685194 0.004422485 0.02354631 |
| 2 | 0 0 0 0 | 0.04038528 0.03023461 0.007934525 0.04224519 |
| 3 | 0 0 0 0 | 0.04881172 0.03651927 0.009583421 0.05121019 |
| 4 | 0 0 0 0 | 0.04097202 0.03047935 0.007995502 0.04408714 |
| 5 | 0 0 0 0 | 0.02512543 0.017943 0.004694329 0.03175793 |
| 6 | 0 0 0 0 | 0.01505159 0.008952334 0.002310691 0.03036735 |
| 7 | 0 0 0 0 | 0.01311699 0.006055 0.001526135 0.03749177 |
| 8 | 0 0 0 0 | 0.01157188 0.004898251 0.001222567 0.03587551 |
| 9 | 0 0 0 0 | 0.006985744 0.002913808 0.0007259889 0.02193004 |
| 10 | 0 0 0 0 | 0.002615787 0.001091065 0.000271844 0.008211627 |
| 11 | 0 0 0 0 | 0.0005867454 0.000244736 6.097713e-05 0.001841944 |
| 12 | 0 0 0 0 | 7.188993e-05 2.998585e-05 7.471114e-06 0.0002256809 |

FX-GLOW-012: Operation Screen: the glow is screened on, which never passes white.

Frame 0:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 8 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 9 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 10 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 11 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 12 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 8 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 9 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 10 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 11 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 12 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 8 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 9 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 10 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 11 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 12 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 0.9729091 0.7975697 0.2492025 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 0.9764428 0.8145887 0.2619585 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 0.9731552 0.7982325 0.2496743 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4036874 0.1828793 0.04716002 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3936583 0.1733252 0.04436425 1 |
| 9 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 10 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05052244 0.02350067 0.1564167 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04584002 0.02149857 0.1559865 1 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 0.9782061 0.8231779 0.2684007 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 0.982845 0.8455199 0.2851463 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 0.9785291 0.8240479 0.26902 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4303165 0.1985211 0.05155605 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4171507 0.1859788 0.04788585 1 |
| 9 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 10 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05219144 0.02421429 0.15657 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04604451 0.021586 0.1560053 1 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 1 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 0.9782061 0.8231779 0.2684007 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 0.982845 0.8455199 0.2851463 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 0.9785291 0.8240479 0.26902 1 |
| 5 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 6 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4303165 0.1985211 0.05155605 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4171507 0.1859788 0.04788585 1 |
| 9 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 10 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05219144 0.02421429 0.15657 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04604451 0.021586 0.1560053 1 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 1 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 0.9729091 0.7975697 0.2492025 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 0.9764428 0.8145887 0.2619585 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 0.9731552 0.7982325 0.2496743 1 |
| 5 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 6 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4036874 0.1828793 0.04716002 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3936583 0.1733252 0.04436425 1 |
| 9 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 10 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05052244 0.02350067 0.1564167 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04584002 0.02149857 0.1559865 1 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 1 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 2 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 3 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 4 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 5 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 6 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 7 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 8 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 9 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 10 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 11 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 12 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 1 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 2 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 3 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 4 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 5 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 6 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 7 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 8 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 9 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 10 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 11 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 12 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 1 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 2 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 3 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 4 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 5 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 6 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 7 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 8 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 9 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 10 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 11 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 12 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

FX-GLOW-013: Tint #ff4000: the glow is orange, whatever the colour that lit it.

Frame 0:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 1 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 2 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 3 | 0 0 0 0 | 0.02048408 0.001050207 0 0.02048408 |
| 4 | 0 0 0 0 | 0.01763485 0.0009041294 0 0.01763485 |
| 5 | 0 0 0 0 | 0.01270317 0.0006512848 0 0.01270317 |
| 6 | 0 0 0 0 | 0.01214694 0.000622767 0 0.01214694 |
| 7 | 0 0 0 0 | 0.01499671 0.0007688732 0 0.01499671 |
| 8 | 0 0 0 0 | 0.0143502 0.0007357272 0 0.0143502 |
| 9 | 0 0 0 0 | 0.008772017 0.0004497366 0 0.008772017 |
| 10 | 0 0 0 0 | 0.003284651 0.0001684023 0 0.003284651 |
| 11 | 0 0 0 0 | 0.0007367777 3.777419e-05 0 0.0007367777 |
| 12 | 0 0 0 0 | 9.027237e-05 4.628215e-06 0 9.027237e-05 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |
| 1 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 2 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 3 | 0 0 0 0 | 0.09383044 0.004810636 0 0.09383044 |
| 4 | 0 0 0 0 | 0.08077915 0.004141503 0 0.08077915 |
| 5 | 0 0 0 0 | 0.05818883 0.00298331 0 0.05818883 |
| 6 | 0 0 0 0 | 0.05564092 0.00285268 0 0.05564092 |
| 7 | 0 0 0 0 | 0.06869472 0.003521941 0 0.06869472 |
| 8 | 0 0 0 0 | 0.06573331 0.003370111 0 0.06573331 |
| 9 | 0 0 0 0 | 0.04018157 0.002060087 0 0.04018157 |
| 10 | 0 0 0 0 | 0.01504585 0.0007713923 0 0.01504585 |
| 11 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 12 | 0 0 0 0 | 0.0004135064 2.120025e-05 0 0.0004135064 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |
| 1 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 2 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 3 | 0 0 0 0 | 0.2643659 0.01355389 0 0.2643659 |
| 4 | 0 0 0 0 | 0.227594 0.01166862 0 0.227594 |
| 5 | 0 0 0 0 | 0.1639461 0.00840543 0 0.1639461 |
| 6 | 0 0 0 0 | 0.1567674 0.008037382 0 0.1567674 |
| 7 | 0 0 0 0 | 0.1935463 0.009923016 0 0.1935463 |
| 8 | 0 0 0 0 | 0.1852026 0.009495237 0 0.1852026 |
| 9 | 0 0 0 0 | 0.113211 0.005804265 0 0.113211 |
| 10 | 0 0 0 0 | 0.04239144 0.002173386 0 0.04239144 |
| 11 | 0 0 0 0 | 0.009508795 0.0004875108 0 0.009508795 |
| 12 | 0 0 0 0 | 0.001165048 5.973138e-05 0 0.001165048 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |
| 1 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.35836 0.7363237 0.1878208 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.443752 0.7407017 0.1878208 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.375905 0.7372232 0.1878208 1 |
| 5 | 0 0 0 0 | 0.3024954 0.01550878 0 0.3024954 |
| 6 | 0 0 0 0 | 0.2892501 0.01482969 0 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6756572 0.1511772 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6602622 0.1503879 0.03310477 1 |
| 9 | 0 0 0 0 | 0.2088844 0.01070939 0 0.2088844 |
| 10 | 0 0 0 0 | 0.07821603 0.004010093 0 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06273079 0.02211851 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04733582 0.02132922 0.1559265 1 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |
| 1 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.484214 0.7427761 0.1878208 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.596313 0.7485234 0.1878208 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.507246 0.7439569 0.1878208 1 |
| 5 | 0 0 0 0 | 0.397106 0.02035941 0 0.397106 |
| 6 | 0 0 0 0 | 0.3797179 0.01946793 0 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.7873496 0.1569036 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.7671396 0.1558674 0.03310477 1 |
| 9 | 0 0 0 0 | 0.2742166 0.01405893 0 0.2742166 |
| 10 | 0 0 0 0 | 0.1026794 0.005264318 0 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06821815 0.02239985 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04800815 0.02136369 0.1559265 1 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |
| 1 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.484214 0.7427761 0.1878208 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.596313 0.7485234 0.1878208 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.507246 0.7439569 0.1878208 1 |
| 5 | 0 0 0 0 | 0.397106 0.02035941 0 0.397106 |
| 6 | 0 0 0 0 | 0.3797179 0.01946793 0 0.3797179 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.7873496 0.1569036 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.7671396 0.1558674 0.03310477 1 |
| 9 | 0 0 0 0 | 0.2742166 0.01405893 0 0.2742166 |
| 10 | 0 0 0 0 | 0.1026794 0.005264318 0 0.1026794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06821815 0.02239985 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04800815 0.02136369 0.1559265 1 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |
| 1 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.35836 0.7363237 0.1878208 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.443752 0.7407017 0.1878208 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.375905 0.7372232 0.1878208 1 |
| 5 | 0 0 0 0 | 0.3024954 0.01550878 0 0.3024954 |
| 6 | 0 0 0 0 | 0.2892501 0.01482969 0 0.2892501 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6756572 0.1511772 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6602622 0.1503879 0.03310477 1 |
| 9 | 0 0 0 0 | 0.2088844 0.01070939 0 0.2088844 |
| 10 | 0 0 0 0 | 0.07821603 0.004010093 0 0.07821603 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06273079 0.02211851 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04733582 0.02132922 0.1559265 1 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |
| 1 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 2 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 3 | 0 0 0 0 | 0.2643659 0.01355389 0 0.2643659 |
| 4 | 0 0 0 0 | 0.227594 0.01166862 0 0.227594 |
| 5 | 0 0 0 0 | 0.1639461 0.00840543 0 0.1639461 |
| 6 | 0 0 0 0 | 0.1567674 0.008037382 0 0.1567674 |
| 7 | 0 0 0 0 | 0.1935463 0.009923016 0 0.1935463 |
| 8 | 0 0 0 0 | 0.1852026 0.009495237 0 0.1852026 |
| 9 | 0 0 0 0 | 0.113211 0.005804265 0 0.113211 |
| 10 | 0 0 0 0 | 0.04239144 0.002173386 0 0.04239144 |
| 11 | 0 0 0 0 | 0.009508795 0.0004875108 0 0.009508795 |
| 12 | 0 0 0 0 | 0.001165048 5.973138e-05 0 0.001165048 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |
| 1 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 2 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 3 | 0 0 0 0 | 0.09383044 0.004810636 0 0.09383044 |
| 4 | 0 0 0 0 | 0.08077915 0.004141503 0 0.08077915 |
| 5 | 0 0 0 0 | 0.05818883 0.00298331 0 0.05818883 |
| 6 | 0 0 0 0 | 0.05564092 0.00285268 0 0.05564092 |
| 7 | 0 0 0 0 | 0.06869472 0.003521941 0 0.06869472 |
| 8 | 0 0 0 0 | 0.06573331 0.003370111 0 0.06573331 |
| 9 | 0 0 0 0 | 0.04018157 0.002060087 0 0.04018157 |
| 10 | 0 0 0 0 | 0.01504585 0.0007713923 0 0.01504585 |
| 11 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 12 | 0 0 0 0 | 0.0004135064 2.120025e-05 0 0.0004135064 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 1 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 2 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 3 | 0 0 0 0 | 0.02048408 0.001050207 0 0.02048408 |
| 4 | 0 0 0 0 | 0.01763485 0.0009041294 0 0.01763485 |
| 5 | 0 0 0 0 | 0.01270317 0.0006512848 0 0.01270317 |
| 6 | 0 0 0 0 | 0.01214694 0.000622767 0 0.01214694 |
| 7 | 0 0 0 0 | 0.01499671 0.0007688732 0 0.01499671 |
| 8 | 0 0 0 0 | 0.0143502 0.0007357272 0 0.0143502 |
| 9 | 0 0 0 0 | 0.008772017 0.0004497366 0 0.008772017 |
| 10 | 0 0 0 0 | 0.003284651 0.0001684023 0 0.003284651 |
| 11 | 0 0 0 0 | 0.0007367777 3.777419e-05 0 0.0007367777 |
| 12 | 0 0 0 0 | 9.027237e-05 4.628215e-06 0 9.027237e-05 |

FX-GLOW-014: After Effects' own defaults, threshold 60, radius 10, intensity 1, Add.

Frame 0:

Row 0, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04707526 0.03481983 0.009130763 0.05191549 |
| 1 | 0 0 0 0 | 0.05896779 0.0433626 0.01136663 0.06663251 |
| 2 | 0 0 0 0 | 0.068281 0.04978958 0.01304417 0.07981802 |
| 3 | 0 0 0 0 | 0.07327714 0.05279516 0.01382065 0.08968335 |
| 4 | 0 0 0 0 | 0.07312381 0.05180955 0.01354745 0.09502083 |
| 5 | 0 0 0 0 | 0.06812968 0.0471841 0.01231878 0.09539405 |
| 6 | 0 0 0 0 | 0.05954241 0.04001998 0.0104264 0.09105308 |
| 7 | 0 0 0 0 | 0.04904886 0.03174489 0.008247757 0.08272188 |
| 8 | 0 0 0 0 | 0.03824788 0.02365869 0.006125681 0.07142347 |
| 9 | 0 0 0 0 | 0.02831464 0.01664118 0.004291062 0.05838714 |
| 10 | 0 0 0 0 | 0.01991421 0.01108738 0.002845849 0.04495804 |
| 11 | 0 0 0 0 | 0.01328481 0.007010829 0.001790842 0.03242599 |
| 12 | 0 0 0 0 | 0.008375448 0.004206231 0.001069371 0.02179266 |
| 13 | 0 0 0 0 | 0.004866619 0.002314128 0.0005850971 0.01348313 |
| 14 | 0 0 0 0 | 0.002620117 0.001177679 0.0002959669 0.007689786 |
| 15 | 0 0 0 0 | 0.001276616 0.0005324864 0.0001326715 0.004007625 |

Row 1, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.06497467 0.04805937 0.01260255 0.0716553 |
| 1 | 0 0 0 0 | 0.08138909 0.05985035 0.01568857 0.09196817 |
| 2 | 0 0 0 0 | 0.09424347 0.06872105 0.01800395 0.1101672 |
| 3 | 0 0 0 0 | 0.1011393 0.07286944 0.01907568 0.1237836 |
| 4 | 0 0 0 0 | 0.1009277 0.07150908 0.0186986 0.1311506 |
| 5 | 0 0 0 0 | 0.09403461 0.0651249 0.01700274 0.1316657 |
| 6 | 0 0 0 0 | 0.0821822 0.05523676 0.01439083 0.1256742 |
| 7 | 0 0 0 0 | 0.06769869 0.04381524 0.0113838 0.1141752 |
| 8 | 0 0 0 0 | 0.05279085 0.03265443 0.008454847 0.09858079 |
| 9 | 0 0 0 0 | 0.03908071 0.02296865 0.005922652 0.08058767 |
| 10 | 0 0 0 0 | 0.02748618 0.01530313 0.003927926 0.06205242 |
| 11 | 0 0 0 0 | 0.01833609 0.009676555 0.002471773 0.04475532 |
| 12 | 0 0 0 0 | 0.01156004 0.005805565 0.001475978 0.03007888 |
| 13 | 0 0 0 0 | 0.006717051 0.003194028 0.0008075683 0.01860982 |
| 14 | 0 0 0 0 | 0.003616363 0.001625467 0.0004085022 0.01061367 |
| 15 | 0 0 0 0 | 0.001762023 0.0007349536 0.0001831171 0.005531443 |

Row 2, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08270484 0.06117372 0.01604151 0.09120846 |
| 1 | 0 0 0 0 | 0.1035984 0.07618221 0.01996963 0.1170643 |
| 2 | 0 0 0 0 | 0.1199605 0.08747352 0.02291683 0.1402294 |
| 3 | 0 0 0 0 | 0.128738 0.09275392 0.02428101 0.1575615 |
| 4 | 0 0 0 0 | 0.1284686 0.09102234 0.02380103 0.1669387 |
| 5 | 0 0 0 0 | 0.1196946 0.08289606 0.02164242 0.1675944 |
| 6 | 0 0 0 0 | 0.1046079 0.07030967 0.01831778 0.1599679 |
| 7 | 0 0 0 0 | 0.08617218 0.05577146 0.01449019 0.1453311 |
| 8 | 0 0 0 0 | 0.06719633 0.0415651 0.01076199 0.1254813 |
| 9 | 0 0 0 0 | 0.04974499 0.02923629 0.007538814 0.1025783 |
| 10 | 0 0 0 0 | 0.03498656 0.01947902 0.004999771 0.07898518 |
| 11 | 0 0 0 0 | 0.02333961 0.01231707 0.003146266 0.05696807 |
| 12 | 0 0 0 0 | 0.01471453 0.007389776 0.00187874 0.03828675 |
| 13 | 0 0 0 0 | 0.008549988 0.004065608 0.001027936 0.02368803 |
| 14 | 0 0 0 0 | 0.004603189 0.002069022 0.0005199736 0.01350991 |
| 15 | 0 0 0 0 | 0.002242841 0.0009355064 0.0002330858 0.007040853 |

Row 3, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.09711846 0.07183494 0.01883719 0.1071041 |
| 1 | 0 0 0 0 | 0.1216533 0.08945908 0.0234499 0.137466 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.09684 0.8184117 0.2147315 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.107148 0.8246124 0.2163334 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.106831 0.822579 0.2157698 1 |
| 5 | 0 0 0 0 | 0.1405547 0.097343 0.02541422 0.1968024 |
| 6 | 0 0 0 0 | 0.1228388 0.08256309 0.02151016 0.1878468 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4197369 0.1983595 0.05012028 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3974539 0.1816773 0.04574233 1 |
| 9 | 0 0 0 0 | 0.05841444 0.03433153 0.008852663 0.1204554 |
| 10 | 0 0 0 0 | 0.04108394 0.02287378 0.005871121 0.09275054 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.0725934 0.03568268 0.1596211 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06246515 0.02989666 0.1581326 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05522627 0.02599316 0.1571335 1 |
| 14 | 0 0 0 0 | 0.005405423 0.002429607 0.0006105935 0.01586439 |
| 15 | 0 0 0 0 | 0.002633719 0.001098544 0.0002737075 0.008267918 |

Row 4, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1052351 0.07783851 0.0204115 0.1160552 |
| 1 | 0 0 0 0 | 0.1318204 0.09693557 0.02540971 0.1489546 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.108613 0.8269963 0.2169805 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.119782 0.8337152 0.2187164 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.119439 0.8315119 0.2181056 1 |
| 5 | 0 0 0 0 | 0.1523015 0.1054784 0.02753819 0.21325 |
| 6 | 0 0 0 0 | 0.133105 0.08946325 0.02330786 0.2035459 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4281938 0.2038329 0.05154234 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4040486 0.1857565 0.04679851 1 |
| 9 | 0 0 0 0 | 0.06329639 0.03720077 0.009592518 0.1305224 |
| 10 | 0 0 0 0 | 0.04451751 0.02478545 0.006361796 0.1005021 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07488393 0.03689147 0.1599298 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06390922 0.03062189 0.158317 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05606536 0.02639216 0.1572344 1 |
| 14 | 0 0 0 0 | 0.005857178 0.00263266 0.0006616234 0.01719025 |
| 15 | 0 0 0 0 | 0.00285383 0.001190355 0.0002965824 0.008958904 |

Row 5, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1052351 0.07783851 0.0204115 0.1160552 |
| 1 | 0 0 0 0 | 0.1318204 0.09693557 0.02540971 0.1489546 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.108613 0.8269963 0.2169805 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.119782 0.8337152 0.2187164 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.119439 0.8315119 0.2181056 1 |
| 5 | 0 0 0 0 | 0.1523015 0.1054784 0.02753819 0.21325 |
| 6 | 0 0 0 0 | 0.133105 0.08946325 0.02330786 0.2035459 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4281938 0.2038329 0.05154234 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4040486 0.1857565 0.04679851 1 |
| 9 | 0 0 0 0 | 0.06329639 0.03720077 0.009592518 0.1305224 |
| 10 | 0 0 0 0 | 0.04451751 0.02478545 0.006361796 0.1005021 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07488393 0.03689147 0.1599298 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06390922 0.03062189 0.158317 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05606536 0.02639216 0.1572344 1 |
| 14 | 0 0 0 0 | 0.005857178 0.00263266 0.0006616234 0.01719025 |
| 15 | 0 0 0 0 | 0.00285383 0.001190355 0.0002965824 0.008958904 |

Row 6, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.09711846 0.07183494 0.01883719 0.1071041 |
| 1 | 0 0 0 0 | 0.1216533 0.08945908 0.0234499 0.137466 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.09684 0.8184117 0.2147315 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.107148 0.8246124 0.2163334 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.106831 0.822579 0.2157698 1 |
| 5 | 0 0 0 0 | 0.1405547 0.097343 0.02541422 0.1968024 |
| 6 | 0 0 0 0 | 0.1228388 0.08256309 0.02151016 0.1878468 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4197369 0.1983595 0.05012028 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3974539 0.1816773 0.04574233 1 |
| 9 | 0 0 0 0 | 0.05841444 0.03433153 0.008852663 0.1204554 |
| 10 | 0 0 0 0 | 0.04108394 0.02287378 0.005871121 0.09275054 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.0725934 0.03568268 0.1596211 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.06246515 0.02989666 0.1581326 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05522627 0.02599316 0.1571335 1 |
| 14 | 0 0 0 0 | 0.005405423 0.002429607 0.0006105935 0.01586439 |
| 15 | 0 0 0 0 | 0.002633719 0.001098544 0.0002737075 0.008267918 |

Row 7, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08270484 0.06117372 0.01604151 0.09120846 |
| 1 | 0 0 0 0 | 0.1035984 0.07618221 0.01996963 0.1170643 |
| 2 | 0 0 0 0 | 0.1199605 0.08747352 0.02291683 0.1402294 |
| 3 | 0 0 0 0 | 0.128738 0.09275392 0.02428101 0.1575615 |
| 4 | 0 0 0 0 | 0.1284686 0.09102234 0.02380103 0.1669387 |
| 5 | 0 0 0 0 | 0.1196946 0.08289606 0.02164242 0.1675944 |
| 6 | 0 0 0 0 | 0.1046079 0.07030967 0.01831778 0.1599679 |
| 7 | 0 0 0 0 | 0.08617218 0.05577146 0.01449019 0.1453311 |
| 8 | 0 0 0 0 | 0.06719633 0.0415651 0.01076199 0.1254813 |
| 9 | 0 0 0 0 | 0.04974499 0.02923629 0.007538814 0.1025783 |
| 10 | 0 0 0 0 | 0.03498656 0.01947902 0.004999771 0.07898518 |
| 11 | 0 0 0 0 | 0.02333961 0.01231707 0.003146266 0.05696807 |
| 12 | 0 0 0 0 | 0.01471453 0.007389776 0.00187874 0.03828675 |
| 13 | 0 0 0 0 | 0.008549988 0.004065608 0.001027936 0.02368803 |
| 14 | 0 0 0 0 | 0.004603189 0.002069022 0.0005199736 0.01350991 |
| 15 | 0 0 0 0 | 0.002242841 0.0009355064 0.0002330858 0.007040853 |

Row 8, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.06497467 0.04805937 0.01260255 0.0716553 |
| 1 | 0 0 0 0 | 0.08138909 0.05985035 0.01568857 0.09196817 |
| 2 | 0 0 0 0 | 0.09424347 0.06872105 0.01800395 0.1101672 |
| 3 | 0 0 0 0 | 0.1011393 0.07286944 0.01907568 0.1237836 |
| 4 | 0 0 0 0 | 0.1009277 0.07150908 0.0186986 0.1311506 |
| 5 | 0 0 0 0 | 0.09403461 0.0651249 0.01700274 0.1316657 |
| 6 | 0 0 0 0 | 0.0821822 0.05523676 0.01439083 0.1256742 |
| 7 | 0 0 0 0 | 0.06769869 0.04381524 0.0113838 0.1141752 |
| 8 | 0 0 0 0 | 0.05279085 0.03265443 0.008454847 0.09858079 |
| 9 | 0 0 0 0 | 0.03908071 0.02296865 0.005922652 0.08058767 |
| 10 | 0 0 0 0 | 0.02748618 0.01530313 0.003927926 0.06205242 |
| 11 | 0 0 0 0 | 0.01833609 0.009676555 0.002471773 0.04475532 |
| 12 | 0 0 0 0 | 0.01156004 0.005805565 0.001475978 0.03007888 |
| 13 | 0 0 0 0 | 0.006717051 0.003194028 0.0008075683 0.01860982 |
| 14 | 0 0 0 0 | 0.003616363 0.001625467 0.0004085022 0.01061367 |
| 15 | 0 0 0 0 | 0.001762023 0.0007349536 0.0001831171 0.005531443 |

Row 9, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.04707526 0.03481983 0.009130763 0.05191549 |
| 1 | 0 0 0 0 | 0.05896779 0.0433626 0.01136663 0.06663251 |
| 2 | 0 0 0 0 | 0.068281 0.04978958 0.01304417 0.07981802 |
| 3 | 0 0 0 0 | 0.07327714 0.05279516 0.01382065 0.08968335 |
| 4 | 0 0 0 0 | 0.07312381 0.05180955 0.01354745 0.09502083 |
| 5 | 0 0 0 0 | 0.06812968 0.0471841 0.01231878 0.09539405 |
| 6 | 0 0 0 0 | 0.05954241 0.04001998 0.0104264 0.09105308 |
| 7 | 0 0 0 0 | 0.04904886 0.03174489 0.008247757 0.08272188 |
| 8 | 0 0 0 0 | 0.03824788 0.02365869 0.006125681 0.07142347 |
| 9 | 0 0 0 0 | 0.02831464 0.01664118 0.004291062 0.05838714 |
| 10 | 0 0 0 0 | 0.01991421 0.01108738 0.002845849 0.04495804 |
| 11 | 0 0 0 0 | 0.01328481 0.007010829 0.001790842 0.03242599 |
| 12 | 0 0 0 0 | 0.008375448 0.004206231 0.001069371 0.02179266 |
| 13 | 0 0 0 0 | 0.004866619 0.002314128 0.0005850971 0.01348313 |
| 14 | 0 0 0 0 | 0.002620117 0.001177679 0.0002959669 0.007689786 |
| 15 | 0 0 0 0 | 0.001276616 0.0005324864 0.0001326715 0.004007625 |

FX-GLOW-015: The patches half covering: they glow as in FX-GLOW-001, at half the strength.

Frame 0:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001619494 0.001212441 0.0003181832 0.001694079 |
| 1 | 0 0 0 0 | 0.004519583 0.003383605 0.0008879657 0.004727729 |
| 2 | 0 0 0 0 | 0.00810873 0.006070635 0.001593128 0.008482172 |
| 3 | 0 0 0 0 | 0.009800628 0.007332496 0.001924201 0.0102822 |
| 4 | 0 0 0 0 | 0.00822654 0.006119774 0.001605371 0.008852005 |
| 5 | 0 0 0 0 | 0.005044792 0.003602674 0.0009425477 0.006376495 |
| 6 | 0 0 0 0 | 0.003022122 0.001797488 0.0004639504 0.006097288 |
| 7 | 0 0 0 0 | 0.002633685 0.001215749 0.0003064239 0.00752776 |
| 8 | 0 0 0 0 | 0.002323453 0.000983492 0.0002454722 0.00720324 |
| 9 | 0 0 0 0 | 0.001402628 0.0005850469 0.0001457672 0.004403209 |
| 10 | 0 0 0 0 | 0.000525209 0.0002190687 5.458201e-05 0.001648766 |
| 11 | 0 0 0 0 | 0.0001178093 4.913916e-05 1.224325e-05 0.0003698335 |
| 12 | 0 0 0 0 | 1.443437e-05 6.020687e-06 1.500083e-06 4.531319e-05 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.007418342 0.005553773 0.001457487 0.007759988 |
| 1 | 0 0 0 0 | 0.02070264 0.01549912 0.004067463 0.02165609 |
| 2 | 0 0 0 0 | 0.03714328 0.02780747 0.007297567 0.03885389 |
| 3 | 0 0 0 0 | 0.04489328 0.03358762 0.008814095 0.0470992 |
| 4 | 0 0 0 0 | 0.03768292 0.02803256 0.007353649 0.04054797 |
| 5 | 0 0 0 0 | 0.02310844 0.0165026 0.004317484 0.02920851 |
| 6 | 0 0 0 0 | 0.01384329 0.00823367 0.002125196 0.02792956 |
| 7 | 0 0 0 0 | 0.012064 0.005568925 0.001403622 0.03448206 |
| 8 | 0 0 0 0 | 0.01064293 0.004505036 0.001124423 0.03299554 |
| 9 | 0 0 0 0 | 0.006424952 0.002679897 0.0006677089 0.02016957 |
| 10 | 0 0 0 0 | 0.0024058 0.001003478 0.0002500212 0.007552424 |
| 11 | 0 0 0 0 | 0.0005396434 0.0002250894 5.608209e-05 0.001694079 |
| 12 | 0 0 0 0 | 6.611884e-05 2.757868e-05 6.871358e-06 0.000207564 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02090107 0.01564767 0.004106448 0.02186365 |
| 1 | 0 0 0 0 | 0.05832938 0.04366854 0.01146002 0.0610157 |
| 2 | 0 0 0 0 | 0.1046506 0.07834714 0.02056079 0.1094702 |
| 3 | 0 0 0 0 | 0.1264861 0.09463261 0.02483358 0.1327013 |
| 4 | 0 0 0 0 | 0.1061711 0.07898133 0.0207188 0.1142433 |
| 5 | 0 0 0 0 | 0.06510769 0.04649582 0.01216445 0.08229454 |
| 6 | 0 0 0 0 | 0.03900327 0.02319824 0.005987707 0.07869111 |
| 7 | 0 0 0 0 | 0.03399014 0.01569036 0.003954683 0.09715267 |
| 8 | 0 0 0 0 | 0.0299863 0.01269287 0.003168045 0.09296444 |
| 9 | 0 0 0 0 | 0.0181022 0.007550569 0.00188126 0.05682746 |
| 10 | 0 0 0 0 | 0.006778306 0.002827284 0.0007044311 0.02127884 |
| 11 | 0 0 0 0 | 0.001520437 0.0006341861 0.0001580104 0.004773042 |
| 12 | 0 0 0 0 | 0.0001862888 7.770252e-05 1.935995e-05 0.0005848084 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03856435 0.02887136 0.007576765 0.0403404 |
| 1 | 0 0 0 0 | 0.107623 0.08057238 0.02114476 0.1125795 |
| 2 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.672951 0.5038076 0.1322152 0.7039432 |
| 3 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7132394 0.5338558 0.1400989 0.7468066 |
| 4 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.6757563 0.5049777 0.1325067 0.7127499 |
| 5 | 0 0 0 0 | 0.1201295 0.08578897 0.0224445 0.1518408 |
| 6 | 0 0 0 0 | 0.07196455 0.04280284 0.01104786 0.1451922 |
| 7 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.2226128 0.09564482 0.02391404 0.6812162 |
| 8 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.2152254 0.09011418 0.02246262 0.6734885 |
| 9 | 0 0 0 0 | 0.0334002 0.01393148 0.003471094 0.1048518 |
| 10 | 0 0 0 0 | 0.01250659 0.005216593 0.001299739 0.03926138 |
| 11 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02548705 0.01182124 0.07856051 0.5107675 |
| 12 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02302542 0.01079448 0.07830469 0.5030398 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.05062601 0.03790137 0.009946528 0.05295755 |
| 1 | 0 0 0 0 | 0.1412839 0.1057728 0.02775814 0.1477906 |
| 2 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7333431 0.5490204 0.1440804 0.7671167 |
| 3 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7862325 0.5884667 0.1544299 0.8233863 |
| 4 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7370259 0.5505565 0.1444632 0.7786778 |
| 5 | 0 0 0 0 | 0.1577021 0.1126209 0.0294644 0.1993316 |
| 6 | 0 0 0 0 | 0.09447268 0.05619016 0.01450327 0.1906035 |
| 7 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.242228 0.1046995 0.02619622 0.7372814 |
| 8 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.23253 0.09743902 0.02429085 0.7271368 |
| 9 | 0 0 0 0 | 0.04384668 0.01828879 0.004556737 0.137646 |
| 10 | 0 0 0 0 | 0.01641823 0.006848172 0.001706254 0.05154104 |
| 11 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02636446 0.01218722 0.0786517 0.5135219 |
| 12 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02313293 0.01083932 0.07831586 0.5033773 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.05062601 0.03790137 0.009946528 0.05295755 |
| 1 | 0 0 0 0 | 0.1412839 0.1057728 0.02775814 0.1477906 |
| 2 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7333431 0.5490204 0.1440804 0.7671167 |
| 3 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7862325 0.5884667 0.1544299 0.8233863 |
| 4 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7370259 0.5505565 0.1444632 0.7786778 |
| 5 | 0 0 0 0 | 0.1577021 0.1126209 0.0294644 0.1993316 |
| 6 | 0 0 0 0 | 0.09447268 0.05619016 0.01450327 0.1906035 |
| 7 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.242228 0.1046995 0.02619622 0.7372814 |
| 8 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.23253 0.09743902 0.02429085 0.7271368 |
| 9 | 0 0 0 0 | 0.04384668 0.01828879 0.004556737 0.137646 |
| 10 | 0 0 0 0 | 0.01641823 0.006848172 0.001706254 0.05154104 |
| 11 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02636446 0.01218722 0.0786517 0.5135219 |
| 12 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02313293 0.01083932 0.07831586 0.5033773 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03856435 0.02887136 0.007576765 0.0403404 |
| 1 | 0 0 0 0 | 0.107623 0.08057238 0.02114476 0.1125795 |
| 2 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.672951 0.5038076 0.1322152 0.7039432 |
| 3 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.7132394 0.5338558 0.1400989 0.7468066 |
| 4 | 0.4798611 0.3592501 0.09427866 0.5019608 | 0.6757563 0.5049777 0.1325067 0.7127499 |
| 5 | 0 0 0 0 | 0.1201295 0.08578897 0.0224445 0.1518408 |
| 6 | 0 0 0 0 | 0.07196455 0.04280284 0.01104786 0.1451922 |
| 7 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.2226128 0.09564482 0.02391404 0.6812162 |
| 8 | 0.159898 0.06669469 0.01661729 0.5019608 | 0.2152254 0.09011418 0.02246262 0.6734885 |
| 9 | 0 0 0 0 | 0.0334002 0.01393148 0.003471094 0.1048518 |
| 10 | 0 0 0 0 | 0.01250659 0.005216593 0.001299739 0.03926138 |
| 11 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02548705 0.01182124 0.07856051 0.5107675 |
| 12 | 0.0226817 0.01065111 0.07826897 0.5019608 | 0.02302542 0.01079448 0.07830469 0.5030398 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02090107 0.01564767 0.004106448 0.02186365 |
| 1 | 0 0 0 0 | 0.05832938 0.04366854 0.01146002 0.0610157 |
| 2 | 0 0 0 0 | 0.1046506 0.07834714 0.02056079 0.1094702 |
| 3 | 0 0 0 0 | 0.1264861 0.09463261 0.02483358 0.1327013 |
| 4 | 0 0 0 0 | 0.1061711 0.07898133 0.0207188 0.1142433 |
| 5 | 0 0 0 0 | 0.06510769 0.04649582 0.01216445 0.08229454 |
| 6 | 0 0 0 0 | 0.03900327 0.02319824 0.005987707 0.07869111 |
| 7 | 0 0 0 0 | 0.03399014 0.01569036 0.003954683 0.09715267 |
| 8 | 0 0 0 0 | 0.0299863 0.01269287 0.003168045 0.09296444 |
| 9 | 0 0 0 0 | 0.0181022 0.007550569 0.00188126 0.05682746 |
| 10 | 0 0 0 0 | 0.006778306 0.002827284 0.0007044311 0.02127884 |
| 11 | 0 0 0 0 | 0.001520437 0.0006341861 0.0001580104 0.004773042 |
| 12 | 0 0 0 0 | 0.0001862888 7.770252e-05 1.935995e-05 0.0005848084 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.007418342 0.005553773 0.001457487 0.007759988 |
| 1 | 0 0 0 0 | 0.02070264 0.01549912 0.004067463 0.02165609 |
| 2 | 0 0 0 0 | 0.03714328 0.02780747 0.007297567 0.03885389 |
| 3 | 0 0 0 0 | 0.04489328 0.03358762 0.008814095 0.0470992 |
| 4 | 0 0 0 0 | 0.03768292 0.02803256 0.007353649 0.04054797 |
| 5 | 0 0 0 0 | 0.02310844 0.0165026 0.004317484 0.02920851 |
| 6 | 0 0 0 0 | 0.01384329 0.00823367 0.002125196 0.02792956 |
| 7 | 0 0 0 0 | 0.012064 0.005568925 0.001403622 0.03448206 |
| 8 | 0 0 0 0 | 0.01064293 0.004505036 0.001124423 0.03299554 |
| 9 | 0 0 0 0 | 0.006424952 0.002679897 0.0006677089 0.02016957 |
| 10 | 0 0 0 0 | 0.0024058 0.001003478 0.0002500212 0.007552424 |
| 11 | 0 0 0 0 | 0.0005396434 0.0002250894 5.608209e-05 0.001694079 |
| 12 | 0 0 0 0 | 6.611884e-05 2.757868e-05 6.871358e-06 0.000207564 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001619494 0.001212441 0.0003181832 0.001694079 |
| 1 | 0 0 0 0 | 0.004519583 0.003383605 0.0008879657 0.004727729 |
| 2 | 0 0 0 0 | 0.00810873 0.006070635 0.001593128 0.008482172 |
| 3 | 0 0 0 0 | 0.009800628 0.007332496 0.001924201 0.0102822 |
| 4 | 0 0 0 0 | 0.00822654 0.006119774 0.001605371 0.008852005 |
| 5 | 0 0 0 0 | 0.005044792 0.003602674 0.0009425477 0.006376495 |
| 6 | 0 0 0 0 | 0.003022122 0.001797488 0.0004639504 0.006097288 |
| 7 | 0 0 0 0 | 0.002633685 0.001215749 0.0003064239 0.00752776 |
| 8 | 0 0 0 0 | 0.002323453 0.000983492 0.0002454722 0.00720324 |
| 9 | 0 0 0 0 | 0.001402628 0.0005850469 0.0001457672 0.004403209 |
| 10 | 0 0 0 0 | 0.000525209 0.0002190687 5.458201e-05 0.001648766 |
| 11 | 0 0 0 0 | 0.0001178093 4.913916e-05 1.224325e-05 0.0003698335 |
| 12 | 0 0 0 0 | 1.443437e-05 6.020687e-06 1.500083e-06 4.531319e-05 |

FX-GLOW-016: Radius keyed from 0 at frame 0 to 8 at frame 4, linear: frame 0 is FX-GLOW-003, frame 2 is FX-GLOW-001, frame 4 is radius 8.

Frame 0: the same as FX-GLOW-003 frame 0.

Frame 2: the same as FX-GLOW-001 frame 0.

Frame 4:

Row 0, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03861559 0.02879682 0.007555318 0.04110682 |
| 1 | 0 0 0 0 | 0.05359347 0.03983036 0.01044787 0.0579091 |
| 2 | 0 0 0 0 | 0.06584961 0.04863703 0.01275285 0.07305901 |
| 3 | 0 0 0 0 | 0.07200304 0.05260034 0.01378221 0.0835584 |
| 4 | 0 0 0 0 | 0.07063072 0.05063126 0.01324973 0.08806813 |
| 5 | 0 0 0 0 | 0.06289394 0.04370966 0.01141441 0.08710566 |
| 6 | 0 0 0 0 | 0.05163359 0.03422673 0.008908205 0.08197366 |
| 7 | 0 0 0 0 | 0.03973401 0.0246744 0.006390618 0.07358974 |
| 8 | 0 0 0 0 | 0.0290034 0.01664089 0.004282369 0.06236494 |
| 9 | 0 0 0 0 | 0.02010475 0.01062253 0.00271371 0.04899278 |
| 10 | 0 0 0 0 | 0.01309573 0.006428015 0.001630522 0.03501403 |
| 11 | 0 0 0 0 | 0.007798461 0.003591629 0.0009050296 0.02234218 |
| 12 | 0 0 0 0 | 0.004200226 0.001839476 0.0004609381 0.01263298 |
| 13 | 0 0 0 0 | 0.001995385 0.0008322907 0.0002073691 0.006264026 |
| 14 | 0 0 0 0 | 0.0008825831 0.0003681322 9.172187e-05 0.002770655 |
| 15 | 0 0 0 0 | 0.0003405379 0.000142041 3.539018e-05 0.001069036 |

Row 1, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.06215594 0.04635158 0.01216109 0.06616584 |
| 1 | 0 0 0 0 | 0.08626445 0.06411124 0.01681697 0.09321092 |
| 2 | 0 0 0 0 | 0.105992 0.07828652 0.02052709 0.1175963 |
| 3 | 0 0 0 0 | 0.1158966 0.08466589 0.02218394 0.1344962 |
| 4 | 0 0 0 0 | 0.1136877 0.08149645 0.02132686 0.1417551 |
| 5 | 0 0 0 0 | 0.1012345 0.07035539 0.01837272 0.1402059 |
| 6 | 0 0 0 0 | 0.08310981 0.0550916 0.01433871 0.1319454 |
| 7 | 0 0 0 0 | 0.06395615 0.0397161 0.01028639 0.1184506 |
| 8 | 0 0 0 0 | 0.04668409 0.0267853 0.006892932 0.1003831 |
| 9 | 0 0 0 0 | 0.03236075 0.0170981 0.004368008 0.07885914 |
| 10 | 0 0 0 0 | 0.02107898 0.01034658 0.0026245 0.05635883 |
| 11 | 0 0 0 0 | 0.01255246 0.005781112 0.001456742 0.03596214 |
| 12 | 0 0 0 0 | 0.006760715 0.002960833 0.0007419294 0.02033414 |
| 13 | 0 0 0 0 | 0.003211787 0.001339661 0.0003337829 0.01008262 |
| 14 | 0 0 0 0 | 0.001420612 0.0005925483 0.0001476362 0.004459666 |
| 15 | 0 0 0 0 | 0.0005481324 0.0002286302 5.69643e-05 0.001720728 |

Row 2, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08863851 0.06610044 0.01734253 0.09435689 |
| 1 | 0 0 0 0 | 0.1230188 0.0914269 0.02398211 0.132925 |
| 2 | 0 0 0 0 | 0.1511517 0.1116418 0.029273 0.1677002 |
| 3 | 0 0 0 0 | 0.1652763 0.1207392 0.03163578 0.1918006 |
| 4 | 0 0 0 0 | 0.1621263 0.1162194 0.03041352 0.2021522 |
| 5 | 0 0 0 0 | 0.1443672 0.1003315 0.02620073 0.199943 |
| 6 | 0 0 0 0 | 0.1185201 0.07856429 0.02044796 0.1881629 |
| 7 | 0 0 0 0 | 0.09120573 0.0566378 0.01466907 0.1689184 |
| 8 | 0 0 0 0 | 0.06657463 0.03819762 0.00982978 0.1431529 |
| 9 | 0 0 0 0 | 0.04614859 0.02438303 0.00622907 0.1124584 |
| 10 | 0 0 0 0 | 0.03006002 0.01475491 0.003742712 0.08037145 |
| 11 | 0 0 0 0 | 0.01790064 0.008244251 0.002077412 0.0512844 |
| 12 | 0 0 0 0 | 0.00964123 0.004222345 0.001058041 0.02899783 |
| 13 | 0 0 0 0 | 0.004580222 0.001910446 0.0004759966 0.01437849 |
| 14 | 0 0 0 0 | 0.002025888 0.0008450134 0.0002105391 0.006359781 |
| 15 | 0 0 0 0 | 0.0007816733 0.000326042 8.123488e-05 0.002453873 |

Row 3, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1121812 0.08365694 0.02194877 0.1194184 |
| 1 | 0 0 0 0 | 0.1556931 0.1157102 0.03035185 0.1682303 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.147271 0.8569877 0.2248688 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.165148 0.8685015 0.2278591 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.161161 0.8627811 0.2263122 1 |
| 5 | 0 0 0 0 | 0.1827116 0.1269799 0.03315973 0.2530485 |
| 6 | 0 0 0 0 | 0.1499995 0.09943124 0.02587901 0.2381397 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4339771 0.2045493 0.05166999 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4028039 0.1812114 0.04554537 1 |
| 9 | 0 0 0 0 | 0.05840581 0.03085925 0.007883533 0.1423277 |
| 10 | 0 0 0 0 | 0.03804407 0.01867387 0.00473679 0.1017184 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06784132 0.03165296 0.1585556 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.05738818 0.02656283 0.1572655 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05098295 0.02363688 0.1565289 1 |
| 14 | 0 0 0 0 | 0.002563971 0.001069452 0.000266459 0.008048961 |
| 15 | 0 0 0 0 | 0.0009892884 0.0004126398 0.0001028112 0.00310563 |

Row 4, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1261603 0.09408161 0.02468385 0.1342994 |
| 1 | 0 0 0 0 | 0.1750943 0.1301291 0.03413405 0.1891939 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.17111 0.8745947 0.2294854 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.191213 0.8875432 0.2328484 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.18673 0.88111 0.2311087 1 |
| 5 | 0 0 0 0 | 0.2054797 0.1428031 0.03729184 0.2845814 |
| 6 | 0 0 0 0 | 0.1686912 0.1118216 0.02910385 0.2678148 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4483611 0.2134816 0.05398345 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4133033 0.1872355 0.04709562 1 |
| 9 | 0 0 0 0 | 0.06568389 0.03470468 0.008865917 0.1600635 |
| 10 | 0 0 0 0 | 0.04278482 0.02100086 0.005327051 0.1143937 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07066443 0.03295316 0.1588833 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.05890869 0.02722873 0.1574324 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05170529 0.02393817 0.156604 1 |
| 14 | 0 0 0 0 | 0.002883472 0.001202719 0.000299663 0.009051959 |
| 15 | 0 0 0 0 | 0.001112566 0.0004640598 0.0001156227 0.003492629 |

Row 5, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1261603 0.09408161 0.02468385 0.1342994 |
| 1 | 0 0 0 0 | 0.1750943 0.1301291 0.03413405 0.1891939 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.17111 0.8745947 0.2294854 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.191213 0.8875432 0.2328484 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.18673 0.88111 0.2311087 1 |
| 5 | 0 0 0 0 | 0.2054797 0.1428031 0.03729184 0.2845814 |
| 6 | 0 0 0 0 | 0.1686912 0.1118216 0.02910385 0.2678148 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4483611 0.2134816 0.05398345 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4133033 0.1872355 0.04709562 1 |
| 9 | 0 0 0 0 | 0.06568389 0.03470468 0.008865917 0.1600635 |
| 10 | 0 0 0 0 | 0.04278482 0.02100086 0.005327051 0.1143937 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.07066443 0.03295316 0.1588833 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.05890869 0.02722873 0.1574324 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05170529 0.02393817 0.156604 1 |
| 14 | 0 0 0 0 | 0.002883472 0.001202719 0.000299663 0.009051959 |
| 15 | 0 0 0 0 | 0.001112566 0.0004640598 0.0001156227 0.003492629 |

Row 6, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1121812 0.08365694 0.02194877 0.1194184 |
| 1 | 0 0 0 0 | 0.1556931 0.1157102 0.03035185 0.1682303 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.147271 0.8569877 0.2248688 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.165148 0.8685015 0.2278591 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.161161 0.8627811 0.2263122 1 |
| 5 | 0 0 0 0 | 0.1827116 0.1269799 0.03315973 0.2530485 |
| 6 | 0 0 0 0 | 0.1499995 0.09943124 0.02587901 0.2381397 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4339771 0.2045493 0.05166999 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4028039 0.1812114 0.04554537 1 |
| 9 | 0 0 0 0 | 0.05840581 0.03085925 0.007883533 0.1423277 |
| 10 | 0 0 0 0 | 0.03804407 0.01867387 0.00473679 0.1017184 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.06784132 0.03165296 0.1585556 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.05738818 0.02656283 0.1572655 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.05098295 0.02363688 0.1565289 1 |
| 14 | 0 0 0 0 | 0.002563971 0.001069452 0.000266459 0.008048961 |
| 15 | 0 0 0 0 | 0.0009892884 0.0004126398 0.0001028112 0.00310563 |

Row 7, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08863851 0.06610044 0.01734253 0.09435689 |
| 1 | 0 0 0 0 | 0.1230188 0.0914269 0.02398211 0.132925 |
| 2 | 0 0 0 0 | 0.1511517 0.1116418 0.029273 0.1677002 |
| 3 | 0 0 0 0 | 0.1652763 0.1207392 0.03163578 0.1918006 |
| 4 | 0 0 0 0 | 0.1621263 0.1162194 0.03041352 0.2021522 |
| 5 | 0 0 0 0 | 0.1443672 0.1003315 0.02620073 0.199943 |
| 6 | 0 0 0 0 | 0.1185201 0.07856429 0.02044796 0.1881629 |
| 7 | 0 0 0 0 | 0.09120573 0.0566378 0.01466907 0.1689184 |
| 8 | 0 0 0 0 | 0.06657463 0.03819762 0.00982978 0.1431529 |
| 9 | 0 0 0 0 | 0.04614859 0.02438303 0.00622907 0.1124584 |
| 10 | 0 0 0 0 | 0.03006002 0.01475491 0.003742712 0.08037145 |
| 11 | 0 0 0 0 | 0.01790064 0.008244251 0.002077412 0.0512844 |
| 12 | 0 0 0 0 | 0.00964123 0.004222345 0.001058041 0.02899783 |
| 13 | 0 0 0 0 | 0.004580222 0.001910446 0.0004759966 0.01437849 |
| 14 | 0 0 0 0 | 0.002025888 0.0008450134 0.0002105391 0.006359781 |
| 15 | 0 0 0 0 | 0.0007816733 0.000326042 8.123488e-05 0.002453873 |

Row 8, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.06215594 0.04635158 0.01216109 0.06616584 |
| 1 | 0 0 0 0 | 0.08626445 0.06411124 0.01681697 0.09321092 |
| 2 | 0 0 0 0 | 0.105992 0.07828652 0.02052709 0.1175963 |
| 3 | 0 0 0 0 | 0.1158966 0.08466589 0.02218394 0.1344962 |
| 4 | 0 0 0 0 | 0.1136877 0.08149645 0.02132686 0.1417551 |
| 5 | 0 0 0 0 | 0.1012345 0.07035539 0.01837272 0.1402059 |
| 6 | 0 0 0 0 | 0.08310981 0.0550916 0.01433871 0.1319454 |
| 7 | 0 0 0 0 | 0.06395615 0.0397161 0.01028639 0.1184506 |
| 8 | 0 0 0 0 | 0.04668409 0.0267853 0.006892932 0.1003831 |
| 9 | 0 0 0 0 | 0.03236075 0.0170981 0.004368008 0.07885914 |
| 10 | 0 0 0 0 | 0.02107898 0.01034658 0.0026245 0.05635883 |
| 11 | 0 0 0 0 | 0.01255246 0.005781112 0.001456742 0.03596214 |
| 12 | 0 0 0 0 | 0.006760715 0.002960833 0.0007419294 0.02033414 |
| 13 | 0 0 0 0 | 0.003211787 0.001339661 0.0003337829 0.01008262 |
| 14 | 0 0 0 0 | 0.001420612 0.0005925483 0.0001476362 0.004459666 |
| 15 | 0 0 0 0 | 0.0005481324 0.0002286302 5.69643e-05 0.001720728 |

Row 9, the 16 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.03861559 0.02879682 0.007555318 0.04110682 |
| 1 | 0 0 0 0 | 0.05359347 0.03983036 0.01044787 0.0579091 |
| 2 | 0 0 0 0 | 0.06584961 0.04863703 0.01275285 0.07305901 |
| 3 | 0 0 0 0 | 0.07200304 0.05260034 0.01378221 0.0835584 |
| 4 | 0 0 0 0 | 0.07063072 0.05063126 0.01324973 0.08806813 |
| 5 | 0 0 0 0 | 0.06289394 0.04370966 0.01141441 0.08710566 |
| 6 | 0 0 0 0 | 0.05163359 0.03422673 0.008908205 0.08197366 |
| 7 | 0 0 0 0 | 0.03973401 0.0246744 0.006390618 0.07358974 |
| 8 | 0 0 0 0 | 0.0290034 0.01664089 0.004282369 0.06236494 |
| 9 | 0 0 0 0 | 0.02010475 0.01062253 0.00271371 0.04899278 |
| 10 | 0 0 0 0 | 0.01309573 0.006428015 0.001630522 0.03501403 |
| 11 | 0 0 0 0 | 0.007798461 0.003591629 0.0009050296 0.02234218 |
| 12 | 0 0 0 0 | 0.004200226 0.001839476 0.0004609381 0.01263298 |
| 13 | 0 0 0 0 | 0.001995385 0.0008322907 0.0002073691 0.006264026 |
| 14 | 0 0 0 0 | 0.0008825831 0.0003681322 9.172187e-05 0.002770655 |
| 15 | 0 0 0 0 | 0.0003405379 0.000142041 3.539018e-05 0.001069036 |

FX-GLOW-017: Threshold keyed from 100 at frame 0 to 20 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001, frame 4 has the purple glowing too.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2: the same as FX-GLOW-001 frame 0.

Frame 4: the same as FX-GLOW-005 frame 0.

FX-GLOW-018: Intensity keyed from 0 at frame 0 to 2 at frame 4, linear: frame 0 is the drawing, frame 2 is FX-GLOW-001.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2: the same as FX-GLOW-001 frame 0.

Frame 4:

Row 0, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.006452673 0.004830821 0.001267761 0.006749846 |
| 1 | 0 0 0 0 | 0.01800771 0.01348155 0.003537988 0.01883704 |
| 2 | 0 0 0 0 | 0.03230822 0.02418769 0.00634762 0.03379615 |
| 3 | 0 0 0 0 | 0.03904938 0.02921541 0.007666737 0.04096815 |
| 4 | 0 0 0 0 | 0.03277762 0.02438348 0.006396401 0.03526971 |
| 5 | 0 0 0 0 | 0.02010034 0.0143544 0.003755463 0.02540635 |
| 6 | 0 0 0 0 | 0.01204127 0.007161867 0.001848552 0.02429388 |
| 7 | 0 0 0 0 | 0.01049359 0.004844 0.001220908 0.02999342 |
| 8 | 0 0 0 0 | 0.009257506 0.003918601 0.0009780535 0.02870041 |
| 9 | 0 0 0 0 | 0.005588596 0.002331046 0.0005807912 0.01754403 |
| 10 | 0 0 0 0 | 0.00209263 0.000872852 0.0002174752 0.006569301 |
| 11 | 0 0 0 0 | 0.0004693963 0.0001957888 4.878171e-05 0.001473555 |
| 12 | 0 0 0 0 | 5.751194e-05 2.398868e-05 5.976891e-06 0.0001805447 |

Row 1, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02955746 0.02212832 0.005807175 0.0309187 |
| 1 | 0 0 0 0 | 0.08248709 0.06175431 0.0162063 0.08628597 |
| 2 | 0 0 0 0 | 0.1479928 0.1107954 0.02907624 0.1548085 |
| 3 | 0 0 0 0 | 0.1788716 0.1338257 0.03511866 0.1876609 |
| 4 | 0 0 0 0 | 0.1501429 0.1116922 0.0292997 0.1615583 |
| 5 | 0 0 0 0 | 0.0920727 0.06575254 0.01720248 0.1163777 |
| 6 | 0 0 0 0 | 0.05515687 0.03280603 0.008467578 0.1112818 |
| 7 | 0 0 0 0 | 0.0480675 0.02218869 0.005592555 0.1373894 |
| 8 | 0 0 0 0 | 0.04240543 0.01794975 0.004480124 0.1314666 |
| 9 | 0 0 0 0 | 0.02559942 0.01067771 0.002660403 0.08036313 |
| 10 | 0 0 0 0 | 0.009585611 0.003998232 0.0009961784 0.03009169 |
| 11 | 0 0 0 0 | 0.002150142 0.0008968407 0.0002234521 0.006749846 |
| 12 | 0 0 0 0 | 0.0002634423 0.0001098838 2.737807e-05 0.0008270128 |

Row 2, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08327769 0.06234619 0.01636163 0.08711298 |
| 1 | 0 0 0 0 | 0.2324061 0.1739918 0.045661 0.2431094 |
| 2 | 0 0 0 0 | 0.4169674 0.3121644 0.08192188 0.4361705 |
| 3 | 0 0 0 0 | 0.5039682 0.3770518 0.0989463 0.5287317 |
| 4 | 0 0 0 0 | 0.4230254 0.3146912 0.08255145 0.4551881 |
| 5 | 0 0 0 0 | 0.2594134 0.1852568 0.04846772 0.3278923 |
| 6 | 0 0 0 0 | 0.1554037 0.09243049 0.02385727 0.3135349 |
| 7 | 0 0 0 0 | 0.1354294 0.06251629 0.01575694 0.3870927 |
| 8 | 0 0 0 0 | 0.1194766 0.05057316 0.01262268 0.3704052 |
| 9 | 0 0 0 0 | 0.07212597 0.0300843 0.007495645 0.2264219 |
| 10 | 0 0 0 0 | 0.02700731 0.01126496 0.002806718 0.08478288 |
| 11 | 0 0 0 0 | 0.006057992 0.002526835 0.0006295729 0.01901759 |
| 12 | 0 0 0 0 | 0.0007422446 0.000309596 7.713729e-05 0.002330096 |

Row 3, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1536548 0.1150343 0.03018867 0.1607313 |
| 1 | 0 0 0 0 | 0.4288102 0.3210306 0.08424865 0.4485587 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.725316 1.291665 0.338974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.88584 1.411388 0.3703856 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.736493 1.296327 0.3401356 1 |
| 5 | 0 0 0 0 | 0.4786411 0.3418154 0.0894273 0.6049908 |
| 6 | 0 0 0 0 | 0.2867337 0.1705426 0.04401881 0.5785001 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.5684263 0.2482165 0.06217774 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5389919 0.2261803 0.05639475 1 |
| 9 | 0 0 0 0 | 0.1330789 0.05550824 0.01383014 0.4177688 |
| 10 | 0 0 0 0 | 0.04983093 0.02078486 0.005178647 0.1564321 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05636374 0.02588125 0.1570881 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04655571 0.02179024 0.1560688 1 |

Row 4, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.201713 0.1510133 0.0396307 0.2110027 |
| 1 | 0 0 0 0 | 0.5629279 0.4214384 0.1105988 0.5888531 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.965941 1.47181 0.3862498 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.176672 1.628978 0.4274858 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.980614 1.47793 0.3877747 1 |
| 5 | 0 0 0 0 | 0.6283443 0.448724 0.1173972 0.794212 |
| 6 | 0 0 0 0 | 0.3764146 0.2238827 0.05778645 0.7594359 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6465804 0.2842936 0.0712708 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6079399 0.2553653 0.06367909 1 |
| 9 | 0 0 0 0 | 0.1747016 0.07286939 0.01815575 0.5484331 |
| 10 | 0 0 0 0 | 0.0654164 0.02728568 0.006798356 0.2053588 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05985971 0.02733944 0.1574514 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04698405 0.02196891 0.1561133 1 |

Row 5, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.201713 0.1510133 0.0396307 0.2110027 |
| 1 | 0 0 0 0 | 0.5629279 0.4214384 0.1105988 0.5888531 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.965941 1.47181 0.3862498 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 2.176672 1.628978 0.4274858 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.980614 1.47793 0.3877747 1 |
| 5 | 0 0 0 0 | 0.6283443 0.448724 0.1173972 0.794212 |
| 6 | 0 0 0 0 | 0.3764146 0.2238827 0.05778645 0.7594359 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.6465804 0.2842936 0.0712708 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.6079399 0.2553653 0.06367909 1 |
| 9 | 0 0 0 0 | 0.1747016 0.07286939 0.01815575 0.5484331 |
| 10 | 0 0 0 0 | 0.0654164 0.02728568 0.006798356 0.2053588 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05985971 0.02733944 0.1574514 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04698405 0.02196891 0.1561133 1 |

Row 6, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.1536548 0.1150343 0.03018867 0.1607313 |
| 1 | 0 0 0 0 | 0.4288102 0.3210306 0.08424865 0.4485587 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.725316 1.291665 0.338974 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.88584 1.411388 0.3703856 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.736493 1.296327 0.3401356 1 |
| 5 | 0 0 0 0 | 0.4786411 0.3418154 0.0894273 0.6049908 |
| 6 | 0 0 0 0 | 0.2867337 0.1705426 0.04401881 0.5785001 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.5684263 0.2482165 0.06217774 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5389919 0.2261803 0.05639475 1 |
| 9 | 0 0 0 0 | 0.1330789 0.05550824 0.01383014 0.4177688 |
| 10 | 0 0 0 0 | 0.04983093 0.02078486 0.005178647 0.1564321 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.05636374 0.02588125 0.1570881 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.04655571 0.02179024 0.1560688 1 |

Row 7, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.08327769 0.06234619 0.01636163 0.08711298 |
| 1 | 0 0 0 0 | 0.2324061 0.1739918 0.045661 0.2431094 |
| 2 | 0 0 0 0 | 0.4169674 0.3121644 0.08192188 0.4361705 |
| 3 | 0 0 0 0 | 0.5039682 0.3770518 0.0989463 0.5287317 |
| 4 | 0 0 0 0 | 0.4230254 0.3146912 0.08255145 0.4551881 |
| 5 | 0 0 0 0 | 0.2594134 0.1852568 0.04846772 0.3278923 |
| 6 | 0 0 0 0 | 0.1554037 0.09243049 0.02385727 0.3135349 |
| 7 | 0 0 0 0 | 0.1354294 0.06251629 0.01575694 0.3870927 |
| 8 | 0 0 0 0 | 0.1194766 0.05057316 0.01262268 0.3704052 |
| 9 | 0 0 0 0 | 0.07212597 0.0300843 0.007495645 0.2264219 |
| 10 | 0 0 0 0 | 0.02700731 0.01126496 0.002806718 0.08478288 |
| 11 | 0 0 0 0 | 0.006057992 0.002526835 0.0006295729 0.01901759 |
| 12 | 0 0 0 0 | 0.0007422446 0.000309596 7.713729e-05 0.002330096 |

Row 8, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02955746 0.02212832 0.005807175 0.0309187 |
| 1 | 0 0 0 0 | 0.08248709 0.06175431 0.0162063 0.08628597 |
| 2 | 0 0 0 0 | 0.1479928 0.1107954 0.02907624 0.1548085 |
| 3 | 0 0 0 0 | 0.1788716 0.1338257 0.03511866 0.1876609 |
| 4 | 0 0 0 0 | 0.1501429 0.1116922 0.0292997 0.1615583 |
| 5 | 0 0 0 0 | 0.0920727 0.06575254 0.01720248 0.1163777 |
| 6 | 0 0 0 0 | 0.05515687 0.03280603 0.008467578 0.1112818 |
| 7 | 0 0 0 0 | 0.0480675 0.02218869 0.005592555 0.1373894 |
| 8 | 0 0 0 0 | 0.04240543 0.01794975 0.004480124 0.1314666 |
| 9 | 0 0 0 0 | 0.02559942 0.01067771 0.002660403 0.08036313 |
| 10 | 0 0 0 0 | 0.009585611 0.003998232 0.0009961784 0.03009169 |
| 11 | 0 0 0 0 | 0.002150142 0.0008968407 0.0002234521 0.006749846 |
| 12 | 0 0 0 0 | 0.0002634423 0.0001098838 2.737807e-05 0.0008270128 |

Row 9, the 13 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.006452673 0.004830821 0.001267761 0.006749846 |
| 1 | 0 0 0 0 | 0.01800771 0.01348155 0.003537988 0.01883704 |
| 2 | 0 0 0 0 | 0.03230822 0.02418769 0.00634762 0.03379615 |
| 3 | 0 0 0 0 | 0.03904938 0.02921541 0.007666737 0.04096815 |
| 4 | 0 0 0 0 | 0.03277762 0.02438348 0.006396401 0.03526971 |
| 5 | 0 0 0 0 | 0.02010034 0.0143544 0.003755463 0.02540635 |
| 6 | 0 0 0 0 | 0.01204127 0.007161867 0.001848552 0.02429388 |
| 7 | 0 0 0 0 | 0.01049359 0.004844 0.001220908 0.02999342 |
| 8 | 0 0 0 0 | 0.009257506 0.003918601 0.0009780535 0.02870041 |
| 9 | 0 0 0 0 | 0.005588596 0.002331046 0.0005807912 0.01754403 |
| 10 | 0 0 0 0 | 0.00209263 0.000872852 0.0002174752 0.006569301 |
| 11 | 0 0 0 0 | 0.0004693963 0.0001957888 4.878171e-05 0.001473555 |
| 12 | 0 0 0 0 | 5.751194e-05 2.398868e-05 5.976891e-06 0.0001805447 |

FX-GLOW-019: FX-GLOW-001 moved three pixels right: the glow is done on the drawing before it is moved, and the part of it that spread past the drawing's left edge now shows in columns 1 and 2.

Frame 0:

Row 0, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 8.629798e-05 6.460735e-05 1.695503e-05 9.027237e-05 |
| 2 | 0 0 0 0 | 0.0007043398 0.000527307 0.0001383822 0.0007367777 |
| 3 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 4 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 5 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 6 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 7 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 8 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 9 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 10 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 11 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 12 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 13 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 14 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 15 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

Row 1, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.0003953011 0.0002959439 7.766509e-05 0.0004135064 |
| 2 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 3 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 4 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 5 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 6 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 7 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 8 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 9 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 10 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 11 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 12 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 13 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 14 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 15 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 2, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.001113755 0.0008338173 0.0002188202 0.001165048 |
| 2 | 0 0 0 0 | 0.009090154 0.006805383 0.001785949 0.009508795 |
| 3 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 4 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 5 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 6 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 7 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 8 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 9 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 10 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 11 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 12 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 13 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 14 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 15 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 3, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.002054978 0.001538468 0.000403743 0.002149619 |
| 2 | 0 0 0 0 | 0.01677215 0.01255654 0.003295237 0.01754458 |
| 3 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 4 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 5 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 6 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 7 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 8 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 9 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 10 | 0.3185468 0.1328683 0.03310477 1 | 0.4434865 0.1905424 0.04764125 1 |
| 11 | 0.3185468 0.1328683 0.03310477 1 | 0.4287694 0.1795243 0.04474976 1 |
| 12 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 13 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 14 | 0.0451862 0.02121901 0.1559265 1 | 0.05077497 0.02355013 0.1565073 1 |
| 15 | 0.0451862 0.02121901 0.1559265 1 | 0.04587096 0.02150463 0.1559976 1 |

Row 4, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.002697707 0.00201965 0.0005300205 0.002821948 |
| 2 | 0 0 0 0 | 0.02201793 0.01648382 0.004325878 0.02303195 |
| 3 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 4 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 5 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 6 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 7 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 8 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 9 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 10 | 0.3185468 0.1328683 0.03310477 1 | 0.4825636 0.208581 0.05218778 1 |
| 11 | 0.3185468 0.1328683 0.03310477 1 | 0.4632433 0.1941168 0.04839193 1 |
| 12 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 13 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 14 | 0.0451862 0.02121901 0.1559265 1 | 0.05252296 0.02427923 0.1566889 1 |
| 15 | 0.0451862 0.02121901 0.1559265 1 | 0.04608513 0.02159396 0.1560199 1 |

Row 5, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.002697707 0.00201965 0.0005300205 0.002821948 |
| 2 | 0 0 0 0 | 0.02201793 0.01648382 0.004325878 0.02303195 |
| 3 | 0 0 0 0 | 0.1008565 0.07550664 0.01981535 0.1055014 |
| 4 | 0 0 0 0 | 0.2814639 0.2107192 0.05529942 0.2944266 |
| 5 | 0.9559734 0.7156935 0.1878208 1 | 1.460957 1.093752 0.2870353 1 |
| 6 | 0.9559734 0.7156935 0.1878208 1 | 1.566322 1.172336 0.3076533 1 |
| 7 | 0.9559734 0.7156935 0.1878208 1 | 1.468294 1.096812 0.2877977 1 |
| 8 | 0 0 0 0 | 0.3141721 0.224362 0.0586986 0.397106 |
| 9 | 0 0 0 0 | 0.1882073 0.1119413 0.02889322 0.3797179 |
| 10 | 0.3185468 0.1328683 0.03310477 1 | 0.4825636 0.208581 0.05218778 1 |
| 11 | 0.3185468 0.1328683 0.03310477 1 | 0.4632433 0.1941168 0.04839193 1 |
| 12 | 0 0 0 0 | 0.0873508 0.03643469 0.009077875 0.2742166 |
| 13 | 0 0 0 0 | 0.0327082 0.01364284 0.003399178 0.1026794 |
| 14 | 0.0451862 0.02121901 0.1559265 1 | 0.05252296 0.02427923 0.1566889 1 |
| 15 | 0.0451862 0.02121901 0.1559265 1 | 0.04608513 0.02159396 0.1560199 1 |

Row 6, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.002054978 0.001538468 0.000403743 0.002149619 |
| 2 | 0 0 0 0 | 0.01677215 0.01255654 0.003295237 0.01754458 |
| 3 | 0 0 0 0 | 0.07682741 0.05751717 0.01509434 0.08036564 |
| 4 | 0 0 0 0 | 0.2144051 0.1605153 0.04212433 0.2242794 |
| 5 | 0.9559734 0.7156935 0.1878208 1 | 1.340645 1.003679 0.2633974 1 |
| 6 | 0.9559734 0.7156935 0.1878208 1 | 1.420907 1.063541 0.2791032 1 |
| 7 | 0.9559734 0.7156935 0.1878208 1 | 1.346233 1.00601 0.2639782 1 |
| 8 | 0 0 0 0 | 0.2393206 0.1709077 0.04471365 0.3024954 |
| 9 | 0 0 0 0 | 0.1433669 0.08527129 0.02200941 0.2892501 |
| 10 | 0.3185468 0.1328683 0.03310477 1 | 0.4434865 0.1905424 0.04764125 1 |
| 11 | 0.3185468 0.1328683 0.03310477 1 | 0.4287694 0.1795243 0.04474976 1 |
| 12 | 0 0 0 0 | 0.06653946 0.02775412 0.00691507 0.2088844 |
| 13 | 0 0 0 0 | 0.02491546 0.01039243 0.002589323 0.07821603 |
| 14 | 0.0451862 0.02121901 0.1559265 1 | 0.05077497 0.02355013 0.1565073 1 |
| 15 | 0.0451862 0.02121901 0.1559265 1 | 0.04587096 0.02150463 0.1559976 1 |

Row 7, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.001113755 0.0008338173 0.0002188202 0.001165048 |
| 2 | 0 0 0 0 | 0.009090154 0.006805383 0.001785949 0.009508795 |
| 3 | 0 0 0 0 | 0.04163884 0.0311731 0.008180814 0.04355649 |
| 4 | 0 0 0 0 | 0.1162031 0.08699591 0.0228305 0.1215547 |
| 5 | 0 0 0 0 | 0.2084837 0.1560822 0.04096094 0.2180852 |
| 6 | 0 0 0 0 | 0.2519841 0.1885259 0.04947315 0.2643659 |
| 7 | 0 0 0 0 | 0.2115127 0.1573456 0.04127573 0.227594 |
| 8 | 0 0 0 0 | 0.1297067 0.09262839 0.02423386 0.1639461 |
| 9 | 0 0 0 0 | 0.07770183 0.04621525 0.01192864 0.1567674 |
| 10 | 0 0 0 0 | 0.06771472 0.03125814 0.007878469 0.1935463 |
| 11 | 0 0 0 0 | 0.05973832 0.02528658 0.00631134 0.1852026 |
| 12 | 0 0 0 0 | 0.03606299 0.01504215 0.003747822 0.113211 |
| 13 | 0 0 0 0 | 0.01350366 0.00563248 0.001403359 0.04239144 |
| 14 | 0 0 0 0 | 0.003028996 0.001263418 0.0003147864 0.009508795 |
| 15 | 0 0 0 0 | 0.0003711223 0.000154798 3.856864e-05 0.001165048 |

Row 8, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.0003953011 0.0002959439 7.766509e-05 0.0004135064 |
| 2 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 3 | 0 0 0 0 | 0.01477873 0.01106416 0.002903587 0.01545935 |
| 4 | 0 0 0 0 | 0.04124354 0.03087715 0.008103149 0.04314298 |
| 5 | 0 0 0 0 | 0.07399638 0.0553977 0.01453812 0.07740423 |
| 6 | 0 0 0 0 | 0.08943582 0.06691284 0.01755933 0.09383044 |
| 7 | 0 0 0 0 | 0.07507145 0.05584612 0.01464985 0.08077915 |
| 8 | 0 0 0 0 | 0.04603635 0.03287627 0.008601238 0.05818883 |
| 9 | 0 0 0 0 | 0.02757844 0.01640301 0.004233789 0.05564092 |
| 10 | 0 0 0 0 | 0.02403375 0.01109434 0.002796277 0.06869472 |
| 11 | 0 0 0 0 | 0.02120271 0.008974876 0.002240062 0.06573331 |
| 12 | 0 0 0 0 | 0.01279971 0.005338857 0.001330201 0.04018157 |
| 13 | 0 0 0 0 | 0.004792806 0.001999116 0.0004980892 0.01504585 |
| 14 | 0 0 0 0 | 0.001075071 0.0004484204 0.000111726 0.003374923 |
| 15 | 0 0 0 0 | 0.0001317211 5.49419e-05 1.368903e-05 0.0004135064 |

Row 9, the 15 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 1 | 0 0 0 0 | 8.629798e-05 6.460735e-05 1.695503e-05 9.027237e-05 |
| 2 | 0 0 0 0 | 0.0007043398 0.000527307 0.0001383822 0.0007367777 |
| 3 | 0 0 0 0 | 0.003226336 0.00241541 0.0006338806 0.003374923 |
| 4 | 0 0 0 0 | 0.009003856 0.006740775 0.001768994 0.009418522 |
| 5 | 0 0 0 0 | 0.01615411 0.01209384 0.00317381 0.01689808 |
| 6 | 0 0 0 0 | 0.01952469 0.01460771 0.003833368 0.02048408 |
| 7 | 0 0 0 0 | 0.01638881 0.01219174 0.003198201 0.01763485 |
| 8 | 0 0 0 0 | 0.01005017 0.007177201 0.001877732 0.01270317 |
| 9 | 0 0 0 0 | 0.006020634 0.003580934 0.0009242762 0.01214694 |
| 10 | 0 0 0 0 | 0.005246795 0.002422 0.0006104539 0.01499671 |
| 11 | 0 0 0 0 | 0.004628753 0.0019593 0.0004890267 0.0143502 |
| 12 | 0 0 0 0 | 0.002794298 0.001165523 0.0002903956 0.008772017 |
| 13 | 0 0 0 0 | 0.001046315 0.000436426 0.0001087376 0.003284651 |
| 14 | 0 0 0 0 | 0.0002346982 9.789441e-05 2.439085e-05 0.0007367777 |
| 15 | 0 0 0 0 | 2.875597e-05 1.199434e-05 2.988446e-06 9.027237e-05 |

Frame 3: the same as FX-GLOW-019 frame 0.

FX-GLOW-020: Radius 2.5: a radius is not rounded, so this is neither radius 2 nor 3.

Frame 0:

Row 0, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 1.937964e-05 1.450865e-05 3.807532e-06 2.027216e-05 |
| 1 | 0 0 0 0 | 0.0001829537 0.0001369691 3.594505e-05 0.0001913795 |
| 2 | 0 0 0 0 | 0.0005184903 0.00038817 0.0001018682 0.000542369 |
| 3 | 0 0 0 0 | 0.0006632002 0.0004965076 0.0001302994 0.0006937434 |
| 4 | 0 0 0 0 | 0.000518662 0.0003882416 0.000101886 0.0005429082 |
| 5 | 0 0 0 0 | 0.0001894113 0.0001396626 3.661615e-05 0.0002116517 |
| 6 | 0 0 0 0 | 8.017121e-05 3.986529e-05 1.012526e-05 0.0002111125 |
| 7 | 0 0 0 0 | 0.0001669995 6.982765e-05 1.740301e-05 0.0005231752 |
| 8 | 0 0 0 0 | 0.000166484 6.944177e-05 1.730174e-05 0.000522636 |
| 9 | 0 0 0 0 | 6.079157e-05 2.535664e-05 6.317725e-06 0.0001908403 |
| 10 | 0 0 0 0 | 6.45763e-06 2.693527e-06 6.71105e-07 2.027216e-05 |
| 11 | 0 0 0 0 | 1.717535e-07 7.163973e-08 1.784937e-08 5.391784e-07 |

Row 1, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0007286403 0.0005454996 0.0001431565 0.0007621972 |
| 1 | 0 0 0 0 | 0.006878737 0.005149795 0.00135147 0.007195532 |
| 2 | 0 0 0 0 | 0.01949432 0.0145945 0.003830063 0.02039212 |
| 3 | 0 0 0 0 | 0.02493516 0.01866781 0.004899028 0.02608353 |
| 4 | 0 0 0 0 | 0.01950078 0.0145972 0.003830734 0.02041239 |
| 5 | 0 0 0 0 | 0.007121532 0.005251067 0.001376703 0.007957729 |
| 6 | 0 0 0 0 | 0.003014296 0.001498864 0.0003806918 0.007937457 |
| 7 | 0 0 0 0 | 0.006278885 0.002625397 0.0006543225 0.01967046 |
| 8 | 0 0 0 0 | 0.006259505 0.002610888 0.000650515 0.01965019 |
| 9 | 0 0 0 0 | 0.002285656 0.0009533647 0.0002375353 0.00717526 |
| 10 | 0 0 0 0 | 0.0002427955 0.0001012719 2.523236e-05 0.0007621972 |
| 11 | 0 0 0 0 | 6.45763e-06 2.693527e-06 6.71105e-07 2.027216e-05 |

Row 2, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.006878737 0.005149795 0.00135147 0.007195532 |
| 1 | 0 0 0 0 | 0.06493879 0.0486167 0.01275857 0.0679295 |
| 2 | 0 0 0 0 | 0.1840363 0.1377796 0.03615775 0.192512 |
| 3 | 0 0 0 0 | 0.2354006 0.1762337 0.04624933 0.2462418 |
| 4 | 0 0 0 0 | 0.1840973 0.137805 0.03616409 0.1927034 |
| 5 | 0 0 0 0 | 0.06723091 0.04957276 0.01299678 0.07512503 |
| 6 | 0 0 0 0 | 0.0284565 0.01415005 0.003593925 0.07493365 |
| 7 | 0 0 0 0 | 0.05927588 0.02478509 0.006177139 0.1856992 |
| 8 | 0 0 0 0 | 0.05909293 0.02464812 0.006141194 0.1855079 |
| 9 | 0 0 0 0 | 0.02157776 0.009000251 0.002242455 0.06773812 |
| 10 | 0 0 0 0 | 0.002292113 0.0009560582 0.0002382064 0.007195532 |
| 11 | 0 0 0 0 | 6.096333e-05 2.542827e-05 6.335574e-06 0.0001913795 |

Row 3, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0195137 0.01460901 0.003833871 0.02041239 |
| 1 | 0 0 0 0 | 0.1842193 0.1379166 0.0361937 0.1927034 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.47805 1.106549 0.2903936 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.623761 1.215636 0.3190215 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.478223 1.106621 0.2904116 1 |
| 5 | 0 0 0 0 | 0.1907216 0.1406287 0.03686945 0.2131158 |
| 6 | 0 0 0 0 | 0.0807258 0.04014105 0.0101953 0.2125729 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4867015 0.203179 0.05062816 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4861824 0.2027905 0.05052619 1 |
| 9 | 0 0 0 0 | 0.0612121 0.02553204 0.006361428 0.1921605 |
| 10 | 0 0 0 0 | 0.0065023 0.00271216 0.0006757473 0.02041239 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.04535915 0.02129115 0.1559444 1 |

Row 4, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02564442 0.0191988 0.005038377 0.02682545 |
| 1 | 0 0 0 0 | 0.2420964 0.1812465 0.04756485 0.253246 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.642074 1.229346 0.3226194 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.833563 1.372705 0.3602415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.642301 1.22944 0.322643 1 |
| 5 | 0 0 0 0 | 0.2506416 0.1848108 0.0484529 0.2800714 |
| 6 | 0 0 0 0 | 0.1060878 0.05275237 0.01339841 0.2793579 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.5395315 0.2252689 0.05613358 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5388494 0.2247582 0.05599957 1 |
| 9 | 0 0 0 0 | 0.08044341 0.03355357 0.008360029 0.2525325 |
| 10 | 0 0 0 0 | 0.008545161 0.003564253 0.0008880503 0.02682545 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.04541348 0.02131381 0.1559501 1 |

Row 5, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.02564442 0.0191988 0.005038377 0.02682545 |
| 1 | 0 0 0 0 | 0.2420964 0.1812465 0.04756485 0.253246 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.642074 1.229346 0.3226194 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.833563 1.372705 0.3602415 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.642301 1.22944 0.322643 1 |
| 5 | 0 0 0 0 | 0.2506416 0.1848108 0.0484529 0.2800714 |
| 6 | 0 0 0 0 | 0.1060878 0.05275237 0.01339841 0.2793579 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.5395315 0.2252689 0.05613358 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.5388494 0.2247582 0.05599957 1 |
| 9 | 0 0 0 0 | 0.08044341 0.03355357 0.008360029 0.2525325 |
| 10 | 0 0 0 0 | 0.008545161 0.003564253 0.0008880503 0.02682545 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.04541348 0.02131381 0.1559501 1 |

Row 6, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0195137 0.01460901 0.003833871 0.02041239 |
| 1 | 0 0 0 0 | 0.1842193 0.1379166 0.0361937 0.1927034 |
| 2 | 0.9559734 0.7156935 0.1878208 1 | 1.47805 1.106549 0.2903936 1 |
| 3 | 0.9559734 0.7156935 0.1878208 1 | 1.623761 1.215636 0.3190215 1 |
| 4 | 0.9559734 0.7156935 0.1878208 1 | 1.478223 1.106621 0.2904116 1 |
| 5 | 0 0 0 0 | 0.1907216 0.1406287 0.03686945 0.2131158 |
| 6 | 0 0 0 0 | 0.0807258 0.04014105 0.0101953 0.2125729 |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.4867015 0.203179 0.05062816 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.4861824 0.2027905 0.05052619 1 |
| 9 | 0 0 0 0 | 0.0612121 0.02553204 0.006361428 0.1921605 |
| 10 | 0 0 0 0 | 0.0065023 0.00271216 0.0006757473 0.02041239 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.04535915 0.02129115 0.1559444 1 |

Row 7, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.006878737 0.005149795 0.00135147 0.007195532 |
| 1 | 0 0 0 0 | 0.06493879 0.0486167 0.01275857 0.0679295 |
| 2 | 0 0 0 0 | 0.1840363 0.1377796 0.03615775 0.192512 |
| 3 | 0 0 0 0 | 0.2354006 0.1762337 0.04624933 0.2462418 |
| 4 | 0 0 0 0 | 0.1840973 0.137805 0.03616409 0.1927034 |
| 5 | 0 0 0 0 | 0.06723091 0.04957276 0.01299678 0.07512503 |
| 6 | 0 0 0 0 | 0.0284565 0.01415005 0.003593925 0.07493365 |
| 7 | 0 0 0 0 | 0.05927588 0.02478509 0.006177139 0.1856992 |
| 8 | 0 0 0 0 | 0.05909293 0.02464812 0.006141194 0.1855079 |
| 9 | 0 0 0 0 | 0.02157776 0.009000251 0.002242455 0.06773812 |
| 10 | 0 0 0 0 | 0.002292113 0.0009560582 0.0002382064 0.007195532 |
| 11 | 0 0 0 0 | 6.096333e-05 2.542827e-05 6.335574e-06 0.0001913795 |

Row 8, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0007286403 0.0005454996 0.0001431565 0.0007621972 |
| 1 | 0 0 0 0 | 0.006878737 0.005149795 0.00135147 0.007195532 |
| 2 | 0 0 0 0 | 0.01949432 0.0145945 0.003830063 0.02039212 |
| 3 | 0 0 0 0 | 0.02493516 0.01866781 0.004899028 0.02608353 |
| 4 | 0 0 0 0 | 0.01950078 0.0145972 0.003830734 0.02041239 |
| 5 | 0 0 0 0 | 0.007121532 0.005251067 0.001376703 0.007957729 |
| 6 | 0 0 0 0 | 0.003014296 0.001498864 0.0003806918 0.007937457 |
| 7 | 0 0 0 0 | 0.006278885 0.002625397 0.0006543225 0.01967046 |
| 8 | 0 0 0 0 | 0.006259505 0.002610888 0.000650515 0.01965019 |
| 9 | 0 0 0 0 | 0.002285656 0.0009533647 0.0002375353 0.00717526 |
| 10 | 0 0 0 0 | 0.0002427955 0.0001012719 2.523236e-05 0.0007621972 |
| 11 | 0 0 0 0 | 6.45763e-06 2.693527e-06 6.71105e-07 2.027216e-05 |

Row 9, the 12 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 0 | 0 0 0 0 | 1.937964e-05 1.450865e-05 3.807532e-06 2.027216e-05 |
| 1 | 0 0 0 0 | 0.0001829537 0.0001369691 3.594505e-05 0.0001913795 |
| 2 | 0 0 0 0 | 0.0005184903 0.00038817 0.0001018682 0.000542369 |
| 3 | 0 0 0 0 | 0.0006632002 0.0004965076 0.0001302994 0.0006937434 |
| 4 | 0 0 0 0 | 0.000518662 0.0003882416 0.000101886 0.0005429082 |
| 5 | 0 0 0 0 | 0.0001894113 0.0001396626 3.661615e-05 0.0002116517 |
| 6 | 0 0 0 0 | 8.017121e-05 3.986529e-05 1.012526e-05 0.0002111125 |
| 7 | 0 0 0 0 | 0.0001669995 6.982765e-05 1.740301e-05 0.0005231752 |
| 8 | 0 0 0 0 | 0.000166484 6.944177e-05 1.730174e-05 0.000522636 |
| 9 | 0 0 0 0 | 6.079157e-05 2.535664e-05 6.317725e-06 0.0001908403 |
| 10 | 0 0 0 0 | 6.45763e-06 2.693527e-06 6.71105e-07 2.027216e-05 |
| 11 | 0 0 0 0 | 1.717535e-07 7.163973e-08 1.784937e-08 5.391784e-07 |

FX-GLOW-021: FX-GLOW-007 and FX-GLOW-013 with the colours written in capitals: the same.

Frame 0:

Row 0, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 9.027237e-05 4.628215e-06 0 9.027237e-05 |
| 8 | 0 0 0 0 | 0.0007367777 3.777419e-05 0 0.0007367777 |
| 9 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 10 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 11 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 12 | 0 0 0 0 | 0.0203938 0.001045579 0 0.0203938 |
| 13 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 14 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 15 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |

Row 1, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 0.0004135064 2.120025e-05 0 0.0004135064 |
| 8 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 9 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |
| 10 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 11 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 12 | 0 0 0 0 | 0.09341694 0.004789436 0 0.09341694 |
| 13 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 14 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 15 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |

Row 2, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 0.001165048 5.973138e-05 0 0.001165048 |
| 8 | 0 0 0 0 | 0.009508795 0.0004875108 0 0.009508795 |
| 9 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |
| 10 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 11 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 12 | 0 0 0 0 | 0.2632008 0.01349416 0 0.2632008 |
| 13 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 14 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 15 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |

Row 3, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3206964 0.1329785 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3360914 0.1337678 0.03310477 1 |
| 9 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |
| 10 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.4475731 0.04184917 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.5308154 0.04611696 0.1559265 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.4475731 0.04184917 0.1559265 1 |
| 14 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 15 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |

Row 4, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3213687 0.133013 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3415787 0.1340492 0.03310477 1 |
| 9 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |
| 10 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.5734265 0.0483016 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.6827042 0.05390421 0.1559265 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.5734265 0.0483016 0.1559265 1 |
| 14 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 15 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |

Row 5, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3213687 0.133013 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3415787 0.1340492 0.03310477 1 |
| 9 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |
| 10 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.5734265 0.0483016 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.6827042 0.05390421 0.1559265 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.5734265 0.0483016 0.1559265 1 |
| 14 | 0 0 0 0 | 0.2944266 0.01509509 0 0.2944266 |
| 15 | 0 0 0 0 | 0.1055014 0.005408998 0 0.1055014 |

Row 6, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0.3185468 0.1328683 0.03310477 1 | 0.3206964 0.1329785 0.03310477 1 |
| 8 | 0.3185468 0.1328683 0.03310477 1 | 0.3360914 0.1337678 0.03310477 1 |
| 9 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |
| 10 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 11 | 0.0451862 0.02121901 0.1559265 1 | 0.4475731 0.04184917 0.1559265 1 |
| 12 | 0.0451862 0.02121901 0.1559265 1 | 0.5308154 0.04611696 0.1559265 1 |
| 13 | 0.0451862 0.02121901 0.1559265 1 | 0.4475731 0.04184917 0.1559265 1 |
| 14 | 0 0 0 0 | 0.2242794 0.01149868 0 0.2242794 |
| 15 | 0 0 0 0 | 0.08036564 0.004120303 0 0.08036564 |

Row 7, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 0.001165048 5.973138e-05 0 0.001165048 |
| 8 | 0 0 0 0 | 0.009508795 0.0004875108 0 0.009508795 |
| 9 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |
| 10 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 11 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 12 | 0 0 0 0 | 0.2632008 0.01349416 0 0.2632008 |
| 13 | 0 0 0 0 | 0.2180852 0.01118111 0 0.2180852 |
| 14 | 0 0 0 0 | 0.1215547 0.006232044 0 0.1215547 |
| 15 | 0 0 0 0 | 0.04355649 0.002233118 0 0.04355649 |

Row 8, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 0.0004135064 2.120025e-05 0 0.0004135064 |
| 8 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 9 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |
| 10 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 11 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 12 | 0 0 0 0 | 0.09341694 0.004789436 0 0.09341694 |
| 13 | 0 0 0 0 | 0.07740423 0.003968473 0 0.07740423 |
| 14 | 0 0 0 0 | 0.04314298 0.002211917 0 0.04314298 |
| 15 | 0 0 0 0 | 0.01545935 0.0007925926 0 0.01545935 |

Row 9, the 9 pixels that change:

| x | drawing | glowing |
| --- | --- | --- |
| 7 | 0 0 0 0 | 9.027237e-05 4.628215e-06 0 9.027237e-05 |
| 8 | 0 0 0 0 | 0.0007367777 3.777419e-05 0 0.0007367777 |
| 9 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |
| 10 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 11 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 12 | 0 0 0 0 | 0.0203938 0.001045579 0 0.0203938 |
| 13 | 0 0 0 0 | 0.01689808 0.0008663552 0 0.01689808 |
| 14 | 0 0 0 0 | 0.009418522 0.0004828825 0 0.009418522 |
| 15 | 0 0 0 0 | 0.003374923 0.0001730305 0 0.003374923 |

FX-GLOW-022: Radius 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-023: Radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-024: Radius keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-025: Threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-026: Intensity 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-027: Intensity -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-028: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-029: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-030: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-031: A tint written "orange". The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-032: Operation "multiply", which is not Add or Screen. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-GLOW-033: Based on "dark", which is not bright or colors. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.


## Limit fixtures

D-90, accepted on 2026-09-25. Every case is a project of one composition 6 by 2 at 24 fps, five frames long, in `Fixtures/limits/`: one of the adjustment fixtures' drawings, `bg` or `dot`, with an adjustment layer above it holding one effect. Values are linear premultiplied working values; each cell is red, green, blue and alpha.

**Every number below is produced by `tools/limits_reference.py`**, which works each setting's value at the frame from document 20, holds it inside its range, and renders each pixel from document 21 through `tools/adjust_reference.py`, summing the blur in two dimensions at once. The same numbers are in `Fixtures/limits/expected_limits.json`. At 20 stops a value is about a million and at sigma 500 about a millionth, so the tolerance is relative: a sample agrees when it is within 1e-4 of the expected value's own size, and exactly when the expected value is 0. FX-LIMIT-006 to 010 hold a value past a limit: the file is read, the effect is kept as written, and every frame is the drawing's own, with the warning `EFFECT_PARAMETER_INVALID` (D-46).

Commands, D-46: setting a blur's sigma to 501 is refused with "A Gaussian blur's sigma runs from 0 to 500, and this is 501.", and to -1 with "A Gaussian blur's sigma runs from 0 to 500, and this is -1."; setting exposure to 21 stops is refused with "Exposure runs from -20 to 20 stops, and this is 21.", and to -21 with "Exposure runs from -20 to 20 stops, and this is -21.". Each refusal leaves the effect, the document revision and the undo stack as they were. Sigma 500, 20 stops and -20 stops are accepted.

FX-LIMIT-001: Exposure 20 stops, the top of its range: every colour 2^20 times as much, the coverage unchanged.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 |
| 0, 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 |

FX-LIMIT-002: Exposure -20 stops, the bottom of its range: every colour 2^20 times less.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 9.53674316e-07 0 0 1 | 9.53674316e-07 0 0 1 | 9.53674316e-07 0 0 1 | 9.53674316e-07 0 0 1 | 9.53674316e-07 0 0 1 | 9.53674316e-07 0 0 1 |
| 0, 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 | 2.05860615e-07 2.05860615e-07 2.05860615e-07 1 |

FX-LIMIT-003: A Gaussian blur of sigma 500, the top of its range: one pixel spread almost evenly over the frame and far past it.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 0, 0 | 6.40054744e-07 6.40054744e-07 6.40054744e-07 6.40054744e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40059865e-07 6.40059865e-07 6.40059865e-07 6.40059865e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40054744e-07 6.40054744e-07 6.40054744e-07 6.40054744e-07 | 6.40048344e-07 6.40048344e-07 6.40048344e-07 6.40048344e-07 |
| 0, 1 | 6.40053464e-07 6.40053464e-07 6.40053464e-07 6.40053464e-07 | 6.40057304e-07 6.40057304e-07 6.40057304e-07 6.40057304e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40057304e-07 6.40057304e-07 6.40057304e-07 6.40057304e-07 | 6.40053464e-07 6.40053464e-07 6.40053464e-07 6.40053464e-07 | 6.40047063e-07 6.40047063e-07 6.40047063e-07 6.40047063e-07 |

FX-LIMIT-004: Exposure eased from 0 to 20 stops on the overshooting curve: past the middle it would pass 20, and is held at 20.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 1, 0 | 451419.543 0 0 1 | 451419.543 0 0 1 | 451419.543 0 0 1 | 451419.543 0 0 1 | 451419.543 0 0 1 | 451419.543 0 0 1 |
| 1, 1 | 97443.6483 97443.6483 97443.6483 1 | 97443.6483 97443.6483 97443.6483 1 | 97443.6483 97443.6483 97443.6483 1 | 97443.6483 97443.6483 97443.6483 1 | 97443.6483 97443.6483 97443.6483 1 | 97443.6483 97443.6483 97443.6483 1 |
| 2, 0 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 | 1048576 0 0 1 |
| 2, 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 | 226346.14 226346.14 226346.14 1 |

FX-LIMIT-005: A blur eased from 0 to 500 on the overshooting curve: held at 500.

| frame, row | x = 0 | x = 1 | x = 2 | x = 3 | x = 4 | x = 5 |
| --- | --- | --- | --- | --- | --- | --- |
| 2, 0 | 6.40054744e-07 6.40054744e-07 6.40054744e-07 6.40054744e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40059865e-07 6.40059865e-07 6.40059865e-07 6.40059865e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40054744e-07 6.40054744e-07 6.40054744e-07 6.40054744e-07 | 6.40048344e-07 6.40048344e-07 6.40048344e-07 6.40048344e-07 |
| 2, 1 | 6.40053464e-07 6.40053464e-07 6.40053464e-07 6.40053464e-07 | 6.40057304e-07 6.40057304e-07 6.40057304e-07 6.40057304e-07 | 6.40058584e-07 6.40058584e-07 6.40058584e-07 6.40058584e-07 | 6.40057304e-07 6.40057304e-07 6.40057304e-07 6.40057304e-07 | 6.40053464e-07 6.40053464e-07 6.40053464e-07 6.40053464e-07 | 6.40047063e-07 6.40047063e-07 6.40047063e-07 6.40047063e-07 |

FX-LIMIT-006: Exposure 21 stops, one past the top. The file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are the bg drawing, unchanged.

FX-LIMIT-007: Exposure -21 stops, one past the bottom. The file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are the bg drawing, unchanged.

FX-LIMIT-008: Exposure 128 stops over a drawing that is mostly transparent: past the limit, so left out, where before it made every transparent pixel not-a-number. The file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are the dot drawing, unchanged.

FX-LIMIT-009: A Gaussian blur of sigma 501, one past the top. The file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are the dot drawing, unchanged.

FX-LIMIT-010: A blur keyed from 0 at frame 0 to 600 at frame 4: one key past the top, so left out of every frame, frame 0 as well. The file is read, the effect is kept as written and left out of every frame, with a warning. Frames 0 and 4 are the dot drawing, unchanged.


## Line recolour fixtures

D-91, accepted on 2026-09-25. Every case is a project of one composition 16 by 10 at 24 fps, five frames long, in `Fixtures/recolor/`, holding one drawing the same size with `core.line_recolor` on it; the drawing is `Fixtures/recolor/media/face.png`: a box of line `#1e1a24` in columns 2 to 13 and rows 2 to 7, filled with skin `#f6d6be`, with the line at half covering down its left side in column 1, its antialiased edge, and a red trace line `#c82828` across row 5, columns 4 to 11. Unless the case says: the line chosen, tolerance 0, new colour `#ff0000`. Values are linear premultiplied working values, and only the pixels that change are listed: every other pixel is the drawing's own, exactly.

**Every number below is produced by `tools/recolor_reference.py`**, which works D-91's rule in double precision. The same numbers are in `Fixtures/recolor/expected_recolor.json`. Tolerance 2e-5.

**Checked by what they claim.** The tool checks each case's claim on its numbers: in FX-RECOLOR-001 exactly the 38 pixels of line change, the half-covering edge to red at half covering; no colour, a colour one step off at tolerance 0, and the line's own colour as the new one leave the drawing exactly (002, 003, 009); tolerance 1 and capitals give 001 (004, 006); the keyed tolerance chooses nothing below 10 and the line from 10 (007); and at tolerance 255 every pixel that shows turns red at its own covering (011).

FX-RECOLOR-001: The line chosen, new colour #ff0000: the box's line and its half-covering edge turn red, the edge still half covering; the skin and the trace line are untouched.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 3, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 4, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 5, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 6, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 7, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 8: unchanged.

Row 9: unchanged.

FX-RECOLOR-002: No colour chosen: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-RECOLOR-003: The line chosen one step off in blue, tolerance 0: nothing is chosen, and the drawing is untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-RECOLOR-004: The same at tolerance 1: the line is chosen again, and this is FX-RECOLOR-001.

Frame 0: the same as FX-RECOLOR-001 frame 0.

FX-RECOLOR-005: The line and the trace line chosen, new colour #3060ff: both turn blue.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 3, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 4, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 5, the 11 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 4 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 5 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 6 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 7 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 8 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 9 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 10 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 11 | 0.5775804 0.02121901 0.02121901 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 6, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 7, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.01483637 0.05871469 0.5019608 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.02955683 0.1169707 1 1 |

Row 8: unchanged.

Row 9: unchanged.

FX-RECOLOR-006: FX-RECOLOR-001 with both colours written in capitals: the same.

Frame 0: the same as FX-RECOLOR-001 frame 0.

FX-RECOLOR-007: #28242e chosen, 10 above the line on every channel, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1, at 0 and 5, choose nothing; frame 2, at exactly 10, is FX-RECOLOR-001, and so is frame 4.

Frame 0: every pixel is the drawing's, unchanged.

Frame 1: every pixel is the drawing's, unchanged.

Frame 2: the same as FX-RECOLOR-001 frame 0.

Frame 4: the same as FX-RECOLOR-001 frame 0.

FX-RECOLOR-008: FX-RECOLOR-001 moved three pixels right: the same, moved.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 12 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 14 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 15 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 3, the 2 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 4, the 2 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 5, the 2 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 6, the 2 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 7, the 12 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 4 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 14 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 15 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 8: unchanged.

Row 9: unchanged.

Frame 3: the same as FX-RECOLOR-008 frame 0.

FX-RECOLOR-009: The new colour is the line's own: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-RECOLOR-010: The new colour is the skin's: the line turns to skin and vanishes into it, and its edge is skin at half covering.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 3, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 4, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 5, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 6, the 3 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 7, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.462598 0.3375401 0.2584685 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0.9215819 0.6724432 0.5149177 1 |

Row 8: unchanged.

Row 9: unchanged.

FX-RECOLOR-011: Tolerance 255: every pixel that shows is chosen and turns red; the empty ones stay empty.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 3, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 4, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 5, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 4 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 5 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 6 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 7 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 8 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 9 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 10 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 11 | 0.5775804 0.02121901 0.02121901 1 | 1 0 0 1 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 6, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 7, the 13 pixels that change:

| x | drawing | recoloured |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0.5019608 0 0 0.5019608 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 1 0 0 1 |

Row 8: unchanged.

Row 9: unchanged.

FX-RECOLOR-012: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-013: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-014: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-015: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-016: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-017: A new colour written "red". The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-RECOLOR-018: A new colour left empty, which this effect does not allow. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.


## Directional blur fixtures

D-92, accepted on 2026-09-25. Every case is a project of one composition 16 by 10 at 24 fps, five frames long, in `Fixtures/directional_blur/`, holding one drawing the same size with `core.directional_blur` on it; the drawing is `Fixtures/directional_blur/media/bars.png`: a line `#1e1a24` down its left edge, column 0, rows 2 to 7; a block of skin `#f6d6be` in columns 5 to 9 and rows 3 to 6; and one red pixel `#c82828` at half covering at (12, 4). Unless the case says: direction 0, length 4. Values are linear premultiplied working values, and only the pixels that change are listed: every other pixel is the drawing's own, exactly.

**Every number below is produced by `tools/directional_blur_reference.py`**, which works D-92's rule in double precision. The same numbers are in `Fixtures/directional_blur/expected_directional_blur.json`. Tolerance 2e-5.

**Checked by what they claim.** The tool checks each case's claim on its numbers: at direction 0 the block's row 4 is four fifths covered and nothing spreads sideways, at 90 the reverse, and the block's covering is kept (001, 002); the opposite direction and ten turns give the same (003, 008); length 0 leaves the drawing exactly (005); length 1 covers the edge column three quarters and the one outside a quarter (006); length 2.5 reaches a quarter of a quarter two columns out (007); moved right, the line streaks into the grown border (009); and the keyed length and direction pass through the fixed cases (010, 011).

FX-DIRBLUR-001: Direction 0, length 4: every edge streaks up and down, two pixels each way, and left and right stay sharp.

Frame 0:

Row 0, the 1 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |

Row 1, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.005193213 0.004131929 0.007056782 0.4 |
| 5 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 6 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 7 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 8 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 9 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |

Row 2, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.007789819 0.006197894 0.01058517 0.6 |
| 5 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 6 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 7 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 8 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 9 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 12 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 3, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.01038643 0.008263858 0.01411356 0.8 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 12 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 4, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 5, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 12 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 6, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.01038643 0.008263858 0.01411356 0.8 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 12 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 7, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.007789819 0.006197894 0.01058517 0.6 |
| 5 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 6 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 7 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 8 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 9 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |

Row 8, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.005193213 0.004131929 0.007056782 0.4 |
| 5 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 6 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 7 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 8 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 9 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |

Row 9, the 1 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |

FX-DIRBLUR-002: Direction 90, length 4: the same streak left and right, and up and down stay sharp.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 3 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |

Row 3, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 3 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 4 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 10 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 11 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |

Row 4, the 15 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 3 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 4 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 10 | 0 0 0 0 | 0.4266173 0.2711075 0.2080973 0.5003922 |
| 11 | 0 0 0 0 | 0.2423009 0.1366189 0.1051138 0.3003922 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.05798455 0.002130222 0.002130222 0.1003922 |
| 13 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |
| 14 | 0 0 0 0 | 0.05798455 0.002130222 0.002130222 0.1003922 |

Row 5, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 3 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 4 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 10 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 11 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 3 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |
| 4 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.9215819 0.6724432 0.5149177 1 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7372655 0.5379545 0.4119341 0.8 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5529491 0.4034659 0.3089506 0.6 |
| 10 | 0 0 0 0 | 0.3686327 0.2689773 0.2059671 0.4 |
| 11 | 0 0 0 0 | 0.1843164 0.1344886 0.1029835 0.2 |

Row 7, the 3 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002596606 0.002065965 0.003528391 0.2 |
| 1 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |
| 2 | 0 0 0 0 | 0.002596606 0.002065965 0.003528391 0.2 |

Row 8: unchanged.

Row 9: unchanged.

FX-DIRBLUR-003: Direction 270, the opposite way: the streak runs both ways, so this is FX-DIRBLUR-002.

Frame 0: the same as FX-DIRBLUR-002 frame 0.

FX-DIRBLUR-004: Direction 45, length 6: a diagonal streak, up and right and down and left, between pixels.

Frame 0:

Row 0, the 9 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.0004500303 0.0003580622 0.0006115223 0.03466296 |
| 2 | 0 0 0 0 | 0.001947923 0.001549846 0.00264693 0.1500361 |
| 3 | 0 0 0 0 | 0.0002250151 0.0001790311 0.0003057611 0.01733148 |
| 7 | 0 0 0 0 | 0.0140346 0.01024051 0.007841587 0.01522882 |
| 8 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 9 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 10 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 11 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 12 | 0 0 0 0 | 0.001937774 0.00141392 0.001082697 0.002102661 |

Row 1, the 13 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0003841249 0.0003056252 0.0005219669 0.02958668 |
| 1 | 0 0 0 0 | 0.002013829 0.001602283 0.002736485 0.1551123 |
| 2 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 3 | 0 0 0 0 | 0.0002250151 0.0001790311 0.0003057611 0.01733148 |
| 6 | 0 0 0 0 | 0.03194475 0.02330887 0.01784857 0.03466296 |
| 7 | 0 0 0 0 | 0.1702153 0.1241996 0.09510479 0.184699 |
| 8 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 9 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 10 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 11 | 0 0 0 0 | 0.1542429 0.1125452 0.08618051 0.1673676 |
| 12 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 14 | 0 0 0 0 | 0.00441518 0.0001622038 0.0001622038 0.007644269 |
| 15 | 0 0 0 0 | 0.0006096092 2.239567e-05 2.239567e-05 0.001055453 |

Row 2, the 15 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002557063 0.002034502 0.003474658 0.1969542 |
| 1 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 2 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 3 | 0 0 0 0 | 0.0002250151 0.0001790311 0.0003057611 0.01733148 |
| 5 | 0 0 0 0 | 0.02726655 0.01989536 0.01523471 0.02958668 |
| 6 | 0 0 0 0 | 0.1702153 0.1241996 0.09510479 0.184699 |
| 7 | 0 0 0 0 | 0.3404306 0.2483992 0.1902096 0.3693981 |
| 8 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 9 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 10 | 0 0 0 0 | 0.3291364 0.2401583 0.1838992 0.3571429 |
| 11 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 12 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 13 | 0 0 0 0 | 0.01004958 0.000369199 0.000369199 0.01739944 |
| 14 | 0 0 0 0 | 0.03908369 0.001435847 0.001435847 0.06766796 |
| 15 | 0 0 0 0 | 0.00441518 0.0001622038 0.0001622038 0.007644269 |

Row 3, the 15 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002941188 0.002340128 0.003996625 0.2265409 |
| 1 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 2 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 3 | 0 0 0 0 | 0.0002250151 0.0001790311 0.0003057611 0.01733148 |
| 4 | 0 0 0 0 | 0.02726655 0.01989536 0.01523471 0.02958668 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.208776 0.1523359 0.1166499 0.2265409 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.3789913 0.2765355 0.2117547 0.41124 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5651789 0.4123895 0.3157838 0.6132705 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5379124 0.3924942 0.3005491 0.5836838 |
| 10 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 11 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 12 | 0 0 0 0 | 0.02455023 0.01196956 0.009239415 0.03218283 |
| 13 | 0 0 0 0 | 0.03492102 0.00128292 0.00128292 0.06046087 |
| 14 | 0 0 0 0 | 0.01004958 0.000369199 0.000369199 0.01739944 |

Row 4, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002941188 0.002340128 0.003996625 0.2265409 |
| 1 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 2 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 3 | 0 0 0 0 | 0.03216977 0.0234879 0.01815433 0.05199443 |
| 4 | 0 0 0 0 | 0.1702153 0.1241996 0.09510479 0.184699 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.3789913 0.2765355 0.2117547 0.41124 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.7053872 0.5146942 0.3941227 0.7654092 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.6874771 0.5016258 0.3841157 0.7459751 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 10 | 0 0 0 0 | 0.3404306 0.2483992 0.1902096 0.3693981 |
| 11 | 0 0 0 0 | 0.1787931 0.1245147 0.09541993 0.1995504 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.06255826 0.01202316 0.009624237 0.09924077 |
| 13 | 0 0 0 0 | 0.008577852 0.000315131 0.000315131 0.01485135 |

Row 5, the 13 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002941188 0.002340128 0.003996625 0.2265409 |
| 1 | 0 0 0 0 | 0.002397953 0.001907908 0.003258452 0.184699 |
| 2 | 0 0 0 0 | 0.01623484 0.01199111 0.01083137 0.184699 |
| 3 | 0 0 0 0 | 0.170413 0.1243569 0.09537346 0.1999278 |
| 4 | 0 0 0 0 | 0.3404306 0.2483992 0.1902096 0.3693981 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.6874771 0.5016258 0.3841157 0.7459751 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.7053872 0.5146942 0.3941227 0.7654092 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.3789913 0.2765355 0.2117547 0.41124 |
| 10 | 0 0 0 0 | 0.1802649 0.1245688 0.09547399 0.2020985 |
| 11 | 0 0 0 0 | 0.06686577 0.02459179 0.01913149 0.09512383 |
| 12 | 0 0 0 0 | 0.008577852 0.000315131 0.000315131 0.01485135 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002941188 0.002340128 0.003996625 0.2265409 |
| 1 | 0 0 0 0 | 0.001947923 0.001549846 0.00264693 0.1500361 |
| 2 | 0 0 0 0 | 0.01642241 0.0120125 0.009535806 0.05199443 |
| 3 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 4 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5379124 0.3924942 0.3005491 0.5836838 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5651789 0.4123895 0.3157838 0.6132705 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5492066 0.4007351 0.3068595 0.595939 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.3789913 0.2765355 0.2117547 0.41124 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.2131912 0.1524981 0.1168121 0.2341852 |
| 10 | 0 0 0 0 | 0.06635024 0.02133121 0.01667055 0.09725464 |
| 11 | 0 0 0 0 | 0.01004958 0.000369199 0.000369199 0.01739944 |

Row 7, the 11 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.002557063 0.002034502 0.003474658 0.1969542 |
| 1 | 0 0 0 0 | 0.0003841249 0.0003056252 0.0005219669 0.02958668 |
| 2 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 3 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 4 | 0 0 0 0 | 0.3291364 0.2401583 0.1838992 0.3571429 |
| 5 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 6 | 0 0 0 0 | 0.3564029 0.2600536 0.1991339 0.3867295 |
| 7 | 0 0 0 0 | 0.3404306 0.2483992 0.1902096 0.3693981 |
| 8 | 0 0 0 0 | 0.1702153 0.1241996 0.09510479 0.184699 |
| 9 | 0 0 0 0 | 0.02787616 0.01991776 0.0152571 0.03064214 |
| 10 | 0 0 0 0 | 0.00441518 0.0001622038 0.0001622038 0.007644269 |

Row 8, the 8 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0003841249 0.0003056252 0.0005219669 0.02958668 |
| 2 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 3 | 0 0 0 0 | 0.1542429 0.1125452 0.08618051 0.1673676 |
| 4 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 5 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 6 | 0 0 0 0 | 0.1861877 0.135854 0.1040291 0.2020305 |
| 7 | 0 0 0 0 | 0.1702153 0.1241996 0.09510479 0.184699 |
| 8 | 0 0 0 0 | 0.03194475 0.02330887 0.01784857 0.03466296 |

Row 9, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 2 | 0 0 0 0 | 0.001937774 0.00141392 0.001082697 0.002102661 |
| 3 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 4 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 5 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 6 | 0 0 0 0 | 0.01597238 0.01165443 0.008924284 0.01733148 |
| 7 | 0 0 0 0 | 0.0140346 0.01024051 0.007841587 0.01522882 |

FX-DIRBLUR-005: Length 0: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-DIRBLUR-006: Length 1, direction 90: two samples half a pixel either side, so the block's left edge column is three quarters covered and the column outside it one quarter.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 2 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |

Row 3, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |
| 4 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 10 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |

Row 4, the 9 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |
| 4 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 10 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |
| 11 | 0 0 0 0 | 0.07248068 0.002662778 0.002662778 0.1254902 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.1449614 0.005325556 0.005325556 0.2509804 |
| 13 | 0 0 0 0 | 0.07248068 0.002662778 0.002662778 0.1254902 |

Row 5, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |
| 4 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 10 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |

Row 6, the 6 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |
| 4 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6911864 0.5043324 0.3861882 0.75 |
| 10 | 0 0 0 0 | 0.2303955 0.1681108 0.1287294 0.25 |

Row 7, the 2 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.006491516 0.005164912 0.008820977 0.5 |
| 1 | 0 0 0 0 | 0.003245758 0.002582456 0.004410489 0.25 |

Row 8: unchanged.

Row 9: unchanged.

FX-DIRBLUR-007: Length 2.5, direction 90: four samples, spaced evenly across 2.5 pixels.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 3 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |

Row 3, the 11 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |
| 3 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |
| 4 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 10 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 11 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |

Row 4, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |
| 3 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |
| 4 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 10 | 0 0 0 0 | 0.3445137 0.2388226 0.1830324 0.3855392 |
| 11 | 0 0 0 0 | 0.1421597 0.04513427 0.03528893 0.2089052 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.0845608 0.003106574 0.003106574 0.1464052 |
| 13 | 0 0 0 0 | 0.0845608 0.003106574 0.003106574 0.1464052 |
| 14 | 0 0 0 0 | 0.01812017 0.0006656944 0.0006656944 0.03137255 |

Row 5, the 11 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |
| 3 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |
| 4 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 10 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 11 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |

Row 6, the 11 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |
| 3 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |
| 4 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.863983 0.6304155 0.4827353 0.9375 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5951883 0.4342862 0.332551 0.6458333 |
| 10 | 0 0 0 0 | 0.3263936 0.238157 0.1823667 0.3541667 |
| 11 | 0 0 0 0 | 0.05759887 0.0420277 0.03218235 0.0625 |

Row 7, the 3 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 1 | 0 0 0 0 | 0.003786718 0.003012865 0.00514557 0.2916667 |
| 2 | 0 0 0 0 | 0.0008114395 0.0006456139 0.001102622 0.0625 |

Row 8: unchanged.

Row 9: unchanged.

FX-DIRBLUR-008: Direction 3600, ten turns: this is FX-DIRBLUR-001.

Frame 0: the same as FX-DIRBLUR-001 frame 0.

FX-DIRBLUR-009: Direction 90, length 6, moved three pixels right: the line on the drawing's left edge streaks three pixels past it into the grown border.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 6 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |

Row 3, the 16 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.1335093 0.097539 0.07607995 0.2857143 |
| 6 | 0 0 0 0 | 0.2651638 0.1936023 0.1496396 0.4285714 |
| 7 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 13 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 14 | 0 0 0 0 | 0.2633091 0.1921266 0.1471193 0.2857143 |
| 15 | 0 0 0 0 | 0.1316546 0.09606331 0.07355967 0.1428571 |

Row 4, the 16 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.1335093 0.097539 0.07607995 0.2857143 |
| 6 | 0 0 0 0 | 0.2651638 0.1936023 0.1496396 0.4285714 |
| 7 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0.5680357 0.3857748 0.2957603 0.6431373 |
| 13 | 0 0 0 0 | 0.4363812 0.2897115 0.2222006 0.5002801 |
| 14 | 0 0 0 0 | 0.3047266 0.1936482 0.1486409 0.357423 |
| 15 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.1730721 0.0975849 0.07508125 0.2145658 |

Row 5, the 16 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.1335093 0.097539 0.07607995 0.2857143 |
| 6 | 0 0 0 0 | 0.2651638 0.1936023 0.1496396 0.4285714 |
| 7 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 13 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 14 | 0 0 0 0 | 0.2633091 0.1921266 0.1471193 0.2857143 |
| 15 | 0 0 0 0 | 0.1316546 0.09606331 0.07355967 0.1428571 |

Row 6, the 16 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.1335093 0.097539 0.07607995 0.2857143 |
| 6 | 0 0 0 0 | 0.2651638 0.1936023 0.1496396 0.4285714 |
| 7 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0.6582728 0.4803165 0.3677983 0.7142857 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0.5266182 0.3842532 0.2942387 0.5714286 |
| 13 | 0 0 0 0 | 0.3949637 0.2881899 0.220679 0.4285714 |
| 14 | 0 0 0 0 | 0.2633091 0.1921266 0.1471193 0.2857143 |
| 15 | 0 0 0 0 | 0.1316546 0.09606331 0.07355967 0.1428571 |

Row 7, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 1 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 2 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 4 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 5 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |
| 6 | 0 0 0 0 | 0.001854719 0.001475689 0.002520279 0.1428571 |

Row 8: unchanged.

Row 9: unchanged.

Frame 3: the same as FX-DIRBLUR-009 frame 0.

FX-DIRBLUR-010: Length keyed from 0 at frame 0 to 8 at frame 4, direction 90, linear: frame 0 untouched, frame 2 at length 4 is FX-DIRBLUR-002, frame 4 at 8.

Frame 0: every pixel is the drawing's, unchanged.

Frame 2: the same as FX-DIRBLUR-002 frame 0.

Frame 4:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 5 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 2 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 3 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 4 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |

Row 3, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.1038405 0.07586366 0.05917329 0.2222222 |
| 2 | 0 0 0 0 | 0.2062385 0.1505796 0.1163864 0.3333333 |
| 3 | 0 0 0 0 | 0.3086365 0.2252955 0.1735994 0.4444444 |
| 4 | 0 0 0 0 | 0.4110345 0.3000114 0.2308125 0.5555556 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 10 | 0 0 0 0 | 0.4095919 0.2988636 0.2288523 0.4444444 |
| 11 | 0 0 0 0 | 0.307194 0.2241477 0.1716392 0.3333333 |
| 12 | 0 0 0 0 | 0.204796 0.1494318 0.1144261 0.2222222 |
| 13 | 0 0 0 0 | 0.102398 0.07471591 0.05721307 0.1111111 |

Row 4, the 16 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.1038405 0.07586366 0.05917329 0.2222222 |
| 2 | 0 0 0 0 | 0.2062385 0.1505796 0.1163864 0.3333333 |
| 3 | 0 0 0 0 | 0.3086365 0.2252955 0.1735994 0.4444444 |
| 4 | 0 0 0 0 | 0.4110345 0.3000114 0.2308125 0.5555556 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5442036 0.374763 0.2872488 0.611329 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5442036 0.374763 0.2872488 0.611329 |
| 10 | 0 0 0 0 | 0.4418056 0.3000471 0.2300358 0.5002179 |
| 11 | 0 0 0 0 | 0.3394076 0.2253312 0.1728227 0.3891068 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.2370096 0.1506153 0.1156096 0.2779956 |
| 13 | 0 0 0 0 | 0.1346116 0.07589936 0.05839653 0.1668845 |
| 14 | 0 0 0 0 | 0.03221364 0.001183457 0.001183457 0.05577342 |
| 15 | 0 0 0 0 | 0.03221364 0.001183457 0.001183457 0.05577342 |

Row 5, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.1038405 0.07586366 0.05917329 0.2222222 |
| 2 | 0 0 0 0 | 0.2062385 0.1505796 0.1163864 0.3333333 |
| 3 | 0 0 0 0 | 0.3086365 0.2252955 0.1735994 0.4444444 |
| 4 | 0 0 0 0 | 0.4110345 0.3000114 0.2308125 0.5555556 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 10 | 0 0 0 0 | 0.4095919 0.2988636 0.2288523 0.4444444 |
| 11 | 0 0 0 0 | 0.307194 0.2241477 0.1716392 0.3333333 |
| 12 | 0 0 0 0 | 0.204796 0.1494318 0.1144261 0.2222222 |
| 13 | 0 0 0 0 | 0.102398 0.07471591 0.05721307 0.1111111 |

Row 6, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.1038405 0.07586366 0.05917329 0.2222222 |
| 2 | 0 0 0 0 | 0.2062385 0.1505796 0.1163864 0.3333333 |
| 3 | 0 0 0 0 | 0.3086365 0.2252955 0.1735994 0.4444444 |
| 4 | 0 0 0 0 | 0.4110345 0.3000114 0.2308125 0.5555556 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5119899 0.3735795 0.2860654 0.5555556 |
| 10 | 0 0 0 0 | 0.4095919 0.2988636 0.2288523 0.4444444 |
| 11 | 0 0 0 0 | 0.307194 0.2241477 0.1716392 0.3333333 |
| 12 | 0 0 0 0 | 0.204796 0.1494318 0.1144261 0.2222222 |
| 13 | 0 0 0 0 | 0.102398 0.07471591 0.05721307 0.1111111 |

Row 7, the 5 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 1 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 2 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 3 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |
| 4 | 0 0 0 0 | 0.001442559 0.001147758 0.001960217 0.1111111 |

Row 8: unchanged.

Row 9: unchanged.

FX-DIRBLUR-011: Direction keyed from 0 at frame 0 to 90 at frame 4, length 4: frame 0 is FX-DIRBLUR-001, frame 2 streaks at 45 degrees, frame 4 is FX-DIRBLUR-002.

Frame 0: the same as FX-DIRBLUR-001 frame 0.

Frame 2:

Row 0, the 2 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 1 | 0 0 0 0 | 0.0006300424 0.0005012871 0.0008561312 0.04852814 |
| 2 | 0 0 0 0 | 0.0004455072 0.0003544635 0.0006053762 0.03431458 |

Row 1, the 9 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0005377748 0.0004278753 0.0007307537 0.04142136 |
| 1 | 0 0 0 0 | 0.00281936 0.002243196 0.003831079 0.2171573 |
| 2 | 0 0 0 0 | 0.00107555 0.0008557506 0.001461507 0.08284271 |
| 6 | 0 0 0 0 | 0.04472265 0.03263241 0.024988 0.04852814 |
| 7 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 8 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 9 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 10 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 11 | 0 0 0 0 | 0.03162369 0.0230746 0.01766918 0.03431458 |

Row 2, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003579889 0.002848303 0.004864521 0.2757359 |
| 1 | 0 0 0 0 | 0.003357135 0.002671072 0.004561833 0.2585786 |
| 2 | 0 0 0 0 | 0.00107555 0.0008557506 0.001461507 0.08284271 |
| 5 | 0 0 0 0 | 0.03817317 0.02785351 0.02132859 0.04142136 |
| 6 | 0 0 0 0 | 0.2383014 0.1738794 0.1331467 0.2585786 |
| 7 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 8 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 9 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 10 | 0 0 0 0 | 0.2764746 0.2017329 0.1544753 0.3 |
| 11 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 13 | 0 0 0 0 | 0.01406941 0.0005168786 0.0005168786 0.02435922 |
| 14 | 0 0 0 0 | 0.009948575 0.0003654884 0.0003654884 0.01722457 |

Row 3, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.004117663 0.003276179 0.005595274 0.3171573 |
| 1 | 0 0 0 0 | 0.003357135 0.002671072 0.004561833 0.2585786 |
| 2 | 0 0 0 0 | 0.00107555 0.0008557506 0.001461507 0.08284271 |
| 4 | 0 0 0 0 | 0.03817317 0.02785351 0.02132859 0.04142136 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.2922864 0.2132702 0.1633099 0.3171573 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.5305878 0.3871497 0.2964566 0.5757359 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.568761 0.4150032 0.3177852 0.6171573 |
| 10 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 11 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 12 | 0 0 0 0 | 0.01200899 0.0004411835 0.0004411835 0.0207919 |
| 13 | 0 0 0 0 | 0.04888942 0.001796088 0.001796088 0.08464522 |
| 14 | 0 0 0 0 | 0.01406941 0.0005168786 0.0005168786 0.02435922 |

Row 4, the 14 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.004117663 0.003276179 0.005595274 0.3171573 |
| 1 | 0 0 0 0 | 0.003357135 0.002671072 0.004561833 0.2585786 |
| 2 | 0 0 0 0 | 0.00107555 0.0008557506 0.001461507 0.08284271 |
| 3 | 0 0 0 0 | 0.04472265 0.03263241 0.024988 0.04852814 |
| 4 | 0 0 0 0 | 0.2383014 0.1738794 0.1331467 0.2585786 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.5305878 0.3871497 0.2964566 0.5757359 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.7688892 0.5610291 0.4296033 0.8343146 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8452355 0.6167361 0.4722605 0.9171573 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.8005129 0.5841037 0.4472725 0.8686292 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 10 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 11 | 0 0 0 0 | 0.08835533 0.0561482 0.04309836 0.1036346 |
| 12 | 0.2899227 0.01065111 0.01065111 0.5019608 | 0.06793312 0.002495711 0.002495711 0.1176167 |
| 13 | 0 0 0 0 | 0.01200899 0.0004411835 0.0004411835 0.0207919 |

Row 5, the 13 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.004117663 0.003276179 0.005595274 0.3171573 |
| 1 | 0 0 0 0 | 0.003357135 0.002671072 0.004561833 0.2585786 |
| 2 | 0 0 0 0 | 0.00107555 0.0008557506 0.001461507 0.08284271 |
| 3 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 4 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.8005129 0.5841037 0.4472725 0.8686292 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.8452355 0.6167361 0.4722605 0.9171573 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.7688892 0.5610291 0.4296033 0.8343146 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.5305878 0.3871497 0.2964566 0.5757359 |
| 10 | 0 0 0 0 | 0.2523708 0.1743963 0.1336636 0.2829379 |
| 11 | 0 0 0 0 | 0.09361207 0.0344285 0.02678408 0.1331734 |
| 12 | 0 0 0 0 | 0.01200899 0.0004411835 0.0004411835 0.0207919 |

Row 6, the 12 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.004117663 0.003276179 0.005595274 0.3171573 |
| 1 | 0 0 0 0 | 0.002727093 0.002169785 0.003705701 0.2100505 |
| 2 | 0 0 0 0 | 0.0006300424 0.0005012871 0.0008561312 0.04852814 |
| 3 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 4 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0.568761 0.4150032 0.3177852 0.6171573 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0.6069341 0.4428567 0.3391138 0.6585786 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0.5305878 0.3871497 0.2964566 0.5757359 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0.2922864 0.2132702 0.1633099 0.3171573 |
| 10 | 0 0 0 0 | 0.04812175 0.028219 0.02169408 0.05864593 |
| 11 | 0 0 0 0 | 0.01406941 0.0005168786 0.0005168786 0.02435922 |

Row 7, the 9 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0.01298303 0.01032982 0.01764195 1 | 0.003579889 0.002848303 0.004864521 0.2757359 |
| 1 | 0 0 0 0 | 0.0005377748 0.0004278753 0.0007307537 0.04142136 |
| 3 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 4 | 0 0 0 0 | 0.2764746 0.2017329 0.1544753 0.3 |
| 5 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 6 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 7 | 0 0 0 0 | 0.3146477 0.2295865 0.1758039 0.3414214 |
| 8 | 0 0 0 0 | 0.2383014 0.1738794 0.1331467 0.2585786 |
| 9 | 0 0 0 0 | 0.03817317 0.02785351 0.02132859 0.04142136 |

Row 8, the 7 pixels that change:

| x | drawing | blurred |
| --- | --- | --- |
| 0 | 0 0 0 0 | 0.0005377748 0.0004278753 0.0007307537 0.04142136 |
| 3 | 0 0 0 0 | 0.03162369 0.0230746 0.01766918 0.03431458 |
| 4 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 5 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 6 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 7 | 0 0 0 0 | 0.07634634 0.05570702 0.04265718 0.08284271 |
| 8 | 0 0 0 0 | 0.04472265 0.03263241 0.024988 0.04852814 |

Row 9: unchanged.

Frame 4: the same as FX-DIRBLUR-002 frame 0.

FX-DIRBLUR-012: Length 501, above 500. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-DIRBLUR-013: Length -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-DIRBLUR-014: Direction 3601, past ten turns. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-DIRBLUR-015: Length keyed to 600 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.


## Select colour fixtures

D-93, accepted on 2026-09-25. Every case is a project of one composition 16 by 10 at 24 fps, five frames long, in `Fixtures/select_color/`, holding one drawing the same size with `core.select_color` on it; the drawing is line recolour's, `Fixtures/select_color/media/face.png`: a box of line `#1e1a24` in columns 2 to 13 and rows 2 to 7, filled with skin `#f6d6be`, with the line at half covering down its left side in column 1 and a red trace line `#c82828` across row 5, columns 4 to 11. Unless the case says: the line chosen, tolerance 0, keep `chosen`. Values are linear premultiplied working values, and only the pixels that change are listed: every other pixel is the drawing's own, exactly.

**Every number below is produced by `tools/select_color_reference.py`**, which works D-93's rule in double precision. The same numbers are in `Fixtures/select_color/expected_select_color.json`. Tolerance 2e-5.

**Checked by what they claim.** The tool checks each case's claim on its numbers: keep chosen and keep others split the drawing between them pixel for pixel, the half-covering edge kept at half (001, 002); no colour leaves the drawing exactly, whichever is kept (003, 004); the trace line chosen too keeps it (005); capitals choose the same (006); the keyed tolerance keeps nothing until it reaches 10 (007); moved, the same frame moves (008); and tolerance 255 with keep others leaves nothing (009).

FX-SELECT-001: The line chosen, keep chosen: only the box's line and its half-covering edge are left, the edge still half covering; the skin and the trace line become transparent.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2: unchanged.

Row 3, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 4, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 5, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 5 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 6 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 7 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 8 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 9 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 10 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 11 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 6, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 7: unchanged.

Row 8: unchanged.

Row 9: unchanged.

FX-SELECT-002: The line chosen, keep others: the line and its edge become transparent, and the skin and the trace line are left.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 3, the 3 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 4, the 3 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 5, the 3 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 6, the 3 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 7, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 8: unchanged.

Row 9: unchanged.

FX-SELECT-003: No colour chosen, keep chosen: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELECT-004: No colour chosen, keep others: the drawing, untouched.

Frame 0: every pixel is the drawing's, unchanged.

FX-SELECT-005: The line and the trace line chosen, keep chosen: both lines are left, and only the skin goes.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2: unchanged.

Row 3, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 4, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 5, the 2 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 6, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 7: unchanged.

Row 8: unchanged.

Row 9: unchanged.

FX-SELECT-006: FX-SELECT-001 with the colour written in capitals: the same.

Frame 0: the same as FX-SELECT-001 frame 0.

FX-SELECT-007: #28242e chosen, 10 above the line on every channel, keep chosen, tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frames 0 and 1 choose nothing and so keep nothing, and the frame is empty; frame 2, at exactly 10, is FX-SELECT-001, and so is frame 4.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 3, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 4, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 5, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 5 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 6 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 7 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 8 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 9 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 10 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 11 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 6, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 4 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 5 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 7, the 13 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 1 | 0.006516973 0.005185166 0.008855569 0.5019608 | 0 0 0 0 |
| 2 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 3 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 4 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 5 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 6 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 7 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 8 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 9 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 10 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 11 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 12 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |
| 13 | 0.01298303 0.01032982 0.01764195 1 | 0 0 0 0 |

Row 8: unchanged.

Row 9: unchanged.

Frame 1: the same as FX-SELECT-007 frame 0.

Frame 2: the same as FX-SELECT-001 frame 0.

Frame 4: the same as FX-SELECT-001 frame 0.

FX-SELECT-008: FX-SELECT-001 moved three pixels right: the same, moved.

Frame 0:

Row 0: unchanged.

Row 1: unchanged.

Row 2: unchanged.

Row 3, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 14 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 15 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 4, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 14 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 15 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 5, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 8 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 9 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 10 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 11 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 12 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 13 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 14 | 0.5775804 0.02121901 0.02121901 1 | 0 0 0 0 |
| 15 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 6, the 10 pixels that change:

| x | drawing | select |
| --- | --- | --- |
| 6 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 7 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 8 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 9 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 10 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 11 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 12 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 13 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 14 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |
| 15 | 0.9215819 0.6724432 0.5149177 1 | 0 0 0 0 |

Row 7: unchanged.

Row 8: unchanged.

Row 9: unchanged.

Frame 3: the same as FX-SELECT-008 frame 0.

FX-SELECT-009: Tolerance 255, keep others: every pixel that shows is chosen, so the frame is empty.

Frame 0: the same as FX-SELECT-007 frame 0.

FX-SELECT-010: Tolerance 256, above 255. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-011: Tolerance -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-012: Tolerance keyed to 300 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-013: Nine colours, one more than eight. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-014: A colour written "#12345", one digit short. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-015: Keep "both", which is not a choice. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.

FX-SELECT-016: Keep "Chosen", in a capital, which is kept as written and is not the word. The file is read, the effect is kept as written and left out of every frame, with a warning. Warning `EFFECT_PARAMETER_INVALID`.

Frame 0: every pixel is the drawing's, unchanged.

Frame 4: every pixel is the drawing's, unchanged.


## Persistence fixtures

`Fixtures/projects/minimal_project.json`: smallest valid project. `cel_holds_project.json`: explicit exposure spans. `unicode_paths_project.json`: non-ASCII display/path fields. `missing_media_project.json`: valid project with intentionally unavailable asset. `unknown_effect_project.json`: structurally valid unknown effect that must survive load/save with a warning.

## Failure fixtures

FX-IO-001 interrupted replacement retains last valid project. FX-IO-002 disk-full/write failure reports `PROJECT_SAVE_FAILED` and does not truncate the previous valid save. FX-MATTE-001 creates A->B and B->A matte references and must be rejected with `MATTE_CYCLE`. FX-PARENT-004, above, does the same for a parent loop with `PARENT_CYCLE`, and FX-PRE-015 for a composition that holds a layer of itself, with `COMPOSITION_CYCLE` (D-67).

## Image/filter fixtures

Gaussian blur uses a synthetic single-pixel impulse in a transparent image. Expected weights are independently generated from the normalized sigma/kernel definition in 21. Test symmetry, normalization, alpha behavior and expanded bounds.

Mask rasterization fixtures must record the exact rasterizer/reference tool once selected in B-06; until then, polygon interior/exterior topology tests are authoritative but subpixel edge goldens are OPEN.

**Selected in B-06 on 2026-09-06 and recorded in ADR-016.** The rasterizer is a 4x4 ordered grid of sample points per pixel, coverage being the count of samples inside the polygon over sixteen, with insideness by the even-odd rule; the sample for grid cell (i, j) of pixel (x, y) is at `(x + (i + 0.5)/4, y + (j + 0.5)/4)`. The reference tool is a second implementation written from document 19 by a different method - summing the signed angle each edge subtends at the sample point - living in `tests/b06_mask.rs` rather than a drawing library, on the same discipline H-03 and H-04 use. Coverage is therefore an exact rational and the edge quantum is one sixteenth, which this sentence's OPEN now closes: subpixel edge goldens are written, in `verification/B-06_mask_table.md`. Raising the grid changes every one of them and is an amendment to ADR-016. Two decisions have since leaned on this rule without amending it. D-77, accepted on 2026-09-19 and built in B-24b, flattens a curved mask segment into straight pieces **before** the sampler sees it, so a curve and a corner meet the same 4x4 grid and a mask written before that decision draws pixel for pixel what it drew on 2026-09-06 - FX-MSK-001 is that claim as a fixture. D-78, accepted on 2026-09-20 and built in B-25b, draws a shape layer from the same grid, the same sixteenths and the same tie-break: a fill is even-odd insideness exactly as a mask's is, and a stroke is the same grid asking a different question of each sample, whether its distance to the flattened path is at most half the stroke's width. So there is still one rasterization rule in this build and one edge quantum, one sixteenth, wherever an outline meets a pixel.

## CPU/GPU comparison

Simple arithmetic: absolute per-channel error <= 1e-5. Filter operations: tolerance is declared per fixture after the independent CPU reference is implemented; default target <= 2e-5 for float output. Exported integer PNG tests compare decoded integer samples exactly where deterministic quantization is specified.

A backend that exceeds tolerance requires diagnosis. Do not loosen tolerances globally to hide a backend-specific error.

## Workflow fixtures

RSH-01 and RSH-02 are specified in 22. They are manually reviewed in addition to automated tests. Their visual acceptance cannot replace the numeric fixtures above.

Related documents: 11, 20, 21, 22 and 28.
