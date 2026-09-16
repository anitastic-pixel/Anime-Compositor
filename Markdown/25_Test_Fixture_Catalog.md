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
