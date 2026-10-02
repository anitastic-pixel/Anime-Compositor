# D-259: Half, a middle picture quality

The viewer's quality menu (the chip that says "Draft, not final" or "Full") now has **Half resolution** under the Full/Draft button. It is also in View › Half quality and in the command finder.

- **Half draws the picture at half its size each way**: 960 × 540 for the reference shot. Draft is a quarter (480 × 270) and Full is the whole (1920 × 1080).
- **The chip says "Half, not final"** in orange, as Draft does. Exports are never drawn at Half.
- **D still switches between Draft and Full.** From Half, D goes to Draft.
- **Refine when idle only works at Full**, as before. At Half the picture stays at Half while you drag.

## Pictures

- `D-259 pictures/half_menu.png`: the menu open at Half, with Half resolution lit in orange and the chip reading "Half, not final". The picture behind it is drawn on the CPU, because a screenshot can't capture what the card paints.
- `D-259_half_frame_10.png`: frame 10 at Half, 960 × 540. This is the new expected picture.

## The check, written first (`tests/d259_half.rs`, committed in f9cbfab)

**On the build before the change**, it did not build: "no variant `Half`".

**On the new build** it writes `D-259_half_table.md`:

| Check | Result |
|---|---|
| Full frames 10 and 100 are byte for byte as before Half | pass (same SHA-256 as 258a0de) |
| Draft frames 10 and 100 are byte for byte as before Half | pass |
| Half is 960 by 540 | pass |
| Half says it differs from an export | pass |
| Half frame 10 is the recorded picture | pass. The hash was taken from the first build with Half, after I looked at the picture. |
| Half at least as quick as Full | pass. Median of 7 renders of frame 10 on the CPU, nothing remembered between them: **Full 52.6 ms, Half 42.8 ms, Draft 39.5 ms** |

**Machine:** this desktop (NVIDIA GeForce RTX 4070 Ti SUPER; the times above are on the CPU), release build, `cargo test --release`. Times vary run to run. The first run gave Full 54.8, Half 44.4 and Draft 40.9 ms.

Half is only a little slower than Draft because most of a frame's time goes on reading the drawings, at their own size, before anything is shrunk (D-37).

## In the running app (`d259_check.js`)

| Step | What the page said |
|---|---|
| On the card, Half pressed | chip "Half, not final"; picture 960 × 540; Half lit |
| The frame's answer | Half, 960 × 540, differs from export, drawn on the GPU |
| A layer selected | its outline drawn over a 960 × 540 picture |
| D | Draft, 480 × 270; Half no longer lit |
| D again | Full, 1920 × 1080 |
| The same on the CPU | the same, drawn on the CPU |
| Play at Half | plays at Half, 960 × 540 |
| Errors on the page | none |

The window's own settings were put back afterwards.

## Checks

- All 81 of the app's tests pass. The list of wired controls now includes `halfq`.
- The preview, cache, draft-effect and export suites pass. None of their written tables changed.
- No exported picture can change: an export never asks for a preview quality.

## Limits, stated

- **The window opens at Draft**, as before (D-33). Half is chosen each time, like Full.
- **Half has no stand-in while dragging.** Refine when idle swaps in Draft only when Full is chosen.

## Playtest

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Click the quality chip above the picture, then Half resolution | "Half, not final"; sharper than Draft | |
| 2 | Press D | Draft, and Half no longer lit in the menu | |
| 3 | Choose Half, then play | It plays at Half | |

Anything marked ✗, tell me the row number.
