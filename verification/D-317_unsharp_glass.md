# D-317 / B-198: Unsharp Mask's Threshold and CC Glass (reduced)

Found by P-26. The Shockwave tutorial (1) bends the picture behind its ring with After Effects' **CC Glass**, and crisps it with **Unsharp Mask**. Here the stand-ins were Displacement Map with Bevel Alpha, and Sharpen without a threshold.

## What changed

- **Sharpen** already worked like After Effects' Unsharp Mask (Amount and Radius). It now has a **Threshold** too:
  - 0 to 255, and it takes keys.
  - A difference smaller than the threshold is left alone, so flat areas and fine noise are not crisped.
  - At 0, its starting value, the picture is as before.
  - Searching the effects for "unsharp mask" finds it.
- A new effect, **CC Glass**, under Stylize. Its settings:
  - **Bump Map**: a layer (none means the layer itself). **Property** says what of it makes the bump: Alpha, Luminance, and so on.
  - **Softness**: smooths the bump.
  - **Height**: how steep the bump is. Below 0 it dips instead.
  - **Displacement**: how far its slopes bend the picture.
  - **Light Direction**, **Light Color** and **Light Intensity**: light its slopes, as Bevel Alpha does.
- Reduced: After Effects also has a light height, point lights, and Ambient, Diffuse, Specular, Roughness and Metal shading. These are left until asked for.
- Both are drawn on the CPU (Sharpen only when it has a threshold), so the preview and the export match.

## Checks (cargo test)

`tests/b198_unsharp_glass.rs`: 5 of 5 pass.

| Check | Expected | Got |
|---|---|---|
| Sharpen at Threshold 0 | exactly as before | so |
| Sharpen with a threshold | differences under it kept, the rest sharpened as before; 255 keeps everything | so |
| CC Glass at height 100 with no displacement | exactly Bevel Alpha | so |
| Height 0; no displacement and no light | unchanged | so |
| A slope bends the picture | by the amount worked by hand | so |
| Another layer as the bump map | used; a black layer gives no bump | so |
| Wrong property, colour, height and threshold | refused with a sentence | refused |
| Saved and read back | written as read | so |

FX-SHARPEN-001 to 018 are unchanged and still pass. Also still passing: the whole core suite and the app suite (90 pass, 5 set aside as before).

## Pictures (the test copy, never the owner's app)

The CC Glass setup is a 640 x 360 card of blue stripes. A hidden layer, Lens, holds a white-to-black round gradient in the middle.

| Picture | Look for | Pass? |
|---|---|---|
| `D-317 pictures/1_stripes.png` | the stripes alone | pass |
| `D-317 pictures/2_cc_glass_as_it_starts.png` | the stripes' own edges as the bump: each stripe edge bent and lit, a glassy ripple | pass |
| `D-317 pictures/3_bump_map_lens.png` | Bump Map set to Lens: a round glass lens in the middle bends the stripes around it, lit on its upper left | pass |
| `D-317 pictures/4_height_minus_100.png` | Height -100: the lens dips instead, so the stripes bend the other way and the light falls on the other side | pass |
| `D-317 pictures/5_sharpen_threshold_0.png` | Fractal Noise clouds in bands, sharpened hard (Amount 400, Radius 4): the clouds go grainy | pass |
| `D-317 pictures/6_sharpen_threshold_40.png` | Threshold 40: the soft clouds are left alone, while the band edges stay crisp | pass |

## For the owner to try

1. Put **CC Glass** on a picture with some transparent parts, such as text. Its edges turn to glass.
2. Make a layer with a soft white blob, hide it, and pick it as CC Glass's **Bump Map** with Property **Luminance**. The picture bends around the blob like a lens. Raise **Displacement** for a stronger bend.
3. Put **Sharpen** on a noisy picture with Amount around 200. Raise **Threshold**: the noise stops being crisped, but the strong edges stay sharp.
