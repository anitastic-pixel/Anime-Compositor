# B-95: Bulge, by hand

Built on 2026-09-26 against D-152, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the nineteenth of the thirty.

The generated halves are `verification/B-95_bulge_table.md`, which renders every FX-BULGE case
against the numbers written before the code, and `verification/B-12b_state_fields_table.md`,
which checks the Bulge card sends every setting the command reads. This sheet covers what the
tables cannot: how it looks and feels in the window. The picture in
`verification/B-95a proposal/` shows what to expect.

## Before you start

Open a project with a full-frame background layer with plenty of detail. Select the layer and
press **Full resolution**.

## What to check

1. **Adding it.** Pick **Bulge** in **Add effect…**. It goes to the end of the stack, and the
   card shows **Centre** 50, 50, **Radius** 50 and **Height** 1. A small patch in the middle of
   the picture, 50 pixels round, is swelled out as if under a lens, most at its centre, blending
   back to the picture at its edge.
2. **Bigger.** Radius 300: the swell covers much more of the picture.
3. **Pinch.** Height -2: the patch is pinched inward instead, the picture squeezed toward the
   middle.
4. **Strong.** Height 4: a big magnifying bubble in the middle, softer than the rest because it
   is enlarged.
5. **Nothing.** Height 0, or Radius 0: the picture untouched.
6. **Centre.** Centre 25, 25: the bulge moves up and to the left.
7. **Out of range.** Type 4.5 in Height: it is refused with a sentence saying it runs from -4 to
   4, and the card keeps its old number.
8. **Keyed.** Key Height at 0 and at 3 at a later frame, with Radius 200. Play: the middle of
   the picture swells up like a lens sliding into place, an impact or "zoom in on the eyes"
   beat.
9. **Draft preview.** Switch to a half-size draft: the bulge covers the same part of the
   picture as at full size.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- **The bulge is always a circle**, with no separate width and height.
- The swell's shape is one fixed curve, with no taper setting.
- A strong swell softens the part of the picture it magnifies.
- A swell only shows what is inside the drawing, never anything past its edge, and the layer
  does not grow.
- It is modelled on After Effects' Bulge and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
