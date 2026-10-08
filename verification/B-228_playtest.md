# B-228: depth and normals from an EXR file

Built on 2026-10-08 under your effects loop request. This is P0-5, "Depth and extra channels",
decided as D-348.

A 3D program can save more than colour in an EXR file: how far away each pixel is (the **depth**,
often called Z) and which way each surface faces (the **normals**). The app now reads those from
the file a layer is drawn from, and two new effects use them. Both are in a new **3D Channel**
group:

- **Pass Extract**, our own effect after After Effects' 3D Channel Extract: it shows the depth or
  the normals as a picture. **Black Point** is the value drawn black, **White Point** the value
  drawn white. **Invert** turns it over; **Clamp** keeps the result between black and white.
- **Depth Key**, our own effect after Depth Matte: it takes out what is nearer than **Depth**.
  **Feather** softens the edge, in the same units as the depth. **Invert** keeps the near part
  instead.

Both are drawn by the graphics card in the viewer; exports use the processor. The depth and
normals are read from the file again whenever needed, so they are never written into your project.

The check, `verification/D-348_depth_channels_table.md`, draws all 36 cases of FX-DEPTH-001 to 036
against numbers worked out by a separate program (using OpenEXR's own library) before the effects
existed, and holds the card to the processor within 1 level on every case and on the sample. The
pictures are in `verification/D-348 pictures/`, made from the sample file: `before.png`,
`depth.png`, `normals.png`, `key_near_out.png` and `key_far_out.png`. Clear parts show as white in
most picture viewers.

## What to check

Import `Fixtures/depth_channel/sample/spheres.exr` (three balls on a chequered floor, 320 by 180,
the sky clear; its depth runs from about 2 at the bottom edge to hundreds at the horizon, and the sky
is 1000). Put it in a composition its size.

1. **Pass Extract as added.** Add **Pass Extract** (search "depth" or "3d channel"). Everything
   past depth 1 is white, so the picture is nearly all white. Set **Black Point** 2 and **White
   Point** 16: it should look like `depth.png`, near things dark, far things light, the sky white.
2. **Normals.** Set **3D Channel** to **Surface Normals**, Black Point -1, White Point 1, Clamp
   on: it should look like `normals.png`, the balls rainbow-shaded, the floor one pale green, the
   sky mid grey.
3. **Invert and Clamp.** Back on Z-Depth (2 to 16), turn **Invert** on: near light, far dark.
4. **Depth Key.** Remove Pass Extract and add **Depth Key**. Set **Depth** 8 and **Feather** 3:
   it should look like `key_near_out.png`, the red ball and the near floor gone, the edge soft.
   Turn **Invert** on: it should look like `key_far_out.png`, only the near part left.
5. **Not an EXR.** Add Pass Extract to a solid or a PNG layer: the layer is drawn unchanged, and
   the warnings say the layer has no depth to read. Its settings stay as you set them.
6. **Save and open.** Save, close and reopen: the settings should be as you left them, and the
   picture the same.
7. **Same as an export.** Export a frame and put it next to the viewer. They should look the same.

## Not done, on purpose or for later

- After Effects reads depth from its own 3D layers in a nested composition. Here only an EXR
  file's own depth and normals are read; a solid, text, shape, composition or adjustment layer
  has none.
- Only the depth and normals are read. Object ID, material ID, UV, coverage and other passes, the
  RLA and RPF file kinds, Cryptomatte, ID Matte, EXtractoR, IDentifier, Fog 3D and Depth of Field
  are separate rows, not built.
- After Effects' Depth Matte has an **Antialias** setting and the Info panel shows the depth under
  the pointer; neither is here.
- An EXR with no plain colour channels (some Blender files keep everything under named layers) is
  still refused when imported, as before.
- Pass Extract starts at Black 0 and White 1, not After Effects' 5000 and 0, which are meant for its
  own 3D layers, not a file's units.

## If something is wrong

Say which step. The most likely fault would be at the edges of the picture after a blur or a
transform before the effect (the pass must stay where the picture was, not move with the blur).
