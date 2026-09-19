# B-21c: GIF and animated PNG out, by hand

Built on 2026-09-19 against D-72, which the owner accepted the same day.

The generated half is two tables. `verification/B-21c_films_table.md`, 18 of 18, walks the GIF
delays of FX-FMT-020 to 023, then exports eight frames of the reference shot as a GIF and as an
animated PNG and reads both files back: the GIF's delays are `4 4 4 4 4 5 4 4`, and every frame
of the animated PNG is value for value the PNG frame the same export writes.
`verification/B-21c_panel_table.md`, 8 of 8, calls what the window calls. This sheet covers what
neither can judge: what the files look like when something other than this program plays them.

## The two files the table made

Running `cargo test --test b21c_films` leaves `reference_shot.gif` and
`reference_shot_animated.png` in `verification/B-21c films/`. They are 20 MB together and are
made again on every run, so they are not kept in git.

1. Drag `reference_shot_animated.png` into Chrome, Edge or Firefox. It plays eight frames and
   loops. It looks exactly like the shot does in the viewer. (Windows Photos shows an animated
   PNG as a still; that is Photos, not the file.)
2. Drag `reference_shot.gif` into the same browser. It plays the same eight frames at the same
   speed. The colours are close but coarser: a GIF has 256 colours a frame. That is the format,
   and it is why the list calls it a preview.

## From the window

3. Open a project of your own with a short work area (set one with B and N; twenty or thirty
   frames is plenty).
4. In the list beside **Export...**, choose **Animated PNG, 8-bit**. Click **Export...** and
   pick a folder. The status line says "Exporting N frames as one animated PNG, into ...", the
   bar fills, and it ends with "Exported N frames as one file, ...`yourproject_animated.png`."
   The folder holds that one file. In a browser it plays like the viewer does, see-through
   where the shot is see-through.
5. Choose **GIF, a preview: 256 colours** and export to the same folder. One file,
   `yourproject.gif`. It plays at the same speed. Soft see-through edges are hard in the GIF:
   a GIF pixel is either there or not, and half or more counts as there.
6. **Cancel.** Start a longer GIF export and click **Cancel export** part way. The status line
   says "Nothing was exported. Export stopped at your request. An animated file is whole or it
   is nothing, so no file was left." The folder has no half-made `.gif` in it.
7. **The old formats are as they were.** Choose **PNG, 8-bit** and export: numbered frames, as
   before.

## Ceilings, stated

- A GIF's 256 colours are picked a frame at a time, with no dithering choice (D-72). On a shot
  with big soft gradients you will see bands. How close the colours come is measured in the
  table (on the reference frame, 1.49 of 255 on average, 54 at the worst single value) but no
  document sets a bar for it; your eye on step 2 is the check.
- An animated PNG is always 8-bit here, because the window's PNG export is. The core writes 16
  bits if asked; nothing in the window asks.
- Neither file carries sound (D-71's ceiling).
- An animated PNG cannot hold a frame rate whose numerator or denominator is above 65535; the
  export is refused with a sentence saying so. 24000/1001 fits.

## Result

(the owner's words go here)
