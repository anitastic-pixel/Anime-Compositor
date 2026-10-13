# D-635: the wireframe views of Wave World and Shatter

B-515, After Effects' Wireframe Preview for Wave World and the four wireframe views for Shatter (Wireframe Front View, Wireframe, and each + Forces), under the coordinator's approval of the D-635 design for the owner's /loop request: "both are previews that never change the simulation, so the choices are ours to make". Wave World's grid is drawn in white, seen from above at the Vertical Rotation (30 degrees when added) with no perspective, its producers marked red and blue; Shatter's pieces are drawn as white outlines, straight on or through the effect's camera, its force spheres red and blue. The rules are stated in `tools/wave_world_reference.py` (step 8) and `tools/shatter_reference.py` (step 9). Every expected pixel was written by the tools before this code existed. Tolerance 2e-5.

## Wave World: FX-WW-049 to 062 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-WW-049 frame 0: Wireframe Preview with its controls as added: the water's grid seen from 30 degrees above, square-on; flat at frame 0, then the ripples raised, and a red cross on producer 1. | largest difference 0.0e0 | yes |
| FX-WW-049 frame 12: Wireframe Preview with its controls as added: the water's grid seen from 30 degrees above, square-on; flat at frame 0, then the ripples raised, and a red cross on producer 1. | largest difference 0.0e0 | yes |
| FX-WW-049: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-050 frame 12: Vertical rotation 80: seen from nearly straight above, the grid almost the layer's own shape. | largest difference 0.0e0 | yes |
| FX-WW-050: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-051 frame 12: Vertical rotation 10: seen low, the ripples standing up. | largest difference 0.0e0 | yes |
| FX-WW-051: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-052 frame 12: Vertical rotation -30: seen from below the water, the grid upside down. | largest difference 0.0e0 | yes |
| FX-WW-052: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-053 frame 12: Horizontal rotation 30: the grid turned about the upright. | largest difference 0.0e0 | yes |
| FX-WW-053: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-054 frame 24: Horizontal rotation keyed 0 at frame 0 to 60 at frame 24: the view turns as the water runs. | largest difference 0.0e0 | yes |
| FX-WW-054 frame 6: Horizontal rotation keyed 0 at frame 0 to 60 at frame 24: the view turns as the water runs. | largest difference 0.0e0 | yes |
| FX-WW-054: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-055 frame 12: Vertical scale 4: the ripples drawn four times higher. | largest difference 0.0e0 | yes |
| FX-WW-055: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-056 frame 12: Vertical scale 0: the ripples drawn flat, so frame 12 is FX-WW-049's frame 0. | largest difference 0.0e0 | yes |
| FX-WW-056: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-057 frame 20: Producer 2 switched on (amplitude 0.6): a blue cross on it as well. | largest difference 0.0e0 | yes |
| FX-WW-057: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-058 frame 30: The logo as ground, steepness 0.6, on a grid 15 across: the lines over the dry disc left out. | largest difference 0.0e0 | yes |
| FX-WW-058: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-WW-059 frame 0: View "solid". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-059 frame 4: View "solid". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-059: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-060 frame 0: Vertical rotation 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-060 frame 4: Vertical rotation 91, above 90. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-060: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-061 frame 0: Vertical scale -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-061 frame 4: Vertical scale -1, below 0. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-061: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-WW-062 frame 0: Horizontal rotation keyed to 3601 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-062 frame 4: Horizontal rotation keyed to 3601 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-WW-062: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_ww_059.json is refused in a sentence naming view | Wave World's view is "height_map" or "wireframe_preview", and this is "solid". | yes |
| fx_ww_060.json is refused in a sentence naming vertical rotation | Wave World's vertical rotation runs from -90 to 90, and this is 91. | yes |
| fx_ww_061.json is refused in a sentence naming vertical scale | Wave World's vertical scale runs from 0 to 10, and this is -1. | yes |
| fx_ww_062.json is refused in a sentence naming horizontal rotation | The layer "holder" has a core.wave_world whose settings this build cannot use. Wave World's horizontal rotation runs from -3600 to 3600, and this is 3601. The effect is kept and bypassed. | yes |
| fx_ww_049.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_050.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_051.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_052.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_053.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_054.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_055.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_056.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_057.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_058.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_059.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_060.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_061.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_062.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ww_001.json, from before D-635, opens as the height map as added and is saved without the view or the Wireframe Controls | [] | yes |
| fx_ww_001.json reads the Wireframe Controls as added: horizontal rotation 0, vertical rotation 30, vertical scale 1 | Some([0.0]) Some([30.0]) Some([1.0]) | yes |
| fx_ww_051.json is saved with the view and the two controls it moved, the horizontal rotation it left alone not written | view "wireframe_preview"; horizontal_rotation -; vertical_rotation 10; vertical_scale 2 | yes |

