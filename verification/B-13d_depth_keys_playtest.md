# B-13d: a depth that keys, by hand

Built on 2026-09-16 against D-58, which the owner accepted on 2026-09-15. This is the second
half of that contract: B-13c gave a layer a depth and a composition a camera, and this gives
the depth the same stopwatch every other number in the window already had.

No photograph: every step is a drag, a click or a typed number, and the capture script cannot
drag.

## What this is for

The owner played B-13c on 2026-09-15 and reported three things about it. Two are answered here
and one is not, and it is worth saying which is which before starting.

- *"would like this to be keyable and drag-click like all the other changable values."*
  **Answered.** The Depth row is one of the window's blue numbers now, and it keys.
- *"I would like the changes to update in realtime when values change."* **Answered** by the
  drag: the picture follows the number while it is moving, rather than at the end.
- *"maybe change said values through keyframes though a camera layer maybe?"* **Half answered,
  and that half changed later the same day this sheet was written.** The camera keys from the
  window now: B-13e built it on 2026-09-16 in the shape you chose when asked - key it where it
  is - and `verification/B-13e_camera_keys_playtest.md` is its sheet. A camera is still not a
  layer; that half stays deferred, and it is listed at the bottom.

## What to check by hand

Open `Fixtures/reference_shot` as a project, or any shot with a layer or three in it.

1. **The Depth row changed shape.** Select a layer and look at the Inspector, under Parent. The
   Depth row was a box you typed into on 2026-09-15; it is a blue number now, like Position and
   Rotation above it. Drag it sideways. The layer moves nearer and further **while you drag**,
   not when you let go. Arrow keys nudge it, and pressing it still lets you type an exact
   number. This is the "drag-click like all the other changable values" from the playtest.
2. **One drag is one undo.** Drag the Depth a long way, in one motion, and press Ctrl+Z once.
   The whole drag comes back, not the last little piece of it. That is the same rule every
   other scrub in this window follows.
3. **The diamond is there now.** Beside the Depth row is the same stopwatch diamond the five
   transform rows have. Put the playhead at frame 0 and press it. A key appears on the timeline
   under the layer, on a row of its own called Depth. Nothing moves on screen: the key holds the
   value the depth already had, which is what the diamond does everywhere else.
4. **The second key writes itself.** Move the playhead to a later frame - 24, say - and drag the
   Depth. A second key appears where the playhead is, and you did **not** have to press the
   diamond again. Once a property is animated, changing its value writes a key at the frame you
   are looking at. This is what After Effects does and it is the step most worth confirming
   feels right, because it is the one that decides whether animating a depth is comfortable.
5. **Watch it.** Play the shot. The layer travels in depth: it grows or shrinks as it comes
   toward or moves away from the camera, and if other layers sit at other depths it crosses them
   at its own rate. **This is the thing the whole unit exists for.** If the motion is there but
   too subtle to read, push the numbers harder - the playtest of 2026-09-15 said exactly that
   about depth, and it is still true that depth wants large numbers before it looks like much.
6. **It behaves like a property, because it is one.** With the Depth keyed, try the things you
   would try on a Position track and check each one works on this one:
   - the arrow keys or the key-navigation buttons jump the playhead between depth keys;
   - U opens the animated properties and Depth is among them;
   - F9 eases a depth key, and the graph editor draws it a curve;
   - a depth key can be dragged along its row to another frame;
   - a depth key can be copied and pasted.
   None of these was written for the depth specifically. It gets them by being a property, and
   the point of this step is to find the one that is not true.
7. **The keys travel with the layer.** This is the step most likely to catch a real defect, so
   it is worth doing carefully. With two or more depth keys on a layer, drag the layer's bar
   along the timeline, or press `[` or `]`. The depth keys must move **with** the bar and keep
   their spacing, exactly as the position keys do. If they stay behind while the bar moves, that
   is a bug and it is the one this build specifically went looking for.
8. **It survives the file.** Save, close and reopen. The depth keys are where you left them,
   with their eases, and the shot plays the same. Then open a project made before today - any
   fixture under `Fixtures/projects/` - and save it: it still has no depth written into any
   layer that never had one.
9. **The camera keys too, since B-13e - this step has been turned around.** It used to ask you
   to confirm the opposite: that there was no diamond beside the Camera block's three rows and
   no Camera group on the timeline. B-13e built both on 2026-09-16, later the same day this
   sheet was written, so following the old wording would have looked like finding a bug. What
   you should see is a diamond beside each of the three camera rows and a Camera group at the
   top of the timeline. Keying them is walked properly by
   `verification/B-13e_camera_keys_playtest.md` and is not this sheet's job.

## What checks it by machine

- `verification/B-13d_depth_keys_table.md`: that the depth takes a key, takes a second one by
  being changed while animated, gives its keys back to the panels, can be moved along its row,
  travels when the layer is shifted, and forgets them again on undo.
- Easing a depth key is **not** in that table. F9 and the graph editor reach a depth because it
  is a property, and `verification/D-52_ease_table.md` and `verification/D-53_path_table.md` are
  where that machinery is checked; repeating their rendering of a curve here would only restate
  them. Step 6 above is where a person confirms it reaches the depth.
- `verification/B-13c_camera_table.md`: unchanged and still 105 of 105. The arithmetic of where
  a layer at a depth lands is D-58's and this unit did not touch it - a keyed depth is sampled
  at the frame and then goes through the same two lines.
- `verification/B-09_persistence_table.md`: a keyed depth round trips, which is step 8 by
  machine.
- **Not** checked by a test: whether animating a depth is comfortable, whether the automatic
  second key in step 4 is a pleasant surprise or an unwanted one, and whether the depth track
  belongs where it is on the timeline. Steps 1 to 9 are the check.

## What this build does not do

**The camera keys from the window now, which it did not when this sheet was written.** B-13e
built it on 2026-09-16 in the shape you chose when asked: the camera stays a property of the
composition, its three rows gained diamonds, and it got a fixed row group of its own at the top
of the timeline. B-13f then fixed three gestures that were claimed to reach the camera's keys
and did not - F9, Ctrl+Alt+G's hold and Ctrl+V's paste. None of that is this sheet's to check;
`verification/B-13e_camera_keys_playtest.md` is where it is walked.

**A camera is still not a layer.** The owner raised this as a question rather than a request
(*"unsure if that's how AE can do it or if that's a good idea"*), and it was deliberately
deferred rather than answered. After Effects does have camera layers, with in and out points and
something to parent them to; D-58 names that rig as a later contract, and it stays one.

**The Depth row is still in the Inspector rather than on the camera.** It sits under Parent,
where it was on 2026-09-15, because a depth reads beside the other thing that says where a layer
sits. It was not moved in among the transform five, so the playtest of 2026-09-15 will still
find it where it left it.

**Nothing here changes what a depth means.** Depth still overrides the layer stack, a layer at
or behind the camera is still refused by name, and a matte still sits on its own plane. Those
are D-58's and `verification/B-13c_camera_playtest.md` is still where they are walked.
