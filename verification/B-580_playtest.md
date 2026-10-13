# B-580: Particle Plane's textured particles and Composite With Original

Built on 2026-10-13 as D-710 (P0-6), under your /loop request. Particle Plane (B-492) gains
what Particle Space already had (D-611), and one new switch:

- **Particle Type** adds **Textured Disc** and **Textured Faded Disc**: each particle shows a
  layer of the composition, the faded one with a soft edge.
- **Texture Layer**: the layer they show. Left empty, the textured kinds draw as plain discs.
  A layer name that is not in the composition leaves the effect out with a warning.
- **Rotation Speed** (degrees a second) and **Initial Rotation** (how far each particle may
  start turned): how the textured particles turn.
- **Composite With Original**: Off (as before) draws the particles on an empty layer, so the
  layer's own picture disappears. On draws them over the layer's picture.

Textured Square is still refused on the Plane: fixture FX-PPLANE-055 expects it to be, and
changing that is your decision. Black Matte and Hold Particle Release are still not built (no
source says what they do).

The check, `verification/D-710_particle_plane_texture_table.md` (164 of 164), holds every pixel
to numbers worked out by a separate program before the code existed, and the graphics card's
picture to within 1 level of the processor's. Files saved before this change open and save
exactly as before. The pictures are in `verification/D-710 pictures/`.

## What to check

1. **As added, over the street.** `1_as_added_over_the_street.png`: the usual thin yellow-to-red
   streaks bursting from the middle, and the street still visible under them (Composite With
   Original On).
2. **Fountain of street discs.** `2_street_discs_fountain.png`: a column of round discs, each
   showing a small turned copy of the street picture, shot up from the middle over the street.
3. **Faded discs in a vortex.** `3_street_faded_discs_vortex.png`: soft-edged discs showing the
   street, swirled over the whole picture, the street visible behind.
4. **On an empty layer.** `4_street_discs_on_empty.png`: the same kind of discs with Composite
   With Original Off: only the discs, no street behind them (the empty part is see-through, which a picture
   viewer may show as white or as a checkerboard).
5. **In the app.** Add Particle Plane to a layer, set Particle Type to Textured Disc, pick a
   layer under Texture Layer, and play. Raise Rotation Speed: the discs turn. Switch Composite
   With Original On: the layer's own picture shows under the particles.
6. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: the textured discs work as Particle Space's do (D-611);
Composite With Original is Off when added, so older projects look the same.

## Speed

`verification/B-580_particle_plane_texture_timing_table.md`. PROVISIONAL (another lane's test shared the graphics card): on the reference shot, textured discs showing a full-size layer on each of three layers add about 20 to 70 ms a frame on the graphics card and 250 to 350 ms on the processor. A quiet re-measure is owed.
