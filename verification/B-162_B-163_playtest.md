# B-162 and B-163: a viewer that keeps up with your hand, by hand

Built on 2026-09-29 under D-233. The checks are in `verification/B-162_latest_request_table.md` and `verification/B-163_refine_on_stop_table.md`. This sheet is for judging it in the app, by eye.

## What changed

- **B-163, Refine when idle.** There is a new tick in the viewer's bottom row, beside the Full/Draft button, and it is on to begin with. At Full:
  - while you drag the playhead or drag a value, the viewer shows the quick Draft picture;
  - a fifth of a second after your hand stops, it makes the Full picture and swaps it in.
  - While the Draft stand-in is showing, a small *sharpening…* appears beside the quality label.
  - The Full picture you end on is exactly the picture Full has always made.
- **B-162.** The viewer only ever makes the newest frame you asked for. A sharp picture of a place you have already dragged away from is thrown away, not drawn.

## Before you start

- Use the release build. It opens on the reference shot.
- Press the **Full resolution** button (or **D**) so the label says **Full**.
- Check that **Refine when idle** is ticked.
- For a clearer test, add a heavy effect to the background layer, such as **Radial Blur** with Amount 50, so that a Full frame takes a while to make.

## What to check

1. **Drag the playhead fast.** Drag along the timeline ruler, back and forth.
   - The picture should keep up with your hand. It will look softer while you drag.
   - The label should say **Draft**, and *sharpening…* should be showing.
2. **Let go.** Stop dragging and keep still.
   - Within a moment, the picture should sharpen. That is a fifth of a second, plus however long this shot takes to make one Full frame.
   - The label should go back to **Full … same as export**, and *sharpening…* should go away.
3. **Drag a value.** Select a layer and drag one of its numbers, for example Position or an effect's Amount.
   - The picture should follow your hand, soft, and sharpen a moment after you stop.
4. **Start again quickly.** Let go, and then, before the picture has sharpened, drag again.
   - You should never see a sharp picture of the old place flash up while you are dragging.
5. **Arrow keys.** Hold → for a second, then let go.
   - The picture should move along softly and sharpen where you stop.
6. **Untick Refine when idle.** Drag the playhead again.
   - The viewer should behave as it did before this change: Full pictures only, which keep up more slowly on a heavy shot.
   - Tick it again afterwards.
7. **At Draft.** Press **Draft resolution**.
   - The tick should turn grey, because it does nothing at Draft.
   - Dragging should look exactly as it did before this change.
8. **The button still means what it says.** While a soft stand-in is showing, the Full/Draft button should still read **Draft resolution**, because you chose Full.
9. **Play.** Press Play at Full. Playback should play Full pictures, as before. Refine when idle does not change playback.
10. **After a restart.** Untick Refine when idle and restart the app. It should still be unticked.

## What to report

- Any time the picture you end on looks different from Full as it was before, for example softer or shifted.
- Any flash of a sharp picture of a place you have already dragged away from.
- Whether the swap from soft to sharp feels too soon, too late, or distracting. A fifth of a second is a starting point and can be changed.
- Whether layer outlines (the boxes round a selected layer) ever sit in the wrong place while the picture is soft.
