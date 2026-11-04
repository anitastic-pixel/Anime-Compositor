# B-953 playtest: Interpret Footage Loop (D-1073)

The tutorial (nTpT2PELCbA, about 1:30) opens Interpret Footage on an image sequence and sets
Loop to 6 times. Every layer made from it then plays the drawings through six times.

## Steps

1. Open a project with an image sequence in the Project panel (the reference shot will do).
2. Click the sequence in the Project panel. Its line at the top of the panel shows
   **Interpret as**, and beside it **Loop [1] times**. A still or a sound has no Loop box.
3. Type 3 in the Loop box and press Enter (or Tab away).
4. With the sequence still chosen, make a New Comp from Selection. The new composition is three
   times as long as the sequence has drawings (24 drawings make 72 frames).
5. Play it. The drawings run through once, then start again from the first, three times in all,
   with no blank or held frame at the joins.
6. Drag the sequence into another composition. That layer loops 3 times too.
7. Save, close and open the project again. The Loop box still says 3.
8. Press Ctrl+Z until the Loop box says 1 again: setting Loop is one step to undo.
9. Type 0 or 100 in the box. The status line says the sequence cannot loop that many times, and nothing
   changes.

## What you should see

`verification/B-953 pictures/loop.png`: nine frames of a composition made from a sequence set to
loop 3 times. Each row is one pass through the drawings; the columns are its first, middle and
last drawing. The three rows are the same. The table `verification/B-953_interpret_loop_table.md`
measured them as identical, pixel for pixel.

Wrong if:
- the rows of the picture differ, or the circle stops moving after the first pass when you play;
- the composition from step 4 is not three times as long as the sequence;
- the Loop box shows for a still.

Not the same as After Effects:
- In After Effects, changing Loop changes the layers already made from the footage. Here it
  changes only layers and compositions made after it; an older layer keeps its own exposure
  sheet, which you can extend in the Sheet.
- Only Loop is built; Interpret Footage's frame rate, fields and pixel aspect are not.
