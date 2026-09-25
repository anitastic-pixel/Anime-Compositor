# B-17b: adjustment layers

D-66, accepted by the owner on 2026-09-17. Every expected pixel is `Fixtures/adjust/expected_adjust.json`, written by `tools/adjust_reference.py` before this code existed and printed in document 25 as FX-ADJ-001 to 013. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6.

## FX-ADJ-001 to 013, rendered whole (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADJ-001 frame 0: One stop brighter on everything below. | largest difference 6.1e-8 | yes |
| FX-ADJ-002 frame 0: The same at half opacity: half way between. | largest difference 3.8e-8 | yes |
| FX-ADJ-003 frame 0: A layer above the adjustment layer is not adjusted. | largest difference 8.2e-8 | yes |
| FX-ADJ-004 frame 0: A mask on the adjustment layer: only the left three columns. | largest difference 6.1e-8 | yes |
| FX-ADJ-005 frame 0: Moved four pixels right: only the right two columns. | largest difference 6.1e-8 | yes |
| FX-ADJ-006 frame 0: A matte-only layer shapes the adjustment: only the left three columns. | largest difference 6.1e-8 | yes |
| FX-ADJ-007 frame 0: A blur spreads one pixel over the frame and is cut off at its edge. | largest difference 1.6e-8 | yes |
| FX-ADJ-008 frame 0: A full tint on a half-covered picture keeps its coverage. | largest difference 3.0e-8 | yes |
| FX-ADJ-009 frame 0: Two adjustment layers, lower one first. | largest difference 3.1e-8 | yes |
| FX-ADJ-010 frame 0: The same two, the other way up. | largest difference 3.1e-8 | yes |
| FX-ADJ-011 frame 0: No effects switched on: the frame is the frame without the layer. | largest difference 3.1e-8 | yes |
| FX-ADJ-012 frame 0: Behind the picture in depth: drawn first, onto nothing, so nothing changes. | largest difference 3.1e-8 | yes |
| FX-ADJ-013 frame 0: Only between its in and out frames (frame 1 only). | largest difference 3.1e-8 | yes |
| FX-ADJ-013 frame 1: Only between its in and out frames (frame 1 only). | largest difference 6.1e-8 | yes |
| FX-ADJ-013 frame 2: Only between its in and out frames (frame 1 only). | largest difference 3.1e-8 | yes |

## Tiled against untiled: the same frame whatever it is cut into

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADJ-001 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-002 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-003 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-004 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-005 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-006 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-007 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-008 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-009 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-010 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-011 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-012 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-013 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-013 frame 1: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-ADJ-013 frame 2: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |

## Nothing to say and something to say

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADJ-011, an effect switched off: no diagnostic (a bypass a person chose is not a fault) | [] | yes |
| an adjustment layer with an effect this build does not have: EFFECT_UNSUPPORTED at the frame | [EffectUnsupported] | yes |
| and that frame is the frame beneath it, untouched | byte-identical to the frame with the layer removed | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_adj_006.json's `adj` reads as kind adjustment, with its matte and effect | adjustment, matte Some("matte"), 1 effect(s) | yes |
| written back: `kind` is adjustment and there is no asset_id, source_offset_frames or exposure_spans | kind "adjustment", keys present: ["kind"] | yes |
| and it opens again as the same layer | equal | yes |
| an adjustment layer given an asset_id: PROJECT_SCHEMA_INVALID | Some(ProjectSchemaInvalid) | yes |
| an adjustment layer with blend_mode multiply: PROJECT_SCHEMA_INVALID | Some(ProjectSchemaInvalid) | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| layer.set_blend_mode multiply on an adjustment layer: COMMAND_INVALID_VALUE | Some(CommandInvalidValue) | yes |
| layer.set_blend_mode normal on one: accepted | accepted | yes |
| a new adjustment layer needs no asset, and its anchor and position are the centre of the composition | added true, anchor Vec2(3.0, 1.0), position Vec2(3.0, 1.0) | yes |

## The draft preview (D-33, D-66)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ADJ-007's blur of sigma 1 is sigma 0.25 on the quarter-size frame | Some(0.25) | yes |

## The trace (ADR-012)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the frame render_traced returns is the frame render returns | byte-identical | yes |
| the manifest names the adjustment layer's composite image as the adjusted frame, once | 1 line(s); 12 images written | yes |

## Result

44 of 44 checks pass.
