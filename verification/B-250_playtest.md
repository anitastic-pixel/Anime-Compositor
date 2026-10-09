# B-250: text shaping (Arabic, Devanagari, accents, ligatures)

Built on 2026-10-09 under your effects loop request. This is the second and last part of P0-7,
"Text engine", decided as D-371, using the `rustybuzz` crate you accepted as D-354. Ligatures are
on, as you chose on 2026-10-09 ("ligatures on by default").

Until now a text layer drew one letter at a time, each in its own shape, left to right. Now the
words are **shaped** first, the way a word processor does it:

- **Arabic** letters join into words, take the right form for the start, middle or end of a
  word, and read from right to left.
- **Devanagari** builds its joined letters (conjuncts), and the short i vowel is drawn before the
  letter it follows when typed.
- **An accent typed after its letter** (as some keyboards do) sits on that letter.
- **Ligatures**: in the bundled font, "fi", "fl", "ff" and "ffi" are drawn as one joined shape.
- **Kerning**, when it is ticked, now matches what HarfBuzz (the shaper in browsers and most
  design programs) gives. In Tahoma that fixes pairs the old kerning got wrong.

Nothing else changes: Latin and Japanese text without ligature pairs draws exactly as before
(every earlier text check still gives the same pictures, byte for byte). Text is still drawn by
the processor.

**The picture:** `verification/B-250 pictures/before_after.png`, the same three lines before and
after, at the same size and place. In the "before" Latin line the letters "ff" at the end run off
the edge: without the ligatures the line is a little wider.

The check, `verification/D-371_text_shaping_table.md`, 16 of 16, holds every shaped glyph to
HarfBuzz itself, worked by a separate program before the code existed, and draws four pictures in
`verification/D-371 pictures/`. One of the 16 is marked "in dispute": see the decision at the end.

## What to check

Make a composition and take the Text tool (T).

1. **Ligatures.** Type `office fly fit staff` in the bundled font, about 100 pixels. Look closely
   at "ffi", "fl", "fi" and "ff": the f's top reaches over and joins the next letter, as in the
   right-hand bottom line of `before_after.png`. The line is slightly narrower than before.
2. **Arabic.** Change the font to `segoeui.ttf` (or `arial.ttf`) and paste
   `السلام عليكم`. The letters should be joined into two words, read right to left, the first
   letter typed on the right: compare the top right of `before_after.png`.
3. **Devanagari.** Change the font to `Nirmala.ttc` and paste `नमस्ते हिन्दी`. The "st" and "nd"
   should be joined shapes, with no little slanted stroke under the letters, and the vowel hook
   of "hi" drawn before its letter: the middle right of `before_after.png`.
4. **An accent.** In the bundled font, type `cafe` and then the combining acute accent
   (Windows: paste `é` written as e plus U+0301, for example from a character map). It shows
   as one é, not an e with a loose accent beside it.
5. **Typewriter on a ligature.** Put the typewriter from B-230 on `office`. The "ffi" appears as
   one shape when its first "f" is reached; the next two steps add nothing visible, then "c"
   appears.
6. **Save and open.** Save, close and reopen: the text looks the same. A project saved before this
   build opens and draws as it did, apart from the joins and ligatures above.

## Not done, or for later (logged as gaps)

- In a line mixing left-to-right and right-to-left words, a punctuation mark or space between
  them always sides with the run before it. The full Unicode rule would sometimes put it with the
  paragraph's direction.
- Figures at the very start of an Arabic paragraph come out in reverse order.
- An Arabic phrase that figures split in two, inside an English line, is laid out as two separate
  right-to-left pieces instead of one.
- When a text box wraps a line, the words are not shaped again at the break, so an Arabic letter
  at the end of a wrapped line keeps its joined form.
- There is no switch to turn ligatures off (you chose them on; a switch can come later).

## A decision for you: D-372

The program that worked out the expected numbers drew each letter's outline the way FreeType
and HarfBuzz do: it nudges the letter sideways so its left edge sits exactly where the font's
spacing table says. Our text drawing, since D-263, draws each letter's points as the font stores
them, without that nudge. In the bundled font the two differ for about half its letters, by at
most 0.7 pixel at size 100 for ordinary letters (for example "y" moves 0.4 pixel), and up to 2.9
pixels for a few Japanese brackets. The check FX-SHAPE-020 shows the difference and passes once
the nudge is taken out. Choose one:

- **(a)** Add the nudge to our text drawing, to match FreeType, HarfBuzz and Windows. Some
  letters move a fraction of a pixel to the right in every text picture.
- **(b)** Keep our drawing as it is, and have FX-SHAPE-020's expected boxes redrawn without the
  nudge, as the D-350 check already does. This changes a fixture, so it needs your say.

## If something is wrong

Say which step. The most likely fault would be in a line that mixes Arabic and English (see the
gaps), or a box-wrapped Arabic line.
