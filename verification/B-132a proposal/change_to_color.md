# B-132a: Change to Color, one colour turned into another with its shades (D-197)

Written on 2026-09-29, before any code. The seventh of the After Effects picks, A7, accepted with the rest by your "take everything". It does what After Effects' Change to Color does, by a rule of our own.

## What you will see

A new effect, **Change to Color**, in the **Color** folder after Leave Color. It turns one colour of a drawing into another, and the shades painted in that colour with it: a red jacket, its shadow tone and its highlight all turn blue, while the skin and a pink ribbon beside them stay as they are. It is how a costume gets a second colour, or a night version, without repainting. Its card, in After Effects' words:

- **From**, the colour to change, #ff0000 when added; pick it from the drawing.
- **To**, the colour it becomes, #0080ff when added.
- **Change**, what of To is taken: **Hue** when added, which keeps each shade's own light and dark; **Hue & Lightness**; **Hue & Saturation**; or **Hue, Lightness & Saturation**, which makes every changed colour To itself, flat.
- **Change By**: **Setting To Color** when added, each changed part set to To's; or **Transforming To Color**, each shifted by as far as To is from From, so a shadow a little off the base colour stays a little off.
- **Hue Tolerance** 5, **Lightness Tolerance** 50 and **Saturation Tolerance** 50, 0 to 100: how far from From a colour may be and still change whole. Hue Tolerance 5 is 9 degrees each way round the colour wheel.
- **Softness** 50, 0 to 100: a band past each tolerance, that part of it wide, over which the change fades out, so nearby colours change partly rather than not at all.
- **View Correction Matte**, off when added: shows instead how much each pixel changes, white for whole, black for not at all, to tune the tolerances by.

The four tolerances and softness can be keyed. Colours are compared by hue, lightness and saturation; a grey has no hue, so a grey From changes nothing. Pixels that do not show stay as they are, and every pixel keeps its own covering, so soft edges stay soft.

`change_to_color.png` in this folder is worked by the rule itself: a figure in a red jacket; the jacket turned blue with Hue, its shadow and highlight kept; the same with Hue, Lightness & Saturation, one flat blue; and the View Correction Matte.

## How it differs from what we already have

**Recolor** (D-87) turns exact colour-model colours into one new colour, flat; Change to Color takes a range and keeps the shading. **Hue/Saturation** turns every colour's hue at once. **Leave Color** (D-141) drains every colour but one; Change to Color changes one and leaves the rest.

## Known limits

- A grey To has no hue: with Hue or Hue & Lightness the changed colours take red's hue. To turn a colour grey, choose Hue & Saturation.
- Lines drawn in the From colour change with it; mask them out or pick a narrower tolerance.

## How you will check it

The build's test draws the nineteen fixture cases and the eight wrong settings and compares every pixel with the numbers `tools/change_to_color_reference.py` worked out, and writes `verification/B-132_change_to_color_table.md`, with pictures. A playtest sheet walks you through recolouring a jacket and tuning the tolerances with the matte.
