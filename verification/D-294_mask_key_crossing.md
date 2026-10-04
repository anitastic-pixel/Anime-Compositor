# D-294 / B-179: a mask key that crosses itself is refused

Found by P-26, tutorial 4 (Chris Connor lightsaber): Draw refused a wedge mask whose outline crossed itself. The same outline set as a mask *key* was accepted, and on those frames the layer drew with no mask at all.

## What changed

- `src/command.rs`: when masks are set, each key's outline is held to the same rule as the path. A crossing key is refused, the sentence names its frame, and nothing changes.
- When a mask crosses only part-way between two keys, as its points move, the layer is still drawn without that mask for those frames, and a warning is logged on each one (as before, document 28). Filling crossing masks the way After Effects does is a separate gap and was not built.

## Checks (cargo test)

`tests/b179_mask_key_crossing.rs`, 1 of 1 pass:
- A square path with a bowtie key at frame 4 (its edges cross at (2,2)) is refused with "The mask crosses itself at its key on frame 4, which this build does not draw." The layer is left with no mask.
- The same square with a plain square key at frame 4 is taken.

`tests/b06_mask.rs` passes unchanged, including the existing sentence for a crossing path.

## For the owner to try

1. Draw a four-point mask on a layer and key its path at frame 0.
2. Go to frame 10 and drag one corner across the opposite edge so the outline makes a bowtie.
3. The move is refused with a sentence naming frame 10, and the mask keeps its last good shape.

Pixels, exports, saved files and fixtures are unchanged.
