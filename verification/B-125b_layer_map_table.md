# B-125b: a layer of this composition as an effect's setting (D-189)

Written by `tests/b125_layer_map.rs`. Each row is one case of document 25's FX-LMAP table: the program makes the map an effect on the holder would read from the layer its setting names, and it is compared with `Fixtures/layer_map/expected_layer_map.json`, which `tools/effect_layer_reference.py` worked out pixel by pixel from documents 20 and 21 without running the program. Tolerance 1e-6. Nothing is seen in the app until Compound Blur (A2), the first effect with a layer setting; `verification/B-125 pictures/maps.png` shows every map the program made, enlarged.

## FX-LMAP-001 to 042: the map each case reads

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-LMAP-001: `card`, a plain drawing 5 by 3, in `holder`, a solid the size of the composition: centred at (1, 1), transparent round it. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-002: `card_placed`, the same drawing moved, scaled 200 by 50, turned 30 degrees, at half opacity and Multiply: the same map as FX-LMAP-001, because nothing after step 3 counts. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-003: `card_hidden`, switched off and used only as `holder`'s matte: the same map as FX-LMAP-001. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-004: `card_masked`, its mask keeping columns 0 to 2: the map is cut the same way. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-005: `card_bright`, with Exposure +1: the map is twice as bright. | 8 by 6, largest difference 2.4e-7; says [] | yes |
| FX-LMAP-006: `card_off_fx`, the same Exposure switched off: FX-LMAP-001's map. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-007: `card_soft`, with Gaussian Blur sigma 0.5: blurred, and cut back to the card's own 5 by 3, so none of the blur's spread shows outside it. | 8 by 6, largest difference 1.8e-7; says [] | yes |
| FX-LMAP-008: `self` names itself: its drawing through its mask, and not its Exposure, fitted to its own 5 by 3. | 5 by 3, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-009: `flip` shows the card on frame 0 and its second drawing from frame 1: the map follows the drawing exposed. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-010: `flip_late` starts on frame 1: on frame 0 the map is empty, and nothing is said. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-011: `flip_late` on frame 1 shows its first drawing, by its own timing. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-012: `flip_ahead`, its source one frame ahead: on frame 0 it shows the second drawing. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-013: `solid`, a solid 4 by 2: centred at (2, 2). | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-014: `shape`, a box from (1, 1) to (5, 4): a shape layer's picture is the composition's size, so the map is the box where it is drawn. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-015: `nest`, a composition layer: the inner composition's frame, 8 by 4, centred. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-016: `nest_bright`, the same with Exposure +1: brighter. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-017: `nest` on frame 2, past the inner composition's two frames: empty, nothing said. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-018: `nest_gone` shows a composition not in the project: empty, and `COMPOSITION_REFERENCE_MISSING`. | 8 by 6, largest difference 0.0e0; says ["COMPOSITION_REFERENCE_MISSING"] | yes |
| FX-LMAP-019: `rig`, a null, with tile: empty, nothing said. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-020: `adjust`, an adjustment layer: white through its mask over columns 0 to 3, its Exposure not run. | 8 by 6, largest difference 0.0e0; says [] | yes |
| FX-LMAP-021: `gone_file`, a drawing whose file is missing: empty, and `MEDIA_MISSING`. | 8 by 6, largest difference 0.0e0; says ["MEDIA_MISSING"] | yes |
| FX-LMAP-022: Centre, `big` 10 by 8 into 8 by 6: dx = dy = -1, the middle shows. | 8 by 6, largest difference 1.3e-7; says [] | yes |
| FX-LMAP-023: Tile, `card` into 8 by 6: from (1, 1), repeating every 5 across and 3 down. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-024: Tile, `big` into 8 by 6: a picture bigger than the holder tiles to FX-LMAP-022's centre. | 8 by 6, largest difference 1.3e-7; says [] | yes |
| FX-LMAP-025: Stretch, `card` into 8 by 6: bilinear, held at the card's edges. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-026: Stretch, `big` into 8 by 6. | 8 by 6, largest difference 1.9e-7; says [] | yes |
| FX-LMAP-027: Stretch, `card` into `holder_card`, a solid 5 by 3: the card exactly. | 5 by 3, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-028: Centre, `card` into `holder_small`, 3 by 2: dx = -1 and dy = floor(-1/2) = -1. | 3 by 2, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-029: Tile, `card` into `holder_small`: ((x + 1) mod 5, (y + 1) mod 3). | 3 by 2, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-030: Stretch, `card_masked` into 8 by 6: premultiplied, so the clear columns fade the covering and never tint the colour. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-031: Draft, `stripes_fx`, a drawing with an effect, so its effects run at a quarter (D-99) on 2 by 2: `card_draft` is taken down to 2 by 1, its mask (0, 0)-(4, 3) becomes (0, 0)-(1, 0.75), then Exposure +1, then Gaussian Blur sigma 2 / 4. | 2 by 2, largest difference 4.2e-8; says [] | yes |
| FX-LMAP-032: Draft, `holder`, a solid, whose effects run at full size: FX-LMAP-001's map exactly. | 8 by 6, largest difference 1.2e-7; says [] | yes |
| FX-LMAP-033: Draft, `nest` holding: its picture is 2 by 1 at Draft. `nest_bright`'s inner frame is drawn at a quarter, then its Exposure. | 2 by 1, largest difference 0.0e0; says [] | yes |
| FX-LMAP-034: Draft, `adjust` holding: its effects run on the 2 by 2 frame. `big` is taken down to 3 by 2 and stretched. | 2 by 2, largest difference 5.6e-8; says [] | yes |
| FX-LMAP-040: A name that is no layer: no map, and `EFFECT_LAYER_MISSING`. | no map; says ["EFFECT_LAYER_MISSING"] | yes |
| FX-LMAP-041: `dot`, a layer of another composition: no map, and `EFFECT_LAYER_MISSING`. | no map; says ["EFFECT_LAYER_MISSING"] | yes |
| FX-LMAP-042: An empty name: no map, nothing said. | no map; says [] | yes |

## Result

37 of 37 checks pass.
