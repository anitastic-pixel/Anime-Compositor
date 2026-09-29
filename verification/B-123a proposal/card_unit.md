# B-123a: the batch's five new effects on the graphics card

This is item 9 of the nine you accepted on 2026-09-28 ("1, 2,3,5,6,7,8,9", then "include 4 as well,
just 1 thru 9"). Items 4 to 8 added five effects that only the processor draws: Color Lookup,
Line Blur, HSV Key, Paraffin and Kira-kira. This moves all five to the graphics card in one step,
as Motion Tile moved in item 1 (D-178).

## What changes

When one of the five is the last effect on a drawn layer, the preview's card draws it, as it
already does for the other sixty-odd effects. Nothing else changes:

- Exports, renders to file and the fixtures stay on the processor, byte-exact. The processor stays
  the authority (ADR-006).
- A frame the card cannot draw (too big, no double precision) is drawn by the processor, and says
  so, as before.
- No setting, file format or default changes. A project looks the same; it draws faster in the
  viewer.

## How each is done on the card

| Effect | On the card | What stays on the processor |
|---|---|---|
| Color Lookup | the colour-effects pass, the table sent with the settings | reading the .cube file |
| Line Blur | one pass: the direction and the blur along it, per pixel | nothing |
| HSV Key | the colour-effects pass, on the same 8-bit steps the processor uses | nothing |
| Paraffin | the colour-effects pass | finding the figure's near and far side, one quick scan |
| Kira-kira | two passes: each star lights its patch, then the light is laid on | finding the highlights and the stars, one quick scan |

## The measure

The same as every earlier card unit (D-100, D-165, D-178): every fixture frame of the five, and the
reference shot with each on three layers (one after a Drop Shadow, so the effect starts from a
corner that is not the picture's), at Full and Draft, drawn by the processor and by the card.
**No channel of any pixel may differ by more than 1 level of 255.** Both must give the same
warnings. The table and pictures of the worst frame come with the build.

## If refused

The five stay on the processor, and nothing else moves.
