# B-150c before the fix: what the window did with a stretched layer's keys

Kept by hand from the first run of `a_stretched_layers_keys_are_shown_and_set_where_they_play` in `app/src/main.rs`, on 2026-09-29, with the checks written and nothing fixed yet. Each FAIL row is the owner's "key diamonds" gap: the Actual column is where the key went, in the layer's own unstretched frame numbers. The table after the fix is `verification/B-150c_key_diamonds_table.md`.

(The graph row passed by coincidence: with the key stored in the wrong place and the graph read the old way, the two mistakes cancel on a straight line.)

| Check | Expected | Actual | Result |
| at 100% a key set with the playhead on frame 12 and dragged to 20 is stored on 20, as before | [0,0]@20 linear | [0,0]@20 linear | pass |
| at 200% a key set with the playhead on frame 12 is stored at 8, which plays on 12: 4 + (8 - 4) x 200 / 100 | [0,0]@8 linear | [0,0]@12 linear | FAIL |
| the timeline draws every key where it plays, by that rule | present | absent | FAIL |
| and a mask's or a shape's path keys too, on their own rows and on their diamond | 2 | 0 | FAIL |
| the key named by the frame it is shown on, 12, is the one held | [0,0]@8 hold | [0,0]@12 hold | FAIL |
| dragged from 12 to 20 on the timeline, it is stored at 12, which plays on 20 | [0,0]@12 hold | [0,0]@20 hold | FAIL |
| chosen and dragged four frames on, it plays on 24, stored at 14 | [0,0]@14 hold | [0,0]@24 hold | FAIL |
| its diamond pressed with the playhead on 24 takes it off |  |  | pass |
| an opacity keyed on 4 and typed to 0 on frame 12 has keys stored at 4 and 8 | 1@4 linear, 0@8 linear | 1@4 linear, 0@12 linear | FAIL |
| the graph draws it as it plays: 1 on 4, 0.5 on 8, 0.25 on 10, 0 on 12 | 1, 0.5, 0.25, 0 | 1, 0.5, 0.25, 0 | pass |
| a mask's path keyed with the playhead on 12 is stored at 8 too | 8 | 12 | FAIL |
| and dragged four frames on it plays on 16, stored at 10 | 10 | 16 | FAIL |
| at 50% a key set with the playhead on frame 9 is stored at 14, which plays on 9: 4 + (14 - 4) x 50 / 100 | 0@14 linear | 0@9 linear | FAIL |
| a key stored at 15 plays half way between 9 and 10, is shown there, and is held when named 9.5 | 0@14 linear, 0@15 hold | 0@9 linear, 0@15 linear | FAIL |
| dragged two frames on it plays on 11.5, stored at 19 | 0@14 linear, 0@19 hold | 0@9 linear, 0@15 linear | FAIL |
| a half-way key chosen on the timeline puts the playhead on a whole frame | present | absent | FAIL |
| set back to 100%, a key set on frame 12 is stored on 12 again | [100,100]@12 linear | [100,100]@12 linear | pass |
