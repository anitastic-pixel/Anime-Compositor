# D-280: Transform, blending, masks, matte and parent live on the timeline only

Built on 2026-10-03 at the owner's request ("perfect! proceed with your recommendation"). After Effects keeps a layer's Transform, blend mode, masks, track matte and parent in the timeline, not in Effect controls. This build does the same. Before removing anything from Effect controls, the timeline was given the pieces it lacked, so nothing that could be done before is lost.

The same build fixes the owner's "small formatting error": the "Match a file…" button in the New composition window wrapped onto two lines. It now stays on one line.

## What moved where

| Was in Effect controls | Now on the timeline |
|---|---|
| Transform: anchor, position, scale, rotation, opacity, depth | the layer's **Transform** group (already there since D-272) |
| Blend mode | the layer's **Blending** group (already there since D-272) |
| Matte list and "matte only" | **Blending > Track matte**, new. Adjustment layers get it too. |
| Parent list | the **Parent** column of the layer's row. It is now a list (None, or another layer) beside the pick whip, so a parent can be chosen or cleared with the keys. |
| Each mask's mode, Inverted, On/Off and Delete | the mask's own row under **Masks**. On/Off and Delete are new there. |
| A mask's "Edit points" | the mask's name, which can now be reached with Tab and pressed with Enter |

Effect controls now shows one line in their place, "Transform, blending and masks: On the timeline". Its **Show on the timeline** button opens the layer with Transform, Blending and Masks open. The Layer section, shape contents, Camera and Effects stay in Effect controls.

No saved data and no command changes. The timeline sends the same commands Effect controls sent.

## Window evidence

The test copy was built from this change (`$S/tgt`, release) and driven by `cdp3.ps1` on the reference shot.

| Step | Seen |
|---|---|
| Ctrl+N | "Match a file…" is 26 px high (one line) and set not to wrap. |
| Chose layer1 | Effect controls showed "Show on the timeline", with no Anchor point, Blend mode or Matte rows. |
| Clicked Show on the timeline | The timeline came to the front with Transform, Blending and Masks open under layer1: Blend mode, Track matte, Anchor, Position, Scale, Rotation, Opacity, Depth. |
| Parent list: chose layer2 | The list offered None, layer2, layer3, layer4. layer1's parent became layer2, and Undo read "Undo Set parent to layer-2". |
| Parent list: chose None | The parent was cleared, and the list read None. |
| Track matte: chose layer2 | The list offered none, layer2, layer3, layer4. layer1's matte became layer2 (alpha). |
| Track matte: chose none | The matte was cleared. |
| Drew a rectangle mask | Mask 1's row showed Add, Inverted, **On** and **Delete**. |
| Clicked On | The mask turned off, and the button read Off. |
| Clicked Off | The mask turned on again. |
| Tab to the mask's name, Enter | The mask was picked, so its points show in the viewer. |
| Clicked Delete | layer1 had no masks. |
| Page errors | None. |

Mid-test, the first run found a fault: the new button used a helper that only exists in another part of the page, so choosing a layer stopped Effect controls drawing. It was fixed and the run above is after the fix.

The app's own checks: 88 passed, 0 failed, 5 ignored. The keyboard check said the anchor could no longer be moved without the mouse, because its route had been the Effect controls row. That check now looks for the timeline's Transform group, where the anchor is typed.

## Pictures

In `verification/D-280 pictures/`:
- `e1_match_a_file_one_line.png`: the New composition window with "Match a file…" on one line;
- `e2_effect_controls.png`: Effect controls with the "Show on the timeline" button;
- `e3_timeline_groups.png`: the groups opened by the button;
- `e4_parent_list.png`: the Parent list set to layer2;
- `e5_track_matte.png`: Track matte set to layer2;
- `e6_mask_row.png`: a mask's row with On and Delete;
- `e7_mask_off.png`: the mask turned off.

## Playtest

1. Press Ctrl+N. "Match a file…" sits on one line. Press Esc.
2. Click a layer. Effect controls no longer lists Transform, Blend or masks. Click **Show on the timeline**. The layer opens on the timeline with Masks, Blending and Transform open.
3. In the layer's row, click the Parent list (it says None), pick another layer, then pick None again.
4. Under Blending, set Track matte to another layer. The picture shows only where that layer is. Set it back to none.
5. Draw a mask on the layer. In its row, click **On**: the mask stops cutting. Click **Off** to bring it back, then **Delete**.
6. Say whether anything you used to do in Effect controls is now missing.
