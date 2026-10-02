# W-44: the Animate workspace (D-248)

The sixth screen of the redesign. Animate now opens as the Sandbox's Animate board: rows, graph and sheet together.

- **The layout.**
  - Effect controls sits at the top left, 340 wide, with Project and Effects as tabs behind it.
  - The viewer is smaller, beside it.
  - The timeline takes the lower half.
  - The Sheet stands as a full-height column on the right.
  - Compose is unchanged.
- **Timeline, Both, Graph are remembered per workspace.** Animate starts on Both, and the other workspaces start on Timeline. Each one then keeps whichever you last pressed in it.
- **The graph has a sidebar on its left**, in the gutter under the layer names. It has two parts.
  - **In the graph · tick to add.** Each keyed setting of the chosen layer is listed with:
    - a tick box;
    - its colour;
    - its name;
    - its value at the playhead.

    The one in front is always ticked. Tick another to draw it behind, in its own colour, and untick it to take it away. Click a name to put that setting in front. The **All** button in the timeline's header ticks every one, or unticks them all. This replaces the old drop-down list of settings.
  - **Shape of the curve.** Three buttons, **Easy ease · F9**, **Linear** and **Hold**. They act on the chosen keys of the setting in front, or on all of its keys when none is chosen. The button for the shape those keys share is lit.
- **The graph fills its half.** It grows and shrinks with the panel instead of keeping one height.
- **Curves behind the front one are solid lines in their own colours.** The second line of the front setting (y, for a position) is dashed.
- **The Sheet:**
  - its tab reads "Sheet · *the composition's name*" with the hint "lit row = the playhead · click a row to go there";
  - a **×** at the right of its tab puts it away, and the Sheet button on the timeline brings it back;
  - while the Sheet column is out, the small frame strip under the viewer is hidden, since the Sheet shows the same frames;
  - **a cell whose drawing is missing is struck out in red**, with the cells that hold it. Pointing at one reads "Drawing N is missing";
  - the first column is headed "Frame".
- **Tab names are no longer in capitals.**

It sends the same commands as before. No new command, key or fixture was added.

**One change in how it behaves:** **Easy ease in the sidebar works on held keys.** It lets go of the hold first, then eases. F9 and the right-click menu still leave held keys alone, as they always have.

## Pictures

