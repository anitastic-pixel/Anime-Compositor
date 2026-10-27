# B-788: Interpret as, checked against OpenColorIO

Written by `tests/b788_interpret_as.rs` (D-909). The expected numbers are OpenColorIO 2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless), written by `tools/interpret_reference.py` into `Fixtures/colour/interpret/expected_interpretation.json` before the code. The bar: |build - expected| at most 1e-6 of the colour's size (at least 1), at every pixel.

**47 of 47 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| chart_tagged_aces2065_1.json (tagged aces2065-1, working linear-srgb): every pixel of every patch within the bar of OpenColorIO | 30 of 30 | 30 of 30 | pass |
| chart_tagged_aces2065_1.json: frame 0 drawn through the compositor equals the decoded drawing, bit for bit | true, no diagnostics | true, no diagnostics | pass |
| chart_tagged_aces2065_1.json: today's view (sRGB curve, 8 bits) shows each patch as OpenColorIO's numbers give | 30 of 30 | 30 of 30 | pass |
| tagged ACES2065-1, today's view gives back X-Rite's own sRGB bytes for every ColorChecker patch | 24 of 24 | 24 of 24 | pass |
| chart_tagged_aces2065_1.json saves back byte for byte | true | true | pass |
| chart_tagged_acescg.json (tagged acescg, working linear-srgb): every pixel of every patch within the bar of OpenColorIO | 30 of 30 | 30 of 30 | pass |
| chart_tagged_acescg.json: frame 0 drawn through the compositor equals the decoded drawing, bit for bit | true, no diagnostics | true, no diagnostics | pass |
| chart_tagged_acescg.json: today's view (sRGB curve, 8 bits) shows each patch as OpenColorIO's numbers give | 30 of 30 | 30 of 30 | pass |
| chart_tagged_acescg.json saves back byte for byte | true | true | pass |
| chart_tagged_linear_rec709.json (tagged linear-srgb, working linear-srgb): every pixel of every patch within the bar of OpenColorIO | 30 of 30 | 30 of 30 | pass |
| chart_tagged_linear_rec709.json: frame 0 drawn through the compositor equals the decoded drawing, bit for bit | true, no diagnostics | true, no diagnostics | pass |
| chart_tagged_linear_rec709.json: today's view (sRGB curve, 8 bits) shows each patch as OpenColorIO's numbers give | 30 of 30 | 30 of 30 | pass |
| chart_tagged_linear_rec709.json saves back byte for byte | true | true | pass |
| chart_managed_acescg.json (tagged aces2065-1, working acescg): every pixel of every patch within the bar of OpenColorIO | 30 of 30 | 30 of 30 | pass |
| chart_managed_acescg.json: frame 0 drawn through the compositor equals the decoded drawing, bit for bit | true, no diagnostics | true, no diagnostics | pass |
| chart_managed_acescg.json saves back byte for byte | true | true | pass |
| chart_managed_acescg_linear_rec709.json (tagged linear-srgb, working acescg): every pixel of every patch within the bar of OpenColorIO | 30 of 30 | 30 of 30 | pass |
| chart_managed_acescg_linear_rec709.json: frame 0 drawn through the compositor equals the decoded drawing, bit for bit | true, no diagnostics | true, no diagnostics | pass |
| chart_managed_acescg_linear_rec709.json saves back byte for byte | true | true | pass |
| ramp tagged acescg, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged aces2065-1, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged acescct, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged acescc, working linear-srgb: within the bar of the formula column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged linear-rec2020, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged linear-p3-d65, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged bt1886-rec709, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged gamma22-rec709, working linear-srgb: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged srgb, working acescg: within the bar of the formula column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged linear-srgb, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged acescg, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged aces2065-1, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged acescct, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged acescc, working acescg: within the bar of the formula column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged linear-rec2020, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged linear-p3-d65, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged bt1886-rec709, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| ramp tagged gamma22-rec709, working acescg: within the bar of the ocio column, times alpha exact, the bytes path the same bits | pass | pass | pass |
| a project saved before D-909 (minimal_project.json) opens with colour management off, reads its drawings as recorded, and saves byte for byte | off, true, true | off, true, true | pass |
| today's two words keep their names in the decoded-drawing key (document 27), so no kept drawing is thrown away | Srgb, LinearLight | Srgb, LinearLight | pass |
| each of the ten words reads and writes as itself | true | true | pass |
| unknown_space.json opens with one COLOR_SPACE_UNKNOWN naming the asset and the word | 1, true | 1, true | pass |
| its frame 0 is transparent and its log says COLOR_SPACE_UNKNOWN (so an export is marked incomplete) | transparent, true | transparent, true | pass |
| and it saves back byte for byte, the word included | true | true | pass |
| refused_acescg_without_management.json is refused at /color_settings/working_space | PROJECT_SCHEMA_INVALID, true | PROJECT_SCHEMA_INVALID, true | pass |
| refused_unknown_working_space.json is refused at /color_settings/working_space | PROJECT_SCHEMA_INVALID, true | PROJECT_SCHEMA_INVALID, true | pass |
| ramp_tags.json saves back byte for byte | true | true | pass |
| ramp_tags_managed_acescg.json saves back byte for byte | true | true | pass |

