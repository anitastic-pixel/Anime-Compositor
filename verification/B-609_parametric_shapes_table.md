# B-609: parametric shapes

Written by `tests/b609_parametric_shapes.rs` from `Fixtures/parametric_shapes/`, against D-738 (a). Each line is one check: what `tools/parametric_shape_reference.py` says, and what the build gives.

| Case | Check | Should | The build | Matches |
| --- | --- | --- | --- | --- |
| FX-SHPP-001 | "Box" at frame 0: the outline | 4 points, first [160.0,40.0] | 4 points, first [160.0,40.0] | yes |
| FX-SHPP-001 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-002 | "Rounded" at frame 0: the outline | 8 points, first [350.0,70.0] | 8 points, first [350.0,70.0] | yes |
| FX-SHPP-002 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-003 | "Rounded" at frame 12: the outline | 8 points, first [365.0,60.0] | 8 points, first [365.0,60.0] | yes |
| FX-SHPP-003 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-004 | "Rounded" at frame 24: the outline | 8 points, first [380.0,50.0] | 8 points, first [380.0,50.0] | yes |
| FX-SHPP-004 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-005 | "Oval" at frame 0: the outline | 4 points, first [100,165.0] | 4 points, first [100.0,165.0] | yes |
| FX-SHPP-005 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-006 | "Star" at frame 0: the outline | 10 points, first [300.0,150.0] | 10 points, first [300.0,150.0] | yes |
| FX-SHPP-006 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-007 | "Hex" at frame 0: the outline | 6 points, first [220.0,115.35898384862244] | 6 points, first [220.0,115.35898384862244] | yes |
| FX-SHPP-007 | drawn as a plain shape of that outline | the same pixels | the same pixels | yes |
| FX-SHPP-008 | shape 0: type | "rectangle" | "rectangle" | yes |
| FX-SHPP-008 | shape 0: path, the base outline, no keys | as the reference | as the reference | yes |
| FX-SHPP-008 | shape 1: type | "rectangle" | "rectangle" | yes |
| FX-SHPP-008 | shape 1: path, the base outline, no keys | as the reference | as the reference | yes |
| FX-SHPP-008 | shape 2: type | "ellipse" | "ellipse" | yes |
| FX-SHPP-008 | shape 2: path, the base outline, no keys | as the reference | as the reference | yes |
| FX-SHPP-008 | shape 3: type | "star" | "star" | yes |
| FX-SHPP-008 | shape 3: path, the base outline, no keys | as the reference | as the reference | yes |
| FX-SHPP-008 | shape 4: type | "polygon" | "polygon" | yes |
| FX-SHPP-008 | shape 4: path, the base outline, no keys | as the reference | as the reference | yes |
| FX-SHPP-008 | the oval keeps roundness | {"base":12,"keyframes":[]} | {"base":12,"keyframes":[]} | yes |
| FX-SHPP-008 | the oval keeps bulge | 7 | 7 | yes |
| FX-SHPP-008 | saved, then read again | the same settings | the same settings | yes |
| FX-SHPP-020 | refused_open.json | refused, PROJECT_SCHEMA_INVALID, naming closed | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0: expected a rectangle to be closed, not open. | yes |
| FX-SHPP-021 | refused_type.json | refused, PROJECT_SCHEMA_INVALID, naming heart | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0/parametric/type: expected one of rectangle, ellipse, polygon, star, not "heart". | yes |
| FX-SHPP-022 | refused_no_size.json | refused, PROJECT_SCHEMA_INVALID, naming size | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0/parametric/size: expected this field to be present. | yes |
| FX-SHPP-023 | refused_negative_size.json | refused, PROJECT_SCHEMA_INVALID, naming size | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0: expected a rectangle's size from 0 to 100000, not -10. | yes |
| FX-SHPP-024 | refused_two_points.json | refused, PROJECT_SCHEMA_INVALID, naming points | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0: expected a polygon's points from 3 to 100, not 2. | yes |
| FX-SHPP-025 | refused_expression.json | refused, PROJECT_SCHEMA_INVALID, naming expression | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0: expected no expression on a rectangle's position: it takes keys only. | yes |
| FX-SHPP-026 | refused_keyed_radius.json | refused, PROJECT_SCHEMA_INVALID, naming radius | refused, PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/shapes/0: expected a star's outer radius from 0 to 100000, not -5. | yes |

B-609: 34 of 34 checks pass.
