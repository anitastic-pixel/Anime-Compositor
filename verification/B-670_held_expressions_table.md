# D-790: an expression on a colour, a point or a word

B-670, after lane w7e's replay of Smertimba's soft neon glow, whose seven Drop Shadows' Color read a Color Control by `effect("Color Control")("Color")` and whose project would not open (PROJECT_SCHEMA_INVALID). Now the project opens; the expression's text is kept and saved as read; the setting is drawn at its keys or constant, and every frame says EXPRESSION_TYPE naming the setting. Each expected frame is the same file without the expression, drawn as before D-790.

## It opens, draws as without it, and says so each frame

| Check | The build's answer | Matches |
| --- | --- | --- |
| Tint's Color, a linear triple, `[1, 0, 0]`: the project opens | opens | yes |
| Tint's Color, a linear triple, `[1, 0, 0]`: frame 2 is the frame without the expression | sample-identical | yes |
| Tint's Color, a linear triple, `[1, 0, 0]`: frame 2 says EXPRESSION_TYPE on color, the plain file nothing | ["EXPRESSION_TYPE The expression on art's Tint color does not work at frame 2."] / [] | yes |
| Tint's Color keyed from blue to red, with an expression, at frame 2: the project opens | opens | yes |
| Tint's Color keyed from blue to red, with an expression, at frame 2: frame 2 is the frame without the expression | sample-identical | yes |
| Tint's Color keyed from blue to red, with an expression, at frame 2: frame 2 says EXPRESSION_TYPE on color, the plain file nothing | ["EXPRESSION_TYPE The expression on art's Tint color does not work at frame 2."] / [] | yes |
| Drop Shadow's Color, a word, `effect("Color Control")("Color")` (Smertimba's neon): the project opens | opens | yes |
| Drop Shadow's Color, a word, `effect("Color Control")("Color")` (Smertimba's neon): frame 0 is the frame without the expression | sample-identical | yes |
| Drop Shadow's Color, a word, `effect("Color Control")("Color")` (Smertimba's neon): frame 0 says EXPRESSION_TYPE on color, the plain file nothing | ["EXPRESSION_TYPE The expression on art's Drop Shadow color does not work at frame 0."] / [] | yes |
| Twirl's Center, a point, `[4, 2]`: the project opens | opens | yes |
| Twirl's Center, a point, `[4, 2]`: frame 1 is the frame without the expression | sample-identical | yes |
| Twirl's Center, a point, `[4, 2]`: frame 1 says EXPRESSION_TYPE on center, the plain file nothing | ["EXPRESSION_TYPE The expression on art's Twirl center does not work at frame 1."] / [] | yes |

## It is saved as it was read

| Check | The build's answer | Matches |
| --- | --- | --- |
| Tint's Color, a linear triple, `[1, 0, 0]`: the setting's record is saved as read, expression text and all | {"base":[0,0,1],"expression":{"enabled":true,"text":"[1, 0, 0]"},"keyframes":[]} | yes |
| Tint's Color, a linear triple, `[1, 0, 0]`: opened and saved again, the same bytes | byte-identical | yes |
| Tint's Color keyed from blue to red, with an expression, at frame 2: the setting's record is saved as read, expression text and all | {"base":[0,0,1],"expression":{"enabled":true,"text":"[0, 1, 0]"},"keyframes":[{"frame":0,"interp":"linear","value":[0,0,1]},{"frame":4,"interp":"linear","value":[1,0,0]}]} | yes |
| Tint's Color keyed from blue to red, with an expression, at frame 2: opened and saved again, the same bytes | byte-identical | yes |
| Drop Shadow's Color, a word, `effect("Color Control")("Color")` (Smertimba's neon): the setting's record is saved as read, expression text and all | {"base":"#ff00f0","expression":{"enabled":true,"text":"effect(\"Color Control\")(\"Color\")"},"keyframes":[]} | yes |
| Drop Shadow's Color, a word, `effect("Color Control")("Color")` (Smertimba's neon): opened and saved again, the same bytes | byte-identical | yes |
| Twirl's Center, a point, `[4, 2]`: the setting's record is saved as read, expression text and all | {"base":[8,5],"expression":{"enabled":true,"text":"[4, 2]"},"keyframes":[]} | yes |
| Twirl's Center, a point, `[4, 2]`: opened and saved again, the same bytes | byte-identical | yes |

## Switched off, it is kept and nothing is said

| Check | The build's answer | Matches |
| --- | --- | --- |
| Tint's Color with its expression switched off: frame 2 as without it, nothing said | [] | yes |

## An export meets it

| Check | The build's answer | Matches |
| --- | --- | --- |
| the neon project's export is refused with EXPRESSION_TYPE, as any failing expression's is (document 28) | Blocked ["EXPRESSION_TYPE"] | yes |

## An old project

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_mix_001.json, written before D-790, opens and saves holding what it held | the same | yes |

## Result

23 of 23 checks pass.
