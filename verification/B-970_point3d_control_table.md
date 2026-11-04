# B-970: 3D Point Control (D-1090)

Document 25's FX-EXPR-023, from `tools/expression_reference.py`, against the build (`tests/b970_point3d_control.rs`). Point3D's 3D Point Control runs from (0, 0, 0) at frame 0 to (480, -240, 960) at frame 48. Each row is one expression on the property named; each cell is the value at that frame in the file's units, and the diagnostic if the expression fails. A cell passes when the build's is exactly the expected.

| property | expression | frame | expected | build | result |
| --- | --- | --- | --- | --- | --- |
| Point3D rotation | `effect("3D Point Control")("3D Point")[0]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[0]` | 12 | 120 | 120 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[0]` | 24 | 240 | 240 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[0]` | 48 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[1]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[1]` | 12 | -60 | -60 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[1]` | 24 | -120 | -120 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[1]` | 48 | -240 | -240 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[2]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[2]` | 12 | 240 | 240 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[2]` | 24 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[2]` | 48 | 960 | 960 | pass |
| Point3D rotation | `effect("ADBE Point3D Control")(1)[2]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect("ADBE Point3D Control")(1)[2]` | 12 | 240 | 240 | pass |
| Point3D rotation | `effect("ADBE Point3D Control")(1)[2]` | 24 | 480 | 480 | pass |
| Point3D rotation | `effect("ADBE Point3D Control")(1)[2]` | 48 | 960 | 960 | pass |
| Point3D rotation | `effect(1)("ADBE Point3D Control-0001")[0]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect(1)("ADBE Point3D Control-0001")[0]` | 12 | 120 | 120 | pass |
| Point3D rotation | `effect(1)("ADBE Point3D Control-0001")[0]` | 24 | 240 | 240 | pass |
| Point3D rotation | `effect(1)("ADBE Point3D Control-0001")[0]` | 48 | 480 | 480 | pass |
| Point3D rotation | `p = effect("3D Point Control")("3D Point"); p[0] + p[1] + p[2]` | 0 | 0 | 0 | pass |
| Point3D rotation | `p = effect("3D Point Control")("3D Point"); p[0] + p[1] + p[2]` | 12 | 300 | 300 | pass |
| Point3D rotation | `p = effect("3D Point Control")("3D Point"); p[0] + p[1] + p[2]` | 24 | 600 | 600 | pass |
| Point3D rotation | `p = effect("3D Point Control")("3D Point"); p[0] + p[1] + p[2]` | 48 | 1200 | 1200 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").valueAtTime(1)[2]` | 0 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").valueAtTime(1)[2]` | 12 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").valueAtTime(1)[2]` | 24 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").valueAtTime(1)[2]` | 48 | 480 | 480 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").value[1]` | 0 | 0 | 0 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").value[1]` | 12 | -60 | -60 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").value[1]` | 24 | -120 | -120 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point").value[1]` | 48 | -240 | -240 | pass |
| Controls rotation | `thisComp.layer("Point3D").effect("3D Point Control")("3D Point")[2] / 10` | 0 | 0 | 0 | pass |
| Controls rotation | `thisComp.layer("Point3D").effect("3D Point Control")("3D Point")[2] / 10` | 12 | 24 | 24 | pass |
| Controls rotation | `thisComp.layer("Point3D").effect("3D Point Control")("3D Point")[2] / 10` | 24 | 48 | 48 | pass |
| Controls rotation | `thisComp.layer("Point3D").effect("3D Point Control")("3D Point")[2] / 10` | 48 | 96 | 96 | pass |
| Point3D position | `p = effect("3D Point Control")("3D Point"); [p[0] + 960, p[1] + 540]` | 0 | (960, 540) | (960, 540) | pass |
| Point3D position | `p = effect("3D Point Control")("3D Point"); [p[0] + 960, p[1] + 540]` | 12 | (1080, 480) | (1080, 480) | pass |
| Point3D position | `p = effect("3D Point Control")("3D Point"); [p[0] + 960, p[1] + 540]` | 24 | (1200, 420) | (1200, 420) | pass |
| Point3D position | `p = effect("3D Point Control")("3D Point"); [p[0] + 960, p[1] + 540]` | 48 | (1440, 300) | (1440, 300) | pass |
| Point3D depth | `effect("3D Point Control")("3D Point")[2]` | 0 | 0 | 0 | pass |
| Point3D depth | `effect("3D Point Control")("3D Point")[2]` | 12 | 240 | 240 | pass |
| Point3D depth | `effect("3D Point Control")("3D Point")[2]` | 24 | 480 | 480 | pass |
| Point3D depth | `effect("3D Point Control")("3D Point")[2]` | 48 | 960 | 960 | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")` | 0 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")` | 12 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")` | 24 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")` | 48 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D position | `effect("3D Point Control")("3D Point")` | 0 | (0, 0) | (0, 0) | pass |
| Point3D position | `effect("3D Point Control")("3D Point")` | 12 | (120, -60) | (120, -60) | pass |
| Point3D position | `effect("3D Point Control")("3D Point")` | 24 | (240, -120) | (240, -120) | pass |
| Point3D position | `effect("3D Point Control")("3D Point")` | 48 | (480, -240) | (480, -240) | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[3]` | 0 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[3]` | 12 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[3]` | 24 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("3D Point")[3]` | 48 | 0, EXPRESSION_TYPE | 0, EXPRESSION_TYPE | pass |
| Point3D rotation | `effect("3D Point Control")("Point")[0]` | 0 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("3D Point Control")("Point")[0]` | 12 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("3D Point Control")("Point")[0]` | 24 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("3D Point Control")("Point")[0]` | 48 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("Point Control")("Point")[0]` | 0 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("Point Control")("Point")[0]` | 12 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("Point Control")("Point")[0]` | 24 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |
| Point3D rotation | `effect("Point Control")("Point")[0]` | 48 | 0, EXPRESSION_REFERENCE_MISSING | 0, EXPRESSION_REFERENCE_MISSING | pass |

64 of 64 cells pass.
