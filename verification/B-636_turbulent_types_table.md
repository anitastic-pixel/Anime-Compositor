# B-636: Turbulent Displace's offset and types (D-757)

D-757 adds After Effects' Offset (Turbulence) as `offset` (pixels, x then y, keyable, -100000 to 100000, absent (0, 0)), which slides the field as D-411's drift does, and the Displacement words cross, bulge, twist, turbulent_smoother, bulge_smoother and twist_smoother (document 21). A file without them draws as before. Every expected pixel is `Fixtures/turbulent_types/expected_turbulent_types.json`, written by `tools/turbulent_types_reference.py` before this code existed and printed in document 25 as FX-TURB-TY-001 to 018. Tolerance 2e-5.

## FX-TURB-TY-001 to 018 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TURB-TY-001 frame 0: Offset (3, 0), amount 30, size 8, speed 0: the field slides three pixels right, so each pixel gets the push the pixel three to its left had in FX-TURB-AE-007. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-002 frame 0: Offset (0, 0) written: FX-TURB-AE-007 exactly. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-003 frame 0: Offset keyed from (0, 0) at frame 0 to (8, -4) at frame 4, linear: frame 0 is FX-TURB-AE-007, frame 2 offset (4, -2), frame 4 (8, -4), one whole wave across. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-003 frame 2: Offset keyed from (0, 0) at frame 0 to (8, -4) at frame 4, linear: frame 0 is FX-TURB-AE-007, frame 2 offset (4, -2), frame 4 (8, -4), one whole wave across. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-003 frame 4: Offset keyed from (0, 0) at frame 0 to (8, -4) at frame 4, linear: frame 0 is FX-TURB-AE-007, frame 2 offset (4, -2), frame 4 (8, -4), one whole wave across. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-004 frame 0: Displacement cross: the sideways push from a field stretched eight times across, the up-and-down push from one stretched eight times down. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-005 frame 0: Displacement bulge: each pixel reads from uphill on the field, so its hills swell. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-006 frame 0: Displacement twist: the bulge's push turned a quarter, carrying the pixels round the hills. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-007 frame 0: Displacement turbulent_smoother, complexity 4: FX-TURB-AE-007's push with each finer octave a quarter of the last's strength, not a half. | largest difference 2.7e-7 | yes |
| FX-TURB-TY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-008 frame 0: Displacement bulge_smoother, complexity 4. | largest difference 2.7e-7 | yes |
| FX-TURB-TY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-009 frame 0: Displacement twist_smoother, complexity 4. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-010 frame 0: Displacement turbulent_smoother, complexity 1: one octave has nothing finer to weaken, so this is FX-TURB-AE-007 with complexity 1. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-011 frame 0: Bulge with edges repeat: nothing grows, and a push past the drawing's edge reads the nearest edge pixel. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-012 frame 0: Cross moved three pixels right: worked in the drawing's own space, so it moves with it. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-013 frame 0: Twist with offset (2.5, -1.5), seed 8 and speed 90: the offset slides the twist's field as it slides the turbulent one, and the field moves from frame 0 to 2. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-013 frame 2: Twist with offset (2.5, -1.5), seed 8 and speed 90: the offset slides the twist's field as it slides the turbulent one, and the field moves from frame 0 to 2. | largest difference 2.7e-7 | yes |
| FX-TURB-TY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-014 frame 0: Bulge in a file without units, D-127's classic push: amount 3, size 8. | largest difference 2.5e-7 | yes |
| FX-TURB-TY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-TURB-TY-015 frame 0: Displacement "Bulge": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-015 frame 4: Displacement "Bulge": the word is exact, so a capital is not it. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-TY-016 frame 0: Displacement "cross_displacement", After Effects' name, which is not one of the words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-016 frame 4: Displacement "cross_displacement", After Effects' name, which is not one of the words. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-TY-017 frame 0: Offset (100001, 0), above 100000 across. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-017 frame 4: Offset (100001, 0), above 100000 across. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-TURB-TY-018 frame 0: Offset keyed to (0, -200000) at frame 4, below -100000 down. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-018 frame 4: Offset keyed to (0, -200000) at frame 4, below -100000 down. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-TURB-TY-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## Older projects: no offset, as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| turbulent_displace/fx_turb_001.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_002.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_003.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_004.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_005.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_006.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_007.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_008.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_009.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_010.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_011.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_012.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_013.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_014.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_015.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_016.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_017.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_018.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_019.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_020.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_021.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_022.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_023.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_024.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_025.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_displace/fx_turb_026.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_001.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_002.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_003.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_004.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_005.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_006.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_007.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_008.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_009.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_010.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_011.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| turbulent_ae/fx_turb_ae_012.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_001.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_002.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_003.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_004.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_005.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_006.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_007.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_008.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_009.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_010.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_011.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_012.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_013.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_014.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_015.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |
| heat_shimmer/fx_shimmer_016.json (no offset) is saved as it was written, and draws frames 0 and 4 bit for bit as its twin with Offset (0, 0) | saved the same; bit-identical | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_ty_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_turb_ty_001.json is saved with Offset (3, 0) | {"amount":30,"complexity":2,"displacement":"turbulent","edges":"transparent","evolution":0,"offset":[3,0],"seed":0,"size":8,"speed":0,"units":"after_effects"} | yes |
| fx_turb_ty_005.json is saved with Displacement bulge and no offset written | {"amount":30,"complexity":2,"displacement":"bulge","edges":"transparent","evolution":0,"seed":0,"size":8,"speed":0,"units":"after_effects"} | yes |
| fx_turb_ty_015.json (Displacement "Bulge") is refused in a sentence naming displacement | Turbulent Displace's displacement is one of turbulent, horizontal, vertical, cross, bulge, twist, turbulent_smoother, bulge_smoother, twist_smoother, and this is "Bulge". | yes |
| fx_turb_ty_016.json (Displacement "cross_displacement") is refused in a sentence naming displacement | Turbulent Displace's displacement is one of turbulent, horizontal, vertical, cross, bulge, twist, turbulent_smoother, bulge_smoother, twist_smoother, and this is "cross_displacement". | yes |
| fx_turb_ty_017.json (Offset (100001, 0)) is refused in a sentence naming offset | Turbulent Displace's offset runs from -100000 to 100000, and this is 100001. | yes |
| the sentence for a wrong word lists the nine words | Turbulent Displace's displacement is one of turbulent, horizontal, vertical, cross, bulge, twist, turbulent_smoother, bulge_smoother, twist_smoother, and this is "cross_displacement". | yes |
| a file with a Turbulent Displace whose Offset is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| a half-size draft preview halves the Offset, a distance | TurbulentDisplace { amount: 30.0, size: 4.0, complexity: 2.0, evolution: 0.0, speed: 0.0, seed: 0.0, edges: "transparent", frame: 0, displacement: "turbulent", pinning: "none", units: "after_effects", new_seed_every: 0.0, drift_direction: 0.0, drift_speed: 0.0, offset: [1.5, 0.0] } | yes |
| fx_turb_ty_001.json grows the drawing's bounds as the plain turbulent push does | 3 against 3 | yes |
| fx_turb_ty_005.json grows the drawing's bounds as the plain turbulent push does | 3 against 3 | yes |
| fx_turb_ty_006.json grows the drawing's bounds as the plain turbulent push does | 3 against 3 | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| offset (0, 100001) is refused with a sentence, and nothing changes | Turbulent Displace's offset runs from -100000 to 100000, and this is 100001. | yes |
| displacement "Twist" is refused with a sentence, and nothing changes | Turbulent Displace's displacement is one of turbulent, horizontal, vertical, cross, bulge, twist, turbulent_smoother, bulge_smoother, twist_smoother, and this is "Twist". | yes |
| offset keyed to (-200000, 0) is refused with a sentence, and nothing changes | Turbulent Displace's offset runs from -100000 to 100000, and this is -200000. | yes |
| twist_smoother with offset (12, -7.5) is taken | taken | yes |
| offset keyed from (0, 0) to (8, -4) is taken | taken | yes |
| undo 2 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_turb_ty_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_003.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_004.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_005.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_012.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_turb_ty_013.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |

