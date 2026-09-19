# B-18b: precompositions

D-67, accepted by the owner on 2026-09-18. Every expected pixel is `Fixtures/precomp/expected_precomp.json`, written by `tools/precomp_reference.py` before this code existed and printed in document 25 as FX-PRE-001 to 015. The build's frame is compared sample by sample; the answer is the largest difference over all of them, against the catalogue's tolerance of 1e-6.

## FX-PRE-001 to 015 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PRE-001 frame 0: A composition layer shows the inner composition's frame. | largest difference 3.1e-8 | yes |
| FX-PRE-002 frame 0: A blur on the composition layer blurs the inner picture as one, not each drawing on its own. | largest difference 9.3e-8 | yes |
| FX-PRE-003 frame 0: Half opacity on the composition layer: the inner picture at half. | largest difference 1.5e-8 | yes |
| FX-PRE-004 frame 0: Moved two pixels right: the inner picture moves as one drawing. | largest difference 0.0e0 | yes |
| FX-PRE-005 frame 0: A two-by-two inner composition lands centred, at its own size. | largest difference 3.1e-8 | yes |
| FX-PRE-006 frame 0: The inner composition's own time: a source offset of one frame, and nothing past its end. | largest difference 3.1e-8 | yes |
| FX-PRE-006 frame 1: The inner composition's own time: a source offset of one frame, and nothing past its end. | largest difference 0.0e0 | yes |
| FX-PRE-006 frame 2: The inner composition's own time: a source offset of one frame, and nothing past its end. | largest difference 0.0e0 | yes |
| FX-PRE-007 frame 0: Only between the composition layer's own in and out frames (frame 1 only). | largest difference 0.0e0 | yes |
| FX-PRE-007 frame 1: Only between the composition layer's own in and out frames (frame 1 only). | largest difference 3.1e-8 | yes |
| FX-PRE-007 frame 2: Only between the composition layer's own in and out frames (frame 1 only). | largest difference 0.0e0 | yes |
| FX-PRE-008 frame 0: Two levels deep: each level's own opacity and effects. | largest difference 3.1e-8 | yes |
| FX-PRE-009 frame 0: An adjustment layer inside stays inside: the layer above the composition layer is not adjusted. | largest difference 8.2e-8 | yes |
| FX-PRE-010 frame 0: An adjustment layer above a composition layer adjusts it like any layer. | largest difference 6.1e-8 | yes |
| FX-PRE-011 frame 0: A mask on the composition layer: only the left three columns. | largest difference 3.1e-8 | yes |
| FX-PRE-012 frame 0: A matte-only layer shapes the composition layer: only the left three columns. | largest difference 3.1e-8 | yes |
| FX-PRE-013 frame 0: The same composition twice, the second moved three right: two layers. | largest difference 3.0e-8 | yes |
| FX-PRE-014 frame 0: A composition that does not exist: drawn as nothing, with COMPOSITION_REFERENCE_MISSING. | largest difference 0.0e0 | yes |
| FX-PRE-014: opening it says so, once | ["COMPOSITION_REFERENCE_MISSING"] | yes |
| FX-PRE-014: and so does the frame, so an export is marked | ["COMPOSITION_REFERENCE_MISSING"] | yes |
| FX-PRE-015: Main holds a layer of Inner and Inner a layer of Main: the file is refused. | refused to open: Some("COMPOSITION_CYCLE") | yes |

## Tiled against untiled: the same frame whatever it is cut into

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PRE-001 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-002 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-003 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-004 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-005 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-006 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-006 frame 1: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-006 frame 2: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-007 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-007 frame 1: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-007 frame 2: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-008 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-009 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-010 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-011 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-012 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-013 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |
| FX-PRE-014 frame 0: one tile, 12 tiles of 1, tiles of 4 and unculled | byte-identical | yes |

## A composition layer against a drawn layer of the same picture

