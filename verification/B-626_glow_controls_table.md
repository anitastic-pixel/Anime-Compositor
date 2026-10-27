# B-626: After Effects' further Glow controls

D-747, audit row 17 (https://claude.ai/artifact/A63KdY44N4HfUHzUnLQhTp). An orange disc (255, 160, 40) of radius 20 on grey 64, at 8 bpc, with a Glow in After Effects units (threshold 60, radius 10, intensity 1) and each new control. The rule is our basis: After Effects publishes no formula, and Wave 1's tutorials use the starting values only. FX-GLDISP-009 to 016 are held by B-625's test.

## An older project, without the new settings, reads them at After Effects' start and draws as before

| Check | The build's answer | Matches |
| --- | --- | --- |
| the settings it reads: Behind, both, Original Colors, Triangle A>B>A, 1 loop, phase 0, midpoint 50, #ffffff and #000000 | GlowLook { composite: "behind", dimensions: "both", glow_colors: "original", color_looping: "triangle_aba", color_loops: 1.0, color_phase: 0.0, midpoint: 50.0, color_a: "#ffffff", color_b: "#000000", quality: "exact" } | yes |
| it draws cleanly, and as the same Glow with the starting values written out | [] | yes |
| saving it again writes none of the new settings | [] | yes |

## Composite Original None: the light alone

| Check | The build's answer | Matches |
| --- | --- | --- |
| draws cleanly; a corner far from the disc is empty, the disc's middle is its light | [] corner [0, 0, 0, 0] middle [255, 255, 0, 255] | yes |
| with Intensity 0 nothing is left | [0, 0, 0, 0] | yes |

## Glow Dimensions

| Check | The build's answer | Matches |
| --- | --- | --- |
| Horizontal: 6 pixels right of the disc the grey is lit; 6 pixels above it, where its row has no disc, it stays 64 | [] right [77, 77, 64, 255] above [64, 64, 64, 255] | yes |
| Vertical: the other way round | [] right [64, 64, 64, 255] above [85, 85, 64, 255] | yes |
| Horizontal and Vertical: both lit | right [75, 75, 64, 255] above [82, 82, 64, 255] | yes |

## Glow Colors A & B: Color A red, Color B blue, Sawtooth A>B, Composite Original None

| Check | The build's answer | Matches |
| --- | --- | --- |
| the brightest light is Color A, red; the faint light 10 pixels out leans to Color B, blue | [] middle [255, 0, 0, 255] 10 out [1, 0, 254, 1] | yes |
| Sawtooth B>A turns the middle blue | [0, 0, 255, 255] | yes |
| Color Loops 3, Color Phase 90 and A & B Midpoint 25 draw cleanly and change it | [] | yes |
| with Original Colors, Color A and Color B do nothing |  | yes |

## Glow Operations

| Check | The build's answer | Matches |
| --- | --- | --- |
| Normal On Top: the light covers the disc, so its green rises from 160 towards the light's | [255, 255, 0, 255] | yes |
| Normal Behind: the opaque picture covers its light, so nothing changes |  | yes |
| Lighten On Top never darkens |  | yes |
| Add On Top lays the light on as Add Behind does |  | yes |
| normal on_top draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [72, 72, 61, 255] | yes |
| normal behind draws cleanly and leaves this picture as it was | []; the middle [255, 160, 40, 255], 6 pixels out [64, 64, 64, 255] | yes |
| add on_top draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [75, 75, 64, 255] | yes |
| add behind draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [75, 75, 64, 255] | yes |
| multiply on_top draws cleanly and changes the frame | []; the middle [255, 160, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |
| multiply behind draws cleanly and changes the frame | []; the middle [255, 160, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |
| screen on_top draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [72, 72, 64, 255] | yes |
| screen behind draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [72, 72, 64, 255] | yes |
| overlay on_top draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [67, 67, 61, 255] | yes |
| overlay behind draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [72, 72, 61, 255] | yes |
| soft_light on_top draws cleanly and changes the frame | []; the middle [255, 202, 6, 255], 6 pixels out [67, 67, 62, 255] | yes |
| soft_light behind draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [72, 72, 61, 255] | yes |
| hard_light on_top draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [72, 72, 61, 255] | yes |
| hard_light behind draws cleanly and changes the frame | []; the middle [255, 255, 0, 255], 6 pixels out [67, 67, 61, 255] | yes |
| darken on_top draws cleanly and changes the frame | []; the middle [255, 160, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |
| darken behind draws cleanly and changes the frame | []; the middle [255, 160, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |
| lighten on_top draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [72, 72, 64, 255] | yes |
| lighten behind draws cleanly and changes the frame | []; the middle [255, 255, 40, 255], 6 pixels out [72, 72, 64, 255] | yes |
| difference on_top draws cleanly and changes the frame | []; the middle [0, 95, 40, 255], 6 pixels out [70, 70, 64, 255] | yes |
| difference behind draws cleanly and changes the frame | []; the middle [0, 95, 40, 255], 6 pixels out [70, 70, 64, 255] | yes |
| hue on_top draws cleanly and changes the frame | []; the middle [197, 197, 0, 255], 6 pixels out [64, 64, 64, 255] | yes |
| hue behind draws cleanly and changes the frame | []; the middle [255, 222, 179, 255], 6 pixels out [71, 71, 71, 255] | yes |
| saturation on_top draws cleanly and leaves this picture as it was | []; the middle [255, 160, 40, 255], 6 pixels out [64, 64, 64, 255] | yes |
| saturation behind draws cleanly and changes the frame | []; the middle [251, 251, 36, 255], 6 pixels out [71, 71, 71, 255] | yes |
| color on_top draws cleanly and changes the frame | []; the middle [197, 197, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |
| color behind draws cleanly and changes the frame | []; the middle [255, 222, 179, 255], 6 pixels out [71, 71, 71, 255] | yes |
| luminosity on_top draws cleanly and changes the frame | []; the middle [255, 222, 179, 255], 6 pixels out [71, 71, 71, 255] | yes |
| luminosity behind draws cleanly and changes the frame | []; the middle [197, 197, 0, 255], 6 pixels out [64, 64, 61, 255] | yes |

## The card draws only what it can: Add or Screen, both directions, the light's own colours, the picture kept (in linear light)

| Check | The build's answer | Matches |
| --- | --- | --- |
| as before: Glows on the card, 1 | 1 | yes |
| Screen On Top: Glows on the card, 1 | 1 | yes |
| Composite Original None: Glows on the card, 0 | 0 | yes |
| Horizontal: Glows on the card, 0 | 0 | yes |
| A & B Colors: Glows on the card, 0 | 0 | yes |
| Overlay: Glows on the card, 0 | 0 | yes |

## Saved and opened again

| Check | The build's answer | Matches |
| --- | --- | --- |
| every setting comes back as written | ("overlay", GlowLook { composite: "on_top", dimensions: "vertical", glow_colors: "a_and_b", color_looping: "triangle_bab", color_loops: 4.0, color_phase: -45.0, midpoint: 70.0, color_a: "#12abef", color_b: "#fedcba", quality: "exact" }) | yes |

## Refused, with a warning, and the Glow left out (no silent fallback)

| Check | The build's answer | Matches |
| --- | --- | --- |
| composite "sideways" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| dimensions "diagonal" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| glow_colors "rainbow" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| color_looping "zigzag" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| color_loops 0 | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| color_loops 128 | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| color_phase 4000 | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| midpoint 101 | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| color_a "red" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |
| operation "color_dodge" | ["EFFECT_PARAMETER_INVALID Layer art's core.glow has a setting this build cannot use, so it is not drawn."] | yes |

## Result

61 of 61 checks pass.
