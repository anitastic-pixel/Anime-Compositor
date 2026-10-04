# D-310 / B-193: Bulge's Vertical Radius and Taper Radius

Found by P-26, tutorial 1 (Shockwave). In After Effects, Bulge has a Horizontal Radius, a Vertical Radius and a Taper Radius, so it can swell a tall oval with a soft edge. Here it was always a circle with one fixed edge.

## What changed

Bulge has two new rows under Height:

- **Vertical Radius**, 0 to 10000 pixels: how far it reaches up and down. At 0 it follows Radius, so the bulge stays a circle, as before.
- **Taper Radius**, 0 to 10000 pixels: the swell fades to nothing over this many pixels in from the edge. At 0 the edge is as before.

Both can be keyed. One thing to know: a keyed Vertical Radius that passes through 0 snaps to a circle for that frame, because 0 means "follow Radius".

With both at 0 the effect draws exactly as before. Its 23 fixtures (FX-BULGE-001 to 023) still pass. Older files read and save the same: a new row is written only once it is changed.

The graphics card draws only the plain circle. An oval or a taper is drawn on the CPU, so the preview and the export match.

## Checks (cargo test)

`tests/b193_bulge_radii.rs`, 5 of 5 pass:

| Check | Expected | Got |
|---|---|---|
| Both at 0, and Vertical Radius equal to Radius | the same picture as before, pixel for pixel; a pixel 4 from the centre moves as D-152's rule says | so |
| Radius 20, Vertical Radius 10 | a pixel 4 below the centre moves as the oval rule says; one 12 below is outside and stays; one 12 across is inside and moves | so |
| Radius 20, Taper Radius 5 | a pixel 3 from the edge gets 0.648 of the swell; one 10 from the edge and the middle are as before | so |
| Half-size draft | Radius, Vertical Radius and Taper Radius all halve | so |
| Saving | an old file writes nothing new; a set one is kept; one never set stays unwritten | so |

Unchanged and still passing: FX-BULGE-001 to 023 (`tests/b95_bulge.rs`), the effect-cost table, the whole core suite and the app suite.

## Pictures (the test copy, never the owner's app)

A grey solid made into a grid of black dots with Halftone (size 24), then Bulge at Radius 120, Height 2.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-310 pictures/1_circle_as_before.png` | as before | a round swell in the middle; the panel shows Vertical Radius and Taper Radius | pass |
| `D-310 pictures/2_vertical_radius_200.png` | Vertical Radius 200 | the swell is a tall oval, the same width as before | pass |
| `D-310 pictures/3_vertical_200_taper_80.png` | and Taper Radius 80 | the same oval, its outer rings of dots bent less, so the edge is softer | pass |

## For the owner to try

1. Put Bulge on any picture with a pattern in it.
2. Set **Vertical Radius** to twice the Radius: the swell becomes a tall oval.
3. Set **Taper Radius** to about half the Radius: the edge of the swell softens.
