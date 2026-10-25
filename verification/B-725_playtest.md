# B-725 playtest: Twirl's outer swirl turns as far as After Effects' (D-845)

## What changed for you

Twirl now turns the outer part of its circle a little further, as After Effects' Twirl does.
- The centre still turns by the whole Angle, and the edge of the circle still not at all.
- Half the Angle is still reached half way out.
- From 0.6 to 0.9 of the way out, a pixel now turns about 0.03 to 0.04 of the Angle more than
  before. On a Twirl of 90 degrees that is 2 to 4 degrees; on 360, 10 to 15 degrees.
- Eight tutorial twirls were measured ring by ring. Seven of them, from six creators, turn more than
  ours did toward the edge, and the new rule is closer to every one of those seven.

**An older project changes.** Every Twirl you saved before today now swirls its outer part a
little further. Nothing in the file changes and nothing warns. The middle and the edge of the
circle stay where they were.

## What to look at

Each sheet in `verification/B-725 pictures/` has three pictures and a curve:
1. After Effects' own frame from a tutorial.
2. Ours before this change (D-744's smoothstep), the same Angle and Radius.
3. Ours after it.
4. The curve: how far each ring turns, as a share of Angle, from the centre (left) to the edge
   (right). Yellow is After Effects, dashed orange is ours before, blue is ours after.

On each sheet the blue curve should lie on the yellow one, and the third picture should look
like the first. The angle gap is the median difference in degrees between ours and After
Effects' turn, over all rings and over the outer rings (0.6 to 1.0 of the way out).

| Sheet | Angle, Radius | Gap all / outer, before | Gap all / outer, after | Likeness (SSIM), before / after | Verdict |
| --- | --- | --- | --- | --- | --- |
| `tw_jakeinmotion.jpg` (Jake In Motion, a grid) | 182, 20 | 2.25 / 5.75 | 0.62 / 0.62 | 0.747 / 0.873 | matches |
| `tw_amfilm.jpg` (AM Film, a photo) | -360, 30 | 3.62 / 11.25 | 1.88 / 1.62 | 0.389 / 0.688 | matches |
| `tw_dope_a211_r25.jpg` (Dope Motions) | 211, 25 | 4.25 / 7.0 | 1.0 / 0.75 | 0.513 / 0.883 | matches |
| `tw_erv_a100_r30.jpg` (Ervin) | 100, 30 | 1.38 / 3.25 | 0.5 / 0.38 | 0.435 / 0.825 | matches |
| `tw_wiredboy27.jpg` (wiredboy27, text) | 119, 30 | 4.5 / 5.12 | 2.0 / 0.62 | 0.749 / 0.840 | matches |
| `tw_ag_a69_r30.jpg` (Ag Motion Designs) | 69, 30 | 2.13 / 3.5 | 1.0 / 0.88 | 0.588 / 0.733 | closer |
| `tw_dope_a58_r30.jpg` (Dope Motions) | 58, 30 | 5.25 / 4.0 | 3.75 / 1.5 | 0.461 / 0.622 | closer |
| `tw_kuro_a70_r30.jpg` (KuroCutt) | 70, 30 | 0.5 / 1.38 | 0.5 / 0.75 | 0.838 / 0.843 | matches |
| `tw_kuro_a70_r100.jpg` (KuroCutt) | 70, 100 | 0.5 / none | 0.5 / none | 0.830 / 0.820 | as before |
| `tw_ilija_a-55_r30.jpg` (Ilija Kostic) | -55, 30 | 0.75 / 1.0 | 0.25 / 2.25 | 0.902 / 0.917 | as before |
| `tw_esk_a1165_r30.jpg` (eskchat, a shape layer) | 1165, 30 | 46.5 / 45.4 | 61.8 / 90.6 | 0.457 / 0.286 | neither matches |

Reading the last rows:
- KuroCutt at Radius 100 has no outer rings in frame.
- Ilija Kostic's outer rings are plain colour, so the outer gap there is noise.
- eskchat matched neither before nor after. Its Twirl is on a shape layer and turns more than
  three whole turns. At that size a ring cannot be told from the ring a turn further on, and our
  stand-in's circle runs past After Effects'. It is reported, not used.

Our pictures are the tutorial's untwirled frame, twirled by the app at the tutorial's settings.

## Try it

1. Put a Twirl of 90 on a grid or checkerboard, with Twirl Radius 30.
   - The middle turns a quarter turn, as before.
   - Lines near the circle's edge bend a little more than before, then meet the untouched grid
     smoothly at the edge.
2. Set Angle 0.
   - Nothing changes, as before.
3. Open a project saved before today that has a Twirl.
   - It opens with no warning.
   - The outer part of its swirl is turned a little further.

## Checks

`verification/B-725_twirl_tail_table.md`: 101 of 101 checks pass. They cover:
- every Twirl fixture, FX-TWIRL-001 to 027, at its new numbers on the processor;
- the same cases on the card, within 1 level (all at 0);
- the frame the same in tiles of 1 and of 64.

`b94_twirl`, `b623_twirl` and `b107_gpu_fx` pass too.
