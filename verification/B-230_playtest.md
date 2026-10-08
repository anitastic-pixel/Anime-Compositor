# B-230: text animators (typewriter, fade by character, wave)

Built on 2026-10-08 under your effects loop request. This is the first part of P0-7, "Text
engine", decided as D-350. Shaping (scripts such as Arabic or Devanagari that join their letters)
is not part of it and is still to do.

A new effect, **Text Animator**, in the new **Text** group of the effects list, after After
Effects' text animators. Put it on a text layer and it changes the letters one by one:

- What it does to a picked letter: **Position** (moves it, x then y, in pixels), **Scale**
  (per cent, x then y), **Rotation** (degrees, about the letter's foot), **Opacity**, **Fill
  Color** (turn the Fill Color choice On to recolour) and **Tracking Amount** (extra room after
  the letter).
- Which letters are picked, After Effects' **Range Selector**: **Start**, **End** and **Offset**
  (per cent of the line), **Based On** (Characters, Characters Excluding Spaces, or Words),
  **Shape** (Square, Ramp Up, Ramp Down, Triangle, Round, Smooth), **Amount**, **Smoothness**
  (how soft the Square's edge is) and **Ease High** and **Ease Low**.

Every number has a stopwatch, so it can be keyed, and the one-number settings take expressions.
Several animators on one layer add up, top first. With no animator a text layer looks exactly as
before.

The letters are still drawn by the processor, as all text is; whatever effects come after the
animator still run on the graphics card. The check, `verification/D-350_text_animator_table.md`,
61 of 61, holds every letter's place, size, turn, opacity and colour to numbers worked out by a
separate program before the code existed. The pictures are in `verification/D-350 pictures/`
(drawn over dark grey so white letters show).

## What to check

Make a composition, take the Text tool (T), click in the viewer and type `Typewriter text`. Make the
text white and about 100 pixels. Add **Text Animator** from the effects list (the **Text** group).

1. **Nothing changes yet.** As added, every setting is at rest, so the words look as they did.
2. **Typewriter.** Set **Opacity** to 0: the whole line disappears. Set **Smoothness** to 0. Turn
   on the stopwatch for **Start**: at frame 0 set Start 0, at frame 15 set Start 100. Play: the
   letters appear one at a time, left to right, like `typewriter_frame_00.png` through
   `typewriter_frame_15.png` (frame 6 shows `Typewr`). Set Smoothness back to 100: each letter
   now fades in as it comes (`typewriter_soft.png` is Start 43.3).
3. **Fade in by character.** New text, `Fade in by letter`, a new Text Animator: Opacity 0,
   **Shape** Ramp Up, **Ease High** 50, **Ease Low** 50. Key **Offset** from -100 at frame 0 to
   100 at frame 24. Play: the line fades in from the left with a soft travelling edge. At Offset
   -30 it should look like `fade_in_by_character.png`. Shape Ramp Down gives the other way round
   (`fade_out_by_character.png` is Offset 30).
4. **Word by word.** Set **Based On** to Words, Position 0, 100, Ease Low 100: whole words rise
   and fade in together, and the spaces never count (`word_by_word.png` is `Smooth word by word`
   at Offset -20, Ease High 25).
5. **A wave.** New text, `Wave of letters`, a new Text Animator: **Position** 0, -60, **Start**
   20, **End** 50, **Shape** Triangle. The letters in the range rise, most in the middle
   (`wave.png` is Offset 10). Key **Offset** from -100 at frame 0 to 100 at frame 48 and play: a
   bump runs along the line. Shape Smooth or Round gives a softer bump (`humps.png`).
6. **Turn, size and colour.** Rotation 30, Scale 150 by 50, Fill Color On with a green, Tracking
   Amount 50: the picked letters turn about their feet, stretch, recolour and spread out
   (`turn_scale_colour.png` shows two animators doing this on `Spin & Scale`).
7. **Two animators.** Put both a wave and a fade on one line: they add up
   (`thirty_characters.png`).
8. **An expression.** On **Start** of a typewriter, type the expression `time*24*10`: the line
   types itself at ten per cent a frame without keys.
9. **On a solid.** Add a Text Animator to a solid: the solid is drawn unchanged and a warning
   says the layer has no words.
10. **Save and open.** Save, close and reopen: the settings and keys should be as you left them.
    A project saved before this build opens exactly as it did.
11. **Same as an export.** Export a frame of the typewriter and put it next to the viewer. They
    should look the same.

## Not done, on purpose or for later

- **Shaping** of joined scripts is the rest of P0-7.
- After Effects' **Wiggly** and **Expression** selectors, a second selector on one animator, a
  selector's **Mode** (Add, Subtract and so on), **Randomize Order**, **Units: Index** and
  **Based On: Lines** are not here. One animator has one Range Selector, in per cent.
- After Effects' per-character 3D, per-character motion blur, Anchor Point Grouping and Grouping
  Alignment are not here; each letter turns and scales about its own foot.
- Ease High and Ease Low go from 0 to 100 here (After Effects also allows below 0).
- Expressions work on the one-number settings only (Start, End, Offset, Amount, Opacity and so
  on), not on Position, Scale or Fill Color, as for every other effect.

## If something is wrong

Say which step. The most likely fault would be in step 6 (a letter turning about the wrong point)
or with a centred or right-aligned line and Tracking Amount (step 6 with alignment changed).
