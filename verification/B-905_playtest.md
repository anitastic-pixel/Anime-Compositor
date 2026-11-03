# B-905 playtest: Extra Angle past a full turn

What changed (D-1025): Particle Space's and Particle Plane's **Extra Angle** takes whole turns,
as After Effects' CC Particle World does. The snow tutorial types "0x+77" after "1x+0"; typing
"1x+77" (437 degrees) used to switch the whole effect off. Now anything from 0 to 100 turns is
taken. Where Extra Angle is how wide the cone opens (Direction Axis, Cone Axis, Fractal Uni), a
cone wider than one turn is drawn as one turn, every way. Saved projects look exactly as before.

1. Add a solid, then Effect > Simulation > Particle Space. Set Animation to Direction Axis.
   - Expect: Extra Angle reads "1x+0°".
2. Click Extra Angle and type `1x+77`, then Enter.
   - Expect: it reads "1x+77°"; the particles are still drawn (before, they vanished and the
     effect was off); the spray goes every way, as at "1x+0°".
3. Type `0x+77`.
   - Expect: the spray narrows to a cone about 77 degrees wide, upwards.
4. Drag Extra Angle far to the right.
   - Expect: it keeps counting turns (2x, 3x ...) and stops at 100x+0°; nothing switches off.
5. Do steps 2 and 3 on Particle Plane with Animation Direction.
   - Expect: the same: taken, and a fan every way past one turn.
6. Save, close and reopen; open a particle project saved before this change.
   - Expect: your 1x+77 is still there; the old project looks as it did.

The snow look itself (dots far too small, the falling area too narrow) is not this change: see
D-1026, proposed, and its sheet `verification/D-1026 pictures/snow.png`.
