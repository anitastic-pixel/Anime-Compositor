# B-794: the viewer's colour view, checked against OpenColorIO

Written by `tests/b794_colour_view.rs` (D-915). The expected levels are OpenColorIO 2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless, display "sRGB - Display"), written by `tools/view_reference.py` into `Fixtures/colour/view/expected_view.json` before the code. The bar: within 1 level of 255 on every channel on at least 99.9% of a barred case's pixels. A view this build does not have is kept, shown as Standard and warned `COLOR_VIEW_UNAVAILABLE`.

**62 of 62 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| SHA-256 of "abc" (the standard's own example) | ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad | ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad | pass |
| the built-in ACES table's SHA-256 | 7cf78d4b78be8c92617b9c32c58fed6326a9fec405a08345a90f6412a127df72 | 7cf78d4b78be8c92617b9c32c58fed6326a9fec405a08345a90f6412a127df72 | pass |
| its colours (97 cubed) | 912673 | 912673 | pass |
| ACES 2.0 SDR view, white ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, white ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| ACES 2.0 SDR view, fire to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, fire to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 red ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 green ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 blue ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 cyan ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 magenta ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 yellow ramp to 64, acescg: within 1 level on at least 99.9% | true | true | pass |
| ACES 2.0 SDR view, ColorChecker, 4 stops under to 6 over, acescg: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, ColorChecker, 4 stops under to 6 over, acescg: within 1 level on at least 99.9% | true | true | pass |
| ACES 2.0 SDR view, white ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, white ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| ACES 2.0 SDR view, fire to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, fire to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 red ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 green ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 blue ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 cyan ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 magenta ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, pure Rec.709 yellow ramp to 64, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| ACES 2.0 SDR view, ColorChecker, 4 stops under to 6 over, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, ColorChecker, 4 stops under to 6 over, linear-srgb: within 1 level on at least 99.9% | true | true | pass |
| Un-tone-mapped view, stress: 4000 colours, half outside Rec.709, acescg: within 1 level on at least 99.9% | true | true | pass |
| hdr_scene_aces.json, 8-bit: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_aces.json, Float: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_aces.json, 8 bpc (After Effects): the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_un_tone_mapped.json, 8-bit: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_un_tone_mapped.json, Float: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_un_tone_mapped.json, 8 bpc (After Effects): the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_standard.json, 8-bit: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_standard.json, Float: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_standard.json, 8 bpc (After Effects): the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_off.json, 8-bit: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_off.json, Float: the card within 1 level of the processor on every pixel | true | true | pass |
| hdr_scene_off.json, 8 bpc (After Effects): the card within 1 level of the processor on every pixel | true | true | pass |
| an export on the processor: ACES 2.0 SDR and Un-tone-mapped files are byte for byte the Standard file | true, true | true, true | pass |
| an export on the card: ACES 2.0 SDR and Un-tone-mapped files are byte for byte the Standard file | true, true | true, true | pass |
| hdr_scene_aces.json (no view) shows | ACES 2.0 SDR | ACES 2.0 SDR | pass |
| hdr_scene_un_tone_mapped.json shows | Un-tone-mapped | Un-tone-mapped | pass |
| hdr_scene_standard.json shows | Standard | Standard | pass |
| hdr_scene_off.json shows | Standard | Standard | pass |
| hdr_scene_off_with_view.json shows | Standard | Standard | pass |
| hdr_scene_aces.json saves back byte for byte | true | true | pass |
| hdr_scene_un_tone_mapped.json saves back byte for byte | true | true | pass |
| hdr_scene_standard.json saves back byte for byte | true | true | pass |
| hdr_scene_off.json saves back byte for byte | true | true | pass |
| hdr_scene_off_with_view.json saves back byte for byte | true | true | pass |
| hdr_scene_unknown_view.json saves back byte for byte | true | true | pass |
| project.set_view standard on hdr_scene_aces.json writes view after alpha_mode | true, true | true, true | pass |
| and back to ACES 2.0 SDR the file is as it was, no view | true | true | pass |
| a file that had a view keeps the word when set to ACES 2.0 SDR | true | true | pass |
| hdr_scene_off.json: project.set_view is refused, the file unchanged | Colour views need colour management, which this project does not have on., true | Colour views need colour management, which this project does not have on., true | pass |
| hdr_scene_off_with_view.json: project.set_view is refused, the file unchanged | Colour views need colour management, which this project does not have on., true | Colour views need colour management, which this project does not have on., true | pass |
| a view this build does not have is refused | false | false | pass |
| hdr_scene_unknown_view.json opens with one COLOR_VIEW_UNAVAILABLE naming aces-1.3-sdr | 1, true | 1, true | pass |
| and shows | Standard | Standard | pass |
| a view that is not a word is refused at /color_settings/view | PROJECT_SCHEMA_INVALID, true | PROJECT_SCHEMA_INVALID, true | pass |

## The baked table and the processor against OpenColorIO, case by case

Each case's colours (in the working space named) through this build's processor view, against OpenColorIO's own processor. The pure primary and secondary ramps and the stress colours are reported for ACES 2.0 SDR, not barred (D-915: a 97-point table cannot follow the gamut compression's edge there to 1 level).

