# B-235: Path Stroke

Built on 2026-10-08 under your effects loop request, decided as D-356. This is P0-22, "effects
draw along paths": one shared piece that lets an effect follow a layer's masks and trim the line
with a Start and an End. It is proved by **Path Stroke**, our version of After Effects' Stroke
(Generate). Find it in the **Generate** group of the effects list. Vegas, Scribble, Write-on,
Fill, Audio Waveform and Energy Stroke can use the same piece later. They are not built yet.

Its settings, with After Effects' names:

- **Path**: which mask to follow, by number (1 the first). **All Masks**: follow every mask on
  the layer. **Stroke Sequentially** (with All Masks on): draw the masks one after another as one
  line, instead of all at once.
- **Color**, **Brush Size** (2 when added, like After Effects), **Brush Hardness** (how soft the
  edge is), **Opacity**.
- **Start** and **End** (per cent of the path): key them to make the line draw itself on.
- **Spacing**: the gap between dabs, as a per cent of the brush. A small gap gives a smooth line,
  100 gives a string of beads, and 0 gives one continuous line.
- **Paint Style**: On Original Image (the line over the layer), On Transparent (the line alone)
  or Reveal Original Image (the layer shows only where the line is).

It runs on the graphics card. The check, `verification/D-356_stroke_table.md` (247 of 247),
holds every pixel to numbers worked out by a separate program before the code existed. It also
holds the card's picture to within 1 level of the processor's. The pictures are in
`verification/D-356 pictures/`.

## What to check

Use the reference shot's street (`town.png` is how it looks without the effect). Draw a circle
mask on the layer, then add **Path Stroke** from **Generate**. The mask's mode does not matter:
mode None leaves the picture as it is.

1. **As added.** A thin white line runs round the circle, and nothing else changes.
2. **Draw on.** Key **End** from 0 at the first frame to 100 a second later, then play. The
   line should draw itself round the circle, starting at the mask's first point and going in the
   direction the mask was drawn, until it closes (`write_on_strip.png`; one frame at a time in
   `write_on_f00.png` to `write_on_f24.png`). The pictures use an orange line, Brush Size 8, Hardness 60.
3. **A piece that travels.** Key **Start** from 0 to 75 and **End** from 25 to 100 over the same
   second. A quarter of the circle should chase round it at the same length the whole way
   (`travel_strip.png`, `travel_f00.png` to `travel_f24.png`). The dabs should stay put as it
   moves, with no crawling.
4. **Beads.** Set **Spacing** to 100, **Hardness** to 0 and **Brush Size** to 16. You should see
   soft round dabs, one brush apart (`beads.png`).
5. **Reveal.** Set **Paint Style** to Reveal Original Image and **Brush Size** to 30. The street
   should show only along the circle, with the rest see-through (`reveal.png`).
6. **More masks.** Add a second mask and turn on **All Masks**: both get a line. Turn on
   **Stroke Sequentially** and key End again: the first mask draws on, then the second.
7. **No mask.** Delete the masks, or set Path to a number the layer has no mask for. The layer
   should show as it is, with a warning that the effect has no path. It should not be an error.
8. **Moving mask.** Key the mask's shape. The line should follow it each frame.
9. **Save and open.** Save, close and reopen. The settings and keys should be as you left them.
10. **Same as an export.** Export a frame and put it next to the viewer at Full. They should look
    the same.

## Not done, on purpose or for later

- **Speed** (`verification/B-235_stroke_timing_table.md`): with one on each of three 1080p
  layers, it adds about 8 ms a frame as added and about 34 ms with a wide continuous brush on all
  masks. Without it a frame takes about 12 ms.
- **Masks are always closed.** After Effects can stroke an open mask; ours always joins the last
  point to the first, as our masks always do.
- **Text outlines** and **shape-layer paths** cannot be followed yet. Text waits on the text
  change that is waiting for you.
- **On an adjustment layer**, Path Stroke warns that it has no path and draws nothing.
- A mask of mode None whose outline crosses itself still gives the usual crossing-mask warning.
- Where dabs overlap they do not build up. The nearest one counts. Adobe does not publish how
  theirs combine.
- With Stroke Sequentially, each mask takes its share of the time by its length.

## If something is wrong

Say which step. The most likely faults would be in step 2 (the line starting at the wrong place
or going the wrong way) or step 3 (dabs crawling as it travels).
