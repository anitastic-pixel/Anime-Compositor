# Time and animation evaluation model

Version 0.2 | 2026-09-04 | Proposed baseline

## Canonical units

FrameRate is a reduced rational `{numerator, denominator}` in frames per second. Examples: 24/1 and 24000/1001. Composition time is addressed primarily by signed integer FrameIndex values. Floating-point seconds are presentation data, not project identity.

A future SampleTime may include a rational subframe component. G1 viewer, editing and export requests use subframe zero only. This prevents premature motion-blur/subframe complexity while keeping the evaluator interface extensible.

## Composition interval

A composition owns `start_frame` and `duration_frames`. Its valid frame interval is half-open:

`[start_frame, start_frame + duration_frames)`

The last exportable frame is therefore `start_frame + duration_frames - 1`. UI inclusive ranges convert to this internal half-open convention immediately.

Seconds at an exact composition frame are:

`seconds = frame_index * fps_denominator / fps_numerator`

Use exact integer/rational arithmetic for conversions that determine frame identity. Floating point may be used for display and interpolation after the surrounding frame/key identities are established.

## Layer-local time

A layer is active on `[in_frame, out_frame)`. Its integer local frame is:

`local_frame = composition_frame - in_frame + source_offset_frames`

Frames outside the active interval produce transparent output and do not request media. Moving a layer changes `in_frame/out_frame` and moves every keyframe on the layer by the same number of frames (owner decision, 2026-09-13); trimming and changing source offset are distinct commands.

A composition layer (D-67, accepted on 2026-09-18) shows its composition at `local_frame`, by the same formula; outside that composition's own `[start_frame, start_frame + duration_frames)` the layer's picture is transparent (FX-PRE-006, FX-PRE-007). Frame numbers pass between the two, not seconds: the two compositions' rates are not compared, and time remapping is not proposed.

## Exposure evaluation

If a layer has an ExposureMap, find the unique ExposureSpan for which `start_frame <= local_frame < end_frame_exclusive`. Return that span's drawing number. If no span covers the frame, the layer source is transparent for that frame.

Sequence gaps are not collapsed. If drawing 1002 is referenced but absent, evaluation returns a missing-source diagnostic for 1002 rather than substituting 1001 or 1003.

An exposure map is evaluated before property animation and does not change transform/effect keyframe time.

## Property keyframes

G1 keyframes occur on integer composition frames. Each keyframe contains a value and the interpolation mode used from that keyframe to the next keyframe.

Evaluation rules:

- zero keyframes: return base value;
- before first keyframe: return first keyframe value;
- exactly on a keyframe: return that keyframe value;
- after last keyframe: return last keyframe value;
- hold segment: return the left keyframe value;
- linear segment: component-wise linear interpolation from left to right;
- eased segment: component-wise linear interpolation from left to right, at a fraction the segment's curve gives rather than at `u` itself.

For scalar/vector/color values, with `u=(f-f0)/(f1-f0)`, linear evaluation is `v0 + u*(v1-v0)`. Rotation in G1 interpolates the stored numeric degrees directly; automatic shortest-path wrapping is not performed. This makes authored values deterministic. The one exception is a `position` segment carrying spatial handles, where the fraction is the same but the point it names is on a curve rather than on the straight line; see the motion path below.

### Eased segments

Added on 2026-09-12 by D-52. An eased segment is a linear segment evaluated at a different fraction, and nothing else: `v0 + e*(v1-v0)`, where `e` comes from the segment's curve. **An ease changes when a value arrives, never what values are available between two keys**, which is why a pair of keys and a pair of keys with an ease between them travel the same path and differ only in timing.

The curve is the cubic Bezier of document 19's four numbers, with its ends pinned at (0,0) and (1,1). Writing `P(a, b, t)` for `3(1-t)^2 t a + 3(1-t) t^2 b + t^3`, the curve is the pair `x(t) = P(x1, x2, t)` and `y(t) = P(y1, y2, t)`. **`t` is the curve's own parameter and is not time.** Given `u`, the fraction of the way through the segment, `e` is `y` at the `t` where `x(t) = u`.

That equation has no closed form worth writing and is solved. Any solver is permitted; what is fixed is the answer, to the tolerance document 25 states, because `x` is non-decreasing for `x1` and `x2` within 0 and 1 and the solution is therefore unique. The reference is `tools/ease_reference.py`, which solves it a second way - by bisection, in another language, written from this section - and prints the expected values document 25 pins.

Two cases are worth stating because they are the ones a reader can check by hand. With `x1 = 1/3` and `x2 = 2/3`, `x(t)` works out to exactly `t`, so the solve is free. Easy ease is that pair with `y1 = 0` and `y2 = 1`, and it therefore reduces to `e = 3u^2 - 2u^3`: the value is exactly half way at exactly half the segment, and the ends are approached at zero rate. And `[1/3, 1/3, 2/3, 2/3]` gives `e = u`, so the curve that looks linear *is* linear rather than nearly so.

