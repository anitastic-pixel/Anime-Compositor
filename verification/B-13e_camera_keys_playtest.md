# B-13e: a camera that keys, by hand

Built on 2026-09-16 against D-58. This is the last piece of that contract and the thing the
owner asked for after playing B-13c: the camera's three properties key, drag and ease like every
other number in the window.

No photograph: every step is a drag, a click or a typed number, and the capture script cannot
drag.

## What this is for

The owner played B-13c on 2026-09-15 and reported three things. B-13d answered two of them for
the depth. This answers the third, for the camera:

- *"I would like the changes to update in realtime when values change."* **Answered.** The
  camera's numbers drag, and the picture follows while the drag is happening.
- *"also ensure that I can maybe change said values through keyframes though a camera layer
  maybe? unsure if that's how AE can do it or if that's a good idea."* **Answered as far as the
  keyframes go, and deliberately not as far as the camera layer goes.** When asked, the owner
  chose to key the camera where it is: it stays a property of the composition, its three rows
  gain diamonds, and it gets its own row group at the top of the timeline. A camera that is a
  layer is still deferred, and the bottom of this sheet says what that would mean.

## What to check by hand

Open `Fixtures/reference_shot` as a project, or any shot with two or three layers in it at
different depths - the parallax is what makes a camera move worth watching.

1. **The camera has a group at the top of the timeline.** Look at the top of the layer list,
   above the layers. There is a row called Camera with a twirl. It is closed to begin with.
   Open it: three rows appear - Place, Depth and Lens - and they look like a layer's property
   rows, because they are the same rows.
2. **The Camera block in the Inspector drags now.** The three boxes under the composition's
   camera were boxes you typed into on 2026-09-15. They are blue numbers now. Drag the Lens
   sideways: the picture zooms **while you drag**. Arrow keys nudge it and pressing it still
   lets you type an exact number. The Lens still reads in millimetres, as it always did.
3. **One drag is one undo.** Drag the camera's Depth a long way in one motion and press Ctrl+Z
   once. The whole drag comes back, not the last pixel of it.
4. **The diamonds are there.** Beside each of the three rows - in the Inspector and on the
   timeline group both - is the stopwatch diamond. Put the playhead at frame 0 and press the one
   beside Place. A key appears on the Camera group's Place row. Nothing moves: the key holds the
   camera where it already was.
5. **The second key writes itself.** Move the playhead to a later frame - 48, say - and drag the
   camera's Place, or type a new one. A second key appears at the playhead and you did **not**
   press the diamond again, which is what the window does for every other animated property.
6. **Watch it. This is what the unit exists for.** Play the shot. The camera travels, and the
   layers part: the near one crosses the frame faster than the far one, from this one camera
   move rather than a key on every layer. If it reads as too subtle, push the numbers harder -
   the B-13c playtest said exactly that, and depth still wants large numbers.
7. **A keyed camera behaves like a keyed property, because it is one.** With the camera keyed,
   try the things you would try on a Position track:
   - a camera key drags along its row to another frame;
   - F9 eases a camera key. **This was broken when the sheet was written**: F9 built its
     request out of a layer id, and a camera has none, so the window refused it. B-13f fixed it
     on 2026-09-16 and this step is where you confirm the fix;
   - Ctrl+Alt+G holds a camera key, and Ctrl+C then Ctrl+V copies one onto another frame. Both
     were broken the same way and by the same one-line cause, and both are fixed;
   - dragging the playhead near a camera key snaps to it, as it does to a layer's;
   - a camera key is deleted with the Delete key;
   - the camera's Place can be given motion-path handles, and its Depth and Lens refuse them by
     name, because they are lengths and not points.
   None of these was written for the camera. It gets them by being named as a target where a
   layer is usually named, and the point of this step is to find the one that is not true.
8. **The Lens is millimetres to you and pixels in the file.** Type 50 into the Lens, save,
   reopen: it still says 50. That it is stored as 2666.67 on a 1920-wide composition is D-58's
   arithmetic and is not something this window should ever show you.
9. **It survives the file.** Save, close and reopen. The camera keys are where you left them,
   with their eases, and the shot plays the same. Then open a project made before today - any
   fixture under `Fixtures/projects/` - and save it: it still has no camera written into it at
   all, because nobody touched one.
10. **The camera is not in the layer list.** Check that the parent chooser does not offer the
    camera, that the matte chooser does not, that Select All does not select it, and that Delete
    with the camera group open does not delete it. This is the deferred camera-as-layer showing
    its edge, and any of those offering the camera would be a bug.

## What checks it by machine

- `verification/B-13e_camera_keys_table.md`: that each of the three properties takes a key,
  takes a second by being changed while animated, gives its keys back to the panels, moves along
  its row, eases, takes path handles on the place and refuses them on the other two, and that
  every refusal is a sentence that names what was wrong.
- `verification/B-13c_camera_table.md`: unchanged. Where a layer at a depth lands is D-58's
  arithmetic and this unit did not touch it - a keyed camera is sampled at the frame and then
  goes through the same two lines.
- `verification/B-13c_panel_table.md` and `verification/B-13d_depth_keys_table.md`: the panel and
  the depth, both unchanged by this.
- **Not** checked by a test: whether animating a camera is comfortable, whether the Camera group
  belongs at the top of the timeline or somewhere else, and whether the three rows are the right
  three. Steps 1 to 10 are the check.

## What this build does not do

**A camera is still not a layer.** The owner raised it as a question rather than a request and
chose "key it where it is" when asked, so the camera stays a property of the composition. After
Effects does have camera layers, with in and out points, a parent and a stack position; D-58
names that rig as a later contract and it stays one. Step 10 is where that edge is felt.

**The camera cannot be dragged about in the picture.** It is placed by dragging its numbers, not
by pulling a handle in the viewer. W-04 asks for a camera move and not for a handle to make it
with.

**J and K do not step over the camera's keys.** Those walk the layer list looking for the next
key, and the camera is deliberately not in that list. The playhead is moved by hand or by the
timeline for now - though since B-13f it does snap to a camera key when you drag it near one.

**The graph editor cannot be pointed at the camera.** It draws whichever layer is selected, and
the camera is not selectable, so it has no way to be told to look at one. B-13e's notes claimed
it did draw the camera; that was wrong, and B-13f corrected the record rather than building the
thing, because whether a camera needs a curve to pull by hand is a question for the owner and
not one to assume. F9 and the curve button on the Camera block are how a camera key is eased
today.

**Nothing here sends `camera.set_property` any more.** The Inspector's three boxes send what
every other number in the window sends. The command still exists and the window still answers
it; it is simply no longer how the window asks. If anything about the Camera block behaves
differently from the rest of the Inspector, that is worth reporting, because it should now be
made of exactly the same parts.
