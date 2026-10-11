# B-426: Glint Streaks

Built on 2026-10-11 as D-546, your pick #15 (Glint Streaks). It is not a new effect: **Cross
Glare** gains four settings, as `docs/effects/PLUGINS.md` settles ("a colour per ray and shimmer
for Cross Glare"), after the Glimmer plugin's rays with their own colours and shimmer. Glimmer
publishes no formula, so how the colours mix and how the shimmer moves are our own.

- **Ray Colours** (One Colour as added): One Colour tints every ray with Colour, as before;
  **Alternate** gives every other ray the Second Colour; **Gradient** runs the rays from Colour
  round the star to the Second Colour.
- **Second Colour** (light blue as added): the other colour.
- **Shimmer** (0 as added): how far each ray dims as it shimmers, 0 to 100 per cent. At 0 the
  rays hold still.
- **Shimmer Period** (24 frames): how long a ray takes to dim and come back. Each ray is on its
  own beat, so the rays never all dim at once.

Shimmer and Shimmer Period can be keyed. With One Colour and no shimmer, Cross Glare draws
exactly as before.

Not built: picking a colour for each ray one by one, more than eight rays, rays of different
lengths, and shimmer that shortens the rays instead of dimming them.

The check, `verification/D-546_glint_streaks_table.md` (203 of 203 checks), holds every pixel to numbers
worked out by a separate program before the code existed, runs Cross Glare's 29 earlier checks
again unchanged, and holds the graphics card's picture to within 1 level of the processor's. The
pictures are in `verification/D-546 pictures/`.

## What to check

1. **One colour.** `town_glare.png`: the town with Cross Glare, white rays, for comparison.
2. **Alternate.** `town_alternate.png`: every other ray of each star orange, the rest white.
3. **Gradient.** `town_gradient.png`: eight rays from red to blue round each star.
4. **Shimmer.** `town_shimmer_0.png` and `town_shimmer_6.png`: Shimmer 100 at frames 0 and 6:
   some rays bright and some dim, and a quarter period later they have changed, each its own way.
5. **In the app.** Add Cross Glare to a layer with bright parts. Set Ray Colours to Alternate
   and pick a Second Colour; then Gradient with 6 or 8 points.
6. **Shimmer playing.** Set Shimmer to 100 and play: the rays dim and come back, each on its own
   beat, and the loop repeats every Shimmer Period frames. Set the period to 8 for a quick
   flicker, 48 for a slow breath.
7. **Out of range.** Type 101 in Shimmer: it is refused with a sentence saying the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the shimmer's shape (a smooth dim and back, never to
black unless Shimmer is 100) and how out of step the rays are.

## Speed

On the graphics card, measured while other builds were running (so provisional), the reference shot played again took 44.6 ms a frame with gradient colours and full shimmer and 42.2 with alternating colours, against 40.1 for Cross Glare as before: the new settings cost little.
