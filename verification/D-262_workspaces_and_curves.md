# D-262: workspaces keep their own changes, and curves behind have their own height

Two of your answers from the W-46 page, both page-only.

**1. Each workspace keeps the changes you make in it.**
- Move a panel or drag a border in Compose, go to Animate, and come back: Compose is as you left it.
- Animate keeps its own changes the same way, and so does Render.
- **Reset workspace** puts the one you are in back as it was built, or as you saved it, and forgets its changes. The other workspaces keep theirs.
- **Save workspace…** keeps the arrangement under a new name. The workspace you made the changes in then goes back as built, because the changes now live in the saved one.
- A workspace you never changed keeps nothing of its own, so a later build's improved layout still reaches it.

**3. Curves behind the front one fill the graph's height**, each on its own scale. Before, they shared the front one's, so a Rotation in degrees was a nearly flat line under a Position in pixels. The numbers at the side are still the front curve's.

Your other three answers needed no change: the Sheet keeps paper order, a render is watched in Render, and there is no text tool.

## Picture

`D-262 pictures/curves_own_height.png`: layer2 has Position in front (blue) and Rotation ticked behind (green). Rotation's three keys (0°, 30° and −10°) reach from the bottom of the graph to its top.

## What it does, checked in the running app (`d262_check.js`)

Nothing was saved. The window's own layout and settings were put back afterwards.

| Step | What the page said |
|---|---|
| Compose as built | Effects in the lower left, Project 300 wide, nothing kept |
| Moved Effects to the right, made Project 420 wide | Effects on the right, Project 420 |
| Went to Animate | Animate as built (Effect controls, Project and Effects at the left, Sheet column out); Compose's changes kept |
| Put the Sheet away in Animate | Sheet column away |
| Back to Compose | Effects on the right, Project 420 wide, as left |
| Back to Animate | Sheet column still away, Effects at the left |
| Went to Render, untouched | kept: Animate's and Compose's changes only |
| Reset in Compose | Effects back in the lower left, Project 300; only Animate's changes kept |
| Animate | still as left: Sheet column away |
| Reset in Animate | Sheet column out again; nothing kept |
| The graph, Position in front, Rotation ticked | the graph is 221 high; Position's line spans 16 to 215; Rotation's also spans 16 to 215, with its keys at 165, 16 and 215; numbers at the side 500, 1000, 1500 (Position's) |
| Errors on the page | none |

## Limits, stated

- **The kept changes live in this window's storage**, like the panel sizes always have. They aren't in the project, so another machine lays out as it last had it.
- **The numbers at the side belong to the front curve only.** To read a value behind, hover its key dots, or click its name to bring it to the front.

## Checks

- All 81 of the app's tests pass. No wiring, key or command changed.
- No exported picture can change. No engine code was touched.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | In Compose, drag the Effects panel's name onto the right side, then click Animate, then Compose | Effects still on the right | |
| 2 | In Animate, press × on the Sheet, then Compose, then Animate | the Sheet still put away | |
| 3 | In Compose, Window › Reset workspace | Compose as it first was; Animate keeps its put-away Sheet | |
| 4 | Give layer2 keys on Position and Rotation, open the graph and tick Rotation | the Rotation curve as tall as the Position curve | |

Anything marked ✗, tell me the row number.
