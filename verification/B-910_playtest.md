# B-910 playtest: Particle Space's Tri, Quad and Motion Polygon

What changed (D-1031): Particle Space's Particle Type list has three more choices at the end,
after CC Particle World's flat shapes: Tri, Quad and Motion Polygon.

1. Add Particle Space to any layer (Effects: Particle Space). Turn Composite With Original on so
   the layer shows under the particles. Play a second or two.
2. Set Particle Type to Tri. Set Initial Rotation to 360 and Rotation Speed to 90.
   - Expect: small flat triangles, each pointing a different way and slowly turning, no shading.
   - Compare with `verification/B-910 pictures/1_tris_turning.png`.
3. Set Particle Type to Quad. Set Color Map to Custom.
   - Expect: flat turning squares, like confetti, in the custom colours. Naming a Texture Layer
     changes nothing for Quad (only the Textured types use it).
   - Compare with `verification/B-910 pictures/2_quads_confetti.png`.
4. Set Particle Type to Motion Polygon, Velocity about 1.2, Transfer Mode Add.
   - Expect: streaks pointing outwards along the way each particle flies, thin at the particle and
     wider at the tail, like sparks. With Velocity 0 nothing is drawn (a particle that does not
     move has no direction).
   - Compare with `verification/B-910 pictures/3_motion_polygon_sparks.png`.
5. Save, close and reopen the project.
   - Expect: the Particle Type is still the one you chose.

The three are drawn by the processor, not the graphics card, so a busy Particle Space with them
may preview more slowly than with Disc. Cube is not built (D-1032 is a proposal for you).

The checks are in `verification/B-910_particle_shapes_table.md` (82 of 82): every fixture frame
FX-PSHAPE-001 to 018 matches the reference tool, files reopen unchanged, "cube" and "triangle"
are refused with a sentence, and the card path gives the same pixels.
