# B-611 playtest: open masks (D-737)

## What to try

1. Pick a picture layer and press **G** for the pen. Click three or more points. The status line says: "Click the first point to close the mask, Enter to leave it open, or Escape to start again."
2. Press **Enter**. The mask stays open: no line runs from the last point back to the first. Its mode is None and it cuts nothing from the layer, so the picture does not change.
3. Two points and **Enter** also work: an open mask can be a single straight line.
4. Clicking the first point instead closes the mask, as before: mode Add, and it cuts the layer.
5. Add **Path Stroke** to the layer. On an open mask it draws along the legs only, not across the gap.
6. On the timeline, open the layer's Masks group. Each mask's row now has a **Closed** / **Open** switch next to its mode. Click it to open or close the mask. While it is open, the mode list is greyed out at None. Closing it sets the mode to Add.
7. An open mask can't be given a mode. If something asks for one, a sentence says why: "Mask "Mask 1" is open, so its mode is None: it encloses nothing to add or take away. Close the mask first, then set its mode."

## What the check found (private copy, a 320 by 200 copy of `Fixtures/open_masks/fx_msko_002.json`): 17 of 17 pass

- The fixture opens with one closed mask of mode None, and its timeline row has the Closed switch.
- Clicking it opened the mask with mode None, greyed the mode list, and changed the switch to "Open" (click to close). Path Stroke stopped drawing the bottom piece.
- Asking for Add on the open mask was refused with the sentence above, and nothing changed.
- Closing it again gave it Add. Two undos brought back the closed mask of mode None, and the same pixels.
- **G** took the pen. Three clicks and **Enter** added Mask 2: open, mode None, three points. The frame was exactly as before, pixel for pixel.
- Two clicks and **Enter** added Mask 3, an open line of two points.
- Three clicks, then a click on the first point, added Mask 4: closed, mode Add.
- No page errors.

Fixture check `tests/b611_open_masks.rs`: 29 of 29 (see `B-611_open_masks_table.md`). FX-MSKO-001 to 010 match `tools/open_mask_reference.py` within 2e-5. Seven files keep exactly what they held when opened and saved. The two broken files are refused, naming `closed`. Opening and closing a mask is one undo step each. An open mask of mode Add is refused. An open mask of two points is accepted, but a closed one of two points is refused. An open mask whose legs cross is accepted.

## Pictures (`B-611 pictures/`)

- `all.png`: the first four pictures below on one sheet.
- `00_closed_roof.png`: the closed roof. Path Stroke draws all three sides, the bottom too.
- `01_opened_roof.png`: the same mask opened with the switch. The two legs are drawn and the bottom is gone.
- `02_closed_again_add.png`: closed again, so its mode is Add and it cuts the layer to the roof.
- `05_pen_closed_add.png`: after the pen added two open masks and one closed one. The open ones cut nothing.
- `06_window_open_masks.jpg`: the window. Mask 2, drawn with the pen and left open, is picked, with no line back to its first point. The timeline rows show Mask 2 and Mask 3 switched to **Open**, with greyed None modes.

## How this differs from After Effects

- In After Effects, the pen leaves a mask open when you switch tools. Here, **Enter** leaves it open (as it already did for shapes), and Escape still throws the unfinished path away.
- An open mask is always mode None, so it cuts nothing; to cut with it, close it first.
- Effects that fill an area (not the ones that draw along a path) treat an open mask as closed.
- An older build that opens a file with an open mask keeps the setting. It draws Path Stroke along it closed, and warns about a two-point mask.
