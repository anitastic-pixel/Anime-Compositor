# B-385: Unmult

Built on 2026-10-10 as D-505, under your /loop request: Unmult, in **Lines & Mattes** (beside
the other keyers), with After Effects' settings (Adobe added it in After Effects 26.0). It turns a
black or white background into see-through, so fire, flares or smoke shot on black (or ink and
smoke shot on white) lay over other layers without a box around them.

- **Background Color** (Black): the background to remove, black or white.
- **Black Level** (0), used with Black: anything this dark or darker goes completely.
- **White Level** (100), used with White: anything this light or lighter goes completely.
  After Effects shows one level whose name follows the background; we show both, because the
  panel cannot hide one, and only the one for the chosen background does anything.
- **Softness** (0): how gradually pixels just past the level come back instead of all at once.
- **Remove Color Matting** (On): brings back each pixel's colour from before the background, so
  a faint orange glow stays orange instead of turning muddy. Off keeps the dimmed colour.
- **Clip HDR Results** (Off): only matters for colours brighter than white (from bright
  footage or an Exposure before it); On holds them at white.

Adobe publishes no formula and no settings when added, so the rule and those settings are ours,
written down in document 21. One known difference: we work in linear light, where After
Effects' usual project is 8-bit and gamma-encoded, so the same footage can keep a little more or
less covering than there (here a mid grey keeps about a fifth).

The check, `verification/D-505_unmult_table.md` (132 of 132), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to within 1 level of the processor's. The pictures are in
`verification/D-505 pictures/`; the ones ending in "over sky" lay the result over a blue sky so
you can see what is see-through.

## What to check

1. **The shot.** `1_flare_before.png`: an orange flare with a white core on black, and a grey
   square. `1b_flare_before_over_sky.png`: laid over the sky as it is, a black box covers the sky.
2. **As added.** `2_flare_unmult.png` and `2b_flare_unmult_over_sky.png`: the black is gone; over
   the sky the flare glows with no box, its faint edge still orange, and the grey square turns
   into a see-through white.
3. **Black Level 10, Softness 20.** `3_black_level_10_softness_20.png` and `3b`: the faintest
   part of the glow is gone too, and the grey square is fainter.
4. **Remove Color Matting off.** `4_remove_color_matting_off.png` and `4b`: the same see-through
   parts, but the faint glow and the grey square keep their darkened colour, so over the sky the
   glow's edge looks dirtier.
5. **Smoke on white.** `5_smoke_before.png`: dark smoke and a grey square on white.
   `5b_smoke_unmult_white.png` (Background Color White) and `5c_smoke_unmult_white_over_sky.png`:
   the white is gone and the smoke lies over the sky as soft dark smoke.
6. **In the app.** Add Unmult (Lines & Mattes) to fire or flare footage on black, lay it over
   another layer, try the levels, Softness and Remove Color Matting, and switch to White on
   footage shot on white. Keyframe Black Level and play it.
7. **Out of range.** Type 101 in Black Level: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the settings when added; how much covering a dim pixel
keeps (compare with After Effects' own Unmult on the same shot); and the two level settings in
place of After Effects' one.

## Speed

Measured on a quiet machine (`verification/B-385_unmult_timing_table.md`). Three Unmults on a 1920 by 1080 shot,
card / processor, milliseconds a frame played again: the shot after a moving Noise 12.1 / 40.8 ms a frame, as added (black) 17.3 / 42.9, white, white level 80, softness 20 17.5 / 43.3, black level 20, softness 10, matting off 17.2 / 43.2; about 1.7 to 1.8 ms a 1080p layer on the card, over the row's Target P1 (1 ms), as the other keyers are; on the processor about 0.7 to 0.8 ms a layer.
