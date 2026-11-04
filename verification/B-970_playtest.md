# B-970: 3D Point Control (D-1090)

Built on 2026-11-03 as D-1090 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). Lane w12e's feature
sweep found After Effects' 3D Point Control missing (gap G17); the other six Expression Controls
were already here.

**3D Point Control** (Effects panel, Expression Controls) draws nothing. It holds one point of
three numbers, x, y and z, that expressions read as
`effect("3D Point Control")("3D Point")`: `[0]` is x, `[1]` is y and `[2]` is z. It can be keyed
like any setting, and a null may carry it.

The check, `verification/B-970_point3d_control_table.md` (64 of 64 cells pass), compares sixteen
expressions at four frames against numbers worked out by a separate program
(`tools/expression_reference.py`) before the code existed.

## What to check

1. **The sheet.** `verification/B-970 pictures/1_3d_point_control.png`: five frames of an orange
   square driven by a null's 3D Point Control (x and y move it, z grows and turns it). A thin blue
   outline, worked out separately, marks where it should be. The orange square should sit exactly
   inside the blue outline in every frame.
2. **In the app.** Add a null (Layer > New null, Ctrl+Alt+Shift+Y). In the Effects panel open Expression
   Controls and add **3D Point Control** to it. Effect controls should show one setting, 3D Point,
   with three number boxes. Type 100, 50, 300.
3. **Read it.** Make a shape or solid. Alt+click the stopwatch of its Rotation and type
   `thisComp.layer("Null 1").effect("3D Point Control")("3D Point")[2]` (use the name the null shows in the timeline).
   The layer should turn to 300 degrees. Change the third box to 90: it should turn to 90.
4. **Key it.** On the null, key 3D Point at two times with different numbers and play: the
   rotation should follow the z between the keys.
5. **Save and open.** Save, close and open the project: the control and its three numbers
   should come back.

A 2D layer's Position also accepts the whole point and drops the z, as in After Effects (D-1046,
D-1094): `effect("3D Point Control")("3D Point")` on Position puts the layer at the point's x and y.
Known differences: no handle is drawn in the viewer for the
point, as none is for Point Control. An older build keeps the effect but does not run it, so an
expression reading it shows an error there.
