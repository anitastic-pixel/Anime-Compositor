# B-571: Mesh Warp's vertex handles, and the mesh on the viewer

Built on 2026-10-13 as D-701, closing the gaps B-482 (D-603) left on Mesh Warp. Each vertex of
the mesh now has four Bezier handles (right, left, down and up). Moving a handle bends the cells
next to it without moving the vertex. The vertices and handles can be dragged on the viewer
instead of typed. Adobe does not publish how the mesh is worked out, so the rule is this
program's own.

The check, `verification/D-701_mesh_handles_table.md` (96 of 96), holds every pixel to numbers a
separate program worked out before the code was written, and holds the graphics card's picture to
the processor's (within 1 level). D-603's own check still passes: with no handle moved, every
frame is as it was. The pictures are in `verification/D-701 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect.
2. **`2_handle_lifted.png`**: no vertex moved, only the middle vertex's right handle lifted:
   the street right of the middle bends up, and the picture's outline stays where it was.
3. **`3_top_sags.png`**: a 1 by 1 mesh with the top edge's two handles pulled down: the top edge
   sags in a curve, deepest in the middle, clear above it, the corners where they were.
4. **`4_s_curve.png`** against **`5_bow_without_handles.png`**: the same middle vertex pushed
   right in both. In 5 the street bows; in 4 the handles are turned too, so the bend becomes an S.
5. **In the app, on the viewer.** Add Mesh Warp to a layer and press **Edit the mesh on the
   viewer** under its settings. The grid shows on the picture: a small square at each vertex and
   a round knob on a short arm for each handle. Drag a square: when you let go, the picture bends
   to follow. Drag a knob: the cells next to that vertex curve. Escape during a drag puts it back.
   One drag is one step of undo. **Reset handles** puts every handle back where it started.
   **Stop editing the mesh** hides the grid.

Not like After Effects: the grid on the viewer is drawn with straight lines between the vertices,
not following the curves; the handles of the outside edges that would point off the mesh are not
shown; and if Distortion Mesh has keys, a drag on the viewer changes its base value, not a key.

## Speed

`verification/B-571_mesh_handles_timing_table.md`, measured while other work was running, so the
numbers are PROVISIONAL. With three busy layers, moving handles cost about 3 to 4 ms a 1080p layer
on the graphics card and about 8 to 11 ms on the processor.
