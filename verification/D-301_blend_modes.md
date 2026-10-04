# D-301 / B-186: four more blend modes: Overlay, Soft Light, Stencil Alpha, Stencil Luma

Found by P-26, tutorials 2 (Video Copilot, Advanced Electric) and 3 (Video Copilot, Colorful Glitch). They put layers in Overlay and Soft Light to tint a picture, and cut a picture to a shape with Stencil Alpha. A layer here could only be Normal, Multiply, Screen or Add.

## What changed

The blend list in the timeline's Mode column, the right-click menu and the command search now have eight modes:

| Mode | What it does |
|---|---|
| Overlay | darks get darker and lights get lighter under the layer's colour; a 50% grey changes nothing |
| Soft Light | the same idea, gentler |
| Stencil Alpha | the layer is not drawn; everything beneath shows only where the layer is, by how solid it is, and the rest of the frame is cleared |
| Stencil Luma | the same, by how bright the layer is: white keeps, black clears |

Overlay and Soft Light mix the colours as After Effects does, on the colours as you see them, not as stored. Older files are unchanged, since none could name the new four. A file naming a mode that isn't one of the eight is still refused, and the reason lists the eight.

The graphics card does not draw the four new modes. A frame that uses one is drawn on the CPU, so the picture is right but slower, and the viewer says so.

## Checks (cargo test)

`tests/b186_blend_modes.rs`, 4 of 4 pass. Worked by hand on a 16x16 backdrop, colours as you see them (0 to 1):

| Check | Expected | Got |
|---|---|---|
| Overlay with a 50% grey layer | the backdrop unchanged | so, to 0.00001 |
| Soft Light with a 50% grey layer | the backdrop unchanged | so |
| Overlay, (0.8, 0.3, 0.5) over (0.25, 0.75, 0.5) | (0.4, 0.65, 0.5) | so |
| Soft Light, the same | (0.4, 0.675, 0.5) | so |
| Stencil Alpha, 8x8 at 50% opacity in the top left quarter | half the backdrop there, all clear elsewhere | so, in tiles of 4, so the shortcut that skips empty tiles is tested too |
| Stencil Luma, 8x8 of grey 0.2 (stored) | the backdrop times 0.4845 there, all clear elsewhere | so |
| The file with each of the eight | kept as written | kept |
| Mode "dodge" | refused, naming the place and the eight | refused |
| The graphics card with Multiply | draws it | so |
| The graphics card with each of the new four | hands the frame to the CPU and says so | so |

Unchanged and still passing: the blend fixtures (`tests/b05c_blend.rs`), the adjustment layer blends (`tests/b182_adjustment_blend.rs`), the culling table (`tests/p05_culling.rs`), the GPU chain (`verification/B-155_gpu_chain_table.md`) and the GPU preview (`tests/b44_gpu_preview.rs`). App suite: 89 pass.

## Pictures (the test copy, never the owner's app)

A 640x360 background of orange-and-blue Fractal Noise clouds, and a pink 320x180 solid in the middle.

| Picture | Mode | Look for | Pass? |
|---|---|---|---|
| `D-301 pictures/1_normal.png` | Normal | a flat pink box over the clouds | pass |
| `D-301 pictures/2_overlay.png` | Overlay | the box is pink, but the clouds show through it, darker and lighter | pass |
| `D-301 pictures/3_soft_light.png` | Soft Light | the same as Overlay, softer | pass |
| `D-301 pictures/4_stencil_alpha.png` | Stencil Alpha | the clouds only inside the box; outside is empty (checkerboard) | pass |
| `D-301 pictures/5_stencil_luma.png` | Stencil Luma, with Fractal Noise on the box | the clouds inside the box only where the box's noise is bright; dark holes where it is dark | pass |

The window run also read the blend list: "Overlay, Soft light, Stencil alpha, Stencil luma" are in it.

## For the owner to try

1. Put any picture at the bottom and a coloured solid above it.
2. In the timeline's Mode column, set the solid to Overlay, then Soft Light: the picture shows through, tinted.
3. Set it to Stencil Alpha: only the part of the picture under the solid remains. Add a mask to the solid to cut the picture to any shape.

Not built: After Effects' other modes (Color Dodge, Linear Light, Difference, Hue and the rest) wait until a tutorial needs one. Silhouette Alpha and Luma, the stencils turned inside out, would be small additions if wanted.

Fixtures are unchanged.
