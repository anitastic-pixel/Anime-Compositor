# D-293 / B-178: Alpha inverted, Luma and Luma inverted mattes

Found by P-26: the Advanced Electric and Colorful Glitch tutorials use a Luma matte, and a matte here could only be Alpha. The window's `mode` was not read.

## What changed

- `src/model.rs`: a matte carries a mode: alpha (default), alpha_inverted, luma or luma_inverted. `src/persist.rs` saves and reads it; an unknown word is refused on load.
- `src/compose.rs`: for luma, the matte's cover is its picture's brightness over black in display values (0.2126 R + 0.7152 G + 0.0722 B, sRGB-encoded). Inverted means one minus the cover.
- `src/command.rs`: a new Set matte mode command, with undo. Changing the matte layer keeps the mode.
- `app/src/main.rs` and `app/ui/index.html`: next to a layer's matte, a list chooses Alpha / Alpha inverted / Luma / Luma inverted. Paste and Pre-compose keep the mode.
- Documents 19 and 21 describe the four modes.

## Checks (cargo test)

`tests/b178_matte_modes.rs`, 2 of 2 pass. A white frame is matted by a grey square over its left half. The grey's display value is exactly 0.5, so every expected number below is worked out by hand.

| Mode | Left half cover | Right half cover | Full | Draft |
|---|---|---|---|---|
| alpha | 1 | 0 | pass | pass |
| alpha_inverted | 0 | 1 | pass | pass |
| luma | 0.5 | 0 | pass | pass |
| luma_inverted | 0.5 | 1 | pass | pass |

The file keeps `luma_inverted` through save and load, and `lumen` is refused, naming the four allowed words.

App checks, in `the_inspector_chooses_a_matte_and_refuses_the_ones_that_would_not_work`: the window answers "Set matte mode to luma inverted". Changing the matte layer keeps `luma_inverted`, and "lumen" is refused in a sentence. Result: pass (app suite run on 2026-10-04: 89 passed, 5 ignored as always, none failed).

## For the owner to try

1. Put a black-to-white gradient layer above a picture layer. On the picture layer, set the gradient as its matte.
2. Next to the matte, pick Luma. The picture shows through where the gradient is bright and fades where it is dark.
3. Pick Luma inverted: the opposite. Pick Alpha inverted: the picture shows only outside the gradient layer's rectangle.
4. Undo steps back through each change.

Files with an alpha matte are unchanged and draw exactly as before. No fixture changed.
