# B-954 playtest: Ctrl + wheel sizes the expression text (D-1074)

The tutorial (4GeGLgK2BBk, about 3:40) holds Ctrl and turns the mouse wheel over an expression
to make its text bigger. This is a change to the window only, so there is no picture: the check
is the steps below and the table `verification/B-954_expression_text_size_table.md`.

## Steps

1. Open the reference shot. Alt-click the stopwatch of any layer's Position to give it an
   expression. The expression box opens under the property.
2. Hold Ctrl and turn the wheel away from you over the box, one notch at a time. The text grows
   a pixel per notch. Turn it towards you: it shrinks.
3. Keep turning: it stops growing at 32 pixels and stops shrinking at 8.
4. Turn the wheel without Ctrl over the box: the timeline scrolls as before, and the text size
   does not change.
5. Give a second property an expression. Its box opens at the same size.
6. Close the window and open it again. The expression boxes keep the size you chose.

Wrong if:
- Ctrl + wheel over the box zooms the timeline or the whole window instead;
- the plain wheel stops scrolling the timeline over an expression box;
- the size goes back to small after reopening the window.

Not the same as After Effects:
- The size is the window's preference, kept for every project, not saved in the project.
