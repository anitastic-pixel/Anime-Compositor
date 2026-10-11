# B-404: Fog 3D

Built on 2026-10-11 as D-524, under your /loop request: Fog 3D, in the **3D Channel** group. A 3D
program can save, with each picture, how far away every pixel is (a "depth" or "Z" pass) in an
EXR file. This effect uses it to lay fog over the picture: clear near the camera, thicker with
distance, as haze in real air.

- **Fog Color** (white) and **Fog Opacity** (100): the fog's colour and how much it can hide.
- **Fog Start Depth** (0) and **Fog End Depth** (10): where the fog begins and where it is
  thickest, in the file's own depth units (Pass Extract shows them).
- **Scattering Density** (50): how soon after the start it thickens; 0 evenly.
- **Foggy Background** (on): the empty sky behind the render filled with fog; off leaves it
  clear for compositing.
- **Gradient Layer** and **Layer Contribution**: a grey layer whose brightness thins the fog
  where it is dark.

Adobe does not say how thick the fog is at each distance, so the rule is ours. You type the
depths; in After Effects you would read them off the picture first. Picking them by clicking is
not built (a later decision).

The check, `verification/D-524_fog_3d_table.md` (237 of 237), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the card to the processor
within 1 level. The pictures are in `verification/D-524 pictures/`, made from the small render of
three balls the depth effects already use (`Fixtures/depth_channel/sample/spheres.exr`).

## What to check

1. **Before.** `before.png`: a red ball near, a blue one further, a yellow one far, on a checked
   floor.
2. **As added.** `as_added.png`: thick white fog almost at once; only the near floor clear, the
   red ball a faint pink, the blue ball barely seen.
3. **Even fog.** `blue_grey_5_to_15.png`: blue-grey fog from 5 to 15; the red ball almost clear,
   the yellow ball deep in fog.
4. **Dense.** `dense_4_to_30.png`: thick soon after depth 4; every ball faded.
5. **Thin.** `warm_opacity_60.png`: warm fog at opacity 60; the far part tinted, never hidden.
6. **Reversed.** `reversed.png`: start 14, end 6, Foggy Background off; the near part white, the
   far part and the sky clear.
7. **In the app.** Import an EXR with a depth pass and add Fog 3D (3D Channel). Raise Fog Start
   Depth: the fog moves back. Key Fog End Depth over a second: the fog rolls in.
8. **No depth.** On a PNG or an EXR without a depth, the picture is unchanged and a warning says
   the channel is missing.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how fast the fog thickens at density 50, and typing the
depths until clicking exists.

## Speed

On a 1920 by 1080 EXR at Full a frame took about 6 milliseconds with Fog 3D, the same as without it (PROVISIONAL: the machine was busy) (`verification/B-404_fog_3d_timing_table.md`)
