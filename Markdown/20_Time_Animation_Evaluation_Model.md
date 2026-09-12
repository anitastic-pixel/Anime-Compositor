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

Frames outside the active interval produce transparent output and do not request media. Moving a layer changes `in_frame/out_frame`; trimming and changing source offset are distinct commands.

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

For scalar/vector/color values, with `u=(f-f0)/(f1-f0)`, linear evaluation is `v0 + u*(v1-v0)`. Rotation in G1 interpolates the stored numeric degrees directly; automatic shortest-path wrapping is not performed. This makes authored values deterministic.

### Eased segments

Added on 2026-09-12 by D-52. An eased segment is a linear segment evaluated at a different fraction, and nothing else: `v0 + e*(v1-v0)`, where `e` comes from the segment's curve. **An ease changes when a value arrives, never what values are available between two keys**, which is why a pair of keys and a pair of keys with an ease between them travel the same path and differ only in timing.

The curve is the cubic Bezier of document 19's four numbers, with its ends pinned at (0,0) and (1,1). Writing `P(a, b, t)` for `3(1-t)^2 t a + 3(1-t) t^2 b + t^3`, the curve is the pair `x(t) = P(x1, x2, t)` and `y(t) = P(y1, y2, t)`. **`t` is the curve's own parameter and is not time.** Given `u`, the fraction of the way through the segment, `e` is `y` at the `t` where `x(t) = u`.

That equation has no closed form worth writing and is solved. Any solver is permitted; what is fixed is the answer, to the tolerance document 25 states, because `x` is non-decreasing for `x1` and `x2` within 0 and 1 and the solution is therefore unique. The reference is `tools/ease_reference.py`, which solves it a second way - by bisection, in another language, written from this section - and prints the expected values document 25 pins.

Two cases are worth stating because they are the ones a reader can check by hand. With `x1 = 1/3` and `x2 = 2/3`, `x(t)` works out to exactly `t`, so the solve is free. Easy ease is that pair with `y1 = 0` and `y2 = 1`, and it therefore reduces to `e = 3u^2 - 2u^3`: the value is exactly half way at exactly half the segment, and the ends are approached at zero rate. And `[1/3, 1/3, 2/3, 2/3]` gives `e = u`, so the curve that looks linear *is* linear rather than nearly so.

The ease belongs to the keyframe the segment starts at, like the mode itself. A keyframe's own value is returned exactly at its frame whatever its ease says, which follows from the rules above and is not a special case: the third rule fires before the sixth.

Opacity is clamped to 0..1 at command validation. Scale may be negative to permit mirroring unless a later UX decision forbids it.

## Evaluation order at one frame

1. Validate composition frame.
2. Snapshot document revision.
3. Resolve composition layer order and dependency graph.
4. For each required layer, derive layer-local frame.
5. Resolve exposure/source drawing.
6. Evaluate animated properties/effect parameters at the composition frame.
7. Evaluate source, mask, effects, transform and matte using 21.
8. Composite the ordered result.
9. Apply output/display transform only for the requested destination.

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
