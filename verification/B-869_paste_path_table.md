# B-869: a path pasted into Position (D-989)

Written by `b869_paste_path_into_position` in `app/src/tests/w12t.rs`.

After Effects' paste of a Path into Position (tQ5wqg10x_I 1:12). `motionSends` is the page's own code run under Node; the square's requests go to the window's own command handler on the reference shot's first drawn layer without Position keys. The private-app check is `verification/B-869_private_app_table.md`.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| an open path of two points at frame 10, with a depth of 5: two keys two seconds apart, no roving | /property.set_base?layer=a&frame=10&drag=1&prop=position&value=10%2C20%2C5 then /property.set_base?layer=a&frame=58&drag=1&prop=position&value=30%2C40%2C5 | /property.set_base?layer=a&frame=10&drag=1&prop=position&value=10%2C20%2C5 then /property.set_base?layer=a&frame=58&drag=1&prop=position&value=30%2C40%2C5 | pass |
| a path of one point: nothing sent | 0 | 0 | pass |
| a closed square pasted at frame 0: a key on each corner and back to the first; entries to undo | 100,100 500,100 500,300 100,300 100,100; 1 | 100,100 500,100 500,300 100,300 100,100; 1 | pass |
| the first and last keys two seconds apart (24 frames a second) | 0, 48 | 0, 48 | pass |
| which keys rove | false, true, true, true, false | false, true, true, true, false | pass |
| the second corner keeps the path's handles | Some([0.0, -40.0, 0.0, 40.0]) | Some([0.0, -40.0, 0.0, 40.0]) | pass |
| one Ctrl+Z: no Position keys again | 0 | 0 | pass |
| pressing a property's name remembers it | true | true | pass |
| pressing a mask's or shape's name names its path | true | true | pass |
| Ctrl+C on a named path with no keys copies the path | true | true | pass |
| Ctrl+V with Position's name pressed pastes the path as motion | true | true | pass |
