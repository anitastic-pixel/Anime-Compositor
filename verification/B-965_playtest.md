# B-965: Path Options, text on a path

Built on 2026-11-03 as D-1085 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). The tutorial is
tQ5wqg10x_I at 2:20 to 2:40: a circle mask drawn on a text layer, then Path Options' Path set to
that mask, and the words run round the circle.

**Path Options** is in the Effects finder under **Text**. Put it on a text layer that has a mask
and the letters are set along the mask instead of on a straight line. The settings (the value when
added in brackets):

- **Path** (1): which of the layer's masks, by number. 0 means none.
- **Reverse Path** (Off): walk the mask the other way round. On a circle this moves the words from
  inside the ring to outside it, as in the tutorial at 2:40.
- **Perpendicular To Path** (On): each letter turns to follow the path. Off keeps them upright.
- **Force Alignment** (Off): On spreads the letters out to fill the path from First Margin to Last
  Margin.
- **First Margin** and **Last Margin** (0): how far along the path, in pixels, the words start (or
  end, for right-aligned words). Both can be keyed, so the words can slide round the path.

The mask stays a mask: with mode None it cuts nothing and only carries the words. If Path names a
mask the layer does not have, the frame says so (EFFECT_PATH_MISSING) and the words are drawn where
they would be without a path.

The ellipse mask tool now starts at the top and goes anticlockwise, as After Effects' does, so the
same First Margin puts the words in the same place as in the tutorial.

The check, `verification/B-965_text_path_table.md` (37 of 37), holds where each letter lands and
how far it is turned to numbers worked out by a separate program (`tools/text_path_reference.py`)
before the code existed.

## What to check

1. **The tutorial, side by side.** `verification/B-965 pictures/1_tutorial_vs_ours.png`: on the
   left the tutorial at 2:30 and 2:40, on the right our app drawing the same settings. Top row:
   Reverse Path off, First Margin 73.6, "CIRCULAR TEXT" inside the ring along the bottom. Bottom
   row: Reverse Path on, First Margin 1146.6, the words outside the ring along the top. The grey
   ring on our side is drawn on the sheet to show where the mask is.
2. **In the app.** Make a text layer, draw an ellipse mask on it, add Text > Path Options. The words
   should jump onto the ring. Turn Reverse Path on: they go outside. Drag First Margin: they slide
   round. Key First Margin at two times and play: they travel round the ring.
3. **Perpendicular To Path off.** The letters stay upright while still following the ring.

Known differences: one Path Options per layer is used; the mask's curves are cut into many short
straight pieces (on a circle a letter is at most 0.17 degrees out); no vertical text on a path.
The tutorial's words are in Helvetica Light; ours are in the font that comes with the program, so
they are a little shorter and sit a little differently round the ring.