| Check | The build's answer | Matches |
| --- | --- | --- |
| blend mode normal: the composition layer and the drawn layer | byte-identical | yes |
| blend mode multiply: the composition layer and the drawn layer | byte-identical | yes |
| blend mode screen: the composition layer and the drawn layer | byte-identical | yes |
| blend mode add: the composition layer and the drawn layer | byte-identical | yes |

## Inner on its own against its picture inside Main, with a camera

| Check | The build's answer | Matches |
| --- | --- | --- |
| Inner's camera moved one pixel right: Inner rendered alone, and Main showing it | byte-identical | yes |
| and the camera took part: that frame is not the frame without it | different | yes |

## What goes wrong inside is said about the frame that was asked for

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PRE-006 with its drawings moved away: frame 0 of Main shows frame 1 of Inner, and MEDIA_MISSING is recorded against frame 0 | ["MEDIA_MISSING"] | yes |

## The file (document 19)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_pre_001.json's `pre` reads as kind composition, showing comp-inner | composition, showing Some("comp-inner") | yes |
| written back: composition_id and source_offset_frames, and no asset_id or exposure_spans | kind "composition", keys present: ["composition_id", "source_offset_frames"] | yes |
| and it opens again as the same project | equal | yes |
| FX-PRE-014 written back still names the composition that is not there | comp-gone is kept | yes |
| a composition layer given an asset_id: PROJECT_SCHEMA_INVALID | Some(ProjectSchemaInvalid) | yes |
| a composition layer given exposure_spans: PROJECT_SCHEMA_INVALID | Some(ProjectSchemaInvalid) | yes |
| a composition layer with no composition_id: PROJECT_SCHEMA_INVALID | Some(ProjectSchemaInvalid) | yes |

## Commands (document 24)

| Check | The build's answer | Matches |
| --- | --- | --- |
| layer.add_composition: a 2 by 2 composition into a 6 by 2 one has its anchor at its own centre and its position at the outer centre | added true, anchor Vec2(1.0, 1.0), position Vec2(3.0, 1.0) | yes |
| a composition layer naming a composition that is not there: COMMAND_TARGET_MISSING | Some(CommandTargetMissing) | yes |
| Main shown inside Inner, which Main already shows: COMPOSITION_CYCLE | COMPOSITION_CYCLE: "Main" cannot be shown here, because it would end up inside itself. | yes |
| Main shown inside itself: COMPOSITION_CYCLE | COMPOSITION_CYCLE: "Main" cannot be shown here, because it would end up inside itself. | yes |
| composition.delete on Inner while Main shows it: refused, naming Main | This composition is shown by a layer of "Main", so it stays. Delete that layer first if this one should go. | yes |
| and once that layer is deleted, Inner can be | accepted | yes |
| layer.precompose on a layer without the matte it uses: COMMAND_INVALID_VALUE, with a sentence | "pre" and "matte" are tied by a parent or a matte, so they are pre-composed together or not at all. Choose both. | yes |
| on both together: Main holds one composition layer and Precomp 1 holds the two | Main: Precomp 1 (composition); Precomp 1: matte (raster), pre (composition) | yes |
| the frame is the frame it was | byte-identical | yes |
| it is one undo, and Undo puts the project back exactly | 1 undo record(s) added; equal | yes |
| pre-composing the lower of two layers leaves the new layer in its place, beneath the other | Precomp 1 (composition), adj (adjustment) | yes |
| and the frame, now two compositions deep, is the frame it was | byte-identical | yes |

## The draft preview (D-33, D-67)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-PRE-001's inner picture is rendered 6 by 2 for a full frame and 2 by 1 for a draft | full (6, 2), draft (2, 1) | yes |
| the draft frame of Main is 2 by 1 | 2 by 1 | yes |
| and it is the draft frame of Inner on its own | byte-identical | yes |

## The trace (ADR-012)

| Check | The build's answer | Matches |
| --- | --- | --- |
| the frame render_traced returns is the frame render returns | byte-identical | yes |
| FX-PRE-006 frame 0: the manifest names the layer's decode image as frame 1 of comp-inner, once | 1 line(s) | yes |

## Result

70 of 70 checks pass.
