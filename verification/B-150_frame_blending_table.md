# B-150: Time Stretch, Frame Mix and Drawing Dissolve

D-216 and ADR-020, accepted on 2026-09-29 with the owner's answers: a stretched layer's keys stretch with it, as in After Effects; the Drawing Dissolve works whatever the composition's switch says; speeding up mixes only the two nearest frames; and the composition's switch is off until it is turned on. Every expected number is `Fixtures/frame_blending/expected_frame_blending.json`, written by `tools/frame_blending_reference.py` before this code existed and printed in document 25 as FX-FBLEND-001 to 067. Each frame is 8 by 1 pixels; the answer is the largest difference over all its samples, against the catalogue's pixel tolerance of 1e-6.

This table is the core's half. The window's, B-150c's key diamonds shown and set where a stretched layer's keys play, is `verification/B-150c_key_diamonds_table.md`.

## FX-FBLEND-001 to 008: where in its source a layer is, and where its keys are read

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FBLEND-001 frame 2: Stretch 100, in 2, offset 1: t is n - 2 + 1, whole. | t 1, f 1, w 0, u 2 | yes |
| FX-FBLEND-001 frame 3: Stretch 100, in 2, offset 1: t is n - 2 + 1, whole. | t 2, f 2, w 0, u 3 | yes |
| FX-FBLEND-001 frame 4: Stretch 100, in 2, offset 1: t is n - 2 + 1, whole. | t 3, f 3, w 0, u 4 | yes |
| FX-FBLEND-001 frame 7: Stretch 100, in 2, offset 1: t is n - 2 + 1, whole. | t 6, f 6, w 0, u 7 | yes |
| FX-FBLEND-002 frame 0: Stretch 200: half a frame of source a frame. | t 0, f 0, w 0, u 0 | yes |
| FX-FBLEND-002 frame 1: Stretch 200: half a frame of source a frame. | t 0.5, f 0, w 0.5, u 0.5 | yes |
| FX-FBLEND-002 frame 2: Stretch 200: half a frame of source a frame. | t 1, f 1, w 0, u 1 | yes |
| FX-FBLEND-002 frame 3: Stretch 200: half a frame of source a frame. | t 1.5, f 1, w 0.5, u 1.5 | yes |
| FX-FBLEND-002 frame 4: Stretch 200: half a frame of source a frame. | t 2, f 2, w 0, u 2 | yes |
| FX-FBLEND-002 frame 5: Stretch 200: half a frame of source a frame. | t 2.5, f 2, w 0.5, u 2.5 | yes |
| FX-FBLEND-003 frame 0: Stretch 50: two frames of source a frame. | t 0, f 0, w 0, u 0 | yes |
| FX-FBLEND-003 frame 1: Stretch 50: two frames of source a frame. | t 2, f 2, w 0, u 2 | yes |
| FX-FBLEND-003 frame 2: Stretch 50: two frames of source a frame. | t 4, f 4, w 0, u 4 | yes |
| FX-FBLEND-003 frame 3: Stretch 50: two frames of source a frame. | t 6, f 6, w 0, u 6 | yes |
| FX-FBLEND-004 frame 0: Stretch 150: two thirds of a frame a frame. | t 0, f 0, w 0, u 0 | yes |
| FX-FBLEND-004 frame 1: Stretch 150: two thirds of a frame a frame. | t 0.6666666666666666, f 0, w 0.6666666666666666, u 0.6666666666666666 | yes |
| FX-FBLEND-004 frame 2: Stretch 150: two thirds of a frame a frame. | t 1.3333333333333333, f 1, w 0.33333333333333326, u 1.3333333333333333 | yes |
| FX-FBLEND-004 frame 3: Stretch 150: two thirds of a frame a frame. | t 2, f 2, w 0, u 2 | yes |
| FX-FBLEND-004 frame 4: Stretch 150: two thirds of a frame a frame. | t 2.6666666666666665, f 2, w 0.6666666666666665, u 2.6666666666666665 | yes |
| FX-FBLEND-005 frame 10: Stretch 300, in 10, offset 2. | t 2, f 2, w 0, u 10 | yes |
| FX-FBLEND-005 frame 11: Stretch 300, in 10, offset 2. | t 2.3333333333333335, f 2, w 0.3333333333333335, u 10.333333333333334 | yes |
| FX-FBLEND-005 frame 12: Stretch 300, in 10, offset 2. | t 2.6666666666666665, f 2, w 0.6666666666666665, u 10.666666666666666 | yes |
| FX-FBLEND-005 frame 13: Stretch 300, in 10, offset 2. | t 3, f 3, w 0, u 11 | yes |
| FX-FBLEND-006 frame 0: Stretch 33.3, not a round share: f is 3, 6 and 9. | t 0, f 0, w 0, u 0 | yes |
| FX-FBLEND-006 frame 1: Stretch 33.3, not a round share: f is 3, 6 and 9. | t 3.0030030030030033, f 3, w 0.0030030030030032684, u 3.0030030030030033 | yes |
| FX-FBLEND-006 frame 2: Stretch 33.3, not a round share: f is 3, 6 and 9. | t 6.0060060060060065, f 6, w 0.006006006006006537, u 6.0060060060060065 | yes |
| FX-FBLEND-006 frame 3: Stretch 33.3, not a round share: f is 3, 6 and 9. | t 9.00900900900901, f 9, w 0.009009009009009361, u 9.00900900900901 | yes |
| FX-FBLEND-007 frame 0: Stretch 10000, the most: a hundredth of a frame a frame. | t 0, f 0, w 0, u 0 | yes |
| FX-FBLEND-007 frame 1: Stretch 10000, the most: a hundredth of a frame a frame. | t 0.01, f 0, w 0.01, u 0.01 | yes |
| FX-FBLEND-007 frame 2: Stretch 10000, the most: a hundredth of a frame a frame. | t 0.02, f 0, w 0.02, u 0.02 | yes |
| FX-FBLEND-007 frame 99: Stretch 10000, the most: a hundredth of a frame a frame. | t 0.99, f 0, w 0.99, u 0.99 | yes |
| FX-FBLEND-007 frame 100: Stretch 10000, the most: a hundredth of a frame a frame. | t 1, f 1, w 0, u 1 | yes |
| FX-FBLEND-008 frame 5: Stretch 1, the least: a hundred frames a frame. | t 0, f 0, w 0, u 5 | yes |
| FX-FBLEND-008 frame 6: Stretch 1, the least: a hundred frames a frame. | t 100, f 100, w 0, u 105 | yes |
| FX-FBLEND-008 frame 7: Stretch 1, the least: a hundred frames a frame. | t 200, f 200, w 0, u 205 | yes |

