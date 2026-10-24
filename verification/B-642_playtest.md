# B-642 playtest: in 8 bpc (After Effects) colours mix in display values (D-763)

## What to try

1. Make a composition, open Composition Settings and set the working depth to **8 bpc (After Effects)**.
2. Put a white solid at the bottom and a blue drawing or solid above it, smaller than the frame. Add **Drop Shadow** to the blue layer: colour black, Opacity 50 %, Softness 0, Distance 10.
3. Look at the shadow where it falls on the white. It is mid grey, as in After Effects (about 127 of 255). Before this build it was light grey (188).
4. Set the depth back to **Display**. The shadow goes back to light grey: Display looks exactly as before.
5. In 8 bpc again, put a layer set to **Screen** or **Add** over a coloured one. It lightens more than before, as After Effects' does. A layer at 50 % opacity, or a feathered mask edge, looks darker than before.
6. Look at the viewer's note while an 8 bpc composition plays: "The CPU drew this frame: its composition works in 8 bpc (After Effects)." The graphics card does not draw 8 bpc frames.

## What the check found: 52 of 52 pass

Fixture check `tests/b642_display_mix.rs` (see `B-642_display_mix_table.md`). FX-MIX8-001 to 012 are Screen, Add, Normal at 50 %, a Drop Shadow at 50 %, a feathered mask and a Mosaic, each in 8 bpc and in Display, compared with `tools/display_mix_reference.py`, which works the rule on its own. Each 8 bpc scene switched to Display draws the old linear frame. A composition inside another follows the outer one's depth. The five old 8 bpc projects (FX-8BPC-001 to 005) open with no warnings and draw as before. The app's tests: 118 passed, 0 failed.

## Pictures (`B-642 pictures/`)

Each sheet shows three frames side by side: After Effects' own frame from a tutorial, this build before, this build after, with the difference numbers printed (levels of 255; smaller is closer to After Effects).

- `threeway_km1_screen.png`: Screen. Median difference 7 before, 3 after; overall brightness 56.7 before, 63.2 after, After Effects 62.9.
- `threeway_km4_add.png`: Add. Median 30 before, 4 after; brightness 157.5 before, 186.2 after, After Effects 188.0.
- `threeway_gds2_multiply.png`: Multiply at 70 %. Median 4 before, 2 after; 90th percentile 24 before, 5 after.
- `threeway_jim_dropshadow50.png`: Drop Shadow 50 %. Shadow strength 0.373 before, 0.5 after, After Effects 0.5.
- `threeway_vlad_mosaic.png`: Mosaic. Average block error 18.3 before, 1.9 after.
- `threeway_bullo_gauss_edge.png`: a Gaussian Blur edge over a background. Edge difference 37.7 before (ours too light), 14.7 after.

## How this differs from After Effects

- bullo's blur is about 13 % wider than After Effects'; that is the blur's width, not the mixing, and is not changed here.
- jim's sheet uses a stand-in drawing, so only the shadow's strength is compared.
- Bevel Alpha and motion blur are unchanged; an adjustment layer's effects still work in linear light.
- After Effects' 32 bpc (without Linearize Working Space) also mixes display values; our 32 bpc (After Effects) stays linear, as you picked.
