# ADR-019: Motion blur averages the layer drawn at moments inside the shutter; drawings hold

Status: ACCEPTED (the owner, 2026-09-28: "I approve of motion blur plan")
Date: 2026-09-28
Deciders: Andrew (owner)
Relates to: D-188, document 20 "Extension boundary" and the camera paragraph of "Evaluation order at one frame", document 21 "Deferred rendering questions", D-57 (parents), D-58 (camera), D-59 (expressions), D-101 (`GPU_PREVIEW_ON_CPU`)

## Context

Document 20 says the frame is a whole number, that there is no shutter and no sub-frame camera
sampling, and that motion blur "requires an ADR and new fixtures so the integer-frame contract is
not retroactively reinterpreted". On 2026-09-28 the owner's list put motion blur in as item 10,
to be written up to an ADR and to stop before code. This is that ADR.

Three things have to be decided:

- when inside a frame a layer is drawn;
- what is read at those moments and what is not;
- how the drawings are put together.

For anime the second is the one that matters. A cel on twos must not be smeared into the next
drawing.

## Decision

**1. The shutter is a composition setting, and each layer has a switch.** The composition has an
optional `motion_blur`: `enabled`, `shutter_angle` (0 to 720 degrees), `shutter_phase` (-360 to
360) and `samples` (2 to 64, whole).

- When absent it means off, with 180, -90 and 16.
- A layer has `motion_blur: true` when its switch is on. It is allowed on raster, solid, shape and
  composition layers only.
- A layer is blurred only when both switches are on.
- Both fields are written only when they differ from absent, so a file that never used motion
  blur is saved unchanged.

**2. The moments.** Frame `n` is drawn at `t_k = n + phase/360 + (angle/360)(k + 1/2)/N`, for
`k = 0 .. N-1`, in 64-bit numbers. An angle of 0 is motion blur off.

**3. Only where the layer is, is read at each moment.** At each `t_k` these are read, by
document 20's key rules applied at a fraction of a frame:

- the layer's anchor, position, scale and rotation;
- its depth;
- the same for each parent up its chain;
- the camera's position, depth and zoom, and the camera's parent.

Everything else is read once, at the whole frame `n`:

- the exposure's drawing;
- in and out points;
- masks and effects, which run once;
- shape outlines;
- a composition layer's picture;
- opacity;
- the drawing order by depth.

A held key changes at exactly its frame. A property with an enabled expression holds its
whole-frame value across the shutter, so D-59 stands: an expression never reads a sub-frame.

**4. The average.** The layer's picture, after its effects, is resampled into the frame once per
moment, by document 21's rule for step 4. The `N` pictures are summed in the order of `k` in the
working numbers, and divided by `N` once, in linear premultiplied light. Matte, opacity and blend
then follow once.

- If all `N` placements are the same, the layer is drawn once, bit for bit as with the switch
  off.
- A matte layer's picture is blurred by its own switch.
- A moment at which the layer is level with or behind the camera contributes nothing, and
  `CAMERA_PLANE_BEHIND` is reported.

**5. Where it runs.** It runs on the processor first. Until a card unit is written, the viewer
draws a frame that has any blurred moving layer on the processor, and says so with
`GPU_PREVIEW_ON_CPU` (D-101). Draft uses the same moments as Full.

## Consequences

- The integer-frame contract is kept: everything document 20 says about one frame is still
  true at each whole frame. The only new thing is reading a transform at a fraction of a frame,
  which uses the existing key rules.
- Effects are never run more than once a frame for motion blur, so the effect cache still works.
  Only the resampling repeats. Cost grows with samples times the moving layers' area.
- An effect that changes over time (a wiggle, a twinkle) is not smeared by its own change, and
  expression motion is not blurred. Both were stated in the proposal, which the owner accepted as written.
- FX-MB-001 to 050 in document 25 pin the moments, the values read between frames, the pictures
  and the files that are refused.

## Alternatives not taken

- **A blur along a velocity per pixel ("vector" motion blur).** It is cheaper, but it guesses
  where a turning or scaling layer's pixels go, and cannot be pinned to an exact second
  reckoning.
- **Averaging whole frames.** It re-runs every effect and every drawing `N` times, and smears
  one drawing into the next, which is the one thing anime must not do.
- **After Effects' adaptive sample limit.** The number of samples would change with speed, so the
  same settings would give different pictures on different frames. Fixed samples are
  predictable.
- **Jittered moments.** Evenly spaced moments are exact and repeatable. Jitter would need a seed
  for no visible gain at 16 samples.