## FX-FBLEND-010 to 039: the frames, and what each frame says

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FBLEND-010 frame 0: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010 frame 1: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010 frame 2: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010 frame 3: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010 frame 4: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010 frame 5: Stretch 100 with both switches on: every time is a whole frame, so the drawings on twos, red, blue, green, exactly as with both switches off. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-010: opening it warns of nothing | [] | yes |
| FX-FBLEND-011 frame 0: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 1: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 10: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 11: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 2: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 3: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 4: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 5: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 6: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 7: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 8: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011 frame 9: Stretch 200, the layer's switch off: each drawing is held four frames, red, blue, green, green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-011: opening it warns of nothing | [] | yes |
| FX-FBLEND-012 frame 0: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 1: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 10: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 11: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 2: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 3: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 4: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 5: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 6: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 7: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 8: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012 frame 9: Stretch 200, both switches on: frames 3 and 7 fall half way between two drawings and are half of each; frames 1 and 5 fall between two frames of the same drawing and are that drawing, bit for bit; frames 9 and 11 would mix with frame 6, past the last drawing, so they are green alone. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-012: opening it warns of nothing | [] | yes |
| FX-FBLEND-013 frame 0: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 1: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 10: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 11: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 2: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 3: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 4: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 5: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 6: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 7: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 8: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013 frame 9: Stretch 200, the layer's switch on and the composition's off: as FX-FBLEND-011. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-013: opening it warns of nothing | [] | yes |
| FX-FBLEND-014 frame 0: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014 frame 1: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014 frame 2: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014 frame 3: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014 frame 4: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014 frame 5: A file written before D-216, with none of the four fields: the drawings on twos, and saved again it still has none. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-014: opening it warns of nothing | [] | yes |
| FX-FBLEND-015 frame 0: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 1: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 2: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-015 frame 3: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 4: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 5: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-015 frame 6: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 7: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015 frame 8: Stretch 150, both switches on: the times go in thirds, so frames 1, 2, 4, 5, 7 and 8 are two thirds of one and a third of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-015: opening it warns of nothing | [] | yes |
| FX-FBLEND-016 frame 0: Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and two thirds of the way on; frame 3 is local frame 4 exactly, and local frame 3, stepped over, is never mixed in. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-016 frame 1: Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and two thirds of the way on; frame 3 is local frame 4 exactly, and local frame 3, stepped over, is never mixed in. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-016 frame 2: Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and two thirds of the way on; frame 3 is local frame 4 exactly, and local frame 3, stepped over, is never mixed in. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-016 frame 3: Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and two thirds of the way on; frame 3 is local frame 4 exactly, and local frame 3, stepped over, is never mixed in. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-016 frame 4: Stretch 75, both switches on: sped up, frames 1 and 2 fall a third and two thirds of the way on; frame 3 is local frame 4 exactly, and local frame 3, stepped over, is never mixed in. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-016: opening it warns of nothing | [] | yes |
| FX-FBLEND-017 frame 0: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 1: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 10: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 11: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 2: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 3: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 4: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 5: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 6: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 7: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 8: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017 frame 9: A gap: nothing exposed on local frames 2 and 3. Stretch 200, both on: frame 3 is red half mixed with nothing, so red at half covering; frames 4 to 7 are empty; frame 7 mixes nothing with green. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-017: opening it warns of nothing | [] | yes |
| FX-FBLEND-018 frame 0: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 1: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 10: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 11: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 2: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 3: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-018 frame 4: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-018 frame 5: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-018 frame 6: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-018 frame 7: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-018 frame 8: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018 frame 9: Drawing 2 has no file. Stretch 200, both on: frames 3 and 7 mix with an empty picture and say MEDIA_SEQUENCE_GAP, as frames 4 to 6 do; frames that do not read drawing 2 say nothing. No other drawing stands in for it. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-018: opening it warns of nothing | [] | yes |
| FX-FBLEND-019 frame 0: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 1: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 10: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 11: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 2: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 3: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 4: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 5: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 6: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 7: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 8: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019 frame 9: Source offset 1, stretch 200, both on: every time is half a frame on, so frames 0 to 3 are red, half, blue, blue... one frame earlier than 012. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-019: opening it warns of nothing | [] | yes |
| FX-FBLEND-020 frame 0: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 1: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 10: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 11: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 2: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 3: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 4: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 5: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 6: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 7: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 8: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020 frame 9: In point 3, stretch 200, both on: nothing before frame 3, and from there FX-FBLEND-012's frames, three frames later. The stretch runs from the in point. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-020: opening it warns of nothing | [] | yes |
| FX-FBLEND-021 frame 0: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 1: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 10: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 11: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 2: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 3: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 4: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 5: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 6: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 7: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 8: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021 frame 9: A composition layer showing a composition, 6 frames long, of the three drawings on twos, stretched 200 with both switches on: FX-FBLEND-012's frames. The composition's end is its source's end. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-021: opening it warns of nothing | [] | yes |
| FX-FBLEND-022 frame 0: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 1: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 2: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 3: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 4: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 5: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 6: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022 frame 7: A composition layer showing a white dot 2 pixels wide moving right a pixel a frame, stretched 200 with both on: on odd frames the inner composition is drawn twice, a frame apart, and the two are half and half, so the dot is 3 pixels with half-covered ends. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-022: opening it warns of nothing | [] | yes |
| FX-FBLEND-023 frame 0: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 1: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 2: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 3: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 4: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 5: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 6: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023 frame 7: Keys stretch with the layer: its position is keyed from x 0 at frame 0 to x 8 at frame 4. Stretched 200 with Frame Mix, that key plays at frame 8, so the layer is at x = n on frame n, half the speed it had, in step with its drawings. The stored keys stay at 0 and 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-023: opening it warns of nothing | [] | yes |
| FX-FBLEND-024 frame 0: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024 frame 1: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024 frame 2: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024 frame 3: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024 frame 4: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024 frame 5: A file that writes the defaults: stretch 100 and the composition's switch false. They read as absent, the drawings on twos, and saved again neither field is written. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-024: opening it warns of nothing | [] | yes |
| FX-FBLEND-025 frame 0: Sped up: stretch 50, no mixing, position keyed from x 0 at frame 0 to x 2 at frame 4. The key at 4 plays at frame 2: x is 0, 1, 2, then held at 2, while the drawings go red, blue, green, then nothing past the last. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-025 frame 1: Sped up: stretch 50, no mixing, position keyed from x 0 at frame 0 to x 2 at frame 4. The key at 4 plays at frame 2: x is 0, 1, 2, then held at 2, while the drawings go red, blue, green, then nothing past the last. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-025 frame 2: Sped up: stretch 50, no mixing, position keyed from x 0 at frame 0 to x 2 at frame 4. The key at 4 plays at frame 2: x is 0, 1, 2, then held at 2, while the drawings go red, blue, green, then nothing past the last. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-025 frame 3: Sped up: stretch 50, no mixing, position keyed from x 0 at frame 0 to x 2 at frame 4. The key at 4 plays at frame 2: x is 0, 1, 2, then held at 2, while the drawings go red, blue, green, then nothing past the last. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-025: opening it warns of nothing | [] | yes |
| FX-FBLEND-026 frame 0: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 1: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 2: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 3: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 4: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 5: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 6: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 7: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 8: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026 frame 9: In point 2, stretch 200 with Frame Mix, position keyed from x 0 at frame 2 to x 4 at frame 4: the keys stretch from the in point, so the key at 4 plays at frame 6: the layer is at x = n - 2 up to there, then stays at 4. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-026: opening it warns of nothing | [] | yes |
| FX-FBLEND-027 frame 0: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 1: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 2: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 3: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 4: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 5: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 6: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027 frame 7: A held key: opacity 1, held, at frame 0 and 0 at frame 3. Stretched 200 without mixing, the layer vanishes at frame 6, not 3. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-027: opening it warns of nothing | [] | yes |
| FX-FBLEND-028 frame 0: A composition layer stretched 200, no mixing, showing the moving dot of FX-FBLEND-022, its own position keyed from x 0 at frame 0 to x 4 at frame 2: the outer key plays at frame 4, so the layer is at x = n, and the dot inside is where its own keys put it at the inner frame floor(n / 2). | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-028 frame 1: A composition layer stretched 200, no mixing, showing the moving dot of FX-FBLEND-022, its own position keyed from x 0 at frame 0 to x 4 at frame 2: the outer key plays at frame 4, so the layer is at x = n, and the dot inside is where its own keys put it at the inner frame floor(n / 2). | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-028 frame 2: A composition layer stretched 200, no mixing, showing the moving dot of FX-FBLEND-022, its own position keyed from x 0 at frame 0 to x 4 at frame 2: the outer key plays at frame 4, so the layer is at x = n, and the dot inside is where its own keys put it at the inner frame floor(n / 2). | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-028 frame 3: A composition layer stretched 200, no mixing, showing the moving dot of FX-FBLEND-022, its own position keyed from x 0 at frame 0 to x 4 at frame 2: the outer key plays at frame 4, so the layer is at x = n, and the dot inside is where its own keys put it at the inner frame floor(n / 2). | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-028 frame 4: A composition layer stretched 200, no mixing, showing the moving dot of FX-FBLEND-022, its own position keyed from x 0 at frame 0 to x 4 at frame 2: the outer key plays at frame 4, so the layer is at x = n, and the dot inside is where its own keys put it at the inner frame floor(n / 2). | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-028: opening it warns of nothing | [] | yes |
| FX-FBLEND-030 frame 0: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030 frame 1: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030 frame 2: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030 frame 3: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030 frame 4: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030 frame 5: Drawing Dissolve 1 on twos, stretch 100: the second frame of each hold is half the drawing and half the next; the last drawing has no next and holds. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-030: opening it warns of nothing | [] | yes |
| FX-FBLEND-031 frame 0: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-031 frame 1: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-031 frame 2: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-031 frame 3: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-031 frame 4: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-031 frame 5: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 2.0e-8, says [] | yes |
| FX-FBLEND-031 frame 6: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-031 frame 7: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-031 frame 8: Drawing Dissolve 2 on threes: a third, then two thirds, of the next. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-031: opening it warns of nothing | [] | yes |
| FX-FBLEND-032 frame 0: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032 frame 1: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032 frame 2: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032 frame 3: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032 frame 4: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032 frame 5: Drawing Dissolve 5 on twos: a hold of two frames dissolves over one at most, so the same frames as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-032: opening it warns of nothing | [] | yes |
| FX-FBLEND-033 frame 0: Drawing Dissolve 3 on ones: every drawing is shown once, whole. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-033 frame 1: Drawing Dissolve 3 on ones: every drawing is shown once, whole. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-033 frame 2: Drawing Dissolve 3 on ones: every drawing is shown once, whole. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-033: opening it warns of nothing | [] | yes |
| FX-FBLEND-034 frame 0: Drawing Dissolve 1 with a frame of nothing between red and blue: a hold before a gap does not dissolve. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-034 frame 1: Drawing Dissolve 1 with a frame of nothing between red and blue: a hold before a gap does not dissolve. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-034 frame 2: Drawing Dissolve 1 with a frame of nothing between red and blue: a hold before a gap does not dissolve. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-034 frame 3: Drawing Dissolve 1 with a frame of nothing between red and blue: a hold before a gap does not dissolve. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-034 frame 4: Drawing Dissolve 1 with a frame of nothing between red and blue: a hold before a gap does not dissolve. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-034: opening it warns of nothing | [] | yes |
| FX-FBLEND-035 frame 0: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035 frame 1: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035 frame 2: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035 frame 3: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035 frame 4: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035 frame 5: Drawing Dissolve 1 where the same drawing is exposed twice running: frame 1 is red, bit for bit. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-035: opening it warns of nothing | [] | yes |
| FX-FBLEND-036 frame 0: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-036 frame 1: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-036 frame 2: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-036 frame 3: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says ["MEDIA_SEQUENCE_GAP"] | yes |
| FX-FBLEND-036 frame 4: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-036 frame 5: Drawing Dissolve 1 into drawing 2, which has no file: frame 1 is red at half and says MEDIA_SEQUENCE_GAP, as frames 2 and 3 do. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-036: opening it warns of nothing | [] | yes |
| FX-FBLEND-037 frame 0: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037 frame 1: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037 frame 2: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037 frame 3: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037 frame 4: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037 frame 5: Drawing Dissolve 1 with the composition's frame blending switch off: the dissolve is the layer's own and still happens, as FX-FBLEND-030. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-037: opening it warns of nothing | [] | yes |
| FX-FBLEND-038 frame 0: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 1: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 10: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 11: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 2: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 3: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 4: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 5: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 6: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 7: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 8: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038 frame 9: Drawing Dissolve 1, stretch 200 and both switches on: the dissolved frames are mixed again, so frame 1 is three quarters red and a quarter blue. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-038: opening it warns of nothing | [] | yes |
| FX-FBLEND-039 frame 0: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 1: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 10: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 11: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 2: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 3: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 4: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 5: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 6: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 7: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 8: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039 frame 9: Drawing Dissolve 1 and stretch 200, the layer's switch off: each dissolved frame is held two frames. | largest difference 0.0e0, says [] | yes |
| FX-FBLEND-039: opening it warns of nothing | [] | yes |

