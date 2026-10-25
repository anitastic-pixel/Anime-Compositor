# B-708: the T switch (Preserve Underlying Transparency) against the reference

Written by `tests/b708_preserve_transparency.rs`. Each case is `Fixtures/preserve_transparency/`'s, worked by `tools/preserve_transparency_reference.py`: a 10 by 8 frame, a backdrop of strips opaque, at 50%, opaque, at 25% and clear, under four strips in the mode, the last at 50%. CPU: the largest difference over every channel of every pixel, linear premultiplied, against the tolerance. Card: the largest difference from the CPU's frame in levels of 255, and where the frame was drawn.

| Fixture | Mode | T | Pixels | CPU largest difference | Tolerance | Card | Card vs CPU (levels) | Pass |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| FX-PRESERVE-001 | normal | on | 80 | 3.07e-8 | 2e-5 | GPU | 1 | yes |
| FX-PRESERVE-002 | multiply | on | 80 | 2.90e-8 | 2e-5 | GPU | 1 | yes |
| FX-PRESERVE-003 | screen | on | 80 | 6.85e-8 | 2e-5 | GPU | 1 | yes |
| FX-PRESERVE-004 | add | on | 80 | 3.63e-8 | 2e-5 | GPU | 0 | yes |
| FX-PRESERVE-005 | overlay | on | 80 | 4.48e-8 | 2e-5 | CPU | 0 | yes |
| FX-PRESERVE-006 | normal | on | 80 | 2.33e-8 | 2e-5 | GPU | 1 | yes |
| FX-PRESERVE-007 | stencil_alpha | on | 80 | 1.59e-9 | 2e-5 | CPU | 0 | yes |
| FX-PRESERVE-008 | normal | off | 80 | 2.33e-8 | 2e-5 | GPU | 1 | yes |
| FX-PRESERVE-020 | adjustment layer (refused) | on | - | reason: no preserve_transparency on a null or adjustment layer | - | - | - | yes |
| FX-PRESERVE-021 | null layer (refused) | on | - | reason: no preserve_transparency on a null or adjustment layer | - | - | - | yes |

B-708: 10 of 10 fixtures pass.
