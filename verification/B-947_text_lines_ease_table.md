# D-1067: Text Animator, Based On Lines and Ease from -100 to 100

B-947. The animator here moves picked letters down 120 pixels and fades them out, Shape Square unless a row says, so a picked letter is gone. FX-TXA-001 to 023 are unchanged and pass in B-230.

## Based On Lines

| Check | The build's answer | Matches |
| --- | --- | --- |
| two lines, Square, Start 0, End 50: the first line all picked, the second not | L 1.00, I 1.00, Q 1.00, U 1.00, I 1.00, D 1.00, D 0.00, E 0.00, S 0.00, I 0.00, G 0.00, N 0.00 | yes |
| a line's spaces belong to it: "ONE TWO" then "THREE", Start 50: only THREE picked | O 0.00, N 0.00, E 0.00,   0.00, T 0.00, W 0.00, O 0.00, T 1.00, H 1.00, R 1.00, E 1.00, E 1.00 | yes |
| Based On Words on the same title: the line break ends LIQUID, so one word of two is picked (before D-1067 both were one word) | L 1.00, I 1.00, Q 1.00, U 1.00, I 1.00, D 1.00, D 0.00, E 0.00, S 0.00, I 0.00, G 0.00, N 0.00 | yes |
| 1_lines_first_picked.png: DESIGN only (LIQUID moved away and faded); 2_lines_offset_half.png: LIQUID only | drawn | yes |

## Ease from -100 to 100 (Ramp Up over eight letters, the third letter)

| Check | The build's answer | Matches |
| --- | --- | --- |
| Ease Low 50 picks the third letter less than no ease, Ease Low -50 more | Ease Low -50: 0.533; 0: 0.312; 50: 0.115 | yes |
| Ease High 50 picks it more than no ease, Ease High -50 less | Ease High -50: 0.182; 0: 0.312; 50: 0.500 | yes |
| Ease Low -100 is allowed; -150 is outside -100..100, so the animator is switched off with a message | -100 runs: true; -150 runs: false | yes |
| Ease Low -80: the pick still rises letter by letter, from 0 to 1 | [0.325, 0.535, 0.665, 0.760, 0.836, 0.896, 0.946, 0.985] | yes |

## Result

8 of 8 checks pass.
