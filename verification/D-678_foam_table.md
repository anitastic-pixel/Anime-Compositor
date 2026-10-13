# D-678: Foam

B-557, after After Effects' Foam (Simulation): bubbles born in a producer from the in point, growing, drifting with the wind and turbulence, pushing and dragging one another and popping, drawn as rings in the Draft view or as textured bubbles in the Rendered view, the layer itself when the texture is Layer. Nothing is carried from frame to frame: every frame steps the bubbles from the in point with each setting as it was at each step (document 20). Adobe publishes no formula, so the rule is ours, stated in `tools/foam_reference.py`. Every expected pixel is `Fixtures/foam/expected_foam.json`, written by the tool before this code existed and printed in document 25 as FX-FOAM-001 to 061. Tolerance 2e-5.

## FX-FOAM-001 to 061 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FOAM-001 frame 0: As added: the Draft view, the red producer in the middle and thirty tiny blue rings a second drifting right; frame 0 the producer alone. | largest difference 2.0e-8 | yes |
| FX-FOAM-001 frame 12: As added: the Draft view, the red producer in the middle and thirty tiny blue rings a second drifting right; frame 0 the producer alone. | largest difference 2.9e-8 | yes |
| FX-FOAM-001 frame 24: As added: the Draft view, the red producer in the middle and thirty tiny blue rings a second drifting right; frame 0 the producer alone. | largest difference 3.0e-8 | yes |
| FX-FOAM-001 frame 35: As added: the Draft view, the red producer in the middle and thirty tiny blue rings a second drifting right; frame 0 the producer alone. | largest difference 3.0e-8 | yes |
| FX-FOAM-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-002 frame 0: A close look, Rendered: bigger bubbles from a bigger producer, fifteen a second; frame 0 clear, frame 1 the first born, too small to see. | largest difference 0.0e0 | yes |
| FX-FOAM-002 frame 1: A close look, Rendered: bigger bubbles from a bigger producer, fifteen a second; frame 0 clear, frame 1 the first born, too small to see. | largest difference 3.4e-9 | yes |
| FX-FOAM-002 frame 12: A close look, Rendered: bigger bubbles from a bigger producer, fifteen a second; frame 0 clear, frame 1 the first born, too small to see. | largest difference 2.2e-8 | yes |
| FX-FOAM-002 frame 24: A close look, Rendered: bigger bubbles from a bigger producer, fifteen a second; frame 0 clear, frame 1 the first born, too small to see. | largest difference 3.0e-8 | yes |
| FX-FOAM-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-003 frame 12: The close look in the Draft view: rings. | largest difference 2.9e-8 | yes |
| FX-FOAM-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-004 frame 12: The producer at 25, 75, wide and flat, turned 30 degrees. | largest difference 3.0e-8 | yes |
| FX-FOAM-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-005 frame 12: Zoom 2, the producer kept where it was put on screen: bubbles twice as big, drifting twice as fast. | largest difference 3.0e-8 | yes |
| FX-FOAM-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-006 frame 12: Zoom 2 with Zoom Producer Point on: the producer grown too. | largest difference 3.0e-8 | yes |
| FX-FOAM-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-007 frame 12: Production Rate 2: sixty a second. | largest difference 3.0e-8 | yes |
| FX-FOAM-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-008 frame 12: Size Variance 0: every bubble the same size. | largest difference 2.2e-8 | yes |
| FX-FOAM-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-009 frame 12: Size 6: big bubbles crowding and pushing. | largest difference 3.0e-8 | yes |
| FX-FOAM-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-010 frame 24: Lifespan 15 (half a second): the old ones popped. | largest difference 2.4e-8 | yes |
| FX-FOAM-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-011 frame 6: Bubble Growth Speed 1: full size in a tenth of a second. | largest difference 2.8e-8 | yes |
| FX-FOAM-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-012 frame 24: Strength 1 with Lifespan 30: most pop well before the second is out. | largest difference 3.0e-8 | yes |
| FX-FOAM-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-013 frame 12: Initial Speed 2 towards 180 degrees: thrown down. | largest difference 1.5e-8 | yes |
| FX-FOAM-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-014 frame 24: Wind 4 towards 0 degrees: blown up and away. | largest difference 3.0e-8 | yes |
| FX-FOAM-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-015 frame 24: Turbulence 5: wandering. | largest difference 2.9e-8 | yes |
| FX-FOAM-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-016 frame 12: Wobble 0.4: squashed and stretched. | largest difference 2.9e-8 | yes |
| FX-FOAM-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-017 frame 24: Repulsion 0: bubbles overlap. | largest difference 3.0e-8 | yes |
| FX-FOAM-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-018 frame 24: Repulsion 10: pushed hard apart. | largest difference 3.0e-8 | yes |
| FX-FOAM-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-019 frame 24: Pop Velocity 20 with short lives: each pop shoves its neighbours. | largest difference 1.5e-8 | yes |
| FX-FOAM-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-020 frame 24: Viscosity 1: the wind barely moves them. | largest difference 2.9e-8 | yes |
| FX-FOAM-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-021 frame 24: Stickiness 0: neighbours do not drag each other. | largest difference 2.9e-8 | yes |
| FX-FOAM-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-022 frame 24: Universe Size 0.5 with a strong wind: gone before the layer's edge. | largest difference 3.0e-8 | yes |
| FX-FOAM-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-023 frame 24: Blend Mode Solid Old on Top. | largest difference 3.0e-8 | yes |
| FX-FOAM-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-024 frame 24: Blend Mode Solid New on Top. | largest difference 3.0e-8 | yes |
| FX-FOAM-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-025 frame 24: Bubble Texture Spit. | largest difference 3.0e-8 | yes |
| FX-FOAM-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-026 frame 24: Bubble Texture Bubblegum. | largest difference 3.0e-8 | yes |
| FX-FOAM-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-027 frame 24: Bubble Texture Dishwater. | largest difference 3.0e-8 | yes |
| FX-FOAM-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-028 frame 24: Bubble Texture Layer: the picture in every bubble. | largest difference 1.7e-7 | yes |
| FX-FOAM-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-029 frame 24: Layer texture, Bubble Orientation Physical: each picture turned its own way. | largest difference 1.7e-7 | yes |
| FX-FOAM-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-030 frame 24: Layer texture, Bubble Orientation Velocity: each picture's top the way it moves. | largest difference 1.6e-7 | yes |
| FX-FOAM-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-031 frame 24: Reflection Strength 0.7, Convergence 0: each bubble shows the picture round it. | largest difference 5.6e-8 | yes |
| FX-FOAM-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-032 frame 24: Reflection Strength 0.7, Convergence 1: each bubble shows the whole picture. | largest difference 6.5e-8 | yes |
| FX-FOAM-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-033 frame 24: Simulation Quality High: two steps a frame. | largest difference 3.0e-8 | yes |
| FX-FOAM-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-034 frame 12: Simulation Quality Intense: four steps a frame. | largest difference 2.3e-8 | yes |
| FX-FOAM-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-035 frame 24: Random Seed 7: other bubbles. | largest difference 2.9e-8 | yes |
| FX-FOAM-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-036 frame 24: Production Rate keyed from 0 at frame 0 to 2 at frame 24: born slowly, then faster. | largest difference 3.0e-8 | yes |
| FX-FOAM-036: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-037 frame 24: Wind Direction keyed from 0 to 180 over frames 0 to 24: blown up, then round and down. | largest difference 2.9e-8 | yes |
| FX-FOAM-037: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-038 frame 12: FX-FOAM-002 moved 3 pixels right: the same, moved. | largest difference 2.2e-8 | yes |
| FX-FOAM-038: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-039 frame 18: FX-FOAM-002 with the layer starting at frame 6: frame 6 clear, frame 18 002's frame 12. | largest difference 2.2e-8 | yes |
| FX-FOAM-039 frame 6: FX-FOAM-002 with the layer starting at frame 6: frame 6 clear, frame 18 002's frame 12. | largest difference 0.0e0 | yes |
| FX-FOAM-039: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-040 frame 12: FX-FOAM-002 at 30 frames a second: frame 12 is two fifths of a second in. | largest difference 2.8e-8 | yes |
| FX-FOAM-040: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-041 frame 24: The half picture with the Layer texture: half of each bubble's picture clear. | largest difference 1.4e-7 | yes |
| FX-FOAM-041: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-FOAM-042 frame 0: Production Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-042 frame 4: Production Rate -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-043 frame 0: Size 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-043 frame 4: Size 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-044 frame 0: Size Variance 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-044 frame 4: Size Variance 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-045 frame 0: Lifespan 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-045 frame 4: Lifespan 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-046 frame 0: Bubble Growth Speed 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-046 frame 4: Bubble Growth Speed 0, below 0.01. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-047 frame 0: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-047 frame 4: Strength 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-048 frame 0: Wind Speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-048 frame 4: Wind Speed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-049 frame 0: Viscosity 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-049 frame 4: Viscosity 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-050 frame 0: Zoom 0, below 0.05. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-050 frame 4: Zoom 0, below 0.05. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-051 frame 0: Universe Size 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-051 frame 4: Universe Size 11, above 10. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-052 frame 0: Reflection Strength 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-052 frame 4: Reflection Strength 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-053 frame 0: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-053 frame 4: Random Seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-054 frame 0: Producer X 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-054 frame 4: Producer X 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-054: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-055 frame 0: A view written "flow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-055 frame 4: A view written "flow". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-055: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-056 frame 0: A blend mode written "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-056 frame 4: A blend mode written "add". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-056: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-057 frame 0: A bubble texture written "soap". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-057 frame 4: A bubble texture written "soap". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-057: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-058 frame 0: A bubble orientation written "spin". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-058 frame 4: A bubble orientation written "spin". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-058: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-059 frame 0: A quality written "ultra". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-059 frame 4: A quality written "ultra". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-060 frame 0: A zoom producer point written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-060 frame 4: A zoom producer point written "yes". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-FOAM-061 frame 0: Production Rate keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-061 frame 4: Production Rate keyed to 200 at frame 4, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-FOAM-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_foam_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_foam_029.json is saved with its 32 settings, 26 numbers and six words, and no clock or history | 32 settings; size 6, view "rendered", bubble texture "layer", bubble orientation "physical" | yes |
| fx_foam_042.json is refused in a sentence naming production rate | Foam's production rate runs from 0 to 100, and this is -1. | yes |
| fx_foam_043.json is refused in a sentence naming size | Foam's size runs from 0 to 10, and this is 11. | yes |
| fx_foam_044.json is refused in a sentence naming size variance | Foam's size variance runs from 0 to 1, and this is 2. | yes |
| fx_foam_045.json is refused in a sentence naming lifespan | Foam's lifespan runs from 1 to 3000, and this is 0. | yes |
| fx_foam_046.json is refused in a sentence naming growth speed | Foam's growth speed runs from 0.01 to 10, and this is 0. | yes |
| fx_foam_047.json is refused in a sentence naming strength | Foam's strength runs from 0 to 100, and this is 101. | yes |
| fx_foam_048.json is refused in a sentence naming wind speed | Foam's wind speed runs from 0 to 100, and this is -1. | yes |
| fx_foam_049.json is refused in a sentence naming viscosity | Foam's viscosity runs from 0 to 1, and this is 2. | yes |
| fx_foam_050.json is refused in a sentence naming zoom | Foam's zoom runs from 0.05 to 50, and this is 0. | yes |
| fx_foam_051.json is refused in a sentence naming universe size | Foam's universe size runs from 0.1 to 10, and this is 11. | yes |
| fx_foam_052.json is refused in a sentence naming reflection strength | Foam's reflection strength runs from 0 to 1, and this is 1.5. | yes |
| fx_foam_053.json is refused in a sentence naming random seed | Foam's random seed runs from 0 to 10000, and this is -1. | yes |
| fx_foam_054.json is refused in a sentence naming producer x | Foam's producer x runs from -1000 to 1000, and this is 1001. | yes |
| fx_foam_055.json is refused in a sentence naming view | Foam's view is draft, rendered, and this is "flow". | yes |
| fx_foam_056.json is refused in a sentence naming blend mode | Foam's blend mode is transparent, solid_old_on_top, solid_new_on_top, and this is "add". | yes |
| fx_foam_057.json is refused in a sentence naming bubble texture | Foam's bubble texture is default_bubble, spit, bubblegum, dishwater, layer, and this is "soap". | yes |
| fx_foam_058.json is refused in a sentence naming bubble orientation | Foam's bubble orientation is fixed, physical, velocity, and this is "spin". | yes |
| fx_foam_059.json is refused in a sentence naming quality | Foam's quality is normal, high, intense, and this is "ultra". | yes |
| fx_foam_060.json is refused in a sentence naming zoom producer point | Foam's zoom producer point is off, on, and this is "yes". | yes |
| fx_foam_061.json is refused in a sentence naming production rate | Foam's production rate runs from 0 to 100, and this is 200. | yes |
| a file with a Foam whose size is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Foam whose view is a number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Foam without its bubble texture is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Tiles

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_foam_001.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_foam_016.json frame 12 in tiles of 1 and of 64 | byte-identical | yes |
| fx_foam_028.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |
| fx_foam_031.json frame 24 in tiles of 1 and of 64 | byte-identical | yes |

