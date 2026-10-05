# D-319 / B-200: Float working depth

From D-308, approved by the owner on 2026-10-04, after P-26. In tutorial 2 (Advanced Electric) the reflection stub is a copy of the bolt in Add, pushed 12 stops up. After Effects draws it in 32 bits per channel, where it blazes. Here Add was held to 1, so the stub stayed a dim grey.

## What changed

- The New composition and Composition settings window has a **Working depth** row:
  - **Display (0 to 1)** is how every composition drew before.
  - **Float (past white)** is After Effects' 32 bits per channel.
- In Float:
  - **Add** is not held to 1.
  - **Screen** keeps the brighter of the two where both are past white (Nuke's rule). The plain formula would turn dark there.
  - **Fractal Noise** goes past white. It still stops at black.
- Everything else draws the same in both depths. Document 21 lists the effects that stay held to 0 to 1 in Float too: Overlay, Soft Light, Solid Composite, Echo, Glow, Light Wrap and the colour corrections.
- A composition inside another draws in the outermost one's depth, as in After Effects.
- An old file is Display, draws exactly as before and is saved as it was. Float is saved as `float_depth: true`. A wrong value in a file is refused. Changing it is one undo step.
- The preview card does not draw Float yet. Such a frame is drawn by the CPU and says `GPU_PREVIEW_ON_CPU`, so the viewer and an export always match.

## Checks (cargo test)

All 37 checks pass, in `verification/D-319_float_depth_table.md`:

| Check | Expected | Got |
|---|---|---|
| FX-BLEND-ADDF-001 to 003: Add in Float and in Display | every pixel as `Fixtures/float_depth/expected_float_depth.json`, within 2e-5 | so |
| FX-BLEND-SCRF-001 to 003: Screen in Float (both past white, one past white) and in Display | the same | so |
| FX-FNOISE-HDR-001 to 003: Fractal Noise past white in Float, held in Display, its Screen in Float | the same | so |
| A file without the setting | Display, nothing written | so |
| Set to Float by the command | written, read back, draws ADDF-001's frame | so |
| Undo | back to Display, ADDF-002's frame, nothing written | so |
| `float_depth` of "yes", 1 or null in a file | refused | refused |
| A composition inside another: Float outside, Display inside; Display outside, Float inside; both Float | the outer one's depth each time | so |
| The preview card, Float | handed to the CPU, the CPU's picture exactly | so |
| The preview card, Display | drawn on the card as before | so |
| Every fixture project from before | saved byte for byte as before | so (D-253, D-261, D-263, D-264 tables) |

Also passing: the whole core suite and the app suite (see below).

## Pictures (the test copy, never the owner's app)

The scene is P-26's probe of tutorial 2's reflection stub, in `target/p26/d319_steps.js`:
- a 640 by 360 composition with a black Ground solid;
- a thin blue Bolt solid (0, 0.09, 0.45) above it in Add, with Blur 30 and then Exposure.

In `verification/D-319 pictures/`:

| Picture | Look for | Pass? |
|---|---|---|
| `1_display_exposure_0.png` | Display, Exposure 0: a soft dark-blue smear. | |
| `2_display_exposure_12_grey_stub.png` | Display, Exposure 12: still a soft, dim teal smear, fading into black. Twelve stops brighter has hardly brightened it, because Add is held to 1. This is the dull stub P-26 found. | |
| `3_float_exposure_12_bright.png` | The same layers in Float: a solid, blazing cyan bar with a blue rim. The pixels are past white, which the screen can only show as full cyan. The bar above the viewer says "drawn on the CPU, not the card". | |
| `4_settings_window_float.png` | Composition settings, Basic tab: the last row, **Working depth**, reads **Float (past white)**. | |
| `5_undo_back_to_display.png` | One undo: the status line says "Undone: Change the settings of Stub", and the picture is picture 2 again. | |

The run also read back the project after each step:
- after Float, the composition was saved with `float_depth: true`;
- after the undo, the line was gone.

The owner's own Unsaved project, recent list and window were put back after the run.
