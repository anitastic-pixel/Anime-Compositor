# B-78: Brightness & Contrast, by hand

Built on 2026-09-26 against D-135, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the second of the thirty.

The generated halves are `verification/B-78_brightness_contrast_table.md`, which renders every
FX-BRICON case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Brightness & Contrast card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks and
feels in the window. The picture in `verification/B-78a proposal/` shows what to expect.

## Before you start

Open a project with a coloured drawn layer, a character with dark lines, light skin and some
empty space around it, select the layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Brightness & Contrast** in **Add effect…**. It goes to the end of the
   stack, and the card shows **Brightness** at 0 and **Contrast** at 0. Nothing changes yet.
2. **Brighter.** Brightness 50: everything lifts evenly. Black lines turn a dark grey, the skin
   gets lighter, and white stays white. The empty parts stay empty.
3. **Darker.** Brightness -50: everything drops evenly. White turns a light grey, black stays
   black.
4. **More contrast.** Contrast 50: darks get darker and lights get lighter, the middle stays put.
   Contrast 100: every colour snaps to full or nothing in each channel, a hard poster look.
5. **Less contrast.** Contrast -50: everything is drawn half way to a flat grey. Contrast -100:
   the whole drawing is one flat middle grey silhouette.
6. **Out of range.** Type 151 in Brightness, or 101 in Contrast: it is refused with a sentence
   saying the range, and the card keeps its old number.
7. **Keyed.** Key Brightness at 0 and at 100 at a later frame. Play: the drawing fades up
   towards white.
8. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
   and key is still there.

## Known limits, on purpose

- The change is worked on the stored (sRGB) values, as a paint program does, so Brightness is
  levels of 255 added and the middle of Contrast is the grey a paint program calls the middle.
- **Contrast is steep near the top.** Above about 80 each step makes a big jump: 90 pushes the
  tones apart 10 times, 100 pushes them apart 100 times. Small moves near 100 look like big ones.
- Contrast is worked first, then brightness is added.
- It is modelled on After Effects' Brightness & Contrast and is not claimed to match it.

## What to answer

"works", or which step number did something else and what it did.
