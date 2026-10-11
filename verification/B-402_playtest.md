# B-402: Cryptomatte

Built on 2026-10-11 as D-522, under your /loop request: Cryptomatte, in the **3D Channel**
group. Many 3D programs (Blender, Redshift, Arnold, V-Ray, Octane) can save a "Cryptomatte" in an
EXR file: for every pixel, which objects are in it and how much of the pixel each covers. This
effect picks objects by name and gives a matte with soft edges, even at blurred or see-through
edges, where ID Key's edges are hard.

- **Layer** (empty): which Cryptomatte in the file, such as CryptoObject or CryptoMaterial.
  Empty takes the first. Case does not matter.
- **Selection** (empty): the names of the objects or materials to pick, as the 3D program named
  them, a comma between, such as `red ball, blue ball`.
- **Output** (Colors): Colors shows every object in its own flat colour, the picked ones white;
  Matte shows the picked objects white on black; Keyed keeps the picture only on them.

You type the names; in After Effects you click on the objects instead. Clicking to pick is not
built (a later decision). The file's list of names is not shown either, so a name has to be typed
exactly as the 3D program wrote it.

The check, `verification/D-522_cryptomatte_table.md` (228 of 228), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-522 pictures/`, made from a small
render of three balls over a floor (`Fixtures/cryptomatte/sample/balls.exr`).

## What to check

1. **Before.** `before.png`: a red, a blue and a yellow ball over a two-tone floor, blue sky.
2. **Colors.** `colors_objects.png`: each ball, the floor and the sky a different flat colour.
3. **Materials.** `colors_materials.png`: the red and blue balls the same colour (they share the
   material "paint").
4. **One picked.** `colors_floor_picked.png`: the floor white, the rest in colours.
5. **Matte.** `matte_yellow_ball.png`: the yellow ball white with a smooth edge, the rest black.
6. **Keyed.** `keyed_red_blue.png`: the red and blue balls alone, the rest clear (the yellow
   ball, in front, cuts into the blue one).
7. **By material.** `keyed_material_gold.png`: the yellow ball alone.
8. **In the app.** Import an EXR with a Cryptomatte, add Cryptomatte (3D Channel): the colours
   show. Type a name in Selection and press Enter, then set Output to Keyed: only that object
   stays. A name the file does not have keys out everything.
9. **No Cryptomatte.** On a PNG or an EXR without one, the picture is unchanged and a warning
   says the channels are missing.
10. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: typing names until clicking exists, the starting Colors
view, and the colours themselves (they come from each name's number, so they will not match
other programs' colours).

## Speed

On a 1920 by 1080 EXR at Full a frame took about 3.3 to 3.9 milliseconds with Cryptomatte once the file was read, against 3.3 without it (PROVISIONAL: the machine was busy) (`verification/B-402_cryptomatte_timing_table.md`)
