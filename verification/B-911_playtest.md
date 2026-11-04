# B-911 playtest: expressions find a layer by its name or number

What changed (D-1033): an expression can now name a layer the way After Effects tutorials type it.

1. Make a composition with two layers. Rename the top one `Null 1` (F2) and give it a few
   Position keys so it moves across the frame.
2. On the other layer, open Position's expression (Alt+click the stopwatch) and type
   `thisComp.layer("Null 1").position`.
   - Expect: no warning; the layer follows Null 1 exactly as it moves.
3. Change the expression to `thisComp.layer(1).position`.
   - Expect: the same, because Null 1 is layer 1, the top of the timeline.
4. Change it to the tutorial's two lines:
   `target = thisComp.layer("Null 1");`
   `[target.position[0], 540]`
   - Expect: the layer follows Null 1 left and right only, staying at the middle height.
5. Rename Null 1 to something else.
   - Expect: the expression now shows a warning that there is no layer named "Null 1", and the
     layer stays where its own keys put it. (A link made by dragging the expression's link onto
     another property keeps working after a rename, because it remembers the layer itself.)
6. Type `thisComp.layer(9).position` in a composition with fewer than 9 layers.
   - Expect: a warning that there is no layer number 9.

Not built yet: reading a shape layer's contents, such as `content("Repeater 1").copies`
(D-1034 is a proposal for you). Typing the number in works today.

The checks are in `verification/B-911_expression_layer_names_table.md` (18 of 18): each row is
one expression, its value from the reference tool and the build's.
