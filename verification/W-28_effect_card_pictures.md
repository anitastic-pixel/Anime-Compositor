# W-28: dials and pictures on the third batch's cards

Asked for on 2026-09-27 with the card work: "also improve the visual designs or interactibility/features of effects if needed."

Nothing about the picture the effects make has changed, only how their settings are shown and changed, so there is no fixture table. The two photographs in `verification/W-28 pictures/` were taken in a browser, with the real page and made-up settings:

- `wipe_and_mirror_cards.png`: the Mirror, Radial Wipe and Iris Wipe cards
- `angle_dials.png`: the Venetian Blinds, Rain and Cross Glare cards with their dials, and a second Radial Wipe set to **Both**

## What was added

- **Dials** (W-27's, the same as Drop Shadow's) beside ten angles:
  - Halftone's Angle
  - Emboss's Direction
  - Wave Warp's Direction
  - Twirl's Angle
  - Mirror's Angle
  - Linear Wipe's Angle
  - Radial Wipe's Start angle
  - Venetian Blinds' Angle
  - Cross Glare's Angle
  - Rain's Direction
- **A picture of what is kept.** Mirror, Radial Wipe and Iris Wipe already had W-27's box for their Centre. The box now tints blue the part of the drawing each keeps:
  - Mirror: the kept half, and the mirror line.
  - Radial Wipe: what is left of the turn, and a line from the centre toward the start angle.
  - Iris Wipe: what the circle keeps.

Ripple, Twirl, Bulge and Speed Lines already had the Centre box from W-27, since every setting of two numbers gets one.

## Before you start

Open any project with a drawn layer, for example the reference shot the release build opens on. Select a layer and add the effects named below from **Add effect…**.

## What to check

1. **Dials.** Add **Rain**. Beside **Direction** is a dial whose line points almost straight down, and the number says `170°`. Drag round the dial: the line follows the pointer, the number changes, and the rain on the drawing slants as you go. Shift snaps to 15 degrees. Let go, then Ctrl+Z once: the whole drag is undone.
2. **The other dials.** Halftone, Emboss, Wave Warp, Twirl, Mirror, Linear Wipe, Radial Wipe, Venetian Blinds and Cross Glare each have the same dial on the settings listed above. Escape during a drag puts it back.
3. **Mirror.** Add **Mirror**. The box shows a blue half and a line through the centre mark. Turn the Angle dial: the line turns and the blue half turns with it, and the drawing keeps the same half the box shows. At 0 the right half is blue.
4. **Radial Wipe.** Add **Radial Wipe** and raise **Completion**. A slice opens in the box from straight up, going clockwise, and the same slice goes on the drawing. Change **Wipe** to **Counterclockwise**, then **Both**: the slice opens the other way, then both ways. Turn the Start angle dial: the line from the centre follows.
5. **Iris Wipe.** Add **Iris Wipe** and raise **Completion**. The blue circle in the box shrinks as the drawing's circle does. Set **Invert** to **On**: the blue is now outside a growing hole.
6. **Centres.** In any of these boxes, press anywhere: the centre jumps there and drags from there, and the blue part moves with it.

## Known limits, on purpose

- Iris Wipe's picture leaves out the Feather. It draws the circle where the edge is sharp, since the box does not know the drawing's size in pixels. Radial Wipe's picture puts the edge at the middle of its feather.
- A dial shows the number as a direction clockwise from up. For Venetian Blinds 0 lies level, and for Mirror 0 keeps the right half, so the dial's line is not the slat or the mirror line. The Mirror box shows the line itself.
- Camera Shake's Rotation is the most a jolt tips, not a direction, so it has no dial.
- The boxes use the composition's shape, which is the drawing's shape only when the layer fills the composition (as in W-27).

## What checks it by machine

- `verification/B-12b_state_fields_table.md`: every card still sends every setting the command reads. Rerun on 2026-09-27 after this change, unchanged.
- `verification/B-12c_keyboard_table.md`: every command a picture sends can still be sent without a mouse. Rerun, unchanged.
- The app's own tests: 65 of 65 pass.

## What to answer

"works", or which step number did something else and what it did.
