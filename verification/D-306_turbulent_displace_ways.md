# D-306 / B-191: Turbulent Displace's Displacement and Pinning

Found by P-26, tutorials 4 and 5 (the lightsabers). They set After Effects' Turbulent Displace to push in only one direction and to pin the layer's edges, so a blade wobbles without its ends tearing. Here the effect always pushed both ways, everywhere.

## What changed

Turbulent Displace has two new choices, named as in After Effects.

| Choice | Options | Where it starts |
|---|---|---|
| Displacement | Turbulent (both ways), Horizontal Displacement (sideways only), Vertical Displacement (up and down only) | Turbulent |
| Pinning | None, Pin All (the push fades smoothly to nothing over one Size from each edge) | None |

At their starting values the effect draws exactly as before, and its 26 fixtures FX-TURB-001 to 026 still pass. Older files read and save the same.

Not built: After Effects' other displacement types (Bulge, Twist, Cross, the Smoother ones) and its other pinning choices (each edge, and the locked versions). The tutorials did not need them.

Pin All measures from the edges of the picture the effect receives. That is the layer's own edges, unless an earlier effect on the layer has made it bigger.

The graphics card draws only the starting values. The CPU draws a one-way or pinned push, so the preview and the export match.

## Checks (cargo test)

`tests/b191_turbulent_displace_ways.rs`, 3 of 3 pass, on a 64x64 solid in a 96x96 frame, pushed up to 8 pixels:

| Check | Expected | Got |
|---|---|---|
| Horizontal stripes pushed sideways only | unchanged away from the sides | so |
| The same pushed both ways | the stripe edge moves | so |
| The same pushed up and down only | the stripe edge moves | so |
| A solid pushed both ways, unpinned | its outer pixels fray, below 90% covering | so |
| The same, Pin All | its outer pixels stay above 99%; nothing spills past it | so |
| Its middle, more than one Size from every edge | pushed the same, pinned or not | so |
| Saving: an old file / both set / a wrong word ("some") | nothing new written / both kept / reported | so |

Unchanged and still passing: FX-TURB-001 to 026 (`tests/b70_turbulent_displace.rs`), the effect-cost table, the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

A 400x240 yellow solid cut into horizontal stripes by Venetian Blinds (width 24), then Turbulent Displace at Amount 20, Size 50.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-306 pictures/1_turbulent_as_before.png` | as it starts | stripes wavy and pushed sideways; every outer edge ragged | pass |
| `D-306 pictures/2_horizontal_only.png` | Horizontal Displacement | stripes dead straight; only their left and right ends shift | pass |
| `D-306 pictures/3_vertical_only.png` | Vertical Displacement | stripes wave up and down; left and right edges straight | pass |
| `D-306 pictures/4_turbulent_pin_all.png` | Turbulent, Pin All | the middle wavy; all four outer edges straight; the top and bottom stripes nearly flat | pass |

## For the owner to try

1. Put Turbulent Displace on a long thin layer, such as a lightsaber blade.
2. Set **Displacement** to **Vertical Displacement**: the blade wobbles but stays the same length.
3. Set **Pinning** to **Pin All**: the ends stay put while the middle wobbles.

Fixtures are unchanged.
