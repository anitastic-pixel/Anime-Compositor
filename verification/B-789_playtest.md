# B-789 playtest: stored colours, blend modes and depths with colour management (D-910, step C4)

What changed: a project with colour management on and working in ACEScg now gives the same
picture as the same project working in plain linear Rec.709 (today's colours). A solid's colour,
a gradient's two colours, a blend mode such as Overlay and a luma matte all keep the meaning they
have today. Nothing changes for any project without colour management, which is every project
you have.

The colour management switch is still **not in the window** (it comes with step C5, together with
a viewer that shows ACEScg properly). So in the app an ACEScg project is still shown through the
old sRGB curve and looks duller than its Rec.709 twin; that is gap 2 below, not a fault. The
pictures in `verification/B-789 pictures/` show what the two projects really hold, the ACEScg one
taken back to Rec.709 for viewing.

The test files are in `Fixtures/colour/c4/` in the project folder.

## Steps

1. Open `verification/B-789 pictures/`. There are four pairs: `solid`, `gradient`, `overlay` and
   `luma_matte`, each as `_rec709.png` and `_acescg_back_to_rec709.png`.
   - Each pair should look identical: an orange bar; an orange-to-blue gradient; a tan
     (light brown) overlay; a red bar that fades in from black on the left.
2. In the app, open `Fixtures/colour/c4/refused_eight_bpc.json`.
   - It does not open. The message says: "With colour management on, a composition works in
     Display or Float: 8 bpc (AE) and 32 bpc (AE) copy After Effects without colour management."
     The note names the composition. The file on disk is unchanged.
3. Open `Fixtures/colour/c4/refused_ae_32bpc.json`. Same message, same result.
4. Open `Fixtures/colour/c4/float_acescg.json`. It opens normally.
5. Open any project you already have. It looks and saves exactly as before.

## What counts as wrong

- Any pair in step 1 that does not look the same.
- Step 2 or 3 opening, or a different or missing message.
- Step 4 refusing to open.
- Any old project looking different.

## Not in this step (the gaps)

1. The graphics card does not draw ACEScg yet: such a frame is drawn by the processor, and the
   notes say so (GPU_PREVIEW_ON_CPU).
2. The viewer still shows ACEScg numbers through the old sRGB curve (C5), and the switch is not in
   the window (C5).
3. Effects run on Rec.709 numbers, so a colour outside Rec.709's range is clipped where an effect
   clips, and an effect working channel by channel works on Rec.709's channels.
4. A composition already in Display depth stays in Display; nothing chooses Float for you.
5. Exports still write the working numbers labelled sRGB (C6).

The numbers behind each picture are in `verification/B-789_stored_colours_table.md`
(14 of 14 checks pass).
