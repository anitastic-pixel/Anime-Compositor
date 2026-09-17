# B-16b: the exported EXR files, checked by OpenEXR

The four files in `verification/B-16b export/` were written by `tests/b16b_exr.rs` (FX-EXR-009 and 010) and checked on 2026-09-17 with OpenEXR 3.4.15's own library, which shares nothing with the build:

```
python tools/exr_reference.py check "verification/B-16b export"
```

It exited 0. Its output, unchanged:

| Case | Check | Expected | Actual | Result |
|---|---|---|---|---|
| none_float_as_float | parts | 1 | 1 | pass |
| none_float_as_float | attributes, and no others | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | pass |
| none_float_as_float | type | scanlineimage | scanlineimage | pass |
| none_float_as_float | compression | ZIP_COMPRESSION | ZIP_COMPRESSION | pass |
| none_float_as_float | channels, as stored | A, B, G, R | A, B, G, R | pass |
| none_float_as_float | sample type | {'float32'} | {'float32'} | pass |
| none_float_as_float | data window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| none_float_as_float | display window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| none_float_as_float | line order | INCREASING_Y | INCREASING_Y | pass |
| none_float_as_float | pixel aspect ratio | 1.0 | 1.0 | pass |
| none_float_as_float | screen window centre | [0.0, 0.0] | [0.0, 0.0] | pass |
| none_float_as_float | screen window width | 1.0 | 1.0 | pass |
| none_float_as_float | chromaticities | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | pass |
| none_float_as_float | frames per second, as a fraction | [24, 1] | [24, 1] | pass |
| none_float_as_float | every sample, exactly | 192 samples | 192 samples | pass |
| none_float_as_half | parts | 1 | 1 | pass |
| none_float_as_half | attributes, and no others | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | pass |
| none_float_as_half | type | scanlineimage | scanlineimage | pass |
| none_float_as_half | compression | ZIP_COMPRESSION | ZIP_COMPRESSION | pass |
| none_float_as_half | channels, as stored | A, B, G, R | A, B, G, R | pass |
| none_float_as_half | sample type | {'float16'} | {'float16'} | pass |
| none_float_as_half | data window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| none_float_as_half | display window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| none_float_as_half | line order | INCREASING_Y | INCREASING_Y | pass |
| none_float_as_half | pixel aspect ratio | 1.0 | 1.0 | pass |
| none_float_as_half | screen window centre | [0.0, 0.0] | [0.0, 0.0] | pass |
| none_float_as_half | screen window width | 1.0 | 1.0 | pass |
| none_float_as_half | chromaticities | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | pass |
| none_float_as_half | frames per second, as a fraction | [24, 1] | [24, 1] | pass |
| none_float_as_half | every sample, exactly | 192 samples | 192 samples | pass |
| specials_as_float | parts | 1 | 1 | pass |
| specials_as_float | attributes, and no others | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | pass |
| specials_as_float | type | scanlineimage | scanlineimage | pass |
| specials_as_float | compression | ZIP_COMPRESSION | ZIP_COMPRESSION | pass |
| specials_as_float | channels, as stored | A, B, G, R | A, B, G, R | pass |
| specials_as_float | sample type | {'float32'} | {'float32'} | pass |
| specials_as_float | data window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| specials_as_float | display window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| specials_as_float | line order | INCREASING_Y | INCREASING_Y | pass |
| specials_as_float | pixel aspect ratio | 1.0 | 1.0 | pass |
| specials_as_float | screen window centre | [0.0, 0.0] | [0.0, 0.0] | pass |
| specials_as_float | screen window width | 1.0 | 1.0 | pass |
| specials_as_float | chromaticities | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | pass |
| specials_as_float | frames per second, as a fraction | [24000, 1001] | [24000, 1001] | pass |
| specials_as_float | every sample, exactly | 192 samples | 192 samples | pass |
| specials_as_half | parts | 1 | 1 | pass |
| specials_as_half | attributes, and no others | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | ['channels', 'chromaticities', 'compression', 'dataWindow', 'displayWindow', 'framesPerSecond', 'lineOrder', 'pixelAspectRatio', 'screenWindowCenter', 'screenWindowWidth', 'type'] | pass |
| specials_as_half | type | scanlineimage | scanlineimage | pass |
| specials_as_half | compression | ZIP_COMPRESSION | ZIP_COMPRESSION | pass |
| specials_as_half | channels, as stored | A, B, G, R | A, B, G, R | pass |
| specials_as_half | sample type | {'float16'} | {'float16'} | pass |
| specials_as_half | data window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| specials_as_half | display window | [0, 0, 7, 5] | [0, 0, 7, 5] | pass |
| specials_as_half | line order | INCREASING_Y | INCREASING_Y | pass |
| specials_as_half | pixel aspect ratio | 1.0 | 1.0 | pass |
| specials_as_half | screen window centre | [0.0, 0.0] | [0.0, 0.0] | pass |
| specials_as_half | screen window width | 1.0 | 1.0 | pass |
| specials_as_half | chromaticities | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | [0.6399999856948853, 0.33000001311302185, 0.30000001192092896, 0.6000000238418579, 0.15000000596046448, 0.05999999865889549, 0.3127000033855438, 0.32899999618530273] | pass |
| specials_as_half | frames per second, as a fraction | [24000, 1001] | [24000, 1001] | pass |
| specials_as_half | every sample, exactly | 192 samples | 192 samples | pass |

**60 of 60 checks pass.**
