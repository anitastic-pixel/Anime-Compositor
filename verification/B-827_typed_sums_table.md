# B-827: sums typed into number fields (D-947)

Written by `b827_sums_in_number_fields` in `app/src/tests/w12r.rs`.

Each row runs the page's own `typedSum` under Node: the text typed, the value the field held, and the number it becomes ("not taken" leaves the field as it was). The last row pins that `scrubValue`, the number field every panel uses, reads through it. The private-app check is `verification/B-827_private_app_table.md`.

**21 of 21 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| `+400` typed over 960 | 1360 | 1360 | pass |
| `-400` typed over 960 | -400 | -400 | pass |
| `960-400` typed over 960 | 560 | 560 | pass |
| `*2` typed over 960 | 1920 | 1920 | pass |
| `/2` typed over 960 | 480 | 480 | pass |
| `+ 400` typed over 960 | 1360 | 1360 | pass |
| `2*(3+4)` typed over 0 | 14 | 14 | pass |
| `1+2*3` typed over 0 | 7 | 7 | pass |
| `-(5-8)` typed over 0 | 3 | 3 | pass |
| `+10` typed over -5 | 5 | 5 | pass |
| `*-1` typed over 30 | -30 | -30 | pass |
| `12.5` typed over 3 | 12.5 | 12.5 | pass |
| `.5` typed over 3 | 0.5 | 0.5 | pass |
| `1e3` typed over 0 | 1000 | 1000 | pass |
| `/0` typed over 5 | not taken | not taken | pass |
| `abc` typed over 5 | not taken | not taken | pass |
| `2+` typed over 5 | not taken | not taken | pass |
| `(2` typed over 5 | not taken | not taken | pass |
| `alert(1)` typed over 5 | not taken | not taken | pass |
| `` typed over 5 | not taken | not taken | pass |
| the one number field every panel uses reads what is typed through typedSum | true | true | pass |
