# B-614: Merge Paths

Written by `tests/b614_shape_merge.rs` from `Fixtures/shape_merge/`, against D-738 (d). Each line is one check: what `tools/shape_merge_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-SHPM-001 | frame 0: how many shapes are drawn | 10 | 10 | yes |
| FX-SHPM-001 | drawn 1 | "Sun" {}: 1 outlines, first point [90.0,50.0], fill opacity 1 | "Sun" {}: 1 outlines, first point [90.0,50.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 2 | "Slab" {}: 2 outlines, first point [300.0,40.0], fill opacity 1 | "Slab" {}: 2 outlines, first point [300.0,40.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 3 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 4 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 5 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 6 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 7 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 8 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 9 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn 10 | "Plain" {}: 1 outlines, first point [20,390], fill opacity 1 | "Plain" {}: 1 outlines, first point [20.0,390.0], fill opacity 1 | yes |
| FX-SHPM-001 | drawn as shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPM-002 | frame 12: how many shapes are drawn | 10 | 10 | yes |
| FX-SHPM-002 | drawn 1 | "Sun" {}: 1 outlines, first point [90.0,50.0], fill opacity 1 | "Sun" {}: 1 outlines, first point [90.0,50.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 2 | "Slab" {}: 1 outlines, first point [300.0,40.0], fill opacity 1 | "Slab" {}: 1 outlines, first point [300.0,40.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 3 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 4 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 5 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 6 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 7 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 8 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 9 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn 10 | "Plain" {}: 1 outlines, first point [20,390], fill opacity 1 | "Plain" {}: 1 outlines, first point [20.0,390.0], fill opacity 1 | yes |
| FX-SHPM-002 | drawn as shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPM-003 | frame 24: how many shapes are drawn | 10 | 10 | yes |
| FX-SHPM-003 | drawn 1 | "Sun" {}: 2 outlines, first point [90.0,50.0], fill opacity 1 | "Sun" {}: 2 outlines, first point [90.0,50.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 2 | "Slab" {}: 1 outlines, first point [300.0,40.0], fill opacity 1 | "Slab" {}: 1 outlines, first point [300.0,40.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 3 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | "Wide" {}: 1 outlines, first point [425.5263157894737,110.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 4 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | "Left" {"Xor":1}: 1 outlines, first point [535.0,180.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 5 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | "Left" {"Xor":0}: 1 outlines, first point [535.0,60.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 6 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | "A" {}: 2 outlines, first point [102.0,220.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 7 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | "Spoke" {"Spokes":5}: 1 outlines, first point [212.2968060221314,288.69615242270663], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 8 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | "P1" {}: 1 outlines, first point [402.5,242.5], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 9 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | "J1" {}: 1 outlines, first point [520.0,360.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn 10 | "Plain" {}: 1 outlines, first point [20,390], fill opacity 1 | "Plain" {}: 1 outlines, first point [20.0,390.0], fill opacity 1 | yes |
| FX-SHPM-003 | drawn as shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPM-004 | the groups and their merges, note and blend included | as the reference | as the reference | yes |
| FX-SHPM-004 | saved, then read again | the same operations | the same operations | yes |
| FX-SHPM-020 | refused_mode.json | refused, PROJECT_SCHEMA_INVALID, naming mode | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Union"'s operation 1 (merge) mode to be one of add, merge, subtract, intersect, exclude, not "union". | yes |
| FX-SHPM-021 | refused_no_mode.json | refused, PROJECT_SCHEMA_INVALID, naming mode | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Bite"'s operation 1 (merge) mode to be one of add, merge, subtract, intersect, exclude, not missing. | yes |

B-614: 40 of 40 checks pass.
