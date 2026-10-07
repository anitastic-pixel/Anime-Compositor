# B-34: limits for the blur and exposure

D-90, accepted by the owner on 2026-09-25 ("works; proceed with limits"): a Gaussian blur's sigma runs from 0 to 500 and exposure from -20 to 20 stops; D-318 (2026-10-04) widens exposure to -40 to 40. Every expected pixel is `Fixtures/limits/expected_limits.json`, written by `tools/limits_reference.py` before this code existed and printed in document 25 as FX-LIMIT-001 to 010. The build's frame is compared sample by sample; the answer is the largest difference as a share of the expected value, against the catalogue's 1e-4, and an expected 0 must be exactly 0.

## FX-LIMIT-001 to 016 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LIMIT-001 frame 0: Exposure 20 stops, the top of its range: every colour 2^20 times as much, the coverage unchanged. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-001: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-002 frame 0: Exposure -20 stops, the bottom of its range: every colour 2^20 times less. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-002: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-003 frame 0: A Gaussian blur of sigma 500, the top of its range: one pixel spread almost evenly over the frame and far past it. | largest difference 8.7e-8 of the value | yes |
| FX-LIMIT-003: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-004, superseded by D-318: the ease past 20 is no longer held at 20: frame 2 is brighter than 2^20 | red 9.4906264e7, warned false | yes |
| FX-LIMIT-005 frame 2: A blur eased from 0 to 500 on the overshooting curve: held at 500. | largest difference 8.7e-8 of the value | yes |
| FX-LIMIT-005: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-006, superseded by D-318: its value is in range now: it opens with no warning and is drawn | red 2.097152e6, warned false | yes |
| FX-LIMIT-007, superseded by D-318: its value is in range now: it opens with no warning and is drawn | red 4.7683716e-7, warned false | yes |
| FX-LIMIT-008 frame 0: Exposure 128 stops over a drawing that is mostly transparent: past the limit, so left out, where before it made every transparent pixel not-a-number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-008 frame 4: Exposure 128 stops over a drawing that is mostly transparent: past the limit, so left out, where before it made every transparent pixel not-a-number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-008: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIMIT-008 saved again keeps the setting as written | {"stops":128} | yes |
| FX-LIMIT-009 frame 0: A Gaussian blur of sigma 501, one past the top. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-009 frame 4: A Gaussian blur of sigma 501, one past the top. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-009: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIMIT-009 saved again keeps the setting as written | {"sigma_px":501} | yes |
| FX-LIMIT-010 frame 0: A blur keyed from 0 at frame 0 to 600 at frame 4: one key past the top, so left out of every frame, frame 0 as well. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-010 frame 4: A blur keyed from 0 at frame 0 to 600 at frame 4: one key past the top, so left out of every frame, frame 0 as well. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 0.0e0 of the value | yes |
| FX-LIMIT-010: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIMIT-010 saved again keeps the setting as written | {"sigma_px":{"base":0,"keyframes":[{"frame":0,"interp":"linear","value":0},{"frame":4,"interp":"linear","value":600}]}} | yes |
| FX-LIMIT-011 frame 0: Exposure 40 stops, the top of D-318's range: every colour 2^40 times as much, the coverage unchanged. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-011: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-012 frame 0: Exposure -40 stops, the bottom of D-318's range: every colour 2^40 times less. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-012: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-013 frame 0: Exposure 21 stops, past D-90's old top: an ordinary value now, every colour 2^21 times as much. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-013: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-014 frame 1: Exposure eased from 0 to 40 stops on the overshooting curve: past the middle it would pass 40, and is held at 40. | largest difference 1.1e-7 of the value | yes |
| FX-LIMIT-014 frame 2: Exposure eased from 0 to 40 stops on the overshooting curve: past the middle it would pass 40, and is held at 40. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-014: what opening it warns of, and what frame 2 warns of | [] and [] | yes |
| FX-LIMIT-015 frame 0: Exposure 41 stops, one past D-318's top. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-015 frame 4: Exposure 41 stops, one past D-318's top. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-015: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIMIT-015 saved again keeps the setting as written | {"stops":41} | yes |
| FX-LIMIT-016 frame 0: Exposure -41 stops, one past D-318's bottom. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-016 frame 4: Exposure -41 stops, one past D-318's bottom. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.4e-7 of the value | yes |
| FX-LIMIT-016: what opening it warns of, and what frame 2 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-LIMIT-016 saved again keeps the setting as written | {"stops":-41} | yes |

## Commands (D-46)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_limit_003.json: "A Gaussian blur's sigma runs from 0 to 500, and this is 501." and nothing changes | A Gaussian blur's sigma runs from 0 to 500, and this is 501. | yes |
| fx_limit_003.json: "A Gaussian blur's sigma runs from 0 to 500, and this is -1." and nothing changes | A Gaussian blur's sigma runs from 0 to 500, and this is -1. | yes |
| fx_limit_001.json: "Exposure runs from -40 to 40 stops, and this is 41." and nothing changes | Exposure runs from -40 to 40 stops, and this is 41. | yes |
| fx_limit_001.json: "Exposure runs from -40 to 40 stops, and this is -41." and nothing changes | Exposure runs from -40 to 40 stops, and this is -41. | yes |
| fx_limit_001.json: "Exposure runs from -40 to 40 stops, and this is 128." and nothing changes | Exposure runs from -40 to 40 stops, and this is 128. | yes |
| fx_limit_003.json: GaussianBlur { sigma_px: 0.0, edges: "transparent", dimensions: "both", units: "sigma" }, on the limit, is taken | taken | yes |
| fx_limit_003.json: GaussianBlur { sigma_px: 500.0, edges: "transparent", dimensions: "both", units: "sigma" }, on the limit, is taken | taken | yes |
| fx_limit_001.json: Exposure { stops: -20.0, offset: 0.0, gamma: 1.0, bypass: "off" }, on the limit, is taken | taken | yes |
| fx_limit_001.json: Exposure { stops: 20.0, offset: 0.0, gamma: 1.0, bypass: "off" }, on the limit, is taken | taken | yes |
| fx_limit_001.json: Exposure { stops: 21.0, offset: 0.0, gamma: 1.0, bypass: "off" }, on the limit, is taken | taken | yes |
| fx_limit_001.json: Exposure { stops: -40.0, offset: 0.0, gamma: 1.0, bypass: "off" }, on the limit, is taken | taken | yes |
| fx_limit_001.json: Exposure { stops: 40.0, offset: 0.0, gamma: 1.0, bypass: "off" }, on the limit, is taken | taken | yes |

## Result

52 of 52 checks pass.
