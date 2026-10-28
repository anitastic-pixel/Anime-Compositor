# B-834 playtest: Ctrl+click back to linear, both sides, one Ctrl+Z (D-954)

What changed: a Ctrl+click on a key that turned it auto bezier, then a second Ctrl+click, now puts the key back to plain linear on both sides, and each click is one step to undo.

## Steps

1. Open the reference shot. Show layer3's Rotation keys (it has three: 0, 30, 0).
2. Ctrl+click the middle key (frame 40). Its diamond turns to the auto shape and the rotation eases into and out of 30. Ctrl+Z once: the key is plain linear again. Ctrl+Shift+Z to bring it back.
3. Ctrl+click the middle key again. The diamond is plain again, and in the graph both lines either side of it are straight, the one coming in from frame 0 as well.
4. Ctrl+Z once: the key is auto again, with both sides eased. Ctrl+Z once more: as it started.

## What you should not see

- After the second Ctrl+click, a curve still bending into frame 40.
- Ctrl+Z needed two or three times to take back one click.

The computer's checks are in `verification/B-834_ctrl_click_linear_table.md`; the check in a private copy of the app is `verification/B-825_private_app_table.md`.
