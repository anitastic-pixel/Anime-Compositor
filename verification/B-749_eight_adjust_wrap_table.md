# B-749 (D-869): adjustment layers and Light Wrap in 8 bpc on the card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Every fixture project holding an adjustment layer or a Light Wrap, each composition set to 8 bpc (After Effects), drawn by the CPU and by the card at its first and middle frames, Full and Draft. The bar: within 1 level of 255 on at least 99.9% of pixels and within 2 everywhere. A frame the card refuses must be the CPU's exactly, and none for being 8 bpc.

## The frames the card drew

| Check | The build's answer | Matches |
| --- | --- | --- |
| An adjustment layer | 70 fixtures, 280 frames drawn on the card (268 byte for byte), 8420 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| A Light Wrap | 24 fixtures, 96 frames drawn on the card (96 byte for byte), 8256 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |

## The frames the card refused, by its reason

| Check | The build's answer | Matches |
| --- | --- | --- |
| The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet. | 4 frames, 4 of them the CPU's exactly | yes |
| The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | 84 frames, 84 of them the CPU's exactly | yes |

## All the frames

| Check | The build's answer | Matches |
| --- | --- | --- |
| 115 fixture projects | frames with a pixel over 1 level: none | yes |

## Result

5 of 5 checks pass.
