# B-427: the card's remainders

D-547: this machine's graphics card takes the remainder of a negative whole number as if it had no sign (-7 % 6 gave 3, not -1; -2 wrapped into 7 gave 2, not 5). Every `%` in the card's code was read. The noise's period, used by Noise Alpha's Cycle Noise and Fractal Noise's Cycle Evolution, was the only one that could meet a negative number with any divisor: a negative Noise Phase or Evolution with a cycle that is not a power of two drew a different noise on the card than on the processor. Each case below is the reference shot with the effect on its first three layers, at frames 0, 100 and 239, Full and Draft, the card's picture against the processor's, within 1 level of 255 (ADR-006, D-100). The cases with a cycle of 2 or a positive depth agreed before the fix too.

## Noise Alpha, phase -100, cycle 3

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2064903 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 1314 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 1294 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 1248 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 295 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 254 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 293 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Noise Alpha, phase -1000, cycle 7

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2065588 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 1283 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 1259 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 1234 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 361 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 332 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 376 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Noise Alpha, phase -100, cycle 2 (agreed before)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2064917 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 1300 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 1262 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 1167 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 352 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 292 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 337 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Noise Alpha, phase 500, cycle 3 (agreed before)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2062904 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 1313 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 1206 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 1193 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 355 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 311 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 352 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Fractal Noise, evolution -500, cycle 3

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2040598 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 880 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 961 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 783 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 39 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 70 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 42 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Fractal Noise, turbulent block, evolution -2000, cycle 5

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2043056 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 918 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 1028 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 806 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 64 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 44 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Fractal Noise, evolution -500, cycle 2 (agreed before)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2041772 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 845 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 964 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 747 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 66 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 63 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 61 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Fractal Noise, evolution 1500, cycle 3 (agreed before)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the processor's frame 0 differs from the same with no cycle, so the cycle is in play | 2043869 pixels changed | yes |
| frame 0, Full | largest difference 1 of 255, 859 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Full | largest difference 1 of 255, 994 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Full | largest difference 1 of 255, 756 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 0, Draft | largest difference 1 of 255, 55 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 100, Draft | largest difference 1 of 255, 76 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| frame 239, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Result

56 of 56 checks pass.
