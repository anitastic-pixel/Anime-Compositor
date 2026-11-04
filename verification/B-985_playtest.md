# B-985 playtest: Card Wipe's Position Jitter and Rotation Jitter

What changed (D-1105): Card Wipe has After Effects' **Position Jitter** (X, Y and Z Jitter Amount
and Speed) and **Rotation Jitter** (X, Y and Z Rot Jitter Amount and Speed), twelve settings at
the bottom of its list, each with a key diamond. Each card is slid and turned by its own smooth
random amount, which drifts to a new value Speed times a second. Amounts are added at 0 and
speeds at 1, so a new Card Wipe, and every older project, looks exactly as before.

1. Make a layer with a picture on it and add Card Wipe from the effect search. Set Transition
   Completion to 30, Rows 6 and Columns 10.
   - Expect: the wipe as before. The twelve new settings read 0 (amounts) and 1 (speeds).
2. Set X Jitter Amount and Y Jitter Amount to 0.2, and both speeds to 6. Play.
   - Expect: every card, flipped or not, wobbles a little around its place, each its own way,
     smoothly, without jumps. Compare `verification/B-985 pictures/1_position_jitter.png`.
3. Put them back to 0. Set Z Jitter Amount 1, speed 6. Play.
   - Expect: cards move toward and away from you, looking larger and smaller. Compare
     `2_depth_jitter.png`.
4. Put it back to 0. Set X and Y Rot Jitter Amount 25, Z Rot Jitter Amount 15, all three speeds
   6. Play.
   - Expect: cards tip and twist at random and the light on them changes as they turn. Compare
     `3_rotation_jitter.png`.
5. Set a speed to 0.
   - Expect: that wobble freezes in place (the cards stay moved, but no longer drift).
6. Type 101 into X Jitter Amount.
   - Expect: it is refused with "Card Wipe's x jitter amount runs from 0 to 100, and this is
     101." and nothing changes.
7. Save, close and open the project again.
   - Expect: the settings and any keys are as you left them; the same frame looks the same. A
     project saved before this change opens looking exactly as it did.

Pass if every "Expect" holds. If anything looks different, say which step.

Checked by `tests/b985_card_wipe_jitter.rs`, `verification/B-985_card_wipe_jitter_table.md`
(67 of 67): FX-CW-070 to 081 against the tool's expected pixels, the graphics card equal to the
processor on 070 to 078, and the pictures in `verification/B-985 pictures/` (`0_no_jitter.png`
for comparison).