The ease belongs to the keyframe the segment starts at, like the mode itself. A keyframe's own value is returned exactly at its frame whatever its ease says, which follows from the rules above and is not a special case: the third rule fires before the sixth.

### The motion path

Added on 2026-09-12 by D-53, and it is the other half of D-52's picture: an ease changes *when* the layer is a given fraction of the way along a segment, and a path changes *where* that fraction is. They are independent and they compose in that order.

The path applies to `position` and to nothing else. A segment from key A to key B is the cubic Bezier with control points `P0 = A.value`, `P1 = A.value + A.out`, `P2 = B.value + B.in`, `P3 = B.value`, where `out` and `in` are the second and first halves of document 19's `spatial` array, offsets in composition pixels from the key that carries them:

`B(t) = (1-t)^3 P0 + 3(1-t)^2 t P1 + 3(1-t) t^2 P2 + t^3 P3`

**`t` is the fraction of the way through the segment, after the ease**: it is `u` on a linear segment and `e` on an eased one, and it is *not* arc length along the curve. D-53 records why, and records the consequence: on a curved segment with uneven handles the layer's speed varies a little even under linear timing. A segment whose handles are absent takes them at the thirds - `P1 = P0 + (P3 - P0)/3` and `P2 = P3 - (P3 - P0)/3` - which reduces the cubic to `P0 + t(P3 - P0)` identically, so an unpathed segment is the straight line this section already specified and no rule above it changes. Handles of zero are not that: they give `P0 + (3t^2 - 2t^3)(P3 - P0)`, the straight line at an easy-ease speed, which is why absence is the thirds and not zero.

Every rule in the list above still fires first. A keyframe's own value is returned exactly at its frame, a hold segment returns the left value and the path has no say in either, and a curve is only consulted between two keys.

There is nothing to solve here: the cubic is evaluated directly at a `t` that is already known, which is the whole benefit of the parameterization D-53 chose. The reference is `tools/path_reference.py`, which evaluates it a second way - by de Casteljau, in another language, written from this section - and prints the expected values document 25 pins.

Opacity is clamped to 0..1 at command validation. Scale may be negative to permit mirroring unless a later UX decision forbids it.

## Evaluation order at one frame

1. Validate composition frame.
2. Snapshot document revision.
3. Resolve composition layer order and dependency graph.
4. For each required layer, derive layer-local frame.
5. Resolve exposure/source drawing.
6. Evaluate animated properties/effect parameters at the composition frame. D-68, accepted on 2026-09-18: an effect parameter with keys is evaluated exactly as a transform property is, a colour's three numbers at the same fraction in linear light, and the result is then held inside the parameter's range, because an ease may overshoot. D-69, accepted on 2026-09-18: a position written as X and Y apart is `[x at the frame, y at the frame]`, each evaluated as any number is; a key's `kind` and `roving` are not read by evaluation at all.
7. Evaluate source, mask, effects, transform and matte using 21.
8. Composite the ordered result.
9. Apply output/display transform only for the requested destination.

Parents, accepted by D-57, create dependencies in the same way: a parent's anchor, position, scale and rotation are evaluated at the same composition frame as its child's, whether or not that frame is inside the parent's in and out points, whether or not the parent is enabled, and whatever drawing its exposure holds. Nothing else of the parent is evaluated for the child.

The camera, specified by D-58 and built by B-13c, is evaluated the same way and at the same composition frame: its position, depth and zoom are read once for the frame being drawn, whatever drawing each layer's exposure holds at that frame. There is no shutter and no subframe camera sampling in G2, so a camera move and a cel on twos stay independent of one another.

Expressions, accepted by D-59 on 2026-09-16, are part of step 6. A property with an enabled expression is its keyed value put through the expression at the same composition frame, and that result is what every later step, parent and camera reads. An expression may read another property at another whole frame through `valueAtTime`; it never reads a sub-frame, and it never changes which drawing an exposure holds. Dependencies between expressions are resolved at evaluation and bounded by document 09, not sorted in advance.

Mattes create dependencies but not a second time domain: matte layers evaluate at the same composition frame unless later time-remapping is explicitly introduced.

## Rounding and conversions

UI time entry in seconds converts to the nearest frame using round-half-away-from-zero unless the command explicitly requests floor/ceil semantics. Timecode display never changes stored frame identity.

When importing a sequence, file number is not assumed to equal composition frame. The sequence manifest maps drawing numbers; exposure spans map local frames to drawing numbers.

For 24000/1001 and similar rates, do not store rounded decimal rates such as 23.976 as authority. Display may show the conventional decimal label.

## Determinism requirements

Given the same project snapshot, frame index, media bytes and implementation version, the evaluator must choose the same source drawing, property values, dependency order and effect parameters. Expressions in G2 may add seeded randomness only under 09 and R-13.

## Extension boundary

Motion blur, audio sample time, retiming curves, frame blending, optical flow and arbitrary subframe keyframes are outside G1. Adding them requires an ADR and new fixtures so the integer-frame contract is not retroactively reinterpreted.

Related documents: 07, 19, 21 and 25.
