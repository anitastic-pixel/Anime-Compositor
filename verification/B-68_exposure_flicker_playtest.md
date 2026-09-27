# B-68: Exposure Flicker, by hand

Built on 2026-09-26 against D-125, which you accepted the same day as the third of the second
batch of ten ("go ahead with the ten").

The generated halves are `verification/B-68_exposure_flicker_table.md`, 130 of 130, which renders
every FX-FLICKER case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Exposure Flicker card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window. The picture in `verification/B-68a proposal/` shows what to expect.

## Before you start

Open a project with a drawn layer at least a second long, select the layer and press
**Full resolution**.

## What to check

1. **Adding it.** Pick **Exposure Flicker** in **Add effect…**. It goes to the end of the stack,
   and the card shows **Amount** 0.25, **Hold** 1 and **Seed** 0. Play: the drawing's
   brightness jitters a little on every frame, the whole drawing at once.
2. **Stronger.** Amount 1.5: the jitter is plain to see, some frames much brighter, some much
   darker.
3. **Hold.** Hold 4: each brightness lasts four frames before it jumps to the next. Hold 100: it
   hardly changes over a few seconds.
4. **Seed.** Seed 7: a different pattern of bright and dark frames. Back to 0: the first
   pattern again, exactly.
5. **Same every time.** Step to a frame, note how bright it is, step away and back: it is the
   same brightness.
6. **Amount 0.** The drawing is as it was on every frame.
7. **Edges.** Soft edges stay soft; nothing outside the drawing lights up.
8. **Out of range.** Type 5 in Amount: it is refused with a sentence saying it runs from 0 to 4,
   and the card keeps its old number.
9. **Keyed.** Key Amount at 0 and at 2 at a later frame. Play: the flicker starts still and
   grows.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every
    setting and key is still there, and the flicker is the same as before.

## Known limits, on purpose

- The brightness jumps from one held value to the next and never glides between them.
- The whole layer flickers as one; no part of it is brighter than another.
- A bright colour pushed up passes white in the working values, and the screen clips it.
- It is modelled on After Effects' Exposure with a seeded wiggle on its stops and is not
  claimed to match any plug-in.

## What to answer

"works", or which step number did something else and what it did.