## Shatter: FX-SH-084 to 092 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-SH-084 frame 3: Wireframe Front, pattern bricks three across, every piece freed in place and turning in the picture's plane: the outlines turned. | largest difference 0.0e0 | yes |
| FX-SH-084: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-085 frame 3: Wireframe through the camera, the pieces tumbling about y: the outlines narrowing as they turn. | largest difference 0.0e0 | yes |
| FX-SH-085: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-086 frame 0: Wireframe Front + Forces, bricks four across: the bricks' outlines and force 1's sphere as a red circle about the middle; frame 0 at rest, frame 2 flying. | largest difference 0.0e0 | yes |
| FX-SH-086 frame 2: Wireframe Front + Forces, bricks four across: the bricks' outlines and force 1's sphere as a red circle about the middle; frame 0 at rest, frame 2 flying. | largest difference 0.0e0 | yes |
| FX-SH-086: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-087 frame 3: Wireframe + Forces with FX-SH-018's two forces: a red and a blue circle, seen through the camera. | largest difference 0.0e0 | yes |
| FX-SH-087: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-088 frame 3: Wireframe Front with Render Layer: only the pieces not freed are outlined. | largest difference 0.0e0 | yes |
| FX-SH-088: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-089 frame 3: Wireframe through a Comp Camera 12 pixels back: FX-SH-053's flight outlined in strong perspective. | largest difference 0.0e0 | yes |
| FX-SH-089: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-090 frame 3: Wireframe through Corner Pins, FX-SH-080's pins: the outlines on the pinned shape. | largest difference 0.0e0 | yes |
| FX-SH-090: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-091 frame 0: Wireframe Front at rest with the custom map `cracks`: each colour region outlined, the clear corner no piece. | largest difference 0.0e0 | yes |
| FX-SH-091: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-SH-092 frame 0: View "solid". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-092 frame 4: View "solid". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 1.5e-7 | yes |
| FX-SH-092: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| fx_sh_092.json is refused in a sentence naming view | Shatter's view is one of "rendered", "wireframe_front", "wireframe", "wireframe_front_forces", "wireframe_forces", and this is "solid". | yes |
| fx_sh_084.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_085.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_086.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_087.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_088.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_089.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_090.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_091.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_sh_092.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |

## Pictures (verification/D-635 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_wave_world_wireframe_as_added.png: drawn without a warning, only in white, red and blue, each colour there as asked | 15112 white, 9 red, 0 blue, 0 of any other colour; warnings [] | yes |
| 2_wave_world_wireframe_turned_two_producers.png: drawn without a warning, only in white, red and blue, each colour there as asked | 18831 white, 9 red, 9 blue, 0 of any other colour; warnings [] | yes |
| 3_wave_world_wireframe_island_shore.png: drawn without a warning, only in white, red and blue, each colour there as asked | 18134 white, 9 red, 0 blue, 0 of any other colour; warnings [] | yes |
| 4_shatter_wireframe_front.png: drawn without a warning, only in white, red and blue, each colour there as asked | 3882 white, 0 red, 0 blue, 0 of any other colour; warnings [] | yes |
| 5_shatter_wireframe_forces_turned_camera.png: drawn without a warning, only in white, red and blue, each colour there as asked | 2032 white, 624 red, 485 blue, 0 of any other colour; warnings [] | yes |

## Result

90 of 90 checks pass.
