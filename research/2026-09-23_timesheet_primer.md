# Timesheets, for the owner

Written on 2026-09-23 alongside D-84, for the owner's later study of timesheets (also called
exposure sheets, X-sheets or dope sheets) and of what a compositor does with one. Like everything
in `research/`, it authorises nothing. What the XDTS file itself says comes from CELSYS's public
specification. What is said about paper practice is general knowledge, not checked against any
studio's sheets, and is marked where it is less certain.

## What a timesheet is

A timesheet is the timing of a cut, written down. Time runs **down** the page, one row per
frame, at 24 frames to a second. Across the page are **columns**, one for each stack of
drawings, lettered A, B, C and so on. A cut is one shot. On paper, one sheet commonly holds six
seconds, which is 144 rows.

In a column, the number written on a row is **which drawing is shown** from that frame on. The
animator does not write the number again on every row. A line drawn down from it, or just empty
rows, means the same drawing **holds**. A cross (×) means **nothing** is shown in that column
from there on, until the next number. This is called a blank cell.

So a column that reads `1, (empty), 2, (empty), 3, (empty)` shows each drawing for two frames.
That is animating **on twos**, the everyday rate for anime. **On ones** is a new drawing every
frame, and **on threes** is one every three frames. A single column can change rate
mid-shot, and it can go back to an earlier drawing. For example, 1, 2, 3, 2, 1 is a cycle,
since the drawings are just numbered pictures.

## Columns are layers

Each column is a layer of the finished picture. In a compositor, drawing A's folder becomes
layer A, B's becomes B, and so on, stacked over the background. Which column is on top is the
sheet's to say. XDTS numbers its columns from 0 at the **bottom**. On paper, studios have their
own habits. Check the sheet rather than assume that A is at the back.

The background is usually **not** a column. It is noted on the sheet and painted separately,
and it holds for the whole cut. That is why Import Cut reports a `BG` folder as "not used"
instead of guessing where it goes.

## The other columns

A full sheet has more than drawings:

- **Dialogue**: who speaks, and the frames each line runs across. The mouth drawings are
  timed against it.
- **Camerawork**: pans, follows, zooms, shakes. These are the compositor's instructions and
  the reason a compositor reads the sheet at all.
- **Marks**: CSP's XDTS has two tick marks. The *inbetween* mark shows where inbetweens go. The
  *reverse sheet* mark is, in CSP's words, a "reverse sheet symbol". Its exact meaning in
  studio practice is not confirmed here.

This build reads only the drawing columns. It reports the others, so nothing is silently
dropped. Reading dialogue and camerawork as markers is a later option (document 15, R-01).

## What XDTS is

XDTS is a text file. Its first line is `exchangeDigitalTimeSheet Save Data`, and the rest is a
structured list:

- **columns**, grouped into fields: 0 for drawings, 3 for dialogue, 5 for camerawork;
- the **frames** where each column changes, counted from 0;
- the sheet's **length** in frames;
- the column **names**.

Like the animator's pencil, it writes a column only where something changes. `SYMBOL_NULL_CELL`
is the cross. `SYMBOL_HYPHEN` means "carry on". It has **no frame rate**, so Import Cut uses 24
and says so.

Programs that save XDTS:

- Clip Studio Paint and OpenToonz can, at least in recent versions.
- Toon Boom Harmony was not confirmed.
- A **paper** sheet cannot be read. It would first have to be typed into one of those programs
  and saved from there.

## How to learn from the sample cut

`Fixtures/xdts/fx_xdts_040` is a two-second made-up cut with three columns:

- **A** is a body on twos that holds, then steps back.
- **B** is a mouth that moves while a line of dialogue runs, then goes blank.
- **C** is an effect that comes in late.

Every drawing is a coloured square. Its row says which column it belongs to, and how far right
it sits says its drawing number.

Once B-28c is built:

1. Import that folder.
2. Step through it a frame at a time.
3. Keep document 25's printed sheet for FX-XDTS-040 open beside it.

Each row of the printed sheet is one frame, and the squares should do what it says. Then try
editing a number in the `.xdts` file with a text editor and importing it again.
