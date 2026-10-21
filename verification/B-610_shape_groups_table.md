# B-610: shape groups

Written by `tests/b610_shape_groups.rs` from `Fixtures/shape_groups/`, against D-738 (b). Each line is one check: what `tools/shape_group_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-SHPG-001 | "Torso" at frame 0: placed | {"fill_end":[240.0,150.0],"fill_opacity":0.5,"fill_start":[160.0,150.0],"points":"4 points, first [240.0,110.0]","stroke_opacity":0.5,"stroke_width":4.0} | {"fill_end":[240.0,150.0],"fill_opacity":0.5,"fill_start":[160.0,150.0],"points":"4 points, first [240.0,110.0]","stroke_opacity":0.5,"stroke_width":4.0} | yes |
| FX-SHPG-001 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-002 | "Torso" at frame 12: placed | {"fill_end":[228.2842712474619,178.2842712474619],"fill_opacity":0.5,"fill_start":[171.7157287525381,121.7157287525381],"points":"4 points, first [256.5685424949238,150.0]","stroke_opacity":0.5,"stroke_width":4.0} | {"fill_end":[228.2842712474619,178.2842712474619],"fill_opacity":0.5,"fill_start":[171.7157287525381,121.7157287525381],"points":"4 points, first [256.5685424949238,150.0]","stroke_opacity":0.5,"stroke_width":4.0} | yes |
| FX-SHPG-002 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-003 | "Hand" at frame 0: placed | {"points":"4 points, first [248.7289443967175,142.847092948762]","stroke_opacity":0.5,"stroke_width":1.7320508075688776} | {"points":"4 points, first [248.72894439671754,142.847092948762]","stroke_opacity":0.5,"stroke_width":1.7320508075688776} | yes |
| FX-SHPG-003 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-004 | "Hand" at frame 24: placed | {"points":"4 points, first [207.152907051238,198.7289443967175]","stroke_opacity":0.5,"stroke_width":1.7320508075688776} | {"points":"4 points, first [207.152907051238,198.72894439671754]","stroke_opacity":0.5,"stroke_width":1.7320508075688776} | yes |
| FX-SHPG-004 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-005 | "Free" at frame 0: placed | {"fill_opacity":0.8,"points":"3 points, first [30,260]"} | {"fill_opacity":0.8,"points":"3 points, first [30.0,260.0]"} | yes |
| FX-SHPG-005 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-006 | "Badge" at frame 12: placed | {"fill_opacity":1,"points":"10 points, first [340.0,50.0]"} | {"fill_opacity":1.0,"points":"10 points, first [340.0,50.0]"} | yes |
| FX-SHPG-006 | drawn as a plain shape of that outline and paint | the same pixels | the same pixels | yes |
| FX-SHPG-007 | the groups, blend included | as the reference | as the reference | yes |
| FX-SHPG-007 | each shape's group | [0,1,null,2] | [0,1,null,2] | yes |
| FX-SHPG-007 | every path as read, not placed | as the reference | as the reference | yes |
| FX-SHPG-007 | saved, then read again | the same groups | the same groups | yes |
| FX-SHPG-020 | refused_shape_group.json | refused, PROJECT_SCHEMA_INVALID, naming group | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected shape "Torso"'s group to be one of the layer's 3 groups, not 3. | yes |
| FX-SHPG-021 | refused_parent.json | refused, PROJECT_SCHEMA_INVALID, naming parent | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Arm"'s parent to be another of the layer's 3 groups, not 7. | yes |
| FX-SHPG-022 | refused_loop.json | refused, PROJECT_SCHEMA_INVALID, naming parent | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Body" not inside itself, but its parent leads back to it. | yes |
| FX-SHPG-023 | refused_opacity.json | refused, PROJECT_SCHEMA_INVALID, naming opacity | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Body"'s opacity from 0 to 1, not 1.5. | yes |
| FX-SHPG-024 | refused_skew.json | refused, PROJECT_SCHEMA_INVALID, naming skew | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Arm"'s skew from -85 to 85, not 90. | yes |
| FX-SHPG-025 | refused_no_rotation.json | refused, PROJECT_SCHEMA_INVALID, naming rotation | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected group "Drift"'s rotation, which is missing. | yes |
| FX-SHPG-026 | refused_expression.json | refused, PROJECT_SCHEMA_INVALID, naming expression | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/groups: expected no expression on group "Body"'s position: it takes keys only. | yes |

B-610: 23 of 23 checks pass.
