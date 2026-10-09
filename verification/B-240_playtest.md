# B-240: Spin & Zoom Blur

Built on 2026-10-08 under your effects loop request, decided as D-361. **Spin & Zoom Blur** (in
**Blur & Sharpen**) is our version of CycoreFX's CC Radial Blur: a camera zooming or turning about
a centre while the shutter is open. It sits beside our Radial Blur, which it extends with more
kinds of streak and a quality setting.

The settings, with the values they start at:

- **Type** (Straight Zoom):
  - **Straight Zoom**: everything streaks outward from the centre, evenly.
  - **Fading Zoom**: the same streaks, fading as they run out.
  - **Centered Zoom**: streaks both outward and inward, half as long each way.
  - **Rotate**: everything streaks round the centre one way, as a wheel turning.
  - **Rotate Fading**: the same, fading.
  - **Scratch**: the turn spread both ways, as Radial Blur's spin.
- **Amount** (10): degrees for the turns, per cent of the distance for the zooms, -360 to 360.
  Negative turns the other way, or zooms inward.
- **Quality** (50): 1 to 100. Low values show separate copies instead of a smooth streak; high
  values are smoother and slower.
- **Centre** (50, 50): per cent of the layer's width and height.

The check, `verification/D-361_spin_zoom_blur_table.md` (142 of 142), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-361 pictures/`.

## What to check

**`wheel.png`** is a car wheel on a pale road: a dark tyre, a silver rim with five dark spokes and
a dark hub. **`before.png`** is the wheel with no effect.

1. **Turning.** `rotate_20.png` is Rotate 20 about the middle. The spokes should smear round into
   a blur, while the hub and the tyre, the same all the way round, stay as they were.
2. **Zooming.** `straight_zoom_30.png` is Straight Zoom 30. The tyre's inner edge should smear
   inward toward the hub; the spokes, lying along the streaks, stay sharp.
3. **Fading.** `fading_zoom_30.png` is Fading Zoom 30: the same streaks as picture 2, but softer
   at their ends, so the tyre's edge moves less.
4. **Scratch.** `scratch_20.png` is Scratch 20: smeared like picture 1, but evenly both ways, so
   the spokes stay centred where they were.
5. **In the app.** Put Spin & Zoom Blur on a layer, choose Rotate and key Amount from 0 to 40:
   a wheel spinning up. Try Quality 5 to see separate copies, then 100. Move the Centre.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

With Draw on: GPU Spin & Zoom Blur as added costs about 2 ms a 1080p layer, Rotate 30 about 5 ms
(`verification/B-240_spin_zoom_blur_timing_table.md`).

## Not built

- The wheel tutorial keeps the blur inside the wheel with an ellipse mask that follows the car.
  Masks are there already; following the car with a tracked mask is not.
