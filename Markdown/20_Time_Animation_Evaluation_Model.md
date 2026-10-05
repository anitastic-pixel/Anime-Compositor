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

A layer with a `time_stretch` other than 100 (D-216) reads its source and its keys at a time that may fall between two local frames; see Time stretch, frame blending and the drawing dissolve below. A layer with a `time_remap` (D-323) reads its source at the time its remap keys give; see Time remapping below.

Frames outside the active interval produce transparent output and do not request media. Moving a layer changes `in_frame/out_frame` and moves every keyframe on the layer by the same number of frames (owner decision, 2026-09-13); trimming and changing source offset are distinct commands.

A composition layer (D-67, accepted on 2026-09-18) shows its composition at `local_frame`, by the same formula; outside that composition's own `[start_frame, start_frame + duration_frames)` the layer's picture is transparent (FX-PRE-006, FX-PRE-007). Frame numbers pass between the two, not seconds: the two compositions' rates are not compared. Time remapping (D-323, below) changes which inner frame is shown, never this rule.

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

### A mask's path

Added on 2026-09-20 by B-24d, under D-77, which stated the rule when masks were proposed. A mask's outline is a property whose value is a whole list of points, and every rule in the list above applies to it unchanged: zero keys give the base, before the first key and after the last the shape holds, a key's own outline is returned exactly at its frame, and a hold segment returns the left key's outline.

Between two keys the outline is interpolated **point by point, the point and both of its handles, componentwise**, at the same fraction every other property uses - `u` on a linear segment, `e` on an eased one. A handle is an offset from its own point (document 19), so a handle interpolates as a handle and travels with the point it belongs to rather than towards some other point's.

The two keys must therefore hold the same number of points, and they do: a path's points are the path's, not a key's, so adding or removing one changes every key together. A file whose keys disagree is refused as `PROJECT_SCHEMA_INVALID`, and a command that would make them disagree is refused as `MASK_INVALID_OUTLINE`. There is no correspondence problem to solve and no pairing rule to specify, which is the whole reason D-77 put it that way.

A mask's path takes no spatial handles of its own in the sense the motion path means: the handles it carries shape the outline, not the way the outline travels between keys, and each one moves in a straight line at the segment's fraction. The reference is `tools/mask_reference.py`, which interpolates a second way, in another language, written from this section and D-77, and prints the expected values document 25 pins as FX-MSK-031 to 035.

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

The camera, specified by D-58 and built by B-13c, is evaluated the same way and at the same composition frame: its position, depth and zoom are read once for the frame being drawn, whatever drawing each layer's exposure holds at that frame. There is no shutter and no subframe camera sampling in G2, so a camera move and a cel on twos stay independent of one another. Motion blur, accepted with ADR-019 and D-188 on 2026-09-28, reads the camera between frames, and only to place a blurred layer; see Motion blur below.

Expressions, accepted by D-59 on 2026-09-16, are part of step 6. A property with an enabled expression is its keyed value put through the expression at the same composition frame, and that result is what every later step, parent and camera reads. An expression may read another property at another whole frame through `valueAtTime`; it never reads a sub-frame, and it never changes which drawing an exposure holds. Dependencies between expressions are resolved at evaluation and bounded by document 09, not sorted in advance.

Mattes create dependencies but not a second time domain: matte layers evaluate at the same composition frame. A remapped matte layer reads its own source by its own remap at that frame, as any layer does; the matte's time is not the matted layer's.

### Motion blur

Accepted by the owner with ADR-019 and D-188 on 2026-09-28, and built on the processor by B-124b the same day; FX-MB-001 to 050 in document 25 are its cases. A layer is blurred when its composition's `motion_blur` is enabled with a shutter angle above 0 and the layer's own switch is on. For frame `n`, shutter angle `A` in degrees, phase `P` in degrees and `N` samples, it is drawn at the moments

`t_k = n + P / 360 + (A / 360) * (k + 1/2) / N`, for `k = 0 .. N-1`,

worked in 64-bit numbers (FX-MB-001 to 005). At each `t_k`, step 6 is taken for the layer's anchor, position, scale, rotation and depth, for the same of each parent up its chain, and for the camera's position, depth and zoom and the camera's parent. A key segment is read at `u = (t - f0) / (f1 - f0)` by the rules above, unchanged: a hold keeps its left value until exactly the next key's frame, an ease and a motion path are read at that `u`, and a time before the first key or after the last takes that key's value (FX-MB-006 to 009). A property with an enabled expression is not read between frames: its value at `n` holds across the shutter, so D-59 stands.

