# D-323: Time Remapping

ADR-021 and D-308, accepted by the owner on 2026-10-04: a drawn or composition layer may key which of its source frames it shows, as After Effects' Time Remap does. Enable Time Remapping (Layer menu, Ctrl+Alt+T) writes two keys that change no frame; Freeze Frame on a composition layer is one hold key. Every expected number is `Fixtures/time_remap/expected_time_remap.json`, written by `tools/time_remap_reference.py` before this code existed and printed in document 25 as FX-TREMAP-001 to 055. Each frame is 8 by 1 pixels; the answer is the largest difference over all its samples, against the catalogue's pixel tolerance of 1e-6.

## FX-TREMAP-001 to 009 and 030: which source time each frame shows

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TREMAP-001 frame 0: Keys 0 at 0 and 11 at 11: t is n, as with no remap. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-001 frame 3: Keys 0 at 0 and 11 at 11: t is n, as with no remap. | t 3, f 3, w 0, u 3 | yes |
| FX-TREMAP-001 frame 11: Keys 0 at 0 and 11 at 11: t is n, as with no remap. | t 11, f 11, w 0, u 11 | yes |
| FX-TREMAP-002 frame 0: Keys 0 at 0 and 11 at 6: twelve frames of source in six, then held on 11. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-002 frame 3: Keys 0 at 0 and 11 at 6: twelve frames of source in six, then held on 11. | t 5.5, f 5, w 0.5, u 3 | yes |
| FX-TREMAP-002 frame 6: Keys 0 at 0 and 11 at 6: twelve frames of source in six, then held on 11. | t 11, f 11, w 0, u 6 | yes |
| FX-TREMAP-002 frame 9: Keys 0 at 0 and 11 at 6: twelve frames of source in six, then held on 11. | t 11, f 11, w 0, u 9 | yes |
| FX-TREMAP-003 frame 0: Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames of source a frame, then one. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-003 frame 2: Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames of source a frame, then one. | t 4, f 4, w 0, u 2 | yes |
| FX-TREMAP-003 frame 4: Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames of source a frame, then one. | t 8, f 8, w 0, u 4 | yes |
| FX-TREMAP-003 frame 8: Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames of source a frame, then one. | t 12, f 12, w 0, u 8 | yes |
| FX-TREMAP-003 frame 11: Tutorial 1's burst then real speed: 0 at 0, 8 at 4, 16 at 12; two frames of source a frame, then one. | t 15, f 15, w 0, u 11 | yes |
| FX-TREMAP-004 frame 0: One key, 5 at 3: every frame is source frame 5, a freeze. | t 5, f 5, w 0, u 0 | yes |
| FX-TREMAP-004 frame 3: One key, 5 at 3: every frame is source frame 5, a freeze. | t 5, f 5, w 0, u 3 | yes |
| FX-TREMAP-004 frame 9: One key, 5 at 3: every frame is source frame 5, a freeze. | t 5, f 5, w 0, u 9 | yes |
| FX-TREMAP-005 frame 0: 11 at 0 and 0 at 11: backwards. | t 11, f 11, w 0, u 0 | yes |
| FX-TREMAP-005 frame 5: 11 at 0 and 0 at 11: backwards. | t 6, f 6, w 0, u 5 | yes |
| FX-TREMAP-005 frame 11: 11 at 0 and 0 at 11: backwards. | t 0, f 0, w 0, u 11 | yes |
| FX-TREMAP-006 frame 0: A hold key, 2 at 0, then 9 at 6: 2 until frame 6, then 9. | t 2, f 2, w 0, u 0 | yes |
| FX-TREMAP-006 frame 5: A hold key, 2 at 0, then 9 at 6: 2 until frame 6, then 9. | t 2, f 2, w 0, u 5 | yes |
| FX-TREMAP-006 frame 6: A hold key, 2 at 0, then 9 at 6: 2 until frame 6, then 9. | t 9, f 9, w 0, u 6 | yes |
| FX-TREMAP-006 frame 8: A hold key, 2 at 0, then 9 at 6: 2 until frame 6, then 9. | t 9, f 9, w 0, u 8 | yes |
| FX-TREMAP-007 frame 0: Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so t is n until u passes 4. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-007 frame 1: Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so t is n until u passes 4. | t 1, f 1, w 0, u 0.5 | yes |
| FX-TREMAP-007 frame 2: Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so t is n until u passes 4. | t 2, f 2, w 0, u 1 | yes |
| FX-TREMAP-007 frame 8: Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so t is n until u passes 4. | t 8, f 8, w 0, u 4 | yes |
| FX-TREMAP-007 frame 10: Stretch 200, keys 0 at 0 and 8 at 4: the keys are read at u = n / 2, so t is n until u passes 4. | t 8, f 8, w 0, u 5 | yes |
| FX-TREMAP-008 frame 0: Easy ease from 0 at 0 to 8 at 8: slow, fast, slow. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-008 frame 2: Easy ease from 0 at 0 to 8 at 8: slow, fast, slow. | t 1.25, f 1, w 0.25, u 2 | yes |
| FX-TREMAP-008 frame 4: Easy ease from 0 at 0 to 8 at 8: slow, fast, slow. | t 4, f 4, w 0, u 4 | yes |
| FX-TREMAP-008 frame 6: Easy ease from 0 at 0 to 8 at 8: slow, fast, slow. | t 6.75, f 6, w 0.75, u 6 | yes |
| FX-TREMAP-008 frame 8: Easy ease from 0 at 0 to 8 at 8: slow, fast, slow. | t 8, f 8, w 0, u 8 | yes |
| FX-TREMAP-009 frame 3: In 3 and an offset of 5: the offset is not used, 0 at 3 and 6 at 9 give t = n - 3. | t 0, f 0, w 0, u 3 | yes |
| FX-TREMAP-009 frame 6: In 3 and an offset of 5: the offset is not used, 0 at 3 and 6 at 9 give t = n - 3. | t 3, f 3, w 0, u 6 | yes |
| FX-TREMAP-009 frame 9: In 3 and an offset of 5: the offset is not used, 0 at 3 and 6 at 9 give t = n - 3. | t 6, f 6, w 0, u 9 | yes |
| FX-TREMAP-030 frame 0: 0 at 0 and 49 at 49: at frame 1 the straight line lands at 0.9999999999999999 in 64-bit numbers, within 1e-9 of 1, so t is 1 and the drawing is frame 1's. | t 0, f 0, w 0, u 0 | yes |
| FX-TREMAP-030 frame 1: 0 at 0 and 49 at 49: at frame 1 the straight line lands at 0.9999999999999999 in 64-bit numbers, within 1e-9 of 1, so t is 1 and the drawing is frame 1's. | t 1, f 1, w 0, u 1 | yes |
| FX-TREMAP-030 frame 49: 0 at 0 and 49 at 49: at frame 1 the straight line lands at 0.9999999999999999 in 64-bit numbers, within 1e-9 of 1, so t is 1 and the drawing is frame 1's. | t 49, f 49, w 0, u 49 | yes |

