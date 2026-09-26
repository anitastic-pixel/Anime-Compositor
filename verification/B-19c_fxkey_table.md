# B-19c: keyframed effect settings

D-68, accepted by the owner on 2026-09-18. Every expected pixel is `Fixtures/fxkey/expected_fxkey.json`, written by `tools/fxkey_reference.py` before this code existed and printed in document 25 as FX-FXK-001 to 009. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6.

## FX-FXK-001 to 009 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FXK-001 frame 0: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between. | largest difference 3.1e-8 | yes |
| FX-FXK-001 frame 1: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between. | largest difference 3.1e-8 | yes |
| FX-FXK-001 frame 2: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between. | largest difference 6.1e-8 | yes |
| FX-FXK-001 frame 3: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between. | largest difference 1.2e-7 | yes |
| FX-FXK-001 frame 4: Exposure keyed from 0 stops at frame 1 to 2 stops at frame 3, linear: the first key's value before it, the last key's after it, one stop between. | largest difference 1.2e-7 | yes |
| FX-FXK-001: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-002 frame 1: A hold key: 0 stops until the next key, then 1 stop. | largest difference 3.1e-8 | yes |
| FX-FXK-002 frame 2: A hold key: 0 stops until the next key, then 1 stop. | largest difference 6.1e-8 | yes |
| FX-FXK-002: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-003 frame 1: An eased key, 0 to 2 stops over four frames on the ease-in-out curve: slow, then fast, then slow. | largest difference 4.5e-8 | yes |
| FX-FXK-003 frame 2: An eased key, 0 to 2 stops over four frames on the ease-in-out curve: slow, then fast, then slow. | largest difference 6.1e-8 | yes |
| FX-FXK-003 frame 3: An eased key, 0 to 2 stops over four frames on the ease-in-out curve: slow, then fast, then slow. | largest difference 1.1e-7 | yes |
| FX-FXK-003: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-004 frame 0: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.0e-8 | yes |
| FX-FXK-004 frame 1: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.7e-8 | yes |
| FX-FXK-004 frame 2: A tint's colour keyed from red to blue: each of its three numbers goes the same fraction of the way, in linear light. | largest difference 3.0e-8 | yes |
| FX-FXK-004: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-005 frame 1: A tint's amount eased from 0 to 1 on a curve that overshoots: the amount stops at 1 and goes no further. | largest difference 1.7e-8 | yes |
| FX-FXK-005 frame 2: A tint's amount eased from 0 to 1 on a curve that overshoots: the amount stops at 1 and goes no further. | largest difference 0.0e0 | yes |
| FX-FXK-005: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-006 frame 0: A blur on a drawing's own layer, keyed from no blur to 2 pixels. | largest difference 0.0e0 | yes |
| FX-FXK-006 frame 2: A blur on a drawing's own layer, keyed from no blur to 2 pixels. | largest difference 1.6e-8 | yes |
| FX-FXK-006: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-007 frame 2: A blur eased from 1 pixel to none on the overshooting curve: a blur of less than nothing is no blur. | largest difference 0.0e0 | yes |
| FX-FXK-007: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-008 frame 2: Two settings of one effect keyed at once, and a second effect left plain. | largest difference 3.1e-8 | yes |
| FX-FXK-008: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-FXK-009 frame 0: A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FXK-009 frame 4: A tint amount keyed to 1.5, outside 0 to 1: the file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 3.1e-8 | yes |
| FX-FXK-009: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fxk_003.json opened and saved holds what it held, keys and eases included | the same | yes |
| fx_fxk_008.json opened and saved holds what it held, keys and eases included | the same | yes |
| fx_fxk_009.json opened and saved holds what it held, keys and eases included | the same | yes |
| a project made before D-68 saves with every setting still a plain number | the same | yes |
| FX-FXK-001 with keys on a radius, which exposure does not have, saves them as written and opens again | the same, opens | yes |
| FX-FXK-001 with path handles on a setting's key is refused | Some("PROJECT_SCHEMA_INVALID") | yes |
| FX-FXK-001 with two keys on one frame is refused | Some("PROJECT_SCHEMA_INVALID") | yes |

## Commands and undo

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FXK-001 as opened: the keys of stops | 1:[0.0] 3:[2.0] | yes |
| the layer moved one frame later: its effect's keys moved with it | 2:[0.0] 4:[2.0] | yes |
| undo puts them back | 1:[0.0] 3:[2.0] | yes |
| the keys of stops replaced by three, as one entry to undo | 0:[0.0] 2:[1.0] 4:[3.0], undo depth 1 | yes |
| undo: frame 2 is the frame it was | byte-identical | yes |
| no keys: the setting is constant again and is written as a plain number | 0 | yes |
| a tint amount keyed to 1.5 is refused and nothing changes | Some("EFFECT_PARAMETER_INVALID") | yes |
| two keys on one frame is refused and nothing changes | Some("COMMAND_INVALID_VALUE") | yes |
| one number for a colour is refused and nothing changes | Some("COMMAND_INVALID_VALUE") | yes |
| a setting a tint does not have is refused and nothing changes | Some("COMMAND_INVALID_VALUE") | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FXK-006 frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## Result

48 of 48 checks pass.
