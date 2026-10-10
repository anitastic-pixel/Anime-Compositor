# B-331: Noise HLS

Built on 2026-10-10 as D-451, under your /loop request: After Effects' Noise HLS, in
**Generate** (beside Noise Alpha; the app has no Noise & Grain group). It flecks a picture's
colours: each pixel's hue (its place round the colour wheel), lightness and saturation (how
strong its colour is) move by a small random amount of their own.

- **Noise** (Uniform): *Uniform* flecks every pixel on its own; *Squared* makes the flecks more
  contrasting, more pixels moved nearly the full amount; *Grain* is soft, neighbouring pixels
  moving together like a film's grain.
- **Hue** (0%): how far the colours turn round the wheel, up to half way round at 100.
- **Lightness** (10%): how much lighter or darker each pixel gets.
- **Saturation** (0%): how much stronger or weaker each colour gets.
- **Grain Size** (1): for Grain only, how big the soft blotches are, in pixels.
- **Noise Phase** (0°): turning it moves through the noise, a whole new pattern each full turn;
  key it to make the flecks move.

After Effects publishes no formula or starting values for this effect, so how each pixel is
worked out is our own rule, written down in document 21. Every one of After Effects' settings is
offered. The flecks belong to the drawing, so they move with the layer. See-through parts stay
see-through.

The check, `verification/D-451_noisehls_table.md` (184 of 184), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-451 pictures/`
(`1_before.png` is the street with nothing on it).

## What to check

1. **As added.** `2_as_added.png`: the street finely flecked lighter and darker, the colours
   otherwise the same.
2. **Hue.** `3_hue_60.png`: the colours flecked far round the wheel (the blue sky a mix of
   purples, blues and greens, the buildings likewise), while the grey road stays grey.
3. **Grain.** `4_grain_4_saturation_60.png`: soft blotches of stronger and weaker colour, a few
   pixels across, rather than single-pixel flecks.
4. **It moves.** `5_phase_frame_24.png`: Squared, with Noise Phase keyed, half way through the
   shot: strong flecks of its own pattern.
5. **In the app.** Put Noise HLS (Generate) on a colourful layer. As added it flecks lighter and
   darker. Set Lightness 0 and Hue 50: the colours fleck round the wheel while greys stay grey.
   Add Saturation 50: greys gain a little colour. Choose Grain and raise Grain Size: the flecks
   grow into soft blotches. Key Noise Phase from 0 to 720: the flecks change smoothly as you
   play. Move the layer: the flecks move with it.
6. **Out of range.** Type 101 in Hue: it is refused with a sentence saying it runs from 0 to 100.
7. **Saved and opened again.** Save, close and open the project: the same settings, keys and
   picture.

Our own choices you may want to judge (After Effects says nothing on them): Hue 100 turns a
colour up to half way round the wheel; Lightness 10 when added; a grey given Saturation turns
reddish first; a full turn of Noise Phase gives a whole new pattern.

## Speed

Measured on a quiet machine
(`verification/B-331_noisehls_timing_table.md`). Three Noise HLSs on a 1920 by 1080 shot, card / processor, milliseconds a
frame played again: Noise alone 12.9 / 44.6 ms a frame, as added (Uniform, lightness 10) 18.1 / 68.2, Squared, hue 40, lightness 20, saturation 40 18.3 / 68.6, Grain 2.5, hue 40, lightness 20, saturation 40 20.3 / 72.8; over Target P1: about 1.7 to 2.5 ms a layer on the card, against 1 ms (three value noises and an HSL round trip a pixel, in the grade pass's double precision).
