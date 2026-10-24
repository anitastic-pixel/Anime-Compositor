# B-657: Radio Waves, Parameters Are Set At and Sides up to 128

D-777, accepted on 2026-10-24. Every expected pixel is `Fixtures/radio_waves/expected_radio_waves.json`, written by `tools/radio_waves_reference.py` before this code existed and printed in document 25 as FX-RWAVE-028 to 035. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-RWAVE-028 to 035 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RWAVE-028 frame 4: As FX-RWAVE-008, 128 sides, After Effects' top: three waves round the middle at frame 4, each sample within 0.01 of FX-RWAVE-008's 64 sides. | largest difference 3.2e-8 | yes |
| FX-RWAVE-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-029 frame 0: As FX-RWAVE-015, Parameters Are Set At Birth: each wave keeps the producer point it was born at, so at frame 4 the wave born at frame 0 is round (4, 5), the one born at frame 2 round (8, 5) and the dot born at frame 4 at (12, 5); at frame 0 it is FX-RWAVE-015's dot. | largest difference 1.4e-8 | yes |
| FX-RWAVE-029 frame 4: As FX-RWAVE-015, Parameters Are Set At Birth: each wave keeps the producer point it was born at, so at frame 4 the wave born at frame 0 is round (4, 5), the one born at frame 2 round (8, 5) and the dot born at frame 4 at (12, 5); at frame 0 it is FX-RWAVE-015's dot. | largest difference 2.9e-8 | yes |
| FX-RWAVE-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-030 frame 0: As FX-RWAVE-015, Parameters Are Set At Each Frame, written: FX-RWAVE-015 exactly, every wave round where the point is now. | largest difference 1.4e-8 | yes |
| FX-RWAVE-030 frame 4: As FX-RWAVE-015, Parameters Are Set At Each Frame, written: FX-RWAVE-015 exactly, every wave round where the point is now. | largest difference 2.9e-8 | yes |
| FX-RWAVE-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-031 frame 2: As FX-RWAVE-008, the opacity keyed from 100 at frame 0 to 0 at frame 4, linear, set at birth: at frame 4 the wave born at frame 0 is whole, the one born at frame 2 at half strength and the dot born at frame 4 gone; set at each frame all three would be gone. | largest difference 2.9e-8 | yes |
| FX-RWAVE-031 frame 4: As FX-RWAVE-008, the opacity keyed from 100 at frame 0 to 0 at frame 4, linear, set at birth: at frame 4 the wave born at frame 0 is whole, the one born at frame 2 at half strength and the dot born at frame 4 gone; set at each frame all three would be gone. | largest difference 2.9e-8 | yes |
| FX-RWAVE-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-032 frame 4: As FX-RWAVE-008, set at birth with nothing keyed: FX-RWAVE-008 exactly. | largest difference 2.9e-8 | yes |
| FX-RWAVE-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-033 frame 2: As the ring, spin 22.5 a frame, lifespan keyed from 10 at frame 0 to 1 at frame 1, hold, set at birth: the wave born at frame 0 keeps its 10 and is FX-RWAVE-006 at frame 2. | largest difference 1.4e-8 | yes |
| FX-RWAVE-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-RWAVE-034 frame 0: Sides 129, above 128. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-034 frame 4: Sides 129, above 128. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-RWAVE-035 frame 0: Parameters set at "death", which is not "birth" or "each_frame". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-035 frame 4: Parameters set at "death", which is not "birth" or "each_frame". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-RWAVE-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Projects from before D-777

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-RWAVE-015 frame 0: As FX-RWAVE-008, the producer point keyed from (25, 50) at frame 0 to (75, 50) at frame 4, linear: every wave alive is sent from where the point is now, so frame 4's three are round (12, 5). | largest difference 1.4e-8 | yes |
| FX-RWAVE-015 frame 4: As FX-RWAVE-008, the producer point keyed from (25, 50) at frame 0 to (75, 50) at frame 4, linear: every wave alive is sent from where the point is now, so frame 4's three are round (12, 5). | largest difference 2.9e-8 | yes |
| FX-RWAVE-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| fx_rwave_015.json, written before D-777, saves no `parameters_at`: each frame, as before | None | yes |
| and opens as each frame | each_frame | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_rwave_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_rwave_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| the waves' birth settings are never saved | None | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| parameters set at "death" is refused with a sentence, and nothing changes | Radio Waves' parameters are set at "birth" or "each_frame", and this is "death". | yes |
| sides 128.5 is refused with a sentence, and nothing changes | Radio Waves's sides runs from 3 to 128, and this is 128.5. | yes |
| sides keyed to 129 is refused with a sentence, and nothing changes | Radio Waves's sides runs from 3 to 128, and this is 129. | yes |
| sides 128 is taken | taken | yes |
| each frame is taken | taken | yes |
| the producer point keyed across the layer is taken | taken | yes |
| undo 3 times: frame 0 is the frame it was | byte-identical | yes |

## A half-size draft

| Check | The build's answer | Matches |
| --- | --- | --- |
| halves a wave's expansion, 6 to 3, velocity, 8 to 4, and widths, 2 to 1 and 10 to 5, and keeps its birth frame, producer, lifespan and opacity | [2.0, 25.0, 50.0, 3.0, 0.0, 90.0, 4.0, 0.0, 5.0, 100.0, 0.0, 0.0, 1.0, 5.0] | yes |

## Result

39 of 39 checks pass.
