# B-774 (D-894): pixels nobody can see are not graded

The rule, for the card's straight 8-bit viewer picture against the CPU's: where both alpha bytes are 0, the colours are not compared. Each pair is also composited over black and over white, as a viewer would see it, rounded to 8 bits.

## The rule against what a viewer sees

| Check | The build's answer | Matches |
| --- | --- | --- |
| 13312 pairs of different hidden colours, both alpha 0 | graded difference 0; largest difference over black or white 0 | yes |
| alpha 0 against alpha 1, colours 255 apart | graded difference 255; excused false | yes |
| both alpha 1, colours 255 apart | graded difference 255; over black or white 1 | yes |
| the same hidden colour | graded difference 0; excused false | yes |

## Result

4 of 4 checks pass.
