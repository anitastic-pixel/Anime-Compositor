# D-1078: Noise, Clip Result Values

B-958. A 240 by 60 ramp, black at the left to white at the right, with a mono Noise of amount 40 (each channel moves up to 0.2 through the sRGB curve). Clip on holds a channel inside black and white, as Noise always did; Clip off wraps one that would pass white round to dark and one that would pass black round to light, as After Effects' 8-bit Noise does.

## Clip on is Noise as it was

| Check | The build's answer | Matches |
| --- | --- | --- |
| a Noise with no clip word and one with Clip on draw the same, bit for bit | the same | yes |

## Clip off wraps only near black and white

| Check | The build's answer | Matches |
| --- | --- | --- |
| in the middle third of the ramp, where the grain cannot pass black or white, every pixel is Clip on's | 4800 of 4800 the same | yes |
| at the white end, Clip on holds many pixels at exactly white (255) | 313 of 720 | yes |
| and Clip off turns those round to dark (below 128) instead: the speckle of dark dots on white | 307 of 720 | yes |
| at the black end, Clip on holds many at exactly black (0), and Clip off turns them round to light (above 128) | 326 held, 323 light, of 720 | yes |
| under the grain is the ramp: amount 0, the middle row's level at the left, middle and right | 1, 128, 254 | yes |

## On the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| the viewer with the card draws Clip off as the processor does (it hands a wrap to the CPU) | largest difference 1 level(s) | yes |
| clip.png: the ramp with Noise 40, Clip on above, Clip off below | drawn | yes |

## Result

8 of 8 checks pass.
