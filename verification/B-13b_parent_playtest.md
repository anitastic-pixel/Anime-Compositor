# B-13b: the parent pick list, by hand

Built on 2026-09-15 against D-57, which the owner accepted the same day.

No photograph: every item is a click or a drag, and the capture script cannot click.

## What this is for

Parenting is one layer riding on another, so that a mouth cel moves with the head cel it belongs
to instead of being keyed twice. The arithmetic is checked by machine against numbers generated
from document 21. What is not checked by machine is whether the pick list is a thing an artist
can use, which is what these steps are.

## What to check by hand

Open `verification/W-16_key_shapes_project.json`. If the composition has only one layer, add a
second with the New layer button; the steps below call them the head and the mouth.

1. **Choose a parent.** Click the mouth layer. In the Inspector, under the matte chooser, there
   is a new row, **Parent**, with a list starting at "none". Choose the head. The mouth does not
   move: it is where it was, and the status strip says "Set parent to ..." .
2. **It rides.** Drag the head layer's position in the Inspector, or move it with the arrow keys.
   The mouth moves with it, keeping its place on the head. Scrub the playhead: it stays with it
   at every frame.
3. **It rides on a turn and a scale too.** Set the head's rotation to 30 and its scale to 150.
   The mouth turns and grows with it, and stays where it sits on the head.
4. **The mouth is still its own layer.** Set the mouth's opacity to 50: the head does not fade.
   Switch the head's eye off: the mouth is still drawn. That is D-57's rule - a parent carries
   the transform and nothing else - and the second half of it is the part most likely to look
   like a bug if you are expecting a group.
5. **Clear it.** Set the mouth's Parent back to "none". The mouth does not move. Ctrl+Z puts the
   parent back, and the mouth still does not move.
6. **It cannot ride on itself.** The mouth's own name is not in its list. Choose the head, then
   open the head's Parent list and choose the mouth: nothing changes and the strip says the two
   layers would ride on each other.
7. **Deleting the head.** With the mouth riding on the head, click the head and press Delete
   layer. The head goes, the mouth stays exactly where it was on screen rather than jumping, and
   **one** Ctrl+Z brings the head back with the mouth riding on it again. This is the one place
   parenting does not behave like a matte, which leaves its reference behind when its layer goes.
8. **When the shape cannot be kept.** Set the head's scale to 200 by 50 - stretched one way -
   and give the mouth a rotation of 45. Now set the mouth's Parent to the head. The change
   happens and the strip says it could not keep the shape exactly: the anchor is where it was
   and the rest of the layer has moved. Look at it: the mouth has slanted. Ctrl+Z undoes it.
   This is document 21's rule rather than a defect, and the point of the step is that the window
   tells you at the moment it happens instead of leaving you to find it in the picture.
9. **It survives the file.** With the mouth riding on the head, save, close and reopen: the
   Parent row still says the head, and the picture is the same.

## What checks it by machine

- `verification/B-13b_parenting_table.md`: where a parented layer's pixels land, FX-PARENT-001 to
  008, against a generator that walks document 21's four steps and never builds a matrix. 52 of
  52 checks, including the loop refusal, the missing parent, and the delete in step 7.
- `verification/B-13b_panel_table.md`: the chooser itself - the layer chosen is the layer used,
  the refusals in steps 6 and 8, and that deleting a parent is one entry in the history rather
  than two. 27 of 27 checks.
- `verification/B-09_persistence_table.md`: `Fixtures/projects/parenting_project.json` is read
  and written back byte for byte, which is step 9 by machine.
- **Not** checked by a test: that the mouth looks right on the head at steps 2, 3 and 8, and
  that the list is findable at all. Steps 1 to 9 are the check.
- New capabilities the owner may cut: the Parent row on the Inspector and the `layer.set_parent`
  command behind it. Cutting them leaves the transform chain in the core with no way to reach it
  from the window, and leaves the fixture table as the only thing that exercises it.

## What this build does not do

Parenting without keeping place, which After Effects offers on a modifier key. D-57 leaves it
undecided and nothing in W-04 asks for it, so a parent chosen here always keeps the layer where
it is. A locked layer refuses a parent, and a locked child stops the layer it rides on from
being deleted; both are the existing lock rule rather than anything parenting added.
