# B-584: Particle Plane's Textured Square

Built on 2026-10-13 as D-714 (P0-6), after your decision that day: fixture FX-PPLANE-055 (and
D-710's FX-PPTEX-024, which said the same) now expects the word "cube" to be refused instead of
"textured_square", and Particle Plane gains **Textured Square**.

- **Particle Type** adds **Textured Square**: each particle is a square showing the Texture
  Layer, turned by Rotation Speed and Initial Rotation exactly as the textured discs are (B-580).
  With no Texture Layer it is a plain square in the particle's colour.
- It is Particle Space's Textured Square (D-611), on the flat emitter.

The check, `verification/D-714_particle_plane_square_table.md` (98 of 98), holds every pixel to
numbers worked out by a separate program before the code existed, and the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-714 pictures/`.

## What to check

1. **Fountain of street squares.** `1_street_squares_fountain_over_the_street.png`: a column of
   small turning squares, each showing a copy of the street picture, shot up from the middle over
   the street (Composite With Original On).
2. **Upright squares on an empty layer.** `2_street_squares_upright_on_empty.png`: larger squares,
   not turned, each a small upright copy of the street, on an empty layer (the empty part is
   see-through, which a picture viewer may show as white or as a checkerboard).
3. **In the app.** Add Particle Plane to a layer, set Particle Type to Textured Square, pick a
   layer under Texture Layer, and play. Raise Rotation Speed: the squares turn.
4. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

`verification/B-584_particle_plane_square_timing_table.md`, measured on a quiet machine (one run):
textured squares cost the same as textured discs, within a millisecond, on the card and on the
processor.