## The file

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fblend_010.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_011.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_012.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_013.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_014.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_015.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_016.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_017.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_018.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_019.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_020.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_021.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_022.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_023.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_025.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_026.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_027.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_028.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_030.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_031.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_032.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_033.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_034.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_035.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_036.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_037.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_038.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| fx_fblend_039.json opened and saved holds what it held, a stretch of 100 or a switch written false left out | the same | yes |
| FX-FBLEND-024: a file that writes stretch 100 and the switch false is saved without either | false | yes |
| FX-FBLEND-014: a file written before D-216 is saved with none of the four fields | the same | yes |
| FX-FBLEND-050: A stretch below 1. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-051: A stretch above 10000. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-052: A stretch below 0, playing backwards, which is not part of this. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-053: A stretch written as a word. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-054: A layer's frame blending that is Pixel Motion, which is not part of this. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-055: A layer's frame blending written true rather than the word frame_mix. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-056: A composition's switch that is not true or false. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-057: A drawing dissolve that is not a whole number of frames. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-058: A drawing dissolve above 100 frames. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-059: A drawing dissolve below 0. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-060: A stretch on a solid layer, which has no drawings to time. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-061: Frame blending on a null layer. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-062: Frame blending on an adjustment layer. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |
| FX-FBLEND-063: A drawing dissolve on a composition layer, which has no exposure of its own. Refused as PROJECT_SCHEMA_INVALID | PROJECT_SCHEMA_INVALID: This project file cannot be opened, because part of it does not match the project format. | yes |

