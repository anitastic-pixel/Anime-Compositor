# P-15: dragging a mask point, by hand

Built on 2026-09-22, out of the B-24g playtest. The owner ran step 1 of that sheet on the build
that contained it and reported: the point lights up now, but the drag still lags heavily.

That report is what found this. B-24g had assumed the renderer was the ceiling, at about twelve
frames a second, and said so in its own sheet. It was wrong, and the number that says so came
from the owner's window rather than from an argument.

## What the window measured

A stopwatch was added to the page for this, and it splits one drag update four ways: the command,
the window's own work on the frame, the trip into the page, and the paint. The owner dragged a
mask point at Draft resolution and the window answered:

```
1 updates — command 4.6 ms, window 17750.1 ms, trip 50.8 ms, paint 0.1 ms
```

**17.75 seconds inside the window for one frame**, against the 2.502 ms
`verification/P-01_frame_trace.md` measures for a warm Draft frame of the reference shot. One
update in the whole drag, because one is all there was time for. The trip and the paint, which is
where this was expected to be, are nothing.

## What it was

`src/mask.rs` worked a mask's coverage out by asking every pixel of the layer whether it was
inside the outline - sixteen samples a pixel, and every one of those samples walked every edge of
the flattened outline. A curved mask drawn round a character flattens into the high hundreds or
low thousands of edges, so a 1920x1080 layer cost something like forty-five thousand million
crossing tests, single-threaded, **for every update of the drag**.

Two things hid it:

- **No fixture P-01 traces carries a mask**, so its `layer mask` row reads 0.000 ms in all twelve
  of its tables. The stage was instrumented; there was simply never anything in it.
- **Draft resolution does not help.** A mask is rasterised in layer space, before the transform
  and the composite, so a draft preview of a 1080p cel still rasterises the mask at 1920x1080.
  That is where document 21 puts the mask and is not a fault, but it is why switching to Draft
  changed nothing for the owner.

## What changed

The outline's crossings with a sample row do not depend on where along that row the sample is, so
they are now worked out once for the row and the row is walked once. The sample grid, the
crossing arithmetic, the comparison and the order of the operations are the old ones, unchanged,
which is why this is the same picture and not a similar one.

Then the mask settings, on the owner's word that the mask itself was smooth but its settings
were not. Two of them were left behind by the first pass and both are done now:

- **Expansion.** D-77 grows or shrinks a mask by asking how far each sample is from the outline,
  and that had no shortcut along a row, so an expanded mask kept the old cost. It does now: for a
  growth a sample is in when it is inside *or* within the expansion of the outline, for a shrink
  when it is inside *and* far enough from it, so an edge further away than the expansion cannot
  change either answer and is dropped for that row before any distance is worked out.
- **Threads.** The rasterising and the feather's blur are both worked out a row at a time, and a
  row reads the outline and writes only its own pixels. They now run across the machine's cores
  instead of one. The rows are the same rows; only who works them out changed.

`verification/B-06_mask_table.md` is the proof, and it went from 41 to 45 checks. Two compare the
fast field against the sample-by-sample field pixel for pixel over twelve generated outlines. Two
more do the same for an expanded mask, grown by 2.3 pixels and shrunk by 1.7, against a field
worked out a second time in the fixture with its own insideness test and its own distance. All
four require **zero** differing pixels and a largest difference of **0.000000**. Not a tolerance -
the same numbers.

And one thing that is not speed but is what the speed was for: **the mask's three numbers now
follow the hand while they are dragged.** Opacity, Feather and Expansion used to wait for the hand
to let go before the picture changed, because an expanded mask cost seconds a frame and a picture
that arrives a second late is worse than one that waits. They are dragged like every other number
in the window now, through the same queue the points use, and the whole pull is still one thing
to undo.

`verification/P-15_mask_cost.md` is what it costs now, measured at three layer sizes, with a
second table separating what the new rule saved from what the threads saved. On a 1920x1080 layer
with a mask of eight corners, which flattens to 1,392 edges:

| Mask | Before, ms | Now, ms |
|---|---|---|
| plain | 30,567.8 | 6.33 |
| expanded 4 px | 269,706.1 | 32.68 |

Thirty seconds and four and a half minutes, against six and thirty-three **milliseconds**. The
threads are between five and ten times of that; the rest is the rule.

## What to check

1. **A mask point dragged.** Open a project with a masked layer, grab a point and drag it in
   circles for a few seconds, then let go. It should keep up with your hand now, and stop when
   your hand stops.
2. **Read the line.** The status strip prints the same four numbers when you let go. `window`
   is the one this changed. Report the line either way.
3. **The mask still looks right.** The edge of the mask must look exactly as it did before -
   same softness, same position, no jagged step and no shifted edge. This is the thing the
   fixture says cannot have changed, and it is worth one look by eye anyway.
4. **A feathered mask.** Put some feather on the mask and drag a point again. The blur is the
   same blur, spread across the cores, so this should keep up too. Say if it does not.
5. **An expanded mask.** Set the mask panel's expansion to something clearly visible, positive
   and then negative, and drag a point at each. This is the setting the whole second pass was
   for, so it is the one worth the most attention: it should keep up now, and the grown or shrunk
   edge must sit exactly where it did before, with the same rounded corners.
6. **Both at once.** Expansion and feather together, and drag again.
7. **The settings dragged.** Drag the blue Expansion number itself, slowly, back and forth past
   zero, and then the Feather number. The picture should change under the hand rather than when
   it lets go. Note anything that stutters, and at what value.
8. **Undo after dragging a setting.** One Ctrl+Z after letting go of Expansion must take the
   whole drag back to where it started, not one step of it.
9. **Escape while dragging a setting.** Hold Expansion, move, and press Escape without letting
   go: the number and the picture must both return to what they were.
10. **Undo.** One Ctrl+Z after a point drag still takes back the whole drag.

## Ceilings, stated

- **A very large expansion gives the saving back.** The band keeps the edges within the expansion
  of each row, so an expansion as wide as the mask keeps all of them and costs what it used to.
  Expansions of a few pixels, which is what the setting is for, keep almost none.
- **Two masks cost twice.** Nothing here shares work between masks on the same layer, and a
  layer with several of them pays for each.
- **The feather's blur is still a blur.** It was spread across the cores, not made cheaper, and a
  very large feather radius still costs in proportion to it.
- **Nothing was cached.** Each update rasterises the mask again from its points. If a drag that
  changes one point of one mask is still not fast enough, remembering the field between updates
  is the next thing and it is not done here.
- **No graphics card is involved.** ADR-006 and P-07 are untouched by this, and this work is
  evidence that the first thing to do about speed was not a graphics card.

## Result

**Passed on 2026-09-23.** The owner played all ten steps on the build of 2026-09-23 and answered
"1 thru 5 pass" for this sheet and the four walked with it (B-24g, B-24h, B-24i, B-24j), with
nothing reported beside the steps. The four numbers step 2 asks for were not reported.

