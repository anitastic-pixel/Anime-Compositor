# B-613 playtest: path operations (D-738 (d))

## What to try

1. On a shape layer, pick a shape in Effect Controls and press **Group** beside it. The group's section now has an **Operations** line with an **Add operation…** list.
2. From the list pick **Round Corners**. A **Round Corners** line appears with a **Remove** button and a **radius** row, set to After Effects' 10. The shape's corners round off. Drag the radius up and they round more.
3. Try the others from the same list:
   - **Pucker & Bloat**: amount below 0 pulls the sides in to spikes, above 0 swells them out into petals.
   - **Twist**: angle turns the shape more the further a point is from the center. Center is two numbers.
   - **Zig Zag**: size, ridges and a **Corner** / **Smooth** list. Ridges split every side.
   - **Wiggle Paths**: size, detail, wiggles/s, correlation and seed. The shape moves on its own as the frames play, with no keys.
   - **Offset Paths**: amount, miter limit and a **Miter** / **Round** / **Bevel** list. A positive amount grows the shape, a negative one shrinks it.
4. A group can have up to eight. They work in the order listed, top first. Round Corners then Twist is not the same as Twist then Round Corners.
5. Every number has its stopwatch and can be keyed. Keyed numbers show as timeline rows, named like "Corners radius".
6. **Remove** takes that one operation away. Undo brings it back.
7. A value out of range, such as 500 ridges, or a ninth operation is refused with a sentence that names it.

## What the check found (private copy, `Fixtures/shape_operations/host.json`): 23 of 23 pass

- Frames 0, 12 and 24 draw differently: Corners' radius, Puff's amount and Cut's amount are keyed.
- Wobble's card has no keys but moves between frames 0 and 1. That is Wiggle Paths.
- Effect Controls shows each operation's line with its Remove button and its rows, named in words. There are 12 Remove buttons, one for each operation in the file. The Points lists read corner, smooth and the Join lists round, miter, bevel, as the file has them. Each of the 11 groups has an Add operation list.
- The three keyed numbers are timeline rows: "Corners radius", "Puff amount" and "Cut amount". At frame 12 Corners' radius is 15, halfway between its keys.
- Swirl's twist was set to 200 degrees. 500 ridges on Zig was refused in words and nothing changed. Undo gave Swirl back its 120.
- The Join list switched Grow to miter and its corners changed. The Points list switched Zig to smooth. Two undos put both back.
- Remove took Swirl's Twist away and kept its rounding. Add operation put a new Pucker & Bloat (amount 0) under Zig's Zig Zag, and its amount was set to -80.
- Corners took eight operations. The ninth was refused in words. Ten undos gave back the fixture as it was.
- No page errors.

Fixture check `tests/b613_shape_operations.rs`: 73 of 73 (see `B-613_shape_operations_table.md`). FX-SHPO-001 to 004 list the 13 shapes drawn at frames 0, 6, 12 and 24. Each shape's points, handles, opacities and stroke width match `tools/shape_operation_reference.py` within 1e-9, and each frame is drawn pixel for pixel as plain shapes of those outlines and paint. When saved, every operation is kept as read. That includes Spike's `note` and `blend`, fields this build does not know. The saved file reads back the same. Each of the eleven broken files is refused and names the bad field. App test `a_group_operates_from_the_window` covers add, choice, set, key, refuse, remove and undo.

## Pictures (`B-613 pictures/`)

- `all.png`: the three fixture frames on top, then the three changes below them, on one sheet.
- `00_fixture_f00.png`, `00_fixture_f12.png`, `00_fixture_f24.png`: the fixture at frames 0, 12 and 24.
  - Top row: Corners' square goes from sharp corners to round. Puff's two pentagons go from spiky stars to plain to flower petals. Swirl's bar is rounded and twisted. Zig's line is a zig zag.
  - Middle row: Wobble's card is wiggled. Grow's triangle is grown with round corners. Spike's star is grown with mitred points, and its open hook sits below it. Cut's oval shrinks, then grows. Waves' circle is zig-zagged smooth.
  - Bottom row: the grey triangle is in no group and is drawn as it is. Inner's tile is rounded, scaled and twisted by Outer.
- `01_swirl_twist_200.png`: Swirl's bar twisted 200 degrees instead of 120.
- `02a_grow_round.png`, `02b_grow_miter.png`: Grow's triangle with round joins, then miter joins. Look at its three corners.
- `02c_zig_smooth.png`: Zig's line with smooth points.
- `03_swirl_no_twist.png`: Swirl's Twist removed. The bar is rounded and straight.
- `04_zig_puckered.png`: Pucker & Bloat -80 added under Zig's Zig Zag.
- `05_window_operations.jpg`: the window at frame 6. Effect Controls shows Swirl's Twist and Zig's and Wobble's operations.
- `06_effect_controls_rows.png`: the panel up close: each operation's line, its Remove button, its Corner list, its rows, and the Add operation list.

## How this differs from After Effects

- Merge Paths came in its own build, B-614 (see `B-614_playtest.md`).
- An operation belongs to a group and changes everything in the group. To change only part of a group, group that part.
- Offset Paths gives straight sides only. On a curve it makes many short sides, close enough not to show. Where shrinking folds a shape over itself, the fold is filled in alternation.
- Wiggle Paths moves smoothly at random from its seed, but not with After Effects' own random numbers, so the same seed wiggles differently. It has no smooth/corner points and no temporal or spatial phase.
- Effects that take a shape layer's paths, and the viewer's handles, use each shape without its operations.
- There is no Contents tree on the timeline. Keyed operation numbers are listed by group and number name.
