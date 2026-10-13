# B-515: the wireframe views of Wave World and Shatter

Built on 2026-10-12 as D-635, part of the P0-17 group (D-600, your plan: "how AE treats
them/are done"), approved by the coordinator for your /loop request. Both are our versions of
After Effects' wireframe views: quick line drawings to help you set an effect up. They never
change the water or the pieces; switching back to the normal view gives exactly the picture
you had before.

**Wave World**, the **View** menu:

- **Height Map** (as before): the grey map.
- **Wireframe Preview**: the water drawn as a white grid of lines, seen from above at an
  angle, with producer 1 marked by a small red cross and producer 2 by a blue one (a producer
  with Amplitude 0 has no mark). Dry land leaves a hole in the grid.
- Three new **Wireframe Controls**, used only by the preview: **Horizontal Rotation** (turns
  the grid round, 0 as added), **Vertical Rotation** (how high above the water you look from:
  90 straight down, 30 as added) and **Vertical Scale** (how tall the waves are drawn, 1 as
  added).

**Shatter**, the **View** menu gains four choices:

- **Wireframe Front View**: the outline of every piece in white, seen straight on, ignoring
  the camera.
- **Wireframe**: the same outlines seen through the effect's camera.
- **Wireframe Front View + Forces** and **Wireframe + Forces**: the same, plus the edge of
  each force's sphere, force 1 in red and force 2 in blue.

Every piece shows through the others, so you can see pieces behind pieces. Adobe gives no
formula, so the way the lines are drawn, the colours and the settings as added are ours; Wave
World's preview has no perspective (far parts are not drawn smaller). Projects saved before
this open unchanged.

The check, `verification/D-635_wireframes_table.md` (90 of 90), holds every pixel to numbers
worked out by separate programs before the code existed. The pictures are in
`verification/D-635 pictures/`, saved over black so the white lines show (in the app the lines
are drawn on a clear layer).

## What to check

1. **Wave World as added.** `1_wave_world_wireframe_as_added.png`: a grid seen from 30
   degrees above, ripples rising from the red cross in the middle.
2. **Turned, with two producers.** `2_wave_world_wireframe_turned_two_producers.png`: the grid
   turned 30 degrees and seen from higher, the waves drawn three times as tall; a red and a
   blue cross, the blue one making faster ripples.
3. **An island.** `3_wave_world_wireframe_island_shore.png`: a big wave on the left; on the
   right the dry land is a hole in the grid.
4. **Shatter, Wireframe Front View.** `4_shatter_wireframe_front.png`: the brick outlines,
   the middle ones blown away and a few still flying.
5. **Shatter, Wireframe + Forces through a turned camera.**
   `5_shatter_wireframe_forces_turned_camera.png`: the pieces' outlines seen from the side, a
   big red circle for force 1 and a smaller blue one for force 2.
6. **In the app.** On Wave World, set View to Wireframe Preview and move Vertical Rotation and
   Horizontal Rotation: the grid should tilt and turn. Set it back to Height Map: the same map
   as before. On Shatter, try each of the four wireframe views and drag Force 1's Radius: the
   red circle should grow.
7. **Saved and opened again.** Save, close and open the project: the same views and controls.
   An older project with Wave World should open looking exactly as before.

Our own choices you may want to judge: the 30-degree tilt as added, white lines one pixel
wide, and the red and blue marks.

## Speed

Not timed: the views are drawn in place of the picture, and the water or the pieces under them
are worked out exactly as before (see the Wave World and Shatter timing tables).
