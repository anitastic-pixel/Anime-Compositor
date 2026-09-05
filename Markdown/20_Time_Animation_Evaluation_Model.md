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
- linear segment: component-wise linear interpolation from left to right.

For scalar/vector/color values, with `u=(f-f0)/(f1-f0)`, linear evaluation is `v0 + u*(v1-v0)`. Rotation in G1 interpolates the stored numeric degrees directly; automatic shortest-path wrapping is not performed. This makes authored values deterministic.

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
