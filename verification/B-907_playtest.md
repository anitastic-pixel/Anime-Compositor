# B-907 playtest: Ctrl+D copies a chosen mask

What changed (D-1028): with a mask chosen, **Ctrl+D** copies that mask, not the layer. The copy
has everything the mask had (its shape and path
keys, mode, feather, opacity, expansion, inverted), sits just below it in the timeline, and is
named the next free "Mask N". Choosing a layer again makes Ctrl+D copy layers, as before.

1. Add a solid and draw an ellipse mask on it with the Ellipse tool.
   - Expect: "Mask 1" under the layer in the timeline.
2. Set Mask 1 to Subtract and Feather 20, and click its path stopwatch diamond.
3. Click the name "Mask 1" in the timeline, then press **Ctrl+D**.
   - Expect: a "Mask 2" right under Mask 1, also Subtract, Feather 20, with the same key; still
     one layer in the timeline. The copy is the one now marked with the diamond after its name.
4. Drag a point of the copy on the picture.
   - Expect: only the copy moves; the original stays where it was.
5. Press **Ctrl+Z**.
   - Expect: the point moves back; Ctrl+Z again takes the copy away.
6. Click the layer's own name, then press Ctrl+D.
   - Expect: the whole layer is copied, as before.
7. Double-click a mask's outline on the picture, then press Ctrl+D.
   - Expect: that mask is copied.
8. On a shape layer, click a shape's name and press Ctrl+D.
   - Expect: the layer is copied (shapes are not masks; unchanged).

The command is checked on a project in `verification/B-907_mask_duplicate_table.md`.
