# B-887: Color Grade's key tools in the page

D-1007. Every case of `Fixtures/color_grade_5/expected_key_tools.json` (written by `tools/color_grade_5_reference.py` before the page code) run through the page's own `keyTool`, then the strip used in a private, hidden copy of the app.

| Check | Got | Passes |
|---|---|---|
| KEYTOOL-001: Set on an orange: hue 25, range 15, softness 15, its saturation and lightness 20 either side. | hue 25 hue_range 15 hue_softness 15 saturation_low 56 saturation_high 96 lightness_low 30 lightness_high 70 softness 0 | yes |
| KEYTOOL-002: Set on a sky blue. | hue 213 hue_range 15 hue_softness 15 saturation_low 41 saturation_high 81 lightness_low 33 lightness_high 73 softness 0 | yes |
| KEYTOOL-003: Set on a grey: every hue (range 180), its lightness 20 either side, saturation 0 to 20. | hue 0 hue_range 180 hue_softness 0 saturation_low 0 saturation_high 20 lightness_low 30 lightness_high 70 softness 0 | yes |
| KEYTOOL-004: Set on a skin tone. | hue 22 hue_range 15 hue_softness 15 saturation_low 33 saturation_high 73 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-005: Add an orange to the skin key: the hue range grows towards it, the saturation's high end up to take it. | hue 22 hue_range 15 hue_softness 15 saturation_low 33 saturation_high 76 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-006: Add a colour already inside the skin key: nothing changes. | hue 22 hue_range 15 hue_softness 15 saturation_low 33 saturation_high 73 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-007: Add a grey to the skin key: every hue (range 180), the saturation's low end down to 0. | hue 22 hue_range 180 hue_softness 15 saturation_low 0 saturation_high 73 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-008: Add a sky blue to an orange key: past half a turn the other way, so the range grows the short way round. | hue 307 hue_range 94 hue_softness 15 saturation_low 56 saturation_high 96 lightness_low 30 lightness_high 70 softness 0 | yes |
| KEYTOOL-009: Remove a paler skin tone from the skin key: the saturation's high end comes down past it, the cheapest way out (a smaller share than the lightness's). | hue 22 hue_range 15 hue_softness 15 saturation_low 33 saturation_high 67 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-010: Remove an orange from a wide warm key (hue 20, range 40, softness 10) whose saturation (0 to 100) is the cheapest band to trim: its high end comes down below the orange's. | hue 20 hue_range 40 hue_softness 10 saturation_low 0 saturation_high 74 lightness_low 0 lightness_high 100 softness 0 | yes |
| KEYTOOL-011: Remove an orange from the same warm key with narrow saturation (72 to 78) and lightness (45 to 55): now the hue is the cheapest, its side towards the orange shrinks. | hue 358 hue_range 17 hue_softness 10 saturation_low 72 saturation_high 78 lightness_low 45 lightness_high 55 softness 0 | yes |
| KEYTOOL-012: Remove a leaf green from the skin key: the key leaves it out already, so nothing changes. | hue 22 hue_range 15 hue_softness 15 saturation_low 33 saturation_high 73 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-013: Remove a duller skin tone from the skin key: the saturation's low end comes up past it, the cheapest way out. | hue 22 hue_range 15 hue_softness 15 saturation_low 42 saturation_high 73 lightness_low 47 lightness_high 87 softness 0 | yes |
| KEYTOOL-014: The red dot: hue 0, range 30, softness 15, saturation and lightness 0 to 100. | hue 0 hue_range 30 hue_softness 15 saturation_low 0 saturation_high 100 lightness_low 0 lightness_high 100 softness 0 | yes |
| KEYTOOL-015: The cyan dot after the skin key: the key softness kept. | hue 180 hue_range 30 hue_softness 15 saturation_low 0 saturation_high 100 lightness_low 0 lightness_high 100 softness 10 | yes |
| clicking the red dot writes the red key into the project, the key softness kept | {"key_hue": 0, "key_hue_range": 30, "key_hue_softness": 15, "key_saturation_low": 0, "key_saturation_high": 100, "key_lightness_low": 0, "key_lightness_high": 100, "key_softness": 0} | yes |
| Set, then a click on the picture: the key becomes Set's from the picked pixel | picked #adadad, {"key_hue": 0, "key_hue_range": 180, "key_hue_softness": 15, "key_saturation_low": 0, "key_saturation_high": 20, "key_lightness_low": 48, "key_lightness_high": 88, "key_softness": 0} | yes |
| Show Key offers Off, White/Black, Color/Gray and Color/Black | ["Off", "White/Black", "Color/Gray", "Color/Black"] | yes |

## Result

18 of 18 checks pass.
