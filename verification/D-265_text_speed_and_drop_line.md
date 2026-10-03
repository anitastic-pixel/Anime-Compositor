# D-265: Text speed, the typing box and the panel drop line

You asked for three things:

- "long boundary when selected; improve realtime performance"
- "when dragging a panel to another, the placement highlight is too big and not contained in it's own section"

## 1. Text draws faster, and the pictures did not change

Styled text was slow to draw, and it was drawn again on every frame while playing. Three changes fixed this.

- **The stroke and faux bold** used to test every edge of every letter for every pixel near the words. They now sweep along each row and look only at the edges close by. The answer is the same; only the search is shorter.
- **The shadow** used to blur the whole 1920 × 1080 frame. It now blurs only the part the shadow reaches. Outside that part the shadow is empty, and blurring empty pixels gives empty pixels, so nothing changes.
- **A text layer is drawn once.** Text settings have no keys, so a text layer is the same picture on every frame. The last four text pictures drawn are kept, and playing or scrubbing reuses them. Changing anything about the words draws them again.

**Timings.** These are for drawing one 1920 × 1080 text picture.

- Machine: AMD Ryzen 9 9900X, 24 logical processors.
- Build: release.
- Each number is the middle one of 7 runs.
- "Before" is commit 5516fa7, built and run side by side on the same machine for this table.

| Text | Before | After | A repeat frame now |
|---|---|---|---|
| "Text", 120 px, plain | 2.87 ms | 2.71 ms | under 0.001 ms |
| 85-letter paragraph, plain | 4.40 ms | 4.50 ms | under 0.001 ms |
| The same paragraph with faux bold, a 4 px stroke, a background box and a soft shadow | 93.47 ms | 28.24 ms | under 0.001 ms |
| 360 letters, plain | 9.91 ms | 9.80 ms | under 0.001 ms |
| 360 letters, styled as above | 401.85 ms | 73.47 ms | under 0.001 ms |

Plain text was already quick, and it stays the same. The first time styled words are drawn it is now 3 to 5 times faster. After that it is free until something changes.

**Pictures checked bit for bit.** Eight text pictures were drawn on 5516fa7 and on this build, and every number of every pixel was compared. All eight are identical. They cover:

- plain text;
- a hard shadow;
- the full style;
- odd stroke and shadow sizes;
- a big shadow with italic;
- a shadow with no distance;
- a shadow thrown 300 px left;
- words hanging off the frame's left and bottom edges.

The D-263 and D-264 checks also pass. They include three D-263 pictures held byte for byte to 2edc62d.

## 2. The typing box fits the words

For one line of words (point text, no box width), the dashed typing box was twice the frame's width, 3840 pixels, so a long dashed line ran far past the words. It is now as wide as the longest line, plus a little room for the caret. For "Hello there" at 120 px, the box is 651 pixels wide.

- `point_typing.png`: "Hello there" being typed with the Text tool.
- `point_selected.png`: the same layer after Escape. Its selection outline is the box round the words, as it was after D-264.

## 3. The panel drop line stays in its place

While you drag a panel by its tab, the place it would land in gets a blue line. That line used the class name `drop`, which is also what the menus use. So the place under the hand was briefly given menu styling: it was lifted out of the window, and the rest of the layout jumped round it. In the photograph with the old name, the whole viewer section vanished. Its box measured 908 × 9 pixels at the window's top-left corner.

The line now has a name of its own. The place keeps its size, 2050 × 881 in the photograph, and the blue line sits just inside its edge.

- `drag_before.png`: the old class name put on the viewer's place, which is what the last build did mid-drag. The viewer is gone.
- `drag_after.png`: this build, mid-drag from the left column over the viewer. The viewer is outlined in blue and stays where it is.

## How the pictures were taken

Your own window was open, so it was left alone. A second copy of this build was started from a separate folder, with its own browser profile. It opened the reference shot, and it was stopped afterwards without saving. The drag was sent as pointer events to the page. The text layer was undone at the end, back to 4 layers. The page showed no errors.

## Checks

- Shape strokes: the check holding the fast edge search to the slow one passes.
- The whole core suite: 251 test groups ok, none of the committed checks failed. This includes `d263_text` and `d264_text_styles`.
- The app's 88 checks pass.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | With the T tool, click the picture and type a few words | The dashed box hugs the words and grows as you type | |
| 2 | Give a text layer Stroke, Background and Shadow, then press play | Playback runs as smoothly as without the text layer | |
| 3 | Drag the Effects panel's tab over the viewer, and over the timeline | A blue line just inside the section under your hand; nothing else moves | |
| 4 | Let go | The panel lands in that section | |
