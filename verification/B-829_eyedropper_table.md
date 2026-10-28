# B-829: an eyedropper beside every colour field (D-949)

Written by `b829_eyedropper_beside_every_colour_field` in `app/src/tests/w12r.rs`.

The first rows run the page's own `wantsDropper` under Node on made-up fields; then the real window answers picks from `Fixtures/colour/view` with colour management off and on, each compared with the bytes the screen shows; the last rows pin the wiring. The private-app check is `verification/B-829_private_app_table.md`.

**14 of 14 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a colour field in a panel gets a dropper | true | true | pass |
| one in a dialog does not (the picture cannot be clicked under it) | false | false | pass |
| a field that has its dropper gets no second one | false | false | pass |
| a number field gets none | false | false | pass |
| nor a button | false | false | pass |
| off: a plain question is still answered only with off (FX-COLC7-004) | {"managed":false} | {"managed":false} | pass |
| off: a colour with pick=1 likewise (only a pixel is picked) | {"managed":false} | {"managed":false} | pass |
| off: picking 10, 10 gives the colour shown there | #0f0f0f, managed false | #0f0f0f, managed false | pass |
| off: picking 256, 128 gives the colour shown there | #0000bc, managed false | #0000bc, managed false | pass |
| off: picking 507, 251 gives the colour shown there | #ffff00, managed false | #ffff00, managed false | pass |
| on: pick=1 changes nothing in the answer | true | true | pass |
| every colour field added to the page is given its dropper | true | true | pass |
| a pick asks with pick=1 and takes any answer with a hex | true | true | pass |
| colour management off no longer stops a pick | true | true | pass |
