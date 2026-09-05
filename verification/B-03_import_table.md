# B-03 import fixture table

Test T-01, requirement R-01. Produced by `tests/b03_import.rs`. **46 of 46 checks pass.**

Every row compares an expected value written into the test against what the import actually produced. The reference shot's two deliberate defects are the point of the first block: `layer3/layer3_007.png` does not exist, and drawing 13 of layer 2 lives under the Japanese filename `layer2_桜_013.png`.

| Check | Expected | Actual | Result |
|---|---|---|---|
| layer1: files selected | `1` | `1` | PASS |
| layer1: frames grouped | `1` | `1` | PASS |
| layer1: inferred pattern | `layer1_%03d.png` | `layer1_%03d.png` | PASS |
| layer1: drawing range | `0-0` | `0-0` | PASS |
| layer1: missing drawings | `none` | `none` | PASS |
| layer1: names not matching the pattern | `0` | `0` | PASS |
| layer1: gap diagnostic raised | `false` | `false` | PASS |
| layer2: files selected | `24` | `24` | PASS |
| layer2: frames grouped | `24` | `24` | PASS |
| layer2: inferred pattern | `layer2_%03d.png` | `layer2_%03d.png` | PASS |
| layer2: drawing range | `0-23` | `0-23` | PASS |
| layer2: missing drawings | `none` | `none` | PASS |
| layer2: names not matching the pattern | `1` | `1` | PASS |
| layer2: gap diagnostic raised | `false` | `false` | PASS |
| layer3: files selected | `11` | `11` | PASS |
| layer3: frames grouped | `11` | `11` | PASS |
| layer3: inferred pattern | `layer3_%03d.png` | `layer3_%03d.png` | PASS |
| layer3: drawing range | `0-11` | `0-11` | PASS |
| layer3: missing drawings | `7` | `7` | PASS |
| layer3: names not matching the pattern | `0` | `0` | PASS |
| layer3: gap diagnostic raised | `true` | `true` | PASS |
| layer4: files selected | `20` | `20` | PASS |
| layer4: frames grouped | `20` | `20` | PASS |
| layer4: inferred pattern | `layer4_%03d.png` | `layer4_%03d.png` | PASS |
| layer4: drawing range | `0-19` | `0-19` | PASS |
| layer4: missing drawings | `none` | `none` | PASS |
| layer4: names not matching the pattern | `0` | `0` | PASS |
| layer4: gap diagnostic raised | `false` | `false` | PASS |
| layer2: drawing 13 maps to its Japanese filename | `layer2_桜_013.png` | `layer2_桜_013.png` | PASS |
| layer2: drawing 13 decodes | `1920x1080` | `1920x1080` | PASS |
| layer3: drawing 7 refuses to decode | `MEDIA_SEQUENCE_GAP` | `MEDIA_SEQUENCE_GAP` | PASS |
| layer3: drawings 6 and 8 exist, so substitution was possible and did not happen | `6 and 8 present, 7 absent` | `6 and 8 present, 7 absent` | PASS |
| layer1: dimensions | `1920x1080` | `1920x1080` | PASS |
| layer1: fully opaque, per the fixture README | `true` | `true` | PASS |
| layer3: alpha is binary, per the fixture README | `true` | `true` | PASS |
| layer4: an interior pixel is at exactly code 128, per the fixture README | `true` | `true` | PASS |
| import tags buffers as sRGB / straight, per document 21 | `Srgb/Straight` | `Srgb/Straight` | PASS |
| mismatched dimensions: diagnostic raised | `true` | `true` | PASS |
| mismatched dimensions: sequence takes the majority size | `64x32` | `64x32` | PASS |
| mismatched dimensions: the odd drawing is still imported | `3` | `3` | PASS |
| 16-bit PNG: reported unsupported rather than truncated to 8-bit | `true` | `true` | PASS |
| 16-bit PNG: dropped from the frame map, leaving the 8-bit file | `1` | `1` | PASS |
| file with no number: diagnostic raised | `true` | `true` | PASS |
| file with no number: excluded, the rest import | `2` | `2` | PASS |
| two files claiming drawing 7: diagnostic raised | `true` | `true` | PASS |
| two files claiming drawing 7: one entry in the frame map | `1` | `1` | PASS |

## The diagnostics, exactly as a user would read them

Verbatim output, not a description of it. Severities and identifiers follow document 28.

### duplicate number

```
ERROR [MEDIA_SEQUENCE_DUPLICATE_NUMBER]
Two selected files both claim drawing 7.
cel_007.png and cel_07.png both end in 7. Only cel_007.png was imported.
Rename one file, or select only one of them.
```


### layer2

```
INFO [MEDIA_SEQUENCE_NAME_VARIANT]
One file does not match the pattern layer2_%03d.png but carries a clear number and was imported.
Imported under its own name: layer2_桜_013.png
```


### layer3

```
WARNING [MEDIA_SEQUENCE_GAP]
One drawing is missing from layer3_%03d.png: 7.
The sequence runs 0 to 11 and contains 11 files. Frames exposing a missing drawing render transparent; no neighbouring drawing is substituted.
Add the missing files to the folder and relink the sequence, or leave the gap if the hole is intended.
```


### mismatched dimensions

```
WARNING [MEDIA_SEQUENCE_DIMENSION_MISMATCH]
The drawings in shot_%03d.png are not all the same size. The sequence is treated as 64x32.
Sizes found:
  48x24: drawing 2
  64x32: drawings 0-1
Re-export the odd drawings at the sequence size. Until then they are placed at the layer origin and not scaled.
```


### no frame number

```
ERROR [MEDIA_SEQUENCE_UNNUMBERED]
One selected file has no number in its name and was not imported.
Not imported: notes.png
Import it as a still image, or rename it so it carries a drawing number.
```


### unsupported bit depth

```
ERROR [MEDIA_UNSUPPORTED_FORMAT]
deep_000.png uses a PNG format this build cannot read.
Found Rgba at 16 bits per channel. Supported: 8-bit RGBA and 8-bit RGB.
Re-export as 8-bit RGBA. The file is left untouched and the asset record is kept.
```


### Silent by design

layer1, layer4 produced no diagnostics at all. For layer 2 that is the result under test: a naive importer reports a false gap at drawing 13 because the pattern does not generate its filename.

## Not run by this test

- Relink after a moved or renamed sequence (R-08, B-09). Import here always finds every file where the selection said it was.
- Save and reopen of the Unicode filename (T-08, B-09). This test proves it survives import, not that it survives a round trip through the project file.
- The exposure sheet, holds and out-of-order re-exposure (T-02, B-04). Import produces a drawing-number map; nothing here maps composition frames onto it.
- Content fingerprints for cache invalidation (document 27, B-09). Deliberately not computed: it would mean a full read of every file for a benefit nothing yet consumes.
- Formats other than PNG (out of G1 scope, document 04).

## Four diagnostic identifiers are proposals

`MEDIA_SEQUENCE_GAP`, `MEDIA_UNSUPPORTED_FORMAT` and `MEDIA_DECODE_FAILED` come from document 28. The other four here — dimension mismatch, duplicate number, unnumbered file, name variant — do not appear in it. T-01 requires mismatched-dimension behaviour and document 28 defines no identifier for it, so they are registered as **D-19** in document 14 rather than quietly invented. If the owner names them differently, this table's identifiers change.
