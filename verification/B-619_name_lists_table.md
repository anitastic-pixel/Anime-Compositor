# B-619: the page's name lists against the core's

Written by `app/src/tests/name_lists.rs`. Each line is one list the window keeps a copy of: what the core says, what the page says, and whether they are the same. A range written `null` has no end.

| List | The core | The page | Same |
| --- | --- | --- | --- |
| SHAPE_PROPS | ["fill_start","fill_end","stroke_start","stroke_end","trim_start","trim_end","trim_offset","fill_color","fill_opacity","stroke_color","stroke_opacity","stroke_width","path_size","path_position","path_roundness","path_points","path_rotation","path_inner_radius","path_outer_radius","path_inner_roundness","path_outer_roundness"] | ["fill_start","fill_end","stroke_start","stroke_end","trim_start","trim_end","trim_offset","fill_color","fill_opacity","stroke_color","stroke_opacity","stroke_width","path_size","path_position","path_roundness","path_points","path_rotation","path_inner_radius","path_outer_radius","path_inner_roundness","path_outer_roundness"] | yes |
| MASK_PROPS | ["feather","opacity","expansion"] | ["feather","opacity","expansion"] | yes |
| GROUP_PROPS | ["anchor","position","scale","rotation","opacity","skew","skew_axis"] | ["anchor","position","scale","rotation","opacity","skew","skew_axis"] | yes |
| REPEATER_PROPS | ["copies","offset","anchor","position","scale","rotation","start_opacity","end_opacity"] | ["copies","offset","anchor","position","scale","rotation","start_opacity","end_opacity"] | yes |
| PARAMETRIC_SETTINGS, the kinds | ["ellipse","polygon","rectangle","star"] | ["ellipse","polygon","rectangle","star"] | yes |
| PARAMETRIC_SETTINGS, rectangle | ["size","position","roundness"] | ["size","position","roundness"] | yes |
| PARAMETRIC_SETTINGS, ellipse | ["size","position"] | ["size","position"] | yes |
| PARAMETRIC_SETTINGS, polygon | ["points","position","rotation","outer_radius","outer_roundness"] | ["points","position","rotation","outer_radius","outer_roundness"] | yes |
| PARAMETRIC_SETTINGS, star | ["points","position","rotation","inner_radius","outer_radius","inner_roundness","outer_roundness"] | ["points","position","rotation","inner_radius","outer_radius","inner_roundness","outer_roundness"] | yes |
| OPERATIONS, the types | ["merge","offset","pucker_bloat","round_corners","twist","wiggle","zig_zag"] | ["merge","offset","pucker_bloat","round_corners","twist","wiggle","zig_zag"] | yes |
| OPERATIONS, round_corners: numbers and ranges | [["radius",0.0,100000.0]] | [["radius",0,100000]] | yes |
| OPERATIONS, round_corners: choice and words | null | null | yes |
| OPERATIONS, pucker_bloat: numbers and ranges | [["amount",-100.0,100.0]] | [["amount",-100,100]] | yes |
| OPERATIONS, pucker_bloat: choice and words | null | null | yes |
| OPERATIONS, twist: numbers and ranges | [["angle",null,null],["center",null,null]] | [["angle",null,null],["center",null,null]] | yes |
| OPERATIONS, twist: choice and words | null | null | yes |
| OPERATIONS, zig_zag: numbers and ranges | [["size",0.0,10000.0],["ridges",0.0,100.0]] | [["size",0,10000],["ridges",0,100]] | yes |
| OPERATIONS, zig_zag: choice and words | ["points",["corner","smooth"]] | ["points",["corner","smooth"]] | yes |
| OPERATIONS, wiggle: numbers and ranges | [["size",0.0,10000.0],["detail",0.0,100.0],["wiggles_per_second",0.0,1000.0],["correlation",0.0,100.0],["seed",0.0,100000.0]] | [["size",0,10000],["detail",0,100],["wiggles_per_second",0,1000],["correlation",0,100],["seed",0,100000]] | yes |
| OPERATIONS, wiggle: choice and words | null | null | yes |
| OPERATIONS, offset: numbers and ranges | [["amount",-10000.0,10000.0],["miter_limit",1.0,100.0]] | [["amount",-10000,10000],["miter_limit",1,100]] | yes |
| OPERATIONS, offset: choice and words | ["join",["miter","round","bevel"]] | ["join",["miter","round","bevel"]] | yes |
| OPERATIONS, merge: numbers and ranges | [] | [] | yes |
| OPERATIONS, merge: choice and words | ["mode",["add","merge","subtract","intersect","exclude"]] | ["mode",["add","merge","subtract","intersect","exclude"]] | yes |

B-619: 24 of 24 lists match.
