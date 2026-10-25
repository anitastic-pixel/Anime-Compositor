# B-639: Drop Shadow's Shadow Only

D-760, accepted by the owner on 2026-10-23 in the audit's waves ("if it is to be a 1 to 1 match, then go for it"). Every expected pixel is `Fixtures/drop_shadow/expected_drop_shadow.json`, written by `tools/drop_shadow_reference.py` before this code existed and printed in document 25 as FX-SHADOW-024 to 030. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5. FX-SHADOW-001 to 023 are B-58's and did not move.

## FX-SHADOW-024 to 030 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHADOW-024 frame 0: Shadow Only "on" at the defaults: the card is left out and the shadow alone is drawn, FX-SHADOW-001's shadow exactly, black at half covering wherever it lies wholly under the card's moved shape; in row 6, where the card was, the shadow's top edge shows part covered, and above it, where the card was, the frame is empty. | largest difference 1.0e-8 | yes |
| FX-SHADOW-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-025 frame 0: Shadow Only "on", distance 0, softness 0: the shadow exactly where the card was, black at half the card's covering at every pixel; the soft pixel at (10, 4) is black at half of 128/255. | largest difference 1.5e-8 | yes |
| FX-SHADOW-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-026 frame 0: Shadow Only "off" written in the file: FX-SHADOW-001 exactly, as a file without the word. | largest difference 1.9e-7 | yes |
| FX-SHADOW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-027 frame 0: Shadow Only "on", colour #2040a0, opacity 100, direction 90, distance 2, softness 5 (3 before D-774): the blue shadow alone, softened, two pixels right of the card's place. | largest difference 1.5e-7 | yes |
| FX-SHADOW-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-028 frame 0: Shadow Only "on" with opacity 0: nothing at all, the frame is empty, though the layer still grows. | largest difference 0.0e0 | yes |
| FX-SHADOW-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SHADOW-029 frame 0: Shadow Only written "On", the panel's name, not the word "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-029 frame 4: Shadow Only written "On", the panel's name, not the word "on". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SHADOW-030 frame 0: Shadow Only written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-030 frame 4: Shadow Only written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-SHADOW-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## A file from before D-760

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_shadow_001.json, which has no `shadow_only`, opens with Shadow Only off | DropShadow { color: "#000000", opacity: 50.0, direction: 135.0, distance: 5.0, softness: 0.0, shadow_only: "off" } | yes |
| its frame 0 is the frame of fx_shadow_026.json, which says "off" | byte-identical | yes |
| saved again, it still has no `shadow_only` | {"color":"#000000","direction":135,"distance":5,"opacity":50,"softness":0} | yes |
| fx_shadow_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_shadow_024.json is saved with Shadow Only "on" | "on" | yes |
| fx_shadow_026.json, which wrote "off", keeps it | "off" | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| Shadow Only "On" is refused with a sentence, and nothing changes | Drop Shadow's shadow only is "on" or "off", and this is "On". | yes |
| Shadow Only "yes" is refused with a sentence, and nothing changes | Drop Shadow's shadow only is "on" or "off", and this is "yes". | yes |
| Shadow Only "on" is taken | taken | yes |
| undo 1 times: frame 0 is the frame it was | byte-identical | yes |
| fx_shadow_001.json with Shadow Only turned on draws fx_shadow_024.json's frame | byte-identical | yes |

## The card against the processor

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SHADOW-001, fx_shadow_001.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-002, fx_shadow_002.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-003, fx_shadow_003.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-004, fx_shadow_004.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-005, fx_shadow_005.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-006, fx_shadow_006.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-007, fx_shadow_007.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-008, fx_shadow_008.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-009, fx_shadow_009.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-010, fx_shadow_010.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-011, fx_shadow_011.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-012, fx_shadow_012.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-013, fx_shadow_013.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-014, fx_shadow_014.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-015, fx_shadow_015.json, frames [0, 2, 4] at Full and Draft | largest difference 0 of 255; on the card in 6 of 6 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-016, fx_shadow_016.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-017, fx_shadow_017.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-018, fx_shadow_018.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-019, fx_shadow_019.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-020, fx_shadow_020.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-021, fx_shadow_021.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-022, fx_shadow_022.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-023, fx_shadow_023.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-024, fx_shadow_024.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-025, fx_shadow_025.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-026, fx_shadow_026.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-027, fx_shadow_027.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-028, fx_shadow_028.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-029, fx_shadow_029.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-030, fx_shadow_030.json, frames [0, 4] at Full and Draft | largest difference 0 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-031, fx_shadow_031.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-032, fx_shadow_032.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| FX-SHADOW-033, fx_shadow_033.json, frames [0] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |

## Result

64 of 64 checks pass.
