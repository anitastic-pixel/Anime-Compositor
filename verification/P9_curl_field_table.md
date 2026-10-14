# Audit P9: the card keeps Curl Noise's field

Written by `tests/p9_curl_field.rs`. The card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory.

Curl Noise first works out a field (which way each flow line runs, and the grey it starts from), then draws the flow lines through it. The field comes from Curl Noise's own noise, its settings and the time, never from the picture, so the card now keeps it and draws it again only when one of those changes. A Source This Layer field is made from the picture, so it is drawn every time, as before.

Each case puts a Levels, then a Curl Noise, on the reference shot's first three layers, draws a first frame, then a second whose picture underneath has changed. The second is then drawn again on a card that has forgotten everything.

Before this change the card drew the field for each of the three layers, every frame (13 passes for these frames). The three layers are the same size with the same settings, so they now share one field (11 passes on a forgotten card).

**The rule:** where the field is kept, the second frame runs exactly 1 pass fewer than on the forgotten card (the one shared field); where it must be drawn again, the same number. Either way the second frame is **the very same picture as the forgotten card's** (0 levels, 0 pixels apart) and **within 1 level of 255 of the processor's**. One row keeps its old gap: Source This Layer at Full was 8 levels from the processor on 5024 pixels before this change too (its flow lines follow the picture's own field, which parts on a hair after the Levels, D-583); this change does not touch that field, so that row passes when it is no further than before.

**12 of 12 checks pass.**

| Case | Quality | The field must be | Passes run | Passes on a forgotten card | The field was | Difference from the forgotten card's picture (of 255) | Difference from the processor's (of 255) | Result |
|---|---|---|---:|---:|---|---|---|---|
| Levels keyed, so the picture changes; Curl Noise still (Speed 0, Turbulence Speed 0); frame 0 then frame 100 | Full | kept (1 pass fewer) | 10 | 11 | kept | 0, 0 pixels | 1, 1190 pixels | PASS |
| Levels keyed, so the picture changes; Curl Noise still (Speed 0, Turbulence Speed 0); frame 0 then frame 100 | Draft | kept (1 pass fewer) | 10 | 11 | kept | 0, 0 pixels | 1, 79 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5; Curl Noise as added; frame 50 both times | Full | kept (1 pass fewer) | 10 | 11 | kept | 0, 0 pixels | 1, 1342 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5; Curl Noise as added; frame 50 both times | Draft | kept (1 pass fewer) | 10 | 11 | kept | 0, 0 pixels | 1, 87 pixels | PASS |
| Levels keyed; Curl Noise moving with time (Speed 10, Turbulence Speed 20, as added); frame 0 then frame 100 | Full | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 1251 pixels | PASS |
| Levels keyed; Curl Noise moving with time (Speed 10, Turbulence Speed 20, as added); frame 0 then frame 100 | Draft | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 89 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5 and Swirl from 45 to 90; frame 50 both times | Full | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 1350 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5 and Swirl from 45 to 90; frame 50 both times | Draft | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 69 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5 and Size from 100 to 60; frame 50 both times | Full | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 1278 pixels | PASS |
| Levels changed from gamma 1.2 to 1.5 and Size from 100 to 60; frame 50 both times | Draft | drawn again (no fewer) | 11 | 11 | 0 fewer | 0, 0 pixels | 1, 77 pixels | PASS |
| Source This Layer (the field is the picture's own); Levels keyed; Curl Noise still; frame 0 then frame 100 | Full | drawn again (no fewer) | 16 | 16 | 0 fewer | 0, 0 pixels | 8, 5024 pixels | PASS |
| Source This Layer (the field is the picture's own); Levels keyed; Curl Noise still; frame 0 then frame 100 | Draft | drawn again (no fewer) | 16 | 16 | 0 fewer | 0, 0 pixels | 1, 96 pixels | PASS |
