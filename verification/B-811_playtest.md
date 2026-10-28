# B-811 playtest: Set Matte

What changed: a new effect, **Set Matte** (Effect > Lines & Mattes), after After Effects' Set Matte.
It replaces a layer's see-through-ness with one channel of another layer (or of itself, before its
effects): its alpha, red, green or blue, luminance, hue, lightness or saturation. This is what the
tutorial MN518F3vOvE does at 6:45 to cut the grown shadow back to the drawing's own outline.

1. Open a project with two layers, a photo and a drawing on a clear background. Select the photo
   and add Effect > Lines & Mattes > Set Matte.
   - Expect: Take Matte From Layer **None**, Use For Matte **Alpha Channel**, Invert Matte
     **Off**, Stretch Matte To Fit **On**, Composite Matte With Original **On**, Premultiply
     Matte Layer **On**, and the photo unchanged.
2. Set Take Matte From Layer to the drawing.
   - Expect: the photo shows only where the drawing is solid, as if cut out by it.
3. Turn Invert Matte **On**.
   - Expect: the opposite: the photo shows only where the drawing is clear.
4. Invert Off, Use For Matte **Luminance**.
   - Expect: the photo shows through the drawing's bright parts, fades in its dark ones.
5. Use a drawing smaller than the photo; switch Stretch Matte To Fit **Off**, then **On**.
   - Expect: off, the drawing's matte sits in the middle at its own size; on, it is stretched to
     cover the whole photo.
6. Hide the drawing's layer (its eye).
   - Expect: the photo is still cut out by it: a hidden layer can still be the matte.
7. Delete the drawing's layer.
   - Expect: a warning that the matte layer is missing, and the photo drawn as if Set Matte were
     off.
8. Save, close and reopen the project.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b811_set_matte` (96 of 96, `B-811_set_matte_table.md`): every
pixel of FX-SETMATTE-001 to 024 against `tools/set_matte_reference.py`, the two layers that read
each other refused on opening, the wrong-word warnings, round trips and tiles.

The sheet `verification/B-811 pictures/set_matte_vs_tutorial.png` puts the tutorial (MN518F3vOvE about 6:45: Color Keep, F's Max 3, Set Matte from the figure's Alpha) above the release app drawing the same three steps on a small stand-in figure. Pass if ours shows the magenta kept, its ink-line gaps filled by Max, then cut to the figure and laid over it, as the tutorial's shadow is.
