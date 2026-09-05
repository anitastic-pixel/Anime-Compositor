# Anime shot workflows and acceptance scenarios

Version 0.2 | 2026-09-04 | Proposed baseline

## W-01: assemble and finish a cel shot

Inputs: one background image, three transparent PNG cel sequences, a written exposure reference and an output specification. Use original or licensed artwork. Proposed reference scene: 240 composition frames at 24 fps, 1080p, including drawings held for two and three frames.

The artist imports media, reviews sequence grouping and missing-frame warnings, creates a composition and assigns exposures. They stack layers, adjust anchors and transforms, apply a matte, add one blur and one color operation, inspect alpha and preview the work area. They save, close, reopen and export a PNG sequence.

Acceptance: cel identity at each frame matches the exposure reference; effect order survives reopening; the export contains exactly 240 correctly named files. Inspect designated edge and overlap frames against fixtures. Requirements R-01 through R-10; tests T-01 through T-09.

## W-02: replace revised drawings

Inputs: an existing project and a replacement sequence with revised artwork. Relink by explicit user choice; preserve layer identity, timing, masks and effects. Present changed dimensions, frame range or alpha interpretation before applying the change.

Acceptance: media replacement invalidates affected cache entries and preserves intentional holds. If frames are absent, report them. Never silently slide subsequent drawings to fill a gap. Undo restores the prior reference. Requirements R-01, R-02, R-07 and R-08.

## W-03: scanned traditional material

MVP entry point is prepared, aligned, transparent artwork. The compositor does not promise automatic scan registration or cleanup. Provide non-destructive interpretation, color adjustment and compositing once preparation is complete.

Later discovery should investigate paper removal, line-color separation and edge cleanup using representative scans. Record registration errors and line damage separately. These tools must not change exposure timing; artistic approval is required before proposing them as defaults.

---

## W-04: 2.5D parallax shot

Post-MVP inputs: separated background planes, a foreground cel and a camera move reference. Place flat layers at different depths, animate one perspective camera and preserve cel exposure timing independently of camera sampling.

Acceptance: the camera produces the specified parallax, saved transforms reproduce after reopening and occlusion follows documented rules. Limit the first implementation to an explicitly ordered set of non-intersecting flat planes; general intersecting transparency is outside this stage. R-12; T-11.

## W-05: recover and hand off

The artist saves a project, creates a recoverable autosave, closes the app unexpectedly in a controlled test and opens the recovery copy. They then package the project with media they have rights to redistribute and open it from another directory.

Acceptance: the last successful manual save remains valid; recovery states its timestamp; missing files can be relinked. MVP supports project plus a manually preserved relative media tree. Automated collection and a manifest are later R-14 work; do not treat that later packaging feature as an MVP dependency.

## Failure behavior

Missing frame: identify the path and composition usage; offer relink or an explicit missing-frame policy. Invalid image: reject the item with a readable reason. Low memory: reduce preview caching or fail the operation safely. Export failure: preserve the project and show the last completed frame; never mark the job successful.

Unknown effect: preserve serialized parameters, display an unsupported-effect notice and block a fidelity-sensitive export until the user explicitly chooses how to proceed. Changed media: ask whether to reload and invalidate relevant results.

## Discovery checklist

Observe Ani finishing one real shot and record where time is spent. Ask which tasks repeat across five shots, what metadata arrives with cels, which alpha/color failures occur and which handoff formats collaborators actually need. Compare the written exposure reference with exported frames.

No interviews have yet been conducted. These workflows are proposed scenarios informed by the user's stated goals, not a claim that all anime studios use the same process. Related documents: 03, 05, 07, 11 and 16.
