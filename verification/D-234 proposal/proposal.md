# D-234 (PROPOSED): one colour table for a stack of colour effects, preview only

Written on 2026-09-29 by the agent. Nothing is built. This is for you to decide, and my
recommendation is **not to do it now**.

## What it is

Suppose a layer has several colour effects in a row, for example Curves, then Levels, then
Hue/Saturation, then Brightness/Contrast, then Tint. Today the viewer works out every one of
them, for every pixel, one after another.

The idea is to do that work once for a sample of colours instead. The program would work the
whole stack out for a grid of colours, 17, 33 or 65 steps along each of red, green and blue.
It would keep the answers in one table, the same kind of table a `.cube` colour lookup file
holds. After that, each pixel only looks its colour up in the table and blends the nearest
eight entries. The picture you export would still be worked exactly, with no table.

## What you would see

It would look almost the same, but not exactly. The table only knows the colours on its grid,
and it blends in between them. Wherever an effect bends colours sharply, the blend misses a
little. Hue/Saturation does that most, near greys and wherever it pushes saturation to full.

## The pictures

Every picture shows three panels side by side:
- left: the exact result;
- middle: the table's result;
- right: the difference, made 32 times brighter so you can see it. Black there means the two
  panels are the same.

The frame is the reference shot's frame (`verification/B-05a_reference_frame.png`), and
`untouched_frame.png` shows it with no effects.

- `gentle_grid17_side_by_side.png`, `gentle_grid33_side_by_side.png` and
  `gentle_grid65_side_by_side.png` show a mild grade, the kind you might leave on a shot:
  - a gentle S curve;
  - slight Levels;
  - hue +8, saturation +15;
  - contrast +10;
  - a 12% warm tint.
- `strong_grid17_side_by_side.png`, `strong_grid33_side_by_side.png` and
  `strong_grid65_side_by_side.png` show a hard grade:
  - a strong curve;
  - crushed Levels;
  - hue +40, saturation +60, lightness -10;
  - contrast +40;
  - a 30% blue tint.

## The numbers

A level is one step of 255 in the 8-bit picture you see. The program's standard, which you
accepted for the card, is **at most 1 level** off. In the tables, **worst** is the largest
difference anywhere, and **% off by >1** is the share of pixels more than 1 level off.

I measured two things for each grade:
- **The frame:** the reference frame itself, 2,073,600 pixels.
- **Every colour:** all 16,777,216 colours a pixel can have, so no picture can do worse.

The mild grade:

| Grid | Frame: worst | Frame: % off by >1 | Frame: PSNR | Every colour: worst | Every colour: % off by >1 |
| --- | --- | --- | --- | --- | --- |
| 17 | 9 | 21.8% | 48.8 dB | 11 | 28.8% |
| 33 | 5 | 2.8% | 56.1 dB | 6 | 5.3% |
| 65 | 3 | 0.18% | 60.4 dB | 3 | 0.23% |

The hard grade:

| Grid | Frame: worst | Frame: % off by >1 | Frame: PSNR | Every colour: worst | Every colour: % off by >1 |
| --- | --- | --- | --- | --- | --- |
| 17 | 27 | 27.3% | 42.0 dB | 38 | 41.0% |
| 33 | 14 | 10.6% | 49.0 dB | 18 | 12.3% |
| 65 | 8 | 1.6% | 58.6 dB | 12 | 2.3% |

**No grid size meets the 1-level standard**, not even 65 on the mild grade.

To find where the misses come from, I also measured each effect alone, over every colour:
- Curves, Levels and Tint alone stay within 1 level at 33 and 65.
- Brightness/Contrast alone stays within 1 level at 65 only.
- Hue/Saturation alone misses by up to 5 levels (mild) and 21 levels (hard), even at 65.
- The hard grade still misses by 2 levels at 65 with Hue/Saturation taken out.

So the misses are not all from one effect.

## How much faster (an estimate, not a timing)

I did not time this. These are counts of pixel operations, and each sRGB curve step is counted
as about 20 simple operations.

- **Today:** each effect turns the pixel's colour into sRGB and back, about 120 operations, plus
  its own maths. The five-effect stack comes to about 700 operations a pixel.
- **The table:** one trip into sRGB and back, plus an eight-entry blend, about 170 operations a
  pixel. That is **about 4 times fewer**.
- **G13 already in the queue:** G13 joins neighbouring colour effects into one pass with the
  same maths. That drops most of the repeated trips into sRGB and back without changing
  pixels, and leaves the stack at roughly 350 to 450 operations a pixel. **So the table's gain
  over G13 is only about 2 times.**
- **Building the table:** it is built again whenever a setting changes. That costs 35,937 stack
  workings at 33, about 2% of one 1920 by 1080 frame, or 274,625 at 65, about 13% of one.

## My recommendation

**Do not build it now.** It breaks the 1-level standard at every size I tried. The worst place
is Hue/Saturation, the effect people use most. And G13, which does not change pixels, already
takes most of the speed. If a colour stack is still slow after G13 is built and measured, this
can be looked at again.

## Your choices

1. **Decline for now** (recommended). Nothing changes. G13 goes ahead as planned.
2. **Accept a checked table, preview only, off by default.** A viewer switch would turn it on,
   and the table would only be used for a stack where a check finds it within 1 level. The catch
   is that an honest check has to cover every colour, which costs about 8 frames of work each
   time a setting changes. A cheaper check on the current frame alone cannot promise anything
   about the next frame.
3. **Accept the table as it is, preview only, 65 grid, off by default.** This means accepting
   that the viewer can be off by up to about 12 levels in hard grades, while exports stay
   exact.

## Where it comes from

- `tools/d234_lut_collapse_experiment.py` makes every number and picture here. Run it with
  `python tools/d234_lut_collapse_experiment.py`.
- Before it prints anything, it checks its own versions of the five effects and of the table
  lookup against the reference tools already in `tools/`: curves, levels, hue_saturation,
  brightness_contrast and cube_lut. It stops if they disagree.
- Tint is written from `src/effects.rs`, because there is no reference tool for it.
- `experiment_output.txt` is the printout from the run that made these files.
