# D-304 / B-189: Motion Tile takes Tile Center, Tile Width and Tile Height

Found by P-26, tutorial 3 (Video Copilot, Colorful Glitch). It shrinks Motion Tile's tiles to 28 per cent across to repeat a grid texture many times. Here, a tile was always the drawing at its own size, centred on its middle.

## What changed

Motion Tile has three new settings, named as in After Effects. All three take keys.

| Setting | What it does | Where it starts |
|---|---|---|
| Tile Center | where a tile sits, in per cent of the drawing's width and height | 50, 50 (the middle) |
| Tile Width | each tile's width, 1 to 1000 per cent of the drawing | 100 |
| Tile Height | each tile's height, 1 to 1000 per cent of the drawing | 100 |

At their starting values Motion Tile draws exactly as before, using the same code, so older files look the same. Each setting is saved to the file only once it is moved or keyed, so older files also save the same.

Output Width and Height still make the layer bigger, as before. Tile Width and Height change what fills the layer, not its size. Mirror still turns every other tile over.

The graphics card can only tile at the drawing's own size, so the CPU draws resized tiles. The picture is the same either way; only the speed differs.

Also fixed in the panel: a point setting still at its starting value (Fractal Noise's Offset Turbulence, from D-299, and Tile Center) showed one box instead of two. With Tile Center this crashed the effect's panel. Both now show both numbers.

## Checks (cargo test)

`tests/b189_motion_tile_size.rs`, 3 of 3 pass. Each was worked out by hand on an 8x8 white square whose left half is wiped clear, so a row of the drawing reads 0 0 0 0 1 1 1 1:

| Check | Expected row | Got |
|---|---|---|
| Nothing set | 0 0 0 0 1 1 1 1, as before | so |
| Tile Width 50 | 1 1 0 0 1 1 0 0 (two half-size copies) | so |
| Tile Width 50, Mirror on | 0 0 0 0 1 1 1 1 | so |
| Tile Width 50, Tile Center 25 | 0 0 1 1 0 0 1 1 | so |
| Tile Width 50, Output Width 200 | 1 1 0 0 sixteen pixels across | so |
| Tile Width 200 (middle half, doubled) | 0 0 0 0.25 0.75 1 1 1 | so, to 0.000001 |
| Tile Height 50, square wiped from the top | the same down a column | so |
| Tile Width 50 only, down a column | unchanged | so |
| The graphics-card preview, Tile Width 50 | 255 255 0 0 255 255 0 0 | so |
| Saving: nothing set / two set / a file that wrote 100 | none written / those two kept / kept | so |

Unchanged and still passing: Motion Tile's fixtures FX-TILE-001 to 023 (`tests/b97_motion_tile.rs`), the effect-cost table, the whole core suite and the app suite. The app's command check now includes the three new settings.

## Pictures (the test copy, never the owner's app)

A 320x180 Fractal Noise block pattern (size 30) with Motion Tile at Output Width and Height 200.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-304 pictures/1_tiles_at_full_size_as_before.png` | as they start | the pattern repeated at full size, as before | pass |
| `D-304 pictures/2_tile_width_28.png` | Tile Width 28 | the same rows, squeezed into many narrow columns | pass |
| `D-304 pictures/3_width_28_height_50_mirrored.png` | Tile Width 28, Tile Height 50, Mirror on | narrow and short tiles, each next one turned over | pass |

## For the owner to try

1. Put Motion Tile on a pattern and set Output Width and Height to 200.
2. Lower **Tile Width** to about 28: the pattern repeats in narrow strips, as in the glitch tutorial.
3. Move **Tile Center** to slide the tiles.

Not built: After Effects' Phase and Horizontal Phase Shift, which slide alternate rows of tiles.

Fixtures are unchanged.