| Case | Working space | View | Pixels | Largest difference | Within 1 level | Result |
|---|---|---|---|---|---|---|
| white ramp to 64 | acescg | ACES 2.0 SDR | 512 | 1 | 100.00% | pass |
| white ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| fire to 64 | acescg | ACES 2.0 SDR | 512 | 1 | 100.00% | pass |
| fire to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 red ramp to 64 | acescg | ACES 2.0 SDR | 512 | 2 | 97.07% | reported |
| pure Rec.709 red ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 green ramp to 64 | acescg | ACES 2.0 SDR | 512 | 1 | 100.00% | reported |
| pure Rec.709 green ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 blue ramp to 64 | acescg | ACES 2.0 SDR | 512 | 3 | 98.63% | reported |
| pure Rec.709 blue ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 cyan ramp to 64 | acescg | ACES 2.0 SDR | 512 | 1 | 100.00% | reported |
| pure Rec.709 cyan ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 magenta ramp to 64 | acescg | ACES 2.0 SDR | 512 | 3 | 90.23% | reported |
| pure Rec.709 magenta ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 yellow ramp to 64 | acescg | ACES 2.0 SDR | 512 | 4 | 88.67% | reported |
| pure Rec.709 yellow ramp to 64 | acescg | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| ColorChecker, 4 stops under to 6 over | acescg | ACES 2.0 SDR | 264 | 1 | 100.00% | pass |
| ColorChecker, 4 stops under to 6 over | acescg | Un-tone-mapped | 264 | 1 | 100.00% | pass |
| white ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 1 | 100.00% | pass |
| white ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| fire to 64 | linear-srgb | ACES 2.0 SDR | 512 | 1 | 100.00% | pass |
| fire to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 red ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 2 | 97.07% | reported |
| pure Rec.709 red ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 green ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 1 | 100.00% | reported |
| pure Rec.709 green ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 blue ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 3 | 98.63% | reported |
| pure Rec.709 blue ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 cyan ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 1 | 100.00% | reported |
| pure Rec.709 cyan ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 magenta ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 3 | 90.23% | reported |
| pure Rec.709 magenta ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| pure Rec.709 yellow ramp to 64 | linear-srgb | ACES 2.0 SDR | 512 | 4 | 88.67% | reported |
| pure Rec.709 yellow ramp to 64 | linear-srgb | Un-tone-mapped | 512 | 0 | 100.00% | pass |
| ColorChecker, 4 stops under to 6 over | linear-srgb | ACES 2.0 SDR | 264 | 1 | 100.00% | pass |
| ColorChecker, 4 stops under to 6 over | linear-srgb | Un-tone-mapped | 264 | 0 | 100.00% | pass |
| stress: 4000 colours, half outside Rec.709 | acescg | ACES 2.0 SDR | 4000 | 6 | 99.80% | reported |
| stress: 4000 colours, half outside Rec.709 | acescg | Un-tone-mapped | 4000 | 0 | 100.00% | pass |

## The card against the processor

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. `media/hdr_scene.exr` through each project, at each depth. 8 bpc (After Effects) frames with a view are drawn by the processor (the card's 8 bpc sum is in display values), so they must match exactly; every other frame must be the card's. Under a view each drawing goes to the card in full precision, never half: in the first run the 8-bit Un-tone-mapped frame was 39 levels off on 7,904 pixels, where the matrix to Rec.709 cancels bright channels. "Past 1" counts the processor's pixels with a channel over 1.0, and the largest difference among them.

| Project | View | Depth | Drawn by | Pixels past 1 | Largest difference | Largest past 1 | Pixels over 1 level | Result |
|---|---|---|---|---|---|---|---|---|
| hdr_scene_aces.json | ACES 2.0 SDR | 8-bit | card | 59360 | 0 | 0 | 0 | pass |
| hdr_scene_aces.json | ACES 2.0 SDR | Float | card | 59360 | 0 | 0 | 0 | pass |
| hdr_scene_aces.json | ACES 2.0 SDR | 8 bpc (After Effects) | processor (refused) | 0 | 0 | 0 | 0 | pass |
| hdr_scene_un_tone_mapped.json | Un-tone-mapped | 8-bit | card | 59360 | 0 | 0 | 0 | pass |
| hdr_scene_un_tone_mapped.json | Un-tone-mapped | Float | card | 59360 | 0 | 0 | 0 | pass |
| hdr_scene_un_tone_mapped.json | Un-tone-mapped | 8 bpc (After Effects) | processor (refused) | 0 | 0 | 0 | 0 | pass |
| hdr_scene_standard.json | Standard | 8-bit | card | 59360 | 1 | 1 | 0 | pass |
| hdr_scene_standard.json | Standard | Float | card | 59360 | 0 | 0 | 0 | pass |
| hdr_scene_standard.json | Standard | 8 bpc (After Effects) | card | 0 | 0 | 0 | 0 | pass |
| hdr_scene_off.json | Standard | 8-bit | card | 61344 | 1 | 1 | 0 | pass |
| hdr_scene_off.json | Standard | Float | card | 61344 | 0 | 0 | 0 | pass |
| hdr_scene_off.json | Standard | 8 bpc (After Effects) | card | 0 | 0 | 0 | 0 | pass |
