# D-337 / B-216: Glow and Solid Composite on display values

From P-26's tutorial 2 (Advanced Electric), 2026-10-07.

## What was wrong

D-333 gave the app After Effects' own 32 bpc, where the blurs work on the colours as you see them (display values) rather than on light. But Glow and Solid Composite still worked on light. Tutorial 2 lays its glow layer on black with Solid Composite and glows it three times, and its ground reflections are blurred, laid on black and lifted with Exposure. Working on light, the black lifted the faint blue edge of the bolt into a bright cyan pool where the tutorial shows a dim gold one, and the bolt's glow was much fainter than the tutorial's.

## What changed

Only in a composition at **8 bpc** or **32 bpc (After Effects)**:

- **Solid Composite** lays the layer on its solid in display values, and the solid's colour is used as written.
- **Glow** lays its light on in display values, and its tint colour is used as written. Which parts glow is decided as before. In After Effects units its brightness is the one Creative COW measured in After Effects (intensity times 16 at threshold 0), which assumes this display-value working.

Float compositions are unchanged. Classic Glow units change only by working in display values.

**A specification change.** D-333's fixtures FX-AE32-001 and 002 (a blur laid on black, then Exposure) were recalculated for this rule by `tools/ae_32bpc_reference.py`. Their middle is now dimmer than Float's but still past white.

## Checks

`tests/b216_glow_display.rs` holds the app to `Fixtures/glow_display/expected_glow_display.json`, written by `tools/glow_display_reference.py` before the code. Results are in `verification/D-337_glow_display_table.md`: **24 of 24 pass**. B-214 (`verification/D-333_ae_32bpc_table.md`) checks the changed FX-AE32-001 and 002, and passes.

| Check | Result |
| --- | --- |
| FX-GLDISP-001: tutorial 2's kind of Glow at 32 bpc (After Effects) | matches |
| FX-GLDISP-002: the glow layer in small, black under it, then the Glow | matches |
| FX-GLDISP-003: a tint, used as written | matches |
| FX-GLDISP-004 and 005: classic units; Screen | match |
| FX-GLDISP-006: 8 bpc, held to 8 bits | matches |
| FX-GLDISP-007: Float, exactly as before | matches |
| FX-GLDISP-008: Solid Composite with a purple at half opacity | matches |
| Opened and saved, and drawn in tiles | the same |

## Pictures (`verification/D-337 pictures/`)

Tutorial 2's glow layer at a quarter size: a gold line (the bolt's colour, #ba9a3c) on black, glowed three times at intensity 0.1, threshold 0.

| Float | 32 bpc (After Effects), D-337 |
| --- | --- |
| ![Float](D-337%20pictures/1_float.png) | ![32 bpc](D-337%20pictures/2_ae_32bpc.png) |

**What to look for:** on the left the line has only a thin, faint halo. On the right a wide gold halo surrounds it, as in the tutorial. 30 pixels below the middle of the line, the halo reads 55, 46, 18 out of 255 on the right against 5, 4, 1 on the left, and it stays gold (red above green above blue).

## How to try it in the app

In Composition Settings set the depth to 32 bpc (After Effects). Put a thin bright line on black and add Glow in After Effects units with threshold 0, intensity 0.1. The halo should be wide and warm. Switch the depth to Float and it should become faint.
