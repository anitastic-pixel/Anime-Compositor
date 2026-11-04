# B-889: Color Grade's section switches and WB Selector in the page

D-1009. Every case of `Fixtures/color_grade_6/expected_white_balance.json` (written by `tools/color_grade_6_reference.py` before the page code) run through the page's own `wbPick`, then the eyedropper and a switch used in a private, hidden copy of the app.

| Check | Got | Passes |
|---|---|---|
| WBPICK-001: A grey: nothing to correct. | #808080: temperature 0, tint 0 | yes |
| WBPICK-002: A warm white (a tungsten-lit wall): cooler, temperature down. | #ffe0c0: temperature -92, tint 8 | yes |
| WBPICK-003: A cold white (shade under a blue sky): warmer, temperature up. | #c8d8ff: temperature 79, tint -29 | yes |
| WBPICK-004: A greenish grey (fluorescent light): tint towards magenta, up. | #90a890: temperature 0, tint 98 | yes |
| WBPICK-005: A magenta grey: tint down. | #a890a8: temperature 0, tint -98 | yes |
| WBPICK-006: The warm white picked again after the first pick (temperature -20, tint -6 already set, the picture showing them): the change adds to them. | #f8f0e8: temperature -42, tint -6 | yes |
| WBPICK-007: A strong orange: held to -100. | #ff6010: temperature -100, tint 100 | yes |
| WBPICK-008: A colour with no blue: nothing changes. | #ff8000: temperature 10, tint 5 | yes |
| each of the six sections has a switch offering On and Off, On first | {"Basic Correction": ["On", "Off"], "Creative": ["On", "Off"], "Curves": ["On", "Off"], "Color Wheels": ["On", "Off"], "HSL Secondary": ["On", "Off"], "Vignette": ["On", "Off"]} | yes |
| WB Selector, then a click on the picture: temperature and tint become wbPick's from the picked pixel, added to what was set | picked #f5dec4; from temperature 20, tint -10 to -53, -2; status: Change Color Grade settings | yes |
| choosing Off for Vignette writes vignette_on "off" into the project | vignette_on off | yes |

## Result

11 of 11 checks pass.
