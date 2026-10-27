# B-794 playtest: the viewer's colour view (D-915, colour management step C5)

What changed: when a project has colour management on, the View menu over the viewer has a
"Colour view" choice:

- **ACES 2.0 SDR** (the default). Bright parts roll off gently into white, the way a film camera
  would show them, instead of clipping hard.
- **Un-tone-mapped**. The working colours, clipped at white with no roll-off.
- **Standard**. Exactly today's viewer.

This changes only what the viewer shows. **Exports never take the colour view.** A project
without colour management looks exactly as before, and the choice is not in its menu.

The colour-management switch itself is **not in the window yet**. It is part of step C4, which
is not on main yet. Until it is, only project files that already have colour management on show
the choice. The test files in `Fixtures/colour/view/` in the project folder have it on.

## Steps

1. Open `Fixtures/colour/view/hdr_scene_aces.json`.
   - The picture shows eight horizontal bands: grey, orange fire, red, green, blue, cyan, magenta
     and yellow. Each band runs from black on the left to 64 times brighter than white on the
     right.
   - Each band fades smoothly towards white on the right. Most of the colour stays visible
     before it turns white. Compare with the middle panel of
     `verification/B-794 pictures/sheet_standard_vs_aces.png`.
2. Open the View menu over the viewer. Below "Alpha only" there is a "Colour view" box that
   shows "ACES 2.0 SDR" (as in `verification/B-794 pictures/window_view_menu.png`).
3. Change it to "Standard".
   - Each band now turns to flat white about halfway across, and the bright colours go pastel or
     white early. This is today's viewer (top panel of the sheet).
   - The project shows as changed (unsaved).
4. Press Ctrl+Z. The box goes back to "ACES 2.0 SDR" and the picture looks as in step 1.
5. Change it to "Un-tone-mapped". The red, green, blue, cyan, magenta and yellow bands end in
   flat pure colour, not white (bottom panel of the sheet).
6. Export the frame (any format) with the view set to "ACES 2.0 SDR", then again with it set to
   "Standard". The two exported files are identical. Exports never take the view.
7. Open `Fixtures/colour/view/hdr_scene_unknown_view.json`.
   - The notes list has a warning `COLOR_VIEW_UNAVAILABLE` naming "aces-1.3-sdr", a view this
     version does not have.
   - The picture is shown as Standard. The box reads "aces-1.3-sdr (not known to this build,
     Standard shown)".
   - Save the file under a new name and open the saved file in a text editor: "aces-1.3-sdr" is
     still there.
8. Open `Fixtures/colour/view/hdr_scene_off.json`, or any project you already have. The View menu
   has no "Colour view" box, and the picture looks exactly as before.

## What counts as wrong

- Step 1 looking like the top panel of the sheet (hard clipping to white).
- No "Colour view" box in step 2, or a box in step 8.
- Changing the view not changing the picture, or Ctrl+Z not putting it back.
- The two exports in step 6 differing.
- Step 7 with no warning, or the word gone after saving.
- Any old project looking different.

## Not in this step (the gaps)

1. No colour-management switch in the window (step C4). Only files that already have it on show
   the colour view.
2. A composition set to "8 bpc (After Effects)" with a colour view showing is drawn by the
   processor, not the graphics card. It is correct but slower. The notes say "The CPU drew this
   frame".
3. The ACES table matches OpenColorIO within 1 level on every case that has a bar. Very saturated
   pure primaries pushed far past white (the red, blue, magenta and yellow bands' far right ends)
   are up to 4 levels off on some pixels (6 on the stress case). The table reports these but
   does not bar them.
4. The trace pictures and thumbnails stay Standard.
5. Exports are still written as the working numbers labelled sRGB (step C6).

Before this sheet was written, a private copy of the release app (its own folder and browser
profile, not your app) opened `hdr_scene_aces.json`. Its menu was switched through the three
views, and the frame the window sent was compared with the pictures in
`verification/B-794 pictures/`:

| Drawn on | ACES 2.0 SDR | Standard | Un-tone-mapped |
|---|---|---|---|
| Processor | identical | identical | identical |
| Graphics card | identical | within 1 level (as today) | identical |

The numbers are in `verification/B-794_colour_view_table.md` (62 of 62 checks pass). It has
three parts: the view against OpenColorIO, the card against the processor (including colours
brighter than white), and the built-in table against OpenColorIO's own.
