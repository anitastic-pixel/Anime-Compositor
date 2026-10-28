# B-801 playtest: Layer Styles, Inner Shadow and Drop Shadow (D-922)

What changed: a layer can now have After Effects' Inner Shadow and Drop Shadow layer styles,
from Layer > Layer style: Drop Shadow / Layer style: Inner Shadow (or the command palette, or
the Effects browser's new Layer Styles group). Each has After Effects' settings and defaults:
Blend Mode (Multiply), Color (black), Opacity 75, Use Global Light (On), Angle 120,
Distance 5, Choke (Inner Shadow) or Spread (Drop Shadow) 0, Size 5, Noise 0. Styles always run
after the layer's effects, whatever their place in the list, Inner Shadow before Drop Shadow,
as in After Effects. Layer > Global Light angle sets the composition's light (120 until
changed); every style with Use Global Light on follows it. In the timeline a styled layer's
keyed settings sit under their own "Layer Styles" group. Outer Glow, Inner Glow and Stroke are
not built yet (still proposed in D-922).

## Steps

1. Open the program, make a composition with a background picture and, above it, a picture
   with a transparent background (a character cut-out).
2. Select the cut-out and pick Layer > Layer style: Inner Shadow. A dark shadow appears inside
   its top-left edge (light from the upper left, 120 degrees).
3. In the Effect Controls set Blend Mode to Add (Linear Dodge), Color orange, Use Global Light
   Off, Angle 1, Distance about 40, Size 20, Opacity 50 (the tutorial's settings at 28:54).
4. Pick Layer > Layer style: Drop Shadow. A soft dark shadow falls behind the cut-out, down
   and to the right.
5. Pick Layer > Global Light angle, type 45, Enter. The Drop Shadow (Use Global Light on)
   swings to the lower left; the Inner Shadow you turned Off stays where it was.
6. Edit, Undo: the light goes back to 120.
7. Save, close, open again: both styles and the Global Light angle are as you left them.

## What to see

- Step 3: an orange rim of light along the cut-out's left edge only, inside the shape, never
  spilling onto the background: the tutorial's rim light.
- In Color Dodge with a bright yellow (video 2 at 5:23) the rim burns brighter and stronger
  on light parts and almost vanishes on near-black parts.
- Drop Shadow in a mode other than Multiply/Normal, or Multiply with a colour other than black,
  draws, and the status lists it once as "not matched exactly yet" (it is drawn behind the
  layer; After Effects' own result there is not yet pinned by a fixture).

## Pictures

`verification/B-801 pictures/rim_light_sheet.jpg`: a stand-in cut-out (made for this check,
not the tutorial's character) over a stand-in scene: no style (top left), the w12a tutorial's
orange Add rim light (top middle), video 2's yellow Color Dodge rim light (top right), the Drop
Shadow at its defaults with distance 20 and size 20 (bottom left), and the two tutorial
stills, w12a at 28:54 (bottom middle) and video 2 after 5:23 (bottom right). Distances and sizes
are about half the tutorials', because the picture is about half their size.

## Fixtures

`verification/B-801_layer_styles_table.md`: 101 of 101 (FX-LSTYLE-001 to 023, the Global
Light, saving and opening, commands, tiles).
