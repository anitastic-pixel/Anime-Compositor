# B-800 playtest: a layer's fourteen more blend modes (D-921)

What changed: a layer can now be put in Color Burn, Classic Color Burn, Linear Burn, Darker Color,
Color Dodge, Classic Color Dodge, Lighter Color, Linear Light, Vivid Light, Pin Light, Hard Mix,
Exclusion, Subtract or Divide, as in After Effects. Add is now shown as "Add (Linear Dodge)", After
Effects' other name for it; files still save it as add, so old projects open unchanged. The
sixteen modes there before mix exactly as before. A file with Dissolve is still refused, with a
reason that lists all thirty modes (D-920).

## Steps

1. Open the program and make a composition with two layers: a picture or video at the bottom and
   a second, different picture above it.
2. On the top layer's row in the timeline, open the blend mode menu (it says "Normal").
3. Check the menu lists, in this order: Normal, Darken, Multiply, Color Burn, Classic Color Burn,
   Linear Burn, Darker Color, Add (Linear Dodge), Lighten, Screen, Color Dodge, Classic Color
   Dodge, Lighter Color, Overlay, Soft Light, Hard Light, Linear Light, Vivid Light, Pin Light,
   Hard Mix, Difference, Exclusion, Subtract, Divide, Hue, Saturation, Color, Luminosity,
   Stencil Alpha, Stencil Luma.
4. Pick each of the fourteen new ones in turn and look at the viewer.
5. Save the project, close it, open it again: the top layer still shows the mode you picked.
6. Open an older project that uses Add: its menu now says Add (Linear Dodge) and it looks the same.

## What to see

- Color Burn / Linear Burn: darker than Multiply, Color Burn with more contrast and colour.
  Classic Color Burn: the same, except where the top is pure black the bottom shows through.
- Color Dodge / Linear Dodge (Add): brighter than Screen; Color Dodge blows bright parts out to
  white. Classic Color Dodge: the same, except where the top is pure white.
- Darker Color / Lighter Color: whole colours, not channel by channel: each pixel is either the
  top's or the bottom's colour, whichever is darker (or lighter).
- Linear Light, Vivid Light, Pin Light: like Hard Light, a 50% grey top changes nothing; light
  parts brighten, dark parts darken, each more strongly (Pin Light swaps in whichever is more
  extreme). Hard Mix: every channel turns fully on or off, a poster of eight colours.
- Exclusion: like Difference but softer. Subtract: the top taken away from the bottom (white
  makes black). Divide: the bottom divided by the top (white changes nothing, darker tops brighten).
- With a graphics card, every new mode but Hard Mix draws on the card. Hard Mix is drawn by the
  processor (the status says so): its all-or-nothing edge cannot be matched on the card exactly.

## Pictures

`verification/B-800 pictures/v1_dodgeburn_sheet.jpg`: wave 12's anime filter (w12b, video 1):
the frame with no filter (top left), the stand-in before this change with Multiply and Screen
(top right), the same filter with the tutorial's Color Burn and Color Dodge (bottom left), and
the tutorial's finished frame (bottom right). With the real modes the light and the yellow
warmth come closer to the tutorial's.

## Fixtures

`verification/B-800_more_blend_modes_table.md`: 15 of 15 (FX-LBLEND-010 to 023 and 009).
