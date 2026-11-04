# B-968: Source Text expressions (D-1088)

Document 25's FX-EXPR-022, from `tools/expression_reference.py`, against the build (`tests/b968_source_text_expression.rs`). Each row is the Source Text expression of the text layer Words in `Fixtures/projects/source_text_expression_project.json`, whose own words are "25%"; each cell is the words drawn at that frame, and the diagnostic if the expression fails. A cell passes when the build's is exactly the expected.

| Words Source Text expression | frame | expected | build | result |
| --- | --- | --- | --- | --- |
| `thisComp.layer("Controls").effect("Slider Control")("Slider")` | 0 | "0" | "0" | pass |
| `thisComp.layer("Controls").effect("Slider Control")("Slider")` | 7 | "14.583333333333334" | "14.583333333333334" | pass |
| `thisComp.layer("Controls").effect("Slider Control")("Slider")` | 12 | "25" | "25" | pass |
| `thisComp.layer("Controls").effect("Slider Control")("Slider")` | 48 | "100" | "100" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider"))` | 0 | "0" | "0" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider"))` | 7 | "15" | "15" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider"))` | 12 | "25" | "25" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider"))` | 48 | "100" | "100" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider")) + "%"` | 0 | "0%" | "0%" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider")) + "%"` | 7 | "15%" | "15%" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider")) + "%"` | 12 | "25%" | "25%" | pass |
| `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider")) + "%"` | 48 | "100%" | "100%" | pass |
| `"Frame " + time * 24` | 0 | "Frame 0" | "Frame 0" | pass |
| `"Frame " + time * 24` | 7 | "Frame 7" | "Frame 7" | pass |
| `"Frame " + time * 24` | 12 | "Frame 12" | "Frame 12" | pass |
| `"Frame " + time * 24` | 48 | "Frame 48" | "Frame 48" | pass |
| `value + " at " + Math.floor(time) + "s"` | 0 | "25% at 0s" | "25% at 0s" | pass |
| `value + " at " + Math.floor(time) + "s"` | 7 | "25% at 0s" | "25% at 0s" | pass |
| `value + " at " + Math.floor(time) + "s"` | 12 | "25% at 0s" | "25% at 0s" | pass |
| `value + " at " + Math.floor(time) + "s"` | 48 | "25% at 2s" | "25% at 2s" | pass |
| `thisComp.layer("Target").position` | 0 | "300,400" | "300,400" | pass |
| `thisComp.layer("Target").position` | 7 | "300,400" | "300,400" | pass |
| `thisComp.layer("Target").position` | 12 | "300,400" | "300,400" | pass |
| `thisComp.layer("Target").position` | 48 | "300,400" | "300,400" | pass |
| `1 + 2 + "a" + 1 + 2` | 0 | "3a12" | "3a12" | pass |
| `1 + 2 + "a" + 1 + 2` | 7 | "3a12" | "3a12" | pass |
| `1 + 2 + "a" + 1 + 2` | 12 | "3a12" | "3a12" | pass |
| `1 + 2 + "a" + 1 + 2` | 48 | "3a12" | "3a12" | pass |
| `s = "Score "; s + 10` | 0 | "Score 10" | "Score 10" | pass |
| `s = "Score "; s + 10` | 7 | "Score 10" | "Score 10" | pass |
| `s = "Score "; s + 10` | 12 | "Score 10" | "Score 10" | pass |
| `s = "Score "; s + 10` | 48 | "Score 10" | "Score 10" | pass |
| `time / 1e7` | 0 | "0" | "0" | pass |
| `time / 1e7` | 7 | "2.916666666666667e-8" | "2.916666666666667e-8" | pass |
| `time / 1e7` | 12 | "5e-8" | "5e-8" | pass |
| `time / 1e7` | 48 | "2e-7" | "2e-7" | pass |
| `time * 1e21` | 0 | "0" | "0" | pass |
| `time * 1e21` | 7 | "291666666666666700000" | "291666666666666700000" | pass |
| `time * 1e21` | 12 | "500000000000000000000" | "500000000000000000000" | pass |
| `time * 1e21` | 48 | "2e+21" | "2e+21" | pass |
| `-time * 1.5` | 0 | "0" | "0" | pass |
| `-time * 1.5` | 7 | "-0.4375" | "-0.4375" | pass |
| `-time * 1.5` | 12 | "-0.75" | "-0.75" | pass |
| `-time * 1.5` | 48 | "-3" | "-3" | pass |
| `time / 0` | 0 | "NaN" | "NaN" | pass |
| `time / 0` | 7 | "Infinity" | "Infinity" | pass |
| `time / 0` | 12 | "Infinity" | "Infinity" | pass |
| `time / 0` | 48 | "Infinity" | "Infinity" | pass |
| `"x" - 1` | 0 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `"x" - 1` | 7 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `"x" - 1` | 12 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `"x" - 1` | 48 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `thisComp.layer("Nobody").position` | 0 | "25%", EXPRESSION_REFERENCE_MISSING | "25%", EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer("Nobody").position` | 7 | "25%", EXPRESSION_REFERENCE_MISSING | "25%", EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer("Nobody").position` | 12 | "25%", EXPRESSION_REFERENCE_MISSING | "25%", EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer("Nobody").position` | 48 | "25%", EXPRESSION_REFERENCE_MISSING | "25%", EXPRESSION_REFERENCE_MISSING | pass |
| `thisComp.layer("Controls")` | 0 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `thisComp.layer("Controls")` | 7 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `thisComp.layer("Controls")` | 12 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `thisComp.layer("Controls")` | 48 | "25%", EXPRESSION_TYPE | "25%", EXPRESSION_TYPE | pass |
| `thisProperty` | 0 | "25%", EXPRESSION_SYNTAX | "25%", EXPRESSION_SYNTAX | pass |
| `thisProperty` | 7 | "25%", EXPRESSION_SYNTAX | "25%", EXPRESSION_SYNTAX | pass |
| `thisProperty` | 12 | "25%", EXPRESSION_SYNTAX | "25%", EXPRESSION_SYNTAX | pass |
| `thisProperty` | 48 | "25%", EXPRESSION_SYNTAX | "25%", EXPRESSION_SYNTAX | pass |
| the tutorial's, switched off | 0 | "25%" | "25%" | pass |
| the tutorial's, switched off | 7 | "25%" | "25%" | pass |
| the tutorial's, switched off | 12 | "25%" | "25%" | pass |
| the tutorial's, switched off | 48 | "25%" | "25%" | pass |

68 of 68 cells pass.
