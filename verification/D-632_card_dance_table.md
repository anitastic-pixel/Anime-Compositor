# D-632: Card Dance

B-512, after After Effects' Card Dance (Simulation), under the coordinator's approval of the D-630 design for the owner's /loop request. The layer cut into Rows and Columns of cards, each moved, turned and scaled by the red, green, blue, intensity or alpha of one of two Gradient Layers (or the layer itself) under its centre, times a Multiplier plus an Offset, in the Rotation Order and the Transformation Order; the backs from a Back Layer; seen by the effect's own camera or the composition camera and lit by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units and every setting when added are ours, stated in `tools/card_dance_reference.py`. Every expected pixel is `Fixtures/card_dance/expected_card_dance.json`, written by the tool before this code existed and printed in document 25 as FX-CD-001 to 064. Tolerance 2e-5.

## FX-CD-001 to 064 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CD-001 frame 0: As added: eight by eight cards, each moved toward the viewer by the brightness of the layer's own picture under its centre (white one card height nearer, black one farther). | largest difference 2.1e-7 | yes |
| FX-CD-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-002 frame 0: Four by two cards with every source none: the picture unchanged. | largest difference 1.5e-7 | yes |
| FX-CD-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-003 frame 0: Z position from intensity 1 of the gradient layer `ramp`, multiplier 0.4: the brighter (lower left) cards nearer and larger. | largest difference 2.1e-7 | yes |
| FX-CD-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-004 frame 0: X position offset 0.25: every card a quarter of a card to the right. | largest difference 1.5e-7 | yes |
| FX-CD-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-005 frame 0: Y position from green 1 of `colours`, multiplier 0.3: cards lower where the gradient is greener. | largest difference 1.8e-7 | yes |
| FX-CD-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-006 frame 0: X rotation offset 180 with the back layer: every card turned over top to bottom, showing its share of the back mirrored across. | largest difference 9.7e-8 | yes |
| FX-CD-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-007 frame 0: Y rotation offset 180 with the back layer: the back picture whole, upright, its white marker at the top left. | largest difference 9.7e-8 | yes |
| FX-CD-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-008 frame 0: Y rotation offset 180 without a back layer: every card's back clear. | largest difference 0.0e0 | yes |
| FX-CD-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-009 frame 0: Z rotation from red 1 of `colours`, multiplier 60: cards turned in the picture's plane, more to the right. | largest difference 1.7e-7 | yes |
| FX-CD-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-010 frame 0: X scale offset 0.5: half-width cards with gaps between them. | largest difference 1.5e-7 | yes |
| FX-CD-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-011 frame 0: Y scale from blue 2 of `colours` as gradient layer 2, multiplier 0.4: taller cards to the left. | largest difference 1.9e-7 | yes |
| FX-CD-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-012 frame 0: X scale from alpha 1 of `colours`, multiplier 0.5, offset 0.5: narrower cards where the gradient is clearer (lower). | largest difference 1.9e-7 | yes |
| FX-CD-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-013 frame 0: Z position from intensity 2 with gradient layer 2 `ramp`. | largest difference 2.1e-7 | yes |
| FX-CD-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-014 frame 0: Three turns, rotation order xyz, with the back layer. | largest difference 1.4e-7 | yes |
| FX-CD-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-015 frame 0: FX-CD-014 with rotation order xzy. | largest difference 1.4e-7 | yes |
| FX-CD-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-016 frame 0: FX-CD-014 with rotation order yxz. | largest difference 1.2e-7 | yes |
| FX-CD-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-017 frame 0: FX-CD-014 with rotation order yzx. | largest difference 1.2e-7 | yes |
| FX-CD-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-018 frame 0: FX-CD-014 with rotation order zxy. | largest difference 1.2e-7 | yes |
| FX-CD-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-019 frame 0: FX-CD-014 with rotation order zyx. | largest difference 1.3e-7 | yes |
| FX-CD-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-020 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order scale, rotate, position. | largest difference 2.2e-7 | yes |
| FX-CD-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-021 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order scale, position, rotate. | largest difference 1.4e-7 | yes |
| FX-CD-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-022 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order rotate, scale, position. | largest difference 1.7e-7 | yes |
| FX-CD-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-023 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order rotate, position, scale. | largest difference 1.6e-7 | yes |
| FX-CD-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-024 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order position, scale, rotate. | largest difference 1.7e-7 | yes |
| FX-CD-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-025 frame 0: Scale 0.6 by 0.8, turned 30 about z, moved 0.3 right and 0.2 up, in the order position, rotate, scale. | largest difference 1.4e-7 | yes |
| FX-CD-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-026 frame 0: Scale -1 across: each card mirrored left to right, still its front. | largest difference 1.5e-7 | yes |
| FX-CD-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-027 frame 0: Overlapping cards: scale 1.4 at the same depth, the first card in rows then columns on top. | largest difference 1.7e-7 | yes |
| FX-CD-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-028 frame 0: Z position offset 10 card heights toward the viewer: every card behind the camera, nothing drawn. | largest difference 0.0e0 | yes |
| FX-CD-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-029 frame 0: Columns follow rows, rows 3 (columns 4 ignored), with depth from `ramp`. | largest difference 1.8e-7 | yes |
| FX-CD-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-030 frame 0: The camera turned 30 degrees about y, with depth from `ramp`. | largest difference 2.9e-7 | yes |
| FX-CD-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-031 frame 0: The camera turned -20 about x and 15 about z. | largest difference 2.1e-7 | yes |
| FX-CD-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-032 frame 0: The camera at x 40, y 60 per cent. | largest difference 1.7e-7 | yes |
| FX-CD-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-033 frame 0: Camera Z position 1.7 and focal length 20, turned 25 about y. | largest difference 1.3e-7 | yes |
| FX-CD-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-034 frame 0: Comp Camera with no camera in the file: D-58's default camera, the same as the effect's own camera as added; the effect's camera turns are not used. | largest difference 2.1e-7 | yes |
| FX-CD-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-035 frame 0: Comp Camera with the composition camera 12 pixels back (zoom 12): a strong perspective on the cards. | largest difference 1.7e-7 | yes |
| FX-CD-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-036 frame 0: Comp Camera with the composition camera 2 to the right: the cards seen from the right and the whole picture moved 2 left. | largest difference 2.2e-7 | yes |
| FX-CD-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-037 frame 0: FX-CD-035 on the holder moved 3 right and 1 down: the camera sees the layer from its upper left. | largest difference 1.2e-7 | yes |
| FX-CD-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-038 frame 0: Comp Camera with the composition camera on the layer's plane (depth 0, zoom 12): nothing drawn. | largest difference 0.0e0 | yes |
| FX-CD-038: what opening it warns of, and what frame 4 warns of | [] and ["CAMERA_PLANE_BEHIND"] | yes |
| FX-CD-039 frame 0: A distant light from the top left, depth 0.5, on turned cards. | largest difference 7.4e-8 | yes |
| FX-CD-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-040 frame 0: A point light in the middle, depth 0.3. | largest difference 8.0e-8 | yes |
| FX-CD-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-041 frame 0: Specular 1, sharpness 20: a highlight on the turned cards. | largest difference 1.2e-7 | yes |
| FX-CD-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-042 frame 0: Light colour #ffcc88 at intensity 1.5, ambient 0.5, diffuse 0.25. | largest difference 1.1e-7 | yes |
| FX-CD-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-043 frame 0: Y rotation offset keyed 0 at frame 0 to 180 at frame 4, with the back layer: frame 0 the picture, frame 4 FX-CD-007. | largest difference 1.5e-7 | yes |
| FX-CD-043 frame 2: Y rotation offset keyed 0 at frame 0 to 180 at frame 4, with the back layer: frame 0 the picture, frame 4 FX-CD-007. | largest difference 2.5e-8 | yes |
| FX-CD-043 frame 4: Y rotation offset keyed 0 at frame 0 to 180 at frame 4, with the back layer: frame 0 the picture, frame 4 FX-CD-007. | largest difference 9.7e-8 | yes |
| FX-CD-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-044 frame 0: FX-CD-003 on the holder moved 3 right and 1 down: the same, moved. | largest difference 2.1e-7 | yes |
| FX-CD-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-045 frame 0: A gradient layer not in the composition, `gone`: read as clear, so every card at -1 times the multiplier, 0.4 farther; and the warning every frame. | largest difference 1.6e-7 | yes |
| FX-CD-045 frame 4: A gradient layer not in the composition, `gone`: read as clear, so every card at -1 times the multiplier, 0.4 farther; and the warning every frame. | largest difference 1.6e-7 | yes |
| FX-CD-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CD-046 frame 0: The back layer with its in point at frame 4: clear backs before it, the back at it. | largest difference 0.0e0 | yes |
| FX-CD-046 frame 4: The back layer with its in point at frame 4: clear backs before it, the back at it. | largest difference 9.7e-8 | yes |
| FX-CD-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-047 frame 0: A back layer not in the composition, `gone`: clear backs, and the warning. | largest difference 0.0e0 | yes |
| FX-CD-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CD-048 frame 0: Sixteen columns of one row, each turned by red 1 of `colours` about y with the back layer: blinds opening across. | largest difference 1.9e-7 | yes |
| FX-CD-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CD-049 frame 0: Rotation order "xxy". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-049 frame 4: Rotation order "xxy". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-050 frame 0: Transformation order "scale". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-050 frame 4: Transformation order "scale". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-051 frame 0: X position source "purple_1". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-051 frame 4: X position source "purple_1". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-052 frame 0: Camera system "corner_pins", not built yet. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-052 frame 4: Camera system "corner_pins", not built yet. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-053 frame 0: Rows 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-053 frame 4: Rows 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-054 frame 0: Columns 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-054 frame 4: Columns 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-055 frame 0: Z position multiplier 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-055 frame 4: Z position multiplier 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-056 frame 0: X scale offset -11, below -10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-056 frame 4: X scale offset -11, below -10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-057 frame 0: Y rotation offset 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-057 frame 4: Y rotation offset 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-058 frame 0: Camera Z position 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-058 frame 4: Camera Z position 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-059 frame 0: Focal length 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-059 frame 4: Focal length 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-060 frame 0: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-060 frame 4: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-061 frame 0: Light colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-061 frame 4: Light colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-062 frame 0: Gradient layer 2 written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-062 frame 4: Gradient layer 2 written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-063 frame 0: Rows & Columns "both". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-063 frame 4: Rows & Columns "both". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-063: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CD-064 frame 0: X position offset keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-064 frame 4: X position offset keyed to 120 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-CD-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| Card Dance never grows the layer's bounds | 0 | yes |
| a half-size draft preview changes no setting, each a share of the layer, a count or a word (the composition camera, worked out each frame, is halved) | CardDance { rows_columns: "independent", rows: 8.0, columns: 8.0, back_layer: String(""), back_map: None, gradient_layer_1: String(""), gradient_map_1: None, gradient_layer_2: String(""), gradient_map_2: None, rotation_order: "xyz", transformation_order: "scale_rotate_position", sources: ["none", "none", "intensity_1", "none", "none", "none", "none", "none"], multipliers: [1.0, 1.0, 1.0, 90.0, 90.0, 90.0, 0.5, 0.5], offsets: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0], camera_system: "camera_position", camera_x_rotation: 0.0, camera_y_rotation: 0.0, camera_z_rotation: 0.0, camera_xy_position: [50.0, 50.0], camera_z_position: 1.0, focal_length: 50.0, light_type: "distant", light_intensity: 1.0, light_color: "#ffffff", light_position: [50.0, 50.0], light_depth: 1.0, ambient_light: 0.25, diffuse_reflection: 0.75, specular_reflection: 0.0, highlight_sharpness: 5.0, view: [0.0, 0.0, 0.0] } | yes |
| fx_cd_049.json is refused in a sentence naming rotation order | Card Dance's rotation order is one of "xyz", "xzy", "yxz", "yzx", "zxy", "zyx", and this is "xxy". | yes |
| fx_cd_050.json is refused in a sentence naming transformation order | Card Dance's transformation order is one of "scale_rotate_position", "scale_position_rotate", "rotate_scale_position", "rotate_position_scale", "position_scale_rotate", "position_rotate_scale", and this is "scale". | yes |
| fx_cd_051.json is refused in a sentence naming x position source | Card Dance's x position source is one of "none", "red_1", "green_1", "blue_1", "intensity_1", "alpha_1", "red_2", "green_2", "blue_2", "intensity_2", "alpha_2", and this is "purple_1". | yes |
| fx_cd_052.json is refused in a sentence naming camera system | Card Dance's camera system is one of "camera_position", "comp_camera", and this is "corner_pins". | yes |
| fx_cd_053.json is refused in a sentence naming rows | Card Dance's rows runs from 1 to 1000, and this is 0. | yes |
| fx_cd_054.json is refused in a sentence naming columns | Card Dance's columns runs from 1 to 1000, and this is 1001. | yes |
| fx_cd_055.json is refused in a sentence naming z position multiplier | Card Dance's z position multiplier runs from -100 to 100, and this is 101. | yes |
| fx_cd_056.json is refused in a sentence naming x scale offset | Card Dance's x scale offset runs from -10 to 10, and this is -11. | yes |
| fx_cd_057.json is refused in a sentence naming y rotation offset | Card Dance's y rotation offset runs from -3600 to 3600, and this is 3601. | yes |
| fx_cd_058.json is refused in a sentence naming camera z position | Card Dance's camera z position runs from 0.01 to 10, and this is 0. | yes |
| fx_cd_059.json is refused in a sentence naming focal length | Card Dance's focal length runs from 1 to 1000, and this is 0. | yes |
| fx_cd_060.json is refused in a sentence naming light type | Card Dance's light type is one of "distant", "point", and this is "spot". | yes |
| fx_cd_061.json is refused in a sentence naming light colour | Card Dance's light colour is written #rrggbb, and this is "white". | yes |
| fx_cd_062.json is refused in a sentence naming gradient layer 2 | Card Dance's gradient layer 2 is the name of a layer of this composition, and this is 3. | yes |
| fx_cd_063.json is refused in a sentence naming rows & columns | Card Dance's rows & columns is one of "independent", "columns_follow_rows", and this is "both". | yes |
| fx_cd_064.json is refused in a sentence naming x position offset | The layer "holder" has a core.card_dance whose settings this build cannot use. Card Dance's x position offset runs from -100 to 100, and this is 120. The effect is kept and bypassed. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_cd_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_cd_040.json is saved with its forty-eight settings, the back `back`, a point light, X Position Source "none", Camera X, Y Position [50, 50] | {"ambient_light":0.25,"back_layer":"back","camera_system":"camera_position","camera_x_rotation":0,"camera_xy_position":[50,50],"camera_y_rotation":0,"camera_z_position":1,"camera_z_rotation":0,"columns":4,"diffuse_reflection":0.75,"focal_length":50,"gradient_layer_1":"","gradient_layer_2":"","highlight_sharpness":5,"light_color":"#ffffff","light_depth":0.3,"light_intensity":1,"light_position":[50,50],"light_type":"point","rotation_order":"xyz","rows":2,"rows_columns":"independent","specular_reflection":0,"transformation_order":"scale_rotate_position","x_position_multiplier":1,"x_position_offset":0,"x_position_source":"none","x_rotation_multiplier":90,"x_rotation_offset":40,"x_rotation_source":"none","x_scale_multiplier":0.5,"x_scale_offset":1,"x_scale_source":"none","y_position_multiplier":1,"y_position_offset":0,"y_position_source":"none","y_rotation_multiplier":90,"y_rotation_offset":-25,"y_rotation_source":"none","y_scale_multiplier":0.5,"y_scale_offset":1,"y_scale_source":"none","z_position_multiplier":1,"z_position_offset":0,"z_position_source":"none","z_rotation_multiplier":90,"z_rotation_offset":30,"z_rotation_source":"none"} | yes |
| the three pictures the effect is drawn with, and the composition camera it is given, are never saved | None None None None | yes |
| a file with a Card Dance whose rows is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Dance whose camera X, Y Position is a number, not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Dance with no X Scale Source is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Card Dance with no gradient layer 2 is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Pictures (verification/D-632 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, 1008 distinct colours | 1008 colours; 3931 clear pixels between and around the cards; warnings [] | yes |
| 2_tilted_cards_backs.png: drawn without a warning, 2355 distinct colours | 2355 colours; 41165 clear pixels between and around the cards; warnings [] | yes |
| 3_turned_by_the_ramp.png: drawn without a warning, 3274 distinct colours | 3274 colours; 33505 clear pixels between and around the cards; warnings [] | yes |
| 4_scattered_by_colours.png: drawn without a warning, 840 distinct colours | 840 colours; 57367 clear pixels between and around the cards; warnings [] | yes |
| 5_seen_from_the_side.png: drawn without a warning, 1103 distinct colours | 1103 colours; 87304 clear pixels between and around the cards; warnings [] | yes |
| 6_composition_camera.png: drawn without a warning, 1156 distinct colours | 1156 colours; 6629 clear pixels between and around the cards; warnings [] | yes |
| 7_point_light_shine.png: drawn without a warning, 10913 distinct colours | 10913 colours; 25051 clear pixels between and around the cards; warnings [] | yes |

## Result

243 of 243 checks pass.
