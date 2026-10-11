# B-405: IDentifier

Built on 2026-10-11 as D-525, under your /loop request: IDentifier, in the **3D Channel** group.
A 3D program can save, with each picture, a number for every object (an "object id") and for
every material (a "material id") in an EXR file. This effect shows those numbers, so you can see
which objects you can cut out, and cut one out.

- **Channel** (Object ID): the object ids or the material ids.
- **ID** (0): the id the two mattes keep; it can be keyed.
- **Output** (Colors): Colors shows every id as its own flat colour; Luma Matte shows the chosen
  id white on black; Alpha Matte keeps the picture only where the chosen id is; Raw shows the ids
  as dark greys whose 8-bit value is the id, so a colour picker reads it.

The plug-in's maker does not publish its colours, so ours are our own: they will not match After
Effects' Colors picture, only its purpose. You type the id; picking it by clicking the picture is
not built (a later decision, as for ID Key).

The check, `verification/D-525_identifier_table.md` (192 of 192), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-525 pictures/`, made from the small
render of three balls the id effects already use
(`Fixtures/depth_channel/sample/spheres_blender.exr`).

## What to check

1. **Before.** `before.png`: a red ball near, a blue one further, a yellow one far, on a checked
   floor.
2. **Object colours.** `object_colors.png`: the sky, the floor and each ball a flat colour of its
   own, every ball's outline sharp.
3. **Material colours.** `material_colors.png`: the sky one colour, the floor's two kinds of
   square two colours, the red and blue balls sharing a colour (they share a material), the
   yellow ball another.
4. **Luma matte.** `luma_matte_object_2.png`: the blue ball white, everything else black.
5. **Alpha matte.** `alpha_matte_material_3.png`: the red and blue balls in their own colours,
   everything else transparent.
6. **Raw.** `raw_objects.png`: nearly black; the sky darkest (id 0) and the floor a little
   lighter (id 4).
7. **In the app.** Import an EXR with object ids (Blender's IndexOB pass, for one) and add
   IDentifier (3D Channel): every object a colour. Switch Output to Luma Matte and raise ID: one
   object after another turns white.
8. **No ids.** On a PNG or an EXR without ids, the picture is unchanged and a warning says the
   channel is missing.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: whether the colours are distinct enough, and typing the
id until clicking exists.

## Speed

On a 1920 by 1080 EXR at Full a frame took about 4 milliseconds with IDentifier, the same as without it (PROVISIONAL: the machine was busy) (`verification/B-405_identifier_timing_table.md`)
