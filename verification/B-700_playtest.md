# B-700 playtest: an expression runs on an effect's colour and point, and reads any effect (D-820)

What changed: an expression on an effect's colour or point setting (for example a Fill's Color
reading a Color Control) used to be kept but not run; the setting drew its typed value and every
frame said EXPRESSION_TYPE (B-670). Now the expression runs. An expression can also read any
effect's setting, on the same layer or another, not only the six Expression Controls: for
example a Gaussian Blur's amount can read a Slider Control or another layer's Directional Blur.

## What the pictures show

Each sheet in `B-700 pictures/` is frame 0 of a small project, drawn by this build twice: once
with the expression switched off (what the build before drew: the typed value), and once with it
on.

- `p1_fill_before_after.png`: a Fill whose typed colour is blue and whose expression reads an
  orange Color Control on the same layer. Before: blue. After: orange.
- `p2_circle_before_after.png`: a white Circle whose typed centre is the middle and whose
  expression reads the position of a small red square, another layer. Before: the disc in the
  middle. After: the disc centred on the red square.
- `p3_blur_before_after.png`: a white square layer with a Gaussian Blur whose typed amount is 0.5
  and whose expression reads a Slider Control set to 8. Before: a sharp square (the expression
  read another effect, which used to be refused). After: a soft square. The third picture is the
  blur typed as 8, which matches the second exactly.
- `neon_after.png`: lane w7e's replay of Smertimba's soft neon glow, seven Drop Shadows whose
  Shadow Color reads one Color Control. Left: the Color Control holds the magenta the shadows
  were typed with. Right: only the Color Control changed, to cyan, and all seven shadows follow.
  Neither frame says EXPRESSION_TYPE any more. (The two shadows at softness 1000 and 2000 are
  still bypassed with EFFECT_PARAMETER_INVALID; that is another lane's item.)

## Try it yourself (5 minutes)

The window has no control yet for writing an expression on a colour, so this needs a project
that already carries one: the projects above are in this session's scratch folder
(`wave8/w8e/proofs/p1_fill.json`, `p2_circle.json`, `p3_blur.json`, `neon_cyan_control.json`).

1. Open `p1_fill.json`.
   - Expected: the whole frame is orange, and there is no EXPRESSION_TYPE beside the frame.
2. Open `p2_circle.json` and move the small red square with the selection tool.
   - Expected: the white disc follows the square.
3. Open `p3_blur.json` and change the Slider Control's Slider in Effect Controls.
   - Expected: the white square gets softer as the slider goes up, sharp at 0.
4. Save `p1_fill.json` under a new name, close it and open the copy.
   - Expected: it opens the same; the expression text is still in the file.

Pass if every "Expected" holds. The numbers are in `B-700_effect_setting_expressions_table.md`
(17 of 17).

Still held, not part of this fix: an expression on a choice from a list (Gaussian Blur's Units,
Fill's All Masks) still says EXPRESSION_TYPE, because this build's lists are not After Effects'
menus item for item. Gaussian Blur's After Effects name "Blurriness" is not taken; use
`"sigma_px"` or `1`.
