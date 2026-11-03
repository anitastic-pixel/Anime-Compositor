# B-815 playtest: Bulge's Antialiasing and Pin All Edges

What changed (D-935): Bulge has two more settings, as After Effects' Bulge does.
**Antialiasing (Best Quality Only)**: Low or High. High reads each pixel four times a quarter
pixel apart and averages them, so thin lines through a strong bulge or pinch come out smoother.
**Pin All Edges**: Off or On. On keeps the layer's border where it is; the pull fades in over a
quarter of the shorter side, so a pinch no longer pulls the edges in and leaves empty gaps.
A Bulge added from the Effect menu starts on Low and Off; saved projects look exactly as before.

1. Import a picture with straight lines (a grid or a brick wall). Add Effect > Distort > Bulge.
   Set Radius past the picture's edges (for a 1920 by 1080 picture, Radius 1000, Vertical
   Radius 1500) and Height -0.7.
   - Expect: Antialiasing **Low**, Pin All Edges **Off**; the middle pinched, the top and bottom
     edges pulled in with empty curved gaps, as in the tutorial at 37:20.
2. Set Pin All Edges to **On**.
   - Expect: the gaps gone; the border straight and where it was; the middle still pinched.
3. Set Antialiasing to **High**.
   - Expect: the same shape, the lines a little softer and less stair-stepped (zoom in to see).
4. Set Height to 0.
   - Expect: the picture exactly as it was without the effect.
5. Turn the viewer to a lower resolution (Half).
   - Expect: the same shape; the lines may look a little harder (the preview reads Low).
6. Open a project saved before this change that has a Bulge.
   - Expect: Low and Off, and the picture exactly as before.
7. Save, close and reopen.
   - Expect: every setting reads as you left it and the picture is the same.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `cargo test --test b815_bulge_quality` (36 of 36, `B-815_bulge_quality_table.md`):
every pixel of FX-BULGE-024 to 034 against `tools/bulge_quality_reference.py`, the two wrong
words refused, round trips and tiles. The old cases still pass: `b95_bulge` (88 of 88),
`b193_bulge_radii` (5 of 5) and `b107_gpu_fx` (7632 of 7632, card against processor; High and
Pin stay on the processor); the app's contract tests 10 of 10.

Tutorial sheet: `verification/B-815 pictures/bulge_quality_vs_tutorial.png`, rendered by the
release app (processor, full size) on a 480 by 270 grid with Radius 260, Vertical Radius 420,
Height -0.7, beside the tutorial (jv4axtpnZps, 37:20, Height -0.7, High, Pin off). Top: the
tutorial's pinch pulls the top and bottom edges in, leaving black curved gaps; ours with Pin
off does the same (row 2). Row 3: Pin On keeps the red border straight and fills the gaps.
Row 4, zoomed four times: Low's lines step between dark and light; High's are evener and
softer. The pixel at the frame's very middle, on a line, reads Low (110, 112, 116), High
(146, 147, 148). Adobe gives no formula for either switch, so both rules are ours (D-935).
