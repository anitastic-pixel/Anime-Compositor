# B-229: object and material ids, named channels and Blender's EXR files

Built on 2026-10-08 under your effects loop request. This is the second part of P0-5, "Depth and
extra channels", decided as D-349. B-228 read the depth and normals; this reads the rest.

A 3D program can number each object (the **object id**) and each material (the **material id**)
and save those numbers in the EXR file beside the colour, along with other passes such as Blender's
mist. The app now reads them:

- **ID Key**, our own effect after After Effects' ID Matte, in the **3D Channel** group: it keeps
  only the pixels whose id is **ID Selection**. **Aux. Channel** picks object or material ids.
  **Feather** softens the edge, in pixels. **Invert** cuts that object out instead.
- **Pass Extract** gains three choices in its **3D Channel** list: **Object ID**, **Material ID**
  and **Named Channel**. Named Channel shows a **Channel Names** box: type one channel name exactly
  as the file has it (for example `ViewLayer.Mist.Z`), or three with commas between for red, green
  and blue. This is what After Effects' EXtractoR does.
- **Blender's files open.** Blender saves its picture under a layer name (`ViewLayer.Combined.R`
  and so on) instead of plain R, G, B. Those files used to be refused on import; now the colour is
  taken from `Composite.Combined` if the file has it, else from the first `<name>.Combined` layer.
- **The import note is honest.** The "channels ignored" note no longer lists the depth, normals or
  ids, because effects can now read them. Channels nothing reads (for example a mist, unless a Pass
  Extract names it) are still listed.

ID Key and Pass Extract are drawn by the graphics card in the viewer; exports use the processor.
The ids are read from the file whenever needed, never written into your project.

The check, `verification/D-349_id_key_table.md`, draws the 28 cases FX-DEPTH-037 to 064 against
numbers worked out by a separate program (using OpenEXR's own library) before the code existed, and
holds the card to the processor within 1 level on every case and on the sample. The pictures are in
`verification/D-349 pictures/`. Clear parts show as white in most picture viewers.

## What to check

Import `Fixtures/depth_channel/sample/spheres_blender.exr`. It is the same three balls on a floor
as B-228's sample, saved the way Blender saves a multilayer file. Put it in a composition its size
(320 by 180).

1. **It opens.** Before this, the file was refused. It should look like `before.png`. The import
   note should list only `ViewLayer.Mist.Z` as ignored.
2. **Object ids as a picture.** Add **Pass Extract**, set **3D Channel** to **Object ID**, Black
   Point 0, White Point 4. It should look like `object_ids.png`: the sky black, the three balls
   three greys, the floor white.
3. **Material ids.** Switch to **Material ID**: it should look like `material_ids.png`. The red and
   blue balls share a material, so they are the same grey.
4. **A named channel.** Switch to **Named Channel**, Black 0, White 1, and type `ViewLayer.Mist.Z`
   in **Channel Names**, then press Enter. It should look like `mist.png`: near dark, far light.
   Type `Mist.Z` instead: the layer goes back to its colour and a warning says the file has no
   channel by that name (the name must be whole and exact).
5. **ID Key.** Remove Pass Extract and add **ID Key**. Set **ID Selection** to 2: only the middle
   ball is left (`key_object_2.png`). Try 1, 3 and 4: one ball each, then the floor.
6. **Feather and Invert.** At ID 2 set **Feather** 3 and turn **Invert** on: that ball is cut out
   with a soft edge (`key_object_2_inverted_soft.png`).
7. **Material.** Invert off, Feather 0, **Aux. Channel** Material ID, ID 3: the red and blue balls
   together (`key_material_3.png`).
8. **Not an EXR, or no ids.** Add ID Key to a PNG layer, or to B-228's `spheres.exr` (which has no
   ids): the layer is drawn unchanged and the warnings say there are no ids to read.
9. **Save and open.** Save, close and reopen: the settings, the typed channel names included,
   should be as you left them.
10. **Same as an export.** Export a frame and put it next to the viewer. They should look the same.

## Not done, on purpose or for later

- **Cryptomatte** is not read. Its ids are scrambled into a special encoding and need their own
  decoder; that is a separate row. IDentifier is also its own row, not built here.
- After Effects' ID Matte has **Use Coverage** (for RLA/RPF files only) and lets you click on the
  picture to pick an id. Neither is here: you type the id.
- EXtractoR can pick an alpha channel and un-multiply. Pass Extract's named channel shows one or
  three channels as an opaque picture only.
- A Blender file with several view layers always shows the Composite, else the first layer. There is
  no setting to choose another one, and multiview (stereo) files are not read.
- ID Key's Feather is a Gaussian blur of the cut-out, in pixels. After Effects does not document how
  its feather works, so this one is ours.

## If something is wrong

Say which step. The most likely fault would be in step 1 (a Blender file whose layer names differ
from these) or at the edge of a feathered cut-out next to the picture's border.