## FX-TREMAP-010 to 019: the frames, and what each frame says

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TREMAP-010 frame 0: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010 frame 1: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010 frame 2: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010 frame 3: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010 frame 4: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010 frame 5: The drawings on twos with keys 0 at 0 and 5 at 5: red, red, blue, blue, green, green, as without a remap. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-010: opening it warns of nothing | [] | yes |
| FX-TREMAP-011 frame 0: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011 frame 1: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011 frame 2: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011 frame 3: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011 frame 4: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011 frame 5: One key, 2 at 0: frozen on source frame 2, blue, all six frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-011: opening it warns of nothing | [] | yes |
| FX-TREMAP-012 frame 0: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 1: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 2: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 3: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 4: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 5: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 6: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012 frame 7: A composition layer whose inner dot moves a pixel a frame, keys 0 at 0, 4 at 2 and 7 at 5: the dot moves two pixels a frame, then one, then stops at the inner composition's last frame. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-012: opening it warns of nothing | [] | yes |
| FX-TREMAP-013 frame 0: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 1: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 2: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 3: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 4: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 5: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 6: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013 frame 7: The same layer backwards, 7 at 0 and 0 at 7: the dot moves left. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-013: opening it warns of nothing | [] | yes |
| FX-TREMAP-014 frame 0: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 1: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 2: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 3: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 4: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 5: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 6: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 7: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 8: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014 frame 9: The drawings on twos slowed by keys 0 at 0 and 4 at 8, both frame blending switches on: frames 3 and 7 fall half way between two drawings and are half of each. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-014: opening it warns of nothing | [] | yes |
| FX-TREMAP-015 frame 0: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 1: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 2: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 3: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 4: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 5: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 6: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 7: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 8: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015 frame 9: The same with the layer's frame blending off: each source time rounds down, each drawing held four frames. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-015: opening it warns of nothing | [] | yes |
| FX-TREMAP-016 frame 0: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 1: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 10: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 11: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 2: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 3: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 4: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 5: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 6: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 7: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 8: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016 frame 9: Keys -2 at 0 and 10 at 12: before the inner composition starts and after it ends the layer is clear. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-016: opening it warns of nothing | [] | yes |
| FX-TREMAP-017 frame 0: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 1: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 2: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 3: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 4: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 5: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 6: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 7: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 8: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017 frame 9: Stretch 200 and keys 0 at 0 and 8 at 4: read at half the frame, the dot moves a pixel a frame as with no remap and no stretch. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-017: opening it warns of nothing | [] | yes |
| FX-TREMAP-018 frame 0: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 1: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 10: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 11: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 2: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 3: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 4: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 5: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 6: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 7: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 8: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018 frame 9: In 3, offset 5, keys 0 at 3 and 6 at 9: the offset is not used. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-018: opening it warns of nothing | [] | yes |
| FX-TREMAP-019 frame 0: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 1: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 2: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 3: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 4: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 5: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 6: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019 frame 7: Easy ease from 0 at 0 to 7 at 7: the dot starts slow and ends slow; each source time rounds down. | largest difference 0.0e0, says [] | yes |
| FX-TREMAP-019: opening it warns of nothing | [] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tremap_010.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_011.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_012.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_013.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_014.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_015.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_016.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_017.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_018.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| fx_tremap_019.json opened and saved holds what it held, its Time Remap and keys included | the same | yes |
| a file with no Time Remap, as every file before D-323, is saved as it was, with no time_remap | the same | yes |
| FX-TREMAP-050: Time Remap on a solid layer, which has no source time. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-TREMAP-051: Time Remap on a null layer. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-TREMAP-052: Time Remap on an adjustment layer. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-TREMAP-053: A Time Remap key that is a pair of numbers. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-TREMAP-054: A Time Remap with an expression, which is not part of this. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-TREMAP-055: A Time Remap written as a bare number rather than a property. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |

