# B-24g: the drag on the picture, by hand

Built on 2026-09-22, out of the B-24c playtest. The owner passed all fourteen of its steps and
reported two things none of them asks about: the picture lagged considerably behind every point
dragged, and a point pressed showed no sign of having been pressed.

Nothing was decided for this and no command changed, so there is no generated table of its own;
`verification/B-24c_panel_table.md` still passes at 17 of 17 and the window's tests are unaltered.
This sheet is the whole of the evidence, because what changed is how it feels under the hand,
which only a hand can judge.

## What changed

**The queue.** The page used to send one request for every movement of the pointer and wait for
each. A pointer reports about sixty times a second; a frame of the reference shot takes about
eighty milliseconds to come back (`verification/B-08_preview_latency.md`). So a two-second pull
queued something like a hundred and twenty requests behind a renderer that could answer about
twelve, and the picture went on catching up long after the hand had stopped. Every one of those
requests but the last described a shape nobody was going to look at, and each carries the whole
path rather than a step of it. The page now keeps one request in flight and the newest waiting
behind it, and drops the ones overtaken unsent. The same queue now serves the layer's own
handles - moving, scaling, turning a layer - which had the same problem.

**The light.** A point pressed, hovered over, or reached with Tab, is now filled rather than
hollow, with a white edge, and so is a handle. Nothing else about pressing a point changed.

## What to check

1. **A point dragged.** Draw a mask (**Q** and drag). Now drag one of its points around in
   circles for a few seconds and let go. The outline follows the hand as it always did. The
   picture behind it should now keep up within about a frame, and, the thing to watch for,
   **it should stop when your hand stops** rather than carrying on for seconds afterwards.
2. **A handle dragged.** The same on an ellipse's round handles.
3. **A layer dragged.** Drag a whole layer about by its middle, and scale and turn it by its
   corners and its knob. Same again: it should stop when you stop.
4. **The shape is the one you left.** After a fast drag, the mask must sit exactly where you let
   go - not at some shape from the middle of the pull. Scrub a frame away and back to be sure.
5. **Undo.** One **Ctrl+Z** after a drag takes back the whole drag, as before, and leaves the
   mask where it was before you touched it. This is the thing dropping requests could have
   broken, so it is worth doing twice.
6. **Escape.** Press Escape in the middle of a drag: the point goes back where it started.
7. **A point pressed.** Press a point once, without moving. It fills in yellow with a white edge
   and stays lit. Press another: the light moves to it.
8. **Hovering.** Move the pointer over a point without pressing: it fills while you are over it.
9. **Tab.** Press Tab until a point takes the focus ring: it is lit as a pressed one is.
10. **Shapes too.** Repeat 1, 4 and 7 on a shape layer's shape.

## Ceilings, stated

- **The picture still cannot answer faster than the renderer can draw it**, which is about twelve
  frames a second on the reference shot. The outline under your hand is drawn by the window and
  is immediate; the picture behind it trails by about a frame. Making the picture itself keep up
  is a different job - P-04 in document 15's performance section, which is proposed and has not
  been accepted.
- Nothing was measured for this sheet. The eighty milliseconds above is B-08's measurement of the
  preview path, not a measurement of this change. No timing claim is made about the fix itself.
- The drops are invisible by design. If a drag ever ends on the wrong shape, that is this change
  and it should be reported as step 4 failing.

## Result

**Passed on 2026-09-23.** The owner played all ten steps on the build of 2026-09-23 and answered
"1 thru 5 pass" for this sheet and the four walked with it (P-15, B-24h, B-24i, B-24j), with
nothing reported beside the steps.

