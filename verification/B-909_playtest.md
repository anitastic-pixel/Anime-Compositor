# B-909 playtest: Trim Comp to Work Area, Ctrl+Shift+X

What changed (D-1030): After Effects' Composition > Trim Comp to Work Area. With a work area set
by B and N, Ctrl+Shift+X makes that stretch the whole composition.

1. Open any project with a composition of a few seconds. Put the playhead at 1 second and press B,
   then at 3 seconds and press N.
   - Expect: the blue work-area band runs from 1 to 3 seconds.
2. Add a marker (*) at 0.5 seconds and another at 2 seconds.
3. Press Ctrl+Shift+X.
   - Expect: the timeline now runs only from 1 to 3 seconds (the ruler starts at 1 second, as After
     Effects' does after a trim). The marker at 2 seconds is still there; the one at 0.5 seconds is
     gone. The picture at 2 seconds looks exactly as it did before.
4. Press Ctrl+Z.
   - Expect: the composition is its old length again, both markers back, the work area 1 to 3.
5. Press Ctrl+Shift+X twice.
   - Expect: the second press does nothing but say "The work area is already the whole
     composition: set it with B and N first." in the status line.
6. Check it is also in the Composition menu (Trim comp to work area), in the command finder, and on
   the timeline's right-click menu.

The checks are in `verification/B-909_trim_to_work_area_table.md` (13 of 13).
