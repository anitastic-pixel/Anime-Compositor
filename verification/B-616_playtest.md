# B-616 playtest: chosen keys of masks, shapes and groups move, delete and reverse together (D-741)

## What to try

1. Open a project with an animated shape layer whose groups move or turn, or whose trim, repeater or rectangle size is keyed. Open its rows in the timeline.
2. Choose two or more of those keys and drag them along the timeline, or press the arrow keys. They all move together.
3. Choose them again and press **Delete**. They all go. A number left with no keys keeps the value its first chosen key had.
4. Undo, choose them again, and use **Time-Reverse Keyframes**. The animation plays backwards between the first and last chosen key.
5. Do the same on a layer with two animated masks: choose keys of both masks' paths and of one mask's feather, and move them together.
6. Drag one key onto another of the same row that is not chosen. Nothing moves.
7. Each of these is one undo.

Before this build, chosen keys of a group, a trim, a repeater, a path operation, a mask's feather or a rectangle's size were refused when moved, deleted or reversed together, and moving keys of two masks' paths at once moved only one of them.

## What the check found (private copy, `Fixtures/shape_groups/host.json`): 13 of 13 pass

- The fixture's groups move and turn over frames 0 to 24.
- Every group key chosen and moved 5 later: the keys are at 5 and 29, and frame 17 draws exactly what frame 12 drew, pixel for pixel. One undo puts them back.
- Every group key chosen and reversed: the keys stay at 0 and 24, frame 0 draws exactly what frame 24 drew and frame 24 what frame 0 drew. One undo puts frame 0 back.
- Every group key chosen and deleted: no group keys are left, and frame 12 draws exactly what frame 0 drew. One undo puts the keys and frame 12 back.
- One key moved onto another that is not chosen: refused, nothing moves.
- No page errors.

Fixture check `app/src/tests/key_requests.rs`: 65 of 65 (see `B-616_key_requests_table.md`). FX-KREQ-001 to 016 send the timeline's three requests with chosen keys of a trim, two masks' paths and a feather, a mask's expansion, groups' transforms, repeaters, path operations and a rectangle's size. Each case checks every key the layer holds afterwards (frame, value and ease), the numbers left with no keys, that nothing else changed, and one undo. FX-KREQ-004 is the refusal. Before the fix, 32 of those 65 checks failed, every one a request refused or a key left behind. The app's other tests still pass.

## Pictures (`B-616 pictures/`)

- `all.png`: every picture below on one sheet.
- `00_before_f00.png`, `00_before_f12.png`, `00_before_f24.png`: the fixture as it opens, at frames 0, 12 and 24.
- `01_moved_f17.png`: after every group key is moved 5 later, frame 17, the same as frame 12 before.
- `02_reversed_f00.png`, `02_reversed_f24.png`: after reversing, frame 0 is frame 24 before and frame 24 is frame 0 before.
- `03_deleted_f12.png`: after deleting, frame 12 is frame 0 before.

## How this differs from After Effects

- None known. After Effects moves, deletes and reverses chosen keys of any property together.
