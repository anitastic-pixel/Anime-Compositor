# B-613: path operations

Written by `tests/b613_shape_operations.rs` from `Fixtures/shape_operations/`, against D-738 (d). Each line is one check: what `tools/shape_operation_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-SHPO-001 | frame 0: how many shapes are drawn | 13 | 13 | yes |
| FX-SHPO-001 | drawn 1 | "Box" {}: first point [100.0,30.0], fill opacity 1 | "Box" {}: first point [100.0,30.0], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 2 | "Penta" {"Puff":1}: first point [260.0,6.0], fill opacity 1 | "Penta" {"Puff":1}: first point [260.0,6.0], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 3 | "Penta" {"Puff":0}: first point [170.0,6.0], fill opacity 1 | "Penta" {"Puff":0}: first point [170.0,6.0], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 4 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 5 | "Line" {}: first point [470.0,76.0] | "Line" {}: first point [470.0,76.0] | yes |
| FX-SHPO-001 | drawn 6 | "Card" {}: first point [102.97185431217092,173.08164935359653], fill opacity 1 | "Card" {}: first point [102.97185431217093,173.08164935359653], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 7 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 8 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 9 | "Hook" {}: first point [278.10263340389895,245.6920997883031] | "Hook" {}: first point [278.10263340389895,245.6920997883031] | yes |
| FX-SHPO-001 | drawn 10 | "Oval" {}: first point [440.0,185.0022893781614], fill opacity 1 | "Oval" {}: first point [440.0,185.0022893781614], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 11 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 12 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | yes |
| FX-SHPO-001 | drawn 13 | "Plain" {}: first point [50,350], fill opacity 1 | "Plain" {}: first point [50.0,350.0], fill opacity 1 | yes |
| FX-SHPO-001 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPO-002 | frame 6: how many shapes are drawn | 13 | 13 | yes |
| FX-SHPO-002 | drawn 1 | "Box" {}: first point [92.5,30.0], fill opacity 1 | "Box" {}: first point [92.5,30.0], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 2 | "Penta" {"Puff":1}: first point [260.0,18.0], fill opacity 1 | "Penta" {"Puff":1}: first point [260.0,18.0], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 3 | "Penta" {"Puff":0}: first point [170.0,18.0], fill opacity 1 | "Penta" {"Puff":0}: first point [170.0,18.0], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 4 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 5 | "Line" {}: first point [470.0,76.0] | "Line" {}: first point [470.0,76.0] | yes |
| FX-SHPO-002 | drawn 6 | "Card" {}: first point [103.19290692221144,170.4150259461039], fill opacity 1 | "Card" {}: first point [103.19290692221144,170.4150259461039], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 7 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 8 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 9 | "Hook" {}: first point [278.10263340389895,245.6920997883031] | "Hook" {}: first point [278.10263340389895,245.6920997883031] | yes |
| FX-SHPO-002 | drawn 10 | "Oval" {}: first point [440.0,180.00114468908072], fill opacity 1 | "Oval" {}: first point [440.0,180.00114468908072], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 11 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 12 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | yes |
| FX-SHPO-002 | drawn 13 | "Plain" {}: first point [50,350], fill opacity 1 | "Plain" {}: first point [50.0,350.0], fill opacity 1 | yes |
| FX-SHPO-002 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPO-003 | frame 12: how many shapes are drawn | 13 | 13 | yes |
| FX-SHPO-003 | drawn 1 | "Box" {}: first point [85.0,30.0], fill opacity 1 | "Box" {}: first point [85.0,30.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 2 | "Penta" {"Puff":1}: first point [260.0,30.0], fill opacity 1 | "Penta" {"Puff":1}: first point [260.0,30.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 3 | "Penta" {"Puff":0}: first point [170.0,30.0], fill opacity 1 | "Penta" {"Puff":0}: first point [170.0,30.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 4 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 5 | "Line" {}: first point [470.0,76.0] | "Line" {}: first point [470.0,76.0] | yes |
| FX-SHPO-003 | drawn 6 | "Card" {}: first point [103.41395953225194,167.7484025386113], fill opacity 1 | "Card" {}: first point [103.41395953225194,167.7484025386113], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 7 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 8 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 9 | "Hook" {}: first point [278.10263340389895,245.6920997883031] | "Hook" {}: first point [278.10263340389895,245.6920997883031] | yes |
| FX-SHPO-003 | drawn 10 | "Oval" {}: first point [440.0,175.0], fill opacity 1 | "Oval" {}: first point [440.0,175.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 11 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 12 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | yes |
| FX-SHPO-003 | drawn 13 | "Plain" {}: first point [50,350], fill opacity 1 | "Plain" {}: first point [50.0,350.0], fill opacity 1 | yes |
| FX-SHPO-003 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPO-004 | frame 24: how many shapes are drawn | 13 | 13 | yes |
| FX-SHPO-004 | drawn 1 | "Box" {}: first point [70.0,30.0], fill opacity 1 | "Box" {}: first point [70.0,30.0], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 2 | "Penta" {"Puff":1}: first point [260.0,54.0], fill opacity 1 | "Penta" {"Puff":1}: first point [260.0,54.0], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 3 | "Penta" {"Puff":0}: first point [170.0,54.0], fill opacity 1 | "Penta" {"Puff":0}: first point [170.0,54.0], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 4 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | "Bar" {}: first point [425.4996489784029,66.87239023500057], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 5 | "Line" {}: first point [470.0,76.0] | "Line" {}: first point [470.0,76.0] | yes |
| FX-SHPO-004 | drawn 6 | "Card" {}: first point [100.66729225159152,170.77351244959618], fill opacity 1 | "Card" {}: first point [100.66729225159152,170.77351244959618], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 7 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | "Tri" {}: first point [138.08976879281255,220.96903179580733], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 8 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | "Star" {}: first point [304.33262130060064,158.03002064031264], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 9 | "Hook" {}: first point [278.10263340389895,245.6920997883031] | "Hook" {}: first point [278.10263340389895,245.6920997883031] | yes |
| FX-SHPO-004 | drawn 10 | "Oval" {}: first point [439.786056446067,165.00228885415615], fill opacity 1 | "Oval" {}: first point [439.786056446067,165.00228885415615], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 11 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | "Disc" {}: first point [540.0,178.0], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 12 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | "Tile" {}: first point [333.3649866881296,309.4749031690471], fill opacity 1 | yes |
| FX-SHPO-004 | drawn 13 | "Plain" {}: first point [50,350], fill opacity 1 | "Plain" {}: first point [50.0,350.0], fill opacity 1 | yes |
| FX-SHPO-004 | drawn as plain shapes of those outlines and paint | the same pixels | the same pixels | yes |
| FX-SHPO-005 | the groups and their operations, note and blend included | as the reference | as the reference | yes |
| FX-SHPO-005 | saved, then read again | the same operations | the same operations | yes |
| FX-SHPO-020 | refused_type.json | refused, PROJECT_SCHEMA_INVALID, naming spin | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Corners"'s operation 2 to be one of round_corners, pucker_bloat, twist, zig_zag, wiggle, offset, merge, not "spin". | yes |
| FX-SHPO-021 | refused_radius.json | refused, PROJECT_SCHEMA_INVALID, naming radius | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Swirl"'s operation 1 (round_corners) radius from 0 to 100000, not -1. | yes |
| FX-SHPO-022 | refused_amount_key.json | refused, PROJECT_SCHEMA_INVALID, naming amount | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Puff"'s operation 1 (pucker_bloat) amount from -100 to 100, not 150. | yes |
| FX-SHPO-023 | refused_ridges.json | refused, PROJECT_SCHEMA_INVALID, naming ridges | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Zig"'s operation 1 (zig_zag) ridges from 0 to 100, not 500. | yes |
| FX-SHPO-024 | refused_points.json | refused, PROJECT_SCHEMA_INVALID, naming points | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Zig"'s operation 1 (zig_zag) points to be one of corner, smooth, not "wavy". | yes |
| FX-SHPO-025 | refused_join.json | refused, PROJECT_SCHEMA_INVALID, naming join | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Grow"'s operation 1 (offset) join to be one of miter, round, bevel, not "square". | yes |
| FX-SHPO-026 | refused_miter_limit.json | refused, PROJECT_SCHEMA_INVALID, naming miter_limit | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Spike"'s operation 1 (offset) miter_limit from 1 to 100, not 0.5. | yes |
| FX-SHPO-027 | refused_no_seed.json | refused, PROJECT_SCHEMA_INVALID, naming seed | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Wobble"'s operation 1 (wiggle) seed, which is missing. | yes |
| FX-SHPO-028 | refused_correlation.json | refused, PROJECT_SCHEMA_INVALID, naming correlation | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Wobble"'s operation 1 (wiggle) correlation from 0 to 100, not 120. | yes |
| FX-SHPO-029 | refused_expression.json | refused, PROJECT_SCHEMA_INVALID, naming expression | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected no expression on group "Outer"'s operation 1 (twist) angle: it takes keys only. | yes |
| FX-SHPO-030 | refused_nine.json | refused, PROJECT_SCHEMA_INVALID, naming operations | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Swirl"'s operations to be at most 8, not 9. | yes |

B-613: 73 of 73 checks pass.
