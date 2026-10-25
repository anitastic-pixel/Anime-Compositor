# D-830: effects along an adjustment layer's masks

B-710. Before D-830 each effect below, on an adjustment layer, was not drawn and said EFFECT_PATH_MISSING every frame (D-786, D-791). Now it draws along the adjustment layer's own masks on the frame beneath, as After Effects' does. The bands are `Fixtures/effect_mix`'s; the masks are two, drawn as paths only (mode None), so the adjustment layer covers the whole frame. Each expected frame is the same effect on the band layer itself carrying the same masks, drawn by code that existed before D-830; "unlike the frame without it" shows the effect drew something. Fill's own cases are FX-FILLADJ-007, 008 and 010 to 012 (B-666's table).

## Along the adjustment layer's masks, as on the layer itself

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path Stroke on Mask 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Path Stroke on Mask 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Path Stroke on All Masks, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Path Stroke on All Masks, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Vegas on Mask 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Vegas on Mask 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Energy Stroke along the masks, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Energy Stroke along the masks, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Energy Stroke between its points, Alpha Mode Masks, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Energy Stroke between its points, Alpha Mode Masks, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Audio Spectrum on Path 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Audio Spectrum on Path 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Audio Waveform on Path 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Audio Waveform on Path 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Inner/Outer Key, Foreground 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Inner/Outer Key, Foreground 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Reshape, Mask 1 to Mask 2, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Reshape, Mask 1 to Mask 2, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Mask Smear, Mask 1 moved 3 across, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Mask Smear, Mask 1 moved 3 across, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Scribble on Mask 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Scribble on Mask 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's), frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's), frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Particle Playground, Wall Mask 1, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Particle Playground, Wall Mask 1, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Fill, Fill Mask 2, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Fill, Fill Mask 2, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Fill, All Masks, frame 0: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |
| Fill, All Masks, frame 2: on the adjustment layer as on the layer itself, nothing said | largest difference 0.0e0, unlike the frame without it, said [] | yes |

## The masks go with the adjustment layer's transform

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path Stroke on Mask 1: the adjustment layer at 200% and 1 pixel right, as its masks so moved on the layer itself | largest difference 0.0e0, unlike the masks unmoved, said [] | yes |
| Fill, Fill Mask 1 (feathered): the adjustment layer at 200% and 1 pixel right, as its masks so moved on the layer itself | largest difference 0.0e0, unlike the masks unmoved, said [] | yes |
| Scribble on Mask 1: the adjustment layer at 200% and 1 pixel right, as its masks so moved on the layer itself | largest difference 0.0e0, unlike the masks unmoved, said [] | yes |
| Reshape, Mask 1 to Mask 2: the adjustment layer at 200% and 1 pixel right, as its masks so moved on the layer itself | largest difference 0.0e0, unlike the masks unmoved, said [] | yes |

## On the graphics card

| Check | The build's answer | Matches |
| --- | --- | --- |
| Path Stroke on Mask 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Path Stroke on All Masks: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Vegas on Mask 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Energy Stroke along the masks: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Energy Stroke between its points, Alpha Mode Masks: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), handed to the CPU, said "" / "GPU_PREVIEW_ON_CPU" | yes |
| Audio Spectrum on Path 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Audio Waveform on Path 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Inner/Outer Key, Foreground 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), handed to the CPU, said "" / "GPU_PREVIEW_ON_CPU" | yes |
| Reshape, Mask 1 to Mask 2: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Mask Smear, Mask 1 moved 3 across: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Scribble on Mask 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Liquify with Freeze Area Mask 1 (fx_liquify_brushes_011's): the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Particle Playground, Wall Mask 1: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), handed to the CPU, said "" / "GPU_PREVIEW_ON_CPU" | yes |
| Fill, Fill Mask 2: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |
| Fill, All Masks: the card's frame within 1 level of the CPU's | 0 levels at most (0 pixels differ), drawn on the card, said "" / "" | yes |

## Old projects

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_adj_004.json opens, saves and opens again holding what it held | the same | yes |
| fx_filladj_004.json opens, saves and opens again holding what it held | the same | yes |
| fx_filladj_001.json opens, saves and opens again holding what it held | the same | yes |

## Result

52 of 52 checks pass.