Everything else is read once, at `n`, exactly as without motion blur: the layer's in and out points, the drawing its exposure holds, its masks, effects and shape outlines, a composition layer's local frame, its opacity, and the draw order by depth. A layer outside its in and out points at `n` is not drawn; one inside them is drawn at every moment, even one outside them (FX-MB-022). **Drawings hold**: a cel on twos is never mixed with the next drawing (FX-MB-023). How the moments are put together is document 21's.

### Time stretch, frame blending and the drawing dissolve

**Accepted by the owner with ADR-020 and D-216 on 2026-09-29, keys stretching with the layer by the owner's first answer (B-150a2); built in B-150b.** FX-FBLEND-001 to 067 in document 25 are its cases. A raster or composition layer may carry a `time_stretch` in per cent, from 1 to 10000, 100 when absent. Its source time at composition frame `n`, inside its in and out points, is

`t = (n - in_frame) * 100 / time_stretch + source_offset_frames`,

worked in 64-bit numbers, with `f = floor(t)` and `w = t - f` (FX-FBLEND-001 to 008). At 100 it is the local frame above, exactly. The stretch runs from the in point; the in and out points are composition frames and the stretch does not move them.

`P(f)`, the layer's picture at the whole local frame `f`, is exposure evaluation, or a composition layer's inner frame, exactly as above, with the drawing dissolve below. The layer's source at `n` is `P(f)`. When the layer's `frame_blend` is `"frame_mix"`, its composition's `frame_blending` is true, `w` is above 0 and `f + 1` is before the end of the layer's source, it is instead `P(f) + w * (P(f + 1) - P(f))`, each working number apart (document 21). The end of a raster layer's source is the largest `end_frame_exclusive` of its exposure, and of a composition layer its composition's `start_frame + duration_frames`; a still has none. `P(f + 1)` is read only when it is mixed in. A gap mixes in as transparent; a missing drawing number mixes in as transparent and reports `MEDIA_SEQUENCE_GAP`, never another drawing (FX-FBLEND-017, 018). A stretch below 100 steps over local frames, and only the two either side of `t` are mixed (FX-FBLEND-016).

The drawing dissolve: a raster layer may carry `drawing_dissolve`, a whole number of frames `D` from 0 to 100, 0 when absent. At a local frame `f` in the span `[s, e)` holding drawing `A`, when a span begins at exactly `e` holding drawing `B`, let `d = min(D, e - s - 1)`; when `d > 0` and `f >= e - d`, `P(f) = A + ((f - (e - d) + 1) / (d + 1)) * (B - A)`. Otherwise `P(f)` is `A`, as before: before a gap, on the last span and on ones nothing dissolves (FX-FBLEND-030 to 039). The dissolve does not read the composition's switch.

