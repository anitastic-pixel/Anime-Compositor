# B-656 playtest: Hue/Saturation gains After Effects' Colorize (D-776)

## What changed for you

Hue/Saturation now has After Effects' **Colorize** switch and its three numbers: Colorize Hue
(0 to 360), Colorize Saturation (0 to 100) and Colorize Lightness (-100 to 100), with After
Effects' defaults Off, 0, 25 and 0. With Colorize on, the whole layer is painted one hue and
saturation while each pixel keeps its own lightness, the way After Effects does it; the master and
colour-range settings are not used while it is on, as After Effects greys them out.

With Colorize off, Hue/Saturation draws exactly as before. Saved projects open and save back as
they did; the new words are only written once you change them.

Colorize is drawn on the processor, not the graphics card (noted for the speed lane).

## What to look at

Each picture in `verification/B-656 pictures/` is After Effects on the left (Jake In Motion,
https://www.youtube.com/watch?v=76LQhG-nl_0, the time in the name), ours before in the middle, ours
after on the right. The numbers are the median difference, out of 255, at 13 points of the photo.

| Picture | Settings | What to judge | Median gap before | Median gap after | Verdict |
| --- | --- | --- | --- | --- | --- |
| `cz_t224_defaults.jpg` | 2:24, Colorize on at 0 / 25 / 0 | a dusty pink photo | 93.6 | 1.9 | matches |
| `cz_t230_hue80.jpg` | 2:30, hue 80 | a dusty olive photo | 105.9 | 1.5 | matches |
| `cz_t242_hue180.jpg` | 2:42, hue 180, saturation 70, lightness -4 | a strong cyan photo | 34.6 | 2.3 | matches |
| `cz_t248_hue180.jpg` | 2:48, the same settings, another frame | the same cyan | 34.1 | 2.4 | matches |
| `cz_t254_off.jpg` | 2:54, Colorize unchecked | the photo unchanged, before and after alike | 2.3 | 2.3 | matches (unchanged) |

The largest single-point gaps after are 10 to 15 levels, on edges where After Effects' half-size
frame does not line up with ours. Before, the old build had no Colorize and drew the photo
unchanged, so its gaps are the whole effect.

The fixture checks are in `verification/B-656_hue_saturation_colorize_table.md` (52 of 52 pass).

## Try it

1. Put Hue/Saturation on a colourful picture. Set Colorize to On: the picture turns a dusty pink.
2. Turn Colorize Hue to 180 and Colorize Saturation to 70: a strong cyan, dark parts still dark.
3. Move Master Hue: nothing changes while Colorize is on. Set Colorize Off: the picture is back.

## Known differences from After Effects (not fixed here)

- Only one creator shows Colorize settled (Jake In Motion); a raised Colorize Lightness is taken to
  work as the master Lightness does, not seen settled (2:36 is mid-drag).
- The small hue strip preview in Effect Controls does not show the Colorize colour.
- After Effects' 16-bit and 32-bit Colorize are not seen.
