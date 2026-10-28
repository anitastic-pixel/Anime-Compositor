# B-829 playtest: an eyedropper beside every colour field (D-949)

What changed: every colour swatch in the panels has a small dropper button right after it. Press it, then click the picture: the swatch takes the colour you clicked. It works with colour management on or off.

## Steps

1. Open a project with colour management off (File > Project Settings shows Off) and a picture with clear colours.
2. Choose a layer and add **Fill** (or Glow) from the Effects panel. Its Color swatch has a small dropper button after it.
3. Press the dropper. The cursor over the picture becomes a cross.
4. Click a red area of the picture. The swatch turns that red and the layer is filled with it.
5. Press **Ctrl+Z**: the colour goes back.
6. Press the dropper again, then **Escape**: the cross goes and nothing changes.
7. Do the same on a shape layer's Fill colour and on a solid's Colour in the layer's properties.
8. Turn colour management on and pick again: it still works, and the Info row reads as before.

## What you should not see

- A dropper inside the Solid settings or Composition settings windows (the picture cannot be clicked while they are open).
- Two droppers after one swatch.
- A click on the picture while picking selecting or moving a layer.

The computer's checks are in `verification/B-829_eyedropper_table.md`; the check in a private copy of the app is `verification/B-829_private_app_table.md`.