## FX-FBLEND-040 to 044: the Time Stretch command

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FBLEND-040: In 0, out 6, 100 to 200: the out point goes to 12. One undo puts it back. | taken, out point 12, undone true | yes |
| FX-FBLEND-041: In 10, out 17, 100 to 150: 7 frames become 10.5, rounded half away to 11. One undo puts it back. | taken, out point 21, undone true | yes |
| FX-FBLEND-042: In 0, out 12, 200 to 50: 12 frames become 3. One undo puts it back. | taken, out point 3, undone true | yes |
| FX-FBLEND-043: In 4, out 10, 100 to 1: 6 frames become 0.06, and a layer keeps one frame. One undo puts it back. | taken, out point 5, undone true | yes |
| FX-FBLEND-044: In 0, out 5, 300 to 100: 5 frames become 1.666..., rounded to 2. One undo puts it back. | taken, out point 2, undone true | yes |
| a stretch of 0.5 is refused with a sentence, and nothing changes | A layer cannot be stretched to 0.5%. | yes |
| a stretch of 10001 is refused with a sentence, and nothing changes | A layer cannot be stretched to 10001%. | yes |
| a stretch of -100 is refused with a sentence, and nothing changes | A layer cannot be stretched to -100%. | yes |
| a stretch that is no number is refused with a sentence, and nothing changes | A layer cannot be stretched to NaN%. | yes |
| a drawing dissolve of 101 frames is refused with a sentence, and nothing changes | Drawings cannot dissolve over 101 frames. | yes |
| a stretch on a solid layer is refused with a sentence | "card" has no frames of its own to time this way. | yes |
| frame mix on a solid layer is refused with a sentence | "card" has no frames of its own to time this way. | yes |
| a drawing dissolve on a solid layer is refused with a sentence | "card" has no frames of its own to time this way. | yes |
| on FX-FBLEND-011, the layer's frame mix switched on is taken | taken | yes |
| on FX-FBLEND-011, the composition's frame blending switched on is taken | taken | yes |
| with its frame mix and the composition's switch on, FX-FBLEND-011 frame 3 is no longer a drawing held but a mix, as FX-FBLEND-012 frame 3 is | changed | yes |
| then a drawing dissolve of 2 frames is taken, and frame 1 changes with it | taken, frame 1 changed | yes |
| undo 3 times: frame 3 is the frame it was | byte-identical | yes |

