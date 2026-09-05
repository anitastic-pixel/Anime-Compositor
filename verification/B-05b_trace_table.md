# B-05b render trace, fixture results

**33 of 33 checks pass.** ADR-012 render trace mode. Produced by `tests/b05b_trace.rs`.

## The fixture

Two layers, in the document 21 working space, linear light and premultiplied:

| Layer | ID | Source | Every pixel | Transform | Opacity |
|---|---|---|---|---|---|
| 0 | `bg` | 4x4 | `(0.2, 0.4, 0.6, 1.0)` | identity | 1.0 |
| 1 | `fg` | 2x2 | `(0.6, 0.0, 0.0, 1.0)` | translate `(1, 1)` | 0.5 |

The translation is a whole number of pixels, so the bilinear taps land exactly on source pixel centres and `fg` is moved without being blurred. Every expected value below follows from those two rows by hand.

| Check | Expected | Actual | |
|---|---|---|---|
| the frame returned by render_traced is byte-identical to the untraced render | `identical` | `identical` | pass |
| the top layer's composite image is byte-identical to frame.png | `identical` | `identical` | pass |
| the trace directory holds exactly the eight stage images, the frame and the manifest | `frame.png, layer00_bg_composite.png, layer00_bg_decode.png, layer00_bg_opacity.png, layer00_bg_transform.png, layer01_fg_composite.png, layer01_fg_decode.png, layer01_fg_opacity.png, layer01_fg_transform.png, manifest.md` | `frame.png, layer00_bg_composite.png, layer00_bg_decode.png, layer00_bg_opacity.png, layer00_bg_transform.png, layer01_fg_composite.png, layer01_fg_decode.png, layer01_fg_opacity.png, layer01_fg_transform.png, manifest.md` | pass |
| no image claims a stage document 21 lists but this build does not implement | `no mask, effects or matte image` | `no mask, effects or matte image` | pass |
| the fg decode image is at the layer's own 2x2 extent, not the composition's | `2x2` | `2x2` | pass |
| every stage after the transform is at the 4x4 composition extent | `4x4` | `4x4` | pass |
| transform stage: fg lands on destination pixel (1,1) unresampled | `(0.600000, 0.000000, 0.000000, 1.000000)` | `(0.600000, 0.000000, 0.000000, 1.000000)` | pass |
| transform stage: fg lands on destination pixel (2,2), the far corner of its 2x2 | `(0.600000, 0.000000, 0.000000, 1.000000)` | `(0.600000, 0.000000, 0.000000, 1.000000)` | pass |
| transform stage: an integer translation invents nothing at (0,0) | `(0.000000, 0.000000, 0.000000, 0.000000)` | `(0.000000, 0.000000, 0.000000, 0.000000)` | pass |
| transform stage: an integer translation invents nothing at (3,3) | `(0.000000, 0.000000, 0.000000, 0.000000)` | `(0.000000, 0.000000, 0.000000, 0.000000)` | pass |
| opacity stage: 0.5 halves RGB and alpha together, because the buffer is premultiplied | `(0.300000, 0.000000, 0.000000, 0.500000)` | `(0.300000, 0.000000, 0.000000, 0.500000)` | pass |
| opacity stage: opacity does not spread a layer beyond where it was | `(0.000000, 0.000000, 0.000000, 0.000000)` | `(0.000000, 0.000000, 0.000000, 0.000000)` | pass |
| composite stage after layer 0 is the background alone | `(0.200000, 0.400000, 0.600000, 1.000000)` | `(0.200000, 0.400000, 0.600000, 1.000000)` | pass |
| composite stage after layer 1: 0.3 + 0.2*0.5 = 0.4 red, 0.4*0.5 = 0.2 green | `(0.400000, 0.200000, 0.300000, 1.000000)` | `(0.400000, 0.200000, 0.300000, 1.000000)` | pass |
| composite stage after layer 1: a pixel fg does not cover keeps the background exactly | `(0.200000, 0.400000, 0.600000, 1.000000)` | `(0.200000, 0.400000, 0.600000, 1.000000)` | pass |
| frame.png pixel (0,0), background alone: linear (0.2,0.4,0.6,1) encodes to sRGB 8-bit | `(124, 170, 203, 255)` | `(124, 170, 203, 255)` | pass |
| frame.png pixel (1,1), fg over bg: linear (0.4,0.2,0.3,1) encodes to sRGB 8-bit | `(170, 124, 149, 255)` | `(170, 124, 149, 255)` | pass |
| the transform image is written before opacity, so fg is still fully opaque there | `(203, 0, 0, 255)` | `(203, 0, 0, 255)` | pass |
| the transform and opacity images are different files with different contents | `different` | `different` | pass |
| the opacity image unpremultiplies before encoding, so fg keeps its colour at half alpha | `(203, 0, 0, 128)` | `(203, 0, 0, 128)` | pass |
| a pixel the layer never covered writes fully transparent black | `(0, 0, 0, 0)` | `(0, 0, 0, 0)` | pass |
| each image names its pipeline stage | `opacity` | `opacity` | pass |
| each image names its layer | `fg` | `fg` | pass |
| each image names the document 21 step it belongs to | `6` | `6` | pass |
| each image states its own colour space | `sRGB IEC 61966-2-1, 8 bits per channel` | `sRGB IEC 61966-2-1, 8 bits per channel` | pass |
| each image states its own alpha mode | `Straight` | `Straight` | pass |
| each image states what it was converted from, so the conversion is not silent | `converted from linear light, premultiplied, float32` | `converted from linear light, premultiplied, float32` | pass |
| the finished frame carries the composition frame number it was traced at | `7` | `7` | pass |
| the manifest names the composition frame | `names frame 7` | `names frame 7` | pass |
| the manifest names every stage of document 21's order this build does not implement | `3 of 3` | `3 of 3` | pass |
| the manifest carries the exact layer IDs, not just the file names | `both` | `both` | pass |
| an ID that reduces to separators keeps what is left of it, not the separators | `layer00_2_decode.png` | `layer00_2_decode.png` | pass |
| an ID of two hundred characters is shortened to a file name a filesystem accepts | `layer01_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz_decode.png` | `layer01_zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz_decode.png` | pass |

## Notes

- Every expected value above is derived from the two fixture layers stated in the header of `tests/b05b_trace.rs`, by applying document 21's compositing and output-encoding rules by hand. The 8-bit values are the sRGB transfer function evaluated to six decimal places and quantised with document 27's rounding rule.
- The tile size for this fixture is 2, so the 4x4 composition is cut into four tiles and every stage image crosses tile seams. B-05a already proved tiling is invisible; this fixture would still catch a trace facility that assembled tiles differently.

## What this does not cover

Trace mode shows the four stages of document 21's seven-step layer render order that this build implements. It cannot show the polygon mask, layer effects, the alpha matte or the multiply, screen and add blend modes, because the renderer does not have them. Every trace manifest says so in its own words, so a trace directory can never be mistaken for a complete pipeline.
