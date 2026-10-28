# B-833 playtest: F4, Ctrl+Shift+Y, angles in turns, and Wiggle position (D-953)

What changed: four small After Effects habits from the tutorials.

## Steps

1. Open the reference shot. Press **F4**. The two mode columns beside the layer names go, leaving the switches. Press F4 again: the switches go and the modes come back. Press it a third time: both are shown. Close and reopen the app: the columns are as you left them.
2. Add a solid (Ctrl+Y) and keep it selected. Press **Ctrl+Shift+Y**. The solid dialog opens with the solid's name, size and colour already in it. Change the colour and the name, press OK: the solid changes. Press Ctrl+Z once: it is back as it was.
3. Select a drawn layer (not a solid) and press Ctrl+Shift+Y: nothing opens and the line at the bottom says to select a solid.
4. Set a layer's Rotation to 390. It reads **1x+30°**. Type `-1x-30` in it: it reads -1x-30° and the layer turns back the other way. Type a plain `45`: it reads 0x+45°.
5. Select a drawn layer and choose **Animation > Wiggle position** (or type it in the command finder). Two Slider Controls appear in its effects, 2 and 30, and Position gets an expression. Play: the layer shakes about, never more than 30 pixels from where it was. Set the second slider to 0: it stops. Ctrl+Z once after applying takes it all off.

## What you should not see

- A Rotation that reads 390° rather than 1x+30°.
- Ctrl+Shift+Y making a new solid instead of changing the selected one.
- Two Ctrl+Z needed to take off a Wiggle position applied once.

The computer's checks are in `verification/B-833_small_shortcuts_table.md`; the check in a private copy of the app is `verification/B-833_private_app_table.md`.
