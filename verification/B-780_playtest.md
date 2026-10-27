# B-780 playtest: frames made ahead give way to your hand (D-900, PROPOSED)

What changed: while a shot plays, the program keeps making the frames playback will ask for next,
as before (B-154). Those frames are now made on low-priority threads and stop at once when you
ask for a frame yourself (a click, a drag, an undo), so they never hold up what you are doing.
The pictures themselves are unchanged.

What did not change: D-900 also tried making the loop ahead while you are not playing, once your
hand has been still for half a second, and making it with several workers at once. Both are
switched off. Several workers made first playback slower, and making frames at rest made the next
drag on the processor at Full slower. They wait for your decision in D-900.

## Steps

1. Open a heavy project (many layers, or effects such as blurs and glows). Viewer on **Full**.
2. Press play and let the loop run once.
   - **Expect:** as before: the loop fills in and plays smoothly from the second time round.
3. While it plays, click a frame on the ruler, then drag a layer.
   - **Expect:** playback stops and the frame you clicked shows as quickly as before; the drag
     follows your hand as before.
4. Change a layer (for example its opacity), then play again.
   - **Expect:** every frame shows the change. None shows the layer as it was.
5. Press **Ctrl+Z** (undo), then play again.
   - **Expect:** every frame shows the undone layer, never the changed one.
6. Switch to another composition tab while the first is playing, play it, then switch back.
   - **Expect:** each shows only its own frames.
7. Rest on a frame without playing for a few seconds, then press play.
   - **Expect:** as before; nothing is made while you rest (that is the part left off).

## If something looks wrong

Say which step, and whether playback was **slower**, a frame was **wrong** (an older picture
after a change), **blank**, or **stuck**. The byte-for-byte check behind this is
`verification/B-780_ram_workers_table.md`.
