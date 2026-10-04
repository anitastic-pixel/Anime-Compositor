# D-303 / B-188: Gaussian Blur takes Blur Dimensions

Found by P-26, tutorials 3 (Video Copilot, Colorful Glitch) and 4 (Lightsaber). They blur along one direction only to make streaks: After Effects' Blur Dimensions, or its Fast Box Blur set to Horizontal. Here, a Gaussian Blur always blurred both ways.

## What changed

Gaussian Blur ("Blur" in the effect list) has a new choice, **Blur Dimensions**:

| Choice | What it does |
|---|---|
| Horizontal and Vertical | as before, and where it starts |
| Horizontal | spreads each pixel left and right only |
| Vertical | spreads each pixel up and down only |

A file without it blurs both ways, so older files look and save as before. A wrong word in a file (say "diagonal") is reported and the effect is left out, as a wrong Edges word is.

The graphics card blurs both ways only, so a one-way blur is drawn on the CPU. The picture is the same; only the speed differs.

## Checks (cargo test)

`tests/b188_blur_dimensions.rs`, 3 of 3 pass. Worked by hand on one white pixel in a 16x16 frame, Radius 1; the blur reaches 3 pixels each way, with document 21's seven weights:

| Check | Expected | Got |
|---|---|---|
| Horizontal | only the dot's row is lit, the seven weights across it | so, to 0.000001 |
| Vertical | only the dot's column is lit, the seven weights down it | so |
| Horizontal and Vertical | each pixel the across weight times the down weight, as before | so |
| No Blur Dimensions in the file | the same as Horizontal and Vertical | the same |
| Horizontal with Edges "repeat" on a one-pixel layer | the dot alone | so |
| The graphics-card preview, Horizontal | only row 8 lit, 7 pixels across | so |
| Saving: not set / a file that wrote "both" / vertical | not written / kept / kept | so |
| "diagonal" | "Blur's dimensions are "both", "horizontal" or "vertical", and this is "diagonal"." | so |

Unchanged and still passing: the blur fixtures (`tests/b07_effects.rs`), edges (`tests/b52_edges.rs`), the GPU chain, the whole core suite (263 targets; only the two old scratch files fail, as before) and the app suite (89 pass).

## Pictures (the test copy, never the owner's app)

A dark blue background and a 60x60 warm yellow square with Blur at Radius 25.

| Picture | Blur Dimensions | Look for | Pass? |
|---|---|---|---|
| `D-303 pictures/1_both_as_before.png` | Horizontal and Vertical | a soft round glow, as before | pass |
| `D-303 pictures/2_horizontal_streak.png` | Horizontal | a sideways streak; the top and bottom edges stay sharp | pass |
| `D-303 pictures/3_vertical_streak.png` | Vertical | an up-and-down streak; the left and right edges stay sharp | pass |

The window run also read the choices: "Horizontal and Vertical, Horizontal, Vertical".

## For the owner to try

1. Put Blur on a small bright shape and raise Radius.
2. Set **Blur Dimensions** to Horizontal: a light streak, as for a lightsaber's or a glitch's smear.

Fixtures are unchanged.
