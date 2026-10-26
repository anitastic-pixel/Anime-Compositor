# B-748 (D-868): every layer blend mode on the graphics card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Written by `tests/b748_card_blend_modes.rs`. Each frame is drawn by the CPU and by the card. The bar: within 1 level of 255 on at least 99.9% of pixels and within 2 everywhere. A frame the card refuses must be the CPU's exactly.

## Fixtures saved with a mode beyond Normal, Multiply, Screen and Add (saved before B-748), as saved and in 8 bpc

| Check | The build's answer | Matches |
| --- | --- | --- |
| layer_blend_modes/fx_lblend_001 (hard_light), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_001 (hard_light), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_002 (darken), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_002 (darken), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_003 (lighten), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_003 (lighten), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_004 (difference), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_004 (difference), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_005 (hue), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_005 (hue), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_006 (saturation), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_006 (saturation), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_007 (color), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_007 (color), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_008 (luminosity), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_008 (luminosity), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| layer_blend_modes/fx_lblend_009 (color_dodge, color_dodge, color_dodge, color_dodge), as saved | refused when it opens: This project file cannot be opened, because part of it does not match the project format. | yes |
| layer_blend_modes/fx_lblend_009 (color_dodge, color_dodge, color_dodge, color_dodge), 8 bpc | refused when it opens: This project file cannot be opened, because part of it does not match the project format. | yes |
| preserve_transparency/fx_preserve_005 (overlay), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| preserve_transparency/fx_preserve_005 (overlay), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| preserve_transparency/fx_preserve_007 (stencil_alpha), as saved | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |
| preserve_transparency/fx_preserve_007 (stencil_alpha), 8 bpc | opens with 0 warnings; on the card 2 frames, 86 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1; refused 0, 0 of them the CPU's exactly | yes |

## Each mode, Preserve Transparency off and on, linear light and 8 bpc: a 384 px layer of mixed pixels at 70% over a 512 px backdrop

| Check | The build's answer | Matches |
| --- | --- | --- |
| normal, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| normal, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| normal, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| normal, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| darken, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| darken, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| darken, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| darken, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| multiply, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| multiply, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| multiply, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| multiply, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| add, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| add, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| add, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| add, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| lighten, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 3 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| screen, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| screen, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| screen, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| screen, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| overlay, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| overlay, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| overlay, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| overlay, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| soft_light, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| soft_light, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| soft_light, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| soft_light, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| hard_light, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hard_light, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| hard_light, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hard_light, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| difference, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| difference, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| difference, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| difference, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| hue, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hue, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| hue, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hue, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| saturation, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| saturation, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| saturation, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| saturation, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 2 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| color, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| color, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| color, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| color, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| luminosity, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| luminosity, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1; 1 fully transparent pixels with different hidden colours not graded (D-894), largest difference over black or white 0 | yes |
| luminosity, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| luminosity, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, linear, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, linear, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, 8 bpc, Preserve Transparency off | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, 8 bpc, Preserve Transparency on | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha scaled to nothing | the card refuses it, so the CPU draws it; the CPU's frame is clear | yes |
| stencil_luma scaled to nothing | the card refuses it, so the CPU draws it; the CPU's frame is clear | yes |
| normal, Float | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| darken, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| multiply, Float | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| add, Float | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| screen, Float | on the card: 1 frames, 262144 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| overlay, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| soft_light, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| hard_light, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| difference, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| hue, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| saturation, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| color, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| luminosity, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| stencil_alpha, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |
| stencil_luma, Float | the card refused: The CPU drew this frame: in Float a layer has a blend mode the GPU does not draw. | yes |

## Every fixture project with two layers or more, its upper layers set to one of the twelve modes, first frame, Full

| Check | The build's answer | Matches |
| --- | --- | --- |
| color, 8 bpc | on the card: 137 frames, 25988 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| color, as saved | on the card: 136 frames, 25828 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| darken, 8 bpc | on the card: 136 frames, 26092 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| darken, as saved | on the card: 136 frames, 26092 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| difference, 8 bpc | on the card: 138 frames, 2104936 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| difference, as saved | on the card: 137 frames, 2104776 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hard_light, 8 bpc | on the card: 136 frames, 2103184 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hard_light, as saved | on the card: 136 frames, 2103184 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hue, 8 bpc | on the card: 135 frames, 31004 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| hue, as saved | on the card: 135 frames, 31004 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, 8 bpc | on the card: 140 frames, 26584 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| lighten, as saved | on the card: 140 frames, 26584 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| luminosity, 8 bpc | on the card: 136 frames, 25820 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| luminosity, as saved | on the card: 135 frames, 25660 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| overlay, 8 bpc | on the card: 138 frames, 4177192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| overlay, as saved | on the card: 138 frames, 4177192 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| saturation, 8 bpc | on the card: 135 frames, 29872 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| saturation, as saved | on the card: 134 frames, 29712 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| soft_light, 8 bpc | on the card: 138 frames, 2103752 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| soft_light, as saved | on the card: 138 frames, 2103752 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, 8 bpc | on the card: 135 frames, 2099216 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_alpha, as saved | on the card: 135 frames, 2099216 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, 8 bpc | on the card: 130 frames, 24752 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| stencil_luma, as saved | on the card: 130 frames, 24752 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| 1718 fixture projects | 172 frames the card refused, 172 of them the CPU's exactly | yes |

## Result

129 of 129 checks pass.
