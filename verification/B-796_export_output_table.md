# B-796: the export's Output, checked against OpenColorIO

Written by `tests/b796_export_output.rs` (D-917). The expected values are OpenColorIO 2.5.1's (built-in config `ocio://studio-config-v4.0.0_aces-v2.0_ocio-v2.5`, lossless), written by `tools/output_reference.py` into `Fixtures/colour/output/expected_output.json` before the code. Every check reads the exported file back. The bar: sRGB and Rec.709 within 1 level on every channel of every pixel, at 8 and 16 bits; the ACES 2.0 SDR view within 1 level of 255 (257 of 65535) on 99.9% of pixels; EXR numbers within 1e-6 of the colour's size, alpha exact. With colour management off every existing export suite is unchanged (run beside this test).

**75 of 75 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| FX-COLOUT-001: acescg, sRGB, eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, sRGB, sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, sRGB, eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, Rec.709 (BT.1886), eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, Rec.709 (BT.1886), sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, Rec.709 (BT.1886), eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, ACES 2.0 SDR view, eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, ACES 2.0 SDR view, sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: acescg, ACES 2.0 SDR view, eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, sRGB, eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, sRGB, sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, sRGB, eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, Rec.709 (BT.1886), eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, Rec.709 (BT.1886), sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, Rec.709 (BT.1886), eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, ACES 2.0 SDR view, eight_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, ACES 2.0 SDR view, sixteen_straight, within the bar | true | true | pass |
| FX-COLOUT-001: linear-srgb, ACES 2.0 SDR view, eight_premultiplied, within the bar | true | true | pass |
| FX-COLOUT-002: acescg, EXR default, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: acescg, EXR default, chromaticities | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | pass |
| FX-COLOUT-002: acescg, EXR acescg, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: acescg, EXR acescg, chromaticities | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | pass |
| FX-COLOUT-002: acescg, EXR aces2065-1, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: acescg, EXR aces2065-1, chromaticities | [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.077], [0.32168, 0.33767]] | [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.077], [0.32168, 0.33767]] | pass |
| FX-COLOUT-002: linear-srgb, EXR default, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: linear-srgb, EXR default, chromaticities | [[0.64, 0.33], [0.3, 0.6], [0.15, 0.06], [0.3127, 0.329]] | [[0.64, 0.33], [0.3, 0.6], [0.15, 0.06], [0.3127, 0.329]] | pass |
| FX-COLOUT-002: linear-srgb, EXR acescg, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: linear-srgb, EXR acescg, chromaticities | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | [[0.713, 0.293], [0.165, 0.83], [0.128, 0.044], [0.32168, 0.33767]] | pass |
| FX-COLOUT-002: linear-srgb, EXR aces2065-1, numbers within 1e-6, alpha exact | true | true | pass |
| FX-COLOUT-002: linear-srgb, EXR aces2065-1, chromaticities | [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.077], [0.32168, 0.33767]] | [[0.7347, 0.2653], [0.0, 1.0], [0.0001, -0.077], [0.32168, 0.33767]] | pass |
| FX-COLOUT-003: acescg, default: the PNG's colour chunk | sRGB 0 | sRGB 0 | pass |
| FX-COLOUT-003: acescg, default: the PNG's ColorSpace tag | sRGB IEC 61966-2-1, 8 bits per channel | sRGB IEC 61966-2-1, 8 bits per channel | pass |
| FX-COLOUT-003: acescg, default: the animated PNG's chunk, and its first frame is the PNG's | sRGB 0, true | sRGB 0, true | pass |
| FX-COLOUT-003: acescg, default: the MP4's colr (primaries, transfer, matrix, full range) | [1, 13, 1, 0] | [1, 13, 1, 0] | pass |
| FX-COLOUT-003: acescg, default: a GIF is written | true | true | pass |
| FX-COLOUT-003: acescg, rec709: the PNG's colour chunk | cICP 1 1 0 1 | cICP 1 1 0 1 | pass |
| FX-COLOUT-003: acescg, rec709: the PNG's ColorSpace tag | Rec.709, BT.1886 (gamma 2.4), 8 bits per channel | Rec.709, BT.1886 (gamma 2.4), 8 bits per channel | pass |
| FX-COLOUT-003: acescg, rec709: the animated PNG's chunk, and its first frame is the PNG's | cICP 1 1 0 1, true | cICP 1 1 0 1, true | pass |
| FX-COLOUT-003: acescg, rec709: the MP4's colr (primaries, transfer, matrix, full range) | [1, 1, 1, 0] | [1, 1, 1, 0] | pass |
| FX-COLOUT-003: acescg, rec709: a GIF is written | true | true | pass |
| FX-COLOUT-003: acescg, aces-view: the PNG's colour chunk | sRGB 0 | sRGB 0 | pass |
| FX-COLOUT-003: acescg, aces-view: the PNG's ColorSpace tag | sRGB IEC 61966-2-1, the ACES 2.0 SDR view, 8 bits per channel | sRGB IEC 61966-2-1, the ACES 2.0 SDR view, 8 bits per channel | pass |
| FX-COLOUT-003: acescg, aces-view: the animated PNG's chunk, and its first frame is the PNG's | sRGB 0, true | sRGB 0, true | pass |
| FX-COLOUT-003: acescg, aces-view: the MP4's colr (primaries, transfer, matrix, full range) | [1, 13, 1, 0] | [1, 13, 1, 0] | pass |
| FX-COLOUT-003: acescg, aces-view: a GIF is written | true | true | pass |
| FX-COLOUT-003: linear-srgb, default: the PNG's colour chunk | sRGB 0 | sRGB 0 | pass |
| FX-COLOUT-003: linear-srgb, default: the PNG's ColorSpace tag | sRGB IEC 61966-2-1, 8 bits per channel | sRGB IEC 61966-2-1, 8 bits per channel | pass |
| FX-COLOUT-003: linear-srgb, default: the animated PNG's chunk, and its first frame is the PNG's | sRGB 0, true | sRGB 0, true | pass |
| FX-COLOUT-003: linear-srgb, default: the MP4's colr (primaries, transfer, matrix, full range) | [1, 13, 1, 0] | [1, 13, 1, 0] | pass |
| FX-COLOUT-003: linear-srgb, default: a GIF is written | true | true | pass |
| FX-COLOUT-003: linear-srgb, rec709: the PNG's colour chunk | cICP 1 1 0 1 | cICP 1 1 0 1 | pass |
| FX-COLOUT-003: linear-srgb, rec709: the PNG's ColorSpace tag | Rec.709, BT.1886 (gamma 2.4), 8 bits per channel | Rec.709, BT.1886 (gamma 2.4), 8 bits per channel | pass |
| FX-COLOUT-003: linear-srgb, rec709: the animated PNG's chunk, and its first frame is the PNG's | cICP 1 1 0 1, true | cICP 1 1 0 1, true | pass |
| FX-COLOUT-003: linear-srgb, rec709: the MP4's colr (primaries, transfer, matrix, full range) | [1, 1, 1, 0] | [1, 1, 1, 0] | pass |
| FX-COLOUT-003: linear-srgb, rec709: a GIF is written | true | true | pass |
| FX-COLOUT-003: linear-srgb, aces-view: the PNG's colour chunk | sRGB 0 | sRGB 0 | pass |
| FX-COLOUT-003: linear-srgb, aces-view: the PNG's ColorSpace tag | sRGB IEC 61966-2-1, the ACES 2.0 SDR view, 8 bits per channel | sRGB IEC 61966-2-1, the ACES 2.0 SDR view, 8 bits per channel | pass |
| FX-COLOUT-003: linear-srgb, aces-view: the animated PNG's chunk, and its first frame is the PNG's | sRGB 0, true | sRGB 0, true | pass |
| FX-COLOUT-003: linear-srgb, aces-view: the MP4's colr (primaries, transfer, matrix, full range) | [1, 13, 1, 0] | [1, 13, 1, 0] | pass |
| FX-COLOUT-003: linear-srgb, aces-view: a GIF is written | true | true | pass |
| FX-COLOUT-004: acescg for a PNG is refused, naming it, nothing written | false, COMMAND_INVALID_VALUE, true, 0 | false, COMMAND_INVALID_VALUE, true, 0 | pass |
| FX-COLOUT-004: rec709 for an EXR is refused, naming it, nothing written | false, COMMAND_INVALID_VALUE, true, 0 | false, COMMAND_INVALID_VALUE, true, 0 | pass |
| FX-COLOUT-004: rec709 with colour management off is refused, nothing written | false, COMMAND_INVALID_VALUE, true, 0 | false, COMMAND_INVALID_VALUE, true, 0 | pass |
| FX-COLOUT-004: aces-view with colour management off is refused, nothing written | false, COMMAND_INVALID_VALUE, true, 0 | false, COMMAND_INVALID_VALUE, true, 0 | pass |
| FX-COLOUT-004: acescg for an EXR with colour management off is refused | false, COMMAND_INVALID_VALUE, true, 0 | false, COMMAND_INVALID_VALUE, true, 0 | pass |
| FX-COLOUT-005: off, a PNG carries no colour chunk | none | none | pass |
| FX-COLOUT-005: off, the PNG's ColorSpace tag is today's | sRGB IEC 61966-2-1, 8 bits per channel | sRGB IEC 61966-2-1, 8 bits per channel | pass |
| FX-COLOUT-005: off, the PNG's pixels are those of colour management on in linear Rec.709 at sRGB (today's pixels) | true | true | pass |
| FX-COLOUT-005: off, an animated PNG carries no colour chunk | none | none | pass |
| FX-COLOUT-005: off, the MP4's colr is today's | [1, 1, 1, 0] | [1, 1, 1, 0] | pass |
| FX-COLOUT-005: off, the EXR's chromaticities are Rec.709's | [[0.64, 0.33], [0.3, 0.6], [0.15, 0.06], [0.3127, 0.329]] | [[0.64, 0.33], [0.3, 0.6], [0.15, 0.06], [0.3127, 0.329]] | pass |
| FX-COLOUT-006: the card with default: written, nothing refused, within 1 level of the processor | true, true, true | true, true, true | pass |
| FX-COLOUT-006: the card with rec709: written, nothing refused, within 1 level of the processor | true, true, true | true, true, true | pass |
| FX-COLOUT-006: the card with aces-view: written, nothing refused, within 1 level of the processor | true, true, true | true, true, true | pass |
| the pictures are written | true | true | pass |

