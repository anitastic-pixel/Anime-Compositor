# B-05 model, commands and undo

Test T-03 (model half), requirements R-03 and R-07. Produced by `tests/b05_model.rs`. **61 of 61 checks pass.**

## What to check by eye

Three project dumps sit beside this file:

- `B-05_project_before.json` — the reference shot as four layers, before any edit.
- `B-05_project_after.json` — after every edit listed in the table below.
- `B-05_project_undone.json` — after pressing undo until nothing is left to undo.

The before and undone files should be identical. Any difference at all means undo did not restore the model exactly, which is the whole of R-07. Diffing them is the check; the test also makes it and reports it as a row below, but the files are there so the claim can be verified without trusting the test.

The after file should differ from the before file only in the edits that were made. An edit that appears there and is not in the table is a bug even if every check passes.

These dumps are the save format, written by the same `persist::to_json` the application saves through, so what is shown here is what would be on disk. Any of them can be opened again: `persist::load_str` reads one back into a project, and B-09's own artifacts cover that round trip.

The before file is 301 lines and the after file is 386 lines.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| opening document: revision | `0` | `0` | PASS |
| opening document: undo depth | `0` | `0` | PASS |
| opening document: dirty | `false` | `false` | PASS |
| opening document: layer order is bottom to top | `layer1, layer2, layer3, layer4` | `layer1, layer2, layer3, layer4` | PASS |
| rename: the new name is in the model | `sakura` | `sakura` | PASS |
| rename: the layer ID did not change, because names are not identity | `layer-2` | `layer-2` | PASS |
| rename: document is now dirty | `true` | `true` | PASS |
| hide: layer 3 is disabled | `false` | `false` | PASS |
| locked layer: the edit is rejected | `COMMAND_LAYER_LOCKED` | `COMMAND_LAYER_LOCKED` | PASS |
| rejected command: revision unchanged | `3` | `3` | PASS |
| rejected command: undo stack unchanged | `3` | `3` | PASS |
| rejected command: the model is untouched | `layer1` | `layer1` | PASS |
| locked layer: unlocking is still allowed | `ok` | `ok` | PASS |
| reorder: layer 1 moved to the top of the stack | `sakura, layer3, layer4, layer1` | `sakura, layer3, layer4, layer1` | PASS |
| reorder: an untouched layer is bit-identical afterwards | `Layer { id: Id("layer-4"), name: "layer4", asset_id: Id("asset-layer4"), enabled: true, locked: false, in_frame: 0, out_frame: 240, source_offset_frames: 0, transform: Transform { anchor: Property { base: Vec2(0.0, 0.0), keyframes: [] }, position: Property { base: Vec2(0.0, 0.0), keyframes: [] }, scale: Property { base: Vec2(1.0, 1.0), keyframes: [] }, rotation: Property { base: Scalar(0.0), keyframes: [] }, opacity: Property { base: Scalar(1.0), keyframes: [] } }, exposure_spans: [ExposureSpan { start_frame: 0, end_frame_exclusive: 3, drawing_number: 0 }, ExposureSpan { start_frame: 3, end_frame_exclusive: 6, drawing_number: 1 }], matte: None, blend_mode: Normal }` | `Layer { id: Id("layer-4"), name: "layer4", asset_id: Id("asset-layer4"), enabled: true, locked: false, in_frame: 0, out_frame: 240, source_offset_frames: 0, transform: Transform { anchor: Property { base: Vec2(0.0, 0.0), keyframes: [] }, position: Property { base: Vec2(0.0, 0.0), keyframes: [] }, scale: Property { base: Vec2(1.0, 1.0), keyframes: [] }, rotation: Property { base: Scalar(0.0), keyframes: [] }, opacity: Property { base: Scalar(1.0), keyframes: [] } }, exposure_spans: [ExposureSpan { start_frame: 0, end_frame_exclusive: 3, drawing_number: 0 }, ExposureSpan { start_frame: 3, end_frame_exclusive: 6, drawing_number: 1 }], matte: None, blend_mode: Normal }` | PASS |
| reorder undone: the exact prior order is back | `layer1, sakura, layer3, layer4` | `layer1, sakura, layer3, layer4` | PASS |
| reorder redone: the moved order is back | `sakura, layer3, layer4, layer1` | `sakura, layer3, layer4, layer1` | PASS |
| scalar edit: rotation is 12.5 degrees | `12.5` | `12.5` | PASS |
| scalar edit undone: the exact prior value returns | `0` | `0` | PASS |
| scalar edit redone: the exact value returns | `12.5` | `12.5` | PASS |
| opacity: a value above 1 is clamped, not rejected | `1` | `1` | PASS |
| scale: a negative value is accepted, because it mirrors | `(-1, 1)` | `(-1, 1)` | PASS |
| property type: a scalar cannot be written to position | `COMMAND_INVALID_VALUE` | `COMMAND_INVALID_VALUE` | PASS |
| unknown layer: the command is rejected | `COMMAND_TARGET_MISSING` | `COMMAND_TARGET_MISSING` | PASS |
| keyframes: three were stored | `3` | `3` | PASS |
| rule, before the first keyframe (frame -5) | `(20, 0)` | `(20, 0)` | PASS |
| rule, exactly on the first keyframe | `(20, 0)` | `(20, 0)` | PASS |
| rule, inside a hold segment (frame 6) | `(20, 0)` | `(20, 0)` | PASS |
| rule, on the second keyframe | `(100, 0)` | `(100, 0)` | PASS |
| rule, halfway along a linear segment (frame 18) | `(100, 30)` | `(100, 30)` | PASS |
| rule, one third along a linear segment (frame 16) | `(100, 20)` | `(100, 20)` | PASS |
| rule, on the last keyframe | `(100, 60)` | `(100, 60)` | PASS |
| rule, after the last keyframe (frame 300) | `(100, 60)` | `(100, 60)` | PASS |
| an unkeyframed property still returns its base value | `(0, 0)` | `(0, 0)` | PASS |
| setting a keyframe where one exists replaces it rather than duplicating | `3` | `3` | PASS |
| removing a keyframe that is not there is rejected | `COMMAND_TARGET_MISSING` | `COMMAND_TARGET_MISSING` | PASS |
| drag: 100 intermediate values produced one history record | `13` | `13` | PASS |
| drag: the final value is the one that landed | `(100, 0)` | `(100, 0)` | PASS |
| drag undone: the value from before the drag returns in one step | `(0, 0)` | `(0, 0)` | PASS |
| drag redone: the final value returns | `(100, 0)` | `(100, 0)` | PASS |
| drag cancelled: no history record and the value is restored | `13, (100, 0)` | `13, (100, 0)` | PASS |
| transaction: an invalid second command rejects the whole batch | `COMMAND_INVALID_VALUE` | `COMMAND_INVALID_VALUE` | PASS |
| transaction rejected: the asset from the first command was not added either | `4` | `4` | PASS |
| transaction rejected: revision unchanged | `19` | `19` | PASS |
| transaction: import plus create layer is one history record | `14` | `14` | PASS |
| transaction: both parts landed | `5 assets, 5 layers` | `5 assets, 5 layers` | PASS |
| transaction undone: both parts are gone in one step | `4 assets, 4 layers` | `4 assets, 4 layers` | PASS |
| matte: layer 3 now has a dependent | `layer-fx` | `layer-fx` | PASS |
| matte: a cycle is rejected | `MATTE_CYCLE` | `MATTE_CYCLE` | PASS |
| matte: a reference to a layer that is not there is rejected | `MATTE_REFERENCE_MISSING` | `MATTE_REFERENCE_MISSING` | PASS |
| matte: the dependent layer still records the reference after its target is deleted | `layer-3` | `layer-3` | PASS |
| matte: undoing the delete restores the target and the dependent exactly | `Layer { id: Id("layer-fx"), name: "fx", asset_id: Id("asset-effects"), enabled: true, locked: false, in_frame: 0, out_frame: 240, source_offset_frames: 0, transform: Transform { anchor: Property { base: Vec2(0.0, 0.0), keyframes: [] }, position: Property { base: Vec2(0.0, 0.0), keyframes: [] }, scale: Property { base: Vec2(1.0, 1.0), keyframes: [] }, rotation: Property { base: Scalar(0.0), keyframes: [] }, opacity: Property { base: Scalar(1.0), keyframes: [] } }, exposure_spans: [], matte: Some(MatteReference { layer_id: Id("layer-3") }), blend_mode: Normal }` | `Layer { id: Id("layer-fx"), name: "fx", asset_id: Id("asset-effects"), enabled: true, locked: false, in_frame: 0, out_frame: 240, source_offset_frames: 0, transform: Transform { anchor: Property { base: Vec2(0.0, 0.0), keyframes: [] }, position: Property { base: Vec2(0.0, 0.0), keyframes: [] }, scale: Property { base: Vec2(1.0, 1.0), keyframes: [] }, rotation: Property { base: Scalar(0.0), keyframes: [] }, opacity: Property { base: Scalar(1.0), keyframes: [] } }, exposure_spans: [], matte: Some(MatteReference { layer_id: Id("layer-3") }), blend_mode: Normal }` | PASS |
| matte: the restored target is back in its original position | `sakura, layer3, layer4, layer1, fx` | `sakura, layer3, layer4, layer1, fx` | PASS |
| after save: dirty | `false` | `false` | PASS |
| after an edit following save: dirty | `true` | `true` | PASS |
| after undoing back to the saved state: dirty is false again | `false` | `false` | PASS |
| and redo is still available even though the document is clean | `1` | `1` | PASS |
| undoing every command returns the project to its opening state, byte for byte | `identical` | `identical` | PASS |
| after undoing everything: undo depth | `0` | `0` | PASS |
| after undoing everything: every undone command is redoable | `16` | `16` | PASS |
| a new command clears the redo stack | `0` | `0` | PASS |

