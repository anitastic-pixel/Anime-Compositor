# B-150: Time stretch, Frame mix and Drawing dissolve, by hand

Built on 2026-09-29 against D-216 and ADR-020, which you accepted with four answers: a stretched
layer's keys stretch with it, as in After Effects; the Drawing dissolve works whatever the
composition's switch says; sped up, only the two nearest drawings are mixed; and the
composition's switch is off until you turn it on.

- **Time stretch** slows a drawn or composition layer down or speeds it up, in percent: 200 plays
  it at half speed, 50 at double. The layer's end moves so it still shows all it showed.
- **Frame mix** is After Effects' frame blending: where a stretched layer falls between two
  drawings, it shows both, faint, a ghosted in-between. It needs the layer's own switch and the
  composition's **Frame blending** switch over the timeline, both on.
- **Drawing dissolve** fades one drawing into the next over the last frames of its hold, on a
  drawn layer.

The generated half is `verification/B-150_frame_blending_table.md`, 390 of 390 checks passing,
which renders every FX-FBLEND case against the numbers written before the code and draws the
pictures below. B-150c, your "Fix key diamonds", has its own table,
`verification/B-150c_key_diamonds_table.md`, 17 of 17, beside the run from before the fix,
`verification/B-150c_key_diamonds_before.md`, where 13 of those failed. This sheet covers what
the tables cannot: how it looks and feels in the window.

## The pictures

`verification/B-150 pictures/`, twice enlarged, a grey line between frames:

- `ball_0001.png` to `ball_0003.png`: a made-up orange ball in three drawings, moving right.
- `on_twos.png`: the three drawings held two frames each, as drawn: six frames, each ball whole.
- `stretched_held.png`: Time stretch 200, no mixing: each drawing held four frames, whole.
- `stretched_mixed.png`: Time stretch 200 with Frame mix: the fourth and eighth frames show
  two faint balls, a ghosted in-between; the rest are whole.
- `dissolved.png`: Drawing dissolve 1: the second frame of each hold is half this drawing and
  half the next; the last drawing just holds.

## Before you start

Make a composition 48 by 32 at 24 frames a second and zoom the viewer in. Import `ball_0001.png`
to `ball_0003.png` as one sequence and put it in the composition. Make each drawing hold two
frames, by dragging the end of each drawing's block on the layer's bar, as for any cel on twos;
it should look like `on_twos.png`. Press **Full resolution**.

## What to check

1. **Where it is.** Select the layer: the panel on the right shows **Time stretch** 100 %,
   **Frame mix** unticked and **Drawing dissolve** 0 frames. Over the timeline, beside **Motion
   blur**, is a **Frame blending** switch, off. A solid, shape, null or sound layer shows none of
   the three; a composition layer shows Time stretch and Frame mix only.
2. **Stretch.** Type 200 in Time stretch: the layer's bar gets twice as long and each drawing
   holds four frames, whole, as `stretched_held.png`.
3. **Frame mix, one switch.** Tick Frame mix: nothing changes yet, because Frame blending over
   the timeline is off.
4. **Frame mix, both switches.** Turn on **Frame blending**: frames 3 and 7, the fourth and eighth, show two faint balls,
   as `stretched_mixed.png`; the rest are unchanged. Turn it off: they go back.
5. **Speed up.** Time stretch 50: the bar halves and the ball skips drawings; with Frame mix on,
   the frames still look whole or mix just two drawings, never three.
6. **Dissolve.** Time stretch back to 100 and Frame blending off. Drawing dissolve 1: the second
   frame of each hold is half one ball and half the next, as `dissolved.png`, with the composition
   switch off. Drawing dissolve 2 on twos does the same, since a hold of two can dissolve over one
   frame at most.
7. **Keys stretch.** Time stretch 100, dissolve 0. Key Position at frame 0 and frame 6 so the ball
   also slides. Set Time stretch 200: the slide now takes 12 frames, ending where the bar ends,
   as in After Effects. The two Position diamonds on the timeline move with it, to frames 0 and
   12: the second sits right where the ball stops.
8. **Key diamonds, stretched.** Still at 200, put the playhead on frame 6 and press Position's
   diamond: the new diamond appears right under the playhead. Drag the diamond at 12 to 18: it
   follows the pointer two frames at a time (at 200 % a key can only sit on every other frame)
   and stays where you let go; play, and the ball stops on frame 18. J and K jump the playhead
   from diamond to diamond. Ctrl+Z puts it back on 12.
9. **Key diamonds, sped up.** Time stretch 50: the diamonds close up to frames 0, 1½ and 3, the
   middle one half way between two frames, which is where it plays. Click it: the playhead goes
   to frame 2. Time stretch back to 100: they are at 0, 3 and 6, the frames they are kept at.
10. **Trim.** On the layer stretched to 200, drag its start two frames later: it is taken, and
   the frames left look as they did. Drag it by one frame: it is refused with a sentence, because
   that would start the layer part way through a drawing.
11. **Out of range.** Type 0 or 10001 in Time stretch, or 101 in Drawing dissolve: refused with a
   sentence saying what it runs to, and the number goes back.
12. **Card.** With the graphics card on, a mixed or dissolved frame is drawn on the processor and
    the viewer says so; a stretched frame with no mixing stays on the card.
13. **Draft.** Press **Draft**: the picture is smaller and the same frames are mixed.
14. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the stretch,
    both switches and the dissolve are still there.

## Known limits, on purpose

- A stretched layer's keys are kept at whole frames of the layer's own time, so at 200 % a key
  can only be on every other frame and a dragged one lands on the nearest; at 50 % some play half
  way between two frames.
- The drawing blocks on a stretched drawn layer's bar are still drawn at their unstretched frames;
  the picture and the key diamonds are right.
- Frame mix and the dissolve run on the processor; a graphics card version is its own later unit.
- There is no Pixel Motion, After Effects' optical-flow blending: D-216 left it out.
- No time remapping curve: the stretch is one steady speed.
- No timing measured in this unit; nothing here claims a speed.

## What to answer

"works", or which step number did something else and what it did.
