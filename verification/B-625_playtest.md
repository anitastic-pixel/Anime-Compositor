# B-625: Glow's strength and fall-off, by hand

Built on 2026-10-23 against D-746 (audit row 12, https://claude.ai/artifact/A63KdY44N4HfUHzUnLQhTp),
accepted that day under your instruction "Proceed, if an effect is more like a
inspiration/modified version of a AE effect, only improve it. But it is to be a 1 to 1 match,
then go for it. Proceed down all the waves."

The generated half is `verification/B-625_glow_strength_table.md` (28 of 28), which renders every
FX-GLDISP case and two pictures rebuilt from tutorial frames. This sheet covers how it looks.

## Three-way sheets: After Effects / before / after

Each sheet in `verification/B-625 pictures/` puts After Effects' own frame from a tutorial on the
left, the app before this change in the middle, and after on the right, with the light each adds
measured in levels, by distance from what glows. The verdict is ours / After Effects on the first
5 pixels: 0.8 to 1.25 matches, 0.67 to 1.5 close, otherwise differs.

"Fit" cases set the rule's two numbers; "check" cases took no part in that.

| Case | Role | After Effects | Before | After | Before verdict | After verdict |
| --- | --- | --- | --- | --- | --- | --- |
| Valeri 1 | fit | 76.1 | 113.2 | 121.8 | 1.49 close | 1.60 differs |
| Valeri 2 | fit | 67.6 | 121.1 | 109.3 | 1.79 differs | 1.62 differs |
| Motion Nations 1 | check | 12.5 | 180.7 | 16.5 | 14.46 differs | 1.32 close |
| Motion Nations 2 | fit | 18.7 | 190.6 | 17.0 | 10.19 differs | 0.91 matches |
| Motion Nations 3 | check | 6.7 | 64.1 | 6.8 | 9.57 differs | 1.01 matches |
| Motion Nations 4 | fit | 12.5 | 154.0 | 16.6 | 12.32 differs | 1.33 close |
| Robin 1 | check | 17.7 | 153.1 | 14.2 | 8.65 differs | 0.80 matches |
| School of Motion square | check | -1.2 | 73.6 | -19.6 | can't tell (under 6 levels) | can't tell (under 6 levels) |
| Ducky 1 | footage | 74.2 | 168.4 | 73.0 | 2.27 differs | 0.98 matches |
| Ducky 2 | footage | 96.6 | 184.3 | 73.0 | 1.91 differs | 0.76 close |
| SFP 1 | footage | 48.8 | 111.8 | 22.2 | 2.29 differs | 0.45 differs |
| School of Motion footage 1 | footage | 33.6 | 77.9 | 18.4 | 2.32 differs | 0.55 differs |

Pictures: `3_sheet_motion_nations_2_fit.png`, `4_sheet_robin_1_check.png`,
`5_sheet_school_of_motion_square_gap.png`. All twelve sheets were made by the lane and are listed
in its report.

## Before you start

Open a project, set the composition to **8 bpc**, and put white or brightly coloured text on a
black background card.

## What to check

1. **Default Glow.** Add **Glow**. The halo is soft and modest, close to the left of
   `3_sheet_motion_nations_2_fit.png`, not the blown-out white of its middle.
2. **Colour stays.** On magenta-to-blue text, the letters keep their colour; before, they went
   white.
3. **Only the bright channel glows.** A mid-blue shape (57, 67, 179) on grey, Threshold 60: the
   shape turns fully blue and only blue spreads out.
4. **Bigger radius reaches further, softly.** Radius 400, Intensity 2, Threshold 0 on a white
   disc: a wide, smooth fall-off like `1_valeri_disc.png`.
5. **Intensity 0.** Nothing changes; no dark ring appears.
6. **Other depths unchanged.** Switch the composition to Float: the Glow looks as it did before
   this change.

## Known limits, on purpose

- A glow on a layer with empty surroundings, over another layer, is brighter than After Effects
  (Valeri 1.6 times) and, where only one channel glows, darkens the other channels just outside
  the shape more than After Effects (the School of Motion square). Both come from how 8 bpc
  layers combine (D-333) and are not changed here.
- On recorded footage the halo is about half After Effects' (SFP, School of Motion footage): the
  footage's scale in those tutorials is unknown, so it is not used to set the rule.

## What to answer

"works", or which step number did something else and what it did.
