# B-654 playtest: Drop Shadow's soft edge is as soft as After Effects' (D-774)

## What changed for you

Drop Shadow's Softness now blurs the shadow as much as After Effects' does, which is less than
ours did.
- The blur's width (sigma) is now Softness / 5. Before, it was Softness / 3.
- Softness 50 now gives a blur of 10 pixels. Before, it gave about 17.
- Seven tutorials from seven creators measured After Effects' edge. Ours was 1.57 to 1.83 times
  as wide (median 1.68). Now it reads 0.94 to 1.10 times (median 1.01).

**An older project changes.** Every Drop Shadow with Softness above 0 that you saved before today
now draws a tighter shadow. Nothing in the file changes and nothing warns. To get the old look
back, multiply Softness by 5 / 3.

Radial Shadow, Rim Light, Outline, Bevel Alpha and a text layer's own shadow are not changed.

## What to look at

Each sheet in `verification/B-654 pictures/` has three pictures:
1. After Effects' own frame from a tutorial.
2. Ours before this change, the same settings.
3. Ours after it.

Above each picture is the shadow's blur (sigma, in pixels) and how it compares with After
Effects'.

| Sheet | After Effects | Ours before | Ours after | Verdict |
| --- | --- | --- | --- | --- |
| `jake_0044_s36_3way.png` (Jake In Motion, Softness 36) | 6.88 | 11.80 (1.72x) | 7.08 (1.03x) | matches |
| `mediaz_0113_s33_3way.png` (Anthony Mediaz, Softness 33) | 2.27 | 3.57 (1.57x) | 2.14 (0.94x) | matches |
| `shortorial_0023_s100_3way.png` (Shortorial, Softness 100) | 7.17 | 11.47 (1.60x) | 6.88 (0.96x) | matches |

Two more cases are kept outside the repository:
- Jake In Motion at Softness 11: 2.14 in After Effects, 3.52 before, 2.11 after.
- Emment at Softness 48: 3.12, then 5.36, then 3.22.

On each sheet the right-hand shadow's soft edge should look like the left-hand one. The middle
one should look clearly blurrier.

## Try it

1. Put Drop Shadow on a white solid over black. Set Distance 0, Opacity 100, Softness 50.
   - The shadow fades out over about 20 to 30 pixels around the solid. Before, it took about 50.
2. Set Softness 0.
   - The shadow is a hard edge, exactly as before.
3. Open a project saved before today that uses Drop Shadow.
   - It opens with no warning.
   - Its shadow is tighter. Multiply Softness by 5 / 3 and it looks as it did.

## Checks

`verification/B-654_drop_shadow_softness_table.md`: 88 of 88 checks pass. They cover:
- the edge's width at Softness 10, 25, 50 and 100: 2.04, 5.00, 9.98 and 19.80 pixels, against
  2, 5, 10 and 20;
- every Drop Shadow fixture at its numbers;
- an old project, which opens, saves back unchanged and draws a tighter shadow with no warning;
- the card against the processor, within 1 level.
