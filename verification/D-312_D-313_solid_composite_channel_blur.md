# D-312, D-313 / B-195: Solid Composite and Channel Blur

Found by P-26. Tutorial 2 (Advanced Electric) lays its lightning on a solid colour with After Effects' Solid Composite, and tutorial 3 (Colorful Glitch) blurs red, green and blue apart with Channel Blur. Here there was neither.

## What changed

- **Solid Composite** (Effects panel, Color Correction): the layer, at **Source Opacity**, laid by a **Blending Mode** (Normal, Add, Screen or Multiply) on a solid **Color** at **Opacity**. It starts as After Effects' does: the layer over white. It does not grow the layer.
- **Channel Blur** (Blur & Sharpen): **Red, Green, Blue and Alpha Blurriness**, each 0 to 500 in Blur's units, with Blur's **Edges** and **Blur Dimensions**. All 0 changes nothing, which is how it starts. Four the same is exactly Blur.
- Both are drawn on the CPU, so the preview and the export match.

## Checks (cargo test)

`tests/b195_solid_composite_channel_blur.rs`, 4 of 4 pass:

| Check | Expected | Got |
|---|---|---|
| Solid Composite, each mode and opacity, worked by hand on two pixels | the hand result | so |
| Solid Composite's size | the layer does not grow | so |
| A wrong mode, colour or opacity | refused with a sentence | refused |
| Channel Blur, all four at 0 | the picture unchanged, bit for bit | so |
| Channel Blur, four the same, both edge settings, all three dimensions | Blur's picture, bit for bit, the same size | so |
| Red alone on a red/green line | green, blue and alpha unchanged; red soft across the line, even about it | so |
| Alpha alone on a one-colour drawing | Blur's picture, no dark rim, grows as Blur | so |
| A half-size draft | all four blurs halved | so |
| Both effects saved and read back | written as read | so |

Unchanged and still passing: the fixture round trip of every fixture project (`tests/d253_d254_roundtrip.rs`), the whole core suite and the app suite (89 pass).

## Pictures (the test copy, never the owner's app)

A white 320 x 180 card in a 640 x 360 composition.

| Picture | Look for | Pass? |
|---|---|---|
| `D-312 D-313 pictures/1_channel_blur_green_12_blue_40.png` | the card still white; at most a thin coloured fringe at its edges, since only green and blue are blurred and the alpha is not | pass |
| `D-312 D-313 pictures/2_and_alpha_20.png` | the card's edges now soft all round | pass |
| `D-312 D-313 pictures/3_solid_composite_navy_normal.png` | the soft card on a navy rectangle the size of the layer | pass |
| `D-312 D-313 pictures/4_screen_on_brown_source_60.png` | Screen on brown at Source 60: a pale card fading into a brown rectangle | pass |

## For the owner to try

1. Put **Channel Blur** on a drawing. Raise **Blue Blurriness** only: blue spreads past the lines, red and green stay sharp.
2. Put **Solid Composite** on a lightning layer, colour dark blue, mode Screen. The bolt glows over the blue as in the tutorial.
