# B-927 playtest: Freeze Frame on plain footage

What changed (D-1047): as in After Effects (tutorial E8DdN7vMnmE at 1:10), Freeze Frame works on
a video or still layer straight away. It turns Time Remapping on by itself and writes one hold key
at the playhead. Before, the menu item was greyed out until you turned Time Remapping on with
Ctrl+Alt+T first.

1. Import a video and drag it into a composition. Move the playhead to the middle of it.
2. Right-click the layer in the timeline. Choose **Freeze frame**.
   - Expect: the item is not greyed out. After choosing it, the picture stops moving: every frame
     shows the frame that was under the playhead.
3. Open the layer's properties in the timeline.
   - Expect: a **Time Remap** row with one square (hold) key at the playhead.
4. Press Ctrl+Z once.
   - Expect: the Time Remap row is gone and the video plays again.
5. Do the same on a drawn image-sequence layer.
   - Expect: as before this change, the drawing under the playhead is held over the whole layer
     and no Time Remap row appears.
6. Right-click a solid or text layer.
   - Expect: Freeze frame is still greyed out (it has no frames to hold).
7. Save, close and reopen the project from step 2.
   - Expect: the video is still frozen on the same frame.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by the app test `d1047_freeze_frame_on_footage_turns_time_remapping_on` (one hold key
with the value 3 at frame 3, no exposures written, one undo entry, undo removes the remap; the drawn
layer still frozen by its exposures) and `d314_chosen_keys_reverse_and_a_drawing_freezes`, both
passing. The key's numbers are FX-TREMAP-045's, already checked by B-202.