## The chart, patch by patch

Each patch's colour as OpenColorIO gives it and as this build decodes it (linear, in the working space), the worst pixel's difference over the colour's size, and, in today's working space, the bytes today's view shows (sRGB curve, 8 bits): OpenColorIO's / this build's. With ACEScg working there is no view to judge until C5 (D-909 gap 2).

| Project | Patch | OpenColorIO | This build | Worst difference | Within the bar | View bytes |
|---|---|---|---|---|---|---|
| chart_tagged_aces2065_1.json | dark skin | 0.171441 0.084376 0.057805 | 0.171441 0.084376 0.057805 | 1.5e-8 | pass | [115, 82, 68] / [115, 82, 68] pass |
| chart_tagged_aces2065_1.json | light skin | 0.539479 0.304987 0.223228 | 0.539480 0.304987 0.223228 | 6.0e-8 | pass | [194, 150, 130] / [194, 150, 130] pass |
| chart_tagged_aces2065_1.json | blue sky | 0.122139 0.194618 0.337164 | 0.122139 0.194618 0.337164 | 3.0e-8 | pass | [98, 122, 157] / [98, 122, 157] pass |
| chart_tagged_aces2065_1.json | foliage | 0.095307 0.149960 0.056128 | 0.095307 0.149960 0.056128 | 7.5e-9 | pass | [87, 108, 67] / [87, 108, 67] pass |
| chart_tagged_aces2065_1.json | blue flower | 0.234551 0.215860 0.439657 | 0.234551 0.215860 0.439657 | 2.9e-8 | pass | [133, 128, 177] / [133, 128, 177] pass |
| chart_tagged_aces2065_1.json | bluish green | 0.135633 0.508881 0.401978 | 0.135633 0.508881 0.401978 | 1.5e-8 | pass | [103, 189, 170] / [103, 189, 170] pass |
| chart_tagged_aces2065_1.json | orange | 0.672443 0.208637 0.025187 | 0.672443 0.208637 0.025187 | 6.0e-8 | pass | [214, 126, 44] / [214, 126, 44] pass |
| chart_tagged_aces2065_1.json | purplish blue | 0.080220 0.104616 0.381326 | 0.080220 0.104616 0.381326 | 3.0e-8 | pass | [80, 91, 166] / [80, 91, 166] pass |
| chart_tagged_aces2065_1.json | moderate red | 0.533276 0.102242 0.124772 | 0.533276 0.102242 0.124772 | 7.9e-9 | pass | [193, 90, 99] / [193, 90, 99] pass |
| chart_tagged_aces2065_1.json | purple | 0.111932 0.045186 0.149960 | 0.111932 0.045186 0.149960 | 3.1e-10 | pass | [94, 60, 108] / [94, 60, 108] pass |
| chart_tagged_aces2065_1.json | yellow green | 0.337164 0.502886 0.051269 | 0.337164 0.502886 0.051269 | 3.0e-8 | pass | [157, 188, 64] / [157, 188, 64] pass |
| chart_tagged_aces2065_1.json | orange yellow | 0.745404 0.366253 0.027321 | 0.745404 0.366253 0.027321 | 6.0e-8 | pass | [224, 163, 46] / [224, 163, 46] pass |
| chart_tagged_aces2065_1.json | blue | 0.039546 0.046665 0.304987 | 0.039546 0.046665 0.304987 | 1.5e-8 | pass | [56, 61, 150] / [56, 61, 150] pass |
| chart_tagged_aces2065_1.json | green | 0.061246 0.296138 0.066626 | 0.061246 0.296138 0.066626 | 4.1e-10 | pass | [70, 148, 73] / [70, 148, 73] pass |
| chart_tagged_aces2065_1.json | red | 0.428690 0.036889 0.045186 | 0.428690 0.036889 0.045186 | 2.9e-8 | pass | [175, 54, 60] / [175, 54, 60] pass |
| chart_tagged_aces2065_1.json | yellow | 0.799103 0.571125 0.013702 | 0.799103 0.571125 0.013702 | 6.0e-8 | pass | [231, 199, 31] / [231, 199, 31] pass |
| chart_tagged_aces2065_1.json | magenta | 0.496933 0.093059 0.300544 | 0.496933 0.093059 0.300544 | 4.0e-10 | pass | [187, 86, 149] / [187, 86, 149] pass |
| chart_tagged_aces2065_1.json | cyan | 0.002428 0.234551 0.356400 | 0.002428 0.234551 0.356400 | 3.0e-8 | pass | [8, 133, 161] / [8, 133, 161] pass |
| chart_tagged_aces2065_1.json | white 9.5 | 0.896269 0.896269 0.887923 | 0.896269 0.896269 0.887923 | 6.0e-8 | pass | [243, 243, 242] / [243, 243, 242] pass |
| chart_tagged_aces2065_1.json | neutral 8 | 0.577580 0.577580 0.577580 | 0.577580 0.577580 0.577580 | 1.2e-7 | pass | [200, 200, 200] / [200, 200, 200] pass |
| chart_tagged_aces2065_1.json | neutral 6.5 | 0.351533 0.351533 0.351533 | 0.351533 0.351533 0.351533 | 3.0e-8 | pass | [160, 160, 160] / [160, 160, 160] pass |
| chart_tagged_aces2065_1.json | neutral 5 | 0.194618 0.194618 0.191202 | 0.194618 0.194618 0.191202 | 2.9e-8 | pass | [122, 122, 121] / [122, 122, 121] pass |
| chart_tagged_aces2065_1.json | neutral 3.5 | 0.090842 0.090842 0.090842 | 0.090842 0.090842 0.090842 | 2.2e-8 | pass | [85, 85, 85] / [85, 85, 85] pass |
| chart_tagged_aces2065_1.json | black 2 | 0.034340 0.034340 0.034340 | 0.034340 0.034340 0.034340 | 3.8e-9 | pass | [52, 52, 52] / [52, 52, 52] pass |
| chart_tagged_aces2065_1.json | ACEScg red 0.5 | 0.852525 -0.065128 -0.012002 | 0.852525 -0.065128 -0.012002 | 6.0e-8 | pass | [238, 0, 0] / [238, 0, 0] pass |
| chart_tagged_aces2065_1.json | ACEScg green 0.5 | -0.310896 0.570402 -0.064484 | -0.310896 0.570402 -0.064484 | 7.4e-9 | pass | [0, 199, 0] / [0, 199, 0] pass |
| chart_tagged_aces2065_1.json | ACEScg blue 0.5 | -0.041629 -0.005274 0.576486 | -0.041629 -0.005274 0.576486 | 2.6e-8 | pass | [0, 0, 200] / [0, 0, 200] pass |
| chart_tagged_aces2065_1.json | grey 2 | 2.000000 2.000000 2.000000 | 2.000000 2.000000 2.000000 | 1.2e-7 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_aces2065_1.json | grey 8 | 7.999999 8.000000 8.000000 | 8.000000 8.000000 8.000000 | 1.2e-7 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_aces2065_1.json | warm bright | 6.177596 0.617142 0.063261 | 6.177597 0.617142 0.063261 | 7.8e-8 | pass | [255, 206, 71] / [255, 206, 71] pass |
| chart_tagged_acescg.json | dark skin | 0.140142 0.086216 0.057974 | 0.140142 0.086216 0.057974 | 4.8e-11 | pass | [105, 83, 68] / [105, 83, 68] pass |
| chart_tagged_acescg.json | light skin | 0.453451 0.309145 0.223811 | 0.453451 0.309145 0.223811 | 5.9e-8 | pass | [179, 151, 130] / [179, 151, 130] pass |
| chart_tagged_acescg.json | blue sky | 0.168636 0.202494 0.335505 | 0.168636 0.202494 0.335505 | 1.5e-8 | pass | [114, 124, 157] / [114, 124, 157] pass |
| chart_tagged_acescg.json | foliage | 0.096196 0.140172 0.057415 | 0.096196 0.140172 0.057415 | 3.7e-9 | pass | [87, 105, 68] / [87, 105, 68] pass |
| chart_tagged_acescg.json | blue flower | 0.266789 0.234184 0.436801 | 0.266789 0.234184 0.436801 | 2.9e-8 | pass | [141, 133, 176] / [141, 133, 176] pass |
| chart_tagged_acescg.json | bluish green | 0.232330 0.483746 0.404037 | 0.232330 0.483746 0.404037 | 1.5e-8 | pass | [132, 185, 170] / [132, 185, 170] pass |
| chart_tagged_acescg.json | orange | 0.498596 0.215162 0.026614 | 0.498596 0.215162 0.026614 | 3.0e-8 | pass | [187, 128, 45] / [187, 128, 45] pass |
| chart_tagged_acescg.json | purplish blue | 0.134697 0.125136 0.377885 | 0.134697 0.125136 0.377885 | 1.5e-8 | pass | [103, 99, 165] / [103, 99, 165] pass |
| chart_tagged_acescg.json | moderate red | 0.404477 0.123384 0.123666 | 0.404477 0.123384 0.123666 | 3.0e-8 | pass | [170, 98, 99] / [170, 98, 99] pass |
| chart_tagged_acescg.json | purple | 0.109181 0.056373 0.148512 | 0.109181 0.056373 0.148512 | 7.4e-9 | pass | [93, 67, 108] / [93, 67, 108] pass |
| chart_tagged_acescg.json | yellow green | 0.311493 0.460154 0.057277 | 0.311493 0.460154 0.057277 | 3.8e-9 | pass | [151, 181, 68] / [151, 181, 68] pass |
| chart_tagged_acescg.json | orange yellow | 0.571211 0.356825 0.030869 | 0.571211 0.356825 0.030869 | 1.9e-9 | pass | [199, 161, 49] / [199, 161, 49] pass |
| chart_tagged_acescg.json | blue | 0.085586 0.066525 0.301746 | 0.085586 0.066525 0.301745 | 3.0e-8 | pass | [83, 73, 149] / [83, 73, 149] pass |
| chart_tagged_acescg.json | green | 0.094560 0.267646 0.069966 | 0.094560 0.267646 0.069966 | 3.0e-8 | pass | [87, 141, 75] / [87, 141, 75] pass |
| chart_tagged_acescg.json | red | 0.309547 0.055156 0.044335 | 0.309547 0.055156 0.044335 | 7.4e-9 | pass | [151, 66, 59] / [151, 66, 59] pass |
| chart_tagged_acescg.json | yellow | 0.634336 0.537830 0.020292 | 0.634336 0.537830 0.020292 | 5.9e-8 | pass | [209, 194, 39] / [209, 194, 39] pass |
| chart_tagged_acescg.json | magenta | 0.407887 0.127429 0.297159 | 0.407887 0.127429 0.297159 | 2.9e-8 | pass | [171, 100, 148] / [171, 100, 148] pass |
| chart_tagged_acescg.json | cyan | 0.094533 0.233632 0.355307 | 0.094533 0.233632 0.355307 | 3.0e-8 | pass | [87, 133, 161] / [87, 133, 161] pass |
| chart_tagged_acescg.json | white 9.5 | 0.894853 0.895617 0.888028 | 0.894853 0.895617 0.888028 | 6.0e-8 | pass | [243, 243, 242] / [243, 243, 242] pass |
| chart_tagged_acescg.json | neutral 8 | 0.577580 0.577580 0.577580 | 0.577580 0.577580 0.577580 | 3.5e-11 | pass | [200, 200, 200] / [200, 200, 200] pass |
| chart_tagged_acescg.json | neutral 6.5 | 0.351533 0.351533 0.351533 | 0.351533 0.351533 0.351533 | 3.0e-8 | pass | [160, 160, 160] / [160, 160, 160] pass |
| chart_tagged_acescg.json | neutral 5 | 0.194038 0.194351 0.191245 | 0.194038 0.194351 0.191245 | 1.5e-8 | pass | [122, 122, 121] / [122, 122, 121] pass |
| chart_tagged_acescg.json | neutral 3.5 | 0.090842 0.090842 0.090842 | 0.090842 0.090842 0.090842 | 7.4e-9 | pass | [85, 85, 85] / [85, 85, 85] pass |
| chart_tagged_acescg.json | black 2 | 0.034340 0.034340 0.034340 | 0.034340 0.034340 0.034340 | 3.7e-9 | pass | [52, 52, 52] / [52, 52, 52] pass |
| chart_tagged_acescg.json | ACEScg red 0.5 | 0.579194 -0.019713 -0.014421 | 0.579194 -0.019713 -0.014421 | 3.7e-9 | pass | [200, 0, 0] / [200, 0, 0] pass |
| chart_tagged_acescg.json | ACEScg green 0.5 | -0.147504 0.481175 -0.054803 | -0.147504 0.481175 -0.054803 | 3.7e-9 | pass | [0, 184, 0] / [0, 184, 0] pass |
| chart_tagged_acescg.json | ACEScg blue 0.5 | 0.068309 0.038538 0.569224 | 0.068309 0.038538 0.569224 | 7.5e-9 | pass | [74, 55, 199] / [74, 55, 199] pass |
| chart_tagged_acescg.json | grey 2 | 2.000000 2.000000 2.000000 | 2.000000 2.000000 2.000000 | 0.0e0 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_acescg.json | grey 8 | 8.000000 8.000000 8.000000 | 8.000000 8.000000 8.000000 | 0.0e0 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_acescg.json | warm bright | 4.372701 0.823911 0.059639 | 4.372702 0.823911 0.059639 | 1.1e-7 | pass | [255, 234, 69] / [255, 234, 69] pass |
| chart_tagged_linear_rec709.json | dark skin | 0.117940 0.089621 0.062763 | 0.117940 0.089621 0.062763 | 3.1e-11 | pass | [96, 84, 71] / [96, 84, 71] pass |
| chart_tagged_linear_rec709.json | light skin | 0.393575 0.318126 0.237895 | 0.393575 0.318126 0.237895 | 4.8e-10 | pass | [168, 153, 134] / [168, 153, 134] pass |
| chart_tagged_linear_rec709.json | blue sky | 0.188038 0.201907 0.317491 | 0.188038 0.201907 0.317491 | 3.9e-10 | pass | [120, 124, 153] / [120, 124, 153] pass |
| chart_tagged_linear_rec709.json | foliage | 0.109289 0.135972 0.067282 | 0.109289 0.135972 0.067282 | 3.4e-10 | pass | [93, 103, 73] / [93, 103, 73] pass |
| chart_tagged_linear_rec709.json | blue flower | 0.263774 0.239198 0.411096 | 0.263774 0.239198 0.411096 | 3.3e-10 | pass | [140, 134, 172] / [140, 134, 172] pass |
| chart_tagged_linear_rec709.json | bluish green | 0.325827 0.465026 0.409230 | 0.325827 0.465026 0.409231 | 4.6e-10 | pass | [155, 182, 171] / [155, 182, 171] pass |
| chart_tagged_linear_rec709.json | orange | 0.380001 0.232521 0.057004 | 0.380001 0.232521 0.057004 | 4.0e-10 | pass | [166, 132, 68] / [166, 132, 68] pass |
| chart_tagged_linear_rec709.json | purplish blue | 0.142973 0.129207 0.345178 | 0.142973 0.129207 0.345178 | 5.0e-10 | pass | [106, 101, 159] / [106, 101, 159] pass |
| chart_tagged_linear_rec709.json | moderate red | 0.295735 0.143119 0.129424 | 0.295735 0.143119 0.129424 | 2.3e-10 | pass | [148, 106, 101] / [148, 106, 101] pass |
| chart_tagged_linear_rec709.json | purple | 0.093115 0.061319 0.137606 | 0.093115 0.061319 0.137606 | 4.9e-10 | pass | [86, 70, 104] / [86, 70, 104] pass |
| chart_tagged_linear_rec709.json | yellow green | 0.349922 0.444299 0.106661 | 0.349922 0.444299 0.106661 | 1.8e-10 | pass | [160, 178, 92] / [160, 178, 92] pass |
| chart_tagged_linear_rec709.json | orange yellow | 0.472821 0.367488 0.077724 | 0.472821 0.367488 0.077724 | 4.4e-10 | pass | [183, 163, 79] / [183, 163, 79] pass |
| chart_tagged_linear_rec709.json | blue | 0.089356 0.071027 0.271516 | 0.089356 0.071027 0.271516 | 1.2e-10 | pass | [84, 75, 142] / [84, 75, 142] pass |
| chart_tagged_linear_rec709.json | green | 0.152161 0.252837 0.092133 | 0.152161 0.252837 0.092133 | 3.1e-10 | pass | [109, 138, 86] / [109, 138, 86] pass |
| chart_tagged_linear_rec709.json | red | 0.210610 0.072867 0.050988 | 0.210610 0.072867 0.050988 | 1.4e-10 | pass | [127, 76, 64] / [127, 76, 64] pass |
| chart_tagged_linear_rec709.json | yellow | 0.572477 0.537642 0.089658 | 0.572477 0.537642 0.089658 | 3.5e-10 | pass | [199, 194, 84] / [199, 194, 84] pass |
| chart_tagged_linear_rec709.json | magenta | 0.307419 0.149399 0.280844 | 0.307419 0.149399 0.280844 | 3.7e-10 | pass | [151, 108, 144] / [151, 108, 144] pass |
| chart_tagged_linear_rec709.json | cyan | 0.154115 0.225505 0.336599 | 0.154115 0.225505 0.336599 | 3.6e-10 | pass | [109, 131, 157] / [109, 131, 157] pass |
| chart_tagged_linear_rec709.json | white 9.5 | 0.894789 0.895462 0.889001 | 0.894789 0.895462 0.889001 | 3.0e-10 | pass | [243, 243, 242] / [243, 243, 242] pass |
| chart_tagged_linear_rec709.json | neutral 8 | 0.577580 0.577580 0.577580 | 0.577580 0.577580 0.577580 | 3.5e-11 | pass | [200, 200, 200] / [200, 200, 200] pass |
| chart_tagged_linear_rec709.json | neutral 6.5 | 0.351533 0.351533 0.351533 | 0.351533 0.351533 0.351533 | 4.7e-10 | pass | [160, 160, 160] / [160, 160, 160] pass |
| chart_tagged_linear_rec709.json | neutral 5 | 0.194012 0.194287 0.191643 | 0.194012 0.194287 0.191643 | 3.0e-10 | pass | [122, 122, 121] / [122, 122, 121] pass |
| chart_tagged_linear_rec709.json | neutral 3.5 | 0.090842 0.090842 0.090842 | 0.090842 0.090842 0.090842 | 3.3e-11 | pass | [85, 85, 85] / [85, 85, 85] pass |
| chart_tagged_linear_rec709.json | black 2 | 0.034340 0.034340 0.034340 | 0.034340 0.034340 0.034340 | 2.8e-11 | pass | [52, 52, 52] / [52, 52, 52] pass |
| chart_tagged_linear_rec709.json | ACEScg red 0.5 | 0.347726 0.022397 -0.002763 | 0.347726 0.022397 -0.002763 | 3.6e-10 | pass | [159, 41, 0] / [159, 41, 0] pass |
| chart_tagged_linear_rec709.json | ACEScg green 0.5 | 0.070339 0.429836 0.002013 | 0.070339 0.429836 0.002013 | 6.2e-11 | pass | [75, 175, 7] / [75, 175, 7] pass |
| chart_tagged_linear_rec709.json | ACEScg blue 0.5 | 0.081935 0.047767 0.500750 | 0.081935 0.047767 0.500750 | 1.3e-10 | pass | [81, 62, 188] / [81, 62, 188] pass |
| chart_tagged_linear_rec709.json | grey 2 | 2.000000 2.000000 2.000000 | 2.000000 2.000000 2.000000 | 0.0e0 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_linear_rec709.json | grey 8 | 8.000000 8.000000 8.000000 | 8.000000 8.000000 8.000000 | 0.0e0 | pass | [255, 255, 255] / [255, 255, 255] pass |
| chart_tagged_linear_rec709.json | warm bright | 2.963455 1.062733 0.232297 | 2.963455 1.062733 0.232297 | 1.7e-9 | pass | [255, 255, 132] / [255, 255, 132] pass |
| chart_managed_acescg.json | dark skin | 0.136497 0.090130 0.063059 | 0.136497 0.090130 0.063059 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | light skin | 0.444880 0.320347 0.238706 | 0.444880 0.320347 0.238706 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | blue sky | 0.156935 0.191448 0.317112 | 0.156935 0.191448 0.317112 | 1.4e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | foliage | 0.112007 0.144861 0.067217 | 0.112007 0.144861 0.067217 | 8.2e-11 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | blue flower | 0.237923 0.220183 0.410907 | 0.237923 0.220183 0.410907 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | bluish green | 0.274979 0.481244 0.408200 | 0.274979 0.481244 0.408200 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | orange | 0.484304 0.238725 0.058631 | 0.484304 0.238725 0.058631 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | purplish blue | 0.102769 0.106626 0.344800 | 0.102769 0.106626 0.344800 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | moderate red | 0.367575 0.132801 0.130725 | 0.367575 0.132801 0.130725 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | purple | 0.091072 0.051281 0.137696 | 0.091072 0.051281 0.137696 | 7.5e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | yellow green | 0.379885 0.485178 0.106647 | 0.379885 0.485178 0.106647 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | orange yellow | 0.582651 0.388307 0.079261 | 0.582651 0.388307 0.079261 | 1.2e-7 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | blue | 0.054540 0.049640 0.271211 | 0.054540 0.049640 0.271211 | 3.8e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | green | 0.141252 0.276563 0.091663 | 0.141252 0.276563 0.091663 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | red | 0.277495 0.064503 0.052183 | 0.277495 0.064503 0.052183 | 3.7e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | yellow | 0.684487 0.579629 0.090970 | 0.684487 0.579629 0.090970 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | magenta | 0.350504 0.124200 0.281858 | 0.350504 0.124200 0.281858 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | cyan | 0.098010 0.219896 0.335752 | 0.098010 0.219896 0.335752 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | white 9.5 | 0.895874 0.896157 0.889010 | 0.895874 0.896157 0.889010 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | neutral 8 | 0.577580 0.577580 0.577580 | 0.577580 0.577580 0.577580 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | neutral 6.5 | 0.351533 0.351533 0.351533 | 0.351533 0.351533 0.351533 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | neutral 5 | 0.194456 0.194572 0.191646 | 0.194456 0.194572 0.191646 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | neutral 3.5 | 0.090842 0.090842 0.090842 | 0.090842 0.090842 0.090842 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | black 2 | 0.034340 0.034340 0.034340 | 0.034340 0.034340 0.034340 | 2.8e-11 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | ACEScg red 0.5 | 0.500000 -0.000000 -0.000000 | 0.500000 0.000000 -0.000000 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | ACEScg green 0.5 | -0.000000 0.500000 0.000000 | 0.000000 0.500000 -0.000000 | 6.2e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | ACEScg blue 0.5 | -0.000000 0.000000 0.500000 | -0.000000 -0.000000 0.500000 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | grey 2 | 2.000000 2.000000 2.000000 | 2.000000 2.000000 2.000000 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | grey 8 | 8.000000 8.000000 8.000000 | 8.000000 8.000000 8.000000 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg.json | warm bright | 4.000000 1.000000 0.250000 | 4.000000 1.000000 0.250000 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | dark skin | 0.105711 0.091247 0.066843 | 0.105711 0.091247 0.066843 | 7.6e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | light skin | 0.360583 0.322343 0.249896 | 0.360583 0.322343 0.249896 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | blue sky | 0.198880 0.202488 0.302158 | 0.198880 0.202488 0.302158 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | foliage | 0.116358 0.133175 0.075675 | 0.116358 0.133175 0.075675 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | blue flower | 0.262410 0.243236 0.389224 | 0.262410 0.243236 0.389224 | 4.1e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | bluish green | 0.377040 0.454504 0.413625 | 0.377040 0.454504 0.413625 | 3.5e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | orange | 0.314625 0.240512 0.082894 | 0.314625 0.240512 0.082894 | 1.8e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | purplish blue | 0.147880 0.133079 0.317346 | 0.147880 0.133079 0.317346 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | moderate red | 0.236039 0.153647 0.134353 | 0.236039 0.153647 0.134353 | 3.5e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | purple | 0.084427 0.064577 0.128330 | 0.084427 0.064577 0.128330 | 7.5e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | yellow green | 0.370440 0.433132 0.148671 | 0.370440 0.433132 0.148671 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | orange yellow | 0.418339 0.370984 0.117618 | 0.418339 0.370984 0.117618 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | blue | 0.091764 0.075011 0.245793 | 0.091764 0.075011 0.245793 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | green | 0.183499 0.243609 0.110979 | 0.183499 0.243609 0.110979 | 4.5e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | red | 0.156280 0.082241 0.056676 | 0.156280 0.082241 0.056676 | 7.5e-9 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | yellow | 0.537774 0.534061 0.148697 | 0.537774 0.534061 0.148697 | 5.9e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | magenta | 0.252508 0.162259 0.266990 | 0.252508 0.162259 0.266990 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | cyan | 0.187000 0.221988 0.320665 | 0.187000 0.221988 0.320665 | 4.7e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | white 9.5 | 0.894743 0.895327 0.889828 | 0.894743 0.895327 0.889828 | 6.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | neutral 8 | 0.577580 0.577580 0.577580 | 0.577580 0.577580 0.577580 | 3.5e-11 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | neutral 6.5 | 0.351533 0.351533 0.351533 | 0.351533 0.351533 0.351533 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | neutral 5 | 0.193993 0.194232 0.191981 | 0.193993 0.194232 0.191981 | 3.9e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | neutral 3.5 | 0.090842 0.090842 0.090842 | 0.090842 0.090842 0.090842 | 3.3e-11 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | black 2 | 0.034340 0.034340 0.034340 | 0.034340 0.034340 0.034340 | 2.8e-11 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | ACEScg red 0.5 | 0.220663 0.044895 0.007219 | 0.220663 0.044895 0.007219 | 1.5e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | ACEScg green 0.5 | 0.189159 0.398846 0.050298 | 0.189159 0.398846 0.050298 | 3.0e-8 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | ACEScg blue 0.5 | 0.090177 0.056259 0.442483 | 0.090177 0.056259 0.442483 | 3.0e-10 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | grey 2 | 2.000000 2.000000 2.000000 | 2.000000 2.000000 2.000000 | 0.0e0 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | grey 8 | 8.000000 8.000000 8.000000 | 8.000000 8.000000 8.000000 | 0.0e0 | pass | (ACEScg numbers; no view until C5) |
| chart_managed_acescg_linear_rec709.json | warm bright | 2.188715 1.184980 0.379592 | 2.188715 1.184980 0.379592 | 5.3e-8 | pass | (ACEScg numbers; no view until C5) |