## FX-FBLEND-045 to 049: where keys play on a stretched layer

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FBLEND-045: In 0, stretched from 100 to 200 by the command: the keys stay stored at 0 and 4 and play at 0 and 8. | stored at [0, 4], playing at [0.0, 8.0], each read back at its own frame: true | yes |
| FX-FBLEND-046: In 10, stretch 150, keys at 10, 13 and 17: they play at 10, 14.5 and 20.5, between frames where the stretch puts them. | stored at [10, 13, 17], playing at [10.0, 14.5, 20.5], each read back at its own frame: true | yes |
| FX-FBLEND-047: In 0, stretch 50, keys at 0, 1, 2 and 3: they play at 0, 0.5, 1 and 1.5, and no two land together, which moving the stored keys would do. | stored at [0, 1, 2, 3], playing at [0.0, 0.5, 1.0, 1.5], each read back at its own frame: true | yes |
| FX-FBLEND-048: In 0, stretch 200, a key set with the playhead on frame 5: the key time there is 2.5, stored as 3, rounded half away from zero, which plays at 6. | stored at [3], playing at [6.0], each read back at its own frame: true | yes |
| FX-FBLEND-049: In 1, stretch 300, a key set with the playhead on frame 7: the key time is 3 exactly, stored as 3, which plays at 7. | stored at [3], playing at [7.0], each read back at its own frame: true | yes |

