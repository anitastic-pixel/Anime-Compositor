# W-27: the effect cards' pictures

Asked for on 2026-09-26: "make the UI of each effect ... more alike AE, that have more
interactable visual controls/representation. like curves, levels, hue and saturation, gradient,
improve direction-based values to be a spinning wheel to determine direction with the value in
degrees next to it".

Nothing about the picture the effects make has changed, only how their settings are shown and
changed, so there is no fixture table. `verification/W-27 pictures/effect_cards.png` is a
photograph of the new cards, taken in a browser with the real page and made-up settings.

## Before you start

Open any project with a drawn layer, for example `C:\Users\Andrew\Downloads\project.json`, select
the layer, and add the effects named below from **Add effect…**.

## What to check

1. **Dials.** Add **Drop Shadow**. Beside **Direction** is a small round dial with a blue line,
   and the number says `135°`. Drag round the dial: the line follows the pointer, the number
   changes, and the shadow swings round the drawing as you go. Hold Shift: it jumps in 15 degree
   steps. Keep going round past the top: the number carries on past 360 rather than jumping back.
   Let go, then Ctrl+Z once: the whole drag is undone.
2. **The other dials.** Directional Blur's **Direction**, Bloom's **Streak Angle**, Rim Light's
   **Light Direction** and Hue/Saturation's **Hue** each have the same dial. Hue stays between
   -180 and 180 however far you turn it.
3. **Escape.** Start dragging a dial, press Escape before letting go: it goes back to where it
   was.
4. **Curves.** Add **Curves**. A square graph shows a straight diagonal line. Press on the
   middle of the line and drag up: a point appears, the line bends through it, and the drawing
   lightens as you drag. The points box under the graph follows. Press on the graph away from
   the line: a point appears on the line there. Drag a point off the graph and let go: it is
   gone. Double-click a point: it is gone. The two end points never go.
5. **Channels.** Pick **Red** in the list above the graph: the graph shows the red curve in red,
   and the master curve you bent stays faintly behind. Bend the red one up: the drawing warms.
   **Reset** puts the curve shown back to straight.
6. **Levels.** Add **Levels**. Under a small graph are two grey ramps with triangles. Drag the
   top left (black) triangle right: the darks go black and the graph's line moves with it. Drag
   the grey middle triangle left: the middle lightens and **Gamma** goes above 1. Drag the lower
   white triangle left: the whites dim. The numbers under it follow each drag.
7. **Hue/Saturation.** Add **Hue/Saturation**. Two rainbow bars: the top one is the colours
   going in, the bottom one what each becomes. Turn the Hue dial: the bottom bar slides. Lower
   Saturation: the bottom bar greys out. Raise Lightness: it goes pale.
8. **Gradient.** Add **Gradient**. A box shows the ramp with **Start Point** and **End Point**
   marks. Drag a mark: the ramp in the box and on the drawing both follow, and the numbers
   update. Change a colour, an opacity or the Shape: the box repaints.
9. **Centres.** Radial Blur and Chromatic Aberration show the same box with one mark for
   **Centre**. Press anywhere in the box: the centre jumps there, and drags from there.
10. **Keyed.** Key Drop Shadow's Direction at two frames, go to a frame between, and turn the
    dial: it changes the key at that frame, as typing the number does.

## Known limits, on purpose

- Levels shows the line the settings make of a grey ramp, not a histogram of the layer's own
  colours, since the card does not have the layer's pixels.
- The pictures are about the size of the card, so they place a point to about a whole step
  (one level, a tenth of a per cent); type the number for anything finer.
- The boxes for points use the composition's shape, which is the drawing's shape only when the
  layer fills the composition.
- A curve point pulled hard makes the smooth line overshoot and flatten at the top or bottom.
  That is what the effect really does (B-54's known limits), and the graph shows it rather than
  hiding it.

## What checks it by machine

- `verification/B-12b_state_fields_table.md`: every card still sends every setting the command
  reads.
- `verification/B-12c_keyboard_table.md`: every command a picture sends can still be sent
  without a mouse, from the numbers beside it.

## What to answer

"works", or which step number did something else and what it did.
