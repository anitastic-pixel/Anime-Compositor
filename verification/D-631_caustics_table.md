# D-631: Caustics

B-511, after After Effects' Caustics (Simulation), under the coordinator's approval of the D-630 design for the owner's /loop request. A bottom (the layer's own drawing, or a chosen layer placed by Scaling, Repeat Mode and If Layer Size Differs, softened by Bottom Blur) seen through water whose waves come from a water surface layer's brightness, by Wave Height and Smoothing, bent by Water Depth and the Refractive Index, brightened and darkened by Caustics Strength where the waves gather and spread the light, under a Surface Color at Surface Opacity that can mirror a placed sky and is lit by Card Wipe's light. Adobe's help publishes no formula, so the rule, the units and most settings when added are ours, stated in `tools/caustics_reference.py`. Every expected pixel is `Fixtures/caustics/expected_caustics.json`, written by the tool before this code existed and printed in document 25 as FX-CA-001 to 038. Tolerance 2e-5.

## FX-CA-001 to 038 (document 25)

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-CA-001 frame 0: As added, no water surface: flat water, the checker seen straight through, 30% of the blue surface over it, lit evenly. | largest difference 5.5e-8 | yes |
| FX-CA-001: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-002 frame 0: The ripples as the water surface, wave height 0.4, depth 0.2: the checker bent by the waves. | largest difference 2.4e-7 | yes |
| FX-CA-002: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-003 frame 0: FX-CA-002 with wave height 0.8 and depth 0.4: bent much further. | largest difference 8.3e-7 | yes |
| FX-CA-003: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-004 frame 0: Refractive index 1 and surface opacity 0: nothing bends and nothing covers, the checker exactly. | largest difference 9.1e-8 | yes |
| FX-CA-004: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-005 frame 0: Water depth 0 and surface opacity 0: the checker exactly. | largest difference 9.1e-8 | yes |
| FX-CA-005: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-006 frame 0: Caustics strength 1: bright and dark patches where the waves gather and spread the light. | largest difference 1.9e-5 | yes |
| FX-CA-006: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-007 frame 0: Smoothing 40: the waves smoothed before they bend the checker. | largest difference 9.7e-8 | yes |
| FX-CA-007: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-008 frame 0: Smoothing 0 and refractive index 2: unsmoothed and bent hard. | largest difference 6.8e-7 | yes |
| FX-CA-008: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-009 frame 0: Bottom blur 25: the checker soft under the waves. | largest difference 1.1e-7 | yes |
| FX-CA-009: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-010 frame 0: The `floor` layer (20 by 12) as the bottom, stretched to the layer: red and white stripes and a green line, bent. | largest difference 1.6e-7 | yes |
| FX-CA-010: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-011 frame 0: The floor centred, once: the stripes in the middle, only the surface round them. | largest difference 3.0e-7 | yes |
| FX-CA-011: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-012 frame 0: The floor centred, tiled at scaling 0.5: many small copies. | largest difference 2.8e-7 | yes |
| FX-CA-012: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-013 frame 0: The floor centred, reflected at scaling 0.5: copies mirrored at their edges. | largest difference 2.9e-7 | yes |
| FX-CA-013: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-014 frame 0: The checker as the bottom at scaling 2.3, tiled: larger squares. | largest difference 1.3e-7 | yes |
| FX-CA-014: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-015 frame 0: The `sky` layer stretched, intensity 1, surface opacity 0.8: the sky mirrored on the water, bent by the waves. | largest difference 1.7e-7 | yes |
| FX-CA-015: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-016 frame 0: The sky centred, reflected at scaling 0.7, convergence 0.6, intensity 0.7. | largest difference 2.9e-7 | yes |
| FX-CA-016: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-017 frame 0: Flat water, an orange surface fully opaque, a point light low at the top left: brighter near the light. | largest difference 3.0e-8 | yes |
| FX-CA-017: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-018 frame 0: The ripples under that point light with a sharp highlight, surface opacity 0.6. | largest difference 4.0e-7 | yes |
| FX-CA-018: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-019 frame 0: A pink light at intensity 2, ambient 0.1, diffuse 1.2, from the left. | largest difference 1.5e-7 | yes |
| FX-CA-019: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-020 frame 2: A water surface whose in point is frame 4, `late`: flat before it (a clear picture, every pixel the same), the ripples after. | largest difference 5.5e-8 | yes |
| FX-CA-020 frame 6: A water surface whose in point is frame 4, `late`: flat before it (a clear picture, every pixel the same), the ripples after. | largest difference 2.4e-7 | yes |
| FX-CA-020: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-021 frame 0: A water surface not in the composition, `gone`: flat water, and the warning. | largest difference 5.5e-8 | yes |
| FX-CA-021: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CA-022 frame 0: A bottom not in the composition, `gone`: a clear bottom, only the surface, and the warning. | largest difference 2.2e-8 | yes |
| FX-CA-022: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CA-023 frame 0: A sky not in the composition, `gone`: no sky, as FX-CA-002, and the warning. | largest difference 2.4e-7 | yes |
| FX-CA-023: what opening it warns of, and what frame 4 warns of | ["EFFECT_LAYER_MISSING"] and ["EFFECT_LAYER_MISSING"] | yes |
| FX-CA-024 frame 0: Wave height keyed 0 at frame 0 to 0.8 at frame 8: flat at frame 0, then the waves grow. | largest difference 5.5e-8 | yes |
| FX-CA-024 frame 4: Wave height keyed 0 at frame 0 to 0.8 at frame 8: flat at frame 0, then the waves grow. | largest difference 2.4e-7 | yes |
| FX-CA-024 frame 8: Wave height keyed 0 at frame 0 to 0.8 at frame 8: flat at frame 0, then the waves grow. | largest difference 4.2e-7 | yes |
| FX-CA-024: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-025 frame 0: FX-CA-002 on the holder moved 3 right and 2 down: the same, moved. | largest difference 2.4e-7 | yes |
| FX-CA-025: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-026 frame 1: The holder's in point at frame 2: nothing at frame 1, FX-CA-002 at frame 3. | largest difference 0.0e0 | yes |
| FX-CA-026 frame 3: The holder's in point at frame 2: nothing at frame 1, FX-CA-002 at frame 3. | largest difference 2.4e-7 | yes |
| FX-CA-026: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-027 frame 0: The holder named as its own bottom: its own drawing, as FX-CA-002. | largest difference 2.4e-7 | yes |
| FX-CA-027: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-028 frame 0: Everything at once: caustics, sky, blur and a highlight. | largest difference 9.0e-6 | yes |
| FX-CA-028: what opening it warns of, and what frame 4 warns of | [] and [] | yes |
| FX-CA-029 frame 0: Refractive index 0.9, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-029 frame 4: Refractive index 0.9, below 1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-029: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-030 frame 0: Bottom scaling 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-030 frame 4: Bottom scaling 0, below 0.1. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-030: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-031 frame 0: Bottom blur 60, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-031 frame 4: Bottom blur 60, above 50. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-031: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-032 frame 0: Bottom repeat mode "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-032 frame 4: Bottom repeat mode "mirror". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-032: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-033 frame 0: Sky fit "tile", which Caustics does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-033 frame 4: Sky fit "tile", which Caustics does not offer. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-033: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-034 frame 0: Surface colour "blue". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-034 frame 4: Surface colour "blue". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-034: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-035 frame 0: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-035 frame 4: Light type "spot". The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-035: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-036 frame 0: A water surface written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-036 frame 4: A water surface written as the number 3, not a word. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-036: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-037 frame 0: Light position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-037 frame 4: Light position with one number. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-037: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |
| FX-CA-038 frame 0: Smoothing keyed to 60 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-038 frame 4: Smoothing keyed to 60 at frame 4. The file is read, the effect is kept as written and left out of every frame, with a warning. | largest difference 9.1e-8 | yes |
| FX-CA-038: what opening it warns of, and what frame 4 warns of | ["EFFECT_PARAMETER_INVALID"] and ["EFFECT_PARAMETER_INVALID"] | yes |

