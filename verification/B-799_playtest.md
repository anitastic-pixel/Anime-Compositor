# B-799 playtest: Directional Blur's Taper Ends

What changed: Directional Blur has a new setting, **Taper Ends**, under Edges. On (the default)
is the streak that fades away at both ends, as After Effects' does and as it has been since
B-632. Off brings back the old streak: evenly bright all along, with hard ends, a little stronger.

1. Open any project with a drawing, select a layer, and add Effect > Blur & Sharpen >
   Directional Blur. Set Length to about 60 so the streak is easy to see.
   - Expect: Taper Ends reads **On**, and the streak fades out softly at both ends.
2. Set Taper Ends to **Off**.
   - Expect: the streak becomes evenly bright from end to end and stops with a hard edge. The
     drawing does not move and the streak is the same length.
3. Set it back to **On**.
   - Expect: exactly the picture from step 1 again.
4. Set it to Off, save the project, close it and open it again.
   - Expect: Taper Ends still reads Off and the streak is still hard-ended.
5. Open a project saved before this change that has a Directional Blur.
   - Expect: Taper Ends reads On and the picture is exactly as it was.
6. Optional: with the graphics card on and off (or at Draft and Full), the Off streak looks the same.

Pass if every "Expect" holds. If anything looks different, say which step.
