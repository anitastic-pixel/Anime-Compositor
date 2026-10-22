# B-612 playtest: repeaters (D-738 (c))

## What to try

1. On a shape layer, pick a shape in Effect Controls and press **Group** beside it. The new group's section ends in a **Repeater** line with an **Add Repeater** button.
2. Press **Add Repeater**. The group is drawn three times, each copy 100 to the right of the one before, as After Effects starts a repeater.
3. Under it are a **Below** / **Above** list and eight rows: **copies**, **offset**, **copy anchor**, **copy position**, **copy scale**, **copy rotation**, **start opacity** and **end opacity**. Each can be dragged, typed or keyed with its stopwatch, like any other number.
4. Try copy rotation 30 and copy position 5, 0: the copies fan out in an arc. A copy scale below 100% shrinks each copy more than the one before.
5. End opacity 25% fades the copies out from the first to the last.
6. Copies 3.5 draws a fourth copy at half its opacity. Offset 0.5 slides every copy half a step along.
7. **Below** draws each copy under the one before, so the first copy is on top. **Above** draws each copy over the one before. You only see the difference where copies overlap.
8. **Remove Repeater** takes it away and the group is drawn once again. Undo brings it back.
9. A copies value over 1000 is refused with a sentence that names the range.

## What the check found (private copy, `Fixtures/shape_repeaters/host.json`): 19 of 19 pass

- Frames 0, 12 and 24 draw differently: Row's copies are keyed from 4 down to 2, and Fan's offset is keyed from -1 to 1.
- Effect Controls shows the Repeater line, the Below/Above list and the eight rows for each of the three groups. The lists read below, above, above, as the file has them.
- The two keyed repeater numbers are timeline rows, named "Row copies" and "Fan offset". At frame 12 Row has 3 copies, halfway along its keys.
- Fan's copies set to 3 took effect. 5000 was refused in words and nothing changed. Undo gave Fan its 6 copies back.
- The list switched Fan to below and its overlapping petals stacked the other way. Undo put Fan back to above.
- Remove Repeater took Stack's away and the button changed to Add Repeater. Add Repeater gave 3 copies, 100 apart, below. Two undos gave Stack its own repeater back.
- No page errors.

Fixture check `tests/b612_shape_repeaters.rs`: 87 of 87 (see `B-612_shape_repeaters_table.md`). FX-SHPR-001 to 004 list the shapes drawn at frames 0, 6, 12 and 18, in drawing order: 19, 19, 16 and 16. Each shape's points, opacities and stroke width match `tools/shape_repeater_reference.py` within 1e-9, and each frame is drawn pixel for pixel as plain shapes of those outlines and paint. Saved, every repeater is kept as read, including `blend_mode`, a field this build does not know, and the saved file reads back the same. The seven broken files are each refused, naming the bad field. App test `a_group_repeats_from_the_window` covers add, composite, set, key, refuse, remove and undo.

## Pictures (`B-612 pictures/`)

- `all.png`: the first three pictures and the three after them, on one sheet.
- `00_fixture_f00.png`, `00_fixture_f12.png`, `00_fixture_f24.png`: the fixture at frames 0, 12 and 24. Row's squares (left) fade from full to 25% and drop from 4 copies to 3 to 2. Under each square, Stack repeats a small triangle twice. Fan's petals (right) slide along as the offset runs from -1 to 1. The green triangle is in no group and is drawn once.
- `01_fan_three_copies.png`: Fan cut to 3 copies.
- `02a_fan_above.png`, `02b_fan_below.png`: Fan's petals stacked above and below. Look where the petals overlap.
- `03_stack_no_repeater.png`: Stack's repeater removed. One row of small triangles is left under the squares.
- `04_stack_default_repeater.png`: Add Repeater on Stack: 3 copies 100 to the right.
- `06_window_repeaters.jpg`: the window at frame 6, with Effect Controls showing each group's Repeater line and its rows.

## How this differs from After Effects

- A repeater belongs to a group and repeats the whole group, the groups inside it included. To repeat only part of a group, group that part.
- A fraction of a copy draws the last copy at that fraction of its opacity. This is this build's reading of After Effects.
- Effects that take a shape layer's paths take each shape once, not its copies. The viewer's handles show the first copy where an offset of 0 puts it.
- There is no Ctrl+G, and no Contents tree on the timeline. Keyed repeater numbers are listed by name, as group numbers are.
