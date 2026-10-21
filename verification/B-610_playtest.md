# B-610 playtest: shape groups (D-738 (b))

## What to try

1. Open Effect Controls on a shape layer. Beside each shape there is a switch saying which group it is in ("in no group", "in Body" and so on) and a **Group** button.
2. Click **Group** beside a shape. It goes into a new group, "Group 1", inside the group it was already in. Nothing moves: a new group starts with no change of its own.
3. Below the shapes, each group has its own section, "Group: Body", with an **Ungroup** button and seven numbers: Anchor, Position, Scale, Rotation, Opacity, Skew and Skew Axis. Change or key them as you would a layer's; every shape in the group (and in any group inside it) follows.
4. **Ungroup** gives the group's shapes back to the group above it (or to no group) and takes the group, and its numbers, away, as After Effects does.
5. Use the switch to move a shape into another group or out of every group. It is then placed by that group's numbers.
6. Click **Edit points** on a shape in a turned or moved group and drag a point on the viewer. It lands where you let go.
7. A Skew outside -85 to 85 is refused, with a sentence saying why.

## What the check found (private copy, `Fixtures/shape_groups/host.json`): 18 of 18 pass

- The fixture opens with four shapes in three groups: Torso in Body, Hand in Arm (which sits inside Body), Free in no group, Badge in Drift.
- Effect Controls has a section for each group with its seven numbers, Arm says it sits "in Body", there is an Ungroup button for each group, and each shape's switch shows its group.
- The timeline shows the fixture's keyed group numbers as rows named "Body rotation" and "Drift position".
- Body's position keyed (200, 150) at frame 0 and (240, 150) at frame 24 is (220, 150) at frame 12. Two undos take the keys off.
- A skew of 90 is refused: "The shapes cannot be drawn: they need group "Body"'s skew from -85 to 85, not 90."
- Group beside Free put it in a new "Group 1"; Ungroup gave it back to no group and took the group away.
- The switch took Hand out of its group; undo put it back in Arm.
- At frame 12, where Body is turned 45 degrees, Torso's first point is drawn at (256.57, 150). Dragged 10 px right with the mouse, it is stored at (47.071, -47.071) in Body's own space, which is exactly where the hand let go once Body turns it again.
- No page errors.

Fixture check `tests/b610_shape_groups.rs`: 23 of 23 (see `B-610_shape_groups_table.md`). Each shape, placed by its groups, matches `tools/shape_group_reference.py` within 1e-9 and is drawn with the same pixels as a plain shape of that outline; saving writes every group as read; the seven broken files are refused, naming the word.

## Pictures (`B-610 pictures/`)

- `all.png`: every picture below on one sheet.
- `00_fixture_f00.png`, `00_fixture_f12.png`, `00_fixture_f24.png`: the fixture at frames 0, 12 and 24. Body (the square, half see-through) turns 0, 45 and 90 degrees and carries Arm's squashed, skewed ellipse with it; the star drifts right and down with Drift; the green triangle is in no group and stays still.
- `01_body_position_keyed_f12.png`: Body moved 20 px right by its newly keyed position.
- `02_free_grouped.png`: the triangle after Group; nothing moves.
- `03_hand_in_no_group.png`: Hand out of every group, so it sits at the layer's top-left corner (only a corner of it shows).
- `04_torso_point_dragged_f12.png`: the square's right corner dragged 10 px right at frame 12.

## How this differs from After Effects

- Group is a button beside each shape, one shape at a time; there is no Ctrl+G for several chosen shapes yet (a request can group several: `shape.group?shapes=0,2`).
- The timeline lists keyed group numbers by name ("Body rotation"), not as a Contents tree.
- A grouped shape's gradient handles are shown in the group's own space, not where the group puts them.
- Group numbers have no curve in the graph editor yet, as shape numbers don't.
