# B-330: Noise Alpha

Built on 2026-10-10 as D-450, under your /loop request: After Effects' Noise Alpha, in
**Generate** (beside Add Grain; the app has no Noise & Grain group). It speckles how solid the
layer is (its covering, or alpha): some pixels become partly see-through, and, if you ask, some
empty ones gain a little black.

- **Noise** (Uniform Random): *Uniform* speckles evenly; *Squared* makes them more contrasting,
  more pixels near the full strength. The *Random* kinds stay still; the *Animation* kinds change
  as Noise Phase turns.
- **Amount** (20%): how strong the speckles are; 0 turns it off.
- **Original Alpha** (Clamp): where the speckles go. *Clamp*: only the fully solid parts. *Add*:
  everywhere, empty parts included. *Scale*: in proportion to how solid each pixel is, so empty
  parts stay empty. *Edges*: only the soft, partly see-through edges.
- **Overflow** (Clip): what happens when a speckle would push a pixel past solid or past empty.
  *Clip* stops it there; *Wrap Back* bounces it back; *Wrap* comes round from the other end, so a
  solid pixel pushed further can all but vanish.
- **Random Seed** (0): a different pattern, for the Random kinds only.
- **Noise Phase** (0°): with an Animation kind, turning it moves through the noise, a whole new
  pattern each full turn. **Cycle Noise** (Off) and **Cycle** (1): with Cycle Noise on, the pattern
  comes back to the start every Cycle turns, so it loops.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. Every one of After Effects' settings is
offered. The speckles belong to the drawing, so they move with the layer.

The check, `verification/D-450_noisealpha_table.md` (210 of 210), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-450 pictures/`
(`1_before.png` is the street with nothing on it; see-through pixels show whatever your picture
viewer puts behind them, often a checkerboard or white).

## What to check

1. **As added.** `2_as_added.png`: the street faintly speckled, about half its pixels a little
   see-through, the colours unchanged.
2. **Squared and stronger.** `3_squared_60.png`: much stronger, more contrasting speckles than
   picture 2.
3. **Wrap.** `4_wrap_100.png`: Amount 100 with Wrap: a heavy speckle where some pixels have all
   but vanished.
4. **It moves.** `5_animation_frame_24.png`: an Animation kind half way through the shot, a
   speckle of its own.
5. **In the app.** Put Noise Alpha (Generate) on a layer with soft edges, over a coloured
   background. Clamp: the solid middle speckles, the edges stay smooth. Edges: only the soft
   edges speckle. Add: the empty area around the layer gains a faint black speckle. Scale: the
   empty area stays empty. Choose Uniform Animation and key Noise Phase from 0 to 720: the speckle
   moves smoothly as you play; Random Seed now changes nothing. Turn Cycle Noise on with Cycle 1:
   the pattern loops every turn. Move the layer: the speckle moves with it.
6. **Out of range.** Type 101 in Amount: it is refused with a sentence saying it runs from 0 to
   100.
7. **Saved and opened again.** Save, close and open the project: the same settings, keys and
   picture.

Our own choices you may want to judge (After Effects says nothing on them): the Random kinds
stay still from frame to frame; Squared pushes the speckles towards full strength; Clamp touches
only fully solid pixels; empty pixels that gain covering under Add or Wrap turn black.

## Speed

Measured on a quiet machine
(`verification/B-330_noisealpha_timing_table.md`). Three Noise Alphas on a 1920 by 1080 shot, card / processor, milliseconds a
frame played again: Noise alone 12.1 / 40.3 ms a frame, as added (Uniform Random, Clamp) 13.3 / 44.9, Squared Random, Add, Wrap Back 13.5 / 51.0, Uniform Animation, Scale 13.4 / 45.8.