**Keys stretch with the layer**, as in After Effects (the owner's answer of 2026-09-29, B-150a2). Every key of a stretched layer - its transform and depth, its masks, its effects' settings - is read at the key time

`u = in_frame + (n - in_frame) * 100 / time_stretch`,

in 64-bit numbers, by the rules above for a time between two keys, never rounded; at 100, `u` is `n` exactly (the times cases give `u` too). A key stored at frame `k` so plays at composition frame `in_frame + (k - in_frame) * time_stretch / 100`, with the drawing it was set against (FX-FBLEND-023, 025 to 028). Stored key frames stay whole and are never moved by a stretch: the Time Stretch command changes only where they play, so no two keys land together (FX-FBLEND-045 to 047). A key set on a stretched layer with the playhead on frame `n` is stored at `round_half_away(u)` and plays where that frame puts it (FX-FBLEND-048, 049). Under motion blur each moment `t_k` is read at its own `u`. A parent's keys are read by the parent's own stretch, and a composition layer's inner keys at the inner composition's frame `f`, as that composition is drawn. A property with an enabled expression is still evaluated at `n` (D-59), and an effect that changes with the frame number itself rather than by keys, a seed or a drift, still reads `n`. Steps 6 to 9 of the evaluation order are otherwise unchanged.

Trimming the in point of a stretched layer by `d` frames must leave every surviving frame's drawing and key where it was: the offset moves by `x = d * 100 / time_stretch` source frames and every stored key of the layer by `d - x`. When `x` is not a whole number the trim is refused and nothing changes. At 100 this is the trim as it has always been (FX-FBLEND-064 to 067). Moving a whole layer in time moves its keys with it, as before.

Echo, Posterize Time and a layer setting read a layer's source by its own timing, and so read this same `t`. The Time Stretch command keeps the in point and sets the out point to `in_frame + max(1, round_half_away((out_frame - in_frame) * new / old))`, as one undo entry, and moves no stored key (FX-FBLEND-040 to 045).

### Time remapping

**Accepted by the owner with ADR-021 and D-308 on 2026-10-04; built as D-323.** FX-TREMAP-001 to 055 in document 25 are its cases. A raster or composition layer may carry `time_remap`, a property holding one number of source frames, keyed like any other with hold, linear and eased keys; absent means off, and the layer plays as above, bit for bit. A file with a `time_remap` on any other kind of layer, a pair of numbers in it, or an expression on it is refused (FX-TREMAP-050 to 055).

When it is present, the source time at composition frame `n`, inside the in and out points, is

`t = time_remap(u)`,

read at the key time `u` of the section above, so a stretch stretches the remap keys too (FX-TREMAP-007, 017). `source_offset_frames` is not used (FX-TREMAP-009, 018). A `t` within 1e-9 of a whole number is that whole number, so a straight line of keys never lands a hair short of a drawing (FX-TREMAP-030). Then `f = floor(t)`, `w = t - f`, `P(f)`, Frame Mix and the end of the source are exactly the section above's, and a `t` before the source's start or past its end shows nothing (FX-TREMAP-001 to 019).

Enable Time Remapping (Layer menu, Ctrl+Alt+T) writes two linear keys that keep every frame's picture: at `in_frame` the value `source_offset_frames`, and at `k = max(in_frame + 1, ceil(u(out_frame - 1)))` the value `k - in_frame + source_offset_frames` (FX-TREMAP-040 to 044). Turning it off removes the property and its keys. Freeze Frame on a composition layer, or on a layer that already has a remap, writes one hold key at `round_half_away(u(n))` holding the source time at the playhead, and removes any other remap keys (FX-TREMAP-045 to 047); a raster layer without a remap keeps D-314's freeze. Each is one undo entry. The remap keys move with the layer and with an in-point trim, like the layer's other keys (FX-TREMAP-048); a remap value or key cannot be set while the remap is off.

## Rounding and conversions

UI time entry in seconds converts to the nearest frame using round-half-away-from-zero unless the command explicitly requests floor/ceil semantics. Timecode display never changes stored frame identity.

When importing a sequence, file number is not assumed to equal composition frame. The sequence manifest maps drawing numbers; exposure spans map local frames to drawing numbers.

For 24000/1001 and similar rates, do not store rounded decimal rates such as 23.976 as authority. Display may show the conventional decimal label.

## Determinism requirements

Given the same project snapshot, frame index, media bytes and implementation version, the evaluator must choose the same source drawing, property values, dependency order and effect parameters. Expressions in G2 may add seeded randomness only under 09 and R-13.

## Extension boundary

Audio sample time is set by ADR-018 and D-71, accepted on 2026-09-19, with FX-AUD-001 to 008 as its fixtures: it adds a sum from whole frames to whole samples and changes nothing above. Motion blur was added by ADR-019 and D-188, which the owner accepted on 2026-09-28, with FX-MB-001 to 050 as its fixtures, under Motion blur above. Time stretch, Frame Mix frame blending and the drawing dissolve were added by ADR-020 and D-216, which the owner accepted on 2026-09-29, with FX-FBLEND-001 to 067 as their fixtures, under Time stretch, frame blending and the drawing dissolve above. Time remapping was added by ADR-021 and D-308, which the owner accepted on 2026-10-04, built as D-323, with FX-TREMAP-001 to 055 as its fixtures, under Time remapping above. Optical flow (Pixel Motion), a negative stretch and arbitrary subframe keyframes are outside G1. Adding them requires an ADR and new fixtures so the integer-frame contract is not retroactively reinterpreted.

Related documents: 07, 19, 21 and 25.
