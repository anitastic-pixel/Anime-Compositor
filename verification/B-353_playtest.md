# B-353: Basic Text

Built on 2026-10-11 as D-473, under your /loop request: **Basic Text**, in **Obsolete** (and
under Text in the Effects list, next to Numbers), after After Effects' old Basic Text. Old
projects and tutorials use it; it writes words you type straight onto the layer it is on,
where a text layer is a layer of its own. It draws its words exactly as Numbers draws its number.

- **Text** (Text): the words, up to 1000 characters.
- **Font** (the bundled rounded font): a font file's name, as a text layer names one.
- **Position** (50, 50): per cent of the width and height, where the first line's baseline meets
  its aligned end.
- **Display Options** (Fill Only, Stroke Only, Fill Over Stroke, Stroke Over Fill).
- **Fill Color** (white) and **Stroke Color** (orange), **Stroke Width** (3, 0 to 500 pixels).
- **Size** (64, 1 to 2000 pixels), **Tracking** (0, thousandths of the size added after each
  letter), **Alignment** (Center, Left, Right), **Composite On Original** (On, Off).

Not built: Style (pick a font file of that style instead), Direction (Vertical and Rotate: our
text engine writes across only) and Line Spacing (the font's own line height is used). The Text
box is one line; two lines come only from a saved file.

The check, `verification/D-473_basic_text_table.md` (138 of 138), holds every case's frame to
a second project, written by a separate program before the code existed, that draws the same
words as plain text layers; it holds the graphics card's picture to within 1 level of the
processor's. The pictures are in `verification/D-473 pictures/`.

## What to check

1. **As added.** `1_as_added.png`: the word Text in white in the middle of the plate.
2. **Placed.** `2_hello_world.png`: Hello, world, smaller, starting near the left edge;
   `3_spaced_right.png`: Spaced with its letters set wide, ending near the right edge.
3. **Outlines.** `4_stroke_only.png`: Ring as an orange outline with the plate inside the
   letters; `5_stroke_over_fill.png`: Edge in white with a thick orange edge.
4. **On its own.** `6_composite_off.png`: Alone in light blue with a dark blue edge, on a
   see-through background (white or a checkerboard, by the picture viewer), the plate gone.
5. **Two lines and kana.** `7_two_lines.png`: Basic above Text; `8_kana.png`: konnichiwa.
6. **Keyed.** `9_grow_frame_5.png`: Grow halfway through its size keys.
7. **Missing font.** `10_missing_font.png`: the plain plate; the app warns that the font is not
   on this computer.
8. **In the app.** Add Basic Text (Text, next to Numbers) to a picture; type your words in Text
   and press Enter; drag Size and Position; switch Display Options and Alignment.
9. **Out of range.** Type 2001 in Size: it is refused with a sentence giving the range.
10. **Saved and opened again.** Save, close and open the project: the same words and picture.

## Speed

In `verification/B-353_basic_text_timing_table.md` (quiet): the reference shot with a moving Noise
and a Basic Text at size 200 on each of three layers, played again, 37.8 ms a frame on the
card against 12.6 for the Noise alone; on the processor 62.8 against 44.4:
the card is quicker than the processor for this effect.
