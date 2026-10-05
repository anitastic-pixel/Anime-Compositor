# D-325: a blurred layer keeps its own outline

Written by `d325_a_blurred_layer_keeps_its_own_outline` in `app/src/main.rs`. The outline is the window's `/boxes` answer, which the page draws the layer's box from and carries every mask point through. The layer is a white 1920 by 1080 solid cut by a mask from 744,369 to 1176,702, the owner's case of 4 October at this size.

**7 of 7 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| the masked solid's outline, no effect, Full | 0,0 1920,0 1920,1080 0,1080 | 0,0 1920,0 1920,1080 0,1080 | PASS |
| with Gaussian Blur 20, Full (was 60 wider on every side: -60,-60 1980,-60 ...) | 0,0 1920,0 1920,1080 0,1080 | 0,0 1920,0 1920,1080 0,1080 | PASS |
| and Draft, a quarter of it | 0,0 480,0 480,270 0,270 | 0,0 480,0 480,270 0,270 | PASS |
| the mask's top-left corner, carried to the screen as the page carries it | 744,369 | 744,369 | PASS |
| and its bottom-right corner | 1176,702 | 1176,702 | PASS |
| the blurred picture's middle (over half covered) | 960,535 | 960,535 | PASS |
| the mask's middle | 960,535 | 960,535 | PASS |