## On the card (D-678)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_foam_001.json, frames [0, 12, 24, 35] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_002.json, frames [0, 1, 12, 24] at Full and Draft | largest difference 0 of 255; on the card in 8 of 8 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_003.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_004.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_005.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_006.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_007.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_008.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_009.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_010.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_011.json, frames [6] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_012.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_013.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_014.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_015.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_016.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_017.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_018.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_019.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_020.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_021.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_022.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_023.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_024.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_025.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_026.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_027.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_028.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_029.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_030.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_031.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_032.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_033.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_034.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_035.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_036.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_037.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_038.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_039.json, frames [18, 6] at Full and Draft | largest difference 0 of 255; on the card in 4 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_040.json, frames [12] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_041.json, frames [24] at Full and Draft | largest difference 0 of 255; on the card in 2 of 2 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_042.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_043.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_044.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_045.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_046.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_047.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_048.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_049.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_050.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_051.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_052.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_053.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_054.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_055.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_056.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_057.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_058.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_059.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_060.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| fx_foam_061.json, frames [0, 4] at Full and Draft | largest difference 1 of 255; on the card in 0 of 4 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Foam as added: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Foam as added on three layers, frame 0, Full | largest difference 1 of 255, 8 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam as added on three layers, frame 100, Full | largest difference 1 of 255, 2 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam as added on three layers, frame 239, Full | largest difference 1 of 255, 22 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam as added on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam as added on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam as added on three layers, frame 239, Draft | largest difference 1 of 255, 3 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2069299 pixels changed | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 100, Full | largest difference 1 of 255, 903 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 239, Full | largest difference 1 of 255, 851 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 100, Draft | largest difference 1 of 255, 58 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, bigger bubbles, the Layer texture with a reflection on three layers, frame 239, Draft | largest difference 1 of 255, 65 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 0, Full | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 100, Full | largest difference 1 of 255, 1 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 239, Full | largest difference 1 of 255, 121 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 0, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 100, Draft | largest difference 0 of 255, 0 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Foam Rendered, production rate keyed 0 to 2, Solid New on Top on three layers, frame 239, Draft | largest difference 1 of 255, 6 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-678 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_frame_24.png, frame 24: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 2_rendered_close_look.png, frame 48: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 3_crowded_bubblegum.png, frame 48: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 4_the_street_in_every_bubble.png, frame 48: drawn without a warning, the street changed | 128426 pixels changed; warnings [] | yes |
| 5_windy_and_wandering.png, frame 48: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |
| 6_spit_new_on_top.png, frame 48: drawn without a warning, the street changed | 129600 pixels changed; warnings [] | yes |

## Result

326 of 326 checks pass.
