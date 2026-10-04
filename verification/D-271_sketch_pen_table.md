# D-271: a pen's pressure in a sketch

Written by `tests/d271_sketch_pen.rs`.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| A pen stroke, light to firm, is taken | taken | taken | PASS |
| A mouse stroke, with no pressure, is taken | taken | taken | PASS |
| A stroke with fewer pressures than points is refused | refused | refused | PASS |
| A pressure past 1 is refused | refused | refused | PASS |
| A pressure that is not a number is refused | refused | refused | PASS |
| The pen stroke is saved with its pressure | [0.1,0.55,1] | [0.1,0.55,1] | PASS |
| The mouse stroke is saved with no pressure line | no line | no line | PASS |
| Opened again, both strokes are the same, every pressure | the same | the same | PASS |
| Saved again, the text is the same, byte for byte | the same | the same | PASS |
| Undo twice takes back both strokes | 0 | 0 | PASS |
| A file with one pressure for three points is refused, saying where | refused at /strokes/0/pressure | refused at /strokes/0/pressure | PASS |
