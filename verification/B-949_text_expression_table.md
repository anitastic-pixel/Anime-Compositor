# D-1069: Text Animator, the Expression Selector

B-949. The title is "Bounce" with an animator moving picked letters 200 pixels up, the Range Selector picking every letter, and the Expression Selector on, with the expression a row gives on its Amount. Each number is how much the animator picks a letter (1 is fully, 200 pixels up; 0 is in its place; below 0 is below its place). FX-TXA-001 to 023 are unchanged and pass in B-230.

## Off is as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| a file with no Expression words and one with it off (and an expression of 0) pick the same: End 50, the first half | 1.000, 1.000, 1.000, 0.000, 0.000, 0.000 | yes |

## What the expression is told

| Check | The build's answer | Matches |
| --- | --- | --- |
| textIndex*10: 10, 20, ... 60 per cent | 0.100, 0.200, 0.300, 0.400, 0.500, 0.600 | yes |
| textTotal*10: six letters, 60 per cent each | 0.600, 0.600, 0.600, 0.600, 0.600, 0.600 | yes |
| selectorValue[0]*textIndex/textTotal with End 50: the first half ramps, the rest 0 | 0.167, 0.333, 0.500, 0.000, 0.000, 0.000 | yes |
| a list answer takes its first number: 30 per cent | 0.300, 0.300, 0.300, 0.300, 0.300, 0.300 | yes |
| answers past 100 or -100 are held there: -150, -100, -50, 0, 50, 100 | -1.000, -1.000, -0.500, 0.000, 0.500, 1.000 | yes |
| no expression: every letter takes the Amount, 40 | 0.400, 0.400, 0.400, 0.400, 0.400, 0.400 | yes |
| the animator's own Amount 50 comes after: half of each | 0.050, 0.100, 0.150, 0.200, 0.250, 0.300 | yes |
| inPoint is the layer's first frame in seconds: 0 here, 0.5 for a layer starting at frame 12 | 0.100, 0.100, 0.100, 0.100, 0.100, 0.100 | 50 | yes |
| Math.exp: e to the power, 20 + 2.718 per cent | 0.227, 0.227, 0.227, 0.227, 0.227, 0.227 | yes |

## The tutorial's bounce, letter by letter (24 frames a second)

| Check | The build's answer | Matches |
| --- | --- | --- |
| frame 0 | 1.000, 1.000, 1.000, 1.000, 1.000, 1.000 | yes |
| frame 1 | 1.000, 1.000, 1.000, 1.000, 1.000, 1.000 | yes |
| frame 2 | 0.583, 1.000, 1.000, 1.000, 1.000, 1.000 | yes |
| frame 3 | 0.167, 0.583, 1.000, 1.000, 1.000, 1.000 | yes |
| frame 4 | -0.220, 0.167, 0.583, 1.000, 1.000, 1.000 | yes |
| frame 6 | -0.583, -0.464, -0.220, 0.167, 0.583, 1.000 | yes |
| frame 9 | -0.493, -0.570, -0.608, -0.583, -0.464, -0.220 | yes |
| frame 14 | -0.063, -0.127, -0.206, -0.298, -0.397, -0.493 | yes |
| frame 24 | 0.017, 0.025, 0.033, 0.040, 0.047, 0.050 | yes |
| frame 40 | 0.000, -0.000, -0.000, -0.001, -0.001, -0.002 | yes |
| at frame 2 the first letter has started down and the second has not | 0.583, 1.000, 1.000, 1.000, 1.000, 1.000 | yes |
| the first letter overshoots below its place before it settles | -0.570, -0.608, -0.583, -0.464, -0.220, 0.167 | yes |
| by frame 40 every letter has settled to within 1 per cent of its place | 0.000, -0.000, -0.000, -0.001, -0.001, -0.002 | yes |

## Errors

| Check | The build's answer | Matches |
| --- | --- | --- |
| an expression giving not-a-number (the square root of a negative) puts every letter at the Amount (30) and is reported | 0.300, 0.300, 0.300, 0.300, 0.300, 0.300; expression_amount: The expression gives a number that is not finite | yes |
| textIndex on another setting is an error that says where it has a value | rotation: textIndex has a value only on a Text Animator's Expression Selector | yes |
| an unknown Expression Selector word switches the animator off | runs: false | yes |

## Through compose, and saved

| Check | The build's answer | Matches |
| --- | --- | --- |
| frame 40 of the bounce is nearly the letters at rest (under a pixel off, edges only); frame 4 is far from it | mean difference 0.00041 at frame 40, 0.04888 at frame 4 | yes |
| frame 2 of the bounce differs from frame 40 | compared | yes |
| saved with it on, both words are written, the expression with them | selector "on", amount expression kept true | yes |
| a file from before it saves again with no Expression words | no expression_ words | yes |
| bounce_frames.png: frames 0, 2, 4, 6, 9 and 24 from the top; the letters drop in one after another and bounce | drawn | yes |

## Result

31 of 31 checks pass.
