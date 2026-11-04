# B-968: Source Text expressions (D-1088)

Built on 2026-11-03 as D-1088 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). The tutorial is
pArivwQxgVY at 4:30 to 5:04: a text layer's Source Text is alt-clicked and pick-whipped to a
Slider Control, and the text then shows the slider's number.

A text layer's words can now come from an expression, worked out on every frame. A number shows
as JavaScript writes it (14.583333333333334, not rounded), so use `Math.round(...)` to round it;
`+ "%"` joins words on. `value` is the words you typed. With the expression switched off, or if
it fails, the typed words are drawn, and a failing one is said beside it and stops an export.

The check, `verification/B-968_source_text_expression_table.md` (68 of 68), runs seventeen
expressions at four frames each against words worked out by a separate program
(`tools/expression_reference.py`) before the code existed.

## What to check

1. **The sheet.** `verification/B-968 pictures/1_source_text_expression.png`: at the top the
   tutorial's frame. Below, our app drawing a text layer at frames 0, 12, 24, 36 and 48 with four
   expressions. Each picture should match the grey "should read" under it: the slider's number
   counting 0 to 100, the same rounded with "%", "Frame" and the frame number, and with the
   expression switched off, the typed words "25%" every time.
2. **In the app.** Make a text layer and a Null or adjustment layer named Controls with
   Effects > Expression Controls > Slider Control; key the slider 0 at frame 0 and 100 a second
   later. Select the text layer; in Effect controls, **Alt+click the word "Text"** under Source
   text. An expression box appears holding `value`. Type
   `Math.round(thisComp.layer("Controls").effect("Slider Control")("Slider")) + "%"` and click
   away. Play: the text counts up from 0% to 100%.
3. **Switch it off.** Press the `=` button beside the box: the typed words come back. Press it
   again: the count returns. Alt+click "Text" again: the expression is taken away.
4. **A mistake.** Type `"x" - 1`: a warning appears under the box, and the typed words are drawn.

Known differences: the pick whip is not on Source Text yet (type or paste the expression), and
JavaScript's text methods such as `.toFixed()` and the `? :` choice are not in our expression
language. A saved project with an expression on its words opens in an older build with the words
as typed.
