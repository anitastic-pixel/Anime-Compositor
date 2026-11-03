# B-906 playtest: Shift and Ctrl while drawing a rectangle or ellipse

What changed (D-1027): while you drag the Rectangle, Rounded Rectangle or Ellipse tool, as a mask
or as a shape, **Shift** makes it a square or circle and **Ctrl** draws it out from where you
pressed, as in After Effects. Both together: a square or circle round the press point. The keys
can be pressed or let go mid-drag. Polygon and Star are unchanged.

1. Add a solid, choose the Ellipse tool and drag on the picture holding nothing.
   - Expect: an oval from corner to corner, as before.
2. Drag again holding **Shift**.
   - Expect: a perfect circle; its size follows the longer way you dragged.
3. Drag holding **Ctrl**.
   - Expect: the press point stays the middle; the oval grows out both ways.
4. Drag holding **Ctrl+Shift**.
   - Expect: a circle centred on the press point.
5. Start a drag with nothing held, press Shift halfway, then let it go.
   - Expect: it snaps to a circle while held and back to an oval when let go.
6. Do steps 2 to 4 with the Rectangle tool, then with no layer chosen (a shape layer).
   - Expect: a square, a rectangle from the middle, a square from the middle; the same as masks.
7. Drag a Star holding Shift.
   - Expect: as before (Shift on stars is not built).

The arithmetic is checked in `verification/B-906_drawing_keys_table.md`.
