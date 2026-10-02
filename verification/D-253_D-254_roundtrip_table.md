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
| Their saved text, all of it, is byte for byte as before (SHA-256) | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | aeadb2e51777a56efb28e3113b652eaa65c03d10cf2ba66b8d4798f0c86dae2d | pass |
| A status written by a later build is kept through open and save | "retake" | "retake" | pass |
| A composition's label is kept through open and save | 5 | 5 | pass |
| A footage item's label is kept through open and save | 2 | 2 | pass |
