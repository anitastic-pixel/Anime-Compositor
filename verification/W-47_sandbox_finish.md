# W-47: what the Sandbox had that the app still missed

You asked me to go through every Sandbox board again, find what was left out, and give the app the Sandbox's colours. This is the page-only part of that request. The Text tool changes the project file, so it is a separate item, D-263, built check first.

## What was missing, and what changed

| Where | The Sandbox | The app before | The app now |
|---|---|---|---|
| App icon (taskbar, title bar, the exe) | the orange clapperboard | a placeholder | the orange clapperboard on a dark rounded square, at 16 to 256 pixels |
| Logo, top left | the orange clapperboard | a grey film icon | the orange clapperboard |
| Workspace switcher | an icon for each workspace, with the name on the one in use | four words | four icons, and the name only on the one in use. Pointing at an icon names it. Alt+1 to 4 are unchanged |
| Top bar order | menus, cut, tools, search, Effects, then the workspaces at the right | the workspaces after a gap in the middle | as the Sandbox |
| The cut in the top bar | ‹ Ep 03 › **Cut 012** › | the project name only | ‹ *project* › **cut on screen** ›. The arrows open the composition before or after it in the Project panel, and are greyed when there is none |
| The cut's status | a coloured dot | not in the top bar | a dot in the status colour. Clicking it opens the same status menu as the Project panel |
| Layer rows | a coloured strip down the left of every row | a small square, and only when a label was chosen | a strip on every row. A layer with no label of its own takes its drawings' or composition's label, or else a colour for its kind. **Only the screen does this: the file still has no label until you choose one** |
| Project rows | a coloured square on every row | only labelled rows | every row, by the same rule |
| Mode on a row | "Normal", no arrow | "normal" with an arrow | "Normal", no arrow; it still opens the same list |
| Find a layer | a dark box with a magnifier | a plain light field | a dark box with a magnifier |
| Panel tabs | the panel's name in a tab with an orange line on top | a grey strip | the name in its own tab with the orange line, on a darker strip. Panels stacked together show the line on the one in front |
| Viewer frame number | blue, in a dark box | blue, bare | blue, in a dark box |
| Playing speed | a green chip | green text with a dot | a green chip. It turns grey when stopped and orange when slower than the cut's speed, as before |
| Timeline frame box | blue on the darkest grey | blue on grey | as the Sandbox |
| Composition bars | teal | purple | teal |
| Adjustment layer bars | purple | blue stripes | purple |
| Buttons and fields | cool grey buttons, fields darker than the panel | warm grey for both | as the Sandbox |
| Every grey on the page | a slight blue cast (#1a1a1c, #2a2a2e) | neutral greys | the same slight blue cast, in all 66 greys of the stylesheet. **The picture itself is untouched: only the page around it changed** |
| The strip along the bottom | none | up to four lines of sentences | one quiet line. A sentence too long to fit is cut with "…", and pointing at it shows the whole sentence. Error details opens the strip to its full height again |
| Render progress | a thin blue bar under "Rendering 1 of 3" | a plain grey bar | a thin blue bar |
| Sketch paper | on a dotted desk, with room round it | filling the desk | on the dotted desk, with room round it |
| Sketch turn | "0°" next to the flip button | shown only when turned | always shown: 0° in grey, a turn in orange |

No command, key, saved setting or fixture changed. The 88 app checks pass, as before.

## Seen in the Sandbox, but not built

- **The Text tool (T in the toolbar).** It comes with D-263, next.
- **"Keep animating while it renders" and the choice of a popup or its own window** for the render watcher. Both were deliberately left out of D-252.
- **Taller timeline rows** (30 instead of 26). The Sheet, the graph and the rows all line up on the same row height, so this would be its own change, not a colour change.

## Pictures

| | Picture |
|---|---|
| The Sandbox's board above, the app below, for each workspace | `W-47 pictures/sandbox_vs_app_compose.png`, `_animate.png`, `_render.png`, `_sketch.png` |
| Each workspace on its own | `W-47 pictures/app_compose.png`, `app_animate.png`, `app_render.png`, `app_sketch.png` |
| The top bar, close up | `W-47 pictures/top_bar.png` |
| The timeline rows with their colour strips | `W-47 pictures/timeline_rows.png` |
| The one-line bottom strip | `W-47 pictures/bottom_strip.png` |
| The new icon at each size Windows uses | `W-47 pictures/app_icon_sizes.png` |

All four workspaces were photographed from the real window. The page reported no errors, and the window's own layout and settings were put back afterwards.

## Checked in the running app

The check used the reference shot.

| What | What the page said |
|---|---|
| The cut in the top bar | "reference shot". Both arrows greyed, since the project has one composition |
| The status dot | "Not started", and pointing at it says "Where reference shot has got to. Click to change it" |
| The layer strips | all four layers aqua, the colour for drawings, since none has a label of its own |
| Panel tabs with their own name box | Project, Effects, Viewer, Effect controls, Sheet, Render queue |
| The workspace buttons | named Compose, Animate, Render and Sketch; only Compose shows its name |

## Playtest

| # | Try this | You should see | OK? |
|---|---|---|---|
| 1 | Look at the taskbar and the window's title bar | the orange clapperboard | |
| 2 | Press Alt+1 to Alt+4 in turn | the name appears only beside the workspace you are in | |
| 3 | Open a project with two or more compositions, then press › beside the cut's name | the next composition opens. ‹ goes back, and is greyed on the first | |
| 4 | Click the coloured dot after the cut's name | the same status menu as in the Project panel | |
| 5 | Give a layer a label (right-click › Label), then take it off | the strip takes the label's colour, then goes back to the colour for its kind | |
| 6 | Type in Find a layer | the rows filter as before, in the dark box | |
| 7 | Point at a sentence cut short in the bottom strip | the whole sentence as a tooltip | |
| 8 | Click Error details | the strip opens to full height, with every line whole | |
| 9 | Render 30 frames from the Render workspace | the thin blue bar fills under "Rendering" | |
| 10 | Open Sketch and turn the paper | the readout goes from 0° in grey to the angle in orange, and the paper stays clear of the edges | |
| 11 | Look over each workspace | the colours sit with the Sandbox's: nothing looks out of place or unreadable | |
