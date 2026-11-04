# B-967: Text Animate In presets (D-1087)

Built on 2026-11-03 as D-1087 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). The tutorials are
jxSV7B5Z7Is at 7:18 (Animation Presets > Text > Animate In, Drop In By Character) and
Bw3aQf5ZkX4 at 11:30 (Straight In By Character): pick a preset, the words come in letter by letter.

Four more built-in animation presets, after D-739's twelve (so the folder now holds 16):

- **Drop In By Character**: each letter falls in from above and fades up, one after another.
- **Straight In By Character**: each letter slides in from the right and fades up.
- **Fade Up Characters**: each letter fades up in place.
- **Fade Up Words**: each word fades up in place.

Each is one Text Animator whose range slides across the words over 30 frames, starting at the playhead. They are for text layers; on another layer the Text Animator draws
nothing.

The check, `verification/B-608_animation_preset_table.md` (150 of 150), applies each new preset to a
text layer at frame 10 and checks the saved layer holds the keys [10, -100] and [40, 100], the
move, the fade, the range and the panel's sentence, to numbers worked out by a separate program
(`tools/animation_preset_reference.py`) before the code existed.

## What to check

1. **The sheet.** `verification/B-967 pictures/1_animate_in.png`: at the top the tutorial's
   Animate In folder. Below, our app drawing "TUTORIAL" with each preset at frames 0, 6, 12, 18,
   24 and 30. At frame 0 nothing shows; letters (or, in the last row, words) arrive left to right;
   at frame 30 every row reads normally and nothing moves after.
2. **In the app.** Make a text layer, put the playhead at 0, open the Effects panel's
   **Animation presets** folder and double-click **Drop In By Character**. Play: the letters drop
   in one by one. Press Ctrl+Z once: it all comes off.
3. **Change it.** Apply a preset, open the Text Animator in Effect Controls, and drag the second
   Offset key later: the letters come in more slowly.

Known differences: After Effects' presets also randomise or ease some of these; ours are linear
and in order. The timing (30 frames) is ours; the tutorials show the look, not the numbers.