## Each Output against OpenColorIO, case by case

`media/output_chart.exr` (264 by 4: a white ramp to 64, fire to 64, the ColorChecker from 4 stops under to 6 over, an orange with alpha 0 to 1) exported from each working space. "Largest" is the largest difference on any channel of any pixel.

| Working space | Output | Samples | Pixels | Largest | Within | Result |
|---|---|---|---|---|---|---|
| acescg | sRGB | eight_straight | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| acescg | sRGB | sixteen_straight | 1056 | 1 of 65535 | 1056 of 1056 within 257 | pass |
| acescg | sRGB | eight_premultiplied | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| acescg | Rec.709 (BT.1886) | eight_straight | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| acescg | Rec.709 (BT.1886) | sixteen_straight | 1056 | 0 of 65535 | 1056 of 1056 within 257 | pass |
| acescg | Rec.709 (BT.1886) | eight_premultiplied | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| acescg | ACES 2.0 SDR view | eight_straight | 1056 | 1 of 255 | 1056 of 1056 within 1 | pass |
| acescg | ACES 2.0 SDR view | sixteen_straight | 1056 | 251 of 65535 | 1056 of 1056 within 257 | pass |
| acescg | ACES 2.0 SDR view | eight_premultiplied | 1056 | 1 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | sRGB | eight_straight | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | sRGB | sixteen_straight | 1056 | 1 of 65535 | 1056 of 1056 within 257 | pass |
| linear-srgb | sRGB | eight_premultiplied | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | Rec.709 (BT.1886) | eight_straight | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | Rec.709 (BT.1886) | sixteen_straight | 1056 | 1 of 65535 | 1056 of 1056 within 257 | pass |
| linear-srgb | Rec.709 (BT.1886) | eight_premultiplied | 1056 | 0 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | ACES 2.0 SDR view | eight_straight | 1056 | 1 of 255 | 1056 of 1056 within 1 | pass |
| linear-srgb | ACES 2.0 SDR view | sixteen_straight | 1056 | 251 of 65535 | 1056 of 1056 within 257 | pass |
| linear-srgb | ACES 2.0 SDR view | eight_premultiplied | 1056 | 1 of 255 | 1056 of 1056 within 1 | pass |
| acescg | EXR Working space | float | 1056 | 1.27e-7 of the colour's size | alpha exact: true | pass |
| acescg | EXR ACEScg | float | 1056 | 1.27e-7 of the colour's size | alpha exact: true | pass |
| acescg | EXR ACES2065-1 | float | 1056 | 1.27e-7 of the colour's size | alpha exact: true | pass |
| linear-srgb | EXR Working space | float | 1056 | 4.74e-9 of the colour's size | alpha exact: true | pass |
| linear-srgb | EXR ACEScg | float | 1056 | 1.27e-7 of the colour's size | alpha exact: true | pass |
| linear-srgb | EXR ACES2065-1 | float | 1056 | 1.14e-7 of the colour's size | alpha exact: true | pass |

## The card against the processor

Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB of its own memory. `chart_rec709.json`, 8 bits, straight alpha, exported with the tick "Faster on the graphics card" and each picture Output, against the same export on the processor.

| Output | What the export said | Largest difference (levels) | Result |
|---|---|---|---|
| sRGB | the card drew 1 of 1 frames | 1 | pass |
| Rec.709 (BT.1886) | the card drew 1 of 1 frames | 1 | pass |
| ACES 2.0 SDR view | the card drew 1 of 1 frames | 1 | pass |
