# B-21b: the new drawing formats in the window, by hand

Built on 2026-09-19 against D-72, which the owner accepted the same day.

The generated half is two tables. `verification/B-21b_formats_table.md`, 11 of 11, reads every
fixture file and compares it, value by value, with its PNG twin. `verification/B-21b_panel_table.md`,
13 of 13, calls what the window calls with the files already chosen. This sheet covers what
neither can judge: the file picker, and what you see.

## Before you start

Open any project you are happy to change without saving. The test files are in
`Fixtures/formats/media`. They are tiny, 16 by 12 pixels, so zoom the viewer in. Nothing below
changes them.

## What to check

1. **The picker shows the new files.** Click **Import drawings...** and go to
   `Fixtures/formats/media`. Files ending `.bmp`, `.tga`, `.tif`, `.webp` and `.jpg` are listed
   beside the `.png` ones.
2. **A TGA with alpha.** Choose `tga32.tga` alone. The status line says "Import tga32.tga: a
   still picture, shown on every frame of a layer made from it." Make a layer from it. Then
   import `tga32.twin.png` and make a layer from that. The two pictures look the same, and
   both are see-through in the same places (turn the checkerboard on to see it).
3. **The others.** Do the same with `bmp24.bmp`, `tiff_lzw.tif`, `webp_lossless.webp` and
   `jpeg_q95.jpg`, each against its `.twin.png`. Each pair looks the same. The JPEG has no
   see-through part.
4. **The grey JPEG.** `jpeg_grey.jpg` is grey, not tinted.
5. **Relink.** Choose a PNG drawing under Drawings, **Relink...**, and choose `tga32.tga`. The
   box says the colour reading does not change. **Apply the relink**: the layer shows the TGA.
   Ctrl+Z brings the PNG back.
6. **Your own file.** Import a TGA, TIFF, JPEG or WebP of your own, or a numbered sequence of
   them chosen together. It comes in as a PNG would.
7. **A file that is not a picture.** Rename any text file to end in `.tga` and import it, then
   make a layer from it. The notes name the file and say it could not be read; the program does
   not close.

## One thing decided differently from D-72's sentence, for the owner to settle

D-72 says a 16-bit TIFF "is read down to 8, and said so". Saying so needs a new diagnostic in
document 28, which D-72 did not write. Until the owner decides, a TIFF deeper than 8 bits is
**refused** with the existing MEDIA_UNSUPPORTED_FORMAT, exactly as a 16-bit PNG already is, and
the message says to re-export at 8 bits. Nothing is changed silently either way.

Result: the owner, 2026-09-19: "B-21b playtest works, keep the refusal".
