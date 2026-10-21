# B-611: open masks

D-737: a mask may be open, with no piece from its last point back to its first. It cuts nothing and is drawn along open by the effects that take a path. Every expected pixel is `Fixtures/open_masks/expected_open_masks.json`, written by `tools/open_mask_reference.py` before this code existed and printed in document 25 as FX-MSKO. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 2e-5.

## FX-MSKO-001 to 010 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MSKO-001 frame 0: An open roof, two legs from (2, 7) up to (8, 2) and down to (14, 7), as Mask 1, stroked: drawn open, nothing along the bottom, FX-STROKE-049's line on the night drawing. | largest difference 2.8e-8 | yes |
| FX-MSKO-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-002 frame 0: The same roof closed, `closed` left out: the bottom is drawn too, as every mask was. | largest difference 3.2e-8 | yes |
| FX-MSKO-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-003 frame 0: The open roof, End 50: the left leg alone, half of its open length. | largest difference 2.8e-8 | yes |
| FX-MSKO-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-004 frame 0: A mask of two points, open, a line from (3, 8) to (12, 3): drawn along, and no warning that it has too few points. | largest difference 2.9e-8 | yes |
| FX-MSKO-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-005 frame 0: The curved ring open: a circle with no piece back from its last point, (4, 5), to its first, (8, 1). | largest difference 2.9e-8 | yes |
| FX-MSKO-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-006 frame 0: Every mask, sequentially, End 60: the open roof then the closed box, one length end to end, the roof's without its bottom. | largest difference 2.9e-8 | yes |
| FX-MSKO-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-MSKO-010 frame 0: Two open masks, the roof and the line, `closed` written on a third, closed box of mode Add that keeps the drawing inside it, and no effect: the open masks cut nothing; the box cuts as it always has. | largest difference 7.3e-9 | yes |
| FX-MSKO-010 frame 4: Two open masks, the roof and the line, `closed` written on a third, closed box of mode Add that keeps the drawing inside it, and no effect: the open masks cut nothing; the box cuts as it always has. | largest difference 7.3e-9 | yes |
| FX-MSKO-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_msko_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_msko_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| FX-MSKO-020: refused_closed_word.json is refused, PROJECT_SCHEMA_INVALID, naming closed | PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/masks/0/closed: expected true or false. | yes |
| FX-MSKO-021: refused_open_add.json is refused, PROJECT_SCHEMA_INVALID, naming closed | PROJECT_SCHEMA_INVALID: At /compositions/0/layers/0/masks/0/closed: expected an open mask of mode none, since it encloses nothing to add or take away (D-737). | yes |

## Opening and closing, one step each

| Check | The build's answer | Matches |
| --- | --- | --- |
| the closed roof opened by one command draws FX-MSKO-001's frame | taken true, the same pixels true | yes |
| one undo closes it again | closed true | yes |
| an open mask of mode Add is refused with a sentence, and nothing changes | Mask "Mask 1" is open, so its mode is None: it encloses nothing to add or take away. | yes |
| an open mask of two points is taken; closed, two points are refused | open taken true; closed: A mask needs at least three points, and this one has 2. | yes |
| an open mask whose legs cross is taken: it encloses nothing | taken true | yes |

## Result

29 of 29 checks pass.
