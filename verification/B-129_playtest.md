# B-129: Gradient Wipe, by hand

Built on 2026-09-29 against D-194, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Gradient Wipe does, by a rule of our own: a
transition that wipes a layer away in the order of another layer's brightness, the darkest parts
first and the brightest last. Transition Completion says how far it has gone, and Transition
Softness how soft the edge is. It is the third effect that reads another layer (D-189), after
Compound Blur and Displacement Map.

The generated halves are `verification/B-129_gradient_wipe_table.md`, 132 of 132 checks passing,
which renders every FX-GWIPE case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Gradient Wipe
card sends every setting the command reads. This sheet covers what the tables cannot: how it looks
and feels in the window.

## The pictures

`verification/B-129 pictures/`, B-127's street at a quarter of 1920 by 1080, over a plum solid
that stands for the next shot, so what is wiped away shows plum:

- `town.png`: the street.
- `map_spot.png`: a map, black at a point left of the middle and turning white further out.
- `map_clouds.png`: a map of soft clouds.
- `before.png`: the street with no effect.
- `spot_30.png`: the spot at Transition Completion 30%, Softness 0. A hard-edged hole round the
  spot; outside it not one pixel of the street changes.
- `spot_60_soft.png`: 60%, Softness 25. A wider hole with a soft edge.
- `spot_30_invert.png`: 30% with Invert Gradient. Wiped from the outside in: the street is kept
  round the spot and gone further out.
- `clouds_40.png`: the clouds at 40%, Softness 10. Soft patches of the street gone.
- `clouds_75.png`: the clouds at 75%. Most of the street gone.

## Before you start

Import `verification/B-129 pictures/town.png` and `verification/B-129 pictures/map_spot.png`,
make the composition 480 by 270, and make a plum solid the size of it. Put the layers in this
order, top first: `town`, `map_spot`, the solid. Switch `map_spot` off with its eye: a map is read
whether or not it is shown. Press **Full resolution**.

## What to check

1. **Adding it.** On `town`, pick **Gradient Wipe** in **Add effect…**, under **Transition**,
   after Linear Wipe; typing "gradient", "reveal" or "dissolve" in the search finds it too. The
   card shows **Transition Completion** 0, **Transition Softness** 0, **Gradient Layer** None,
   **Gradient Placement** Stretch Gradient to Fit and **Invert Gradient** Off. Nothing changes.
2. **No layer.** Drag Transition Completion to 50 with no Gradient Layer: still nothing changes,
   since there is nothing to wipe by.
3. **The map.** Set Gradient Layer to `map_spot` and Transition Completion to 30: it looks like
   `spot_30.png`. The list names every layer of the composition; `town` says "(this layer)".
4. **Completion.** Drag Transition Completion from 0 to 100: the hole grows from the spot outward
   until the street is gone. At 100 nothing of it is left, even where the map is white.
5. **Softness.** At 60, set Transition Softness to 25: like `spot_60_soft.png`. At 100 the edge is
   as soft as it gets: at completion 50 each pixel is kept by how bright the map is there.
6. **Invert.** Back to 30 and Softness 0, and Invert Gradient On: like `spot_30_invert.png`.
7. **Another map.** Import `map_clouds.png`, switch it off, name it, Softness 10: 40 looks like
   `clouds_40.png` and 75 like `clouds_75.png`.
8. **Itself.** Set Gradient Layer to `town (this layer)`: the street's own dark parts, the road
   and the windows, go first.
9. **Out of range.** Type 101 in Transition Completion: it is refused with a sentence saying it
   runs from 0 to 100, and the card keeps its old number.
10. **Sizes.** Make a small solid, 120 by 60, white, and name it, at completion 50. Stretch
    Gradient to Fit: nothing goes, since it is white all over. Center Gradient: all but the
    middle 120 by 60 goes, since outside the map reads as black. Tile Gradient: nothing goes.
11. **A circle.** Add a Gradient Wipe, a Displacement Map or a Compound Blur to `map_spot` and
    name `town`. It is refused with a sentence: the two layers would read each other in a circle.
    Naming the layer itself is never a circle.
12. **A missing layer.** Delete `map_spot` while `town` names it: the wipe goes and the whole
    street is back, and the warning panel says the effect reads a layer that is not in the
    composition. Undo: it comes back and so does the wipe.
13. **Keyed.** Key Transition Completion from 0 at the first frame to 100 a second later and
    play: the street is wiped away from the spot outward over the second. Key Softness too: the
    edge softens as it goes.
14. **Moving.** Move, scale or rotate `map_spot`: nothing changes, since a map is read before its
    layer is placed. Move `town`: the hole moves with it, since the map lies on the layer.
15. **Draft.** Press **Draft**: the picture is smaller and the hole with it, the same shape.
16. **Undo and saved.** Ctrl+Z steps back each change, the layer named included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- A map is read the size of its own drawing and fitted to the layer by Gradient Placement, not
  where it sits in the frame. On an adjustment layer the map is fitted to the frame.
- The map's brightness is read as the picture shows it over black: a clear part of a map reads
  as black and goes first.
- The picture never grows, and nothing but the layer's own pixels is taken away.
- It is drawn on the processor; the graphics card learns it in a card unit of its own.
  The reference shot's frame 100 at full size, 1920 by 1080, takes 49 to 54 ms without it
  and 62 to 67 ms with a Gradient Wipe on its first layer reading another, at completion 0,
  50 or 100 and softness 0 or 25: about 14 ms, nearly all of it reading the map. The same
  build made reading a map about three times faster, for Compound Blur and Displacement Map
  too.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
