# B-826: parent and track matte on every chosen layer, and a matte pick whip (D-946)

Written by `b826_parent_and_matte_on_every_chosen_layer` in `app/src/tests/w12r.rs`.

The first rows run the page's own `manyRequests`, `parentRequest`, `matteRequest` and `matteOnly` under Node on made-up layers; one row asks the real window; the rest pin the lines that wire them in. The private-app check is `verification/B-826_private_app_table.md`.

**20 of 20 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| parent M set on A with A, C, B chosen: all three, in the order chosen | /layer.set_parent?layer=a&parent=m&frame=12 /layer.set_parent?layer=c&parent=m&frame=12 /layer.set_parent?layer=b&parent=m&frame=12 | /layer.set_parent?layer=a&parent=m&frame=12 /layer.set_parent?layer=c&parent=m&frame=12 /layer.set_parent?layer=b&parent=m&frame=12 | pass |
| set on A chosen alone: A only | /layer.set_parent?layer=a&parent=m&frame=12 | /layer.set_parent?layer=a&parent=m&frame=12 | pass |
| set on A when A is not among the chosen: A only | /layer.set_parent?layer=a&parent=m&frame=12 | /layer.set_parent?layer=a&parent=m&frame=12 | pass |
| the parent itself among the chosen is left out | /layer.set_parent?layer=a&parent=m&frame=12 | /layer.set_parent?layer=a&parent=m&frame=12 | pass |
| a chosen layer the parent rides on is left out (no loop) | /layer.set_parent?layer=a&parent=p&frame=12 | /layer.set_parent?layer=a&parent=p&frame=12 | pass |
| a sound among the chosen is left out | /layer.set_parent?layer=a&parent=m&frame=12 | /layer.set_parent?layer=a&parent=m&frame=12 | pass |
| None clears every chosen layer's parent | /layer.set_parent?layer=a&parent=&frame=12 /layer.set_parent?layer=c&parent=&frame=12 | /layer.set_parent?layer=a&parent=&frame=12 /layer.set_parent?layer=c&parent=&frame=12 | pass |
| matte M on A, C, M and a sound: A and C | /layer.set_matte?layer=a&matte=m&only=true /layer.set_matte?layer=c&matte=m&only=true | /layer.set_matte?layer=a&matte=m&only=true /layer.set_matte?layer=c&matte=m&only=true | pass |
| a null is never a matte |  |  | pass |
| a first matte is matte only (its eye off) | true | true | pass |
| changing a matte keeps the tick as it was (off) | false | false | pass |
| changing a matte keeps the tick as it was (on) | true | true | pass |
| clearing a matte keeps the tick as it was | false | false | pass |
| the window: 3 layers given one parent with drag=1, then drag_end | 3 parented, 1 entry to undo | 3 parented, 1 entry to undo | pass |
| the Parent list sends for every chosen layer | true | true | pass |
| the Track matte list too, hiding a first matte | true | true | pass |
| several requests are one entry to undo | true | true | pass |
| the track matte has a pick whip | true | true | pass |
| which sends for every chosen layer | true | true | pass |
| and so does the parent whip | true | true | pass |
