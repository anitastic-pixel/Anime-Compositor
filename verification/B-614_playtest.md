# B-614 playtest: Merge Paths (D-738 (d))

## What to try

1. On a shape layer, select two shapes in Effect Controls that overlap, and press **Group** so both are in one group.
2. In the group's **Add operation…** list pick **Merge Paths**. A **Merge Paths** line appears with a **Remove** button and a **Mode** list, set to **Add**, as After Effects' new one is. The two shapes become one outline, painted as the first shape, with one stroke around the outside.
3. Switch the **Mode** list:
   - **Merge**: both outlines kept as they are, one shape. Where they overlap is left unfilled.
   - **Subtract**: the second shape bites out of the first.
   - **Intersect**: only where both overlap is kept.
   - **Exclude**: where they overlap is cut out, the rest kept.
4. Operations after Merge Paths in the list change the merged outline. Add Round Corners below it and the merged shape's corners round.
5. A merge in a group that holds another group merges that group's shapes and its repeated copies too.
6. **Remove** takes the merge away and each shape is drawn with its own paint again. Undo brings it back.

## What the check found (private copy, `Fixtures/shape_merge/host.json`): 22 of 22 pass

- Frames 0, 12 and 24 draw differently: the Moon slides off the Sun and the Hole in the pink square grows.
- The file's eight merges are in all five modes. Effect Controls shows a **Mode** list on each one, reading what the file has. Each list offers Add, Merge, Subtract, Intersect, Exclude. There are 11 Remove buttons, one for each operation in the file. The Add operation list offers Merge Paths.
- Union switched to Subtract: the Sun loses a bite. Then Intersect: only the lens where they meet is left. Bite switched to Add: the pink square is whole. A mode it does not have ("union") was refused in words and nothing changed. Three undos gave back the fixture, pixel for pixel.
- Remove took Union's merge away: the Sun and the blue Moon are drawn apart. Add operation put a new Merge Paths back, set to Add, and the picture was the fixture again, pixel for pixel.
- A merge added to Spokes, the inner group, set to Exclude, left the wheel as it was. It merges each bar on its own before Spokes repeats it, as After Effects' Merge Paths above a Repeater does.
- Wheel's own merge set to Exclude opened the hub, where all six bars cross. Undo gave the fixture back.
- No page errors.

Fixture check `tests/b614_shape_merge.rs`: 40 of 40 (see `B-614_shape_merge_table.md`). FX-SHPM-001 to 003 list the 10 shapes drawn at frames 0, 12 and 24. Each shape's outlines, points, opacities and stroke width match `tools/shape_merge_reference.py` within 1e-9, and each frame is drawn pixel for pixel as shapes of those outlines and paint. When saved, every merge is kept as read. That includes Join's `note` and `blend`, fields this build does not know. The saved file reads back the same. Both broken files are refused and name `mode`. App test `a_group_merges_from_the_window` covers add, mode, refuse and undo. The B-609, B-610, B-611, B-612 and B-613 fixture checks still pass unchanged.

## Pictures (`B-614 pictures/`)

- `all.png`: every picture below on one sheet.
- `00_fixture_f00.png`, `00_fixture_f12.png`, `00_fixture_f24.png`: the fixture at frames 0, 12 and 24.
  - Top row: Union's Sun and Moon are one yellow outline. Bite's pink square has the Hole cut from it, growing. Overlap keeps the green trapezoid where the bar and triangle meet. Xor's two purple squares lose their overlap, repeated twice.
  - Middle row: Keep's two green squares keep both outlines, rounded, their overlap unfilled. Wheel's six bars are one yellow star with rounded corners. Puffed's two bloated cyan squares are one shape.
  - Bottom row: Join's two salmon squares sharing a side are one square, no line between. The grey triangle is in no group.
- `01a_union_subtract.png`, `01b_union_intersect.png`: Union as Subtract (a crescent), then Intersect (a lens).
- `01c_bite_add.png`: Bite as Add: the pink square is whole.
- `02_union_no_merge.png`: Union's merge removed: the Moon has its own blue paint again.
- `03_wheel_exclude.png`: Wheel as Exclude: the hub opens where the bars cross.
- `04_window_merge.jpg`: the window at frame 12. Effect Controls shows Union's, Bite's, Overlap's and Xor's Merge Paths lines, each with Remove and its Mode. The viewer is in Draft, so its shapes look blocky. The thin lines over them are each shape's own path, drawn before the merge.

## How this differs from After Effects

- Add, Subtract, Intersect and Exclude work on each outline flattened to corners. A merged curve has no handles, only many short sides, close enough not to show. An open path is closed by a straight line.
- Shapes are filled by the even-odd rule here, where After Effects fills by non-zero. So with **Merge**, or a path that crosses itself, where outlines overlap is left unfilled.
- The merged shape takes the paint of the first path drawn, which is After Effects' bottom one.
- A merge over more than 20000 sides in all leaves the paths as they are.
- The viewer's handles and effects that take a shape layer's paths use each shape before the merge.
- There is no Contents tree on the timeline. Merge Paths has no numbers to key, so it has no timeline rows.