## The settings

| Check | The build's answer | Matches |
| --- | --- | --- |
| Caustics never grows the layer's bounds | 0 | yes |
| a half-size draft preview changes no setting, each a share of the layer, a count or a word | Caustics { bottom_layer: String(""), bottom_map: None, bottom_scaling: 1.0, bottom_repeat_mode: "tiled", bottom_fit: "stretch", bottom_blur: 0.0, water_surface: String("ripples"), surface_map: None, wave_height: 0.4, smoothing: 5.0, water_depth: 0.2, refractive_index: 1.2, surface_color: "#2a7fd4", surface_opacity: 0.3, caustics_strength: 0.0, sky_layer: String(""), sky_map: None, sky_scaling: 1.0, sky_repeat_mode: "tiled", sky_fit: "stretch", sky_intensity: 0.3, sky_convergence: 0.2, light_type: "distant", light_intensity: 1.0, light_color: "#ffffff", light_position: [50.0, 50.0], light_height: 1.0, ambient_light: 0.25, diffuse_reflection: 0.75, specular_reflection: 0.0, highlight_sharpness: 5.0 } | yes |
| fx_ca_029.json is refused in a sentence naming refractive index | Caustics's refractive index runs from 1 to 2, and this is 0.9. | yes |
| fx_ca_030.json is refused in a sentence naming bottom scaling | Caustics's bottom scaling runs from 0.1 to 10, and this is 0. | yes |
| fx_ca_031.json is refused in a sentence naming bottom blur | Caustics's bottom blur runs from 0 to 50, and this is 60. | yes |
| fx_ca_032.json is refused in a sentence naming bottom repeat mode | Caustics' bottom repeat mode is one of "once", "tiled", "reflected", and this is "mirror". | yes |
| fx_ca_033.json is refused in a sentence naming sky fit | Caustics' sky fit is one of "center", "stretch", and this is "tile". | yes |
| fx_ca_034.json is refused in a sentence naming surface colour | Caustics's surface colour is written #rrggbb, and this is "blue". | yes |
| fx_ca_035.json is refused in a sentence naming light type | Caustics' light type is one of "distant", "point", and this is "spot". | yes |
| fx_ca_036.json is refused in a sentence naming water surface | Caustics' water surface is the name of a layer of this composition, and this is 3. | yes |
| fx_ca_037.json is refused in a sentence naming light position | Caustics' light position is two numbers, x then y, and this has 1. | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_ca_001.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_002.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_003.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_004.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_005.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_006.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_007.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_008.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_009.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_010.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_011.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_012.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_013.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_014.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_015.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_016.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_017.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_018.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_019.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_020.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_021.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_022.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_023.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_024.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_025.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_026.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_027.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_028.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_029.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_030.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_031.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_032.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_033.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_034.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_035.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_036.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_037.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_038.json opened and saved holds what it held, out-of-range values, wrong words and keys included | the same | yes |
| fx_ca_028.json is saved with its twenty-eight settings, the water surface `ripples`, the sky `sky`, Light Position [15, 20] | {"ambient_light":0.25,"bottom_blur":10,"bottom_fit":"stretch","bottom_layer":"","bottom_repeat_mode":"tiled","bottom_scaling":1,"caustics_strength":0.7,"diffuse_reflection":0.75,"highlight_sharpness":20,"light_color":"#ffffff","light_height":0.8,"light_intensity":1,"light_position":[15,20],"light_type":"point","refractive_index":1.2,"sky_convergence":0.2,"sky_fit":"stretch","sky_intensity":0.6,"sky_layer":"sky","sky_repeat_mode":"tiled","sky_scaling":1,"smoothing":5,"specular_reflection":1,"surface_color":"#2a7fd4","surface_opacity":0.5,"water_depth":0.2,"water_surface":"ripples","wave_height":0.4} | yes |
| the three pictures the effect is drawn with are never saved | None None None | yes |
| fx_ca_037.json, its Light Position one number in a list, is saved as it was written | [10] | yes |
| a file with a Caustics whose wave height is a word is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Caustics whose Light Position is a number, not a list is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Caustics with no bottom fit is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |
| a file with a Caustics with no sky layer is refused as a fault in its shape | This project file cannot be opened, because part of it does not match the project format. | yes |

## Pictures (verification/D-631 pictures)

| Check | The build's answer | Matches |
| --- | --- | --- |
| 1_as_added_with_water.png: drawn without a warning, every pixel covered, 3829 distinct colours | 3829 colours; 0 clear pixels; warnings [] | yes |
| 2_deeper_waves.png: drawn without a warning, every pixel covered, 4312 distinct colours | 4312 colours; 0 clear pixels; warnings [] | yes |
| 3_caustics.png: drawn without a warning, every pixel covered, 24848 distinct colours | 24848 colours; 0 clear pixels; warnings [] | yes |
| 4_floor_tiled.png: drawn without a warning, every pixel covered, 8395 distinct colours | 8395 colours; 0 clear pixels; warnings [] | yes |
| 5_sky_mirrored.png: drawn without a warning, every pixel covered, 35477 distinct colours | 35477 colours; 0 clear pixels; warnings [] | yes |
| 6_point_light_highlight.png: drawn without a warning, every pixel covered, 17941 distinct colours | 17941 colours; 0 clear pixels; warnings [] | yes |
| 7_everything.png: drawn without a warning, every pixel covered, 67975 distinct colours | 67975 colours; 0 clear pixels; warnings [] | yes |

## Result

153 of 153 checks pass.
