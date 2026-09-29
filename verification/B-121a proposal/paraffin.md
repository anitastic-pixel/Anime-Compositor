# B-121a: Paraffin, a soft wash of colour over a figure from one side (D-185)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build. It is our own effect, built from scratch; nothing is ported.

## What it is

In anime finishing, *paraffin* (パラ, "para") is the soft airbrushed wash a compositor lays over a character: a violet shadow rising from the feet so the figure sits in the scene, or a warm light falling from the head. It is done on every cel of a shot, fitted to the figure, and fades out with no line where it starts.

## What you will see

A new effect, **Paraffin**, in the Stylize group beside Distance Gradation and Rim Light. Its card has five rows:

- **Colour**, #6450a0 when added, a dusk violet.
- **Direction**, 0 to 360 degrees, the way the wash comes from, clockwise from up: 0 from above, 90 from the right, 180 from below, as added.
- **Spread**, 0 to 100 % of the figure, 70 when added: how far across the figure the wash reaches before it has faded to nothing.
- **Opacity**, 0 to 100 %, 50 when added: how strong it is where it is strongest.
- **Blend**: Normal, Multiply, Screen, Add, Overlay or Soft Light, Multiply when added.

Direction, Spread and Opacity can be keyed, so the wash can swing round a figure as a light passes.

What it does: it finds the figure, every pixel at least half covered, and measures it along the direction. The wash is strongest at the figure's near edge and fades on a smooth S-shaped curve, the way an airbrush does, to nothing at Spread per cent of the way across. Each pixel keeps its own covering, so nothing spills past the cel, and the empty pixels stay empty. As added it lays a violet shadow up the lower 70 % of the figure, darkest at the feet, as a floor shadow is. Soft Light and Overlay tint more gently and leave whites nearly white.

## How it differs from what we already have

- **Gradient** runs between two points you place on the frame. Paraffin fits itself to the figure: on a full-frame cel it runs across the figure, not across the frame, and it follows each drawing of a cycle without keys.
- **Distance Gradation** and **Rim Light** shade by distance from the figure's edge. Paraffin shades by position along one direction, as a light or a floor shadow does.
- **Vignette** darkens the frame's corners, not the figure.
- It adds two blends, **Overlay** and **Soft Light**, the ones paraffin is most often laid with. They tint a figure and keep its paint's light and dark, where Normal paints over it.

`paraffin.png` in this folder is worked by the reference rule itself, three times enlarged, over a pale blue background: a figure before; as added; a warm light from above, spread 50, opacity 70, screen; and a blue shade from the right, spread 60, opacity 60, multiply.

## Known limits

- The figure is the whole layer. Two characters on one cel share one wash; split them onto two layers to wash each.
- A wash is straight across; it does not follow the shape of a body the way a painted shadow does.
- It runs on the processor. The graphics card learns it in item 9, with the other new effects.

## How you will check it

The build's test draws the seventeen fixture cases and compares every pixel with the numbers `tools/paraffin_reference.py` worked out, and writes `verification/B-121_paraffin_table.md`, with pictures. A playtest sheet walks you through adding it to a figure, turning the direction, and trying the blends.
