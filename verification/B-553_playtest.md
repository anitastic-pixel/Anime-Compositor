# B-553: Mercury

Built on 2026-10-13 as D-674 under your /loop request: **Mercury**, in **Generate** beside Hair,
after what CycoreFX's CC Mr. Mercury is for. It turns the layer into drops of liquid metal. From the
layer's start, drops are born at a producer, thrown out and pulled down. Each drop swells in and
shrinks out over its life, and drops run together where they meet, lit like a shiny liquid. Each
drop takes the picture's colour where it was born, and the layer becomes the drops alone.

Nothing is remembered from frame to frame. Every frame works out each drop's path from the layer's
start, so any frame is the same picture whether it is drawn alone, scrubbed to or rendered out of
order.

The settings (the value when added in brackets):

- **Producer** (the middle), **Radius X** and **Radius Y** (10 each): where drops are born, and how
  big that area is, as a share of the layer's height.
- **Birth Rate** (20): how many drops are born each second. If you key it, the drops are counted
  from the layer's start.
- **Longevity** (2): how many seconds a drop lives. Only its value at the layer's start is used.
- **Animation** (Explosive): how the drops move. Explosive throws them every way; Direction throws
  them one way; Twirl spins them round the producer like a pinwheel; Vortex circles them round
  it; Fixed keeps them where they were born.
- **Direction** (0), **Velocity** (50), **Extra** (50), **Gravity** (50), **Resistance** (0): the
  way and speed they are thrown, how much randomness is added, how hard they fall, and how fast
  the air slows them.
- **Blob Influence** (100): how much drops run together. At 0 they are hard separate discs.
- **Influence Map** (In and Out), **Blob Birth Size** (100), **Blob Death Size** (50): how drops
  grow and shrink over their lives.
- **Light and Shading** (the same as Drizzle's): Light Intensity, Light Color, Light Type, Light
  Height, Light Position, Light Direction, Ambient, Diffuse, Specular, Roughness, Metal.

CycoreFX publishes no formula and no values when added, so the rule and those values are ours. They
are written down in document 21 and in `tools/mercury_reference.py`.

The check is `verification/D-674_mercury_table.md`, and 217 of 217 checks pass. It
compares every pixel with numbers that a separate program worked out before the code existed. It
also checks that the graphics card's picture stays within 1 level of the processor's (56 of
56 comparisons). The pictures are in `verification/D-674 pictures/` and use the street picture
from D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png` (frame 24): about ten shiny drops in the street's blue
   and yellow, scattered round the middle on a clear background (white here). Some are round, some
   have run together into longer blobs.
2. **Fountain.** `2_fountain.png` (frame 48): mostly dark grey drops from the lower part of the
   picture, a big clump of them run together above the bottom middle, with smaller drops and a few
   pale ones scattered round and below it.
3. **Puddle.** `3_puddle.png` (frame 48): one big lumpy blob filling most of the picture, where
   the fixed drops have all run together, shiny on top and taking reds and greens from the street
   lower down.
4. **Vortex.** `4_vortex.png` (frame 48): dozens of drops of every size spread over the whole
   picture, singly and in small run-together groups. The circling shows when it plays, not in one
   still.
5. **Orange point light.** `5_orange_point_light.png`: a few big drops, two of them run together
   into a curved blob. Each has a small orange highlight and an orange glow on the right-hand
   edge.
6. **Unlit.** `6_unlit.png`: the same drops with only ambient light: flat blue and yellow colour
   with soft edges, no shine.
7. **In the app.** Put Mercury (Generate) on the street picture and play. Drops should spray out,
   fall and fade. Try each Animation.
8. **Scrub.** Jump to frame 40, then play from 0 to 40. Frame 40 should be the same picture both
   times.
9. **Keyed birth rate.** Key Birth Rate from 0 to 40. The spray should start empty and thicken
   smoothly.
10. **Out of range.** Type 2000 in Gravity. It should be refused with a sentence that gives the
    range.
11. **Saved and opened again.** Save, close and open the project. The settings and the picture
    should be the same.

Choices of ours that you may want to judge:
- the values when added and the units;
- the shape of the drops where they run together;
- that only five ways of moving are offered (Explosive, Direction, Twirl, Vortex, Fixed);
- that there is no seed (CC Mr. Mercury has none);
- that After Effects' own lights are not offered.

## Speed

`verification/B-553_mercury_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 31.8 / 48.8, as added 36.1 / 70.0; Mercury, forty big drops a second running together, a point light 38.6 / 74.0; Mercury, birth rate keyed 0 to 40 (the settings read back to the in point) 48.7 / 73.8; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
