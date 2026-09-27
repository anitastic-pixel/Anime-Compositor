# B-100: Venetian Blinds, by hand

Built on 2026-09-26 against D-157, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the twenty-fourth of the thirty.

The generated halves are `verification/B-100_venetian_blinds_table.md`, which renders every
FX-BLINDS case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Venetian Blinds card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-100a proposal/` shows what to expect.

One case, FX-BLINDS-026, a file with completion written as the word "50", is shown in the table
as in dispute rather than passed or failed, for the same reason as Linear Wipe's FX-LWIPE-032:
D-164, proposed in document 14, asks you whether such a file should open.

## Before you start

Open a project with two layers: a full-frame background below and a full-frame picture above.
Select the top layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Venetian Blinds** in **Add effect…** on the top layer. It goes to the
   end of the stack, and the card shows **Completion** 0, **Angle** 0, **Width** 20 and
   **Feather** 0. Nothing changes.
2. **Closing.** Completion 50: the top layer is cut into level stripes 20 pixels tall, and the
   lower half of each stripe is gone, showing the background through, as through half-open
   blinds.
3. **Upright.** Angle 90: the stripes stand upright, each cleared from its left side.
4. **Tilted.** Angle 30: the stripes lean.
5. **Width and feather.** Width 60: fewer, taller stripes. Feather 10: the moving edge of each
   stripe fades rather than cuts; the other edge stays hard.
6. **Out of range.** Type 0.5 in Width: it is refused with a sentence saying it runs from 1 to
   10000, and the card keeps its old number.
7. **Keyed.** Key Completion at 0 and at 100 a second later. Play: the picture closes away in
   stripes, revealing the one beneath.
8. **Draft preview.** Switch to a half-size draft: the stripes cover the same parts of the
   picture as at full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- **Only the moving edge is feathered**; the fixed edge of each stripe stays hard.
- The stripes are counted from the layer's own top-left corner, so moving the layer moves them
  with it.
- At angles that are not whole quarter turns, the stripe edges fall between pixels and look a
  little stepped with feather 0.
- It is modelled on After Effects' Venetian Blinds and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
