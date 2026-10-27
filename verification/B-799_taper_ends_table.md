# B-799: Directional Blur's Taper Ends (D-919)

Owner, 2026-10-27, audit row 11: "have the end taper as a toggle option". Directional Blur has a
new setting, Taper Ends, On (the default, D-753's fading streak, exactly as before) or Off (D-98's
flat, hard-ended streak, exactly as before B-632, commit 438fe5d0).

Every expected pixel is `Fixtures/directional_blur/expected_directional_blur.json`, written by
`tools/directional_blur_reference.py` and committed before this code (commit 4449a294).

| Check | Where it is checked | Result |
|---|---|---|
| FX-DIRBLUR-001 to 015 (Taper Ends left out, so on) draw exactly as before: the expected file's 15 old cases are unchanged | `git diff` of the expected file; B-36 table | PASS |
| FX-DIRBLUR-016 to 026, 001 to 011 with Taper Ends off, are D-98's flat average: against commit 438fe5d0's 001 to 011, 2560 pixels, worst difference 0 | `tools/directional_blur_reference.py` check | PASS |
| The build draws 016 to 026 on the processor within 2e-5 (worst 1.9e-7) | `verification/B-36_directional_blur_table.md`, 112 of 112 | PASS |
| FX-DIRBLUR-027, Taper Ends "on" written, is 001 exactly, and is saved with the word kept | B-36 table | PASS |
| FX-DIRBLUR-028 ("Off") and 029 ("yes") open, keep the word, leave the blur out with `EFFECT_PARAMETER_INVALID` | B-36 and B-49 tables | PASS |
| A project that never turned it off saves byte for byte as before (no `taper` written); 001, 007, 010 to 016, 025, 027 to 029 open and save unchanged | B-36 table, "The file" | PASS |
| `taper` that is not a word refuses the file as a fault in its shape; "yes" and "Off" set from a command are refused with a sentence and nothing changes; "off" is taken and undone | B-36 table | PASS |
| On the card against the processor, 016 to 026 at every frame, Full and Draft: 110 comparisons, the blur on the card, worst 1 level of 255 (15 pixels in all differ by 1) | `verification/B-49_gpu_directional_table.md`, 298 of 298 | PASS |
| The same through D-617's fixture check, 001 to 029 | `verification/B-632_directional_taper_table.md`, 190 of 190 | PASS |
| Taper-off frames are the same whole and in tiles of 1 (019 frame 0, 024 frame 3) | B-36 table | PASS |
| The panel: a new Directional Blur starts On; setting Off reaches the blur; "yes" is refused; the save keeps Off; an older page's send without it is On | app test `directional_taper_ends_typed_in_the_panel_reaches_the_blur` | PASS |

Test runs (release, worktree `worktree-agent-af2d683ea293aae7f`):

- `b36_directional_blur`: test result: ok. 1 passed; 0 failed; 0 ignored
- `b49_gpu_directional`: test result: ok. 1 passed; 0 failed; 1 ignored
- `b632_directional_taper`: test result: ok. 1 passed; 0 failed; 0 ignored
- `b52_edges`: test result: ok. 1 passed; 0 failed; 0 ignored
- `anime_compositor_app`: test result: ok. 141 passed; 0 failed; 13 ignored
- library: test result: ok. 29 passed; 0 failed; 1 ignored
- `d99_draft_effects` and `p16_effect_cost` (measurements, ignored by default): build
