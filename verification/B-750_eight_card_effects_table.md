# B-750 (D-870): Exposure, Drop Shadow, Fill, Solid Composite and Bloom in 8 bpc on the card

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. Every fixture project holding one of the five, each composition set to 8 bpc (After Effects), drawn by the CPU and by the card at its first and middle frames, Full and Draft. The bar: within 1 level of 255 on at least 99.9% of pixels and within 2 everywhere. A frame the card refuses must be the CPU's exactly. Each row counts every frame of a fixture holding the effect; "on the card" counts those whose plan leaves that effect to the card.

## The frames the card drew, as saved

| Check | The build's answer | Matches |
| --- | --- | --- |
| Exposure | 128 fixtures, 512 frames drawn on the card (240 with it on the card, 504 byte for byte), 56620 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Drop Shadow | 45 fixtures, 176 frames drawn on the card (140 with it on the card, 176 byte for byte), 14820 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fill | 56 fixtures, 224 frames drawn on the card (110 with it on the card, 195 byte for byte), 15472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Solid Composite | 31 fixtures, 124 frames drawn on the card (76 with it on the card, 118 byte for byte), 14044 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bloom | 28 fixtures, 112 frames drawn on the card (66 with it on the card, 112 byte for byte), 9632 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |

## The frames the card refused, as saved

| Check | The build's answer | Matches |
| --- | --- | --- |
| The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | 16 frames, 16 of them the CPU's exactly | yes |

## The frames the card drew, each of the five at Mix 50

| Check | The build's answer | Matches |
| --- | --- | --- |
| Exposure | 128 fixtures, 512 frames drawn on the card (228 with it on the card, 509 byte for byte), 56620 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Drop Shadow | 45 fixtures, 176 frames drawn on the card (140 with it on the card, 176 byte for byte), 14820 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Fill | 56 fixtures, 224 frames drawn on the card (110 with it on the card, 222 byte for byte), 15472 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Solid Composite | 31 fixtures, 124 frames drawn on the card (76 with it on the card, 122 byte for byte), 14044 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |
| Bloom | 28 fixtures, 112 frames drawn on the card (66 with it on the card, 112 byte for byte), 9632 pixels: largest 0, 0 over 1, 0 over 2, 100.0000% within 1 | yes |

## The frames the card refused, each of the five at Mix 50

| Check | The build's answer | Matches |
| --- | --- | --- |
| The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | 16 frames, 16 of them the CPU's exactly | yes |

## The frames the card drew, every Exposure with Bypass on

| Check | The build's answer | Matches |
| --- | --- | --- |
| Exposure | 128 fixtures, 512 frames drawn on the card (236 with it on the card, 448 byte for byte), 56620 pixels: largest 1, 0 over 1, 0 over 2, 100.0000% within 1 | yes |

## The frames the card refused, every Exposure with Bypass on

| Check | The build's answer | Matches |
| --- | --- | --- |
| The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there. | 16 frames, 16 of them the CPU's exactly | yes |

## Frames with a pixel over 1 level

| Check | The build's answer | Matches |
| --- | --- | --- |
| all three | none | yes |

## Result

15 of 15 checks pass.