## FX-FBLEND-064 to 067: trimming a stretched layer's in point

| Check | The build's answer | Matches |
| --- | --- | --- |
| FX-FBLEND-064: Stretch 200, in 0, keys at 0 and 4, the in point trimmed to 2: the offset goes from 0 to 1, the keys to 1 and 5, and they still play at 0 and 8. | offset 1, keys at [1, 5], playing at [0.0, 8.0] (were [0.0, 8.0]) | yes |
| FX-FBLEND-065: Stretch 200, the in point trimmed by 1 frame, half a source frame: refused, nothing changes. | Moving the in point by 1 frames would start the layer part way through a frame of its 200% stretched source. | yes |
| FX-FBLEND-066: Stretch 50, offset 1, keys at 2 and 6, the in point trimmed from 0 to 1: the offset goes to 3, the keys to 1 and 5, playing at 1 and 3 as before. | offset 3, keys at [1, 5], playing at [1.0, 3.0] (were [1.0, 3.0]) | yes |
| FX-FBLEND-067: Stretch 100, the in point trimmed from 0 to 3: the offset moves by 3 and the keys stay, as trimming always has. | offset 3, keys at [0, 4], playing at [0.0, 4.0] (were [0.0, 4.0]) | yes |

## A mixed or dissolved layer is marked as built by the processor, which the card lays (B-152)

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fblend_012.json frame 1, a frame half way between two frames: marked as mixed true | true | yes |
| fx_fblend_012.json frame 2, a whole frame of the same layer: marked as mixed false | false | yes |
| fx_fblend_011.json frame 1, a stretched frame held, not mixed: marked as mixed false | false | yes |
| fx_fblend_030.json frame 1, a dissolved frame: marked as mixed true | true | yes |
| fx_fblend_021.json frame 1, a composition layer between two of its frames: marked as mixed true | true | yes |

