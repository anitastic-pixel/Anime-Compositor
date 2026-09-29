# B-127a: Compound Blur, blurred where another layer is bright (D-191)

Written on 2026-09-29, before any code. The second of the After Effects picks, A2, accepted with the rest by your "take everything". It does what After Effects' Compound Blur does, by a rule of our own, and it is the first effect to use the layer setting you accepted as D-189: an effect that reads another layer of the same composition as a map.

## What you will see

A new effect, **Compound Blur**, under **Blur**, after Lens Blur. It blurs a layer more where a map layer is brighter: sharp where the map is black, blurred the most where it is white, and in between for grey. Its card has these rows:

- **Blur Layer**, a list of this composition's layers, **None** when added. Pick the map. The map layer can be switched off; it still works as a map, so you do not have to see it.
- **If Sizes Differ**, **Stretch to fit** when added, or **Centre**, or **Tile**: what to do when the map layer is not the size of this one.
- **Maximum Blur**, 0 to 500 pixels, 20 when added: how blurred the white parts are. It can be keyed.
- **Invert Blur**, Off or On: On blurs the dark parts instead.
- **Edges**, **Transparent** or **Repeat**, as Lens Blur's: whether the layer's border fades into clear, or keeps its colour.

The map is the other layer's own picture, with its masks and effects, but not where it is moved to: it lies on this layer corner to corner. The layer does not grow. A draft blurs less with the smaller picture, so a draft looks like the full picture, smaller. On an adjustment layer it blurs everything below it by the map.

`compound_blur.png` in this folder is worked by the rule itself, at a quarter of 1920 by 1080, with Maximum Blur a quarter of 24: a street; a gradient layer, white at the top and bottom, black across the middle; the street blurred by it, so only the middle is sharp, as a small-model photograph looks; the same with Invert Blur on; a second map, black round a point and white further out; and the street blurred by that one, sharp at the point.

## The layer list, and copies

- The list shows every layer of this composition by name. If the layer it names is deleted, the list says **missing** and the effect does nothing, with a warning, until you pick another or undo the delete.
- A layer can name itself: its own brightness is then its map.
- Two layers may not blur each other by each other, nor go round in a longer circle: picking the layer that would close the circle is refused with a sentence saying so, and a file with such a circle is refused when opened, as a circle of mattes is. An effect switched off still counts, so switching it on can never close one. An adjustment layer ends the chain, since its map is only its white rectangle.
- **Duplicate** keeps the map, except that a layer mapped by itself becomes mapped by its copy. Duplicating a composition points each copy at the copy of its map. Copying and pasting the effect keeps its map.
- A **preset** does not keep a map, since the layer belongs to one composition: a preset saved with Compound Blur has Blur Layer None, and you pick the layer after applying it.

## How it differs from what we already have

**Gaussian Blur** and **Lens Blur** blur the whole layer the same. **Selective Colour Blur** blurs by the layer's own colours. Compound Blur blurs each pixel by how bright another layer is there, so a gradient, a shape layer or a keyed mask can say where the blur goes: a fake depth of field, a focus pull, heat haze behind a map, a blur that follows a moving shape.

## Known limits

- Between the blurs it makes, at Maximum Blur and a half, a quarter, an eighth and a sixteenth of it, each pixel mixes the two nearest. That is very close to a blur of its own size, and much faster; a thin bright edge between a sharp and a blurred part can show a faint double where the two mix.
- It is about twice the cost of a Gaussian Blur of Maximum Blur's size. The playtest sheet will time it.
- It runs on the processor. The graphics card learns it in a card unit of its own.

## How you will check it

The build's test draws the nineteen fixture cases and the seven wrong settings and compares every pixel with the numbers `tools/compound_blur_reference.py` worked out, opens the four circle files, and writes `verification/B-127_compound_blur_table.md`, with pictures. A playtest sheet walks you through adding it, picking a map and trying each setting.
