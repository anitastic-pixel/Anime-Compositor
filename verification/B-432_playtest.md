# B-432: Pass Extract's alpha channel and UnMult

Built on 2026-10-11 as D-552, under your /loop request, after fnord's EXtractoR. Pass Extract
(3D Channel) already pulled any named channel out of an EXR render (D-349). Two things are new:

- **Channel Names** may now be **four** names with commas between, the fourth the alpha. For a
  Blender render, `ViewLayer.Combined.R, ViewLayer.Combined.G, ViewLayer.Combined.B,
  ViewLayer.Combined.A` gives the picture back with its sky clear. Any channel can be the alpha:
  `ViewLayer.Normal.X, ViewLayer.Normal.Y, ViewLayer.Normal.Z, ViewLayer.Combined.A` shows the
  normals with the sky cut away.
- **UnMult** (Off when added): divides the three values by that alpha first. Turn it on for a
  render whose colour is stored already multiplied by its alpha (most renders are), so soft edges
  are not darkened twice.

With one or three names nothing changes: the picture is solid, as before, and UnMult has nothing
to divide by. Projects saved before open and save exactly as they did.

The check, `verification/D-552_extractor_alpha_table.md` (184 of 184), holds every
pixel to numbers worked out by a separate program before the code existed, and holds the
graphics card's picture to the processor's (every one matched exactly). The pictures are in
`verification/D-552 pictures/`, made from D-349's Blender-style sample render.

## What to check

1. **Before.** `before.png`: the three balls on a floor, the sky clear.
2. **The render through Pass Extract.** `combined_unmult_off.png` and `combined_unmult_on.png`:
   the same picture as before.png (this render's alpha is only 0 or 1, so UnMult changes
   nothing here; the table checks they match).
3. **Normals cut by the alpha.** `normals_over_alpha.png`: the balls and floor in pastel normal
   colours, the sky clear rather than grey.
4. **Mist cut by the alpha.** `mist_over_alpha.png`: near dark, far light, the sky clear rather
   than white.
5. **In the app.** Import an EXR render with an alpha, add Pass Extract, set Pass to Named
   Channel and type four names, the fourth the alpha: the empty parts go clear. Turn UnMult on
   and off on a render with soft edges (motion blur or anti-aliasing): with it on, the edges stop
   looking dark.
6. **A missing name.** Type a fourth name the file lacks: the effect is skipped with a
   missing-channel warning, the picture unchanged.
7. **Saved and opened again.** Save, close and open the project: the same names, UnMult and
   picture.

Not built (EXtractoR has them): the "(copy)" alpha choice, which keeps the layer's own alpha,
and the Layers menu, which fills the four names from one layer.

## Speed

`verification/B-432_extractor_alpha_timing_table.md`: PROVISIONAL (machine not quiet: another lane's b155_gpu_chain run and two cargo processes were running), a 1080p Blender-style EXR still: card 2.1 ms a frame with four names and UnMult off, 2.2 with UnMult on, 2.1 with three names, against 2.1 for the Noise alone (no measurable cost); processor 22.4, 22.2 and 23.1 against 20.9; a quiet re-measure is owed.