## FX-TREMAP-040 to 044: Enable Time Remapping changes no frame

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TREMAP-040: In 0, out 12: keys 0 at 0 and 11 at 11. Every frame shows the source time it showed. One undo takes it off. | taken, keys [(0, 0.0, "linear"), (11, 11.0, "linear")], frames kept true, undone true | yes |
| FX-TREMAP-041: In 5, out 20, offset 3: keys 3 at 5 and 17 at 19. Every frame shows the source time it showed. One undo takes it off. | taken, keys [(5, 3.0, "linear"), (19, 17.0, "linear")], frames kept true, undone true | yes |
| FX-TREMAP-042: Stretch 200, in 0, out 12: the last frame's key time is 5.5, so keys 0 at 0 and 6 at 6. Every frame shows the source time it showed. One undo takes it off. | taken, keys [(0, 0.0, "linear"), (6, 6.0, "linear")], frames kept true, undone true | yes |
| FX-TREMAP-043: Stretch 50, in 0, out 6: keys 0 at 0 and 10 at 10. Every frame shows the source time it showed. One undo takes it off. | taken, keys [(0, 0.0, "linear"), (10, 10.0, "linear")], frames kept true, undone true | yes |
| FX-TREMAP-044: One frame long, in 4, out 5, offset 2: keys 2 at 4 and 3 at 5. Every frame shows the source time it showed. One undo takes it off. | taken, keys [(4, 2.0, "linear"), (5, 3.0, "linear")], frames kept true, undone true | yes |
| FX-TREMAP-010's drawings, with Time Remapping turned on: all six frames byte-identical to before | byte-identical | yes |
| turned off again: the remap and its keys are gone, and a file that held one is written without time_remap | taken true, written false | yes |

