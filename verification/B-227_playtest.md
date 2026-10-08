# B-227: Moment Map, each pixel from a different moment

Built on 2026-10-08 under your effects loop request ("Take the first non-done row in
docs/effects/EFFECTS.md ... Implement it with GPU support where it makes sense"). This is P0-4's
missing part, a per-pixel time offset, decided as D-347.

Moment Map is our own effect after After Effects' Time Displacement. Each pixel of a layer is
taken from a different moment of that same layer, chosen by how bright a map is at that pixel:
mid grey is now, white is **Max Displacement Time** later, black is that much earlier. The map is
another layer (its effects count, as for Displacement Map), or the layer itself when none is
picked. **Time Resolution** is how many steps a second the moments are rounded to. It is in the
**Time** group, next to Echo and Posterize Time.

It is drawn by the processor, as Echo is: its pictures are the layer at other frames, which only
the processor makes, and picking a pixel from one is a copy. Exports never use the graphics card.

Moments outside the layer (before its first frame or after its last) are clear, as in Echo.

The check, `verification/D-347_moment_map_table.md`, draws all 28 cases of FX-TDISP-001 to 028
against numbers worked out by a separate program before the effect existed. The pictures are in
`verification/D-347 pictures/`: a street sliding left, at frame 12, before (`before.png`), as
added (`as_added.png`), with a black-to-white ramp as the map (`squeeze.png`), and with Max 0
(`still.png`, the same as before). The timings are in `verification/B-227_moment_map_timing_table.md`.

## What to check

Open the reference shot.

1. **As added.** On layer 1 add **Moment Map** (search "time displacement" or "moment"). Play:
   bright parts of the drawing should run ahead of the dark parts, as if smeared in time. Moments
   before the shot starts are clear, so near frame 0 dark parts disappear.
2. **A map layer.** Make a black-to-white gradient layer (or use any other layer), switch it off,
   and pick it as **Time Displacement Layer**. Where the map is white the layer should be ahead,
   where it is black behind, and where it is mid grey unchanged.
3. **Max Displacement Time.** Set it to 0: the layer should be exactly as without the effect. Set
   it to -1: the effect turns the other way round (white behind, black ahead).
4. **Time Resolution.** Set it to 2: the layer should break into a few bands, each a whole half
   second apart. Set it above the frame rate: nothing new appears between frames.
5. **Map Placement.** With a map smaller than the layer, try **Stretch to Fit**, **Centre** and
   **Tile**: centred, the area outside the map should be unchanged.
6. **Save and open.** Save, close and reopen: the settings should be as you left them.
7. **Same as an export.** Export a frame and put it next to the viewer. They should look the same.

## Not done, on purpose or for later

- On an **adjustment layer** it changes nothing (After Effects shifts everything beneath it). This
  is a gap from a tutorial, logged, not worked round.
- A **Time Resolution** above the frame rate gives no in-between pictures: every pixel is one of
  the layer's own frames. After Effects can draw in-between moments of an animated layer.
- Motion estimated between frames (optical flow) is P0-13, which you parked to be built last.

## If something is wrong

Say which step. The most likely fault would be at the edges of the layer's frames (step 1 near
the start or end of the shot), where a moment falls just outside the layer and should be clear.
