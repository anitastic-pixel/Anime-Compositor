# B-911: a layer by its name or number, and a name holding a layer (D-1033)

Document 25's FX-EXPR-021, from `tools/expression_reference.py`, against the build (`tests/b911_expression_layer_names.rs`). Each row is Spin's rotation expression in `Fixtures/projects/expression_project.json` at frame 12. Tolerance 1e-9.

| Spin rotation expression | expected | build | diagnostic expected | build | result |
| --- | --- | --- | --- | --- | --- |
| `thisComp.layer("Lead").position[0]` | 480 | 480 | none | none | pass |
| `thisComp.layer("layer-lead").position[0]` | 480 | 480 | none | none | pass |
| `thisComp.layer(5).position[0]` | 480 | 480 | none | none | pass |
| `thisComp.layer(1).opacity` | 100 | 100 | none | none | pass |
| `thisComp.layer(9).position[1]` | 540 | 540 | none | none | pass |
| `var L = thisComp.layer("Lead");\nL.position[0] / 10` | 48 | 48 | none | none | pass |
| `target = thisComp.layer("Lead");\n(target.position[0] - (thisComp.width / 2)) / thisComp.width` | -0.25 | -0.25 | none | none | pass |
| `let L = thisComp.layer(5); L.transform.position.valueAtTime(0)[0]` | 0 | 0 | none | none | pass |
| `me = thisLayer; me.position[0]` | 960 | 960 | none | none | pass |
| `c = thisComp.activeCamera; c.zoom / 100` | 19.2 | 19.2 | none | none | pass |
| `thisComp.layer("Lead").position[0]` (Fade renamed Lead, above the real Lead) | 960 | 960 | none | none | pass |
| `thisComp.layer("Nobody").position[0]` | 0 | 0 | EXPRESSION_REFERENCE_MISSING | EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer(0).position[0]` | 0 | 0 | EXPRESSION_REFERENCE_MISSING | EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer(10).position[0]` | 0 | 0 | EXPRESSION_REFERENCE_MISSING | EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer(2.5).position[0]` | 0 | 0 | EXPRESSION_REFERENCE_MISSING | EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer([1, 2]).position[0]` | 0 | 0 | EXPRESSION_TYPE | EXPRESSION_TYPE | pass |
| `L = thisComp.layer(5); L` | 0 | 0 | EXPRESSION_TYPE | EXPRESSION_TYPE | pass |
| `L = thisComp.layer(5); L + 1` | 0 | 0 | EXPRESSION_TYPE | EXPRESSION_TYPE | pass |

18 of 18 rows pass.
