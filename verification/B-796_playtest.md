# B-796 playtest: choosing the colour an export is written in (D-917)

What changed: with colour management on, the export choices under Composition gain one more list, **Output**. It says what colour the file is written in. With colour management off, the list is not there and every export is written exactly as before.

You need: a project with colour management on (Project settings), and one with it off. Any short composition will do.

## 1. The list appears only with colour management on

1. Open a project with colour management **off**. Open the Composition menu and look at the export choices under "Export...".
   - Expected: no Output list. The choices are exactly as before.
2. Turn colour management **on** (or open a project that has it on). Look again.
   - Expected: a list reading "Output: sRGB" sits beside the format.

## 2. The list follows the format

1. With the format on PNG, open the Output list.
   - Expected: three choices: Output: sRGB, Output: Rec.709 (BT.1886), Output: ACES 2.0 SDR view.
2. Change the format to "EXR, full float".
   - Expected: the list now reads Output: working space, Output: ACEScg, Output: ACES2065-1, and it is back on "working space".
3. Change back to PNG.
   - Expected: it is back on "Output: sRGB".

## 3. The ACES view writes what the viewer shows

1. In a colour-managed project with something bright (a glow, or a solid well over white), set View > Colour view to "ACES 2.0 SDR".
2. Export a PNG with "Output: ACES 2.0 SDR view".
3. Open the PNG in any picture viewer and compare it with the viewer.
   - Expected: the PNG looks like the viewer: bright parts roll off softly rather than clipping to flat white.
4. Export again with "Output: sRGB" and compare.
   - Expected: the sRGB file clips the bright parts harder, as exports always have.

The pictures in `verification/B-796 pictures/` show the same thing on a test chart (four rows: a white ramp, a fire ramp, colour patches from dark to very bright, an orange fading in):
- `acescg_before_D-917.png`: what an ACEScg project wrote before this change. The fire turns yellow and the ramps clip early.
- `acescg_default.png`: Output sRGB now. The colours are converted properly.
- `acescg_rec709.png`: Output Rec.709 (BT.1886). Encoded for a video screen (gamma 2.4), so in a plain picture viewer it looks a little lighter than the sRGB file.
- `acescg_aces-view.png`: the ACES 2.0 SDR view. The brights roll off and nothing clips flat.

## 4. The status line and the Render queue say the choice

1. Choose "Output: Rec.709 (BT.1886)", then click "Add to Render queue...".
   - Expected: the queue row ends with "in Rec.709 (BT.1886)".
2. Export with the same choice.
   - Expected: the status line includes "in Rec.709 (BT.1886)".
3. With "Output: sRGB", neither line mentions an Output.

## 5. The files say what they hold

This needs a tool that shows file details. Skip it if you have none; the test `tests/b796_export_output.rs` reads every file back and checks it (table: `verification/B-796_export_output_table.md`, 75 of 75 checks pass).

- An MP4 with Output sRGB or the ACES view: a player that shows colour details (for example, VLC's Codec information, or MediaInfo) reports the transfer as sRGB. With Rec.709 it reports BT.709.
- An EXR with ACEScg: a compositing program that reads EXR chromaticities (Nuke, Resolve) treats it as ACEScg.

## Judge

- Pass: the list appears only with colour management on, its words follow the format, the ACES view export matches the viewer, and the queue and status lines name the choice.
- Fail: anything an export with colour management off writes looks different from before, or a file looks different from what the viewer showed in the matching view.
