# D-630: Wave World

B-510, after After Effects' Wave World (Simulation), under the coordinator's approval of the D-630 design for the owner's /loop request. Water on a grid of Grid Resolution cells across, rippled by up to two producers (a ring or a line, each beating at its Frequency) and shaped by a ground layer, whose brightness raises the floor by Steepness under water of Height; Wave Speed, Damping and Reflect Edges set how the waves cross, die and meet the edge. The water runs from the layer's in point (after the Pre-roll) to the frame drawn, through every frame's settings, and is drawn in place of the layer as a grey height map by Brightness, Contrast and Gamma, dry land level or clear. Adobe's help publishes no formula, so the rule, the units and the settings when added are ours, stated in `tools/wave_world_reference.py`. Every expected pixel is `Fixtures/wave_world/expected_wave_world.json`, written by the tool before this code existed and printed in document 25 as FX-WW-001 to 048. Tolerance 2e-5.

## FX-WW-001 to 048 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WW-001 frame 0: As added: frame 0 is flat water, grey 0.5 everywhere and opaque; then a small ring of ripples spreads from the middle. | largest difference 3.2e-9 | yes |
| FX-WW-001 frame 12: As added: frame 0 is flat water, grey 0.5 everywhere and opaque; then a small ring of ripples spreads from the middle. | largest difference 7.4e-9 | yes |
| FX-WW-001 frame 24: As added: frame 0 is flat water, grey 0.5 everywhere and opaque; then a small ring of ripples spreads from the middle. | largest difference 1.0e-8 | yes |
| FX-WW-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-002 frame 12: Grid 16 and a ring a quarter wide: the water rises in the middle, then rings spread out. | largest difference 9.7e-9 | yes |
| FX-WW-002 frame 24: Grid 16 and a ring a quarter wide: the water rises in the middle, then rings spread out. | largest difference 2.9e-8 | yes |
| FX-WW-002 frame 3: Grid 16 and a ring a quarter wide: the water rises in the middle, then rings spread out. | largest difference 1.2e-8 | yes |
| FX-WW-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-003 frame 12: Phase 180: the same ripples upside down, dipping first. | largest difference 1.4e-8 | yes |
| FX-WW-003 frame 3: Phase 180: the same ripples upside down, dipping first. | largest difference 7.3e-9 | yes |
| FX-WW-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-004 frame 24: Frequency 2.5: closer ripples. | largest difference 7.4e-9 | yes |
| FX-WW-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-005 frame 18: A turned ring, 0.4 by 0.15 at 30 degrees: an oval source. | largest difference 7.4e-9 | yes |
| FX-WW-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-006 frame 18: A line 0.55 long and 0.2 wide at 20 degrees: waves leave both sides of it. | largest difference 1.4e-8 | yes |
| FX-WW-006 frame 6: A line 0.55 long and 0.2 wide at 20 degrees: waves leave both sides of it. | largest difference 1.4e-8 | yes |
| FX-WW-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-007 frame 20: Producer 2 switched on (amplitude 0.6, phase 90): two sources whose waves cross. | largest difference 2.8e-8 | yes |
| FX-WW-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-008 frame 12: Amplitude keyed 0 at frame 0 to 1 at frame 12: the source grows. | largest difference 7.6e-9 | yes |
| FX-WW-008 frame 18: Amplitude keyed 0 at frame 0 to 1 at frame 12: the source grows. | largest difference 7.4e-9 | yes |
| FX-WW-008 frame 6: Amplitude keyed 0 at frame 0 to 1 at frame 12: the source grows. | largest difference 1.4e-8 | yes |
| FX-WW-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-009 frame 18: Frequency keyed 0.5 at frame 0 to 3 at frame 18: the source speeds up smoothly, its phase the running sum of the frequency. | largest difference 1.5e-8 | yes |
| FX-WW-009 frame 24: Frequency keyed 0.5 at frame 0 to 3 at frame 18: the source speeds up smoothly, its phase the running sum of the frequency. | largest difference 9.1e-9 | yes |
| FX-WW-009 frame 9: Frequency keyed 0.5 at frame 0 to 3 at frame 18: the source speeds up smoothly, its phase the running sum of the frequency. | largest difference 7.4e-9 | yes |
| FX-WW-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-010 frame 24: Damping 0.5: the rings fade as they spread. | largest difference 1.5e-8 | yes |
| FX-WW-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-011 frame 12: Wave speed 0.6: the rings spread faster (more steps a frame). | largest difference 7.4e-9 | yes |
| FX-WW-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-012 frame 40: A source near the right edge, edges absorbing (none): the waves leave. | largest difference 1.4e-8 | yes |
| FX-WW-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-013 frame 40: FX-WW-012 with reflect edges all: the waves come back off the edges. | largest difference 1.5e-8 | yes |
| FX-WW-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-014 frame 40: Frame 40 of FX-WW-002, drawn alone: the same whether or not the frames before it were drawn. | largest difference 1.5e-8 | yes |
| FX-WW-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-015 frame 0: Pre-roll 0.5 seconds: frame 0 already has ripples. | largest difference 1.4e-8 | yes |
| FX-WW-015 frame 6: Pre-roll 0.5 seconds: frame 0 already has ripples. | largest difference 1.4e-8 | yes |
| FX-WW-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-016 frame 0: Pre-roll 0.5 with amplitude keyed 0.8 at frame 0 to 0 at frame 6: the pre-roll runs on frame 0's amplitude, then the source stops. | largest difference 2.0e-8 | yes |
| FX-WW-016 frame 12: Pre-roll 0.5 with amplitude keyed 0.8 at frame 0 to 0 at frame 6: the pre-roll runs on frame 0's amplitude, then the source stops. | largest difference 1.4e-8 | yes |
| FX-WW-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-017 frame 15: The holder's in point at frame 3: flat there, and frame 15 is FX-WW-002's frame 12 (time counts from the in point). | largest difference 9.7e-9 | yes |
| FX-WW-017 frame 3: The holder's in point at frame 3: flat there, and frame 15 is FX-WW-002's frame 12 (time counts from the in point). | largest difference 3.2e-9 | yes |
| FX-WW-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-018 frame 30: The logo as ground, steepness 0.6: the white disc is dry land, drawn level; the waves go round it. | largest difference 3.0e-8 | yes |
| FX-WW-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-019 frame 30: FX-WW-018 drawing dry areas transparent: the disc is clear. | largest difference 3.0e-8 | yes |
| FX-WW-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-020 frame 30: The logo as ground, steepness 0.3: all water, slower over the shallow disc. | largest difference 2.8e-8 | yes |
| FX-WW-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-021 frame 30: FX-WW-020 with transparency 0.8: clearer where shallow. | largest difference 4.3e-8 | yes |
| FX-WW-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-022 frame 18: Steepness keyed 0 to 0.3 over frames 0 to 12, wave strength 1, no source: the rising ground alone makes waves. | largest difference 1.7e-8 | yes |
| FX-WW-022 frame 6: Steepness keyed 0 to 0.3 over frames 0 to 12, wave strength 1, no source: the rising ground alone makes waves. | largest difference 1.5e-8 | yes |
| FX-WW-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-023 frame 18: FX-WW-022 with wave strength 0: no waves, flat grey. | largest difference 3.2e-9 | yes |
| FX-WW-023 frame 6: FX-WW-022 with wave strength 0: no waves, flat grey. | largest difference 3.2e-9 | yes |
| FX-WW-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-024 frame 12: A ground layer whose in point is frame 4, `late`: read at the holder's in point (frame 0) it is empty, so the ground is flat, as FX-WW-002. | largest difference 9.7e-9 | yes |
| FX-WW-024 frame 24: A ground layer whose in point is frame 4, `late`: read at the holder's in point (frame 0) it is empty, so the ground is flat, as FX-WW-002. | largest difference 2.9e-8 | yes |
| FX-WW-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-025 frame 0: A ground layer not in the composition, `gone`: flat ground, as FX-WW-002, and the warning. | largest difference 3.2e-9 | yes |
| FX-WW-025 frame 12: A ground layer not in the composition, `gone`: flat ground, as FX-WW-002, and the warning. | largest difference 9.7e-9 | yes |
| FX-WW-025: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-WW-026 frame 12: Brightness 0.3, contrast 2, gamma 2.2. | largest difference 1.5e-8 | yes |
| FX-WW-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-027 frame 12: Contrast 8: the peaks and troughs clipped to white and black. | largest difference 2.9e-8 | yes |
| FX-WW-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-028 frame 12: Gamma 0.5: darker greys. | largest difference 3.5e-9 | yes |
| FX-WW-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-029 frame 12: Height 0: every cell dry, drawn level, the brightness everywhere. | largest difference 3.2e-9 | yes |
| FX-WW-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-030 frame 12: Height 0 drawing dry areas transparent: clear. | largest difference 0.0e0 | yes |
| FX-WW-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-031 frame 12: FX-WW-002 on the holder moved 3 right and 1 down: the same, moved. | largest difference 9.7e-9 | yes |
| FX-WW-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-032 frame 12: Grid 7 (7 by 4 cells): coarse, smooth water. | largest difference 7.4e-9 | yes |
| FX-WW-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-033 frame 3: Grid 1: one cell, the whole layer rising and falling as one. | largest difference 9.9e-9 | yes |
| FX-WW-033 frame 6: Grid 1: one cell, the whole layer rising and falling as one. | largest difference 1.5e-9 | yes |
| FX-WW-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-034 frame 0: Brightness keyed 0.2 to 0.8 over frames 0 to 12: the map follows the frame drawn, not the history. | largest difference 4.1e-10 | yes |
| FX-WW-034 frame 12: Brightness keyed 0.2 to 0.8 over frames 0 to 12: the map follows the frame drawn, not the history. | largest difference 3.0e-8 | yes |
| FX-WW-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-035 frame 0: Grid resolution 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-035 frame 4: Grid resolution 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-036 frame 0: Grid resolution 601, above 600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-036 frame 4: Grid resolution 601, above 600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-037 frame 0: Wave speed -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-037 frame 4: Wave speed -0.1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-038 frame 0: Damping 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-038 frame 4: Damping 1.5, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-039 frame 0: Pre-roll -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-039 frame 4: Pre-roll -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-039: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-040 frame 0: Gamma 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-040 frame 4: Gamma 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-041 frame 0: Transparency 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-041 frame 4: Transparency 2, above 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-042 frame 0: Reflect edges "some". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-042 frame 4: Reflect edges "some". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-043 frame 0: Render dry areas as "hidden". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-043 frame 4: Render dry areas as "hidden". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-044 frame 0: Grid res downsamples "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-044 frame 4: Grid res downsamples "maybe". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-045 frame 0: Producer 1 type "dot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-045 frame 4: Producer 1 type "dot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-046 frame 0: A ground layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-046 frame 4: A ground layer written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-047 frame 0: Steepness keyed to 1.5 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-047 frame 4: Steepness keyed to 1.5 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-048 frame 0: Producer 2 position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-048 frame 4: Producer 2 position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The history

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WW-002's frame 40 drawn first, then again after frames 0, 12, 24 and 39: the same, since every frame runs the water from the in point | largest difference 0e0 | yes |
| FX-WW-002 with the holder's source 5 frames on: frame 12 is the same, since the water runs by the composition's frames from the in point | largest difference 0e0 | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| Wave World never grows the layer's bounds | 0 | yes |
| a half-size draft preview changes no setting, each a share of the layer, a count or a time; it halves only the grid when Grid Res Downsamples is on | WaveWorld { brightness: 0.5, contrast: 0.5, gamma: 1.0, render_dry_areas: "level", transparency: 0.0, grid_resolution: 16.0, grid_res_downsamples: "off", wave_speed: 0.25, damping: 0.05, reflect_edges: "none", pre_roll: 0.0, ground_layer: String(""), ground_map: None, steepness: 0.25, height: 0.5, wave_strength: 0.0, producer_1_type: "ring", producer_1_position: [50.0, 50.0], producer_1_height: 0.25, producer_1_width: 0.25, producer_1_angle: 0.0, producer_1_amplitude: 0.5, producer_1_frequency: 1.0, producer_1_phase: 0.0, producer_2_type: "ring", producer_2_position: [75.0, 25.0], producer_2_height: 0.1, producer_2_width: 0.1, producer_2_angle: 0.0, producer_2_amplitude: 0.0, producer_2_frequency: 1.0, producer_2_phase: 0.0, history: [], fps: 0.0, grid_scale: 0.5 } | yes |
| fx_ww_035.json is refused in a sentence naming grid resolution | Wave World's grid resolution runs from 1 to 600, and this is 0. | yes |
| fx_ww_036.json is refused in a sentence naming grid resolution | Wave World's grid resolution runs from 1 to 600, and this is 601. | yes |
| fx_ww_037.json is refused in a sentence naming wave speed | Wave World's wave speed runs from 0 to 1, and this is -0.1. | yes |
| fx_ww_038.json is refused in a sentence naming damping | Wave World's damping runs from 0 to 1, and this is 1.5. | yes |
| fx_ww_039.json is refused in a sentence naming pre roll | Wave World's pre roll runs from 0 to 10, and this is -1. | yes |
| fx_ww_040.json is refused in a sentence naming gamma | Wave World's gamma runs from 0.1 to 10, and this is 0. | yes |
| fx_ww_041.json is refused in a sentence naming transparency | Wave World's transparency runs from 0 to 1, and this is 2. | yes |
| fx_ww_042.json is refused in a sentence naming reflect edges | Wave World's reflect edges is "none" or "all", and this is "some". | yes |
| fx_ww_043.json is refused in a sentence naming render dry areas | Wave World's render dry areas as is "level" or "transparent", and this is "hidden". | yes |
| fx_ww_044.json is refused in a sentence naming grid res downsamples | Wave World's grid res downsamples is "off" or "on", and this is "maybe". | yes |
| fx_ww_045.json is refused in a sentence naming producer 1 type | Wave World's producer 1 type is "ring" or "line", and this is "dot". | yes |
| fx_ww_046.json is refused in a sentence naming ground layer | Wave World's ground layer is the name of a layer of this composition, and this is 3. | yes |
| fx_ww_048.json is refused in a sentence naming producer 2 position | Wave World's producer 2 position is two numbers, x then y, and this has 1. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ww_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_018.json is saved with its thirty-one settings, the ground layer `logo`, Producer 1 Position [20, 30] | {"brightness":0.5,"contrast":0.5,"damping":0.05,"gamma":1,"grid_res_downsamples":"off","grid_resolution":16,"ground_layer":"logo","height":0.5,"pre_roll":0,"producer_1_amplitude":0.5,"producer_1_angle":0,"producer_1_frequency":1,"producer_1_height":0.25,"producer_1_phase":0,"producer_1_position":[20,30],"producer_1_type":"ring","producer_1_width":0.25,"producer_2_amplitude":0,"producer_2_angle":0,"producer_2_frequency":1,"producer_2_height":0.1,"producer_2_phase":0,"producer_2_position":[75,25],"producer_2_type":"ring","producer_2_width":0.1,"reflect_edges":"none","render_dry_areas":"level","steepness":0.6,"transparency":0,"wave_speed":0.25,"wave_strength":0} | yes |
| the ground picture, the history, the frame rate and the draft's grid scale the effect is drawn with are never saved | None None None None | yes |
| fx_ww_048.json, its Producer 2 Position one number in a list, is saved as it was written | [10] | yes |
| a file with a Wave World whose grid resolution is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Wave World whose Producer 1 Position is a number, not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Wave World with no reflect edges is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Wave World with no ground layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Pictures (verification/D-630 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_ring.png, frame 36: drawn without a warning, 33 distinct greys (ripples, not flat) | 33 greys; 0 clear pixels; warnings [] | yes |
| 2_two_producers_crossing.png, frame 36: drawn without a warning, 256 distinct greys (ripples, not flat) | 256 greys; 0 clear pixels; warnings [] | yes |
| 3_line_reflect_edges.png, frame 48: drawn without a warning, 256 distinct greys (ripples, not flat) | 256 greys; 0 clear pixels; warnings [] | yes |
| 4_shore_round_an_island.png, frame 48: drawn without a warning, 225 distinct greys (ripples, not flat) | 225 greys; 0 clear pixels; warnings [] | yes |
| 5_island_clear.png, frame 48: drawn without a warning, 225 distinct greys (ripples, not flat) | 225 greys; 12142 clear pixels; warnings [] | yes |
| 6_shallows_fading.png, frame 48: drawn without a warning, 205 distinct greys (ripples, not flat) | 205 greys; 0 clear pixels; warnings [] | yes |

## Result

207 of 207 checks pass.
