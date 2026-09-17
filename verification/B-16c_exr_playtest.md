# B-16c: EXR in the window, by hand

Built on 2026-09-17 against D-62, which the owner accepted the same day ("accept d-62").

The generated half is `verification/B-16c_panel_table.md`, 18 of 18. It calls what the window
calls and checks what comes back, starting from files already chosen, because no test can answer
a Windows file picker. Whether the pixels are right is B-16b's table, checked against OpenEXR
itself. This sheet covers what neither can judge: the pickers, the list beside Export, what you
see on screen, and whether the exported files open in another program.

## Before you start

Open any project you are happy to change without saving, or use **Save As...** first and work on
the copy. The EXR test files are in `Fixtures/exr`. Nothing below changes them.

## What to check

1. **The picker shows EXR files.** Click **Import drawings...**. The file type reads "PNG and EXR
   drawings". Go to `Fixtures/exr/compression`: the `.exr` files are listed.
2. **One EXR file.** Choose `zip_half.exr` alone. The status line says "Import zip_half.exr: a
   still picture, shown on every frame of a layer made from it." It appears under Drawings.
   Make a layer from it with **Add layer**: a small coloured picture shows in the viewer, the
   same on every frame. Nothing new appears in the notes.
3. **A file with odd values.** Import `Fixtures/exr/values/specials.exr`. It is added, and the
   notes gain one line saying some values were changed to draw it. Under **Error details** it
   reads MEDIA_EXR_ADJUSTED with "non_finite: 4 alpha_clamped: 2".
4. **A file this build refuses.** Import `Fixtures/exr/refused/multipart.exr`. The status line
   starts "Nothing was imported." and says to re-export it as a single-part RGBA EXR. Nothing is
   added under Drawings.
5. **An EXR sequence.** Import all four files in `Fixtures/exr/sequence` together. The status
   line says "Import render_%04d.exr: 4 drawings, numbered 1 to 5, and drawing 4 is missing."
   A layer made from it shows a picture on its frames and nothing on the missing one, the same
   as a PNG sequence with a gap.
6. **Relink says the colour reading changes.** Choose a PNG drawing set under Drawings and click
   **Relink...**. Choose the four files in `Fixtures/exr/sequence`. Before anything changes, the
   box says "The colour will be read as linear light with premultiplied alpha, where it is read
   as sRGB with straight alpha now". Click **Leave it as it is**: nothing changed. Do it again
   and click **Apply the relink**: the layer now shows the EXR pictures. Press Ctrl+Z: the PNG
   drawings are back.
7. **Relinking EXR to EXR.** Choose the sequence you imported in step 5, **Relink...**, and
   choose the same four files. The box says the colour "is read as linear light with
   premultiplied alpha, as it is now". Click **Leave it as it is**.
8. **The format list.** Beside **Export...** is a list: "PNG, 8-bit", "EXR, half float",
   "EXR, full float". It starts on PNG, and exporting on PNG works as before.
9. **EXR half.** Choose "EXR, half float", click **Export...** and pick an empty folder. The
   green line says "Exporting N frames as EXR, half float, into ...". The result appears in the
   line below it, starting "Exported N frames into ...", and the green line then says "The
   export has ended." The list still reads "EXR, half float" afterwards. The folder now holds files ending in
   `.exr`, numbered like the PNG ones. If the project has a missing drawing (the B-15c package
   does, on purpose), the result instead starts "Nothing was exported" and nothing is written,
   as for PNG: tick **Write frames whose drawing is missing** first, or use a project whose
   drawings are all present.
10. **EXR full float.** The same with "EXR, full float", into another empty folder. The status
    line says "as EXR, full float,". The files are larger than the half ones.
11. **They open elsewhere.** Open one exported file in a program that reads EXR (for example
    Blender, Krita, GIMP or DaVinci Resolve). It shows the frame. It may look darker or brighter
    than in this window if that program shows linear light without converting it; the shapes and
    edges are what to judge.
12. **They come back in.** Import the exported files from step 9 into this window as a sequence
    and make a layer from it. A new layer from any sequence, PNG too, starts with no exposures
    and so shows nothing (document 20). With the layer chosen, press **Add an exposure** a few
    times: each press shows the next drawing on the next frame, and they look the same as the
    frames you exported.
13. **Dropping drawings.** Select several of the exported `.exr` files in File Explorer and drag
    them onto the window. They are imported as a sequence, with the same status line as step 12.
    One file dropped on its own is a still. A project `.json` dropped on the window still opens
    it, as before.

## Known limits

- Audio (WAV) is deferred by the owner on 2026-09-17 ("we can defer audio for much later").
- EXR has no bit-depth or straight-alpha choice: it is written as the window holds it, linear
  light with premultiplied alpha (D-62). The PNG choice is still 8-bit only.
- The format list is kept until the window closes; a new window starts on PNG. The "Write frames
  whose drawing is missing" tick is not kept, on purpose (document 07).
- Files with several parts or layers, deep files and the other files in `Fixtures/exr/refused`
  are refused with a reason rather than drawn (D-62).

## What to answer

"works", or which step number did something else and what it did.
