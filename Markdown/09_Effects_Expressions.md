# Effects and expressions specification

> **PARKED in version 0.3 under D-12.** The G1 effect stack (R-05) is not part of G1-core, and expressions (R-13) remain a G2 concern with an undecided runtime per D-10. This specification is retained unchanged below.
>
> Revisit trigger for effects: repeated manual effort in finishing real shots that an effect would remove.
>
> Note for implementation when promoted: Gaussian blur has unbounded spatial support and therefore requires explicit tile-margin handling under the tile contract in ADR-011 and document 21. That interaction is part of why effects are not in the first milestone.


Version 0.2 | 2026-09-04 | Proposed baseline

## Effect contract

Every effect has an original native identifier, contract version, input/output alpha and color tags, typed parameters with units/defaults/ranges, spatial bounds, temporal dependencies and deterministic behavior. The registry must reject unsupported versions explicitly.

G1 effect set: exposure multiplies unpremultiplied linear RGB by 2 to the power of the exposure in stops and preserves alpha; Gaussian blur filters premultiplied RGB and alpha together with a specified kernel/truncation; tint replaces unpremultiplied RGB with the chosen color while preserving alpha. Avoid ambiguities around fully transparent pixels.

Suggested tests: zero exposure is identity; zero-radius blur is identity; blur of an impulse is symmetric and bounded by the declared support; tint does not change coverage. Specify radius units in source pixels and distinguish preview scaling from final rendering.

## Expansion priorities

After G1, investigate levels/curves, hue/saturation, thresholded glow, directional blur, color selection/keying, line recolor, morphology, distance-based gradients and edge smoothing. Promote only those that solve observed W-01/W-03 tasks.

OLM's published catalog includes smoothing, cel-oriented blur, color keying, directional blur and highlight effects [S-01]. This supports researching those categories; it does not establish a required clone list or universal studio preference.

## Reuse boundaries

Classify each candidate as independent implementation, permitted source reuse, licensed integration or deferred. Retain upstream licenses and notices for permitted reuse and review the exact revision. A downloadable plugin binary is not automatically usable in a different host.

Keep a candidate record with visual goal, reference source, algorithm provenance, license status, quality fixture, temporal artifacts and maintenance owner. Legal review procedure: document 10.

---

## Native expressions: G2 proposal

Begin with scalar/vector arithmetic, time, value, explicit property references, linear interpolation and deterministic seeded noise. Use a small specified language or a sandboxed runtime chosen after evaluation; language implementation is OPEN.

Evaluate against an immutable project snapshot. No filesystem, network, process spawning or native calls. Apply instruction and memory budgets, reject cycles and show property-local errors. Evaluation limits must actually terminate runaway work; a UI timeout alone is insufficient.

Property references use stable IDs. Renaming a layer must not break a reference. Random behavior derives from a saved seed and defined time inputs. Specify units, vector dimensions, coercion and boundary behavior; avoid undocumented conversions.

## Compatibility ladder

Level A: similar creative capability implemented natively. Level B: documented equivalents for a small list of expressions or effects. Level C: an explicitly tested translator with unsupported-feature diagnostics. Level D: broad project/plugin compatibility, deferred as a separate product effort.

Adobe documents an expression language based on JavaScript with additional built-in objects [S-02]. Supporting JavaScript alone therefore does not establish AE compatibility. Expressions, application automation scripts, project import and compiled plugins are different interfaces and require separate decisions.

For any translator, record the tested input, expected native result, tolerance and known differences. Never silently approximate an unknown property or effect. A compatibility report must identify every translated, unsupported and manually resolved element.

## G2 acceptance

Test arithmetic, keyframe/time access, invalid dimensions, renamed references, cyclic references, runaway evaluation and seeded output reproducibility. A failed expression must be visible in preview and block final export until corrected or explicitly disabled. No expression engine is required for G1.

Related documents: 03, 04, 08, 10, 11 and 16.
