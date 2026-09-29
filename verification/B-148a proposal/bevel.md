# B-148a: Bevel Alpha and Bevel Edges, a drawing made to look raised (D-213)

Written on 2026-09-29, before any code. B11 of the second list of After Effects picks, accepted with the rest by your "take everything". B11 said to pick the bevel style After Effects has: After Effects has two bevelling effects of its own, **Bevel Alpha** and **Bevel Edges**, and both are picked, as two effects. Its third, **CC Glass**, a bumpy glass look from a third-party set, is left out. Each does what After Effects' does, by a rule of our own.

## What you will see

Two new effects in the **Stylize** group, after Emboss (After Effects keeps them under Perspective, a group this program does not have). Both make a flat drawing look raised, as if a light from one side catches its edges: the edges facing the light go toward the light's colour, the edges facing away go darker, and the middle is left alone.

- **Bevel Alpha** bevels the edge of what the drawing covers, its own outline: a badge, a title, a cut-out prop.
- **Bevel Edges** bevels the four sides of the layer's rectangle, as a raised tile, button or picture frame.

Their cards, in After Effects' words, the same four settings on each:

- **Edge Thickness**: how wide the bevel is. For Bevel Alpha, 0 to 200 pixels, 2 when added; for Bevel Edges, 0 to 0.5 of the layer's smaller side, 0.1 when added.
- **Light Angle**, in degrees clockwise from up, -60 when added, the light at the upper left.
- **Light Color**, white when added.
- **Light Intensity**, 0 to 1, 0.4 when added.

Edge Thickness, Light Angle and Light Intensity can be keyed; the colour cannot, as with Drop Shadow's. The layer does not grow. A draft looks the same, only smaller.

`bevel.png` in this folder is worked by the rule itself on two made-up drawings: an orange badge with a star and a teal bar beside it, bevelled by Bevel Alpha as it starts, wider and stronger, and with the light from the right; and a title panel of sky, hill and sun, bevelled by Bevel Edges as it starts, wider and stronger, and in a gold light.

## How the rule works, in words

Each pixel is given a slope, from -1 to 1: how much it faces the light. A slope above 0 moves the pixel that far toward the light's colour, times the intensity; a slope below 0 moves it that far toward black. Its covering never changes. For Bevel Alpha the slope comes from the drawing's covering blurred by half the thickness: where the covering rises going away from the light, the edge faces it. For Bevel Edges each pixel within the thickness of the layer's nearest side lies on that side's sloping face, with the corners cut on the diagonal as a picture frame's are, and everything inside is flat.

## How it differs from what we already have

**Emboss** turns the whole picture into grey relief by its brightness; the bevels keep the picture and shade only its edges. **Drop Shadow** and **Radial Shadow** put a shadow behind the drawing; the bevels shade the drawing itself. **Rim Light** lights the edge on one side only; the bevels light one side and darken the other.

## Known limits

- CC Glass is left out.
- They run on the processor; the graphics card version is its own later unit.
- Each is modelled on After Effects' own and is not claimed to match it.

## How you will check it

The build's test draws the eighteen fixture cases and the eight wrong settings and compares every pixel with the numbers `tools/bevel_reference.py` worked out, and writes `verification/B-148_bevel_table.md`, with pictures. A playtest sheet walks you through putting them on a layer.
