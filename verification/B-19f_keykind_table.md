# B-19f: separate X and Y, and auto, continuous and roving keys, in the core (D-69)

Written by `cargo test --test b19f_keykind`. Every expected number is from
`Fixtures/keykind/expected_keykind.json`, which `tools/keykind_reference.py` wrote before
this code existed. Tolerance 1e-9.

**36 of 37 checks match.** The one that does not is FX-KIND-005 as the fixture file writes it, and it is the fixture that is in question, not the build: see the row under it. It awaits the owner's decision on the fixture.

## A separated position, frame by frame (FX-SEP-001, 002)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SEP-001: X goes 0 to 8 at a steady speed while Y goes 0 to 4 on the ease-in-out curve: each has its own ease. | largest difference 1.4e-15 | yes |
| FX-SEP-001: saved again, the position is written as it was read | the same x and y | yes |
| FX-SEP-002: X and Y keyed on different frames, Y with a hold: neither needs a key where the other has one. | largest difference 0.0e0 | yes |
| FX-SEP-002: saved again, the position is written as it was read | the same x and y | yes |

## Edits, against the reference's keys (FX-KIND, FX-ROVE, FX-SEP-003, 004)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KIND-001: A key made auto: it passes through at the slope between its neighbours, 4 over 12 frames, with handles a third long.. The edit: make the key at frame 4 auto | the reference's keys, within 1e-9 | yes |
| FX-KIND-002: Auto on the first and last keys: a straight line out and in.. The edit: make both keys auto | the reference's keys, within 1e-9 | yes |
| FX-KIND-003: An auto key on a position: 5 pixels in and 12 out over 8 frames, so 17/8 of a pixel a frame on both sides.. The edit: make the key at frame 4 auto | the reference's keys, within 1e-9 | yes |
| FX-KIND-004: An auto key follows its neighbours: the next key's value goes from 4 to 12 and the slope through the auto key becomes 1.. The edit: set the value of the key at frame 12 to 12 | the reference's keys, within 1e-9 | yes |
| FX-KIND-005: A corner made continuous: 2 a frame in and 1 a frame out become 1.5 on both sides, the handles keeping their lengths.. The edit: make the key at frame 4 continuous | `[{"frame":0,"interp":[0.3333333333333333,0.3333333333333333,0.6666666666666666,0.7916666666666666],"value":0},{"frame":4,"interp":[0.3333333333333333,0.41666666666666663,0.6666666666666666,0.6666666666666666],"kind":"continuous","value":8},{"frame":8,"interp":"linear","value":12}]` | **NO** |
| FX-KIND-005 with the first key linear, which is what its sentence says: 2 a frame in and 1 out | the reference's keys, within 1e-9 | yes |
| FX-KIND-006: One handle of a continuous key pulled by hand: the other side follows it to the same speed, 2 a frame.. The edit: set the ease of the key at frame 4 to [0.25, 0.5, 0.75, 1] | the reference's keys, within 1e-9 | yes |
| FX-KIND-007: An auto key beside a segment that goes nowhere: that side has no speed to set and is left as it was.. The edit: make the key at frame 4 auto | the reference's keys, within 1e-9 | yes |
| FX-ROVE-001: A roving key: 50 pixels then 100, so it sits a third of the way through the ten frames, on frame 3.. The edit: make the key at frame 2 roving | the reference's keys, within 1e-9 | yes |
| FX-ROVE-002: Two roving keys crowded at the start of a short run: each still gets a frame of its own.. The edit: make the keys at frames 1 and 2 roving | the reference's keys, within 1e-9 | yes |
| FX-ROVE-003: A run with fewer frames than roving keys is refused with COMMAND_INVALID_VALUE and nothing changes.. The edit: move the key at frame 4 to frame 2 | COMMAND_INVALID_VALUE, keys unchanged | yes |
| FX-ROVE-004: A curved path counts at its own length, measured along 64 straight pieces: the arch is longer than the straight run after it.. The edit: make the key at frame 5 roving | the reference's keys, within 1e-9 | yes |
| FX-SEP-003: Separating a position: each key becomes a key of X and a key of Y with the same ease and kind; the path handles are dropped.. The edit: separate the position | the reference's keys, within 1e-9 | yes |
| FX-SEP-004: Joining FX-SEP-002 again: a key wherever either had one, holding the position at that frame, with X's ease where X had a key and else Y's.. The edit: join the position | the reference's keys, within 1e-9 | yes |

## What a file may not say

| Check | The build's answer | Matches |
| --- | --- | --- |
| A file with roving on a first key is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with roving on a last key is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with roving on a rotation key is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with a kind that is not one of the three is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with path handles inside x is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with a pair inside x is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with roving inside x is refused | PROJECT_SCHEMA_INVALID | yes |
| A file with x and y on an anchor is refused | PROJECT_SCHEMA_INVALID | yes |

## Kinds and roving through a save, and undo

| Check | The build's answer | Matches |
| --- | --- | --- |
| Roving keys and a continuous key are written, read, and written the same again | the same keys | yes |
| A roving key moved in time by hand stops roving | moved from frame 1 to 4, roving gone | yes |
| Pulling an auto key's handle makes it continuous | kind "continuous" | yes |
| Separate is one entry to undo, and undo gives back the path handles | "Separate position into X and Y", then the keys as before | yes |
| Joining a position that is not separated is refused | Err("COMMAND_INVALID_VALUE") | yes |

## A separated position under commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Separate is refused while the position has an expression | Err("COMMAND_INVALID_VALUE") | yes |
| A key on `position` is refused while it is separated | Err("COMMAND_INVALID_VALUE") | yes |
| A key on `position_x` at frame 2 moves X there and leaves Y alone | Some((100.0, 2.0)) | yes |
| An expression on `position_y`, value + 10, is in the position | Some((100.0, 12.0)) | yes |
| Joining is refused while X or Y has an expression | Err("COMMAND_INVALID_VALUE") | yes |
| Roving is refused on a separated position | Err("COMMAND_INVALID_VALUE") | yes |
