# B-125a: "a layer of this composition" as an effect's setting (D-189)

Written on 2026-09-28, before any code. Three of the After Effects picks you accepted ("take
everything") read another layer's picture: **Compound Blur** blurs more where the other layer is
bright, **Displacement Map** pushes pixels by it, and **Gradient Wipe** wipes in the order of its
greys. After Effects calls the setting a *layer* setting: a drop-down of the composition's layers.
This unit builds that setting once, so the three effects share it, and settles every question it
raises before any of them is written.

## What you will see

Nothing yet. No effect has a layer setting until Compound Blur (A2), which brings the drop-down
on its card. What this unit gives is the rule for **what picture the named layer gives**, and a
table test that checks it on 37 small cases. `layer_map.png` in this folder shows the three ways
the picture is fitted.

## The rule

**What the named layer gives.** Its own picture at the same frame, by its own timing, after its
masks and its own effects, and before it is placed: the way After Effects reads a map layer "with
its masks and effects". So:

- where the layer is placed, its size, turn, opacity, blend mode and camera make no difference;
- a layer that is switched off, or used only as a matte, still gives its picture, so you can hide
  the layer you use as a map, as in After Effects;
- a drawing gives the drawing exposed on that frame, a solid its colour, a shape layer the frame
  with its shapes, a composition layer the frame of the composition inside it;
- an adjustment layer gives white through its masks;
- a null, a layer not yet in or already out, or a drawing whose file is missing gives an empty
  (clear) picture, and a missing file still says so;
- an effect that spreads the picture, such as a blur, is cut back to the layer's own size.

**Its own picture.** If the effect names the layer it is on, it reads that layer's drawing and
masks, not its effects, which would go round in a circle.

**Only this composition.** The list offers only layers of the same composition, as After Effects'
does. To use a picture from inside another composition, use the composition layer that shows it.

**Fitting it.** The named picture is rarely the size of the layer the effect is on. Each effect
that uses a layer setting has a fit setting beside it, After Effects' three:

| Fit | What it does |
| --- | --- |
| Centre | put in the middle; clear round it, cut off if bigger |
| Tile | put in the middle and repeated to fill |
| Stretch | stretched or squeezed to fill exactly |

**A deleted layer.** The setting keeps the name, so **Undo** of the delete brings everything back.
While the layer is gone the effect is skipped and the warning `EFFECT_LAYER_MISSING` names the
layer and the effect on every frame, as a missing matte does. Nothing is dropped from the file.

**Going round in a circle.** A layer's effect naming a layer whose effect names the first layer
back is refused when you choose it, and a file holding one is refused on opening, with
`EFFECT_LAYER_CYCLE`. This is checked with Compound Blur, the first effect that can make one.

**Draft.** The map is made at the size the effect runs at. Where a layer's effects already run at
a quarter size in Draft (a drawing, a composition layer, an adjustment layer), the named layer is
made at a quarter size too; a solid or shape layer's effects run at full size, so its map is the
full one.

## What is checked

FX-LMAP-001 to 042 in document 25, each worked by `tools/effect_layer_reference.py`, which never
runs the program's code. One small project of 8 by 6 pixels holds every layer the cases name. The
table test will draw each map through the program and compare every pixel to that file's numbers.

## Known limits

- No card version: the effects that use a map run on the processor until a card unit for them.
- The three effects that use the setting come as their own units (A2, A3, A4), each with its own
  card, the drop-down, and what happens to the setting when a layer is duplicated or saved in a
  preset.