| | Picture |
|---|---|
| The Sandbox's Animate board above, the app below | `W-44 pictures/sandbox_vs_app.png` |
| The whole window: layer2 with keys on Position and Rotation, playhead at 50, Rotation ticked | `W-44 pictures/whole_window.png` |
| The graph and its sidebar, close up | `W-44 pictures/graph.png` |
| The real window at 100% / 150% / 200% | `W-44 pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |

The keys in the pictures were added for the picture only and never saved. In `whole_window.png` the viewer is black because the shot's drawings were not loaded in that run. The 100% picture shows the shot.

## What it does, checked in the running app (`w44_check.js`)

The check used the reference shot. It added four Position keys (frames 0, 36, 72, 120) and three Rotation keys (10, 60, 100) to layer2, and put the playhead at 50. Nothing was saved, and the window's own layout and settings were put back afterwards.

| Step | What the page said |
|---|---|
| Opened Animate | Both is pressed; Effect controls 340 wide with Project and Effects tabs; the Sheet column 380 wide and full height; the rows and the graph 319 high each; the frame strip hidden; the Sheet tab "Sheet · reference shot", with × shown |
| The sidebar | "In the graph · tick to add": Position 1170.0, 436.1 (in front), Rotation 24.0; "Shape of the curve · Position" with Linear lit; nothing drawn behind |
| Ticked Rotation | Rotation drawn behind; All lit |
| Unticked it | nothing behind; All not lit |
| Pressed All | Rotation drawn behind |
| Clicked the name Rotation | Rotation in front, the graph reads "rotation value", Position drawn behind |
| Clicked the name Position | Position in front again |
| No key chosen, Hold | all four Position keys hold; Hold lit |
| Easy ease | 0, 36 and 72 eased (120 is the last key and starts no curve); Easy ease lit |
| Linear | all four linear; Linear lit |
| Only the key at 36 chosen, Hold | only 36 holds; 0, 72 and 120 stay linear |
| The Sheet's head | Frame, Action, layer1, layer2, layer3, layer4 |
| Struck-out cells | layer3's drawing 7 and the cell holding it, at frames 14–15, 38–39, 62–63 … 230–231 |
| Clicked the row for frame 20 | the playhead goes to 20; that row is lit |
| Pressed × | the Sheet column goes away, the frame strip comes back, the Sheet button is no longer lit |
| Pressed the Sheet button | the column is back |
| Compose | starts on Timeline |
| Set Compose to Graph, went to Animate | Both |
| Set Animate to Timeline, went to Compose | Graph |
| Back to Animate | Timeline |
| Errors on the page | none |

## Limits, stated

- **The Sheet's columns stay in paper order:** Frame, Action, then the layers. The board puts the layers first, then Action and Dialogue. D-84c and D-84g set the paper order, so it is kept. If you want the board's order, say so, and it becomes a decision.
- **Curves behind share the front curve's scale (B-19d).** So Rotation, in degrees, draws nearly flat under Position, in pixels. The board draws each curve at its own height. Changing this would change B-19d, so it is not done here.
- **The rows don't show twirled-open settings with ◀ ◆ ▶ as the board does.** Those are in Effect controls since W-42.
- **No label colour on the rows.** Label colours are an engine proposal.
- **At 200% on a 1280 by 800 window**, the page is 640 by 400. The timeline's header then wraps to four lines and Both leaves the rows almost no room (`scale_2.0.png`). Timeline gives the rows the whole panel. At 100% and 150% everything fits.
- **Workspaces keep their own Timeline / Both / Graph choice**, as the board says. Whether workspaces keep their own panel changes is still your open question. Until you answer, D-85 stands.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| Animate (top right, or Alt+2) | nothing is sent | the board's layout; Both, or what you last chose in Animate |
| Timeline / Both / Graph | nothing is sent | that view; remembered for this workspace |
| a tick box in the sidebar | nothing is sent | that setting drawn behind, or taken away |
| a setting's name in the sidebar, or its curve | nothing is sent | that setting in front |
| All | nothing is sent | every keyed setting drawn, or only the front one |
| Easy ease · F9 | the same ease as F9 (for held keys, first the same "linear" as Ctrl+Alt+G) | the curve eases; the button is lit |
| Linear / Hold | the same change as the right-click menu's Linear and Hold | the curve straightens or steps |
| a row of the Sheet | nothing is sent | the playhead goes there |
| × on the Sheet's tab | nothing is sent | the Sheet column goes away |
| Sheet (on the timeline) | nothing is sent | the column comes back |

## Keyboard reach

- **The sidebar's tick boxes, names and shape buttons are ordinary tick boxes and buttons.** Tab reaches them, and Space or Enter presses them. The × on the Sheet's tab is a button too.
- **F9 still eases**, Shift+F3 still switches to the graph, and Alt+2 still opens Animate.
- These were not checked with a real keyboard.

## Checks

- **All 81 of the app's tests pass.** The wiring table lost the old setting list (`graphprop`) and gained the Sheet's × (`sheetclose`), so it still has 99 entries. The key table is unchanged.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the reference shot and click layer2. In Effect controls, click the stopwatch beside Position, then go to frame 48 and drag Position's blue number, and again at frame 96.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click Animate at the top right | Effect controls at the top left, a smaller viewer, the timeline in the lower half, the Sheet as a column on the right; Both is pressed | |
| 2 | Look under the layer names in the lower part of the timeline | "In the graph · tick to add" with Position and its value | |
| 3 | Give Rotation two keys the same way, with its stopwatch, then tick Rotation in that list | a second curve in another colour | |
| 4 | Click the word Rotation there | Rotation's curve comes to the front | |
| 5 | Click Hold, then Easy ease · F9, then Linear | the front curve steps, then eases, then goes straight; the button you pressed is lit | |
| 6 | Make the timeline taller by dragging the line above it | the graph grows with it | |
| 7 | Click Timeline, click Compose, then Animate again | Animate is still on Timeline; Compose kept its own | |
| 8 | Click a row in the Sheet | the playhead goes to that frame | |
| 9 | Look at layer3's column in the Sheet | drawing 7 struck out in red | |
| 10 | Click the × at the top right of the Sheet, then the Sheet button on the timeline | it goes away and comes back | |

Anything marked ✗, tell me the row number.
