# B-111b: a camera that rides a layer, by hand

Built on 2026-09-27 against D-171, which is **proposed** and waits for the owner. Playing this
sheet is how to judge it. If what you see here is what you want from a camera parented to a null,
then accepting D-171 and passing this sheet are the same answer.

There are two generated halves:

- `verification/B-111_camera_rig_table.md` checks the frames and the camera's place against
  FX-RIG-001 to 031.
- `verification/B-111b_panel_table.md` checks what the Parent list sends and what comes back.

The fixture cases are drawn in `verification/B-111a proposal/rig_cases.png`. This sheet covers
what none of them can judge: whether the shot moves as you expect and whether the list is usable.

## Before you start

Open a project with a composition and a few layers at different depths, or use **Save As...**
first and work on the copy. Press **New null**. The **Camera** block is in the Inspector, under
its own heading.

## What to check

1. **Nothing changed for an unparented camera.** The Camera block has a new **Parent** line
   reading **none**. The picture is as it was.
2. **The camera rides the null.** Choose the null in **Parent**. Nothing moves on screen: the
   camera keeps where it stands. Its **Place** now reads in the null's own pixels, so the numbers
   change while the picture does not.
3. **Moving the rig moves the shot.** Key the null's **Position** from one side at frame 0 to the
   other at frame 24. Play: the whole shot pans, and nearer layers slide further than far ones,
   as they do when the camera itself is keyed.
4. **Depth rides too.** Key the null's **Depth** from 0 to -500 over the same frames. Play: the
   shot pushes in, and the parallax grows.
5. **Turning and scaling the null do not turn or scale the view.** Rotate the null by 45 and set
   its scale to 200. The camera's place is carried round and out with it, so the shot moves, but
   the picture is never tilted or enlarged. This is D-171's rule: the camera only takes its place
   and depth from its parent.
6. **A null on a null.** Give the null a parent null of its own and move that one. The shot
   follows both, as a layer's chain does.
7. **Clearing it.** Go to a frame in the middle, and choose **none**. Nothing moves on that frame.
   Play: the camera now stays where it was left.
8. **Deleting the rig.** Parent the camera to the null again, and delete the null. The camera
   stays where it stood on the frame you were on, with **none** as its parent. **Ctrl+Z** brings
   the null back with the camera on it.
9. **No sound layers.** The **Parent** list does not offer an audio layer.
10. **Saved and reopened.** Save, then **Open** the same file. The camera is still on the null,
    and nothing is reported in the diagnostics.
11. **A missing parent.** This step is optional and needs a text editor. In a saved file, change
    the camera's `"parent"` to a name no layer has, and open it. The diagnostics warn that the
    camera's parent is not in the composition. The camera stands as if unparented, and the
    **Parent** list shows the name with **(missing)**. Saving keeps the name.
12. **Export.** Export the range as PNG. The frames show the pan and push as the viewer showed
    them.

## A choice for you

D-171 keeps one camera per composition, as a property of the composition, and lets it ride a
layer. After Effects instead has camera *layers*: you can have several cameras, each with an in
and out point, and cut between them on the timeline. On 2026-09-15 you chose to keep the camera
where it is (B-13e). Say whether the parent is enough, or whether you want the full camera layer.
The camera layer is a larger piece of work, and it would come after the redesign.

## Known limits

- D-171's "Not covered" list:
  - one camera per composition, with no in or out point;
  - no turning or look-at camera: the view never rotates;
  - no depth of field.
- The camera's parent is set with keep-place always on, as the layer Parent list does from the
  window.

## A correction to the fixtures

B-111a gave the whole catalogue a tolerance of 1e-9. The frames cannot meet it, because a picture
is decoded and drawn in 32-bit numbers, which differ at about 3e-8. They are now held to 1e-6, as
FX-NULL's frames always were. The points are still held to 1e-9. This is written in document 25
and in `Fixtures/camera_rig/expected_camera_rig.json` as `pixel_tolerance`.

## Result

Accepted by the owner on 2026-09-28 without a separate run (D-175).
