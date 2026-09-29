# B-140: Kaleidoscope

D-205, accepted on 2026-09-28 with the After Effects picks (B3). Every expected pixel is `Fixtures/kaleidoscope/expected_kaleidoscope.json`, written by `tools/kaleidoscope_reference.py` before this code existed and printed in document 25 as FX-KALEIDO-001 to 020. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-KALEIDO-001 to 020 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-KALEIDO-001 frame 0: The settings as they start: 6 segments, Mirror, rotation 0, size 100, the centre in the middle: the wedge from straight up through 60 degrees clockwise is the drawing's own, and it is repeated round, every other copy mirrored. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-002 frame 0: 2 segments, Mirror: the right half is the drawing's own and the left half its mirror image. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-003 frame 0: 2 segments, Repeat: the right half is the drawing's own and the left half the right half turned half round the centre. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-004 frame 0: 6 segments, Repeat: FX-KALEIDO-001's first wedge, every copy turned the same way. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-005 frame 0: Rotation 30: the wedge from 30 degrees through 90 is the drawing's own. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-006 frame 0: Size 200: the pattern twice as large, read from half as far out. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-007 frame 0: Size 50: the pattern half as large, reaching past the drawing, where it reads the drawing mirrored back at its edges. | largest difference 2.3e-7 | yes |
| FX-KALEIDO-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-008 frame 0: The centre at 25 per cent across, 50 down. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-009 frame 0: Segments 6.9, which counts as 6: FX-KALEIDO-001. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-010 frame 0: Rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-KALEIDO-001 and frame 2, rotation 30, FX-KALEIDO-005. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-010 frame 2: Rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-KALEIDO-001 and frame 2, rotation 30, FX-KALEIDO-005. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-010 frame 4: Rotation keyed from 0 at frame 0 to 60 at frame 4, linear: frame 0 is FX-KALEIDO-001 and frame 2, rotation 30, FX-KALEIDO-005. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-011 frame 0: Size keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is size 1000. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-011 frame 2: Size keyed from 100 at frame 0 to 1000 at frame 4, eased past its end: frame 2 would pass 1000, is held at 1000, and is size 1000. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-012 frame 0: FX-KALEIDO-001 moved three pixels right: the pattern moves with the drawing and the three columns left bare stay empty. | largest difference 2.5e-7 | yes |
| FX-KALEIDO-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-013 frame 0: After a Motion Tile that grows the layer: the centre and the wedge are the drawing's own, not the grown buffer's, and the pattern reads the tiles round it. | largest difference 2.7e-7 | yes |
| FX-KALEIDO-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-KALEIDO-014 frame 0: Segments 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-014 frame 4: Segments 1, below 2. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-014: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-015 frame 0: Segments 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-015 frame 4: Segments 33, above 32. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-015: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-016 frame 0: Size 9, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-016 frame 4: Size 9, below 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-016: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-017 frame 0: Centre across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-017 frame 4: Centre across 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-017: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-018 frame 0: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-018 frame 4: Rotation 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-018: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-019 frame 0: A mode "flower", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-019 frame 4: A mode "flower", which is not one. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-019: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-KALEIDO-020 frame 0: Size keyed to 1001 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-020 frame 4: Size keyed to 1001 at frame 4, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.9e-7 | yes |
| FX-KALEIDO-020: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## How far it reaches

| Check | The build's answer | Matches |
| --- | --- | --- |
| it never grows the layer: it declares no growth | 0 | yes |
| a half-size draft preview keeps every setting, as none is a distance in pixels | Kaleidoscope { segments: 8.0, rotation: 30.0, size: 200.0, center: [25.0, 75.0], mode: "repeat" } | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kaleido_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_kaleido_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| a file with no `mode` at all is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a centre of one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with segments that are a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Commands

| Check | The build's answer | Matches |
| --- | --- | --- |
| segments 1 is refused with a sentence, and nothing changes | Kaleidoscope's segments runs from 2 to 32, and this is 1. | yes |
| segments 33 is refused with a sentence, and nothing changes | Kaleidoscope's segments runs from 2 to 32, and this is 33. | yes |
| rotation -3601 is refused with a sentence, and nothing changes | Kaleidoscope's rotation runs from -3600 to 3600, and this is -3601. | yes |
| size 9 is refused with a sentence, and nothing changes | Kaleidoscope's size runs from 10 to 1000, and this is 9. | yes |
| size 1001 is refused with a sentence, and nothing changes | Kaleidoscope's size runs from 10 to 1000, and this is 1001. | yes |
| centre down -1001 is refused with a sentence, and nothing changes | Kaleidoscope's center runs from -1000 to 1000, and this is -1001. | yes |
| mirroring "flower" is refused with a sentence, and nothing changes | Kaleidoscope's mirroring is "mirror" or "repeat", and this is "flower". | yes |
| mirroring "Mirror", written with a capital is refused with a sentence, and nothing changes | Kaleidoscope's mirroring is "mirror" or "repeat", and this is "Mirror". | yes |
| segments keyed to 40 is refused with a sentence, and nothing changes | Kaleidoscope's segments runs from 2 to 32, and this is 40. | yes |
| centre keyed to 2000 across is refused with a sentence, and nothing changes | Kaleidoscope's center runs from -1000 to 1000, and this is 2000. | yes |
| every number at its bottom, is taken | taken | yes |
| every number at its top, is taken | taken | yes |
| segments keyed from 2 to 12 is taken | taken | yes |
| centre keyed across the drawing is taken | taken | yes |
| undo 4 times: frame 0 is the frame it was | byte-identical | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_kaleido_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kaleido_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kaleido_010.json frame 2 in tiles of 1 and of 64 | byte-identical | yes |
| fx_kaleido_013.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up drawing, in `verification/B-140 pictures/`, three times enlarged

| Check | The build's answer | Matches |
| --- | --- | --- |
| before.png, the drawing with no effect, draws cleanly | [] | yes |
| six_mirror.png, as it starts, six wedges mirrored: the wedge from straight up clockwise through 60 degrees is the drawing itself, every pixel inside it within 1 of before.png; the left half the right half mirrored, every pixel; nothing clear; draws cleanly | [], 2049 of 2049 in the wedge, 16000 of 16000 mirrored, 0 clear | yes |
| six_repeat.png, Repeat: the first wedge the same as six_mirror.png, every pixel, and the next wedge different from it in most pixels, as it is turned rather than mirrored; nothing clear; draws cleanly | [], first wedge 2049 of 2049 the same, next wedge 3382 of 3544 different, 0 clear | yes |
| twelve.png, 12 segments: a finer pattern, its first wedge, 30 degrees wide, the drawing itself; nothing clear; draws cleanly | [], 669 of 669 in the wedge, 0 clear | yes |
| rotation_30.png, rotation 30: the pattern turned, the wedge from 30 degrees clockwise through 90 now the drawing itself; draws cleanly | [], 3192 of 3192 in the wedge | yes |
| size_50.png, size 50: the pattern half as large, reaching past the drawing's edges, where it reads the drawing mirrored back, so nothing is clear; draws cleanly | [], 0 clear | yes |
| centre_25.png, the centre a quarter of the way across: the pattern turns round that point, the strip left of it the strip right of it mirrored, every pixel of the left half; nothing clear; draws cleanly | [], 8000 of 8000 mirrored, 0 clear | yes |

## Result

95 of 95 checks pass.
