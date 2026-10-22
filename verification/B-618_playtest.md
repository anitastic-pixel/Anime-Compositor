# B-618 playtest: a shape layer's group keys under Contents, and F9 on every key under the playhead

## What to try

1. Open a shape layer whose groups move or turn, or whose repeater or path operation is keyed (for example `Fixtures/shape_groups/host.json`).
2. Twirl the layer open on the timeline, then open **Contents**. Under the shapes there is a row for each keyed group number, such as "Body rotation" and "Drift position", with its value and its keys.
3. Choose no keys, put the playhead on a key of a group, and press **F9**. That key gets Easy Ease, and so does every other key there, transform or not. One undo puts them back.

Before this build, the group, repeater and operation keys from B-610, B-612 and B-613 showed in the flat key list but not under Contents. F9 with no keys chosen eased only the transform's keys.

## What the check found (private copy, `Fixtures/shape_groups/host.json`): 5 of 5 pass

- The fixture has two keyed group numbers, Body rotation and Drift position.
- With the layer and Contents open, both have their row.
- F9 at frame 0 with nothing chosen turned both groups' keys there from linear to ease. One undo turned them back to linear.
- No page errors.

The app's tests still pass (111).

## Pictures (`B-618 pictures/`)

- `01_contents_open.png`: the whole window with Contents open.
- `02_timeline_rows.png`: the timeline rows close up. Body rotation and Drift position sit under the four shapes, each with its key at frame 0.

## How this differs from After Effects

- After Effects nests a group's numbers under the group's own row. Here they are listed after the shapes, under Contents, named by the group.
