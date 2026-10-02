# D-261: Sketch

A fourth workspace, **Sketch**. Open it with the Sketch tab at the top right, Alt+4, Window › Sketch workspace, or the command search. It is a drawing board for notes and ideas over the cut. Accepted as choice (a): sketches are saved with the project, and they never reach a frame or an export.

What it has, laid out as in the Sandbox:

- **Tools down the left.**
  - Brush (B), Pencil (P), Eraser (E).
  - Three sizes; [ and ] step through them.
  - Five colours: red, blue, green, black, white.
  - Undo the last stroke, which is the same as Ctrl+Z.
- **The picture on a desk in the middle.** The cut lies under the paper at Off, 30%, 60% or 100%.
  - The paper turns 15° left or right and flips left-right. Reset puts it back.
  - Turning changes only how you hold the paper. A line drawn on turned paper lands where you drew it on the picture.
- **The frames under it**, sixteen at a time, with ◀ ▶ to step.
  - A ● marks a frame that has its own sketch.
  - **Onion skin** shows the nearest sketched frame before in red and the nearest after in green.
- **The layers on the right.**
  - **+ New layer**, and an eye on each layer to show or hide it.
  - The swatch beside each layer is the colour last drawn on it.
  - Click a layer's name to choose it.
  - The chosen layer has:
    - a **Name**;
    - **Draws on**: *This frame*, where each frame keeps its own drawing like paper on a peg bar, or *Whole cut*, one drawing shown on every frame;
    - **Clear**, which clears this frame, or the whole layer when it draws on the whole cut;
    - **Delete layer**.
- **Also over the Compose viewer**: a tick that draws the sketches over the finished picture in Compose, to check an idea against it. It is remembered on this computer, not in the project.

Your first stroke makes "Layer 1" for you. Each stroke, layer change and Clear is one step for Ctrl+Z and Ctrl+Shift+Z.

## Pictures (`D-261 pictures/`)

- `d261_board.png`: frame 4 of the reference shot with the cut at 60%.
  - Layer 1 holds a red brush wave, a thick blue ring and a thin blue pencil line.
  - The eraser (thick) has cut a gap in the red wave.
  - The left column shows the eraser and the thick size chosen. The strip has a ● on frame 4.
- `d261_onion.png`: frame 5, which has no sketch of its own, with Onion skin on.
  - Frame 4's drawing is faint red; frame 6's wave is faint green.
  - The **Notes** layer, set to Whole cut, shows its green outline at full strength, as on every frame.
  - The strip has ● on 4 and 6.
- `d261_turned.png`: frame 4 with the paper turned 30° and the cut at 100%.
  - A new red wave was drawn along the bottom of the turned paper.
- `d261_compose.png`: back in Compose with "Also over the Compose viewer" ticked. This one is a photograph of the whole window.
  - The sketches lie over the finished picture.
  - The red wave drawn on the turned paper sits flat along the bottom of the picture, where it was drawn.

## The check, written first

The check is `tests/d261_sketch.rs`, committed in 503440f. **On the build before the change**, it did not build, because there were no sketches.

All 16 checks now pass, in `D-261_sketch_table.md`. In short:

| Check | Result |
|---|---|
| Two sketch layers and four strokes, sent as the page sends them, are taken; a bad point, an unknown tool, or a missing layer is refused | pass |
| Clear on one frame takes only that frame's stroke; undo puts it back; undo again takes back the last stroke | pass |
| Saved and opened again, both layers are the same to every point; saved again, the file text is byte for byte the same | pass |
| Frames 8 to 12 exported with the sketches (including a hidden layer and a whole-cut layer) are byte for byte the frames exported without them | pass |
| A file holding sketch lines no build writes yet keeps them through open and save | pass |
| A project with no sketches saves no sketches line, and all 2297 fixture projects save byte for byte as before (one SHA-256 over all of them) | pass |

## In the running app

