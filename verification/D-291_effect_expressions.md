# D-291 / B-176: expressions on an effect's number settings

Found by P-26 (the After Effects tutorial playtest). The Shockwave and Advanced Electric tutorials type `time*150` into Fractal Noise's Evolution and `wiggle` into Turbulent Displace. Before this change the window answered `"fx:fx-1:evolution" is not a property that can carry an expression`, and a file holding one was refused on load.

## What changed

- `src/effects.rs`, `src/command.rs`: an effect setting can hold an expression beside its keys (`SetEffectExpression`). Changing the keys keeps the expression, and setting an expression keeps the keys. A colour, a point or the Mix is refused in a sentence.
- `src/expr.rs` (`effect_at`): runs each setting's expression with document 09's language. `value` is the keyed value, or the typed number when there are no keys. The answer is held within the setting's range.
- `src/compose.rs`: frames are drawn with it. A failing expression is reported and the setting's own number is used.
- `src/export.rs`: an export over a failing frame is refused, as for a transform.
- `src/persist.rs`: files save and load it (document 19, new D-291 paragraph).
- `app/src/main.rs`, `app/ui/index.html`: to add an expression, Alt-click a number setting's stopwatch in Effect Controls. The box opens under it. The panel's number now follows the playhead for keyed and expression settings. Before, it stayed at the number from when the panel was drawn.

## Checks

`tests/b176_effect_expressions.rs`: 4 of 4 pass. Each picture is compared with the picture of the plain number it should come to, drawn by the code that existed before D-291.

| Check | Expected | Result |
|---|---|---|
| Blur `value*2` on 1, frame 2 | same pixels as Blur 2 | identical |
| Blur `time*24`, frame 3 (24 fps) | same pixels as Blur 3 | identical |
| Keys 2 to 6 over frames 0 to 4, `value+1`, frame 2 | same pixels as Blur 5 | identical |
| `-40` | held at the range end: Blur 0 | identical |
| `value*100` switched off | Blur 2, the typed number | identical |
| Save, load, save | `{"text": "wiggle(2, 1)", "enabled": true}` kept; second save byte-identical | yes |
| An expression on Tint's colour in a file | refused: "a colour or a point takes keys only" | refused |
| `nonsense(` | frame drawn as Blur 2, warning "art's Blur sigma_px does not work at frame 2" | yes |
| Exporting frames 0 to 2 with it | refused, 0 files written | refused, 0 files |
| Keys, then an expression, then new keys, then no keys | each keeps the other; removing both leaves the plain setting | yes |
| An expression on the Mix, or on a choice such as `edges` | refused | refused |

App check `an_effect_setting_takes_an_expression`: set, switch off, remove, a colour refused with "Tint's color cannot carry an expression; it takes keys only.", and an unknown effect refused.

## In the window (a test copy, own profile)

These steps followed Shockwave: a 1920x1080 composition at 23.976 fps, a black solid with Fractal Noise, then Alt-click on Evolution's stopwatch and type `time*150`.

- The stopwatch's tip reads "Alt-click adds an expression". Alt-click opened a box holding `value`, with the keyboard in it.
- The file then holds `{"base":0,"expression":{"enabled":true,"text":"time*150"},"keyframes":[]}`.
- At frame 24 the panel reads 150.15, which is 24 / 23.976 seconds x 150. The noise moved between frames 0 and 24: the mean change was 23 of 255 grey levels.
- The Dark Colour setting does not offer an expression.
- Typing `time*` showed, in red, "EXPRESSION_SYNTAX: Expected a value, found the end. The Fractal Noise Evolution is drawn at its keyed value."

Pictures in `verification/D-291 pictures/`:
- `evolution_f0.png`, `evolution_f24.png`: the whole window at frames 0 and 24.
- `evolution_f24_panel.png`: the panel with the box.
- `evolution_broken_panel.png`: the error.

## For the owner to try

1. Make a solid and add Fractal Noise.
2. In Effect Controls, hold Alt and click the stopwatch beside Evolution. A box opens under it.
3. Type `time*150` and click away. Press play: the noise churns by itself, with no keys.
4. Type `time*` instead: a red sentence says what is wrong, and the noise stops moving.
5. Click the `=` beside Evolution to switch the expression off and on.
6. Save, close and reopen the project: the expression is still there.
7. Hold Alt and click Dark Colour's stopwatch: nothing opens, because a colour takes keys only.

Projects without expressions on effects save and draw exactly as before. No fixture changed.

Not in this step: expressions on a mask's feather, opacity and expansion (P-26 T1-2), and the pick whip from an effect setting.
