# D-300 / B-185: Lightning Bolt has Composite on Original

Found by P-26, tutorial 2 (Video Copilot, Advanced Electric). The tutorial puts Advanced Lightning on a black solid, and the bolt shows over the shot straight away, because After Effects starts Composite on Original off: the solid's own black goes and the layer is the bolt alone. Ours always painted the bolt onto the solid, so the black hid the shot until the layer was set to Add.

## What changed

Lightning Bolt has a new setting in Effect Controls, **Composite on Original**:

| Setting | What it does |
|---|---|
| Off | the layer's own picture goes; only the bolt and its glow show, over whatever is beneath |
| On | the bolt is painted over the layer's own picture, as before |

A newly added Lightning Bolt starts **Off**, as in After Effects. A file saved before this has no such setting and draws as before, which is On; only Off is written into the file.

## Checks (cargo test)

`tests/b185_lightning_composite.rs`, 2 of 2 pass. Worked by hand on an opaque red 16x16 solid:

| Check | Expected | Got |
|---|---|---|
| A file without the setting | the same picture as On | identical |
| A pixel the bolt misses, Off | clear | clear |
| The same pixel, On | the red | the red |
| A pixel the bolt lights | On's red is Off's red plus the solid's, the same green and blue, On opaque | so, to 0.000001 |
| The file | On not written; a file that wrote On keeps it; Off kept | so |
| Composite "maybe" | refused with the reason | refused |

Lightning Bolt's own fixtures, FX-BOLT-001 to 032 (`tests/b126_lightning_bolt.rs`), still pass unchanged. The app suite (89 pass) sends Off from the panel, and the command accepts it.

## Pictures (the test copy, never the owner's app)

A 640x360 green "shot" solid, and a black solid above it with Lightning Bolt from the top middle to the bottom middle, Width 6, Glow 20.

| Picture | Settings | Look for | Pass? |
|---|---|---|---|
| `D-300 pictures/1_off_normal_bolt_over_the_shot.png` | Off, as added; blend Normal | the bolt over the green; no black anywhere | pass |
| `D-300 pictures/2_on_normal_black_hides_the_shot.png` | On; blend Normal | the old picture: the black solid hides the green | pass |
| `D-300 pictures/3_on_add_as_before.png` | On; blend Add | the old work-around still gives the bolt over the green | pass |

The window run also read the new effect's setting as it was added: "new Lightning Bolt starts with composite off".

## For the owner to try

1. Put a shot or a coloured solid at the bottom, add a black solid above it, and add Lightning Bolt to the black solid.
2. The bolt shows over the shot at once, without changing the blend mode.
3. Set Composite on Original to On: the black comes back and hides the shot.

Still proposed, not built: Advanced Lightning's Turbulence, Decay, Conductivity and its lightning types.

Fixtures are unchanged.
