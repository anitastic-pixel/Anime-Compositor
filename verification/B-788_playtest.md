# B-788 playtest: "Interpret as" per file (D-909, colour management step C3)

What changed: every picture file in the project now has an "Interpret as" choice in the Project
panel. It says what colours the file's numbers are in (ACES2065-1, ACEScg, linear Rec.2020, a
camera-style log such as ACEScct, and so on). The app converts the file into its own colours as
it reads it. A file already in the ordinary choice (sRGB for PNGs, linear sRGB for EXRs) looks
exactly as before.

The project file can also say "colour management on, working in ACEScg". That switch is **not
in the window yet**, on purpose: with it on today the app would mix the working numbers into
the old sRGB view and into exports (steps C4 to C6 are what make that right). So only test
files carry it, and the B-788 table checks them.

The test files are in `Fixtures/colour/interpret/` in the project folder.

## Steps

1. Open `Fixtures/colour/interpret/chart_tagged_aces2065_1.json`.
   - A colour chart fills the frame: the 24 squares of a ColorChecker (four rows, the fourth
     white to black) plus a fifth row of test squares (a red, a green, a blue, two squares
     brighter than white that show as plain white, and a bright warm yellow).
   - The first four rows should look like a normal, well-exposed ColorChecker (skin tones, a
     sky blue, a foliage green). Compare with `verification/B-788 pictures/chart_aces2065_1.png`.
2. Click the chart in the Project panel. Under its details there is a box "Interpret as",
   showing "ACES2065-1".
3. Change it to "Linear Rec.709".
   - The chart turns washed out and greyish, and the colours shift (the orange goes
     mustard, the cyan goes slate). The white-to-black row does not change. That is right: the same
     numbers are now read as if they were plain linear sRGB. Compare with
     `chart_linear_rec709.png`.
   - The project shows as changed (unsaved).
4. Press Ctrl+Z. The box goes back to "ACES2065-1" and the chart looks as in step 1.
5. Change it to "ACEScg". The chart is in between: more saturated than step 3, slightly off
   from step 1. Compare with `chart_acescg.png`.
6. Open `Fixtures/colour/interpret/unknown_space.json`.
   - The notes list has a warning `COLOR_SPACE_UNKNOWN` naming the file and the word
     "arri-logc4-awg4" (a camera colour this version does not have).
   - The picture is not drawn (the frame is empty there), and "Interpret as" shows
     "arri-logc4-awg4 (not known to this build)".
   - Save it under a new name and open the saved file in a text editor: the word
     "arri-logc4-awg4" is still there. Nothing was silently changed.
7. Open any project you already have. It looks exactly as before. Each picture's
   "Interpret as" shows what it always was (sRGB for PNGs, linear sRGB for EXRs).

## What counts as wrong

- Step 1 not looking like a normal ColorChecker.
- No "Interpret as" box, or changing it not changing the picture.
- Ctrl+Z not putting it back.
- Step 6 with no warning, or the word gone after saving.
- Any old project looking different.

## Not in this step (the gaps)

1. No "colour management" switch in the window (C4 and C5 first, see above).
2. With management on, stored colours (solid colours, text colours) are not yet converted, and
   8-bit work is not refused (C4).
3. The viewer still shows the working numbers through the old sRGB curve (C5).
4. Exports still write the working numbers labelled sRGB (C6).
5. Thumbnails and the import preview ignore "Interpret as".
6. An EXR's own chromaticities and a PNG's colour chunks are still only reported, not used.
7. No camera log formats beyond the ACES ones (C8).

The numbers behind each picture, checked against OpenColorIO, are in
`verification/B-788_interpret_as_table.md` (47 of 47 checks pass).