## The frame does not depend on how it is cut up

| Check | The build's answer | Matches |
| --- | --- | --- |
| fx_fblend_012.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fblend_015.json frame 4 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fblend_021.json frame 3 in tiles of 1 and of 64 | byte-identical | yes |
| fx_fblend_038.json frame 1 in tiles of 1 and of 64 | byte-identical | yes |

## Pictures: a made-up ball in three drawings on twos, in `verification/B-150 pictures/`

| Check | The build's answer | Matches |
| --- | --- | --- |
| on_twos.png, as drawn: each drawing held two frames, whole | [], ball at [(9.5, 0), (9.5, 0), (23.5, 0), (23.5, 0), (37.5, 0), (37.5, 0)] | yes |
| stretched_held.png, stretched to 200% with no mixing: each drawing held four frames, whole | [], ball at [(9.5, 0), (9.5, 0), (9.5, 0), (9.5, 0), (23.5, 0), (23.5, 0), (23.5, 0), (23.5, 0), (37.5, 0), (37.5, 0), (37.5, 0), (37.5, 0)] | yes |
| stretched_mixed.png, stretched to 200% with Frame Mix: frames 3 and 7 show both drawings faint, a ghosted in-between; the rest are whole | [], ball at [(9.5, 0), (9.5, 0), (9.5, 0), (16.5, 224), (23.5, 0), (23.5, 0), (23.5, 0), (30.5, 224), (37.5, 0), (37.5, 0), (37.5, 0), (37.5, 0)] | yes |
| dissolved.png, Drawing Dissolve 1: the second frame of each hold is half this drawing and half the next; the last drawing holds | [], ball at [(9.5, 0), (16.5, 224), (23.5, 0), (30.5, 224), (37.5, 0), (37.5, 0)] | yes |

## Result

390 of 390 checks pass.
