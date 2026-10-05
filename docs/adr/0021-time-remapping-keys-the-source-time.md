# ADR-021: Time Remapping keys the source time

Status: ACCEPTED by the owner on 2026-10-04 with D-308 ("go ahead and build all of D-308"), built
as D-323
Date: 2026-10-04
Deciders: Andrew (owner)
Relates to: D-308, D-323, P-26 tutorial 1, ADR-020 and D-216 (time stretch, Frame Mix), D-314
(Freeze Frame), document 20 "Layer-local time", "Time stretch, frame blending and the drawing
dissolve" and "Extension boundary"

## Context

Document 20's extension boundary says retiming curves (time remapping) need an ADR and new
fixtures. ADR-020 left time remapping out and said it "can later replace `t`'s formula without
touching the mix". P-26's tutorial 1 needs it: a burst of fast motion that settles to real speed,
done in After Effects with Time Remapping and two keys. D-314's Freeze Frame also refuses
composition layers, because After Effects freezes those with a Time Remap key.

## Decision

**1. A keyed property of source frames.** A raster or composition layer may carry `time_remap`,
a property holding one number, keyable with hold, linear and eased keys like any other. Absent
means off, and the layer plays as ADR-020 says, bit for bit. An expression on it, a pair of
numbers, or a `time_remap` on any other kind of layer, is refused when a file is read.

**2. It replaces the source time.** At composition frame `n` inside the in and out points,
`t = time_remap(u)`, read at the key time `u` of ADR-020, so a stretch stretches the remap keys
too. The source offset is not used. A `t` within 1e-9 of a whole frame is that whole frame, so a
straight line of keys does not land a hair short of a drawing. `f`, `w`, Frame Mix, the end of
the source and everything after are ADR-020's, unchanged. A time before or after the source
shows nothing, as a composition layer already does outside its composition.

**3. Turning it on changes nothing you can see.** Layer > Enable Time Remapping (Ctrl+Alt+T)
writes two linear keys: at the in point, the offset; at `k = max(in + 1, ceil(u(out - 1)))`,
`k - in + offset`. Every frame keeps its picture. Turning it off removes the property and its
keys. Both are one undo entry.

**4. Freeze Frame is one hold key.** Freeze Frame on a composition layer, or on any layer that
already has a remap, writes one hold key at `round_half_away(u(n))`, holding the source time at
the playhead, and removes any other remap keys. A raster layer without a remap keeps D-314's
freeze, which already worked.

**5. Editing.** The remap keys move when the layer is moved and when its in point is trimmed,
like its other keys. Setting a remap value or key while the remap is off is refused.

## Consequences

- Files are unchanged unless the remap is used; it is never written when off.
- A composition layer can now freeze, play backwards, burst and slow down.
- Mixed with Frame Mix, a fractional source time mixes two frames as ADR-020 already does.
- FX-TREMAP-001 to 055 in document 25 pin the times, pictures, commands and refused files.

## Alternatives not taken

- **Time in seconds, as After Effects shows it.** Every other time here is a frame; the panel
  can show seconds later without changing the file.
- **The remap read at `n`, not `u`.** A stretched layer's remap keys would then not stretch with
  its other keys, unlike After Effects.
- **Remapping solids, nulls and adjustment layers.** They have no source time.
