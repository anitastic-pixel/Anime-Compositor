# B-13c: the camera walk, by hand

Built on 2026-09-15 against D-58, which the owner accepted the same day.

No photograph: every step is a click, a drag or a typed number, and the capture script cannot
type.

## What this is for

This is W-04, the shot the whole B-13 entry exists for: a background, a middle ground and a
foreground, and a camera tracked sideways across them. The arithmetic is checked by machine
against numbers a generator worked out without ever building a matrix. What is not checked by
machine is whether a person can set a shot up in this window and whether the parallax looks
like parallax, which is what these steps are.

One thing to know before starting, because it is the one consequence a person meets without
asking for it: **depth overrides the layer stack.** Once two layers are at different depths,
dragging one to the top of the layer list will not bring it in front of a nearer one. That is
what depth means, and every program that has it works this way.

## What to check by hand

Open `Fixtures/reference_shot` as a project, or any shot with three layers in it. The steps
below call them the background, the middle and the foreground. If the composition has fewer
than three layers, add them with the New layer button.

1. **Nothing has changed yet.** Look at the Inspector with a layer selected. Under the Parent
   row there is a new row, **Depth**, reading 0. Under the layer's own fields there is a new
   **Camera** heading with three rows: Place, Depth and Lens, reading 960 and 540, -2666.67 and
   50. Do not touch them. The picture is exactly what it was before this build: that is the
   point of the first row of the fixture table, and it is worth seeing once with your own eyes
   before changing anything.
2. **Push the background back.** Select the background and type 1920 into its Depth. It gets
   smaller, and it stays centred on the middle of the frame. Type 0 again and it comes back to
   the size it was. Ctrl+Z works on each of these.
3. **Build the three planes.** Background Depth 1920, middle 640, foreground 0. The three are
   now at three distances. Nothing moves yet, because the camera has not moved.
4. **Track the camera.** In the Camera block, change Place from 960 to 760 - two hundred pixels
   left. **This is the step the entry exists for.** The foreground slides a long way, the middle
   less, the background least. Put it back to 960 and they return. If all three move by the same
   amount, the depths did not take, so go back to step 3.
5. **Change the lens.** Set Lens to 24. The shot widens and the parallax gets stronger. Set it
   to 100: the shot tightens and the planes flatten towards each other, the way a long lens
   flattens a real one. Put it back to 50. The box shows you millimetres; the file holds pixels,
   and the row in the panel table is the one that checks you get back the number you typed.
6. **Depth beats the layer list.** With the background at 1920, select it and press Move up
   until it is at the top of the layer list. It does **not** come to the front of the picture:
   it is still behind, because it is further away. This is correct and it is the thing most
   likely to look like a bug. Put it back down.
7. **The camera can be too close.** Set the foreground's Depth to -2666.67 or anything beyond
   it, which puts it at or behind the camera itself. The layer is not drawn, and the window says
   so by name rather than showing an empty frame with no explanation. Ctrl+Z.
8. **A matte sits on its own plane.** Only if your shot has a matte: put the matte layer and the
   layer it shapes at different depths and track the camera. The matte slides against its layer.
   That is what D-58 says happens and it is written down as a known consequence, not a defect -
   but it is the one thing in this build most likely to be wrong for the way you actually work,
   so if it gets in your way, say so and it becomes the next decision.
9. **It survives the file.** Save, close and reopen. The three depths and the camera are as you
   left them and the picture is the same. Then open a project made before today - any fixture
   under `Fixtures/projects/` - and save it: it still has no camera in it at all, because a
   camera is written only where somebody has touched one.

## What checks it by machine

- `verification/B-13c_camera_table.md`: where a layer at a depth actually lands, FX-CAM-001 to
  011, against `tools/camera_reference.py`, which walks document 21's four steps and then does
  D-58's two lines by hand, never building a matrix. 105 of 105 checks, including the draw order
  in step 6 and the refusal in step 7.
- `verification/B-13c_panel_table.md`: the controls themselves - the number typed is the number
  stored, the millimetres round trip of step 5, and the refusals. 28 of 28 checks.
- `verification/B-09_persistence_table.md`: `Fixtures/projects/camera_project.json` is read and
  written back byte for byte, and the six files without a camera still have none, which is step
  9 by machine. 99 of 99 checks.
- `verification/B-12b_diagnostic_catalogue.md`: `CAMERA_PLANE_BEHIND`, the sentence step 7 shows.
- **Not** checked by a test: whether the parallax in step 4 looks like parallax, whether the
  Camera block is findable, and whether 50 mm means anything to the person reading it. Steps 1
  to 9 are the check.
- New capabilities the owner may cut: the Depth row, the Camera block, and the
  `layer.set_depth` and `camera.set_property` commands behind them. Cutting them leaves the
  projection in the core with no way to reach it from the window, and leaves the fixture table
  as the only thing that exercises it.

## What this build does not do

**Nothing here keys a depth or a camera.** The boxes set a value and that is all: there is no
stopwatch on either, so a camera move cannot yet be animated from the window. The renderer
follows a keyed camera - `Fixtures/projects/camera_project.json` has one and the fixture table
walks it - so what is missing is the gesture, not the arithmetic. This is the largest gap in
the entry and it is the obvious thing to ask for next.

**Half of that was answered on 2026-09-16.** The owner performed this sheet on 2026-09-15 and
asked for it, so B-13d made a depth key: the Depth row is a blue number that drags and keys like
every other value in the window, and it has its own track on the timeline. Steps 1 to 9 above are
unchanged and still worth walking - none of them was about keying - but the paragraph above is
now only true of the camera. The depth half is `verification/B-13d_depth_keys_playtest.md`.

**The camera is placed by typing, not by dragging it in the picture.** W-04 asks for a camera
move and not for a handle to make it with.

**A camera is not a layer.** One camera per composition, with no in and out points and nothing
to parent it to. D-58 names that refusal and what it costs: the standard After Effects rig of
several cameras and a null object is a later contract if a playtest asks for one.

**A camera that tilts or turns** is excluded by D-56 and would need a real perspective
transform rather than the affine one this is built on. So is depth of field, and so are planes
that intersect.

**A layer cannot opt out of the camera.** After Effects has a 3D switch and a 2D layer ignores
the camera entirely, which is genuinely useful for something pinned in front. Here every layer
has a depth and it defaults to 0. D-58 lists this as undecided, and a playtest is the right
place to find out whether it is wanted.
