# B-887 playtest: Color Grade's key tools and Show Key colours (D-1007)

What it is: in the Lumetri tutorial, a colour is picked for HSL Secondary by clicking it in the
picture. You can then widen the pick with Add and trim it with Remove, and see what is picked with
Show Mask set to Color/Gray. Now Color Grade can do the same.

## Steps

1. Open a project with a colourful picture, such as a street or a face, and select its layer.
2. Effect > Color Grade. Open its HSL Secondary numbers in Effect Controls.
3. Under Key Softness there is a new strip: **Set**, **Add** and **Remove** buttons, six coloured
   dots, and three bars (hue, saturation, lightness).
4. Set **Show Key** to **Color/Gray**. The picture turns grey everywhere, because the key takes
   every colour as added.
5. Set Show Key back to **Off**. Click **Set**, then click a skin tone or a red brick in the viewer.
6. Set Show Key to **Color/Gray** again.
7. Set Show Key to Off. Click **Add**, then click a slightly different shade of the same colour.
   Then click **Remove** and click a colour you do not want.
8. Click the **blue** dot.
9. Try **Color/Black** and **White/Black**.
10. Press Ctrl+Z after a Set or a dot.

## What you should see

- After step 5 the Key Hue and the saturation and lightness numbers change, and the hue bar has a
  short white band, with grey either side, at the colour you clicked.
- After step 6 only that colour and its near neighbours keep their colour. Everything else is
  mid grey.
- After step 7 Add makes more of the picture keep its colour, and Remove makes less of it.
- After step 8 only the blues keep their colour in Color/Gray.
- After step 9 Color/Black shows the picked colours on black, and White/Black shows the key in
  white and black as before. The correction and the vignette never show in these views.
- After step 10 one undo puts back the key as it was before that click.
- Pick with Show Key off: the eyedropper samples the picture as you see it.

## Pictures

`verification/D-1007 pictures/`: the street with a warm key (hue 20, range 30, softness 15).

- `1_before.png`: the street with no effect.
- `2_white_black.png`: the key as white and black.
- `3_color_gray.png`: the warm bricks in their own colour, the rest mid grey.
- `4_color_black.png`: the same on black.

`verification/B-887 pictures/key_strip_panel.png`: the panel in a private copy of the app, after
the scripted page check below.

## Fixtures

- `verification/D-1007_color_grade_5_table.md`: every row "yes". It covers FX-GRADE5-001 to 009,
  saving and opening, a refused word, the commands, the tiles, the card against the processor
  and the pictures.
- `verification/B-887_key_tools_page_table.md`: every row "yes". It covers the 15 key-tool cases
  run through the page's own code, the red dot, Set from the picture, and Show Key's four
  choices.
- FX-GRADE-, FX-GRADE3- and FX-GRADE4- (`verification/D-571_color_grade_3_table.md`,
  `verification/D-637_color_grade_4_table.md`) still pass.
