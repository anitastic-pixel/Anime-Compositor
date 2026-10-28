# B-826 playtest: parent and track matte on every chosen layer (D-946)

What changed: with several layers chosen, setting a Parent or a Track matte on one of them sets it on all of them, and one Ctrl+Z takes it back. A layer given its first matte gets "matte only" ticked, so the matte layer stops showing by itself, as After Effects turns its eye off. The Track matte row has its own pick whip (@).

## Steps

1. Open a project with a null and three drawn layers (or add a null with **Ctrl+Alt+Shift+Y**).
2. Click the first drawn layer, then **Ctrl+click** the other two, so all three are lit.
3. In the Parent list of any one of the three, choose the null. All three now show the null as their Parent.
4. Move the null in the viewer: all three move with it.
5. Press **Ctrl+Z** once: all three lose the parent together.
6. With the three still chosen, drag the **@** beside one of their Parent lists onto the null's row. All three get the null as parent again.
7. Add a text or shape layer to act as a matte. Choose the three layers again, open one's Blending group (twirl it down) and pick the matte layer in **Track matte**. All three are now cut by it, "matte only" is ticked, and the matte layer itself no longer shows over the picture.
8. Untick "matte only" by hand: the matte layer shows again (the tick is yours to change).
9. On another layer, drag the **@** beside Track matte onto the matte layer's row: the label says "Track matte: <name>" while you drag, and letting go sets it.

## What you should not see

- Only the one layer whose list you used changing.
- A layer being made its own parent or matte, or a parent loop (those layers are left out quietly, as the lists already leave them out).

The computer's checks are in `verification/B-826_parent_and_matte_table.md`; the check in a private copy of the app is `verification/B-826_private_app_table.md`.