## The 8-bit ramp, every word

256 pixels, every byte as red, alphas 0 to 255. The judge is the column named; the other column's worst difference is shown beside it (the sRGB and ACEScc rows, where OpenColorIO does not compute the published formula, D-906). "Times alpha" is the straight-alpha drawing equal to the colour times alpha in single precision, every sample; "bytes path" is D-897's drawing kept at 8 bits and widened again.

| Word | Working space | Judge | Worst vs judge | Worst vs the other | Times alpha | Bytes path | Result |
|---|---|---|---|---|---|---|---|
| acescg | linear-srgb | ocio | 1.2e-7 | 5.4e-8 | true | same bits, from memory | pass |
| aces2065-1 | linear-srgb | ocio | 2.2e-7 | 5.8e-8 | true | same bits, from memory | pass |
| acescct | linear-srgb | ocio | 5.3e-7 | 1.0e-7 | true | same bits, from memory | pass |
| acescc | linear-srgb | formula | 1.0e-7 | 3.8e-3 | true | same bits, from memory | pass |
| linear-rec2020 | linear-srgb | ocio | 1.2e-7 | 5.5e-8 | true | same bits, from memory | pass |
| linear-p3-d65 | linear-srgb | ocio | 1.2e-7 | 5.7e-8 | true | same bits, from memory | pass |
| bt1886-rec709 | linear-srgb | ocio | 6.0e-8 | 2.9e-8 | true | same bits, from memory | pass |
| gamma22-rec709 | linear-srgb | ocio | 6.0e-8 | 2.9e-8 | true | same bits, from memory | pass |
| srgb | acescg | formula | 1.7e-7 | 7.0e-7 | true | same bits, from memory | pass |
| linear-srgb | acescg | ocio | 1.2e-7 | 3.0e-8 | true | same bits, from memory | pass |
| acescg | acescg | ocio | 5.0e-10 | 5.0e-13 | true | same bits, from memory | pass |
| aces2065-1 | acescg | ocio | 2.0e-7 | 5.7e-8 | true | same bits, from memory | pass |
| acescct | acescg | ocio | 4.7e-7 | 5.8e-8 | true | same bits, from memory | pass |
| acescc | acescg | formula | 5.8e-8 | 5.5e-3 | true | same bits, from memory | pass |
| linear-rec2020 | acescg | ocio | 1.2e-7 | 3.0e-8 | true | same bits, from memory | pass |
| linear-p3-d65 | acescg | ocio | 1.2e-7 | 3.0e-8 | true | same bits, from memory | pass |
| bt1886-rec709 | acescg | ocio | 1.2e-7 | 5.1e-8 | true | same bits, from memory | pass |
| gamma22-rec709 | acescg | ocio | 1.2e-7 | 5.4e-8 | true | same bits, from memory | pass |
