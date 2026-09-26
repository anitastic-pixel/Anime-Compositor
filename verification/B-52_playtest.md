# B-52: Repeat Edge Pixels, by hand

Built on 2026-09-26 against the B-52 entry in document 15. **Playtest passed on 2026-09-26:** the owner wrote "repeat edge pixel does work".

`verification/B-52_edges.md` explains what was built, with before-and-after pictures. This sheet covers what those cannot show: the switch in the app.

## Before you start

- Use the release build. It opens on the reference shot.
- Select the background layer. It is the picture that fills the frame.
- The composition's background shows as a checkerboard or a flat colour. Either way, a see-through edge shows up against it.

## What to check

1. **The problem, as before.**
   - Add a **Radial Blur** to the background layer. Set **Type** to **Zoom** and **Amount** to about 20. (Since B-53 a zoom about a centre inside the picture no longer fades at its edges. Use **Spin**, as in step 3, to see the problem.)
   - Look at the edges of the frame: they fade to see-through.
   - The new **Edges** setting should read **Transparent**.
2. **The fix.** Switch **Edges** to **Repeat Edge Pixels**. The edges should go solid right to the frame's border, and the middle of the picture should not change.
3. **Spin.** Set **Type** to **Spin** and **Amount** to about 30. The corners should stay solid with Repeat Edge Pixels, and fade with Transparent.
4. **Blur.** Remove the Radial Blur and add **Blur** (Gaussian) with a large amount, about 20. Switch **Edges** between the two choices. With Repeat Edge Pixels the border stays solid. With Transparent it goes soft and see-through, as before.
5. **Directional Blur.** Do the same with a **Directional Blur**, **Length** about 60, at any angle. The streaks should run off the edge solid, with no see-through band.
6. **Undo.** Throw the switch, then press Ctrl+Z. It should go back.
7. **Save and reopen.** Save with Repeat Edge Pixels on, close the project and reopen it. The switch should still read Repeat Edge Pixels.
8. **CPU and GPU.** Switch **Draw on** between **CPU** and **GPU** with Repeat Edge Pixels on. The picture should not change in any way you can see.
9. **Export.** Export a few frames with Repeat Edge Pixels on. The edges should be solid in the file, as in the viewer.

## What to report

- Any edge that still fades with Repeat Edge Pixels on, with the blur, its settings and the frame number.
- Any difference between CPU and GPU.
- Anything that moved in the middle of the picture when you threw the switch.
- A project that opened differently from before.
