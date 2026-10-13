# B-570: Liquify's other brushes, Freeze and Thaw, many strokes, brushing on the viewer

Built on 2026-10-13 as D-700, closing the gaps B-484 (D-605) left on Liquify. Liquify now has
all of After Effects' brushes: D-605's six, and Turbulence, Reflection, Clone, Reconstruction
(with its Revert, Displace, Amplitwist and Affine modes), Freeze Area and Thaw Area. A mask can
hold an area still too (Freeze Area Mask). Strokes brushed before the current one are kept, each
with its own brush, and a stroke can be painted on the viewer instead of typed. Adobe does not
publish how the brushes are worked out, so they are this program's own.

The check, `verification/D-700_liquify_brushes_table.md` (185 of 185), holds every pixel to
numbers a separate program worked out before the code was written, and holds the graphics card's
picture to the processor's (within 1 level). D-605's own check still passes. The pictures are in
`verification/D-700 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect.
2. **`2_warp.png`**: the Warp brush stroked across the middle, as D-605 drew it.
3. **`3_turbulence.png`**: Turbulence across the middle: the street under the stroke stirred
   into smooth wobbles.
4. **`4_reflection.png`**: Reflection across the middle, going right: the street on one side of
   the stroke mirrored across it.
5. **`5_clone.png`**: `2_warp.png`'s push, then one Clone dab low on the street copying from 80
   pixels above it: the push's bend copied down there.
6. **`6_revert.png`**: `2_warp.png`'s push, then one Reconstruction (Revert) dab in the middle:
   the push undone there, the rest still pushed.
7. **`7_freeze_then_warp.png`**: a Freeze Area dab in the middle, then `2_warp.png`'s push: the
   middle held still, the push either side of it.
8. **`8_freeze_mask.png`**: `2_warp.png`'s push with Freeze Area Mask 1, a box about the middle:
   inside the box nothing moves.
9. **In the app, on the viewer.** Add Liquify to a layer and press **Brush on the viewer** under
   its settings. Drag across the picture: when you let go the stroke is drawn. Choose another
   tool (say Twirl Clockwise) and drag again: the first stroke stays as it was and the new one is
   added. Escape during a drag drops that stroke. **Stop brushing** ends it; **Clear strokes**
   takes every stroke away. One stroke is one step of undo.
10. **In the app, Freeze.** Pick Freeze Area, brush a spot, then pick Warp and drag through it:
    the frozen spot does not move. Thaw Area over it lets it move again. Or draw a mask, set
    Freeze Area Mask to its number, and brush: inside the mask nothing moves. A Freeze Area Mask
    that names no mask leaves the layer as it is and says so in the warnings.

Not like After Effects: the strokes are kept as numbers (Earlier Strokes, which you can also
type); each brushed stroke keeps the settings it was brushed with, and the Stroke box is still
drawn with the settings shown; the frozen areas are not shown as a red overlay on the viewer, and
the stroke is not drawn on the viewer while you drag, only when you let go.

## Speed

`verification/B-570_liquify_timing_table.md`, measured while other work was running, so the numbers are
PROVISIONAL. With three busy layers, a Turbulence stroke or a frozen Warp stroke cost about 16 ms a
layer on the graphics card. Four strokes at once ran slower on the card than on the processor (about
860 against 215 ms a frame); that is unexplained and will be measured again on a quiet machine.
