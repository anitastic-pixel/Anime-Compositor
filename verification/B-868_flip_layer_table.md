# B-868: Flip the layer horizontally or vertically (D-988)

Written by `b868_flip_layer` in `app/src/tests/w12t.rs`.

After Effects' Layer > Transform > Flip Horizontal and Flip Vertical (lO54j6So7dA 2:23). `flipSends` is the page's own code run under Node; its requests go to the window's own command handler on the reference shot's first drawn layer. The private-app check is `verification/B-868_private_app_table.md`.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| two layers flipped vertically at frame 12: each Scale's height turned over, depth kept | /property.set_base?layer=a&frame=12&drag=1&prop=scale&value=50%2C80 then /property.set_base?layer=b%20b&frame=12&drag=1&prop=scale&value=-100%2C-100%2C100 | /property.set_base?layer=a&frame=12&drag=1&prop=scale&value=50%2C80 then /property.set_base?layer=b%20b&frame=12&drag=1&prop=scale&value=-100%2C-100%2C100 | pass |
| nothing selected: nothing sent | 0 | 0 | pass |
| the drawn layer's Scale to begin with | 100, 100 | 100, 100 | pass |
| Flip horizontally: Scale and the entries to undo | -100, 100; 1 | -100, 100; 1 | pass |
| one Ctrl+Z: as it started | 100, 100 | 100, 100 | pass |
| Flip vertically | 100, -100 | 100, -100 | pass |
| in the Layer menu: horizontally | true | true | pass |
| in the Layer menu: vertically | true | true | pass |
| in the command finder: horizontally | true | true | pass |
| in the command finder: vertically | true | true | pass |
| the selected unlocked layers, at the playhead | true | true | pass |
