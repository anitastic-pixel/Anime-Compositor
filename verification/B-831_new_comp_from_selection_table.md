# B-831: New Comp from Selection (D-951)

Written by `b831_new_comp_from_selection` in `app/src/tests/w12r.rs`.

The first rows run the page's own `newCompSteps` under Node on a made-up composition and two made-up items; the window rows send those steps to a real window on the reference shot; the rest pin the wiring. The private-app check is `verification/B-831_private_app_table.md`.

**9 of 9 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| a composition: its own size, length and rate, then it as a layer, as one drag | /composition.create?name=Shot%201&width=1280&height=720&frames=48&fps=24000/1001&drag=1 then /layer.add_composition?of=c1&drag=1 then /property.drag_end | /composition.create?name=Shot%201&width=1280&height=720&frames=48&fps=24000/1001&drag=1 then /layer.add_composition?of=c1&drag=1 then /property.drag_end | pass |
| drawings: the app reads their size and count; the rate is the open composition's | /composition.create?name=Run&footage=a1&fps=12/1&drag=1 then /layer.create?asset=a1&name=Run&drag=1 then /property.drag_end | /composition.create?name=Run&footage=a1&fps=12/1&drag=1 then /layer.create?asset=a1&name=Run&drag=1 then /property.drag_end | pass |
| a sound makes nothing | 0 | 0 | pass |
| the window: New Comp from Selection on the reference shot's drawings | 1920x1080, 24 frames, 1 layer showing asset-layer2, 1 entry | 1920x1080, 24 frames, 1 layer showing asset-layer2, 1 entry | pass |
| one Ctrl+Z takes the composition and its layer away | true | true | pass |
| the window: from the open composition | 1 layers, the first a Some(Composition), true | 1 layers, the first a Some(Composition), true | pass |
| a drawing that is not there is refused | true | true | pass |
| in a Project item's right-click menu | true | true | pass |
| in the Composition menu | true | true | pass |
