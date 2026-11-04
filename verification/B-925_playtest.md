# B-925 playtest: Iris Wipe's Iris Points, Outer Radius, Inner Radius and Rotation

What changed (D-1045): Iris Wipe has After Effects' shape controls. **Iris Points**: under 6
the wipe is the circle it always was; 6 is a triangle, 8 a diamond, 10 a five-pointed star, and
so on up to 32. **Outer Radius**: above 0, a hole of that size opens from the centre, as in the
tutorial (Completion is then not read). **Use Inner Radius** with **Inner Radius**: the star's
inner points, so you can make it spikier or fatter. **Rotation** turns the shape clockwise.
An Iris Wipe added from the menu starts as the circle; saved projects look exactly as before.

1. Import a picture and put a black solid under it. Add Effect > Transition > Iris Wipe on the
   picture. Set Iris Points to **6** and Outer Radius to **300**.
   - Expect: a black triangle in the middle, its point straight up and its base flat.
2. Set Iris Points to **8**.
   - Expect: a diamond, its corners up, down, left and right, as in the tutorial at 9:40.
3. Key Outer Radius: 0 at the start, 1800 a second later. Play.
   - Expect: the diamond grows from nothing in the middle until the picture is gone.
4. Set Iris Points to **10**, tick Use Inner Radius **On**, Inner Radius **130**.
   - Expect: a five-pointed star hole. Make Inner Radius bigger: the star gets fatter.
5. Set Rotation to **45** (with 8 points).
   - Expect: the diamond turns into a level square.
6. Set Feather to **20**.
   - Expect: the hole's edge goes soft.
7. Set Invert to **On**.
   - Expect: only the inside of the shape is kept; everything outside it is black.
8. Set Outer Radius back to **0** and move Completion.
   - Expect: the shape (still a diamond with 8 points) closes in from the edges, as the old
     circle did. With Iris Points under 6 it is exactly the old circle.
9. Open a project saved before this change that has an Iris Wipe.
   - Expect: the same circle as before.
10. Save, close and reopen.
    - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b925_iris_shape` (63 of 63, `B-925_iris_shape_table.md`): every
pixel of FX-IRIS-028 to 045 against `tools/iris_shape_reference.py`, the wrong values refused,
round trips, a half-size draft and tiles. The old cases still pass: `b101_iris_wipe` (FX-IRIS-001
to 027) and `b107_gpu_fx` (card against processor; the star and the hole stay on the processor);
the app's effect and setting tests 21 of 21.

Tutorial sheet: `verification/B-925 pictures/iris_shape_vs_tutorial.png`, rendered by the
release app (processor, full size, 1280 by 720) beside the tutorial (rsGGCshMDr8). Row 1: the
tutorial's Iris Points 6 at Outer Radius 1800 leaves only the top corners; ours at the same size
for our width (1200) leaves the same two corners, the slanted sides crossing the top edge about a
tenth of the way in and the sides about a third of the way down. Row 2: the tutorial's 8-point
diamond and ours. Row 3: the tutorial's panel, and our 10-point star with Inner Radius. Row 4: our
6-point triangle standing on its base. Adobe publishes no formula, so the rule is ours (D-1045);
the viewer's on-screen guide for the wipe stays a circle.
