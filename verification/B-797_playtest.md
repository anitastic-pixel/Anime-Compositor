# B-797 playtest: the colour numbers with colour management on (D-918, colour management step C7)

What changed: when a project has colour management on, an **Info** row appears under the viewer.
It shows two lines of numbers, the way After Effects' Info panel does:

- **The point under the pointer.** Its working numbers (in ACEScg, or Linear Rec.709) and the
  three 0 to 255 levels the screen shows for it in the current Colour view.
- **The colour picker you touched last.** The same two sets of numbers for that picker's colour.

The row also has a **Pick from the picture** button, an eyedropper. Press it, then click the
picture, and the colour picker you touched last takes the colour of that pixel.

None of this changes what is saved. A colour you pick or type is stored exactly as before.
**A project without colour management looks exactly as before**: no Info row, no eyedropper.

The colour-management switch itself may not be in the window yet (step C4 runs in parallel).
Until it is, use the test files in `Fixtures/colour/view/`, which already have it on.

## Steps

1. Open `Fixtures/colour/view/hdr_scene_aces.json` (eight coloured bands, dark on the left,
   very bright on the right).
   - Under the viewer there is a row starting with **Info** and a **Pick from the picture**
     button, as in `verification/B-797 pictures/window_info_row.jpg`.
2. Move the pointer slowly over the picture.
   - After a moment the first line shows the point (for example `200, 80`), then
     `Working (ACEScg)` and three numbers, then `Display (ACES 2.0 SDR)` and three levels.
   - Moving to the right along a band, the working numbers grow far past 1. The display levels
     stop at 255 or just under it.
   - Off the picture's edge, the line says "Outside the picture."
3. Open the New Solid dialog and click its colour box once, then close the colour chooser.
   - The second line starts with `Picker #808080` and shows its working numbers and display
     levels.
   - Choose orange, `#ff8000`, in that colour box. The line becomes:
     `Working (ACEScg) 0.6864 0.2680 0.0443   Display (ACES 2.0 SDR) 196 103 25`.
     The display levels may be one off in any channel.
4. With the New Solid colour box touched last, press **Pick from the picture**. The pointer
   becomes a cross. Click the middle of the orange band.
   - The colour box takes that orange. The status line says "Picked #... from the picture at x, y."
   - Nothing on the picture is selected or moved by that click.
   - Pressing Escape instead of clicking cancels the pick.
5. Change View > Colour view to **Un-tone-mapped** and move over the picture again. The display
   levels are now the clipped ones: towards the bright end of a band they reach 255 in that
   band's own colours sooner than with ACES 2.0 SDR. The working numbers do not change.
6. Open `Fixtures/colour/view/hdr_scene_off.json` (the same picture, colour management off).
   - There is no Info row and no Pick from the picture button.
   - Everything looks and behaves exactly as before this change.

## Known limits (D-918)

- The numbers come from the full-quality picture, even when the viewer shows Half or Draft.
- Solo, the onion skin and a turned or mirrored view are not reflected. With a turned or mirrored
  view, the pointer line stays still and the eyedropper says to press Reset first.
- The picker line uses the picker's own 8-bit colour, as the picker stores it.
- A picked colour is the nearest of the picker's 256 levels per channel to the pixel.
- The three pure primaries (`#ff0000`, `#00ff00`, `#0000ff`) through ACES 2.0 SDR can be up to
  2 levels off OpenColorIO. This is the same limit as the viewer itself (D-915).
- The pointer line updates a moment after the pointer stops, not on every movement.

Evidence: `verification/B-797_colour_info_table.md` checks every number against OpenColorIO
2.5.1 (`Fixtures/colour/c7/expected_readout.json`).
