# B-21d: MP4 out, by hand

Built on 2026-09-19 against D-72, which the owner accepted the same day, on the road D-30
recommends: the H.264 encoder Windows carries. **D-30 was closed by the owner on 2026-09-19**; see the
last section.

The generated half is two tables. `verification/B-21d_mp4_table.md`, 16 of 16, writes a small
film at each frame rate FX-FMT-030 lists and reads the time numbers back out of the file's own
bytes (24000/1001 is exact: timescale 24000, every frame 1001), checks the colour arithmetic
against BT.709's published colour-bar numbers, asks for the three odd-size refusals, exports 22
frames of the reference shot, and cancels one. `verification/B-21d_panel_table.md`, 5 of 5,
calls what the window calls. This sheet covers what neither can judge: what the film looks like
when something other than this program plays it.

## The film the table made

Running `cargo test --test b21d_mp4` leaves `reference_shot.mp4`, 430 KB, in
`verification/B-21d film/`. It is frames 16 to 37 of the reference shot, the longest run with no
deliberately missing drawing.

1. Double-click `reference_shot.mp4`. It plays in Windows' own player (and in VLC or a browser
   if you drag it there): a little under a second, the right way up, blue sky, red circle,
   yellow square, green grass, the colours as the viewer shows them. Where the shot is
   see-through the film is black: an MP4 has no see-through.

## From the window

2. Open a project of your own with a short work area (B and N; one or two seconds is plenty).
3. In the list beside **Export...**, choose **MP4 (H.264), over black, no sound**. Click
   **Export...** and pick a folder. The status line says "Exporting N frames as one MP4, over
   black and with no sound, into ...", the bar fills, and it ends with "Exported N frames as one
   file, ...`yourproject.mp4`." The folder holds that one file, and it plays at the speed the
   viewer plays.
4. **An odd size is refused.** Press Ctrl+K and make the composition one pixel narrower (1919
   wide, say). Export as MP4. The status line says an MP4 needs an even width and an even
   height and names the odd one: "its width, 1919, is odd". No file is made, and nothing is
   padded or cropped behind your back. Set the width back.
5. **Cancel.** Start a longer MP4 export and click **Cancel export** part way. "Nothing was
   exported. ... no file was left." The folder has no half-made `.mp4`.
6. **The other formats are as they were.** Choose **PNG, 8-bit** and export: numbered frames.

## Ceilings, stated

- **On Windows the encoder is Windows' own.** This program ships no encoder inside it. On macOS
  or Linux (B-21e) the frames go to an ffmpeg you have installed, and with none found the export
  is refused with a sentence saying so. That road is tested on Windows only: this program has
  never been built or run on macOS or Linux.
- **Over black, no sound.** D-72 and D-71's ceilings.
- **The quality is one fixed rule**: 0.2 bits for every pixel of every frame (about 10 megabits
  a second at 1920x1080 and 24 frames a second), no less than 1 and no more than 100 megabits.
  There is no quality choice in the window. The reference film came out at 3.8 megabits a
  second because it is a simple picture. If flat cel colour shows blocks on your own shot, say
  so and the number moves.
- **H.264's plainest profile (Baseline).** The richer one (High) makes Windows write a table of
  reordered frame times that the exact-clock step below would have to rewrite too; it refuses
  such a file instead of guessing. Baseline files are somewhat larger for the same quality and
  play everywhere.
- **The clock is corrected after Windows writes the file.** Windows counts 23976 ticks a second
  with frames of 1000 for 24000/1001, which is near and is not FX-FMT-030. The build rewrites
  only the time numbers in the file's index, each in place and the same size; the pictures are
  untouched. If the index ever holds something the step does not expect, the export fails with
  a sentence and the file is removed.
- **The colour is BT.709, done here**, because Windows' own conversion uses the
  standard-definition numbers. The file does not carry a label saying BT.709; players assume it
  for HD sizes, which is right. **For a small composition (under about 720 lines) some players
  assume the older numbers and greens and reds shift slightly.** Writing the label means
  rewriting the H.264 stream's own header, which this build does not do.
- Measured by hand, once, with ffprobe and ffmpeg on the development machine (Windows 11), not
  by a test: the 24000/1001 film reads back as 24000/1001 with 48 frames in 48048 ticks; the
  reference film as H.264 Constrained Baseline, 1920x1080, 24/1, 22 frames; a flat colour
  written as 0, 128, 200 decodes as 1, 125, 202, which is H.264's loss at this bitrate.
  ffmpeg is not part of this program and no test depends on it.

## D-30: the one sentence this needs from you

D-30 says which road MP4 export takes, and it is a decision about what you may distribute, so
it is yours and not the agent's. D-72 recommends the encoder Windows carries (Media
Foundation): nothing is shipped inside this program, and the cost is that MP4 export is Windows
only. You said "proceed with B-21b, B-21c, and B-21d" with that in front of you, and then
"proceed with the next items as well", but you have not named the road yourself, and D-30 says
B-21d is not committed until you do. So B-21d is built, tested and sitting uncommitted.

Say **"D-30: Media Foundation, confirmed"** (or your own words for it) and it is committed and
D-30 closes. Say another road and the encoder half is rebuilt on it; the clock, the refusals,
the window and the tables stay.

## Result

D-30, the owner on 2026-09-19: "road 1, Media Foundation confirmed".

The owner, on 2026-09-19: "exporting worked very well"; "b-21c and b-21d can be passed for playtest".