## Document 26's required tests, item by item

| Document 26 asks for | Covered by |
|---|---|
| scalar property edit, undo, redo exact value | the three `scalar edit` rows |
| layer reorder restores exact order and references | the four `reorder` rows, including one asserting an untouched layer is bit-identical after the move |
| deleting/recovering a matte preserves dependent records | the five `matte` rows |
| import/create-layer transaction is all-or-nothing | the six `transaction` rows |
| drag of 100 intermediate values produces one undo item | the five `drag` rows |
| rejected command produces no revision/history change | the three `rejected command` rows, plus every row naming a diagnostic ID |
| save -> edit -> undo back to saved state clears dirty | the four `after save` rows |
| undo/redo after project reopen is empty | not run: reopening is B-09 |

## Not run by this test

- The render half of T-03: FX-XF-001 identity, FX-XF-002 integer translation, FX-XF-003 half-pixel bilinear weights and FX-XF-004 rotation about a nonzero anchor. Those need a transform renderer, which is B-05a. This test covers the model values that renderer will read, and the keyframe rows above are the animation those fixtures will sample.
- Save and reopen, and therefore document 26's "undo/redo after project reopen is empty". That is B-09 and T-07.
- Cache invalidation domains, which document 26 requires every committed command to report to document 27. There is no cache yet; it is B-07.
- Effects and masks. Effects are B-06; masks are parked to G1-rest with R-04 under D-12. A layer therefore serialises `"effects": []`, which is accurate rather than a placeholder, and carries no mask field at all.
- Colour4 and boolean properties, which document 19 lists. G1 needs them once effects have colour parameters, which is B-06.
