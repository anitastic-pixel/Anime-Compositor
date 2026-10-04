# D-302 / B-187: Curves has an Alpha curve

Found by P-26, tutorial 3 (Video Copilot, Colorful Glitch). It uses Curves' Alpha channel to make a layer partly see-through. Here, Curves could only bend Master, Red, Green and Blue.

## What changed

Curves' channel list now reads **Master, Red, Green, Blue, Alpha**. The Alpha curve bends how solid each pixel is, from 0 (not there) to 255 (fully there), and leaves its colour alone. It starts straight, so a Curves you have already made looks the same. A file only writes the Alpha curve when it is bent, or when the file already had one.

The graphics card bends colour only, so a Curves with a bent Alpha curve is drawn on the CPU. The picture is the same; only the speed differs.

**Found on the way, and fixed.** Two save problems:
- A setting added in this round (Fractal Noise's Fractal Type, Noise Type, Invert, Offset, Scale, Cycle; Lightning's Composite on Original; Tint's Preserve Luminosity) that you put back to where it starts was saved with the file's old value instead.
- A keyed Fractal Noise Offset, Scale or Cycle whose plain value sat at its start lost its keys on save.

Both are now saved as you left them.

## Checks (cargo test)

`tests/b187_curves_alpha.rs`, 4 of 4 pass. Worked by hand on a 16x16 solid of colour (0.2, 0.4, 0.6):

| Check | Expected | Got |
|---|---|---|
| Alpha curve 0→0, 255→128 | covering 128/255 = 0.502, colour kept, so (0.2, 0.4, 0.6) x 0.502 | so, to 0.00001 |
| Then a second Curves, 0→0, 128→255 | solid again, colour (0.2, 0.4, 0.6) back | so |
| Alpha to 0, then lifted to 64 | black at 64/255 = 0.251 (a pixel that was gone has no colour) | so |
| A straight Alpha curve against none | the same picture | the same |
| An Alpha curve with 1 point | "Curves' alpha curve takes 2 to 16 points, and this has 1." | so |
| Saving: straight / a file that wrote it / bent / bent then put back straight | not written / kept / kept / saved straight | so |
| Fractal Noise turbulent→basic and cycle 3→0, saved | basic and 0, not the old values | so |
| Fractal Noise Scale Width keyed at 100 and 150 | both keys saved | so |
| Lightning Composite off→on, saved | on | so |

Unchanged and still passing: the Curves fixtures FX-CURVES-001 to 020 (`tests/b54_curves.rs`), effect cost (`tests/p16_effect_cost.rs`), the whole core suite (263 targets; only the two old scratch files `zz_scratch_g2` and `zz_scratch_p25` fail, as before this work) and the app suite (89 pass).

## Pictures (the test copy, never the owner's app)

Orange-and-blue Fractal Noise clouds, and a pink 320x180 solid with Curves on it.

| Picture | Alpha curve | Look for | Pass? |
|---|---|---|---|
| `D-302 pictures/1_alpha_straight_as_before.png` | straight | a flat pink box, as before | pass |
| `D-302 pictures/2_alpha_to_half.png` | 0→0, 255→128 | the box is half see-through, and the clouds show through it | pass |
| `D-302 pictures/3_alpha_to_none.png` | 0→0, 255→0 | the box is gone; only its outline (the selection) shows | pass |

The window run also read Curves' channel list: "Master, Red, Green, Blue, Alpha".

## For the owner to try

1. Put Curves on any layer above another.
2. Pick **Alpha** in the channel list and drag the top-right point down: the layer fades.
3. Save, close and reopen: the curve is still bent.

Fixtures are unchanged.
