# D-332: keys on every keyable setting survive a save

Written by `tests/b213_keyed_settings.rs` from `Fixtures/projects/keyed_settings_project.json`, tutorial 1's Wave layer as the app saved it, with a Bulge added. Before D-332 this file would not open: *At /compositions/0/layers/0/effects/0/parameters/offset: expected an array.*

## FX-KEYALL-001 to 004

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KEYALL-001: the file opens with no error | it opens | yes |
| FX-KEYALL-002: Offset Turbulence keys are 0: 0 eased; 34: 0 eased / 0: 0 eased; 34: 410 eased | 0: 0 eased; 34: 0 eased / 0: 0 eased; 34: 410 eased | yes |
| FX-KEYALL-003: Vertical Radius and Taper Radius keys are 0: 190 linear; 12: 120 linear, and 0: 0 linear; 12: 240 linear | 0: 190 linear; 12: 120 linear, and 0: 0 linear; 12: 240 linear | yes |
| FX-KEYALL-004: saved and opened again, the same keys, and saving that gives the same text | same keys true, same text true | yes |

## Result

4 of 4 checks pass.
