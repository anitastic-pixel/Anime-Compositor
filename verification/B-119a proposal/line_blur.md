# B-119a: Line Blur, softening lines along their length (D-183)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build. It is inspired by OpenToonz's Line Blur, not a port: OpenToonz traces every line into curves first, several thousand lines of program, and this is a lighter rule of our own that looks at the pixels only.

## What you will see

A new effect, **Line Blur**, under Lines & Mattes in the Effects panel, beside Line Width. Its card has three rows, as OpenToonz's has its blur length and power first:

- **Length**, 0 to 50 px, 4 when added: how far along a line each pixel is mixed with its neighbours. Draft halves it with the picture.
- **Strength**, 0 to 100 %, 100 when added: how far the drawing moves towards the softened one.
- **Lines only**, off when added: when on, only dark pixels change, by how dark they are, so the fills and colour edges of a coloured cel are left alone.

What it does: each pixel finds which way the line through it runs, and is softened along that way only, never across it. A jagged diagonal, drawn without smoothing, softens into a slope; a line keeps its thickness; a clean straight line has nothing to smooth and stays as it is. The softening stops where a line ends, so a long Length does not eat lines from their ends. Corners, where the way is uncertain, change less. Length 0 or Strength 0 is the drawing untouched.

`line_blur.png` in this folder is worked by the reference rule itself, four times enlarged: jagged line art before, at Length 4 and at Length 10; then a coloured cel on white paper before, at Length 4, and at Length 4 with Lines only.

## Known limits

- It follows a straight line through each pixel, not the curve itself, so on a tight curve a long Length thins the line a little. Four to ten pixels suits most line art.
- A white line on a white-paper cel has nothing to find; a white line on an empty layer is found by its covering.
- It softens edges between colours inside a cel too, along the edge; Lines only stops that.
- It runs on the processor. The graphics card learns it in item 9, with the other new effects.

## How you will check it

The build's test draws the sixteen fixture cases and compares every pixel with the numbers `tools/line_blur_reference.py` worked out, and writes `verification/B-119_line_blur_table.md`, with pictures. A playtest sheet walks you through adding it to a drawing, moving the Length, and turning on Lines only.
