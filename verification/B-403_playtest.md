# B-403: Depth of Field

Built on 2026-10-11 as D-523, under your /loop request: Depth of Field, in the **3D Channel**
group. A 3D program can save, with each picture, how far away every pixel is (a "depth" or "Z"
pass) in an EXR file. This effect uses it to blur the picture as a camera focused at one distance
would: what is at that distance stays sharp, and the further something is from it, the softer.

- **Focal Plane** (0): the distance in focus, in the file's own depth units (whatever the 3D
  program used; Pass Extract shows them).
- **Maximum Radius** (10): the blur, in pixels, of what is furthest out of focus.
- **Focal Plane Thickness** (0): how deep a slab around the focal plane is also sharp.
- **Focal Bias** (50): how fast things leave focus; 100 twice as fast, 0 never.

Adobe does not say how much blur each distance gets, so the rule is ours: at Focal Bias 50,
something twice as far as the focal plane gets half the Maximum Radius, and anything nearer than
half the focal distance gets all of it, as a real lens roughly behaves. You type the focal
distance; in After Effects you would read it off the picture first. Picking it by clicking is not
built (a later decision).

The check, `verification/D-523_depth_of_field_table.md` (221 of 221), holds every pixel to
numbers worked out by a separate program before the code existed, and holds the card to the
processor within 1 level. The pictures are in `verification/D-523 pictures/`, made from the small
render of three balls the depth effects already use (`Fixtures/depth_channel/sample/spheres.exr`).

## What to check

1. **Before.** `before.png`: a red ball near, a blue one further, a yellow one far, on a checked
   floor.
2. **Red in focus.** `focus_red.png`: the red ball sharp; the blue and yellow balls and the far
   floor soft.
3. **Blue in focus.** `focus_blue.png`: the blue ball sharp; the red and yellow balls soft.
4. **Yellow in focus.** `focus_yellow.png`: the yellow ball sharp; the red ball very soft.
5. **Bias 100.** `focus_blue_bias_100.png`: only a thin band of floor by the blue ball sharp.
6. **Thick focus.** `focus_blue_thick.png`: from the red ball's depth to beyond the blue ball
   sharp; the yellow ball a little soft.
7. **In the app.** Import an EXR with a depth pass and add Depth of Field (3D Channel): at first
   everything is soft (focused at 0). Raise Focal Plane: the sharp band moves away from you.
   Key Focal Plane from near to far over a second: the focus pulls.
8. **No depth.** On a PNG or an EXR without a depth, the picture is unchanged and a warning says
   the channel is missing.
9. **Saved and opened again.** Save, close and open the project: the same settings and picture.

Our own choices you may want to judge: how fast things leave focus at Focal Bias 50, and typing
the focal distance until clicking exists.

## Speed

On a 1920 by 1080 EXR at Full a frame took about 100 to 110 milliseconds with Depth of Field at radius 10 to 60, against about 5 without it: above the target of 8 ms added (PROVISIONAL: the machine was busy, the card shared with four other test runs) (`verification/B-403_depth_of_field_timing_table.md`)
