# D-271: a pen's pressure and the eraser's ring in the Sketch workspace

The owner asked for this on 2026-10-03: "show the eraser as a circle of how much it is erasing, pressure sensitivity when using a pen."

## What changed

- **The eraser's ring.** With the eraser chosen, the paper's pointer becomes a circle exactly as wide as the eraser rubs out, at the paper's size on screen. `[` and `]` shrink and grow it. While a pen presses, the circle follows the width that pressure gives. It has a white edge with a dark edge each side, so it shows on light paper and on a dark picture alike. With the brush or pencil chosen, the pointer stays the crosshair.
- **Pen pressure.** A stroke drawn with a pen is thin where the pen presses lightly and full width where it presses hard: from 15% of the chosen size at the lightest touch to 100% at full pressure. This works for the brush, the pencil and the eraser. A mouse stroke is drawn at one width, as before.
- **A pen's eraser end** rubs out whatever tool is chosen, and the tool stays chosen afterwards.
- **Saving.** A pen stroke keeps one pressure from 0 to 1 for each point and saves it as `pressure`. A mouse stroke saves no such line, so projects without pen strokes save byte for byte as before. A file whose pressures do not match its points is refused, saying where; the pressure is never quietly dropped.
- **Unchanged:** sketches still never reach a frame or an export.

## Checks

| Check | Result |
|---|---|
| `tests/d271_sketch_pen.rs`, written to `verification/D-271_sketch_pen_table.md` | 11 of 11 pass |
| `tests/d261_sketch.rs`: exports byte for byte without sketches, and 2297 fixture projects save byte for byte | pass |
| App checks | 88 pass |

The D-261 check already held a stroke with `"pressure": [0.4]`, as a line a later build might add. This build now reads that line and writes it back unchanged.

## In the window

These checks ran in a separate copy of the app with its own browser profile. Pen input was sent as real pen events, with pressure going from light to firm and back.

| Step | Seen |
|---|---|
| Pen wave, pressure 0.05 → 1 → 0.05 | The page got 40 pen events with pressures 0.05 to 1.00. The stroke was saved with 40 points and 40 pressures from 0.05 to 0.999, as one undo step ("Undo Draw in a sketch") |
| The same wave drawn with the mouse | Saved with 40 points and no pressure |
| Eraser chosen, hand over the paper | Ring 21.7 px wide, matching the eraser's 21.7 px on screen. Its centre was at the hand (768.0, 489.1), and the paper's own pointer was hidden |
| `[` once, then twice | Ring 9.8 px, then 3.9 px, each matching the eraser |
| Pen eraser at pressure 0.3 | Saved as an eraser stroke with 20 pressures of 0.3, cutting a narrow gap |
| Brush chosen again | Ring hidden, crosshair back |
| Pen eraser end, with the brush chosen | Drew with the eraser; the tool stayed brush |
| One Ctrl+Z | Took back that one stroke |
| Page errors | none |

In the screenshot at device pixels, the pen line at the ring was 31 px across and the ring was 32.5 px: the same width.

Pictures are in `verification/D-271 pictures/`:
1. `1_pen_top_mouse_bottom.png`: the pen stroke on top tapers from thin to thick and back; the mouse stroke below keeps one width.
2. `2_eraser_ring_close.png`: the ring over the line, enlarged.
3. `3_pen_eraser_light_press.png`: a light-pressure pen eraser cuts a gap narrower than the line.

## Playtest for the owner

1. Press Alt+4 for the Sketch workspace and press E. The pointer over the paper is a circle. Press `]` and `[`: the circle grows and shrinks.
2. Rub out across a line. What disappears should be exactly what the circle covered.
3. With a pen: choose the brush and draw, pressing lightly, then hard, then lightly. The line should go thin, thick, thin.
4. Turn the pen over and rub with its eraser end. It should rub out, and the brush should still be chosen afterwards.
5. Save, close and open the project again. The pen lines should keep their thick and thin parts.

Limit: the strength of a stroke does not change with pressure, only its width. A stroke still gains a point each time the hand moves 4 screen pixels, so a very quick change of pressure between points is not recorded.
