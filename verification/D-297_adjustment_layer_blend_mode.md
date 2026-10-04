# D-297 / B-182: An adjustment layer takes a blend mode

Found by P-26, tutorial 3 (Video Copilot, Colorful Glitch): the tutorial sets an adjustment layer to Add so its glitch brightens what is under it. D-66 allowed an adjustment layer only Normal: the Mode list was blank for it, and a file with another mode would not open.

## What changed

- An adjustment layer now has the Mode list on the timeline and the Blend mode row in its Blending group, like any layer: Normal, Multiply, Screen or Add.
- In another mode, the picture the layer's effects make is laid over the picture beneath in that mode, then mixed in by the layer's opacity and mask as before. Normal draws exactly as it always did.
- A file with an adjustment layer in Screen (or any of the four) opens and saves that mode.
- The graphics card does not draw this one case yet; such a frame is drawn by the processor, and the Info line says so.

## Checks (cargo test)

`tests/b182_adjustment_blend.rs`, 2 of 2 pass. A grey solid (linear 0.2) under an adjustment layer with Exposure +1, which doubles it to 0.4. Worked by hand:

| Mode | 100% opacity | 50% opacity (halfway back to 0.2) |
|---|---|---|
| Normal | 0.4 | 0.3 |
| Multiply | 0.4 x 0.2 = 0.08 | 0.14 |
| Screen | 0.4 + 0.2 - 0.08 = 0.52 | 0.36 |
| Add | 0.4 + 0.2 = 0.6 | 0.4 |

Each matched to within 0.000001. The file and the command both take a mode.

`tests/b17b_adjust.rs` (the D-66 checks) passes, its two "refused" rows now reading "opens" and "accepted". The app suite's adjustment-layer walk now checks that Multiply is taken.

## For the owner to try

1. Put an adjustment layer over a picture and add Exposure +1 to it.
2. Open the layer's Mode list: it now offers the four modes. Choose Add. The picture gets brighter than with Normal.
3. Choose Multiply: darker than Normal.

Fixtures are unchanged.
