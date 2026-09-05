# CONTEXT.md - Project vocabulary

The shared language of this project. Glossary only: no implementation detail, no decisions. Decisions live in document 14 and `docs/adr/`.

## Core entities

**Project** - The saved document. One JSON file plus external references to media. Owns compositions and assets. Never contains media bytes.

**Composition** - A timed canvas with fixed dimensions and frame rate, holding an ordered layer stack. The reference composition is 1920x1080 at 24 fps.

**Layer** - One entry in a composition's stack, referencing an asset and carrying its own transform, exposure mapping and properties. Has a stable ID that survives rename and reorder.

**Asset** - Imported source material registered in the project. For G1-core this is a PNG sequence. An asset is referenced, never copied into the project.

**Sequence** - A group of numbered PNG files treated as one asset via an explicit numeric pattern. Gaps in numbering are a diagnosable condition, not a silent skip.

**Drawing** - One image within a sequence, identified by its drawing ID rather than by the composition frame it appears on.

## Timing

**Frame** - A composition time position. Frame stepping always moves exactly one composition frame.

**Exposure** - The mapping from composition frames to drawing IDs. This is the project's central concept and its reason to exist. Editing exposure changes which drawing appears when; it never resamples or alters artwork.

**Hold** - A drawing exposed across more than one consecutive frame. Holds are intentional artistic timing and must survive save, preview and export unchanged.

**On 1s / 2s / 3s** - Shorthand for exposure cadence: a new drawing every one, two or three composition frames.

**Keyframe** - A value set on an animatable property at a specific time. Distinct from exposure: moving a keyframe does not change cel timing.

## Rendering

**Tile** - The unit of render work. Frames are divided into tiles composited independently across threads. The tile is also the intended unit of a future GPU dispatch.

**Render plan** - The acyclic evaluation order derived from a composition, a time and a quality request. Preview and export use the same plan.

**Tagged buffer** - An image buffer that always carries its alpha mode and color space. An untagged buffer never crosses a module boundary.

**Working space** - Linear-light premultiplied float32, the internal representation all compositing math occurs in.

**Preview** - An interactive, cancellable, possibly reduced-quality render for on-screen viewing. May be stale; never authoritative.

**Export** - A render of an immutable job snapshot at final quality. The authoritative output. Export never passes through the preview display path.

**Render trace** - A diagnostic mode dumping each intermediate layer buffer as a PNG, so a wrong result can be inspected visually rather than debugged in code.

## Editing

**Command** - The only sanctioned way to change the project. Validated, reversible, and the unit of undo. UI never mutates the model directly.

**Revision** - A monotonic document version. Workers operate on immutable snapshots identified by revision; results for stale revisions are discarded.

**Relink** - Repointing an asset at a moved or renamed file. Missing media preserves the layer and offers relink; it never substitutes a placeholder silently.

## Process

**Fixture** - A test case with expected values derived independently of the implementation. Fixtures are specification, not output.

**Verification artifact** - The evidence attached to completed work that the owner personally judges: fixture pass/fail with expected-versus-actual numbers, exported PNGs, screenshots.

**Reference shot** - The canonical 10-second test shot the owner draws. Deliberately crude artwork with deliberate defects; its value is timing structure and edge types, not quality.

**G1-core** - The narrowed first milestone: import, exposure, layers, transforms, undo, save and recovery, export, color and alpha, offline operation.

**Parked** - Specified but deliberately not being built, with a written revisit trigger. Distinct from rejected.
