# B-825: three shortcut fixes (D-945)

Written by `b825_shortcut_fixes` in `app/src/tests/w12r.rs`.

The first rows run the page's own `easeSide` and `ctrlClickTo` under Node; the rest pin the lines that wire them in. The private-app check is `verification/B-825_private_app_table.md`.

**20 of 20 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| F9 eases | both | both | pass |
| Shift+F9 eases the side reached from | in | in | pass |
| Ctrl+Shift+F9 eases the side left by (the defect named: it is out, not read as anything else) | out | out | pass |
| Ctrl+F9 alone stays out, as before | out | out | pass |
| Ctrl+click on a key with nothing set: auto bezier | auto | auto | pass |
| Ctrl+click on a plain linear key: auto bezier | auto | auto | pass |
| Ctrl+click on an eased key: linear | linear | linear | pass |
| Ctrl+click on a held key: linear | linear | linear | pass |
| Ctrl+click on an auto bezier key: linear | linear | linear | pass |
| Ctrl+click on a continuous key: linear | linear | linear | pass |
| F9 goes through easeSide | true | true | pass |
| the key menu names Ctrl+Shift+F9 | true | true | pass |
| the palette has Easy ease out | true | true | pass |
| Ctrl+M adds to the render queue | true | true | pass |
| Ctrl+Alt+M exports, tested first | true | true | pass |
| the Export menu line says Ctrl+Alt+M | true | true | pass |
| the Add to Render queue line says Ctrl+M | true | true | pass |
| the Render button no longer claims Ctrl+M | true | true | pass |
| Shift adds a key; Ctrl alone no longer does | true | true | pass |
| a Ctrl click that did not move toggles | true | true | pass |
