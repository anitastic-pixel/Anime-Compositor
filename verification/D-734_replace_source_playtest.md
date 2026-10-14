# D-734 playtest: replace layer source

What it is: After Effects' Replace Layer Source (Ctrl+Alt+/). A layer starts showing a different picture, sequence or sound, and keeps everything else: its name, keys, effects, masks, position, timing and Sheet.

## Steps

1. Open the reference shot (or any project with two drawing layers whose drawings differ).
2. On layer 1, add an effect (for example Tint) and move it a little so it has a position you can recognise.
3. Click layer 1 in the timeline.
4. In the Project panel, click the drawings of another layer (for example layer2).
5. Press **Ctrl+Alt+/**.
   - Layer 1 now shows layer2's drawings. Its name, the Tint, its position and its bar on the timeline are unchanged.
   - The status line says "Replace the layer's source with layer2".
6. Press **Ctrl+Z** once.
   - Layer 1 shows its own drawings again, exactly as before.
7. Do it again from the menu: **Layer > Replace layer source**. Same result. Ctrl+Shift+P, type "replace": the line is there with Ctrl+Alt+/.
8. Things it refuses (each says why in the status line, and nothing changes):
   - with nothing picked in the Project panel;
   - picking the drawings the layer already shows;
   - a sound file onto a drawing layer;
   - a solid, shape, text or adjustment layer;
   - a locked layer.

## Pass if

The picture changes, nothing else about the layer does, and one Ctrl+Z puts it back.

If the new drawings do not have a number the Sheet uses, that frame is reported as a missing drawing, as it would be anywhere else.
