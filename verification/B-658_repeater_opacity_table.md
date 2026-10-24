# B-658: Shape Repeater opacity steps by i / copies (D-778)

## The fixtures

`tests/b612_shape_repeaters.rs` against `Fixtures/shape_repeaters/` as D-778 rewrote it: 87 of 87
checks match, 0 differ. The full table is `verification/B-612_shape_repeaters_table.md`. Its
opacity rows are the ones D-778 changed, for example FX-SHPR-001 frame 0, Row's four squares:

| Copy | Should (D-778) | The build | Was (D-738 (c)) |
| --- | --- | --- | --- |
| 0 | 1 | 1 | 1 |
| 1 | 0.8125 | 0.8125 | 0.75 |
| 2 | 0.625 | 0.625 | 0.5 |
| 3 | 0.4375 | 0.4375 | 0.25 |

`cargo test --release --test b612_shape_repeaters`:
`test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s`

`host.json` is a project written before D-778 and is unchanged; it opens and saves back as it did
(FX-SHPR-005).

## Against After Effects

Each tutorial's repeater set up as read on its panel, rendered by the real program at frame 0
(`tools/app_snapshot.py --on cpu --full`) with the build before (440cecf0) and after, each copy's
opacity read at its centre. The After Effects numbers were read from the tutorial's pixels the same
way; that reading runs about 0.01 to 0.05 high on small dots. "Worst gap" is the largest
difference over the copies After Effects shows clearly.

| Tutorial | Settings | After Effects, last copy read | Before | After | Worst gap before | Worst gap after | Verdict |
| --- | --- | --- | --- | --- | --- | --- | --- |
| SSB Otaru, ebvf79jTQWA, about 0:31 | 6 copies, Start 100, End 0 | 0.21 | 0.000 | 0.168 | 0.210 | 0.047 | close (the reading's own bias) |
| Shake my art, DtHNjDQHi2g, about 1:18 | 7 copies, Start 7, End 100 | 0.854 | 1.000 | 0.869 | 0.146 | 0.020 | matches |
| Visual Motions, dJAUUqDuCUM, about 5:43 | 15 copies, Start 0, End 89 | copy 11: 0.663 | 0.701 | 0.651 | 0.038 | 0.020 | matches |
| AITEE Media, 9m2bQ3OfWG8, about 24:12 | 9 copies, Start 100, End 0 | 0.109 | 0.000 | 0.112 | 0.109 | 0.058 (copy 1; the last 0.003) | close |
| Luisa Winters, sjayul5h-So, about 40:33 | 7 copies, Start 100, End 11 (relative to copy 1) | 0.274 | 0.128 | 0.274 | 0.146 | 0.016 | matches |

Every case's after picture equals the D-778 rule within 0.004 (the 8-bit picture's rounding).
Visual Motions' copies turn 24 degrees here, not 30, so no two land on one place; the opacities
do not depend on it.
