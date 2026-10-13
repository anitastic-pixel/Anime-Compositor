# B-551: Drizzle

Built on 2026-10-12 as D-672 under your /loop request: **Drizzle**, in **Generate** beside
Bubbles, after what CycoreFX's CC Drizzle is for. It is raindrops falling on a pond. From the
layer's start, drops land at random places. Each drop makes a ring of ripples that spreads out and
dies away, bending the picture underneath and catching the light.

Nothing is remembered from frame to frame. Every frame counts the drops from the layer's start, so
any frame is the same picture whether it is drawn alone, scrubbed to or rendered out of order.

The settings (the value when added in brackets):

- **Drip Rate** (10): how many drops land each second. If you key it, the drops are counted from
  the layer's start.
- **Longevity** (1): how many seconds a ripple takes to spread and fade. As in CC Drizzle it
  cannot be animated: only its value at the layer's start is used.
- **Rippling** (360): how many rings a ripple has. Each full turn of the dial adds a ring.
- **Displacement** (50): how far the ripples bend the picture. Below 0 bends it the other way.
- **Ripple Height** (50): how tall the water stands. This changes both the bending and the light.
- **Spreading** (50): how wide a ripple grows. At 50 its radius reaches an eighth of the layer's
  height.
- **Light and Shading** (the same as Glue Gun's): Light Intensity, Light Color, Light Type
  (Distant or Point), Light Height, Light Position, Light Direction, Ambient, Diffuse, Specular,
  Roughness, Metal.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/drizzle_reference.py`.

The check is `verification/D-672_drizzle_table.md`, and 211 of 211 checks pass. It compares every
pixel with numbers that a separate program worked out before the code existed. It also checks that
the graphics card's picture stays within 1 level of the processor's (54 of 54 comparisons). The
pictures are in `verification/D-672 pictures/` and use the street picture from D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png` (frame 24): a few small ring ripples across the street.
   Each is lighter on the side facing the light at the top left and darker on the other side.
2. **Downpour.** `2_downpour.png` (frame 48): thirty drops a second, each with three rings and twice
   as wide. The sky is full of overlapping ripples, and the buildings bend inside them.
3. **Point light.** `3_point_light.png`: the same drops as picture 1, lit by a lamp in the middle.
   There is a bright glow round the middle of the picture, and the ripples catch it on their inner
   sides.
4. **Orange shine.** `4_orange_shine.png`: an orange light. The whole picture takes the orange
   light, and the ripples' shine is orange, because the light's colour tints everything it lights.
5. **Big slow ripples.** `5_big_slow_ripples.png` (frame 48): two drops a second that each last
   three seconds and spread very wide. Look for large soft rings, and windows bent inside them.
6. **Bent, unlit.** `6_bent_unlit.png`: the light turned off (ambient only). The picture is only
   bent: one ripple warps the blue building, and the rest of the street is untouched.
7. **In the app.** Put Drizzle (Generate) on the street picture and play. Drops should land and
   their rings spread and fade. Turn Rippling up for more rings, and Spreading up for wider ones.
8. **Scrub.** Jump to frame 40, then play from 0 to 40. Frame 40 should be the same picture both
   times.
9. **Keyed drip rate.** Key Drip Rate from 0 to 40 over a few seconds. The rain should start with
   no drops and thicken smoothly, with no jump.
10. **Out of range.** Type 2000 in Drip Rate. It should be refused with a sentence that gives the
    range.
11. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added;
- the units of Spreading, Displacement and Ripple Height;
- the shape of a ripple (it rises and falls over its life, its rings flattening at the edge);
- that there is no seed (CC Drizzle has none);
- that After Effects' own lights are not offered.

## Speed

`verification/B-551_drizzle_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 14.0 / 58.2, as added 26.4 / 96.5; Drizzle, thirty drops a second, Rippling 720, Spreading 150, a point light 26.7 / 112.9; Drizzle, drip rate keyed 0 to 40 (the settings read back to the in point) 26.8 / 139.2; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
