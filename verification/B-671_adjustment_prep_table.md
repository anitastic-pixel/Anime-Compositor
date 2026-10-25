# D-791: effects prepared from their own layer, on an adjustment layer

B-671. Before D-791 each effect below, on an adjustment layer, was left out of the frame (or drew without its frozen area, its wall or its cut) and nothing was said. Now an Audio Spectrum or Audio Waveform on no mask hears its sound and draws on the frame beneath, as After Effects' does. (Each effect that works from the layer's own masks said EFFECT_PATH_MISSING every frame until D-830, which draws it along the adjustment layer's masks: B-710's table.) The bands are `Fixtures/effect_mix`'s; the adjustment layer carries two masks drawn as paths only (mode None), so it covers the whole frame. No expected pixel comes from the new code: each is the same effect on the layer itself, or the file without what was left out.

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

18 of 18 checks pass.
