# B-150a: time stretch, frame blending and the drawing dissolve, proposed (D-216, ADR-020)

This is item 5 of the fourth batch. On 2026-09-29 you answered "both":

- a layer can be **slowed down or sped up** (Time Stretch), with After Effects' **Frame Mix**
  blending, a switch on the layer and one on the composition, and no Pixel Motion;
- a layer's drawings can **dissolve** from one to the next.

The time rules (document 20) say anything that reads between frames needs a decision (an ADR)
and fixtures of its own first. So this is a proposal. **Nothing is built until you accept it.**

## What it is

- **Time Stretch**, on a drawing layer or a composition layer: a percentage. 100 is as drawn. 200
  plays at half speed, so each frame lasts twice as long. 50 plays at double speed. From 1 to
  10000.
- **Frame Blending**, a switch on the layer and a switch on the composition. When both are on,
  a slowed-down layer does not just hold each frame longer: a moment that falls between two
  frames shows the two mixed, by how far between them it is. That is After Effects' Frame Mix.
- **Drawing Dissolve**, on a drawing layer: a number of frames. The last frames of each held
  drawing fade into the next drawing. With 1 on twos, the second frame of each drawing is half
  of it and half of the next.

## What you will see

`frame_blending.gif` in this folder is a film of a ball bouncing in 6 drawings on twos, played
four ways side by side, worked by the proposed rule. `frame_blending.png` is the same, frames 0
to 11 of each, one row each:

1. **As drawn**: on twos.
2. **Stretched 200%, no blending**: each drawing held four frames. Slower, and steppier.
3. **Stretched 200%, Frame Mix**: every fourth frame shows two balls, each at half strength: the
   drawing going and the drawing coming. The frames between two frames of the same drawing stay
   sharp.
4. **Drawing Dissolve 1, at 100%**: the same speed as 1, but every second frame is two drawings
   at half strength.

The ghosting in 3 and 4 is what Frame Mix looks like in After Effects too: it mixes pictures, it
does not invent in-between drawings. That is Pixel Motion's job, which you asked to leave out.

In the program:

- The composition settings get a **Frame Blending** switch.
- The timeline gets a frame-blending switch on each layer, next to the motion-blur switch, and a
  **Stretch** column.
- **Time Stretch** is a command, undone in one step: it keeps the in point and moves the out point
  so the whole stretched layer still plays (200% on a 6-frame layer makes it 12 frames).
- Layer settings get **Drawing Dissolve**, in frames.

## What changes and what stays

- A stretch changes which frame of the layer's own drawings is shown. It does not move the layer,
  its keys, its masks or its effects.
- Mixing and dissolving happen first, on the drawings. Masks, effects, movement, opacity and
  motion blur then run once on the mixed picture.
- A drawing held on twos is not mixed with itself into anything new: it stays bit for bit sharp.
- At the end of the layer's drawings, the last one is shown alone, never mixed with nothing.
- A drawing whose file is missing is shown as empty and reported, as today, never replaced by
  another drawing.
- A file that uses none of this opens and saves exactly as before.

## Known limits, on purpose

- **Sped up**, Frame Mix mixes only the two frames either side of the moment. Frames stepped over
  are skipped, not averaged in. After Effects may average more when speeding up.
- **No Pixel Motion**, no playing backwards, and no time remapping (a keyed speed curve). Each
  could come later without changing this rule.
- **The exposure sheet (XDTS)** still lists the layer's drawings as timed before the stretch.
- **The graphics card**: at first it does not draw a mixed or dissolved frame. The viewer draws
  such a frame on the processor and says so (D-101). The card is a later unit.
- **Sound layers** are not stretched.

## Questions for you

1. **Keys.** In After Effects, Time Stretch also stretches the layer's keys. Here the keys stay
   where they are: the drawings slow down, the movement does not. Keep that, or stretch the keys
   with the layer?
2. **Drawing Dissolve and the composition switch.** The dissolve is the layer's own and works
   even when the composition's Frame Blending switch is off. Is that right, or should it obey the
   composition switch too?
3. **Speeding up.** Only the two frames either side are mixed. Enough, or should every frame
   stepped over be averaged in?
4. **The composition switch.** As in After Effects, the composition's switch is off until you turn
   it on, and a layer's switch does nothing until then. Should turning on a layer's switch turn
   the composition's on for you?

## How you will check it

The build will draw every case in `Fixtures/frame_blending/` and compare it with the numbers in
`expected_frame_blending.json`, which `tools/frame_blending_reference.py` works out a second way.
Document 25 lists the cases in tables you can read. The drawings are a red, a blue and a green bar
8 pixels wide, so every mixed pixel is a share you can read:

- at 100% nothing changes, switches on or off;
- at 200% without blending each drawing is held four frames;
- at 200% with Frame Mix, frames 3 and 7 are half and half, and the rest are exact drawings;
- with either switch off, no mixing;
- a gap, a missing drawing, an offset, an in point, a composition layer, and keys that do not
  stretch;
- the dissolve on twos, on threes, on ones, before a gap, into the same drawing, into a missing
  one, and together with the stretch;
- where the Time Stretch command puts the out point;
- files that break the rules are refused.

A playtest sheet will come with the build.
