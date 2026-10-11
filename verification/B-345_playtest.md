# B-345: Scatter

Built on 2026-10-11 as D-465, under your /loop request and your go-ahead: **Scatter**, in
**Stylize**, after After Effects' Scatter. Every pixel of the layer is swapped for one a short,
random distance away, so the picture looks frosted, sandy or smeared, without any colour
changing.

- **Scatter Amount** (10, 0 to 127 pixels): how far a pixel may come from. 0 leaves the layer
  alone.
- **Grain** (Both): Both moves pixels any way; Horizontal only across, so the picture smears
  sideways; Vertical only up and down.
- **Randomize Every Frame** (Off): Off keeps the same jumble on every frame; On draws a new one
  each frame, so the grain fizzes when played.

The controls are After Effects' own (Adobe's page could not be read, so they come from Qt's
copies of the effect, which keep its names); how it draws is our own rule, so it is not matched
to After Effects pixel for pixel. The layer does not grow: near its edge some pixels come from
outside it and are left clear, so the edge looks frayed.

The check, `verification/D-465_scatter_table.md` (143 of 143), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-465 pictures/`: the
street (`town.png`) filling the frame (`1_before.png` without the effect).

## What to check

1. **As added.** `2_as_added.png`: the street jumbled finely, still easy to read.
2. **Further.** `3_amount_40.png`: amount 40, jumbled much further, the frame's edges frayed.
3. **Across only.** `4_horizontal_40.png`: the street smeared sideways, each row keeping its own
   colours.
4. **Down only.** `5_vertical_40.png`: smeared up and down, each column keeping its own colours.
5. **Every frame.** `6_randomize_frame_3.png`: with Randomize Every Frame on, frame 3 is a
   different jumble from frame 0.
6. **In the app.** Add Scatter (Stylize) to a picture, raise Scatter Amount, try the three Grain
   choices, then turn on Randomize Every Frame and play: the jumble should fizz.
7. **Out of range.** Type 128 in Scatter Amount: it is refused with a sentence giving the range.
8. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-345_scatter_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a
moving Noise and a Scatter as added on each of three layers, played again,
72.3 ms a frame on the card against 71.2 for the Noise alone; on the processor
189.5 against 121.0: a busy machine cannot say whether the card is quicker than the processor for this effect.
