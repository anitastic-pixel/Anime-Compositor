# B-550: Bubbles

Built on 2026-10-12 as D-671 under your /loop request: **Bubbles**, in **Generate** beside
Particle Space, after what CycoreFX's CC Bubbles is for. The layer is turned into bubbles that rise
(or fall) and wobble from side to side, each one showing the picture through itself, like a glass
ball. When a bubble leaves the top it comes round again from below.

Nothing is remembered from frame to frame: every frame works each bubble out from the layer's
start. So any frame, drawn alone, scrubbed to or rendered out of order, is the same picture.

The settings (the value when added in brackets):

- **Bubble Amount** (100): how many bubbles.
- **Bubble Speed** (0.5): how fast they rise; below 0 they fall. At 1 a bubble crosses a quarter
  of the layer's height a second.
- **Wobble Amplitude** (5) and **Wobble Frequency** (1): how far and how often they swing side to
  side.
- **Bubble Size** (1): at 1 a bubble's radius is about a fortieth of the layer's height, each bubble
  between half and one and a half times that.
- **Random Seed** (0): a different seed scatters them differently.
- **Reflection Type** (Inverse): Inverse shows the picture turned over and spread, as a glass ball
  does; World shows the picture round the bubble upright.
- **Shading Type** (Fade Inwards): None, Lighten (white towards the rim), Darken (black towards the
  rim), Fade Inwards (clear in the middle, solid at the rim, like a soap bubble), Fade Outwards
  (solid in the middle, clear at the rim).

The layer becomes the bubbles alone, on clear (shown white in the pictures). CycoreFX publishes no
formula and no settings when added, so the rule and those settings are ours, written down in
document 21 and in `tools/bubbles_reference.py`.

The check, `verification/D-671_bubbles_table.md` (201 of 201), holds every pixel to numbers worked
out by a separate program before the code existed, and holds the graphics card's picture to within
1 level of the processor's (52 of 52 comparisons). The pictures are in `verification/D-671 pictures/`,
made from the street picture of D-506.

## What to check

1. **As added.** `1_as_added_frame_24.png`: many small bubbles, clear in the middle and coloured at
   the rim, the sky's blue in the top ones, the houses in the middle, the road at the bottom.
2. **Soap bubbles.** `2_soap_bubbles.png`: thirty large bubbles; inside each the street is turned
   over (look for the road's white dashes near the top of the dark bottom bubbles).
3. **World and Lighten.** `3_world_lighten.png`: the same bubbles with the picture upright and the
   rims going to white.
4. **Darken, falling.** `4_darken_falling.png`: sixty bubbles falling, solid, darker at the rims.
5. **Fade Outwards.** `5_fade_outwards.png`: solid in the middle, fading to clear at the rims.
6. **One big bubble.** `6_one_fisheye.png`: a single bubble in the middle showing the whole street
   upside down and bent, as through a glass ball.
7. **In the app.** Put Bubbles (Generate) on the street picture and play: the bubbles rise and
   wobble. Set Bubble Speed to -1: they fall. Switch Reflection Type and each Shading Type.
8. **Scrub.** Jump to frame 40, then play from 0 to 40: the same picture at frame 40.
9. **Keyed speed.** Key Bubble Speed from 0 to 2 over a second: the bubbles start still and speed
   up smoothly, with no jump.
10. **Out of range.** Type 101 in Bubble Speed: it is refused with a sentence saying the range.
11. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; the units of speed, size and
wobble; how World reads the picture (the manual says only that it "reflects the source layer");
that the bubbles are on clear rather than over the layer (put the effect on a copy of the layer
above it to see the picture behind).

## Speed

`verification/B-550_bubbles_timing_table.md`. Three layers after a moving Noise, card / processor, ms a frame (PROVISIONAL, the machine was busy): Noise alone 16.7 / 52.6, as added 23.6 / 82.1; Bubbles, 1000 of size 3, World, Lighten 62.0 / 295.4; Bubbles, speed keyed 0 to 3 (the settings read back to the in point) 21.0 / 92.1; on the card an unknown amount a frame (the machine was busy) as added; a busy machine cannot say whether the card is quicker.
