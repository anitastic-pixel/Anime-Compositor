# D-266: Live text numbers, capped drags and the anchor point tool

You asked for three things:

- "try making it realtime to how I change the text values, because the change only happens after I let go of the value, but not during the drag/change itself."
- "have all values of certain cap, be capped when dragged, because I drag on say opacity, and when going beyond 100%, it doesn't apply to just 100%, just cancels it."
- "let's add a anchor point mover tool like AE does, place alongside the mouse point tool bar at the top. have anchor points be centered, have them behave like AE does."

## 1. Text numbers change the picture while you drag

Before, dragging a number in the Text section (Size, Tracking, Leading, Box width, Starts at, and the stroke, background and shadow settings) changed only the number. The picture changed when you let go. Now every move of the hand changes the picture, as the Transform numbers already did. The colour pickers work the same way: the words change colour while the picker is open.

One drag is still one step to undo, and Escape mid-drag puts the number back.

**In the window:** a Size drag of 120 px to the right was watched while the hand was still down. The program held these sizes one after another: 120, 123, 129, 135 … 234, 240. That is 40 live changes before letting go. One Ctrl+Z put it back to 120.

## 2. A drag past the end of a range stops at the end

Before, dragging opacity past 100% sent 103%, the program refused it, and the drag did nothing. Now every number with a range stops at its end while you drag, type or use the arrow keys. These are capped:

- **Layers:** opacity 0 to 100%, and shape trim start and end 0 to 100%.
- **Camera:** zoom stays above 0.
- **Solids:** size 1 to 8192.
- **Shapes:** mitre 1 to 100, gradient stops 0 to 100%.
- **Masks:** opacity 0 to 100%, feather 0 or more, expansion −8192 to 8192.
- **Effects:** every setting whose description gives a range, such as "pixels, 0 to 500", and Mix 0 to 100%.
- **Text:** size 1 to 2000, tracking −1000 to 1000, leading, box width, stroke width, background padding and roundness, and shadow angle, distance and softness, each within the range the program accepts.
- **Frame number:** never below 0.

**In the window:**
- Opacity at 100%, dragged 240 px to the right, stayed at 100%.
- Dragged 150 px to the left, it stopped at 0%.
- One Ctrl+Z put it back to 100%.

## 3. The anchor point tool

A new button sits beside the Selection tool at the top: a circle with a cross through it. Its key is **Y**, as in After Effects, where it is called the Pan Behind tool.

- **Drag a layer** to move its anchor. The picture stays where it is: the position moves with the anchor, so nothing jumps. A turned or scaled layer works too.
- **Hold Ctrl** while dragging to land the anchor on the nearest corner, side middle or middle of the layer's outline.
- **Double-click the button** to centre the selected layers' anchors in their outlines.
- Each drag is one step to undo, and Escape takes it back.

The anchor mark is now a light cross with a dark edge, so it can be seen on any picture.

**Centred anchors.**
- A new layer made from a drawing now has its anchor in the middle of the drawing, and it stands at the middle of the composition, as in After Effects. A 1920 × 1080 drawing gets anchor 960, 540 and position 960, 540.
- A new text layer gets its anchor moved to the middle of its words when you finish typing. This is the same step of history as the typing.
- A drawing whose files are not on this computer has no size to measure, so it keeps the top-left place, as before.

**In the window**, with a text layer "Anchor":

| Step | Anchor | The words |
|---|---|---|
| Typing finished | 676.4, 386.7, the middle of the words | |
| Dragged with Y | 850.6, 473.8 | did not move |
| Ctrl-dragged near the top-left corner | 480, 303, exactly the corner | did not move |
| Button double-clicked | 676.4, 386.7, the middle again | did not move |

Ctrl+Z then undid one step at a time: the double-click, the snap, the drag, then the typing with its centring, then the layer itself. No errors on the page.

## Pictures (`verification/D-266 pictures/`)

- `new_text_centred.png`: a new text layer after typing; its anchor cross is in the middle of the words.
- `anchor_moved.png`: after a drag with the anchor point tool. The cross is at the words' bottom right, and the words have not moved.
- `anchor_snapped.png`: after a Ctrl drag. The cross sits exactly on the words' top-left corner.
- `opacity_capped.png`: opacity after a drag far past the end, showing 100%.

## How the pictures were taken

Your own window was open, so it was left alone. A second copy of this build was started from a separate folder, with its own browser profile. It opened the reference shot, real mouse drags were sent to it, and it was stopped afterwards without saving.

## Checks

- The app's 88 checks pass. Two rows are new:
  - in `B-12d_new_composition_table.md`, a new layer from a 1920 × 1080 drawing gets anchor 960, 540 and position 960, 540;
  - in `B-12a_editing_table.md`, a drawing whose files are missing keeps 0, 0.
- The core program was not changed, so its suite was not run again.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Make a text layer, then drag its Size number in Effect controls | The words grow and shrink while you drag | |
| 2 | Press Ctrl+Z once | The size before the drag comes back | |
| 3 | Drag a layer's Opacity far to the right, then far to the left | It stops at 100%, then at 0%, and never jumps back | |
| 4 | Press Y and drag on a layer | The anchor cross moves with your hand, and the picture does not move | |
| 5 | Hold Ctrl while dragging with Y | The cross jumps to the nearest corner, side middle or middle | |
| 6 | Double-click the anchor point tool's button | The anchor goes back to the middle | |
| 7 | Import a drawing and make a layer from it | Its anchor cross is in the middle of the drawing, and the drawing is centred in the frame | |
