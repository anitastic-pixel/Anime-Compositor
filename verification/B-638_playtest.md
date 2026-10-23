# B-638 playtest: a new Stroke starts at After Effects' values (D-759)

## What changed for you

A Path Stroke you add from now on starts at **Brush Hardness 79** with **Stroke Sequentially on**,
as After Effects' panel shows a fresh Stroke (five tutorials by five creators all show this). Before,
ours started at 75 and off. Every other starting value already matched.

Stroke Sequentially only does something when All Masks is on, so a new Stroke on one mask looks the
same as before except for a very slightly harder brush edge (79 against 75).

A project you saved before this change looks exactly as it did: its Stroke keeps the 75 and off (or
whatever you set) that the file holds.

No picture of an older project changes, so there is no After Effects / before / after picture sheet.
Instead the proof is the panel's starting values side by side.

## What to look at

`verification/B-638 pictures/1_stroke_start_ae_before_after.png`: After Effects' panel just after
adding a Stroke (Skyona's tutorial, 2:15), then ours before and after this change. The two yellow
rows are the two that changed; after the change they read 79 and on, as After Effects' panel does.

`verification/B-638_stroke_start_table.md`: the app check, 7 of 7 pass.

## Try it

1. Make a layer with a mask, then add Path Stroke to it. In its settings, Brush Hardness shows 79
   and Stroke Sequentially shows On.
2. Type 75 in Brush Hardness and set Stroke Sequentially to Off: both are kept.
3. Open a project you saved before today that has a Stroke: it looks the same as before, and its
   Brush Hardness and Stroke Sequentially show what you saved (75 and Off if you never changed them).

## Known differences from After Effects (not fixed here)

- After Effects' line is about twice as wide as its Brush Size; ours is exactly the Brush Size
  (3 tutorials agree, not yet enough to change it).
- How the Hardness softens the edge, and how Start and End are timed, were not measured.
