# B-782 playtest: a PNG's own colour information is named (D-902)

What changed: a PNG file can carry its own colour information (a gamma, primaries, an ICC
profile, an HDR code). The app does not use it yet; it reads every PNG as ordinary sRGB. Until
now it said nothing about that. Now it says so in the notes list. The picture itself is exactly
as before.

The test files are in `Fixtures/colour/metadata/` in the project folder.

## Steps

1. Open the app and any project (or a new one).
2. Import `Fixtures/colour/metadata/gama_linear.png` on its own (one file chosen).
   - It is added to the project as a still picture, a tiny 2 by 2 square.
   - The notes list gains one warning, `MEDIA_COLOR_METADATA_IGNORED`:
     "gama_linear.png carries colour information this build does not read." with the detail
     "Found: gAMA 100000. The picture is drawn as sRGB, as every PNG is (document 21), with its
     numbers unchanged."
3. Import `Fixtures/colour/metadata/srgb_gama_chrm.png` on its own.
   - It is added, and **no** new colour warning appears: its chunks say plain sRGB, which is
     what the app already assumes.
4. Import `Fixtures/colour/metadata/iccp.png`.
   - One warning naming "iCCP 114 bytes" (an ICC profile the app does not read).
5. Import the three files in `Fixtures/colour/metadata/seq/` together (choose all three).
   - They come in as one sequence, with **one** warning, not three:
     "3 drawings in frame_%04d.png carry colour information this build does not read.",
     ending "Drawings: 1-3."

## What counts as wrong

- No warning in step 2, 4 or 5.
- A warning in step 3.
- Three separate warnings in step 5.
- Any of the pictures looking different from before, or failing to import.

The full list of files and what each must say is in
`verification/B-782_colour_metadata_table.md` (24 of 24 checks pass).
