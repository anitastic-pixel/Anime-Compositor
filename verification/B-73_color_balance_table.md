# B-73: colour balance

D-130, accepted by the owner on 2026-09-26, the eighth of the second batch of ten. Every expected pixel is `Fixtures/color_balance/expected_color_balance.json`, written by `tools/color_balance_reference.py` before this code existed and printed in document 25 as FX-BALANCE-001 to 019. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-BALANCE-001 to 019 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-BALANCE-001 frame 0: All nine at 0, the settings as they start: the drawing, untouched. | largest difference 1.9e-7 | yes |
| FX-BALANCE-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-002 frame 0: Shadows 100, 0, 0: black turns a dark red, its red half way up the scale, #800000 but for rounding (127.5); the line and the dark grey redden too, by less, as they are less deep in the shadows, though the dark grey's red lands on half the scale too, as any grey below half does; the grey, the light grey, the skin and the white, all at or above half brightness, take no shadows and stay as they are. | largest difference 1.9e-7 | yes |
| FX-BALANCE-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-003 frame 0: Shadows -100, -100, -100: the line and the dark grey darken; black, already at 0, is held there and stays black. | largest difference 1.9e-7 | yes |
| FX-BALANCE-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-004 frame 0: Midtones 0, 100, 0: the grey takes nearly all of it and turns a light green; the dark and light greys take about half; black and white take none and stay as they are. | largest difference 1.9e-7 | yes |
| FX-BALANCE-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-005 frame 0: Midtones 0, -100, 0: the grey loses nearly half its green scale and turns a dull purple; black and white stay. | largest difference 1.9e-7 | yes |
| FX-BALANCE-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-006 frame 0: Highlights 0, 0, 100: the skin and the light grey turn bluer; the white's blue is already full and is held there, so the white stays white; black and the line stay. | largest difference 1.9e-7 | yes |
| FX-BALANCE-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-007 frame 0: Highlights 0, 0, -100: the white turns a pale yellow, its blue half way down, #ffff80 but for rounding (127.5); the skin turns more yellow; black stays. | largest difference 1.9e-7 | yes |
| FX-BALANCE-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-008 frame 0: All three 100, 100, 100: every shown pixel, whatever its tone, is lifted half the scale on every channel, held at the top: black turns mid grey (127.5) and white stays white. | largest difference 3.7e-8 | yes |
| FX-BALANCE-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-009 frame 0: Shadows 0, 0, 100 and highlights 100, 0, 0, cool shadows and warm lights: the dark grey turns bluer, the light grey redder, and the grey, a hair above half, takes a two-thousandth of the highlights' red and no blue. | largest difference 1.3e-7 | yes |
| FX-BALANCE-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-010 frame 0: Shadows 100, 0, 0 and midtones 0, 0, 100: the dark grey, a quarter bright, takes about half of each, red and blue up alike, and stays neutral in green. | largest difference 1.9e-7 | yes |
| FX-BALANCE-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-011 frame 0: Shadows keyed from 0, 0, 0 at frame 0 to 100, 0, 0 at frame 4, linear, one key holding all three numbers: frame 0 untouched, frame 2 shadows 50, 0, 0, frame 4 FX-BALANCE-002. | largest difference 1.9e-7 | yes |
| FX-BALANCE-011 frame 2: Shadows keyed from 0, 0, 0 at frame 0 to 100, 0, 0 at frame 4, linear, one key holding all three numbers: frame 0 untouched, frame 2 shadows 50, 0, 0, frame 4 FX-BALANCE-002. | largest difference 1.9e-7 | yes |
| FX-BALANCE-011 frame 4: Shadows keyed from 0, 0, 0 at frame 0 to 100, 0, 0 at frame 4, linear, one key holding all three numbers: frame 0 untouched, frame 2 shadows 50, 0, 0, frame 4 FX-BALANCE-002. | largest difference 1.9e-7 | yes |
| FX-BALANCE-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-012 frame 0: Midtones keyed from -100, -100, -100 at frame 0 to 100, 100, 100 at frame 4: frame 0 darkens the middle tones, frame 2, all nine at 0, is the drawing untouched, and frame 4 lightens them. | largest difference 2.1e-7 | yes |
| FX-BALANCE-012 frame 2: Midtones keyed from -100, -100, -100 at frame 0 to 100, 100, 100 at frame 4: frame 0 darkens the middle tones, frame 2, all nine at 0, is the drawing untouched, and frame 4 lightens them. | largest difference 1.9e-7 | yes |
| FX-BALANCE-012 frame 4: Midtones keyed from -100, -100, -100 at frame 0 to 100, 100, 100 at frame 4: frame 0 darkens the middle tones, frame 2, all nine at 0, is the drawing untouched, and frame 4 lightens them. | largest difference 6.1e-8 | yes |
| FX-BALANCE-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-013 frame 0: Highlights eased from 0, 0, 0 at frame 0 to 0, 0, -100 at frame 4 on a curve that overshoots: at frame 2 the blue has gone past -100 and is held there, so frames 2 and 4 are both FX-BALANCE-007. | largest difference 1.9e-7 | yes |
| FX-BALANCE-013 frame 2: Highlights eased from 0, 0, 0 at frame 0 to 0, 0, -100 at frame 4 on a curve that overshoots: at frame 2 the blue has gone past -100 and is held there, so frames 2 and 4 are both FX-BALANCE-007. | largest difference 1.9e-7 | yes |
| FX-BALANCE-013 frame 4: Highlights eased from 0, 0, 0 at frame 0 to 0, 0, -100 at frame 4 on a curve that overshoots: at frame 2 the blue has gone past -100 and is held there, so frames 2 and 4 are both FX-BALANCE-007. | largest difference 1.9e-7 | yes |
| FX-BALANCE-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-014 frame 0: FX-BALANCE-009 moved three pixels right: the same, moved. | largest difference 1.3e-7 | yes |
| FX-BALANCE-014 frame 3: FX-BALANCE-009 moved three pixels right: the same, moved. | largest difference 1.3e-7 | yes |
| FX-BALANCE-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-BALANCE-015 frame 0: Shadows 101, 0, 0: a red above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-015 frame 4: Shadows 101, 0, 0: a red above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALANCE-016 frame 0: Midtones 0, -101, 0: a green below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-016 frame 4: Midtones 0, -101, 0: a green below -100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALANCE-017 frame 0: Highlights 0, 0, 150: a blue above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-017 frame 4: Highlights 0, 0, 150: a blue above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALANCE-018 frame 0: Highlights keyed to 0, 0, 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-018 frame 4: Highlights keyed to 0, 0, 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-BALANCE-019 frame 0: Shadows written with two numbers, 100, 0: a tone is three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-019 frame 4: Shadows written with two numbers, 100, 0: a tone is three. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-BALANCE-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview changes nothing: it has no distances | ColorBalance { shadows: [0.0, 0.0, 40.0], midtones: [-10.0, 5.0, 0.0], highlights: [30.0, 10.0, -20.0], preserve_luminosity: "off" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_balance_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_balance_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `midtones` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a shadow red that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| shadows 101, 0, 0 is refused with a sentence, and nothing changes | Color Balance's shadows runs from -100 to 100, and this is 101. | yes |
| midtones 0, -101, 0 is refused with a sentence, and nothing changes | Color Balance's midtones runs from -100 to 100, and this is -101. | yes |
| highlights 0, 0, 150 is refused with a sentence, and nothing changes | Color Balance's highlights runs from -100 to 100, and this is 150. | yes |
| shadows of two numbers, 100, 0 is refused with a sentence, and nothing changes | Color Balance's shadows are three numbers, red, green and blue, and this has 2. | yes |
| highlights of four numbers is refused with a sentence, and nothing changes | Color Balance's highlights are three numbers, red, green and blue, and this has 4. | yes |
| highlights keyed to 0, 0, 150 is refused with a sentence, and nothing changes | Color Balance's highlights runs from -100 to 100, and this is 150. | yes |
| all nine at the tops, 100, is taken | taken | yes |
| all nine at the bottoms, -100, is taken | taken | yes |
| shadows keyed from 0, 0, 0 to 100, 0, 0 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_balance_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_balance_012.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |

## Result

76 of 76 checks pass.
