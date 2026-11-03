# B-870 playtest: the Text Animator's Blur (D-990)

What changed: the Text Animator now has a Blur setting, as After Effects' animator does. Each letter the range selector picks is blurred by it, so a title can come in blurred and sharpen letter by letter. The tutorial oTvDbIZsQoE does this at 11:12 to 11:45 with its "CINEMATIC" title.

The picture `verification/B-870 pictures/text_blur_tutorial_before_after.png` shows the tutorial's frame on top and ours below with the same settings.

## Steps

1. Make a text layer and type a word, for example CINEMATIC.
2. With the layer selected, add Text Animator from the Effects window.
3. In Effect Controls, set Tracking Amount to 50 and Blur to 37 and 37.
   Every letter should look soft and spread out.
4. Put the playhead at frame 0, key Start at 0. Move one second on, key Start at 100.
5. Play it. The letters sharpen one by one from left to right, and the spacing closes up with them.
6. Set Blur to 37 and 0. The letters smear sideways only, not up and down.
7. Set Blur back to 0 and 0: the text looks exactly as it did before Blur existed.
8. Save, close and reopen the project: the Blur numbers are still there.

## Not right yet

- The text's drop shadow is cast by the sharp letters, not the blurred ones.
- How strong a given Blur number looks is our best match to After Effects (half the Blur as a Gaussian blur radius); Adobe publishes no formula, so it may look a little stronger or weaker than theirs.
- Our font is not the tutorial's Gotham Extra Light, so the letters look heavier in the picture.
- Randomize Order (letters in a random order) is not built; the tutorial does not use it.

## Checks behind it

- `b870_text_animator_blur`: 13 of 13, `verification/B-870_text_animator_blur_table.md`, pictures in `verification/B-870 pictures/`.
