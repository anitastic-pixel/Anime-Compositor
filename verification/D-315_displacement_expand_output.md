# D-315 / B-196: Displacement Map's Expand Output

Found by P-26. Tutorial 3 (Colorful Glitch) pushes slices of a picture sideways past the layer's edge, using After Effects' Expand Output. Here, a push stopped at the layer's edge.

## What changed

- Displacement Map has a new row, **Expand Output**, Off or On. It starts Off, so every existing project draws as before.
- On, the layer first grows on every side by the larger of the two maximums, so what is pushed out lands past the old edge. Past the layer's edge, the map's nearest edge colour carries on outward.
- With **Wrap Pixels Around** on, Expand Output does nothing, as in After Effects.

## Checks (cargo test)

`tests/b196_displacement_expand.rs`: 2 of 2 pass. The setup is a white 8 x 6 card pushed 4 pixels right by a map that is red everywhere.

| Check | Expected | Got |
|---|---|---|
| Off, or not in the file | the card cut at its old right edge: 4 pixels shown | so |
| On | the card's last 4 pixels land past the old edge: all 8 shown | so |
| Wrap on, Expand off or on | the same picture both ways | so |
| Saved | not written when the file lacked it; kept as "on"; kept as "off" when the file had it | so |
| The wrong word "sideways" | the effect left out and a sentence naming the two words | so |

Still passing: FX-DMAP-001 to 031 (`tests/b128_displacement_map.rs`), Gradient Wipe (`tests/b129_gradient_wipe.rs`), the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

A white 320 x 180 card in a 640 x 360 composition, pushed 100 pixels right by a hidden red map.

| Picture | Look for | Pass? |
|---|---|---|
| `D-315 pictures/1_expand_off.png` | the card moved right and cut at its old right edge; the layer's box is where the card was | pass |
| `D-315 pictures/2_expand_on.png` | the whole card, its full width, 100 pixels right of where it was; the layer's box wider by 100 on each side | pass |
| `D-315 pictures/3_wrap_on_expand_on.png` | Wrap on: the card whole and in place, as before D-315 | pass |

## For the owner to try

1. Put **Displacement Map** on a picture, with a map layer and a large **Max Horizontal Displacement**. The pushed part is cut at the layer's edge.
2. Turn **Expand Output** On. The pushed part now carries on past the edge.
