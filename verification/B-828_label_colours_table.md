# B-828: After Effects' sixteen label colours (D-948)

Written by `b828_sixteen_label_colours` in `app/src/tests/w12r.rs`.

The first rows run the page's own `LABELS` and `LABEL_NAMES` under Node; then the real window sets and refuses labels and the project is saved and opened again; the last row pins the menu's layout. The private-app check is `verification/B-828_private_app_table.md`.

**10 of 10 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| label colours besides none | 16 | 16 | pass |
| their names, in After Effects' order | red, yellow, aqua, pink, lavender, peach, sea foam, blue, green, purple, orange, brown, fuchsia, cyan, sandstone, dark green | red, yellow, aqua, pink, lavender, peach, sea foam, blue, green, purple, orange, brown, fuchsia, cyan, sandstone, dark green | pass |
| the first eight keep the colours they had (a saved 1 to 8 looks as before) | #c8554f #d9c75b #79b8b0 #d99bb5 #9f96c9 #d9a878 #8fb58f #6d86d9 | #c8554f #d9c75b #79b8b0 #d99bb5 #9f96c9 #d9a878 #8fb58f #6d86d9 | pass |
| the new eight, After Effects' 9 to 16 | #4aa44c #8e2c9a #e8920d #7f452a #f46dd6 #3da2a5 #a89677 #1e401e | #4aa44c #8e2c9a #e8920d #7f452a #f46dd6 #3da2a5 #a89677 #1e401e | pass |
| the window: layer.set_label label=16 (dark green) | label 16 | label 16 | pass |
| the window: label=17 is refused and the label stays | refused (1 to 16), label 16 | refused (1 to 16), label 16 | pass |
| saved and opened again, the layer's label | 16 | 16 | pass |
| an older file's label 8 (blue) opens as 8 | 8 | 8 | pass |
| a file with label 17 is refused, not quietly changed | true | true | pass |
| the label squares sit nine to a row | true | true | pass |
