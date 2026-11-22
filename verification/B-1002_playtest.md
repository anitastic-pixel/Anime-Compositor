# B-1002 playtest: a ceiling on undo memory (D-1137)

What changed: undo now has a most it may hold, 512 MB unless you change it in Preferences
("Undo, in MB"). Each step is counted by what it alone holds: the layers its edit changed and its
own copy of the rest of the project (B-999 made steps share every layer an edit did not change).
Past the ceiling the oldest steps are let go, never the newest one. Undo and redo of every step
that is kept give back exactly the same project as before.

There is no separate History panel in this window, so the count of steps let go is said in two
places: the Undo button's hover text, and Preferences under the Undo box.

## Steps kept against the ceiling (p20)

`tests/p20_edit_cost.rs`, release, `CARGO_TARGET_DIR=target/lanes/d1137`; AMD Ryzen 9 9900X,
Windows 11 Education 10.0.26300; 400 rotations of one layer with the ceiling set to 1 MB so a
short run reaches it. Memory counts do not depend on how busy the machine is.

| | 4 layers | 50 layers | 200 layers |
|---|---|---|---|
| Steps kept of 400 | 186 | 18 | 17 |
| Steps let go | 214 | 382 | 383 |
| What the program counts the kept steps as holding | 999,006 bytes | 988,470 bytes | 986,255 bytes |
| What they really hold (counting allocator) | 1,876,377 bytes | 373,881 bytes | 636,315 bytes |
| Steps the 512 MB default would keep, about | 95,326 | 9,323 | 8,825 |

The program's count is an estimate (the length of the values written out), not a measurement: it
reads about half the real size on the 4-layer shot and between 1.5 and 2.6 times it on the larger
ones. So 512 MB set means real undo memory of roughly 270 MB to 1 GB, depending on the project.

One edit (`Document::apply`) now also counts its step: 0.022 ms at 4 layers, 0.083 ms at 50 and
0.140 ms at 200 (busy machine, not final; B-999's run had 0.016, 0.025 and 0.062 ms).

## Try it

1. Open Preferences. Under Memory, find "Undo, in MB": it shows 512, and below it a line like
   "Undo: 0 steps holding about 0.0 of 512 MB now."
2. Type 1 in "Undo, in MB" and press Tab.
3. Open a big project (100 or more layers) and make 40 small edits (nudge a layer with the arrow
   keys, change a rotation).
4. Open Preferences again.
   - Expect: fewer than 40 steps, and "... have been since the project was opened." with a number.
5. Hover over the Undo button.
   - Expect: the hover text ends "N older steps were let go to keep undo under its memory limit
     (Preferences)."
6. Press Ctrl+Z until Undo greys out, then Ctrl+Shift+Z until Redo greys out.
   - Expect: Undo stops after the steps that were kept, each one goes back and forward exactly as
     before, and the picture after the last redo is the one you had after step 3.
7. Set "Undo, in MB" back to 512 (or empty it for the default).

Pass if every "Expect" holds. If an undo gives back a different value, say which step.

Checked by: `tests/b1002_undo_ceiling.rs` (7 checks: every kept step undoes and redoes to the
project copied as the edits were made, equal by value; the oldest go first and are counted; the
newest is always kept; lowering the ceiling lets go at once; a drag is one step; dirty is still a
comparison with the save), the document 26 undo tests `tests/b05_model.rs` and
`tests/b09_persistence.rs`, `tests/p03_byte_equality.rs`, and the app suite.