| Step | What happened |
|---|---|
| Alt+4 | The Sketch board, frame 4, "drawing on Layer 1 with the brush" |
| A drag on the picture | Layer 1 was made, with 1 stroke at frame 4 |
| Blue, thick, a ring; then P and [ | A second stroke; the tool became pencil and the size went from thick to medium |
| A pencil line, then Ctrl+Z, then Redo | 3 strokes, then 2, then 3 |
| E and ], a drag across the red wave | An eraser stroke on Layer 1; the wave has a gap |
| + New layer, Whole cut, renamed "Notes", a green outline | The Notes layer, "whole cut · 1 marks", shown on every frame |
| Frame 6, Layer 1 chosen, a red wave; back to 5, Onion skin | Frame 5 shows frame 4 in red and frame 6 in green; the strip has ● on 4 and 6 |
| Frame 6, Clear | The button read "Clear Layer 1 on frame 6"; frame 6's stroke went and frame 4's stayed |
| Paper turned twice (30°), a drag near the bottom | The stroke's ends were at 20% and 50% across, 88% down: exactly where the drag was aimed on the paper |
| Overlay ticked, Alt+1 | Compose shows the sketches over the picture |
| Errors on the page | none |

The strokes were drawn with real mouse input sent to the window. The window's own settings were put back afterwards. No project file was saved, and nothing new appeared in the app's own folder.

## Faults found and fixed on the way

- **The window refuses a style written into the page's markup.** The first run showed grey colour buttons, no size dots, and grey paper. The page-wide grey behind every canvas was also covering the picture under the overlay in Compose. The colours and sizes now live in the stylesheet, and both sketch canvases are see-through. A new app check, `the_page_writes_no_style_into_its_markup`, fails if anyone writes such a style into the page again.
- **The offline check (B-11) was failing** before this item. My viewer screen work (W-41c, 441acf2) wrote the web address that names SVG drawing into the page. The page now borrows that name from its own SVG, so the page names no web address again.
- **The check's own spelling of whole numbers** was changed to match how the project file writes them (`4`, not `4.0`) before the build. Only how the check writes its expected text changed. What it compares did not.
- **Fifteen GPU checks** stopped building after D-259 added Half quality, because they listed the qualities one by one. Each now treats Half as Full. Their expected values are unchanged.

## Checks

- All 88 of the app's tests pass (87 before, plus the no-style check).
- The whole core suite passes. The only exception is one untracked scratch file from earlier work (`tests/zz_scratch_g2.rs`), which is not part of the project.
- The page, route, keyboard and command-map lists (B-12b, B-12c) gain the Sketch controls, Alt+4, the four commands, and `comp.sketches` in the window's answer.
- No fixture, existing saved project byte, or exported picture changes.

## Limits, stated

- **Lines only.** There is no fill, no pressure from a pen, and no blend modes. The eraser rubs out only on the chosen layer.
- **The cut under the paper is the viewer's own picture.** At Draft quality it is a quarter-size picture blown up, so it looks soft. Choose Full in the viewer for a sharp one.
- **Sketches are never exported** and do not show in Render. That is the decision, not a gap.
- **The overlay in Compose follows the viewer's turn, mirror and zoom.** It shows every visible layer on the frame on screen, without onion skin.
- **Not included:** text notes, shapes, a lasso, moving strokes after drawing, and more colours than the five.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Open a project, press **Alt+4** | The Sketch board: tools left, the picture in the middle, frames under it, Layers on the right | |
| 2 | Drag on the picture | A red line follows your hand; "Layer 1" appears on the right; the frame gets a ● in the strip | |
| 3 | Press **P**, **E**, **B**, and **[** / **]** | The highlighted tool and size change on the left | |
| 4 | Pick blue, draw; then press **Ctrl+Z**, then **Ctrl+Shift+Z** | The blue line goes, then comes back | |
| 5 | With the eraser (E), drag across a line | That line gets a gap; lines on other layers are untouched | |
| 6 | Step to another frame and draw; turn on **Onion skin** on a frame between | The frame before shows faint red, the frame after faint green | |
| 7 | **+ New layer**, choose **Whole cut**, draw, then step frames | That drawing stays on every frame | |
| 8 | Rename the layer in **Name**; click the eye | The name changes; the eye hides and shows it | |
| 9 | Turn the paper with the arrows, draw, press **Reset** | Your line sits where you drew it on the picture | |
| 10 | Press **Clear …** on a frame | Only that frame's drawing on that layer goes; Ctrl+Z brings it back | |
| 11 | Tick **Also over the Compose viewer**, press Alt+1 | The sketches show over the picture in Compose; untick, and they go | |
| 12 | Save, close, open the project again, Alt+4 | Every sketch is back | |
| 13 | Export a few frames | No sketch in any exported frame | |

Anything marked ✗, tell me the row number.
