# W-41c: the Map tab, and the Hand and Zoom tools (D-248)

The third screen of the redesign, last part. Two things from the Sandbox:

- **The Map.** A second tab in the viewer that shows which composition holds which. It can show:
  - a tree;
  - boxes joined by arrows;
  - or both.
- **The Hand and Zoom tools** in the top bar, beside the selection tool, with After Effects' keys **H** and **Z**.

Neither changes the project. The map only reads it. A click on a name or a box opens that composition, as its tab above the timeline does.

## Pictures

| | Picture |
|---|---|
| The Sandbox board it copies | `W-38 pictures/sandbox_map.png` |
| The map in the real window, Both | `W-41c pictures/map_both.png` |
| The real window at 100% / 150% / 200% | `W-41c pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |

In `map_both.png` the reference shot has had a layer pre-composed and a copy of that precomp made, so there is something to map. These edits were in the check only and were never saved.

**The tab row** reads "VIEWER reference shot", then "MAP" (underlined in amber while it is in front) and its ×.

**On the left, the composition tree:**

- reference shot (top level);
- Precomp 1 under it ("layer 1");
- Precomp 1 copy, marked "not used" in amber.

**On the right, the boxes:**

- **reference shot**, outlined in green because it is the one on screen. It reads "4 layers · 10s 0f, 1920×1080".
- An arrow from it to **Precomp 1**, which reads "1 layer · 10s 0f, inside reference shot".
- Under them, a dashed, dimmed **Precomp 1 copy**, which reads "not used anywhere yet".

**At the top right**, the Hand (an open hand) and Zoom (a magnifier) icons sit between the selection arrow and the pen.

## What it does, checked in the running app (`w41c_check.js`)

| Step | What the page said |
|---|---|
| Opened the map (Composition ▸ Map of compositions) | the Map tab shown and in front; the card stops painting the picture it covers |
| Pre-composed a layer, copied the precomp | three boxes, one arrow, three tree lines, as described above |
| Clicked the Precomp 1 box | Precomp 1 opened; its box and its tree line lit together |
| Flowchart / Tree / Both | Tree: tree only, with the line "Flowchart hidden…". Flowchart: boxes only. Both: both. The choice is kept with your preferences |
| Clicked MAP again | the picture is back, the Map tab stays, the card paints again; clicked again, the map is back |
| The zoom tool, click on the picture | 64% → 127% (twice the size, about the spot clicked) |
| …then Alt+click | 127% → 64% |
| The hand tool | the hand is lit in the top bar; the pointer is a hand; the status line says "The hand: drag to move about the picture. The middle button does this with any tool." |
| Errors on the page | none |

## Limits, stated

- **The map only shows compositions.** The Sandbox's extra box listing a composition's other layers is not built. Each box gives the layer count instead.
- **A composition held by two others** sits in the deeper column, beside the first one that holds it. On a large project an arrow can cross another.
- **The map does not move or zoom.** It scrolls when it is bigger than the panel.
- **Double-clicking a composition layer does not open the map.** The Sandbox's note "opened by double-clicking Precomp 1" is not built; the map opens from the Composition menu and the command finder.
- **H is a new key.** The Sandbox (D-248) gives the Hand tool H and the Zoom tool Z, as After Effects does. Z alone was free; Ctrl+Z is still Undo.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| Composition ▸ Map of compositions | nothing is sent to the project | the viewer comes forward with the Map tab in front |
| MAP (in the viewer's tab row) | nothing is sent | switches between the map and the picture |
| × beside MAP | nothing is sent | the map closes; the picture is back |
| a name in the tree, or a box | opens that composition, as its tab does | the timeline shows it; the map lights it |
| Flowchart / Tree / Both | nothing is sent | the map's layout |
| the hand icon, or H | nothing is sent | drag the picture to move about it |
| the zoom icon, or Z | nothing is sent | click to zoom in; Alt+click to zoom out |
| Esc | nothing is sent | back to the selection tool |

## Keyboard reach

- **Every tree line and every box is a button Tab stops at.** Enter opens that composition, and the focus stays on it after the map redraws.
- **The new commands are in Find any command (Ctrl+Shift+P):** "Map of compositions" (also in the Composition menu), "Hand tool" (H) and "Zoom tool" (Z).
- **The zoom's click has keys already:** the picture's zoom is on `.` and `,`, as before.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window at 100%, 150% and 200%. The two new icons fit in the top bar at all three.

## Checks

- **All 81 of the app's tests pass.** In `verification/B-12c_keyboard_table.md`, the wiring table lists the two new buttons, `maptab` and `closemap`, and the key table adds H.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the app on a project that has a pre-composed layer, or pre-compose one (Ctrl+Shift+C) and undo it afterwards.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Composition ▸ Map of compositions | the viewer shows the map: a tree on the left, boxes with arrows on the right; "MAP" in the tab row | |
| 2 | Look at the boxes | the composition you are in has a green outline; an arrow points from it to each composition it holds | |
| 3 | Click a box | that composition opens in the timeline; its box and its tree line light up | |
| 4 | Click Tree, then Flowchart, then Both | only the tree; only the boxes; both | |
| 5 | Click MAP in the tab row | the picture comes back; click it again and the map comes back | |
| 6 | Click the × beside MAP | the map is gone and the picture is back | |
| 7 | Press Z, then click the picture | the picture gets twice as big around the spot you clicked | |
| 8 | Alt+click the picture | it gets half as big | |
| 9 | Press H, then drag the picture | the picture moves about inside the viewer | |
| 10 | Press V | back to the selection arrow; clicking a layer selects it again | |

Anything marked ✗, tell me the row number.
