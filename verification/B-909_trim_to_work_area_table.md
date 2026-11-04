# B-909: Trim Comp to Work Area, Ctrl+Shift+X (D-1030)

Written by `b909_trim_to_work_area` in `app/src/tests/w12v.rs`.

After Effects: Composition > Trim Comp to Work Area (Ctrl+Shift+X) makes the work area the whole composition. The first rows run the window's own commands on `Fixtures/projects/cel_holds_project.json` and read the composition back; the last rows pin how the page sends it.

**13 of 13 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| The project as saved: five frames, the work area the whole of them | first frame 0, 5 frames, work area 0 to 5, markers [], Cel 0 to 5 | first frame 0, 5 frames, work area 0 to 5, markers [], Cel 0 to 5 | pass |
| Trimming with the work area the whole composition is refused, with what to do | The work area is already the whole composition: set it with B and N first. | The work area is already the whole composition: set it with B and N first. | pass |
| and nothing changed | first frame 0, 5 frames, work area 0 to 5, markers [], Cel 0 to 5 | first frame 0, 5 frames, work area 0 to 5, markers [], Cel 0 to 5 | pass |
| B at frame 1, N at frame 3, markers on 0, 2 and 4 | first frame 0, 5 frames, work area 1 to 4, markers [0, 2, 4], Cel 0 to 5 | first frame 0, 5 frames, work area 1 to 4, markers [0, 2, 4], Cel 0 to 5 | pass |
| Ctrl+Shift+X: the composition is frames 1 to 3, the work area all of it, the marker inside kept, the layer untouched | first frame 1, 3 frames, work area 1 to 4, markers [2], Cel 0 to 5 | first frame 1, 3 frames, work area 1 to 4, markers [2], Cel 0 to 5 | pass |
| One entry to undo | Trim the composition to the work area | Trim the composition to the work area | pass |
| A second Ctrl+Shift+X has no work area to trim to and is refused | The work area is already the whole composition: set it with B and N first. | The work area is already the whole composition: set it with B and N first. | pass |
| Undo puts the composition back as it was | first frame 0, 5 frames, work area 1 to 4, markers [0, 2, 4], Cel 0 to 5 | first frame 0, 5 frames, work area 1 to 4, markers [0, 2, 4], Cel 0 to 5 | pass |
| The page holds: Ctrl+Shift+X sends it | true | true | pass |
| The page holds: the command it sends | true | true | pass |
| The page holds: the Composition menu has it | true | true | pass |
| The page holds: the command finder has it, under After Effects' key | true | true | pass |
| The page holds: the timeline's right-click menu has it, greyed with no work area | true | true | pass |
