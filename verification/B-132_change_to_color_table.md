# B-132: Change to Color

D-197, accepted on 2026-09-28 with the After Effects picks (A7). Every expected pixel is `Fixtures/change_to_color/expected_change_to_color.json`, written by `tools/change_to_color_reference.py` before this code existed and printed in document 25 as FX-CTC-001 to 027. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-CTC-001 to 027 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CTC-001 frame 0: The settings as they start: from #ff0000 to #0080ff, Hue, by setting, hue tolerance 5, lightness and saturation tolerance 50, softness 50. The red, its shadow 5.5 degrees round the wheel, its highlight, the dark red and the red at half covering take #0080ff's hue, each keeping its own lightness and saturation, so the shading stays; the dull red, too far from #ff0000's saturation, the pink and the skin's shadow, 15 degrees round, and every other colour are untouched. | largest difference 1.9e-7 | yes |
| FX-CTC-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-002 frame 0: From #c82828, the drawing's own red: the same, and now the dull red, its saturation 0.524 from the red's, just past the tolerance of 0.5, is changed nine tenths of the way. | largest difference 1.9e-7 | yes |
| FX-CTC-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-003 frame 0: Change Hue & Lightness: the changed colours also take #0080ff's lightness, 0.5, so the shadow, the highlight and the dark red lose their light and dark. | largest difference 1.9e-7 | yes |
| FX-CTC-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-004 frame 0: Change Hue & Saturation: the changed colours take #0080ff's saturation, 1, each keeping its own lightness. | largest difference 1.9e-7 | yes |
| FX-CTC-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-005 frame 0: Change Hue, Lightness & Saturation: every changed colour becomes #0080ff itself; the dull red nine tenths of the way there. | largest difference 1.9e-7 | yes |
| FX-CTC-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-006 frame 0: Change By transforming, Hue: each changed hue is turned by as far as #0080ff's is from #c82828's, 209.9 degrees, so the shadow, 354.5 degrees, lands at 204.4, not 209.9. | largest difference 1.9e-7 | yes |
| FX-CTC-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-007 frame 0: Change By transforming, Hue, Lightness & Saturation: each changed colour's lightness is raised by 0.029 and its saturation by 1/3, held at 1, as #0080ff is lighter and fuller than #c82828. | largest difference 1.9e-7 | yes |
| FX-CTC-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-008 frame 0: Hue tolerance 0, softness 0: only a hue exactly the red's changes; the red, the highlight, the dark red and the soft red do, the shadow does not, and the dull red, at softness 0, is past its saturation tolerance. | largest difference 1.9e-7 | yes |
| FX-CTC-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-009 frame 0: Hue tolerance 10, softness 0, 18 degrees each way: the pink and the skin's shadow, 15 degrees off, change whole with the reds; the skin, 25.7 off, does not. | largest difference 1.9e-7 | yes |
| FX-CTC-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-010 frame 0: Softness 0 at the other settings as they start: FX-CTC-002 with the dull red untouched. | largest difference 1.9e-7 | yes |
| FX-CTC-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-011 frame 0: Softness 100: the band past each tolerance is as wide as the tolerance, so the pink changes a third of the way, the skin's shadow a third, and the dull red 95 hundredths. | largest difference 2.6e-7 | yes |
| FX-CTC-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-012 frame 0: View Correction Matte on, at FX-CTC-002's settings: every pixel that shows is grey, white where the colour changes whole, black where not at all, and the dull red's nine tenths between, each at its own covering. | largest difference 1.5e-7 | yes |
| FX-CTC-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-013 frame 0: To #808080, a grey, with Hue & Saturation: the reds turn grey, each at its own lightness. | largest difference 1.9e-7 | yes |
| FX-CTC-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-014 frame 0: From #808080, a grey, has no hue: no colour is near it, and the drawing is untouched. | largest difference 1.9e-7 | yes |
| FX-CTC-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-015 frame 0: All three tolerances 100: every colour takes #0080ff's hue, each keeping its lightness and saturation; the grey, having no saturation, stays grey. | largest difference 1.9e-7 | yes |
| FX-CTC-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-016 frame 0: Hue tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 changes only the hues exactly red's, fading nothing in, since a tolerance of 0 has no band; frame 2, at 10, takes in the pink and the skin's shadow and a seventh of the skin; frame 4, at 20, the skin whole. | largest difference 1.9e-7 | yes |
| FX-CTC-016 frame 2: Hue tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 changes only the hues exactly red's, fading nothing in, since a tolerance of 0 has no band; frame 2, at 10, takes in the pink and the skin's shadow and a seventh of the skin; frame 4, at 20, the skin whole. | largest difference 7.2e-7 | yes |
| FX-CTC-016 frame 4: Hue tolerance keyed from 0 at frame 0 to 20 at frame 4, linear: frame 0 changes only the hues exactly red's, fading nothing in, since a tolerance of 0 has no band; frame 2, at 10, takes in the pink and the skin's shadow and a seventh of the skin; frame 4, at 20, the skin whole. | largest difference 1.9e-7 | yes |
| FX-CTC-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-017 frame 0: Lightness tolerance keyed from 10 at frame 0 to 100 at frame 4, eased past its end: frame 0 changes the red whole, the shadow, 0.137 from its lightness, a quarter, and neither the highlight nor the dark red; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-CTC-017 frame 2: Lightness tolerance keyed from 10 at frame 0 to 100 at frame 4, eased past its end: frame 0 changes the red whole, the shadow, 0.137 from its lightness, a quarter, and neither the highlight nor the dark red; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-CTC-017 frame 4: Lightness tolerance keyed from 10 at frame 0 to 100 at frame 4, eased past its end: frame 0 changes the red whole, the shadow, 0.137 from its lightness, a quarter, and neither the highlight nor the dark red; frame 2 is held at 100 and is frame 4. | largest difference 1.9e-7 | yes |
| FX-CTC-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-018 frame 0: FX-CTC-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-CTC-018 frame 3: FX-CTC-002 moved three pixels right: the same, moved. | largest difference 1.9e-7 | yes |
| FX-CTC-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-019 frame 0: FX-CTC-002 with its colours written in capitals, #C82828 and #0080FF: the same. | largest difference 1.9e-7 | yes |
| FX-CTC-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CTC-020 frame 0: Hue tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-020 frame 4: Hue tolerance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-021 frame 0: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-021 frame 4: Softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-022 frame 0: Saturation tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-022 frame 4: Saturation tolerance keyed to 150 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-023 frame 0: A from colour written "#ff00", two digits short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-023 frame 4: A from colour written "#ff00", two digits short. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-024 frame 0: A to colour written "blue", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-024 frame 4: A to colour written "blue", a name, not #rrggbb. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-024: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-025 frame 0: A change written "saturation", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-025 frame 4: A change written "saturation", not one of the four. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-026 frame 0: A change by written "shift", not "setting" or "transforming". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-026 frame 4: A change by written "shift", not "setting" or "transforming". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-026: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CTC-027 frame 0: A view matte written "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-027 frame 4: A view matte written "yes", not "off" or "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-CTC-027: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it grows the drawing's bounds by nothing | 0 | yes |
| a half-size draft preview keeps every setting, none being a distance | ChangeToColor { from: "#ff0000", to: "#0080ff", change: "hue", change_by: "setting", hue_tolerance: 5.0, lightness_tolerance: 50.0, saturation_tolerance: 50.0, softness: 50.0, view_matte: "on" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ctc_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ctc_019.json, its colours written in capitals, is saved with them in small letters, as Leave Color's colour is | "#c82828" and "#0080ff" | yes |
| a file with no `view_matte` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a hue tolerance that is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a from colour that is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| hue tolerance 101 is refused with a sentence, and nothing changes | Change to Color's hue tolerance runs from 0 to 100, and this is 101. | yes |
| lightness tolerance -1 is refused with a sentence, and nothing changes | Change to Color's lightness tolerance runs from 0 to 100, and this is -1. | yes |
| saturation tolerance 100.5 is refused with a sentence, and nothing changes | Change to Color's saturation tolerance runs from 0 to 100, and this is 100.5. | yes |
| softness 101 is refused with a sentence, and nothing changes | Change to Color's softness runs from 0 to 100, and this is 101. | yes |
| from "#ff00" is refused with a sentence, and nothing changes | Change to Color's from colour is written #rrggbb, and this is "#ff00". | yes |
| to "blue" is refused with a sentence, and nothing changes | Change to Color's to colour is written #rrggbb, and this is "blue". | yes |
| change "saturation" is refused with a sentence, and nothing changes | Change to Color's change is "hue", "hue_lightness", "hue_saturation" or "hue_lightness_saturation", and this is "saturation". | yes |
| change by "shift" is refused with a sentence, and nothing changes | Change to Color's change by is "setting" or "transforming", and this is "shift". | yes |
| view matte "yes" is refused with a sentence, and nothing changes | Change to Color's view matte is "off" or "on", and this is "yes". | yes |
| softness keyed to 150 is refused with a sentence, and nothing changes | Change to Color's softness runs from 0 to 100, and this is 150. | yes |
| every setting at the top of its range, the last words, is taken | taken | yes |
| every setting at the bottom, the other words, is taken | taken | yes |
| hue tolerance keyed from 0 to 100 is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## Pictures: a figure in a red jacket, in `verification/B-132 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the plate with no effect, draws cleanly | [] | yes |
| hue.png, Hue: the jacket, its shadow and highlight turn blue, each as light or dark as it was; draws cleanly; the jacket's red, shadow and highlight at [(80, 75), (98, 75), (60, 75)], and the ribbon, the face, its shade, the paper and the line at [(80, 56), (75, 30), (94, 30), (10, 10), (80, 12)] | [], jacket [[40, 120, 200], [30, 85, 140], [140, 190, 240]], the rest [[255, 0, 64], [246, 214, 190], [220, 160, 140], [244, 236, 216], [30, 26, 36]] | yes |
| transforming.png, Transforming To Color: each turned as far round as #0080ff is from the red; draws cleanly; the jacket's red, shadow and highlight at [(80, 75), (98, 75), (60, 75)], and the ribbon, the face, its shade, the paper and the line at [(80, 56), (75, 30), (94, 30), (10, 10), (80, 12)] | [], jacket [[40, 120, 200], [30, 95, 140], [140, 190, 240]], the rest [[255, 0, 64], [246, 214, 190], [220, 160, 140], [244, 236, 216], [30, 26, 36]] | yes |
| flat.png, Hue, Lightness & Saturation: the jacket one flat #0080ff; draws cleanly; the jacket's red, shadow and highlight at [(80, 75), (98, 75), (60, 75)], and the ribbon, the face, its shade, the paper and the line at [(80, 56), (75, 30), (94, 30), (10, 10), (80, 12)] | [], jacket [[0, 128, 255], [0, 128, 255], [0, 128, 255]], the rest [[255, 0, 64], [246, 214, 190], [220, 160, 140], [244, 236, 216], [30, 26, 36]] | yes |
| matte.png, View Correction Matte: white where the colour changes, black elsewhere; draws cleanly; the jacket's red, shadow and highlight at [(80, 75), (98, 75), (60, 75)], and the ribbon, the face, its shade, the paper and the line at [(80, 56), (75, 30), (94, 30), (10, 10), (80, 12)] | [], jacket [[255, 255, 255], [255, 255, 255], [255, 255, 255]], the rest [[0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]] | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ctc_002.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ctc_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_ctc_018.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |

## Result

105 of 105 checks pass.
