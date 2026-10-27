# B-785 playtest: the named colour spaces, checked against OpenColorIO (D-906)

What changed: the program now knows the colour spaces professional programs name (ACEScg,
ACES2065-1, Rec.2020, P3, and the sRGB, gamma 2.4, gamma 2.2, ACEScct and ACEScc curves), and
how to convert a colour between any two of them. **Nothing in the app uses them yet**, so there
is nothing new to see in the app: every picture and every export is exactly as before. Using
them is the next steps' work (ADR-023, C3 onwards).

The check is a table. OpenColorIO, the colour library After Effects itself uses (version 2.5.1,
the one After Effects 26.5 ships), worked out the right answer for about 5,000 colours per
conversion; the program worked out its own; the table compares them.

## Steps

1. Open `verification/B-785_conversion_table.md` in the project folder.
2. Under "Matrices to ACES2065-1", every one of the 5 rows says **pass**.
3. Under "Conversions", every one of the 30 rows says **pass** in its last column, and the line
   under the table reads "30 of 30 conversions pass."
4. Five rows read "pass by the published formula ... OpenColorIO differs, see note". That is
   expected: for those five, OpenColorIO's own version of the curve is not quite the published
   one, and the program follows the published one. The notes under the table say why, one note
   per curve (srgb, acescc, acescct). They are not failures.
5. Optional, in the app: open any project you already have and export a frame as before. It
   looks exactly as it did before this change.

## What counts as wrong

- Any row saying FAIL, or the line reading fewer than 30 of 30.
- A "pass by the published formula" row without a note under the table explaining it.
- Any picture or export looking different from before.
