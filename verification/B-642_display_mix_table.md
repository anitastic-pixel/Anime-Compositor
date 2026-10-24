# B-642: colours mix in display values in 8 bpc (After Effects) (D-763)

Written by `tests/b642_display_mix.rs`. Every expected pixel is from `tools/display_mix_reference.py`, which never runs the build's code.

## FX-MIX8-001 to 012: each scene in 8 bpc, and the same scene in Display

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIX8-001 frame 0: 8 bpc: Four colours under a Screen layer, its second row at 50% opacity. | largest difference 2.4e-7 | yes |
| FX-MIX8-001: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-002 frame 0: Display, unchanged by D-763: Four colours under a Screen layer, its second row at 50% opacity. | largest difference 5.6e-8 | yes |
| FX-MIX8-002: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-003 frame 0: 8 bpc: The same under an Add layer: where the two pass white it is held to white. | largest difference 1.2e-7 | yes |
| FX-MIX8-003: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-004 frame 0: Display, unchanged by D-763: The same under an Add layer: where the two pass white it is held to white. | largest difference 2.8e-8 | yes |
| FX-MIX8-004: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-005 frame 0: 8 bpc: A Normal layer at 50% opacity over the same colours. | largest difference 8.1e-8 | yes |
| FX-MIX8-005: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-006 frame 0: Display, unchanged by D-763: A Normal layer at 50% opacity over the same colours. | largest difference 1.4e-8 | yes |
| FX-MIX8-006: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-007 frame 0: 8 bpc: A black Drop Shadow at 50%, softness 0, cast one pixel right of a blue block onto white: column 3 is the shadow alone over white. | largest difference 9.0e-8 | yes |
| FX-MIX8-007: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-008 frame 0: Display, unchanged by D-763: A black Drop Shadow at 50%, softness 0, cast one pixel right of a blue block onto white: column 3 is the shadow alone over white. | largest difference 1.2e-8 | yes |
| FX-MIX8-008: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-009 frame 0: 8 bpc: A colour under another whose mask, the left three columns, is feathered 2 pixels (FX-MSK-014's): the soft band at columns 1 to 4. | largest difference 2.3e-7 | yes |
| FX-MIX8-009: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-010 frame 0: Display, unchanged by D-763: A colour under another whose mask, the left three columns, is feathered 2 pixels (FX-MSK-014's): the soft band at columns 1 to 4. | largest difference 4.2e-8 | yes |
| FX-MIX8-010: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-011 frame 0: 8 bpc: Mosaic in blocks of 2 by 2 over stripes of two colours: each block one colour, the mean of its four pixels. | largest difference 3.0e-8 | yes |
| FX-MIX8-011: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| FX-MIX8-012 frame 0: Display, unchanged by D-763: Mosaic in blocks of 2 by 2 over stripes of two colours: each block one colour, the mean of its four pixels. | largest difference 7.6e-8 | yes |
| FX-MIX8-012: what opening it warns of, and what frame 0 warns of | [] and [] | yes |
| fx_mix8_001.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix8_007.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix8_009.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |
| fx_mix8_011.json frame 0 in tiles of 1 and of 64 | byte-identical | yes |

## Display is unchanged: each 8 bpc scene set to Display draws the Display frame

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIX8-001 set to Display draws FX-MIX8-002's frame, the linear mix | largest difference 5.6e-8 | yes |
| And set back to 8 bpc, FX-MIX8-001's frame again | largest difference 2.4e-7 | yes |
| FX-MIX8-003 set to Display draws FX-MIX8-004's frame, the linear mix | largest difference 2.8e-8 | yes |
| And set back to 8 bpc, FX-MIX8-003's frame again | largest difference 1.2e-7 | yes |
| FX-MIX8-005 set to Display draws FX-MIX8-006's frame, the linear mix | largest difference 1.4e-8 | yes |
| And set back to 8 bpc, FX-MIX8-005's frame again | largest difference 8.1e-8 | yes |
| FX-MIX8-007 set to Display draws FX-MIX8-008's frame, the linear mix | largest difference 1.2e-8 | yes |
| And set back to 8 bpc, FX-MIX8-007's frame again | largest difference 9.0e-8 | yes |
| FX-MIX8-009 set to Display draws FX-MIX8-010's frame, the linear mix | largest difference 4.2e-8 | yes |
| And set back to 8 bpc, FX-MIX8-009's frame again | largest difference 2.3e-7 | yes |
| FX-MIX8-011 set to Display draws FX-MIX8-012's frame, the linear mix | largest difference 7.6e-8 | yes |
| And set back to 8 bpc, FX-MIX8-011's frame again | largest difference 3.0e-8 | yes |

## A composition inside another follows the outermost one

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIX8-001's Screen inside a composition: outer 8 bpc, inner Display, draws FX-MIX8-001's frame | largest difference 2.4e-7 | yes |
| FX-MIX8-001's Screen inside a composition: outer Display, inner 8 bpc, draws FX-MIX8-002's frame | largest difference 5.6e-8 | yes |
| FX-MIX8-001's Screen inside a composition: outer 8 bpc, inner 8 bpc, draws FX-MIX8-001's frame | largest difference 2.4e-7 | yes |

## Old 8 bpc projects open as they are and draw their D-330 frames (one layer, nothing beneath)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-8BPC-001 (fx_8bpc_001.json), saved before D-763 with no new line | opens with 0 warnings, largest difference 7.8e-8 | yes |
| FX-8BPC-002 (fx_8bpc_002.json), saved before D-763 with no new line | opens with 0 warnings, largest difference 1.5e-7 | yes |
| FX-8BPC-003 (fx_8bpc_003.json), saved before D-763 with no new line | opens with 0 warnings, largest difference 1.4e-7 | yes |
| FX-8BPC-004 (fx_8bpc_004.json), saved before D-763 with no new line | opens with 0 warnings, largest difference 7.8e-8 | yes |
| FX-8BPC-005 (fx_8bpc_005.json), saved before D-763 with no new line | opens with 0 warnings, largest difference 3.1e-8 | yes |

## The viewer, and the card

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-MIX8-007 (Drop Shadow 50% over white) in the viewer | largest difference 9.0e-8 | yes |
| fx_mix8_001.json (8 bpc): the preview draws the CPU's picture within 1 level of 255, on the CPU | CPU, largest difference 0 of 255 | yes |
| fx_mix8_007.json (8 bpc): the preview draws the CPU's picture within 1 level of 255, on the CPU | CPU, largest difference 0 of 255 | yes |
| fx_mix8_002.json (Display): the preview draws the CPU's picture within 1 level of 255, the card free to draw it | card, largest difference 0 of 255 | yes |

## Result

52 of 52 checks pass.
