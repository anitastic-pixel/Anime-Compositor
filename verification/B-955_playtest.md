# B-955 playtest: Shift + parent pick whip, parent and jump (D-1075)

The tutorial (4GeGLgK2BBk, about 3:20) holds Shift while dragging a layer's parent pick whip
onto another layer. The layer is parented and jumps onto the parent. This is a change to the
window only, so there is no picture: the check is the steps below and the table
`verification/B-955_parent_and_jump_table.md`, which ran the same requests on the reference shot
and measured the child's anchor landing exactly on the parent's (700, 300).

## Steps

1. Open the reference shot. Move two layers apart in the viewer, so you can see both.
2. Drag the first layer's parent pick whip (the spiral) onto the second layer's row, and hold
   Shift before you let go. The label by the hand says **Parent and jump to:** and the layer's
   name. Let go.
3. The first layer jumps: its anchor point (the small cross) now sits on the second layer's
   anchor point. Its Parent column shows the second layer.
4. Move the second layer. The first follows it.
5. Press Ctrl+Z once. The first layer is back where it was, with no parent.
6. Do step 2 again without Shift. The label says **Parent:**, and the layer is parented but does
   not move.
7. Choose two layers, then Shift-drag the whip of one onto a third. Both jump onto it, and one
   Ctrl+Z puts both back.

Wrong if:
- with Shift the layer is parented but stays where it was, or jumps somewhere other than the
  parent's anchor point;
- undoing takes two presses of Ctrl+Z;
- without Shift the layer moves.

Not the same as After Effects:
- A layer whose Position has keyframes, or is separated into X and Y, is only parented here,
  not moved.
- Only Position is set; the child's rotation and scale are left as they were.
