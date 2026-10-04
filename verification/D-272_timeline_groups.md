# D-272: the timeline's layer groups, and an arrow that always closes

Built on 2026-10-03 at the owner's request: "migrate the masks/ blend/ transform into the timeline layer as their own modifier next to the mask line to in their respective grouped way like in AE. also have it so that I am able to collapse the layer no matter what when clicking the toggle arrow, because the mask 1 is still here and not collapsed when I am done."

**Awaiting the owner's playtest.** This is a page change only: no new command, no saved data and no picture changes.

## What changed

- **The layer's arrow always closes.** Before, an open mask (or any property opened with a letter) stayed under the layer after the arrow was clicked. Now the arrow closes everything under the layer, whatever opened it.
- **Opened, a layer shows After Effects' groups,** each with its own arrow:
  - **Masks**: one row per mask with its mode list (Add, Subtract, ...) and an **Inverted** button. A mask's own arrow shows **Mask feather**, **Mask opacity** and **Mask expansion**, which drag or take a typed number as Effect controls' numbers do.
  - **Contents**: a shape layer's shapes, as before.
  - **Effects**: an effect setting that has keys.
  - **Blending**: **Blend mode**, the same list as the layer row's Mode column. Not shown for adjustment or null layers.
  - **Transform**: Anchor, Position, Scale, Rotation, Opacity and Depth, with stopwatches and keys as before.
- The groups that were open come back the next time the layer is opened, for the session.
- **M** shows only the layer's masks, as in After Effects; M again hides them. A, P, S, R, T and U work as before and close the groups while they show their one property. The shortcut line at the bottom of the window now lists M.
- Drawing a new mask opens the layer at its Masks group (a shape layer at Contents), so the new mask is in view.
- Effect controls keeps its Transform and Blend, masks and parent sections as they were; both places change the same settings.

## Window evidence (test copy, layer1 of the reference shot)

| Step | What the timeline showed |
|---|---|
| Draw a mask on layer1 | Masks, Mask 1, Blending, Transform |
| Click layer1's arrow | (nothing under the layer): the reported bug is gone |
| Click the arrow again | Masks, Mask 1, Blending, Transform |
| Open Transform | ... Anchor, Position, Scale, Rotation, Opacity, Depth |
| Open Mask 1 and Blending | Mask 1 shows Mask feather, Mask opacity, Mask expansion; Blending shows Blend mode |
| Mask mode to Subtract, Inverted on, Blend mode to Screen | mask mode subtract, inverted on, layer blend screen; the layer row's Mode list also reads Screen; Undo reads "Undo Set the blend mode to screen" |
| Real mouse drag of 88 px right on Mask feather | feather 0 to 88 px |
| P, then M, then M | Position only; Mask 1 only; nothing |
| The arrow after the letters | the groups as they were left open |
| Page errors | none |

The app's 88 checks pass (5 ignored, as before), and the core suite passes apart from the two known scratch tests that are not part of the project.

## Pictures

In `verification/D-272 pictures/`:
- `1_new_mask_opens_its_group.png`: just after drawing a mask, layer1 opens at Masks.
- `2_layer_arrow_closes_everything.png`: after one click on layer1's arrow, nothing is left under it.
- `3_groups_open_and_edited.png`: Masks, Mask 1 and its three settings (feather 88 px), Blending set to Screen, and Transform open.

## Playtest

1. Select a layer and draw a mask with the mask tool. The layer opens with **Masks** and **Mask 1** under it.
2. Click the layer's arrow. Everything under the layer should close, including Mask 1.
3. Click the arrow again. Open **Mask 1**'s own arrow and drag **Mask feather** right: the mask's edge should soften as you drag.
4. Change Mask 1's list to **Subtract** and click **Inverted**; the picture should follow each.
5. Open **Blending** and pick **Screen**; the Mode list on the layer's row should say Screen too.
6. Open **Transform** and drag Scale; the layer should grow as before.
7. With the layer selected, press **M**: only the masks show. Press M again: they hide.
