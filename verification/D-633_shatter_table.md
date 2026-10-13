# D-633: Shatter

B-513, after After Effects' Shatter (Simulation), under the coordinator's approval of the D-630 design for the owner's /loop request. The layer broken into pieces of one of ten patterns, turned by Direction about the Origin, or of a Custom Shatter Map; the pieces a force sphere reaches freed (a Gradient Layer and its Shatter Threshold allowing), thrown from the force's centre with Randomness and Mass Variance, slowed by Viscosity, pulled by Gravity, tumbling about the Tumble Axis, walked four steps a frame from the layer's in point; drawn as slabs with front, side and back textures in six modes, seen by the effect's own camera or the composition camera and lit by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units and most settings when added are ours, stated in `tools/shatter_reference.py`. Every expected pixel is `Fixtures/shatter/expected_shatter.json`, written by the tool before this code existed and printed in document 25 as FX-SH-001 to 078. Tolerance 2e-5.

## FX-SH-001 to 078 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SH-001 frame 0: As added: bricks ten across, a force in the middle; at frame 0 nothing has moved yet, by frame 2 the middle pieces fly and fall. | largest difference 1.5e-7 | yes |
| FX-SH-001 frame 2: As added: bricks ten across, a force in the middle; at frame 0 nothing has moved yet, by frame 2 the middle pieces fly and fall. | largest difference 1.5e-7 | yes |
| FX-SH-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-002 frame 0: Force 1 radius 0: nothing freed, the picture unchanged. | largest difference 1.5e-7 | yes |
| FX-SH-002 frame 4: Force 1 radius 0: nothing freed, the picture unchanged. | largest difference 1.5e-7 | yes |
| FX-SH-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-003 frame 3: Pattern bricks, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.6e-7 | yes |
| FX-SH-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-004 frame 3: Pattern squares, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.4e-7 | yes |
| FX-SH-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-005 frame 3: Pattern hexagons, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.7e-7 | yes |
| FX-SH-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-006 frame 3: Pattern triangles 1, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.8e-7 | yes |
| FX-SH-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-007 frame 3: Pattern triangles 2, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.3e-7 | yes |
| FX-SH-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-008 frame 3: Pattern rhombus, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.8e-7 | yes |
| FX-SH-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-009 frame 3: Pattern planks, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.9e-7 | yes |
| FX-SH-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-010 frame 3: Pattern herringbone 1, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.8e-7 | yes |
| FX-SH-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-011 frame 3: Pattern octagons squares, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 1.9e-7 | yes |
| FX-SH-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-012 frame 3: Pattern glass, three across, every piece freed in place and turning in the picture's plane at its own speed: the pieces' shapes. | largest difference 2.0e-7 | yes |
| FX-SH-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-013 frame 3: Bricks turned 30 degrees about an origin at 20, 40 per cent. | largest difference 1.6e-7 | yes |
| FX-SH-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-014 frame 3: A custom map `cracks`: each region of one colour a piece, the clear corner no piece at all. | largest difference 1.5e-7 | yes |
| FX-SH-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-015 frame 3: FX-SH-014 with White Tiles Fixed: the white pieces stay. | largest difference 1.5e-7 | yes |
| FX-SH-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-016 frame 3: Custom with no map named: the layer's own drawing makes the pieces. | largest difference 1.5e-7 | yes |
| FX-SH-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-017 frame 0: A force at 25, 50 per cent, radius 0.2, strength 1: only the left pieces fly. | largest difference 1.5e-7 | yes |
| FX-SH-017 frame 3: A force at 25, 50 per cent, radius 0.2, strength 1: only the left pieces fly. | largest difference 1.5e-7 | yes |
| FX-SH-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-018 frame 3: Force 2 at 80, 30 per cent, radius 0.15, strength -1, pulling its pieces in and through. | largest difference 1.5e-7 | yes |
| FX-SH-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-019 frame 2: Force 1 strength -2: the pieces drawn toward the force and past it. | largest difference 1.9e-7 | yes |
| FX-SH-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-020 frame 3: Gradient layer `ramp`, threshold 50: only the pieces over the lighter half of the ramp freed. | largest difference 1.8e-7 | yes |
| FX-SH-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-021 frame 3: FX-SH-020 with the gradient inverted: the darker half freed. | largest difference 1.5e-7 | yes |
| FX-SH-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-022 frame 0: Force 1 swept from the left edge to the right over frames 0 to 7, radius 0.15: pieces freed one after another, each frame drawn alone. | largest difference 1.5e-7 | yes |
| FX-SH-022 frame 3: Force 1 swept from the left edge to the right over frames 0 to 7, radius 0.15: pieces freed one after another, each frame drawn alone. | largest difference 1.5e-7 | yes |
| FX-SH-022 frame 7: Force 1 swept from the left edge to the right over frames 0 to 7, radius 0.15: pieces freed one after another, each frame drawn alone. | largest difference 2.0e-7 | yes |
| FX-SH-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-023 frame 0: Threshold keyed 0 to 100 over frames 0 to 7 with gradient `ramp`: the lighter pieces go first. | largest difference 1.5e-7 | yes |
| FX-SH-023 frame 3: Threshold keyed 0 to 100 over frames 0 to 7 with gradient `ramp`: the lighter pieces go first. | largest difference 1.7e-7 | yes |
| FX-SH-023 frame 7: Threshold keyed 0 to 100 over frames 0 to 7 with gradient `ramp`: the lighter pieces go first. | largest difference 1.8e-7 | yes |
| FX-SH-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-024 frame 3: Gravity 2 alone, every piece freed: the pieces fall. | largest difference 1.9e-7 | yes |
| FX-SH-024 frame 6: Gravity 2 alone, every piece freed: the pieces fall. | largest difference 1.5e-7 | yes |
| FX-SH-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-025 frame 4: Gravity 2 to the right (direction 90). | largest difference 1.7e-7 | yes |
| FX-SH-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-026 frame 4: Gravity 2 inclined 45 degrees toward the viewer: falling and nearing. | largest difference 1.7e-7 | yes |
| FX-SH-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-027 frame 4: Viscosity 4 with strength 2: thrown hard and soon slowed. | largest difference 1.5e-7 | yes |
| FX-SH-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-028 frame 4: Viscosity 0 with strength 1: never slowed. | largest difference 1.5e-7 | yes |
| FX-SH-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-029 frame 5: Gravity keyed 0 at frame 0 to 4 at frame 5, stepped four times a frame. | largest difference 1.8e-7 | yes |
| FX-SH-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-030 frame 3: Tumble axis free, every piece freed in place and turning with the back layer `back`. | largest difference 1.9e-7 | yes |
| FX-SH-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-031 frame 3: Tumble axis none, every piece freed in place and turning with the back layer `back`. | largest difference 1.8e-7 | yes |
| FX-SH-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-032 frame 3: Tumble axis x, every piece freed in place and turning with the back layer `back`. | largest difference 1.9e-7 | yes |
| FX-SH-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-033 frame 3: Tumble axis y, every piece freed in place and turning with the back layer `back`. | largest difference 1.8e-7 | yes |
| FX-SH-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-034 frame 3: Tumble axis z, every piece freed in place and turning with the back layer `back`. | largest difference 2.0e-7 | yes |
| FX-SH-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-035 frame 3: Tumble axis xy, every piece freed in place and turning with the back layer `back`. | largest difference 1.8e-7 | yes |
| FX-SH-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-036 frame 3: Tumble axis xz, every piece freed in place and turning with the back layer `back`. | largest difference 1.7e-7 | yes |
| FX-SH-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-037 frame 3: Tumble axis yz, every piece freed in place and turning with the back layer `back`. | largest difference 1.8e-7 | yes |
| FX-SH-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-038 frame 3: Randomness 0: every piece the same speed and spin. | largest difference 1.8e-7 | yes |
| FX-SH-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-039 frame 3: Mass variance 100: speeds from two thirds to twice. | largest difference 1.6e-7 | yes |
| FX-SH-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-040 frame 3: Render Layer: only the pieces not freed, holes where the others were. | largest difference 1.6e-7 | yes |
| FX-SH-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-041 frame 3: Render Pieces: only the freed pieces. | largest difference 1.7e-7 | yes |
| FX-SH-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-042 frame 3: Front mode Color, colour #ffcc88: the freed fronts flat orange, lit. | largest difference 6.0e-8 | yes |
| FX-SH-042: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-043 frame 3: Front mode Tinted Layer, colour #88ccff. | largest difference 1.5e-7 | yes |
| FX-SH-043: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-044 frame 3: Side mode Color, colour #ff4040, extrusion 0.3: red sides. | largest difference 1.8e-7 | yes |
| FX-SH-044: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-045 frame 3: Side mode Layer with the side layer `colours`. | largest difference 1.8e-7 | yes |
| FX-SH-045: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-046 frame 3: Back mode Color + Opacity, opacity 0.5. | largest difference 1.8e-7 | yes |
| FX-SH-046: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-047 frame 3: Front mode Layer + Opacity 0.5 with the front layer `colours`. | largest difference 8.2e-8 | yes |
| FX-SH-047: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-048 frame 3: Front mode Tinted Layer + Opacity, opacity 0.7. | largest difference 1.5e-7 | yes |
| FX-SH-048: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-049 frame 3: Extrusion 0: flat pieces, no sides. | largest difference 1.8e-7 | yes |
| FX-SH-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-050 frame 3: Extrusion 0.5: thick pieces. | largest difference 1.8e-7 | yes |
| FX-SH-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-051 frame 0: At rest, the camera turned 30 degrees about y: the slabs' sides show at the picture's edge and the back shows nothing. | largest difference 1.6e-7 | yes |
| FX-SH-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-052 frame 3: Comp Camera with no camera in the file: D-58's default camera, as the effect's own as added. | largest difference 1.8e-7 | yes |
| FX-SH-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-053 frame 3: Comp Camera 12 pixels back (zoom 12): a strong perspective on the flight. | largest difference 1.8e-7 | yes |
| FX-SH-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-054 frame 3: FX-SH-053 on the holder moved 3 right and 1 down. | largest difference 1.2e-7 | yes |
| FX-SH-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-055 frame 3: A point light at the top left, depth 0.3, specular 1, sharpness 10. | largest difference 1.8e-7 | yes |
| FX-SH-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-056 frame 3: Light colour #ffcc88, intensity 1.5, ambient 0.5, diffuse 0.25. | largest difference 1.9e-7 | yes |
| FX-SH-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-057 frame 3: A gradient layer not in the composition, `gone`: read as clear, every value 0, nothing passes threshold 50; and the warning. | largest difference 1.5e-7 | yes |
| FX-SH-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-SH-058 frame 3: The back layer with its in point at frame 4: clear backs before it. | largest difference 1.8e-7 | yes |
| FX-SH-058 frame 4: The back layer with its in point at frame 4: clear backs before it. | largest difference 2.4e-7 | yes |
| FX-SH-058: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-059 frame 2: The holder starting at frame 2: time counts from its in point, so frame 2 is at rest and frame 5 is FX-SH-017's frame 3. | largest difference 1.5e-7 | yes |
| FX-SH-059 frame 5: The holder starting at frame 2: time counts from its in point, so frame 2 is at rest and frame 5 is FX-SH-017's frame 3. | largest difference 1.5e-7 | yes |
| FX-SH-059: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-060 frame 6: As added at frame 6. | largest difference 1.5e-7 | yes |
| FX-SH-060: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-061 frame 0: Pattern puzzle, as added (D-636): puzzle pieces ten across; at frame 0 nothing has moved yet, and by frame 4 the middle pieces have flown. | largest difference 1.5e-7 | yes |
| FX-SH-061 frame 4: Pattern puzzle, as added (D-636): puzzle pieces ten across; at frame 0 nothing has moved yet, and by frame 4 the middle pieces have flown. | largest difference 1.5e-7 | yes |
| FX-SH-061: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-062 frame 0: Pattern "lace". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-062 frame 4: Pattern "lace". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-063 frame 0: View Wireframe Front, as added (D-635): the bricks' outlines seen straight on; at rest at frame 0 every pixel lies on an outline (bricks ten across a layer 16 wide), and by frame 4 the middle pieces have flown. | largest difference 0.0e0 | yes |
| FX-SH-063 frame 4: View Wireframe Front, as added (D-635): the bricks' outlines seen straight on; at rest at frame 0 every pixel lies on an outline (bricks ten across a layer 16 wide), and by frame 4 the middle pieces have flown. | largest difference 0.0e0 | yes |
| FX-SH-063: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-064 frame 0: Render "most". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-064 frame 4: Render "most". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-064: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-065 frame 0: Tumble axis "w". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-065 frame 4: Tumble axis "w". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-065: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-066 frame 0: Front mode "glow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-066 frame 4: Front mode "glow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-066: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-067 frame 0: Camera system "corner_pins" with the pins as added (D-634): the camera as added, so FX-SH-001's picture. | largest difference 1.5e-7 | yes |
| FX-SH-067 frame 4: Camera system "corner_pins" with the pins as added (D-634): the camera as added, so FX-SH-001's picture. | largest difference 1.5e-7 | yes |
| FX-SH-067: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-068 frame 0: Repetitions 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-068 frame 4: Repetitions 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-068: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-069 frame 0: Extrusion depth 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-069 frame 4: Extrusion depth 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-069: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-070 frame 0: Force 1 radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-070 frame 4: Force 1 radius -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-070: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-071 frame 0: Shatter threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-071 frame 4: Shatter threshold 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-071: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-072 frame 0: Gravity inclination 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-072 frame 4: Gravity inclination 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-072: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-073 frame 0: Colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-073 frame 4: Colour "white": not a #rrggbb colour. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-073: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-074 frame 0: Custom shatter map written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-074 frame 4: Custom shatter map written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-074: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-075 frame 0: White tiles fixed "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-075 frame 4: White tiles fixed "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-075: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-076 frame 0: Gravity keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-076 frame 4: Gravity keyed to 200 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-076: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-077 frame 0: Invert gradient "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-077 frame 4: Invert gradient "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-077: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-SH-078 frame 0: Mass variance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-078 frame 4: Mass variance 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-078: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| Shatter never grows the layer's bounds | 0 | yes |
| a half-size draft preview changes no setting, each a share of the layer, a count or a word (the composition camera, worked out each frame, is halved) | Shatter { view: "rendered", render: "all", pattern: "bricks", custom_shatter_map: String(""), custom_map: None, white_tiles_fixed: "off", repetitions: 10.0, direction: 0.0, origin: [50.0, 50.0], extrusion_depth: 0.2, force_1_position: [50.0, 50.0], force_1_depth: 0.1, force_1_radius: 0.4, force_1_strength: 5.0, force_2_position: [50.0, 50.0], force_2_depth: 0.1, force_2_radius: 0.0, force_2_strength: 5.0, gradient_layer: String(""), gradient_map: None, shatter_threshold: 0.0, invert_gradient: "off", rotation_speed: 0.2, tumble_axis: "free", randomness: 0.1, viscosity: 0.1, mass_variance: 30.0, gravity: 3.0, gravity_direction: 180.0, gravity_inclination: 0.0, color: "#ffffff", opacity: 1.0, front_mode: "layer", side_mode: "tinted_layer", back_mode: "layer", front_layer: String(""), front_map: None, side_layer: String(""), side_map: None, back_layer: String(""), back_map: None, camera_system: "camera_position", camera_x_rotation: 0.0, camera_y_rotation: 0.0, camera_z_rotation: 0.0, camera_xy_position: [50.0, 50.0], camera_z_position: 1.0, focal_length: 50.0, light_type: "distant", light_intensity: 1.0, light_color: "#ffffff", light_position: [50.0, 50.0], light_depth: 1.0, ambient_light: 0.25, diffuse_reflection: 0.75, specular_reflection: 0.0, upper_left_corner: [0.0, 0.0], upper_right_corner: [100.0, 0.0], lower_left_corner: [0.0, 100.0], lower_right_corner: [100.0, 100.0], auto_focal_length: "off", highlight_sharpness: 5.0, seen: [0.0, 0.0, 0.0], history: [], fps: 0.0 } | yes |
| fx_sh_062.json is refused in a sentence naming pattern | Shatter's pattern is one of "bricks", "squares", "hexagons", "triangles_1", "triangles_2", "rhombus", "planks", "herringbone_1", "octagons_squares", "glass", "custom", "herringbone_2", "squares_triangles", "stars_triangles", "overlapping_squares", "chevrons", "puzzle", "js", "eggs", "crescents", "carpenters_wheel", and this is "lace". | yes |
| fx_sh_064.json is refused in a sentence naming render | Shatter's render is one of "all", "layer", "pieces", and this is "most". | yes |
| fx_sh_065.json is refused in a sentence naming tumble axis | Shatter's tumble axis is one of "free", "none", "x", "y", "z", "xy", "xz", "yz", and this is "w". | yes |
| fx_sh_066.json is refused in a sentence naming front mode | Shatter's front mode is one of "color", "layer", "tinted_layer", "color_plus_opacity", "layer_plus_opacity", "tinted_layer_plus_opacity", and this is "glow". | yes |
| fx_sh_068.json is refused in a sentence naming repetitions | Shatter's repetitions runs from 1 to 1000, and this is 0. | yes |
| fx_sh_069.json is refused in a sentence naming extrusion depth | Shatter's extrusion depth runs from 0 to 1, and this is 2. | yes |
| fx_sh_070.json is refused in a sentence naming force 1 radius | Shatter's force 1 radius runs from 0 to 10, and this is -1. | yes |
| fx_sh_071.json is refused in a sentence naming shatter threshold | Shatter's shatter threshold runs from 0 to 100, and this is 101. | yes |
| fx_sh_072.json is refused in a sentence naming gravity inclination | Shatter's gravity inclination runs from -90 to 90, and this is 91. | yes |
| fx_sh_073.json is refused in a sentence naming colour | Shatter's colour is written #rrggbb, and this is "white". | yes |
| fx_sh_074.json is refused in a sentence naming custom shatter map | Shatter's custom shatter map is the name of a layer of this composition, and this is 3. | yes |
| fx_sh_075.json is refused in a sentence naming white tiles fixed | Shatter's white tiles fixed is one of "off", "on", and this is "yes". | yes |
| fx_sh_077.json is refused in a sentence naming invert gradient | Shatter's invert gradient is one of "off", "on", and this is "maybe". | yes |
| fx_sh_078.json is refused in a sentence naming mass variance | Shatter's mass variance runs from 0 to 100, and this is 101. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_sh_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_063.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_064.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_065.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_066.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_067.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_068.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_069.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_070.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_071.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_072.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_073.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_074.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_075.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_076.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_077.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_078.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_055.json is saved with its fifty-two settings, a point light at [10, 10], Tumble Axis "y", Repetitions 4, no gradient layer | {"ambient_light":0.25,"back_layer":"","back_mode":"layer","camera_system":"camera_position","camera_x_rotation":0,"camera_xy_position":[50,50],"camera_y_rotation":0,"camera_z_position":1,"camera_z_rotation":0,"color":"#ffffff","custom_shatter_map":"","diffuse_reflection":0.75,"direction":0,"extrusion_depth":0.2,"focal_length":50,"force_1_depth":0.5,"force_1_position":[50,50],"force_1_radius":10,"force_1_strength":0.3,"force_2_depth":0.1,"force_2_position":[50,50],"force_2_radius":0,"force_2_strength":5,"front_layer":"","front_mode":"layer","gradient_layer":"","gravity":0,"gravity_direction":180,"gravity_inclination":0,"highlight_sharpness":10,"invert_gradient":"off","light_color":"#ffffff","light_depth":0.3,"light_intensity":1,"light_position":[10,10],"light_type":"point","mass_variance":30,"opacity":1,"origin":[50,50],"pattern":"bricks","randomness":0.6,"render":"all","repetitions":4,"rotation_speed":0.5,"shatter_threshold":0,"side_layer":"","side_mode":"tinted_layer","specular_reflection":1,"tumble_axis":"y","view":"rendered","viscosity":0,"white_tiles_fixed":"off"} | yes |
| the five pictures the effect is drawn with, the composition camera, and the numbers at each frame from the in point are never saved | [None, None, None, None, None, None, None, None] | yes |
| a file with a Shatter whose repetitions is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shatter whose force 1 position is a number, not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shatter with no tumble axis is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Shatter with no side layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Pictures (verification/D-633 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_2.png (frame 2): drawn without a warning, 1280 distinct colours | 1280 colours; 76875 clear pixels where pieces left; warnings [] | yes |
| 2_hexagons_burst.png (frame 3): drawn without a warning, 1966 distinct colours | 1966 colours; 31779 clear pixels where pieces left; warnings [] | yes |
| 3_glass_two_forces.png (frame 3): drawn without a warning, 703 distinct colours | 703 colours; 35729 clear pixels where pieces left; warnings [] | yes |
| 4_custom_cracks.png (frame 4): drawn without a warning, 3710 distinct colours | 3710 colours; 18066 clear pixels where pieces left; warnings [] | yes |
| 5_ramp_threshold.png (frame 4): drawn without a warning, 3554 distinct colours | 3554 colours; 10074 clear pixels where pieces left; warnings [] | yes |
| 6_red_sides_seen_from_the_side.png (frame 3): drawn without a warning, 770 distinct colours | 770 colours; 68787 clear pixels where pieces left; warnings [] | yes |
| 7_composition_camera.png (frame 3): drawn without a warning, 3311 distinct colours | 3311 colours; 25411 clear pixels where pieces left; warnings [] | yes |
| 8_point_light_shine.png (frame 3): drawn without a warning, 5650 distinct colours | 5650 colours; 83813 clear pixels where pieces left; warnings [] | yes |

## Result

292 of 292 checks pass.
