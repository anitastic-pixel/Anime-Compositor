# B-870: the Text Animator's Blur

D-990: After Effects' animator Blur, x then y in pixels, each letter blurred by Blur times how much the Range Selector picks it (a Gaussian of sigma half the Blur, as the text shadow's Softness). The words are "BLUR IN" at 160 pixels on a 1280 by 360 frame. Pictures are in `verification/B-870 pictures/`.

## A file from before Blur

| Check | The build's answer | Matches |
| --- | --- | --- |
| reads with Blur 0, 0 | true | yes |
| saves with no blur key, as before | true | yes |
| Blur 0, 0 draws the same pixels as no Blur | true | yes |

## Blur 24, 24 on every letter

| Check | The build's answer | Matches |
| --- | --- | --- |
| the letters' coverage is kept (within 1 per cent) | 1.0000 of the sharp | yes |
| no pixel is fully covered any more | 1.000 then 0.580 | yes |
| the letters spread past their edges on all four sides | [173, 121, 740, 241] then [143, 91, 771, 271] | yes |

## Blur 24, 0: across only

| Check | The build's answer | Matches |
| --- | --- | --- |
| the rows reached stay; the columns spread | [173, 121, 740, 241] then [143, 121, 771, 241] | yes |

## Start at 60 per cent: the selector picks the last letters only

| Check | The build's answer | Matches |
| --- | --- | --- |
| each letter's blur | B0 L0 U0 R0  24 I24 N24 | yes |
| "B" and "L" are the sharp pixels to the bit | true | yes |
| "N" is blurred | true | yes |

## Saved, read again, and set

| Check | The build's answer | Matches |
| --- | --- | --- |
| Blur 24, 12 is saved | [24,12] | yes |
| and read again as it was | true | yes |
| Blur past 1,000 pixels is refused, not drawn | true | yes |

## Result

13 of 13 checks pass.
