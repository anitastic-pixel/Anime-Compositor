# D-791: effects prepared from their own layer, on an adjustment layer

B-671. Before D-791 each effect below, on an adjustment layer, was left out of the frame (or drew without its frozen area, its wall or its cut) and nothing was said. Now an Audio Spectrum or Audio Waveform on no mask hears its sound and draws on the frame beneath, as After Effects' does, and each effect that works from the layer's own masks, shapes or text says EFFECT_PATH_MISSING every frame and draws as it did. The bands are `Fixtures/effect_mix`'s; the adjustment layer carries two masks drawn as paths only (mode None), so it covers the whole frame. No expected pixel comes from the new code: each is the same effect on the layer itself, or the file without what was left out.

## Drawn on the frame beneath, as on the layer itself

| Check | The build's answer | Matches |
| --- | --- | --- |
| Audio Spectrum, no Audio Layer, Path 0, frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Spectrum, no Audio Layer, Path 0, frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Waveform, no Audio Layer, Path 0, frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Waveform, no Audio Layer, Path 0, frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Spectrum hearing a sound layer (fx_aspec_002), frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Spectrum hearing a sound layer (fx_aspec_002), frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Waveform hearing a sound layer (fx_awave_002), frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Audio Waveform hearing a sound layer (fx_awave_002), frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Energy Stroke between its points (worked before D-791 too), frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Energy Stroke between its points (worked before D-791 too), frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Particle Playground with no wall (worked before D-791 too), frame 0: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |
| Particle Playground with no wall (worked before D-791 too), frame 2: on the adjustment layer as on the layer itself, nothing said | sample-identical, unlike the adjustment layer without it, said [] | yes |

## Said every frame, the frame as it was

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path Stroke on Mask 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Path Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Path Stroke on Mask 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Path Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Path Stroke on Mask 1, on the layer itself: nothing said | [] | yes |
| Path Stroke on All Masks, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Path Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Path Stroke on All Masks, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Path Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Path Stroke on All Masks, on the layer itself: nothing said | [] | yes |
| Vegas on Mask 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Vegas works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Vegas on Mask 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Vegas works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Vegas on Mask 1, on the layer itself: nothing said | [] | yes |
| Energy Stroke along the masks, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Energy Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Energy Stroke along the masks, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Energy Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Energy Stroke along the masks, on the layer itself: nothing said | [] | yes |
| Energy Stroke between its points, Alpha Mode Masks, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Energy Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Energy Stroke between its points, Alpha Mode Masks, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Energy Stroke works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Energy Stroke between its points, Alpha Mode Masks, on the layer itself: nothing said | [] | yes |
| Audio Spectrum on Path 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Audio Spectrum works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Audio Spectrum on Path 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Audio Spectrum works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Audio Spectrum on Path 1, on the layer itself: nothing said | [] | yes |
| Audio Waveform on Path 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Audio Waveform works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Audio Waveform on Path 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Audio Waveform works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Audio Waveform on Path 1, on the layer itself: nothing said | [] | yes |
| Inner/Outer Key, Foreground 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Inner/Outer Key works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Inner/Outer Key, Foreground 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Inner/Outer Key works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Inner/Outer Key, Foreground 1, on the layer itself: nothing said | [] | yes |
| Reshape, Mask 1 to Mask 2, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Reshape works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Reshape, Mask 1 to Mask 2, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Reshape works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Reshape, Mask 1 to Mask 2, on the layer itself: nothing said | [] | yes |
| Mask Smear, Mask 1 moved 3 across, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Mask Smear works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Mask Smear, Mask 1 moved 3 across, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Mask Smear works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Mask Smear, Mask 1 moved 3 across, on the layer itself: nothing said | [] | yes |
| Scribble on Mask 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Scribble works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Scribble on Mask 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Scribble works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Scribble on Mask 1, on the layer itself: nothing said | [] | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's), frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Liquify works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's), frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Liquify works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's), on the layer itself: nothing said | [] | yes |
| Particle Playground, Wall Mask 1, frame 0: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Particle Playground works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Particle Playground, Wall Mask 1, frame 2: drawn as before D-791, and said | sample-identical, said ["EFFECT_PATH_MISSING Adjustment layer adj's Particle Playground works from a mask, a shape or the text of its own layer, which an effect on an adjustment layer cannot use yet."] | yes |
| Particle Playground, Wall Mask 1, on the layer itself: nothing said | [] | yes |

## Nothing named, nothing said (as before D-791)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Inner/Outer Key, Foreground 0: the frame as without it, nothing said | sample-identical, said [] | yes |
| Reshape, no masks: the frame as without it, nothing said | sample-identical, said [] | yes |
| Mask Smear, no masks: the frame as without it, nothing said | sample-identical, said [] | yes |

## Old projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_aspec_002.json, written before D-791, opens and saves holding what it held | the same | yes |
| fx_awave_002.json, written before D-791, opens and saves holding what it held | the same | yes |
| fx_filladj_001.json, written before D-791, opens and saves holding what it held | the same | yes |

## Result

57 of 57 checks pass.
