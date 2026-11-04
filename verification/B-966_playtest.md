# B-966: Create Shapes from Text and Create Masks from Text

Built on 2026-11-03 as D-1086 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). The tutorial is
Nc2w1Kt3XjE at 1:15 to 1:58: a text layer, Layer > Create > Create Shapes from Text, and a new
shape layer "OOOOOOO Outlines" appears with each letter a group holding its paths and a Merge Paths.

Two new commands, both for a text layer:

- **Create shapes from text** makes a shape layer "<name> Outlines" just in front of the text
  layer. Each letter is a group named after the letter, holding Path 1, Path 2 ... (one per
  outline, so O has two: the outside and the hole), a Merge Paths set to Merge, a Fill in the
  letter's colour, and a Stroke if the words had one.
- **Create masks from text** makes a solid "<name> Outlines", the size of the composition and in
  the words' colour, with one mask per outline, each named after its letter. Outsides are Add,
  holes (the middle of O, the two in 8 and in B) are Subtract.

Either way the new layer takes the text layer's place, times, position, turn and parent, and the
text layer is switched off (not deleted), as in After Effects. The outlines are the letters as they
are drawn on the playhead's frame, so a Text Animator's turn, move and colour are kept. One Undo
takes it all back.

Where to find them: right-click a text layer in the timeline; the **Layer** menu at the top; or the
command finder (type "shapes from text" or "masks from text"). On a layer that is not text they are
greyed out in the right-click menu, and the finder says the layer has no outlines. Words that are
only spaces are refused with a message.

The check, `verification/B-966_text_outlines_table.md` (45 of 45), holds every letter's points and
handles, which outlines are holes, the masks' order and modes, the groups, and that the new layer
draws the same picture as the words did, to numbers worked out by a separate program
(`tools/text_outlines_reference.py`) before the code existed.

## What to check

1. **The sheet.** `verification/B-966 pictures/1_text_shapes_masks.png`: at the top the tutorial's
   result at 1:58. Below, our app: the text layer, then the shape layer, then the masked solid,
   for "O8B" (the holes must stay open) and for "Spin" with a Text Animator (turned 30, moved, red).
   The three pictures in each row should be the same shape; the masked solid is one colour, the
   words' own (white for "Spin", whose red came from the Text Animator). The blue squares are the path points the
   separate program expects; the yellow lines are Add masks and the red ones Subtract.
2. **In the app.** Make a text layer, right-click it, Create shapes from text. A new layer appears
   above it, the words look the same, and the text layer's eye is off. Twirl the new layer open:
   one group per letter. Move one group's Position: just that letter moves.
3. **Masks.** Undo, then Create masks from text. Open the new solid's masks: one per outline,
   named after the letters, the holes set to Subtract.

Known differences: a Stroke is drawn over the Fill on the shape layer (After Effects' default puts
the Fill over the Stroke for text with "Fill Over Stroke"); a text background box and shadow are
not carried over to the new layer (the record says so); faux bold is not in the outlines.