## FX-TREMAP-045 to 047: Freeze Frame by Time Remap

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TREMAP-045: No remap, freeze at 5: one hold key, 5 at 5. Every frame then shows the playhead's source time. One undo puts it back. | taken, keys [(5, 5.0, "hold")], all frames at 5: true, undone true | yes |
| FX-TREMAP-046: Stretch 200, no remap, freeze at 5: the key time 2.5 rounds to 3, holding the source time 2.5. Every frame then shows the playhead's source time. One undo puts it back. | taken, keys [(3, 2.5, "hold")], all frames at 2.5: true, undone true | yes |
| FX-TREMAP-047: Keys 0 at 0 and 11 at 6, freeze at 3: the keys are replaced by one, 5.5 at 3. Every frame then shows the playhead's source time. One undo puts it back. | taken, keys [(3, 5.5, "hold")], all frames at 5.5: true, undone true | yes |

## FX-TREMAP-048: the keys move with the layer

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-TREMAP-048: Keys 0 at 0 and 11 at 6, the layer moved 2 frames later: the keys move to 2 and 8, the values stay. | taken, keys [(2, 0.0, "linear"), (8, 11.0, "linear")] | yes |
| the in point trimmed from 0 to 2: frame 4 still shows what it showed | taken, frame 4 byte-identical | yes |

## What is refused, with a sentence, changing nothing

| Check | The build's answer | Matches |
| --- | --- | --- |
| a Time Remap key while the remap is off is refused | Time Remapping is off on this layer: turn it on first. | yes |
| a Time Remap value while the remap is off is refused | Time Remapping is off on this layer: turn it on first. | yes |
| a freeze on frame 9, after the layer's out point is refused | "bar" is not on screen at frame 9. | yes |
| an expression on a Time Remap is refused (FX-TREMAP-054's rule, at the command) | Time Remap cannot carry an expression. | yes |
| Enable Time Remapping on a solid layer is refused | "card" has no frames of its own to time this way. | yes |
| with the remap on, a key of 1 at frame 3 is taken, and frame 3 shows source frame 1 | taken, t Some(1.0) | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_tremap_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tremap_014.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_tremap_017.json frame 5 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up ball in seventeen drawings, in `verification/D-323 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| as_drawn.png, no remap: drawing 0, 1, 2 ... 11, one a frame | [], drawings [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0] | yes |
| burst_then_real_speed.png, tutorial 1's keys 0 at 0, 8 at 4, 16 at 12: two drawings a frame for four frames, then one a frame | [], drawings [0.0, 2.0, 4.0, 6.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0] | yes |
| backwards.png, keys 11 at 0 and 0 at 11: the ball goes right to left | [], drawings [11.0, 10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0] | yes |
| frozen_at_5.png, Freeze Frame at 5: drawing 5 on every frame | [], drawings [5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0] | yes |

## Result

180 of 180 checks pass.