## On the card against the processor: within 1 level of 255

| Check | The build's answer | Matches |
| --- | --- | --- |
| the card draws the plain turbulent push with an offset (001 to 003); the other words stay on the processor, as D-306's horizontal, vertical and pinning do, so those frames are the processor's | see the rows below | yes |
| FX-TURB-TY-001, fx_turb_ty_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-002, fx_turb_ty_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-003, fx_turb_ty_003.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-004, fx_turb_ty_004.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-005, fx_turb_ty_005.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-006, fx_turb_ty_006.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-007, fx_turb_ty_007.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-008, fx_turb_ty_008.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-009, fx_turb_ty_009.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-010, fx_turb_ty_010.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-011, fx_turb_ty_011.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-012, fx_turb_ty_012.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-013, fx_turb_ty_013.json, frames [0, 2] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-014, fx_turb_ty_014.json, frames [0] at Full and Draft | largest difference 1 of 255; on the card in 0 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-015, fx_turb_ty_015.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-016, fx_turb_ty_016.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-017, fx_turb_ty_017.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-TURB-TY-018, fx_turb_ty_018.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10): the processor's frame 100 differs from the shot without it | 1607962 pixels changed | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 0, Full | largest difference 1 of 255, 1792 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 100, Full | largest difference 1 of 255, 969 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 239, Full | largest difference 1 of 255, 854 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 0, Draft | largest difference 1 of 255, 59 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 100, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 30, size 40, offset (25, -10), frame 239, Draft | largest difference 1 of 255, 50 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25): the processor's frame 100 differs from the shot without it | 1547001 pixels changed | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 0, Full | largest difference 1 of 255, 2311 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 100, Full | largest difference 1 of 255, 1054 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 239, Full | largest difference 1 of 255, 1027 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 0, Draft | largest difference 1 of 255, 91 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 100, Draft | largest difference 1 of 255, 69 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace amount 60, size 20, speed 20, seed 7, drifting at 30 degrees at 2.5 pixels a frame, offset (-40.5, 13.25), frame 239, Draft | largest difference 1 of 255, 71 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's): the processor's frame 100 differs from the shot without it | 1577438 pixels changed | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 0, Full | largest difference 1 of 255, 35011 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 100, Full | largest difference 1 of 255, 34774 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 239, Full | largest difference 1 of 255, 34649 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 0, Draft | largest difference 1 of 255, 3394 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 100, Draft | largest difference 1 of 255, 3390 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Turbulent Displace bulge, amount 30, size 40 (the processor's), frame 239, Draft | largest difference 1 of 255, 3338 pixels differ; 0 of 0 on the card; warnings CPU [] GPU [] | yes |

## Pictures: in `verification/B-636 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_turbulent.png, the street with amount 20, size 40, as before; draws cleanly | [] | yes |
| 2_offset_40_0.png, the same warp slid 40 pixels right; draws cleanly and differs from 1_turbulent.png | []; 67190 pixels differ | yes |
| 3_cross.png, cross: streaks, stretched across for the sideways push and down for the up-and-down one; draws cleanly and differs from 1_turbulent.png | []; 67571 pixels differ | yes |
| 4_bulge.png, bulge: the field's hills swell; draws cleanly and differs from 1_turbulent.png | []; 70258 pixels differ | yes |
| 5_twist.png, twist: the pixels carried round the hills; draws cleanly and differs from 1_turbulent.png | []; 68102 pixels differ | yes |
| 6_turbulent_smoother.png, turbulent_smoother, complexity 4: finer octaves weaker; draws cleanly and differs from 1_turbulent.png | []; 24022 pixels differ | yes |

## Result

186 of 186 checks pass.
