# B-276: Color Grade, first part

Built on 2026-10-09 under your effects loop request, decided as D-397. **Color Grade** (in
**Color Correction**) is our own version of Lumetri Color: a whole colour grade in one effect.
This first part has three of Lumetri's sections; Curves, Color Wheels and HSL Secondary come next
as their own part.

- **Basic Correction**: Temperature and Tint (white balance), Exposure (stops), Contrast,
  Highlights, Shadows, Whites, Blacks and Saturation.
- **Creative**: Look (a lookup file of the project, as Color Lookup takes), Look Intensity,
  Faded Film, Vibrance, Creative Saturation, and a Shadow Tint and a Highlight Tint (a hue and an
  amount each) with a Tint Balance between them.
- **Vignette**: Amount, Midpoint, Roundness and Feather.

Every setting starts where Lumetri's does, so the effect as added changes nothing. Every number
can be keyed. The steps run in Lumetri's order: Basic, then the look, then the rest of Creative,
then the vignette.

The check, `verification/D-397_color_grade_table.md` (196 of 196), holds every pixel to numbers
worked out by a separate program before the code existed, and the graphics card's picture to
within 1 level of the processor's. The pictures are in `verification/D-397 pictures/`.

## What to check

**`1_before.png`** is the street with no effect.

1. **Warmer.** `2_warm.png`: Temperature 60. The street is warmer, yellow-orange, and about as
   bright as before.
2. **Teal and orange.** `3_teal_orange.png`: Contrast 30, a teal Shadow Tint and an orange
   Highlight Tint. The dark road turns teal, the sky and light walls warmer.
3. **A look.** `4_look.png`: the test's own look file (`look.cube`, in the same folder) at
   intensity 100: a little more contrast, a little less colour, cool shadows, warm lights.
4. **Faded film.** `5_faded_film.png`: Faded Film 70. The darkest parts lifted to a grey, the
   picture washed out like an old print.
5. **Vignette.** `6_vignette.png`: Vignette Amount -3. The corners darker, the middle as it was.
6. **As added.** Add Color Grade and change nothing: the picture should not change at all.
7. **In the app.** Key Exposure from 0 to 2 over a second: the picture should brighten steadily.
   Try a positive Vignette Amount: the edges go white instead of black. Roundness 100 makes it a
   circle; Feather 0 gives it a hard edge.
8. **Choosing a look.** Import a .cube file, then pick it as the Look the same way you pick a
   Color Lookup's file. The other settings should stay as they were. Set Look Intensity to 0:
   the look disappears; 200 pushes it twice as far.
9. **Saved and opened again.** Change several settings, pick a look, save, close and open again.
   You should get the same settings and the same picture.

**Timing.** Provisional, measured while the machine was busy (`verification/B-276_color_grade_timing_table.md`): about 3.7 ms a layer on the graphics card for the basic settings, about 7.8 ms with every step but the look. To be measured again on a quiet machine.

## Not built

- **Curves, Color Wheels and HSL Secondary.** The next part (D-398).
- **Adobe's bundled looks.** Lumetri comes with Adobe's own looks; they are Adobe's and are not
  copied. Any .cube file of your own works as the Look.
- **Sharpen** (in Lumetri's Creative section) is left out; the Sharpen effect does that job.
- **A squarer vignette.** Lumetri's Roundness goes below 0 for a squarer shape; ours runs from 0
  (the frame's shape) to 100 (a circle).
- **Highlights past white.** Lumetri can bring back highlights brighter than white in some
  footage; ours works on colours from black to white.
- **The method is ours.** Adobe does not publish how Lumetri works. Each section uses a rule
  this program already had where it could: Vibrance's for the saturations, Color Lookup's for
  the look, Color Balance's for the tints and Vignette's for the vignette.
