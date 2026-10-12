# B-491: Particle Space's textured particles

Built on 2026-10-12 as D-611, the second step of the particle engine (P0-6), under your /loop
request. Particle Space (Generate) gets three more **Particle Types**, after what CycoreFX's CC
Particle World textured types are for:

- **Textured Square**, **Textured Disc** and **Textured Faded Disc**: each particle shows a layer
  of the composition, picked in **Texture Layer** (the layer is used whole, at its own size, and
  can be switched off so it only feeds the particles).
- **Rotation Speed** (0 when added, -3600 to 3600 degrees a second) turns each particle as it
  flies; **Initial Rotation** (0 when added, up to 360 degrees) starts each one turned by its own
  random share of that amount. Both are read when the particle is born.

As a particle gets smaller, the texture is taken down through smaller, averaged copies of
itself, so a far, tiny particle shows the texture's overall colour and does not flicker. With no
Texture Layer picked, the textured types are plain shapes in the Color Map's colour. If the
picked layer is deleted, the effect is left out and a warning says so.

CycoreFX publishes no formula and no settings when added, so the rules and those settings are
ours, written down in document 21 and in `tools/particle_texture_reference.py`.

The check, `verification/D-611_particle_texture_table.md` (164 of 164), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the graphics card's
picture to within 1 level of the processor's. The pictures are in `verification/D-611 pictures/`,
drawn over the street picture of D-506, with the street also used as the texture.

## What to check

1. **No texture.** `1_squares_no_texture.png`: small yellow-to-red squares flying out, each
   turned at its own angle.
2. **Street squares.** `2_street_squares_turning.png`: large squares each showing the whole
   street picture, turned every which way, bursting out over the street.
3. **Street discs.** `3_street_discs_fountain.png`: a column of small round discs thrown upward;
   being small, they show the street's average colour (grey-blue) rather than its details.
4. **Fog.** `4_street_faded_discs_fog.png`: soft round discs swirling, seen from a little above;
   the far ones dark with Depth Cue Fog.
5. **In the app.** Put a picture layer in a composition and switch it off. Add Particle Space to
   another layer, set Particle Type to Textured Square and Texture Layer to the picture. Play.
   Raise Rotation Speed and Initial Rotation: the squares turn.
6. **Missing layer.** Delete the picture layer: the particles go and a warning says the texture layer is missing.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.
   An older project with Particle Space opens and looks as before.

Our own choices you may want to judge: the rotation ranges and that Initial Rotation is a random
share per particle; how much far particles soften the texture. Not built, because no source says
what it does beyond its name: CC Particle World's Texture Time (the layer is always read at the
frame being drawn).

## Speed

`verification/B-491_particle_texture_timing_table.md`. PROVISIONAL (the machine was busy with other lanes' builds): on the reference shot, with a 1920 by 1080 layer as the texture, each of three layers adds about 14 ms a frame on the graphics card with 1000 particles alive and about 65 with 6000; on the processor about 92 to 95. A quiet re-measure is owed.
