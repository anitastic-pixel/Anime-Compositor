# B-22c: the export choices, by hand

Built on 2026-09-19 against D-73, which the owner accepted the same day ("accept D-73, proceed
with B-22b and B-22c").

The generated half is two tables. `verification/B-22b_choices_table.md`, 20 of 20, checks the
bits a second asked of the encoder at each quality (FX-FMT-040), the dithering to the pixel
(FX-FMT-050 to 052), and that the written MP4 says it is BT.709 (FX-FMT-060), on both roads.
`verification/B-22c_panel_table.md`, 13 of 13, checks the page's two controls, what they send,
the refusal, and the status line. This sheet covers what neither can judge: whether High looks
better than Preview, whether dithering looks better than not, and that the choice made in the
window reaches the file.

## The films the table made

Running `cargo test --test b22b_choices` leaves five files in `verification/B-22 films/` (not
kept in git; about five minutes to make). The side-by-side pictures in
`verification/B-22c playtest/` were cut from them and enlarged.

1. **Preview against High.** Open `preview_against_high.png`. Left is a piece of the last frame
   of `preview.mp4`, right the same piece of `high.mp4`, both enlarged four times. High should
   be a little crisper in the scratchy texture; Preview a little softer, and still a fair
   picture. The difference is small, and the agent measured why: on this 22-frame shot the
   first 17 frames of the two films are the same to the pixel, and only the last five differ.
   The encoder Windows carries spends the same on a short, simple start whatever it is allowed,
   and the allowance only begins to bite later. A longer, busier shot of your own (step 7) is
   the fairer trial. Then play `preview.mp4`, `standard.mp4` and `high.mp4`: the same shot, the
   same speed, the same colours. On this shot they weigh about 364, 429 and 510 KB.
2. **Plain against dithered.** Open `plain_against_dithered.png`. Left is `plain.gif`, right
   `dithered.gif`. Where the plain one shows steps in soft shading, the dithered one should show
   a fine grain instead. Then open both GIFs in a browser.
3. **What to look for that is not right.** The agent saw stray coloured specks in
   `dithered.gif`, and they are plain in the side-by-side picture: red in the pink tree, yellow
   in its dark trunk, orange and pink in the grey cloud. The sky's bands are gone, as intended. That is the accepted rule doing what it says: the
   error it carries forward builds up where the 256 colours have nothing close. If it bothers
   you, say so: the mend is a small change to D-73 (holding the carried error within limits),
   with its fixture first, and not a repair to this build.

## From the window

4. Open a project of your own with a short work area (B and N; a second or two).
5. In the list beside **Export...**, three entries now end in **lossless**: PNG, EXR full float
   and animated PNG. Those keep every value; there is no quality to choose for them.
6. Choose **MP4**. A second list appears beside it: **Preview, smallest**, **Standard**,
   **High, largest**, on Standard. Choose **GIF**: the list goes and a **Dither** tick box
   appears. Choose **PNG**: neither shows.
7. Choose **MP4** and **High**, export into a folder. The status line says "... as one MP4 at
   High quality, over black and with no sound, ...". Rename the file, export again at
   **Preview**. The Preview file should be the smaller, and both play alike. (On a very small or
   very simple composition the two can come out the same size: the number asked for is never
   under one megabit a second, and an encoder does not spend bits a simple picture does not
   need.)
8. Choose **GIF**, tick **Dither**, export. The status line ends its format with "dithered,".
   Untick and export again under another name: the dithered file is bigger and grainier.
9. **Kept for the sitting.** After an export the page reloads: the format, the quality and the
   tick should still be as you left them.

## Ceilings, stated

- Three qualities and no number box; one dithering method (D-73).
- Dithering a frame of photographic noise is slow: the nearest colour is found by a plain search.
  Flat cel colour is quick.
- High stays on H.264's Baseline profile on the Windows road, so it is not the best an MP4 can be
  at that size.
- What a file weighs depends on the picture; only the number asked of the encoder is a fixture.
- The ffmpeg road (not Windows) passes the same fixture rows when run here on Windows. The
  program has still never been built or run on macOS or Linux.
- No test can answer the Windows folder dialog, so step 7 is the only proof that the choice made
  in the window reaches the file.

## Result

(for the owner)
