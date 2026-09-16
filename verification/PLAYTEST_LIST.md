# What is waiting to be played with

The things in this build that a test cannot judge, gathered in one place on 2026-09-15 at the
owner's request. Every one of them needs a person at the window, because what each one asks is
whether the thing is usable, not whether it is correct. The tables already say it is correct.

They are in the order worth doing them in. The owner performed the first three on 2026-09-15
and all three passed; what they reported is written under each. The fourth was not ready when
this list was written and is ready now, because D-58 was accepted the same day and B-13c was
built against it.

## 1. Parenting, which was built yesterday and has never been used

`verification/B-13b_parent_playtest.md`, nine steps by hand.

**PASSED on 2026-09-15**: the owner reports that the parenting works.

52 checks say the arithmetic is right and 27 say the chooser in the window reaches it. What
none of them can say is whether the pick list is a thing an artist can use: whether the layer
you meant is the layer you can find in it, whether it is obvious what will happen before it
happens, and whether the sentence the window shows when a parent change cannot be exact means
anything to the person reading it.

**The one to watch for**: set a parent, then clear it, and see whether the layer stayed where
you left it both times. It should not move. If it does, that is the keep-place rule failing in
a case the fixtures did not think of, which is exactly what a playtest is for.

## 2. The ten habits, again, now that there is more to have them in

`verification/W-20_ten_ae_habits.md`, `verification/W-22_drag_layers_and_ten_habits.md` and
`verification/W-23_menu_ends_and_ten_habits.md` each ran the same ten reflexes from other
software against the window as it was at the time. The window has gained parenting, a work
area, a blend settings panel, compositions and a layer clipboard since. The ten are worth
running once more against what is there now, because every one of them was a place the window
did not behave the way a hand expected, and new features are new places for that.

Nothing to prepare: open a shot and try to work in it for twenty minutes.

**PASSED on 2026-09-15**: the owner reports that the habits work out well.

## 3. W-01 with the network actually off

`verification/T-10_adapter_off_checklist.md`, written today and never performed.

This is the last thing standing between T-10 and a clean result, and it cannot be automated
because it requires the machine to be disconnected. Ten rows, one pass through the reference
shot. Read the "Before you start" section first: the order matters, because the program has to
be started for the first time **after** the adapter is already off.

**PASSED on 2026-09-15**: the owner reports that it works with no network on. That was the last
thing standing between T-10 and a clean result, and document 11 now records T-10 as run and
passing.

## 4. W-04, the camera walk, which is ready now

`verification/B-13c_camera_playtest.md`, nine steps by hand.

The owner accepted D-58 on 2026-09-15 and B-13c was built against it the same day, so this is
no longer waiting on anything: a background, a middle ground and a foreground, and a camera
tracked sideways across them, which is the shot the whole entry exists for. 105 checks say the
projection is right and 28 say the controls reach it. What none of them can say is whether the
parallax looks like parallax and whether a person can set the shot up without being told how.

**The one to watch for**: step 4, where the camera tracks two hundred pixels. The foreground
should slide a long way, the middle less and the background least. If all three move together,
the depths did not take.

The thing to know before starting, because it is the one consequence a person meets without
asking for it: **depth overrides the layer stack.** Once two layers are at different depths,
dragging one to the top of the layer list will not bring it in front of the other. That is what
depth means and every program that has it works that way, but it is worth knowing before it
happens rather than after.

The largest gap when this was written was that nothing in the window keyed a depth or a camera.
**The owner played this on 2026-09-15 and asked for both halves of it.** B-13d built the depth
on 2026-09-16 and B-13e built the camera the same day; they are numbers 5 and 6 below. A camera
move can be animated from the window now, which it could not be when this walk was written.

## 5. A depth that keys, which is what the last playtest asked for

`verification/B-13d_depth_keys_playtest.md`, nine steps by hand.

This one exists because of the playtest above. The owner walked W-04 on 2026-09-15, reported
that the camera and the parallax work, and asked for two things: that the values update while
they change rather than after, and that the depth be "keyable and drag-click like all the other
changable values". Both are built. The Depth row is a blue number that drags, and it keys.

**The one to watch for**: step 4, where a second key writes itself. Once a depth has one key,
changing its value at another frame writes a key there without the diamond being pressed again.
That is what After Effects does, and whether it is a pleasant surprise or an unwanted one is a
question only a person at the window can answer.

**The other one to watch for**: step 7, dragging the layer's bar along the timeline with depth
keys on it. The keys must travel with the bar. They did not until this build - the code that
moved a layer moved its five transform tracks and left the depth behind - and a fix nobody
checks is a fix nobody knows about.

The sheet ends by saying the camera does not key from the window. That was true when it was
written and stopped being true the same day: B-13e built it in the shape the owner chose, and it
is number 6 below. Step 9 of the sheet, which asks a person to confirm the camera has no
diamonds, is the one step of it that is now out of date.

## 6. A camera that keys, which is the rest of what that playtest asked for

`verification/B-13e_camera_keys_playtest.md`, ten steps by hand.

The other half of number 5, built the same day. The owner asked whether the camera's values
could be changed through keyframes, *"though a camera layer maybe? unsure if that's how AE can
do it or if that's a good idea"*, and chose the answer when asked: key it where it is. The
camera stays a property of the composition, its three rows gain the same diamonds every other
number in the window has, and it gets a group of its own at the top of the timeline. A camera
that is a layer was deferred rather than refused, and the sheet says what that would have meant.

**The one to watch for**: step 6, the camera move itself. Two or three layers at different
depths, the camera keyed across them, and the near one should cross the frame faster than the
far one from that single move rather than from a key on every layer. That is the whole reason
D-58 exists and it is the first time in this project it can be animated.

**The other one to watch for**: step 10, where the camera should be absent from everything that
belongs to layers - the parent chooser, the matte chooser, Select All, and Delete. That is the
edge of the deferred camera-as-layer, and the camera appearing in any of those is a bug.

The thing to know before starting: the Camera block's three boxes are no longer special. They
send what every other number in the window sends, which is what made keying them possible. If
anything about them behaves differently from the rest of the Inspector, that is worth saying.

## Not on this list

The six diagnostics that are specified and not built, T-16's legal review, and the D-49
re-record: none of them is something to play with. They are in document 11 and document 14
where they belong.
