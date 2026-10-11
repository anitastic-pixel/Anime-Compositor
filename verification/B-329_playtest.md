# B-329: Block Load

Built on 2026-10-11 as D-449, under your /loop request and your go-ahead to finish it: **Block
Load**, in **Stylize**, after After Effects' CC Block Load. It brings the layer in the way an
old web page loaded a picture: big square blocks first, then finer and finer passes, each
sweeping down from the top, until the picture is sharp.

- **Completion** (0%, 0 to 100): how far the loading has got. Key it from 0 to 100 to play it.
- **Scans** (4, 1 to 16): how many passes; more passes start from bigger blocks (16 pixels
  square at 4, 8 at 3).
- **Start Cleared** (on): on, nothing shows at 0%; off, the roughest blocks show at once.
- **Bilinear** (off): on, the blocks are blended smoothly into each other instead of flat.

The controls are After Effects' own; how they draw is our own rule beyond what the sources show,
not matched to After Effects pixel for pixel.

The check, `verification/D-449_block_load_table.md` (159 of 159), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-449 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **Roughest.** `2_coarsest.png` (Start Cleared off, Completion 0): the street in big blocks.
2. **Half way.** `3_half_way.png` (Completion 50): the top part in small blocks, the lower part
   still in bigger ones, with a straight line where the pass has reached.
3. **Smoothed.** `4_half_way_bilinear.png`: the same with Bilinear on, blurry instead of blocky.
4. **Nearly done.** `5_nearly_done.png` (Completion 90): the top sharp, the bottom in small
   blocks.
5. **In the app.** Add Block Load (Stylize) to a picture. Key Completion from 0 at the start to
   100 a second later and play it: blocks, then finer blocks sweeping down, then the picture.
   Try Scans 2 and 8, Start Cleared off, and Bilinear on.
6. **Out of range.** Type 17 in Scans: it is refused with a sentence giving the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-329_block_load_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Block Load on three layers, played again, 60.4 ms a frame on the card at
completion 50 against 53.9 for the Noise alone; on the processor 216.2 against
117.1: a busy machine cannot say whether the card is quicker than the processor for this effect.
