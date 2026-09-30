# B-172: colour effects next to each other drawn in one go

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 13 of the GPU plan (G13), which you added to the queue "to your discretion", in the form you
kept when you declined the colour table (D-234): the same maths in the same order, D-244.

When a layer has several colour effects one after another (Levels, Curves, Hue/Saturation,
Vibrance, Channel Mixer, Invert and the like: 23 kinds, each of which only looks at the pixel it
is changing), the graphics card used to run them one at a time, writing the whole picture out and
reading it back in between. Now it runs a row of them in one go, handing each pixel straight from
one effect to the next. **The pixels are byte for byte what they were**:
`verification/B-172_fused_table.md` draws 38 frames both ways, with every one of the 23 kinds in
it; all are identical. Exports never use the graphics card and are untouched.

On the reference shot with a Noise and eight colour effects on three layers, at Full: 43.1 to
33.6 ms a frame. Dragging a slider on the fifth of eight colour effects: 14.1 to 13.1 ms
(`verification/B-172_timing_table.md`).

## What to check

Open the reference shot. Set **Draw on: GPU** and **Full resolution**. On one layer, add six or
more colour effects one after another, for example Levels, Hue/Saturation, Curves, Vibrance,
Channel Mixer and Posterize, with settings strong enough to see.

1. **Play.** Play the shot. It should look as it would have before, and play at least as smoothly.
2. **Drag in the middle.** Drag a slider on the third or fourth effect. The picture should follow
   the slider, with nothing flashing or lagging a step behind.
3. **Drag the first and the last.** Do the same on the first effect, then on the last.
4. **Break the row.** Put a Gaussian Blur between two of the colour effects, then drag a slider on
   an effect after the blur. The picture should still follow.
5. **Switch an effect off.** Turn off one effect in the middle of the row with its checkbox, then
   on again. The picture should change and change back.
6. **Draft.** Switch to **Draft** and repeat step 2.

## If something is wrong

Say which step. The most likely fault would be a picture that does not follow a slider inside the
row (steps 2 and 5), because the card now keeps fewer in-between pictures. The check draws a
slider change in the middle of a row four times, back and forth, and each frame matches.
