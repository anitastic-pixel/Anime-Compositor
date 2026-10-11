# D-508: Energy Stroke

B-388, after the purpose of Video Copilot's Saber (docs/effects/PLUGINS.md pick #11), built separately from Path Stroke: a glowing beam, a white core inside a coloured glow, between two points or along the layer's masks, shapes or text outlines. Saber's controls are named in its tutorials but no formula and no settings when added are published, so the rule and those settings are ours, stated in `tools/energy_stroke_reference.py`: the core a smoothstep round each bit of the path, the glow four soft rings each half the last, flicker and distortion from value noise. Every expected pixel is `Fixtures/energy_stroke/expected_energy_stroke.json`, written by the tool before this code existed and printed in document 25 as FX-ENERGY-001 to 053. Tolerance 2e-5.

## FX-ENERGY-001 to 053 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-ENERGY-001 frame 0: The settings as added: a beam from a quarter across to three quarters across, half way down, core 8 across, softness 30, a blue glow spread 40 at intensity 100, added to the drawing. On a drawing this small the glow reaches every pixel. | largest difference 5.1e-8 | yes |
| FX-ENERGY-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-002 frame 0: Core 3, spread 6: a thin white line with its blue glow, the rest of the drawing lit faintly. | largest difference 5.1e-8 | yes |
| FX-ENERGY-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-003 frame 0: Composite transparent: the light alone, the night sky gone. | largest difference 1.9e-8 | yes |
| FX-ENERGY-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-004 frame 0: Composite black: the light over black, every pixel whole. | largest difference 1.9e-8 | yes |
| FX-ENERGY-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-005 frame 0: Glow spread 0: the core alone, no glow anywhere. | largest difference 1.9e-9 | yes |
| FX-ENERGY-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-006 frame 0: Core size 0: the glow alone, blue all through, no white. | largest difference 2.5e-8 | yes |
| FX-ENERGY-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-007 frame 0: Core softness 0 and 100: hard, then soft from the middle out. | largest difference 1.1e-8 | yes |
| FX-ENERGY-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-008 frame 0: Core softness 100. | largest difference 2.4e-8 | yes |
| FX-ENERGY-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-009 frame 0: Glow intensity 400, an orange glow: past 1 where the glow is strong. | largest difference 5.7e-8 | yes |
| FX-ENERGY-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-010 frame 0: End offset 50: the beam from its start to half way, ending at the middle. | largest difference 1.9e-8 | yes |
| FX-ENERGY-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-011 frame 0: Start offset 75, End offset 25: taken smaller first, the middle half. | largest difference 1.9e-8 | yes |
| FX-ENERGY-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-012 frame 0: Start offset 40, End offset 40: nothing drawn, the drawing as it was. | largest difference 7.3e-9 | yes |
| FX-ENERGY-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-013 frame 0: End offset keyed from 0 at frame 0 to 100 at frame 4, linear: the beam draws on, frames 0, 2 and 4. | largest difference 0.0e0 | yes |
| FX-ENERGY-013 frame 2: End offset keyed from 0 at frame 0 to 100 at frame 4, linear: the beam draws on, frames 0, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-013 frame 4: End offset keyed from 0 at frame 0 to 100 at frame 4, linear: the beam draws on, frames 0, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-014 frame 0: Start size 100, End size 0: the beam tapers to a point at its end. | largest difference 2.8e-8 | yes |
| FX-ENERGY-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-015 frame 0: Start size 0 and End size 0: no width anywhere, nothing drawn. | largest difference 7.3e-9 | yes |
| FX-ENERGY-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-016 frame 0: Core start and end keyed: the beam from the top-left corner to the bottom-right at frame 0, swinging to the bottom-left to the top-right at frame 4, frames 0 and 4. | largest difference 3.0e-8 | yes |
| FX-ENERGY-016 frame 4: Core start and end keyed: the beam from the top-left corner to the bottom-right at frame 0, swinging to the bottom-left to the top-right at frame 4, frames 0 and 4. | largest difference 3.0e-8 | yes |
| FX-ENERGY-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-017 frame 0: Path From Masks, the box from (2, 2) to (13, 7): the beam all round it. | largest difference 1.9e-8 | yes |
| FX-ENERGY-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-018 frame 0: The box, End offset 50: half way round it, from its top-left corner. | largest difference 1.9e-8 | yes |
| FX-ENERGY-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-019 frame 0: The box, End offset 50, Evolution 90: the same half, begun a quarter of the way round, at the top-right corner. | largest difference 1.9e-8 | yes |
| FX-ENERGY-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-020 frame 0: The box, End offset 50, Evolution 450: a turn and a quarter, FX-ENERGY-019's frame. | largest difference 1.9e-8 | yes |
| FX-ENERGY-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-021 frame 0: The box, End offset 25, Evolution keyed from 0 at frame 0 to 360 at frame 4: a quarter of the box travelling round it, frames 0, 1, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-021 frame 1: The box, End offset 25, Evolution keyed from 0 at frame 0 to 360 at frame 4: a quarter of the box travelling round it, frames 0, 1, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-021 frame 2: The box, End offset 25, Evolution keyed from 0 at frame 0 to 360 at frame 4: a quarter of the box travelling round it, frames 0, 1, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-021 frame 4: The box, End offset 25, Evolution keyed from 0 at frame 0 to 360 at frame 4: a quarter of the box travelling round it, frames 0, 1, 2 and 4. | largest difference 1.9e-8 | yes |
| FX-ENERGY-021: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-022 frame 0: Two masks, the box and a small box inside it: both, the glow of each the nearer's. | largest difference 1.9e-8 | yes |
| FX-ENERGY-022: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-023 frame 0: The box, distortion 1.5 at scale 4, speed 0: the beam wobbles but stays joined round the box, and is the same at frame 0 and 3. | largest difference 3.0e-8 | yes |
| FX-ENERGY-023 frame 3: The box, distortion 1.5 at scale 4, speed 0: the beam wobbles but stays joined round the box, and is the same at frame 0 and 3. | largest difference 3.0e-8 | yes |
| FX-ENERGY-023: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-024 frame 0: Distortion 1.5 at scale 4, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 3.0e-8 | yes |
| FX-ENERGY-024 frame 1: Distortion 1.5 at scale 4, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 3.0e-8 | yes |
| FX-ENERGY-024 frame 2: Distortion 1.5 at scale 4, speed 50: the wobble moves, frames 0, 1 and 2. | largest difference 3.0e-8 | yes |
| FX-ENERGY-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-025 frame 0: FX-ENERGY-023 with Random Seed 7: a different wobble. | largest difference 2.9e-8 | yes |
| FX-ENERGY-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-026 frame 0: The straight beam with distortion 1 at scale 3: an open path wobbles too. | largest difference 3.0e-8 | yes |
| FX-ENERGY-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-027 frame 0: Flicker 80 at speed 30: the whole light dims and brightens frame by frame, frames 0 to 4. | largest difference 2.2e-8 | yes |
| FX-ENERGY-027 frame 1: Flicker 80 at speed 30: the whole light dims and brightens frame by frame, frames 0 to 4. | largest difference 1.4e-8 | yes |
| FX-ENERGY-027 frame 2: Flicker 80 at speed 30: the whole light dims and brightens frame by frame, frames 0 to 4. | largest difference 1.2e-8 | yes |
| FX-ENERGY-027 frame 3: Flicker 80 at speed 30: the whole light dims and brightens frame by frame, frames 0 to 4. | largest difference 1.4e-8 | yes |
| FX-ENERGY-027 frame 4: Flicker 80 at speed 30: the whole light dims and brightens frame by frame, frames 0 to 4. | largest difference 1.3e-8 | yes |
| FX-ENERGY-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-028 frame 0: Flicker 80 at speed 0: held, the same each frame, frames 0 and 4. | largest difference 2.2e-8 | yes |
| FX-ENERGY-028 frame 4: Flicker 80 at speed 0: held, the same each frame, frames 0 and 4. | largest difference 2.2e-8 | yes |
| FX-ENERGY-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-029 frame 0: Path From Shapes, the box as a shape layer's shape: FX-ENERGY-017's beam on the shape layer (transparent), added. | largest difference 1.9e-8 | yes |
| FX-ENERGY-029: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-030 frame 0: An open shape, a roof of two legs: drawn open, nothing along the bottom. | largest difference 2.5e-8 | yes |
| FX-ENERGY-030: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-031 frame 0: The open roof, Evolution 180: an open path has no evolution, FX-ENERGY-030's frame. | largest difference 2.5e-8 | yes |
| FX-ENERGY-031: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-032 frame 0: The box mask with Path From Points: the mask is not used, the straight beam, FX-ENERGY-002's frame. | largest difference 5.1e-8 | yes |
| FX-ENERGY-032: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-033 frame 0: FX-ENERGY-002 moved three pixels right: the beam moves with the drawing. | largest difference 5.1e-8 | yes |
| FX-ENERGY-033: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-034 frame 0: Path From Text Outlines, "Yes" (D-373's text layer, 48 by 30): every outline glows, core 1.5, spread 3, the light alone (Transparent). | largest difference 3.0e-8 | yes |
| FX-ENERGY-034: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-035 frame 0: "Yes", End offset keyed from 0 at frame 0 to 100 at frame 4: each outline draws on round itself, frames 0, 2 and 4. | largest difference 0.0e0 | yes |
| FX-ENERGY-035 frame 2: "Yes", End offset keyed from 0 at frame 0 to 100 at frame 4: each outline draws on round itself, frames 0, 2 and 4. | largest difference 3.0e-8 | yes |
| FX-ENERGY-035 frame 4: "Yes", End offset keyed from 0 at frame 0 to 100 at frame 4: each outline draws on round itself, frames 0, 2 and 4. | largest difference 3.0e-8 | yes |
| FX-ENERGY-035: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-ENERGY-036 frame 0: Path From Masks, no masks. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-036 frame 4: Path From Masks, no masks. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-036: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGY-037 frame 0: Path From Masks, the one mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-037 frame 4: Path From Masks, the one mask switched off. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-037: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGY-038 frame 0: Path From Shapes on the night drawing, which is not a shape layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-038 frame 4: Path From Shapes on the night drawing, which is not a shape layer. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 7.3e-9 | yes |
| FX-ENERGY-038: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGY-039 frame 0: Path From Text Outlines on a shape layer with no shapes. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-ENERGY-039 frame 4: Path From Text Outlines on a shape layer with no shapes. Nothing to draw along: the layer is drawn without the effect, which is kept as written, and EFFECT_PATH_MISSING is said every frame. | largest difference 0.0e0 | yes |
| FX-ENERGY-039: what opening it warns of, and what frame 4 warns of | [] and ["EFFECT_PATH_MISSING"] | yes |
| FX-ENERGY-040 frame 0: Core size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-040 frame 4: Core size 201, above 200. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-040: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-041 frame 0: Core softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-041 frame 4: Core softness -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-041: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-042 frame 0: Glow spread 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-042 frame 4: Glow spread 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-042: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-043 frame 0: Glow intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-043 frame 4: Glow intensity 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-043: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-044 frame 0: Start offset -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-044 frame 4: Start offset -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-044: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-045 frame 0: End size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-045 frame 4: End size 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-045: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-046 frame 0: Distortion scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-046 frame 4: Distortion scale 0, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-046: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-047 frame 0: Flicker amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-047 frame 4: Flicker amount 101, above 100. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-047: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-048 frame 0: Core start x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-048 frame 4: Core start x 1001, above 1000. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-048: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-049 frame 0: Evolution 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-049 frame 4: Evolution 3601, above 3600. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-049: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-050 frame 0: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-050 frame 4: Random seed -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-050: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-051 frame 0: Composite "screen", which is not "add", "transparent" or "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-051 frame 4: Composite "screen", which is not "add", "transparent" or "black". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-051: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-052 frame 0: Path From "mask", which is not "points", "masks", "shapes" or "text". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-052 frame 4: Path From "mask", which is not "points", "masks", "shapes" or "text". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-052: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-ENERGY-053 frame 0: Glow colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-053 frame 4: Glow colour "#12345", not six hex digits. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 7.3e-9 | yes |
| FX-ENERGY-053: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_energy_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_039.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_040.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_041.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_042.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_043.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_044.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_045.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_046.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_047.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_048.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_energy_009.json is saved with its twenty settings, an orange glow at intensity 400, Core Start [25, 50] | {"composite":"add","core_end":[75,50],"core_size":3,"core_softness":30,"core_start":[25,50],"distortion_amount":0,"distortion_scale":60,"distortion_speed":20,"end_offset":100,"end_size":100,"evolution":0,"flicker_amount":0,"flicker_speed":20,"glow_color":"#ff8020","glow_intensity":400,"glow_spread":6,"random_seed":0,"source":"points","start_offset":0,"start_size":100} | yes |
| fx_energy_040.json is refused in a sentence naming core size | Energy Stroke's core size runs from 0 to 200, and this is 201. | yes |
| fx_energy_041.json is refused in a sentence naming core softness | Energy Stroke's core softness runs from 0 to 100, and this is -1. | yes |
| fx_energy_042.json is refused in a sentence naming glow spread | Energy Stroke's glow spread runs from 0 to 1000, and this is 1001. | yes |
| fx_energy_043.json is refused in a sentence naming glow intensity | Energy Stroke's glow intensity runs from 0 to 1000, and this is 1001. | yes |
| fx_energy_044.json is refused in a sentence naming start offset | Energy Stroke's start offset runs from 0 to 100, and this is -1. | yes |
| fx_energy_045.json is refused in a sentence naming end size | Energy Stroke's end size runs from 0 to 100, and this is 101. | yes |
| fx_energy_046.json is refused in a sentence naming distortion scale | Energy Stroke's distortion scale runs from 1 to 1000, and this is 0. | yes |
| fx_energy_047.json is refused in a sentence naming flicker amount | Energy Stroke's flicker amount runs from 0 to 100, and this is 101. | yes |
| fx_energy_048.json is refused in a sentence naming core start | Energy Stroke's core start runs from -1000 to 1000, and this is 1001. | yes |
| fx_energy_049.json is refused in a sentence naming evolution | Energy Stroke's evolution runs from -3600 to 3600, and this is 3601. | yes |
| fx_energy_050.json is refused in a sentence naming random seed | Energy Stroke's random seed runs from 0 to 10000, and this is -1. | yes |
| fx_energy_051.json is refused in a sentence naming composite | Energy Stroke's composite is "add", "transparent" or "black", and this is "screen". | yes |
| fx_energy_052.json is refused in a sentence naming path from | Energy Stroke's path from is "points", "masks", "shapes" or "text", and this is "mask". | yes |
| fx_energy_053.json is refused in a sentence naming glow colour | Energy Stroke's glow colour is written #rrggbb, and this is "#12345". | yes |
| a file with an Energy Stroke whose core size is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with an Energy Stroke whose Core End is one number is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## On the card (D-508)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_energy_001.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_002.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_003.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_004.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_005.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_006.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_007.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_008.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_009.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_010.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_011.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_012.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_013.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_014.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_015.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_016.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_017.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_018.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_019.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_020.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_021.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_022.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_023.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_024.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_025.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_026.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 5 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_027.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_028.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_029.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_030.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_031.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_032.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_033.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 10 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_034.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_035.json, frames 0 to 4 at Full and Draft | largest difference 1 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_036.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_037.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_038.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_039.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_040.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_041.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_042.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_043.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_044.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_045.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_046.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_047.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_048.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_049.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_050.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_051.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_052.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| fx_energy_053.json, frames 0 to 4 at Full and Draft | largest difference 0 of 255; on the card in 0 of 10 frames; refused by the card: false; the same warnings: true | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added): the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 91622 pixels changed | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 0, Full | largest difference 1 of 255, 2145 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 100, Full | largest difference 1 of 255, 1131 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 239, Full | largest difference 1 of 255, 1309 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 0, Draft | largest difference 1 of 255, 72 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 100, Draft | largest difference 1 of 255, 40 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke as added (a blue beam across the middle, added) on three layers, frame 239, Draft | largest difference 1 of 255, 51 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 0, Full | largest difference 1 of 255, 467 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 100, Full | largest difference 1 of 255, 221 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 239, Full | largest difference 1 of 255, 548 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 0, Draft | largest difference 1 of 255, 54 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 100, Draft | largest difference 1 of 255, 23 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke orange, intensity 300, diagonal, transparent on three layers, frame 239, Draft | largest difference 1 of 255, 62 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 2073600 pixels changed | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 0, Full | largest difference 1 of 255, 251 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 100, Full | largest difference 1 of 255, 254 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 239, Full | largest difference 1 of 255, 289 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 0, Draft | largest difference 1 of 255, 14 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 100, Draft | largest difference 1 of 255, 7 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke distortion 20 at scale 80, flicker 60 at speed 40, black on three layers, frame 239, Draft | largest difference 1 of 255, 13 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100: the processor's frame 100 differs from the shot without it, so the comparisons below test the effect | 32192 pixels changed | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 0, Full | largest difference 1 of 255, 3781 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 100, Full | largest difference 1 of 255, 1229 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 239, Full | largest difference 1 of 255, 1383 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 0, Draft | largest difference 1 of 255, 129 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 100, Draft | largest difference 1 of 255, 36 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |
| the reference shot, Energy Stroke tapered, End Offset keyed from 0 to 100 on three layers, frame 239, Draft | largest difference 1 of 255, 45 pixels differ; 3 of 3 on the card; warnings CPU [] GPU [] | yes |

## Pictures (verification/D-508 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added.png: drawn without a warning, the street changed, lit at (240, 135) | 24641 pixels changed; at (240, 135) [255, 255, 255, 255]; warnings [] | yes |
| 2_orange_saber.png: drawn without a warning, the street changed, lit at (240, 135) | 28389 pixels changed; at (240, 135) [255, 255, 255, 255]; warnings [] | yes |
| 3_round_a_mask.png: drawn without a warning, the street changed, lit at (240, 40) | 41546 pixels changed; at (240, 40) [255, 255, 255, 255]; warnings [] | yes |
| 4_wobble.png: drawn without a warning, the street changed, lit at (240, 135) | 57536 pixels changed; at (240, 135) [90, 140, 170, 255]; warnings [] | yes |
| 5_over_black.png: drawn without a warning, the street changed, lit at (240, 135) | 129600 pixels changed; at (240, 135) [255, 255, 255, 255]; warnings [] | yes |

## Result

296 of 296 checks pass.
