# B-301: Light Burst

Built on 2026-10-10 as D-422, under your /loop request: our version of CycoreFX's CC Light Burst
2.5, in **Generate**. The whole layer becomes light that bursts into rays streaking outward from a
centre, brightening the picture, like light pouring through and past a shape.

- **Center** (50, 50 per cent): the point the rays burst from.
- **Intensity** (100): how bright the rays are; 0 leaves the layer as it was, more than 100 goes
  past white.
- **Ray Length** (50): how far each ray streaks, 0 to 100.
- **Burst** (Straight): Straight streaks outward evenly, Fade streaks outward fading as it goes,
  Center streaks half outward and half inward toward the centre.
- **Set Color** (Off), **Color** (white): with Set Color on the rays are all this colour.

CycoreFX's manual names these settings but gives no formula, ranges or starting values, so those
are ours. CC's Halo Alpha setting is not built, and the manual's tutorial asks for Ray Length 150,
which is past our 0 to 100: both are logged. Light Rays, which shares the drawing of the rays,
draws exactly as before.

The check, `verification/D-422_light_burst_table.md` (205 of 205), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-422 pictures/`
(`town.png` is the layer).

## What to check

1. **Before.** `1_before.png`: the street with no effect.
2. **As added.** `2_as_added.png`: the street brighter, the houses and road streaking outward from
   the middle of the picture, more toward the edges.
3. **The manual's tutorial.** `3_tutorial.png`, Intensity 1200, Ray Length 35: on a layer with no
   see-through parts the street is blown out almost to white, streaks showing only in the road.
4. **Fade.** `4_fade.png`, Ray Length 80: the streaks fade as they run out, softer than as added.
5. **Center.** `5_center.png`, Ray Length 80: each house streaks both toward the middle and away.
6. **Set Color.** `6_set_color.png`, orange, Intensity 150, Center 25, 40: the street covers its
   whole frame, so its light is even and the picture is washed orange with no streaks. The rays
   only take the layer's shape where the layer has see-through parts (try it on text or a logo).
7. **In the app.** Add a text layer with a word, add Light Burst (Generate) to it, and drag Center
   about the viewer's numbers: the rays follow. Try each Burst, raise Intensity and Ray Length,
   turn Set Color on and pick a colour. Key Ray Length from 0 to 100: the rays grow out of the
   word.
8. **Out of range.** Type 101 in Ray Length: it is refused with a sentence saying it runs from 0 to
   100.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-301_light_burst_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Light Burst on three layers, played again, 31.0 ms a frame on the card as added
against 12.4 for the Noise alone; on the processor 1002.8 against 40.8. Fade at Ray
Length 100 takes 30.5 on the card against the processor's 1410.3: the card is quicker than the processor for this effect.
