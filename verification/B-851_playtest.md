# B-851 playtest: the MP4's estimated size in the Render panel (D-971)

What changed: when the export format is MP4, the Render panel's row says about how big the
file can get, as After Effects' export shows an estimate.

## Steps

1. Open the reference shot (1920 x 1080, 24 frames a second, 10 seconds, no sound).
2. Open the Render panel and set the format to MP4.
3. Set the quality to Preview, then Standard, then High.
4. Set the format to PNG.
5. Render an MP4 at Standard and look at the file's size.

## What to see

- Step 3: the row ends "up to about 5.9 MB" for Preview, "up to about 12 MB" for Standard and
  "up to about 30 MB" for High.
- Step 4: no size is shown (a PNG's size depends on the picture).
- Step 5: the file is no bigger than the estimate. On flat, simple pictures it comes out much
  smaller (an earlier 22-frame test came out at about two fifths of the estimate), which is why
  the row says "up to about".

`B-851_private_app_table.md` has these checks run in a private copy of the app: 4 of 4 pass.

If any of these is not so, B-851 is wrong.
