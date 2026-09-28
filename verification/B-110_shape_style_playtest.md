# B-110b: shape styles over time, and line joins and caps, by hand

Built on 2026-09-27 against D-170, which is **proposed** and waits for the owner. Playing this
sheet is how to judge it: if what you see here is what you want from keyed colours and widths
and from After Effects' joins and caps, accepting D-170 and passing this sheet are the same
answer.

The generated halves are `verification/B-110_shape_style_table.md`, which checks the pixels
against FX-SHP-100 to 127, and `verification/B-110b_panel_table.md`, which checks what the panel
sends and what comes back. The pictures the fixtures were worked from are in
`verification/B-110a proposal/`; `joins_and_caps_enlarged.png` shows the three joins across and
the three caps down. This sheet covers what none of them can judge: whether it looks right on
the picture and whether the panel is usable.

## Before you start

Open any project with a composition, or use **Save As...** first and work on the copy. Press
**New shape layer**, then **G** and click four points across the picture in a zigzag, with
sharp turns, to draw an open line. Under the shape, press **Add stroke**.

## What to check

1. **Nothing changed for an old stroke.** Under **Stroke** there are now rows **Shape 1 stroke
   color**, **Shape 1 stroke opacity** and **Shape 1 stroke width** (called Stroke color and so
   on below), each with a diamond, and a **Corners** line reading
   **join round**, **cap round** and **mitre limit 4**. The line looks as a stroke always has.
2. **Width over time.** Set **Stroke width** to 10 and press its diamond at frame 0. Go to frame
   24 and set it to 40. Play: the line thickens smoothly. The keys show on the timeline, drag,
   take F9 and open in the graph editor as any number's keys do.
3. **Colour over time.** At frame 0 press the **Stroke color** diamond. At frame 24 pick blue in
   the colour box beside **Stroke**. Play: the line turns from white to blue. The three numbers
   under the colour move together; the colour box shows the colour on the frame you are on.
4. **Opacity over time.** Key **Stroke opacity** from 100 at frame 0 to 0 at frame 24. The line
   fades out. Do the same on a shape with a fill: **Fill color** and **Fill opacity** key the
   same way.
5. **Mitred corners.** Choose **join miter**. The corners of the zigzag become sharp points.
   Lower **mitre limit** to 1: every corner is cut flat (bevelled), because any point would
   reach too far. Raise it to 10: they come back to points, except a corner that turns almost
   straight back.
6. **Bevelled corners.** Choose **join bevel**. Every corner is cut flat.
7. **Caps.** Choose **cap butt**: the line ends exactly at its first and last points, flat.
   **cap square**: flat, but half a width beyond them. **cap round**: a half circle beyond them.
8. **With a trim.** Turn **Trim paths** on and drag **End** to 50. With **cap butt** the cut end
   is flat; with **cap round** it is round. On a closed rectangle (**Q**), mitred, animate
   **Offset** from 0 to 360: where the piece of outline passes the rectangle's first corner, the
   corner stays sharp rather than breaking.
9. **Back to round.** Choose **join round**, **cap round** and set the limit to 4. It looks as it
   did in step 1.
10. **No stroke.** Press **No stroke**: the stroke and its keys go. **Add stroke** gives a plain
    stroke without the old keys; **Ctrl+Z** twice brings the keyed stroke back.
11. **Saved and reopened.** Save, then **Open** the same file. The keys, the join, cap and limit
    are as they were, and nothing is reported in the diagnostics.
12. **Export.** Export the range as PNG. The frames show the thickening, fading, colour change
    and corners as the viewer showed them.

## A choice for you

A new stroke starts round at its corners and ends, as every stroke here always has. After
Effects starts a new stroke mitred with butt caps. Say which you want; either is one line.

## Known limits

- D-170's "Not covered" list: no dashes; no tapering along the line; a curve is drawn as many
  short straight pieces, and on a mitred or bevelled curve the joins between those pieces
  differ from the true curve by far less than a pixel.
- There is no expression on a shape's colour, opacity or width; the diamond offers keys only.
- The join, cap and mitre limit are one setting for the whole stroke, not keyed.

## Result

Not yet played.
