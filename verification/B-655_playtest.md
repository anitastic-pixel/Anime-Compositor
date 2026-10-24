# B-655 playtest: a mask's soft edge is as soft as After Effects' (D-775)

## What changed for you

A mask's Feather now blurs the edge as much as After Effects' does, which is less than ours did.
- The blur's width (sigma) is now 0.375 x Feather. Before, it was Feather / 2.
- Feather 100 now gives a blur of 37.5 pixels. Before, it gave 50.
- Eight tutorials from eight creators measured After Effects' mask edge. Seven of them put ours
  1.27 to 1.43 times as wide (median 1.335); the eighth (Graphician) read 1.94, wider still.
  Now the seven read 0.94 to 1.09 times (median 1.00).

**An older project changes.** Every mask with Feather above 0 that you saved before today now
draws a tighter edge, its blur 3 / 4 of what it was. Nothing in the file changes and nothing
warns. To get the old look back, multiply Feather by 4 / 3.

Fill's feathers, Linear Wipe, Radial Wipe, Inner/Outer Key's Edge Feather and every effect's own
feather are not changed.

## What to look at

Each sheet in `verification/B-655 pictures/` has three pictures:
1. After Effects' own frame from a tutorial.
2. Ours before this change, the same mask and feather.
3. Ours after it.

Above each picture is the blur's width as a share of the feather, and how it compares with After
Effects'.

| Sheet | After Effects | Ours before | Ours after | Verdict |
| --- | --- | --- | --- | --- |
| `mycgtutor_7b4MJI14yTE_mask_3way.png` (My CG Tutor, rectangle, Feather 373) | 0.375 | 0.496 (1.32x) | 0.371 (0.99x) | matches |
| `adobecare_iU4h8WalKFM_mask_3way.png` (Adobe Care, ellipse, Feather 183) | 0.349 | 0.499 (1.43x) | 0.374 (1.07x) | matches |
| `guidehub_u9CovhkiNGE_mask_3way.png` (Guide Hub, rectangle on a photo, Feather 80) | 0.392 | 0.497 (1.27x) | 0.374 (0.95x) | matches |

Two more cases are kept outside the repository:
- Animation Explained, circle, Feather 162: 0.395 in After Effects, 0.499 before, 0.373 after.
- LEARN NOW, inverted ellipse, Feather 95: 0.364, then 0.495, then 0.373.

Our pictures are flat-colour stand-ins at the tutorial's shape and size (a photo replaced by its
middle colour). On each sheet the right-hand edge should look like the left-hand one. The middle
one should look clearly blurrier.

## Try it

1. Draw a rectangle mask on a white solid over black. Set Feather 100.
   - The edge fades over about 100 pixels across. Before, it took about 130.
2. Set Feather 0.
   - The edge is hard, exactly as before.
3. Open a project saved before today that uses a feathered mask.
   - It opens with no warning.
   - Its edge is tighter. Multiply Feather by 4 / 3 and it looks as it did.

## Checks

`verification/B-655_mask_feather_table.md`: 78 of 78 checks pass. They cover:
- the edge's width at Feather 4, 8, 20 and 40 against 0.375 x Feather, and a hard edge at 0;
- every mask fixture, and Energy Stroke's FX-ENERGYX-022, at their new numbers (Display Mix's
  FX-MIX8-009 and 010 are checked by `b642_display_mix`, which passes);
- an old project, which opens, saves back unchanged and draws a tighter edge with no warning;
- the card against the processor, within 1 level.
