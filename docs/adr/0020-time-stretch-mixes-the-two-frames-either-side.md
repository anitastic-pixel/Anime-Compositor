# ADR-020: A stretched layer reads its source between frames; Frame Mix mixes the two frames either side; drawings may dissolve

Status: PROPOSED on 2026-09-29 by the agent, awaiting the owner (D-216)
Date: 2026-09-29
Deciders: Andrew (owner)
Relates to: D-216, document 20 "Layer-local time", "Exposure evaluation" and "Extension boundary", document 21 "Layer render order" and "Deferred rendering questions", D-67 (composition layers), D-188 and ADR-019 (motion blur), D-189 (layer settings), D-101 (`GPU_PREVIEW_ON_CPU`)

## Context

Document 20 gives every layer a whole local frame, `composition_frame - in_frame +
source_offset_frames`, and says frame blending and retiming "require an ADR and new fixtures so
the integer-frame contract is not retroactively reinterpreted". On 2026-09-29 the owner answered
"both" to item 5 of the fourth batch: a layer time stretch with After Effects' Frame Mix
blending (a switch on the layer and one on the composition, no Pixel Motion), and a dissolve from
one drawing to the next. This is that ADR.

Four things have to be decided:

- how a stretch turns a composition frame into a time in the layer's source;
- what is shown when that time falls between two frames;
- how a drawing dissolves into the next one;
- what else, if anything, the stretch moves.

## Decision

**1. The stretch is a layer setting, not animated.** A raster or composition layer may carry
`time_stretch`, per cent, from 1 to 10000; absent means 100. At composition frame `n` the source
time is `t = (n - in_frame) * 100 / time_stretch + source_offset_frames` in 64-bit numbers, with
`f = floor(t)` and `w = t - f`. At 100 this is today's local frame, bit for bit. Playing backwards
(below 0) is not part of this.

**2. Without blending, the frame is `P(f)`.** `P(f)` is exactly what the layer shows today at
the whole local frame `f`: its exposure's drawing, or its composition drawn at `f`. A stretched
layer without blending holds each frame for longer, or steps over frames.

**3. Frame Mix.** A layer has `frame_blend: "frame_mix"` when its switch is on; the composition
has `frame_blending: true` when its switch is on. When both are on and `w > 0`, the frame is
`P(f) + w * (P(f + 1) - P(f))`, each linear premultiplied working number apart. Written this
way, two equal pictures mix to the same picture bit for bit, so a drawing held on twos stays
sharp between its own two frames. When `f + 1` is at or past the end of the source (the last
exposure span's end, or the inner composition's end), `P(f)` is used alone. `P(f + 1)` is read
only when it is mixed in. A gap mixes in as transparent, and a missing drawing number mixes in as
transparent with `MEDIA_SEQUENCE_GAP`, never another drawing. Sped up, only the two frames either
side of `t` are mixed.

**4. The drawing dissolve.** A raster layer may carry `drawing_dissolve`, a whole number of
frames `D` from 0 to 100, 0 or absent meaning off. On the last `d = min(D, hold - 1)` frames of a
held drawing `A` whose next span begins where its hold ends, with drawing `B`, the frame is
`A + (k / (d + 1)) * (B - A)` for `k = 1 .. d`. The hold's first frame is always `A` whole and the
next span's first frame `B` whole. Before a gap, on the last drawing, and on ones, nothing
dissolves. The dissolve is the layer's own and does not read the composition's switch. It is
worked inside `P(f)`, so a stretched, mixed layer mixes dissolved frames.

**5. Nothing else moves.** Keys stay on their composition frames: a stretch does not stretch the
layer's keys. In and out points are composition frames. Masks, effects, transform, matte,
opacity, blend and motion blur run once, on the one mixed picture, at `n`. Echo, Posterize Time
and layer settings read a layer's source by its own timing and so read the same `t`. The Time
Stretch command keeps the in point and moves the out point by the same share, as one undo entry.

**6. Where it runs.** On the processor first. Until a card unit is written, the viewer draws a
frame with a mixed or dissolved layer on the processor and says so with `GPU_PREVIEW_ON_CPU`
(D-101). Draft uses the same rule.

## Consequences

- The integer-frame contract is kept: every picture read is still a whole frame's picture. The
  only new thing is a share between two of them.
- A mixed frame costs two source pictures. For a composition layer that is two renders of the
  inner composition. The picture cache holds whole frames and serves both.
- Keys not stretching differs from After Effects, where the Time Stretch command also stretches
  the layer's keys. It is a question for the owner.
- Files are unchanged unless the new settings are used; the defaults are never written.
- FX-FBLEND-001 to 063 in document 25 pin the times, the pictures, the command and the files that
  are refused.

## Alternatives not taken

- **Pixel Motion (optical flow).** It guesses where each pixel goes, cannot be pinned to an exact
  second reckoning, and tears flat anime colour. The owner asked for no Pixel Motion.
- **Time remapping (a keyed time curve).** It is the bigger feature and needs keys on time
  itself. A fixed stretch covers the asked-for use, and time remapping can later replace `t`'s
  formula without touching the mix.
- **Averaging every frame stepped over when sped up.** It would blur fast playback more like a
  long shutter, but the number of frames mixed would change with the stretch. Two frames are
  predictable, and motion blur covers the smear.
- **A dissolve through the composition switch.** It would tie two unrelated switches together;
  the dissolve is a property of how the drawings are timed, not of the composition.
