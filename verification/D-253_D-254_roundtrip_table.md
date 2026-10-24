# D-253 and D-254: a cut status and labels, through save and open

Written by `tests/d253_d254_roundtrip.rs`. The fixture hash was taken on the build before either change (e395d8b).

| Check | Expected | Actual | Result |
|---|---|---|---|
| There is no label colour 9 | refused | refused | pass |
| A status set in the app reads back after save and open | retake | retake | pass |
| A composition's label reads back | 5 | 5 | pass |
| A footage item's label reads back | 2 | 2 | pass |
| A project without them reads as Not started, with no labels | "" 0 0 | "" 0 0 | pass |
| Fixture projects that open | 2297 | 2297 | pass |
| Their saved text, all of it, is byte for byte as before (SHA-256) | c66b9656cbfaf5eb03156712d66f3fca7f09c3cf7a860981ef0b7e73d1ec188d | c66b9656cbfaf5eb03156712d66f3fca7f09c3cf7a860981ef0b7e73d1ec188d | pass |
| A status written by a later build is kept through open and save | "retake" | "retake" | pass |
| A composition's label is kept through open and save | 5 | 5 | pass |
| A footage item's label is kept through open and save | 2 | 2 | pass |
