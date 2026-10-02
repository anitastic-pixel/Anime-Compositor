# W-40a: the timeline's header (D-248)

The second screen of the redesign, first part. The timeline's top rows now look like the Sandbox's: a tab for each opened composition, Timeline / Both / Graph beside the frame number, Find a layer, the Note button, the composition's name, layer count and length at the right, and the three secondary switches as icons. Nothing about the picture, the project file or an export changed: this is where the controls are, not what they do.

## Before, after, and the Sandbox

| | Picture |
|---|---|
| Before W-40a | `W-39 pictures/after_compose.png` |
| After W-40a, the reference shot | `W-40 pictures/compose.png` |
| The Sandbox boards it copies | `W-38 pictures/sandbox_compose.png` and `sandbox_animate.png` |
| Both: the rows with the graph under them | `W-40 pictures/both.png` |
| Graph only | `W-40 pictures/graph.png` |
| Two composition tabs, a note, Find a layer | `W-40 pictures/tabs_note_find.png` |

The last three use a small made-up project, not the reference shot, so nothing of yours was touched. It is two solids and a null, with Solid 1 moving through four position keys (frames 0, 36, 72 and 120) and Solid 2 pre-composed into "Precomp 1".

What now matches the Sandbox, left to right:

- **the composition tabs**: one for each composition you have opened, with the current one lit orange along its top. Clicking a tab opens that composition. The × closes a tab, and only shows when there are two or more. Closing the current tab opens the one beside it. A tab is only a shortcut, so closing it never deletes anything.
- **"Main holds Precomp 1"** at the right of the tabs row. It lists the compositions this one has inside it, and clicking a name opens it. It shows only when there are any.
- **Timeline / Both / Graph**, beside the boxed frame number. **Both is new**: the layer rows stay on top and the graph sits under them, on the same frames, so a key in a row is straight above its point on the curve (`both.png`, frames 0, 36, 72 and 120).
- **the time** under the playhead, written as seconds and frames with the rate: "2s 2f · 24 fps".
- **Find a layer**: type some letters and only the layers whose names contain them stay in the list. Capital letters don't matter. Clearing the box brings them all back. In `tabs_note_find.png`, "so" leaves Solid 1 only. The project is not changed: it only hides rows from view.
- **Note**: puts a note on the ruler at the playhead, the same as the * key always has (W-24's markers). In `tabs_note_find.png` it is the orange flag at frame 36. Double-click it on the ruler to name it, as before.
- **+ New layer**, unchanged.
- **the three icons**: hide shy layers, motion blur, and frame blending. They do what the three worded buttons did, and hovering shows what each one is.
- the zoom, unchanged, then **"Main · 3 layers · 10s 0f"**: the composition's name, how many layers it has, and how long it is.

**Not built yet**, each waiting for the step named:

- **W-40b**: the Sheet button at the right of this row, which shows the Sheet as a tall column beside the timeline (the Sandbox's Animate board). Also notes showing grey in the Sheet's Action column, as D-248 says.
- **W-40c**: the green "ready frames" line under the work area and the "N of 240 ready" count. These need a new question the engine can answer, and a fixture for it first.
- **The Timeline | Sheet strip** above the composition tabs is the workspace's way of stacking two panels in one place (D-85). It stays until W-40b decides where the Sheet lives.
- **Both is not remembered per workspace.** Each time the app opens, the timeline starts on Timeline. Say so if you want it remembered.

## Click-through: what each part of the header does

| Clicked | What runs | What you see |
|---|---|---|
| A composition tab | the same "open this composition" the Project panel uses | that composition in the viewer and the timeline |
| The × on a tab | nothing in the project; the tab is dropped from the row | the tab goes; if it was the open one, its neighbour opens |
| A name after "holds" | the same "open this composition" | the inside composition opens, with its own tab |
| Timeline / Both / Graph | the old Timeline / Graph switch, plus the new Both | rows only / rows above the graph / graph only |
| The frame box | unchanged | type a frame, the playhead goes there |
| Find a layer | nothing in the project; the page hides rows | only the matching layers listed |
| Note | the same as the * key (`timeline.set_markers`) | an orange flag on the ruler at the playhead; Ctrl+Z takes it away |
| The shy / blur / frame-blend icons | the same switches as the old worded buttons | shy layers hidden; motion blur or frame blending on or off for the composition |

## Keyboard reach

- **Tab** reaches every new control: the composition tabs and their ×, the "holds" names, Timeline / Both / Graph, the frame box, Find a layer, Note, + New layer, the three icons and the zoom.
- **Both** has a palette line, "Timeline and graph together", so Ctrl+Shift+P → type "together" → Enter. It is also in the Animation menu under "Timeline or graph".
- **Shift+F3** still switches between Timeline and Graph, as before.
- **\*** still adds a note, as before.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show the same 1280 by 800 window at 100%, 150% and 200%.

- At 100% and 150%, the header is the tabs row plus one row of controls.
- At 200%, the controls wrap onto a second row rather than cutting anything off. The layer rows then need the timeline made taller (drag its top edge, or the ` key) to be seen at that size.

## Checks

- The page's wiring tests all pass, with their lists updated in the same change:
  - **Controls**: three new ones, Both, Find a layer and Note, so 86 now. All three are real controls the Tab key stops at.
  - **Keys**: no new key.
  - **Menu lines**: the new Animation menu line is a palette line, which the W-39 row checks. The table is `verification/B-12c_keyboard_table.md`.
- All 80 of the app's tests pass.
- The export tests pass: `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`.
- Checked in the running app: the motion blur icon lights when switched on, Note adds one note and Ctrl+Z takes it away, and Tab stops at Both, Find a layer, Note and the icons.
- The engine code was not touched. Only the page and the test list changed.

## Playtest

Open the app as you normally do, with a project that has at least one pre-composition.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look at the top of the timeline | a tab with the composition's name; under it Timeline / Both / Graph, the frame, Find a layer, Note, + New layer, three small icons, the zoom, and "name · N layers · length" | |
| 2 | Click Both | the layer rows on top, the graph under them, the playhead lined up through both | |
| 3 | Pick a layer with position keys, while in Both | its keys in the row sit straight above the points on the curve | |
| 4 | Click Graph, then Timeline | graph only, then rows only | |
| 5 | Press Shift+F3 twice | Graph, then back | |
| 6 | Type part of a layer's name into Find a layer | only matching layers listed | |
| 7 | Clear the box | every layer back | |
| 8 | Move the playhead, click Note | an orange flag on the ruler there | |
| 9 | Press Ctrl+Z | the flag goes | |
| 10 | Click the name after "holds" at the right of the tabs row | the inside composition opens, and a second tab appears for it | |
| 11 | Click the first tab | back to the outer composition | |
| 12 | Click × on the second tab | the tab goes; nothing is deleted (it's still in the Project panel) | |
| 13 | Hover each of the three icons | a tip naming shy layers, motion blur, frame blending | |
| 14 | Click the motion blur icon twice | it lights, then goes dark; the picture's blur follows | |

Anything marked ✗, tell me the row number.
