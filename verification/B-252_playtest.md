# B-252: Path Stroke along a text layer's letters (Text Outlines)

Built on 2026-10-09 (D-373, accepted under your /loop request), on top of B-251's letter placing.

Path Stroke can now draw along the letters of a text layer. Put Path Stroke on a text layer and
set **Path From** to **Text Outlines**: every outline of every letter becomes one path, closed,
counted in reading order (the first letter's outlines, then the next letter's). A letter with a
hole has more than one outline: "e" has its outer edge and its eye. Path picks one outline by
number; All Masks takes them all; with Stroke Sequentially on they draw one after another as
one long line, so keying End from 0 to 100 writes the word on, letter by letter. Text Animators
that move letters move the outlines with them. Typing new words changes the outlines on the next
frame.

If there is nothing to draw along (the layer is not a text layer, it holds only spaces, or Path
asks for an outline past the last), the layer is drawn without the stroke and a warning
saying it has no text outline to draw along is shown, every frame, as Path Stroke already does with masks.

**The picture:** `verification/D-373 pictures/write_strip.png`. "Write" in white over the town,
an orange line, All Masks and Stroke Sequentially on, End keyed from 0 to 100 over one second:

- **End 0** (left): no line yet, the plain white word.
- **End 50** (middle): the line has gone all the way round the W and the r and part of the i;
  the rest of the word is still plain.
- **End 100** (right): every letter outlined in orange, the i's dot and the e's eye too.

Each is also alone as `write_end_000.png`, `write_end_050.png` and `write_end_100.png`.
`write_reveal.png` is the same word with Reveal Original Image and a wide soft brush: only a band
along the letters' edges shows.

## Checks

- `verification/D-373_stroke_text_table.md`: **64 of 64**. FX-STROKE-062 to 069 match the
  independent reference (HarfBuzz and fontTools, written before the code) within 0.00000003; the
  four outlines of "Yes" are the same length as the reference's to 14 decimal places; saving,
  loading, keying, undo and tiles; the viewer on the card within 1 level of 255 of the export at
  Full and Draft.
- Older checks still pass: b155, b175, b230, b235, b236, b28b_timesheet, d263, d264, d371.
- Timing: `verification/B-252_stroke_text_timing_table.md` (quiet machine). On a 1080p text
  layer the stroke adds about 10 ms a frame on the card and about 5 ms on the processor, which
  is **over** the 4 ms target: the stroked text layer is redrawn every frame instead of kept.
  Fixing that is a separate change.

## What to check

1. Make a text layer, size 200, and type `Write`. Add Path Stroke, set Path From to Text
   Outlines, turn All Masks and Stroke Sequentially on, pick a colour and Brush Size 6.
2. Key End at 0 on the first frame and 100 a second later. Play: the line should draw round the
   W first, then each letter in turn, as in `write_strip.png`.
3. Set All Masks off and step Path through 1, 2, 3...: one outline at a time; on the "e" you get
   its eye and its outer edge as two steps.
4. Set Path to 99: the stroke disappears and the warning "Layer ...'s Path Stroke has no text
   outline 99 to draw along, so it draws nothing." appears.
5. Save, close and reopen the project: Path From still says Text Outlines and the keys are kept.

## Not like After Effects

After Effects does this with **Layer > Create > Create Masks from Text**, which makes a new solid
holding one mask per outline and turns the text off; Stroke then draws along those masks. Here the
text layer is the path source directly and stays live (edit the words and the stroke follows).
Not done: a command that turns letters into editable masks or shape layers; choosing letters by
hand rather than by outline number; reordering the outlines; open (unclosed) masks.
